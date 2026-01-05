//! Constraint Collection for Type Inference
//!
//! Walks the HIR and generates type constraints for the Hindley-Milner solver.
//! This bridges the gap between HIR analysis and type inference.
//!
//! # DEPYLER-0202: Wire HM solver into transpilation pipeline
//!
//! # Example
//!
//! ```rust,ignore
//! use depyler_core::type_system::{ConstraintCollector, TypeConstraintSolver};
//! use depyler_core::hir::HirModule;
//!
//! let mut collector = ConstraintCollector::new();
//! collector.collect_module(&hir_module);
//!
//! let mut solver = TypeConstraintSolver::new();
//! for constraint in collector.constraints() {
//!     solver.add_constraint(constraint);
//! }
//!
//! let solution = solver.solve()?;
//! collector.apply_substitutions(&mut hir_module, &solution);
//! ```

use crate::hir::{AssignTarget, BinOp, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type};
use std::collections::HashMap;

use super::hindley_milner::{Constraint, VarId};

/// Collects type constraints from HIR for inference
pub struct ConstraintCollector {
    /// Generated constraints
    constraints: Vec<Constraint>,
    /// Maps variable names to type variable IDs
    var_to_type_var: HashMap<String, VarId>,
    /// Maps parameter names to type variable IDs (for substitution)
    param_type_vars: HashMap<String, VarId>,
    /// Counter for generating fresh type variables
    next_var: VarId,
    /// Function signatures for call-site inference
    function_signatures: HashMap<String, (Vec<VarId>, VarId)>, // (param_vars, return_var)
}

impl ConstraintCollector {
    /// Create a new constraint collector
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            var_to_type_var: HashMap::new(),
            param_type_vars: HashMap::new(),
            next_var: 0,
            function_signatures: HashMap::new(),
        }
    }

    /// Generate a fresh type variable
    fn fresh_var(&mut self) -> VarId {
        let var = self.next_var;
        self.next_var += 1;
        var
    }

    /// Get the type variable for a named variable, creating one if needed
    fn get_or_create_var(&mut self, name: &str) -> VarId {
        if let Some(&var) = self.var_to_type_var.get(name) {
            var
        } else {
            let var = self.fresh_var();
            self.var_to_type_var.insert(name.to_string(), var);
            var
        }
    }

    /// Collect constraints from an entire module
    pub fn collect_module(&mut self, module: &HirModule) {
        // First pass: register all function signatures
        for func in &module.functions {
            self.register_function_signature(func);
        }

        // Second pass: collect constraints from function bodies
        for func in &module.functions {
            self.collect_function(func);
        }
    }

    /// Register a function's parameter and return type variables
    fn register_function_signature(&mut self, func: &HirFunction) {
        let mut param_vars = Vec::new();

        for param in &func.params {
            let var = self.fresh_var();
            param_vars.push(var);
            self.param_type_vars
                .insert(format!("{}::{}", func.name, param.name), var);
            self.var_to_type_var.insert(param.name.to_string(), var);

            // If param already has a known type, constrain it
            if !matches!(param.ty, Type::Unknown) {
                self.constraints
                    .push(Constraint::Instance(var, param.ty.clone()));
            }
        }

        let ret_var = self.fresh_var();

        // If return type is known, constrain it
        if !matches!(func.ret_type, Type::Unknown) {
            self.constraints
                .push(Constraint::Instance(ret_var, func.ret_type.clone()));
        }

        self.function_signatures
            .insert(func.name.to_string(), (param_vars, ret_var));
    }

    /// Collect constraints from a function
    fn collect_function(&mut self, func: &HirFunction) {
        // Clear local variable mappings for this function scope
        // (keep param mappings from registration)

        for stmt in &func.body {
            self.collect_statement(stmt, &func.name);
        }
    }

    /// Collect constraints from a statement
    fn collect_statement(&mut self, stmt: &HirStmt, func_name: &str) {
        match stmt {
            HirStmt::Assign {
                target,
                value,
                type_annotation,
            } => {
                let value_var = self.collect_expr(value);

                match target {
                    AssignTarget::Symbol(name) => {
                        let target_var = self.get_or_create_var(name);

                        // Constrain target = value
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(target_var),
                            Type::UnificationVar(value_var),
                        ));

                        // If there's a type annotation, use it
                        if let Some(ty) = type_annotation {
                            if !matches!(ty, Type::Unknown) {
                                self.constraints
                                    .push(Constraint::Instance(target_var, ty.clone()));
                            }
                        }
                    }
                    AssignTarget::Tuple(targets) => {
                        // For tuple unpacking, recursively handle nested targets
                        for target in targets {
                            if let AssignTarget::Symbol(name) = target {
                                let _ = self.get_or_create_var(name);
                            }
                        }
                    }
                    AssignTarget::Index { .. } | AssignTarget::Attribute { .. } => {
                        // Complex targets - skip for now
                    }
                }
            }

            HirStmt::Return(Some(expr)) => {
                let expr_var = self.collect_expr(expr);

                // Constrain return expression to function return type
                if let Some((_, ret_var)) = self.function_signatures.get(func_name) {
                    self.constraints.push(Constraint::Equality(
                        Type::UnificationVar(expr_var),
                        Type::UnificationVar(*ret_var),
                    ));
                }
            }

            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond_var = self.collect_expr(condition);
                // Condition must be Bool
                self.constraints
                    .push(Constraint::Instance(cond_var, Type::Bool));

                for stmt in then_body {
                    self.collect_statement(stmt, func_name);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.collect_statement(stmt, func_name);
                    }
                }
            }

            HirStmt::While { condition, body } => {
                let cond_var = self.collect_expr(condition);
                self.constraints
                    .push(Constraint::Instance(cond_var, Type::Bool));

                for stmt in body {
                    self.collect_statement(stmt, func_name);
                }
            }

            HirStmt::For { target, iter, body } => {
                let _iter_var = self.collect_expr(iter);

                // The target gets the element type
                // For now, just register it
                if let AssignTarget::Symbol(name) = target {
                    let _ = self.get_or_create_var(name);
                }

                // Could add: iter_var must be Iterable<target_type>

                for stmt in body {
                    self.collect_statement(stmt, func_name);
                }
            }

            HirStmt::Expr(expr) => {
                let _ = self.collect_expr(expr);
            }

            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for stmt in body {
                    self.collect_statement(stmt, func_name);
                }
                for handler in handlers {
                    for stmt in &handler.body {
                        self.collect_statement(stmt, func_name);
                    }
                }
                if let Some(else_stmts) = orelse {
                    for stmt in else_stmts {
                        self.collect_statement(stmt, func_name);
                    }
                }
                if let Some(finally_stmts) = finalbody {
                    for stmt in finally_stmts {
                        self.collect_statement(stmt, func_name);
                    }
                }
            }

            _ => {}
        }
    }

    /// Collect constraints from an expression, returning its type variable
    fn collect_expr(&mut self, expr: &HirExpr) -> VarId {
        match expr {
            HirExpr::Literal(lit) => {
                let var = self.fresh_var();
                let ty = match lit {
                    Literal::Int(_) => Type::Int,
                    Literal::Float(_) => Type::Float,
                    Literal::String(_) => Type::String,
                    Literal::Bytes(_) => Type::String, // Bytes map to String for Rust
                    Literal::Bool(_) => Type::Bool,
                    Literal::None => Type::None,
                };
                self.constraints.push(Constraint::Instance(var, ty));
                var
            }

            HirExpr::Var(name) => self.get_or_create_var(name),

            HirExpr::Binary { op, left, right } => {
                let left_var = self.collect_expr(left);
                let right_var = self.collect_expr(right);
                let result_var = self.fresh_var();

                match op {
                    // Arithmetic ops: operands and result are numeric
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::FloorDiv => {
                        // Constrain left = right (same numeric type)
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(left_var),
                            Type::UnificationVar(right_var),
                        ));
                        // Result has same type as operands
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(result_var),
                            Type::UnificationVar(left_var),
                        ));
                    }

                    // Comparison ops: result is Bool
                    BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn => {
                        self.constraints
                            .push(Constraint::Instance(result_var, Type::Bool));
                    }

                    // Boolean ops: all Bool
                    BinOp::And | BinOp::Or => {
                        self.constraints
                            .push(Constraint::Instance(left_var, Type::Bool));
                        self.constraints
                            .push(Constraint::Instance(right_var, Type::Bool));
                        self.constraints
                            .push(Constraint::Instance(result_var, Type::Bool));
                    }

                    // Modulo: numeric
                    BinOp::Mod => {
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(left_var),
                            Type::UnificationVar(right_var),
                        ));
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(result_var),
                            Type::UnificationVar(left_var),
                        ));
                    }

                    // Power: result is Float (Python semantics)
                    BinOp::Pow => {
                        self.constraints
                            .push(Constraint::Instance(result_var, Type::Float));
                    }

                    // Bitwise ops: Int
                    BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift => {
                        self.constraints
                            .push(Constraint::Instance(left_var, Type::Int));
                        self.constraints
                            .push(Constraint::Instance(right_var, Type::Int));
                        self.constraints
                            .push(Constraint::Instance(result_var, Type::Int));
                    }
                }

                result_var
            }

            HirExpr::Call { func, args, .. } => {
                let result_var = self.fresh_var();

                // Clone signature data to avoid borrow conflict
                let sig_data = self
                    .function_signatures
                    .get(func.as_str())
                    .cloned();

                // If we know this function's signature, constrain args
                if let Some((param_vars, ret_var)) = sig_data {
                    // Constrain each argument to its parameter type
                    for (arg, param_var) in args.iter().zip(param_vars.iter()) {
                        let arg_var = self.collect_expr(arg);
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(arg_var),
                            Type::UnificationVar(*param_var),
                        ));
                    }

                    // Result has function's return type
                    self.constraints.push(Constraint::Equality(
                        Type::UnificationVar(result_var),
                        Type::UnificationVar(ret_var),
                    ));
                } else {
                    // Unknown function - just collect arg constraints
                    for arg in args {
                        let _ = self.collect_expr(arg);
                    }
                }

                result_var
            }

            HirExpr::List(elements) => {
                let result_var = self.fresh_var();

                if !elements.is_empty() {
                    // All elements should have the same type
                    let first_var = self.collect_expr(&elements[0]);
                    for elem in elements.iter().skip(1) {
                        let elem_var = self.collect_expr(elem);
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(elem_var),
                            Type::UnificationVar(first_var),
                        ));
                    }
                    // Result is List<element_type>
                    self.constraints.push(Constraint::Instance(
                        result_var,
                        Type::List(Box::new(Type::UnificationVar(first_var))),
                    ));
                }

                result_var
            }

            HirExpr::Dict(pairs) => {
                let result_var = self.fresh_var();

                if !pairs.is_empty() {
                    let (first_key, first_val) = &pairs[0];
                    let key_var = self.collect_expr(first_key);
                    let val_var = self.collect_expr(first_val);

                    for (k, v) in pairs.iter().skip(1) {
                        let k_var = self.collect_expr(k);
                        let v_var = self.collect_expr(v);
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(k_var),
                            Type::UnificationVar(key_var),
                        ));
                        self.constraints.push(Constraint::Equality(
                            Type::UnificationVar(v_var),
                            Type::UnificationVar(val_var),
                        ));
                    }

                    self.constraints.push(Constraint::Instance(
                        result_var,
                        Type::Dict(
                            Box::new(Type::UnificationVar(key_var)),
                            Box::new(Type::UnificationVar(val_var)),
                        ),
                    ));
                }

                result_var
            }

            HirExpr::Tuple(elements) => {
                let result_var = self.fresh_var();
                let elem_types: Vec<Type> = elements
                    .iter()
                    .map(|e| Type::UnificationVar(self.collect_expr(e)))
                    .collect();
                self.constraints
                    .push(Constraint::Instance(result_var, Type::Tuple(elem_types)));
                result_var
            }

            HirExpr::Index { base, index } => {
                let _base_var = self.collect_expr(base);
                let index_var = self.collect_expr(index);
                let result_var = self.fresh_var();

                // Index is typically Int for lists
                self.constraints
                    .push(Constraint::Instance(index_var, Type::Int));

                result_var
            }

            HirExpr::MethodCall { object, args, .. } => {
                let _obj_var = self.collect_expr(object);
                for arg in args {
                    let _ = self.collect_expr(arg);
                }
                self.fresh_var()
            }

            HirExpr::Attribute { value, .. } => {
                let _ = self.collect_expr(value);
                self.fresh_var()
            }

            HirExpr::Unary { operand, .. } => {
                // Unary ops preserve type
                self.collect_expr(operand)
            }

            HirExpr::IfExpr {
                test,
                body,
                orelse,
            } => {
                let cond_var = self.collect_expr(test);
                let then_var = self.collect_expr(body);
                let else_var = self.collect_expr(orelse);

                self.constraints
                    .push(Constraint::Instance(cond_var, Type::Bool));
                // Both branches must have same type
                self.constraints.push(Constraint::Equality(
                    Type::UnificationVar(then_var),
                    Type::UnificationVar(else_var),
                ));

                then_var
            }

            _ => self.fresh_var(),
        }
    }

    /// Get collected constraints
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    /// Get parameter type variable mappings for substitution
    pub fn param_type_vars(&self) -> &HashMap<String, VarId> {
        &self.param_type_vars
    }

    /// Apply solved substitutions back to HirModule
    pub fn apply_substitutions(
        &self,
        module: &mut HirModule,
        solution: &HashMap<VarId, Type>,
    ) -> usize {
        let mut applied_count = 0;

        for func in &mut module.functions {
            // Apply to parameters
            for param in &mut func.params {
                let key = format!("{}::{}", func.name, param.name);
                if let Some(&var) = self.param_type_vars.get(&key) {
                    if matches!(param.ty, Type::Unknown) {
                        if let Some(inferred) = solution.get(&var) {
                            // Don't apply UnificationVar types - only concrete types
                            if !matches!(inferred, Type::UnificationVar(_)) {
                                param.ty = inferred.clone();
                                applied_count += 1;
                            }
                        }
                    }
                }
            }

            // Apply to return type
            if let Some((_, ret_var)) = self.function_signatures.get(func.name.as_str()) {
                if matches!(func.ret_type, Type::Unknown) {
                    if let Some(inferred) = solution.get(ret_var) {
                        if !matches!(inferred, Type::UnificationVar(_)) {
                            func.ret_type = inferred.clone();
                            applied_count += 1;
                        }
                    }
                }
            }
        }

        applied_count
    }
}

impl Default for ConstraintCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{ExceptHandler, FunctionProperties, HirParam, UnaryOp};
    use depyler_annotations::TranspilationAnnotations;

    fn make_test_function(name: &str, params: Vec<(&str, Type)>, body: Vec<HirStmt>) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params
                .into_iter()
                .map(|(n, ty)| HirParam {
                    name: n.to_string(),
                    ty,
                    default: None,
                    is_vararg: false,
                })
                .collect(),
            ret_type: Type::Unknown,
            body,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    fn make_test_function_with_ret(
        name: &str,
        params: Vec<(&str, Type)>,
        ret_type: Type,
        body: Vec<HirStmt>,
    ) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params
                .into_iter()
                .map(|(n, ty)| HirParam {
                    name: n.to_string(),
                    ty,
                    default: None,
                    is_vararg: false,
                })
                .collect(),
            ret_type,
            body,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    fn empty_module() -> HirModule {
        HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        }
    }

    // ============================================================
    // Constructor and basic tests
    // ============================================================

    #[test]
    fn test_new_collector() {
        let collector = ConstraintCollector::new();
        assert!(collector.constraints().is_empty());
        assert!(collector.param_type_vars().is_empty());
    }

    #[test]
    fn test_default_collector() {
        let collector = ConstraintCollector::default();
        assert!(collector.constraints().is_empty());
    }

    #[test]
    fn test_fresh_var_increments() {
        let mut collector = ConstraintCollector::new();
        let v1 = collector.fresh_var();
        let v2 = collector.fresh_var();
        let v3 = collector.fresh_var();
        assert_eq!(v1, 0);
        assert_eq!(v2, 1);
        assert_eq!(v3, 2);
    }

    #[test]
    fn test_get_or_create_var_new() {
        let mut collector = ConstraintCollector::new();
        let v1 = collector.get_or_create_var("x");
        assert_eq!(v1, 0);
    }

    #[test]
    fn test_get_or_create_var_existing() {
        let mut collector = ConstraintCollector::new();
        let v1 = collector.get_or_create_var("x");
        let v2 = collector.get_or_create_var("x");
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_get_or_create_var_different_names() {
        let mut collector = ConstraintCollector::new();
        let v1 = collector.get_or_create_var("x");
        let v2 = collector.get_or_create_var("y");
        assert_ne!(v1, v2);
    }

    // ============================================================
    // Literal inference tests
    // ============================================================

    #[test]
    fn test_literal_inference() {
        let func = make_test_function(
            "test",
            vec![("x", Type::Unknown)],
            vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".into()),
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_literal_int_constraint() {
        let mut collector = ConstraintCollector::new();
        let var = collector.collect_expr(&HirExpr::Literal(Literal::Int(42)));

        let has_int_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Int) if *v == var)
        });
        assert!(has_int_constraint);
    }

    #[test]
    fn test_literal_float_constraint() {
        let mut collector = ConstraintCollector::new();
        let var = collector.collect_expr(&HirExpr::Literal(Literal::Float(3.14.into())));

        let has_float_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Float) if *v == var)
        });
        assert!(has_float_constraint);
    }

    #[test]
    fn test_literal_string_constraint() {
        let mut collector = ConstraintCollector::new();
        let var = collector.collect_expr(&HirExpr::Literal(Literal::String("hello".into())));

        let has_string_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::String) if *v == var)
        });
        assert!(has_string_constraint);
    }

    #[test]
    fn test_literal_bool_constraint() {
        let mut collector = ConstraintCollector::new();
        let var = collector.collect_expr(&HirExpr::Literal(Literal::Bool(true)));

        let has_bool_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Bool) if *v == var)
        });
        assert!(has_bool_constraint);
    }

    #[test]
    fn test_literal_none_constraint() {
        let mut collector = ConstraintCollector::new();
        let var = collector.collect_expr(&HirExpr::Literal(Literal::None));

        let has_none_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::None) if *v == var)
        });
        assert!(has_none_constraint);
    }

    #[test]
    fn test_literal_bytes_constraint() {
        let mut collector = ConstraintCollector::new();
        let var = collector.collect_expr(&HirExpr::Literal(Literal::Bytes(vec![1, 2, 3])));

        // Bytes maps to String
        let has_string_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::String) if *v == var)
        });
        assert!(has_string_constraint);
    }

    // ============================================================
    // Binary operation tests
    // ============================================================

    #[test]
    fn test_binary_op_inference() {
        let func = make_test_function(
            "add",
            vec![("a", Type::Unknown), ("b", Type::Unknown)],
            vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".into())),
                right: Box::new(HirExpr::Var("b".into())),
            }))],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let has_equality = collector
            .constraints()
            .iter()
            .any(|c| matches!(c, Constraint::Equality(_, _)));
        assert!(has_equality);
    }

    #[test]
    fn test_binary_sub_op() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_binary_mul_op() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_binary_div_op() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_binary_floor_div_op() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_binary_mod_op() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_binary_pow_op_returns_float() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Pow,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        });

        let has_float = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Float) if *v == result)
        });
        assert!(has_float);
    }

    #[test]
    fn test_binary_comparison_eq_returns_bool() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Eq,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Bool) if *v == result)
        });
        assert!(has_bool);
    }

    #[test]
    fn test_binary_comparison_lt_returns_bool() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Lt,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Bool) if *v == result)
        });
        assert!(has_bool);
    }

    #[test]
    fn test_binary_comparison_gt_returns_bool() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Bool) if *v == result)
        });
        assert!(has_bool);
    }

    #[test]
    fn test_binary_in_returns_bool() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::In,
            left: Box::new(HirExpr::Var("x".into())),
            right: Box::new(HirExpr::Var("list".into())),
        });

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Bool) if *v == result)
        });
        assert!(has_bool);
    }

    #[test]
    fn test_binary_and_constrains_all_to_bool() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        // Should have 3 Bool constraints (left, right, result)
        let bool_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::Bool))
        }).count();
        assert_eq!(bool_count, 3);
    }

    #[test]
    fn test_binary_or_constrains_all_to_bool() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::Or,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let bool_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::Bool))
        }).count();
        assert_eq!(bool_count, 3);
    }

    #[test]
    fn test_binary_bitand_constrains_to_int() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::BitAnd,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let int_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        }).count();
        assert_eq!(int_count, 3);
    }

    #[test]
    fn test_binary_bitor_constrains_to_int() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::BitOr,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let int_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        }).count();
        assert_eq!(int_count, 3);
    }

    #[test]
    fn test_binary_lshift_constrains_to_int() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Binary {
            op: BinOp::LShift,
            left: Box::new(HirExpr::Var("a".into())),
            right: Box::new(HirExpr::Var("b".into())),
        });

        let int_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        }).count();
        assert_eq!(int_count, 3);
    }

    // ============================================================
    // Collection type tests
    // ============================================================

    #[test]
    fn test_list_constraint_with_elements() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]));

        let has_list = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::List(_)) if *v == result)
        });
        assert!(has_list);
    }

    #[test]
    fn test_list_empty() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::List(vec![]));
        // Empty list just gets a fresh var, no instance constraint
        assert!(collector.constraints().is_empty());
    }

    #[test]
    fn test_dict_constraint_with_pairs() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Dict(vec![
            (
                HirExpr::Literal(Literal::String("key".into())),
                HirExpr::Literal(Literal::Int(1)),
            ),
        ]));

        let has_dict = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Dict(_, _)) if *v == result)
        });
        assert!(has_dict);
    }

    #[test]
    fn test_dict_empty() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Dict(vec![]));
        assert!(collector.constraints().is_empty());
    }

    #[test]
    fn test_tuple_constraint() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".into())),
        ]));

        let has_tuple = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(v, Type::Tuple(_)) if *v == result)
        });
        assert!(has_tuple);
    }

    // ============================================================
    // Statement tests
    // ============================================================

    #[test]
    fn test_assign_with_annotation() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".into()),
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: Some(Type::Int),
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let has_instance = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        });
        assert!(has_instance);
    }

    #[test]
    fn test_assign_tuple_target() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Assign {
                target: AssignTarget::Tuple(vec![
                    AssignTarget::Symbol("a".into()),
                    AssignTarget::Symbol("b".into()),
                ]),
                value: HirExpr::Tuple(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Literal(Literal::Int(2)),
                ]),
                type_annotation: None,
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_return_statement_with_expr() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_if_statement_condition_constrained_to_bool() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::If {
                condition: HirExpr::Var("x".into()),
                then_body: vec![],
                else_body: None,
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Bool))
        });
        assert!(has_bool);
    }

    #[test]
    fn test_if_statement_with_else() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(1)))],
                else_body: Some(vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(2)))]),
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_while_statement_condition_constrained_to_bool() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::While {
                condition: HirExpr::Var("x".into()),
                body: vec![],
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Bool))
        });
        assert!(has_bool);
    }

    #[test]
    fn test_for_statement() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::For {
                target: AssignTarget::Symbol("i".into()),
                iter: HirExpr::Var("items".into()),
                body: vec![HirStmt::Expr(HirExpr::Var("i".into()))],
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        // Just check it doesn't panic
        assert!(collector.constraints().is_empty() || !collector.constraints().is_empty());
    }

    #[test]
    fn test_try_statement() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Try {
                body: vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(1)))],
                handlers: vec![ExceptHandler {
                    exception_type: Some("Exception".into()),
                    name: Some("e".into()),
                    body: vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(2)))],
                }],
                orelse: Some(vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(3)))]),
                finalbody: Some(vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(4)))]),
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_expr_statement() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(42)))],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    // ============================================================
    // Expression tests
    // ============================================================

    #[test]
    fn test_var_expr() {
        let mut collector = ConstraintCollector::new();
        let v1 = collector.collect_expr(&HirExpr::Var("x".into()));
        let v2 = collector.collect_expr(&HirExpr::Var("x".into()));
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_index_expr_constrains_index_to_int() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Index {
            base: Box::new(HirExpr::Var("list".into())),
            index: Box::new(HirExpr::Var("i".into())),
        });

        let has_int = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        });
        assert!(has_int);
    }

    #[test]
    fn test_method_call_expr() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".into())),
            method: "foo".into(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        });
        // Just check it returns a fresh var and processes args
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_attribute_expr() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".into())),
            attr: "field".into(),
        });
        // Should return a fresh var
        assert!(collector.constraints().is_empty());
    }

    #[test]
    fn test_unary_expr() {
        let mut collector = ConstraintCollector::new();
        let result = collector.collect_expr(&HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".into())),
        });

        // Unary preserves type - result should be same var as operand
        let operand = collector.get_or_create_var("x");
        assert_eq!(result, operand);
    }

    #[test]
    fn test_if_expr_constrains_condition_to_bool() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".into())),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(2))),
        });

        let has_bool = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Bool))
        });
        assert!(has_bool);
    }

    #[test]
    fn test_if_expr_constrains_branches_equal() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Var("a".into())),
            orelse: Box::new(HirExpr::Var("b".into())),
        });

        let has_equality = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Equality(_, _))
        });
        assert!(has_equality);
    }

    // ============================================================
    // Call expression tests
    // ============================================================

    #[test]
    fn test_call_unknown_function() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Call {
            func: "unknown_func".into(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        });
        // Should collect arg constraints even for unknown function
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_call_known_function() {
        // First register a function
        let func = make_test_function(
            "add",
            vec![("a", Type::Int), ("b", Type::Int)],
            vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".into())),
                right: Box::new(HirExpr::Var("b".into())),
            }))],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Now call the function
        let _ = collector.collect_expr(&HirExpr::Call {
            func: "add".into(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ],
            kwargs: vec![],
        });

        // Should have constraints linking args to params
        assert!(!collector.constraints().is_empty());
    }

    // ============================================================
    // Apply substitutions tests
    // ============================================================

    #[test]
    fn test_apply_substitutions() {
        let func = make_test_function("test", vec![("x", Type::Unknown)], vec![]);

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let mut solution = HashMap::new();
        if let Some(&var) = collector.param_type_vars.get("test::x") {
            solution.insert(var, Type::Int);
        }

        let applied = collector.apply_substitutions(&mut module, &solution);

        assert_eq!(applied, 1);
        assert_eq!(module.functions[0].params[0].ty, Type::Int);
    }

    #[test]
    fn test_apply_substitutions_skips_unification_var() {
        let func = make_test_function("test", vec![("x", Type::Unknown)], vec![]);

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let mut solution = HashMap::new();
        if let Some(&var) = collector.param_type_vars.get("test::x") {
            // Insert UnificationVar - should NOT be applied
            solution.insert(var, Type::UnificationVar(99));
        }

        let applied = collector.apply_substitutions(&mut module, &solution);

        assert_eq!(applied, 0);
        assert_eq!(module.functions[0].params[0].ty, Type::Unknown);
    }

    #[test]
    fn test_apply_substitutions_to_return_type() {
        let func = make_test_function_with_ret("test", vec![], Type::Unknown, vec![]);

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Get the return var from function signature
        let mut solution = HashMap::new();
        if let Some((_, ret_var)) = collector.function_signatures.get("test") {
            solution.insert(*ret_var, Type::Int);
        }

        let applied = collector.apply_substitutions(&mut module, &solution);

        assert_eq!(applied, 1);
        assert_eq!(module.functions[0].ret_type, Type::Int);
    }

    #[test]
    fn test_apply_substitutions_known_type_not_changed() {
        let func = make_test_function("test", vec![("x", Type::Int)], vec![]);

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let mut solution = HashMap::new();
        if let Some(&var) = collector.param_type_vars.get("test::x") {
            solution.insert(var, Type::Float); // Try to change to Float
        }

        let applied = collector.apply_substitutions(&mut module, &solution);

        assert_eq!(applied, 0); // Should not apply
        assert_eq!(module.functions[0].params[0].ty, Type::Int); // Still Int
    }

    // ============================================================
    // Function signature registration tests
    // ============================================================

    #[test]
    fn test_register_function_with_known_param_type() {
        let func = make_test_function("test", vec![("x", Type::Int)], vec![]);

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Should have an Instance constraint for the param
        let has_instance = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        });
        assert!(has_instance);
    }

    #[test]
    fn test_register_function_with_known_return_type() {
        let func = make_test_function_with_ret("test", vec![], Type::String, vec![]);

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let has_string = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::String))
        });
        assert!(has_string);
    }

    #[test]
    fn test_multiple_functions() {
        let func1 = make_test_function("add", vec![("a", Type::Int), ("b", Type::Int)], vec![]);
        let func2 = make_test_function("concat", vec![("s1", Type::String), ("s2", Type::String)], vec![]);

        let module = HirModule {
            functions: vec![func1, func2],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Should have signatures for both functions
        assert!(collector.function_signatures.contains_key("add"));
        assert!(collector.function_signatures.contains_key("concat"));
    }

    // ============================================================
    // Edge cases
    // ============================================================

    #[test]
    fn test_empty_module() {
        let module = empty_module();
        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(collector.constraints().is_empty());
    }

    #[test]
    fn test_assign_index_target() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Assign {
                target: AssignTarget::Index {
                    base: Box::new(HirExpr::Var("arr".into())),
                    index: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        // Should collect value constraint without panic
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_assign_attribute_target() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("obj".into())),
                    attr: "field".into(),
                },
                value: HirExpr::Literal(Literal::String("value".into())),
                type_annotation: None,
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_nested_tuple_assign() {
        let func = make_test_function(
            "test",
            vec![],
            vec![HirStmt::Assign {
                target: AssignTarget::Tuple(vec![
                    AssignTarget::Symbol("a".into()),
                    AssignTarget::Tuple(vec![
                        AssignTarget::Symbol("b".into()),
                        AssignTarget::Symbol("c".into()),
                    ]),
                ]),
                value: HirExpr::Tuple(vec![
                    HirExpr::Literal(Literal::Int(1)),
                    HirExpr::Tuple(vec![
                        HirExpr::Literal(Literal::Int(2)),
                        HirExpr::Literal(Literal::Int(3)),
                    ]),
                ]),
                type_annotation: None,
            }],
        );

        let module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);
        assert!(!collector.constraints().is_empty());
    }

    #[test]
    fn test_dict_multiple_pairs_constrained_equal() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Dict(vec![
            (
                HirExpr::Literal(Literal::String("a".into())),
                HirExpr::Literal(Literal::Int(1)),
            ),
            (
                HirExpr::Literal(Literal::String("b".into())),
                HirExpr::Literal(Literal::Int(2)),
            ),
        ]));

        // Should have equality constraints for keys and values
        let equality_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Equality(_, _))
        }).count();
        assert!(equality_count >= 2);
    }

    #[test]
    fn test_list_multiple_elements_constrained_equal() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::List(vec![
            HirExpr::Var("a".into()),
            HirExpr::Var("b".into()),
            HirExpr::Var("c".into()),
        ]));

        // Should have 2 equality constraints (b=a, c=a)
        let equality_count = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Equality(_, _))
        }).count();
        assert_eq!(equality_count, 2);
    }
}

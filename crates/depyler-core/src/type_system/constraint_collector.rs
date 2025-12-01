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
    use crate::hir::{FunctionProperties, HirParam};
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
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Should have constraints for x = 42
        assert!(!collector.constraints().is_empty());
    }

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
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Should have equality constraint between a and b
        let has_equality = collector
            .constraints()
            .iter()
            .any(|c| matches!(c, Constraint::Equality(_, _)));
        assert!(has_equality);
    }

    #[test]
    fn test_apply_substitutions() {
        let func = make_test_function("test", vec![("x", Type::Unknown)], vec![]);

        let mut module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Create a solution mapping x's type var to Int
        let mut solution = HashMap::new();
        if let Some(&var) = collector.param_type_vars.get("test::x") {
            solution.insert(var, Type::Int);
        }

        let applied = collector.apply_substitutions(&mut module, &solution);

        assert_eq!(applied, 1);
        assert_eq!(module.functions[0].params[0].ty, Type::Int);
    }
}

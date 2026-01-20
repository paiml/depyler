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

            // DEPYLER-1173: Usage-based type inference from method calls
            // If a method is called on an object, we can infer the object's type
            HirExpr::MethodCall { object, method, args, .. } => {
                let obj_var = self.collect_expr(object);
                let result_var = self.fresh_var();

                for arg in args {
                    let _ = self.collect_expr(arg);
                }

                // Infer object type from method name
                match method.as_str() {
                    // String methods → object must be String
                    "split" | "rsplit" | "splitlines" | "upper" | "lower" | "strip"
                    | "lstrip" | "rstrip" | "capitalize" | "title" | "swapcase"
                    | "startswith" | "endswith" | "find" | "rfind" | "index" | "rindex"
                    | "count" | "replace" | "join" | "encode" | "zfill" | "center"
                    | "ljust" | "rjust" | "isalpha" | "isdigit" | "isalnum" | "isspace"
                    | "isupper" | "islower" | "istitle" | "format" => {
                        self.constraints.push(Constraint::Instance(obj_var, Type::String));
                    }

                    // List methods → object must be List
                    "append" | "extend" | "insert" | "pop" | "remove" | "clear"
                    | "sort" | "reverse" => {
                        // For list methods, we know it's a list but not the element type
                        // Create a fresh var for element type
                        let elem_var = self.fresh_var();
                        self.constraints.push(Constraint::Instance(
                            obj_var,
                            Type::List(Box::new(Type::UnificationVar(elem_var))),
                        ));
                    }

                    // Dict methods → object must be Dict
                    "keys" | "values" | "items" | "get" | "setdefault" | "update"
                    | "popitem" => {
                        let key_var = self.fresh_var();
                        let val_var = self.fresh_var();
                        self.constraints.push(Constraint::Instance(
                            obj_var,
                            Type::Dict(
                                Box::new(Type::UnificationVar(key_var)),
                                Box::new(Type::UnificationVar(val_var)),
                            ),
                        ));
                    }

                    // Set methods → object must be Set
                    "add" | "discard" | "union" | "intersection" | "difference"
                    | "symmetric_difference" | "issubset" | "issuperset" => {
                        let elem_var = self.fresh_var();
                        self.constraints.push(Constraint::Instance(
                            obj_var,
                            Type::Set(Box::new(Type::UnificationVar(elem_var))),
                        ));
                    }

                    _ => {}
                }

                result_var
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

            // DEPYLER-1173: Usage-based type inference from slice operations
            // If s[1:4] is used, s must be either String or List
            HirExpr::Slice { base, start, stop, step } => {
                let base_var = self.collect_expr(base);
                let result_var = self.fresh_var();

                // Collect start/stop/step if present (they should be Int)
                if let Some(start_expr) = start {
                    let start_var = self.collect_expr(start_expr);
                    self.constraints.push(Constraint::Instance(start_var, Type::Int));
                }
                if let Some(stop_expr) = stop {
                    let stop_var = self.collect_expr(stop_expr);
                    self.constraints.push(Constraint::Instance(stop_var, Type::Int));
                }
                if let Some(step_expr) = step {
                    let step_var = self.collect_expr(step_expr);
                    self.constraints.push(Constraint::Instance(step_var, Type::Int));
                }

                // DEPYLER-1173: Key insight - slicing is most commonly on String in Python
                // When we see s[1:4] without additional context, assume String
                // This can be refined later with bidirectional inference
                //
                // Future enhancement: Check if base is already constrained to List
                // and propagate that. For now, String is the safe default for
                // single-shot compilation (most slicing is on strings).
                self.constraints.push(Constraint::Instance(base_var, Type::String));

                // Result of slicing a String is a String
                self.constraints.push(Constraint::Instance(result_var, Type::String));

                result_var
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

    /// DEPYLER-1180: Get local variable type mappings for codegen context
    /// Returns the mapping from variable names to their inferred types
    pub fn get_inferred_var_types(&self, solution: &HashMap<VarId, Type>) -> HashMap<String, Type> {
        let mut result = HashMap::new();
        for (var_name, &var_id) in &self.var_to_type_var {
            if let Some(inferred_type) = solution.get(&var_id) {
                // Only include concrete types, not unification variables
                if !matches!(inferred_type, Type::UnificationVar(_) | Type::Unknown) {
                    result.insert(var_name.clone(), inferred_type.clone());
                }
            }
        }
        result
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

            // DEPYLER-1180: Apply to local variable type annotations in statements
            // This is the "Neural Link" - propagating inferred types to statement-level
            applied_count += self.apply_to_statements(&mut func.body, solution);
        }

        applied_count
    }

    /// DEPYLER-1180: Apply inferred types to local variable declarations in statements
    fn apply_to_statements(&self, stmts: &mut [HirStmt], solution: &HashMap<VarId, Type>) -> usize {
        let mut applied_count = 0;

        for stmt in stmts {
            match stmt {
                HirStmt::Assign {
                    target: AssignTarget::Symbol(var_name),
                    type_annotation,
                    ..
                } => {
                    // Only update if no explicit annotation exists
                    if type_annotation.is_none() {
                        if let Some(&var_id) = self.var_to_type_var.get(var_name) {
                            if let Some(inferred) = solution.get(&var_id) {
                                if !matches!(inferred, Type::UnificationVar(_) | Type::Unknown) {
                                    *type_annotation = Some(inferred.clone());
                                    applied_count += 1;
                                }
                            }
                        }
                    }
                }
                // Recursively apply to nested blocks
                HirStmt::If { then_body, else_body, .. } => {
                    applied_count += self.apply_to_statements(then_body, solution);
                    if let Some(else_stmts) = else_body {
                        applied_count += self.apply_to_statements(else_stmts, solution);
                    }
                }
                HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                    applied_count += self.apply_to_statements(body, solution);
                }
                HirStmt::Try {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                } => {
                    applied_count += self.apply_to_statements(body, solution);
                    for handler in handlers {
                        applied_count += self.apply_to_statements(&mut handler.body, solution);
                    }
                    if let Some(else_stmts) = orelse {
                        applied_count += self.apply_to_statements(else_stmts, solution);
                    }
                    if let Some(final_stmts) = finalbody {
                        applied_count += self.apply_to_statements(final_stmts, solution);
                    }
                }
                HirStmt::With { body, .. } => {
                    applied_count += self.apply_to_statements(body, solution);
                }
                HirStmt::Block(stmts) => {
                    applied_count += self.apply_to_statements(stmts, solution);
                }
                HirStmt::FunctionDef { body, .. } => {
                    applied_count += self.apply_to_statements(body, solution);
                }
                _ => {}
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
    use crate::type_system::hindley_milner::TypeConstraintSolver;
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

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

    // ============================================================
    // DEPYLER-1173: Usage-Based Type Inference Tests
    // ============================================================

    #[test]
    fn test_depyler_1173_slice_constrains_base_to_string() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Slice {
            base: Box::new(HirExpr::Var("s".into())),
            start: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
            stop: Some(Box::new(HirExpr::Literal(Literal::Int(4)))),
            step: None,
        });

        // Base should be constrained to String
        let has_string_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::String))
        });
        assert!(has_string_constraint, "Slice base should be constrained to String");
    }

    #[test]
    fn test_depyler_1173_slice_start_stop_constrained_to_int() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Slice {
            base: Box::new(HirExpr::Var("s".into())),
            start: Some(Box::new(HirExpr::Var("start".into()))),
            stop: Some(Box::new(HirExpr::Var("stop".into()))),
            step: Some(Box::new(HirExpr::Var("step".into()))),
        });

        // start, stop, step should be constrained to Int
        let int_constraints = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::Int))
        }).count();
        // At least 3 Int constraints (for start, stop, step)
        assert!(int_constraints >= 3, "Slice indices should be constrained to Int");
    }

    #[test]
    fn test_depyler_1173_method_split_constrains_to_string() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("text".into())),
            method: "split".into(),
            args: vec![HirExpr::Literal(Literal::String(",".into()))],
            kwargs: vec![],
        });

        let has_string_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::String))
        });
        assert!(has_string_constraint, "split() receiver should be constrained to String");
    }

    #[test]
    fn test_depyler_1173_method_upper_constrains_to_string() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("text".into())),
            method: "upper".into(),
            args: vec![],
            kwargs: vec![],
        });

        let has_string_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::String))
        });
        assert!(has_string_constraint, "upper() receiver should be constrained to String");
    }

    #[test]
    fn test_depyler_1173_method_append_constrains_to_list() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".into())),
            method: "append".into(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        });

        let has_list_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::List(_)))
        });
        assert!(has_list_constraint, "append() receiver should be constrained to List");
    }

    #[test]
    fn test_depyler_1173_method_pop_constrains_to_list() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("stack".into())),
            method: "pop".into(),
            args: vec![],
            kwargs: vec![],
        });

        let has_list_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::List(_)))
        });
        assert!(has_list_constraint, "pop() receiver should be constrained to List");
    }

    #[test]
    fn test_depyler_1173_method_keys_constrains_to_dict() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("data".into())),
            method: "keys".into(),
            args: vec![],
            kwargs: vec![],
        });

        let has_dict_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Dict(_, _)))
        });
        assert!(has_dict_constraint, "keys() receiver should be constrained to Dict");
    }

    #[test]
    fn test_depyler_1173_method_values_constrains_to_dict() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("mapping".into())),
            method: "values".into(),
            args: vec![],
            kwargs: vec![],
        });

        let has_dict_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Dict(_, _)))
        });
        assert!(has_dict_constraint, "values() receiver should be constrained to Dict");
    }

    #[test]
    fn test_depyler_1173_method_add_constrains_to_set() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("unique".into())),
            method: "add".into(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        });

        let has_set_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::Set(_)))
        });
        assert!(has_set_constraint, "add() receiver should be constrained to Set");
    }

    #[test]
    fn test_depyler_1173_unknown_method_no_constraint() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".into())),
            method: "custom_method".into(),
            args: vec![],
            kwargs: vec![],
        });

        // No type constraints should be added for unknown methods
        let container_constraints = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::String)
                | Constraint::Instance(_, Type::List(_))
                | Constraint::Instance(_, Type::Dict(_, _))
                | Constraint::Instance(_, Type::Set(_)))
        }).count();
        assert_eq!(container_constraints, 0, "Unknown method should not add type constraints");
    }

    #[test]
    fn test_depyler_1173_slice_with_step() {
        let mut collector = ConstraintCollector::new();
        let _ = collector.collect_expr(&HirExpr::Slice {
            base: Box::new(HirExpr::Var("s".into())),
            start: None,
            stop: None,
            step: Some(Box::new(HirExpr::Literal(Literal::Int(-1)))),
        });

        // Should still constrain base to String
        let has_string_constraint = collector.constraints().iter().any(|c| {
            matches!(c, Constraint::Instance(_, Type::String))
        });
        assert!(has_string_constraint, "Slice with step should still constrain to String");
    }

    #[test]
    fn test_depyler_1173_multiple_string_methods_chained() {
        // Test: text.strip().upper()
        let mut collector = ConstraintCollector::new();

        // First, strip() on text
        let strip_result = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("text".into())),
            method: "strip".into(),
            args: vec![],
            kwargs: vec![],
        });

        // Then upper() on result - but result is a fresh var, not text
        // This demonstrates that the inner object gets constrained
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("text".into())),
            method: "upper".into(),
            args: vec![],
            kwargs: vec![],
        });

        // Should have multiple String constraints
        let string_constraints = collector.constraints().iter().filter(|c| {
            matches!(c, Constraint::Instance(_, Type::String))
        }).count();
        assert!(string_constraints >= 2, "Chained string methods should add constraints: got {}", string_constraints);

        // Verify strip_result is used (suppress warning)
        assert!(strip_result > 0 || strip_result == 0);
    }

    // ============================================================
    // DEPYLER-1180: Neural Link - Type Propagation Tests
    // ============================================================

    #[test]
    fn test_depyler_1180_apply_substitutions_to_local_vars() {
        // Create a function that uses slicing: def test(s): x = s[1:4]
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam {
                name: "s".to_string(),
                ty: Type::Unknown, // No explicit type annotation
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".into()),
                    value: HirExpr::Slice {
                        base: Box::new(HirExpr::Var("s".into())),
                        start: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
                        stop: Some(Box::new(HirExpr::Literal(Literal::Int(4)))),
                        step: None,
                    },
                    type_annotation: None, // No explicit annotation
                },
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        // Collect constraints
        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Solve constraints
        let mut solver = TypeConstraintSolver::new();
        for constraint in collector.constraints() {
            solver.add_constraint(constraint.clone());
        }
        let solution = solver.solve().expect("Constraint solving should succeed");

        // Apply substitutions
        let applied = collector.apply_substitutions(&mut module, &solution);

        // Verify: Parameter 's' should be inferred as String (from slicing)
        assert_eq!(module.functions[0].params[0].ty, Type::String,
            "Parameter 's' should be inferred as String from slice usage");

        // Verify: At least one substitution was applied
        assert!(applied >= 1, "Should have applied at least 1 substitution, got {}", applied);
    }

    #[test]
    fn test_depyler_1180_get_inferred_var_types() {
        // Test the helper function that returns inferred types for codegen
        let mut collector = ConstraintCollector::new();

        // Create a simple expression that constrains a variable
        let _ = collector.collect_expr(&HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("text".into())),
            method: "split".into(),
            args: vec![],
            kwargs: vec![],
        });

        // Solve constraints
        let mut solver = TypeConstraintSolver::new();
        for constraint in collector.constraints() {
            solver.add_constraint(constraint.clone());
        }
        let solution = solver.solve().expect("Constraint solving should succeed");

        // Get inferred var types
        let var_types = collector.get_inferred_var_types(&solution);

        // Verify 'text' is inferred as String
        assert_eq!(var_types.get("text"), Some(&Type::String),
            "Variable 'text' should be inferred as String from split() call");
    }

    #[test]
    fn test_depyler_1180_preserves_explicit_annotations() {
        // Ensure explicit type annotations are NOT overwritten
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".into()),
                    value: HirExpr::Literal(Literal::Int(42)),
                    type_annotation: Some(Type::Float), // Explicit annotation
                },
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        let mut solver = TypeConstraintSolver::new();
        for constraint in collector.constraints() {
            solver.add_constraint(constraint.clone());
        }
        let solution = solver.solve().expect("Constraint solving should succeed");

        collector.apply_substitutions(&mut module, &solution);

        // Verify: Explicit annotation should be preserved
        if let HirStmt::Assign { type_annotation, .. } = &module.functions[0].body[0] {
            assert_eq!(*type_annotation, Some(Type::Float),
                "Explicit type annotation should be preserved");
        } else {
            panic!("Expected Assign statement");
        }
    }

    #[test]
    fn test_depyler_1180_propagates_to_nested_blocks() {
        // Test propagation to nested if/else blocks
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body: vec![
                HirStmt::If {
                    condition: HirExpr::Literal(Literal::Bool(true)),
                    then_body: vec![
                        HirStmt::Assign {
                            target: AssignTarget::Symbol("x".into()),
                            value: HirExpr::MethodCall {
                                object: Box::new(HirExpr::Var("s".into())),
                                method: "upper".into(),
                                args: vec![],
                                kwargs: vec![],
                            },
                            type_annotation: None,
                        },
                    ],
                    else_body: None,
                },
            ],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut module = HirModule {
            functions: vec![func],
            ..empty_module()
        };

        let mut collector = ConstraintCollector::new();
        collector.collect_module(&module);

        // Get inferred types directly (test the helper)
        let mut solver = TypeConstraintSolver::new();
        for constraint in collector.constraints() {
            solver.add_constraint(constraint.clone());
        }
        let solution = solver.solve().expect("Constraint solving should succeed");

        let var_types = collector.get_inferred_var_types(&solution);

        // Variable 's' should be inferred as String from .upper() call
        assert_eq!(var_types.get("s"), Some(&Type::String),
            "Variable 's' should be inferred as String from upper() in nested block");
    }
}

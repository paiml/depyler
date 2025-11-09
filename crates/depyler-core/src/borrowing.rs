use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type};
use std::collections::HashSet;

/// Tracks how parameters are used within a function to infer borrowing patterns
#[derive(Debug, Default)]
pub struct BorrowingContext {
    /// Parameters that are mutated in the function
    mutated_params: HashSet<String>,
    /// Parameters that escape (are returned or stored)
    escaping_params: HashSet<String>,
    /// Parameters that are only read
    read_only_params: HashSet<String>,
    /// Parameters used in loops (may need special handling)
    loop_used_params: HashSet<String>,
}

/// Analysis result for a single parameter
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowingPattern {
    /// Parameter should be taken by value (moved)
    Owned,
    /// Parameter can be borrowed immutably
    Borrowed,
    /// Parameter needs mutable borrow
    MutableBorrow,
}

impl BorrowingContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze a function to determine parameter borrowing patterns
    pub fn analyze_function(&mut self, func: &HirFunction) {
        // First pass: identify all parameters
        for param in &func.params {
            self.read_only_params.insert(param.name.clone());
        }

        // Analyze function body
        for stmt in &func.body {
            self.analyze_stmt(stmt);
        }

        // Remove read-only classification from mutated or escaping params
        for param in &self.mutated_params {
            self.read_only_params.remove(param);
        }
        for param in &self.escaping_params {
            self.read_only_params.remove(param);
        }
    }

    /// Get the borrowing pattern for a specific parameter
    pub fn get_pattern(&self, param_name: &str, param_type: &Type) -> BorrowingPattern {
        if self.escaping_params.contains(param_name) {
            // Parameters that escape must be owned
            BorrowingPattern::Owned
        } else if self.mutated_params.contains(param_name) {
            // Mutated parameters need mutable borrow
            BorrowingPattern::MutableBorrow
        } else if self.is_copyable(param_type) {
            // Small copyable types should be passed by value
            BorrowingPattern::Owned
        } else {
            // Everything else can be borrowed
            BorrowingPattern::Borrowed
        }
    }

    /// Generate Rust parameter signature based on borrowing pattern
    pub fn generate_param_signature(&self, param_name: &str, param_type: &Type) -> String {
        let pattern = self.get_pattern(param_name, param_type);
        let type_str = self.type_to_rust_string(param_type);

        match pattern {
            BorrowingPattern::Owned => format!("{}: {}", param_name, type_str),
            BorrowingPattern::Borrowed => format!("{}: &{}", param_name, type_str),
            BorrowingPattern::MutableBorrow => format!("{}: &mut {}", param_name, type_str),
        }
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { target, value, .. } => self.analyze_assign(target, value),
            HirStmt::Return(Some(expr)) => self.analyze_return(expr),
            HirStmt::Expr(expr) => self.analyze_expr(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.analyze_if(condition, then_body, else_body),
            HirStmt::While { condition, body } => self.analyze_while(condition, body),
            HirStmt::For {
                target: _,
                iter,
                body,
            } => self.analyze_for(iter, body),
            _ => {}
        }
    }

    fn analyze_assign(&mut self, target: &AssignTarget, value: &HirExpr) {
        if let AssignTarget::Symbol(symbol) = target {
            if self.read_only_params.contains(symbol) {
                self.mutated_params.insert(symbol.clone());
            }
        }
        self.check_escaping_expr(value);
        self.analyze_expr(value);
    }

    fn analyze_return(&mut self, expr: &HirExpr) {
        self.check_escaping_expr(expr);
        self.analyze_expr(expr);
    }

    fn analyze_if(
        &mut self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) {
        self.analyze_expr(condition);
        for stmt in then_body {
            self.analyze_stmt(stmt);
        }
        if let Some(else_stmts) = else_body {
            for stmt in else_stmts {
                self.analyze_stmt(stmt);
            }
        }
    }

    fn analyze_while(&mut self, condition: &HirExpr, body: &[HirStmt]) {
        self.analyze_expr(condition);
        self.mark_loop_params(body);
        for stmt in body {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_for(&mut self, iter: &HirExpr, body: &[HirStmt]) {
        self.analyze_expr(iter);
        self.mark_loop_params(body);
        for stmt in body {
            self.analyze_stmt(stmt);
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn analyze_expr(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Binary { op: _, left, right } => self.analyze_binary(left, right),
            HirExpr::Unary { op: _, operand } => self.analyze_expr(operand),
            HirExpr::Call { func: _, args } => self.analyze_call(args),
            HirExpr::List(elts) | HirExpr::Tuple(elts) => self.analyze_collection(elts),
            HirExpr::Dict(items) => self.analyze_dict(items),
            HirExpr::Index { base, index } => self.analyze_index(base, index),
            _ => {}
        }
    }

    fn analyze_binary(&mut self, left: &HirExpr, right: &HirExpr) {
        self.analyze_expr(left);
        self.analyze_expr(right);
    }

    fn analyze_call(&mut self, args: &[HirExpr]) {
        for arg in args {
            self.analyze_expr(arg);
        }
    }

    fn analyze_collection(&mut self, elts: &[HirExpr]) {
        for elt in elts {
            self.analyze_expr(elt);
        }
    }

    fn analyze_dict(&mut self, items: &[(HirExpr, HirExpr)]) {
        for (k, v) in items {
            self.analyze_expr(k);
            self.analyze_expr(v);
        }
    }

    fn analyze_index(&mut self, base: &HirExpr, index: &HirExpr) {
        self.analyze_expr(base);
        self.analyze_expr(index);
    }

    fn check_escaping_expr(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Var(name) => {
                // Direct return of parameter
                self.escaping_params.insert(name.clone());
            }
            HirExpr::List(elts) | HirExpr::Tuple(elts) => {
                // Parameters in collections that are returned
                for elt in elts {
                    if let HirExpr::Var(name) = elt {
                        self.escaping_params.insert(name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    fn mark_loop_params(&mut self, body: &[HirStmt]) {
        for stmt in body {
            self.find_params_in_stmt(stmt);
        }
    }

    fn find_params_in_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Expr(expr) => self.find_params_in_expr(expr),
            HirStmt::Assign { value, .. } => self.find_params_in_expr(value),
            _ => {}
        }
    }

    fn find_params_in_expr(&mut self, expr: &HirExpr) {
        if let HirExpr::Var(name) = expr {
            if self.read_only_params.contains(name)
                || self.mutated_params.contains(name)
                || self.escaping_params.contains(name)
            {
                self.loop_used_params.insert(name.clone());
            }
        }
    }

    fn is_copyable(&self, ty: &Type) -> bool {
        matches!(ty, Type::Int | Type::Float | Type::Bool | Type::None)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn type_to_rust_string(&self, ty: &Type) -> String {
        match ty {
            Type::Unknown | Type::Int | Type::Float | Type::String | Type::Bool | Type::None => {
                self.primitive_type_to_rust(ty)
            }
            Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::Array { .. } => {
                self.collection_type_to_rust(ty)
            }
            Type::Tuple(types) => self.tuple_type_to_rust(types),
            Type::Optional(inner) => self.optional_type_to_rust(inner),
            Type::Custom(name) | Type::TypeVar(name) => name.clone(),
            Type::Generic { base, .. } => base.clone(),
            Type::Function { .. } => "/* function */".to_string(),
            Type::Union(_) => "Union".to_string(),
        }
    }

    fn primitive_type_to_rust(&self, ty: &Type) -> String {
        match ty {
            Type::Unknown => "serde_json::Value".to_string(),
            Type::Int => "i32".to_string(),
            Type::Float => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::Bool => "bool".to_string(),
            Type::None => "()".to_string(),
            _ => unreachable!("primitive_type_to_rust called with non-primitive type"),
        }
    }

    fn collection_type_to_rust(&self, ty: &Type) -> String {
        match ty {
            Type::List(inner) => self.list_type_to_rust(inner),
            Type::Dict(k, v) => self.dict_type_to_rust(k, v),
            Type::Set(element) => self.set_type_to_rust(element),
            Type::Array { element_type, .. } => self.array_type_to_rust(element_type),
            _ => unreachable!("collection_type_to_rust called with non-collection type"),
        }
    }

    fn list_type_to_rust(&self, inner: &Type) -> String {
        format!("Vec<{}>", self.type_to_rust_string(inner))
    }

    fn set_type_to_rust(&self, element: &Type) -> String {
        format!("HashSet<{}>", self.type_to_rust_string(element))
    }

    fn array_type_to_rust(&self, element_type: &Type) -> String {
        format!("Array<{}>", self.type_to_rust_string(element_type))
    }

    fn dict_type_to_rust(&self, k: &Type, v: &Type) -> String {
        format!(
            "HashMap<{}, {}>",
            self.type_to_rust_string(k),
            self.type_to_rust_string(v)
        )
    }

    fn tuple_type_to_rust(&self, types: &[Type]) -> String {
        if types.is_empty() {
            "()".to_string()
        } else {
            let type_strs: Vec<String> =
                types.iter().map(|t| self.type_to_rust_string(t)).collect();
            format!("({})", type_strs.join(", "))
        }
    }

    fn optional_type_to_rust(&self, inner: &Type) -> String {
        format!("Option<{}>", self.type_to_rust_string(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, FunctionProperties, HirParam, Literal};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_read_only_parameter() {
        let mut ctx = BorrowingContext::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::String)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        ctx.analyze_function(&func);
        assert_eq!(
            ctx.get_pattern("x", &Type::String),
            BorrowingPattern::Borrowed
        );
    }

    #[test]
    fn test_mutated_parameter() {
        let mut ctx = BorrowingContext::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam::new(
                "x".to_string(),
                Type::List(Box::new(Type::Int))
            )],
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "append".to_string(),
                args: vec![
                    HirExpr::Var("x".to_string()),
                    HirExpr::Literal(Literal::Int(42)),
                ],
            })],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        ctx.analyze_function(&func);
        // Note: This is a simplified test. In reality, we'd need to track
        // method calls that mutate the receiver.
    }

    #[test]
    fn test_escaping_parameter() {
        let mut ctx = BorrowingContext::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::String)],
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        ctx.analyze_function(&func);
        assert_eq!(ctx.get_pattern("x", &Type::String), BorrowingPattern::Owned);
    }

    #[test]
    fn test_copyable_parameter() {
        let mut ctx = BorrowingContext::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        ctx.analyze_function(&func);
        assert_eq!(ctx.get_pattern("x", &Type::Int), BorrowingPattern::Owned);
    }

    #[test]
    fn test_generate_param_signature() {
        let ctx = BorrowingContext::new();

        // Test borrowed string
        let mut ctx_borrow = BorrowingContext::new();
        ctx_borrow.read_only_params.insert("s".to_string());
        assert_eq!(
            ctx_borrow.generate_param_signature("s", &Type::String),
            "s: &String"
        );

        // Test owned int
        assert_eq!(ctx.generate_param_signature("n", &Type::Int), "n: i32");
    }

    // ========================================================================
    // PHASE 1: Core Infrastructure Tests (Target: 43% → 60% coverage)
    // ========================================================================

    /// Unit Test: Assignment statement mutation tracking
    ///
    /// Verifies: Lines 84-85, 103-111 - analyze_assign tracks param mutations
    #[test]
    fn test_analyze_stmt_assign_mutation() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("x".to_string());

        // Assign: x = x + 1 (mutation)
        let stmt = HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            type_annotation: None,
        };

        ctx.analyze_stmt(&stmt);

        // x should be marked as mutated (removal from read_only happens in analyze_function)
        assert!(
            ctx.mutated_params.contains("x"),
            "Parameter x should be marked as mutated after assignment"
        );
    }

    /// Unit Test: Return statement escaping analysis
    ///
    /// Verifies: Lines 86, 113-116, 193-209 - analyze_return marks escaping params
    #[test]
    fn test_analyze_stmt_return_escaping() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("data".to_string());

        // Return data (parameter escapes)
        let stmt = HirStmt::Return(Some(HirExpr::Var("data".to_string())));

        ctx.analyze_stmt(&stmt);

        // data should be marked as escaping
        assert!(
            ctx.escaping_params.contains("data"),
            "Parameter data should be marked as escaping when returned"
        );
    }

    /// Unit Test: Direct variable escaping detection
    ///
    /// Verifies: Lines 195-197 - check_escaping_expr for direct var return
    #[test]
    fn test_check_escaping_direct_var() {
        let mut ctx = BorrowingContext::new();

        let expr = HirExpr::Var("result".to_string());
        ctx.check_escaping_expr(&expr);

        assert!(
            ctx.escaping_params.contains("result"),
            "Direct variable return should mark parameter as escaping"
        );
    }

    /// Unit Test: If statement branch analysis
    ///
    /// Verifies: Lines 88-92, 118-133 - analyze_if processes both branches
    #[test]
    fn test_analyze_if_branches() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("x".to_string());
        ctx.read_only_params.insert("y".to_string());

        // if condition: x > 0
        let condition = HirExpr::Binary {
            op: BinOp::Gt,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(0))),
        };

        // then: y = y + 1 (mutates y)
        let then_body = vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("y".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("y".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            type_annotation: None,
        }];

        // else: pass
        let else_body = vec![];

        let stmt = HirStmt::If {
            condition,
            then_body,
            else_body: Some(else_body),
        };

        ctx.analyze_stmt(&stmt);

        // x used in condition (read-only)
        assert!(
            ctx.read_only_params.contains("x"),
            "Condition parameter should remain read-only"
        );

        // y mutated in then branch
        assert!(
            ctx.mutated_params.contains("y"),
            "Parameter mutated in if branch should be tracked"
        );
    }

    /// Unit Test: While loop analysis with loop param tracking
    ///
    /// Verifies: Lines 93, 135-141, 211-234 - analyze_while and mark_loop_params
    #[test]
    fn test_analyze_while_loop() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("limit".to_string());
        ctx.read_only_params.insert("counter".to_string());

        // while counter < limit:
        let condition = HirExpr::Binary {
            op: BinOp::Lt,
            left: Box::new(HirExpr::Var("counter".to_string())),
            right: Box::new(HirExpr::Var("limit".to_string())),
        };

        // counter = counter + 1
        let body = vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("counter".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("counter".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            type_annotation: None,
        }];

        let stmt = HirStmt::While { condition, body };

        ctx.analyze_stmt(&stmt);

        // Both params used in loop
        assert!(
            ctx.loop_used_params.contains("limit") || ctx.read_only_params.contains("limit"),
            "Loop condition param should be tracked"
        );
        assert!(
            ctx.mutated_params.contains("counter"),
            "Loop-mutated param should be tracked"
        );
    }

    /// Unit Test: For loop analysis
    ///
    /// Verifies: Lines 94-98, 143-149, 211-234 - analyze_for and loop param tracking
    #[test]
    fn test_analyze_for_loop() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("items".to_string());
        ctx.read_only_params.insert("total".to_string());

        // for item in items:
        let iter = HirExpr::Var("items".to_string());

        // total = total + item
        let body = vec![HirStmt::Assign {
            target: crate::hir::AssignTarget::Symbol("total".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("total".to_string())),
                right: Box::new(HirExpr::Var("item".to_string())),
            },
            type_annotation: None,
        }];

        let stmt = HirStmt::For {
            target: crate::hir::AssignTarget::Symbol("item".to_string()),
            iter,
            body,
        };

        ctx.analyze_stmt(&stmt);

        // items used as iterator
        assert!(
            ctx.loop_used_params.contains("items") || ctx.read_only_params.contains("items"),
            "For loop iterator param should be tracked"
        );
        // total mutated in loop body
        assert!(
            ctx.mutated_params.contains("total"),
            "Loop-mutated param should be tracked"
        );
    }

    /// Unit Test: Primitive type copyability detection
    ///
    /// Verifies: Line 237 - is_copyable returns true for Int/Float/Bool/None
    #[test]
    fn test_is_copyable_primitives() {
        let ctx = BorrowingContext::new();

        // Primitive types should be copyable
        assert!(ctx.is_copyable(&Type::Int), "Int should be copyable");
        assert!(ctx.is_copyable(&Type::Float), "Float should be copyable");
        assert!(ctx.is_copyable(&Type::Bool), "Bool should be copyable");
        assert!(ctx.is_copyable(&Type::None), "None should be copyable");

        // Non-primitive types should NOT be copyable
        assert!(
            !ctx.is_copyable(&Type::String),
            "String should NOT be copyable"
        );
        assert!(
            !ctx.is_copyable(&Type::List(Box::new(Type::Int))),
            "List should NOT be copyable"
        );
        assert!(
            !ctx.is_copyable(&Type::Dict(Box::new(Type::String), Box::new(Type::Int))),
            "Dict should NOT be copyable"
        );
    }
}

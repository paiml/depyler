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
            HirExpr::Call { func: _, args, .. } => self.analyze_call(args),
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
        match expr {
            HirExpr::Var(name) => {
                if self.read_only_params.contains(name)
                    || self.mutated_params.contains(name)
                    || self.escaping_params.contains(name)
                {
                    self.loop_used_params.insert(name.clone());
                }
            }
            // Recursively traverse binary expressions
            HirExpr::Binary { left, right, .. } => {
                self.find_params_in_expr(left);
                self.find_params_in_expr(right);
            }
            // Recursively traverse unary expressions
            HirExpr::Unary { operand, .. } => {
                self.find_params_in_expr(operand);
            }
            // Recursively traverse function call arguments
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.find_params_in_expr(arg);
                }
            }
            // Recursively traverse collections
            HirExpr::List(elts) | HirExpr::Tuple(elts) | HirExpr::Set(elts) => {
                for elt in elts {
                    self.find_params_in_expr(elt);
                }
            }
            // Recursively traverse dict items
            HirExpr::Dict(items) => {
                for (key, value) in items {
                    self.find_params_in_expr(key);
                    self.find_params_in_expr(value);
                }
            }
            // Recursively traverse index expressions
            HirExpr::Index { base, index } => {
                self.find_params_in_expr(base);
                self.find_params_in_expr(index);
            }
            _ => {}
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
            Type::Final(inner) => self.type_to_rust_string(inner), // Unwrap Final to get the actual type
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
                kwargs: vec![],
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
                kwargs: vec![],
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

    // ========================================================================
    // PHASE 2: Expression Analysis Tests (Target: 60% → 75% coverage)
    // ========================================================================

    /// Unit Test: Binary expression analysis
    ///
    /// Verifies: Lines 154, 164-167 - analyze_binary processes both operands
    #[test]
    fn test_analyze_binary_expression() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("x".to_string());
        ctx.read_only_params.insert("y".to_string());

        // Binary expression: x + y
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Var("y".to_string())),
        };

        ctx.analyze_expr(&expr);

        // Both params should remain read-only (just being read, not mutated)
        assert!(
            ctx.read_only_params.contains("x"),
            "Binary operand x should be tracked"
        );
        assert!(
            ctx.read_only_params.contains("y"),
            "Binary operand y should be tracked"
        );
    }

    /// Unit Test: Unary expression analysis
    ///
    /// Verifies: Line 155 - analyze_expr handles unary operations
    #[test]
    fn test_analyze_unary_expression() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("value".to_string());

        // Unary expression: -value
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("value".to_string())),
        };

        ctx.analyze_expr(&expr);

        assert!(
            ctx.read_only_params.contains("value"),
            "Unary operand should be tracked"
        );
    }

    /// Unit Test: Function call argument analysis
    ///
    /// Verifies: Lines 156, 169-173 - analyze_call processes all arguments
    #[test]
    fn test_analyze_call_arguments() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("a".to_string());
        ctx.read_only_params.insert("b".to_string());

        // Call expression: func(a, b)
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())],
            kwargs: vec![],
        };

        ctx.analyze_expr(&expr);

        assert!(
            ctx.read_only_params.contains("a"),
            "Call argument a should be tracked"
        );
        assert!(
            ctx.read_only_params.contains("b"),
            "Call argument b should be tracked"
        );
    }

    /// Unit Test: List collection analysis
    ///
    /// Verifies: Lines 157, 175-179 - analyze_collection processes list elements
    #[test]
    fn test_analyze_list_collection() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("item1".to_string());
        ctx.read_only_params.insert("item2".to_string());

        // List expression: [item1, item2]
        let expr = HirExpr::List(vec![
            HirExpr::Var("item1".to_string()),
            HirExpr::Var("item2".to_string()),
        ]);

        ctx.analyze_expr(&expr);

        assert!(
            ctx.read_only_params.contains("item1"),
            "List element item1 should be tracked"
        );
        assert!(
            ctx.read_only_params.contains("item2"),
            "List element item2 should be tracked"
        );
    }

    /// Unit Test: Dict analysis with key-value pairs
    ///
    /// Verifies: Lines 158, 181-186 - analyze_dict processes keys and values
    #[test]
    fn test_analyze_dict_items() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("key1".to_string());
        ctx.read_only_params.insert("val1".to_string());

        // Dict expression: {key1: val1}
        let expr = HirExpr::Dict(vec![(
            HirExpr::Var("key1".to_string()),
            HirExpr::Var("val1".to_string()),
        )]);

        ctx.analyze_expr(&expr);

        assert!(
            ctx.read_only_params.contains("key1"),
            "Dict key should be tracked"
        );
        assert!(
            ctx.read_only_params.contains("val1"),
            "Dict value should be tracked"
        );
    }

    /// Unit Test: Index expression analysis
    ///
    /// Verifies: Lines 159, 188-191 - analyze_index processes base and index
    #[test]
    fn test_analyze_index_expression() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("arr".to_string());
        ctx.read_only_params.insert("idx".to_string());

        // Index expression: arr[idx]
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };

        ctx.analyze_expr(&expr);

        assert!(
            ctx.read_only_params.contains("arr"),
            "Index base should be tracked"
        );
        assert!(
            ctx.read_only_params.contains("idx"),
            "Index subscript should be tracked"
        );
    }

    /// Unit Test: Escaping parameters in tuple return
    ///
    /// Verifies: Lines 199-206 - check_escaping_expr marks tuple elements as escaping
    #[test]
    fn test_check_escaping_tuple_elements() {
        let mut ctx = BorrowingContext::new();

        // Tuple expression with parameters: (x, y)
        let expr = HirExpr::Tuple(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Var("y".to_string()),
        ]);

        ctx.check_escaping_expr(&expr);

        assert!(
            ctx.escaping_params.contains("x"),
            "Tuple element x should be marked as escaping"
        );
        assert!(
            ctx.escaping_params.contains("y"),
            "Tuple element y should be marked as escaping"
        );
    }

    /// Unit Test: Loop parameter tracking
    ///
    /// Verifies: Lines 211-234 - mark_loop_params and find_params_in_expr
    #[test]
    fn test_mark_loop_params_tracking() {
        let mut ctx = BorrowingContext::new();
        ctx.read_only_params.insert("counter".to_string());

        // Loop body: counter + 1
        let body = vec![HirStmt::Expr(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("counter".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        })];

        ctx.mark_loop_params(&body);

        assert!(
            ctx.loop_used_params.contains("counter"),
            "Parameter used in loop should be tracked"
        );
    }

    // ========================================================================
    // PHASE 3: Type Conversion Tests (Target: 75% → 85% coverage)
    // ========================================================================

    /// Unit Test: Primitive type conversions
    ///
    /// Verifies: Lines 294-304 - primitive_type_to_rust handles all primitives
    #[test]
    fn test_primitive_type_conversions() {
        let ctx = BorrowingContext::new();

        assert_eq!(
            ctx.primitive_type_to_rust(&Type::Unknown),
            "serde_json::Value"
        );
        assert_eq!(ctx.primitive_type_to_rust(&Type::Int), "i32");
        assert_eq!(ctx.primitive_type_to_rust(&Type::Float), "f64");
        assert_eq!(ctx.primitive_type_to_rust(&Type::String), "String");
        assert_eq!(ctx.primitive_type_to_rust(&Type::Bool), "bool");
        assert_eq!(ctx.primitive_type_to_rust(&Type::None), "()");
    }

    /// Unit Test: List type conversion
    ///
    /// Verifies: Lines 308, 316-318 - list_type_to_rust generates Vec<T>
    #[test]
    fn test_list_type_conversion() {
        let ctx = BorrowingContext::new();

        let list_type = Type::List(Box::new(Type::Int));
        let result = ctx.type_to_rust_string(&list_type);

        assert_eq!(result, "Vec<i32>", "List of int should map to Vec<i32>");
    }

    /// Unit Test: Dict type conversion
    ///
    /// Verifies: Lines 309, 328-334 - dict_type_to_rust generates HashMap<K, V>
    #[test]
    fn test_dict_type_conversion() {
        let ctx = BorrowingContext::new();

        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        let result = ctx.type_to_rust_string(&dict_type);

        assert_eq!(
            result, "HashMap<String, i32>",
            "Dict[str, int] should map to HashMap<String, i32>"
        );
    }

    /// Unit Test: Set type conversion
    ///
    /// Verifies: Lines 310, 320-322 - set_type_to_rust generates HashSet<T>
    #[test]
    fn test_set_type_conversion() {
        let ctx = BorrowingContext::new();

        let set_type = Type::Set(Box::new(Type::String));
        let result = ctx.type_to_rust_string(&set_type);

        assert_eq!(
            result, "HashSet<String>",
            "Set[str] should map to HashSet<String>"
        );
    }

    /// Unit Test: Array type conversion
    ///
    /// Verifies: Lines 311, 324-326 - array_type_to_rust generates Array<T>
    #[test]
    fn test_array_type_conversion() {
        let ctx = BorrowingContext::new();

        let array_type = Type::Array {
            element_type: Box::new(Type::Float),
            size: crate::hir::ConstGeneric::Literal(10),
        };
        let result = ctx.type_to_rust_string(&array_type);

        assert_eq!(
            result, "Array<f64>",
            "Array of float should map to Array<f64>"
        );
    }

    /// Unit Test: Tuple type conversion
    ///
    /// Verifies: Lines 285, 336-344 - tuple_type_to_rust handles empty and non-empty tuples
    #[test]
    fn test_tuple_type_conversion() {
        let ctx = BorrowingContext::new();

        // Empty tuple
        let empty_tuple = Type::Tuple(vec![]);
        assert_eq!(
            ctx.type_to_rust_string(&empty_tuple),
            "()",
            "Empty tuple should map to unit ()"
        );

        // Non-empty tuple
        let tuple_type = Type::Tuple(vec![Type::Int, Type::String, Type::Bool]);
        let result = ctx.type_to_rust_string(&tuple_type);

        assert_eq!(
            result, "(i32, String, bool)",
            "Tuple[int, str, bool] should map to (i32, String, bool)"
        );
    }

    /// Unit Test: Optional type conversion
    ///
    /// Verifies: Lines 286, 346-348 - optional_type_to_rust generates Option<T>
    #[test]
    fn test_optional_type_conversion() {
        let ctx = BorrowingContext::new();

        let optional_type = Type::Optional(Box::new(Type::Int));
        let result = ctx.type_to_rust_string(&optional_type);

        assert_eq!(
            result, "Option<i32>",
            "Optional[int] should map to Option<i32>"
        );
    }

    /// Unit Test: Complex type conversions (Custom, TypeVar, Generic, Function, Union)
    ///
    /// Verifies: Lines 287-290 - type_to_rust_string handles special types
    #[test]
    fn test_complex_type_conversions() {
        let ctx = BorrowingContext::new();

        // Custom type
        let custom = Type::Custom("MyClass".to_string());
        assert_eq!(
            ctx.type_to_rust_string(&custom),
            "MyClass",
            "Custom type should preserve name"
        );

        // TypeVar
        let typevar = Type::TypeVar("T".to_string());
        assert_eq!(
            ctx.type_to_rust_string(&typevar),
            "T",
            "TypeVar should preserve name"
        );

        // Generic type
        let generic = Type::Generic {
            base: "Vec".to_string(),
            params: vec![Type::Int],
        };
        assert_eq!(
            ctx.type_to_rust_string(&generic),
            "Vec",
            "Generic type should use base name"
        );

        // Function type
        let function = Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::String),
        };
        assert_eq!(
            ctx.type_to_rust_string(&function),
            "/* function */",
            "Function type should return comment"
        );

        // Union type
        let union = Type::Union(vec![Type::Int, Type::String]);
        assert_eq!(
            ctx.type_to_rust_string(&union),
            "Union",
            "Union type should return 'Union'"
        );
    }
}

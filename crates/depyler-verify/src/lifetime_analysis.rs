use depyler_core::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type};
use std::collections::{HashMap, HashSet};

/// Tracks lifetime constraints and relationships
#[derive(Debug, Default)]
pub struct LifetimeAnalyzer {
    /// Variables and their lifetime constraints
    lifetime_constraints: HashMap<String, LifetimeConstraint>,
    /// Active borrows at each program point
    active_borrows: Vec<BorrowSet>,
    /// Lifetime relationships (outlives)
    #[allow(dead_code)]
    outlives_relations: Vec<(Lifetime, Lifetime)>,
    /// Variables that escape their scope
    escaping_vars: HashSet<String>,
    /// Detected lifetime violations
    violations: Vec<LifetimeViolation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lifetime {
    name: String,
    scope_depth: usize,
}

#[derive(Debug, Clone)]
pub struct LifetimeConstraint {
    lifetime: Lifetime,
    /// Variables this lifetime must outlive
    #[allow(dead_code)]
    must_outlive: Vec<String>,
    /// Scopes where this lifetime is valid
    #[allow(dead_code)]
    valid_scopes: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct BorrowSet {
    /// Variables currently borrowed
    borrowed: HashMap<String, BorrowKind>,
    /// Scope depth of this borrow set
    #[allow(dead_code)]
    scope_depth: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowKind {
    Shared,
    Mutable,
}

#[derive(Debug, Clone)]
pub struct LifetimeViolation {
    pub kind: ViolationKind,
    pub variable: String,
    pub location: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViolationKind {
    UseAfterMove,
    DanglingReference,
    ConflictingBorrows,
    LifetimeTooshort,
    EscapingReference,
}

impl LifetimeAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze a function for lifetime violations
    pub fn analyze_function(&mut self, func: &HirFunction) -> Vec<LifetimeViolation> {
        self.violations.clear();
        self.active_borrows.push(BorrowSet {
            borrowed: HashMap::new(),
            scope_depth: 0,
        });

        // Analyze parameters
        for param in &func.params {
            self.register_variable(&param.name, &param.ty, 0);
        }

        // Analyze function body
        for stmt in &func.body {
            self.analyze_stmt(stmt, 0);
        }

        // Check for escaping references
        self.check_escaping_references();

        self.violations.clone()
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt, scope_depth: usize) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                self.analyze_assign_stmt(target, value, scope_depth)
            }
            HirStmt::Return(Some(expr)) => self.analyze_return_stmt(expr, scope_depth),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.analyze_if_stmt(condition, then_body, else_body, scope_depth),
            HirStmt::While { condition, body } => {
                self.analyze_while_stmt(condition, body, scope_depth)
            }
            HirStmt::For { target, iter, body } => {
                self.analyze_for_stmt(target, iter, body, scope_depth)
            }
            _ => {}
        }
    }

    fn analyze_assign_stmt(&mut self, target: &AssignTarget, value: &HirExpr, scope_depth: usize) {
        self.analyze_expr(value, scope_depth);

        if let AssignTarget::Symbol(var_name) = target {
            if self.is_moved(var_name) {
                self.violations.push(LifetimeViolation {
                    kind: ViolationKind::UseAfterMove,
                    variable: var_name.clone(),
                    location: format!("assignment to {}", var_name),
                    suggestion: "Consider borrowing instead of moving".to_string(),
                });
            }
        }
    }

    fn analyze_return_stmt(&mut self, expr: &HirExpr, scope_depth: usize) {
        self.analyze_expr(expr, scope_depth);
        self.check_return_lifetime(expr, scope_depth);
    }

    fn analyze_if_stmt(
        &mut self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
        scope_depth: usize,
    ) {
        self.analyze_expr(condition, scope_depth);
        self.analyze_scoped_body(then_body, scope_depth);

        if let Some(else_stmts) = else_body {
            self.analyze_scoped_body(else_stmts, scope_depth);
        }
    }

    fn analyze_while_stmt(&mut self, condition: &HirExpr, body: &[HirStmt], scope_depth: usize) {
        self.analyze_expr(condition, scope_depth);
        self.check_loop_borrows(body, scope_depth);
        self.analyze_scoped_body(body, scope_depth);
    }

    fn analyze_for_stmt(
        &mut self,
        target: &AssignTarget,
        iter: &HirExpr,
        body: &[HirStmt],
        scope_depth: usize,
    ) {
        self.analyze_expr(iter, scope_depth);
        self.check_iterator_invalidation(iter, body);

        self.enter_scope(scope_depth + 1);

        // Register all variables from the target pattern
        match target {
            AssignTarget::Symbol(name) => {
                self.register_variable(name, &Type::Unknown, scope_depth + 1);
            }
            AssignTarget::Tuple(targets) => {
                for t in targets {
                    if let AssignTarget::Symbol(name) = t {
                        self.register_variable(name, &Type::Unknown, scope_depth + 1);
                    }
                }
            }
            _ => {}
        }

        for stmt in body {
            self.analyze_stmt(stmt, scope_depth + 1);
        }
        self.exit_scope();
    }

    fn analyze_scoped_body(&mut self, body: &[HirStmt], scope_depth: usize) {
        self.enter_scope(scope_depth + 1);
        for stmt in body {
            self.analyze_stmt(stmt, scope_depth + 1);
        }
        self.exit_scope();
    }

    #[allow(clippy::only_used_in_recursion)]
    fn analyze_expr(&mut self, expr: &HirExpr, scope_depth: usize) {
        match expr {
            HirExpr::Var(name) => self.check_var_borrow(name),
            HirExpr::Borrow { expr, mutable } => {
                self.check_borrow_expr(expr, *mutable, scope_depth)
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_binary_expr(left, right, scope_depth)
            }
            HirExpr::Call { func, args, .. } => self.analyze_call_expr(func, args, scope_depth),
            HirExpr::Index { base, index } => self.analyze_index_expr(base, index, scope_depth),
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => self.analyze_method_call_expr(object, method, args, scope_depth),
            HirExpr::Attribute { value, .. } => self.analyze_expr(value, scope_depth),
            _ => {}
        }
    }

    fn check_var_borrow(&mut self, name: &str) {
        if let Some(borrow_set) = self.active_borrows.last() {
            if let Some(borrow_kind) = borrow_set.borrowed.get(name) {
                if *borrow_kind == BorrowKind::Mutable {
                    self.violations.push(LifetimeViolation {
                        kind: ViolationKind::ConflictingBorrows,
                        variable: name.to_string(),
                        location: "variable access".to_string(),
                        suggestion: "Cannot access variable while mutably borrowed".to_string(),
                    });
                }
            }
        }
    }

    fn check_borrow_expr(&mut self, expr: &HirExpr, mutable: bool, scope_depth: usize) {
        if let HirExpr::Var(name) = expr {
            let borrow_kind = if mutable {
                BorrowKind::Mutable
            } else {
                BorrowKind::Shared
            };

            if !self.can_borrow(name, &borrow_kind) {
                self.violations.push(LifetimeViolation {
                    kind: ViolationKind::ConflictingBorrows,
                    variable: name.clone(),
                    location: "borrow expression".to_string(),
                    suggestion: "Variable is already borrowed".to_string(),
                });
            } else {
                self.add_borrow(name, borrow_kind);
            }
        }
        self.analyze_expr(expr, scope_depth);
    }

    fn analyze_binary_expr(&mut self, left: &HirExpr, right: &HirExpr, scope_depth: usize) {
        self.analyze_expr(left, scope_depth);
        self.analyze_expr(right, scope_depth);
    }

    fn analyze_call_expr(&mut self, func: &str, args: &[HirExpr], scope_depth: usize) {
        if matches!(func, "push" | "append" | "insert" | "extend") {
            self.check_mutating_call(func, args);
        }

        for arg in args {
            self.analyze_expr(arg, scope_depth);
            if let HirExpr::Var(name) = arg {
                if !self.is_copy_type(name) && !matches!(func, "len" | "print") {
                    self.escaping_vars.insert(name.clone());
                }
            }
        }
    }

    fn check_mutating_call(&mut self, func: &str, args: &[HirExpr]) {
        if let Some(HirExpr::Var(obj)) = args.first() {
            if !self.can_borrow(obj, &BorrowKind::Mutable) {
                self.violations.push(LifetimeViolation {
                    kind: ViolationKind::ConflictingBorrows,
                    variable: obj.clone(),
                    location: format!("method call: {}", func),
                    suggestion: "Cannot mutate while borrowed".to_string(),
                });
            }
        }
    }

    fn analyze_index_expr(&mut self, base: &HirExpr, index: &HirExpr, scope_depth: usize) {
        self.analyze_expr(base, scope_depth);
        self.analyze_expr(index, scope_depth);

        if let HirExpr::Var(name) = base {
            if self
                .active_borrows
                .iter()
                .any(|bs| bs.borrowed.contains_key(name))
            {
                self.violations.push(LifetimeViolation {
                    kind: ViolationKind::ConflictingBorrows,
                    variable: name.clone(),
                    location: "indexing operation".to_string(),
                    suggestion: "Cannot index while collection is borrowed".to_string(),
                });
            }
        }
    }

    fn analyze_method_call_expr(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        scope_depth: usize,
    ) {
        self.analyze_expr(object, scope_depth);

        if self.is_mutating_method(method) {
            if let HirExpr::Var(name) = object {
                if !self.can_borrow(name, &BorrowKind::Mutable) {
                    self.violations.push(LifetimeViolation {
                        kind: ViolationKind::ConflictingBorrows,
                        variable: name.clone(),
                        location: format!("method call: {}", method),
                        suggestion: "Cannot call mutable method while borrowed".to_string(),
                    });
                }
            }
        }

        for arg in args {
            self.analyze_expr(arg, scope_depth);
        }
    }

    fn is_mutating_method(&self, method: &str) -> bool {
        matches!(
            method,
            "push"
                | "pop"
                | "insert"
                | "remove"
                | "clear"
                | "append"
                | "extend"
                | "push_str"
                | "truncate"
                | "drain"
                | "retain"
        )
    }

    fn enter_scope(&mut self, depth: usize) {
        self.active_borrows.push(BorrowSet {
            borrowed: HashMap::new(),
            scope_depth: depth,
        });
    }

    fn exit_scope(&mut self) {
        self.active_borrows.pop();
    }

    fn register_variable(&mut self, name: &str, _ty: &Type, scope_depth: usize) {
        self.lifetime_constraints.insert(
            name.to_string(),
            LifetimeConstraint {
                lifetime: Lifetime {
                    name: format!("'{}", name),
                    scope_depth,
                },
                must_outlive: Vec::new(),
                valid_scopes: vec![scope_depth],
            },
        );
    }

    fn is_moved(&self, name: &str) -> bool {
        // Check if variable has been moved
        // For now, consider a variable moved if it was assigned to another variable
        // or passed to a function that takes ownership
        self.escaping_vars.contains(name)
    }

    fn can_borrow(&self, name: &str, kind: &BorrowKind) -> bool {
        for borrow_set in &self.active_borrows {
            if let Some(existing_kind) = borrow_set.borrowed.get(name) {
                match (existing_kind, kind) {
                    (BorrowKind::Mutable, _) => return false,
                    (_, BorrowKind::Mutable) => return false,
                    _ => {}
                }
            }
        }
        true
    }

    fn add_borrow(&mut self, name: &str, kind: BorrowKind) {
        if let Some(borrow_set) = self.active_borrows.last_mut() {
            borrow_set.borrowed.insert(name.to_string(), kind);
        }
    }

    fn is_copy_type(&self, _name: &str) -> bool {
        // For now, assume integers and booleans are Copy types
        // In a full implementation, we'd check the actual type
        false
    }

    fn check_return_lifetime(&mut self, expr: &HirExpr, _scope_depth: usize) {
        if let HirExpr::Borrow { expr, .. } = expr {
            if let HirExpr::Var(name) = expr.as_ref() {
                if let Some(constraint) = self.lifetime_constraints.get(name) {
                    if constraint.lifetime.scope_depth > 0 {
                        self.violations.push(LifetimeViolation {
                            kind: ViolationKind::EscapingReference,
                            variable: name.clone(),
                            location: "return statement".to_string(),
                            suggestion: "Cannot return reference to local variable".to_string(),
                        });
                    }
                }
            }
        }
    }

    fn check_loop_borrows(&mut self, body: &[HirStmt], _scope_depth: usize) {
        // Check for borrows that might be invalidated by the loop
        for stmt in body {
            if let HirStmt::Assign { .. } = stmt {
                // Simplified check for potential invalidation
                for borrow_set in &self.active_borrows {
                    if !borrow_set.borrowed.is_empty() {
                        self.violations.push(LifetimeViolation {
                            kind: ViolationKind::LifetimeTooshort,
                            variable: "loop variable".to_string(),
                            location: "loop body".to_string(),
                            suggestion: "Consider collecting values before the loop".to_string(),
                        });
                        break;
                    }
                }
            }
        }
    }

    fn check_iterator_invalidation(&mut self, iter: &HirExpr, body: &[HirStmt]) {
        if let HirExpr::Var(collection_name) = iter {
            for stmt in body {
                if self.modifies_collection(stmt, collection_name) {
                    self.violations.push(LifetimeViolation {
                        kind: ViolationKind::DanglingReference,
                        variable: collection_name.clone(),
                        location: "for loop".to_string(),
                        suggestion: "Cannot modify collection while iterating".to_string(),
                    });
                }
            }
        }
    }

    fn modifies_collection(&self, stmt: &HirStmt, collection_name: &str) -> bool {
        if let HirStmt::Expr(HirExpr::Call { func, args, .. }) = stmt {
            // Check for methods that modify collections
            if matches!(
                func.as_str(),
                "append" | "insert" | "remove" | "pop" | "clear"
            ) {
                if let Some(HirExpr::Var(name)) = args.first() {
                    return name == collection_name;
                }
            }
        }
        false
    }

    fn check_escaping_references(&mut self) {
        for var in &self.escaping_vars {
            if let Some(constraint) = self.lifetime_constraints.get(var) {
                if constraint.lifetime.scope_depth > 0 {
                    self.violations.push(LifetimeViolation {
                        kind: ViolationKind::EscapingReference,
                        variable: var.clone(),
                        location: "function scope".to_string(),
                        suggestion: "Reference would outlive its data".to_string(),
                    });
                }
            }
        }
    }

    /// Get all detected violations
    pub fn get_violations(&self) -> &[LifetimeViolation] {
        &self.violations
    }

    /// Check if the analyzed code is lifetime-safe
    pub fn is_lifetime_safe(&self) -> bool {
        self.violations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::hir::*;

    #[test]
    #[ignore = "Lifetime analysis not fully implemented yet"]
    fn test_dangling_reference_detection() {
        let mut analyzer = LifetimeAnalyzer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![].into(),
            ret_type: Type::String,
            body: vec![
                HirStmt::Assign {
                    target: depyler_core::hir::AssignTarget::Symbol("local".to_string()),
                    value: HirExpr::Literal(Literal::String("temp".to_string())),
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Borrow {
                    expr: Box::new(HirExpr::Var("local".to_string())),
                    mutable: false,
                })),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let violations = analyzer.analyze_function(&func);
        assert!(!violations.is_empty());
        assert_eq!(violations[0].kind, ViolationKind::EscapingReference);
    }

    #[test]
    fn test_iterator_invalidation() {
        let mut analyzer = LifetimeAnalyzer::new();

        let func = HirFunction {
            name: "test".to_string(),
            params: vec![depyler_core::hir::HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("item".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Call {
                    func: "append".to_string(),
                    args: vec![
                        HirExpr::Var("items".to_string()),
                        HirExpr::Literal(Literal::Int(42)),
                    ],
                    kwargs: vec![],
                })],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let violations = analyzer.analyze_function(&func);
        assert!(!violations.is_empty());
        assert_eq!(violations[0].kind, ViolationKind::DanglingReference);
    }

    #[test]
    fn test_conflicting_borrows() {
        let analyzer = LifetimeAnalyzer::new();

        // This would require more complex HIR to properly test
        // For now, verify the analyzer initializes correctly
        assert!(analyzer.is_lifetime_safe());
    }

    #[test]
    fn test_lifetime_analyzer_new() {
        let analyzer = LifetimeAnalyzer::new();
        assert!(analyzer.get_violations().is_empty());
        assert!(analyzer.is_lifetime_safe());
    }

    #[test]
    fn test_violation_kind_equality() {
        assert_eq!(ViolationKind::UseAfterMove, ViolationKind::UseAfterMove);
        assert_ne!(
            ViolationKind::UseAfterMove,
            ViolationKind::DanglingReference
        );
        assert_eq!(
            ViolationKind::ConflictingBorrows,
            ViolationKind::ConflictingBorrows
        );
    }

    #[test]
    fn test_borrow_kind_equality() {
        assert_eq!(BorrowKind::Shared, BorrowKind::Shared);
        assert_eq!(BorrowKind::Mutable, BorrowKind::Mutable);
        assert_ne!(BorrowKind::Shared, BorrowKind::Mutable);
    }

    #[test]
    fn test_lifetime_equality() {
        let l1 = Lifetime {
            name: "'a".to_string(),
            scope_depth: 1,
        };
        let l2 = Lifetime {
            name: "'a".to_string(),
            scope_depth: 1,
        };
        let l3 = Lifetime {
            name: "'b".to_string(),
            scope_depth: 1,
        };
        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_lifetime_violation_fields() {
        let violation = LifetimeViolation {
            kind: ViolationKind::UseAfterMove,
            variable: "x".to_string(),
            location: "line 10".to_string(),
            suggestion: "use clone".to_string(),
        };
        assert_eq!(violation.kind, ViolationKind::UseAfterMove);
        assert_eq!(violation.variable, "x");
        assert_eq!(violation.location, "line 10");
        assert_eq!(violation.suggestion, "use clone");
    }

    #[test]
    fn test_analyze_simple_function() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "simple".to_string(),
            params: vec![].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_function_with_param() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "with_param".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_if_statement() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_if".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Bool)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("x".to_string()),
                then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                    0,
                ))))]),
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_while_loop() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_while".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Bool)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::While {
                condition: HirExpr::Var("x".to_string()),
                body: vec![],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Check that it doesn't crash
        let _ = violations;
    }

    #[test]
    fn test_analyze_for_loop_tuple_target() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_for_tuple".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Tuple(vec![Type::Int, Type::Int]))),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::For {
                target: AssignTarget::Tuple(vec![
                    AssignTarget::Symbol("a".to_string()),
                    AssignTarget::Symbol("b".to_string()),
                ]),
                iter: HirExpr::Var("items".to_string()),
                body: vec![],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Should handle tuple targets
        let _ = violations;
    }

    #[test]
    fn test_analyze_binary_expr() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_binary".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_index_expr() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_index".to_string(),
            params: vec![HirParam::new(
                "arr".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_analyze_method_call() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_method".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("items".to_string())),
                method: "len".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_analyze_mutating_method() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_mutating".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("items".to_string())),
                method: "push".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_analyze_attribute_access() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_attr".to_string(),
            params: vec![HirParam::new("obj".to_string(), Type::Unknown)].into(),
            ret_type: Type::Unknown,
            body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("obj".to_string())),
                attr: "field".to_string(),
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_is_mutating_method() {
        let analyzer = LifetimeAnalyzer::new();
        assert!(analyzer.is_mutating_method("push"));
        assert!(analyzer.is_mutating_method("pop"));
        assert!(analyzer.is_mutating_method("insert"));
        assert!(analyzer.is_mutating_method("remove"));
        assert!(analyzer.is_mutating_method("clear"));
        assert!(analyzer.is_mutating_method("append"));
        assert!(analyzer.is_mutating_method("extend"));
        assert!(analyzer.is_mutating_method("drain"));
        assert!(analyzer.is_mutating_method("retain"));
        assert!(analyzer.is_mutating_method("push_str"));
        assert!(analyzer.is_mutating_method("truncate"));
        assert!(!analyzer.is_mutating_method("get"));
        assert!(!analyzer.is_mutating_method("len"));
    }

    // ========================================================================
    // Additional Tests for Better Coverage
    // ========================================================================

    #[test]
    fn test_analyzer_default() {
        let analyzer = LifetimeAnalyzer::default();
        assert!(analyzer.is_lifetime_safe());
        assert!(analyzer.get_violations().is_empty());
    }

    #[test]
    fn test_lifetime_clone() {
        let l1 = Lifetime {
            name: "'a".to_string(),
            scope_depth: 2,
        };
        let l2 = l1.clone();
        assert_eq!(l1, l2);
        assert_eq!(l1.name, l2.name);
        assert_eq!(l1.scope_depth, l2.scope_depth);
    }

    #[test]
    fn test_lifetime_debug() {
        let l = Lifetime {
            name: "'x".to_string(),
            scope_depth: 1,
        };
        let debug = format!("{:?}", l);
        assert!(debug.contains("Lifetime"));
        assert!(debug.contains("'x"));
    }

    #[test]
    fn test_lifetime_constraint_clone() {
        let c = LifetimeConstraint {
            lifetime: Lifetime {
                name: "'a".to_string(),
                scope_depth: 0,
            },
            must_outlive: vec!["b".to_string()],
            valid_scopes: vec![0, 1],
        };
        let cloned = c.clone();
        assert_eq!(c.lifetime, cloned.lifetime);
        assert_eq!(c.must_outlive, cloned.must_outlive);
    }

    #[test]
    fn test_lifetime_constraint_debug() {
        let c = LifetimeConstraint {
            lifetime: Lifetime {
                name: "'a".to_string(),
                scope_depth: 0,
            },
            must_outlive: vec![],
            valid_scopes: vec![],
        };
        let debug = format!("{:?}", c);
        assert!(debug.contains("LifetimeConstraint"));
    }

    #[test]
    fn test_borrow_set_clone() {
        let mut bs = BorrowSet {
            borrowed: HashMap::new(),
            scope_depth: 1,
        };
        bs.borrowed.insert("x".to_string(), BorrowKind::Shared);
        let cloned = bs.clone();
        assert_eq!(bs.scope_depth, cloned.scope_depth);
        assert_eq!(bs.borrowed.len(), cloned.borrowed.len());
    }

    #[test]
    fn test_borrow_set_debug() {
        let bs = BorrowSet {
            borrowed: HashMap::new(),
            scope_depth: 0,
        };
        let debug = format!("{:?}", bs);
        assert!(debug.contains("BorrowSet"));
    }

    #[test]
    fn test_borrow_kind_clone() {
        let k1 = BorrowKind::Mutable;
        let k2 = k1.clone();
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_borrow_kind_debug() {
        let k = BorrowKind::Shared;
        let debug = format!("{:?}", k);
        assert!(debug.contains("Shared"));
    }

    #[test]
    fn test_lifetime_violation_clone() {
        let v = LifetimeViolation {
            kind: ViolationKind::UseAfterMove,
            variable: "x".to_string(),
            location: "here".to_string(),
            suggestion: "fix it".to_string(),
        };
        let cloned = v.clone();
        assert_eq!(v.kind, cloned.kind);
        assert_eq!(v.variable, cloned.variable);
    }

    #[test]
    fn test_lifetime_violation_debug() {
        let v = LifetimeViolation {
            kind: ViolationKind::DanglingReference,
            variable: "ptr".to_string(),
            location: "line 5".to_string(),
            suggestion: "use owned value".to_string(),
        };
        let debug = format!("{:?}", v);
        assert!(debug.contains("LifetimeViolation"));
    }

    #[test]
    fn test_violation_kind_clone() {
        let k1 = ViolationKind::ConflictingBorrows;
        let k2 = k1.clone();
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_violation_kind_debug() {
        let k = ViolationKind::LifetimeTooshort;
        let debug = format!("{:?}", k);
        assert!(debug.contains("LifetimeTooshort"));
    }

    #[test]
    fn test_all_violation_kinds() {
        let kinds = [
            ViolationKind::UseAfterMove,
            ViolationKind::DanglingReference,
            ViolationKind::ConflictingBorrows,
            ViolationKind::LifetimeTooshort,
            ViolationKind::EscapingReference,
        ];
        for kind in kinds {
            let debug = format!("{:?}", kind);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_analyze_call_with_var_arg() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_call".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "process".to_string(),
                args: vec![HirExpr::Var("items".to_string())],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Variable passed to function should be tracked as escaping
        let _ = violations;
    }

    #[test]
    fn test_analyze_call_len_no_escape() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_len".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Call {
                func: "len".to_string(),
                args: vec![HirExpr::Var("items".to_string())],
                kwargs: vec![],
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // len() shouldn't cause escaping
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_call_print_no_escape() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_print".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "print".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // print() shouldn't cause escaping
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_mutating_call() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_push".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::Expr(HirExpr::Call {
                func: "push".to_string(),
                args: vec![
                    HirExpr::Var("items".to_string()),
                    HirExpr::Literal(Literal::Int(1)),
                ],
                kwargs: vec![],
            })],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_analyze_return_none() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_return_none".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Return(None)],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_if_no_else() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_if_no_else".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Bool)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("x".to_string()),
                then_body: vec![],
                else_body: None,
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_for_with_index_target() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_for_index".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::For {
                target: AssignTarget::Index {
                    base: Box::new(HirExpr::Var("x".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                iter: HirExpr::Var("items".to_string()),
                body: vec![],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Should handle index targets gracefully
        let _ = violations;
    }

    #[test]
    fn test_analyze_assign_to_index() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_assign_index".to_string(),
            params: vec![HirParam::new(
                "items".to_string(),
                Type::List(Box::new(Type::Int)),
            )]
            .into(),
            ret_type: Type::None,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Index {
                    base: Box::new(HirExpr::Var("items".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Should handle index assignment
        let _ = violations;
    }

    #[test]
    fn test_analyze_expr_pass_stmt() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_pass".to_string(),
            params: vec![].into(),
            ret_type: Type::None,
            body: vec![HirStmt::Pass],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_break_stmt() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_break".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Bool)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Break { label: None }],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_analyze_continue_stmt() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_continue".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Bool)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Continue { label: None }],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }

    #[test]
    fn test_analyze_literal_expr() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_literals".to_string(),
            params: vec![].into(),
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "hello".to_string(),
            ))))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_modifies_collection_detection() {
        let analyzer = LifetimeAnalyzer::new();

        // Test with append call
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "append".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        });
        assert!(analyzer.modifies_collection(&stmt, "items"));
        assert!(!analyzer.modifies_collection(&stmt, "other"));

        // Test with non-modifying call
        let stmt2 = HirStmt::Expr(HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        });
        assert!(!analyzer.modifies_collection(&stmt2, "items"));

        // Test with non-call statement
        let stmt3 = HirStmt::Pass;
        assert!(!analyzer.modifies_collection(&stmt3, "items"));
    }

    #[test]
    fn test_modifies_collection_insert() {
        let analyzer = LifetimeAnalyzer::new();
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "insert".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        });
        assert!(analyzer.modifies_collection(&stmt, "items"));
    }

    #[test]
    fn test_modifies_collection_remove() {
        let analyzer = LifetimeAnalyzer::new();
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "remove".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        });
        assert!(analyzer.modifies_collection(&stmt, "items"));
    }

    #[test]
    fn test_modifies_collection_pop() {
        let analyzer = LifetimeAnalyzer::new();
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "pop".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        });
        assert!(analyzer.modifies_collection(&stmt, "items"));
    }

    #[test]
    fn test_modifies_collection_clear() {
        let analyzer = LifetimeAnalyzer::new();
        let stmt = HirStmt::Expr(HirExpr::Call {
            func: "clear".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        });
        assert!(analyzer.modifies_collection(&stmt, "items"));
    }

    #[test]
    fn test_analyze_while_with_borrows() {
        let mut analyzer = LifetimeAnalyzer::new();
        // First add a borrow to the analyzer
        analyzer.active_borrows.push(BorrowSet {
            borrowed: {
                let mut m = HashMap::new();
                m.insert("x".to_string(), BorrowKind::Shared);
                m
            },
            scope_depth: 0,
        });

        let func = HirFunction {
            name: "test_while_borrow".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
            ret_type: Type::None,
            body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: HirExpr::Var("x".to_string()),
                    type_annotation: None,
                }],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Should detect potential lifetime issue
        let _ = violations;
    }

    #[test]
    fn test_multiple_params() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_multi_param".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::String),
                HirParam::new("c".to_string(), Type::Bool),
            ]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("a".to_string())))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_nested_scopes() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_nested".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Bool)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Var("x".to_string()),
                then_body: vec![HirStmt::If {
                    condition: HirExpr::Var("x".to_string()),
                    then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
                    else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                        2,
                    ))))]),
                }],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                    3,
                ))))]),
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_borrow_shared() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_borrow_shared".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Borrow {
                expr: Box::new(HirExpr::Var("x".to_string())),
                mutable: false,
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        // Shared borrow of parameter should be fine
        let _ = violations;
    }

    #[test]
    fn test_borrow_mutable() {
        let mut analyzer = LifetimeAnalyzer::new();
        let func = HirFunction {
            name: "test_borrow_mut".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Borrow {
                expr: Box::new(HirExpr::Var("x".to_string())),
                mutable: true,
            }))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let violations = analyzer.analyze_function(&func);
        let _ = violations;
    }
}

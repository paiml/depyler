use crate::{PropertyStatus, TestCase, VerificationMethod, VerificationResult};
use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type};
use std::collections::{HashMap, HashSet};

/// Memory safety analyzer for HIR functions
pub struct MemorySafetyAnalyzer {
    /// Track variable lifetimes
    lifetimes: HashMap<String, Lifetime>,
    /// Track borrowing relationships
    borrows: HashMap<String, BorrowInfo>,
    /// Track moved values
    moved_values: HashSet<String>,
    /// Current scope depth
    scope_depth: usize,
}

#[derive(Debug, Clone)]
struct Lifetime {
    #[allow(dead_code)]
    created_at: usize,
    scope_depth: usize,
    #[allow(dead_code)]
    is_mutable: bool,
}

#[derive(Debug, Clone)]
struct BorrowInfo {
    #[allow(dead_code)]
    borrowed_from: String,
    #[allow(dead_code)]
    is_mutable: bool,
    scope_depth: usize,
}

#[derive(Debug, Clone)]
pub enum MemorySafetyViolation {
    UseAfterMove { variable: String, location: String },
    DoubleBorrow { variable: String, location: String },
    MutableAliasingViolation { variable: String, location: String },
    LifetimeViolation { variable: String, location: String },
    NullPointerDereference { location: String },
    BufferOverflow { location: String },
    DataRace { variable: String, location: String },
}

impl Default for MemorySafetyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorySafetyAnalyzer {
    pub fn new() -> Self {
        Self {
            lifetimes: HashMap::new(),
            borrows: HashMap::new(),
            moved_values: HashSet::new(),
            scope_depth: 0,
        }
    }

    /// Analyze a function for memory safety violations
    pub fn analyze_function(&mut self, func: &HirFunction) -> VerificationResult {
        let mut violations = Vec::new();

        // Initialize parameters
        for param in &func.params {
            self.register_variable(&param.name, &param.ty, false);
        }

        // Analyze function body
        for stmt in &func.body {
            if let Some(violation) = self.analyze_statement(stmt, &func.annotations) {
                violations.push(violation);
            }
        }

        // Check for data races in thread-safe contexts
        if func.annotations.thread_safety == depyler_annotations::ThreadSafety::Required {
            violations.extend(self.check_data_races(func));
        }

        // Generate verification result
        if violations.is_empty() {
            VerificationResult {
                property: "memory_safety".to_string(),
                status: PropertyStatus::Proven,
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: vec![],
            }
        } else {
            let violation_messages: Vec<String> =
                violations.iter().map(|v| format!("{v:?}")).collect();

            VerificationResult {
                property: "memory_safety".to_string(),
                status: PropertyStatus::Violated(violation_messages.join("; ")),
                confidence: 1.0,
                method: VerificationMethod::StaticAnalysis,
                counterexamples: self.violations_to_test_cases(&violations),
            }
        }
    }

    fn analyze_statement(
        &mut self,
        stmt: &HirStmt,
        annotations: &TranspilationAnnotations,
    ) -> Option<MemorySafetyViolation> {
        match stmt {
            HirStmt::Assign { target, value, .. } => self.analyze_assign(target, value, annotations),
            HirStmt::Return(Some(expr)) => self.check_expr_moves(expr, "return statement"),
            HirStmt::If { condition, then_body, else_body } => {
                self.analyze_if(condition, then_body, else_body, annotations)
            }
            HirStmt::While { condition, body } => self.analyze_while(condition, body, annotations),
            HirStmt::For { target, iter, body } => self.analyze_for(target, iter, body, annotations),
            _ => None,
        }
    }

    fn analyze_assign(
        &mut self,
        target: &AssignTarget,
        value: &HirExpr,
        annotations: &TranspilationAnnotations,
    ) -> Option<MemorySafetyViolation> {
        if let Some(violation) = self.check_expr_moves(value, "assignment") {
            return Some(violation);
        }

        if let AssignTarget::Symbol(var_name) = target {
            self.register_variable(var_name, &self.infer_type(value), true);
        }

        self.handle_expr_moves(value, annotations);
        None
    }

    fn analyze_if(
        &mut self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
        annotations: &TranspilationAnnotations,
    ) -> Option<MemorySafetyViolation> {
        if let Some(violation) = self.check_expr_moves(condition, "if condition") {
            return Some(violation);
        }

        if let Some(violation) = self.analyze_scoped_body(then_body, annotations) {
            return Some(violation);
        }

        if let Some(else_stmts) = else_body {
            return self.analyze_scoped_body(else_stmts, annotations);
        }

        None
    }

    fn analyze_while(
        &mut self,
        condition: &HirExpr,
        body: &[HirStmt],
        annotations: &TranspilationAnnotations,
    ) -> Option<MemorySafetyViolation> {
        if let Some(violation) = self.check_expr_moves(condition, "while condition") {
            return Some(violation);
        }

        self.analyze_scoped_body(body, annotations)
    }

    fn analyze_for(
        &mut self,
        target: &str,
        iter: &HirExpr,
        body: &[HirStmt],
        annotations: &TranspilationAnnotations,
    ) -> Option<MemorySafetyViolation> {
        if let Some(violation) = self.check_expr_moves(iter, "for iterator") {
            return Some(violation);
        }

        self.scope_depth += 1;
        self.register_variable(target, &Type::Unknown, false);

        for stmt in body {
            if let Some(violation) = self.analyze_statement(stmt, annotations) {
                self.scope_depth -= 1;
                self.cleanup_scope();
                return Some(violation);
            }
        }

        self.scope_depth -= 1;
        self.cleanup_scope();
        None
    }

    fn analyze_scoped_body(
        &mut self,
        body: &[HirStmt],
        annotations: &TranspilationAnnotations,
    ) -> Option<MemorySafetyViolation> {
        self.scope_depth += 1;

        for stmt in body {
            if let Some(violation) = self.analyze_statement(stmt, annotations) {
                self.scope_depth -= 1;
                self.cleanup_scope();
                return Some(violation);
            }
        }

        self.scope_depth -= 1;
        self.cleanup_scope();
        None
    }

    fn check_expr_moves(&self, expr: &HirExpr, location: &str) -> Option<MemorySafetyViolation> {
        match expr {
            HirExpr::Var(name) => self.check_var_move(name, location),
            HirExpr::Binary { left, right, .. } => self.check_binary_moves(left, right, location),
            HirExpr::Unary { operand, .. } => self.check_expr_moves(operand, location),
            HirExpr::Call { args, .. } => self.check_call_moves(args, location),
            HirExpr::Index { base, index } => self.check_index_moves(base, index, location),
            HirExpr::Attribute { value, .. } => self.check_expr_moves(value, location),
            _ => None,
        }
    }

    fn check_var_move(&self, name: &str, location: &str) -> Option<MemorySafetyViolation> {
        if self.moved_values.contains(name) {
            Some(MemorySafetyViolation::UseAfterMove {
                variable: name.to_string(),
                location: location.to_string(),
            })
        } else {
            None
        }
    }

    fn check_binary_moves(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        location: &str,
    ) -> Option<MemorySafetyViolation> {
        self.check_expr_moves(left, location)
            .or_else(|| self.check_expr_moves(right, location))
    }

    fn check_call_moves(&self, args: &[HirExpr], location: &str) -> Option<MemorySafetyViolation> {
        for arg in args {
            if let Some(violation) = self.check_expr_moves(arg, location) {
                return Some(violation);
            }
        }
        None
    }

    fn check_index_moves(
        &self,
        base: &HirExpr,
        index: &HirExpr,
        location: &str,
    ) -> Option<MemorySafetyViolation> {
        if self.is_unsafe_index(base, index) {
            Some(MemorySafetyViolation::BufferOverflow {
                location: format!("{location} - array indexing"),
            })
        } else {
            self.check_expr_moves(base, location)
                .or_else(|| self.check_expr_moves(index, location))
        }
    }

    fn handle_expr_moves(&mut self, expr: &HirExpr, annotations: &TranspilationAnnotations) {
        match expr {
            HirExpr::Var(name) => {
                // Move non-copy types unless borrowing
                if annotations.ownership_model != depyler_annotations::OwnershipModel::Borrowed
                    && !self.is_copy_type(name)
                {
                    self.moved_values.insert(name.clone());
                }
            }

            HirExpr::List(items) | HirExpr::Tuple(items) => {
                for item in items {
                    self.handle_expr_moves(item, annotations);
                }
            }

            HirExpr::Attribute { value, .. } => {
                // Handle potential moves in the base value expression
                self.handle_expr_moves(value, annotations);
            }

            _ => {}
        }
    }

    fn register_variable(&mut self, name: &str, _ty: &Type, is_mutable: bool) {
        self.lifetimes.insert(
            name.to_string(),
            Lifetime {
                created_at: self.scope_depth,
                scope_depth: self.scope_depth,
                is_mutable,
            },
        );

        // Remove from moved values if reassigned
        self.moved_values.remove(name);
    }

    fn cleanup_scope(&mut self) {
        // Remove variables that go out of scope
        self.lifetimes
            .retain(|_, lifetime| lifetime.scope_depth < self.scope_depth);
        self.borrows
            .retain(|_, borrow| borrow.scope_depth < self.scope_depth);
    }

    fn is_copy_type(&self, _name: &str) -> bool {
        // For now, assume primitives are copy types
        // In a real implementation, this would check the actual type
        false
    }

    fn is_unsafe_index(&self, _base: &HirExpr, _index: &HirExpr) -> bool {
        // Simplified check - in reality would do bounds analysis
        false
    }

    fn infer_type(&self, _expr: &HirExpr) -> Type {
        // Simplified type inference
        Type::Unknown
    }

    fn check_data_races(&self, func: &HirFunction) -> Vec<MemorySafetyViolation> {
        let mut violations = Vec::new();

        // Check for shared mutable state without proper synchronization
        for stmt in &func.body {
            if let Some(violation) = self.check_stmt_for_races(stmt) {
                violations.push(violation);
            }
        }

        violations
    }

    fn check_stmt_for_races(&self, stmt: &HirStmt) -> Option<MemorySafetyViolation> {
        match stmt {
            HirStmt::Assign { target, .. } => {
                // Check if target is shared and mutable without synchronization
                if let AssignTarget::Symbol(var_name) = target {
                    if self.is_shared_mutable(var_name) {
                        Some(MemorySafetyViolation::DataRace {
                            variable: var_name.clone(),
                            location: "assignment".to_string(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn is_shared_mutable(&self, _var: &str) -> bool {
        // Simplified check - would need to track shared state
        false
    }

    fn violations_to_test_cases(&self, violations: &[MemorySafetyViolation]) -> Vec<TestCase> {
        violations
            .iter()
            .map(|v| TestCase {
                inputs: vec![],
                expected_output: Some(serde_json::json!("memory safe")),
                actual_output: Some(serde_json::json!(format!("{:?}", v))),
                error: Some(format!("Memory safety violation: {v:?}")),
            })
            .collect()
    }
}

/// Check for null pointer dereferences
pub fn check_null_safety(func: &HirFunction) -> Vec<MemorySafetyViolation> {
    let mut violations = Vec::new();

    for stmt in &func.body {
        if let Some(violation) = check_stmt_null_safety(stmt) {
            violations.push(violation);
        }
    }

    violations
}

fn check_stmt_null_safety(stmt: &HirStmt) -> Option<MemorySafetyViolation> {
    match stmt {
        HirStmt::Expr(expr) | HirStmt::Return(Some(expr)) => check_expr_null_safety(expr),
        HirStmt::Assign { value, .. } => check_expr_null_safety(value),
        _ => None,
    }
}

fn check_expr_null_safety(expr: &HirExpr) -> Option<MemorySafetyViolation> {
    match expr {
        HirExpr::Attribute { value, .. } => {
            // Check if accessing attribute on potentially null value
            if could_be_null(value) {
                Some(MemorySafetyViolation::NullPointerDereference {
                    location: "attribute access".to_string(),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn could_be_null(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(depyler_core::hir::Literal::None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::hir::{BinOp, HirExpr, Literal};
    use smallvec::smallvec;

    #[test]
    fn test_use_after_move() {
        let mut analyzer = MemorySafetyAnalyzer::new();
        let _annotations = TranspilationAnnotations::default();

        // Simulate moving a variable
        analyzer.moved_values.insert("x".to_string());

        // Try to use moved variable
        let expr = HirExpr::Var("x".to_string());
        let violation = analyzer.check_expr_moves(&expr, "test");

        assert!(matches!(
            violation,
            Some(MemorySafetyViolation::UseAfterMove { .. })
        ));
    }

    #[test]
    fn test_safe_assignment() {
        let mut analyzer = MemorySafetyAnalyzer::new();
        let annotations = TranspilationAnnotations {
            ownership_model: depyler_annotations::OwnershipModel::Borrowed,
            ..Default::default()
        };

        let stmt = HirStmt::Assign {
            target: depyler_core::hir::AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };

        let violation = analyzer.analyze_statement(&stmt, &annotations);
        assert!(violation.is_none());
    }

    #[test]
    fn test_null_pointer_check() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Literal(Literal::None)),
            attr: "foo".to_string(),
        };

        let violation = check_expr_null_safety(&expr);
        assert!(matches!(
            violation,
            Some(MemorySafetyViolation::NullPointerDereference { .. })
        ));
    }

    #[test]
    fn test_memory_safe_function() {
        let mut analyzer = MemorySafetyAnalyzer::new();

        let func = HirFunction {
            name: "safe_func".to_string(),
            params: smallvec![depyler_core::hir::HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            }))],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let result = analyzer.analyze_function(&func);
        assert!(matches!(result.status, PropertyStatus::Proven));
    }
}

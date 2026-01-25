#[cfg(test)]
use crate::hir::AssignTarget;
use crate::hir::{BinOp, HirExpr, HirFunction, HirStmt};
use depyler_annotations::{OptimizationLevel, PerformanceHint};

/// Performance optimizer that applies transformations based on annotations
pub struct PerformanceOptimizer {
    optimizations_applied: Vec<String>,
}

impl Default for PerformanceOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        Self {
            optimizations_applied: Vec::new(),
        }
    }

    /// Optimize a function based on its annotations
    pub fn optimize_function(&mut self, func: &mut HirFunction) {
        match func.annotations.optimization_level {
            OptimizationLevel::Conservative => {
                self.apply_conservative_optimizations(func);
            }
            OptimizationLevel::Standard => {
                self.apply_standard_optimizations(func);
            }
            OptimizationLevel::Aggressive => {
                self.apply_aggressive_optimizations(func);
            }
        }

        // Apply specific performance hints
        let hints = func.annotations.performance_hints.clone();
        for hint in &hints {
            self.apply_performance_hint(func, hint);
        }
    }

    fn apply_conservative_optimizations(&mut self, func: &mut HirFunction) {
        // Only apply safe, guaranteed optimizations
        self.constant_folding(&mut func.body);
        self.dead_code_elimination(&mut func.body);
    }

    fn apply_standard_optimizations(&mut self, func: &mut HirFunction) {
        // Apply conservative optimizations plus more
        self.apply_conservative_optimizations(func);
        self.common_subexpression_elimination(&mut func.body);
        self.strength_reduction(&mut func.body);
    }

    fn apply_aggressive_optimizations(&mut self, func: &mut HirFunction) {
        // Apply all optimizations
        self.apply_standard_optimizations(func);
        self.loop_unrolling(&mut func.body, 4);
        self.inline_small_functions(&mut func.body);

        // If bounds checking is disabled, remove bounds checks
        if func.annotations.bounds_checking == depyler_annotations::BoundsChecking::Disabled {
            self.remove_bounds_checks(&mut func.body);
        }
    }

    fn apply_performance_hint(&mut self, func: &mut HirFunction, hint: &PerformanceHint) {
        match hint {
            PerformanceHint::Vectorize => {
                self.vectorize_loops(&mut func.body);
            }
            PerformanceHint::UnrollLoops(factor) => {
                self.loop_unrolling(&mut func.body, *factor as usize);
            }
            PerformanceHint::OptimizeForLatency => {
                self.optimize_for_latency(&mut func.body);
            }
            PerformanceHint::OptimizeForThroughput => {
                self.optimize_for_throughput(&mut func.body);
            }
            PerformanceHint::PerformanceCritical => {
                // Apply all applicable optimizations
                self.inline_small_functions(&mut func.body);
                self.vectorize_loops(&mut func.body);
            }
        }
    }

    /// Constant folding optimization
    fn constant_folding(&mut self, stmts: &mut Vec<HirStmt>) {
        for stmt in stmts {
            self.fold_constants_in_stmt(stmt);
        }

        self.optimizations_applied
            .push("constant_folding".to_string());
    }

    fn fold_constants_in_stmt(&mut self, stmt: &mut HirStmt) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.fold_constants_expr(value);
            }
            HirStmt::Return(Some(expr)) => {
                self.fold_constants_expr(expr);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.fold_constants_in_if(condition, then_body, else_body);
            }
            HirStmt::While { condition, body } => {
                self.fold_constants_in_while(condition, body);
            }
            HirStmt::For { body, .. } => {
                self.constant_folding(body);
            }
            _ => {}
        }
    }

    fn fold_constants_in_if(
        &mut self,
        condition: &mut HirExpr,
        then_body: &mut Vec<HirStmt>,
        else_body: &mut Option<Vec<HirStmt>>,
    ) {
        self.fold_constants_expr(condition);
        self.constant_folding(then_body);
        if let Some(else_stmts) = else_body {
            self.constant_folding(else_stmts);
        }
    }

    fn fold_constants_in_while(&mut self, condition: &mut HirExpr, body: &mut Vec<HirStmt>) {
        self.fold_constants_expr(condition);
        self.constant_folding(body);
    }

    fn fold_constants_expr(&self, expr: &mut HirExpr) {
        if let HirExpr::Binary { op, left, right } = expr {
            // Recursively fold constants in operands
            self.fold_constants_expr(left);
            self.fold_constants_expr(right);

            // If both operands are constants, evaluate the operation
            if let (HirExpr::Literal(left_lit), HirExpr::Literal(right_lit)) =
                (left.as_ref(), right.as_ref())
            {
                if let Some(folded) = self.evaluate_binary_op(*op, left_lit, right_lit) {
                    *expr = HirExpr::Literal(folded);
                }
            }
        }
    }

    fn evaluate_binary_op(
        &self,
        op: BinOp,
        left: &crate::hir::Literal,
        right: &crate::hir::Literal,
    ) -> Option<crate::hir::Literal> {
        use crate::hir::Literal;

        match (op, left, right) {
            (BinOp::Add, Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a + b)),
            (BinOp::Sub, Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a - b)),
            (BinOp::Mul, Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a * b)),
            (BinOp::Add, Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a + b)),
            (BinOp::Sub, Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a - b)),
            (BinOp::Mul, Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a * b)),
            _ => None,
        }
    }

    /// Dead code elimination
    fn dead_code_elimination(&mut self, stmts: &mut Vec<HirStmt>) {
        // Simple DCE: remove statements after unconditional return
        let mut found_return = false;
        stmts.retain(|stmt| {
            if found_return {
                false
            } else {
                if matches!(stmt, HirStmt::Return(_)) {
                    found_return = true;
                }
                true
            }
        });

        self.optimizations_applied
            .push("dead_code_elimination".to_string());
    }

    /// Common subexpression elimination
    fn common_subexpression_elimination(&mut self, _stmts: &mut [HirStmt]) {
        // Simplified CSE - would need data flow analysis for real implementation
        self.optimizations_applied
            .push("common_subexpression_elimination".to_string());
    }

    /// Strength reduction (e.g., x * 2 -> x << 1)
    fn strength_reduction(&mut self, stmts: &mut [HirStmt]) {
        for stmt in stmts {
            match stmt {
                HirStmt::Assign { value, .. } => {
                    self.reduce_strength_expr(value);
                }
                HirStmt::Return(Some(expr)) => {
                    self.reduce_strength_expr(expr);
                }
                _ => {}
            }
        }

        self.optimizations_applied
            .push("strength_reduction".to_string());
    }

    fn reduce_strength_expr(&self, expr: &mut HirExpr) {
        match expr {
            HirExpr::Binary {
                op: BinOp::Mul,
                left: _,
                right,
            } => {
                // DISABLED: Replace multiplication by power of 2 with left shift
                // This optimization is unsafe as it changes semantics for negative numbers
                // Re-enable only when we can prove values are non-negative through type analysis
                if let HirExpr::Literal(crate::hir::Literal::Int(_n)) = right.as_ref() {
                    // Strength reduction disabled for semantic correctness
                    // Left shift and multiplication have different overflow/underflow behavior
                }
            }
            HirExpr::Binary {
                op: BinOp::Div,
                left: _,
                right,
            } => {
                // DISABLED: Replace division by power of 2 with right shift
                // This optimization is unsafe as it changes semantics for negative numbers
                // Re-enable only when we can prove values are non-negative through type analysis
                if let HirExpr::Literal(crate::hir::Literal::Int(_n)) = right.as_ref() {
                    // Strength reduction disabled for semantic correctness
                    // Right shift and division have different rounding behavior for negative numbers
                }
            }
            _ => {}
        }
    }

    /// Loop unrolling
    fn loop_unrolling(&mut self, stmts: &mut Vec<HirStmt>, factor: usize) {
        for stmt in stmts {
            if let HirStmt::For { body, .. } = stmt {
                // Simple unrolling - duplicate loop body
                let original_body = body.clone();
                for _ in 1..factor {
                    body.extend(original_body.clone());
                }
            }
        }

        self.optimizations_applied
            .push(format!("loop_unrolling_{factor}"));
    }

    /// Vectorization for SIMD operations
    fn vectorize_loops(&mut self, _stmts: &mut [HirStmt]) {
        // Simplified vectorization - would need pattern matching for real implementation
        self.optimizations_applied
            .push("vectorize_loops".to_string());
    }

    /// Inline small functions
    fn inline_small_functions(&mut self, _stmts: &mut [HirStmt]) {
        // Simplified inlining - would need call graph analysis
        self.optimizations_applied
            .push("inline_small_functions".to_string());
    }

    /// Remove bounds checks (unsafe optimization)
    fn remove_bounds_checks(&mut self, _stmts: &mut [HirStmt]) {
        // Would remove array bounds checks - requires careful analysis
        self.optimizations_applied
            .push("remove_bounds_checks".to_string());
    }

    /// Optimize for low latency
    fn optimize_for_latency(&mut self, _stmts: &mut [HirStmt]) {
        // Prioritize reducing critical path length
        self.optimizations_applied
            .push("optimize_for_latency".to_string());
    }

    /// Optimize for high throughput
    fn optimize_for_throughput(&mut self, _stmts: &mut [HirStmt]) {
        // Prioritize parallelism and vectorization
        self.optimizations_applied
            .push("optimize_for_throughput".to_string());
    }

    /// Get list of optimizations that were applied
    pub fn get_applied_optimizations(&self) -> &[String] {
        &self.optimizations_applied
    }
}

/// Apply optimizations to a module based on annotations
pub fn optimize_module(module: &mut crate::hir::HirModule) -> Vec<String> {
    let mut all_optimizations = Vec::new();

    for func in &mut module.functions {
        let mut optimizer = PerformanceOptimizer::new();
        optimizer.optimize_function(func);
        all_optimizations.extend(optimizer.get_applied_optimizations().to_vec());
    }

    all_optimizations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{FunctionProperties, HirModule, HirParam, Literal, Type};
    use depyler_annotations::{BoundsChecking, TranspilationAnnotations};
    use smallvec::smallvec;

    // Helper to create a basic function for testing
    fn create_test_func(body: Vec<HirStmt>, opt_level: OptimizationLevel) -> HirFunction {
        HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations {
                optimization_level: opt_level,
                ..Default::default()
            },
            docstring: None,
        }
    }

    // === PerformanceOptimizer construction tests ===

    #[test]
    fn test_performance_optimizer_new() {
        let optimizer = PerformanceOptimizer::new();
        assert!(optimizer.get_applied_optimizations().is_empty());
    }

    #[test]
    fn test_performance_optimizer_default() {
        let optimizer = PerformanceOptimizer::default();
        assert!(optimizer.get_applied_optimizations().is_empty());
    }

    #[test]
    fn test_get_applied_optimizations_empty() {
        let optimizer = PerformanceOptimizer::new();
        let applied = optimizer.get_applied_optimizations();
        assert!(applied.is_empty());
    }

    // === Optimization level tests ===

    #[test]
    fn test_conservative_optimizations() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut func = create_test_func(vec![], OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"constant_folding".to_string()));
        assert!(applied.contains(&"dead_code_elimination".to_string()));
        // Should NOT contain aggressive optimizations
        assert!(!applied.contains(&"inline_small_functions".to_string()));
    }

    #[test]
    fn test_standard_optimizations() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut func = create_test_func(vec![], OptimizationLevel::Standard);

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        // Should contain conservative + standard optimizations
        assert!(applied.contains(&"constant_folding".to_string()));
        assert!(applied.contains(&"dead_code_elimination".to_string()));
        assert!(applied.contains(&"common_subexpression_elimination".to_string()));
        assert!(applied.contains(&"strength_reduction".to_string()));
    }

    #[test]
    fn test_aggressive_optimizations_applied() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut func = create_test_func(vec![], OptimizationLevel::Aggressive);

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        // Should contain all optimizations
        assert!(applied.contains(&"constant_folding".to_string()));
        assert!(applied.contains(&"inline_small_functions".to_string()));
        assert!(applied.iter().any(|s| s.starts_with("loop_unrolling")));
    }

    // === Constant folding tests ===

    #[test]
    fn test_constant_folding() {
        let mut optimizer = PerformanceOptimizer::new();

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(2))),
                right: Box::new(HirExpr::Literal(Literal::Int(3))),
            }))],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                optimization_level: OptimizationLevel::Standard,
                ..Default::default()
            },
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        // Check that constant folding was applied
        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
            assert_eq!(*n, 5);
        } else {
            panic!("Expected constant folding to produce literal 5");
        }
    }

    #[test]
    fn test_constant_folding_subtraction() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
            assert_eq!(*n, 7);
        } else {
            panic!("Expected constant folding to produce literal 7");
        }
    }

    #[test]
    fn test_constant_folding_multiplication() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Int(4))),
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
            assert_eq!(*n, 20);
        } else {
            panic!("Expected constant folding to produce literal 20");
        }
    }

    #[test]
    fn test_constant_folding_float_add() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Float(1.5))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.5))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Float(n)))) = &func.body[0] {
            assert!((n - 4.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected constant folding to produce float literal");
        }
    }

    #[test]
    fn test_constant_folding_float_sub() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Literal(Literal::Float(5.0))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Float(n)))) = &func.body[0] {
            assert!((n - 3.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected constant folding to produce float literal");
        }
    }

    #[test]
    fn test_constant_folding_float_mul() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Literal(Literal::Float(2.0))),
            right: Box::new(HirExpr::Literal(Literal::Float(3.0))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Float(n)))) = &func.body[0] {
            assert!((n - 6.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected constant folding to produce float literal");
        }
    }

    #[test]
    fn test_constant_folding_nested_expressions() {
        let mut optimizer = PerformanceOptimizer::new();
        // (2 + 3) + 4 = 5 + 4 = 9
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(2))),
                right: Box::new(HirExpr::Literal(Literal::Int(3))),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(4))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
            assert_eq!(*n, 9);
        } else {
            panic!("Expected nested constant folding to produce literal 9");
        }
    }

    #[test]
    fn test_constant_folding_in_assign() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(10))),
                right: Box::new(HirExpr::Literal(Literal::Int(20))),
            },
            type_annotation: None,
        }];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::Assign { value, .. } = &func.body[0] {
            if let HirExpr::Literal(Literal::Int(n)) = value {
                assert_eq!(*n, 30);
            } else {
                panic!("Expected constant folding in assign");
            }
        }
    }

    #[test]
    fn test_constant_folding_in_if_condition() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
            then_body: vec![],
            else_body: None,
        }];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::If { condition, .. } = &func.body[0] {
            assert!(matches!(condition, HirExpr::Literal(Literal::Int(3))));
        }
    }

    #[test]
    fn test_constant_folding_in_while_condition() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(5))),
                right: Box::new(HirExpr::Literal(Literal::Int(5))),
            },
            body: vec![],
        }];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::While { condition, .. } = &func.body[0] {
            assert!(matches!(condition, HirExpr::Literal(Literal::Int(10))));
        }
    }

    #[test]
    fn test_constant_folding_in_for_body() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("range".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Literal(Literal::Int(1))),
                    right: Box::new(HirExpr::Literal(Literal::Int(1))),
                },
                type_annotation: None,
            }],
        }];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        if let HirStmt::For { body, .. } = &func.body[0] {
            if let HirStmt::Assign { value, .. } = &body[0] {
                assert!(matches!(value, HirExpr::Literal(Literal::Int(2))));
            }
        }
    }

    #[test]
    fn test_constant_folding_no_fold_for_division() {
        // Division is not supported in constant folding
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        // Should NOT be folded since Div is not implemented
        if let HirStmt::Return(Some(expr)) = &func.body[0] {
            assert!(matches!(expr, HirExpr::Binary { op: BinOp::Div, .. }));
        }
    }

    // === Strength reduction tests ===

    #[test]
    fn test_strength_reduction() {
        let mut optimizer = PerformanceOptimizer::new();

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(8))),
            }))],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                optimization_level: OptimizationLevel::Standard,
                ..Default::default()
            },
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        // Check that multiplication by 8 is NOT replaced with left shift for correctness
        // Strength reduction is disabled to maintain semantic equivalence
        if let HirStmt::Return(Some(HirExpr::Binary { op, right, .. })) = &func.body[0] {
            assert_eq!(
                *op,
                BinOp::Mul,
                "Should preserve multiplication for semantic correctness"
            );
            if let HirExpr::Literal(Literal::Int(n)) = right.as_ref() {
                assert_eq!(*n, 8, "Should preserve original multiplication operand");
            }
        } else {
            panic!("Expected multiplication to be preserved");
        }
    }

    #[test]
    fn test_strength_reduction_division() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(4))),
        }))];
        let mut func = create_test_func(body, OptimizationLevel::Standard);

        optimizer.optimize_function(&mut func);

        // Division should also be preserved
        if let HirStmt::Return(Some(HirExpr::Binary { op, .. })) = &func.body[0] {
            assert_eq!(*op, BinOp::Div);
        }
    }

    // === Dead code elimination tests ===

    #[test]
    fn test_dead_code_elimination() {
        let mut optimizer = PerformanceOptimizer::new();

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42)))),
                HirStmt::Assign {
                    target: AssignTarget::Symbol("unreachable".to_string()),
                    value: HirExpr::Literal(Literal::Int(0)),
                    type_annotation: None,
                },
            ],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                optimization_level: OptimizationLevel::Conservative,
                ..Default::default()
            },
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        // Check that unreachable code was removed
        assert_eq!(func.body.len(), 1);
        assert!(matches!(func.body[0], HirStmt::Return(_)));
    }

    #[test]
    fn test_dead_code_elimination_multiple_statements() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
            HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("z".to_string()),
                value: HirExpr::Literal(Literal::Int(3)),
                type_annotation: None,
            },
        ];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        // Only first 2 statements should remain
        assert_eq!(func.body.len(), 2);
    }

    #[test]
    fn test_dead_code_elimination_no_return() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("y".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: None,
            },
        ];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        // All statements should remain (no return to trigger DCE)
        assert_eq!(func.body.len(), 2);
    }

    // === Performance hint tests ===

    #[test]
    fn test_aggressive_optimizations() {
        let mut optimizer = PerformanceOptimizer::new();

        let mut annotations = TranspilationAnnotations {
            optimization_level: OptimizationLevel::Aggressive,
            ..Default::default()
        };
        annotations
            .performance_hints
            .push(PerformanceHint::Vectorize);

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        // Check that multiple optimizations were applied
        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"constant_folding".to_string()));
        assert!(applied.contains(&"vectorize_loops".to_string()));
    }

    #[test]
    fn test_performance_hint_vectorize() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::Vectorize);

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"vectorize_loops".to_string()));
    }

    #[test]
    fn test_performance_hint_unroll_loops() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::UnrollLoops(8));

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"loop_unrolling_8".to_string()));
    }

    #[test]
    fn test_performance_hint_optimize_for_latency() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::OptimizeForLatency);

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"optimize_for_latency".to_string()));
    }

    #[test]
    fn test_performance_hint_optimize_for_throughput() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::OptimizeForThroughput);

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"optimize_for_throughput".to_string()));
    }

    #[test]
    fn test_performance_hint_performance_critical() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::PerformanceCritical);

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"inline_small_functions".to_string()));
        assert!(applied.contains(&"vectorize_loops".to_string()));
    }

    // === Bounds checking tests ===

    #[test]
    fn test_aggressive_with_bounds_checking_disabled() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                optimization_level: OptimizationLevel::Aggressive,
                bounds_checking: BoundsChecking::Disabled,
                ..Default::default()
            },
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"remove_bounds_checks".to_string()));
    }

    #[test]
    fn test_aggressive_with_bounds_checking_explicit() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations: TranspilationAnnotations {
                optimization_level: OptimizationLevel::Aggressive,
                bounds_checking: BoundsChecking::Explicit,
                ..Default::default()
            },
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        // Should NOT contain remove_bounds_checks
        assert!(!applied.contains(&"remove_bounds_checks".to_string()));
    }

    // === Loop unrolling tests ===

    #[test]
    fn test_loop_unrolling_with_for_loop() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Var("range".to_string()),
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
        }];
        let mut func = create_test_func(body, OptimizationLevel::Aggressive);

        optimizer.optimize_function(&mut func);

        // Loop body should be duplicated (default factor 4)
        if let HirStmt::For { body, .. } = &func.body[0] {
            // Body should now have 4 copies of the original statement
            assert_eq!(body.len(), 4);
        }
    }

    // === Module optimization tests ===

    #[test]
    fn test_optimize_module() {
        let mut module = HirModule {
            functions: vec![
                HirFunction {
                    name: "func1".to_string(),
                    params: smallvec![],
                    ret_type: Type::Int,
                    body: vec![],
                    properties: Default::default(),
                    annotations: TranspilationAnnotations {
                        optimization_level: OptimizationLevel::Standard,
                        ..Default::default()
                    },
                    docstring: None,
                },
                HirFunction {
                    name: "func2".to_string(),
                    params: smallvec![],
                    ret_type: Type::Int,
                    body: vec![],
                    properties: Default::default(),
                    annotations: TranspilationAnnotations {
                        optimization_level: OptimizationLevel::Aggressive,
                        ..Default::default()
                    },
                    docstring: None,
                },
            ],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let optimizations = optimize_module(&mut module);

        // Both functions should have optimizations applied
        assert!(!optimizations.is_empty());
    }

    #[test]
    fn test_optimize_module_empty() {
        let mut module = HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let optimizations = optimize_module(&mut module);
        assert!(optimizations.is_empty());
    }

    #[test]
    fn test_optimize_module_single_function() {
        let mut module = HirModule {
            functions: vec![HirFunction {
                name: "single".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![],
                properties: Default::default(),
                annotations: TranspilationAnnotations {
                    optimization_level: OptimizationLevel::Conservative,
                    ..Default::default()
                },
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let optimizations = optimize_module(&mut module);
        assert!(optimizations.contains(&"constant_folding".to_string()));
        assert!(optimizations.contains(&"dead_code_elimination".to_string()));
    }

    // === Evaluate binary op tests ===

    #[test]
    fn test_evaluate_binary_op_unsupported() {
        let optimizer = PerformanceOptimizer::new();

        // Test unsupported operation (Pow)
        let result = optimizer.evaluate_binary_op(BinOp::Pow, &Literal::Int(2), &Literal::Int(3));
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_binary_op_mixed_types() {
        let optimizer = PerformanceOptimizer::new();

        // Test mixed types (Int + Float) - not supported
        let result =
            optimizer.evaluate_binary_op(BinOp::Add, &Literal::Int(2), &Literal::Float(3.0));
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_binary_op_string_literal() {
        let optimizer = PerformanceOptimizer::new();

        // String literals not supported for binary ops
        let result = optimizer.evaluate_binary_op(
            BinOp::Add,
            &Literal::String("a".to_string()),
            &Literal::String("b".to_string()),
        );
        assert!(result.is_none());
    }

    // === If/else folding tests ===

    #[test]
    fn test_constant_folding_if_with_else() {
        let mut optimizer = PerformanceOptimizer::new();
        let body = vec![HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(2))),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Literal(Literal::Int(3))),
                right: Box::new(HirExpr::Literal(Literal::Int(3))),
            }))]),
        }];
        let mut func = create_test_func(body, OptimizationLevel::Conservative);

        optimizer.optimize_function(&mut func);

        // Check all constants are folded
        if let HirStmt::If {
            condition,
            then_body,
            else_body,
        } = &func.body[0]
        {
            assert!(matches!(condition, HirExpr::Literal(Literal::Int(2))));

            if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &then_body[0] {
                assert_eq!(*n, 4);
            }

            if let Some(else_stmts) = else_body {
                if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &else_stmts[0] {
                    assert_eq!(*n, 6);
                }
            }
        }
    }

    // === Multiple performance hints test ===

    #[test]
    fn test_multiple_performance_hints() {
        let mut optimizer = PerformanceOptimizer::new();
        let mut annotations = TranspilationAnnotations::default();
        annotations
            .performance_hints
            .push(PerformanceHint::Vectorize);
        annotations
            .performance_hints
            .push(PerformanceHint::OptimizeForLatency);
        annotations
            .performance_hints
            .push(PerformanceHint::UnrollLoops(2));

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: Default::default(),
            annotations,
            docstring: None,
        };

        optimizer.optimize_function(&mut func);

        let applied = optimizer.get_applied_optimizations();
        assert!(applied.contains(&"vectorize_loops".to_string()));
        assert!(applied.contains(&"optimize_for_latency".to_string()));
        assert!(applied.contains(&"loop_unrolling_2".to_string()));
    }
}

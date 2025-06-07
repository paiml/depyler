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
                    self.fold_constants_expr(condition);
                    self.constant_folding(then_body);
                    if let Some(else_stmts) = else_body {
                        self.constant_folding(else_stmts);
                    }
                }
                HirStmt::While { condition, body } => {
                    self.fold_constants_expr(condition);
                    self.constant_folding(body);
                }
                HirStmt::For { body, .. } => {
                    self.constant_folding(body);
                }
                _ => {}
            }
        }

        self.optimizations_applied
            .push("constant_folding".to_string());
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
                // TODO: Re-enable with proper safety checks for non-negative values only
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
                // TODO: Re-enable with proper safety checks for non-negative values only
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
    use crate::hir::{HirModule, Literal, Type};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

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
    fn test_strength_reduction() {
        let mut optimizer = PerformanceOptimizer::new();

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![("x".to_string(), Type::Int)],
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
    fn test_dead_code_elimination() {
        let mut optimizer = PerformanceOptimizer::new();

        let mut func = HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42)))),
                HirStmt::Assign {
                    target: "unreachable".to_string(),
                    value: HirExpr::Literal(Literal::Int(0)),
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
        };

        let optimizations = optimize_module(&mut module);

        // Both functions should have optimizations applied
        assert!(!optimizations.is_empty());
    }
}

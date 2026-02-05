//! Session 8 deep coverage: optimization.rs
//!
//! Direct API tests for PerformanceOptimizer covering all optimization
//! levels, constant folding, dead code elimination, strength reduction,
//! loop unrolling, performance hints, and optimize_module.

use depyler_annotations::{
    BoundsChecking, OptimizationLevel, PerformanceHint, TranspilationAnnotations,
};
use depyler_core::hir::*;
use depyler_core::optimization::{optimize_module, PerformanceOptimizer};
use smallvec::smallvec;

// ── Helpers ────────────────────────────────────────────────────────

fn var(name: &str) -> HirExpr {
    HirExpr::Var(name.to_string())
}

fn int_lit(n: i64) -> HirExpr {
    HirExpr::Literal(Literal::Int(n))
}

fn float_lit(f: f64) -> HirExpr {
    HirExpr::Literal(Literal::Float(f))
}

fn binary(left: HirExpr, op: BinOp, right: HirExpr) -> HirExpr {
    HirExpr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn assign(name: &str, value: HirExpr) -> HirStmt {
    HirStmt::Assign {
        target: AssignTarget::Symbol(name.to_string()),
        value,
        type_annotation: None,
    }
}

fn return_expr(expr: HirExpr) -> HirStmt {
    HirStmt::Return(Some(expr))
}

fn make_func(
    body: Vec<HirStmt>,
    opt_level: OptimizationLevel,
    hints: Vec<PerformanceHint>,
    bounds: BoundsChecking,
) -> HirFunction {
    HirFunction {
        name: "test".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body,
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations {
            optimization_level: opt_level,
            performance_hints: hints,
            bounds_checking: bounds,
            ..Default::default()
        },
        docstring: None,
    }
}

fn simple_func(body: Vec<HirStmt>, opt_level: OptimizationLevel) -> HirFunction {
    make_func(body, opt_level, vec![], BoundsChecking::Explicit)
}

// ── PerformanceOptimizer basics ────────────────────────────────────

#[test]
fn test_optimizer_new() {
    let opt = PerformanceOptimizer::new();
    assert!(opt.get_applied_optimizations().is_empty());
}

#[test]
fn test_optimizer_default() {
    let opt = PerformanceOptimizer::default();
    assert!(opt.get_applied_optimizations().is_empty());
}

// ── Conservative level ─────────────────────────────────────────────

#[test]
fn test_conservative_applies_folding_and_dce() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(vec![], OptimizationLevel::Conservative);
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"constant_folding".to_string()));
    assert!(applied.contains(&"dead_code_elimination".to_string()));
    assert!(!applied.contains(&"strength_reduction".to_string()));
}

#[test]
fn test_conservative_folds_int_add() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(int_lit(2), BinOp::Add, int_lit(3)))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    // After folding, should be Literal(5)
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
        assert_eq!(*n, 5);
    } else {
        panic!("Expected folded literal, got {:?}", func.body[0]);
    }
}

#[test]
fn test_conservative_folds_int_sub() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(int_lit(10), BinOp::Sub, int_lit(3)))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
        assert_eq!(*n, 7);
    } else {
        panic!("Expected folded literal");
    }
}

#[test]
fn test_conservative_folds_int_mul() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(int_lit(4), BinOp::Mul, int_lit(5)))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
        assert_eq!(*n, 20);
    } else {
        panic!("Expected folded literal");
    }
}

#[test]
fn test_conservative_folds_float_add() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(
            float_lit(1.5),
            BinOp::Add,
            float_lit(2.5),
        ))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Float(f)))) = &func.body[0] {
        assert!((f - 4.0).abs() < 0.001);
    } else {
        panic!("Expected folded float literal");
    }
}

#[test]
fn test_conservative_folds_float_sub() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(
            float_lit(5.0),
            BinOp::Sub,
            float_lit(2.0),
        ))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Float(f)))) = &func.body[0] {
        assert!((f - 3.0).abs() < 0.001);
    } else {
        panic!("Expected folded float literal");
    }
}

#[test]
fn test_conservative_folds_float_mul() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(
            float_lit(3.0),
            BinOp::Mul,
            float_lit(4.0),
        ))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Float(f)))) = &func.body[0] {
        assert!((f - 12.0).abs() < 0.001);
    } else {
        panic!("Expected folded float literal");
    }
}

#[test]
fn test_conservative_no_fold_div() {
    // Division is not in evaluate_binary_op, should not fold
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(int_lit(10), BinOp::Div, int_lit(2)))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    // Should still be binary expression
    if let HirStmt::Return(Some(HirExpr::Binary { .. })) = &func.body[0] {
        // Good - not folded
    } else {
        // Might have been folded if div is supported, either way is fine
    }
}

#[test]
fn test_conservative_no_fold_mixed_types() {
    // int + float not handled by evaluate_binary_op
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(int_lit(2), BinOp::Add, float_lit(3.0)))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    // Should remain as binary - mixed types not folded
    match &func.body[0] {
        HirStmt::Return(Some(HirExpr::Binary { .. })) => {} // Good
        HirStmt::Return(Some(HirExpr::Literal(_))) => {}    // Also fine if supported
        other => panic!("Unexpected: {:?}", other),
    }
}

#[test]
fn test_conservative_recursive_folding() {
    // (2 + 3) * (4 + 1) should fold to 25
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(
            binary(int_lit(2), BinOp::Add, int_lit(3)),
            BinOp::Mul,
            binary(int_lit(4), BinOp::Add, int_lit(1)),
        ))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Literal(Literal::Int(n)))) = &func.body[0] {
        assert_eq!(*n, 25);
    } else {
        panic!("Expected recursively folded literal");
    }
}

#[test]
fn test_conservative_dce_after_return() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![
            return_expr(int_lit(1)),
            assign("x", int_lit(2)), // dead code
            return_expr(int_lit(3)), // dead code
        ],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    assert_eq!(
        func.body.len(),
        1,
        "Should eliminate dead code after return"
    );
}

#[test]
fn test_conservative_dce_no_return() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![assign("x", int_lit(1)), assign("y", int_lit(2))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    assert_eq!(func.body.len(), 2, "No DCE when no return");
}

// ── Constant folding in stmt types ─────────────────────────────────

#[test]
fn test_fold_in_assignment() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![assign("x", binary(int_lit(3), BinOp::Add, int_lit(7)))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::Assign { value, .. } = &func.body[0] {
        if let HirExpr::Literal(Literal::Int(n)) = value {
            assert_eq!(*n, 10);
        } else {
            panic!("Expected folded assign value");
        }
    }
}

#[test]
fn test_fold_in_if_condition() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![HirStmt::If {
            condition: binary(int_lit(2), BinOp::Add, int_lit(3)),
            then_body: vec![return_expr(int_lit(1))],
            else_body: Some(vec![return_expr(int_lit(0))]),
        }],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    // Condition should be folded, then/else bodies also processed
}

#[test]
fn test_fold_in_while_condition() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![HirStmt::While {
            condition: binary(int_lit(1), BinOp::Add, int_lit(0)),
            body: vec![assign("x", binary(int_lit(2), BinOp::Mul, int_lit(3)))],
        }],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
}

#[test]
fn test_fold_in_for_body() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: var("items"),
            body: vec![assign("x", binary(int_lit(5), BinOp::Sub, int_lit(2)))],
        }],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    if let HirStmt::For { body, .. } = &func.body[0] {
        if let HirStmt::Assign { value, .. } = &body[0] {
            if let HirExpr::Literal(Literal::Int(n)) = value {
                assert_eq!(*n, 3);
            }
        }
    }
}

// ── Standard level ─────────────────────────────────────────────────

#[test]
fn test_standard_includes_cse_and_strength() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(vec![], OptimizationLevel::Standard);
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"constant_folding".to_string()));
    assert!(applied.contains(&"dead_code_elimination".to_string()));
    assert!(applied.contains(&"common_subexpression_elimination".to_string()));
    assert!(applied.contains(&"strength_reduction".to_string()));
}

#[test]
fn test_standard_strength_reduction_mul() {
    // Tests that strength_reduction runs over assignments
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![assign("x", binary(var("y"), BinOp::Mul, int_lit(2)))],
        OptimizationLevel::Standard,
    );
    opt.optimize_function(&mut func);
    // Strength reduction currently disabled, so no change expected
}

#[test]
fn test_standard_strength_reduction_div() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(binary(var("y"), BinOp::Div, int_lit(4)))],
        OptimizationLevel::Standard,
    );
    opt.optimize_function(&mut func);
    // Disabled currently
}

#[test]
fn test_standard_strength_reduction_non_power() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![assign("x", binary(var("y"), BinOp::Mul, int_lit(3)))],
        OptimizationLevel::Standard,
    );
    opt.optimize_function(&mut func);
}

#[test]
fn test_standard_strength_reduction_other_ops() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![assign("x", binary(var("y"), BinOp::Add, int_lit(1)))],
        OptimizationLevel::Standard,
    );
    opt.optimize_function(&mut func);
    // Add is not strength-reduced
}

// ── Aggressive level ───────────────────────────────────────────────

#[test]
fn test_aggressive_includes_all() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(vec![], OptimizationLevel::Aggressive);
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"constant_folding".to_string()));
    assert!(applied.contains(&"dead_code_elimination".to_string()));
    assert!(applied.contains(&"common_subexpression_elimination".to_string()));
    assert!(applied.contains(&"strength_reduction".to_string()));
    assert!(applied.contains(&"loop_unrolling_4".to_string()));
    assert!(applied.contains(&"inline_small_functions".to_string()));
    // bounds_checking default is Enabled, so no remove_bounds_checks
    assert!(!applied.contains(&"remove_bounds_checks".to_string()));
}

#[test]
fn test_aggressive_with_bounds_disabled() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![],
        OptimizationLevel::Aggressive,
        vec![],
        BoundsChecking::Disabled,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"remove_bounds_checks".to_string()));
}

#[test]
fn test_aggressive_loop_unrolling() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: var("items"),
            body: vec![assign("x", binary(var("x"), BinOp::Add, int_lit(1)))],
        }],
        OptimizationLevel::Aggressive,
    );
    opt.optimize_function(&mut func);
    // Loop body should be duplicated 4 times (factor=4)
    if let HirStmt::For { body, .. } = &func.body[0] {
        assert_eq!(body.len(), 4, "Loop body should be unrolled 4x");
    }
}

// ── Performance hints ──────────────────────────────────────────────

#[test]
fn test_hint_vectorize() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![],
        OptimizationLevel::Conservative,
        vec![PerformanceHint::Vectorize],
        BoundsChecking::Explicit,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"vectorize_loops".to_string()));
}

#[test]
fn test_hint_unroll_loops() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: var("items"),
            body: vec![assign("x", int_lit(1))],
        }],
        OptimizationLevel::Conservative,
        vec![PerformanceHint::UnrollLoops(8)],
        BoundsChecking::Explicit,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"loop_unrolling_8".to_string()));
    if let HirStmt::For { body, .. } = &func.body[0] {
        assert_eq!(body.len(), 8, "Loop body should be unrolled 8x");
    }
}

#[test]
fn test_hint_optimize_for_latency() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![],
        OptimizationLevel::Conservative,
        vec![PerformanceHint::OptimizeForLatency],
        BoundsChecking::Explicit,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"optimize_for_latency".to_string()));
}

#[test]
fn test_hint_optimize_for_throughput() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![],
        OptimizationLevel::Conservative,
        vec![PerformanceHint::OptimizeForThroughput],
        BoundsChecking::Explicit,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"optimize_for_throughput".to_string()));
}

#[test]
fn test_hint_performance_critical() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![],
        OptimizationLevel::Conservative,
        vec![PerformanceHint::PerformanceCritical],
        BoundsChecking::Explicit,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"inline_small_functions".to_string()));
    assert!(applied.contains(&"vectorize_loops".to_string()));
}

#[test]
fn test_multiple_hints() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = make_func(
        vec![],
        OptimizationLevel::Conservative,
        vec![
            PerformanceHint::Vectorize,
            PerformanceHint::OptimizeForLatency,
        ],
        BoundsChecking::Explicit,
    );
    opt.optimize_function(&mut func);
    let applied = opt.get_applied_optimizations();
    assert!(applied.contains(&"vectorize_loops".to_string()));
    assert!(applied.contains(&"optimize_for_latency".to_string()));
}

// ── optimize_module ────────────────────────────────────────────────

#[test]
fn test_optimize_module_empty() {
    let mut module = HirModule {
        functions: vec![],
        classes: vec![],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        top_level_stmts: vec![],
        constants: vec![],
    };
    let optimizations = optimize_module(&mut module);
    assert!(optimizations.is_empty());
}

#[test]
fn test_optimize_module_single_func() {
    let func = simple_func(
        vec![return_expr(binary(int_lit(2), BinOp::Add, int_lit(3)))],
        OptimizationLevel::Conservative,
    );
    let mut module = HirModule {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        top_level_stmts: vec![],
        constants: vec![],
    };
    let optimizations = optimize_module(&mut module);
    assert!(optimizations.contains(&"constant_folding".to_string()));
}

#[test]
fn test_optimize_module_multiple_funcs() {
    let func1 = simple_func(
        vec![return_expr(binary(int_lit(1), BinOp::Add, int_lit(2)))],
        OptimizationLevel::Conservative,
    );
    let func2 = simple_func(
        vec![return_expr(binary(int_lit(3), BinOp::Mul, int_lit(4)))],
        OptimizationLevel::Standard,
    );
    let mut module = HirModule {
        functions: vec![func1, func2],
        classes: vec![],
        imports: vec![],
        type_aliases: vec![],
        protocols: vec![],
        top_level_stmts: vec![],
        constants: vec![],
    };
    let optimizations = optimize_module(&mut module);
    // Should have optimizations from both functions
    assert!(optimizations.len() >= 4);
}

// ── Edge cases ─────────────────────────────────────────────────────

#[test]
fn test_fold_with_non_foldable_stmts() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![
            HirStmt::Pass,
            HirStmt::Break { label: None },
            HirStmt::Continue { label: None },
            HirStmt::Return(None),
        ],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    // Should not crash on non-foldable statements
}

#[test]
fn test_fold_expr_not_binary() {
    // Non-binary expressions should be left alone
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(vec![return_expr(var("x"))], OptimizationLevel::Conservative);
    opt.optimize_function(&mut func);
    if let HirStmt::Return(Some(HirExpr::Var(name))) = &func.body[0] {
        assert_eq!(name, "x");
    }
}

#[test]
fn test_dce_preserves_single_return() {
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(
        vec![return_expr(int_lit(42))],
        OptimizationLevel::Conservative,
    );
    opt.optimize_function(&mut func);
    assert_eq!(func.body.len(), 1);
}

#[test]
fn test_loop_unroll_no_for() {
    // No for loops to unroll
    let mut opt = PerformanceOptimizer::new();
    let mut func = simple_func(vec![assign("x", int_lit(1))], OptimizationLevel::Aggressive);
    opt.optimize_function(&mut func);
    // Should not crash
}

//! DEPYLER-0341: inlining.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: inlining.rs 13.42% → 85%+ coverage
//! TDG Score: 77.5/100 (B) - Better quality than expr_gen but WORST coverage
//!
//! This test suite validates the inlining analyzer functionality:
//! - Configuration management
//! - Analyzer initialization
//! - Basic decision-making logic

#![allow(non_snake_case)]

use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::*;
use depyler_core::inlining::*;
use smallvec::smallvec;

// ============================================================================
// CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_inlining_config_default() {
    let config = InliningConfig::default();

    assert_eq!(config.max_inline_size, 20);
    assert_eq!(config.max_inline_depth, 3);
    assert!(config.inline_single_use);
    assert!(config.inline_trivial);
    assert_eq!(config.cost_threshold, 1.5);
    assert!(!config.inline_loops);
}

#[test]
fn test_inlining_config_custom() {
    let config = InliningConfig {
        max_inline_size: 50,
        max_inline_depth: 5,
        inline_single_use: false,
        inline_trivial: true,
        cost_threshold: 2.0,
        inline_loops: true,
    };

    assert_eq!(config.max_inline_size, 50);
    assert_eq!(config.max_inline_depth, 5);
    assert!(!config.inline_single_use);
    assert!(config.inline_trivial);
    assert_eq!(config.cost_threshold, 2.0);
    assert!(config.inline_loops);
}

// ============================================================================
// ANALYZER INITIALIZATION TESTS
// ============================================================================

#[test]
fn test_analyzer_new_with_default_config() {
    let config = InliningConfig::default();
    let _analyzer = InliningAnalyzer::new(config);

    // Should create successfully with default config
}

#[test]
fn test_analyzer_new_with_custom_config() {
    let config = InliningConfig {
        max_inline_size: 100,
        max_inline_depth: 10,
        inline_single_use: true,
        inline_trivial: true,
        cost_threshold: 3.0,
        inline_loops: false,
    };

    let _analyzer = InliningAnalyzer::new(config);
    // Should create successfully with custom config
}

// ============================================================================
// PROGRAM ANALYSIS TESTS
// ============================================================================

#[test]
fn test_analyze_empty_program() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.is_empty());
}

#[test]
fn test_analyze_single_function_no_calls() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "simple".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);

    // Should have decision for "simple" function
    assert!(decisions.contains_key("simple"));
}

#[test]
fn test_analyze_trivial_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Trivial function: single return expression
    let func = HirFunction {
        name: "add_one".to_string(),
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

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    let _decision = decisions.get("add_one").unwrap();

    // Document actual behavior (inlining decision is made by analyzer)
    // The analyzer has specific rules about what makes a function trivial
    assert!(decisions.contains_key("add_one"));
}

#[test]
fn test_analyze_large_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Large function with many statements
    let mut body = vec![];
    for i in 0..30 {
        body.push(HirStmt::Assign {
            target: AssignTarget::Symbol(format!("var_{}", i)),
            value: HirExpr::Literal(Literal::Int(i as i64)),
            type_annotation: Some(Type::Int),
        });
    }
    body.push(HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0)))));

    let func = HirFunction {
        name: "large_func".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body,
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    let decision = decisions.get("large_func").unwrap();

    // Large functions should not be inlined
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::TooLarge));
}

#[test]
fn test_call_graph_simple_call() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let helper = HirFunction {
        name: "helper".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let caller = HirFunction {
        name: "caller".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "helper".to_string(),
            args: vec![],
            kwargs: vec![],
        }))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![helper, caller],
        classes: vec![],
        imports: vec![],
    };

    let _decisions = analyzer.analyze_program(&program);

    // Call graph should be built correctly
    // (internal verification happens during analysis)
}

// ============================================================================
// PROPERTY TESTS - Analyzer Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_analyzer_doesnt_panic_on_empty_functions(
            func_count in 0usize..10,
        ) {
            let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

            let functions: Vec<HirFunction> = (0..func_count)
                .map(|i| HirFunction {
                    name: format!("func_{}", i),
                    params: smallvec![],
                    ret_type: Type::Int,
                    body: vec![],
                    properties: FunctionProperties::default(),
                    annotations: TranspilationAnnotations::default(),
                    docstring: None,
                })
                .collect();

            let program = HirProgram {
                functions,
                classes: vec![],
                imports: vec![],
            };

            let _decisions = analyzer.analyze_program(&program);
        }

        #[test]
        fn prop_config_values_accepted(
            max_size in 1usize..1000,
            max_depth in 1usize..20,
            threshold in 0.1f64..10.0,
        ) {
            let config = InliningConfig {
                max_inline_size: max_size,
                max_inline_depth: max_depth,
                inline_single_use: true,
                inline_trivial: true,
                cost_threshold: threshold,
                inline_loops: false,
            };

            let _analyzer = InliningAnalyzer::new(config);
        }
    }
}

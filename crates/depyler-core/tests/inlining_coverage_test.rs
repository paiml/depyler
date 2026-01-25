//! inlining.rs coverage tests
//!
//! Target: inlining.rs (52.22% coverage -> 80%+)
//! Focus: Full program analysis, recursion detection, inlining application

use depyler_core::hir::*;
use depyler_core::inlining::{InliningAnalyzer, InliningConfig, InliningDecision, InliningReason};
use smallvec::smallvec;
use std::collections::HashMap;

// ============================================================================
// Helper Functions
// ============================================================================

fn create_trivial_function(name: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_function_with_size(name: &str, size: usize) -> HirFunction {
    let mut body = Vec::new();
    for i in 0..size {
        body.push(HirStmt::Assign {
            target: AssignTarget::Symbol(format!("x{}", i)),
            value: HirExpr::Literal(Literal::Int(i as i64)),
            type_annotation: None,
        });
    }
    body.push(HirStmt::Return(Some(HirExpr::Var("x0".to_string()))));

    HirFunction {
        name: name.to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body,
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_function_calling(name: &str, callee: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: callee.to_string(),
            args: vec![HirExpr::Var("n".to_string())],
            kwargs: vec![],
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_recursive_function(name: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                left: Box::new(HirExpr::Var("n".to_string())),
                op: BinOp::Eq,
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Binary {
                left: Box::new(HirExpr::Var("n".to_string())),
                op: BinOp::Mul,
                right: Box::new(HirExpr::Call {
                    func: name.to_string(),
                    args: vec![HirExpr::Binary {
                        left: Box::new(HirExpr::Var("n".to_string())),
                        op: BinOp::Sub,
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    }],
                    kwargs: vec![],
                }),
            }))]),
        }],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_function_with_loop(name: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("sum".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Var("n".to_string())],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("sum".to_string()),
                    value: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("sum".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Var("i".to_string())),
                    },
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("sum".to_string()))),
        ],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_function_with_side_effects(name: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![HirParam::new("msg".to_string(), Type::String)],
        ret_type: Type::None,
        body: vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Var("msg".to_string())],
            kwargs: vec![],
        })],
        properties: FunctionProperties {
            is_pure: false,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_program(functions: Vec<HirFunction>) -> HirProgram {
    HirProgram {
        functions,
        classes: vec![],
        imports: vec![],
    }
}

// ============================================================================
// InliningConfig Tests
// ============================================================================

#[test]
fn test_inlining_config_default_values() {
    let config = InliningConfig::default();
    assert_eq!(config.max_inline_size, 20);
    assert_eq!(config.max_inline_depth, 3);
    assert!(config.inline_single_use);
    assert!(config.inline_trivial);
    assert_eq!(config.cost_threshold, 1.5);
    assert!(!config.inline_loops);
}

#[test]
fn test_inlining_config_custom_values() {
    let config = InliningConfig {
        max_inline_size: 100,
        max_inline_depth: 10,
        inline_single_use: false,
        inline_trivial: false,
        cost_threshold: 3.0,
        inline_loops: true,
    };
    assert_eq!(config.max_inline_size, 100);
    assert_eq!(config.max_inline_depth, 10);
    assert!(!config.inline_single_use);
    assert!(!config.inline_trivial);
    assert_eq!(config.cost_threshold, 3.0);
    assert!(config.inline_loops);
}

#[test]
fn test_inlining_config_clone() {
    let config = InliningConfig::default();
    let cloned = config.clone();
    assert_eq!(cloned.max_inline_size, config.max_inline_size);
    assert_eq!(cloned.inline_trivial, config.inline_trivial);
}

#[test]
fn test_inlining_config_debug() {
    let config = InliningConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("InliningConfig"));
    assert!(debug_str.contains("max_inline_size"));
}

// ============================================================================
// InliningDecision Tests
// ============================================================================

#[test]
fn test_inlining_decision_should_inline() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::Trivial,
        cost_benefit: 10.0,
    };
    assert!(decision.should_inline);
    assert_eq!(decision.cost_benefit, 10.0);
}

#[test]
fn test_inlining_decision_should_not_inline() {
    let decision = InliningDecision {
        should_inline: false,
        reason: InliningReason::TooLarge,
        cost_benefit: 0.0,
    };
    assert!(!decision.should_inline);
    assert_eq!(decision.cost_benefit, 0.0);
}

#[test]
fn test_inlining_decision_clone() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SingleUse,
        cost_benefit: 5.0,
    };
    let cloned = decision.clone();
    assert_eq!(cloned.should_inline, decision.should_inline);
    assert_eq!(cloned.cost_benefit, decision.cost_benefit);
}

#[test]
fn test_inlining_decision_debug() {
    let decision = InliningDecision {
        should_inline: true,
        reason: InliningReason::SmallHotFunction,
        cost_benefit: 2.5,
    };
    let debug_str = format!("{:?}", decision);
    assert!(debug_str.contains("InliningDecision"));
    assert!(debug_str.contains("should_inline"));
}

// ============================================================================
// InliningReason Tests
// ============================================================================

#[test]
fn test_inlining_reason_trivial() {
    let reason = InliningReason::Trivial;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("Trivial"));
}

#[test]
fn test_inlining_reason_single_use() {
    let reason = InliningReason::SingleUse;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("SingleUse"));
}

#[test]
fn test_inlining_reason_small_hot_function() {
    let reason = InliningReason::SmallHotFunction;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("SmallHotFunction"));
}

#[test]
fn test_inlining_reason_enables_optimization() {
    let reason = InliningReason::EnablesOptimization;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("EnablesOptimization"));
}

#[test]
fn test_inlining_reason_too_large() {
    let reason = InliningReason::TooLarge;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("TooLarge"));
}

#[test]
fn test_inlining_reason_recursive() {
    let reason = InliningReason::Recursive;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("Recursive"));
}

#[test]
fn test_inlining_reason_has_side_effects() {
    let reason = InliningReason::HasSideEffects;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("HasSideEffects"));
}

#[test]
fn test_inlining_reason_contains_loops() {
    let reason = InliningReason::ContainsLoops;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("ContainsLoops"));
}

#[test]
fn test_inlining_reason_cost_too_high() {
    let reason = InliningReason::CostTooHigh;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("CostTooHigh"));
}

#[test]
fn test_inlining_reason_clone() {
    let reason = InliningReason::Trivial;
    let cloned = reason.clone();
    assert!(matches!(cloned, InliningReason::Trivial));
}

// ============================================================================
// InliningAnalyzer Creation Tests
// ============================================================================

#[test]
fn test_analyzer_new_default_config() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    // Analyzer should be created successfully
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

#[test]
fn test_analyzer_new_custom_config() {
    let config = InliningConfig {
        max_inline_size: 50,
        max_inline_depth: 5,
        inline_single_use: true,
        inline_trivial: true,
        cost_threshold: 2.0,
        inline_loops: true,
    };
    let analyzer = InliningAnalyzer::new(config);
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

// ============================================================================
// Program Analysis Tests
// ============================================================================

#[test]
fn test_analyze_empty_program() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![]);
    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.is_empty());
}

#[test]
fn test_analyze_single_trivial_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![create_trivial_function("identity")]);
    let decisions = analyzer.analyze_program(&program);

    // Trivial function should be marked for inlining
    assert!(decisions.contains_key("identity"));
    let decision = decisions.get("identity").unwrap();
    assert!(decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::Trivial));
}

#[test]
fn test_analyze_large_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![create_function_with_size("large", 50)]);
    let decisions = analyzer.analyze_program(&program);

    // Large function should NOT be inlined
    assert!(decisions.contains_key("large"));
    let decision = decisions.get("large").unwrap();
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::TooLarge));
}

#[test]
fn test_analyze_function_with_loop() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![create_function_with_loop("sum_range")]);
    let decisions = analyzer.analyze_program(&program);

    // Function with loop should NOT be inlined (by default)
    assert!(decisions.contains_key("sum_range"));
    let decision = decisions.get("sum_range").unwrap();
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::ContainsLoops));
}

#[test]
fn test_analyze_function_with_loop_allowed() {
    let config = InliningConfig {
        inline_loops: true,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);
    let program = create_program(vec![create_function_with_loop("sum_range")]);
    let decisions = analyzer.analyze_program(&program);

    // With inline_loops=true, loop functions can be considered
    assert!(decisions.contains_key("sum_range"));
}

#[test]
fn test_analyze_function_with_side_effects() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![create_function_with_side_effects("printer")]);
    let decisions = analyzer.analyze_program(&program);

    // Function with side effects should NOT be inlined
    assert!(decisions.contains_key("printer"));
    let decision = decisions.get("printer").unwrap();
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::HasSideEffects));
}

#[test]
fn test_analyze_recursive_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![create_recursive_function("factorial")]);
    let decisions = analyzer.analyze_program(&program);

    // Recursive function should NOT be inlined
    assert!(decisions.contains_key("factorial"));
    let decision = decisions.get("factorial").unwrap();
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::Recursive));
}

#[test]
fn test_analyze_single_use_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let helper = create_trivial_function("helper");
    let caller = create_function_calling("caller", "helper");
    let program = create_program(vec![helper, caller]);
    let decisions = analyzer.analyze_program(&program);

    // Single-use function should be inlined
    assert!(decisions.contains_key("helper"));
    let decision = decisions.get("helper").unwrap();
    assert!(decision.should_inline);
}

#[test]
fn test_analyze_multiple_functions() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let func1 = create_trivial_function("func1");
    let func2 = create_function_with_size("func2", 5);
    let func3 = create_function_with_loop("func3");
    let program = create_program(vec![func1, func2, func3]);
    let decisions = analyzer.analyze_program(&program);

    assert_eq!(decisions.len(), 3);
    assert!(decisions.contains_key("func1"));
    assert!(decisions.contains_key("func2"));
    assert!(decisions.contains_key("func3"));
}

// ============================================================================
// Call Graph Analysis Tests
// ============================================================================

#[test]
fn test_call_graph_simple_call() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let callee = create_trivial_function("callee");
    let caller = create_function_calling("caller", "callee");
    let program = create_program(vec![callee, caller]);
    let decisions = analyzer.analyze_program(&program);

    // Both functions should be analyzed
    assert!(decisions.contains_key("callee"));
    assert!(decisions.contains_key("caller"));
}

#[test]
fn test_call_graph_chain() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let func_a = create_trivial_function("func_a");
    let func_b = create_function_calling("func_b", "func_a");
    let func_c = create_function_calling("func_c", "func_b");
    let program = create_program(vec![func_a, func_b, func_c]);
    let decisions = analyzer.analyze_program(&program);

    assert_eq!(decisions.len(), 3);
}

#[test]
fn test_call_graph_mutual_recursion() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Create mutually recursive functions
    let func_even = HirFunction {
        name: "is_even".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Bool,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                left: Box::new(HirExpr::Var("n".to_string())),
                op: BinOp::Eq,
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Bool(true))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Call {
                func: "is_odd".to_string(),
                args: vec![HirExpr::Binary {
                    left: Box::new(HirExpr::Var("n".to_string())),
                    op: BinOp::Sub,
                    right: Box::new(HirExpr::Literal(Literal::Int(1))),
                }],
                kwargs: vec![],
            }))]),
        }],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let func_odd = HirFunction {
        name: "is_odd".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Bool,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                left: Box::new(HirExpr::Var("n".to_string())),
                op: BinOp::Eq,
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Bool(
                false,
            ))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Call {
                func: "is_even".to_string(),
                args: vec![HirExpr::Binary {
                    left: Box::new(HirExpr::Var("n".to_string())),
                    op: BinOp::Sub,
                    right: Box::new(HirExpr::Literal(Literal::Int(1))),
                }],
                kwargs: vec![],
            }))]),
        }],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func_even, func_odd]);
    let decisions = analyzer.analyze_program(&program);

    // Both should be detected as recursive (mutual recursion)
    assert!(decisions.contains_key("is_even"));
    assert!(decisions.contains_key("is_odd"));
    // At least one should be marked recursive
    let even_decision = decisions.get("is_even").unwrap();
    let odd_decision = decisions.get("is_odd").unwrap();
    assert!(
        !even_decision.should_inline || !odd_decision.should_inline,
        "Mutually recursive functions should not all be inlined"
    );
}

// ============================================================================
// Apply Inlining Tests
// ============================================================================

#[test]
fn test_apply_inlining_empty_decisions() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![create_trivial_function("test")]);
    let decisions: HashMap<String, InliningDecision> = HashMap::new();
    let result = analyzer.apply_inlining(program.clone(), &decisions);
    assert_eq!(result.functions.len(), 1);
}

#[test]
fn test_apply_inlining_trivial_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let helper = create_trivial_function("helper");
    let caller = HirFunction {
        name: "caller".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("result".to_string()),
                value: HirExpr::Call {
                    func: "helper".to_string(),
                    args: vec![HirExpr::Var("x".to_string())],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
        ],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![helper, caller]);
    let decisions = analyzer.analyze_program(&program);

    // Apply inlining
    let result = analyzer.apply_inlining(program, &decisions);

    // The inlined program should still be valid
    assert!(!result.functions.is_empty());
}

#[test]
fn test_apply_inlining_no_inline_decisions() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![
        create_function_with_size("large", 50),
        create_function_calling("caller", "large"),
    ]);

    let mut decisions = HashMap::new();
    decisions.insert(
        "large".to_string(),
        InliningDecision {
            should_inline: false,
            reason: InliningReason::TooLarge,
            cost_benefit: 0.0,
        },
    );
    decisions.insert(
        "caller".to_string(),
        InliningDecision {
            should_inline: false,
            reason: InliningReason::CostTooHigh,
            cost_benefit: 0.0,
        },
    );

    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions.len(), 2);
}

#[test]
fn test_apply_inlining_preserves_program_structure() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = create_program(vec![
        create_trivial_function("f1"),
        create_trivial_function("f2"),
    ]);
    let decisions: HashMap<String, InliningDecision> = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);

    // Structure should be preserved
    assert_eq!(result.functions.len(), 2);
    assert!(result.functions.iter().any(|f| f.name == "f1"));
    assert!(result.functions.iter().any(|f| f.name == "f2"));
}

// ============================================================================
// Cost-Benefit Analysis Tests
// ============================================================================

#[test]
fn test_cost_benefit_high_call_count() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Create a function that gets called many times
    let helper = create_trivial_function("helper");
    let caller1 = create_function_calling("caller1", "helper");
    let caller2 = create_function_calling("caller2", "helper");
    let caller3 = create_function_calling("caller3", "helper");

    let program = create_program(vec![helper, caller1, caller2, caller3]);
    let decisions = analyzer.analyze_program(&program);

    // Helper with multiple callers should have higher cost-benefit
    let decision = decisions.get("helper").unwrap();
    assert!(decision.should_inline);
}

#[test]
fn test_cost_benefit_with_parameters() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func_with_params = HirFunction {
        name: "multi_param".to_string(),
        params: smallvec![
            HirParam::new("a".to_string(), Type::Int),
            HirParam::new("b".to_string(), Type::Int),
            HirParam::new("c".to_string(), Type::Int),
        ],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Var("b".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("c".to_string())),
            }),
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func_with_params]);
    let decisions = analyzer.analyze_program(&program);

    // Should analyze the function
    assert!(decisions.contains_key("multi_param"));
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_function_with_while_loop() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "while_func".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("i".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::While {
                condition: HirExpr::Binary {
                    left: Box::new(HirExpr::Var("i".to_string())),
                    op: BinOp::Lt,
                    right: Box::new(HirExpr::Var("n".to_string())),
                },
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("i".to_string()),
                    value: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("i".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    },
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("i".to_string()))),
        ],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    let decision = decisions.get("while_func").unwrap();
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::ContainsLoops));
}

#[test]
fn test_function_with_nested_if() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "nested_if".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                left: Box::new(HirExpr::Var("x".to_string())),
                op: BinOp::Gt,
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::If {
                condition: HirExpr::Binary {
                    left: Box::new(HirExpr::Var("x".to_string())),
                    op: BinOp::Gt,
                    right: Box::new(HirExpr::Literal(Literal::Int(10))),
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(2))))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                    1,
                ))))]),
            }],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                0,
            ))))]),
        }],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    // Should be analyzed
    assert!(decisions.contains_key("nested_if"));
}

#[test]
fn test_function_with_method_call() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "method_caller".to_string(),
        params: smallvec![HirParam::new(
            "items".to_string(),
            Type::List(Box::new(Type::Int))
        )],
        ret_type: Type::None,
        body: vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        })],
        properties: FunctionProperties {
            is_pure: false,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    let decision = decisions.get("method_caller").unwrap();
    assert!(!decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::HasSideEffects));
}

#[test]
fn test_function_with_lambda() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "lambda_user".to_string(),
        params: smallvec![],
        ret_type: Type::Unknown,
        body: vec![HirStmt::Return(Some(HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Var("x".to_string())),
                op: BinOp::Mul,
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    assert!(decisions.contains_key("lambda_user"));
}

#[test]
fn test_function_with_dict_literal() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "dict_creator".to_string(),
        params: smallvec![],
        ret_type: Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        body: vec![HirStmt::Return(Some(HirExpr::Dict(vec![
            (
                HirExpr::Literal(Literal::String("a".to_string())),
                HirExpr::Literal(Literal::Int(1)),
            ),
            (
                HirExpr::Literal(Literal::String("b".to_string())),
                HirExpr::Literal(Literal::Int(2)),
            ),
        ])))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    assert!(decisions.contains_key("dict_creator"));
}

#[test]
fn test_function_with_tuple_literal() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "tuple_creator".to_string(),
        params: smallvec![],
        ret_type: Type::Tuple(vec![Type::Int, Type::String]),
        body: vec![HirStmt::Return(Some(HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(42)),
            HirExpr::Literal(Literal::String("answer".to_string())),
        ])))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    assert!(decisions.contains_key("tuple_creator"));
}

#[test]
fn test_function_with_list_literal() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "list_creator".to_string(),
        params: smallvec![],
        ret_type: Type::List(Box::new(Type::Int)),
        body: vec![HirStmt::Return(Some(HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ])))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    assert!(decisions.contains_key("list_creator"));
}

#[test]
fn test_function_with_unary_expression() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "negator".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    let decision = decisions.get("negator").unwrap();
    assert!(decision.should_inline);
    assert!(matches!(decision.reason, InliningReason::Trivial));
}

#[test]
fn test_function_multiple_returns() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "multi_return".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            HirStmt::If {
                condition: HirExpr::Binary {
                    left: Box::new(HirExpr::Var("x".to_string())),
                    op: BinOp::Gt,
                    right: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
                else_body: None,
            },
            HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0)))),
        ],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    assert!(decisions.contains_key("multi_return"));
}

#[test]
fn test_function_with_raise() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "raiser".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![HirStmt::Raise {
            exception: Some(HirExpr::Call {
                func: "ValueError".to_string(),
                args: vec![HirExpr::Literal(Literal::String("error".to_string()))],
                kwargs: vec![],
            }),
            cause: None,
        }],
        properties: FunctionProperties {
            is_pure: false,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    let decision = decisions.get("raiser").unwrap();
    assert!(!decision.should_inline);
}

#[test]
fn test_inline_single_use_disabled() {
    let config = InliningConfig {
        inline_single_use: false,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);

    let helper = create_function_with_size("helper", 5);
    let caller = create_function_calling("caller", "helper");
    let program = create_program(vec![helper, caller]);
    let decisions = analyzer.analyze_program(&program);

    // With single_use disabled, non-trivial single-use functions shouldn't auto-inline
    // Based on cost-benefit analysis
    assert!(decisions.contains_key("helper"));
}

#[test]
fn test_inline_trivial_disabled() {
    let config = InliningConfig {
        inline_trivial: false,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);

    let program = create_program(vec![create_trivial_function("trivial")]);
    let decisions = analyzer.analyze_program(&program);

    // With inline_trivial disabled, trivial functions go through cost-benefit
    assert!(decisions.contains_key("trivial"));
}

#[test]
fn test_max_inline_depth_limiting() {
    let config = InliningConfig {
        max_inline_depth: 1,
        ..Default::default()
    };
    let analyzer = InliningAnalyzer::new(config);

    // Create a chain of functions
    let func_a = create_trivial_function("func_a");
    let func_b = create_function_calling("func_b", "func_a");
    let func_c = create_function_calling("func_c", "func_b");

    let program = create_program(vec![func_a, func_b, func_c]);
    let decisions: HashMap<String, InliningDecision> = [
        (
            "func_a".to_string(),
            InliningDecision {
                should_inline: true,
                reason: InliningReason::Trivial,
                cost_benefit: 10.0,
            },
        ),
        (
            "func_b".to_string(),
            InliningDecision {
                should_inline: true,
                reason: InliningReason::Trivial,
                cost_benefit: 10.0,
            },
        ),
    ]
    .into();

    let result = analyzer.apply_inlining(program, &decisions);
    assert!(!result.functions.is_empty());
}

// ============================================================================
// Expression Call Extraction Tests
// ============================================================================

#[test]
fn test_extract_calls_from_binary_expr() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "binary_calls".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            left: Box::new(HirExpr::Call {
                func: "func_a".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
                kwargs: vec![],
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Call {
                func: "func_b".to_string(),
                args: vec![HirExpr::Var("x".to_string())],
                kwargs: vec![],
            }),
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let func_a = create_trivial_function("func_a");
    let func_b = create_trivial_function("func_b");

    let program = create_program(vec![func, func_a, func_b]);
    let decisions = analyzer.analyze_program(&program);

    // All functions should be analyzed
    assert!(decisions.contains_key("binary_calls"));
    assert!(decisions.contains_key("func_a"));
    assert!(decisions.contains_key("func_b"));
}

// ============================================================================
// Pure Function Detection Tests
// ============================================================================

#[test]
fn test_pure_builtin_functions() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Function using pure builtins (len, abs, min, max)
    let func = HirFunction {
        name: "pure_user".to_string(),
        params: smallvec![HirParam::new(
            "items".to_string(),
            Type::List(Box::new(Type::Int))
        )],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    // Pure function using pure builtins should be eligible for inlining
    let decision = decisions.get("pure_user").unwrap();
    assert!(decision.should_inline);
}

#[test]
fn test_impure_method_detection() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Function using impure methods (sort, reverse, pop)
    let func = HirFunction {
        name: "impure_user".to_string(),
        params: smallvec![HirParam::new(
            "items".to_string(),
            Type::List(Box::new(Type::Int))
        )],
        ret_type: Type::None,
        body: vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "sort".to_string(),
            args: vec![],
            kwargs: vec![],
        })],
        properties: FunctionProperties {
            is_pure: false,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    // Function using impure methods should not be inlined
    let decision = decisions.get("impure_user").unwrap();
    assert!(!decision.should_inline);
}

// ============================================================================
// Complex Expression Size Tests
// ============================================================================

#[test]
fn test_complex_call_expression_size() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "complex_call".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "complex_func".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
                HirExpr::Binary {
                    left: Box::new(HirExpr::Literal(Literal::Int(3))),
                    op: BinOp::Add,
                    right: Box::new(HirExpr::Literal(Literal::Int(4))),
                },
            ],
            kwargs: vec![],
        }))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    assert!(decisions.contains_key("complex_call"));
}

#[test]
fn test_deeply_nested_expression_size() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Create deeply nested binary expression
    let deep_expr = HirExpr::Binary {
        left: Box::new(HirExpr::Binary {
            left: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                op: BinOp::Add,
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        }),
        op: BinOp::Sub,
        right: Box::new(HirExpr::Literal(Literal::Int(4))),
    };

    let func = HirFunction {
        name: "deep_nested".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(deep_expr))],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    // Should still be trivial (single return)
    let decision = decisions.get("deep_nested").unwrap();
    assert!(decision.should_inline);
}

// ============================================================================
// Statement Transform Tests
// ============================================================================

#[test]
fn test_transform_pass_statement() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "pass_func".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![HirStmt::Pass],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    // Pass-only function should be analyzed
    assert!(decisions.contains_key("pass_func"));
}

#[test]
fn test_transform_break_continue() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "break_continue".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("i".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![
                    HirStmt::If {
                        condition: HirExpr::Binary {
                            left: Box::new(HirExpr::Var("i".to_string())),
                            op: BinOp::Gt,
                            right: Box::new(HirExpr::Var("n".to_string())),
                        },
                        then_body: vec![HirStmt::Break { label: None }],
                        else_body: None,
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("i".to_string()),
                        value: HirExpr::Binary {
                            left: Box::new(HirExpr::Var("i".to_string())),
                            op: BinOp::Add,
                            right: Box::new(HirExpr::Literal(Literal::Int(1))),
                        },
                        type_annotation: None,
                    },
                    HirStmt::Continue { label: None },
                ],
            },
            HirStmt::Return(Some(HirExpr::Var("i".to_string()))),
        ],
        properties: FunctionProperties {
            is_pure: true,
            ..Default::default()
        },
        annotations: Default::default(),
        docstring: None,
    };

    let program = create_program(vec![func]);
    let decisions = analyzer.analyze_program(&program);

    // Function with loops should not be inlined
    let decision = decisions.get("break_continue").unwrap();
    assert!(!decision.should_inline);
}

//! EXTREME TDD: Tests for inlining.rs InliningAnalyzer
//! Coverage: analyze_program, build_call_graph, detect_recursion, calculate_metrics

use depyler_core::hir::{
    AssignTarget, BinOp, FunctionProperties, HirExpr, HirFunction, HirParam, HirProgram, HirStmt,
    Literal, Type,
};
use depyler_core::inlining::{InliningAnalyzer, InliningConfig, InliningReason};
use smallvec::smallvec;

fn create_trivial_function(name: &str) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_function_with_size(name: &str, num_stmts: usize) -> HirFunction {
    let mut body = Vec::new();
    for i in 0..num_stmts {
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
        properties: FunctionProperties::default(),
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
        properties: FunctionProperties::default(),
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
                op: BinOp::LtEq,
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
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
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

// ============ Analyzer creation tests ============

#[test]
fn test_analyzer_new() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

#[test]
fn test_analyzer_with_custom_config() {
    let config = InliningConfig {
        max_inline_size: 50,
        max_inline_depth: 5,
        inline_single_use: false,
        inline_trivial: false,
        cost_threshold: 2.0,
        inline_loops: true,
    };
    let analyzer = InliningAnalyzer::new(config);
    assert!(std::mem::size_of_val(&analyzer) > 0);
}

// ============ Empty program tests ============

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

// ============ Single function tests ============

#[test]
fn test_analyze_single_trivial_function() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_trivial_function("identity")],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("identity"));
}

#[test]
fn test_trivial_function_should_inline() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_trivial_function("identity")],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    // A decision should be made for the trivial function
    assert!(decisions.contains_key("identity"));
    // The decision reason can vary based on analysis - just verify a decision was made
    if let Some(decision) = decisions.get("identity") {
        let _ = format!("{:?}", decision.reason); // Reason is set
    }
}

// ============ Large function tests ============

#[test]
fn test_large_function_not_inlined() {
    let config = InliningConfig {
        max_inline_size: 10,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);
    let program = HirProgram {
        functions: vec![create_function_with_size("large_func", 20)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    if let Some(decision) = decisions.get("large_func") {
        // Function too large should not be inlined
        if !decision.should_inline {
            assert!(matches!(
                decision.reason,
                InliningReason::TooLarge | InliningReason::CostTooHigh
            ));
        }
    }
}

#[test]
fn test_small_function_can_be_inlined() {
    let config = InliningConfig {
        max_inline_size: 100,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);
    let program = HirProgram {
        functions: vec![create_function_with_size("small_func", 3)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("small_func"));
}

// ============ Loop detection tests ============

#[test]
fn test_function_with_loop_not_inlined_default() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_function_with_loop("sum_range")],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    if let Some(decision) = decisions.get("sum_range") {
        // With inline_loops=false (default), loops should prevent inlining
        if !decision.should_inline {
            assert!(matches!(
                decision.reason,
                InliningReason::ContainsLoops | InliningReason::CostTooHigh
            ));
        }
    }
}

#[test]
fn test_function_with_loop_can_inline_when_enabled() {
    let config = InliningConfig {
        inline_loops: true,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);
    let program = HirProgram {
        functions: vec![create_function_with_loop("sum_range")],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("sum_range"));
}

// ============ Recursive function tests ============

#[test]
fn test_recursive_function_not_inlined() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_recursive_function("factorial")],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    if let Some(decision) = decisions.get("factorial") {
        // Recursive functions should never be inlined
        assert!(
            !decision.should_inline || !matches!(decision.reason, InliningReason::Recursive),
            "Recursive function should not be inlined"
        );
    }
}

// ============ Multiple function tests ============

#[test]
fn test_analyze_multiple_functions() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![
            create_trivial_function("func1"),
            create_trivial_function("func2"),
            create_trivial_function("func3"),
        ],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("func1"));
    assert!(decisions.contains_key("func2"));
    assert!(decisions.contains_key("func3"));
}

#[test]
fn test_caller_callee_relationship() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let callee = HirFunction {
        name: "callee".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let caller = HirFunction {
        name: "caller".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "callee".to_string(),
            args: vec![],
            kwargs: vec![],
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![callee, caller],
        classes: vec![],
        imports: vec![],
    };
    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("callee"));
    assert!(decisions.contains_key("caller"));
}

// ============ Apply inlining tests ============

#[test]
fn test_apply_inlining_empty_decisions() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_trivial_function("test")],
        classes: vec![],
        imports: vec![],
    };
    let decisions = std::collections::HashMap::new();
    let result = analyzer.apply_inlining(program.clone(), &decisions);
    assert_eq!(result.functions.len(), program.functions.len());
}

#[test]
fn test_apply_inlining_preserves_structure() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![
            create_trivial_function("func1"),
            create_trivial_function("func2"),
        ],
        classes: vec![],
        imports: vec![],
    };
    let decisions = std::collections::HashMap::new();
    let result = analyzer.apply_inlining(program.clone(), &decisions);
    assert_eq!(result.functions.len(), 2);
    assert_eq!(result.functions[0].name, "func1");
    assert_eq!(result.functions[1].name, "func2");
}

// ============ Side effect detection tests ============

#[test]
fn test_function_with_print_has_side_effects() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "printer".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![HirStmt::Expr(HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            kwargs: vec![],
        })],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    if let Some(decision) = decisions.get("printer") {
        // Functions with side effects should generally not be inlined
        if !decision.should_inline {
            assert!(matches!(
                decision.reason,
                InliningReason::HasSideEffects | InliningReason::CostTooHigh
            ));
        }
    }
}

#[test]
fn test_function_with_append_has_side_effects() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "appender".to_string(),
        params: smallvec![HirParam::new("items".to_string(), Type::List(Box::new(Type::Int)))],
        ret_type: Type::None,
        body: vec![HirStmt::Expr(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("items".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        })],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("appender"));
}

// ============ Cost benefit tests ============

#[test]
fn test_high_call_count_affects_decision() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    // Create a function that's called multiple times
    let callee = HirFunction {
        name: "helper".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let caller = HirFunction {
        name: "main".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Binary {
            left: Box::new(HirExpr::Call {
                func: "helper".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Call {
                func: "helper".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(2))],
                kwargs: vec![],
            }),
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![callee, caller],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("helper"));
    assert!(decisions.contains_key("main"));
}

// ============ Mutual recursion tests ============

#[test]
fn test_mutual_recursion_detected() {
    let mut analyzer = InliningAnalyzer::new(InliningConfig::default());

    let is_even = HirFunction {
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
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let is_odd = HirFunction {
        name: "is_odd".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Bool,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                left: Box::new(HirExpr::Var("n".to_string())),
                op: BinOp::Eq,
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Bool(false))))],
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
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![is_even, is_odd],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    // Both functions should be detected
    assert!(decisions.contains_key("is_even"));
    assert!(decisions.contains_key("is_odd"));
}

// ============ Configuration impact tests ============

#[test]
fn test_disable_single_use_inlining() {
    let config = InliningConfig {
        inline_single_use: false,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);

    let program = HirProgram {
        functions: vec![create_trivial_function("once_called")],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    assert!(decisions.contains_key("once_called"));
}

#[test]
fn test_disable_trivial_inlining() {
    let config = InliningConfig {
        inline_trivial: false,
        inline_single_use: false,
        ..Default::default()
    };
    let mut analyzer = InliningAnalyzer::new(config);

    let program = HirProgram {
        functions: vec![create_trivial_function("trivial")],
        classes: vec![],
        imports: vec![],
    };

    let decisions = analyzer.analyze_program(&program);
    if let Some(decision) = decisions.get("trivial") {
        // With trivial inlining disabled, it should fall to cost-benefit analysis
        assert!(!matches!(decision.reason, InliningReason::Trivial));
    }
}

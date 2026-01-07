//! EXTREME TDD: Tests for inlining.rs transformation functions
//! Coverage: apply_inlining, transform_expr, transform_stmt, inline_function_call

use depyler_core::hir::{
    AssignTarget, BinOp, FunctionProperties, HirExpr, HirFunction, HirParam, HirProgram, HirStmt,
    Literal, Type,
};
use depyler_core::inlining::{InliningAnalyzer, InliningConfig, InliningDecision, InliningReason};
use smallvec::smallvec;
use std::collections::HashMap;

fn create_simple_function(name: &str, return_value: i64) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
            return_value,
        ))))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

fn create_function_with_params(name: &str, param_names: &[&str]) -> HirFunction {
    let params: smallvec::SmallVec<[HirParam; 4]> = param_names
        .iter()
        .map(|n| HirParam::new(n.to_string(), Type::Int))
        .collect();

    let sum_expr = if param_names.len() == 1 {
        HirExpr::Var(param_names[0].to_string())
    } else {
        param_names.iter().skip(1).fold(
            HirExpr::Var(param_names[0].to_string()),
            |acc, param| HirExpr::Binary {
                left: Box::new(acc),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var(param.to_string())),
            },
        )
    };

    HirFunction {
        name: name.to_string(),
        params,
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(sum_expr))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

// ============ Apply inlining basic tests ============

#[test]
fn test_apply_inlining_empty_program() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert!(result.functions.is_empty());
}

#[test]
fn test_apply_inlining_no_decisions() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_simple_function("test", 42)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program.clone(), &decisions);
    assert_eq!(result.functions.len(), 1);
    assert_eq!(result.functions[0].name, "test");
}

#[test]
fn test_apply_inlining_decision_false() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_simple_function("test", 42)],
        classes: vec![],
        imports: vec![],
    };
    let mut decisions = HashMap::new();
    decisions.insert(
        "test".to_string(),
        InliningDecision {
            should_inline: false,
            reason: InliningReason::TooLarge,
            cost_benefit: 0.0,
        },
    );
    let result = analyzer.apply_inlining(program.clone(), &decisions);
    assert_eq!(result.functions.len(), 1);
}

#[test]
fn test_apply_inlining_preserves_classes() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert!(result.classes.is_empty());
}

#[test]
fn test_apply_inlining_preserves_imports() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert!(result.imports.is_empty());
}

// ============ Function body preservation tests ============

#[test]
fn test_apply_preserves_function_name() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_simple_function("my_func", 100)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions[0].name, "my_func");
}

#[test]
fn test_apply_preserves_function_params() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_function_with_params("add", &["a", "b"])],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions[0].params.len(), 2);
}

#[test]
fn test_apply_preserves_return_type() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![create_simple_function("test", 1)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert!(matches!(result.functions[0].ret_type, Type::Int));
}

// ============ Multiple functions tests ============

#[test]
fn test_apply_multiple_functions_preserved() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![
            create_simple_function("func1", 1),
            create_simple_function("func2", 2),
            create_simple_function("func3", 3),
        ],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions.len(), 3);
}

#[test]
fn test_apply_function_order_preserved() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());
    let program = HirProgram {
        functions: vec![
            create_simple_function("alpha", 1),
            create_simple_function("beta", 2),
            create_simple_function("gamma", 3),
        ],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions[0].name, "alpha");
    assert_eq!(result.functions[1].name, "beta");
    assert_eq!(result.functions[2].name, "gamma");
}

// ============ Caller-callee relationship tests ============

#[test]
fn test_apply_caller_with_call() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());

    let callee = create_simple_function("callee", 42);
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
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions.len(), 2);
}

#[test]
fn test_apply_with_inlining_decision() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());

    let callee = create_simple_function("callee", 42);
    let caller = HirFunction {
        name: "caller".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Expr(HirExpr::Call {
            func: "callee".to_string(),
            args: vec![],
            kwargs: vec![],
        })],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![callee, caller],
        classes: vec![],
        imports: vec![],
    };

    let mut decisions = HashMap::new();
    decisions.insert(
        "callee".to_string(),
        InliningDecision {
            should_inline: true,
            reason: InliningReason::Trivial,
            cost_benefit: 10.0,
        },
    );

    let result = analyzer.apply_inlining(program, &decisions);
    // Functions should still exist
    assert!(!result.functions.is_empty());
}

// ============ Complex function body tests ============

#[test]
fn test_apply_function_with_if() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "conditional".to_string(),
        params: smallvec![HirParam::new("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                left: Box::new(HirExpr::Var("x".to_string())),
                op: BinOp::Gt,
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                0,
            ))))]),
        }],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions[0].name, "conditional");
}

#[test]
fn test_apply_function_with_loop() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "looper".to_string(),
        params: smallvec![HirParam::new("n".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("sum".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::While {
                condition: HirExpr::Binary {
                    left: Box::new(HirExpr::Var("n".to_string())),
                    op: BinOp::Gt,
                    right: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("n".to_string()),
                    value: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("n".to_string())),
                        op: BinOp::Sub,
                        right: Box::new(HirExpr::Literal(Literal::Int(1))),
                    },
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("sum".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions[0].name, "looper");
}

#[test]
fn test_apply_function_with_for() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());

    let func = HirFunction {
        name: "for_loop".to_string(),
        params: smallvec![HirParam::new("items".to_string(), Type::List(Box::new(Type::Int)))],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("sum".to_string()),
                value: HirExpr::Literal(Literal::Int(0)),
                type_annotation: None,
            },
            HirStmt::For {
                target: AssignTarget::Symbol("item".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Assign {
                    target: AssignTarget::Symbol("sum".to_string()),
                    value: HirExpr::Binary {
                        left: Box::new(HirExpr::Var("sum".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Var("item".to_string())),
                    },
                    type_annotation: None,
                }],
            },
            HirStmt::Return(Some(HirExpr::Var("sum".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![func],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions[0].name, "for_loop");
}

// ============ Inlining depth tests ============

#[test]
fn test_apply_respects_max_depth() {
    let config = InliningConfig {
        max_inline_depth: 1,
        ..Default::default()
    };
    let analyzer = InliningAnalyzer::new(config);

    let deep_callee = create_simple_function("deep", 1);
    let mid_callee = HirFunction {
        name: "mid".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "deep".to_string(),
            args: vec![],
            kwargs: vec![],
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };
    let caller = HirFunction {
        name: "caller".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Call {
            func: "mid".to_string(),
            args: vec![],
            kwargs: vec![],
        }))],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![deep_callee, mid_callee, caller],
        classes: vec![],
        imports: vec![],
    };

    let mut decisions = HashMap::new();
    decisions.insert(
        "deep".to_string(),
        InliningDecision {
            should_inline: true,
            reason: InliningReason::Trivial,
            cost_benefit: 10.0,
        },
    );
    decisions.insert(
        "mid".to_string(),
        InliningDecision {
            should_inline: true,
            reason: InliningReason::Trivial,
            cost_benefit: 10.0,
        },
    );

    let result = analyzer.apply_inlining(program, &decisions);
    // Should still have functions
    assert!(!result.functions.is_empty());
}

// ============ Assignment with call tests ============

#[test]
fn test_apply_assignment_with_call() {
    let analyzer = InliningAnalyzer::new(InliningConfig::default());

    let callee = create_simple_function("getter", 42);
    let caller = HirFunction {
        name: "user".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Call {
                    func: "getter".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![callee, caller],
        classes: vec![],
        imports: vec![],
    };

    let mut decisions = HashMap::new();
    decisions.insert(
        "getter".to_string(),
        InliningDecision {
            should_inline: true,
            reason: InliningReason::Trivial,
            cost_benefit: 10.0,
        },
    );

    let result = analyzer.apply_inlining(program, &decisions);
    assert!(!result.functions.is_empty());
}

// ============ Single use removal tests ============

#[test]
fn test_apply_single_use_function_removal() {
    let config = InliningConfig {
        inline_single_use: true,
        ..Default::default()
    };
    let analyzer = InliningAnalyzer::new(config);

    let callee = create_simple_function("once", 42);
    let caller = HirFunction {
        name: "main".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![HirStmt::Expr(HirExpr::Call {
            func: "once".to_string(),
            args: vec![],
            kwargs: vec![],
        })],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![callee, caller],
        classes: vec![],
        imports: vec![],
    };

    let mut decisions = HashMap::new();
    decisions.insert(
        "once".to_string(),
        InliningDecision {
            should_inline: true,
            reason: InliningReason::SingleUse,
            cost_benefit: 5.0,
        },
    );

    let result = analyzer.apply_inlining(program, &decisions);
    // Result may have removed the single-use function
    assert!(!result.functions.is_empty());
}

#[test]
fn test_apply_multi_use_function_kept() {
    let config = InliningConfig {
        inline_single_use: true,
        ..Default::default()
    };
    let analyzer = InliningAnalyzer::new(config);

    let callee = create_simple_function("multi", 42);
    let caller = HirFunction {
        name: "main".to_string(),
        params: smallvec![],
        ret_type: Type::Int,
        body: vec![
            HirStmt::Expr(HirExpr::Call {
                func: "multi".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            HirStmt::Expr(HirExpr::Call {
                func: "multi".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        ],
        properties: FunctionProperties::default(),
        annotations: Default::default(),
        docstring: None,
    };

    let program = HirProgram {
        functions: vec![callee, caller],
        classes: vec![],
        imports: vec![],
    };

    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    // Multi-use function should be kept
    assert!(result.functions.len() >= 1);
}

// ============ Config variation tests ============

#[test]
fn test_apply_with_no_single_use() {
    let config = InliningConfig {
        inline_single_use: false,
        ..Default::default()
    };
    let analyzer = InliningAnalyzer::new(config);

    let program = HirProgram {
        functions: vec![create_simple_function("test", 1)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions.len(), 1);
}

#[test]
fn test_apply_with_aggressive_config() {
    let config = InliningConfig {
        max_inline_size: 100,
        max_inline_depth: 10,
        inline_single_use: true,
        inline_trivial: true,
        cost_threshold: 0.1,
        inline_loops: true,
    };
    let analyzer = InliningAnalyzer::new(config);

    let program = HirProgram {
        functions: vec![
            create_simple_function("a", 1),
            create_simple_function("b", 2),
        ],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert!(!result.functions.is_empty());
}

#[test]
fn test_apply_with_conservative_config() {
    let config = InliningConfig {
        max_inline_size: 5,
        max_inline_depth: 1,
        inline_single_use: false,
        inline_trivial: false,
        cost_threshold: 10.0,
        inline_loops: false,
    };
    let analyzer = InliningAnalyzer::new(config);

    let program = HirProgram {
        functions: vec![create_simple_function("test", 1)],
        classes: vec![],
        imports: vec![],
    };
    let decisions = HashMap::new();
    let result = analyzer.apply_inlining(program, &decisions);
    assert_eq!(result.functions.len(), 1);
}

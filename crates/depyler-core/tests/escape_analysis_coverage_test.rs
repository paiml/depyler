//! Comprehensive coverage tests for escape_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets 60%→95% coverage for escape_analysis module
//! Covers: Try/Except, With, Assert, FunctionDef, Raise, Block statements,
//! MethodCall, Slice, Dict, Comprehension, Lambda, IfExpr, Await, Yield,
//! FString expressions, branch merging, loop state, nested captures,
//! StrategicCloneAnalysis, and analyze_ownership result types.

use depyler_core::escape_analysis::*;
use depyler_core::hir::*;
use smallvec::{smallvec, SmallVec};

// ============================================================================
// Helpers
// ============================================================================

fn make_var(name: &str) -> HirExpr {
    HirExpr::Var(name.to_string())
}

fn make_int(n: i64) -> HirExpr {
    HirExpr::Literal(Literal::Int(n))
}

fn make_str(s: &str) -> HirExpr {
    HirExpr::Literal(Literal::String(s.to_string()))
}

fn make_param(name: &str, ty: Type) -> HirParam {
    HirParam {
        name: name.to_string(),
        ty,
        default: None,
        is_vararg: false,
    }
}

fn make_func(name: &str, params: Vec<HirParam>, body: Vec<HirStmt>) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: SmallVec::from_vec(params),
        ret_type: Type::Unknown,
        body,
        properties: Default::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

fn assign(target: &str, value: HirExpr) -> HirStmt {
    HirStmt::Assign {
        target: AssignTarget::Symbol(target.to_string()),
        value,
        type_annotation: None,
    }
}

fn expr_stmt(expr: HirExpr) -> HirStmt {
    HirStmt::Expr(expr)
}

fn call(func_name: &str, args: Vec<HirExpr>) -> HirExpr {
    HirExpr::Call {
        func: func_name.to_string(),
        args,
        kwargs: vec![],
    }
}

fn method_call(obj: HirExpr, method: &str, args: Vec<HirExpr>) -> HirExpr {
    HirExpr::MethodCall {
        object: Box::new(obj),
        method: method.to_string(),
        args,
        kwargs: vec![],
    }
}

// ============================================================================
// UseAfterMoveAnalysis: Statement coverage
// ============================================================================

#[test]
fn test_uam_try_except_statement() {
    let func = make_func(
        "try_fn",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::Try {
            body: vec![assign("y", make_var("x"))],
            handlers: vec![],
            orelse: Some(vec![expr_stmt(call("print", vec![make_var("x")]))]),
            finalbody: Some(vec![expr_stmt(call("cleanup", vec![]))]),
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_with_statement() {
    let func = make_func(
        "with_fn",
        vec![make_param("f", Type::String)],
        vec![HirStmt::With {
            context: call("open", vec![make_var("f")]),
            target: Some("handle".to_string()),
            body: vec![expr_stmt(call("print", vec![make_var("handle")]))],
            is_async: false,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_with_no_target() {
    let func = make_func(
        "with_fn2",
        vec![],
        vec![HirStmt::With {
            context: call("lock", vec![]),
            target: None,
            body: vec![expr_stmt(call("critical_section", vec![]))],
            is_async: false,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_assert_statement() {
    let func = make_func(
        "assert_fn",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::Assert {
            test: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(make_var("x")),
                right: Box::new(make_int(0)),
            },
            msg: Some(make_str("must be positive")),
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_assert_without_message() {
    let func = make_func(
        "assert_fn2",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::Assert {
            test: make_var("x"),
            msg: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_nested_function_def() {
    let func = make_func(
        "outer",
        vec![make_param("x", Type::Int)],
        vec![
            assign("y", make_int(10)),
            HirStmt::FunctionDef {
                name: "inner".to_string(),
                params: Box::new(smallvec![make_param("z", Type::Int)]),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(make_var("y")),
                    right: Box::new(make_var("z")),
                }))],
                docstring: None,
            },
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_raise_statement() {
    let func = make_func(
        "raise_fn",
        vec![make_param("msg", Type::String)],
        vec![HirStmt::Raise {
            exception: Some(call("ValueError", vec![make_var("msg")])),
            cause: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_raise_with_cause() {
    let func = make_func(
        "raise_cause_fn",
        vec![make_param("e", Type::Unknown)],
        vec![HirStmt::Raise {
            exception: Some(call("RuntimeError", vec![make_str("err")])),
            cause: Some(make_var("e")),
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_block_statement() {
    let func = make_func(
        "block_fn",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::Block(vec![
            assign("y", make_var("x")),
            expr_stmt(call("print", vec![make_var("y")])),
        ])],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_pass_break_continue() {
    let func = make_func(
        "control_fn",
        vec![],
        vec![
            HirStmt::Pass,
            HirStmt::Break { label: None },
            HirStmt::Continue { label: None },
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_return_none() {
    let func = make_func("none_fn", vec![], vec![HirStmt::Return(None)]);

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// UseAfterMoveAnalysis: Expression coverage
// ============================================================================

#[test]
fn test_uam_method_call_expression() {
    let func = make_func(
        "method_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![expr_stmt(method_call(
            make_var("items"),
            "append",
            vec![make_int(42)],
        ))],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_method_call_borrowing() {
    let func = make_func(
        "borrow_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![
            expr_stmt(method_call(make_var("items"), "len", vec![])),
            expr_stmt(method_call(make_var("items"), "len", vec![])),
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_slice_expression() {
    let func = make_func(
        "slice_fn",
        vec![make_param("data", Type::List(Box::new(Type::Int)))],
        vec![assign(
            "sub",
            HirExpr::Slice {
                base: Box::new(make_var("data")),
                start: Some(Box::new(make_int(1))),
                stop: Some(Box::new(make_int(5))),
                step: Some(Box::new(make_int(2))),
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_slice_no_step() {
    let func = make_func(
        "slice2_fn",
        vec![make_param("data", Type::List(Box::new(Type::Int)))],
        vec![assign(
            "sub",
            HirExpr::Slice {
                base: Box::new(make_var("data")),
                start: None,
                stop: Some(Box::new(make_int(3))),
                step: None,
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_dict_expression() {
    let func = make_func(
        "dict_fn",
        vec![make_param("k", Type::String), make_param("v", Type::Int)],
        vec![assign(
            "d",
            HirExpr::Dict(vec![(make_var("k"), make_var("v"))]),
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_list_tuple_set_expressions() {
    let func = make_func(
        "collections_fn",
        vec![make_param("x", Type::Int)],
        vec![
            assign("lst", HirExpr::List(vec![make_var("x"), make_int(1)])),
            assign("tup", HirExpr::Tuple(vec![make_var("x"), make_int(2)])),
            assign("st", HirExpr::Set(vec![make_var("x"), make_int(3)])),
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_index_expression() {
    let func = make_func(
        "index_fn",
        vec![
            make_param("data", Type::List(Box::new(Type::Int))),
            make_param("i", Type::Int),
        ],
        vec![assign(
            "val",
            HirExpr::Index {
                base: Box::new(make_var("data")),
                index: Box::new(make_var("i")),
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_attribute_expression() {
    let func = make_func(
        "attr_fn",
        vec![make_param("obj", Type::Unknown)],
        vec![assign(
            "val",
            HirExpr::Attribute {
                value: Box::new(make_var("obj")),
                attr: "name".to_string(),
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_list_comprehension() {
    let func = make_func(
        "listcomp_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![assign(
            "doubled",
            HirExpr::ListComp {
                element: Box::new(HirExpr::Binary {
                    op: BinOp::Mul,
                    left: Box::new(make_var("x")),
                    right: Box::new(make_int(2)),
                }),
                generators: vec![HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(make_var("items")),
                    conditions: vec![HirExpr::Binary {
                        op: BinOp::Gt,
                        left: Box::new(make_var("x")),
                        right: Box::new(make_int(0)),
                    }],
                }],
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_set_comprehension() {
    let func = make_func(
        "setcomp_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![assign(
            "unique",
            HirExpr::SetComp {
                element: Box::new(make_var("x")),
                generators: vec![HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(make_var("items")),
                    conditions: vec![],
                }],
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_dict_comprehension() {
    let func = make_func(
        "dictcomp_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![assign(
            "mapping",
            HirExpr::DictComp {
                key: Box::new(make_var("x")),
                value: Box::new(HirExpr::Binary {
                    op: BinOp::Mul,
                    left: Box::new(make_var("x")),
                    right: Box::new(make_int(2)),
                }),
                generators: vec![HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(make_var("items")),
                    conditions: vec![],
                }],
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_generator_expression() {
    let func = make_func(
        "genexp_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![assign(
            "gen",
            HirExpr::GeneratorExp {
                element: Box::new(make_var("x")),
                generators: vec![HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(make_var("items")),
                    conditions: vec![HirExpr::Binary {
                        op: BinOp::Gt,
                        left: Box::new(make_var("x")),
                        right: Box::new(make_int(0)),
                    }],
                }],
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_lambda_expression() {
    let func = make_func(
        "lambda_fn",
        vec![make_param("x", Type::Int)],
        vec![assign(
            "f",
            HirExpr::Lambda {
                params: vec!["y".to_string()],
                body: Box::new(HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(make_var("x")),
                    right: Box::new(make_var("y")),
                }),
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_if_expression() {
    let func = make_func(
        "ifexpr_fn",
        vec![make_param("x", Type::Int)],
        vec![assign(
            "y",
            HirExpr::IfExpr {
                test: Box::new(HirExpr::Binary {
                    op: BinOp::Gt,
                    left: Box::new(make_var("x")),
                    right: Box::new(make_int(0)),
                }),
                body: Box::new(make_var("x")),
                orelse: Box::new(make_int(0)),
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_await_expression() {
    let func = make_func(
        "await_fn",
        vec![make_param("coro", Type::Unknown)],
        vec![assign(
            "result",
            HirExpr::Await {
                value: Box::new(make_var("coro")),
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_yield_expression() {
    let func = make_func(
        "yield_fn",
        vec![make_param("x", Type::Int)],
        vec![
            expr_stmt(HirExpr::Yield {
                value: Some(Box::new(make_var("x"))),
            }),
            expr_stmt(HirExpr::Yield { value: None }),
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_named_expr() {
    let func = make_func(
        "walrus_fn",
        vec![make_param("x", Type::Int)],
        vec![expr_stmt(HirExpr::NamedExpr {
            target: "y".to_string(),
            value: Box::new(make_var("x")),
        })],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_fstring_expression() {
    let func = make_func(
        "fstring_fn",
        vec![make_param("name", Type::String)],
        vec![assign(
            "msg",
            HirExpr::FString {
                parts: vec![
                    FStringPart::Literal("Hello, ".to_string()),
                    FStringPart::Expr(Box::new(make_var("name"))),
                    FStringPart::Literal("!".to_string()),
                ],
            },
        )],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_call_with_kwargs() {
    let func = make_func(
        "kwargs_fn",
        vec![make_param("x", Type::Int)],
        vec![expr_stmt(HirExpr::Call {
            func: "print".to_string(),
            args: vec![make_var("x")],
            kwargs: vec![("end".to_string(), make_str("\\n"))],
        })],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_ownership_function_with_kwargs() {
    let func = make_func(
        "push_fn",
        vec![make_param("x", Type::Int)],
        vec![expr_stmt(HirExpr::Call {
            func: "push".to_string(),
            args: vec![],
            kwargs: vec![("value".to_string(), make_var("x"))],
        })],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// UseAfterMoveAnalysis: Branch merging coverage
// ============================================================================

#[test]
fn test_uam_if_without_else() {
    let func = make_func(
        "if_no_else",
        vec![make_param("x", Type::Int)],
        vec![
            HirStmt::If {
                condition: make_var("x"),
                then_body: vec![assign("y", make_var("x"))],
                else_body: None,
            },
            HirStmt::Return(Some(make_var("x"))),
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_while_loop() {
    let func = make_func(
        "while_fn",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(make_var("x")),
                right: Box::new(make_int(0)),
            },
            body: vec![assign("x", make_int(0))],
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_for_loop() {
    let func = make_func(
        "for_fn",
        vec![make_param("items", Type::List(Box::new(Type::Int)))],
        vec![HirStmt::For {
            target: AssignTarget::Symbol("item".to_string()),
            iter: make_var("items"),
            body: vec![expr_stmt(call("print", vec![make_var("item")]))],
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// StrategicCloneAnalysis: Coverage for collect_var_info branches
// ============================================================================

#[test]
fn test_strategic_clone_if_branch() {
    let func = make_func(
        "if_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::If {
                condition: make_var("a"),
                then_body: vec![assign("b", make_var("a"))],
                else_body: Some(vec![assign("c", make_var("a"))]),
            },
            expr_stmt(call("use", vec![make_var("a")])),
            expr_stmt(call("use", vec![make_var("b")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    // Analysis should complete without panic
    let _ = patterns;
}

#[test]
fn test_strategic_clone_while_loop() {
    let func = make_func(
        "while_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::While {
                condition: make_var("a"),
                body: vec![
                    assign("b", make_var("a")),
                    expr_stmt(call("use", vec![make_var("b")])),
                ],
            },
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_for_loop() {
    let func = make_func(
        "for_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: call("range", vec![make_int(10)]),
                body: vec![expr_stmt(call("use", vec![make_var("a")]))],
            },
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_block() {
    let func = make_func(
        "block_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::Block(vec![
                assign("b", make_var("a")),
                expr_stmt(call("use", vec![make_var("b")])),
            ]),
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_return() {
    let func = make_func(
        "return_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::Return(Some(make_var("a"))),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

// ============================================================================
// StrategicCloneAnalysis: collect_uses_in_expr coverage
// ============================================================================

#[test]
fn test_strategic_clone_method_call_usage() {
    let func = make_func(
        "method_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(method_call(make_var("a"), "to_string", vec![])),
            expr_stmt(method_call(make_var("b"), "to_string", vec![])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    assert!(!patterns.is_empty());
}

#[test]
fn test_strategic_clone_unary_usage() {
    let func = make_func(
        "unary_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(make_var("a")),
            }),
            expr_stmt(call("use", vec![make_var("b")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    assert!(!patterns.is_empty());
}

#[test]
fn test_strategic_clone_collection_usage() {
    let func = make_func(
        "coll_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            assign(
                "lst",
                HirExpr::List(vec![make_var("a"), make_var("b")]),
            ),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_tuple_usage() {
    let func = make_func(
        "tuple_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            assign(
                "tup",
                HirExpr::Tuple(vec![make_var("a"), make_var("b")]),
            ),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_set_usage() {
    let func = make_func(
        "set_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            assign(
                "s",
                HirExpr::Set(vec![make_var("a"), make_var("b")]),
            ),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_dict_usage() {
    let func = make_func(
        "dict_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            assign(
                "d",
                HirExpr::Dict(vec![(make_var("a"), make_var("b"))]),
            ),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_index_usage() {
    let func = make_func(
        "index_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(HirExpr::Index {
                base: Box::new(make_var("a")),
                index: Box::new(make_var("b")),
            }),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_attribute_usage() {
    let func = make_func(
        "attr_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(HirExpr::Attribute {
                value: Box::new(make_var("a")),
                attr: "x".to_string(),
            }),
            expr_stmt(call("use", vec![make_var("b")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    assert!(!patterns.is_empty());
}

#[test]
fn test_strategic_clone_ifexpr_usage() {
    let func = make_func(
        "ifexpr_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(HirExpr::IfExpr {
                test: Box::new(make_var("a")),
                body: Box::new(make_var("b")),
                orelse: Box::new(make_int(0)),
            }),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_needs_clone() {
    let func = make_func(
        "needs_clone_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(call("use", vec![make_var("a")])),
            expr_stmt(call("use", vec![make_var("b")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
    let clones = analysis.needs_clone();
    assert!(!clones.is_empty());
    assert_eq!(clones[0], "b");
}

// ============================================================================
// analyze_ownership: Coverage for all OwnershipFix variants
// ============================================================================

#[test]
fn test_analyze_ownership_empty_function() {
    let func = make_func("empty", vec![], vec![]);

    let result = analyze_ownership(&func);
    assert!(result.use_after_move_errors.is_empty());
    assert!(result.aliasing_patterns.is_empty());
    assert!(result.borrow_sites.is_empty());
    assert!(result.clone_sites.is_empty());
    assert!(result.mut_borrow_sites.is_empty());
}

#[test]
fn test_analyze_ownership_with_aliasing() {
    let func = make_func(
        "alias_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            assign("b", make_var("a")),
            expr_stmt(call("use", vec![make_var("a")])),
            expr_stmt(call("use", vec![make_var("b")])),
        ],
    );

    let result = analyze_ownership(&func);
    assert!(!result.aliasing_patterns.is_empty());
}

#[test]
fn test_uam_errors_accessor() {
    let analysis = UseAfterMoveAnalysis::new();
    assert!(analysis.errors().is_empty());
}

// ============================================================================
// Nested captures coverage
// ============================================================================

#[test]
fn test_uam_nested_captures_with_call() {
    let func = make_func(
        "outer",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![make_param("y", Type::Int)]),
            ret_type: Type::Int,
            body: vec![expr_stmt(HirExpr::Call {
                func: "print".to_string(),
                args: vec![make_var("x")],
                kwargs: vec![("sep".to_string(), make_str(","))],
            })],
            docstring: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_nested_captures_with_binary() {
    let func = make_func(
        "outer",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(make_var("x")),
                right: Box::new(make_int(1)),
            }))],
            docstring: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_nested_captures_with_unary() {
    let func = make_func(
        "outer",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(make_var("x")),
            }))],
            docstring: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_nested_captures_expr_stmt() {
    let func = make_func(
        "outer",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            ret_type: Type::None,
            body: vec![expr_stmt(make_var("x"))],
            docstring: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_nested_captures_assign() {
    let func = make_func(
        "outer",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::FunctionDef {
            name: "inner".to_string(),
            params: Box::new(smallvec![]),
            ret_type: Type::None,
            body: vec![assign("local", make_var("x"))],
            docstring: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// Additional coverage: Try with multiple handlers
// ============================================================================

#[test]
fn test_uam_try_with_handlers() {
    let func = make_func(
        "try_handlers",
        vec![make_param("x", Type::Int)],
        vec![HirStmt::Try {
            body: vec![
                assign("result", call("risky_call", vec![make_var("x")])),
            ],
            handlers: vec![
                ExceptHandler {
                    exception_type: Some("ValueError".to_string()),
                    name: Some("e".to_string()),
                    body: vec![expr_stmt(call("handle_value_error", vec![]))],
                },
                ExceptHandler {
                    exception_type: Some("TypeError".to_string()),
                    name: None,
                    body: vec![expr_stmt(call("handle_type_error", vec![]))],
                },
            ],
            orelse: None,
            finalbody: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_try_bare_except() {
    let func = make_func(
        "try_bare",
        vec![],
        vec![HirStmt::Try {
            body: vec![expr_stmt(call("might_fail", vec![]))],
            handlers: vec![ExceptHandler {
                exception_type: None,
                name: None,
                body: vec![HirStmt::Pass],
            }],
            orelse: None,
            finalbody: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// Additional: StrategicClone with try/with/assert/raise
// ============================================================================

#[test]
fn test_strategic_clone_try() {
    let func = make_func(
        "try_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::Try {
                body: vec![expr_stmt(call("use", vec![make_var("a")]))],
                handlers: vec![],
                orelse: None,
                finalbody: None,
            },
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_with() {
    let func = make_func(
        "with_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::With {
                context: call("open", vec![make_str("file.txt")]),
                target: Some("f".to_string()),
                body: vec![expr_stmt(call("use", vec![make_var("a")]))],
                is_async: false,
            },
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_assert() {
    let func = make_func(
        "assert_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::Assert {
                test: make_var("a"),
                msg: Some(make_str("test")),
            },
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_raise() {
    let func = make_func(
        "raise_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::Raise {
                exception: Some(make_var("a")),
                cause: None,
            },
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_funcdef() {
    let func = make_func(
        "outer_fn",
        vec![],
        vec![
            assign("a", make_int(1)),
            HirStmt::FunctionDef {
                name: "inner".to_string(),
                params: Box::new(smallvec![]),
                ret_type: Type::None,
                body: vec![expr_stmt(call("use", vec![make_var("a")]))],
                docstring: None,
            },
            expr_stmt(call("use", vec![make_var("a")])),
        ],
    );

    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

// ============================================================================
// Additional: Raise without exception (bare raise)
// ============================================================================

#[test]
fn test_uam_bare_raise() {
    let func = make_func(
        "bare_raise_fn",
        vec![],
        vec![HirStmt::Raise {
            exception: None,
            cause: None,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// Additional: If with else
// ============================================================================

#[test]
fn test_uam_if_with_else_both_move() {
    let func = make_func(
        "if_else_fn",
        vec![make_param("x", Type::String)],
        vec![
            HirStmt::If {
                condition: call("condition", vec![]),
                then_body: vec![expr_stmt(call("consume", vec![make_var("x")]))],
                else_body: Some(vec![expr_stmt(call("consume", vec![make_var("x")]))]),
            },
        ],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// Additional: MethodCall with kwargs
// ============================================================================

#[test]
fn test_uam_method_call_with_kwargs() {
    let func = make_func(
        "method_kwargs_fn",
        vec![make_param("obj", Type::Unknown)],
        vec![expr_stmt(HirExpr::MethodCall {
            object: Box::new(make_var("obj")),
            method: "configure".to_string(),
            args: vec![],
            kwargs: vec![
                ("timeout".to_string(), make_int(30)),
                ("retry".to_string(), make_int(3)),
            ],
        })],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ============================================================================
// Additional: With async
// ============================================================================

#[test]
fn test_uam_async_with() {
    let func = make_func(
        "async_with_fn",
        vec![],
        vec![HirStmt::With {
            context: call("aiohttp_session", vec![]),
            target: Some("session".to_string()),
            body: vec![expr_stmt(call("fetch", vec![make_var("session")]))],
            is_async: true,
        }],
    );

    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

use super::FunctionAnalyzer;
use crate::hir::{
    AssignTarget, BinOp, ExceptHandler, HirComprehension, HirExpr, HirStmt, Literal, UnaryOp,
};

// ============================================================================
// Helper functions
// ============================================================================

fn make_assign(name: &str, value: HirExpr) -> HirStmt {
    HirStmt::Assign {
        target: AssignTarget::Symbol(name.to_string()),
        value,
        type_annotation: None,
    }
}

fn make_call(func: &str, args: Vec<HirExpr>) -> HirExpr {
    HirExpr::Call {
        func: func.to_string(),
        args,
        kwargs: vec![],
    }
}

fn make_yield(value: Option<HirExpr>) -> HirExpr {
    HirExpr::Yield {
        value: value.map(Box::new),
    }
}

// ============================================================================
// analyze() tests
// ============================================================================

#[test]
fn test_analyze_empty_function() {
    let props = FunctionAnalyzer::analyze(&[]);

    assert!(props.is_pure);
    assert!(props.always_terminates);
    assert!(props.panic_free);
    assert!(!props.can_fail);
    assert!(!props.is_generator);
    assert_eq!(props.error_types.len(), 0);
}

#[test]
fn test_analyze_simple_return() {
    let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_pure);
    assert!(props.always_terminates);
    assert!(props.panic_free);
    assert!(!props.can_fail);
}

// ============================================================================
// Purity tests
// ============================================================================

#[test]
fn test_pure_function_with_arithmetic() {
    let body = vec![
        make_assign(
            "x",
            HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            },
        ),
        HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
    ];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_pure);
}

#[test]
fn test_impure_with_unknown_call() {
    let body = vec![HirStmt::Expr(make_call("print", vec![]))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.is_pure);
}

#[test]
fn test_pure_with_len_call() {
    let body = vec![HirStmt::Expr(make_call(
        "len",
        vec![HirExpr::List(vec![])],
    ))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_pure);
}

#[test]
fn test_pure_with_max_min_sum() {
    let body = vec![
        HirStmt::Expr(make_call("max", vec![])),
        HirStmt::Expr(make_call("min", vec![])),
        HirStmt::Expr(make_call("sum", vec![])),
        HirStmt::Expr(make_call("abs", vec![])),
    ];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_pure);
}

#[test]
fn test_impure_with_call_in_if() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::Expr(make_call("print", vec![]))],
        else_body: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.is_pure);
}

#[test]
fn test_impure_with_call_in_else() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![],
        else_body: Some(vec![HirStmt::Expr(make_call("print", vec![]))]),
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.is_pure);
}

#[test]
fn test_impure_with_call_in_while() {
    let body = vec![HirStmt::While {
        condition: HirExpr::Literal(Literal::Bool(false)),
        body: vec![HirStmt::Expr(make_call("print", vec![]))],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.is_pure);
}

#[test]
fn test_impure_with_call_in_for() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::List(vec![]),
        body: vec![HirStmt::Expr(make_call("print", vec![]))],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.is_pure);
}

// ============================================================================
// Termination tests
// ============================================================================

#[test]
fn test_always_terminates_no_loops() {
    let body = vec![
        make_assign("x", HirExpr::Literal(Literal::Int(1))),
        HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
    ];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_while_loop_may_not_terminate() {
    let body = vec![HirStmt::While {
        condition: HirExpr::Literal(Literal::Bool(true)),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.always_terminates);
}

#[test]
fn test_for_over_list_terminates() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_for_over_range_terminates() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: make_call("range", vec![HirExpr::Literal(Literal::Int(10))]),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_for_over_tuple_terminates() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::Tuple(vec![]),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_for_over_dict_terminates() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("k".to_string()),
        iter: HirExpr::Dict(vec![]),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_for_over_enumerate_terminates() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: make_call("enumerate", vec![HirExpr::List(vec![])]),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_for_over_zip_terminates() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("pair".to_string()),
        iter: make_call("zip", vec![HirExpr::List(vec![]), HirExpr::List(vec![])]),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.always_terminates);
}

#[test]
fn test_for_over_unknown_iterator_may_not_terminate() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("x".to_string()),
        iter: HirExpr::Var("some_iterable".to_string()),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.always_terminates);
}

// ============================================================================
// Panic-free tests
// ============================================================================

#[test]
fn test_panic_free_simple() {
    let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.panic_free);
}

#[test]
fn test_not_panic_free_with_index() {
    let body = vec![HirStmt::Expr(HirExpr::Index {
        base: Box::new(HirExpr::List(vec![])),
        index: Box::new(HirExpr::Literal(Literal::Int(0))),
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

#[test]
fn test_not_panic_free_with_division() {
    let body = vec![make_assign(
        "x",
        HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Var("n".to_string())),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

#[test]
fn test_not_panic_free_with_floor_div() {
    let body = vec![make_assign(
        "x",
        HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Var("n".to_string())),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

#[test]
fn test_not_panic_free_with_mod() {
    let body = vec![make_assign(
        "x",
        HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Var("n".to_string())),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

#[test]
fn test_not_panic_free_with_raise() {
    let body = vec![HirStmt::Raise {
        exception: Some(make_call(
            "ValueError",
            vec![HirExpr::Literal(Literal::String("error".to_string()))],
        )),
        cause: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

#[test]
fn test_not_panic_free_index_in_if() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::Expr(HirExpr::Index {
            base: Box::new(HirExpr::List(vec![])),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        })],
        else_body: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

#[test]
fn test_not_panic_free_condition_with_index() {
    let body = vec![HirStmt::While {
        condition: HirExpr::Index {
            base: Box::new(HirExpr::List(vec![])),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        },
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.panic_free);
}

// ============================================================================
// can_fail tests
// ============================================================================

#[test]
fn test_can_fail_with_raise() {
    let body = vec![HirStmt::Raise {
        exception: Some(make_call("ValueError", vec![])),
        cause: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
    assert!(props.error_types.contains(&"ValueError".to_string()));
}

#[test]
fn test_can_fail_with_index_error() {
    let body = vec![HirStmt::Expr(HirExpr::Index {
        base: Box::new(HirExpr::List(vec![])),
        index: Box::new(HirExpr::Literal(Literal::Int(0))),
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
    assert!(props.error_types.contains(&"IndexError".to_string()));
}

#[test]
fn test_can_fail_with_division() {
    let body = vec![make_assign(
        "x",
        HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Var("n".to_string())),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
    assert!(props.error_types.contains(&"ZeroDivisionError".to_string()));
}

#[test]
fn test_can_fail_with_open_call() {
    let body = vec![HirStmt::Expr(make_call(
        "open",
        vec![HirExpr::Literal(Literal::String("file.txt".to_string()))],
    ))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
    assert!(props.error_types.contains(&"std::io::Error".to_string()));
}

#[test]
fn test_can_fail_int_string_parsing() {
    // int("123") can fail
    let body = vec![HirStmt::Expr(make_call(
        "int",
        vec![HirExpr::Literal(Literal::String("123".to_string()))],
    ))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
    assert!(props.error_types.contains(&"ValueError".to_string()));
}

#[test]
fn test_cannot_fail_int_variable_cast() {
    // int(x) where x is not a string literal - safe cast
    let body = vec![HirStmt::Expr(make_call(
        "int",
        vec![HirExpr::Var("x".to_string())],
    ))];
    let props = FunctionAnalyzer::analyze(&body);

    // Safe cast doesn't fail
    assert!(!props.can_fail);
}

#[test]
fn test_can_fail_int_with_base() {
    // int("ff", 16) can fail
    let body = vec![HirStmt::Expr(make_call(
        "int",
        vec![
            HirExpr::Literal(Literal::String("ff".to_string())),
            HirExpr::Literal(Literal::Int(16)),
        ],
    ))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
}

#[test]
fn test_can_fail_with_statement_open() {
    let body = vec![HirStmt::With {
        context: make_call(
            "open",
            vec![HirExpr::Literal(Literal::String("file.txt".to_string()))],
        ),
        target: Some("f".to_string()),
        body: vec![],
        is_async: false,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
    assert!(props.error_types.contains(&"std::io::Error".to_string()));
}

#[test]
fn test_can_fail_if_expr() {
    let body = vec![make_assign(
        "x",
        HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(make_call(
                "open",
                vec![HirExpr::Literal(Literal::String("file.txt".to_string()))],
            )),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
}

#[test]
fn test_can_fail_nested_binary() {
    let body = vec![make_assign(
        "x",
        HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Binary {
                op: BinOp::Div, // This can fail
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                right: Box::new(HirExpr::Var("n".to_string())),
            }),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.can_fail);
}

// ============================================================================
// Try/except tests
// ============================================================================

#[test]
fn test_try_except_catches_exception() {
    let body = vec![HirStmt::Try {
        body: vec![HirStmt::Raise {
            exception: Some(make_call("ValueError", vec![])),
            cause: None,
        }],
        handlers: vec![ExceptHandler {
            exception_type: Some("ValueError".to_string()),
            name: None,
            body: vec![],
        }],
        orelse: None,
        finalbody: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    // Even though exception is caught, body_fail is true from the raise
    // (this is conservative - the analyzer marks the body as can_fail)
    assert!(props.can_fail);
    // Error type should be tracked for struct generation
    assert!(props.error_types.contains(&"ValueError".to_string()));
}

#[test]
fn test_try_except_with_uncaught_exception() {
    let body = vec![HirStmt::Try {
        body: vec![],
        handlers: vec![ExceptHandler {
            exception_type: Some("ValueError".to_string()),
            name: None,
            body: vec![HirStmt::Raise {
                exception: Some(make_call("TypeError", vec![])),
                cause: None,
            }],
        }],
        orelse: None,
        finalbody: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    // Handler raises uncaught exception
    assert!(props.can_fail);
    assert!(props.error_types.contains(&"TypeError".to_string()));
}

// ============================================================================
// Generator tests
// ============================================================================

#[test]
fn test_is_generator_simple_yield() {
    let body = vec![HirStmt::Expr(make_yield(Some(HirExpr::Literal(
        Literal::Int(1),
    ))))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_none() {
    let body = vec![HirStmt::Expr(make_yield(None))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_for() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: make_call("range", vec![HirExpr::Literal(Literal::Int(10))]),
        body: vec![HirStmt::Expr(make_yield(Some(HirExpr::Var(
            "i".to_string(),
        ))))],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_while() {
    let body = vec![HirStmt::While {
        condition: HirExpr::Literal(Literal::Bool(true)),
        body: vec![HirStmt::Expr(make_yield(None))],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_if() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::Expr(make_yield(None))],
        else_body: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_else() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![],
        else_body: Some(vec![HirStmt::Expr(make_yield(None))]),
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_try() {
    let body = vec![HirStmt::Try {
        body: vec![HirStmt::Expr(make_yield(None))],
        handlers: vec![],
        orelse: None,
        finalbody: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_handler() {
    let body = vec![HirStmt::Try {
        body: vec![],
        handlers: vec![ExceptHandler {
            exception_type: None,
            name: None,
            body: vec![HirStmt::Expr(make_yield(None))],
        }],
        orelse: None,
        finalbody: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_finally() {
    let body = vec![HirStmt::Try {
        body: vec![],
        handlers: vec![],
        orelse: None,
        finalbody: Some(vec![HirStmt::Expr(make_yield(None))]),
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_with() {
    let body = vec![HirStmt::With {
        context: HirExpr::Var("ctx".to_string()),
        target: Some("f".to_string()),
        body: vec![HirStmt::Expr(make_yield(None))],
        is_async: false,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_assign() {
    let body = vec![make_assign(
        "x",
        HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(make_yield(Some(HirExpr::Literal(Literal::Int(1))))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        },
    )];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_is_generator_yield_in_return() {
    let body = vec![HirStmt::Return(Some(make_yield(Some(HirExpr::Literal(
        Literal::Int(1),
    )))))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_not_generator_without_yield() {
    let body = vec![
        make_assign("x", HirExpr::Literal(Literal::Int(1))),
        HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
    ];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(!props.is_generator);
}

// ============================================================================
// Stack depth tests
// ============================================================================

#[test]
fn test_stack_depth_flat() {
    let body = vec![
        make_assign("x", HirExpr::Literal(Literal::Int(1))),
        make_assign("y", HirExpr::Literal(Literal::Int(2))),
    ];
    let props = FunctionAnalyzer::analyze(&body);

    assert_eq!(props.max_stack_depth, Some(0));
}

#[test]
fn test_stack_depth_if() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![make_assign("x", HirExpr::Literal(Literal::Int(1)))],
        else_body: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert_eq!(props.max_stack_depth, Some(1));
}

#[test]
fn test_stack_depth_nested_if() {
    let body = vec![HirStmt::If {
        condition: HirExpr::Literal(Literal::Bool(true)),
        then_body: vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![],
            else_body: None,
        }],
        else_body: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert_eq!(props.max_stack_depth, Some(2));
}

#[test]
fn test_stack_depth_for_loop() {
    let body = vec![HirStmt::For {
        target: AssignTarget::Symbol("i".to_string()),
        iter: HirExpr::List(vec![]),
        body: vec![make_assign("x", HirExpr::Var("i".to_string()))],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert_eq!(props.max_stack_depth, Some(1));
}

#[test]
fn test_stack_depth_while_loop() {
    let body = vec![HirStmt::While {
        condition: HirExpr::Literal(Literal::Bool(false)),
        body: vec![],
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert_eq!(props.max_stack_depth, Some(1));
}

// ============================================================================
// Complex scenario tests
// ============================================================================

#[test]
fn test_complex_generator_function() {
    // def fibonacci(n):
    //     a, b = 0, 1
    //     while a < n:
    //         yield a
    //         a, b = b, a + b
    let body = vec![
        make_assign("a", HirExpr::Literal(Literal::Int(0))),
        make_assign("b", HirExpr::Literal(Literal::Int(1))),
        HirStmt::While {
            condition: HirExpr::Binary {
                op: BinOp::Lt,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("n".to_string())),
            },
            body: vec![
                HirStmt::Expr(make_yield(Some(HirExpr::Var("a".to_string())))),
                make_assign(
                    "temp",
                    HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("a".to_string())),
                        right: Box::new(HirExpr::Var("b".to_string())),
                    },
                ),
            ],
        },
    ];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
    assert!(!props.always_terminates); // Has while loop
    assert!(props.panic_free);
}

#[test]
fn test_yield_in_list_comprehension() {
    let body = vec![HirStmt::Expr(HirExpr::ListComp {
        element: Box::new(make_yield(Some(HirExpr::Var("x".to_string())))),
        generators: vec![HirComprehension {
            target: "x".to_string(),
            iter: Box::new(HirExpr::List(vec![])),
            conditions: vec![],
        }],
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_yield_in_method_call() {
    let body = vec![HirStmt::Expr(HirExpr::MethodCall {
        object: Box::new(make_yield(Some(HirExpr::Literal(Literal::Int(1))))),
        method: "some_method".to_string(),
        args: vec![],
        kwargs: vec![],
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_yield_in_dict() {
    let body = vec![HirStmt::Expr(HirExpr::Dict(vec![(
        HirExpr::Literal(Literal::String("key".to_string())),
        make_yield(Some(HirExpr::Literal(Literal::Int(1)))),
    )]))];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_yield_in_unary() {
    let body = vec![HirStmt::Expr(HirExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(make_yield(Some(HirExpr::Literal(Literal::Bool(true))))),
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_yield_in_attribute() {
    let body = vec![HirStmt::Expr(HirExpr::Attribute {
        value: Box::new(make_yield(Some(HirExpr::Var("obj".to_string())))),
        attr: "field".to_string(),
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_yield_in_await() {
    let body = vec![HirStmt::Expr(HirExpr::Await {
        value: Box::new(make_yield(Some(HirExpr::Literal(Literal::Int(1))))),
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_yield_in_borrow() {
    let body = vec![HirStmt::Expr(HirExpr::Borrow {
        expr: Box::new(make_yield(Some(HirExpr::Literal(Literal::Int(1))))),
        mutable: false,
    })];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.is_generator);
}

#[test]
fn test_multiple_error_types_deduped() {
    let body = vec![
        HirStmt::Raise {
            exception: Some(make_call("ValueError", vec![])),
            cause: None,
        },
        HirStmt::Raise {
            exception: Some(make_call("ValueError", vec![])),
            cause: None,
        },
        HirStmt::Raise {
            exception: Some(make_call("TypeError", vec![])),
            cause: None,
        },
    ];
    let props = FunctionAnalyzer::analyze(&body);

    // Should be deduplicated
    let value_error_count = props
        .error_types
        .iter()
        .filter(|e| *e == "ValueError")
        .count();
    assert_eq!(value_error_count, 1);
}

#[test]
fn test_extract_exception_var() {
    let body = vec![HirStmt::Raise {
        exception: Some(HirExpr::Var("my_error".to_string())),
        cause: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.error_types.contains(&"my_error".to_string()));
}

#[test]
fn test_extract_exception_method_call() {
    let body = vec![HirStmt::Raise {
        exception: Some(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("argparse".to_string())),
            method: "ArgumentTypeError".to_string(),
            args: vec![],
            kwargs: vec![],
        }),
        cause: None,
    }];
    let props = FunctionAnalyzer::analyze(&body);

    assert!(props.error_types.contains(&"ArgumentTypeError".to_string()));
}

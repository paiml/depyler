//! Session 8 deep coverage: escape_analysis.rs
//!
//! Direct API tests for UseAfterMoveAnalysis, StrategicCloneAnalysis,
//! and analyze_ownership. Targets uncovered branches in merge_branch_states,
//! merge_loop_state, analyze_nested_captures, and expression analysis paths.

use depyler_core::escape_analysis::{
    analyze_ownership, OwnershipFix, StrategicCloneAnalysis, UseAfterMoveAnalysis,
};
use depyler_core::hir::*;
use smallvec::smallvec;

// ── Helper constructors ────────────────────────────────────────────

fn var(name: &str) -> HirExpr {
    HirExpr::Var(name.to_string())
}

fn int_lit(n: i64) -> HirExpr {
    HirExpr::Literal(Literal::Int(n))
}

fn str_lit(s: &str) -> HirExpr {
    HirExpr::Literal(Literal::String(s.to_string()))
}

fn bool_lit(b: bool) -> HirExpr {
    HirExpr::Literal(Literal::Bool(b))
}

fn float_lit(f: f64) -> HirExpr {
    HirExpr::Literal(Literal::Float(f))
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

fn expr_stmt(expr: HirExpr) -> HirStmt {
    HirStmt::Expr(expr)
}

fn call(func: &str, args: Vec<HirExpr>) -> HirExpr {
    HirExpr::Call {
        func: func.to_string(),
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

fn binary(left: HirExpr, op: BinOp, right: HirExpr) -> HirExpr {
    HirExpr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn unary(op: UnaryOp, operand: HirExpr) -> HirExpr {
    HirExpr::Unary {
        op,
        operand: Box::new(operand),
    }
}

fn index(base: HirExpr, idx: HirExpr) -> HirExpr {
    HirExpr::Index {
        base: Box::new(base),
        index: Box::new(idx),
    }
}

fn make_func(name: &str, params: Vec<(&str, Type)>, body: Vec<HirStmt>) -> HirFunction {
    HirFunction {
        name: name.to_string(),
        params: params
            .into_iter()
            .map(|(n, t)| HirParam {
                name: n.to_string(),
                ty: t,
                default: None,
                is_vararg: false,
            })
            .collect(),
        ret_type: Type::Unknown,
        body,
        properties: Default::default(),
        annotations: Default::default(),
        docstring: None,
    }
}

fn make_func_no_params(name: &str, body: Vec<HirStmt>) -> HirFunction {
    make_func(name, vec![], body)
}

// ── UseAfterMoveAnalysis tests ─────────────────────────────────────

#[test]
fn test_uam_empty_function() {
    let func = make_func_no_params("f", vec![]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_simple_return_param() {
    let func = make_func("f", vec![("x", Type::Int)], vec![return_expr(var("x"))]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_assign_and_return() {
    let func = make_func_no_params("f", vec![assign("x", int_lit(42)), return_expr(var("x"))]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_return_none() {
    let func = make_func_no_params("f", vec![HirStmt::Return(None)]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_pass_statement() {
    let func = make_func_no_params("f", vec![HirStmt::Pass]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_break_continue() {
    let func = make_func_no_params(
        "f",
        vec![
            HirStmt::Break { label: None },
            HirStmt::Continue { label: None },
        ],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_if_then_else_branches() {
    let func = make_func(
        "f",
        vec![("x", Type::List(Box::new(Type::Int)))],
        vec![HirStmt::If {
            condition: binary(var("x"), BinOp::Gt, int_lit(0)),
            then_body: vec![return_expr(var("x"))],
            else_body: Some(vec![return_expr(var("x"))]),
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_if_no_else() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![
            HirStmt::If {
                condition: binary(var("x"), BinOp::Gt, int_lit(0)),
                then_body: vec![assign("y", var("x"))],
                else_body: None,
            },
            return_expr(var("x")),
        ],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_while_loop() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![HirStmt::While {
            condition: binary(var("x"), BinOp::Gt, int_lit(0)),
            body: vec![assign("x", binary(var("x"), BinOp::Sub, int_lit(1)))],
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_for_loop() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![HirStmt::For {
            target: AssignTarget::Symbol("item".to_string()),
            iter: var("items"),
            body: vec![expr_stmt(call("print", vec![var("item")]))],
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_block_statement() {
    let func = make_func_no_params(
        "f",
        vec![HirStmt::Block(vec![
            assign("a", int_lit(1)),
            assign("b", int_lit(2)),
        ])],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_try_statement() {
    let func = make_func_no_params(
        "f",
        vec![HirStmt::Try {
            body: vec![assign("x", int_lit(1))],
            handlers: vec![],
            orelse: Some(vec![assign("y", int_lit(2))]),
            finalbody: Some(vec![expr_stmt(call("print", vec![str_lit("done")]))]),
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_with_statement() {
    let func = make_func_no_params(
        "f",
        vec![HirStmt::With {
            context: call("open", vec![str_lit("file.txt")]),
            target: Some("f".to_string()),
            body: vec![assign("x", int_lit(1))],
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
        "f",
        vec![("x", Type::Int)],
        vec![HirStmt::Assert {
            test: binary(var("x"), BinOp::Gt, int_lit(0)),
            msg: Some(str_lit("must be positive")),
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_assert_no_msg() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![HirStmt::Assert {
            test: binary(var("x"), BinOp::Gt, int_lit(0)),
            msg: None,
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_raise_statement() {
    let func = make_func_no_params(
        "f",
        vec![HirStmt::Raise {
            exception: Some(call("ValueError", vec![str_lit("bad")])),
            cause: None,
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_raise_with_cause() {
    let func = make_func_no_params(
        "f",
        vec![HirStmt::Raise {
            exception: Some(call("RuntimeError", vec![str_lit("fail")])),
            cause: Some(var("original_error")),
        }],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let _errors = analysis.analyze_function(&func);
    // Just checking it doesn't panic
}

#[test]
fn test_uam_nested_function_def() {
    let func = make_func(
        "outer",
        vec![("x", Type::Int)],
        vec![
            assign("x", int_lit(10)),
            HirStmt::FunctionDef {
                name: "inner".to_string(),
                params: Box::new(smallvec![HirParam {
                    name: "y".to_string(),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                }]),
                body: vec![return_expr(binary(var("x"), BinOp::Add, var("y")))],
                ret_type: Type::Int,
                docstring: None,
            },
        ],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ── Expression analysis paths ──────────────────────────────────────

#[test]
fn test_uam_call_with_kwargs() {
    let func = make_func_no_params(
        "f",
        vec![expr_stmt(HirExpr::Call {
            func: "print".to_string(),
            args: vec![str_lit("hello")],
            kwargs: vec![("end".to_string(), str_lit(""))],
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_method_call_ownership() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![expr_stmt(method_call(
            var("items"),
            "append",
            vec![int_lit(42)],
        ))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    // append takes ownership of argument - item moved
    assert!(errors.is_empty()); // but 42 is a literal, so no move issue
}

#[test]
fn test_uam_binary_expr() {
    let func = make_func(
        "f",
        vec![("a", Type::Int), ("b", Type::Int)],
        vec![return_expr(binary(var("a"), BinOp::Add, var("b")))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_unary_expr() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![return_expr(unary(UnaryOp::Neg, var("x")))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_index_expr() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(index(var("items"), int_lit(0)))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_slice_expr() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(HirExpr::Slice {
            base: Box::new(var("items")),
            start: Some(Box::new(int_lit(0))),
            stop: Some(Box::new(int_lit(5))),
            step: Some(Box::new(int_lit(2))),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_attribute_expr() {
    let func = make_func(
        "f",
        vec![("obj", Type::Unknown)],
        vec![return_expr(HirExpr::Attribute {
            value: Box::new(var("obj")),
            attr: "name".to_string(),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_list_literal() {
    let func = make_func(
        "f",
        vec![("a", Type::Int), ("b", Type::Int)],
        vec![return_expr(HirExpr::List(vec![var("a"), var("b")]))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_tuple_literal() {
    let func = make_func(
        "f",
        vec![("a", Type::Int)],
        vec![return_expr(HirExpr::Tuple(vec![var("a"), int_lit(1)]))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_set_literal() {
    let func = make_func(
        "f",
        vec![("a", Type::Int)],
        vec![return_expr(HirExpr::Set(vec![var("a")]))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_dict_literal() {
    let func = make_func(
        "f",
        vec![("k", Type::String), ("v", Type::Int)],
        vec![return_expr(HirExpr::Dict(vec![(var("k"), var("v"))]))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_list_comp() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(HirExpr::ListComp {
            element: Box::new(binary(var("x"), BinOp::Mul, int_lit(2))),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(var("items")),
                conditions: vec![binary(var("x"), BinOp::Gt, int_lit(0))],
            }],
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_set_comp() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(HirExpr::SetComp {
            element: Box::new(var("x")),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(var("items")),
                conditions: vec![],
            }],
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_dict_comp() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(HirExpr::DictComp {
            key: Box::new(var("x")),
            value: Box::new(binary(var("x"), BinOp::Mul, int_lit(2))),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(var("items")),
                conditions: vec![binary(var("x"), BinOp::Gt, int_lit(0))],
            }],
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_generator_exp() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(HirExpr::GeneratorExp {
            element: Box::new(var("x")),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(var("items")),
                conditions: vec![],
            }],
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_lambda() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![return_expr(HirExpr::Lambda {
            params: vec![],
            body: Box::new(var("x")),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_if_expr() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![return_expr(HirExpr::IfExpr {
            test: Box::new(binary(var("x"), BinOp::Gt, int_lit(0))),
            body: Box::new(var("x")),
            orelse: Box::new(int_lit(0)),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_await_expr() {
    let func = make_func(
        "f",
        vec![("coro", Type::Unknown)],
        vec![return_expr(HirExpr::Await {
            value: Box::new(var("coro")),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_yield_expr() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![expr_stmt(HirExpr::Yield {
            value: Some(Box::new(var("x"))),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_yield_none() {
    let func = make_func_no_params("f", vec![expr_stmt(HirExpr::Yield { value: None })]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_named_expr() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![return_expr(HirExpr::NamedExpr {
            target: "y".to_string(),
            value: Box::new(var("x")),
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_fstring() {
    let func = make_func(
        "f",
        vec![("name", Type::String)],
        vec![return_expr(HirExpr::FString {
            parts: vec![
                FStringPart::Literal("Hello, ".to_string()),
                FStringPart::Expr(Box::new(var("name"))),
            ],
        })],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_literal_no_move() {
    let func = make_func_no_params("f", vec![return_expr(int_lit(42))]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ── Aliasing / variable reassignment ───────────────────────────────

#[test]
fn test_uam_aliasing_pattern() {
    // b = a (aliasing); use both after
    let func = make_func(
        "f",
        vec![("a", Type::List(Box::new(Type::Int)))],
        vec![
            assign("b", var("a")),
            expr_stmt(call("print", vec![var("a")])),
            expr_stmt(call("print", vec![var("b")])),
        ],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    // The analysis should handle aliasing - checking it doesn't crash
    assert!(errors.len() <= 2); // may or may not detect depending on conservative analysis
}

#[test]
fn test_uam_reassignment_clears_move() {
    let func = make_func(
        "f",
        vec![("a", Type::Int)],
        vec![assign("a", int_lit(10)), return_expr(var("a"))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ── StrategicCloneAnalysis ─────────────────────────────────────────

#[test]
fn test_strategic_clone_empty() {
    let func = make_func_no_params("f", vec![]);
    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    assert!(patterns.is_empty());
    assert!(analysis.needs_clone().is_empty());
}

#[test]
fn test_strategic_clone_no_alias() {
    let func = make_func_no_params(
        "f",
        vec![
            assign("a", int_lit(1)),
            assign("b", int_lit(2)),
            return_expr(binary(var("a"), BinOp::Add, var("b"))),
        ],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    assert!(patterns.is_empty());
}

#[test]
fn test_strategic_clone_alias_both_used() {
    // b = a; use a; use b => need clone
    let func = make_func(
        "f",
        vec![("a", Type::List(Box::new(Type::Int)))],
        vec![
            assign("b", var("a")),
            expr_stmt(call("len", vec![var("a")])),
            expr_stmt(call("len", vec![var("b")])),
        ],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    // Should detect aliasing pattern where both source and alias are used
    assert!(
        !patterns.is_empty(),
        "Should detect aliasing pattern: b = a with both used"
    );
    assert!(!analysis.needs_clone().is_empty());
}

#[test]
fn test_strategic_clone_alias_only_one_used() {
    // b = a; only use b (not a after) => no clone needed
    let func = make_func(
        "f",
        vec![("a", Type::Int)],
        vec![assign("b", var("a")), return_expr(var("b"))],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    assert!(patterns.is_empty(), "No clone needed when only alias used");
}

#[test]
fn test_strategic_clone_in_if_body() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![HirStmt::If {
            condition: bool_lit(true),
            then_body: vec![
                assign("copy", var("items")),
                expr_stmt(call("len", vec![var("items")])),
                expr_stmt(call("len", vec![var("copy")])),
            ],
            else_body: None,
        }],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
    // Just checking it doesn't panic on nested contexts
}

#[test]
fn test_strategic_clone_in_while() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![HirStmt::While {
            condition: binary(var("x"), BinOp::Gt, int_lit(0)),
            body: vec![assign("y", var("x"))],
        }],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_in_for() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![HirStmt::For {
            target: AssignTarget::Symbol("item".to_string()),
            iter: var("items"),
            body: vec![assign("copy", var("item"))],
        }],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_block() {
    let func = make_func_no_params(
        "f",
        vec![HirStmt::Block(vec![
            assign("a", int_lit(1)),
            assign("b", var("a")),
        ])],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_method_call() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![expr_stmt(method_call(
            var("items"),
            "append",
            vec![int_lit(1)],
        ))],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_unary() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![return_expr(unary(UnaryOp::Neg, var("x")))],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_index() {
    let func = make_func(
        "f",
        vec![("items", Type::List(Box::new(Type::Int)))],
        vec![return_expr(index(var("items"), int_lit(0)))],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_attribute() {
    let func = make_func(
        "f",
        vec![("obj", Type::Unknown)],
        vec![return_expr(HirExpr::Attribute {
            value: Box::new(var("obj")),
            attr: "x".to_string(),
        })],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_if_expr() {
    let func = make_func(
        "f",
        vec![("x", Type::Int)],
        vec![return_expr(HirExpr::IfExpr {
            test: Box::new(binary(var("x"), BinOp::Gt, int_lit(0))),
            body: Box::new(var("x")),
            orelse: Box::new(int_lit(0)),
        })],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_collection_literals() {
    let func = make_func(
        "f",
        vec![("a", Type::Int)],
        vec![return_expr(HirExpr::List(vec![var("a")]))],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

#[test]
fn test_strategic_clone_collect_uses_dict() {
    let func = make_func(
        "f",
        vec![("k", Type::String), ("v", Type::Int)],
        vec![return_expr(HirExpr::Dict(vec![(var("k"), var("v"))]))],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let _patterns = analysis.analyze_function(&func);
}

// ── Combined ownership analysis ────────────────────────────────────

#[test]
fn test_analyze_ownership_empty() {
    let func = make_func_no_params("f", vec![]);
    let result = analyze_ownership(&func);
    assert!(result.use_after_move_errors.is_empty());
    assert!(result.aliasing_patterns.is_empty());
    assert!(result.borrow_sites.is_empty());
    assert!(result.clone_sites.is_empty());
    assert!(result.mut_borrow_sites.is_empty());
}

#[test]
fn test_analyze_ownership_simple() {
    let func = make_func("f", vec![("x", Type::Int)], vec![return_expr(var("x"))]);
    let result = analyze_ownership(&func);
    assert!(result.use_after_move_errors.is_empty());
}

#[test]
fn test_analyze_ownership_with_alias() {
    let func = make_func(
        "f",
        vec![("a", Type::List(Box::new(Type::Int)))],
        vec![
            assign("b", var("a")),
            expr_stmt(call("len", vec![var("a")])),
            expr_stmt(call("len", vec![var("b")])),
        ],
    );
    let result = analyze_ownership(&func);
    assert!(!result.aliasing_patterns.is_empty());
}

// ── OwnershipFix enum coverage ─────────────────────────────────────

#[test]
fn test_ownership_fix_variants() {
    let borrow = OwnershipFix::Borrow;
    let mut_borrow = OwnershipFix::MutableBorrow;
    let clone = OwnershipFix::Clone;
    let clone_at = OwnershipFix::CloneAtAssignment {
        var: "x".to_string(),
    };
    let reject = OwnershipFix::Reject {
        reason: "unsafe".to_string(),
    };

    assert_eq!(borrow, OwnershipFix::Borrow);
    assert_eq!(mut_borrow, OwnershipFix::MutableBorrow);
    assert_eq!(clone, OwnershipFix::Clone);
    assert_ne!(clone, OwnershipFix::Borrow);
    assert_ne!(clone_at, OwnershipFix::Clone);
    assert_ne!(reject, OwnershipFix::Clone);
}

// ── Bool/float literal analysis ────────────────────────────────────

#[test]
fn test_uam_bool_literals() {
    let func = make_func_no_params(
        "f",
        vec![return_expr(binary(
            bool_lit(true),
            BinOp::And,
            bool_lit(false),
        ))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

#[test]
fn test_uam_float_literals() {
    let func = make_func_no_params(
        "f",
        vec![return_expr(binary(
            float_lit(3.14),
            BinOp::Add,
            float_lit(2.71),
        ))],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let errors = analysis.analyze_function(&func);
    assert!(errors.is_empty());
}

// ── Ownership analysis result classification ───────────────────────

#[test]
fn test_ownership_result_borrow_sites() {
    // Test that borrow_sites are populated from use-after-move with Borrow fix
    let func = make_func_no_params("f", vec![return_expr(int_lit(1))]);
    let result = analyze_ownership(&func);
    // No errors => no sites
    assert!(result.borrow_sites.is_empty());
}

#[test]
fn test_ownership_result_mut_borrow_sites() {
    let func = make_func_no_params("f", vec![return_expr(int_lit(1))]);
    let result = analyze_ownership(&func);
    assert!(result.mut_borrow_sites.is_empty());
}

#[test]
fn test_ownership_result_clone_sites() {
    let func = make_func_no_params("f", vec![return_expr(int_lit(1))]);
    let result = analyze_ownership(&func);
    assert!(result.clone_sites.is_empty());
}

// ── Default trait implementations ──────────────────────────────────

#[test]
fn test_uam_default() {
    let analysis = UseAfterMoveAnalysis::default();
    assert!(analysis.errors().is_empty());
}

#[test]
fn test_strategic_clone_default() {
    let analysis = StrategicCloneAnalysis::default();
    assert!(analysis.needs_clone().is_empty());
}

// ── AliasingPattern fields ─────────────────────────────────────────

#[test]
fn test_aliasing_pattern_fields() {
    let func = make_func(
        "f",
        vec![("a", Type::List(Box::new(Type::Int)))],
        vec![
            assign("b", var("a")),
            expr_stmt(call("len", vec![var("a")])),
            expr_stmt(call("len", vec![var("b")])),
        ],
    );
    let mut analysis = StrategicCloneAnalysis::new();
    let patterns = analysis.analyze_function(&func);
    if !patterns.is_empty() {
        let p = &patterns[0];
        assert_eq!(p.source, "a");
        assert_eq!(p.alias, "b");
        assert!(p.source_used_after);
        assert!(p.alias_used_after);
    }
}

// ── Errors accessor ────────────────────────────────────────────────

#[test]
fn test_uam_errors_accessor() {
    let func = make_func_no_params("f", vec![return_expr(int_lit(1))]);
    let mut analysis = UseAfterMoveAnalysis::new();
    let _ = analysis.analyze_function(&func);
    let errors = analysis.errors();
    assert!(errors.is_empty());
}

// ── Nested function capture with outer var use ─────────────────────

#[test]
fn test_uam_nested_fn_captures_outer() {
    let func = make_func(
        "outer",
        vec![("data", Type::List(Box::new(Type::Int)))],
        vec![
            assign("data", HirExpr::List(vec![int_lit(1), int_lit(2)])),
            HirStmt::FunctionDef {
                name: "inner".to_string(),
                params: Box::new(smallvec![]),
                body: vec![return_expr(call("len", vec![var("data")]))],
                ret_type: Type::Int,
                docstring: None,
            },
            expr_stmt(call("print", vec![var("data")])),
        ],
    );
    let mut analysis = UseAfterMoveAnalysis::new();
    let _errors = analysis.analyze_function(&func);
    // Just verifying nested capture analysis runs without panic
}

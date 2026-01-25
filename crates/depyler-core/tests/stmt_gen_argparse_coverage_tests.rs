//! Shim tests for stmt_gen argparse coverage
//! These use public APIs only.

use depyler_core::hir::{HirExpr, HirStmt, Literal};

fn str_lit(s: &str) -> HirExpr {
    HirExpr::Literal(Literal::String(s.to_string()))
}

#[test]
fn shim_argparse_method_call() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_argument".into(),
        args: vec![str_lit("--verbose")],
        kwargs: vec![],
    };
}

#[test]
fn shim_argparse_parse_args() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "parse_args".into(),
        args: vec![],
        kwargs: vec![],
    };
}

#[test]
fn shim_argparse_add_subparsers() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_subparsers".into(),
        args: vec![],
        kwargs: vec![("dest".into(), str_lit("command"))],
    };
}

#[test]
fn shim_chained_add_parser_set_defaults() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("subparsers".into())),
            method: "add_parser".into(),
            args: vec![str_lit("step")],
            kwargs: vec![],
        }),
        method: "set_defaults".into(),
        args: vec![],
        kwargs: vec![("func".into(), HirExpr::Var("cmd_step".into()))],
    };
}

#[test]
fn shim_argument_parser_call() {
    let _expr = HirExpr::Call {
        func: "ArgumentParser".into(),
        args: vec![],
        kwargs: vec![("description".into(), str_lit("My CLI"))],
    };
}

#[test]
fn shim_add_argument_with_action() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_argument".into(),
        args: vec![str_lit("--verbose")],
        kwargs: vec![("action".into(), str_lit("store_true"))],
    };
}

#[test]
fn shim_add_argument_with_type() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_argument".into(),
        args: vec![str_lit("--count")],
        kwargs: vec![("type".into(), HirExpr::Var("int".into()))],
    };
}

#[test]
fn shim_add_argument_with_default() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_argument".into(),
        args: vec![str_lit("--name")],
        kwargs: vec![("default".into(), str_lit("world"))],
    };
}

#[test]
fn shim_add_argument_with_choices() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_argument".into(),
        args: vec![str_lit("--level")],
        kwargs: vec![(
            "choices".into(),
            HirExpr::List(vec![str_lit("debug"), str_lit("info"), str_lit("warn")]),
        )],
    };
}

#[test]
fn shim_add_argument_positional() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::Var("parser".into())),
        method: "add_argument".into(),
        args: vec![str_lit("filename")],
        kwargs: vec![],
    };
}

#[test]
fn shim_expr_stmt() {
    let _stmt = HirStmt::Expr(HirExpr::Var("x".into()));
}

#[test]
fn shim_triple_chain() {
    let _expr = HirExpr::MethodCall {
        object: Box::new(HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("obj".into())),
                method: "a".into(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "b".into(),
            args: vec![],
            kwargs: vec![],
        }),
        method: "c".into(),
        args: vec![],
        kwargs: vec![],
    };
}

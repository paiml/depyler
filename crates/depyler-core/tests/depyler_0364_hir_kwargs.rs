//! DEPYLER-0364: HIR Keyword Arguments Support Tests
//!
//! Tests to verify that keyword arguments are properly preserved during AST→HIR conversion
//! and correctly handled throughout the transpilation pipeline.

use anyhow::Result;
use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::{HirExpr, HirModule, HirStmt, Literal};
use rustpython_parser::{parse, Mode};

fn transpile_to_hir(python_code: &str) -> Result<HirModule> {
    let ast = parse(python_code, Mode::Module, "<test>")?;
    AstBridge::new().python_to_hir(ast)
}

#[test]
fn test_depyler_0364_call_with_single_kwarg() {
    let python = r#"
def foo():
    result = bar(10, baz="hello")
"#;
    let hir = transpile_to_hir(python).unwrap();

    assert_eq!(hir.functions.len(), 1);
    let func = &hir.functions[0];
    assert_eq!(func.body.len(), 1);

    // Verify kwargs are preserved
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { func, args, kwargs } => {
                assert_eq!(func, "bar");
                assert_eq!(args.len(), 1);
                assert_eq!(kwargs.len(), 1);
                assert_eq!(kwargs[0].0, "baz");
                match &kwargs[0].1 {
                    HirExpr::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
                    _ => panic!("Expected string literal"),
                }
            }
            _ => panic!("Expected Call, got: {:?}", value),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_call_with_multiple_kwargs() {
    let python = r#"
def foo():
    result = bar(10, 20, baz=30, qux="hello")
"#;
    let hir = transpile_to_hir(python).unwrap();

    assert_eq!(hir.functions.len(), 1);
    let func = &hir.functions[0];

    // Verify kwargs are preserved
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { func, args, kwargs } => {
                assert_eq!(func, "bar");
                assert_eq!(args.len(), 2);
                assert_eq!(kwargs.len(), 2);
                assert_eq!(kwargs[0].0, "baz");
                assert_eq!(kwargs[1].0, "qux");
            }
            _ => panic!("Expected Call"),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_method_call_with_kwargs() {
    let python = r#"
def foo():
    parser.add_argument("name", nargs="+", help="Name")
"#;
    let hir = transpile_to_hir(python).unwrap();

    assert_eq!(hir.functions.len(), 1);
    let func = &hir.functions[0];

    // Verify kwargs are preserved
    match &func.body[0] {
        HirStmt::Expr(HirExpr::MethodCall {
            object,
            method,
            args,
            kwargs,
        }) => {
            assert!(matches!(object.as_ref(), HirExpr::Var(v) if v == "parser"));
            assert_eq!(method, "add_argument");
            assert_eq!(args.len(), 1);
            assert_eq!(kwargs.len(), 2);
            assert_eq!(kwargs[0].0, "nargs");
            assert_eq!(kwargs[1].0, "help");
        }
        _ => panic!("Expected MethodCall"),
    }
}

#[test]
fn test_depyler_0364_kwargs_with_complex_expressions() {
    let python = r#"
def foo():
    result = bar(x=10 + 20, y=func())
"#;
    let hir = transpile_to_hir(python).unwrap();

    let func = &hir.functions[0];
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { kwargs, .. } => {
                assert_eq!(kwargs.len(), 2);
                assert_eq!(kwargs[0].0, "x");
                assert_eq!(kwargs[1].0, "y");
                // Verify first kwarg is a binary expression
                assert!(matches!(&kwargs[0].1, HirExpr::Binary { .. }));
                // Verify second kwarg is a call
                assert!(matches!(&kwargs[1].1, HirExpr::Call { .. }));
            }
            _ => panic!("Expected Call"),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_kwargs_with_nested_calls() {
    let python = r#"
def foo():
    result = outer(inner(x=10), y=20)
"#;
    let hir = transpile_to_hir(python).unwrap();

    let func = &hir.functions[0];
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { func, args, kwargs } => {
                assert_eq!(func, "outer");
                assert_eq!(args.len(), 1);
                assert_eq!(kwargs.len(), 1);
                assert_eq!(kwargs[0].0, "y");
                // Verify the positional arg is a call with kwargs
                match &args[0] {
                    HirExpr::Call {
                        func: inner_func,
                        kwargs: inner_kwargs,
                        ..
                    } => {
                        assert_eq!(inner_func, "inner");
                        assert_eq!(inner_kwargs.len(), 1);
                        assert_eq!(inner_kwargs[0].0, "x");
                    }
                    _ => panic!("Expected inner Call"),
                }
            }
            _ => panic!("Expected Call"),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_empty_kwargs_backward_compat() {
    let python = r#"
def foo():
    result = bar(10, 20)
"#;
    let hir = transpile_to_hir(python).unwrap();

    let func = &hir.functions[0];
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { kwargs, .. } => {
                assert_eq!(kwargs.len(), 0);
            }
            _ => panic!("Expected Call"),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_argparse_argumentparser_kwargs() {
    let python = r#"
import argparse

def foo():
    parser = argparse.ArgumentParser(description="Test program", epilog="End text")
"#;
    let hir = transpile_to_hir(python).unwrap();

    assert_eq!(hir.functions.len(), 1);
    let func = &hir.functions[0];

    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::MethodCall {
                method,
                args,
                kwargs,
                ..
            } => {
                assert_eq!(method, "ArgumentParser");
                assert_eq!(args.len(), 0);
                assert_eq!(kwargs.len(), 2);
                assert_eq!(kwargs[0].0, "description");
                assert_eq!(kwargs[1].0, "epilog");
            }
            _ => panic!("Expected MethodCall, got: {:?}", value),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_argparse_add_argument_kwargs() {
    let python = r#"
def foo():
    parser.add_argument("files", nargs="+", type=str, help="Files to process")
"#;
    let hir = transpile_to_hir(python).unwrap();

    let func = &hir.functions[0];
    match &func.body[0] {
        HirStmt::Expr(HirExpr::MethodCall {
            method,
            args,
            kwargs,
            ..
        }) => {
            assert_eq!(method, "add_argument");
            assert_eq!(args.len(), 1);
            assert_eq!(kwargs.len(), 3);
            assert_eq!(kwargs[0].0, "nargs");
            assert_eq!(kwargs[1].0, "type");
            assert_eq!(kwargs[2].0, "help");
        }
        _ => panic!("Expected MethodCall"),
    }
}

#[test]
fn test_depyler_0364_file_open_with_kwargs() {
    let python = r#"
def foo():
    f = open(filename, mode="r", encoding="utf-8")
"#;
    let hir = transpile_to_hir(python).unwrap();

    let func = &hir.functions[0];
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { func, args, kwargs } => {
                assert_eq!(func, "open");
                assert_eq!(args.len(), 1);
                assert_eq!(kwargs.len(), 2);
                assert_eq!(kwargs[0].0, "mode");
                assert_eq!(kwargs[1].0, "encoding");
            }
            _ => panic!("Expected Call"),
        },
        _ => panic!("Expected Assign"),
    }
}

#[test]
fn test_depyler_0364_kwargs_order_preserved() {
    let python = r#"
def foo():
    result = bar(a=1, b=2, c=3, d=4)
"#;
    let hir = transpile_to_hir(python).unwrap();

    let func = &hir.functions[0];
    match &func.body[0] {
        HirStmt::Assign { value, .. } => match value {
            HirExpr::Call { kwargs, .. } => {
                assert_eq!(kwargs.len(), 4);
                assert_eq!(kwargs[0].0, "a");
                assert_eq!(kwargs[1].0, "b");
                assert_eq!(kwargs[2].0, "c");
                assert_eq!(kwargs[3].0, "d");
            }
            _ => panic!("Expected Call"),
        },
        _ => panic!("Expected Assign"),
    }
}

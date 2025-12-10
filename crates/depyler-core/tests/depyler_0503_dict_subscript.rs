//! DEPYLER-0503: Constant in type annotation error
//!
//! RED → GREEN Phase: TypeExtractor called on non-type-annotation expressions
//!
//! Root Cause (Five-Whys):
//! 1. Why? "Unsupported type annotation: Constant(Int(0))"
//! 2. Why? TypeExtractor::extract_type() called on Constant expression
//! 3. Why? Dict subscript or similar being treated as type annotation
//! 4. Why? Somewhere confusing dict[0] with Generic[T]
//! 5. ROOT: TBD - need to find exact call site

#![allow(non_snake_case)]

use depyler_core::ast_bridge;
use rustpython_ast::Suite;
use rustpython_parser::{ast, Parse};

fn parse_and_generate(python: &str) -> depyler_core::hir::HirModule {
    let statements = Suite::parse(python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });
    let (hir, _type_env) = ast_bridge::AstBridge::new()
        .python_to_hir(ast)
        .expect("Should generate HIR");
    hir
}

#[test]
fn test_dict_with_int_keys() {
    let python = r#"
nested = {
    0: "zero",
    1: "one"
}
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

#[test]
fn test_list_subscript_with_int() {
    let python = r#"
items = [1, 2, 3]
first = items[0]
"#;
    let _hir = parse_and_generate(python);
    // Should transpile without "unsupported type annotation" error
}

#[test]
fn test_nested_dict_structure() {
    let python = r#"
nested = {
    "users": [
        {"name": "Alice", "age": 30}
    ]
}
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

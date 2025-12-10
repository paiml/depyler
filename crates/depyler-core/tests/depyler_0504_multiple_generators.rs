//! DEPYLER-0504: Multiple generators in list comprehension
//!
//! RED → GREEN Phase: List comprehension with multiple for clauses
//!
//! Root Cause (Five-Whys):
//! 1. Why? "Multiple conditions in list comprehension not yet supported"
//! 2. Why? Nested list comprehension with multiple for clauses rejected
//! 3. Why? Current HIR ListComp only supports single generator
//! 4. Why? Original implementation (DEPYLER-0XXX) only handled simple case
//! 5. ROOT: HIR architecture limited to single-generator comprehensions

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
fn test_nested_list_comprehension() {
    // Minimal reproducer: nested list comprehension
    let python = r#"
matrix = [[i + j for j in range(3)] for i in range(3)]
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

#[test]
fn test_flattened_list_comprehension() {
    // Flattened list comprehension with multiple for clauses
    let python = r#"
result = [i * j for i in range(3) for j in range(3)]
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

#[test]
fn test_filtered_nested_comprehension() {
    // Multiple for clauses with if condition
    let python = r#"
result = [(i, j) for i in range(5) for j in range(5) if i < j]
"#;
    let hir = parse_and_generate(python);
    assert_eq!(hir.constants.len(), 1);
}

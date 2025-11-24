//! DEPYLER-0502: Support sorted() with variable reverse parameter
//!
//! RED → GREEN Phase: Tests for dynamic reverse parameter in sorted()
//!
//! Root Cause (Five-Whys):
//! 1. Why fails? sorted(..., reverse=reverse) uses variable, not constant
//! 2. Why? converters.rs only handles ast::Expr::Constant
//! 3. Why? HIR SortByKey has reverse: bool (compile-time only)
//! 4. Why? DEPYLER-0307 implemented simple case first
//! 5. ROOT: HIR architecture doesn't support runtime boolean expressions
//!
//! Solution: Change HIR SortByKey from reverse: bool to reverse_expr: Option<Box<HirExpr>>

use depyler_core::ast_bridge;
use rustpython_ast::Suite;
use rustpython_parser::{ast, Parse};

/// Helper function to parse Python and generate HIR
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
fn test_sorted_with_variable_reverse() {
    let python = r#"
def sort_data(items, reverse):
    return sorted(items, reverse=reverse)
"#;
    let hir = parse_and_generate(python);

    assert_eq!(hir.functions.len(), 1, "Should have 1 function");
    assert_eq!(hir.functions[0].name, "sort_data");

    // Should transpile successfully (not panic with "must be constant")
    // The body should contain a SortByKey expression with variable reverse
}

#[test]
fn test_sorted_with_key_and_variable_reverse() {
    let python = r#"
def sort_by_field(data, field, reverse=False):
    return sorted(data, key=lambda x: x.get(field, ""), reverse=reverse)
"#;
    let hir = parse_and_generate(python);

    assert_eq!(hir.functions.len(), 1);
    assert_eq!(hir.functions[0].params.len(), 3);

    // Verify the function transpiles (real test from data_processor.py)
    // Should handle: reverse=reverse (variable) not reverse=True (constant)
}

#[test]
fn test_sorted_with_expression_reverse() {
    let python = r#"
def sort_desc(items, should_reverse):
    return sorted(items, reverse=not should_reverse)
"#;
    let hir = parse_and_generate(python);

    assert_eq!(hir.functions.len(), 1);

    // Should handle reverse=<expression>, not just reverse=<variable>
}

#[test]
fn test_data_processor_sort_by() {
    // Real-world example that currently fails
    let python = r#"
from typing import List, Dict, Any

def sort_by(data: List[Dict[str, Any]], field: str, reverse: bool = False) -> List[Dict[str, Any]]:
    """Sort records by field value."""
    return sorted(data, key=lambda x: x.get(field, ""), reverse=reverse)
"#;
    let hir = parse_and_generate(python);

    assert_eq!(
        hir.functions.len(),
        1,
        "Should transpile data_processor.py sort_by()"
    );
    assert_eq!(hir.functions[0].name, "sort_by");
    assert_eq!(hir.functions[0].params.len(), 3);
}

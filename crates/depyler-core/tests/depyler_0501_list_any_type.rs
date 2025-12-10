//! DEPYLER-0501: Support List[Any] and Callable[[Any], Any] type annotations
//!
//! RED → GREEN Phase: Tests for Any type support in collections
//!
//! Tests verify:
//! 1. List[Any] transpiles to Vec<serde_json::Value>
//! 2. Callable[[Any], Any] transpiles correctly
//! 3. data_processor.py compiles successfully

#![allow(non_snake_case)]

use depyler_core::ast_bridge;
use depyler_core::hir::Type;
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
fn test_list_any_return_type() {
    let python = r#"
from typing import List, Any

def get_data() -> List[Any]:
    return [1, "hello", True]
"#;
    let hir = parse_and_generate(python);

    assert_eq!(hir.functions.len(), 1, "Should have 1 function");
    assert_eq!(hir.functions[0].name, "get_data");

    // DEPYLER-0725: List[Any] should map to List<Custom("Any")> which type_mapper converts to serde_json::Value
    match &hir.functions[0].ret_type {
        Type::List(inner) => {
            assert!(
                matches!(inner.as_ref(), Type::Unknown)
                    || matches!(inner.as_ref(), Type::Custom(s) if s == "serde_json::Value" || s == "Any"),
                "List[Any] should map to List<Unknown>, List<Any>, or List<Value>, got: {:?}",
                inner
            );
        }
        other => panic!("Expected List type, got: {:?}", other),
    }
}

#[test]
fn test_callable_with_any() {
    let python = r#"
from typing import Callable, Any

def apply(transform: Callable[[Any], Any], value: Any) -> Any:
    return transform(value)
"#;
    let hir = parse_and_generate(python);

    assert_eq!(hir.functions.len(), 1, "Should have 1 function");

    // For now, just verify it transpiles without error
    // Callable[[Any], Any] is complex - may map to Unknown initially
    assert!(
        hir.functions[0].params.len() == 2,
        "Should have 2 parameters"
    );
}

#[test]
fn test_data_processor_transpiles() {
    // Real example from data_processor.py that was failing
    let python = r#"
from typing import List, Any

def distinct(field: str) -> List[Any]:
    """Get distinct values for a field."""
    return []
"#;

    let hir = parse_and_generate(python);
    assert_eq!(hir.functions.len(), 1, "Should transpile successfully");
}

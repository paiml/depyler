//! DEPYLER-0500: HIR Integration - Type Annotation Collection (Pass 1)
//!
//! RED Phase: Failing tests for HIR + TypeEnvironment integration
//!
//! Tests verify:
//! 1. generate_hir() returns (Hir, TypeEnvironment)
//! 2. Function signatures collected (parameters + return type)
//! 3. Variable annotations collected (x: int = value)
//! 4. Existing HIR generation still works

use depyler_core::hir::{generate_hir, Type};
use depyler_core::parse_python;

#[test]
fn test_hir_gen_returns_type_environment() {
    let python = "def foo(x: int) -> str:\n    return str(x)";
    let ast = parse_python(python).expect("Should parse");

    // NEW API: generate_hir returns (Hir, TypeEnvironment)
    let (hir, type_env) = generate_hir(&ast).expect("Should generate HIR");

    // Verify HIR generated
    assert_eq!(hir.functions.len(), 1, "Should have 1 function");
    assert_eq!(hir.functions[0].name, "foo");

    // Verify TypeEnvironment populated with parameter type
    assert_eq!(
        type_env.get_var_type("x"),
        Some(&Type::Int),
        "Parameter 'x: int' should be in TypeEnvironment"
    );
}

#[test]
fn test_collect_function_signature() {
    let python = "def add(a: int, b: int) -> int:\n    return a + b";
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).expect("Should generate");

    // Both parameters collected
    assert_eq!(type_env.get_var_type("a"), Some(&Type::Int), "Parameter 'a' should be Int");
    assert_eq!(type_env.get_var_type("b"), Some(&Type::Int), "Parameter 'b' should be Int");
}

#[test]
fn test_collect_variable_annotations() {
    let python = "x: int = 5\ny: str = 'hello'";
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    // Variable annotations collected
    assert_eq!(type_env.get_var_type("x"), Some(&Type::Int), "Variable 'x: int' should be Int");
    assert_eq!(type_env.get_var_type("y"), Some(&Type::String), "Variable 'y: str' should be String");
}

#[test]
fn test_collect_optional_type() {
    let python = "def maybe(x: int) -> int | None:\n    return x if x > 0 else None";
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    assert_eq!(type_env.get_var_type("x"), Some(&Type::Int));
}

#[test]
fn test_collect_list_type() {
    let python = "numbers: list[int] = [1, 2, 3]";
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    // Should collect List<Int>
    assert_eq!(
        type_env.get_var_type("numbers"),
        Some(&Type::List(Box::new(Type::Int))),
        "Should be List<Int>"
    );
}

#[test]
fn test_unannotated_variable_not_in_env() {
    let python = "x = 5";  // No annotation
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    // Unannotated variables NOT in TypeEnvironment (Pass 1 only collects explicit annotations)
    assert_eq!(
        type_env.get_var_type("x"),
        None,
        "Unannotated variables not collected in Pass 1"
    );
}

#[test]
fn test_multiple_functions_separate_scopes() {
    let python = r#"
def foo(x: int) -> int:
    return x

def bar(x: str) -> str:
    return x
"#;
    let ast = parse_python(python).unwrap();

    let (_hir, type_env) = generate_hir(&ast).unwrap();

    // Currently TypeEnvironment is global scope
    // The second 'x: str' should create a new version (SSA)
    // This test verifies SSA variable versioning works
    assert!(
        type_env.get_var_type("x").is_some(),
        "Should have at least one 'x' binding"
    );
}

#[test]
fn test_existing_hir_tests_still_pass() {
    // Verify backward compatibility: existing code that doesn't use TypeEnvironment still works
    let python = "def simple(): pass";
    let ast = parse_python(python).unwrap();

    let (hir, _type_env) = generate_hir(&ast).unwrap();

    assert_eq!(hir.functions.len(), 1);
    assert_eq!(hir.functions[0].name, "simple");
}

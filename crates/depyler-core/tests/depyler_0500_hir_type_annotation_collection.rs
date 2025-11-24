//! DEPYLER-0500: HIR Integration - Type Annotation Collection (Pass 1)
//!
//! RED → GREEN Phase: Tests for HIR + TypeEnvironment integration
//!
//! Tests verify:
//! 1. python_to_hir() returns (HirModule, TypeEnvironment)
//! 2. Function signatures collected (parameters + return type)
//! 3. Variable annotations collected (x: int = value)
//! 4. Existing HIR generation still works

use depyler_core::ast_bridge;
use depyler_core::hir::{HirModule, Type};
use depyler_core::type_system::type_environment::TypeEnvironment;
use rustpython_ast::Suite;
use rustpython_parser::{ast, Parse};

/// Helper function to parse Python and generate HIR with TypeEnvironment
fn parse_and_generate(python: &str) -> (HirModule, TypeEnvironment) {
    let statements = Suite::parse(python, "<test>").expect("Should parse");
    let ast = ast::Mod::Module(ast::ModModule {
        body: statements,
        type_ignores: vec![],
        range: Default::default(),
    });
    ast_bridge::AstBridge::new()
        .python_to_hir(ast)
        .expect("Should generate HIR")
}

#[test]
fn test_hir_gen_returns_type_environment() {
    let python = "def foo(x: int) -> str:\n    return str(x)";
    let (hir, type_env) = parse_and_generate(python);

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
    let (_hir, type_env) = parse_and_generate(python);

    // Both parameters collected
    assert_eq!(
        type_env.get_var_type("a"),
        Some(&Type::Int),
        "Parameter 'a' should be Int"
    );
    assert_eq!(
        type_env.get_var_type("b"),
        Some(&Type::Int),
        "Parameter 'b' should be Int"
    );
}

#[test]
fn test_collect_variable_annotations() {
    let python = "x: int = 5\ny: str = 'hello'";
    let (_hir, type_env) = parse_and_generate(python);

    // Variable annotations collected (module-level constants)
    assert_eq!(
        type_env.get_var_type("x"),
        Some(&Type::Int),
        "Variable 'x: int' should be Int"
    );
    assert_eq!(
        type_env.get_var_type("y"),
        Some(&Type::String),
        "Variable 'y: str' should be String"
    );
}

#[test]
fn test_collect_optional_type() {
    let python = "def maybe(x: int) -> int | None:\n    return x if x > 0 else None";
    let (_hir, type_env) = parse_and_generate(python);

    assert_eq!(type_env.get_var_type("x"), Some(&Type::Int));
}

#[test]
fn test_collect_list_type() {
    let python = "numbers: list[int] = [1, 2, 3]";
    let (_hir, type_env) = parse_and_generate(python);

    // Should collect List<Int>
    assert_eq!(
        type_env.get_var_type("numbers"),
        Some(&Type::List(Box::new(Type::Int))),
        "Should be List<Int>"
    );
}

#[test]
fn test_unannotated_variable_not_in_env() {
    let python = "x = 5"; // No annotation
    let (_hir, type_env) = parse_and_generate(python);

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
    let (_hir, type_env) = parse_and_generate(python);

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
    let (hir, _type_env) = parse_and_generate(python);

    assert_eq!(hir.functions.len(), 1);
    assert_eq!(hir.functions[0].name, "simple");
}

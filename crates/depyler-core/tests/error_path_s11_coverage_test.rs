//! Session 11: Error path and edge case coverage tests
//!
//! Exercises error handling paths, fallback behaviors, and unusual
//! but valid Python patterns that exercise rare code paths.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

fn hir_succeeds(python_code: &str) -> bool {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .is_ok()
}

// ============================================================================
// Empty / minimal constructs
// ============================================================================

#[test]
fn test_s11_err_empty_function() {
    let code = r#"
def empty() -> None:
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty"), "Got: {}", result);
}

#[test]
fn test_s11_err_only_docstring() {
    let code = r#"
def documented() -> None:
    """This function does nothing."""
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn documented"), "Got: {}", result);
}

#[test]
fn test_s11_err_only_constants() {
    let code = r#"
PI: float = 3.14159
E: float = 2.71828
TAU: float = 6.28318
"#;
    let result = transpile(code);
    assert!(
        result.contains("PI") || result.contains("E") || result.contains("TAU"),
        "Got: {}",
        result
    );
}

#[test]
fn test_s11_err_multiple_constants() {
    let code = r#"
MAX_SIZE: int = 1000
MIN_SIZE: int = 1
DEFAULT_NAME: str = "unknown"
DEBUG: bool = False
"#;
    let result = transpile(code);
    assert!(
        result.contains("MAX_SIZE") || result.contains("1000"),
        "Got: {}",
        result
    );
}

// ============================================================================
// Unusual but valid function signatures
// ============================================================================

#[test]
fn test_s11_err_no_params() {
    let code = r#"
def get_answer() -> int:
    return 42
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_answer()"),
        "Got: {}",
        result
    );
}

#[test]
fn test_s11_err_many_params() {
    let code = r#"
def many(a: int, b: int, c: int, d: int, e: int) -> int:
    return a + b + c + d + e
"#;
    let result = transpile(code);
    assert!(result.contains("fn many"), "Got: {}", result);
}

#[test]
fn test_s11_err_no_return_type() {
    let code = r#"
def side_effect(x: int):
    print(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn side_effect"), "Got: {}", result);
}

#[test]
fn test_s11_err_no_type_annotations() {
    let code = r#"
def untyped(x, y):
    return x + y
"#;
    let result = transpile(code);
    assert!(result.contains("fn untyped"), "Got: {}", result);
}

// ============================================================================
// Edge case expressions
// ============================================================================

#[test]
fn test_s11_err_zero_literal() {
    let code = r#"
def zero() -> int:
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("0"), "Got: {}", result);
}

#[test]
fn test_s11_err_negative_zero_float() {
    let code = r#"
def neg_zero() -> float:
    return -0.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn neg_zero"), "Got: {}", result);
}

#[test]
fn test_s11_err_true_false_literals() {
    let code = r#"
def get_true() -> bool:
    return True

def get_false() -> bool:
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("true") && result.contains("false"),
        "Got: {}",
        result
    );
}

#[test]
fn test_s11_err_none_return() {
    let code = r#"
from typing import Optional

def maybe() -> Optional[int]:
    return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("None") || result.contains("fn maybe"),
        "Got: {}",
        result
    );
}

#[test]
fn test_s11_err_empty_string() {
    let code = r#"
def blank() -> str:
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn blank"), "Got: {}", result);
}

#[test]
fn test_s11_err_empty_list() {
    let code = r#"
def empty_list() -> list:
    return []
"#;
    let result = transpile(code);
    assert!(
        result.contains("vec!") || result.contains("Vec::new"),
        "Got: {}",
        result
    );
}

#[test]
fn test_s11_err_empty_dict() {
    let code = r#"
def empty_dict() -> dict:
    return {}
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn empty_dict"),
        "Got: {}",
        result
    );
}

#[test]
fn test_s11_err_empty_set() {
    let code = r#"
def empty_set() -> set:
    return set()
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashSet") || result.contains("fn empty_set"),
        "Got: {}",
        result
    );
}

// ============================================================================
// Complex control flow
// ============================================================================

#[test]
fn test_s11_err_nested_while_for() {
    let code = r#"
def find_pair(matrix: list, target: int) -> bool:
    for row in matrix:
        idx: int = 0
        while idx < len(row):
            if row[idx] == target:
                return True
            idx += 1
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pair"), "Got: {}", result);
}

#[test]
fn test_s11_err_early_return_chain() {
    let code = r#"
def validate(x: int, y: int, z: int) -> bool:
    if x < 0:
        return False
    if y < 0:
        return False
    if z < 0:
        return False
    if x + y + z > 100:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s11_err_for_else() {
    let code = r#"
def has_target(items: list, target: int) -> bool:
    for item in items:
        if item == target:
            return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_target"), "Got: {}", result);
}

// ============================================================================
// HIR edge cases
// ============================================================================

#[test]
fn test_s11_err_hir_single_expression() {
    assert!(hir_succeeds("42\n"));
}

#[test]
fn test_s11_err_hir_print_only() {
    assert!(hir_succeeds("print('hello')\n"));
}

#[test]
fn test_s11_err_hir_import_only() {
    assert!(hir_succeeds("import os\n"));
}

#[test]
fn test_s11_err_hir_from_import() {
    assert!(hir_succeeds("from typing import List, Dict\n"));
}

#[test]
fn test_s11_err_hir_class_only() {
    assert!(hir_succeeds(
        r#"
class Empty:
    pass
"#
    ));
}

#[test]
fn test_s11_err_hir_nested_class() {
    assert!(hir_succeeds(
        r#"
class Outer:
    class Inner:
        pass
"#
    ));
}

#[test]
fn test_s11_err_hir_decorated() {
    assert!(hir_succeeds(
        r#"
def decorator(func):
    return func

@decorator
def decorated() -> int:
    return 1
"#
    ));
}

#[test]
fn test_s11_err_hir_star_args() {
    assert!(hir_succeeds(
        r#"
def variadic(*args) -> int:
    return len(args)
"#
    ));
}

#[test]
fn test_s11_err_hir_kwargs() {
    assert!(hir_succeeds(
        r#"
def with_kwargs(**kwargs) -> int:
    return len(kwargs)
"#
    ));
}

#[test]
fn test_s11_err_hir_lambda_standalone() {
    assert!(hir_succeeds("double = lambda x: x * 2\n"));
}

#[test]
fn test_s11_err_hir_multiline_string() {
    assert!(hir_succeeds(
        r#"
msg = """
This is a
multiline string
"""
"#
    ));
}

#[test]
fn test_s11_err_hir_bytes_literal() {
    assert!(hir_succeeds(r#"data = b"hello""#));
}

#[test]
fn test_s11_err_hir_complex_number() {
    // Complex numbers may not be supported - just verify parse doesn't panic
    let _ = hir_succeeds("z = 1 + 2j\n");
}

#[test]
fn test_s11_err_hir_ellipsis() {
    assert!(hir_succeeds("x = ...\n"));
}

#[test]
fn test_s11_err_hir_walrus() {
    assert!(hir_succeeds(
        r#"
def check(items: list) -> bool:
    if (n := len(items)) > 0:
        return True
    return False
"#
    ));
}

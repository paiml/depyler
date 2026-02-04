//! Coverage tests for rust_gen/expr_type_helpers.rs
//!
//! DEPYLER-99MODE-001: Targets expr_type_helpers.rs (799 lines)
//! Covers: type inference for expressions, int/float detection,
//! borrowing requirements, variable name heuristics.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_expr_type_int_literal() {
    let code = "def f() -> int:\n    x = 10\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_float_literal() {
    let code = "def f() -> float:\n    x = 3.14\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_string_literal() {
    let code = "def f() -> str:\n    x = \"hello\"\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_bool_literal() {
    let code = "def f() -> bool:\n    return True\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_arithmetic_int() {
    let code = "def f(a: int, b: int) -> int:\n    return a + b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_arithmetic_float() {
    let code = "def f(a: float, b: float) -> float:\n    return a * b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_division_float() {
    let code = "def f(a: int, b: int) -> float:\n    return a / b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_floor_division_int() {
    let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_modulo_int() {
    let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_mixed_arithmetic() {
    let code = r#"
def f(x: int, y: float) -> float:
    return float(x) + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_string_method() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_list_method() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_comparison() {
    let code = r#"
def f(x: int) -> bool:
    return x > 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_ternary() {
    let code = r#"
def f(x: int) -> int:
    return x if x > 0 else -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_type_complex_expression() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c - a // b
"#;
    assert!(transpile_ok(code));
}

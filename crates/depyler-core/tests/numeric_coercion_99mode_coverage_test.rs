//! Coverage tests for rust_gen/numeric_coercion.rs
//!
//! DEPYLER-99MODE-001: Targets numeric_coercion.rs (707 lines)
//! Covers: numeric type detection, float coercion,
//! mixed-type arithmetic, variable name heuristics.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_numeric_int_add() {
    let code = "def f(a: int, b: int) -> int:\n    return a + b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_float_add() {
    let code = "def f(a: float, b: float) -> float:\n    return a + b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_int_multiply() {
    let code = "def f(a: int, b: int) -> int:\n    return a * b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_float_division() {
    let code = "def f(a: int, b: int) -> float:\n    return a / b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_floor_division() {
    let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_modulo() {
    let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_power() {
    let code = "def f(x: int, n: int) -> int:\n    return x ** n\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_mixed_types() {
    let code = r#"
def f(x: int, y: float) -> float:
    return float(x) + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_complex_expr() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c - a % b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_float_literal() {
    let code = r#"
def f() -> float:
    return 3.14 * 2.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_int_literal() {
    let code = r#"
def f() -> int:
    return 42 + 58
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_comparison() {
    let code = r#"
def f(x: int, y: int) -> bool:
    return x > y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_augmented_assign() {
    let code = r#"
def f() -> int:
    x = 0
    x += 5
    x *= 2
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_numeric_loop_accumulator() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i * i
    return total
"#;
    assert!(transpile_ok(code));
}

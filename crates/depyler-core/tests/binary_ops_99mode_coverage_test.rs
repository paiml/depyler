//! Coverage tests for rust_gen/binary_ops.rs
//!
//! DEPYLER-99MODE-001: Targets binary_ops.rs (974 lines)
//! Covers: operator precedence, parenthesization, arithmetic,
//! comparison, logical, bitwise, membership operations.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Arithmetic operators
// ============================================================================

#[test]
fn test_binop_add() {
    let code = "def f(a: int, b: int) -> int:\n    return a + b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_sub() {
    let code = "def f(a: int, b: int) -> int:\n    return a - b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_mul() {
    let code = "def f(a: int, b: int) -> int:\n    return a * b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_div() {
    let code = "def f(a: float, b: float) -> float:\n    return a / b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_mod() {
    let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_floor_div() {
    let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_power() {
    let code = "def f(x: int, n: int) -> int:\n    return x ** n\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Operator precedence
// ============================================================================

#[test]
fn test_binop_precedence_add_mul() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return a + b * c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_precedence_parens() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_precedence_complex() {
    let code = r#"
def f(a: int, b: int, c: int, d: int) -> int:
    return a + b * c - d % 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_precedence_nested_parens() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return ((a + b) * (c - a)) % b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comparison operators
// ============================================================================

#[test]
fn test_binop_lt() {
    let code = "def f(a: int, b: int) -> bool:\n    return a < b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_le() {
    let code = "def f(a: int, b: int) -> bool:\n    return a <= b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_gt() {
    let code = "def f(a: int, b: int) -> bool:\n    return a > b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_ge() {
    let code = "def f(a: int, b: int) -> bool:\n    return a >= b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_eq() {
    let code = "def f(a: int, b: int) -> bool:\n    return a == b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_ne() {
    let code = "def f(a: int, b: int) -> bool:\n    return a != b\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Logical operators
// ============================================================================

#[test]
fn test_binop_and() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a and b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_or() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a or b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_not() {
    let code = r#"
def f(a: bool) -> bool:
    return not a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_logical_complex() {
    let code = r#"
def f(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not c)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Bitwise operators
// ============================================================================

#[test]
fn test_binop_bitand() {
    let code = "def f(a: int, b: int) -> int:\n    return a & b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_bitor() {
    let code = "def f(a: int, b: int) -> int:\n    return a | b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_bitxor() {
    let code = "def f(a: int, b: int) -> int:\n    return a ^ b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_lshift() {
    let code = "def f(a: int, b: int) -> int:\n    return a << b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_rshift() {
    let code = "def f(a: int, b: int) -> int:\n    return a >> b\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Membership operators
// ============================================================================

#[test]
fn test_binop_in_list() {
    let code = r#"
def f(x: int, items: list) -> bool:
    return x in items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex binary operations
// ============================================================================

#[test]
fn test_binop_mixed_types() {
    let code = r#"
def f(x: int, y: float) -> float:
    return float(x) + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_chained_comparison() {
    let code = r#"
def f(x: int) -> bool:
    return x > 0 and x < 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binop_string_concat() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + " " + b
"#;
    assert!(transpile_ok(code));
}

//! Coverage tests for rust_gen/type_gen.rs
//!
//! DEPYLER-99MODE-001: Targets type_gen.rs (1,407 lines)
//! Covers: Python→Rust type conversion, binary operator mapping,
//! collection types, primitive types, import dependency tracking.

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
// Primitive types
// ============================================================================

#[test]
fn test_type_gen_int() {
    let code = r#"
def f(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_float() {
    let code = r#"
def f(x: float) -> float:
    return x * 2.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_bool() {
    let code = r#"
def f(x: bool) -> bool:
    return not x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_str() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection types
// ============================================================================

#[test]
fn test_type_gen_list() {
    let code = r#"
def f() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_dict() {
    let code = r#"
def f() -> dict:
    return {"a": 1, "b": 2}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_set() {
    let code = r#"
def f() -> set:
    return {1, 2, 3}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_tuple() {
    let code = r#"
def f() -> tuple:
    return (1, "hello", True)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Binary operators
// ============================================================================

#[test]
fn test_type_gen_add() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_sub() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a - b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_mul() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_div() {
    let code = r#"
def f(a: float, b: float) -> float:
    return a / b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_modulo() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a % b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_floor_div() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a // b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_power() {
    let code = r#"
def f(x: int, n: int) -> int:
    return x ** n
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comparison operators
// ============================================================================

#[test]
fn test_type_gen_eq() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a == b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_ne() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a != b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_lt() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a < b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_le() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a <= b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_gt() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a > b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_ge() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a >= b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Logical and bitwise operators
// ============================================================================

#[test]
fn test_type_gen_and() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a and b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_or() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a or b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_bitwise_and() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a & b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_bitwise_or() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a | b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_bitwise_xor() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a ^ b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_left_shift() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a << b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_right_shift() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a >> b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex type patterns
// ============================================================================

#[test]
fn test_type_gen_nested_collection() {
    let code = r#"
def f() -> list:
    return [[1, 2], [3, 4]]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_mixed_arithmetic() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c - a % b + (a >> 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_optional_return() {
    let code = r#"
def f(items: list) -> int:
    if len(items) > 0:
        return items[0]
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_gen_class_fields() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def magnitude(self) -> float:
        return (self.x * self.x + self.y * self.y) ** 0.5
"#;
    assert!(transpile_ok(code));
}

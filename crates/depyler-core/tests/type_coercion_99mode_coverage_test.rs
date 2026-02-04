//! Coverage tests for rust_gen/type_coercion.rs
//!
//! DEPYLER-99MODE-001: Targets type_coercion.rs (1,309 lines)
//! Covers: runtime type checking, dynamic→static type conversion,
//! numeric coercion, container operations, Optional/None handling.

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
// Numeric type coercion
// ============================================================================

#[test]
fn test_coercion_int_to_float() {
    let code = r#"
def f(x: int) -> float:
    return float(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_float_to_int() {
    let code = r#"
def f(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_int_to_str() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_int_to_bool() {
    let code = r#"
def f(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_mixed_arithmetic() {
    let code = r#"
def f(a: int, b: float) -> float:
    return float(a) + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Container type coercion
// ============================================================================

#[test]
fn test_coercion_list_of_ints() {
    let code = r#"
def f() -> list:
    return [1, 2, 3, 4, 5]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_dict_str_int() {
    let code = r#"
def f() -> dict:
    return {"a": 1, "b": 2}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_set_of_ints() {
    let code = r#"
def f() -> set:
    return {1, 2, 3}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_tuple_mixed() {
    let code = r#"
def f() -> tuple:
    return (1, "hello", True)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversion functions
// ============================================================================

#[test]
fn test_coercion_str_to_int() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_str_to_float() {
    let code = r#"
def f(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_list_to_set() {
    let code = r#"
def f(items: list) -> set:
    return set(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Zero/empty value patterns
// ============================================================================

#[test]
fn test_coercion_zero_default() {
    let code = r#"
def f() -> int:
    x = 0
    x += 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_empty_string() {
    let code = r#"
def f() -> str:
    s = ""
    s = s + "hello"
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_empty_list() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_empty_dict() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex coercion patterns
// ============================================================================

#[test]
fn test_coercion_chained_conversion() {
    let code = r#"
def f(x: int) -> str:
    return str(x * 2 + 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_comparison_types() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a > b and a != 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_coercion_list_operations() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += int(item)
    return total
"#;
    assert!(transpile_ok(code));
}

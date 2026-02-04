//! Coverage tests for type_system/type_unify.rs
//!
//! DEPYLER-99MODE-001: Targets type_unify.rs (1,453 lines)
//! Covers: type unification, coercion, constraint propagation,
//! optional handling, numeric widening, nested generics.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Basic type unification
// ============================================================================

#[test]
fn test_unify_int_assignment() {
    let code = r#"
def f() -> int:
    x = 5
    y = x
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_float_assignment() {
    let code = r#"
def f() -> float:
    x = 3.14
    y = x
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_str_assignment() {
    let code = r#"
def f() -> str:
    x = "hello"
    y = x
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_bool_assignment() {
    let code = r#"
def f() -> bool:
    x = True
    y = x
    return y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Numeric coercion
// ============================================================================

#[test]
fn test_unify_int_float_coercion() {
    let code = r#"
def f() -> float:
    x = 5
    y = x + 2.5
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_int_to_float_return() {
    let code = r#"
def f(a: int) -> float:
    return a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_numeric_chain() {
    let code = r#"
def f() -> float:
    x = 1
    y = 1.5
    z = x + y
    return z
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection type constraints
// ============================================================================

#[test]
fn test_unify_list_propagation() {
    let code = r#"
def f() -> list:
    x = [1, 2, 3]
    y = x
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_dict_propagation() {
    let code = r#"
def f() -> dict:
    d = {"a": 1}
    d["b"] = 2
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_list_element_inference() {
    let code = r#"
def f() -> list:
    lst = []
    lst.append(5)
    lst.append(10)
    return lst
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_nested_generic() {
    let code = r#"
def f() -> dict:
    data = {"k": [1, 2, 3]}
    return data
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Optional/None type handling
// ============================================================================

#[test]
fn test_unify_optional_ternary() {
    let code = r#"
def f(x: int) -> int:
    result = x if x > 0 else 0
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_none_to_concrete() {
    let code = r#"
from typing import Optional
def f(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Binary operator type unification
// ============================================================================

#[test]
fn test_unify_add_operator() {
    let code = r#"
def f(a, b):
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_compare_operator() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a > b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_string_concat() {
    let code = r#"
def f() -> str:
    s = ""
    s = s + "text"
    return s
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function return type unification
// ============================================================================

#[test]
fn test_unify_return_annotation() {
    let code = r#"
def f(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_multiple_returns() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function types
// ============================================================================

#[test]
fn test_unify_nested_function() {
    let code = r#"
def outer() -> int:
    def inner(x: int) -> int:
        return x * 2
    return inner(5)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex unification
// ============================================================================

#[test]
fn test_unify_loop_variable() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_dict_access() {
    let code = r#"
def f(data: dict) -> int:
    return data["key"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_unify_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

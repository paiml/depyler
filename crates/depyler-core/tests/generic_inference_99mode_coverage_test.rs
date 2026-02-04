//! Coverage tests for generic_inference.rs
//!
//! DEPYLER-99MODE-001: Targets generic_inference.rs (1,367 lines)
//! Covers: generic type parameter inference, list element types,
//! dict value types, comprehension type propagation, optional inference,
//! higher-order function types.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// List element type inference
// ============================================================================

#[test]
fn test_generic_list_int_element() {
    let code = r#"
def f(items: list) -> int:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_list_str_element() {
    let code = r#"
def f(items: list) -> str:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_list_loop_element() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dict key/value type inference
// ============================================================================

#[test]
fn test_generic_dict_access() {
    let code = r#"
def f(data: dict) -> int:
    return data["key"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_dict_get() {
    let code = r#"
def f(data: dict) -> int:
    return data.get("key", 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_dict_iteration() {
    let code = r#"
def f(data: dict) -> list:
    result = []
    for key in data:
        result.append(key)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_dict_items() {
    let code = r#"
def f(data: dict) -> list:
    result = []
    for k, v in data.items():
        result.append(k)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Optional type inference
// ============================================================================

#[test]
fn test_generic_optional_check() {
    let code = r#"
from typing import Optional
def f(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_optional_default() {
    let code = r#"
def f(x: int = None) -> int:
    if x is None:
        return 0
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehension type propagation
// ============================================================================

#[test]
fn test_generic_list_comp() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_list_comp_filter() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_dict_comp() {
    let code = r#"
def f(items: list) -> dict:
    return {str(i): i for i in items}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function call type inference
// ============================================================================

#[test]
fn test_generic_call_known_sig() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def f(items: list) -> list:
    return [double(x) for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_builtin_len() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_builtin_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type annotation propagation
// ============================================================================

#[test]
fn test_generic_typed_list() {
    let code = r#"
from typing import List
def f(items: List[int]) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_typed_dict() {
    let code = r#"
from typing import Dict
def f(data: Dict[str, int]) -> int:
    total = 0
    for k in data:
        total += data[k]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_typed_tuple() {
    let code = r#"
from typing import Tuple
def f(pair: Tuple[int, str]) -> str:
    return str(pair[0]) + pair[1]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_generic_nested_collection() {
    let code = r#"
def f(matrix: list) -> list:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generic_multi_function_chain() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def f(items: list) -> int:
    return sum(square(x) for x in items)
"#;
    assert!(transpile_ok(code));
}

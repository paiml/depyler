//! Coverage tests for borrowing.rs
//!
//! DEPYLER-99MODE-001: Targets borrowing.rs (1,537 lines)
//! Covers: BorrowingContext analysis, ownership inference,
//! mutated/escaping/read-only params, type conversion, loop patterns.

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
// Read-only parameters (borrowed)
// ============================================================================

#[test]
fn test_borrow_read_only_int() {
    let code = r#"
def f(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_string() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_list() {
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
fn test_borrow_read_only_dict() {
    let code = r#"
def f(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Mutated parameters (mutable borrow)
// ============================================================================

#[test]
fn test_borrow_mutated_list_append() {
    let code = r#"
def f(items: list) -> list:
    items.append(42)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutated_list_extend() {
    let code = r#"
def f(items: list) -> list:
    items.extend([1, 2, 3])
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutated_index_assign() {
    let code = r#"
def f(items: list) -> list:
    items[0] = 99
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutated_dict_assign() {
    let code = r#"
def f(d: dict) -> dict:
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutated_attribute() {
    let code = r#"
class Obj:
    def __init__(self):
        self.x = 0

    def set_x(self, value: int):
        self.x = value
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Escaping parameters (owned)
// ============================================================================

#[test]
fn test_borrow_escape_return() {
    let code = r#"
def f(s: str) -> str:
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_escape_in_list() {
    let code = r#"
def f(a: int, b: int) -> list:
    return [a, b]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_escape_in_tuple() {
    let code = r#"
def f(x: int, y: int) -> tuple:
    return (x, y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_escape_in_dict() {
    let code = r#"
def f(key: str, value: int) -> dict:
    return {key: value}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop usage patterns
// ============================================================================

#[test]
fn test_borrow_loop_iteration() {
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
fn test_borrow_loop_index_access() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i in range(len(items)):
        total += items[i]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_loop_with_condition() {
    let code = r#"
def f(items: list, threshold: int) -> int:
    count = 0
    for item in items:
        if item > threshold:
            count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversion - primitives
// ============================================================================

#[test]
fn test_borrow_type_int() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_type_float() {
    let code = r#"
def f(a: float, b: float) -> float:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_type_bool() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a and b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversion - collections
// ============================================================================

#[test]
fn test_borrow_type_list_int() {
    let code = r#"
def f(items: list) -> list:
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_type_dict_str() {
    let code = r#"
def f(d: dict) -> dict:
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_type_set() {
    let code = r#"
def f(s: set) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex borrowing patterns
// ============================================================================

#[test]
fn test_borrow_mixed_params() {
    let code = r#"
def f(items: list, n: int) -> list:
    result = []
    for i in range(n):
        result.append(items[i])
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_conditional_mutation() {
    let code = r#"
def f(items: list, flag: bool) -> list:
    if flag:
        items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_multiple_uses() {
    let code = r#"
def f(data: list) -> int:
    total = sum(data)
    count = len(data)
    avg = total + count
    return avg
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_complex_expression() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    x = a + b
    y = b * c
    z = x - y
    return z
"#;
    assert!(transpile_ok(code));
}

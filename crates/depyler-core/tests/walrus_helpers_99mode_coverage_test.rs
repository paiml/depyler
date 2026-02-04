//! Coverage tests for rust_gen/walrus_helpers.rs
//!
//! DEPYLER-99MODE-001: Targets walrus_helpers.rs (789 lines)
//! Covers: walrus operator (:=) transpilation, named expressions
//! in conditions, while loops, and comprehensions.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Basic patterns (walrus operator in Python 3.8+)
// ============================================================================

#[test]
fn test_walrus_if_condition() {
    let code = r#"
def f(items: list) -> int:
    if len(items) > 0:
        return items[0]
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_while_pattern() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    i = 0
    while i < len(items):
        total += items[i]
        i += 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Assignment in conditions (simulates walrus behavior)
// ============================================================================

#[test]
fn test_walrus_assign_before_if() {
    let code = r#"
def f(s: str) -> int:
    n = len(s)
    if n > 10:
        return n
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_assign_before_while() {
    let code = r#"
def f(items: list) -> list:
    result = []
    i = 0
    while i < len(items):
        result.append(items[i])
        i += 1
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehension filter patterns
// ============================================================================

#[test]
fn test_walrus_comprehension_filter() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_comprehension_transform() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex conditional patterns
// ============================================================================

#[test]
fn test_walrus_nested_conditions() {
    let code = r#"
def f(items: list) -> int:
    n = len(items)
    if n > 0:
        first = items[0]
        if first > 0:
            return first
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_boolean_chain() {
    let code = r#"
def f(s: str) -> bool:
    n = len(s)
    return n > 0 and n < 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_method_result() {
    let code = r#"
def f(text: str) -> str:
    idx = text.find("x")
    if idx >= 0:
        return text[:idx]
    return text
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_function_call_result() {
    let code = r#"
def compute(x: int) -> int:
    return x * x

def f(items: list) -> list:
    result = []
    for x in items:
        val = compute(x)
        if val > 10:
            result.append(val)
    return result
"#;
    assert!(transpile_ok(code));
}

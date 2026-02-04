//! Coverage tests for test_generation.rs
//!
//! DEPYLER-99MODE-001: Targets test_generation.rs (1,290 lines)
//! Covers: property-based test generation, example-based tests,
//! pure function detection, test configuration.

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
// Pure functions (property test candidates)
// ============================================================================

#[test]
fn test_testgen_pure_function_add() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_pure_function_multiply() {
    let code = r#"
def multiply(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_pure_function_max() {
    let code = r#"
def max_val(a: int, b: int) -> int:
    if a > b:
        return a
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_pure_function_abs() {
    let code = r#"
def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Functions with invariants
// ============================================================================

#[test]
fn test_testgen_reverse_invariant() {
    let code = r#"
def reverse(items: list) -> list:
    result = []
    for i in range(len(items) - 1, -1, -1):
        result.append(items[i])
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_sort_invariant() {
    let code = r#"
def is_sorted(items: list) -> bool:
    for i in range(len(items) - 1):
        if items[i] > items[i + 1]:
            return False
    return True
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_identity_invariant() {
    let code = r#"
def identity(x: int) -> int:
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multi-parameter functions
// ============================================================================

#[test]
fn test_testgen_three_params() {
    let code = r#"
def clamp(x: int, low: int, high: int) -> int:
    if x < low:
        return low
    if x > high:
        return high
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_list_param() {
    let code = r#"
def sum_list(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Various return types
// ============================================================================

#[test]
fn test_testgen_return_bool() {
    let code = r#"
def is_even(n: int) -> bool:
    return n % 2 == 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_return_str() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello " + name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_return_list() {
    let code = r#"
def double_all(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_return_float() {
    let code = r#"
def average(a: float, b: float) -> float:
    return (a + b) / 2.0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex test-worthy patterns
// ============================================================================

#[test]
fn test_testgen_recursive() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_testgen_multiple_functions() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def cube(x: int) -> int:
    return x * x * x

def power(base: int, exp: int) -> int:
    result = 1
    for i in range(exp):
        result *= base
    return result
"#;
    assert!(transpile_ok(code));
}

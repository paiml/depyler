//! Coverage tests for optimization.rs
//!
//! DEPYLER-99MODE-001: Targets optimization.rs (1,254 lines)
//! Covers: constant folding, dead code elimination, CSE,
//! loop unrolling, annotation-driven optimizations.

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
// Constant folding
// ============================================================================

#[test]
fn test_opt_const_fold_int() {
    let code = r#"
def f() -> int:
    return 2 + 3 * 4
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_const_fold_float() {
    let code = r#"
def f() -> float:
    return 3.14 * 2.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_const_fold_string() {
    let code = r#"
def f() -> str:
    return "hello" + " " + "world"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Common subexpression elimination
// ============================================================================

#[test]
fn test_opt_cse_basic() {
    let code = r#"
def f(a: int, b: int) -> int:
    x = a + b
    y = a + b
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_cse_complex() {
    let code = r#"
def f(items: list) -> int:
    n = len(items)
    total = n * 2
    half = n // 2
    return total + half
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dead code patterns
// ============================================================================

#[test]
fn test_opt_dead_code_after_return() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_dead_code_unreachable_branch() {
    let code = r#"
def f() -> int:
    x = 10
    if x > 0:
        return x
    return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop optimization targets
// ============================================================================

#[test]
fn test_opt_tight_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_nested_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            total += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_loop_with_accumulator() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item * item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehension optimization
// ============================================================================

#[test]
fn test_opt_list_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_filtered_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function call optimization
// ============================================================================

#[test]
fn test_opt_inline_candidate() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def f(n: int) -> int:
    return square(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_pure_function() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def f(x: int) -> int:
    return add(x, x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex optimization scenarios
// ============================================================================

#[test]
fn test_opt_algorithm() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_string_building() {
    let code = r#"
def f(items: list) -> str:
    parts = []
    for item in items:
        parts.append(str(item))
    return ", ".join(parts)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_opt_multiple_returns() {
    let code = r#"
def classify(x: int) -> str:
    if x > 100:
        return "high"
    if x > 50:
        return "medium"
    if x > 0:
        return "low"
    return "none"
"#;
    assert!(transpile_ok(code));
}

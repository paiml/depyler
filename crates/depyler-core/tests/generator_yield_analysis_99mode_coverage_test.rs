//! Coverage tests for generator_yield_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets generator_yield_analysis.rs (1,072 lines)
//! Covers: yield point detection, state machine planning,
//! live variable tracking, nesting depth analysis.

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
// Single yield patterns
// ============================================================================

#[test]
fn test_yield_analysis_single() {
    let code = r#"
def gen():
    yield 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_yield_with_expr() {
    let code = r#"
def gen(x: int):
    yield x * 2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple yield patterns
// ============================================================================

#[test]
fn test_yield_analysis_sequential() {
    let code = r#"
def gen():
    yield 1
    yield 2
    yield 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_with_computation() {
    let code = r#"
def gen(n: int):
    x = n
    yield x
    x = x + 1
    yield x
    x = x * 2
    yield x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Yield in loops
// ============================================================================

#[test]
fn test_yield_analysis_for_loop() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_while_loop() {
    let code = r#"
def gen(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_loop_with_accumulator() {
    let code = r#"
def gen(items: list):
    total = 0
    for item in items:
        total += item
        yield total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Conditional yield
// ============================================================================

#[test]
fn test_yield_analysis_conditional() {
    let code = r#"
def gen(items: list):
    for item in items:
        if item > 0:
            yield item
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_conditional_else() {
    let code = r#"
def gen(items: list):
    for item in items:
        if item > 0:
            yield item
        else:
            yield 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Live variable tracking
// ============================================================================

#[test]
fn test_yield_analysis_live_vars() {
    let code = r#"
def gen(start: int, step: int):
    current = start
    while current < 100:
        yield current
        current += step
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_multiple_vars() {
    let code = r#"
def gen(n: int):
    a = 0
    b = 1
    for i in range(n):
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Yield with break
// ============================================================================

#[test]
fn test_yield_analysis_break() {
    let code = r#"
def gen(items: list, limit: int):
    count = 0
    for item in items:
        if count >= limit:
            break
        yield item
        count += 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex generator patterns
// ============================================================================

#[test]
fn test_yield_analysis_fibonacci() {
    let code = r#"
def fibonacci(n: int):
    a = 0
    b = 1
    for i in range(n):
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_powers() {
    let code = r#"
def powers(base: int, limit: int):
    val = 1
    while val < limit:
        yield val
        val = val * base
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_analysis_filter_gen() {
    let code = r#"
def positive_only(items: list):
    for item in items:
        if item > 0:
            yield item
"#;
    assert!(transpile_ok(code));
}

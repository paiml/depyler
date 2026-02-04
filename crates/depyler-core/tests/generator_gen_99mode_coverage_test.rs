//! Coverage tests for rust_gen/generator_gen.rs
//!
//! DEPYLER-99MODE-001: Targets generator_gen.rs (1,462 lines)
//! Covers: yield-based generators, state machine generation,
//! sequential yields, loop yields, tuple yields, captured state.

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
// Basic generator patterns
// ============================================================================

#[test]
fn test_generator_simple_yield() {
    let code = r#"
def gen():
    yield 1
    yield 2
    yield 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_yield_in_loop() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_yield_with_accumulator() {
    let code = r#"
def running_sum(items: list):
    total = 0
    for item in items:
        total += item
        yield total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator with parameters
// ============================================================================

#[test]
fn test_generator_with_int_param() {
    let code = r#"
def countdown(n: int):
    while n > 0:
        yield n
        n -= 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_list_param() {
    let code = r#"
def filter_positive(items: list):
    for item in items:
        if item > 0:
            yield item
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_multiple_params() {
    let code = r#"
def range_gen(start: int, stop: int):
    i = start
    while i < stop:
        yield i
        i += 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Sequential yields (multi-state)
// ============================================================================

#[test]
fn test_generator_sequential_yields() {
    let code = r#"
def stages():
    yield 10
    yield 20
    yield 30
    yield 40
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_mixed_yield_values() {
    let code = r#"
def mixed():
    x = 1
    yield x
    x = x + 1
    yield x
    x = x * 2
    yield x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator with conditional yield
// ============================================================================

#[test]
fn test_generator_conditional_yield() {
    let code = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_yield_with_break() {
    let code = r#"
def limited(items: list, limit: int):
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
fn test_generator_fibonacci() {
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
fn test_generator_powers() {
    let code = r#"
def powers_of_two(n: int):
    val = 1
    for i in range(n):
        yield val
        val = val * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_squares() {
    let code = r#"
def squares(n: int):
    for i in range(n):
        yield i * i
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator usage in functions
// ============================================================================

#[test]
fn test_generator_consumed_by_list() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i * 2

def f(n: int) -> list:
    return list(gen(n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_consumed_by_sum() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i

def f(n: int) -> int:
    return sum(gen(n))
"#;
    assert!(transpile_ok(code));
}

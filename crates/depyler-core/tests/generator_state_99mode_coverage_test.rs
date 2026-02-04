//! Coverage tests for generator_state.rs
//!
//! DEPYLER-99MODE-001: Targets generator_state.rs (1,412 lines)
//! Covers: yield detection, loop variable capture, tuple unpacking,
//! parameter capture, nested generators, with statement generators.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_gen_state_simple_yield() {
    let code = r#"
def gen():
    x = 10
    yield x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_multiple_yields() {
    let code = r#"
def gen():
    yield 1
    yield 2
    yield 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_loop_capture() {
    let code = r#"
def gen():
    for i in range(5):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_while_loop() {
    let code = r#"
def gen():
    count = 0
    while count < 3:
        yield count
        count += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_conditional_yield() {
    let code = r#"
def gen(items: list):
    for item in items:
        if item > 0:
            yield item
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_with_params() {
    let code = r#"
def gen(start: int, end: int):
    for i in range(start, end):
        yield i * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_computed_yield() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i * i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_string_yield() {
    let code = r#"
def gen(text: str):
    for word in text.split():
        yield word
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_fibonacci() {
    let code = r#"
def fib():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_with_accumulator() {
    let code = r#"
def running_sum(items: list):
    total = 0
    for item in items:
        total += item
        yield total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_nested_loops() {
    let code = r#"
def pairs(n: int):
    for i in range(n):
        for j in range(i + 1, n):
            yield (i, j)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_state_early_return() {
    let code = r#"
def limited(items: list, limit: int):
    count = 0
    for item in items:
        if count >= limit:
            return
        yield item
        count += 1
"#;
    assert!(transpile_ok(code));
}

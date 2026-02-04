//! Coverage tests for rust_gen/stdlib_method_gen/random.rs
//!
//! DEPYLER-99MODE-001: Targets random.rs (~644 lines)
//! Covers: random.randint, random.choice, random.shuffle,
//! random.random, random.uniform, random.sample.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_random_randint() {
    let code = r#"
import random

def f() -> int:
    return random.randint(1, 100)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_random_choice() {
    let code = r#"
import random

def f(items: list) -> int:
    return random.choice(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_random_random() {
    let code = r#"
import random

def f() -> float:
    return random.random()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_random_uniform() {
    let code = r#"
import random

def f() -> float:
    return random.uniform(0.0, 1.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_random_shuffle() {
    let code = r#"
import random

def f(items: list) -> list:
    random.shuffle(items)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_random_sample() {
    let code = r#"
import random

def f(items: list, n: int) -> list:
    return random.sample(items, n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_random_in_loop() {
    let code = r#"
import random

def f(n: int) -> list:
    result = []
    for i in range(n):
        result.append(random.randint(0, 100))
    return result
"#;
    assert!(transpile_ok(code));
}

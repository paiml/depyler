//! Coverage tests for generative_repair.rs
//!
//! DEPYLER-99MODE-001: Targets generative_repair.rs (782 lines)
//! Covers: MCTS-guided code synthesis, repair patterns,
//! function/class extraction for repair guidance.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_repair_simple_function() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_function_with_types() {
    let code = r#"
def process(a: int, b: str) -> str:
    return b + str(a)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_class_definition() {
    let code = r#"
class MyClass:
    def __init__(self, value: int):
        self.value = value

    def get(self) -> int:
        return self.value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_complex_function() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_empty_function() {
    let code = "def noop():\n    pass\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_repair_with_exception() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

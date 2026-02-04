//! Coverage tests for rust_gen/func_gen_inference.rs
//!
//! DEPYLER-99MODE-001: Targets func_gen_inference.rs (1,315 lines)
//! Covers: return type inference, nested function detection,
//! IO detection, parameter type inference, async patterns.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Return type inference
// ============================================================================

#[test]
fn test_func_infer_return_int() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_return_str() {
    let code = r#"
def f() -> str:
    return "hello"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_return_float() {
    let code = r#"
def f() -> float:
    return 3.14
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_return_bool() {
    let code = r#"
def f() -> bool:
    return True
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_return_list() {
    let code = r#"
def f() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_return_dict() {
    let code = r#"
def f() -> dict:
    return {"key": "value"}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function detection
// ============================================================================

#[test]
fn test_func_infer_nested_simple() {
    let code = r#"
def outer() -> int:
    def inner(x: int) -> int:
        return x * 2
    return inner(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_nested_closure() {
    let code = r#"
def make_adder(n: int):
    def add(x: int) -> int:
        return x + n
    return add
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_nested_with_default() {
    let code = r#"
def create() -> int:
    def helper(x: int, y: int = 10) -> int:
        return x + y
    return helper(5)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Parameter type inference
// ============================================================================

#[test]
fn test_func_infer_param_typed() {
    let code = r#"
def f(x: int, y: str) -> str:
    return str(x) + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_param_untyped() {
    let code = r#"
def f(x, y):
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_param_default() {
    let code = r#"
def f(x: int = 0, y: str = "default") -> str:
    return str(x) + y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// IO detection
// ============================================================================

#[test]
fn test_func_infer_file_open() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as file:
        return file.read()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple return paths
// ============================================================================

#[test]
fn test_func_infer_multiple_returns() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_return_in_loop() {
    let code = r#"
def f(items: list) -> int:
    for item in items:
        if item > 100:
            return item
    return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex function patterns
// ============================================================================

#[test]
fn test_func_infer_recursive() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_multi_function() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def process(items: list) -> int:
    total = 0
    for item in items:
        total += helper(item)
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Async function patterns
// ============================================================================

#[test]
fn test_func_infer_async() {
    let code = r#"
async def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_func_infer_async_with_await() {
    let code = r#"
async def fetch() -> str:
    return "data"

async def f() -> str:
    result = await fetch()
    return result
"#;
    assert!(transpile_ok(code));
}

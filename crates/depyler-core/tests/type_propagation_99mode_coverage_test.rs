//! Coverage tests for type_propagation.rs
//!
//! DEPYLER-99MODE-001: Targets type_propagation.rs (1,335 lines)
//! Covers: call-site type inference, loop variable types,
//! return type propagation, cross-function type flow.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Call-site type propagation
// ============================================================================

#[test]
fn test_propagate_call_int() {
    let code = r#"
def process(x):
    return x + 1

def f() -> int:
    return process(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_call_str() {
    let code = r#"
def show(msg):
    return msg.upper()

def f() -> str:
    return show("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_call_list() {
    let code = r#"
def first(items):
    return items[0]

def f() -> int:
    return first([1, 2, 3])
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop variable type propagation
// ============================================================================

#[test]
fn test_propagate_for_list_int() {
    let code = r#"
def f() -> int:
    total = 0
    for item in [1, 2, 3]:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_for_dict_keys() {
    let code = r#"
def f() -> list:
    d = {"a": 1, "b": 2}
    result = []
    for key in d:
        result.append(key)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_enumerate() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for idx, val in enumerate(items):
        total += idx
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_zip() {
    let code = r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_string_iter() {
    let code = r#"
def f(text: str) -> int:
    count = 0
    for c in text:
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Return type propagation
// ============================================================================

#[test]
fn test_propagate_return_list() {
    let code = r#"
def get_items() -> list:
    return [1, 2, 3]

def f() -> int:
    items = get_items()
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_return_dict() {
    let code = r#"
def get_config() -> dict:
    return {"key": "value"}

def f() -> str:
    config = get_config()
    return config["key"]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Cross-function flow
// ============================================================================

#[test]
fn test_propagate_chain_calls() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def add_one(x: int) -> int:
    return x + 1

def f(n: int) -> int:
    return add_one(double(n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_nested_function() {
    let code = r#"
def outer(n: int) -> int:
    def inner(x: int) -> int:
        return x + n
    return inner(10)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Optional parameter inference
// ============================================================================

#[test]
fn test_propagate_optional_param() {
    let code = r#"
def f(x: int = 0) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_propagate_none_default() {
    let code = r#"
def f(x=None) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex propagation
// ============================================================================

#[test]
fn test_propagate_multi_step() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def sum_squares(items: list) -> int:
    total = 0
    for item in items:
        total += square(item)
    return total

def f() -> int:
    return sum_squares([1, 2, 3, 4, 5])
"#;
    assert!(transpile_ok(code));
}

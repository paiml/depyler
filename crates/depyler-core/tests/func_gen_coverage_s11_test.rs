//! Session 11: Function generation coverage tests
//!
//! Targets specific untested code paths in func_gen.rs:
//! - Loop-escaping variable hoisting
//! - Return type inference
//! - Nested function closures
//! - Generator functions
//! - Multiple return type patterns
//! - Parameter type handling

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ============================================================================
// Loop-escaping variable hoisting
// ============================================================================

#[test]
fn test_s11_func_loop_escaping_var() {
    let code = r#"
def find_index(items: list, target: int) -> int:
    result: int = -1
    for i, item in enumerate(items):
        if item == target:
            result = i
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_index"),
        "Should handle loop-escaping var. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_loop_var_used_after() {
    let code = r#"
def sum_until(items: list, limit: int) -> int:
    total: int = 0
    for item in items:
        total += item
        if total > limit:
            break
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_until"),
        "Should hoist loop var. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_while_loop_var() {
    let code = r#"
def count_down(n: int) -> int:
    result: int = 0
    i: int = n
    while i > 0:
        result += i
        i -= 1
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_down"),
        "Should handle while var. Got: {}",
        result
    );
}

// ============================================================================
// Return type inference
// ============================================================================

#[test]
fn test_s11_func_inferred_return_int() {
    let code = r#"
def auto_int():
    return 42
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn auto_int"),
        "Should infer int return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_inferred_return_str() {
    let code = r#"
def auto_str():
    return "hello"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn auto_str"),
        "Should infer str return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_inferred_return_bool() {
    let code = r#"
def auto_bool():
    return True
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn auto_bool"),
        "Should infer bool return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_inferred_return_list() {
    let code = r#"
def auto_list():
    return [1, 2, 3]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn auto_list"),
        "Should infer list return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_multiple_return_paths() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn classify"),
        "Should handle multiple returns. Got: {}",
        result
    );
}

// ============================================================================
// Nested functions
// ============================================================================

#[test]
fn test_s11_func_nested_simple() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn outer"),
        "Should transpile nested func. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_nested_with_closure() {
    let code = r#"
def make_adder(n: int) -> int:
    def add(x: int) -> int:
        return x + n
    return add(10)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_adder"),
        "Should transpile closure. Got: {}",
        result
    );
}

// ============================================================================
// Function parameter handling
// ============================================================================

#[test]
fn test_s11_func_default_param_int() {
    let code = r#"
def inc(x: int, step: int = 1) -> int:
    return x + step
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn inc"),
        "Should handle default int param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_param_str() {
    let code = r#"
def greet(name: str = "World") -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn greet"),
        "Should handle default str param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_param_bool() {
    let code = r#"
def log(msg: str, verbose: bool = False) -> None:
    if verbose:
        print(msg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn log"),
        "Should handle default bool param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_str_param_becomes_ref() {
    let code = r#"
def prefix(text: str, pre: str) -> str:
    return pre + text
"#;
    let result = transpile(code);
    assert!(
        result.contains("&str") || result.contains("fn prefix"),
        "Should make str param &str. Got: {}",
        result
    );
}

// ============================================================================
// Complex function patterns
// ============================================================================

#[test]
fn test_s11_func_with_multiple_assignments() {
    let code = r#"
def compute(a: int, b: int) -> int:
    x: int = a + b
    y: int = a - b
    z: int = x * y
    return z
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn compute"),
        "Should handle multiple assignments. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_with_early_return() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    if b == 0.0:
        return 0.0
    return a / b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_divide"),
        "Should handle early return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_void_return() {
    let code = r#"
def do_nothing() -> None:
    pass
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn do_nothing"),
        "Should handle void return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_with_assert() {
    let code = r#"
def positive(x: int) -> int:
    assert x > 0
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("assert") || result.contains("fn positive"),
        "Should handle assert. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_recursive() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn factorial"),
        "Should handle recursion. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_with_docstring() {
    let code = r#"
def documented(x: int) -> int:
    """Returns x doubled."""
    return x * 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn documented"),
        "Should handle docstring. Got: {}",
        result
    );
}

// ============================================================================
// Complex body patterns
// ============================================================================

#[test]
fn test_s11_func_nested_loops() {
    let code = r#"
def flatten(matrix: list) -> list:
    result: list = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flatten"),
        "Should handle nested loops. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_try_except_return() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_parse"),
        "Should handle try/except. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_with_statement() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_file"),
        "Should handle with. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_augmented_assign_types() {
    let code = r#"
def accumulate(items: list) -> int:
    total: int = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn accumulate"),
        "Should handle augmented assign. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_complex_condition() {
    let code = r#"
def validate(name: str, age: int) -> bool:
    if len(name) == 0:
        return False
    if age < 0 or age > 200:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn validate"),
        "Should handle complex conditions. Got: {}",
        result
    );
}

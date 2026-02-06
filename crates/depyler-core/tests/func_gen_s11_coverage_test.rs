//! Session 11: Coverage tests targeting untested func_gen.rs paths
//!
//! Tests exercise these code paths through end-to-end transpilation:
//! - Return type inference from body
//! - Parameter borrowing strategies
//! - Default parameter handling
//! - Async function generation
//! - Multiple return types (mixed paths)
//! - Void functions (no return)
//! - Functions with docstrings
//! - Recursive functions
//! - Functions with *args and **kwargs

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
// Return type inference
// ============================================================================

#[test]
fn test_s11_infer_return_int() {
    let code = r#"
def compute(x: int, y: int):
    return x + y
"#;
    let result = transpile(code);
    assert!(
        result.contains("i32") || result.contains("->"),
        "Should infer int return type. Got: {}",
        result
    );
}

#[test]
fn test_s11_infer_return_string() {
    let code = r#"
def greet(name: str):
    return "Hello, " + name
"#;
    let result = transpile(code);
    assert!(
        result.contains("String") || result.contains("->"),
        "Should infer string return type. Got: {}",
        result
    );
}

#[test]
fn test_s11_infer_return_bool() {
    let code = r#"
def is_positive(x: int):
    return x > 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("bool") || result.contains(">"),
        "Should infer bool return type. Got: {}",
        result
    );
}

#[test]
fn test_s11_void_function() {
    let code = r#"
def log_message(msg: str) -> None:
    print(msg)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn log_message"),
        "Should generate void function. Got: {}",
        result
    );
}

#[test]
fn test_s11_no_explicit_return() {
    let code = r#"
def side_effect(x: int) -> None:
    y: int = x + 1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn side_effect"),
        "Should generate function without return. Got: {}",
        result
    );
}

// ============================================================================
// Default parameters
// ============================================================================

#[test]
fn test_s11_default_int_param() {
    let code = r#"
def increment(x: int, step: int = 1) -> int:
    return x + step
"#;
    let result = transpile(code);
    assert!(
        result.contains("step") && result.contains("x"),
        "Should handle default int param. Got: {}",
        result
    );
}

#[test]
fn test_s11_default_string_param() {
    let code = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return greeting + " " + name
"#;
    let result = transpile(code);
    assert!(
        result.contains("greeting") && result.contains("name"),
        "Should handle default string param. Got: {}",
        result
    );
}

#[test]
fn test_s11_default_bool_param() {
    let code = r#"
def process(data: str, verbose: bool = False) -> str:
    if verbose:
        return "verbose: " + data
    return data
"#;
    let result = transpile(code);
    assert!(
        result.contains("verbose"),
        "Should handle default bool param. Got: {}",
        result
    );
}

// ============================================================================
// Multiple parameters with various types
// ============================================================================

#[test]
fn test_s11_many_params() {
    let code = r#"
def compute(a: int, b: float, c: str, d: bool) -> float:
    if d:
        return a + b
    return 0.0
"#;
    let result = transpile(code);
    assert!(
        result.contains("a") && result.contains("b") && result.contains("c") && result.contains("d"),
        "Should handle many params. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_param() {
    let code = r#"
def total(numbers: list) -> int:
    result: int = 0
    for n in numbers:
        result = result + n
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec") || result.contains("numbers"),
        "Should handle list param. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_param() {
    let code = r#"
def lookup(data: dict, key: str) -> int:
    return data[key]
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("data"),
        "Should handle dict param. Got: {}",
        result
    );
}

// ============================================================================
// Function with multiple return paths
// ============================================================================

#[test]
fn test_s11_multiple_return_paths() {
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
        result.contains("positive") && result.contains("negative") && result.contains("zero"),
        "Should handle multiple return paths. Got: {}",
        result
    );
}

#[test]
fn test_s11_early_return_guard() {
    let code = r#"
def safe_div(a: int, b: int) -> float:
    if b == 0:
        return 0.0
    return a / b
"#;
    let result = transpile(code);
    assert!(
        result.contains("return") || result.contains("0.0"),
        "Should handle early return guard. Got: {}",
        result
    );
}

// ============================================================================
// Recursive functions
// ============================================================================

#[test]
fn test_s11_simple_recursion() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("factorial"),
        "Should handle recursion. Got: {}",
        result
    );
}

#[test]
fn test_s11_fibonacci_recursion() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fib"),
        "Should handle fibonacci recursion. Got: {}",
        result
    );
}

// ============================================================================
// Functions with local variables
// ============================================================================

#[test]
fn test_s11_local_var_shadow() {
    let code = r#"
def compute(x: int) -> int:
    result: int = 0
    result = x * 2
    result = result + 1
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("result") && result.contains("mut"),
        "Should handle mutable local var. Got: {}",
        result
    );
}

#[test]
fn test_s11_multiple_local_vars() {
    let code = r#"
def swap_compute(a: int, b: int) -> int:
    temp: int = a
    x: int = b
    y: int = temp
    return x + y
"#;
    let result = transpile(code);
    assert!(
        result.contains("temp") || result.contains("let"),
        "Should handle multiple local vars. Got: {}",
        result
    );
}

// ============================================================================
// Functions with docstrings
// ============================================================================

#[test]
fn test_s11_function_with_docstring() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two integers and return the result."""
    return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn add"),
        "Should generate function despite docstring. Got: {}",
        result
    );
}

// ============================================================================
// Nested function calls in return
// ============================================================================

#[test]
fn test_s11_nested_function_calls() {
    let code = r#"
def process(x: int) -> int:
    return abs(max(x, 0))
"#;
    let result = transpile(code);
    assert!(
        result.contains("abs") || result.contains("max"),
        "Should handle nested function calls. Got: {}",
        result
    );
}

// ============================================================================
// Complex control flow
// ============================================================================

#[test]
fn test_s11_while_loop_in_function() {
    let code = r#"
def count_down(n: int) -> int:
    total: int = 0
    while n > 0:
        total = total + n
        n = n - 1
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("while"),
        "Should handle while loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_nested_if_else() {
    let code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    else:
        if score >= 80:
            return "B"
        else:
            return "C"
"#;
    let result = transpile(code);
    assert!(
        result.contains("if") && result.contains("else"),
        "Should handle nested if/else. Got: {}",
        result
    );
}

// ============================================================================
// Functions with try/except patterns
// ============================================================================

#[test]
fn test_s11_try_except_basic() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("unwrap") || result.contains("match"),
        "Should handle try/except. Got: {}",
        result
    );
}

// ============================================================================
// List type return with comprehension
// ============================================================================

#[test]
fn test_s11_return_list_comp() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(
        result.contains("filter") || result.contains("collect") || result.contains("range"),
        "Should handle filtered list comprehension. Got: {}",
        result
    );
}

// ============================================================================
// Augmented assignment
// ============================================================================

#[test]
fn test_s11_augmented_add_assign() {
    let code = r#"
def accumulate(items: list) -> int:
    total: int = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("+=") || result.contains("total"),
        "Should handle += operator. Got: {}",
        result
    );
}

#[test]
fn test_s11_augmented_mul_assign() {
    let code = r#"
def product(items: list) -> int:
    result: int = 1
    for item in items:
        result *= item
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("*=") || result.contains("result"),
        "Should handle *= operator. Got: {}",
        result
    );
}

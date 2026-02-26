//! Session 11: Edge case coverage tests for uncommon but valid Python patterns
//!
//! These tests exercise less common language features and edge cases that
//! are valid Python but may not be covered by typical tests.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ============================================================================
// Empty function bodies
// ============================================================================

#[test]
fn test_s11_edge_pass_only_function() {
    let code = r#"
def noop() -> None:
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn noop"), "Should generate noop function. Got: {}", result);
}

#[test]
fn test_s11_edge_ellipsis_body() {
    let code = r#"
def stub() -> int:
    ...
"#;
    let result = transpile(code);
    assert!(result.contains("fn stub"), "Should generate stub function. Got: {}", result);
}

// ============================================================================
// Single-line expressions
// ============================================================================

#[test]
fn test_s11_edge_single_return_literal() {
    let code = r#"
def get_42() -> int:
    return 42
"#;
    let result = transpile(code);
    assert!(result.contains("42"), "Should return 42. Got: {}", result);
}

#[test]
fn test_s11_edge_return_negative_literal() {
    let code = r#"
def get_neg() -> int:
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("-1") || result.contains("- 1"), "Should return -1. Got: {}", result);
}

#[test]
fn test_s11_edge_return_large_int() {
    let code = r#"
def big_number() -> int:
    return 1000000
"#;
    let result = transpile(code);
    assert!(result.contains("1000000"), "Should handle large int. Got: {}", result);
}

#[test]
fn test_s11_edge_return_float_literal() {
    let code = r#"
def get_pi() -> float:
    return 3.14159
"#;
    let result = transpile(code);
    assert!(
        result.contains("3.14159") || result.contains("f64"),
        "Should return float. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_return_empty_string() {
    let code = r#"
def empty() -> str:
    return ""
"#;
    let result = transpile(code);
    assert!(
        result.contains("\"\"") || result.contains("String::new"),
        "Should return empty string. Got: {}",
        result
    );
}

// ============================================================================
// Variable naming edge cases
// ============================================================================

#[test]
fn test_s11_edge_single_char_vars() {
    let code = r#"
def calc(a: int, b: int, c: int) -> int:
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn calc"), "Should handle single-char vars. Got: {}", result);
}

#[test]
fn test_s11_edge_long_variable_names() {
    let code = r#"
def compute_with_long_names(first_operand: int, second_operand: int) -> int:
    intermediate_result: int = first_operand + second_operand
    return intermediate_result
"#;
    let result = transpile(code);
    assert!(
        result.contains("intermediate_result"),
        "Should handle long var names. Got: {}",
        result
    );
}

// ============================================================================
// Multiple assignments
// ============================================================================

#[test]
fn test_s11_edge_reassign_different_expression() {
    let code = r#"
def multi_assign(x: int) -> int:
    result: int = x
    result = result + 1
    result = result * 2
    result = result - 3
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("mut"), "Should have mutable variable. Got: {}", result);
}

// ============================================================================
// Nested data structure creation
// ============================================================================

#[test]
fn test_s11_edge_list_of_tuples() {
    let code = r#"
def pairs() -> list:
    return [(1, 2), (3, 4), (5, 6)]
"#;
    let result = transpile(code);
    assert!(
        result.contains("vec!") || result.contains("Vec"),
        "Should create list of tuples. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_dict_literal_complex() {
    let code = r#"
def config() -> dict:
    return {"host": "localhost", "port": "8080", "debug": "true"}
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("localhost"),
        "Should create dict. Got: {}",
        result
    );
}

// ============================================================================
// Range variations
// ============================================================================

#[test]
fn test_s11_edge_range_with_step() {
    let code = r#"
def evens_up_to(n: int) -> list:
    result: list = []
    for i in range(0, n, 2):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens_up_to"), "Should generate function. Got: {}", result);
}

#[test]
fn test_s11_edge_range_negative_step() {
    let code = r#"
def countdown(n: int) -> list:
    result: list = []
    for i in range(n, 0, -1):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn countdown"), "Should generate countdown. Got: {}", result);
}

// ============================================================================
// String escape sequences
// ============================================================================

#[test]
fn test_s11_edge_string_with_newline() {
    let code = r#"
def multiline() -> str:
    return "line1\nline2"
"#;
    let result = transpile(code);
    assert!(result.contains("fn multiline"), "Should handle string with newline. Got: {}", result);
}

#[test]
fn test_s11_edge_string_with_tab() {
    let code = r#"
def tabbed() -> str:
    return "col1\tcol2"
"#;
    let result = transpile(code);
    assert!(result.contains("fn tabbed"), "Should handle string with tab. Got: {}", result);
}

// ============================================================================
// Comparison operators
// ============================================================================

#[test]
fn test_s11_edge_equal_comparison() {
    let code = r#"
def is_equal(a: int, b: int) -> bool:
    return a == b
"#;
    let result = transpile(code);
    assert!(result.contains("=="), "Should handle == comparison. Got: {}", result);
}

#[test]
fn test_s11_edge_not_equal_comparison() {
    let code = r#"
def is_different(a: int, b: int) -> bool:
    return a != b
"#;
    let result = transpile(code);
    assert!(result.contains("!="), "Should handle != comparison. Got: {}", result);
}

#[test]
fn test_s11_edge_gt_lt_comparisons() {
    let code = r#"
def ordering(a: int, b: int) -> int:
    if a > b:
        return 1
    elif a < b:
        return -1
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains(">") && result.contains("<"), "Should handle > and <. Got: {}", result);
}

// ============================================================================
// Complex expression nesting
// ============================================================================

#[test]
fn test_s11_edge_deeply_nested_expr() {
    let code = r#"
def complex_expr(a: int, b: int, c: int) -> int:
    return (a + b) * (c - a) + (b * c)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn complex_expr"),
        "Should handle complex expression. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_chained_method_calls() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(
        result.contains("trim") || result.contains("to_lowercase"),
        "Should chain methods. Got: {}",
        result
    );
}

// ============================================================================
// List operations edge cases
// ============================================================================

#[test]
fn test_s11_edge_list_concatenation() {
    let code = r#"
def concat(a: list, b: list) -> list:
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn concat"), "Should handle list concat. Got: {}", result);
}

#[test]
fn test_s11_edge_nested_if_in_loop() {
    let code = r#"
def process(items: list) -> list:
    result: list = []
    for item in items:
        if item > 0:
            if item < 100:
                result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Should handle nested if in loop. Got: {}", result);
}

// ============================================================================
// Multiple return value functions
// ============================================================================

#[test]
fn test_s11_edge_return_tuple_unpacked() {
    let code = r#"
from typing import Tuple

def divmod_func(a: int, b: int) -> Tuple[int, int]:
    quotient: int = a // b
    remainder: int = a % b
    return (quotient, remainder)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divmod_func"), "Should generate divmod function. Got: {}", result);
}

// ============================================================================
// Boolean expressions in assignments
// ============================================================================

#[test]
fn test_s11_edge_bool_assignment() {
    let code = r#"
def check(x: int, y: int) -> bool:
    both_positive: bool = x > 0 and y > 0
    return both_positive
"#;
    let result = transpile(code);
    assert!(result.contains("&&"), "Should handle bool assignment. Got: {}", result);
}

// ============================================================================
// For loop with dict.items()
// ============================================================================

#[test]
fn test_s11_edge_dict_items_iteration() {
    let code = r#"
def print_dict(data: dict) -> None:
    for key, value in data.items():
        pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_dict"), "Should generate function. Got: {}", result);
}

// ============================================================================
// String formatting patterns
// ============================================================================

#[test]
fn test_s11_edge_format_with_number() {
    let code = r#"
def format_count(name: str, count: int) -> str:
    return f"{name}: {count}"
"#;
    let result = transpile(code);
    assert!(result.contains("format!"), "Should use format!. Got: {}", result);
}

#[test]
fn test_s11_edge_format_multiple_values() {
    let code = r#"
def coordinates(x: float, y: float) -> str:
    return f"({x}, {y})"
"#;
    let result = transpile(code);
    assert!(result.contains("format!"), "Should use format!. Got: {}", result);
}

// ============================================================================
// Math module usage
// ============================================================================

#[test]
fn test_s11_edge_math_floor() {
    let code = r#"
import math

def floor_val(x: float) -> int:
    return int(math.floor(x))
"#;
    let result = transpile(code);
    assert!(result.contains("floor"), "Should handle math.floor. Got: {}", result);
}

#[test]
fn test_s11_edge_math_ceil() {
    let code = r#"
import math

def ceil_val(x: float) -> int:
    return int(math.ceil(x))
"#;
    let result = transpile(code);
    assert!(result.contains("ceil"), "Should handle math.ceil. Got: {}", result);
}

// ============================================================================
// Global constant patterns
// ============================================================================

#[test]
fn test_s11_edge_function_uses_constant() {
    let code = r#"
MAX_SIZE: int = 100

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let result = transpile(code);
    assert!(
        result.contains("MAX_SIZE") || result.contains("100"),
        "Should handle constant. Got: {}",
        result
    );
}

// ============================================================================
// Type annotation edge cases
// ============================================================================

#[test]
fn test_s11_edge_bytes_return() {
    let code = r#"
def encode(s: str) -> bytes:
    return s.encode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode"), "Should generate encode function. Got: {}", result);
}

// ============================================================================
// Walrus operator (if supported)
// ============================================================================

#[test]
fn test_s11_edge_complex_conditional() {
    let code = r#"
def safe_get(items: list, idx: int) -> int:
    if idx >= 0 and idx < len(items):
        return items[idx]
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Should generate safe_get. Got: {}", result);
}

// ============================================================================
// Exception-like patterns in Rust
// ============================================================================

#[test]
fn test_s11_edge_assert_statement() {
    let code = r#"
def positive_only(x: int) -> int:
    assert x > 0
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("assert") || result.contains("panic"),
        "Should handle assert. Got: {}",
        result
    );
}

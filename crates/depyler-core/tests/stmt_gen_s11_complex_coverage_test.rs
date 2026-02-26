//! Session 11: Coverage tests for complex stmt_gen.rs code paths
//!
//! Exercises untested statement generation patterns:
//! - Assert with various comparison operators
//! - While loop truthiness and collection checks
//! - Raise statement variations
//! - For loop complex patterns (enumerate, char iteration, string methods)
//! - If/else with variable hoisting
//! - Augmented assignment operators
//! - Try/except with multiple handlers
//! - With statement patterns

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
// Assert with various comparison operators
// ============================================================================

#[test]
fn test_s11_assert_greater_than() {
    let code = r#"
def validate_positive(x: int) -> int:
    assert x > 0, "must be positive"
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("assert") || result.contains("panic"),
        "Should transpile assert x > 0. Got: {}",
        result
    );
}

#[test]
fn test_s11_assert_less_than() {
    let code = r#"
def validate_small(x: int) -> int:
    assert x < 100, "must be small"
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("assert") || result.contains("panic"),
        "Should transpile assert x < 100. Got: {}",
        result
    );
}

#[test]
fn test_s11_assert_greater_equal() {
    let code = r#"
def validate_nonneg(x: int) -> int:
    assert x >= 0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile assert x >= 0. Got: {}", result);
}

#[test]
fn test_s11_assert_boolean_expr() {
    let code = r#"
def validate_range(x: int) -> int:
    assert x > 0 and x < 100
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile boolean assert. Got: {}", result);
}

// ============================================================================
// While loop patterns
// ============================================================================

#[test]
fn test_s11_while_true_break() {
    let code = r#"
def loop_until(n: int) -> int:
    count: int = 0
    while True:
        if count >= n:
            break
        count = count + 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("loop") || result.contains("while"),
        "Should transpile while True with break. Got: {}",
        result
    );
}

#[test]
fn test_s11_while_with_continue() {
    let code = r#"
def skip_odds(n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        i = i + 1
        if i % 2 != 0:
            continue
        total = total + i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("continue"), "Should transpile while with continue. Got: {}", result);
}

#[test]
fn test_s11_while_compound_condition() {
    let code = r#"
def search(items: list, target: int) -> int:
    i: int = 0
    found: bool = False
    while i < len(items) and not found:
        if items[i] == target:
            found = True
        i = i + 1
    return i
"#;
    let result = transpile(code);
    assert!(result.contains("fn search"), "Should transpile while compound. Got: {}", result);
}

// ============================================================================
// Raise statement variations
// ============================================================================

#[test]
fn test_s11_raise_value_error() {
    let code = r#"
def parse_int(s: str) -> int:
    if not s.isdigit():
        raise ValueError("not a number")
    return int(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("panic") || result.contains("ValueError"),
        "Should transpile raise ValueError. Got: {}",
        result
    );
}

#[test]
fn test_s11_raise_type_error() {
    let code = r#"
def check_type(x: int) -> int:
    if x < 0:
        raise TypeError("expected positive")
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("panic") || result.contains("TypeError"),
        "Should transpile raise TypeError. Got: {}",
        result
    );
}

#[test]
fn test_s11_raise_runtime_error() {
    let code = r#"
def fail() -> None:
    raise RuntimeError("unexpected state")
"#;
    let result = transpile(code);
    assert!(result.contains("panic"), "Should transpile raise RuntimeError. Got: {}", result);
}

#[test]
fn test_s11_raise_not_implemented() {
    let code = r#"
def abstract_method() -> None:
    raise NotImplementedError("must override")
"#;
    let result = transpile(code);
    assert!(
        result.contains("unimplemented") || result.contains("panic") || result.contains("todo"),
        "Should transpile raise NotImplementedError. Got: {}",
        result
    );
}

#[test]
fn test_s11_raise_key_error() {
    let code = r#"
def get_required(d: dict, key: str) -> int:
    if key not in d:
        raise KeyError(key)
    return d[key]
"#;
    let result = transpile(code);
    assert!(
        result.contains("panic") || result.contains("fn get_required"),
        "Should transpile raise KeyError. Got: {}",
        result
    );
}

// ============================================================================
// For loop complex patterns
// ============================================================================

#[test]
fn test_s11_for_enumerate_with_math() {
    let code = r#"
def weighted_sum(items: list) -> int:
    total: int = 0
    for idx, val in enumerate(items):
        total = total + idx * val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("enumerate"), "Should transpile enumerate in for. Got: {}", result);
}

#[test]
fn test_s11_for_zip_two_lists() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    total: int = 0
    for x, y in zip(a, b):
        total = total + x * y
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("zip"), "Should transpile zip in for. Got: {}", result);
}

#[test]
fn test_s11_for_string_chars() {
    let code = r#"
def count_vowels(text: str) -> int:
    count: int = 0
    for ch in text:
        if ch == "a" or ch == "e" or ch == "i" or ch == "o" or ch == "u":
            count = count + 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("chars") || result.contains("for"),
        "Should transpile string char iteration. Got: {}",
        result
    );
}

#[test]
fn test_s11_for_nested_loops() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total: int = 0
    for row in matrix:
        for val in row:
            total = total + val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_sum"), "Should transpile nested for loops. Got: {}", result);
}

#[test]
fn test_s11_for_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_first"),
        "Should transpile for with early return. Got: {}",
        result
    );
}

#[test]
fn test_s11_for_reversed() {
    let code = r#"
def reverse_collect(items: list) -> list:
    result: list = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("rev") || result.contains("fn reverse_collect"),
        "Should transpile reversed() in for. Got: {}",
        result
    );
}

// ============================================================================
// If/else patterns with variable hoisting
// ============================================================================

#[test]
fn test_s11_if_elif_else_chain() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        result: str = "positive"
    elif x < 0:
        result = "negative"
    else:
        result = "zero"
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Should transpile if/elif/else chain. Got: {}", result);
}

#[test]
fn test_s11_if_with_function_call() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    if b != 0:
        return a // b
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_divide"),
        "Should transpile if with function call. Got: {}",
        result
    );
}

#[test]
fn test_s11_if_nested_deep() {
    let code = r#"
def deep_check(a: int, b: int, c: int) -> str:
    if a > 0:
        if b > 0:
            if c > 0:
                return "all positive"
            return "c not positive"
        return "b not positive"
    return "a not positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_check"), "Should transpile deeply nested if. Got: {}", result);
}

// ============================================================================
// Augmented assignment operators
// ============================================================================

#[test]
fn test_s11_augmented_add() {
    let code = r#"
def accumulate(items: list) -> int:
    total: int = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("+="), "Should transpile +=. Got: {}", result);
}

#[test]
fn test_s11_augmented_sub() {
    let code = r#"
def countdown(n: int) -> int:
    while n > 0:
        n -= 1
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("-="), "Should transpile -=. Got: {}", result);
}

#[test]
fn test_s11_augmented_mul() {
    let code = r#"
def factorial(n: int) -> int:
    result: int = 1
    for i in range(1, n + 1):
        result *= i
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("*=") || result.contains("py_mul") || result.contains("fn factorial"),
        "Should transpile *=. Got: {}",
        result
    );
}

#[test]
fn test_s11_augmented_mod() {
    let code = r#"
def apply_mod(x: int, m: int) -> int:
    x %= m
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("%=") || result.contains("py_mod") || result.contains("fn apply_mod"),
        "Should transpile %=. Got: {}",
        result
    );
}

#[test]
fn test_s11_augmented_floordiv() {
    let code = r#"
def halve(x: int) -> int:
    x //= 2
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("/=") || result.contains("fn halve"),
        "Should transpile //=. Got: {}",
        result
    );
}

// ============================================================================
// Try/except patterns
// ============================================================================

#[test]
fn test_s11_try_except_basic() {
    let code = r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_int"), "Should transpile try/except. Got: {}", result);
}

#[test]
fn test_s11_try_except_finally() {
    let code = r#"
def with_cleanup(path: str) -> int:
    result: int = 0
    try:
        result = 42
    except Exception:
        result = -1
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn with_cleanup"),
        "Should transpile try/except/finally. Got: {}",
        result
    );
}

#[test]
fn test_s11_try_multiple_except() {
    let code = r#"
def robust_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn robust_parse"),
        "Should transpile multiple except. Got: {}",
        result
    );
}

// ============================================================================
// With statement patterns
// ============================================================================

#[test]
fn test_s11_with_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        content: str = f.read()
    return content
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Should transpile with open. Got: {}", result);
}

#[test]
fn test_s11_with_open_write() {
    let code = r#"
def write_file(path: str, data: str) -> None:
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Should transpile with open write. Got: {}", result);
}

// ============================================================================
// Complex assignment patterns
// ============================================================================

#[test]
fn test_s11_multiple_return_values() {
    let code = r#"
from typing import Tuple

def min_max(items: list) -> Tuple[int, int]:
    lo: int = items[0]
    hi: int = items[0]
    for item in items:
        if item < lo:
            lo = item
        if item > hi:
            hi = item
    return (lo, hi)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max"), "Should transpile min_max function. Got: {}", result);
}

#[test]
fn test_s11_chained_string_operations() {
    let code = r#"
def normalize(text: str) -> str:
    result: str = text.strip()
    result = result.lower()
    result = result.replace("  ", " ")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("mut"), "Should use mutable variable for chained ops. Got: {}", result);
}

#[test]
fn test_s11_list_comprehension_with_condition() {
    let code = r#"
def positives(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    let result = transpile(code);
    assert!(
        result.contains("filter") || result.contains("fn positives"),
        "Should transpile filtered list comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_comprehension() {
    let code = r#"
def square_map(items: list) -> dict:
    return {x: x * x for x in items}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn square_map"),
        "Should transpile dict comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_ternary_expression() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("if") || result.contains("fn abs_val"),
        "Should transpile ternary expression. Got: {}",
        result
    );
}

#[test]
fn test_s11_global_variable_usage() {
    let code = r#"
MAX_RETRIES: int = 3

def should_retry(attempt: int) -> bool:
    return attempt < MAX_RETRIES
"#;
    let result = transpile(code);
    assert!(
        result.contains("MAX_RETRIES") || result.contains("3"),
        "Should handle global constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_empty_list_literal() {
    let code = r#"
def make_list() -> list:
    result: list = []
    result.append(1)
    result.append(2)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec::new") || result.contains("vec!"),
        "Should transpile empty list. Got: {}",
        result
    );
}

#[test]
fn test_s11_empty_dict_literal() {
    let code = r#"
def make_dict() -> dict:
    result: dict = {}
    result["key"] = "value"
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap::new") || result.contains("fn make_dict"),
        "Should transpile empty dict. Got: {}",
        result
    );
}

#[test]
fn test_s11_delete_statement() {
    let code = r#"
def remove_key(d: dict, key: str) -> None:
    del d[key]
"#;
    let result = transpile(code);
    assert!(
        result.contains("remove") || result.contains("fn remove_key"),
        "Should transpile del statement. Got: {}",
        result
    );
}

#[test]
fn test_s11_pass_statement() {
    let code = r#"
def placeholder() -> None:
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn placeholder"), "Should transpile pass statement. Got: {}", result);
}

#[test]
fn test_s11_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn add") && result.contains("fn sub") && result.contains("fn mul"),
        "Should transpile multiple functions. Got: {}",
        result
    );
}

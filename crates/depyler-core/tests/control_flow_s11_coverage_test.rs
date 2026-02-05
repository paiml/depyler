//! DEPYLER-99MODE-S11: Integration tests targeting control flow and complex patterns
//!
//! Tests for: nested functions with captures, try/except with variable hoisting,
//! complex control flow patterns, generators, comprehensions with filters,
//! and type inference edge cases.

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

// ===== Nested Functions with Captures =====

#[test]
fn test_s11_nested_fn_captures_param() {
    let code = r#"
def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"));
    assert!(result.contains("adder"));
}

#[test]
fn test_s11_nested_fn_captures_multiple() {
    let code = r#"
def make_linear(a: int, b: int):
    def linear(x: int) -> int:
        return a * x + b
    return linear
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_linear"));
    assert!(result.contains("linear"));
}

#[test]
fn test_s11_nested_fn_no_capture() {
    let code = r#"
def outer(x: int) -> int:
    def helper(n: int) -> int:
        return n * 2
    return helper(x) + 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"));
    assert!(result.contains("helper"));
}

#[test]
fn test_s11_nested_fn_recursive() {
    let code = r#"
def factorial(n: int) -> int:
    def fact_helper(n: int, acc: int) -> int:
        if n <= 1:
            return acc
        return fact_helper(n - 1, n * acc)
    return fact_helper(n, 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial"));
    assert!(result.contains("fact_helper"));
}

#[test]
fn test_s11_nested_fn_with_list_param() {
    let code = r#"
from typing import List

def process(items: List[int]) -> int:
    def sum_items(data: List[int]) -> int:
        total = 0
        for x in data:
            total = total + x
        return total
    return sum_items(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"));
}

// ===== Try/Except Patterns =====

#[test]
fn test_s11_try_except_basic() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_div"));
}

#[test]
fn test_s11_try_except_finally() {
    let code = r#"
def with_cleanup(x: int) -> int:
    result = 0
    try:
        result = x * 2
    except ValueError:
        result = -1
    finally:
        print("done")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_cleanup"));
}

#[test]
fn test_s11_try_except_multiple_handlers() {
    let code = r#"
def parse_value(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_value"));
}

#[test]
fn test_s11_try_except_with_variable_hoisting() {
    let code = r#"
def read_file(path: str) -> str:
    content = ""
    try:
        content = "file data"
    except FileNotFoundError:
        content = "not found"
    return content
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"));
    assert!(result.contains("content"));
}

#[test]
fn test_s11_try_except_with_binding() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError as e:
        print(e)
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"));
}

#[test]
fn test_s11_try_except_nested() {
    let code = r#"
def nested_try(x: int) -> int:
    try:
        try:
            return x // 0
        except ZeroDivisionError:
            return x
    except Exception:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_try"));
}

// ===== Complex Control Flow =====

#[test]
fn test_s11_while_with_break() {
    let code = r#"
def find_first_even(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] % 2 == 0:
            return items[i]
        i = i + 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first_even"));
    assert!(result.contains("while") || result.contains("loop"));
}

#[test]
fn test_s11_for_with_else() {
    let code = r#"
def search(items: list, target: int) -> bool:
    for item in items:
        if item == target:
            return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn search"));
}

#[test]
fn test_s11_nested_loops() {
    let code = r#"
from typing import List

def matrix_sum(matrix: List[List[int]]) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total = total + val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_sum"));
}

#[test]
fn test_s11_loop_with_continue() {
    let code = r#"
from typing import List

def sum_positives(items: List[int]) -> int:
    total = 0
    for x in items:
        if x < 0:
            continue
        total = total + x
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positives"));
    assert!(result.contains("continue"));
}

#[test]
fn test_s11_loop_with_break() {
    let code = r#"
from typing import List

def first_negative(items: List[int]) -> int:
    for x in items:
        if x < 0:
            return x
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_negative"));
}

#[test]
fn test_s11_chained_elif() {
    let code = r#"
def classify(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"));
    assert!(result.contains("else if") || result.contains("match"));
}

// ===== Generator Expressions =====

#[test]
fn test_s11_sum_generator() {
    let code = r#"
from typing import List

def total_even(items: List[int]) -> int:
    return sum(x for x in items if x % 2 == 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total_even"));
}

#[test]
fn test_s11_any_generator() {
    let code = r#"
from typing import List

def has_large(items: List[int], threshold: int) -> bool:
    return any(x > threshold for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_large"));
}

#[test]
fn test_s11_all_generator() {
    let code = r#"
from typing import List

def all_positive(items: List[int]) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"));
}

#[test]
fn test_s11_max_generator() {
    let code = r#"
from typing import List

def max_squared(items: List[int]) -> int:
    return max(x * x for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_squared"));
}

// ===== Comprehensions with Complex Filters =====

#[test]
fn test_s11_list_comp_multiple_ifs() {
    let code = r#"
from typing import List

def small_even(n: int) -> List[int]:
    return [x for x in range(n) if x % 2 == 0 if x < 10]
"#;
    let result = transpile(code);
    assert!(result.contains("fn small_even"));
}

#[test]
fn test_s11_list_comp_with_method_call() {
    let code = r#"
from typing import List

def upper_names(names: List[str]) -> List[str]:
    return [name.upper() for name in names]
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_names"));
    assert!(result.contains("to_uppercase") || result.contains("upper"));
}

#[test]
fn test_s11_list_comp_nested() {
    let code = r#"
from typing import List

def flatten(matrix: List[List[int]]) -> List[int]:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"));
}

#[test]
fn test_s11_list_comp_with_ternary() {
    let code = r#"
from typing import List

def abs_values(items: List[int]) -> List[int]:
    return [x if x >= 0 else -x for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_values"));
}

// ===== Type Inference Edge Cases =====

#[test]
fn test_s11_infer_type_from_dict_comp() {
    let code = r#"
def char_counts(s: str) -> dict:
    counts = {}
    for c in s:
        counts[c] = counts.get(c, 0) + 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_counts"));
}

#[test]
fn test_s11_infer_type_from_ternary() {
    let code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_val"));
}

#[test]
fn test_s11_infer_type_from_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"));
}

#[test]
fn test_s11_infer_type_from_augmented_assign() {
    let code = r#"
def accumulate(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"));
    assert!(result.contains("+=") || result.contains("+ i"));
}

// ===== Assert Statements =====

#[test]
fn test_s11_assert_basic() {
    let code = r#"
def validated(x: int) -> int:
    assert x > 0
    return x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn validated"));
    assert!(result.contains("assert"));
}

#[test]
fn test_s11_assert_with_message() {
    let code = r#"
def validated_msg(x: int) -> int:
    assert x > 0, "x must be positive"
    return x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn validated_msg"));
}

// ===== Pass Statement =====

#[test]
fn test_s11_pass_in_function() {
    let code = r#"
def placeholder():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn placeholder"));
}

#[test]
fn test_s11_pass_in_class() {
    let code = r#"
class Empty:
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("Empty"));
}

// ===== With Statement =====

#[test]
fn test_s11_with_statement_basic() {
    let code = r#"
def read_data(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_data"));
}

// ===== Lambda Expressions =====

#[test]
fn test_s11_lambda_basic() {
    let code = r#"
def apply_func(x: int) -> int:
    f = lambda n: n * 2
    return f(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_func"));
}

#[test]
fn test_s11_lambda_multiarg() {
    let code = r#"
def make_pair(x: int, y: int) -> int:
    combine = lambda a, b: a + b
    return combine(x, y)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_pair"));
}

#[test]
fn test_s11_lambda_in_sorted() {
    let code = r#"
from typing import List

def sort_by_second(items: List[tuple]) -> List[tuple]:
    return sorted(items, key=lambda x: x[1])
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_second"));
}

// ===== F-String Edge Cases =====

#[test]
fn test_s11_fstring_with_expression() {
    let code = r#"
def format_expr(x: int) -> str:
    return f"result: {x * 2 + 1}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_expr"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s11_fstring_multiple_vars() {
    let code = r#"
def template(name: str, age: int, city: str) -> str:
    return f"{name}, {age} years old, from {city}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn template"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s11_fstring_nested_calls() {
    let code = r#"
def format_len(s: str) -> str:
    return f"length: {len(s)}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_len"));
}

// ===== Class Patterns =====

#[test]
fn test_s11_class_with_methods() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count = self.count + 1

    def get_count(self) -> int:
        return self.count
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"));
    assert!(result.contains("increment"));
    assert!(result.contains("get_count"));
}

#[test]
fn test_s11_class_with_class_variable() {
    let code = r#"
class Config:
    MAX_SIZE = 100
    MIN_SIZE = 1

    def __init__(self, size: int):
        self.size = size

    def is_valid(self) -> bool:
        return self.size >= 1 and self.size <= 100
"#;
    let result = transpile(code);
    assert!(result.contains("Config"));
    assert!(result.contains("is_valid"));
}

// ===== Multiple Return Types =====

#[test]
fn test_s11_early_return_pattern() {
    let code = r#"
from typing import List

def find_item(items: List[int], target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_item"));
}

#[test]
fn test_s11_guard_clause_pattern() {
    let code = r#"
def process(x: int) -> str:
    if x < 0:
        return "negative"
    if x == 0:
        return "zero"
    if x > 100:
        return "large"
    return "normal"
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"));
}

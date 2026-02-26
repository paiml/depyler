//! Session 12 Batch 17: Tests for codegen helper modules
//!
//! Targets cold paths in helper modules under rust_gen/:
//! - unary_ops (bitwise not, positive, complex negation)
//! - binary_ops (floor div, modulo, power, bitwise)
//! - builtin_conversions (int/float/str/bool conversions with edge cases)
//! - string_analysis (string method detection and optimization)
//! - type_coercion (numeric promotion, collection type inference)
//! - format (f-string with expressions, format spec)
//! - exception_helpers (try/except codegen paths)
//! - import_gen (various import patterns)
//! - walrus_helpers (named expression codegen)
//! - generator_gen (yield/generator patterns)

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

// ===== Unary ops edge cases =====

#[test]
fn test_s12_double_negation() {
    let code = r#"
def double_neg(x: int) -> int:
    return -(-x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_neg"), "Got: {}", result);
}

#[test]
fn test_s12_not_boolean_expr() {
    let code = r#"
def negate_condition(x: int) -> bool:
    return not (x > 5 and x < 10)
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate_condition"), "Got: {}", result);
}

#[test]
fn test_s12_not_in_if_condition() {
    let code = r#"
def check_empty(items: list) -> bool:
    if not items:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_empty"), "Got: {}", result);
}

// ===== Binary ops edge cases =====

#[test]
fn test_s12_integer_division_chain() {
    let code = r#"
def divide_three(a: int, b: int, c: int) -> int:
    return a // b // c
"#;
    let result = transpile(code);
    assert!(result.contains("fn divide_three"), "Got: {}", result);
}

#[test]
fn test_s12_mixed_arithmetic() {
    let code = r#"
def compute(a: int, b: int, c: int) -> int:
    return (a + b) * c - (a // b) + (c % a)
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Got: {}", result);
}

#[test]
fn test_s12_power_in_expression() {
    let code = r#"
def distance(x: float, y: float) -> float:
    return (x ** 2 + y ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Got: {}", result);
}

// ===== Type coercion patterns =====

#[test]
fn test_s12_int_float_mixed_arithmetic() {
    let code = r#"
def average(items: list) -> float:
    total = sum(items)
    count = len(items)
    return total / count
"#;
    let result = transpile(code);
    assert!(result.contains("fn average"), "Got: {}", result);
}

#[test]
fn test_s12_string_concatenation_with_conversion() {
    let code = r#"
def format_score(name: str, score: int) -> str:
    return name + ": " + str(score)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_score"), "Got: {}", result);
}

// ===== Format patterns =====

#[test]
fn test_s12_fstring_with_method_call() {
    let code = r#"
def format_name(name: str) -> str:
    return f"Hello, {name.upper()}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_name"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_with_arithmetic() {
    let code = r#"
def show_total(a: int, b: int) -> str:
    return f"{a} + {b} = {a + b}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_total"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_nested_braces() {
    let code = r#"
def format_dict_value(d: dict, key: str) -> str:
    return f"Value: {d[key]}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_dict_value"), "Got: {}", result);
}

#[test]
fn test_s12_percent_format() {
    let code = r#"
def format_old_style(name: str, age: int) -> str:
    return "%s is %d years old" % (name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_old_style"), "Got: {}", result);
}

// ===== Import patterns =====

#[test]
fn test_s12_import_from_os_path() {
    let code = r#"
from os.path import join, exists

def check_path(base: str, name: str) -> bool:
    path = join(base, name)
    return exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_path"), "Got: {}", result);
}

#[test]
fn test_s12_import_from_typing_multiple() {
    let code = r#"
from typing import List, Dict, Optional, Tuple

def process(items: List[int]) -> Optional[Dict[str, int]]:
    if not items:
        return None
    return {"count": len(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_import_math_specific() {
    let code = r#"
from math import sqrt, ceil, floor

def process_math(x: float) -> tuple:
    return (sqrt(x), ceil(x), floor(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_math"), "Got: {}", result);
}

// ===== Generator patterns =====

#[test]
fn test_s12_generator_with_filter() {
    let code = r#"
def positive_gen(items: list):
    for item in items:
        if item > 0:
            yield item
"#;
    let result = transpile(code);
    assert!(result.contains("positive_gen"), "Got: {}", result);
}

#[test]
fn test_s12_generator_with_transform() {
    let code = r#"
def doubled_gen(items: list):
    for item in items:
        yield item * 2
"#;
    let result = transpile(code);
    assert!(result.contains("doubled_gen"), "Got: {}", result);
}

// ===== Complex data manipulation =====

#[test]
fn test_s12_matrix_addition() {
    let code = r#"
def matrix_add(a: list, b: list) -> list:
    result = []
    for i in range(len(a)):
        row = []
        for j in range(len(a[i])):
            row.append(a[i][j] + b[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_add"), "Got: {}", result);
}

#[test]
fn test_s12_dot_product() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    result = 0
    for i in range(len(a)):
        result += a[i] * b[i]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn dot_product"), "Got: {}", result);
}

// ===== Complex string patterns =====

#[test]
fn test_s12_string_builder_pattern() {
    let code = r#"
def build_csv(rows: list) -> str:
    lines = []
    for row in rows:
        parts = []
        for val in row:
            parts.append(str(val))
        lines.append(",".join(parts))
    return "\n".join(lines)
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_csv"), "Got: {}", result);
}

#[test]
fn test_s12_string_repeat_pattern() {
    let code = r#"
def indent(text: str, level: int) -> str:
    prefix = "  " * level
    return prefix + text
"#;
    let result = transpile(code);
    assert!(result.contains("fn indent"), "Got: {}", result);
}

// ===== Complex conditional patterns =====

#[test]
fn test_s12_guard_clauses() {
    let code = r#"
def process_data(data: list) -> int:
    if not data:
        return 0
    if len(data) == 1:
        return data[0]
    if len(data) == 2:
        return data[0] + data[1]
    return sum(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_data"), "Got: {}", result);
}

#[test]
fn test_s12_complex_condition_with_method() {
    let code = r#"
def is_valid_email(email: str) -> bool:
    if "@" not in email:
        return False
    if email.startswith("@"):
        return False
    if email.endswith("@"):
        return False
    parts = email.split("@")
    if len(parts) != 2:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_email"), "Got: {}", result);
}

// ===== Complex class with methods =====

#[test]
fn test_s12_class_linked_list() {
    let code = r#"
class LinkedList:
    def __init__(self):
        self.head = None
        self.size = 0

    def push(self, value: int):
        self.size += 1

    def length(self) -> int:
        return self.size

    def is_empty(self) -> bool:
        return self.size == 0
"#;
    let result = transpile(code);
    assert!(result.contains("LinkedList"), "Got: {}", result);
    assert!(result.contains("push"), "Got: {}", result);
    assert!(result.contains("is_empty"), "Got: {}", result);
}

#[test]
fn test_s12_class_matrix() {
    let code = r#"
class Matrix:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data = []

    def get(self, r: int, c: int) -> int:
        return self.data[r * self.cols + c]

    def set(self, r: int, c: int, val: int):
        self.data[r * self.cols + c] = val
"#;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

// ===== Augmented assignment patterns =====

#[test]
fn test_s12_augmented_bitwise_and() {
    let code = r#"
def mask_bits(x: int, mask: int) -> int:
    x &= mask
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn mask_bits"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitwise_or() {
    let code = r#"
def set_bit(x: int, bit: int) -> int:
    x |= bit
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_bit"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_shift_left() {
    let code = r#"
def shift_up(x: int, n: int) -> int:
    x <<= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_up"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_power() {
    let code = r#"
def power_assign(x: int, n: int) -> int:
    x **= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn power_assign"), "Got: {}", result);
}

// ===== Complex patterns combining features =====

#[test]
fn test_s12_memoized_fibonacci() {
    let code = r#"
def fibonacci(n: int) -> int:
    memo = {0: 0, 1: 1}
    for i in range(2, n + 1):
        memo[i] = memo[i - 1] + memo[i - 2]
    return memo[n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Got: {}", result);
}

#[test]
fn test_s12_knapsack_dp() {
    let code = r#"
def knapsack(weights: list, values: list, capacity: int) -> int:
    n = len(weights)
    dp = []
    for i in range(n + 1):
        row = []
        for w in range(capacity + 1):
            row.append(0)
        dp.append(row)
    for i in range(1, n + 1):
        for w in range(1, capacity + 1):
            if weights[i - 1] <= w:
                with_item = values[i - 1] + dp[i - 1][w - weights[i - 1]]
                without_item = dp[i - 1][w]
                if with_item > without_item:
                    dp[i][w] = with_item
                else:
                    dp[i][w] = without_item
            else:
                dp[i][w] = dp[i - 1][w]
    return dp[n][capacity]
"#;
    let result = transpile(code);
    assert!(result.contains("fn knapsack"), "Got: {}", result);
}

#[test]
fn test_s12_longest_common_prefix() {
    let code = r#"
def longest_common_prefix(strs: list) -> str:
    if not strs:
        return ""
    prefix = strs[0]
    for s in strs[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""
    return prefix
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_common_prefix"), "Got: {}", result);
}

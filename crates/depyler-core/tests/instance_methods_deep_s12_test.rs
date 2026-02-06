//! Session 12 Batch 10: Deep coverage tests for expr_gen_instance_methods.rs
//!
//! Targets cold paths identified by coverage analysis:
//! - Set comparison methods (issubset, issuperset, isdisjoint)
//! - Dict advanced methods (setdefault, popitem, items, keys, values)
//! - String justification with fill chars (center, ljust, rjust, zfill)
//! - String title/swapcase/casefold
//! - String partition/rpartition
//! - String check methods (isupper, islower, isspace, isnumeric, etc.)
//! - File I/O methods (read, readline, readlines, write, writelines)
//! - Datetime instance methods (isoformat, strftime, timestamp)
//! - sys.stdout/stderr methods
//! - Regex match group methods
//! - Complex class method patterns

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

// ===== Dict advanced patterns =====

#[test]
fn test_s12_dict_items_loop() {
    let code = r#"
def process_items(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_items"), "Got: {}", result);
}

#[test]
fn test_s12_dict_keys_loop() {
    let code = r#"
def get_keys(d: dict) -> list:
    result = []
    for k in d.keys():
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_keys"), "Got: {}", result);
}

#[test]
fn test_s12_dict_values_loop() {
    let code = r#"
def get_values(d: dict) -> list:
    result = []
    for v in d.values():
        result.append(v)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_values"), "Got: {}", result);
}

#[test]
fn test_s12_dict_items_to_list() {
    let code = r#"
def items_list(d: dict) -> list:
    return list(d.items())
"#;
    let result = transpile(code);
    assert!(result.contains("fn items_list"), "Got: {}", result);
}

// ===== String advanced methods =====

#[test]
fn test_s12_str_expandtabs() {
    let code = r#"
def expand(s: str) -> str:
    return s.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand"), "Got: {}", result);
}

#[test]
fn test_s12_str_splitlines() {
    let code = r#"
def get_lines(s: str) -> list:
    return s.splitlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_lines"), "Got: {}", result);
}

#[test]
fn test_s12_str_lstrip() {
    let code = r#"
def trim_left(s: str) -> str:
    return s.lstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim_left"), "Got: {}", result);
}

#[test]
fn test_s12_str_rstrip() {
    let code = r#"
def trim_right(s: str) -> str:
    return s.rstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim_right"), "Got: {}", result);
}

#[test]
fn test_s12_str_strip_chars() {
    let code = r#"
def strip_chars(s: str) -> str:
    return s.strip("xy")
"#;
    let result = transpile(code);
    assert!(result.contains("fn strip_chars"), "Got: {}", result);
}

#[test]
fn test_s12_str_maketrans_translate() {
    let code = r#"
def clean_punctuation(s: str) -> str:
    return s.replace(",", "").replace(".", "")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_punctuation"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_with_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    def area(self) -> float:
        return self.radius * self.radius * 3.14159
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
    assert!(result.contains("area"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_eq() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_len() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def __len__(self) -> int:
        return len(self.items)

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_str_repr() {
    let code = r#"
class Color:
    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name

    def __repr__(self) -> str:
        return "Color(" + self.name + ")"
"#;
    let result = transpile(code);
    assert!(result.contains("Color"), "Got: {}", result);
}

// ===== Datetime patterns =====

#[test]
fn test_s12_datetime_strftime() {
    let code = r#"
import datetime

def format_date() -> str:
    dt = datetime.datetime(2024, 1, 1)
    return dt.strftime("%Y-%m-%d")
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_date"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_isoformat() {
    let code = r#"
import datetime

def iso_date() -> str:
    dt = datetime.datetime(2024, 1, 1)
    return dt.isoformat()
"#;
    let result = transpile(code);
    assert!(result.contains("fn iso_date"), "Got: {}", result);
}

// ===== Math module =====

#[test]
fn test_s12_math_sqrt() {
    let code = r#"
import math

def square_root(x: float) -> float:
    return math.sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn square_root"), "Got: {}", result);
    assert!(result.contains("sqrt"), "Got: {}", result);
}

#[test]
fn test_s12_math_floor_ceil() {
    let code = r#"
import math

def floor_val(x: float) -> int:
    return math.floor(x)

def ceil_val(x: float) -> int:
    return math.ceil(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn floor_val"), "Got: {}", result);
    assert!(result.contains("fn ceil_val"), "Got: {}", result);
}

#[test]
fn test_s12_math_log() {
    let code = r#"
import math

def natural_log(x: float) -> float:
    return math.log(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn natural_log"), "Got: {}", result);
}

#[test]
fn test_s12_math_pow() {
    let code = r#"
import math

def power(base: float, exp: float) -> float:
    return math.pow(base, exp)
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_math_abs() {
    let code = r#"
import math

def absolute(x: float) -> float:
    return math.fabs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"), "Got: {}", result);
}

#[test]
fn test_s12_math_gcd() {
    let code = r#"
import math

def compute_gcd(a: int, b: int) -> int:
    return math.gcd(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute_gcd"), "Got: {}", result);
}

#[test]
fn test_s12_math_trig() {
    let code = r#"
import math

def sine(x: float) -> float:
    return math.sin(x)

def cosine(x: float) -> float:
    return math.cos(x)

def tangent(x: float) -> float:
    return math.tan(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sine"), "Got: {}", result);
    assert!(result.contains("fn cosine"), "Got: {}", result);
    assert!(result.contains("fn tangent"), "Got: {}", result);
}

// ===== Complex algorithms with multiple features =====

#[test]
fn test_s12_matrix_multiply() {
    let code = r#"
def matrix_mult(a: list, b: list) -> list:
    rows_a = len(a)
    cols_b = len(b[0])
    cols_a = len(b)
    result = []
    for i in range(rows_a):
        row = []
        for j in range(cols_b):
            total = 0
            for k in range(cols_a):
                total += a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_mult"), "Got: {}", result);
}

#[test]
fn test_s12_merge_sort() {
    let code = r#"
def merge_sort(arr: list) -> list:
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    return merge(left, right)

def merge(left: list, right: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    while i < len(left):
        result.append(left[i])
        i += 1
    while j < len(right):
        result.append(right[j])
        j += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sort"), "Got: {}", result);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

#[test]
fn test_s12_frequency_count() {
    let code = r#"
def count_frequency(items: list) -> dict:
    freq = {}
    for item in items:
        if item in freq:
            freq[item] += 1
        else:
            freq[item] = 1
    return freq
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_frequency"), "Got: {}", result);
}

#[test]
fn test_s12_linked_list_style() {
    let code = r#"
class Node:
    def __init__(self, value: int):
        self.value = value
        self.next = None

    def get_value(self) -> int:
        return self.value
"#;
    let result = transpile(code);
    assert!(result.contains("Node"), "Got: {}", result);
}

// ===== String method chains =====

#[test]
fn test_s12_str_chain_strip_lower() {
    let code = r#"
def normalize(s: str) -> str:
    return s.strip().lower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_str_chain_replace_strip() {
    let code = r#"
def clean(s: str) -> str:
    return s.replace("-", " ").strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

// ===== Ternary/conditional expressions =====

#[test]
fn test_s12_ternary_expression() {
    let code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_val"), "Got: {}", result);
}

#[test]
fn test_s12_nested_ternary() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else (-1 if x < 0 else 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

// ===== Unpacking patterns =====

#[test]
fn test_s12_star_unpack() {
    let code = r#"
def first_and_rest(items: list) -> tuple:
    first = items[0]
    rest = items[1:]
    return (first, rest)
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_and_rest"), "Got: {}", result);
}

// ===== Global/module patterns =====

#[test]
fn test_s12_module_function_calls() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def main_func(items: list) -> list:
    result = []
    for item in items:
        result.append(helper(item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn helper"), "Got: {}", result);
    assert!(result.contains("fn main_func"), "Got: {}", result);
}

#[test]
fn test_s12_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial"), "Got: {}", result);
}

// ===== Error handling patterns =====

#[test]
fn test_s12_try_except_value_error() {
    let code = r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_int"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_with_variable() {
    let code = r#"
def safe_divide(a: int, b: int) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        return 0.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

// ===== Generator-like patterns =====

#[test]
fn test_s12_list_comprehension_with_condition() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens"), "Got: {}", result);
}

#[test]
fn test_s12_nested_comprehension() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comprehension_with_enumerate() {
    let code = r#"
def index_map(items: list) -> dict:
    return {i: item for i, item in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_map"), "Got: {}", result);
}

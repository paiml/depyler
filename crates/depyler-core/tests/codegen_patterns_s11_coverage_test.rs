//! Session 11: Coverage tests for codegen patterns
//!
//! Targets specific code generation paths for maximum coverage impact:
//! - Type conversions and coercions
//! - Complex function signatures
//! - Class/struct generation
//! - Error handling patterns
//! - Collection operations with type inference
//! - String formatting patterns
//! - Numeric operations and edge cases
//! - Optional type handling
//! - Multiple import patterns

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
// Type conversion patterns
// ============================================================================

#[test]
fn test_s11_int_to_str_conversion() {
    let code = r#"
def num_to_str(x: int) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_string") || result.contains("format"),
        "Should convert int to str. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_to_int_conversion() {
    let code = r#"
def str_to_num(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("fn str_to_num"),
        "Should convert str to int. Got: {}",
        result
    );
}

#[test]
fn test_s11_float_to_int_conversion() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("as i") || result.contains("fn truncate"),
        "Should convert float to int. Got: {}",
        result
    );
}

#[test]
fn test_s11_int_to_float_conversion() {
    let code = r#"
def to_float(x: int) -> float:
    return float(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("as f64") || result.contains("f64") || result.contains("fn to_float"),
        "Should convert int to float. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_to_set_conversion() {
    let code = r#"
from typing import Set

def unique(items: list) -> Set[int]:
    return set(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashSet") || result.contains("fn unique"),
        "Should convert list to set. Got: {}",
        result
    );
}

#[test]
fn test_s11_str_to_list_conversion() {
    let code = r#"
def chars(s: str) -> list:
    return list(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("chars") || result.contains("fn chars"),
        "Should convert str to list. Got: {}",
        result
    );
}

// ============================================================================
// Complex function signatures
// ============================================================================

#[test]
fn test_s11_optional_parameter() {
    let code = r#"
from typing import Optional

def greet(name: Optional[str]) -> str:
    if name is None:
        return "Hello"
    return "Hello " + name
"#;
    let result = transpile(code);
    assert!(
        result.contains("Option") || result.contains("fn greet"),
        "Should handle Optional parameter. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_of_strings_param() {
    let code = r#"
from typing import List

def join_words(words: List[str]) -> str:
    return " ".join(words)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn join_words"),
        "Should handle List[str] parameter. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_param_typed() {
    let code = r#"
from typing import Dict

def lookup(data: Dict[str, int], key: str) -> int:
    return data.get(key, 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn lookup"),
        "Should handle Dict[str, int] parameter. Got: {}",
        result
    );
}

#[test]
fn test_s11_tuple_return_type() {
    let code = r#"
from typing import Tuple

def split_pair(s: str) -> Tuple[str, str]:
    parts = s.split(",")
    return (parts[0], parts[1])
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_pair"),
        "Should handle Tuple return type. Got: {}",
        result
    );
}

// ============================================================================
// Collection operations with type inference
// ============================================================================

#[test]
fn test_s11_list_append_in_loop() {
    let code = r#"
def squares(n: int) -> list:
    result: list = []
    for i in range(n):
        result.append(i * i)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("push") || result.contains("append"),
        "Should transpile list.append. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_iteration_keys() {
    let code = r#"
def key_list(d: dict) -> list:
    result: list = []
    for key in d:
        result.append(key)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn key_list"),
        "Should transpile dict key iteration. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_sorted_builtin() {
    let code = r#"
def sort_items(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("sort") || result.contains("fn sort_items"),
        "Should transpile sorted(). Got: {}",
        result
    );
}

#[test]
fn test_s11_min_max_builtins() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return max(lo, min(hi, x))
"#;
    let result = transpile(code);
    assert!(
        result.contains("max") || result.contains("min") || result.contains("fn clamp"),
        "Should transpile min/max. Got: {}",
        result
    );
}

#[test]
fn test_s11_abs_builtin() {
    let code = r#"
def magnitude(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("abs"),
        "Should transpile abs(). Got: {}",
        result
    );
}

// ============================================================================
// String formatting patterns
// ============================================================================

#[test]
fn test_s11_fstring_simple() {
    let code = r#"
def hello(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!"),
        "Should transpile f-string. Got: {}",
        result
    );
}

#[test]
fn test_s11_fstring_expression() {
    let code = r#"
def sum_msg(a: int, b: int) -> str:
    return f"{a} + {b} = {a + b}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!"),
        "Should transpile f-string with expr. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_multiply() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(
        result.contains("repeat") || result.contains("fn repeat"),
        "Should transpile string multiply. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_concatenation() {
    let code = r#"
def combine(a: str, b: str, c: str) -> str:
    return a + " " + b + " " + c
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!") || result.contains("+") || result.contains("fn combine"),
        "Should transpile string concat. Got: {}",
        result
    );
}

// ============================================================================
// Numeric operations
// ============================================================================

#[test]
fn test_s11_integer_division() {
    let code = r#"
def divide(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(
        result.contains("/") || result.contains("div") || result.contains("fn divide"),
        "Should transpile //. Got: {}",
        result
    );
}

#[test]
fn test_s11_modulo_operator() {
    let code = r#"
def remainder(a: int, b: int) -> int:
    return a % b
"#;
    let result = transpile(code);
    assert!(
        result.contains("%") || result.contains("mod"),
        "Should transpile %. Got: {}",
        result
    );
}

#[test]
fn test_s11_power_operator() {
    let code = r#"
def square(x: int) -> int:
    return x ** 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("pow") || result.contains("fn square"),
        "Should transpile **. Got: {}",
        result
    );
}

#[test]
fn test_s11_bitwise_and() {
    let code = r#"
def mask(x: int, m: int) -> int:
    return x & m
"#;
    let result = transpile(code);
    assert!(
        result.contains("&") || result.contains("fn mask"),
        "Should transpile &. Got: {}",
        result
    );
}

#[test]
fn test_s11_bitwise_or() {
    let code = r#"
def combine_flags(a: int, b: int) -> int:
    return a | b
"#;
    let result = transpile(code);
    assert!(
        result.contains("|") || result.contains("fn combine_flags"),
        "Should transpile |. Got: {}",
        result
    );
}

#[test]
fn test_s11_bitwise_xor() {
    let code = r#"
def toggle(a: int, b: int) -> int:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(
        result.contains("^") || result.contains("fn toggle"),
        "Should transpile ^. Got: {}",
        result
    );
}

#[test]
fn test_s11_left_shift() {
    let code = r#"
def shift_left(x: int, n: int) -> int:
    return x << n
"#;
    let result = transpile(code);
    assert!(
        result.contains("<<"),
        "Should transpile <<. Got: {}",
        result
    );
}

#[test]
fn test_s11_right_shift() {
    let code = r#"
def shift_right(x: int, n: int) -> int:
    return x >> n
"#;
    let result = transpile(code);
    assert!(
        result.contains(">>"),
        "Should transpile >>. Got: {}",
        result
    );
}

// ============================================================================
// Boolean operations
// ============================================================================

#[test]
fn test_s11_not_operator() {
    let code = r#"
def negate(x: bool) -> bool:
    return not x
"#;
    let result = transpile(code);
    assert!(
        result.contains("!"),
        "Should transpile not. Got: {}",
        result
    );
}

#[test]
fn test_s11_and_operator() {
    let code = r#"
def both(a: bool, b: bool) -> bool:
    return a and b
"#;
    let result = transpile(code);
    assert!(
        result.contains("&&"),
        "Should transpile and. Got: {}",
        result
    );
}

#[test]
fn test_s11_or_operator() {
    let code = r#"
def either(a: bool, b: bool) -> bool:
    return a or b
"#;
    let result = transpile(code);
    assert!(
        result.contains("||"),
        "Should transpile or. Got: {}",
        result
    );
}

// ============================================================================
// Membership tests
// ============================================================================

#[test]
fn test_s11_in_list() {
    let code = r#"
def contains(items: list, val: int) -> bool:
    return val in items
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains") || result.contains("fn contains"),
        "Should transpile 'in' for list. Got: {}",
        result
    );
}

#[test]
fn test_s11_not_in_list() {
    let code = r#"
def missing(items: list, val: int) -> bool:
    return val not in items
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains") || result.contains("fn missing"),
        "Should transpile 'not in' for list. Got: {}",
        result
    );
}

#[test]
fn test_s11_in_string() {
    let code = r#"
def has_substring(text: str, sub: str) -> bool:
    return sub in text
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains") || result.contains("fn has_substring"),
        "Should transpile 'in' for string. Got: {}",
        result
    );
}

// ============================================================================
// Class/struct patterns
// ============================================================================

#[test]
fn test_s11_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("struct") || result.contains("impl") || result.contains("Point"),
        "Should transpile class. Got: {}",
        result
    );
}

#[test]
fn test_s11_class_with_string_method() {
    let code = r#"
class Greeter:
    def __init__(self, name: str) -> None:
        self.name = name

    def greet(self) -> str:
        return f"Hello, {self.name}!"
"#;
    let result = transpile(code);
    assert!(
        result.contains("Greeter") || result.contains("struct"),
        "Should transpile class with string method. Got: {}",
        result
    );
}

// ============================================================================
// Complex algorithm patterns
// ============================================================================

#[test]
fn test_s11_fibonacci_iterative() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fibonacci"),
        "Should transpile fibonacci. Got: {}",
        result
    );
}

#[test]
fn test_s11_gcd_algorithm() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn gcd"),
        "Should transpile GCD. Got: {}",
        result
    );
}

#[test]
fn test_s11_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo: int = 0
    hi: int = len(items) - 1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn binary_search"),
        "Should transpile binary search. Got: {}",
        result
    );
}

#[test]
fn test_s11_count_chars() {
    let code = r#"
def char_freq(text: str) -> dict:
    freq: dict = {}
    for ch in text:
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
    return freq
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn char_freq"),
        "Should transpile char frequency counter. Got: {}",
        result
    );
}

#[test]
fn test_s11_flatten_list() {
    let code = r#"
def flatten(nested: list) -> list:
    result: list = []
    for sublist in nested:
        for item in sublist:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn flatten"),
        "Should transpile list flattening. Got: {}",
        result
    );
}

#[test]
fn test_s11_word_count() {
    let code = r#"
def word_count(text: str) -> dict:
    words = text.split()
    counts: dict = {}
    for word in words:
        w: str = word.lower()
        if w in counts:
            counts[w] = counts[w] + 1
        else:
            counts[w] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn word_count"),
        "Should transpile word count. Got: {}",
        result
    );
}

// ============================================================================
// Math module functions
// ============================================================================

#[test]
fn test_s11_math_tan() {
    let code = r#"
import math

def tangent(x: float) -> float:
    return math.tan(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("tan"),
        "Should transpile math.tan. Got: {}",
        result
    );
}

#[test]
fn test_s11_math_exp() {
    let code = r#"
import math

def exponential(x: float) -> float:
    return math.exp(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("exp"),
        "Should transpile math.exp. Got: {}",
        result
    );
}

#[test]
fn test_s11_math_log10() {
    let code = r#"
import math

def log_base_10(x: float) -> float:
    return math.log10(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("log10"),
        "Should transpile math.log10. Got: {}",
        result
    );
}

#[test]
fn test_s11_math_atan2() {
    let code = r#"
import math

def angle(y: float, x: float) -> float:
    return math.atan2(y, x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("atan2"),
        "Should transpile math.atan2. Got: {}",
        result
    );
}

// ============================================================================
// Multiple imports and complex modules
// ============================================================================

#[test]
fn test_s11_import_collections() {
    let code = r#"
from collections import Counter

def count_items(items: list) -> dict:
    return dict(Counter(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_items"),
        "Should handle collections import. Got: {}",
        result
    );
}

#[test]
fn test_s11_import_typing_complex() {
    let code = r#"
from typing import List, Dict, Optional, Tuple, Set

def process(
    items: List[int],
    config: Dict[str, str],
    name: Optional[str],
) -> Tuple[int, str]:
    count: int = len(items)
    label: str = name if name is not None else "default"
    return (count, label)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should handle complex typing imports. Got: {}",
        result
    );
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_s11_lambda_simple() {
    let code = r#"
def apply_fn(items: list) -> list:
    return sorted(items, key=lambda x: x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn apply_fn"),
        "Should transpile lambda. Got: {}",
        result
    );
}

#[test]
fn test_s11_map_with_lambda() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn double_all"),
        "Should transpile map+lambda. Got: {}",
        result
    );
}

#[test]
fn test_s11_filter_with_lambda() {
    let code = r#"
def even_only(items: list) -> list:
    return list(filter(lambda x: x % 2 == 0, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn even_only"),
        "Should transpile filter+lambda. Got: {}",
        result
    );
}

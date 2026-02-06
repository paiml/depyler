//! Session 12: Targeted tests for direct_rules_convert.rs uncovered paths
//!
//! Targets:
//! - Truthiness coercion (apply_truthiness_coercion)
//! - Floor division edge cases
//! - Power operator edge cases
//! - Mixed float/int comparisons
//! - min/max with multiple args
//! - Unary operators with truthiness
//! - String formatting paths
//! - Collection type constructors

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

// ===== Truthiness coercion =====

#[test]
fn test_s12_truthiness_string_if() {
    let code = r#"
def check_name(name: str) -> str:
    if name:
        return "has name"
    return "no name"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_name"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_list_if() {
    let code = r#"
def check_items(items: list) -> bool:
    if items:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_items"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_int_if() {
    let code = r#"
def check_count(n: int) -> str:
    if n:
        return "nonzero"
    return "zero"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_count"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_float_if() {
    let code = r#"
def check_value(x: float) -> bool:
    if x:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_value"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_dict_if() {
    let code = r#"
def check_dict(d: dict) -> bool:
    if d:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_dict"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_optional_if() {
    let code = r#"
def check_optional(x = None) -> bool:
    if x:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_optional"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_negated_string() {
    let code = r#"
def is_empty(s: str) -> bool:
    if not s:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_empty"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_negated_list() {
    let code = r#"
def is_empty_list(items: list) -> bool:
    if not items:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_empty_list"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_negated_int() {
    let code = r#"
def is_zero(n: int) -> bool:
    if not n:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_zero"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_while_string() {
    let code = r#"
def consume(s: str) -> int:
    count = 0
    while s:
        s = s[1:]
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn consume"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_while_list() {
    let code = r#"
def drain(items: list) -> int:
    count = 0
    while items:
        items.pop()
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn drain"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_boolean_and() {
    let code = r#"
def both_true(a: str, b: list) -> bool:
    return a and b
"#;
    let result = transpile(code);
    assert!(result.contains("fn both_true"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_boolean_or() {
    let code = r#"
def either_true(a: str, b: str) -> str:
    return a or b
"#;
    let result = transpile(code);
    assert!(result.contains("fn either_true"), "Got: {}", result);
}

// ===== Floor division edge cases =====

#[test]
fn test_s12_floor_div_negative_result() {
    let code = r#"
def neg_div(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn neg_div"), "Got: {}", result);
}

#[test]
fn test_s12_floor_div_float_operands() {
    let code = r#"
def float_div(a: float, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn float_div"), "Got: {}", result);
}

#[test]
fn test_s12_floor_div_mixed_types() {
    let code = r#"
def mixed_div(a: int, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn mixed_div"), "Got: {}", result);
}

#[test]
fn test_s12_floor_div_in_expression() {
    let code = r#"
def midpoint(lo: int, hi: int) -> int:
    return (lo + hi) // 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn midpoint"), "Got: {}", result);
}

#[test]
fn test_s12_floor_div_augmented() {
    let code = r#"
def halve(x: int) -> int:
    x //= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve"), "Got: {}", result);
}

// ===== Power operator edge cases =====

#[test]
fn test_s12_power_negative_exponent() {
    let code = r#"
def inverse(x: float) -> float:
    return x ** -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn inverse"), "Got: {}", result);
}

#[test]
fn test_s12_power_float_exponent() {
    let code = r#"
def sqrt(x: float) -> float:
    return x ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn sqrt"), "Got: {}", result);
}

#[test]
fn test_s12_power_variable_exponent() {
    let code = r#"
def power(base: float, exp: float) -> float:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_power_integer_literal() {
    let code = r#"
def square(x: int) -> int:
    return x ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn square"), "Got: {}", result);
}

#[test]
fn test_s12_power_cube() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube"), "Got: {}", result);
}

// ===== Mixed float/int comparisons =====

#[test]
fn test_s12_compare_float_int() {
    let code = r#"
def is_above(x: float, threshold: int) -> bool:
    return x > threshold
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_above"), "Got: {}", result);
}

#[test]
fn test_s12_compare_int_float() {
    let code = r#"
def is_below(n: int, limit: float) -> bool:
    return n < limit
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_below"), "Got: {}", result);
}

#[test]
fn test_s12_compare_float_int_eq() {
    let code = r#"
def check_eq(x: float, n: int) -> bool:
    return x == n
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_eq"), "Got: {}", result);
}

// ===== min/max with multiple args =====

#[test]
fn test_s12_min_three_args() {
    let code = r#"
def smallest(a: int, b: int, c: int) -> int:
    return min(a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn smallest"), "Got: {}", result);
}

#[test]
fn test_s12_max_three_args() {
    let code = r#"
def largest(a: int, b: int, c: int) -> int:
    return max(a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn largest"), "Got: {}", result);
}

#[test]
fn test_s12_min_two_args() {
    let code = r#"
def smaller(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn smaller"), "Got: {}", result);
}

#[test]
fn test_s12_max_two_args() {
    let code = r#"
def bigger(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn bigger"), "Got: {}", result);
}

#[test]
fn test_s12_min_list_arg() {
    let code = r#"
def min_of_list(items: list) -> int:
    return min(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_of_list"), "Got: {}", result);
}

#[test]
fn test_s12_max_list_arg() {
    let code = r#"
def max_of_list(items: list) -> int:
    return max(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_of_list"), "Got: {}", result);
}

// ===== String formatting paths =====

#[test]
fn test_s12_string_concat_format() {
    let code = r#"
def greet(name: str, age: int) -> str:
    return name + " is " + str(age) + " years old"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_print_multiple_args() {
    let code = r#"
def show(a: int, b: str, c: float):
    print(a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show"), "Got: {}", result);
}

#[test]
fn test_s12_print_single_arg() {
    let code = r#"
def say(msg: str):
    print(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("fn say"), "Got: {}", result);
}

// ===== Collection type constructors =====

#[test]
fn test_s12_set_from_list() {
    let code = r#"
def unique(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique"), "Got: {}", result);
}

#[test]
fn test_s12_list_from_range() {
    let code = r#"
def make_range(n: int) -> list:
    return list(range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_range"), "Got: {}", result);
}

#[test]
fn test_s12_tuple_from_list() {
    let code = r#"
def to_tuple(items: list) -> tuple:
    return tuple(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_tuple"), "Got: {}", result);
}

#[test]
fn test_s12_dict_from_pairs() {
    let code = r#"
def make_dict(pairs: list) -> dict:
    return dict(pairs)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"), "Got: {}", result);
}

#[test]
fn test_s12_sorted_list() {
    let code = r#"
def sort_items(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_items"), "Got: {}", result);
}

#[test]
fn test_s12_reversed_list() {
    let code = r#"
def reverse_items(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_items"), "Got: {}", result);
}

// ===== Chained comparisons =====

#[test]
fn test_s12_chained_comparison_three() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 <= x <= 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_chained_comparison_lt() {
    let code = r#"
def ascending(a: int, b: int, c: int) -> bool:
    return a < b < c
"#;
    let result = transpile(code);
    assert!(result.contains("fn ascending"), "Got: {}", result);
}

// ===== Unary operators =====

#[test]
fn test_s12_unary_not_comparison() {
    let code = r#"
def is_not_equal(a: int, b: int) -> bool:
    return not (a == b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_not_equal"), "Got: {}", result);
}

#[test]
fn test_s12_unary_not_bool_method() {
    let code = r#"
def not_starts(s: str) -> bool:
    return not s.startswith("x")
"#;
    let result = transpile(code);
    assert!(result.contains("fn not_starts"), "Got: {}", result);
}

#[test]
fn test_s12_unary_negate() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"), "Got: {}", result);
}

#[test]
fn test_s12_unary_bitwise_not() {
    let code = r#"
def complement(x: int) -> int:
    return ~x
"#;
    let result = transpile(code);
    assert!(result.contains("fn complement"), "Got: {}", result);
}

// ===== Boolean method detection for truthiness =====

#[test]
fn test_s12_truthiness_startswith() {
    let code = r#"
def check(s: str) -> str:
    if s.startswith("a"):
        return "starts with a"
    return "no"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_endswith() {
    let code = r#"
def check_end(s: str) -> bool:
    return s.endswith(".py")
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_end"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_isdigit() {
    let code = r#"
def is_num(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_num"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_isalpha() {
    let code = r#"
def is_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alpha"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_contains() {
    let code = r#"
def has_word(text: str, word: str) -> bool:
    return word in text
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_word"), "Got: {}", result);
}

// ===== os.path functions =====

#[test]
fn test_s12_os_path_splitext() {
    let code = r#"
import os

def get_extension(path: str) -> str:
    name, ext = os.path.splitext(path)
    return ext
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_extension"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_expanduser() {
    let code = r#"
import os

def home_path(p: str) -> str:
    return os.path.expanduser(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn home_path"), "Got: {}", result);
}

// ===== Colorsys module =====

#[test]
fn test_s12_colorsys_rgb_to_hsv() {
    let code = r#"
import colorsys

def to_hsv(r: float, g: float, b: float) -> tuple:
    return colorsys.rgb_to_hsv(r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_hsv"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_hsv_to_rgb() {
    let code = r#"
import colorsys

def to_rgb(h: float, s: float, v: float) -> tuple:
    return colorsys.hsv_to_rgb(h, s, v)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_rgb"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_rgb_to_hls() {
    let code = r#"
import colorsys

def to_hls(r: float, g: float, b: float) -> tuple:
    return colorsys.rgb_to_hls(r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_hls"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_hls_to_rgb() {
    let code = r#"
import colorsys

def from_hls(h: float, l: float, s: float) -> tuple:
    return colorsys.hls_to_rgb(h, l, s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn from_hls"), "Got: {}", result);
}

// ===== time module =====

#[test]
fn test_s12_time_sleep() {
    let code = r#"
import time

def wait(seconds: float):
    time.sleep(seconds)
"#;
    let result = transpile(code);
    assert!(result.contains("fn wait"), "Got: {}", result);
}

#[test]
fn test_s12_time_time() {
    let code = r#"
import time

def now() -> float:
    return time.time()
"#;
    let result = transpile(code);
    assert!(result.contains("fn now"), "Got: {}", result);
}

// ===== random module =====

#[test]
fn test_s12_random_random() {
    let code = r#"
import random

def rand_float() -> float:
    return random.random()
"#;
    let result = transpile(code);
    assert!(result.contains("fn rand_float"), "Got: {}", result);
}

// ===== math module edge cases =====

#[test]
fn test_s12_math_log_base() {
    let code = r#"
import math

def log_base(x: float, base: float) -> float:
    return math.log(x, base)
"#;
    let result = transpile(code);
    assert!(result.contains("fn log_base"), "Got: {}", result);
}

#[test]
fn test_s12_math_log_natural() {
    let code = r#"
import math

def ln(x: float) -> float:
    return math.log(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ln"), "Got: {}", result);
}

// ===== dict.items/keys/values in for loop =====

#[test]
fn test_s12_for_dict_items_with_type() {
    let code = r#"
def print_items(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(str(k) + "=" + str(v))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_items"), "Got: {}", result);
}

#[test]
fn test_s12_for_dict_keys() {
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
fn test_s12_for_dict_values() {
    let code = r#"
def sum_values(d: dict) -> int:
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_values"), "Got: {}", result);
}

// ===== len() minus operations =====

#[test]
fn test_s12_len_minus_one() {
    let code = r#"
def last_index(items: list) -> int:
    return len(items) - 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_index"), "Got: {}", result);
}

#[test]
fn test_s12_len_minus_variable() {
    let code = r#"
def remaining(items: list, used: int) -> int:
    return len(items) - used
"#;
    let result = transpile(code);
    assert!(result.contains("fn remaining"), "Got: {}", result);
}

// ===== Type conversion edge cases =====

#[test]
fn test_s12_int_from_string() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"), "Got: {}", result);
}

#[test]
fn test_s12_float_from_string() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_float"), "Got: {}", result);
}

#[test]
fn test_s12_str_from_int() {
    let code = r#"
def int_to_str(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_to_str"), "Got: {}", result);
}

#[test]
fn test_s12_bool_from_int() {
    let code = r#"
def int_to_bool(n: int) -> bool:
    return bool(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_to_bool"), "Got: {}", result);
}

#[test]
fn test_s12_bool_from_string() {
    let code = r#"
def str_to_bool(s: str) -> bool:
    return bool(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn str_to_bool"), "Got: {}", result);
}

// ===== abs/round edge cases =====

#[test]
fn test_s12_abs_float() {
    let code = r#"
def abs_val(x: float) -> float:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"), "Got: {}", result);
}

#[test]
fn test_s12_round_float() {
    let code = r#"
def round_val(x: float) -> int:
    return round(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_val"), "Got: {}", result);
}

#[test]
fn test_s12_round_with_precision() {
    let code = r#"
def round_to(x: float, n: int) -> float:
    return round(x, n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_to"), "Got: {}", result);
}

// ===== range() variants =====

#[test]
fn test_s12_range_one_arg() {
    let code = r#"
def count(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn count"), "Got: {}", result);
}

#[test]
fn test_s12_range_two_args() {
    let code = r#"
def count_from(start: int, end: int) -> list:
    result = []
    for i in range(start, end):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_from"), "Got: {}", result);
}

// ===== Complex patterns =====

#[test]
fn test_s12_nested_ternary() {
    let code = r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else "negative" if x < 0 else "zero"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s12_list_comp_with_condition() {
    let code = r#"
def evens(n: int) -> list:
    return [i for i in range(n) if i % 2 == 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comp_with_condition() {
    let code = r#"
def even_squares(n: int) -> dict:
    return {i: i*i for i in range(n) if i % 2 == 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_squares"), "Got: {}", result);
}

#[test]
fn test_s12_generator_in_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(i * i for i in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"), "Got: {}", result);
}

#[test]
fn test_s12_generator_in_any() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_positive"), "Got: {}", result);
}

#[test]
fn test_s12_generator_in_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

// ===== isinstance patterns =====

#[test]
fn test_s12_isinstance_int() {
    let code = r#"
def is_int(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_int"), "Got: {}", result);
}

#[test]
fn test_s12_isinstance_str() {
    let code = r#"
def is_str(x) -> bool:
    return isinstance(x, str)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_str"), "Got: {}", result);
}

// ===== Augmented assignment edge cases =====

#[test]
fn test_s12_augmented_bitand() {
    let code = r#"
def mask(x: int, m: int) -> int:
    x &= m
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn mask"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitor() {
    let code = r#"
def add_flag(x: int, flag: int) -> int:
    x |= flag
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_flag"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitxor() {
    let code = r#"
def toggle(x: int, mask: int) -> int:
    x ^= mask
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn toggle"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_lshift() {
    let code = r#"
def shift_left(x: int, n: int) -> int:
    x <<= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_left"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_rshift() {
    let code = r#"
def shift_right(x: int, n: int) -> int:
    x >>= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_right"), "Got: {}", result);
}

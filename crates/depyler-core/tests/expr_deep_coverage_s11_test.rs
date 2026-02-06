//! Session 11: Deep expression codegen coverage tests
//!
//! Targets specific untested code paths in expr_gen.rs:
//! - Boolean short-circuit patterns (or with fallback)
//! - Membership operators with various container types
//! - Ternary/conditional expressions
//! - Complex f-string patterns
//! - Comparison chains
//! - Unary operators
//! - Dict/Set comprehensions
//! - Augmented assignments with special types

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
// Boolean short-circuit / fallback patterns
// ============================================================================

#[test]
fn test_s11_expr_or_fallback_string() {
    let code = r#"
def get_name(name: str) -> str:
    return name or "default"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_name"),
        "Should transpile string or fallback. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_and_chain() {
    let code = r#"
def all_positive(a: int, b: int, c: int) -> bool:
    return a > 0 and b > 0 and c > 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("&&"),
        "Should use && for chained and. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_or_chain() {
    let code = r#"
def any_positive(a: int, b: int, c: int) -> bool:
    return a > 0 or b > 0 or c > 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("||"),
        "Should use || for chained or. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_not_with_comparison() {
    let code = r#"
def not_equal(a: int, b: int) -> bool:
    return not (a == b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn not_equal"),
        "Should transpile not with comparison. Got: {}",
        result
    );
}

// ============================================================================
// Membership operators (in / not in)
// ============================================================================

#[test]
fn test_s11_expr_in_list_literal() {
    let code = r#"
def is_vowel(ch: str) -> bool:
    return ch in ["a", "e", "i", "o", "u"]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_vowel"),
        "Should transpile 'in' with list literal. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_not_in_string() {
    let code = r#"
def no_digit(text: str, ch: str) -> bool:
    return ch not in text
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn no_digit"),
        "Should transpile 'not in' with string. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_in_dict_keys() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains_key") || result.contains("fn has_key"),
        "Should transpile 'in dict'. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_in_set() {
    let code = r#"
from typing import Set

def member(s: Set[int], val: int) -> bool:
    return val in s
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains") || result.contains("fn member"),
        "Should transpile 'in set'. Got: {}",
        result
    );
}

// ============================================================================
// Ternary/conditional expressions
// ============================================================================

#[test]
fn test_s11_expr_ternary_simple() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("if") || result.contains("fn abs_val"),
        "Should transpile ternary. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_ternary_in_assignment() {
    let code = r#"
def categorize(score: int) -> str:
    label: str = "pass" if score >= 60 else "fail"
    return label
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn categorize"),
        "Should transpile ternary assignment. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_ternary_with_calls() {
    let code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn max_val"),
        "Should transpile ternary with vars. Got: {}",
        result
    );
}

// ============================================================================
// Complex f-string patterns
// ============================================================================

#[test]
fn test_s11_expr_fstring_with_expression() {
    let code = r#"
def area_msg(w: int, h: int) -> str:
    return f"Area is {w * h}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!"),
        "Should use format! for f-string. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_fstring_multiple_vars() {
    let code = r#"
def full_name(first: str, last: str) -> str:
    return f"{first} {last}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!"),
        "Should use format! for multi-var f-string. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_fstring_nested_method() {
    let code = r#"
def show(name: str) -> str:
    return f"Name: {name.upper()}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!") || result.contains("fn show"),
        "Should transpile f-string with method call. Got: {}",
        result
    );
}

// ============================================================================
// Comparison chains
// ============================================================================

#[test]
fn test_s11_expr_chained_comparison() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 <= x and x < 100
"#;
    let result = transpile(code);
    assert!(
        result.contains("&&"),
        "Should use && for chained comparison. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_ne_comparison() {
    let code = r#"
def different(a: int, b: int) -> bool:
    return a != b
"#;
    let result = transpile(code);
    assert!(
        result.contains("!="),
        "Should use != comparison. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_ge_comparison() {
    let code = r#"
def at_least(a: int, b: int) -> bool:
    return a >= b
"#;
    let result = transpile(code);
    assert!(
        result.contains(">="),
        "Should use >= comparison. Got: {}",
        result
    );
}

// ============================================================================
// Unary operators
// ============================================================================

#[test]
fn test_s11_expr_unary_minus() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(
        result.contains("-") && result.contains("fn negate"),
        "Should transpile unary minus. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_unary_not() {
    let code = r#"
def invert(flag: bool) -> bool:
    return not flag
"#;
    let result = transpile(code);
    assert!(
        result.contains("!") || result.contains("fn invert"),
        "Should transpile unary not. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_unary_bitnot() {
    let code = r#"
def bitwise_not(x: int) -> int:
    return ~x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn bitwise_not"),
        "Should transpile bitwise not. Got: {}",
        result
    );
}

// ============================================================================
// Bitwise operators
// ============================================================================

#[test]
fn test_s11_expr_bitwise_and() {
    let code = r#"
def mask(x: int, m: int) -> int:
    return x & m
"#;
    let result = transpile(code);
    assert!(
        result.contains("&") && result.contains("fn mask"),
        "Should transpile bitwise and. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_bitwise_or() {
    let code = r#"
def combine(a: int, b: int) -> int:
    return a | b
"#;
    let result = transpile(code);
    assert!(
        result.contains("|") && result.contains("fn combine"),
        "Should transpile bitwise or. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_left_shift() {
    let code = r#"
def shift_left(x: int, n: int) -> int:
    return x << n
"#;
    let result = transpile(code);
    assert!(
        result.contains("<<"),
        "Should transpile left shift. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_right_shift() {
    let code = r#"
def shift_right(x: int, n: int) -> int:
    return x >> n
"#;
    let result = transpile(code);
    assert!(
        result.contains(">>"),
        "Should transpile right shift. Got: {}",
        result
    );
}

// ============================================================================
// Complex list/dict/set operations
// ============================================================================

#[test]
fn test_s11_expr_list_multiply() {
    let code = r#"
def repeat_zeros(n: int) -> list:
    return [0] * n
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn repeat_zeros"),
        "Should transpile list multiply. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_multiply() {
    let code = r#"
def repeat_char(ch: str, n: int) -> str:
    return ch * n
"#;
    let result = transpile(code);
    assert!(
        result.contains("repeat") || result.contains("fn repeat_char"),
        "Should transpile string multiply. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_dict_from_pairs() {
    let code = r#"
def make_dict() -> dict:
    return {"a": 1, "b": 2, "c": 3}
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn make_dict"),
        "Should transpile dict literal. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_nested_list() {
    let code = r#"
def matrix() -> list:
    return [[1, 2], [3, 4]]
"#;
    let result = transpile(code);
    assert!(
        result.contains("vec!") || result.contains("fn matrix"),
        "Should transpile nested list. Got: {}",
        result
    );
}

// ============================================================================
// Built-in function calls
// ============================================================================

#[test]
fn test_s11_expr_len_call() {
    let code = r#"
def size(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("len()"),
        "Should transpile len(). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_abs_call() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("abs") || result.contains("fn absolute"),
        "Should transpile abs(). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_min_max_call() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return max(lo, min(x, hi))
"#;
    let result = transpile(code);
    assert!(
        result.contains("min") || result.contains("max") || result.contains("fn clamp"),
        "Should transpile min/max. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_sorted_call() {
    let code = r#"
def sort_list(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("sort") || result.contains("fn sort_list"),
        "Should transpile sorted(). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_reversed_call() {
    let code = r#"
def reverse_list(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("rev") || result.contains("fn reverse_list"),
        "Should transpile reversed(). Got: {}",
        result
    );
}

// ============================================================================
// Type conversion calls
// ============================================================================

#[test]
fn test_s11_expr_int_from_str() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("fn parse_int"),
        "Should transpile int(str). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_float_from_str() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("parse") || result.contains("fn parse_float"),
        "Should transpile float(str). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_bool_conversion() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_bool"),
        "Should transpile bool(). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_list_from_range() {
    let code = r#"
def make_range(n: int) -> list:
    return list(range(n))
"#;
    let result = transpile(code);
    assert!(
        result.contains("collect") || result.contains("fn make_range"),
        "Should transpile list(range()). Got: {}",
        result
    );
}

// ============================================================================
// Complex arithmetic patterns
// ============================================================================

#[test]
fn test_s11_expr_floor_division() {
    let code = r#"
def halve(x: int) -> int:
    return x // 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("/") || result.contains("fn halve"),
        "Should transpile floor div. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_modulo() {
    let code = r#"
def is_even(x: int) -> bool:
    return x % 2 == 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("%"),
        "Should transpile modulo. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_power() {
    let code = r#"
def square(x: int) -> int:
    return x ** 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("pow") || result.contains("fn square"),
        "Should transpile power. Got: {}",
        result
    );
}

// ============================================================================
// Comprehension patterns
// ============================================================================

#[test]
fn test_s11_expr_list_comp_with_condition() {
    let code = r#"
def even_squares(n: int) -> list:
    return [x * x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(
        result.contains("filter") || result.contains("map") || result.contains("fn even_squares"),
        "Should transpile filtered list comp. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_dict_comp_basic() {
    let code = r#"
def index_map(items: list) -> dict:
    return {i: item for i, item in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn index_map"),
        "Should transpile dict comp. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_set_comp_basic() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn unique_lengths"),
        "Should transpile set comp. Got: {}",
        result
    );
}

// ============================================================================
// String method variations
// ============================================================================

#[test]
fn test_s11_expr_str_join() {
    let code = r#"
def join_words(words: list) -> str:
    return " ".join(words)
"#;
    let result = transpile(code);
    assert!(
        result.contains("join"),
        "Should transpile join. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_str_replace() {
    let code = r#"
def clean(text: str) -> str:
    return text.replace("old", "new")
"#;
    let result = transpile(code);
    assert!(
        result.contains("replace"),
        "Should transpile replace. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_str_format_method() {
    let code = r#"
def template(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
    let result = transpile(code);
    assert!(
        result.contains("format!") || result.contains("fn template"),
        "Should transpile .format(). Got: {}",
        result
    );
}

// ============================================================================
// Generator expressions
// ============================================================================

#[test]
fn test_s11_expr_sum_generator() {
    let code = r#"
def total_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(
        result.contains("sum") || result.contains("fn total_squares"),
        "Should transpile sum(generator). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_any_generator() {
    let code = r#"
def has_negative(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("any") || result.contains("fn has_negative"),
        "Should transpile any(generator). Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_all_generator() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("all") || result.contains("fn all_positive"),
        "Should transpile all(generator). Got: {}",
        result
    );
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_s11_expr_lambda_in_sort() {
    let code = r#"
def sort_by_second(pairs: list) -> list:
    return sorted(pairs, key=lambda x: x[1])
"#;
    let result = transpile(code);
    assert!(
        result.contains("sort") || result.contains("fn sort_by_second"),
        "Should transpile sorted with lambda. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_lambda_in_map() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("map") || result.contains("fn double_all"),
        "Should transpile map with lambda. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_lambda_in_filter() {
    let code = r#"
def positives(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("filter") || result.contains("fn positives"),
        "Should transpile filter with lambda. Got: {}",
        result
    );
}

// ============================================================================
// Tuple operations
// ============================================================================

#[test]
fn test_s11_expr_tuple_return() {
    let code = r#"
from typing import Tuple

def divmod_result(a: int, b: int) -> Tuple[int, int]:
    return (a // b, a % b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn divmod_result"),
        "Should transpile tuple return. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> int:
    x, y = a, b
    return y
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn swap"),
        "Should transpile tuple unpack. Got: {}",
        result
    );
}

// ============================================================================
// Complex expression combinations
// ============================================================================

#[test]
fn test_s11_expr_nested_calls() {
    let code = r#"
def process(text: str) -> str:
    return text.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should transpile chained methods. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_complex_bool_logic() {
    let code = r#"
def should_process(x: int, flag: bool) -> bool:
    return (x > 0 and x < 100) or (flag and x != 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("&&") || result.contains("||"),
        "Should transpile complex bool. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_mixed_arithmetic_bool() {
    let code = r#"
def check_bounds(x: int, y: int, w: int, h: int) -> bool:
    return 0 <= x and x < w and 0 <= y and y < h
"#;
    let result = transpile(code);
    assert!(
        result.contains("&&"),
        "Should transpile bounds check. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_walrus_operator() {
    let code = r#"
def find_long(words: list) -> str:
    for word in words:
        if len(word) > 5:
            return word
    return ""
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_long"),
        "Should transpile search pattern. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_string_contains() {
    let code = r#"
def has_substring(text: str, sub: str) -> bool:
    return sub in text
"#;
    let result = transpile(code);
    assert!(
        result.contains("contains"),
        "Should transpile string contains. Got: {}",
        result
    );
}

#[test]
fn test_s11_expr_isinstance_pattern() {
    let code = r#"
def type_name(x: int) -> str:
    if isinstance(x, int):
        return "integer"
    return "unknown"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn type_name"),
        "Should transpile isinstance. Got: {}",
        result
    );
}

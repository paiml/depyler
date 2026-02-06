//! Session 12: Deep coverage tests for direct_rules_convert.rs
//!
//! Targets:
//! - String methods via catch-all (capitalize, title, swapcase, etc.)
//! - Float floor division
//! - In operator with different container types
//! - Complex binary operations
//! - Nested comprehensions
//! - Chained comparisons
//! - Augmented assignment variations
//! - Bitwise operations
//! - Complex slice patterns
//! - Error handling patterns

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

// ===== String method catch-all paths =====

#[test]
fn test_s12_str_capitalize() {
    let code = r#"
def capitalize_it(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn capitalize_it"), "Got: {}", result);
}

#[test]
fn test_s12_str_title() {
    let code = r#"
def titlecase(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn titlecase"), "Got: {}", result);
}

#[test]
fn test_s12_str_swapcase() {
    let code = r#"
def swap_it(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_it"), "Got: {}", result);
}

#[test]
fn test_s12_str_center() {
    let code = r#"
def center_it(s: str) -> str:
    return s.center(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_it"), "Got: {}", result);
}

#[test]
fn test_s12_str_ljust() {
    let code = r#"
def left_justify(s: str) -> str:
    return s.ljust(10)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_justify"), "Got: {}", result);
}

#[test]
fn test_s12_str_rjust() {
    let code = r#"
def right_justify(s: str) -> str:
    return s.rjust(10)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_justify"), "Got: {}", result);
}

#[test]
fn test_s12_str_zfill() {
    let code = r#"
def zero_fill(s: str) -> str:
    return s.zfill(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_fill"), "Got: {}", result);
}

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
fn test_s12_str_encode() {
    let code = r#"
def encode_it(s: str) -> str:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode_it"), "Got: {}", result);
}

#[test]
fn test_s12_str_casefold() {
    let code = r#"
def casefold_it(s: str) -> str:
    return s.casefold()
"#;
    let result = transpile(code);
    assert!(result.contains("fn casefold_it"), "Got: {}", result);
}

#[test]
fn test_s12_str_partition() {
    let code = r#"
def split_at(s: str) -> tuple:
    return s.partition("-")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at"), "Got: {}", result);
}

#[test]
fn test_s12_str_rpartition() {
    let code = r#"
def rsplit_at(s: str) -> tuple:
    return s.rpartition("-")
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_at"), "Got: {}", result);
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

// ===== Float floor division =====

#[test]
fn test_s12_float_floor_div() {
    let code = r#"
def float_div(a: float, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn float_div"), "Got: {}", result);
}

#[test]
fn test_s12_mixed_floor_div() {
    let code = r#"
def mixed_div(a: int, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn mixed_div"), "Got: {}", result);
}

// ===== In operator with different containers =====

#[test]
fn test_s12_in_dict() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_key"), "Got: {}", result);
}

#[test]
fn test_s12_not_in_dict() {
    let code = r#"
def missing_key(d: dict, key: str) -> bool:
    return key not in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn missing_key"), "Got: {}", result);
}

#[test]
fn test_s12_in_list() {
    let code = r#"
def contains(items: list, x: int) -> bool:
    return x in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains"), "Got: {}", result);
}

#[test]
fn test_s12_in_set() {
    let code = r#"
def in_set(s: set, x: int) -> bool:
    return x in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_set"), "Got: {}", result);
}

#[test]
fn test_s12_in_string() {
    let code = r#"
def contains_char(text: str, ch: str) -> bool:
    return ch in text
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains_char"), "Got: {}", result);
}

// ===== Bitwise operations =====

#[test]
fn test_s12_bitwise_and() {
    let code = r#"
def bit_and(a: int, b: int) -> int:
    return a & b
"#;
    let result = transpile(code);
    assert!(result.contains("fn bit_and"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_or() {
    let code = r#"
def bit_or(a: int, b: int) -> int:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("fn bit_or"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_xor() {
    let code = r#"
def bit_xor(a: int, b: int) -> int:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(result.contains("fn bit_xor"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_left_shift() {
    let code = r#"
def left_shift(a: int, n: int) -> int:
    return a << n
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_shift"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_right_shift() {
    let code = r#"
def right_shift(a: int, n: int) -> int:
    return a >> n
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_shift"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_not() {
    let code = r#"
def bit_not(a: int) -> int:
    return ~a
"#;
    let result = transpile(code);
    assert!(result.contains("fn bit_not"), "Got: {}", result);
}

// ===== Complex slice patterns =====

#[test]
fn test_s12_slice_step() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"), "Got: {}", result);
}

#[test]
fn test_s12_slice_reverse() {
    let code = r#"
def reverse_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"), "Got: {}", result);
}

#[test]
fn test_s12_slice_negative_indices() {
    let code = r#"
def last_three(items: list) -> list:
    return items[-3:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_three"), "Got: {}", result);
}

#[test]
fn test_s12_string_slice() {
    let code = r#"
def substring(s: str) -> str:
    return s[1:4]
"#;
    let result = transpile(code);
    assert!(result.contains("fn substring"), "Got: {}", result);
}

// ===== Chained comparisons =====

#[test]
fn test_s12_chained_comparison_three() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 <= x < 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_chained_comparison_equality() {
    let code = r#"
def all_equal(a: int, b: int, c: int) -> bool:
    return a == b == c
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_equal"), "Got: {}", result);
}

// ===== Augmented assignments =====

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
def set_bits(x: int, bits: int) -> int:
    x |= bits
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_bits"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitwise_xor() {
    let code = r#"
def toggle_bits(x: int, bits: int) -> int:
    x ^= bits
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn toggle_bits"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_shift_left() {
    let code = r#"
def shift_up(x: int) -> int:
    x <<= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_up"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_shift_right() {
    let code = r#"
def shift_down(x: int) -> int:
    x >>= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_down"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_floor_div() {
    let code = r#"
def halve(x: int) -> int:
    x //= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_power() {
    let code = r#"
def square(x: int) -> int:
    x **= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn square"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_modulo() {
    let code = r#"
def wrap(x: int, n: int) -> int:
    x %= n
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn wrap"), "Got: {}", result);
}

// ===== Complex expression patterns =====

#[test]
fn test_s12_nested_ternary() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return lo if x < lo else (hi if x > hi else x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

#[test]
fn test_s12_ternary_in_return() {
    let code = r#"
def abs_val(x: int) -> int:
    return -x if x < 0 else x
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"), "Got: {}", result);
}

#[test]
fn test_s12_boolean_short_circuit() {
    let code = r#"
def safe_access(items: list, idx: int) -> bool:
    return idx >= 0 and idx < len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_access"), "Got: {}", result);
}

#[test]
fn test_s12_or_default() {
    let code = r#"
def get_or_default(x: str, default: str) -> str:
    return x or default
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_or_default"), "Got: {}", result);
}

// ===== Dict operations =====

#[test]
fn test_s12_dict_get_with_default() {
    let code = r#"
def lookup(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"), "Got: {}", result);
}

#[test]
fn test_s12_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"), "Got: {}", result);
}

#[test]
fn test_s12_dict_update() {
    let code = r#"
def merge_dicts(a: dict, b: dict) -> dict:
    a.update(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_dicts"), "Got: {}", result);
}

#[test]
fn test_s12_dict_pop() {
    let code = r#"
def remove(d: dict, key: str) -> int:
    return d.pop(key, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove"), "Got: {}", result);
}

#[test]
fn test_s12_dict_keys() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_keys"), "Got: {}", result);
}

#[test]
fn test_s12_dict_values() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_values"), "Got: {}", result);
}

#[test]
fn test_s12_dict_items() {
    let code = r#"
def get_items(d: dict) -> list:
    return list(d.items())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_items"), "Got: {}", result);
}

// ===== List operations =====

#[test]
fn test_s12_list_insert() {
    let code = r#"
def insert_at(items: list, idx: int, val: int) -> list:
    items.insert(idx, val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_at"), "Got: {}", result);
}

#[test]
fn test_s12_list_remove() {
    let code = r#"
def remove_val(items: list, val: int) -> list:
    items.remove(val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_val"), "Got: {}", result);
}

#[test]
fn test_s12_list_extend() {
    let code = r#"
def combine(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Got: {}", result);
}

#[test]
fn test_s12_list_sort() {
    let code = r#"
def sorted_list(items: list) -> list:
    items.sort()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn sorted_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_reverse() {
    let code = r#"
def reversed_list(items: list) -> list:
    items.reverse()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn reversed_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_clear() {
    let code = r#"
def empty_list(items: list) -> list:
    items.clear()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_copy() {
    let code = r#"
def clone_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_list"), "Got: {}", result);
}

// ===== Set operations =====

#[test]
fn test_s12_set_add() {
    let code = r#"
def add_to_set(s: set, x: int) -> set:
    s.add(x)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_to_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_discard() {
    let code = r#"
def discard_from_set(s: set, x: int) -> set:
    s.discard(x)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn discard_from_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_union() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sets"), "Got: {}", result);
}

#[test]
fn test_s12_set_intersection() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn common"), "Got: {}", result);
}

#[test]
fn test_s12_set_difference() {
    let code = r#"
def only_in_a(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_a"), "Got: {}", result);
}

// ===== Type conversion builtins =====

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
def to_bool(n: int) -> bool:
    return bool(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Got: {}", result);
}

// ===== Math builtin functions =====

#[test]
fn test_s12_abs_int() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"), "Got: {}", result);
}

#[test]
fn test_s12_round_float() {
    let code = r#"
def round_it(x: float) -> int:
    return round(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_it"), "Got: {}", result);
}

#[test]
fn test_s12_min_two() {
    let code = r#"
def minimum(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn minimum"), "Got: {}", result);
}

#[test]
fn test_s12_max_two() {
    let code = r#"
def maximum(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn maximum"), "Got: {}", result);
}

#[test]
fn test_s12_sum_list() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

#[test]
fn test_s12_len_list() {
    let code = r#"
def size(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn size"), "Got: {}", result);
}

#[test]
fn test_s12_sorted_builtin() {
    let code = r#"
def sort_copy(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_copy"), "Got: {}", result);
}

#[test]
fn test_s12_reversed_builtin() {
    let code = r#"
def rev_copy(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn rev_copy"), "Got: {}", result);
}

// ===== String formatting =====

#[test]
fn test_s12_fstring_simple() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_expression() {
    let code = r#"
def describe(x: int) -> str:
    return f"Value is {x * 2}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

#[test]
fn test_s12_format_method() {
    let code = r#"
def format_it(name: str, age: int) -> str:
    return "{} is {} years old".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_it"), "Got: {}", result);
}

// ===== Complex patterns =====

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
fn test_s12_dict_comprehension() {
    let code = r#"
def squares(n: int) -> dict:
    return {i: i * i for i in range(n)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Got: {}", result);
}

#[test]
fn test_s12_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"), "Got: {}", result);
}

#[test]
fn test_s12_generator_in_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"), "Got: {}", result);
}

#[test]
fn test_s12_generator_in_min() {
    let code = r#"
def min_abs(items: list) -> int:
    return min(abs(x) for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_abs"), "Got: {}", result);
}

#[test]
fn test_s12_generator_in_max() {
    let code = r#"
def max_len(words: list) -> int:
    return max(len(w) for w in words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_len"), "Got: {}", result);
}

#[test]
fn test_s12_any_generator() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_positive"), "Got: {}", result);
}

#[test]
fn test_s12_all_generator() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

// ===== Power operator =====

#[test]
fn test_s12_power_int() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_power_float() {
    let code = r#"
def fpower(base: float, exp: float) -> float:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn fpower"), "Got: {}", result);
}

// ===== Unary operators =====

#[test]
fn test_s12_unary_not() {
    let code = r#"
def negate(x: bool) -> bool:
    return not x
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"), "Got: {}", result);
}

#[test]
fn test_s12_unary_negative() {
    let code = r#"
def neg(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn neg"), "Got: {}", result);
}

#[test]
fn test_s12_unary_positive() {
    let code = r#"
def pos(x: int) -> int:
    return +x
"#;
    let result = transpile(code);
    assert!(result.contains("fn pos"), "Got: {}", result);
}

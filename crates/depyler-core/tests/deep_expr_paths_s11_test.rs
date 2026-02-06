//! Session 11: Deep expression generation paths coverage
//!
//! Targets the highest-impact uncovered paths in:
//! - expr_gen.rs (68% coverage, 3575 missed regions)
//! - expr_gen_instance_methods.rs (65% coverage, 4421 missed regions)
//! - direct_rules_convert.rs (61% coverage, 3396 missed regions)
//!
//! Focuses on:
//! - Complex method chains on various types
//! - dict/set method dispatch
//! - Unusual string methods
//! - Numeric coercion in expressions
//! - Collection comprehension variants
//! - Complex call expressions

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
// Dict method variations
// ============================================================================

#[test]
fn test_s11_deep_dict_get_no_default() {
    let code = r#"
def lookup(d: dict, key: str) -> str:
    return d.get(key)
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_clear() {
    let code = r#"
def reset(d: dict) -> None:
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reset"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_in_check() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_key"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_not_in() {
    let code = r#"
def missing(d: dict, key: str) -> bool:
    return key not in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn missing"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_len() {
    let code = r#"
def size(d: dict) -> int:
    return len(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn size"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_iteration() {
    let code = r#"
def keys_list(d: dict) -> list:
    result: list = []
    for k in d:
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn keys_list"), "Got: {}", result);
}

// ============================================================================
// Set operations
// ============================================================================

#[test]
fn test_s11_deep_set_add() {
    let code = r#"
def add_item(s: set, item: int) -> None:
    s.add(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_item"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_remove() {
    let code = r#"
def remove_item(s: set, item: int) -> None:
    s.remove(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_item"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_in_check() {
    let code = r#"
def contains(s: set, item: int) -> bool:
    return item in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_union() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sets"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_intersection() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a & b
"#;
    let result = transpile(code);
    assert!(result.contains("fn common"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_difference() {
    let code = r#"
def diff(a: set, b: set) -> set:
    return a - b
"#;
    let result = transpile(code);
    assert!(result.contains("fn diff"), "Got: {}", result);
}

// ============================================================================
// Complex string operations
// ============================================================================

#[test]
fn test_s11_deep_str_format() {
    let code = r#"
def fmt(name: str, age: int) -> str:
    return "{} is {} years old".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fmt"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_zfill() {
    let code = r#"
def pad_num(n: str) -> str:
    return n.zfill(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_num"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_center() {
    let code = r#"
def center_text(text: str, width: int) -> str:
    return text.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_ljust() {
    let code = r#"
def left_pad(text: str, width: int) -> str:
    return text.ljust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_pad"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_rjust() {
    let code = r#"
def right_pad(text: str, width: int) -> str:
    return text.rjust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_pad"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_title() {
    let code = r#"
def titleize(text: str) -> str:
    return text.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn titleize"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_capitalize() {
    let code = r#"
def capitalize_first(text: str) -> str:
    return text.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn capitalize_first"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_isalnum() {
    let code = r#"
def is_alphanumeric(text: str) -> bool:
    return text.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alphanumeric"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_isspace() {
    let code = r#"
def is_whitespace(text: str) -> bool:
    return text.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_whitespace"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_isupper() {
    let code = r#"
def is_upper(text: str) -> bool:
    return text.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_upper"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_islower() {
    let code = r#"
def is_lower(text: str) -> bool:
    return text.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_lower"), "Got: {}", result);
}

// ============================================================================
// List method variations
// ============================================================================

#[test]
fn test_s11_deep_list_clear() {
    let code = r#"
def clear_list(items: list) -> None:
    items.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_list"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_count() {
    let code = r#"
def count_val(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_val"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_index() {
    let code = r#"
def find_idx(items: list, val: int) -> int:
    return items.index(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_idx"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_copy() {
    let code = r#"
def clone_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_list"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_pop_with_index() {
    let code = r#"
def pop_first(items: list) -> int:
    return items.pop(0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_first"), "Got: {}", result);
}

// ============================================================================
// Numeric and comparison patterns
// ============================================================================

#[test]
fn test_s11_deep_abs_builtin() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("abs") || result.contains("fn absolute"), "Got: {}", result);
}

#[test]
fn test_s11_deep_min_two() {
    let code = r#"
def smaller(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("min") || result.contains("fn smaller"), "Got: {}", result);
}

#[test]
fn test_s11_deep_max_two() {
    let code = r#"
def larger(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("max") || result.contains("fn larger"), "Got: {}", result);
}

#[test]
fn test_s11_deep_min_list() {
    let code = r#"
def smallest(items: list) -> int:
    return min(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn smallest"), "Got: {}", result);
}

#[test]
fn test_s11_deep_max_list() {
    let code = r#"
def largest(items: list) -> int:
    return max(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn largest"), "Got: {}", result);
}

#[test]
fn test_s11_deep_sum_builtin() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

#[test]
fn test_s11_deep_round_builtin() {
    let code = r#"
def round_val(x: float) -> int:
    return round(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_val"), "Got: {}", result);
}

// ============================================================================
// Complex index and slice patterns
// ============================================================================

#[test]
fn test_s11_deep_negative_index() {
    let code = r#"
def last(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last"), "Got: {}", result);
}

#[test]
fn test_s11_deep_negative_index_2() {
    let code = r#"
def second_last(items: list) -> int:
    return items[-2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn second_last"), "Got: {}", result);
}

#[test]
fn test_s11_deep_slice_reverse() {
    let code = r#"
def reversed_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reversed_list"), "Got: {}", result);
}

// ============================================================================
// Complex method dispatch on containers
// ============================================================================

#[test]
fn test_s11_deep_list_in_check() {
    let code = r#"
def has_item(items: list, target: int) -> bool:
    return target in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_item"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_not_in_check() {
    let code = r#"
def missing(items: list, target: int) -> bool:
    return target not in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn missing"), "Got: {}", result);
}

#[test]
fn test_s11_deep_string_in_check() {
    let code = r#"
def has_sub(text: str, sub: str) -> bool:
    return sub in text
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_sub"), "Got: {}", result);
}

// ============================================================================
// Complex expression combinations
// ============================================================================

#[test]
fn test_s11_deep_nested_index_access() {
    let code = r#"
def matrix_val(m: list, i: int, j: int) -> int:
    return m[i][j]
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_val"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_index_access() {
    let code = r#"
def get_val(d: dict, key: str) -> int:
    return d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_val"), "Got: {}", result);
}

#[test]
fn test_s11_deep_augmented_assign_list() {
    let code = r#"
def extend_items(a: list, b: list) -> list:
    a += b
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn extend_items"), "Got: {}", result);
}

#[test]
fn test_s11_deep_multiple_return_list() {
    let code = r#"
def make_pair(a: int, b: int) -> list:
    return [a, b]
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_pair"), "Got: {}", result);
}

// ============================================================================
// Type casting within expressions
// ============================================================================

#[test]
fn test_s11_deep_int_in_fstring() {
    let code = r#"
def show(n: int) -> str:
    return f"value is {n}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn show"), "Got: {}", result);
}

#[test]
fn test_s11_deep_float_in_fstring() {
    let code = r#"
def show_float(x: float) -> str:
    return f"value is {x}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_float"), "Got: {}", result);
}

#[test]
fn test_s11_deep_bool_in_fstring() {
    let code = r#"
def show_bool(b: bool) -> str:
    return f"flag is {b}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_bool"), "Got: {}", result);
}

// ============================================================================
// Enum-like patterns
// ============================================================================

#[test]
fn test_s11_deep_class_as_enum() {
    let code = r#"
class Color:
    RED: int = 0
    GREEN: int = 1
    BLUE: int = 2

def color_name(c: int) -> str:
    if c == 0:
        return "red"
    elif c == 1:
        return "green"
    elif c == 2:
        return "blue"
    return "unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("Color") || result.contains("fn color_name"), "Got: {}", result);
}

// ============================================================================
// Complex string method chains
// ============================================================================

#[test]
fn test_s11_deep_method_chain_result() {
    let code = r#"
def process_text(text: str) -> list:
    return text.strip().lower().split()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_text"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_encoding() {
    let code = r#"
def encode_str(text: str) -> str:
    return text.encode("utf-8").decode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode_str"), "Got: {}", result);
}

// ============================================================================
// Comparison patterns
// ============================================================================

#[test]
fn test_s11_deep_is_none() {
    let code = r#"
def check_none(x) -> bool:
    return x is None
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_none"), "Got: {}", result);
}

#[test]
fn test_s11_deep_is_not_none() {
    let code = r#"
def check_not_none(x) -> bool:
    return x is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_not_none"), "Got: {}", result);
}

// ============================================================================
// Exception class patterns
// ============================================================================

#[test]
fn test_s11_deep_custom_exception() {
    let code = r#"
class AppError(Exception):
    def __init__(self, msg: str) -> None:
        self.msg = msg

def validate(x: int) -> int:
    if x < 0:
        raise AppError("negative")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

// ============================================================================
// Property-like access patterns
// ============================================================================

#[test]
fn test_s11_deep_attribute_access() {
    let code = r#"
class Rect:
    def __init__(self, w: int, h: int) -> None:
        self.width = w
        self.height = h

    def area(self) -> int:
        return self.width * self.height

    def perimeter(self) -> int:
        return 2 * (self.width + self.height)
"#;
    let result = transpile(code);
    assert!(result.contains("Rect"), "Got: {}", result);
}

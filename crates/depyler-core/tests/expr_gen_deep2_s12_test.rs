//! Session 12 Batch 36: expr_gen.rs cold paths
//!
//! Targets low-coverage code in expr_gen.rs:
//! - Lambda expressions with captured variables
//! - Walrus operator in comprehensions
//! - Negative slice indices
//! - Generator expressions with filtering
//! - Ternary expressions with type coercion
//! - Named expressions
//! - Complex operator patterns

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

// ===== Lambda expressions =====

#[test]
fn test_s12_b36_lambda_simple() {
    let code = r#"
def apply(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply"), "Got: {}", result);
}

#[test]
fn test_s12_b36_lambda_with_capture() {
    let code = r#"
def multiply_all(items: list, factor: int) -> list:
    return list(map(lambda x: x * factor, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn multiply_all"), "Got: {}", result);
}

#[test]
fn test_s12_b36_lambda_filter() {
    let code = r#"
def filter_positive(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b36_lambda_sorted_key() {
    let code = r#"
def sort_by_length(words: list) -> list:
    return sorted(words, key=lambda w: len(w))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_length"), "Got: {}", result);
}

#[test]
fn test_s12_b36_lambda_in_assign() {
    let code = r#"
def make_doubler():
    double = lambda x: x * 2
    return double(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_doubler"), "Got: {}", result);
}

#[test]
fn test_s12_b36_lambda_multi_param() {
    let code = r#"
def apply_op(a: int, b: int) -> int:
    add = lambda x, y: x + y
    return add(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_op"), "Got: {}", result);
}

// ===== Walrus operator =====

#[test]
fn test_s12_b36_walrus_in_while() {
    let code = r#"
def read_until_empty(items: list) -> int:
    total = 0
    idx = 0
    while (n := items[idx]) != 0:
        total += n
        idx += 1
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_until_empty"), "Got: {}", result);
}

#[test]
fn test_s12_b36_walrus_in_if() {
    let code = r#"
def check_len(s: str) -> str:
    if (n := len(s)) > 10:
        return f"long ({n})"
    return "short"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_len"), "Got: {}", result);
}

// ===== Negative slice indices =====

#[test]
fn test_s12_b36_negative_index() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_item"), "Got: {}", result);
}

#[test]
fn test_s12_b36_negative_slice_end() {
    let code = r#"
def all_but_last(items: list) -> list:
    return items[:-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_but_last"), "Got: {}", result);
}

#[test]
fn test_s12_b36_negative_slice_start() {
    let code = r#"
def last_three(items: list) -> list:
    return items[-3:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_three"), "Got: {}", result);
}

#[test]
fn test_s12_b36_string_reverse() {
    let code = r#"
def reverse_str(s: str) -> str:
    return s[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_str"), "Got: {}", result);
}

#[test]
fn test_s12_b36_slice_step_two() {
    let code = r#"
def odd_indexed(items: list) -> list:
    return items[1::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn odd_indexed"), "Got: {}", result);
}

// ===== Generator expressions =====

#[test]
fn test_s12_b36_gen_sum_filtered() {
    let code = r#"
def sum_positive(items: list) -> int:
    return sum(x for x in items if x > 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b36_gen_any_condition() {
    let code = r#"
def any_empty(strings: list) -> bool:
    return any(len(s) == 0 for s in strings)
"#;
    let result = transpile(code);
    assert!(result.contains("fn any_empty"), "Got: {}", result);
}

#[test]
fn test_s12_b36_gen_all_condition() {
    let code = r#"
def all_nonempty(strings: list) -> bool:
    return all(len(s) > 0 for s in strings)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_nonempty"), "Got: {}", result);
}

#[test]
fn test_s12_b36_gen_min_transform() {
    let code = r#"
def min_abs(items: list) -> int:
    return min(abs(x) for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_abs"), "Got: {}", result);
}

#[test]
fn test_s12_b36_gen_max_transform() {
    let code = r#"
def max_squared(items: list) -> int:
    return max(x * x for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_squared"), "Got: {}", result);
}

// ===== Ternary expressions =====

#[test]
fn test_s12_b36_ternary_basic() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"), "Got: {}", result);
}

#[test]
fn test_s12_b36_ternary_string() {
    let code = r#"
def pluralize(n: int) -> str:
    return "item" if n == 1 else "items"
"#;
    let result = transpile(code);
    assert!(result.contains("fn pluralize"), "Got: {}", result);
}

#[test]
fn test_s12_b36_ternary_in_fstring() {
    let code = r#"
def label(x: int) -> str:
    return f"{'positive' if x > 0 else 'non-positive'}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn label"), "Got: {}", result);
}

#[test]
fn test_s12_b36_nested_ternary() {
    let code = r#"
def sign(x: int) -> str:
    return "positive" if x > 0 else ("negative" if x < 0 else "zero")
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

// ===== Complex operator patterns =====

#[test]
fn test_s12_b36_chained_compare() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_b36_power_op() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube"), "Got: {}", result);
}

#[test]
fn test_s12_b36_floor_div() {
    let code = r#"
def integer_divide(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn integer_divide"), "Got: {}", result);
}

#[test]
fn test_s12_b36_modulo() {
    let code = r#"
def remainder(a: int, b: int) -> int:
    return a % b
"#;
    let result = transpile(code);
    assert!(result.contains("fn remainder"), "Got: {}", result);
}

#[test]
fn test_s12_b36_bitwise_ops() {
    let code = r#"
def bit_ops(a: int, b: int) -> int:
    x = a & b
    y = a | b
    z = a ^ b
    w = ~a
    return x + y + z + w
"#;
    let result = transpile(code);
    assert!(result.contains("fn bit_ops"), "Got: {}", result);
}

#[test]
fn test_s12_b36_shift_ops() {
    let code = r#"
def shifts(x: int, n: int) -> tuple:
    left = x << n
    right = x >> n
    return (left, right)
"#;
    let result = transpile(code);
    assert!(result.contains("fn shifts"), "Got: {}", result);
}

// ===== Complex comprehension patterns =====

#[test]
fn test_s12_b36_nested_list_comp() {
    let code = r#"
def pairs(n: int) -> list:
    return [(i, j) for i in range(n) for j in range(i)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b36_comp_with_method() {
    let code = r#"
def lowercase_words(text: str) -> list:
    return [w.lower() for w in text.split()]
"#;
    let result = transpile(code);
    assert!(result.contains("fn lowercase_words"), "Got: {}", result);
}

#[test]
fn test_s12_b36_comp_with_condition_method() {
    let code = r#"
def digits_only(items: list) -> list:
    return [s for s in items if s.isdigit()]
"#;
    let result = transpile(code);
    assert!(result.contains("fn digits_only"), "Got: {}", result);
}

#[test]
fn test_s12_b36_dict_comp_filter() {
    let code = r#"
def positive_entries(d: dict) -> dict:
    return {k: v for k, v in d.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_entries"), "Got: {}", result);
}

#[test]
fn test_s12_b36_set_comp_transform() {
    let code = r#"
def unique_first_chars(words: list) -> set:
    return {w[0] for w in words if len(w) > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_first_chars"), "Got: {}", result);
}

// ===== Yield and generator functions =====

#[test]
fn test_s12_b36_yield_simple() {
    let code = r#"
def gen_range(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn gen_range"), "Got: {}", result);
}

#[test]
fn test_s12_b36_yield_filtered() {
    let code = r#"
def gen_evens(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    let result = transpile(code);
    assert!(result.contains("fn gen_evens"), "Got: {}", result);
}

// ===== Complex f-string patterns =====

#[test]
fn test_s12_b36_fstring_with_expr() {
    let code = r#"
def describe(items: list) -> str:
    return f"List has {len(items)} items, sum={sum(items)}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

#[test]
fn test_s12_b36_fstring_multipart() {
    let code = r#"
def format_record(name: str, age: int, city: str) -> str:
    return f"Name: {name}, Age: {age}, City: {city}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_record"), "Got: {}", result);
}

#[test]
fn test_s12_b36_fstring_with_method() {
    let code = r#"
def format_upper(name: str) -> str:
    return f"HELLO {name.upper()}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_upper"), "Got: {}", result);
}

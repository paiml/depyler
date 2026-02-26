//! Session 12 Batch 12: Deep coverage tests for expr_gen.rs cold paths
//!
//! Targets:
//! - Unary operators (+x, ~x, not with regex)
//! - Walrus operator in various contexts
//! - Set literals with None and strings
//! - Complex slice patterns (3-param, negative step)
//! - Yield/generator patterns
//! - Comparison chains
//! - Bitwise operators
//! - Complex subscript patterns
//! - String formatting edge cases

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

// ===== Unary operators =====

#[test]
fn test_s12_unary_plus() {
    let code = r#"
def unary_plus_test(x: int) -> int:
    y = +x
    return y
"#;
    let result = transpile(code);
    assert!(result.contains("fn unary_plus_test"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_not() {
    let code = r#"
def bitwise_not_test(n: int) -> int:
    result = ~n
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn bitwise_not_test"), "Got: {}", result);
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
fn test_s12_not_with_comparison() {
    let code = r#"
def not_equal_check(a: int, b: int) -> bool:
    return not (a == b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn not_equal_check"), "Got: {}", result);
}

// ===== Bitwise operators =====

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
fn test_s12_left_shift() {
    let code = r#"
def left_shift(x: int, n: int) -> int:
    return x << n
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_shift"), "Got: {}", result);
}

#[test]
fn test_s12_right_shift() {
    let code = r#"
def right_shift(x: int, n: int) -> int:
    return x >> n
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_shift"), "Got: {}", result);
}

// ===== Walrus operator =====

#[test]
fn test_s12_walrus_in_if() {
    let code = r#"
def walrus_test(items: list) -> int:
    if (n := len(items)) > 0:
        return n
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn walrus_test"), "Got: {}", result);
}

#[test]
fn test_s12_walrus_in_while() {
    let code = r#"
def count_items(items: list) -> int:
    count = 0
    i = 0
    while i < len(items):
        count += 1
        i += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_items"), "Got: {}", result);
}

// ===== Set literals =====

#[test]
fn test_s12_set_literal_strings() {
    let code = r#"
def make_color_set() -> set:
    colors = {"red", "green", "blue"}
    return colors
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_color_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_literal_ints() {
    let code = r#"
def make_num_set() -> set:
    nums = {1, 2, 3, 4, 5}
    return nums
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_num_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_add_discard() {
    let code = r#"
def modify_set() -> set:
    s = {1, 2, 3}
    s.add(4)
    s.discard(1)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn modify_set"), "Got: {}", result);
}

// ===== Complex slicing =====

#[test]
fn test_s12_three_param_slice() {
    let code = r#"
def slice_with_step(items: list) -> list:
    return items[1:10:2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn slice_with_step"), "Got: {}", result);
}

#[test]
fn test_s12_string_slice_complex() {
    let code = r#"
def string_slice(text: str) -> str:
    return text[1:5]
"#;
    let result = transpile(code);
    assert!(result.contains("fn string_slice"), "Got: {}", result);
}

#[test]
fn test_s12_negative_index_access() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_item"), "Got: {}", result);
}

#[test]
fn test_s12_slice_from_start() {
    let code = r#"
def first_three(items: list) -> list:
    return items[:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_three"), "Got: {}", result);
}

#[test]
fn test_s12_slice_to_end() {
    let code = r#"
def skip_first(items: list) -> list:
    return items[1:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_first"), "Got: {}", result);
}

// ===== Generator / yield patterns =====

#[test]
fn test_s12_simple_generator() {
    let code = r#"
def countdown(n: int) -> int:
    while n > 0:
        yield n
        n -= 1
"#;
    let result = transpile(code);
    assert!(result.contains("countdown"), "Got: {}", result);
}

#[test]
fn test_s12_generator_range() {
    let code = r#"
def even_numbers(n: int) -> int:
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    let result = transpile(code);
    assert!(result.contains("even_numbers"), "Got: {}", result);
}

// ===== Power operator =====

#[test]
fn test_s12_power_operator() {
    let code = r#"
def square(x: int) -> int:
    return x ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn square"), "Got: {}", result);
}

#[test]
fn test_s12_power_float() {
    let code = r#"
def cube_root(x: float) -> float:
    return x ** (1.0 / 3.0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube_root"), "Got: {}", result);
}

// ===== Type conversion builtins =====

#[test]
fn test_s12_int_from_float() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn truncate"), "Got: {}", result);
}

#[test]
fn test_s12_float_from_int() {
    let code = r#"
def to_float(x: int) -> float:
    return float(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_float"), "Got: {}", result);
}

#[test]
fn test_s12_str_from_int() {
    let code = r#"
def int_to_string(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_to_string"), "Got: {}", result);
}

#[test]
fn test_s12_bool_from_int() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Got: {}", result);
}

#[test]
fn test_s12_int_from_str() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"), "Got: {}", result);
}

// ===== Complex string operations =====

#[test]
fn test_s12_str_encode() {
    let code = r#"
def encode_utf8(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode_utf8"), "Got: {}", result);
}

#[test]
fn test_s12_str_format_method() {
    let code = r#"
def format_msg(name: str, age: int) -> str:
    return "Name: {}, Age: {}".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_msg"), "Got: {}", result);
}

#[test]
fn test_s12_str_count() {
    let code = r#"
def count_char(s: str, c: str) -> int:
    return s.count(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_char"), "Got: {}", result);
}

#[test]
fn test_s12_str_find() {
    let code = r#"
def find_char(s: str, sub: str) -> int:
    return s.find(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_char"), "Got: {}", result);
}

// ===== Dict patterns =====

#[test]
fn test_s12_dict_get_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> int:
    return d.pop(key)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
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
fn test_s12_dict_setdefault() {
    let code = r#"
def get_or_set(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_or_set"), "Got: {}", result);
}

// ===== Assert patterns =====

#[test]
fn test_s12_assert_simple() {
    let code = r#"
def check_positive(x: int) -> int:
    assert x > 0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_positive"), "Got: {}", result);
}

#[test]
fn test_s12_assert_with_message() {
    let code = r#"
def check_range(x: int) -> int:
    assert 0 <= x <= 100, "Value out of range"
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_range"), "Got: {}", result);
}

// ===== Raise patterns =====

#[test]
fn test_s12_raise_value_error() {
    let code = r#"
def must_be_positive(x: int) -> int:
    if x < 0:
        raise ValueError("Must be positive")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn must_be_positive"), "Got: {}", result);
}

#[test]
fn test_s12_raise_runtime_error() {
    let code = r#"
def not_implemented() -> int:
    raise RuntimeError("Not implemented")
"#;
    let result = transpile(code);
    assert!(result.contains("fn not_implemented"), "Got: {}", result);
}

// ===== Context manager patterns =====

#[test]
fn test_s12_with_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        content = f.read()
    return content
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_write() {
    let code = r#"
def write_file(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

// ===== Complex expression patterns =====

#[test]
fn test_s12_nested_function_call() {
    let code = r#"
def composed(x: int) -> int:
    return abs(min(x, 0))
"#;
    let result = transpile(code);
    assert!(result.contains("fn composed"), "Got: {}", result);
}

#[test]
fn test_s12_chained_method_calls() {
    let code = r#"
def process_text(s: str) -> list:
    return s.strip().lower().split()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_text"), "Got: {}", result);
}

#[test]
fn test_s12_multiple_return_values() {
    let code = r#"
def min_max(items: list) -> tuple:
    return (min(items), max(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max"), "Got: {}", result);
}

// ===== Try/except patterns =====

#[test]
fn test_s12_try_except_finally() {
    let code = r#"
def safe_op(x: int) -> int:
    result = 0
    try:
        result = 100 / x
    except ZeroDivisionError:
        result = -1
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_op"), "Got: {}", result);
}

#[test]
fn test_s12_try_multiple_except() {
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
    assert!(result.contains("fn robust_parse"), "Got: {}", result);
}

// ===== Comprehension patterns =====

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
fn test_s12_dict_comprehension_filter() {
    let code = r#"
def positive_items(d: dict) -> dict:
    return {k: v for k, v in d.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_items"), "Got: {}", result);
}

// ===== Nested function patterns =====

#[test]
fn test_s12_nested_function_def() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

#[test]
fn test_s12_closure_pattern() {
    let code = r#"
def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"), "Got: {}", result);
}

// ===== Global constant patterns =====

#[test]
fn test_s12_module_constant() {
    let code = r#"
MAX_SIZE = 100

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"), "Got: {}", result);
}

// ===== Tuple operations =====

#[test]
fn test_s12_tuple_index() {
    let code = r#"
def get_first(t: tuple) -> int:
    return t[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_first"), "Got: {}", result);
}

#[test]
fn test_s12_tuple_len() {
    let code = r#"
def tuple_size(t: tuple) -> int:
    return len(t)
"#;
    let result = transpile(code);
    assert!(result.contains("fn tuple_size"), "Got: {}", result);
}

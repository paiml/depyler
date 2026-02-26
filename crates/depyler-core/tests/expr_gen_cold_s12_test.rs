//! Session 12 Batch 49: Expression generation cold paths
//!
//! Targets remaining cold paths in expr_gen.rs:
//! - Complex slice operations with step
//! - Nested binary operations
//! - Attribute access patterns
//! - Starred expressions
//! - Complex f-string type inference
//! - Generator expressions in function calls
//! - Comparison chaining
//! - Bitwise operations
//! - Complex subscript patterns

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

// ===== Complex slice operations =====

#[test]
fn test_s12_b49_slice_with_step() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"), "Got: {}", result);
}

#[test]
fn test_s12_b49_slice_reverse() {
    let code = r#"
def reverse_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"), "Got: {}", result);
}

#[test]
fn test_s12_b49_slice_negative_start() {
    let code = r#"
def last_n(items: list, n: int) -> list:
    return items[-n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_n"), "Got: {}", result);
}

#[test]
fn test_s12_b49_slice_both_bounds() {
    let code = r#"
def middle(items: list, start: int, end: int) -> list:
    return items[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn middle"), "Got: {}", result);
}

#[test]
fn test_s12_b49_string_slice() {
    let code = r#"
def first_chars(s: str, n: int) -> str:
    return s[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_chars"), "Got: {}", result);
}

#[test]
fn test_s12_b49_string_slice_from() {
    let code = r#"
def skip_prefix(s: str, n: int) -> str:
    return s[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_prefix"), "Got: {}", result);
}

// ===== Nested binary operations =====

#[test]
fn test_s12_b49_nested_arithmetic() {
    let code = r#"
def quadratic(a: float, b: float, c: float, x: float) -> float:
    return a * x * x + b * x + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn quadratic"), "Got: {}", result);
}

#[test]
fn test_s12_b49_complex_boolean() {
    let code = r#"
def in_range(x: int, low: int, high: int) -> bool:
    return x >= low and x <= high
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_b49_mixed_arithmetic_bool() {
    let code = r#"
def passes_threshold(values: list, threshold: float) -> bool:
    total = sum(values)
    avg = total / len(values)
    return avg > threshold and len(values) >= 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn passes_threshold"), "Got: {}", result);
}

// ===== Attribute access patterns =====

#[test]
fn test_s12_b49_chained_attribute() {
    let code = r#"
class Config:
    def __init__(self):
        self.db = DatabaseConfig()
        self.name = "app"

class DatabaseConfig:
    def __init__(self):
        self.host = "localhost"
        self.port = 5432

def get_host(config: Config) -> str:
    return config.db.host
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_host"), "Got: {}", result);
}

#[test]
fn test_s12_b49_method_on_attribute() {
    let code = r#"
class Wrapper:
    def __init__(self, data: str):
        self.data = data

def upper_data(w: Wrapper) -> str:
    return w.data.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_data"), "Got: {}", result);
}

// ===== Complex f-string patterns =====

#[test]
fn test_s12_b49_fstring_with_format_spec() {
    let code = r##"
def format_price(amount: float) -> str:
    return f"${amount:.2f}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_price"), "Got: {}", result);
}

#[test]
fn test_s12_b49_fstring_with_method() {
    let code = r##"
def greet(name: str) -> str:
    return f"Hello, {name.upper()}!"
"##;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_b49_fstring_with_expr() {
    let code = r##"
def describe(n: int) -> str:
    return f"{n} is {'even' if n % 2 == 0 else 'odd'}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

#[test]
fn test_s12_b49_fstring_multi_part() {
    let code = r##"
def format_record(name: str, age: int, city: str) -> str:
    return f"Name: {name}, Age: {age}, City: {city}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_record"), "Got: {}", result);
}

// ===== Generator expressions in function calls =====

#[test]
fn test_s12_b49_sum_generator() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(i * i for i in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"), "Got: {}", result);
}

#[test]
fn test_s12_b49_any_generator() {
    let code = r#"
def has_negative(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_negative"), "Got: {}", result);
}

#[test]
fn test_s12_b49_all_generator() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b49_min_generator() {
    let code = r#"
def shortest_len(strings: list) -> int:
    return min(len(s) for s in strings)
"#;
    let result = transpile(code);
    assert!(result.contains("fn shortest_len"), "Got: {}", result);
}

#[test]
fn test_s12_b49_max_generator() {
    let code = r#"
def longest_len(strings: list) -> int:
    return max(len(s) for s in strings)
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_len"), "Got: {}", result);
}

// ===== Comparison chaining =====

#[test]
fn test_s12_b49_chained_compare() {
    let code = r#"
def in_bounds(x: int) -> bool:
    return 0 <= x < 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_bounds"), "Got: {}", result);
}

#[test]
fn test_s12_b49_triple_compare() {
    let code = r#"
def between(x: int, low: int, high: int) -> bool:
    return low <= x <= high
"#;
    let result = transpile(code);
    assert!(result.contains("fn between"), "Got: {}", result);
}

// ===== Bitwise operations =====

#[test]
fn test_s12_b49_bitwise_and() {
    let code = r#"
def mask_bits(value: int, mask: int) -> int:
    return value & mask
"#;
    let result = transpile(code);
    assert!(result.contains("fn mask_bits"), "Got: {}", result);
}

#[test]
fn test_s12_b49_bitwise_or() {
    let code = r#"
def set_flags(current: int, flags: int) -> int:
    return current | flags
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_flags"), "Got: {}", result);
}

#[test]
fn test_s12_b49_bitwise_xor() {
    let code = r#"
def toggle_bits(value: int, mask: int) -> int:
    return value ^ mask
"#;
    let result = transpile(code);
    assert!(result.contains("fn toggle_bits"), "Got: {}", result);
}

#[test]
fn test_s12_b49_bitwise_shift() {
    let code = r#"
def shift_ops(x: int) -> int:
    left = x << 2
    right = x >> 1
    return left + right
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_ops"), "Got: {}", result);
}

#[test]
fn test_s12_b49_bitwise_not() {
    let code = r#"
def invert(x: int) -> int:
    return ~x
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert"), "Got: {}", result);
}

// ===== Power and floor division =====

#[test]
fn test_s12_b49_power_int() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_b49_floor_div() {
    let code = r#"
def int_divide(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_divide"), "Got: {}", result);
}

// ===== Ternary in various contexts =====

#[test]
fn test_s12_b49_ternary_in_return() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"), "Got: {}", result);
}

#[test]
fn test_s12_b49_ternary_in_assignment() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    result = lo if x < lo else (hi if x > hi else x)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

#[test]
fn test_s12_b49_ternary_in_list() {
    let code = r#"
def sign_list(items: list) -> list:
    return [1 if x > 0 else (-1 if x < 0 else 0) for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign_list"), "Got: {}", result);
}

// ===== isinstance patterns =====

#[test]
fn test_s12_b49_isinstance_int() {
    let code = r#"
def is_int(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_int"), "Got: {}", result);
}

#[test]
fn test_s12_b49_isinstance_str() {
    let code = r#"
def is_string(x) -> bool:
    return isinstance(x, str)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_string"), "Got: {}", result);
}

// ===== Unary operations =====

#[test]
fn test_s12_b49_unary_neg() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"), "Got: {}", result);
}

#[test]
fn test_s12_b49_unary_not() {
    let code = r#"
def invert_bool(b: bool) -> bool:
    return not b
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert_bool"), "Got: {}", result);
}

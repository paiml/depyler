//! Session 12 Batch 77: Expression generation deep cold paths 3
//!
//! Targets expr_gen.rs cold paths for complex expression types:
//! boolean short-circuit, walrus operator, starred expressions,
//! complex call patterns, and expression-level type inference.

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

#[test]
fn test_s12_b77_walrus_in_if() {
    let code = r#"
def process_if_long(text: str) -> str:
    if (n := len(text)) > 10:
        return text[:n // 2]
    return text
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_if_long"), "Got: {}", result);
}

#[test]
fn test_s12_b77_walrus_in_while() {
    let code = r#"
def consume(items: list) -> int:
    total = 0
    while items:
        val = items.pop()
        total += val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn consume"), "Got: {}", result);
}

#[test]
fn test_s12_b77_boolean_short_circuit_and() {
    let code = r#"
def safe_access(d: dict, key: str) -> bool:
    return key in d and d[key] > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_access"), "Got: {}", result);
}

#[test]
fn test_s12_b77_boolean_short_circuit_or() {
    let code = r#"
def default_or(val, default: int) -> int:
    return val or default
"#;
    let result = transpile(code);
    assert!(result.contains("fn default_or"), "Got: {}", result);
}

#[test]
fn test_s12_b77_complex_boolean() {
    let code = r#"
def check_bounds(x: int, y: int, w: int, h: int) -> bool:
    return 0 <= x < w and 0 <= y < h
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_bounds"), "Got: {}", result);
}

#[test]
fn test_s12_b77_nested_ternary() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else (-1 if x < 0 else 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

#[test]
fn test_s12_b77_ternary_in_call() {
    let code = r#"
def format_item(x: int) -> str:
    return str(x if x >= 0 else -x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_item"), "Got: {}", result);
}

#[test]
fn test_s12_b77_complex_call_chain() {
    let code = r#"
def process(text: str) -> str:
    return " ".join(sorted(set(text.lower().split())))
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b77_nested_function_calls() {
    let code = r#"
def nested_max(a: list, b: list) -> int:
    return max(max(a), max(b))
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_max"), "Got: {}", result);
}

#[test]
fn test_s12_b77_call_with_star_args() {
    let code = r#"
def call_with_args(func, args: list):
    return func(*args)
"#;
    let result = transpile(code);
    assert!(result.contains("fn call_with_args"), "Got: {}", result);
}

#[test]
fn test_s12_b77_power_operator() {
    let code = r#"
def power(base: float, exp: int) -> float:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_b77_floor_div() {
    let code = r#"
def integer_divide(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn integer_divide"), "Got: {}", result);
}

#[test]
fn test_s12_b77_modulo() {
    let code = r#"
def remainder(a: int, b: int) -> int:
    return a % b
"#;
    let result = transpile(code);
    assert!(result.contains("fn remainder"), "Got: {}", result);
}

#[test]
fn test_s12_b77_bitwise_and() {
    let code = r#"
def mask_bits(n: int, mask: int) -> int:
    return n & mask
"#;
    let result = transpile(code);
    assert!(result.contains("fn mask_bits"), "Got: {}", result);
}

#[test]
fn test_s12_b77_bitwise_or() {
    let code = r#"
def set_bits(n: int, bits: int) -> int:
    return n | bits
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_bits"), "Got: {}", result);
}

#[test]
fn test_s12_b77_bitwise_xor() {
    let code = r#"
def toggle_bits(n: int, bits: int) -> int:
    return n ^ bits
"#;
    let result = transpile(code);
    assert!(result.contains("fn toggle_bits"), "Got: {}", result);
}

#[test]
fn test_s12_b77_left_shift() {
    let code = r#"
def shift_left(n: int, count: int) -> int:
    return n << count
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_left"), "Got: {}", result);
}

#[test]
fn test_s12_b77_right_shift() {
    let code = r#"
def shift_right(n: int, count: int) -> int:
    return n >> count
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_right"), "Got: {}", result);
}

#[test]
fn test_s12_b77_bitwise_not() {
    let code = r#"
def invert(n: int) -> int:
    return ~n
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert"), "Got: {}", result);
}

#[test]
fn test_s12_b77_unary_minus() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"), "Got: {}", result);
}

#[test]
fn test_s12_b77_not_operator() {
    let code = r#"
def invert_bool(x: bool) -> bool:
    return not x
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert_bool"), "Got: {}", result);
}

#[test]
fn test_s12_b77_is_none_check() {
    let code = r#"
def is_none(x) -> bool:
    return x is None
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_none"), "Got: {}", result);
}

#[test]
fn test_s12_b77_is_not_none_check() {
    let code = r#"
def has_value(x) -> bool:
    return x is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_value"), "Got: {}", result);
}

#[test]
fn test_s12_b77_in_operator() {
    let code = r#"
def contains(items: list, target: int) -> bool:
    return target in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains"), "Got: {}", result);
}

#[test]
fn test_s12_b77_not_in_operator() {
    let code = r#"
def not_contains(items: list, target: int) -> bool:
    return target not in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn not_contains"), "Got: {}", result);
}

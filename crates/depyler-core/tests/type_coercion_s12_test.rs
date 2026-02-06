//! Session 12 Batch 71: Type coercion and mixed-type expression cold paths
//!
//! Targets expr_gen.rs cold paths for type coercion between
//! int/float, boolean expressions, and complex expression types.

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
fn test_s12_b71_int_float_add() {
    let code = r#"
def mixed_add(x: int, y: float) -> float:
    return x + y
"#;
    let result = transpile(code);
    assert!(result.contains("fn mixed_add"), "Got: {}", result);
}

#[test]
fn test_s12_b71_int_float_multiply() {
    let code = r#"
def scale(n: int, factor: float) -> float:
    return n * factor
"#;
    let result = transpile(code);
    assert!(result.contains("fn scale"), "Got: {}", result);
}

#[test]
fn test_s12_b71_literal_coercion() {
    let code = r#"
def offset(x: float) -> float:
    return x + 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn offset"), "Got: {}", result);
}

#[test]
fn test_s12_b71_beta_decay() {
    let code = r#"
def decay(value: float, rate: float) -> float:
    return value * (1 - rate)
"#;
    let result = transpile(code);
    assert!(result.contains("fn decay"), "Got: {}", result);
}

#[test]
fn test_s12_b71_int_division_to_float() {
    let code = r#"
def average(total: int, count: int) -> float:
    return total / count
"#;
    let result = transpile(code);
    assert!(result.contains("fn average"), "Got: {}", result);
}

#[test]
fn test_s12_b71_float_comparison() {
    let code = r#"
def is_close(a: float, b: float, tol: float) -> bool:
    return abs(a - b) <= tol
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_close"), "Got: {}", result);
}

#[test]
fn test_s12_b71_mixed_comparison() {
    let code = r#"
def in_range(x: int, lo: float, hi: float) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_b71_bool_to_int() {
    let code = r#"
def count_true(items: list) -> int:
    total = 0
    for item in items:
        total += item > 0
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_true"), "Got: {}", result);
}

#[test]
fn test_s12_b71_complex_arithmetic() {
    let code = r#"
def quadratic(a: float, b: float, c: float, x: float) -> float:
    return a * x * x + b * x + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn quadratic"), "Got: {}", result);
}

#[test]
fn test_s12_b71_float_accumulator() {
    let code = r#"
def running_average(values: list) -> list:
    result = []
    total = 0.0
    for i, v in enumerate(values):
        total += v
        result.append(total / (i + 1))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn running_average"), "Got: {}", result);
}

#[test]
fn test_s12_b71_complex_boolean() {
    let code = r#"
def validate(x: int, y: int, z: int) -> bool:
    return (x > 0 and y > 0) or (z > 0 and x + y > z)
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s12_b71_negation_chain() {
    let code = r#"
def none_empty(a: str, b: str, c: str) -> bool:
    return not (not a or not b or not c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn none_empty"), "Got: {}", result);
}

#[test]
fn test_s12_b71_chained_comparison() {
    let code = r#"
def in_bounds(x: int) -> bool:
    return 0 <= x < 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_bounds"), "Got: {}", result);
}

#[test]
fn test_s12_b71_triple_chained_comparison() {
    let code = r#"
def strictly_ordered(a: int, b: int, c: int) -> bool:
    return a < b < c
"#;
    let result = transpile(code);
    assert!(result.contains("fn strictly_ordered"), "Got: {}", result);
}

#[test]
fn test_s12_b71_weighted_sum() {
    let code = r#"
def weighted_avg(values: list, weights: list) -> float:
    total = 0.0
    weight_sum = 0.0
    for v, w in zip(values, weights):
        total += v * w
        weight_sum += w
    if weight_sum == 0.0:
        return 0.0
    return total / weight_sum
"#;
    let result = transpile(code);
    assert!(result.contains("fn weighted_avg"), "Got: {}", result);
}

#[test]
fn test_s12_b71_normalize_vector() {
    let code = r#"
import math

def normalize(vec: list) -> list:
    magnitude = math.sqrt(sum(x * x for x in vec))
    if magnitude == 0.0:
        return vec
    return [x / magnitude for x in vec]
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_b71_clamp() {
    let code = r#"
def clamp(value: float, lo: float, hi: float) -> float:
    if value < lo:
        return lo
    if value > hi:
        return hi
    return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

#[test]
fn test_s12_b71_lerp() {
    let code = r#"
def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * t
"#;
    let result = transpile(code);
    assert!(result.contains("fn lerp"), "Got: {}", result);
}

#[test]
fn test_s12_b71_distance() {
    let code = r#"
import math

def distance(x1: float, y1: float, x2: float, y2: float) -> float:
    dx = x2 - x1
    dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Got: {}", result);
}

#[test]
fn test_s12_b71_sigmoid() {
    let code = r#"
import math

def sigmoid(x: float) -> float:
    return 1.0 / (1.0 + math.exp(-x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sigmoid"), "Got: {}", result);
}

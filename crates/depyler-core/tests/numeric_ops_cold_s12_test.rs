//! Session 12 Batch 54: Numeric operations and math cold paths
//!
//! Targets numeric operation codegen paths:
//! - Integer arithmetic edge cases
//! - Float operations and conversions
//! - Math module function usage
//! - Complex numeric algorithms
//! - Type coercion between int and float

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

// ===== Integer arithmetic =====

#[test]
fn test_s12_b54_int_modulo() {
    let code = r#"
def is_divisible(a: int, b: int) -> bool:
    return a % b == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_divisible"), "Got: {}", result);
}

#[test]
fn test_s12_b54_int_floor_div() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    if b == 0:
        return 0
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_div"), "Got: {}", result);
}

#[test]
fn test_s12_b54_int_power() {
    let code = r#"
def pow_mod(base: int, exp: int, mod: int) -> int:
    result = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp = exp // 2
        base = (base * base) % mod
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pow_mod"), "Got: {}", result);
}

#[test]
fn test_s12_b54_int_abs() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"), "Got: {}", result);
}

#[test]
fn test_s12_b54_int_min_max() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return max(lo, min(x, hi))
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

// ===== Float operations =====

#[test]
fn test_s12_b54_float_division() {
    let code = r#"
def average(items: list) -> float:
    if len(items) == 0:
        return 0.0
    return sum(items) / len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn average"), "Got: {}", result);
}

#[test]
fn test_s12_b54_float_abs() {
    let code = r#"
def float_abs(x: float) -> float:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn float_abs"), "Got: {}", result);
}

#[test]
fn test_s12_b54_float_round() {
    let code = r#"
def round_to(x: float, places: int) -> float:
    return round(x, places)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_to"), "Got: {}", result);
}

#[test]
fn test_s12_b54_float_comparison() {
    let code = r#"
def approx_equal(a: float, b: float, eps: float) -> bool:
    return abs(a - b) < eps
"#;
    let result = transpile(code);
    assert!(result.contains("fn approx_equal"), "Got: {}", result);
}

// ===== Math functions =====

#[test]
fn test_s12_b54_math_sqrt() {
    let code = r#"
import math

def distance(x: float, y: float) -> float:
    return math.sqrt(x * x + y * y)
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Got: {}", result);
}

#[test]
fn test_s12_b54_math_floor_ceil() {
    let code = r#"
import math

def round_both(x: float) -> list:
    return [math.floor(x), math.ceil(x)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_both"), "Got: {}", result);
}

#[test]
fn test_s12_b54_math_log() {
    let code = r#"
import math

def log_base(x: float, base: float) -> float:
    return math.log(x) / math.log(base)
"#;
    let result = transpile(code);
    assert!(result.contains("fn log_base"), "Got: {}", result);
}

#[test]
fn test_s12_b54_math_trig() {
    let code = r#"
import math

def to_cartesian(r: float, theta: float) -> list:
    x = r * math.cos(theta)
    y = r * math.sin(theta)
    return [x, y]
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_cartesian"), "Got: {}", result);
}

// ===== Type conversions =====

#[test]
fn test_s12_b54_int_to_float() {
    let code = r#"
def normalize(value: int, total: int) -> float:
    return float(value) / float(total)
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_b54_float_to_int() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn truncate"), "Got: {}", result);
}

#[test]
fn test_s12_b54_str_to_int() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"), "Got: {}", result);
}

#[test]
fn test_s12_b54_str_to_float() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_float"), "Got: {}", result);
}

#[test]
fn test_s12_b54_int_to_str() {
    let code = r#"
def to_string(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_string"), "Got: {}", result);
}

// ===== Numeric algorithms =====

#[test]
fn test_s12_b54_fibonacci() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Got: {}", result);
}

#[test]
fn test_s12_b54_gcd_euclidean() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Got: {}", result);
}

#[test]
fn test_s12_b54_sieve() {
    let code = r#"
def sieve(n: int) -> list:
    is_prime = [True] * (n + 1)
    primes = []
    for i in range(2, n + 1):
        if is_prime[i]:
            primes.append(i)
            j = i * i
            while j <= n:
                is_prime[j] = False
                j += i
    return primes
"#;
    let result = transpile(code);
    assert!(result.contains("fn sieve"), "Got: {}", result);
}

#[test]
fn test_s12_b54_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Got: {}", result);
}

// ===== Accumulator patterns =====

#[test]
fn test_s12_b54_running_sum() {
    let code = r#"
def running_sum(items: list) -> list:
    result = []
    total = 0
    for item in items:
        total += item
        result.append(total)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn running_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b54_product() {
    let code = r#"
def product(items: list) -> int:
    result = 1
    for item in items:
        result *= item
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn product"), "Got: {}", result);
}

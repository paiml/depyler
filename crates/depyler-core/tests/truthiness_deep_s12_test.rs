//! Session 12 Batch 25: Truthiness coercion and conditional conversion cold paths
//!
//! Targets direct_rules_convert.rs cold paths:
//! - Truthiness coercion: collection truthiness, numeric truthiness, option truthiness
//! - Not operator on collections: `not self.items` → `self.items.is_empty()`
//! - Comparison operators with type coercion: int/float comparisons
//! - Power operator edge cases: negative exponents, float bases
//! - String concatenation via + operator
//! - Date/datetime constructors
//! - Hasher methods: hexdigest, update

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

// ===== Truthiness: collection empty check =====

#[test]
fn test_s12_truthiness_list_if() {
    let code = r#"
class Container:
    def __init__(self, items: list):
        self.items = items

    def has_items(self) -> bool:
        if self.items:
            return True
        return False
"#;
    let result = transpile(code);
    assert!(result.contains("has_items"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_not_list() {
    let code = r#"
class Container:
    def __init__(self, items: list):
        self.items = items

    def is_empty(self) -> bool:
        if not self.items:
            return True
        return False
"#;
    let result = transpile(code);
    assert!(result.contains("is_empty"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_dict_if() {
    let code = r#"
class Cache:
    def __init__(self):
        self.data = {}

    def has_data(self) -> bool:
        if self.data:
            return True
        return False
"#;
    let result = transpile(code);
    assert!(result.contains("has_data"), "Got: {}", result);
}

#[test]
fn test_s12_truthiness_string_if() {
    let code = r#"
class TextBox:
    def __init__(self, text: str):
        self.text = text

    def has_text(self) -> bool:
        if self.text:
            return True
        return False
"#;
    let result = transpile(code);
    assert!(result.contains("has_text"), "Got: {}", result);
}

// ===== Truthiness: numeric zero check =====

#[test]
fn test_s12_truthiness_numeric_while() {
    let code = r#"
class Counter:
    def __init__(self, n: int):
        self.n = n

    def count_down(self) -> int:
        total = 0
        while self.n:
            total += self.n
            self.n -= 1
        return total
"#;
    let result = transpile(code);
    assert!(result.contains("count_down"), "Got: {}", result);
}

// ===== Power operator edge cases =====

#[test]
fn test_s12_power_float_base() {
    let code = r#"
def sqrt_approx(x: float) -> float:
    return x ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn sqrt_approx"), "Got: {}", result);
}

#[test]
fn test_s12_power_negative_exponent() {
    let code = r#"
def inverse(x: int) -> float:
    return x ** -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn inverse"), "Got: {}", result);
}

#[test]
fn test_s12_power_variable_exponent() {
    let code = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_power_int_literal() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube"), "Got: {}", result);
}

// ===== Comparison operators with type coercion =====

#[test]
fn test_s12_float_int_comparison() {
    let code = r#"
def is_above_threshold(value: float, threshold: int) -> bool:
    return value > threshold
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_above_threshold"), "Got: {}", result);
}

#[test]
fn test_s12_int_float_comparison() {
    let code = r#"
def check_bounds(count: int, limit: float) -> bool:
    return count < limit
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_bounds"), "Got: {}", result);
}

#[test]
fn test_s12_float_literal_comparison() {
    let code = r#"
def is_positive(x: float) -> bool:
    return x > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_positive"), "Got: {}", result);
}

#[test]
fn test_s12_negative_int_float_comparison() {
    let code = r#"
def is_below_zero(x: float) -> bool:
    return x < -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_below_zero"), "Got: {}", result);
}

// ===== String concatenation =====

#[test]
fn test_s12_string_concat_operator() {
    let code = r#"
def full_name(first: str, last: str) -> str:
    return first + " " + last
"#;
    let result = transpile(code);
    assert!(result.contains("fn full_name"), "Got: {}", result);
}

#[test]
fn test_s12_string_concat_in_method() {
    let code = r#"
class Builder:
    def __init__(self):
        self.parts = ""

    def add(self, part: str):
        self.parts = self.parts + part

    def build(self) -> str:
        return self.parts
"#;
    let result = transpile(code);
    assert!(result.contains("Builder"), "Got: {}", result);
}

// ===== Date/datetime constructors =====

#[test]
fn test_s12_date_constructor() {
    let code = r#"
class DateHelper:
    def create(self, year: int, month: int, day: int):
        return date(year, month, day)
"#;
    let result = transpile(code);
    assert!(result.contains("DateHelper"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_constructor() {
    let code = r#"
class TimestampHelper:
    def create(self, year: int, month: int, day: int, hour: int, minute: int):
        return datetime(year, month, day, hour, minute)
"#;
    let result = transpile(code);
    assert!(result.contains("TimestampHelper"), "Got: {}", result);
}

// ===== Hasher methods =====

#[test]
fn test_s12_hasher_hexdigest() {
    let code = r#"
class Hasher:
    def __init__(self, data: str):
        self.h = hashlib.sha256(data)

    def get_hash(self) -> str:
        return self.h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("Hasher"), "Got: {}", result);
}

#[test]
fn test_s12_hasher_update() {
    let code = r#"
class IncrementalHasher:
    def __init__(self):
        self.h = hashlib.sha256()

    def feed(self, data: str):
        self.h.update(data)
"#;
    let result = transpile(code);
    assert!(result.contains("IncrementalHasher"), "Got: {}", result);
}

// ===== Complex patterns combining truthiness and operators =====

#[test]
fn test_s12_heap_like_class() {
    let code = r#"
class MinHeap:
    def __init__(self):
        self.heap = []

    def push(self, val: int):
        self.heap.append(val)
        self.heap.sort()

    def pop(self) -> int:
        if not self.heap:
            raise IndexError("empty heap")
        return self.heap.pop(0)

    def peek(self) -> int:
        if not self.heap:
            raise IndexError("empty heap")
        return self.heap[0]

    def size(self) -> int:
        return len(self.heap)
"#;
    let result = transpile(code);
    assert!(result.contains("MinHeap"), "Got: {}", result);
    assert!(result.contains("fn push"), "Got: {}", result);
    assert!(result.contains("fn pop"), "Got: {}", result);
}

#[test]
fn test_s12_linked_list_like() {
    let code = r#"
class Node:
    def __init__(self, value: int):
        self.value = value
        self.next = None

    def has_next(self) -> bool:
        return self.next is not None

    def set_next(self, node):
        self.next = node
"#;
    let result = transpile(code);
    assert!(result.contains("Node"), "Got: {}", result);
}

#[test]
fn test_s12_optional_field_truthiness() {
    let code = r#"
class Optional:
    def __init__(self):
        self.value = None

    def set(self, v: int):
        self.value = v

    def get(self) -> int:
        if self.value:
            return self.value
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("Optional"), "Got: {}", result);
}

// ===== Cast expression safety =====

#[test]
fn test_s12_cast_in_comparison() {
    let code = r#"
def fits_in_byte(n: int) -> bool:
    return n >= 0 and n <= 255
"#;
    let result = transpile(code);
    assert!(result.contains("fn fits_in_byte"), "Got: {}", result);
}

// ===== Complex method with multiple operator types =====

#[test]
fn test_s12_statistical_class() {
    let code = r#"
class Stats:
    def __init__(self, data: list):
        self.data = data

    def mean(self) -> float:
        if not self.data:
            return 0.0
        return sum(self.data) / len(self.data)

    def variance(self) -> float:
        if len(self.data) < 2:
            return 0.0
        avg = self.mean()
        total = 0.0
        for x in self.data:
            total += (x - avg) ** 2
        return total / (len(self.data) - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("Stats"), "Got: {}", result);
    assert!(result.contains("mean"), "Got: {}", result);
    assert!(result.contains("variance"), "Got: {}", result);
}

//! Session 12 Batch 64: Type system and inference cold paths
//!
//! Targets type system codegen paths:
//! - Complex type annotations
//! - Generic type patterns
//! - Union types
//! - Nested generic types
//! - Type inference from complex expressions

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

// ===== Complex type annotations =====

#[test]
fn test_s12_b64_list_of_str() {
    let code = r#"
from typing import List

def upper_all(items: List[str]) -> List[str]:
    return [s.upper() for s in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_all"), "Got: {}", result);
}

#[test]
fn test_s12_b64_dict_str_int() {
    let code = r#"
from typing import Dict

def sum_values(d: Dict[str, int]) -> int:
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_values"), "Got: {}", result);
}

#[test]
fn test_s12_b64_optional_str() {
    let code = r#"
from typing import Optional

def first_word(text: str) -> Optional[str]:
    words = text.split()
    if words:
        return words[0]
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_word"), "Got: {}", result);
}

#[test]
fn test_s12_b64_tuple_annotation() {
    let code = r#"
from typing import Tuple

def min_max(items: list) -> Tuple[int, int]:
    return (min(items), max(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max"), "Got: {}", result);
}

#[test]
fn test_s12_b64_set_annotation() {
    let code = r#"
from typing import Set

def unique_chars(text: str) -> Set[str]:
    return set(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_chars"), "Got: {}", result);
}

// ===== Nested generic types =====

#[test]
fn test_s12_b64_list_of_list() {
    let code = r#"
from typing import List

def flatten(matrix: List[List[int]]) -> List[int]:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b64_dict_of_lists() {
    let code = r#"
from typing import Dict, List

def group_lengths(words: List[str]) -> Dict[int, List[str]]:
    groups = {}
    for word in words:
        n = len(word)
        if n not in groups:
            groups[n] = []
        groups[n].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_lengths"), "Got: {}", result);
}

// ===== Type inference from expressions =====

#[test]
fn test_s12_b64_infer_from_arithmetic() {
    let code = r#"
def compute(a: int, b: int, c: float) -> float:
    result = a * b + c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Got: {}", result);
}

#[test]
fn test_s12_b64_infer_from_comparison() {
    let code = r#"
def check(x: int, y: int) -> bool:
    result = x > y
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"), "Got: {}", result);
}

#[test]
fn test_s12_b64_infer_from_string_op() {
    let code = r#"
def combine(a: str, b: str) -> str:
    result = a + " " + b
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Got: {}", result);
}

#[test]
fn test_s12_b64_infer_from_list_op() {
    let code = r#"
def doubled(items: list) -> list:
    result = items + items
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn doubled"), "Got: {}", result);
}

// ===== Complex type patterns in classes =====

#[test]
fn test_s12_b64_class_typed_fields() {
    let code = r#"
from typing import List, Dict

class Database:
    def __init__(self):
        self.tables: Dict[str, List[Dict[str, str]]] = {}

    def add_table(self, name: str):
        self.tables[name] = []

    def insert(self, table: str, record: Dict[str, str]):
        if table in self.tables:
            self.tables[table].append(record)

    def query(self, table: str) -> List[Dict[str, str]]:
        return self.tables.get(table, [])
"#;
    let result = transpile(code);
    assert!(result.contains("Database"), "Got: {}", result);
}

// ===== Callable type patterns =====

#[test]
fn test_s12_b64_higher_order_func() {
    let code = r#"
def apply_twice(func, x: int) -> int:
    return func(func(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_twice"), "Got: {}", result);
}

#[test]
fn test_s12_b64_map_func() {
    let code = r#"
def transform(items: list, func) -> list:
    result = []
    for item in items:
        result.append(func(item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transform"), "Got: {}", result);
}

#[test]
fn test_s12_b64_predicate_func() {
    let code = r#"
def filter_by(items: list, predicate) -> list:
    result = []
    for item in items:
        if predicate(item):
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_by"), "Got: {}", result);
}

// ===== Mixed numeric types =====

#[test]
fn test_s12_b64_int_float_mix() {
    let code = r#"
def weighted_average(values: list, weights: list) -> float:
    total = 0.0
    weight_sum = 0.0
    for i in range(len(values)):
        total += values[i] * weights[i]
        weight_sum += weights[i]
    if weight_sum == 0.0:
        return 0.0
    return total / weight_sum
"#;
    let result = transpile(code);
    assert!(result.contains("fn weighted_average"), "Got: {}", result);
}

#[test]
fn test_s12_b64_bool_to_int() {
    let code = r#"
def count_true(items: list) -> int:
    count = 0
    for item in items:
        if item:
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_true"), "Got: {}", result);
}

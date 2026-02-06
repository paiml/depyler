//! Session 12 Batch 84: Type inference deep cold paths
//!
//! Targets type_mapper.rs and type inference cold paths for
//! complex type annotations, generic types, optional types,
//! and inference from expressions.

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

// ===== List type annotations =====

#[test]
fn test_s12_b84_list_of_str() {
    let code = r#"
from typing import List

def join_all(items: List[str]) -> str:
    return " ".join(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_all"), "Got: {}", result);
}

#[test]
fn test_s12_b84_list_of_int() {
    let code = r#"
from typing import List

def sum_all(items: List[int]) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_all"), "Got: {}", result);
}

#[test]
fn test_s12_b84_list_of_float() {
    let code = r#"
from typing import List

def average(values: List[float]) -> float:
    return sum(values) / len(values)
"#;
    let result = transpile(code);
    assert!(result.contains("fn average"), "Got: {}", result);
}

// ===== Dict type annotations =====

#[test]
fn test_s12_b84_dict_str_int() {
    let code = r#"
from typing import Dict

def total_counts(counts: Dict[str, int]) -> int:
    return sum(counts.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn total_counts"), "Got: {}", result);
}

#[test]
fn test_s12_b84_dict_str_str() {
    let code = r#"
from typing import Dict

def merge(a: Dict[str, str], b: Dict[str, str]) -> Dict[str, str]:
    result = dict(a)
    result.update(b)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

// ===== Optional type annotations =====

#[test]
fn test_s12_b84_optional_str() {
    let code = r#"
from typing import Optional

def find_name(items: list, idx: int) -> Optional[str]:
    if idx < len(items):
        return items[idx]
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_name"), "Got: {}", result);
}

#[test]
fn test_s12_b84_optional_int() {
    let code = r#"
from typing import Optional

def safe_divide(a: int, b: int) -> Optional[int]:
    if b == 0:
        return None
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

// ===== Tuple type annotations =====

#[test]
fn test_s12_b84_tuple_return() {
    let code = r#"
from typing import Tuple

def min_max(items: list) -> Tuple[int, int]:
    return (min(items), max(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max"), "Got: {}", result);
}

#[test]
fn test_s12_b84_tuple_three() {
    let code = r#"
from typing import Tuple

def stats(values: list) -> Tuple[float, float, float]:
    avg = sum(values) / len(values)
    mn = min(values)
    mx = max(values)
    return (avg, mn, mx)
"#;
    let result = transpile(code);
    assert!(result.contains("fn stats"), "Got: {}", result);
}

// ===== Set type annotations =====

#[test]
fn test_s12_b84_set_str() {
    let code = r#"
from typing import Set

def unique_words(text: str) -> Set[str]:
    return set(text.split())
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_words"), "Got: {}", result);
}

// ===== Nested generics =====

#[test]
fn test_s12_b84_list_of_list() {
    let code = r#"
from typing import List

def flatten(matrix: List[List[int]]) -> List[int]:
    result = []
    for row in matrix:
        result.extend(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b84_dict_of_list() {
    let code = r#"
from typing import Dict, List

def group_lengths(words: List[str]) -> Dict[int, List[str]]:
    groups = {}
    for word in words:
        length = len(word)
        if length not in groups:
            groups[length] = []
        groups[length].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_lengths"), "Got: {}", result);
}

// ===== Callable type patterns =====

#[test]
fn test_s12_b84_higher_order_func() {
    let code = r#"
def apply_twice(func, x: int) -> int:
    return func(func(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_twice"), "Got: {}", result);
}

#[test]
fn test_s12_b84_predicate_func() {
    let code = r#"
def filter_with(items: list, predicate) -> list:
    return [x for x in items if predicate(x)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_with"), "Got: {}", result);
}

#[test]
fn test_s12_b84_map_func() {
    let code = r#"
def map_with(items: list, transform) -> list:
    return [transform(x) for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn map_with"), "Got: {}", result);
}

// ===== Type inference from expressions =====

#[test]
fn test_s12_b84_infer_from_comparison() {
    let code = r#"
def check(a: int, b: int):
    is_equal = a == b
    is_greater = a > b
    is_less = a < b
    return is_equal or is_greater or is_less
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"), "Got: {}", result);
}

#[test]
fn test_s12_b84_infer_from_string_op() {
    let code = r#"
def concat(a: str, b: str):
    combined = a + " " + b
    upper = combined.upper()
    return upper
"#;
    let result = transpile(code);
    assert!(result.contains("fn concat"), "Got: {}", result);
}

#[test]
fn test_s12_b84_infer_from_list_op() {
    let code = r#"
def build(n: int):
    items = []
    for i in range(n):
        items.append(i)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn build"), "Got: {}", result);
}

#[test]
fn test_s12_b84_infer_dict_from_literal() {
    let code = r##"
def make_config(host: str, port: int):
    config = {"host": host, "port": port}
    return config
"##;
    let result = transpile(code);
    assert!(result.contains("fn make_config"), "Got: {}", result);
}

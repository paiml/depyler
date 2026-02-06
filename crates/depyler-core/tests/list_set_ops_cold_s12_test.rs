//! Session 12 Batch 56: List and set operation cold paths
//!
//! Targets list and set codegen cold paths:
//! - List methods (sort, copy, extend, insert, remove, index, count, pop)
//! - Set operations (add, discard, union, intersection, difference)
//! - Tuple operations and unpacking
//! - Complex collection algorithms

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

// ===== List methods =====

#[test]
fn test_s12_b56_list_append() {
    let code = r#"
def collect(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i * i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn collect"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_extend() {
    let code = r#"
def concat(a: list, b: list) -> list:
    result = []
    result.extend(a)
    result.extend(b)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn concat"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_insert() {
    let code = r#"
def insert_sorted(items: list, value: int) -> list:
    for i in range(len(items)):
        if value < items[i]:
            items.insert(i, value)
            return items
    items.append(value)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_sorted"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_remove() {
    let code = r#"
def remove_first(items: list, value: int) -> list:
    items.remove(value)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_first"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_pop() {
    let code = r#"
def pop_last(items: list) -> int:
    return items.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_last"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_pop_idx() {
    let code = r#"
def pop_at(items: list, idx: int) -> int:
    return items.pop(idx)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_at"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_index() {
    let code = r#"
def find_index(items: list, value: int) -> int:
    return items.index(value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_index"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_count() {
    let code = r#"
def count_occurrences(items: list, value: int) -> int:
    return items.count(value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_occurrences"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_sort() {
    let code = r#"
def sort_items(items: list) -> list:
    items.sort()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_items"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_reverse() {
    let code = r#"
def reverse_items(items: list) -> list:
    items.reverse()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_items"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_copy() {
    let code = r#"
def clone_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_list"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_clear() {
    let code = r#"
def clear_list(items: list):
    items.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_list"), "Got: {}", result);
}

// ===== List built-in operations =====

#[test]
fn test_s12_b56_list_len() {
    let code = r#"
def length(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn length"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_sum() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_min() {
    let code = r#"
def minimum(items: list) -> int:
    return min(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn minimum"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_max() {
    let code = r#"
def maximum(items: list) -> int:
    return max(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn maximum"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_sorted() {
    let code = r#"
def get_sorted(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_sorted"), "Got: {}", result);
}

#[test]
fn test_s12_b56_list_in() {
    let code = r#"
def contains(items: list, value: int) -> bool:
    return value in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains"), "Got: {}", result);
}

// ===== Set operations =====

#[test]
fn test_s12_b56_set_add() {
    let code = r#"
def collect_unique(items: list) -> set:
    result = set()
    for item in items:
        result.add(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn collect_unique"), "Got: {}", result);
}

#[test]
fn test_s12_b56_set_discard() {
    let code = r#"
def remove_item(s: set, item: int) -> set:
    s.discard(item)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_item"), "Got: {}", result);
}

#[test]
fn test_s12_b56_set_union() {
    let code = r#"
def combine_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine_sets"), "Got: {}", result);
}

#[test]
fn test_s12_b56_set_intersection() {
    let code = r#"
def common_elements(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn common_elements"), "Got: {}", result);
}

#[test]
fn test_s12_b56_set_difference() {
    let code = r#"
def only_in_first(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_first"), "Got: {}", result);
}

#[test]
fn test_s12_b56_set_in() {
    let code = r#"
def is_member(s: set, value: int) -> bool:
    return value in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_member"), "Got: {}", result);
}

#[test]
fn test_s12_b56_set_len() {
    let code = r#"
def cardinality(s: set) -> int:
    return len(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn cardinality"), "Got: {}", result);
}

// ===== Tuple operations =====

#[test]
fn test_s12_b56_tuple_return() {
    let code = r#"
def divmod_custom(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divmod_custom"), "Got: {}", result);
}

#[test]
fn test_s12_b56_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

#[test]
fn test_s12_b56_tuple_index() {
    let code = r#"
def first(pair: tuple) -> int:
    return pair[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first"), "Got: {}", result);
}

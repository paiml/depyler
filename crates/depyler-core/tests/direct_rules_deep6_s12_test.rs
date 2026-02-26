//! Session 12 Batch 58: Deep direct_rules_convert.rs cold paths
//!
//! Targets the remaining 42% uncovered in direct_rules_convert.rs:
//! - Dynamic call patterns (callable variables)
//! - Complex type conversion patterns
//! - Vector slicing with various index combos
//! - Exception handling codegen paths
//! - Date/time patterns
//! - Complex builtin function calls

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

// ===== Dynamic call patterns =====

#[test]
fn test_s12_b58_call_function_ref() {
    let code = r#"
def apply(func, x: int) -> int:
    return func(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply"), "Got: {}", result);
}

#[test]
fn test_s12_b58_call_with_kwargs() {
    let code = r#"
def create_config(name: str, debug: bool = False, port: int = 8080) -> dict:
    return {"name": name, "debug": debug, "port": port}
"#;
    let result = transpile(code);
    assert!(result.contains("fn create_config"), "Got: {}", result);
}

// ===== Type conversion patterns =====

#[test]
fn test_s12_b58_list_to_set() {
    let code = r#"
def unique(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique"), "Got: {}", result);
}

#[test]
fn test_s12_b58_set_to_list() {
    let code = r#"
def to_sorted_list(s: set) -> list:
    return sorted(list(s))
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_sorted_list"), "Got: {}", result);
}

#[test]
fn test_s12_b58_dict_to_list() {
    let code = r#"
def dict_items_list(d: dict) -> list:
    return list(d.items())
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_items_list"), "Got: {}", result);
}

#[test]
fn test_s12_b58_str_to_list() {
    let code = r#"
def chars(s: str) -> list:
    return list(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn chars"), "Got: {}", result);
}

#[test]
fn test_s12_b58_range_to_list() {
    let code = r#"
def range_list(n: int) -> list:
    return list(range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn range_list"), "Got: {}", result);
}

// ===== Exception handling codegen =====

#[test]
fn test_s12_b58_try_value_error() {
    let code = r##"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"##;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"), "Got: {}", result);
}

#[test]
fn test_s12_b58_try_key_error() {
    let code = r#"
def safe_get(d: dict, key: str) -> str:
    try:
        return d[key]
    except KeyError:
        return "missing"
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_b58_try_index_error() {
    let code = r#"
def safe_index(items: list, idx: int) -> int:
    try:
        return items[idx]
    except IndexError:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_index"), "Got: {}", result);
}

// ===== Builtin function patterns =====

#[test]
fn test_s12_b58_builtin_len() {
    let code = r#"
def sizes(a: list, b: str, c: dict) -> list:
    return [len(a), len(b), len(c)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn sizes"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_range_step() {
    let code = r#"
def odds(n: int) -> list:
    result = []
    for i in range(1, n, 2):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn odds"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_enumerate() {
    let code = r#"
def indexed_items(items: list) -> list:
    result = []
    for idx, val in enumerate(items):
        result.append((idx, val))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_items"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_zip() {
    let code = r#"
def zip_lists(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append((x, y))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn zip_lists"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_sorted_key() {
    let code = r#"
def sort_by_length(words: list) -> list:
    return sorted(words, key=len)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_length"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_sorted_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_desc"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_any() {
    let code = r#"
def has_match(items: list, target: int) -> bool:
    return any(x == target for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_match"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_map_list() {
    let code = r#"
def squares(items: list) -> list:
    return list(map(lambda x: x ** 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Got: {}", result);
}

#[test]
fn test_s12_b58_builtin_filter_list() {
    let code = r#"
def positives(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn positives"), "Got: {}", result);
}

// ===== Complex patterns mixing features =====

#[test]
fn test_s12_b58_matrix_transpose() {
    let code = r#"
def transpose(matrix: list) -> list:
    if not matrix:
        return []
    rows = len(matrix)
    cols = len(matrix[0])
    result = []
    for j in range(cols):
        row = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transpose"), "Got: {}", result);
}

#[test]
fn test_s12_b58_bubble_sort() {
    let code = r#"
def bubble_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Got: {}", result);
}

#[test]
fn test_s12_b58_selection_sort() {
    let code = r#"
def selection_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        min_idx = i
        for j in range(i + 1, n):
            if items[j] < items[min_idx]:
                min_idx = j
        items[i], items[min_idx] = items[min_idx], items[i]
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn selection_sort"), "Got: {}", result);
}

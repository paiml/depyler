//! Session 12 Batch 72: Subscript and slice cold paths
//!
//! Targets expr_gen.rs and direct_rules_convert.rs cold paths for
//! complex subscript access, negative indexing, and slice patterns.

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

#[test]
fn test_s12_b72_negative_index() {
    let code = r#"
def last_element(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_element"), "Got: {}", result);
}

#[test]
fn test_s12_b72_negative_index_second() {
    let code = r#"
def second_last(items: list) -> int:
    return items[-2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn second_last"), "Got: {}", result);
}

#[test]
fn test_s12_b72_string_negative_index() {
    let code = r#"
def last_char(s: str) -> str:
    return s[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_char"), "Got: {}", result);
}

#[test]
fn test_s12_b72_slice_from_start() {
    let code = r#"
def first_n(items: list, n: int) -> list:
    return items[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_n"), "Got: {}", result);
}

#[test]
fn test_s12_b72_slice_to_end() {
    let code = r#"
def skip_n(items: list, n: int) -> list:
    return items[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_n"), "Got: {}", result);
}

#[test]
fn test_s12_b72_slice_both_bounds() {
    let code = r#"
def sublist(items: list, start: int, end: int) -> list:
    return items[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn sublist"), "Got: {}", result);
}

#[test]
fn test_s12_b72_slice_with_step() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"), "Got: {}", result);
}

#[test]
fn test_s12_b72_reverse_slice() {
    let code = r#"
def reversed_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reversed_list"), "Got: {}", result);
}

#[test]
fn test_s12_b72_string_slice() {
    let code = r#"
def substring(s: str, start: int, end: int) -> str:
    return s[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn substring"), "Got: {}", result);
}

#[test]
fn test_s12_b72_string_reverse() {
    let code = r#"
def reverse_string(s: str) -> str:
    return s[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_string"), "Got: {}", result);
}

#[test]
fn test_s12_b72_nested_subscript() {
    let code = r#"
def get_cell(matrix: list, row: int, col: int) -> int:
    return matrix[row][col]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_cell"), "Got: {}", result);
}

#[test]
fn test_s12_b72_dict_nested_access() {
    let code = r#"
def get_nested(data: dict, key1: str, key2: str) -> str:
    return data[key1][key2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_nested"), "Got: {}", result);
}

#[test]
fn test_s12_b72_variable_index() {
    let code = r#"
def at_index(items: list, idx: int) -> int:
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn at_index"), "Got: {}", result);
}

#[test]
fn test_s12_b72_computed_index() {
    let code = r#"
def middle(items: list) -> int:
    return items[len(items) // 2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn middle"), "Got: {}", result);
}

#[test]
fn test_s12_b72_negative_slice() {
    let code = r#"
def last_three(items: list) -> list:
    return items[-3:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_three"), "Got: {}", result);
}

#[test]
fn test_s12_b72_slice_negative_end() {
    let code = r#"
def trim_ends(items: list) -> list:
    return items[1:-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim_ends"), "Got: {}", result);
}

#[test]
fn test_s12_b72_string_step_slice() {
    let code = r#"
def every_other_char(s: str) -> str:
    return s[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other_char"), "Got: {}", result);
}

#[test]
fn test_s12_b72_list_of_dict_access() {
    let code = r#"
def get_names(records: list) -> list:
    result = []
    for record in records:
        result.append(record["name"])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_names"), "Got: {}", result);
}

#[test]
fn test_s12_b72_tuple_index() {
    let code = r#"
def first_of_tuple(t: tuple) -> int:
    return t[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_of_tuple"), "Got: {}", result);
}

#[test]
fn test_s12_b72_slice_copy() {
    let code = r#"
def copy_list(items: list) -> list:
    return items[:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_list"), "Got: {}", result);
}

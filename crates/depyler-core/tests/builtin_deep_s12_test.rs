//! Session 12 Batch 91: Built-in functions deep cold paths
//!
//! Targets codegen for built-in functions: abs, min, max, sum, len,
//! sorted, reversed, enumerate, zip, range, map, filter, any, all,
//! print, input, type, id, hash, ord, chr.

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
fn test_s12_b91_abs_int() {
    let code = r#"
def absolute_int(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute_int"), "Got: {}", result);
}

#[test]
fn test_s12_b91_abs_float() {
    let code = r#"
def absolute_float(x: float) -> float:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute_float"), "Got: {}", result);
}

#[test]
fn test_s12_b91_min_two() {
    let code = r#"
def smaller(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn smaller"), "Got: {}", result);
}

#[test]
fn test_s12_b91_max_two() {
    let code = r#"
def larger(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn larger"), "Got: {}", result);
}

#[test]
fn test_s12_b91_min_list() {
    let code = r#"
def minimum(items: list) -> int:
    return min(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn minimum"), "Got: {}", result);
}

#[test]
fn test_s12_b91_max_list() {
    let code = r#"
def maximum(items: list) -> int:
    return max(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn maximum"), "Got: {}", result);
}

#[test]
fn test_s12_b91_sum_list() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

#[test]
fn test_s12_b91_len_list() {
    let code = r#"
def count(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count"), "Got: {}", result);
}

#[test]
fn test_s12_b91_len_str() {
    let code = r#"
def str_length(s: str) -> int:
    return len(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn str_length"), "Got: {}", result);
}

#[test]
fn test_s12_b91_len_dict() {
    let code = r#"
def dict_size(d: dict) -> int:
    return len(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_size"), "Got: {}", result);
}

#[test]
fn test_s12_b91_print_basic() {
    let code = r#"
def show(message: str):
    print(message)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show"), "Got: {}", result);
}

#[test]
fn test_s12_b91_print_multiple() {
    let code = r#"
def show_pair(a: str, b: int):
    print(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_pair"), "Got: {}", result);
}

#[test]
fn test_s12_b91_ord_chr() {
    let code = r#"
def shift_char(c: str, n: int) -> str:
    return chr(ord(c) + n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_char"), "Got: {}", result);
}

#[test]
fn test_s12_b91_round_builtin() {
    let code = r#"
def round_to(x: float, digits: int) -> float:
    return round(x, digits)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_to"), "Got: {}", result);
}

#[test]
fn test_s12_b91_sorted_with_key() {
    let code = r#"
def sort_by_abs(items: list) -> list:
    return sorted(items, key=abs)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_abs"), "Got: {}", result);
}

#[test]
fn test_s12_b91_any_all_combined() {
    let code = r#"
def has_positive_and_all_finite(items: list) -> bool:
    return any(x > 0 for x in items) and all(x < 1000 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_positive_and_all_finite"), "Got: {}", result);
}

#[test]
fn test_s12_b91_map_filter_chain() {
    let code = r#"
def positive_squares(items: list) -> list:
    positives = list(filter(lambda x: x > 0, items))
    return list(map(lambda x: x * x, positives))
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_squares"), "Got: {}", result);
}

#[test]
fn test_s12_b91_enumerate_zip_combo() {
    let code = r#"
def indexed_pairs(a: list, b: list) -> list:
    result = []
    for i, (x, y) in enumerate(zip(a, b)):
        result.append((i, x, y))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_pairs"), "Got: {}", result);
}

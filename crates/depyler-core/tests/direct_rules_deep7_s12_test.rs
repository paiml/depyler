//! Session 12 Batch 76: Direct rules convert deep cold paths - patterns 7
//!
//! Targets direct_rules_convert.rs for del statements, global/nonlocal,
//! complex assignment patterns, and type conversion edge cases.

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
fn test_s12_b76_del_dict_key() {
    let code = r#"
def remove_key(d: dict, key: str):
    del d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

#[test]
fn test_s12_b76_del_list_index() {
    let code = r#"
def remove_at(items: list, idx: int):
    del items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_at"), "Got: {}", result);
}

#[test]
fn test_s12_b76_isinstance_str() {
    let code = r#"
def is_text(obj) -> bool:
    return isinstance(obj, str)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_text"), "Got: {}", result);
}

#[test]
fn test_s12_b76_isinstance_int() {
    let code = r#"
def is_number(obj) -> bool:
    return isinstance(obj, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_number"), "Got: {}", result);
}

#[test]
fn test_s12_b76_isinstance_list() {
    let code = r#"
def is_list(obj) -> bool:
    return isinstance(obj, list)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_list"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_int() {
    let code = r#"
def to_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_int"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_float() {
    let code = r#"
def to_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_float"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_str() {
    let code = r#"
def to_str(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_str"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_bool() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_list() {
    let code = r#"
def to_list(s: set) -> list:
    return list(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_list"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_set() {
    let code = r#"
def to_set(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_set"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_dict() {
    let code = r#"
def pairs_to_dict(pairs: list) -> dict:
    return dict(pairs)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs_to_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b76_type_call_tuple() {
    let code = r#"
def to_tuple(items: list) -> tuple:
    return tuple(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_tuple"), "Got: {}", result);
}

#[test]
fn test_s12_b76_range_one_arg() {
    let code = r#"
def count_to(n: int) -> list:
    return list(range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_to"), "Got: {}", result);
}

#[test]
fn test_s12_b76_range_two_args() {
    let code = r#"
def count_from_to(start: int, stop: int) -> list:
    return list(range(start, stop))
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_from_to"), "Got: {}", result);
}

#[test]
fn test_s12_b76_range_three_args() {
    let code = r#"
def evens_in_range(start: int, stop: int) -> list:
    return list(range(start, stop, 2))
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens_in_range"), "Got: {}", result);
}

#[test]
fn test_s12_b76_sorted_key() {
    let code = r#"
def sort_by_second(pairs: list) -> list:
    return sorted(pairs, key=lambda x: x[1])
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_second"), "Got: {}", result);
}

#[test]
fn test_s12_b76_sorted_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_desc"), "Got: {}", result);
}

#[test]
fn test_s12_b76_map_with_func() {
    let code = r#"
def apply_abs(items: list) -> list:
    return list(map(abs, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_abs"), "Got: {}", result);
}

#[test]
fn test_s12_b76_filter_with_func() {
    let code = r#"
def keep_truthy(items: list) -> list:
    return list(filter(bool, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn keep_truthy"), "Got: {}", result);
}

#[test]
fn test_s12_b76_zip_three() {
    let code = r#"
def zip_three(a: list, b: list, c: list) -> list:
    return list(zip(a, b, c))
"#;
    let result = transpile(code);
    assert!(result.contains("fn zip_three"), "Got: {}", result);
}

#[test]
fn test_s12_b76_enumerate_start() {
    let code = r#"
def numbered(items: list) -> list:
    return list(enumerate(items, 1))
"#;
    let result = transpile(code);
    assert!(result.contains("fn numbered"), "Got: {}", result);
}

#[test]
fn test_s12_b76_reversed_builtin() {
    let code = r#"
def rev_list(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn rev_list"), "Got: {}", result);
}

#[test]
fn test_s12_b76_min_max_key() {
    let code = r#"
def shortest(words: list) -> str:
    return min(words, key=len)
"#;
    let result = transpile(code);
    assert!(result.contains("fn shortest"), "Got: {}", result);
}

//! Session 12 Batch 90: Type conversion and casting edge cases
//!
//! Targets direct_rules_convert.rs and expr_gen.rs cold paths for
//! type conversion patterns: int(), float(), str(), bool(), list(),
//! set(), dict(), tuple() in various contexts.

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
fn test_s12_b90_int_from_float() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn truncate"), "Got: {}", result);
}

#[test]
fn test_s12_b90_float_from_int() {
    let code = r#"
def to_float(n: int) -> float:
    return float(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_float"), "Got: {}", result);
}

#[test]
fn test_s12_b90_str_from_int() {
    let code = r#"
def int_to_string(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_to_string"), "Got: {}", result);
}

#[test]
fn test_s12_b90_str_from_float() {
    let code = r#"
def float_to_string(x: float) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn float_to_string"), "Got: {}", result);
}

#[test]
fn test_s12_b90_str_from_bool() {
    let code = r#"
def bool_to_string(b: bool) -> str:
    return str(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn bool_to_string"), "Got: {}", result);
}

#[test]
fn test_s12_b90_list_from_range() {
    let code = r#"
def range_to_list(n: int) -> list:
    return list(range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn range_to_list"), "Got: {}", result);
}

#[test]
fn test_s12_b90_list_from_set() {
    let code = r#"
def set_to_sorted_list(s: set) -> list:
    return sorted(list(s))
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_to_sorted_list"), "Got: {}", result);
}

#[test]
fn test_s12_b90_set_from_list() {
    let code = r#"
def deduplicate(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn deduplicate"), "Got: {}", result);
}

#[test]
fn test_s12_b90_dict_from_pairs() {
    let code = r#"
def make_lookup(keys: list, values: list) -> dict:
    return dict(zip(keys, values))
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_lookup"), "Got: {}", result);
}

#[test]
fn test_s12_b90_tuple_from_list() {
    let code = r#"
def freeze(items: list) -> tuple:
    return tuple(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn freeze"), "Got: {}", result);
}

#[test]
fn test_s12_b90_bool_from_value() {
    let code = r#"
def is_truthy(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_truthy"), "Got: {}", result);
}

#[test]
fn test_s12_b90_int_from_str_in_loop() {
    let code = r#"
def sum_strings(items: list) -> int:
    total = 0
    for item in items:
        total += int(item)
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_strings"), "Got: {}", result);
}

#[test]
fn test_s12_b90_chained_conversions() {
    let code = r#"
def round_trip(x: float) -> float:
    return float(str(int(x)))
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_trip"), "Got: {}", result);
}

#[test]
fn test_s12_b90_list_from_string() {
    let code = r#"
def chars(s: str) -> list:
    return list(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn chars"), "Got: {}", result);
}

#[test]
fn test_s12_b90_set_from_string() {
    let code = r#"
def unique_chars(s: str) -> set:
    return set(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_chars"), "Got: {}", result);
}

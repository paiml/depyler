//! Session 12 Batch 97: Array initialization and list construction cold paths
//!
//! Targets array_initialization.rs (73.77% line coverage) and
//! list construction patterns in codegen.

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
fn test_s12_b97_list_repeat() {
    let code = r#"
def zeros(n: int) -> list:
    return [0] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn zeros"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_repeat_float() {
    let code = r#"
def float_zeros(n: int) -> list:
    return [0.0] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn float_zeros"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_repeat_string() {
    let code = r##"
def empty_strings(n: int) -> list:
    return [""] * n
"##;
    let result = transpile(code);
    assert!(result.contains("fn empty_strings"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_repeat_bool() {
    let code = r#"
def false_array(n: int) -> list:
    return [False] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn false_array"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_repeat_none() {
    let code = r#"
def none_array(n: int) -> list:
    return [None] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn none_array"), "Got: {}", result);
}

#[test]
fn test_s12_b97_nested_list_repeat() {
    let code = r#"
def matrix_zeros(rows: int, cols: int) -> list:
    return [[0] * cols for _ in range(rows)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_zeros"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_literal_int() {
    let code = r#"
def get_primes() -> list:
    return [2, 3, 5, 7, 11, 13]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_primes"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_literal_str() {
    let code = r##"
def get_colors() -> list:
    return ["red", "green", "blue"]
"##;
    let result = transpile(code);
    assert!(result.contains("fn get_colors"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_literal_float() {
    let code = r#"
def get_weights() -> list:
    return [0.1, 0.2, 0.3, 0.4]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_weights"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_literal_mixed() {
    let code = r#"
def get_config() -> list:
    return [1, 2, 3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_config"), "Got: {}", result);
}

#[test]
fn test_s12_b97_dict_literal() {
    let code = r##"
def get_defaults() -> dict:
    return {"host": "localhost", "port": 8080, "debug": False}
"##;
    let result = transpile(code);
    assert!(result.contains("fn get_defaults"), "Got: {}", result);
}

#[test]
fn test_s12_b97_set_literal() {
    let code = r##"
def get_vowels() -> set:
    return {"a", "e", "i", "o", "u"}
"##;
    let result = transpile(code);
    assert!(result.contains("fn get_vowels"), "Got: {}", result);
}

#[test]
fn test_s12_b97_tuple_literal() {
    let code = r#"
def get_origin() -> tuple:
    return (0, 0, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_origin"), "Got: {}", result);
}

#[test]
fn test_s12_b97_empty_collections() {
    let code = r#"
def empty_list() -> list:
    return []

def empty_dict() -> dict:
    return {}

def empty_set() -> set:
    return set()

def empty_tuple() -> tuple:
    return ()
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty_list"), "Got: {}", result);
    assert!(result.contains("fn empty_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b97_list_from_comprehension() {
    let code = r#"
def squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Got: {}", result);
}

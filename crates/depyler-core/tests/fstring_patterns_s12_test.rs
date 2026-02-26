//! Session 12 Batch 67: F-string and string formatting cold paths
//!
//! Targets f-string codegen cold paths:
//! - Simple f-strings with variables
//! - F-strings with expressions
//! - F-strings with method calls
//! - F-strings with format specifiers
//! - F-strings with nested expressions
//! - Multi-part f-strings

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
fn test_s12_b67_fstring_simple_var() {
    let code = r##"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"##;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_two_vars() {
    let code = r##"
def pair_str(a: int, b: int) -> str:
    return f"({a}, {b})"
"##;
    let result = transpile(code);
    assert!(result.contains("fn pair_str"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_with_expr() {
    let code = r##"
def describe(n: int) -> str:
    return f"{n} squared is {n * n}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_with_method() {
    let code = r##"
def format_name(name: str) -> str:
    return f"Name: {name.upper()}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_name"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_with_len() {
    let code = r##"
def describe_list(items: list) -> str:
    return f"List has {len(items)} items"
"##;
    let result = transpile(code);
    assert!(result.contains("fn describe_list"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_format_float() {
    let code = r##"
def format_percent(value: float) -> str:
    return f"{value:.1f}%"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_percent"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_format_int_pad() {
    let code = r##"
def format_padded(n: int) -> str:
    return f"{n:05d}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_padded"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_multivar() {
    let code = r##"
def format_point(x: float, y: float, z: float) -> str:
    return f"({x}, {y}, {z})"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_point"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_with_ternary() {
    let code = r##"
def status_msg(ok: bool) -> str:
    return f"Status: {'OK' if ok else 'FAIL'}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn status_msg"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_in_loop() {
    let code = r##"
def numbered_list(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append(f"{i + 1}. {item}")
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn numbered_list"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_nested_quotes() {
    let code = r##"
def quote(text: str) -> str:
    return f'"{text}"'
"##;
    let result = transpile(code);
    assert!(result.contains("fn quote"), "Got: {}", result);
}

#[test]
fn test_s12_b67_str_format_method() {
    let code = r##"
def format_pair(k: str, v: int) -> str:
    return "{}: {}".format(k, v)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_pair"), "Got: {}", result);
}

#[test]
fn test_s12_b67_str_percent_format() {
    let code = r##"
def format_msg(name: str, count: int) -> str:
    return "%s has %d items" % (name, count)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_msg"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_dict_access() {
    let code = r##"
def format_config(config: dict) -> str:
    return f"host={config['host']}, port={config['port']}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_config"), "Got: {}", result);
}

#[test]
fn test_s12_b67_fstring_complex_expr() {
    let code = r##"
def summary(items: list) -> str:
    return f"Count: {len(items)}, Sum: {sum(items)}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn summary"), "Got: {}", result);
}

//! Session 12 Batch 66: Exception handling deep paths
//!
//! Targets try/except/finally/raise codegen paths
//! that are rarely exercised through simple tests.

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
fn test_s12_b66_try_except_return() {
    let code = r#"
def safe_get(d: dict, key: str) -> str:
    try:
        return d[key]
    except KeyError:
        return "default"
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_b66_try_except_variable() {
    let code = r#"
def safe_parse(s: str) -> int:
    result = 0
    try:
        result = int(s)
    except ValueError:
        result = -1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"), "Got: {}", result);
}

#[test]
fn test_s12_b66_try_finally() {
    let code = r#"
def process(x: int) -> int:
    result = 0
    try:
        result = x * 2
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b66_nested_try() {
    let code = r#"
def double_parse(a: str, b: str) -> int:
    try:
        x = int(a)
        try:
            y = int(b)
            return x + y
        except ValueError:
            return x
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_parse"), "Got: {}", result);
}

#[test]
fn test_s12_b66_multi_except_types() {
    let code = r#"
def robust(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
    except OverflowError:
        return -3
"#;
    let result = transpile(code);
    assert!(result.contains("fn robust"), "Got: {}", result);
}

#[test]
fn test_s12_b66_except_with_as() {
    let code = r##"
def describe_error(s: str) -> str:
    try:
        int(s)
        return "ok"
    except ValueError as e:
        return f"error: {e}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn describe_error"), "Got: {}", result);
}

#[test]
fn test_s12_b66_raise_custom() {
    let code = r#"
def check_range(x: int, lo: int, hi: int):
    if x < lo or x > hi:
        raise ValueError("out of range")
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_range"), "Got: {}", result);
}

#[test]
fn test_s12_b66_try_in_loop() {
    let code = r#"
def safe_sum(items: list) -> int:
    total = 0
    for item in items:
        try:
            total += int(item)
        except ValueError:
            pass
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b66_try_with_else() {
    let code = r#"
def parse_or_default(s: str, default: int) -> int:
    try:
        value = int(s)
    except ValueError:
        return default
    else:
        return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_or_default"), "Got: {}", result);
}

#[test]
fn test_s12_b66_try_except_finally_full() {
    let code = r#"
def full_try(x: int) -> int:
    result = 0
    try:
        result = 100 // x
    except ZeroDivisionError:
        result = -1
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn full_try"), "Got: {}", result);
}

#[test]
fn test_s12_b66_raise_if_none() {
    let code = r#"
def require(value, name: str):
    if value is None:
        raise ValueError(name + " is required")
    return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn require"), "Got: {}", result);
}

#[test]
fn test_s12_b66_error_accumulator() {
    let code = r#"
def validate_all(items: list) -> list:
    errors = []
    for i, item in enumerate(items):
        try:
            int(item)
        except ValueError:
            errors.append(i)
    return errors
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_all"), "Got: {}", result);
}

//! Session 12 Batch 86: Error handling and recovery patterns
//!
//! Targets codegen paths for try/except/finally/raise in
//! complex combinations with other features.

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
fn test_s12_b86_retry_pattern() {
    let code = r#"
def retry(func, max_attempts: int):
    for i in range(max_attempts):
        try:
            return func()
        except Exception:
            if i == max_attempts - 1:
                raise
"#;
    let result = transpile(code);
    assert!(result.contains("fn retry"), "Got: {}", result);
}

#[test]
fn test_s12_b86_resource_cleanup() {
    let code = r##"
def process_file(path: str) -> str:
    content = ""
    try:
        with open(path, "r") as f:
            content = f.read()
    except FileNotFoundError:
        content = "not found"
    except PermissionError:
        content = "no permission"
    return content
"##;
    let result = transpile(code);
    assert!(result.contains("fn process_file"), "Got: {}", result);
}

#[test]
fn test_s12_b86_error_chain() {
    let code = r#"
def parse_config(text: str) -> dict:
    try:
        parts = text.split("=")
        key = parts[0].strip()
        value = parts[1].strip()
        return {key: value}
    except IndexError:
        return {}
    except ValueError:
        return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_config"), "Got: {}", result);
}

#[test]
fn test_s12_b86_custom_error() {
    let code = r##"
def validate_age(age: int) -> int:
    if age < 0:
        raise ValueError("Age cannot be negative")
    if age > 150:
        raise ValueError("Age too large")
    return age
"##;
    let result = transpile(code);
    assert!(result.contains("fn validate_age"), "Got: {}", result);
}

#[test]
fn test_s12_b86_error_accumulator() {
    let code = r#"
def parse_numbers(items: list) -> tuple:
    numbers = []
    errors = []
    for item in items:
        try:
            numbers.append(int(item))
        except ValueError:
            errors.append(item)
    return (numbers, errors)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_numbers"), "Got: {}", result);
}

#[test]
fn test_s12_b86_nested_try_except() {
    let code = r#"
def safe_operation(a: str, b: str) -> int:
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
    assert!(result.contains("fn safe_operation"), "Got: {}", result);
}

#[test]
fn test_s12_b86_finally_cleanup() {
    let code = r#"
def counted_operation(counter: list) -> int:
    counter.append(1)
    try:
        result = 42
        return result
    finally:
        counter.append(2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn counted_operation"), "Got: {}", result);
}

#[test]
fn test_s12_b86_try_else_finally() {
    let code = r#"
def full_error_handling(s: str) -> str:
    status = "start"
    try:
        value = int(s)
    except ValueError:
        status = "error"
    else:
        status = "success"
    finally:
        pass
    return status
"#;
    let result = transpile(code);
    assert!(result.contains("fn full_error_handling"), "Got: {}", result);
}

#[test]
fn test_s12_b86_raise_from() {
    let code = r##"
def convert(value: str) -> int:
    try:
        return int(value)
    except ValueError:
        raise TypeError("Cannot convert to int")
"##;
    let result = transpile(code);
    assert!(result.contains("fn convert"), "Got: {}", result);
}

#[test]
fn test_s12_b86_except_base() {
    let code = r#"
def safe_call(func) -> str:
    try:
        result = func()
        return str(result)
    except Exception:
        return "error"
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_call"), "Got: {}", result);
}

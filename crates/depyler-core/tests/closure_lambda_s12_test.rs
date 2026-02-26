//! Session 12 Batch 68: Closure and lambda deep cold paths
//!
//! Targets lambda/closure codegen paths that are rarely exercised.

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
fn test_s12_b68_lambda_identity() {
    let code = r#"
def get_identity():
    return lambda x: x
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_identity"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_add() {
    let code = r#"
def get_adder(n: int):
    return lambda x: x + n
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_adder"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_in_sort() {
    let code = r#"
def sort_by_abs(items: list) -> list:
    return sorted(items, key=lambda x: abs(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_abs"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_in_filter() {
    let code = r#"
def even_only(items: list) -> list:
    return list(filter(lambda x: x % 2 == 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_only"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_in_map() {
    let code = r#"
def triple(items: list) -> list:
    return list(map(lambda x: x * 3, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn triple"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_multi_arg() {
    let code = r#"
def get_multiplier():
    return lambda a, b: a * b
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_multiplier"), "Got: {}", result);
}

#[test]
fn test_s12_b68_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return x + y
    return inner(10)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

#[test]
fn test_s12_b68_counter_closure() {
    let code = r#"
def make_counter(start: int):
    count = start
    def increment() -> int:
        nonlocal count
        count += 1
        return count
    return increment
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_counter"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_in_dict() {
    let code = r#"
def get_ops():
    return {
        "double": lambda x: x * 2,
        "square": lambda x: x ** 2,
        "negate": lambda x: -x
    }
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_ops"), "Got: {}", result);
}

#[test]
fn test_s12_b68_compose() {
    let code = r#"
def compose(f, g):
    return lambda x: f(g(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn compose"), "Got: {}", result);
}

#[test]
fn test_s12_b68_lambda_conditional() {
    let code = r#"
def get_classifier():
    return lambda x: "positive" if x > 0 else ("negative" if x < 0 else "zero")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_classifier"), "Got: {}", result);
}

#[test]
fn test_s12_b68_function_as_param() {
    let code = r#"
def apply(func, items: list) -> list:
    return [func(item) for item in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply"), "Got: {}", result);
}

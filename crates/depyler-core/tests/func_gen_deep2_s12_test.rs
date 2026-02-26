//! Session 12 Batch 81: Function generation deep cold paths 2
//!
//! Targets func_gen.rs cold paths for parameter type inference,
//! return type inference, complex default parameters, decorators,
//! and function signature patterns.

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

// ===== Parameter type inference from body =====

#[test]
fn test_s12_b81_infer_param_from_str_method() {
    let code = r#"
def process(text):
    return text.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b81_infer_param_from_list_method() {
    let code = r#"
def add_item(items, value):
    items.append(value)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_item"), "Got: {}", result);
}

#[test]
fn test_s12_b81_infer_param_from_dict_method() {
    let code = r#"
def safe_get(data, key):
    return data.get(key, None)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_b81_infer_param_from_arithmetic() {
    let code = r#"
def double(x):
    return x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn double"), "Got: {}", result);
}

#[test]
fn test_s12_b81_infer_param_from_comparison() {
    let code = r#"
def is_positive(x):
    return x > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_positive"), "Got: {}", result);
}

// ===== Return type inference =====

#[test]
fn test_s12_b81_return_str_literal() {
    let code = r##"
def get_name():
    return "hello"
"##;
    let result = transpile(code);
    assert!(result.contains("fn get_name"), "Got: {}", result);
}

#[test]
fn test_s12_b81_return_int_literal() {
    let code = r#"
def get_zero():
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_zero"), "Got: {}", result);
}

#[test]
fn test_s12_b81_return_bool_literal() {
    let code = r#"
def get_true():
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_true"), "Got: {}", result);
}

#[test]
fn test_s12_b81_return_float_literal() {
    let code = r#"
def get_pi():
    return 3.14159
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_pi"), "Got: {}", result);
}

#[test]
fn test_s12_b81_return_list_literal() {
    let code = r#"
def get_empty_list():
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_empty_list"), "Got: {}", result);
}

#[test]
fn test_s12_b81_return_dict_literal() {
    let code = r#"
def get_empty_dict():
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_empty_dict"), "Got: {}", result);
}

// ===== Default parameter patterns =====

#[test]
fn test_s12_b81_default_none() {
    let code = r#"
def greet(name: str, title=None) -> str:
    if title is not None:
        return title + " " + name
    return name
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_b81_default_empty_string() {
    let code = r##"
def join_parts(a: str, b: str, sep: str = "") -> str:
    return a + sep + b
"##;
    let result = transpile(code);
    assert!(result.contains("fn join_parts"), "Got: {}", result);
}

#[test]
fn test_s12_b81_default_zero() {
    let code = r#"
def offset(x: int, amount: int = 0) -> int:
    return x + amount
"#;
    let result = transpile(code);
    assert!(result.contains("fn offset"), "Got: {}", result);
}

#[test]
fn test_s12_b81_default_bool() {
    let code = r#"
def process(data: list, reverse: bool = False) -> list:
    if reverse:
        return list(reversed(data))
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b81_multiple_defaults() {
    let code = r##"
def format_record(name: str, age: int = 0, city: str = "Unknown") -> str:
    return name + " " + str(age) + " " + city
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_record"), "Got: {}", result);
}

// ===== Complex function patterns =====

#[test]
fn test_s12_b81_generator_function() {
    let code = r#"
def fibonacci(n: int):
    a = 0
    b = 1
    for i in range(n):
        yield a
        a, b = b, a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci") || result.contains("fibonacci"), "Got: {}", result);
}

#[test]
fn test_s12_b81_recursive_with_types() {
    let code = r#"
def tree_depth(node) -> int:
    if node is None:
        return 0
    left = tree_depth(node.left)
    right = tree_depth(node.right)
    return 1 + max(left, right)
"#;
    let result = transpile(code);
    assert!(result.contains("fn tree_depth"), "Got: {}", result);
}

#[test]
fn test_s12_b81_many_params() {
    let code = r#"
def create_record(name: str, age: int, email: str, phone: str, active: bool) -> dict:
    return {
        "name": name,
        "age": age,
        "email": email,
        "phone": phone,
        "active": active
    }
"#;
    let result = transpile(code);
    assert!(result.contains("fn create_record"), "Got: {}", result);
}

#[test]
fn test_s12_b81_no_return_annotation() {
    let code = r#"
def print_items(items):
    for item in items:
        print(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_items"), "Got: {}", result);
}

#[test]
fn test_s12_b81_star_args() {
    let code = r#"
def sum_all(*args) -> int:
    return sum(args)
"#;
    let result = transpile(code);
    assert!(result.contains("sum_all"), "Got: {}", result);
}

#[test]
fn test_s12_b81_kwargs() {
    let code = r#"
def build_config(**kwargs) -> dict:
    return dict(kwargs)
"#;
    let result = transpile(code);
    assert!(result.contains("build_config"), "Got: {}", result);
}

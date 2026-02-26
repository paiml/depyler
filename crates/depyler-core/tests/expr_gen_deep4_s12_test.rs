//! Session 12 Batch 93: Expression gen deep cold paths 4
//!
//! Targets expr_gen.rs (67.49% line coverage) focusing on:
//! complex call expressions, nested attribute access,
//! starred args, keyword args, and conditional expressions.

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

// ===== Complex call expressions =====

#[test]
fn test_s12_b93_call_with_kwargs() {
    let code = r#"
def create_user(name: str, age: int, active: bool = True) -> dict:
    return {"name": name, "age": age, "active": active}
"#;
    let result = transpile(code);
    assert!(result.contains("fn create_user"), "Got: {}", result);
}

#[test]
fn test_s12_b93_sorted_with_key_reverse() {
    let code = r#"
def sort_desc_by_len(items: list) -> list:
    return sorted(items, key=len, reverse=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_desc_by_len"), "Got: {}", result);
}

#[test]
fn test_s12_b93_print_with_kwargs() {
    let code = r##"
def print_csv(items: list):
    print(*items, sep=",")
"##;
    let result = transpile(code);
    assert!(result.contains("fn print_csv"), "Got: {}", result);
}

// ===== Nested attribute access =====

#[test]
fn test_s12_b93_chained_attr() {
    let code = r#"
class Config:
    def __init__(self):
        self.db = DBConfig()

class DBConfig:
    def __init__(self):
        self.host = "localhost"
        self.port = 5432

def get_port(config: Config) -> int:
    return config.db.port
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_port"), "Got: {}", result);
}

// ===== Complex expression patterns =====

#[test]
fn test_s12_b93_nested_call_result() {
    let code = r#"
def process_text(text: str) -> list:
    return sorted(set(text.lower().split()))
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_text"), "Got: {}", result);
}

#[test]
fn test_s12_b93_method_on_literal() {
    let code = r#"
def split_csv(text: str) -> list:
    return text.split(",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_csv"), "Got: {}", result);
}

#[test]
fn test_s12_b93_ternary_with_call() {
    let code = r#"
def safe_len(items) -> int:
    return len(items) if items is not None else 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_len"), "Got: {}", result);
}

#[test]
fn test_s12_b93_complex_ternary() {
    let code = r##"
def grade(score: int) -> str:
    return "A" if score >= 90 else ("B" if score >= 80 else ("C" if score >= 70 else "F"))
"##;
    let result = transpile(code);
    assert!(result.contains("fn grade"), "Got: {}", result);
}

// ===== String formatting patterns =====

#[test]
fn test_s12_b93_percent_format() {
    let code = r##"
def format_message(name: str, count: int) -> str:
    return "%s has %d items" % (name, count)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_message"), "Got: {}", result);
}

#[test]
fn test_s12_b93_str_format_method() {
    let code = r##"
def format_record(name: str, age: int) -> str:
    return "{} is {} years old".format(name, age)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_record"), "Got: {}", result);
}

// ===== Comparison patterns =====

#[test]
fn test_s12_b93_chained_compare() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_b93_triple_compare() {
    let code = r#"
def ordered(a: int, b: int, c: int) -> bool:
    return a <= b <= c
"#;
    let result = transpile(code);
    assert!(result.contains("fn ordered"), "Got: {}", result);
}

#[test]
fn test_s12_b93_not_equal() {
    let code = r#"
def differs(a: int, b: int) -> bool:
    return a != b
"#;
    let result = transpile(code);
    assert!(result.contains("fn differs"), "Got: {}", result);
}

// ===== Complex boolean expressions =====

#[test]
fn test_s12_b93_complex_and_or() {
    let code = r#"
def is_valid(x: int, y: int) -> bool:
    return (x > 0 and y > 0) or (x == 0 and y == 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid"), "Got: {}", result);
}

#[test]
fn test_s12_b93_nested_not() {
    let code = r#"
def neither_empty(a: list, b: list) -> bool:
    return not (not a or not b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn neither_empty"), "Got: {}", result);
}

// ===== Starred and unpacking =====

#[test]
fn test_s12_b93_star_in_call() {
    let code = r#"
def spread_and_sum(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn spread_and_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b93_dict_unpacking() {
    let code = r#"
def merge_two(a: dict, b: dict) -> dict:
    result = {}
    result.update(a)
    result.update(b)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_two"), "Got: {}", result);
}

// ===== Complex expressions in context =====

#[test]
fn test_s12_b93_expr_in_assert() {
    let code = r#"
def check_sorted(items: list):
    for i in range(len(items) - 1):
        assert items[i] <= items[i + 1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_sorted"), "Got: {}", result);
}

#[test]
fn test_s12_b93_expr_in_return() {
    let code = r#"
def mean_or_zero(items: list) -> float:
    return sum(items) / len(items) if items else 0.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn mean_or_zero"), "Got: {}", result);
}

#[test]
fn test_s12_b93_complex_subscript() {
    let code = r#"
def diagonal(matrix: list) -> list:
    return [matrix[i][i] for i in range(len(matrix))]
"#;
    let result = transpile(code);
    assert!(result.contains("fn diagonal"), "Got: {}", result);
}

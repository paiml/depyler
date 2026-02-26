//! Session 12 Batch 79: Instance method deep cold paths 5
//!
//! Targets expr_gen_instance_methods.rs cold paths for rare
//! string methods with args, collection methods in unusual contexts,
//! and method calls on complex expressions.

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

// ===== String methods with multiple args =====

#[test]
fn test_s12_b79_str_center_default() {
    let code = r#"
def center_text(s: str, width: int) -> str:
    return s.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12_b79_str_ljust_default() {
    let code = r#"
def left_justify(s: str, width: int) -> str:
    return s.ljust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_justify"), "Got: {}", result);
}

#[test]
fn test_s12_b79_str_rjust_default() {
    let code = r#"
def right_justify(s: str, width: int) -> str:
    return s.rjust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_justify"), "Got: {}", result);
}

#[test]
fn test_s12_b79_str_split_no_args() {
    let code = r#"
def get_words(s: str) -> list:
    return s.split()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_words"), "Got: {}", result);
}

#[test]
fn test_s12_b79_str_rfind() {
    let code = r#"
def last_occurrence(s: str, sub: str) -> int:
    return s.rfind(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_occurrence"), "Got: {}", result);
}

#[test]
fn test_s12_b79_str_count() {
    let code = r#"
def count_substr(s: str, sub: str) -> int:
    return s.count(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_substr"), "Got: {}", result);
}

// ===== Method calls on expressions =====

#[test]
fn test_s12_b79_method_on_slice() {
    let code = r#"
def first_word_upper(text: str) -> str:
    return text.split()[0].upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_word_upper"), "Got: {}", result);
}

#[test]
fn test_s12_b79_method_on_subscript() {
    let code = r#"
def capitalize_first(items: list) -> str:
    return items[0].capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn capitalize_first"), "Got: {}", result);
}

#[test]
fn test_s12_b79_method_on_concat() {
    let code = r#"
def greeting_upper(first: str, last: str) -> str:
    return (first + " " + last).upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn greeting_upper"), "Got: {}", result);
}

// ===== Methods as arguments =====

#[test]
fn test_s12_b79_method_result_as_arg() {
    let code = r#"
def process_words(text: str) -> int:
    return len(text.split())
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_words"), "Got: {}", result);
}

#[test]
fn test_s12_b79_chained_methods() {
    let code = r#"
def clean(text: str) -> str:
    return text.strip().lower().replace("  ", " ")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

// ===== List methods in complex patterns =====

#[test]
fn test_s12_b79_list_index_method() {
    let code = r#"
def find_position(items: list, target: int) -> int:
    return items.index(target)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_position"), "Got: {}", result);
}

#[test]
fn test_s12_b79_list_count_method() {
    let code = r#"
def frequency(items: list, target: int) -> int:
    return items.count(target)
"#;
    let result = transpile(code);
    assert!(result.contains("fn frequency"), "Got: {}", result);
}

#[test]
fn test_s12_b79_list_reverse_method() {
    let code = r#"
def reverse_in_place(items: list) -> list:
    items.reverse()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_in_place"), "Got: {}", result);
}

#[test]
fn test_s12_b79_list_clear_method() {
    let code = r#"
def reset(items: list):
    items.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reset"), "Got: {}", result);
}

#[test]
fn test_s12_b79_list_copy_method() {
    let code = r#"
def duplicate(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn duplicate"), "Got: {}", result);
}

// ===== Dict methods in complex patterns =====

#[test]
fn test_s12_b79_dict_items_in_comp() {
    let code = r#"
def filter_dict(d: dict, threshold: int) -> dict:
    return {k: v for k, v in d.items() if v > threshold}
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b79_dict_keys_sorted() {
    let code = r#"
def sorted_keys(d: dict) -> list:
    return sorted(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn sorted_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b79_dict_values_sum() {
    let code = r#"
def total_values(d: dict) -> int:
    return sum(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn total_values"), "Got: {}", result);
}

// ===== Complex class method patterns =====

#[test]
fn test_s12_b79_class_with_dict_methods() {
    let code = r#"
class Counter:
    def __init__(self):
        self.counts = {}

    def add(self, key: str):
        self.counts[key] = self.counts.get(key, 0) + 1

    def get(self, key: str) -> int:
        return self.counts.get(key, 0)

    def most_common(self, n: int) -> list:
        pairs = sorted(self.counts.items(), key=lambda x: x[1], reverse=True)
        return pairs[:n]

    def total(self) -> int:
        return sum(self.counts.values())
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

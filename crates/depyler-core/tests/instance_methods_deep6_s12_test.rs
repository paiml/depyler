//! Session 12 Batch 94: Instance methods deep cold paths 6
//!
//! Targets expr_gen_instance_methods.rs remaining cold paths:
//! methods in complex contexts, method results as arguments,
//! and methods on complex receiver expressions.

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

// ===== String method chains =====

#[test]
fn test_s12_b94_strip_split_join() {
    let code = r#"
def clean_words(text: str) -> str:
    return " ".join(text.strip().split())
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_words"), "Got: {}", result);
}

#[test]
fn test_s12_b94_lower_replace_strip() {
    let code = r#"
def normalize(text: str) -> str:
    return text.lower().replace("-", "_").strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_b94_upper_startswith() {
    let code = r#"
def is_constant_name(name: str) -> bool:
    return name.upper() == name and name.startswith("_") == False
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_constant_name"), "Got: {}", result);
}

// ===== Methods on complex receivers =====

#[test]
fn test_s12_b94_method_on_dict_get() {
    let code = r#"
def get_upper(d: dict, key: str) -> str:
    return d.get(key, "").upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_upper"), "Got: {}", result);
}

#[test]
fn test_s12_b94_method_on_list_index() {
    let code = r#"
def first_upper(items: list) -> str:
    return items[0].upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_upper"), "Got: {}", result);
}

#[test]
fn test_s12_b94_method_on_join_result() {
    let code = r#"
def join_and_upper(items: list) -> str:
    return ", ".join(items).upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_and_upper"), "Got: {}", result);
}

// ===== Method results as arguments =====

#[test]
fn test_s12_b94_split_as_arg() {
    let code = r#"
def word_count(text: str) -> int:
    return len(text.split())
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_count"), "Got: {}", result);
}

#[test]
fn test_s12_b94_strip_as_arg() {
    let code = r#"
def non_empty_length(text: str) -> int:
    return len(text.strip())
"#;
    let result = transpile(code);
    assert!(result.contains("fn non_empty_length"), "Got: {}", result);
}

#[test]
fn test_s12_b94_lower_in_condition() {
    let code = r#"
def case_insensitive_match(a: str, b: str) -> bool:
    return a.lower() == b.lower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn case_insensitive_match"), "Got: {}", result);
}

// ===== List methods in iteration =====

#[test]
fn test_s12_b94_pop_until_empty() {
    let code = r#"
def drain_stack(stack: list) -> list:
    result = []
    while len(stack) > 0:
        result.append(stack.pop())
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn drain_stack"), "Got: {}", result);
}

#[test]
fn test_s12_b94_append_in_nested_loop() {
    let code = r#"
def cartesian(a: list, b: list) -> list:
    result = []
    for x in a:
        for y in b:
            result.append((x, y))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn cartesian"), "Got: {}", result);
}

// ===== Dict methods in iteration =====

#[test]
fn test_s12_b94_dict_items_transform() {
    let code = r#"
def swap_key_value(d: dict) -> dict:
    result = {}
    for k, v in d.items():
        result[v] = k
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_key_value"), "Got: {}", result);
}

#[test]
fn test_s12_b94_dict_get_with_default() {
    let code = r#"
def count_occurrences(items: list) -> dict:
    counts = {}
    for item in items:
        counts[item] = counts.get(item, 0) + 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_occurrences"), "Got: {}", result);
}

// ===== Set methods in complex contexts =====

#[test]
fn test_s12_b94_set_add_in_loop() {
    let code = r#"
def collect_unique(items: list) -> list:
    seen = set()
    result = []
    for item in items:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn collect_unique"), "Got: {}", result);
}

#[test]
fn test_s12_b94_set_operations_combo() {
    let code = r#"
def venn(a: set, b: set) -> dict:
    result = {}
    result["only_a"] = len(a - b)
    result["only_b"] = len(b - a)
    result["both"] = len(a & b)
    result["either"] = len(a | b)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn venn"), "Got: {}", result);
}

// ===== Complex class with many method types =====

#[test]
fn test_s12_b94_full_class() {
    let code = r##"
class Database:
    def __init__(self):
        self.tables = {}

    def create_table(self, name: str):
        self.tables[name] = []

    def insert(self, table: str, record: dict):
        if table in self.tables:
            self.tables[table].append(record)

    def select_all(self, table: str) -> list:
        return self.tables.get(table, [])

    def count(self, table: str) -> int:
        return len(self.tables.get(table, []))

    def delete_table(self, name: str):
        if name in self.tables:
            del self.tables[name]

    def table_names(self) -> list:
        return sorted(self.tables.keys())
"##;
    let result = transpile(code);
    assert!(result.contains("Database"), "Got: {}", result);
}

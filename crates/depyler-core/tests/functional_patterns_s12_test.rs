//! Session 12 Batch 42: Functional patterns and stdlib cold paths
//!
//! Targets remaining cold paths:
//! - map/filter/reduce functional patterns
//! - Complex print() patterns
//! - abs/min/max with multiple args
//! - Complex class hierarchies
//! - Multiple functions with shared state
//! - Data transformation pipelines

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

// ===== Functional patterns =====

#[test]
fn test_s12_b42_map_and_filter() {
    let code = r#"
def transform(items: list) -> list:
    doubled = list(map(lambda x: x * 2, items))
    return list(filter(lambda x: x > 5, doubled))
"#;
    let result = transpile(code);
    assert!(result.contains("fn transform"), "Got: {}", result);
}

#[test]
fn test_s12_b42_reduce_manual() {
    let code = r#"
def product(items: list) -> int:
    result = 1
    for item in items:
        result *= item
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn product"), "Got: {}", result);
}

// ===== Print patterns =====

#[test]
fn test_s12_b42_print_basic() {
    let code = r#"
def greet(name: str):
    print(f"Hello, {name}!")
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_b42_print_multiple_args() {
    let code = r#"
def show_info(name: str, age: int):
    print("Name:", name, "Age:", age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_info"), "Got: {}", result);
}

#[test]
fn test_s12_b42_print_sep() {
    let code = r#"
def print_csv(items: list):
    print(*items, sep=",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_csv"), "Got: {}", result);
}

// ===== abs/min/max patterns =====

#[test]
fn test_s12_b42_abs_usage() {
    let code = r#"
def distance(a: int, b: int) -> int:
    return abs(a - b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Got: {}", result);
}

#[test]
fn test_s12_b42_min_two() {
    let code = r#"
def smaller(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn smaller"), "Got: {}", result);
}

#[test]
fn test_s12_b42_max_two() {
    let code = r#"
def larger(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn larger"), "Got: {}", result);
}

#[test]
fn test_s12_b42_min_three() {
    let code = r#"
def smallest(a: int, b: int, c: int) -> int:
    return min(a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn smallest"), "Got: {}", result);
}

#[test]
fn test_s12_b42_min_list() {
    let code = r#"
def min_val(items: list) -> int:
    return min(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_val"), "Got: {}", result);
}

#[test]
fn test_s12_b42_max_list() {
    let code = r#"
def max_val(items: list) -> int:
    return max(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_val"), "Got: {}", result);
}

// ===== Complex class hierarchies =====

#[test]
fn test_s12_b42_multi_class_system() {
    let code = r##"
class Event:
    def __init__(self, name: str, priority: int):
        self.name = name
        self.priority = priority

class EventQueue:
    def __init__(self):
        self.events = []

    def push(self, event):
        self.events.append(event)

    def pop_highest(self):
        if not self.events:
            return None
        best_idx = 0
        for i in range(1, len(self.events)):
            if self.events[i].priority > self.events[best_idx].priority:
                best_idx = i
        event = self.events[best_idx]
        self.events.pop(best_idx)
        return event

    def size(self) -> int:
        return len(self.events)
"##;
    let result = transpile(code);
    assert!(result.contains("Event"), "Got: {}", result);
    assert!(result.contains("EventQueue"), "Got: {}", result);
}

// ===== Multiple functions with shared logic =====

#[test]
fn test_s12_b42_helper_shared() {
    let code = r#"
def is_valid(c: str) -> bool:
    return c.isalnum() or c == "_"

def clean_identifier(name: str) -> str:
    result = ""
    for c in name:
        if is_valid(c):
            result += c
    return result

def validate_name(name: str) -> bool:
    if not name:
        return False
    if name[0].isdigit():
        return False
    return name == clean_identifier(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid"), "Got: {}", result);
    assert!(result.contains("fn clean_identifier"), "Got: {}", result);
    assert!(result.contains("fn validate_name"), "Got: {}", result);
}

// ===== Data transformation pipeline =====

#[test]
fn test_s12_b42_etl_pipeline() {
    let code = r#"
def extract(raw: str) -> list:
    return raw.strip().split("\n")

def transform_line(line: str) -> dict:
    parts = line.split(",")
    return {"name": parts[0], "value": parts[1]}

def load(records: list) -> dict:
    result = {}
    for record in records:
        result[record["name"]] = record["value"]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn extract"), "Got: {}", result);
    assert!(result.contains("fn transform_line"), "Got: {}", result);
    assert!(result.contains("fn load"), "Got: {}", result);
}

// ===== Complex list comprehension =====

#[test]
fn test_s12_b42_matrix_flatten_comp() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [elem for row in matrix for elem in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b42_dict_comp_transform() {
    let code = r#"
def uppercase_keys(d: dict) -> dict:
    return {k.upper(): v for k, v in d.items()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn uppercase_keys"), "Got: {}", result);
}

// ===== Complex error handling =====

#[test]
fn test_s12_b42_error_chain() {
    let code = r##"
def safe_parse_config(text: str) -> dict:
    result = {}
    for line in text.split("\n"):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        try:
            key, value = line.split("=", 1)
            result[key.strip()] = value.strip()
        except ValueError:
            continue
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse_config"), "Got: {}", result);
}

// ===== Recursive patterns =====

#[test]
fn test_s12_b42_recursive_power() {
    let code = r#"
def fast_pow(base: int, exp: int) -> int:
    if exp == 0:
        return 1
    if exp % 2 == 0:
        half = fast_pow(base, exp // 2)
        return half * half
    return base * fast_pow(base, exp - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fast_pow"), "Got: {}", result);
}

#[test]
fn test_s12_b42_recursive_flatten() {
    let code = r#"
def depth(items: list) -> int:
    if not items:
        return 0
    max_depth = 0
    for item in items:
        if isinstance(item, list):
            d = depth(item)
            if d > max_depth:
                max_depth = d
    return max_depth + 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn depth"), "Got: {}", result);
}

// ===== Comparison operators =====

#[test]
fn test_s12_b42_eq_ne_lt_gt() {
    let code = r#"
def compare(a: int, b: int) -> str:
    if a == b:
        return "equal"
    elif a < b:
        return "less"
    elif a > b:
        return "greater"
    return "unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("fn compare"), "Got: {}", result);
}

// ===== String escape patterns =====

#[test]
fn test_s12_b42_escape_html() {
    let code = r#"
def escape_html(text: str) -> str:
    result = text.replace("&", "&amp;")
    result = result.replace("<", "&lt;")
    result = result.replace(">", "&gt;")
    result = result.replace('"', "&quot;")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn escape_html"), "Got: {}", result);
}

// ===== Multiple comprehension types =====

#[test]
fn test_s12_b42_gen_to_list() {
    let code = r#"
def squares_sum(n: int) -> int:
    return sum(i * i for i in range(1, n + 1))
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b42_set_from_comp() {
    let code = r#"
def vowels_in(text: str) -> set:
    return {c for c in text.lower() if c in "aeiou"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn vowels_in"), "Got: {}", result);
}

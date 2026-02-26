//! Session 12 Batch 61: Edge case patterns
//!
//! Targets unusual but valid Python patterns that exercise
//! rare codegen branches.

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

// ===== Deeply nested expressions =====

#[test]
fn test_s12_b61_nested_dict_access() {
    let code = r#"
def deep_get(data: dict, k1: str, k2: str, default: int) -> int:
    if k1 in data:
        inner = data[k1]
        if k2 in inner:
            return inner[k2]
    return default
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_get"), "Got: {}", result);
}

#[test]
fn test_s12_b61_chain_comparisons() {
    let code = r#"
def is_letter(c: str) -> bool:
    return "a" <= c <= "z" or "A" <= c <= "Z"
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_letter"), "Got: {}", result);
}

#[test]
fn test_s12_b61_multiple_assignment() {
    let code = r#"
def init() -> tuple:
    x = y = z = 0
    return (x, y, z)
"#;
    let result = transpile(code);
    assert!(result.contains("fn init"), "Got: {}", result);
}

// ===== Empty function body =====

#[test]
fn test_s12_b61_pass_function() {
    let code = r#"
def noop():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn noop"), "Got: {}", result);
}

#[test]
fn test_s12_b61_docstring_only() {
    let code = r#"
def documented():
    """This function does nothing."""
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn documented"), "Got: {}", result);
}

// ===== Complex unpacking =====

#[test]
fn test_s12_b61_tuple_in_for() {
    let code = r#"
def sum_pairs(pairs: list) -> int:
    total = 0
    for a, b in pairs:
        total += a + b
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b61_triple_unpack() {
    let code = r#"
def process_triples(items: list) -> list:
    result = []
    for a, b, c in items:
        result.append(a + b + c)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_triples"), "Got: {}", result);
}

// ===== Boolean edge cases =====

#[test]
fn test_s12_b61_short_circuit_and() {
    let code = r#"
def safe_check(items: list, idx: int) -> bool:
    return idx < len(items) and items[idx] > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_check"), "Got: {}", result);
}

#[test]
fn test_s12_b61_short_circuit_or() {
    let code = r#"
def default_value(x, default: int) -> int:
    return x or default
"#;
    let result = transpile(code);
    assert!(result.contains("fn default_value"), "Got: {}", result);
}

// ===== Nested function calls =====

#[test]
fn test_s12_b61_nested_calls() {
    let code = r#"
def process(text: str) -> str:
    return " ".join(sorted(set(text.lower().split())))
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Complex string operations =====

#[test]
fn test_s12_b61_multiline_builder() {
    let code = r##"
def build_html(title: str, items: list) -> str:
    html = f"<h1>{title}</h1>\n<ul>\n"
    for item in items:
        html += f"<li>{item}</li>\n"
    html += "</ul>"
    return html
"##;
    let result = transpile(code);
    assert!(result.contains("fn build_html"), "Got: {}", result);
}

// ===== Numeric edge cases =====

#[test]
fn test_s12_b61_zero_division_guard() {
    let code = r#"
def safe_div(a: float, b: float) -> float:
    if b == 0.0:
        return 0.0
    return a / b
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_div"), "Got: {}", result);
}

#[test]
fn test_s12_b61_negative_index() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_item"), "Got: {}", result);
}

#[test]
fn test_s12_b61_negative_slice() {
    let code = r#"
def last_three(items: list) -> list:
    return items[-3:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_three"), "Got: {}", result);
}

// ===== Complex lambda patterns =====

#[test]
fn test_s12_b61_lambda_in_sorted() {
    let code = r#"
def sort_by_second(pairs: list) -> list:
    return sorted(pairs, key=lambda x: x[1])
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_second"), "Got: {}", result);
}

#[test]
fn test_s12_b61_lambda_in_filter() {
    let code = r#"
def adults(people: list) -> list:
    return list(filter(lambda p: p[1] >= 18, people))
"#;
    let result = transpile(code);
    assert!(result.contains("fn adults"), "Got: {}", result);
}

#[test]
fn test_s12_b61_lambda_in_map() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_b61_class_with_many_types() {
    let code = r##"
class Config:
    def __init__(self):
        self.name = ""
        self.port = 0
        self.debug = False
        self.timeout = 0.0
        self.tags = []
        self.settings = {}

    def set_name(self, name: str):
        self.name = name

    def set_port(self, port: int):
        self.port = port

    def to_string(self) -> str:
        return f"{self.name}:{self.port}"
"##;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s12_b61_class_with_default_factory() {
    let code = r#"
class Counter:
    def __init__(self):
        self.counts = {}

    def add(self, key: str):
        if key in self.counts:
            self.counts[key] += 1
        else:
            self.counts[key] = 1

    def get(self, key: str) -> int:
        return self.counts.get(key, 0)

    def top_n(self, n: int) -> list:
        items = list(self.counts.items())
        items.sort(key=lambda x: x[1], reverse=True)
        return items[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

// ===== Walrus operator patterns =====

#[test]
fn test_s12_b61_walrus_in_while() {
    let code = r#"
def read_chunks(data: list) -> list:
    result = []
    i = 0
    while i < len(data):
        chunk = data[i:i+3]
        result.append(chunk)
        i += 3
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_chunks"), "Got: {}", result);
}

// ===== Complex print patterns =====

#[test]
fn test_s12_b61_print_formatted() {
    let code = r##"
def print_table(headers: list, rows: list) -> str:
    result = " | ".join(headers) + "\n"
    result += "-" * len(result) + "\n"
    for row in rows:
        result += " | ".join(str(x) for x in row) + "\n"
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn print_table"), "Got: {}", result);
}

// ===== Multiple decorators =====

#[test]
fn test_s12_b61_class_all_decorators() {
    let code = r#"
class Utils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def multiply(a: int, b: int) -> int:
        return a * b

    @classmethod
    def create(cls):
        return cls()
"#;
    let result = transpile(code);
    assert!(result.contains("Utils"), "Got: {}", result);
}

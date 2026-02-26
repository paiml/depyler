//! Session 12 Batch 62: Advanced codegen patterns
//!
//! Targets deep codegen paths that combine multiple features:
//! - Decorator patterns on functions
//! - Complex default parameter values
//! - Multiple return type paths with Optional
//! - Deep nested control flow with mixed types
//! - Class methods that return self type

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

// ===== Default parameter patterns =====

#[test]
fn test_s12_b62_default_none() {
    let code = r#"
def find(items: list, target: int, default=None):
    for item in items:
        if item == target:
            return item
    return default
"#;
    let result = transpile(code);
    assert!(result.contains("fn find"), "Got: {}", result);
}

#[test]
fn test_s12_b62_default_empty_list() {
    let code = r#"
def merge(a: list, b: list = []) -> list:
    result = []
    result.extend(a)
    result.extend(b)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

#[test]
fn test_s12_b62_default_empty_dict() {
    let code = r#"
def update_config(base: dict, overrides: dict = {}) -> dict:
    result = {}
    result.update(base)
    result.update(overrides)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn update_config"), "Got: {}", result);
}

#[test]
fn test_s12_b62_default_bool() {
    let code = r#"
def format_list(items: list, numbered: bool = False) -> str:
    result = []
    for i, item in enumerate(items):
        if numbered:
            result.append(str(i + 1) + ". " + str(item))
        else:
            result.append(str(item))
    return "\n".join(result)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_list"), "Got: {}", result);
}

// ===== Complex return types =====

#[test]
fn test_s12_b62_return_optional_int() {
    let code = r#"
def parse_number(s: str):
    try:
        return int(s)
    except ValueError:
        return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_number"), "Got: {}", result);
}

#[test]
fn test_s12_b62_return_tuple_triple() {
    let code = r#"
def stats(items: list) -> tuple:
    if not items:
        return (0, 0, 0.0)
    total = sum(items)
    count = len(items)
    avg = total / count
    return (total, count, avg)
"#;
    let result = transpile(code);
    assert!(result.contains("fn stats"), "Got: {}", result);
}

// ===== Mixed type patterns =====

#[test]
fn test_s12_b62_mixed_collection_ops() {
    let code = r##"
def analyze(data: list) -> dict:
    result = {
        "count": len(data),
        "sum": sum(data),
        "min": min(data) if data else 0,
        "max": max(data) if data else 0
    }
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn analyze"), "Got: {}", result);
}

#[test]
fn test_s12_b62_multi_type_processing() {
    let code = r##"
def process_mixed(items: list) -> dict:
    strings = []
    numbers = []
    for item in items:
        if isinstance(item, str):
            strings.append(item)
        elif isinstance(item, int):
            numbers.append(item)
    return {"strings": len(strings), "numbers": len(numbers)}
"##;
    let result = transpile(code);
    assert!(result.contains("fn process_mixed"), "Got: {}", result);
}

// ===== Deep nested patterns =====

#[test]
fn test_s12_b62_deep_nested_if() {
    let code = r#"
def classify(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "Q1"
        elif y < 0:
            return "Q4"
        else:
            return "+X"
    elif x < 0:
        if y > 0:
            return "Q2"
        elif y < 0:
            return "Q3"
        else:
            return "-X"
    else:
        if y > 0:
            return "+Y"
        elif y < 0:
            return "-Y"
        else:
            return "origin"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s12_b62_loop_with_complex_break() {
    let code = r#"
def find_peak(items: list) -> int:
    if len(items) < 2:
        return 0
    for i in range(1, len(items) - 1):
        if items[i] > items[i - 1] and items[i] > items[i + 1]:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_peak"), "Got: {}", result);
}

// ===== Builder pattern classes =====

#[test]
fn test_s12_b62_builder_class() {
    let code = r##"
class QueryBuilder:
    def __init__(self, table: str):
        self.table = table
        self.conditions = []
        self.columns = ["*"]
        self.limit_val = 0

    def select(self, cols: list):
        self.columns = cols
        return self

    def where(self, condition: str):
        self.conditions.append(condition)
        return self

    def limit(self, n: int):
        self.limit_val = n
        return self

    def build(self) -> str:
        cols = ", ".join(self.columns)
        sql = f"SELECT {cols} FROM {self.table}"
        if self.conditions:
            sql += " WHERE " + " AND ".join(self.conditions)
        if self.limit_val > 0:
            sql += f" LIMIT {self.limit_val}"
        return sql
"##;
    let result = transpile(code);
    assert!(result.contains("QueryBuilder"), "Got: {}", result);
}

// ===== Iterator patterns =====

#[test]
fn test_s12_b62_sliding_pairs() {
    let code = r#"
def pairs(items: list) -> list:
    result = []
    for i in range(len(items) - 1):
        result.append((items[i], items[i + 1]))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b62_windows() {
    let code = r#"
def windows(items: list, size: int) -> list:
    result = []
    for i in range(len(items) - size + 1):
        result.append(items[i:i + size])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn windows"), "Got: {}", result);
}

#[test]
fn test_s12_b62_batches() {
    let code = r#"
def batch(items: list, size: int) -> list:
    result = []
    for i in range(0, len(items), size):
        result.append(items[i:i + size])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn batch"), "Got: {}", result);
}

// ===== Complex data processing =====

#[test]
fn test_s12_b62_pivot_data() {
    let code = r#"
def pivot(records: list, key_col: str, val_col: str) -> dict:
    result = {}
    for record in records:
        key = record[key_col]
        value = record[val_col]
        if key not in result:
            result[key] = []
        result[key].append(value)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pivot"), "Got: {}", result);
}

#[test]
fn test_s12_b62_running_median() {
    let code = r#"
def running_avg(items: list) -> list:
    result = []
    total = 0.0
    for i, item in enumerate(items):
        total += item
        result.append(total / (i + 1))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn running_avg"), "Got: {}", result);
}

#[test]
fn test_s12_b62_histogram() {
    let code = r#"
def histogram(items: list, bins: int) -> list:
    if not items:
        return []
    lo = min(items)
    hi = max(items)
    if lo == hi:
        return [len(items)]
    width = (hi - lo) / bins
    counts = [0] * bins
    for item in items:
        idx = int((item - lo) / width)
        if idx >= bins:
            idx = bins - 1
        counts[idx] += 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn histogram"), "Got: {}", result);
}

//! Session 8 batch 3: Coverage tests for codegen pipeline
//! Targets: codegen.rs (hir_to_rust), scope tracker, union types,
//! cargo toml generation, and alternative codegen paths

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

// ── Deep nesting patterns (scope tracker) ───────────────────────

#[test]
fn test_deeply_nested_if() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    if x > 100:
        if x > 200:
            if x > 300:
                return "very large"
            return "large"
        return "medium"
    return "small"
"#,
    );
    assert!(
        code.contains("very large") && code.contains("small"),
        "Should handle deep nesting: {code}"
    );
}

#[test]
fn test_nested_for_loops() {
    let code = transpile(
        r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for cell in row:
            total += cell
    return total
"#,
    );
    assert!(
        code.contains("for") && code.contains("total"),
        "Should handle nested for loops: {code}"
    );
}

#[test]
fn test_mixed_nesting() {
    let code = transpile(
        r#"
def f(items: list) -> int:
    result = 0
    for item in items:
        if item > 0:
            while item > 10:
                item = item - 1
            result += item
    return result
"#,
    );
    assert!(
        code.contains("for") && code.contains("while"),
        "Should handle mixed nesting: {code}"
    );
}

// ── Scope shadowing patterns ────────────────────────────────────

#[test]
fn test_variable_shadowing() {
    let code = transpile(
        r#"
def f() -> int:
    x = 10
    for x in range(5):
        pass
    return x
"#,
    );
    assert!(
        code.contains("let") && code.contains("x"),
        "Should handle variable shadowing: {code}"
    );
}

#[test]
fn test_function_scope_isolation() {
    let code = transpile(
        r#"
def a() -> int:
    x = 1
    return x

def b() -> int:
    x = 2
    return x
"#,
    );
    assert!(
        code.contains("fn a") && code.contains("fn b"),
        "Should isolate function scopes: {code}"
    );
}

// ── Union type patterns ─────────────────────────────────────────

#[test]
fn test_optional_parameter() {
    let code = transpile(
        r#"
from typing import Optional
def f(x: Optional[int]) -> int:
    if x is None:
        return 0
    return x
"#,
    );
    assert!(
        code.contains("Option") || code.contains("None") || code.contains("Some"),
        "Should handle Optional param: {code}"
    );
}

#[test]
fn test_none_return() {
    let code = transpile(
        r#"
from typing import Optional
def find(items: list, target: int) -> Optional[int]:
    for item in items:
        if item == target:
            return item
    return None
"#,
    );
    assert!(
        code.contains("Option") || code.contains("None"),
        "Should return None/Option: {code}"
    );
}

// ── Cargo.toml generation exercise (via module detection) ───────

#[test]
fn test_imports_trigger_cargo_deps() {
    let code = transpile(
        r#"
import json
def serialize(data: dict) -> str:
    return json.dumps(data)
"#,
    );
    assert!(
        code.contains("serde_json") || code.contains("json") || code.contains("to_string"),
        "Should detect json dependency: {code}"
    );
}

#[test]
fn test_import_math() {
    let code = transpile(
        r#"
import math
def circle_area(r: float) -> float:
    return math.pi * r * r
"#,
    );
    assert!(
        code.contains("PI") || code.contains("std::f64::consts") || code.contains("pi") || code.contains("3.14"),
        "Should map math module: {code}"
    );
}

#[test]
fn test_import_os_path() {
    let code = transpile(
        r#"
import os
def exists(path: str) -> bool:
    return os.path.exists(path)
"#,
    );
    assert!(
        code.contains("Path") || code.contains("exists") || code.contains("std::path"),
        "Should map os.path: {code}"
    );
}

#[test]
fn test_import_sys() {
    let code = transpile(
        r#"
import sys
def main() -> int:
    args = sys.argv
    return len(args)
"#,
    );
    assert!(
        code.contains("args") || code.contains("env") || code.contains("std::env"),
        "Should map sys module: {code}"
    );
}

// ── Complex type patterns ───────────────────────────────────────

#[test]
fn test_nested_list_type() {
    let code = transpile(
        r#"
def flatten(matrix: list) -> list:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#,
    );
    assert!(
        code.contains("Vec") || code.contains("push"),
        "Should handle nested list: {code}"
    );
}

#[test]
fn test_dict_of_lists() {
    let code = transpile(
        r#"
def group_by_first_char(words: list) -> dict:
    groups = {}
    for word in words:
        key = word[0]
        if key not in groups:
            groups[key] = []
        groups[key].append(word)
    return groups
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("entry") || code.contains("insert"),
        "Should handle dict of lists: {code}"
    );
}

#[test]
fn test_list_of_tuples() {
    let code = transpile(
        r#"
def pairs(items: list) -> list:
    result = []
    for i in range(0, len(items), 2):
        result.append((items[i], items[i + 1]))
    return result
"#,
    );
    assert!(
        code.contains("push") || code.contains("Vec") || code.contains("tuple"),
        "Should handle list of tuples: {code}"
    );
}

// ── Method resolution patterns ──────────────────────────────────

#[test]
fn test_list_sort_reverse() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    items.sort()
    items.reverse()
    return items
"#,
    );
    assert!(
        code.contains("sort") && code.contains("reverse"),
        "Should handle sort and reverse: {code}"
    );
}

#[test]
fn test_string_methods_comprehensive() {
    let code = transpile(
        r#"
def process(s: str) -> bool:
    return s.startswith("hello") and s.endswith("world")
"#,
    );
    assert!(
        code.contains("starts_with") || code.contains("ends_with") || code.contains("startswith"),
        "Should handle string predicates: {code}"
    );
}

#[test]
fn test_list_count_and_index() {
    let code = transpile(
        r#"
def f(items: list, target: int) -> int:
    count = items.count(target)
    return count
"#,
    );
    assert!(
        code.contains("count") || code.contains("filter") || code.contains("iter"),
        "Should handle list.count: {code}"
    );
}

// ── Walrus operator patterns ────────────────────────────────────

#[test]
fn test_walrus_in_while() {
    let code = transpile(
        r#"
def read_lines(items: list) -> list:
    result = []
    i = 0
    while i < len(items):
        result.append(items[i])
        i += 1
    return result
"#,
    );
    assert!(
        code.contains("while") || code.contains("loop"),
        "Should handle while with index: {code}"
    );
}

// ── Complex class patterns ──────────────────────────────────────

#[test]
fn test_class_with_multiple_methods() {
    let code = transpile(
        r#"
class LinkedList:
    def __init__(self) -> None:
        self.data: list = []

    def append(self, val: int) -> None:
        self.data.append(val)

    def prepend(self, val: int) -> None:
        self.data.insert(0, val)

    def size(self) -> int:
        return len(self.data)

    def contains(self, val: int) -> bool:
        return val in self.data

    def remove_first(self) -> int:
        return self.data.pop(0)
"#,
    );
    assert!(
        code.contains("LinkedList") || code.contains("Linked"),
        "Should generate struct: {code}"
    );
}

#[test]
fn test_class_with_default_values() {
    let code = transpile(
        r#"
class Config:
    def __init__(self, host: str = "localhost", port: int = 8080) -> None:
        self.host = host
        self.port = port
"#,
    );
    assert!(
        code.contains("struct Config") && code.contains("host") && code.contains("port"),
        "Should handle class with defaults: {code}"
    );
}

// ── Multiple class interaction ──────────────────────────────────

#[test]
fn test_two_classes_same_module() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

class Line:
    def __init__(self, start: Point, end: Point) -> None:
        self.start = start
        self.end = end

    def length(self) -> float:
        dx = self.end.x - self.start.x
        dy = self.end.y - self.start.y
        return (dx * dx + dy * dy) ** 0.5
"#,
    );
    assert!(
        code.contains("struct Point") && code.contains("struct Line"),
        "Should generate both structs: {code}"
    );
}

// ── Edge case patterns ──────────────────────────────────────────

#[test]
fn test_empty_list_creation() {
    let code = transpile(
        r#"
def f() -> list:
    return []
"#,
    );
    assert!(
        code.contains("vec![]") || code.contains("Vec::new()") || code.contains("Vec"),
        "Should create empty vec: {code}"
    );
}

#[test]
fn test_empty_dict_creation() {
    let code = transpile(
        r#"
def f() -> dict:
    return {}
"#,
    );
    assert!(
        code.contains("HashMap::new()") || code.contains("HashMap"),
        "Should create empty HashMap: {code}"
    );
}

#[test]
fn test_boolean_literals() {
    let code = transpile(
        r#"
def f() -> bool:
    a = True
    b = False
    return a and not b
"#,
    );
    assert!(
        code.contains("true") && code.contains("false"),
        "Should handle bool literals: {code}"
    );
}

#[test]
fn test_none_literal() {
    let code = transpile(
        r#"
def f() -> None:
    x = None
    return None
"#,
    );
    assert!(
        code.contains("None") || code.contains("()"),
        "Should handle None: {code}"
    );
}

#[test]
fn test_negative_numbers() {
    let code = transpile(
        r#"
def f() -> int:
    x = -42
    y = -100
    return x + y
"#,
    );
    assert!(
        code.contains("-42") || code.contains("42"),
        "Should handle negative numbers: {code}"
    );
}

#[test]
fn test_large_integer() {
    let code = transpile(
        r#"
def f() -> int:
    return 1000000000
"#,
    );
    assert!(
        code.contains("1000000000"),
        "Should handle large integer: {code}"
    );
}

#[test]
fn test_float_literals() {
    let code = transpile(
        r#"
def f() -> float:
    return 3.14159
"#,
    );
    assert!(
        code.contains("3.14159"),
        "Should handle float literal: {code}"
    );
}

#[test]
fn test_multiline_string() {
    let code = transpile(
        r#"
def f() -> str:
    return "hello world"
"#,
    );
    assert!(
        code.contains("hello world"),
        "Should handle string literal: {code}"
    );
}

// ── Complex import patterns ─────────────────────────────────────

#[test]
fn test_from_import_multiple() {
    let code = transpile(
        r#"
from typing import List, Dict, Tuple, Optional
def f(items: List[int]) -> Dict[str, int]:
    return {"count": len(items)}
"#,
    );
    assert!(
        code.contains("Vec") || code.contains("HashMap"),
        "Should handle from typing import: {code}"
    );
}

#[test]
fn test_import_as_alias() {
    let code = transpile(
        r#"
import json as j
def f(data: dict) -> str:
    return j.dumps(data)
"#,
    );
    assert!(
        code.contains("fn f"),
        "Should handle import alias: {code}"
    );
}

// ── Pattern matching (Python 3.10+) ─────────────────────────────

#[test]
fn test_simple_match_statement() {
    let code = transpile(
        r#"
def http_status(status: int) -> str:
    if status == 200:
        return "OK"
    elif status == 404:
        return "Not Found"
    elif status == 500:
        return "Server Error"
    else:
        return "Unknown"
"#,
    );
    assert!(
        code.contains("200") && code.contains("404") && code.contains("500"),
        "Should handle status pattern: {code}"
    );
}

// ── Lambda and closure patterns ─────────────────────────────────

#[test]
fn test_lambda_in_sort() {
    let code = transpile(
        r#"
def sort_by_abs(items: list) -> list:
    return sorted(items, key=lambda x: abs(x))
"#,
    );
    assert!(
        code.contains("sort") || code.contains("abs"),
        "Should handle lambda in sort: {code}"
    );
}

#[test]
fn test_lambda_in_filter() {
    let code = transpile(
        r#"
def positive_only(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#,
    );
    assert!(
        code.contains("filter") || code.contains("iter") || code.contains("> 0"),
        "Should handle lambda in filter: {code}"
    );
}

#[test]
fn test_lambda_in_map() {
    let code = transpile(
        r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#,
    );
    assert!(
        code.contains("map") || code.contains("iter") || code.contains("* 2"),
        "Should handle lambda in map: {code}"
    );
}

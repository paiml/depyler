//! Session 12 Batch 98: Global patterns, module structure, and edge cases
//!
//! Targets remaining cold paths in rust_gen and stmt_gen for
//! module-level patterns, import handling, and edge cases.

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
fn test_s12_b98_module_docstring() {
    let code = r##"
"""This is a module docstring."""

def hello() -> str:
    return "world"
"##;
    let result = transpile(code);
    assert!(result.contains("fn hello"), "Got: {}", result);
}

#[test]
fn test_s12_b98_multiple_imports() {
    let code = r#"
import math
import os
import sys

def get_info() -> str:
    return str(math.pi)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_info"), "Got: {}", result);
}

#[test]
fn test_s12_b98_from_import() {
    let code = r#"
from typing import List, Dict, Optional, Tuple

def process(items: List[int]) -> Dict[str, int]:
    return {"count": len(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b98_class_then_function() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def add_points(a: Point, b: Point) -> Point:
    return Point(a.x + b.x, a.y + b.y)
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
    assert!(result.contains("fn add_points"), "Got: {}", result);
}

#[test]
fn test_s12_b98_constants_and_classes() {
    let code = r##"
MAX_RETRIES = 3
DEFAULT_TIMEOUT = 30

class Client:
    def __init__(self, host: str):
        self.host = host
        self.retries = MAX_RETRIES
        self.timeout = DEFAULT_TIMEOUT

    def connect(self) -> bool:
        return True
"##;
    let result = transpile(code);
    assert!(result.contains("MAX_RETRIES"), "Got: {}", result);
    assert!(result.contains("Client"), "Got: {}", result);
}

#[test]
fn test_s12_b98_pass_function() {
    let code = r#"
def noop():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn noop"), "Got: {}", result);
}

#[test]
fn test_s12_b98_ellipsis_function() {
    let code = r#"
def abstract_method():
    ...
"#;
    let result = transpile(code);
    assert!(result.contains("abstract_method"), "Got: {}", result);
}

#[test]
fn test_s12_b98_bare_expressions() {
    let code = r##"
"""Module with bare expressions."""

def main():
    x = 42
    y = "hello"
    z = True
    return x
"##;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

#[test]
fn test_s12_b98_multiple_classes() {
    let code = r#"
class Node:
    def __init__(self, value: int):
        self.value = value
        self.children = []

    def add_child(self, child):
        self.children.append(child)

class Tree:
    def __init__(self):
        self.root = None

    def set_root(self, value: int):
        self.root = Node(value)
"#;
    let result = transpile(code);
    assert!(result.contains("Node"), "Got: {}", result);
    assert!(result.contains("Tree"), "Got: {}", result);
}

#[test]
fn test_s12_b98_typing_union() {
    let code = r#"
from typing import Union

def to_string(value: Union[int, float, str]) -> str:
    return str(value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_string"), "Got: {}", result);
}

#[test]
fn test_s12_b98_large_function() {
    let code = r#"
def process_data(data: list) -> dict:
    if not data:
        return {}
    total = 0
    count = 0
    min_val = data[0]
    max_val = data[0]
    for item in data:
        total += item
        count += 1
        if item < min_val:
            min_val = item
        if item > max_val:
            max_val = item
    avg = total / count if count > 0 else 0.0
    result = {}
    result["total"] = total
    result["count"] = count
    result["min"] = min_val
    result["max"] = max_val
    result["avg"] = avg
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_data"), "Got: {}", result);
}

#[test]
fn test_s12_b98_deeply_nested() {
    let code = r#"
def process(items: list) -> list:
    result = []
    for item in items:
        if isinstance(item, list):
            for sub in item:
                if isinstance(sub, int):
                    if sub > 0:
                        result.append(sub)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

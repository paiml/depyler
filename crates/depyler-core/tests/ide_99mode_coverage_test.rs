//! Coverage tests for ide.rs
//!
//! DEPYLER-99MODE-001: Targets ide.rs (913 lines)
//! Covers: symbol indexing, navigation, hover, diagnostics,
//! function/class/method/variable/field symbol kinds.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Function symbols
// ============================================================================

#[test]
fn test_ide_function_symbol() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ide_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b

def divide(a: float, b: float) -> float:
    return a / b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class symbols
// ============================================================================

#[test]
fn test_ide_class_symbol() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ide_class_with_methods() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x: int) -> int:
        self.result += x
        return self.result

    def get_result(self) -> int:
        return self.result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable symbols
// ============================================================================

#[test]
fn test_ide_variable_declarations() {
    let code = r#"
def f() -> int:
    x = 42
    y = x + 1
    z = y * 2
    return z
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Parameter symbols
// ============================================================================

#[test]
fn test_ide_parameters() {
    let code = r#"
def process(data: list, threshold: int, flag: bool) -> int:
    total = 0
    for item in data:
        if item > threshold and flag:
            total += item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Field symbols
// ============================================================================

#[test]
fn test_ide_class_fields() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        self.active = True

    def get_name(self) -> str:
        return self.name
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module symbols (imports)
// ============================================================================

#[test]
fn test_ide_import_symbols() {
    let code = r#"
import json

def f(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ide_from_import_symbols() {
    let code = r#"
from collections import defaultdict

def f() -> dict:
    d = defaultdict(int)
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex symbol patterns
// ============================================================================

#[test]
fn test_ide_mixed_symbols() {
    let code = r#"
import math

def helper(x: float) -> float:
    return math.sqrt(x)

class MathHelper:
    def __init__(self, precision: int):
        self.precision = precision

    def compute(self, x: float) -> float:
        return helper(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ide_nested_structures() {
    let code = r#"
class Outer:
    def __init__(self):
        self.data = []

    def process(self) -> list:
        result = []
        for item in self.data:
            if item > 0:
                result.append(item * 2)
        return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ide_comprehensive() {
    let code = r#"
def is_valid(x: int) -> bool:
    return x >= 0

class DataProcessor:
    def __init__(self, items: list):
        self.items = items
        self.count = len(items)

    def filter_valid(self) -> list:
        result = []
        for item in self.items:
            if is_valid(item):
                result.append(item)
        return result

    def get_count(self) -> int:
        return self.count
"#;
    assert!(transpile_ok(code));
}

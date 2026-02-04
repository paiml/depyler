//! Coverage tests for cargo_first.rs
//!
//! DEPYLER-99MODE-001: Targets cargo_first.rs (1,017 lines)
//! Covers: ephemeral Cargo workspace creation, dependency auto-detection,
//! cargo check verification, error filtering.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Basic compilation verification
// ============================================================================

#[test]
fn test_cargo_first_simple_function() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_class() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x * self.x + self.y * self.y) ** 0.5
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dependency auto-detection
// ============================================================================

#[test]
fn test_cargo_first_json_dep() {
    let code = r#"
import json

def parse(s: str) -> dict:
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_regex_dep() {
    let code = r#"
import re

def find(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_hashlib_dep() {
    let code = r#"
import hashlib

def hash_it(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_collections_dep() {
    let code = r#"
from collections import defaultdict

def count(items: list) -> dict:
    d = defaultdict(int)
    for item in items:
        d[item] += 1
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// No dependency patterns
// ============================================================================

#[test]
fn test_cargo_first_no_deps() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_builtins_only() {
    let code = r#"
def process(items: list) -> list:
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_cargo_first_multi_dep() {
    let code = r#"
import json
import hashlib

def secure_serialize(data: dict) -> str:
    json_str = json.dumps(data)
    return hashlib.sha256(json_str.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_class_with_imports() {
    let code = r#"
import json

class Config:
    def __init__(self):
        self.data = {}

    def load(self, json_str: str):
        self.data = json.loads(json_str)

    def get(self, key: str) -> str:
        return self.data.get(key, "")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_exception_handling() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_first_complex_algorithm() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

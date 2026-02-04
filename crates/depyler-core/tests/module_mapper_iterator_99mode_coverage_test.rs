//! Coverage tests for module_mapper.rs and iterator_utils.rs
//!
//! DEPYLER-99MODE-001: Targets module_mapper.rs (1,345 lines) + iterator_utils.rs (1,199 lines)
//! Covers: module mapping, import resolution, iterator patterns,
//! lazy evaluation, chained iterators.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Module mapping - standard library
// ============================================================================

#[test]
fn test_module_mapper_math() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_os_path() {
    let code = r#"
import os

def f() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_json() {
    let code = r#"
import json

def f(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_collections() {
    let code = r#"
from collections import defaultdict

def f() -> dict:
    d = defaultdict(int)
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_datetime() {
    let code = r#"
from datetime import datetime

def f() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_re() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_hashlib() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_base64() {
    let code = r#"
import base64

def f(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Iterator patterns
// ============================================================================

#[test]
fn test_iterator_for_range() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_for_list() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_enumerate() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, val in enumerate(items):
        total += i * val
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_zip() {
    let code = r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_reversed() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_sorted() {
    let code = "def f(items: list) -> list:\n    return sorted(items)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_map() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Chained iterator patterns
// ============================================================================

#[test]
fn test_iterator_map_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, filter(lambda x: x > 0, items)))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_comprehension_equiv() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_string_iter() {
    let code = r#"
def f(text: str) -> int:
    count = 0
    for c in text:
        if c == 'a':
            count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_dict_iter() {
    let code = r#"
def f(d: dict) -> list:
    keys = []
    for k in d:
        keys.append(k)
    return keys
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_dict_items() {
    let code = r#"
def f(d: dict) -> list:
    pairs = []
    for k, v in d.items():
        pairs.append(k)
    return pairs
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex iterator patterns
// ============================================================================

#[test]
fn test_iterator_nested_loops() {
    let code = r#"
def f(n: int) -> list:
    pairs = []
    for i in range(n):
        for j in range(i + 1, n):
            pairs.append((i, j))
    return pairs
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_early_break() {
    let code = r#"
def f(items: list, target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iterator_continue_skip() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

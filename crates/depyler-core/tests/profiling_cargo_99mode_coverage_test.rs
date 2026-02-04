//! Coverage tests for profiling.rs and cargo_toml_gen.rs
//!
//! DEPYLER-99MODE-001: Targets profiling.rs (1,596 lines) and
//! cargo_toml_gen.rs (1,570 lines)
//! Covers: instruction counting, hot path detection, allocation tracking,
//! loop depth analysis, dependency extraction, cargo.toml generation.

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
// Profiling: instruction count baselines
// ============================================================================

#[test]
fn test_profile_simple_assignment() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_binary_operations() {
    let code = r#"
def f() -> int:
    a = 10
    b = 20
    return a + b * 2 - 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_single_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_nested_loops() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            total += i * j
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_while_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    i = 0
    while i < n:
        total += i
        i += 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Profiling: allocation tracking
// ============================================================================

#[test]
fn test_profile_list_allocation() {
    let code = r#"
def f() -> list:
    return [1, 2, 3, 4, 5]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_dict_allocation() {
    let code = r#"
def f() -> dict:
    return {"key": "value", "num": 42}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_multiple_allocations() {
    let code = r#"
def f() -> list:
    a = [1, 2]
    b = [3, 4]
    c = {"x": 1}
    return a + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Profiling: branch analysis
// ============================================================================

#[test]
fn test_profile_if_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_if_elif_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "pos"
    elif x < 0:
        return "neg"
    else:
        return "zero"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Profiling: type check patterns
// ============================================================================

#[test]
fn test_profile_isinstance_check() {
    let code = r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Profiling: multiple functions
// ============================================================================

#[test]
fn test_profile_multi_function() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def main_func(n: int) -> int:
    total = 0
    for i in range(n):
        total += helper(i)
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profile_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Profiling: complex function
// ============================================================================

#[test]
fn test_profile_complex_function() {
    let code = r#"
def process(items: list) -> dict:
    result = {}
    for item in items:
        if item > 0:
            if item in result:
                result[item] += 1
            else:
                result[item] = 1
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Cargo.toml generation: dependency detection
// ============================================================================

#[test]
fn test_cargo_json_dependency() {
    let code = r#"
import json

def f(s: str) -> dict:
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_regex_dependency() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_datetime_dependency() {
    let code = r#"
from datetime import datetime

def f() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_random_dependency() {
    let code = r#"
import random

def f() -> int:
    return random.randint(1, 10)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_base64_dependency() {
    let code = r#"
import base64

def f(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_hashlib_dependency() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_argparse_dependency() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, default="World")
    args = parser.parse_args()
    print(args.name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_csv_dependency() {
    let code = r#"
import csv

def f(path: str) -> list:
    result = []
    with open(path) as file:
        reader = csv.reader(file)
        for row in reader:
            result.append(row)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_itertools_dependency() {
    let code = r#"
from itertools import chain

def f(a: list, b: list) -> list:
    return list(chain(a, b))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_multiple_dependencies() {
    let code = r#"
import json
import re
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--data", type=str)
    args = parser.parse_args()
    data = json.loads(args.data)
    pattern = re.compile(r"\w+")
    print(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Cargo.toml: no external dependencies
// ============================================================================

#[test]
fn test_cargo_no_dependencies() {
    let code = r#"
def f(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Cargo.toml: async dependency
// ============================================================================

#[test]
fn test_cargo_async_dependency() {
    let code = r#"
import asyncio

async def f():
    await asyncio.sleep(1)
"#;
    assert!(transpile_ok(code));
}

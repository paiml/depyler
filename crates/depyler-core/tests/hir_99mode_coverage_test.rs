//! Coverage tests for hir.rs
//!
//! DEPYLER-99MODE-001: Targets hir.rs (1,248 lines)
//! Covers: HIR data structure construction through transpilation,
//! function/class/module representation, expressions, statements.

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
// Function representations
// ============================================================================

#[test]
fn test_hir_simple_function() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_function_with_params() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_function_with_defaults() {
    let code = r#"
def f(x: int, y: int = 0) -> int:
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_function_no_return() {
    let code = r#"
def f(x: int):
    y = x + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class representations
// ============================================================================

#[test]
fn test_hir_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_class_with_methods() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x: int) -> int:
        self.result += x
        return self.result

    def reset(self):
        self.result = 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_class_multiple_fields() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        self.active = True

    def get_name(self) -> str:
        return self.name

    def get_age(self) -> int:
        return self.age
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression types
// ============================================================================

#[test]
fn test_hir_binary_expr() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a + b * 2 - a % b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_unary_expr() {
    let code = r#"
def f(x: int) -> int:
    return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_comparison_expr() {
    let code = r#"
def f(a: int, b: int) -> bool:
    return a > b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_bool_expr() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    return a and b or not a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_call_expr() {
    let code = r#"
def f(items: list) -> int:
    return len(items) + sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_index_expr() {
    let code = r#"
def f(items: list) -> int:
    return items[0] + items[-1]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_attribute_expr() {
    let code = r#"
class Obj:
    def __init__(self):
        self.x = 0

    def get(self) -> int:
        return self.x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Statement types
// ============================================================================

#[test]
fn test_hir_assign_stmt() {
    let code = r#"
def f() -> int:
    x = 1
    y = 2
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_if_stmt() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_for_stmt() {
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
fn test_hir_while_stmt() {
    let code = r#"
def f(n: int) -> int:
    i = 0
    while i < n:
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_return_stmt() {
    let code = r#"
def f(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module-level patterns
// ============================================================================

#[test]
fn test_hir_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def subtract(a: int, b: int) -> int:
    return a - b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_function_and_class() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

class Worker:
    def __init__(self, value: int):
        self.value = value

    def process(self) -> int:
        return helper(self.value)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Import declarations
// ============================================================================

#[test]
fn test_hir_import() {
    let code = r#"
import json

def f(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hir_from_import() {
    let code = r#"
from collections import defaultdict

def f() -> dict:
    d = defaultdict(int)
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehensive HIR coverage
// ============================================================================

#[test]
fn test_hir_comprehensive() {
    let code = r#"
import math

def is_perfect_square(n: int) -> bool:
    if n < 0:
        return False
    root = int(math.sqrt(float(n)))
    return root * root == n

class NumberAnalyzer:
    def __init__(self, numbers: list):
        self.numbers = numbers

    def count_perfect_squares(self) -> int:
        count = 0
        for n in self.numbers:
            if is_perfect_square(n):
                count += 1
        return count
"#;
    assert!(transpile_ok(code));
}

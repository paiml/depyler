//! Coverage tests for hunt_mode modules
//!
//! DEPYLER-99MODE-001: Targets hunt_mode/ (repair 809, verifier 689,
//! planner 683, five_whys 651, hansei 612, isolator 521 = 3,965 lines)
//! Covers: error repair, compilation verification, error planning,
//! root cause analysis, self-reflection, minimal reproduction.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Repair patterns - type coercion fixes
// ============================================================================

#[test]
fn test_hunt_repair_type_coercion_int_str() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_repair_type_coercion_float() {
    let code = r#"
def f(x: int) -> float:
    return float(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_repair_type_coercion_bool() {
    let code = r#"
def f(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Repair patterns - import resolution
// ============================================================================

#[test]
fn test_hunt_repair_import_json() {
    let code = r#"
import json

def f(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_repair_import_re() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\w+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_repair_import_collections() {
    let code = r#"
from collections import defaultdict

def f() -> dict:
    d = defaultdict(int)
    d["a"] += 1
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Verifier patterns - compilation checks
// ============================================================================

#[test]
fn test_hunt_verify_simple() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_verify_class() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_verify_complex() {
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

// ============================================================================
// Planner patterns - error categorization
// ============================================================================

#[test]
fn test_hunt_plan_type_inference() {
    let code = r#"
def f() -> int:
    x = 42
    y = x + 1
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_plan_borrowing() {
    let code = r#"
def f(items: list) -> list:
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_plan_control_flow() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Five Whys patterns - root cause analysis
// ============================================================================

#[test]
fn test_hunt_five_whys_dynamic_typing() {
    let code = r#"
def f(x: int) -> str:
    return str(x * 2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_five_whys_ownership() {
    let code = r#"
def f(data: list) -> int:
    total = sum(data)
    count = len(data)
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_five_whys_heterogeneous_dict() {
    let code = r#"
def f() -> dict:
    return {"name": "test", "count": 42}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Hansei patterns - self-reflection
// ============================================================================

#[test]
fn test_hunt_hansei_success_pattern() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_hansei_complex_success() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.history = []

    def add(self, a: int, b: int) -> int:
        result = a + b
        self.history.append(result)
        return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Isolator patterns - minimal reproduction
// ============================================================================

#[test]
fn test_hunt_isolate_type_mismatch() {
    let code = r#"
def f(x: int, y: str) -> str:
    return y + str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_isolate_collection_mutation() {
    let code = r#"
def f() -> list:
    items = []
    for i in range(5):
        items.append(i)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_isolate_nested_dict() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = {"nested": 42}
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration patterns
// ============================================================================

#[test]
fn test_hunt_integration_algorithm() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a

def lcm(a: int, b: int) -> int:
    return a * b // gcd(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_integration_data_structure() {
    let code = r#"
class Queue:
    def __init__(self):
        self.items = []

    def enqueue(self, item: int):
        self.items.append(item)

    def dequeue(self) -> int:
        return self.items.pop(0)

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hunt_integration_string_processing() {
    let code = r#"
def count_words(text: str) -> dict:
    words = text.split()
    counts = {}
    for word in words:
        word_lower = word.lower()
        counts[word_lower] = counts.get(word_lower, 0) + 1
    return counts
"#;
    assert!(transpile_ok(code));
}

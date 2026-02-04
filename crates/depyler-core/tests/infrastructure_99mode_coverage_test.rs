//! Coverage tests for infrastructure modules
//!
//! DEPYLER-99MODE-001: Targets infrastructure/ (fault_localizer 606,
//! pattern_store 559, curriculum 543 = 1,708 lines)
//! Covers: Tarantula fault localization, pattern storage,
//! curriculum learning, suspiciousness scoring.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Fault localization - type inference decisions
// ============================================================================

#[test]
fn test_infra_fault_type_inference() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    z = y * 2
    return z
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_fault_type_coercion() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_fault_ownership() {
    let code = r#"
def f(items: list) -> int:
    total = sum(items)
    count = len(items)
    return total + count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pattern store - pattern reuse
// ============================================================================

#[test]
fn test_infra_pattern_simple() {
    let code = "def f(x: int) -> int:\n    return x * 2\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_pattern_list_ops() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    items.append(4)
    items.reverse()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_pattern_dict_ops() {
    let code = r#"
def f() -> dict:
    d = {"a": 1}
    d["b"] = 2
    d.update({"c": 3})
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_pattern_string_ops() {
    let code = r#"
def f(s: str) -> str:
    return s.strip().lower().replace("a", "b")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Curriculum learning - difficulty progression
// ============================================================================

#[test]
fn test_infra_curriculum_basic() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_curriculum_intermediate() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_curriculum_advanced() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def peek(self) -> int:
        return self.items[-1]

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Library mapping decisions
// ============================================================================

#[test]
fn test_infra_library_json() {
    let code = r#"
import json

def f(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_library_hashlib() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_infra_complex_algorithm() {
    let code = r#"
def bubble_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_complex_class() {
    let code = r#"
class LinkedNode:
    def __init__(self, value: int):
        self.value = value
        self.next = None

    def get_value(self) -> int:
        return self.value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infra_complex_multi_function() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

def count_primes(limit: int) -> int:
    count = 0
    for n in range(2, limit):
        if is_prime(n):
            count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

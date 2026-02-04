//! Coverage tests for decision_trace.rs
//!
//! DEPYLER-99MODE-001: Targets decision_trace.rs (2,850 lines)
//! Covers: type mapping decisions, borrow strategy decisions,
//! lifetime inference, method dispatch, import resolution,
//! error handling decisions, ownership decisions.

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
// TypeMapping decisions
// ============================================================================

#[test]
fn test_trace_type_int() {
    let code = r#"
def f() -> int:
    x = 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_float() {
    let code = r#"
def f() -> float:
    x = 3.14
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_str() {
    let code = r#"
def f() -> str:
    x = "hello"
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_bool() {
    let code = r#"
def f() -> bool:
    x = True
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_list_int() {
    let code = r#"
def f() -> list:
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_dict() {
    let code = r#"
def f() -> dict:
    return {"key": "value"}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_optional() {
    let code = r#"
from typing import Optional
def f(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_type_tuple() {
    let code = r#"
def f() -> tuple:
    return (1, "hello", True)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// BorrowStrategy decisions
// ============================================================================

#[test]
fn test_trace_borrow_immutable() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_borrow_mutable() {
    let code = r#"
def f(items: list):
    items.append(1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_borrow_readonly_iteration() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// LifetimeInfer decisions
// ============================================================================

#[test]
fn test_trace_lifetime_return_ref() {
    let code = r#"
def f(items: list) -> int:
    return items[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_lifetime_string_slice() {
    let code = r#"
def f(s: str) -> str:
    return s[1:3]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// MethodDispatch decisions
// ============================================================================

#[test]
fn test_trace_method_str_upper() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_method_list_append() {
    let code = r#"
def f() -> list:
    items = [1, 2]
    items.append(3)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_method_dict_get() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_method_str_split() {
    let code = r#"
def f(text: str) -> list:
    return text.split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_method_list_sort() {
    let code = r#"
def f(items: list) -> list:
    items.sort()
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// ImportResolve decisions
// ============================================================================

#[test]
fn test_trace_import_json() {
    let code = r#"
import json

def f(s: str) -> dict:
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_import_math() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_import_typing() {
    let code = r#"
from typing import List, Dict, Optional

def f(items: List[int]) -> Dict[str, int]:
    result = {}
    for item in items:
        result[str(item)] = item
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_import_collections() {
    let code = r#"
from collections import defaultdict

def f(items: list) -> dict:
    counts = defaultdict(int)
    for item in items:
        counts[item] += 1
    return dict(counts)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// ErrorHandling decisions
// ============================================================================

#[test]
fn test_trace_error_try_except() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_error_multiple_handlers() {
    let code = r#"
def f(d: dict, key: str) -> int:
    try:
        val = d[key]
        return int(val)
    except KeyError:
        return -1
    except ValueError:
        return -2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_error_division() {
    let code = r#"
def f(a: int, b: int) -> float:
    if b == 0:
        return 0.0
    return a / b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership decisions
// ============================================================================

#[test]
fn test_trace_ownership_transfer() {
    let code = r#"
def consume(items: list) -> int:
    return len(items)

def f() -> int:
    data = [1, 2, 3]
    return consume(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_ownership_clone() {
    let code = r#"
def f() -> list:
    x = [1, 2, 3]
    y = x.copy()
    return y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex combined decisions
// ============================================================================

#[test]
fn test_trace_combined_type_method() {
    let code = r#"
def process(text: str) -> list:
    words = text.strip().lower().split()
    result = []
    for word in words:
        if len(word) > 3:
            result.append(word)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_combined_dict_processing() {
    let code = r#"
def count_chars(text: str) -> dict:
    counts = {}
    for ch in text:
        if ch in counts:
            counts[ch] += 1
        else:
            counts[ch] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_combined_error_import() {
    let code = r#"
import json

def parse(s: str) -> dict:
    try:
        return json.loads(s)
    except:
        return {}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_combined_full_pipeline() {
    let code = r#"
from typing import List, Dict

def word_frequency(texts: List[str]) -> Dict[str, int]:
    counts = {}
    for text in texts:
        for word in text.lower().split():
            if word in counts:
                counts[word] += 1
            else:
                counts[word] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_trace_combined_class_method() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1

    def get_count(self) -> int:
        return self.count
"#;
    assert!(transpile_ok(code));
}

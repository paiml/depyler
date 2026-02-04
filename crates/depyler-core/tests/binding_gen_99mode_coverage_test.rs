//! Coverage tests for rust_gen/binding_gen.rs
//!
//! DEPYLER-99MODE-001: Targets binding_gen.rs (1,157 lines)
//! Covers: phantom struct generation, external library type binding,
//! module function stubs, type mapping for external APIs.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// External module import binding
// ============================================================================

#[test]
fn test_binding_gen_import_json() {
    let code = r#"
import json

def f(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_import_math() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_import_os() {
    let code = r#"
import os

def f() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_import_re() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// From-import binding
// ============================================================================

#[test]
fn test_binding_gen_from_import() {
    let code = r#"
from collections import defaultdict

def f() -> dict:
    d = defaultdict(int)
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_from_import_datetime() {
    let code = r#"
from datetime import datetime

def f() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method call binding
// ============================================================================

#[test]
fn test_binding_gen_method_call() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_chained_methods() {
    let code = r#"
import base64

def f(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple imports binding
// ============================================================================

#[test]
fn test_binding_gen_multiple_modules() {
    let code = r#"
import json
import math
import os

def f() -> str:
    data = {"pi": math.pi, "cwd": os.getcwd()}
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type mapping patterns
// ============================================================================

#[test]
fn test_binding_gen_type_int() {
    let code = "def f(x: int) -> int:\n    return x * 2\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_type_str() {
    let code = "def f(s: str) -> str:\n    return s.upper()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_type_list() {
    let code = "def f(items: list) -> int:\n    return len(items)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_type_dict() {
    let code = "def f(d: dict) -> int:\n    return len(d)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Class binding patterns
// ============================================================================

#[test]
fn test_binding_gen_class_definition() {
    let code = r#"
class MyClass:
    def __init__(self, value: int):
        self.value = value

    def get_value(self) -> int:
        return self.value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binding_gen_class_methods() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1

    def get(self) -> int:
        return self.count
"#;
    assert!(transpile_ok(code));
}

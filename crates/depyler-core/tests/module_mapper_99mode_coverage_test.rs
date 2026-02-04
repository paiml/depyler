//! Coverage tests for module_mapper.rs
//!
//! DEPYLER-99MODE-001: Targets module_mapper.rs (1,351 lines)
//! Covers: Python stdlib→Rust crate mapping, module attribute access,
//! item-level mappings, constructor pattern inference.

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
// Standard library imports
// ============================================================================

#[test]
fn test_module_mapper_import_json() {
    let code = r#"
import json

def f(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_os() {
    let code = r#"
import os

def f() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_sys() {
    let code = r#"
import sys

def f() -> list:
    return sys.argv
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_re() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\w+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_math() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_hashlib() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_base64() {
    let code = r#"
import base64

def f(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_import_datetime() {
    let code = r#"
import datetime

def f() -> str:
    return str(datetime.datetime.now())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// From-imports
// ============================================================================

#[test]
fn test_module_mapper_from_collections() {
    let code = r#"
from collections import defaultdict

def f() -> dict:
    d = defaultdict(int)
    d["a"] += 1
    return dict(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_from_pathlib() {
    let code = r#"
from pathlib import Path

def f(p: str) -> str:
    return str(Path(p))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module attribute access
// ============================================================================

#[test]
fn test_module_mapper_math_constants() {
    let code = r#"
import math

def f() -> float:
    return math.pi * 2.0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_math_functions() {
    let code = r#"
import math

def f(x: float, y: float) -> float:
    return math.sqrt(x * x + y * y)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple imports
// ============================================================================

#[test]
fn test_module_mapper_multiple_imports() {
    let code = r#"
import json
import math

def f(data: str) -> float:
    d = json.loads(data)
    return math.sqrt(float(len(data)))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_no_imports() {
    let code = r#"
def f(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration
// ============================================================================

#[test]
fn test_module_mapper_json_operations() {
    let code = r#"
import json

def f(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_mapper_re_operations() {
    let code = r#"
import re

def f(text: str, pattern: str) -> bool:
    return bool(re.match(pattern, text))
"#;
    assert!(transpile_ok(code));
}

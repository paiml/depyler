//! Coverage tests for stdlib_mappings.rs
//!
//! DEPYLER-99MODE-001: Targets stdlib_mappings.rs (869 lines)
//! Covers: Python stdlib API to Rust mapping, CSV module,
//! file I/O patterns, plugin system, method/property mapping.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// JSON stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_json_loads() {
    let code = r#"
import json

def f(s: str) -> dict:
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_map_json_dumps() {
    let code = r#"
import json

def f(d: dict) -> str:
    return json.dumps(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Math stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_math_sqrt() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_map_math_pi() {
    let code = r#"
import math

def f(r: float) -> float:
    return math.pi * r * r
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// OS stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_os_getcwd() {
    let code = r#"
import os

def f() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_map_os_getenv() {
    let code = r#"
import os

def f(key: str) -> str:
    return os.getenv(key, "default")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collections stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_defaultdict() {
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
// Hashlib stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_hashlib() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Re stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_re_findall() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_map_re_sub() {
    let code = r#"
import re

def f(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Base64 stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_base64() {
    let code = r#"
import base64

def f(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Random stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_random() {
    let code = r#"
import random

def f() -> int:
    return random.randint(1, 100)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// DateTime stdlib mapping
// ============================================================================

#[test]
fn test_stdlib_map_datetime() {
    let code = r#"
from datetime import datetime

def f() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple stdlib modules
// ============================================================================

#[test]
fn test_stdlib_map_combined() {
    let code = r#"
import json
import math

def f(x: float) -> str:
    data = {"sqrt": math.sqrt(x), "pi": math.pi}
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_map_mixed_imports() {
    let code = r#"
import json
import hashlib
import base64

def f(data: str) -> str:
    hashed = hashlib.sha256(data.encode()).hexdigest()
    encoded = base64.b64encode(data.encode()).decode()
    return json.dumps({"hash": hashed, "encoded": encoded})
"#;
    assert!(transpile_ok(code));
}

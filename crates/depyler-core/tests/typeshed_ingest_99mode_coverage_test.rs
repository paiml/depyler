//! Coverage tests for typeshed_ingest.rs
//!
//! DEPYLER-99MODE-001: Targets typeshed_ingest.rs (1,266 lines)
//! Covers: .pyi stub parsing, Python→Rust module mapping,
//! function signature extraction, type/crate/function mapping.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Module mapping - JSON
// ============================================================================

#[test]
fn test_typeshed_json_loads() {
    let code = r#"
import json

def parse(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_json_dumps() {
    let code = r#"
import json

def serialize(d: dict) -> str:
    return json.dumps(d)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module mapping - math
// ============================================================================

#[test]
fn test_typeshed_math_sqrt() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_math_sin_cos() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sin(x) + math.cos(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_math_floor_ceil() {
    let code = r#"
import math

def f(x: float) -> int:
    return int(math.floor(x)) + int(math.ceil(x))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_math_constants() {
    let code = r#"
import math

def circle_area(r: float) -> float:
    return math.pi * r * r
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module mapping - os
// ============================================================================

#[test]
fn test_typeshed_os_getcwd() {
    let code = r#"
import os

def cwd() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_os_getenv() {
    let code = r#"
import os

def env(key: str) -> str:
    return os.getenv(key, "default")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module mapping - re
// ============================================================================

#[test]
fn test_typeshed_re_findall() {
    let code = r#"
import re

def find_nums(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_re_sub() {
    let code = r#"
import re

def clean(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module mapping - collections
// ============================================================================

#[test]
fn test_typeshed_collections_defaultdict() {
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
// Module mapping - datetime
// ============================================================================

#[test]
fn test_typeshed_datetime_now() {
    let code = r#"
from datetime import datetime

def current_time() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module mapping - hashlib
// ============================================================================

#[test]
fn test_typeshed_hashlib_sha256() {
    let code = r#"
import hashlib

def hash_str(s: str) -> str:
    return hashlib.sha256(s.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module mapping - base64
// ============================================================================

#[test]
fn test_typeshed_base64_encode() {
    let code = r#"
import base64

def encode(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type annotation mapping
// ============================================================================

#[test]
fn test_typeshed_typing_list() {
    let code = r#"
from typing import List

def f(items: List[int]) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_typing_dict() {
    let code = r#"
from typing import Dict

def f(d: Dict[str, int]) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_typing_optional() {
    let code = r#"
from typing import Optional

def f(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple module usage
// ============================================================================

#[test]
fn test_typeshed_multiple_modules() {
    let code = r#"
import math
import os
import json

def info() -> str:
    data = {"pi": math.pi, "cwd": os.getcwd()}
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typeshed_mixed_imports() {
    let code = r#"
import json
from collections import defaultdict
from datetime import datetime

def process(json_str: str) -> dict:
    data = json.loads(json_str)
    return data
"#;
    assert!(transpile_ok(code));
}

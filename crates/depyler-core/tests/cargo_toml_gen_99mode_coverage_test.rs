//! Coverage tests for cargo_toml_gen.rs
//!
//! DEPYLER-99MODE-001: Targets cargo_toml_gen.rs (1,570 lines)
//! Covers: dependency detection, Cargo.toml generation, import-to-crate mapping,
//! feature detection, library vs binary crate type selection.

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
// JSON dependency detection (serde_json + serde)
// ============================================================================

#[test]
fn test_cargo_dep_json_loads() {
    let code = r#"
import json

def parse(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_json_dumps() {
    let code = r#"
import json

def serialize(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Regex dependency detection
// ============================================================================

#[test]
fn test_cargo_dep_regex() {
    let code = r#"
import re

def find_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_regex_search() {
    let code = r#"
import re

def has_email(text: str) -> bool:
    return re.search(r"@", text) is not None
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// DateTime dependency detection (chrono)
// ============================================================================

#[test]
fn test_cargo_dep_datetime() {
    let code = r#"
from datetime import datetime

def now_str() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Hashing dependency detection (sha2, md5, etc.)
// ============================================================================

#[test]
fn test_cargo_dep_hashlib_sha256() {
    let code = r#"
import hashlib

def hash_data(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hash(data: str) -> str:
    return hashlib.md5(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Base64 dependency detection
// ============================================================================

#[test]
fn test_cargo_dep_base64() {
    let code = r#"
import base64

def encode(data: str) -> str:
    return base64.b64encode(data.encode()).decode()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Random dependency detection (rand)
// ============================================================================

#[test]
fn test_cargo_dep_random() {
    let code = r#"
import random

def roll_dice() -> int:
    return random.randint(1, 6)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collections dependency detection
// ============================================================================

#[test]
fn test_cargo_dep_collections_defaultdict() {
    let code = r#"
from collections import defaultdict

def count_items(items: list) -> dict:
    counts = defaultdict(int)
    for item in items:
        counts[item] += 1
    return dict(counts)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Argparse dependency detection (clap)
// ============================================================================

#[test]
fn test_cargo_dep_argparse() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="test")
    parser.add_argument("--name", type=str, default="world")
    args = parser.parse_args()
    print(f"Hello, {args.name}")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Async dependency detection (tokio)
// ============================================================================

#[test]
fn test_cargo_dep_asyncio() {
    let code = r#"
import asyncio

async def fetch() -> str:
    return "data"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Math module dependency
// ============================================================================

#[test]
fn test_cargo_dep_math() {
    let code = r#"
import math

def hypotenuse(a: float, b: float) -> float:
    return math.sqrt(a * a + b * b)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// OS/sys module dependency
// ============================================================================

#[test]
fn test_cargo_dep_os() {
    let code = r#"
import os

def cwd() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple dependencies combined
// ============================================================================

#[test]
fn test_cargo_dep_multiple_imports() {
    let code = r#"
import json
import re
import hashlib

def process(text: str) -> str:
    numbers = re.findall(r"\d+", text)
    data = json.dumps(numbers)
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_from_imports() {
    let code = r#"
from collections import defaultdict
from datetime import datetime

def log_events(events: list) -> dict:
    counts = defaultdict(int)
    for event in events:
        counts[event] += 1
    return dict(counts)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// No external dependencies
// ============================================================================

#[test]
fn test_cargo_dep_no_imports() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_builtins_only() {
    let code = r#"
def process(items: list) -> int:
    return sum(items) + len(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration patterns
// ============================================================================

#[test]
fn test_cargo_dep_cli_app() {
    let code = r#"
import argparse
import json

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=str)
    args = parser.parse_args()
    data = json.loads(args.input)
    print(json.dumps(data))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_data_processing() {
    let code = r#"
import json
from collections import defaultdict

def analyze(json_str: str) -> dict:
    data = json.loads(json_str)
    counts = defaultdict(int)
    for item in data:
        counts[item] += 1
    return dict(counts)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cargo_dep_class_with_imports() {
    let code = r#"
import json

class DataStore:
    def __init__(self):
        self.data = {}

    def load(self, json_str: str):
        self.data = json.loads(json_str)

    def dump(self) -> str:
        return json.dumps(self.data)
"#;
    assert!(transpile_ok(code));
}

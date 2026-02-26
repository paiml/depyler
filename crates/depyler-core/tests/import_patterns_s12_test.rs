//! Session 12 Batch 51: Import patterns and module-level code
//!
//! Targets cold paths in rust_gen for:
//! - Various import patterns (from X import Y)
//! - Module-level code patterns
//! - Type annotation imports
//! - stdlib module imports
//! - Star imports
//! - Aliased imports

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== From imports =====

#[test]
fn test_s12_b51_from_typing_import() {
    let code = r#"
from typing import List, Dict, Optional, Tuple

def process(items: List[int]) -> Optional[int]:
    if not items:
        return None
    return items[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b51_from_collections_import() {
    let code = r#"
from collections import defaultdict

def group_by_length(words: list) -> dict:
    groups = {}
    for word in words:
        n = len(word)
        if n not in groups:
            groups[n] = []
        groups[n].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_length"), "Got: {}", result);
}

#[test]
fn test_s12_b51_from_math_import() {
    let code = r#"
from math import sqrt, floor, ceil

def hypotenuse(a: float, b: float) -> float:
    return sqrt(a * a + b * b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn hypotenuse"), "Got: {}", result);
}

#[test]
fn test_s12_b51_import_os() {
    let code = r#"
import os

def get_env(name: str, default: str) -> str:
    value = os.environ.get(name, default)
    return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_env"), "Got: {}", result);
}

#[test]
fn test_s12_b51_import_json() {
    let code = r#"
import json

def parse_json(data: str) -> dict:
    return json.loads(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_json"), "Got: {}", result);
}

#[test]
fn test_s12_b51_import_sys() {
    let code = r#"
import sys

def get_args() -> list:
    return sys.argv
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_args"), "Got: {}", result);
}

#[test]
fn test_s12_b51_import_re() {
    let code = r##"
import re

def find_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"##;
    let result = transpile(code);
    assert!(result.contains("fn find_numbers"), "Got: {}", result);
}

#[test]
fn test_s12_b51_import_hashlib() {
    let code = r#"
import hashlib

def sha256_hex(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sha256_hex"), "Got: {}", result);
}

// ===== Multiple imports in same module =====

#[test]
fn test_s12_b51_multiple_imports() {
    let code = r#"
from typing import List, Optional
import math

def safe_sqrt(x: float) -> Optional[float]:
    if x < 0.0:
        return None
    return math.sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_sqrt"), "Got: {}", result);
}

// ===== Aliased import =====

#[test]
fn test_s12_b51_import_os_path() {
    let code = r#"
from os.path import basename

def get_base(path: str) -> str:
    return basename(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_base"), "Got: {}", result);
}

// ===== Complex module with imports + constants + functions =====

#[test]
fn test_s12_b51_full_module() {
    let code = r##"
from typing import List, Dict

VERSION = "1.0"
MAX_SIZE = 1000

def create_index(items: List[str]) -> Dict[str, int]:
    index = {}
    for i, item in enumerate(items):
        index[item] = i
    return index

def lookup(index: Dict[str, int], key: str) -> int:
    return index.get(key, -1)
"##;
    let result = transpile(code);
    assert!(result.contains("fn create_index"), "Got: {}", result);
    assert!(result.contains("fn lookup"), "Got: {}", result);
}

// ===== From imports with specific items =====

#[test]
fn test_s12_b51_from_os_path() {
    let code = r#"
from os.path import join, exists, dirname

def ensure_dir(path: str) -> str:
    parent = dirname(path)
    return parent
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_dir"), "Got: {}", result);
}

#[test]
fn test_s12_b51_from_functools() {
    let code = r#"
from functools import reduce

def product(items: list) -> int:
    result = 1
    for item in items:
        result *= item
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn product"), "Got: {}", result);
}

#[test]
fn test_s12_b51_from_itertools() {
    let code = r#"
from itertools import chain

def flatten(lists: list) -> list:
    result = []
    for sublist in lists:
        for item in sublist:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

// ===== Type annotation patterns =====

#[test]
fn test_s12_b51_type_annotations_complex() {
    let code = r#"
from typing import Dict, List, Tuple

def process_records(records: List[Dict[str, int]]) -> Dict[str, int]:
    result = {}
    for record in records:
        for key, value in record.items():
            if key in result:
                result[key] += value
            else:
                result[key] = value
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_records"), "Got: {}", result);
}

#[test]
fn test_s12_b51_tuple_return_annotation() {
    let code = r#"
from typing import Tuple

def min_max(items: list) -> Tuple[int, int]:
    lo = items[0]
    hi = items[0]
    for item in items:
        if item < lo:
            lo = item
        if item > hi:
            hi = item
    return (lo, hi)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max"), "Got: {}", result);
}

// ===== Module with docstring =====

#[test]
fn test_s12_b51_module_docstring() {
    let code = r##"
"""Module for string utilities."""

def capitalize_words(s: str) -> str:
    result = []
    for word in s.split(" "):
        if word:
            result.append(word[0].upper() + word[1:])
        else:
            result.append(word)
    return " ".join(result)
"##;
    let result = transpile(code);
    assert!(result.contains("fn capitalize_words"), "Got: {}", result);
}

// ===== Functions with type aliases =====

#[test]
fn test_s12_b51_type_alias_like() {
    let code = r#"
from typing import List

Matrix = List[List[float]]

def zeros(rows: int, cols: int) -> list:
    result = []
    for i in range(rows):
        row = []
        for j in range(cols):
            row.append(0.0)
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn zeros"), "Got: {}", result);
}

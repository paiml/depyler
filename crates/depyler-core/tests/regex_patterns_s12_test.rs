//! Session 12 Batch 63: Regex module and stdlib pattern cold paths
//!
//! Targets stdlib codegen cold paths:
//! - re module patterns (match, search, findall, sub, split)
//! - json module patterns (loads, dumps)
//! - os module patterns (path ops, environ)
//! - math module extended functions
//! - collections module patterns

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== re module patterns =====

#[test]
fn test_s12_b63_re_match() {
    let code = r##"
import re

def is_email(text: str) -> bool:
    pattern = r"^[\w.+-]+@[\w-]+\.[\w.]+$"
    return re.match(pattern, text) is not None
"##;
    let result = transpile(code);
    assert!(result.contains("fn is_email"), "Got: {}", result);
}

#[test]
fn test_s12_b63_re_search() {
    let code = r##"
import re

def has_number(text: str) -> bool:
    return re.search(r"\d+", text) is not None
"##;
    let result = transpile(code);
    assert!(result.contains("fn has_number"), "Got: {}", result);
}

#[test]
fn test_s12_b63_re_findall() {
    let code = r##"
import re

def extract_words(text: str) -> list:
    return re.findall(r"\w+", text)
"##;
    let result = transpile(code);
    assert!(result.contains("fn extract_words"), "Got: {}", result);
}

#[test]
fn test_s12_b63_re_sub() {
    let code = r##"
import re

def remove_digits(text: str) -> str:
    return re.sub(r"\d+", "", text)
"##;
    let result = transpile(code);
    assert!(result.contains("fn remove_digits"), "Got: {}", result);
}

#[test]
fn test_s12_b63_re_split() {
    let code = r##"
import re

def split_words(text: str) -> list:
    return re.split(r"\s+", text)
"##;
    let result = transpile(code);
    assert!(result.contains("fn split_words"), "Got: {}", result);
}

// ===== json module =====

#[test]
fn test_s12_b63_json_loads() {
    let code = r#"
import json

def parse_json(data: str) -> dict:
    return json.loads(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_json"), "Got: {}", result);
}

#[test]
fn test_s12_b63_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_json"), "Got: {}", result);
}

// ===== os module =====

#[test]
fn test_s12_b63_os_environ() {
    let code = r#"
import os

def get_env(key: str, default: str) -> str:
    return os.environ.get(key, default)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_env"), "Got: {}", result);
}

#[test]
fn test_s12_b63_os_path_join() {
    let code = r#"
from os.path import join

def make_path(base: str, name: str) -> str:
    return join(base, name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_path"), "Got: {}", result);
}

#[test]
fn test_s12_b63_os_path_exists() {
    let code = r#"
from os.path import exists

def file_exists(path: str) -> bool:
    return exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn file_exists"), "Got: {}", result);
}

// ===== math module extended =====

#[test]
fn test_s12_b63_math_pow() {
    let code = r#"
import math

def power(x: float, y: float) -> float:
    return math.pow(x, y)
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_b63_math_abs() {
    let code = r#"
import math

def math_fabs(x: float) -> float:
    return math.fabs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn math_fabs"), "Got: {}", result);
}

#[test]
fn test_s12_b63_math_exp() {
    let code = r#"
import math

def exponential(x: float) -> float:
    return math.exp(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn exponential"), "Got: {}", result);
}

#[test]
fn test_s12_b63_math_pi() {
    let code = r#"
import math

def circle_area(radius: float) -> float:
    return math.pi * radius ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn circle_area"), "Got: {}", result);
}

// ===== hashlib module =====

#[test]
fn test_s12_b63_hashlib_sha256() {
    let code = r#"
import hashlib

def hash_string(s: str) -> str:
    return hashlib.sha256(s.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn hash_string"), "Got: {}", result);
}

#[test]
fn test_s12_b63_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hash(s: str) -> str:
    return hashlib.md5(s.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn md5_hash"), "Got: {}", result);
}

// ===== Complex stdlib combinations =====

#[test]
fn test_s12_b63_stdlib_combo() {
    let code = r##"
import re
import json

def extract_config(text: str) -> dict:
    match = re.search(r"\{.*\}", text)
    if match:
        return json.loads(match.group(0))
    return {}
"##;
    let result = transpile(code);
    assert!(result.contains("fn extract_config"), "Got: {}", result);
}

#[test]
fn test_s12_b63_path_operations() {
    let code = r#"
from os.path import dirname, basename, splitext

def file_info(path: str) -> dict:
    directory = dirname(path)
    name = basename(path)
    base, ext = splitext(name)
    return {"dir": directory, "name": name, "base": base, "ext": ext}
"#;
    let result = transpile(code);
    assert!(result.contains("fn file_info"), "Got: {}", result);
}

#[test]
fn test_s12_b63_math_complex_formula() {
    let code = r#"
import math

def gaussian(x: float, mu: float, sigma: float) -> float:
    coeff = 1.0 / (sigma * math.sqrt(2.0 * math.pi))
    exponent = -0.5 * ((x - mu) / sigma) ** 2
    return coeff * math.exp(exponent)
"#;
    let result = transpile(code);
    assert!(result.contains("fn gaussian"), "Got: {}", result);
}

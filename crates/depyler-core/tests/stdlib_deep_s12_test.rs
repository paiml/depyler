//! Session 12 Batch 82: Standard library module deep cold paths
//!
//! Targets stdlib method generation cold paths for os, sys, json,
//! math, re, hashlib, and other stdlib modules.

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

// ===== Math module =====

#[test]
fn test_s12_b82_math_sqrt() {
    let code = r#"
import math

def hypotenuse(a: float, b: float) -> float:
    return math.sqrt(a * a + b * b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn hypotenuse"), "Got: {}", result);
}

#[test]
fn test_s12_b82_math_ceil_floor() {
    let code = r#"
import math

def round_up(x: float) -> int:
    return math.ceil(x)

def round_down(x: float) -> int:
    return math.floor(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_up"), "Got: {}", result);
    assert!(result.contains("fn round_down"), "Got: {}", result);
}

#[test]
fn test_s12_b82_math_log() {
    let code = r#"
import math

def log_base(x: float, base: float) -> float:
    return math.log(x) / math.log(base)
"#;
    let result = transpile(code);
    assert!(result.contains("fn log_base"), "Got: {}", result);
}

#[test]
fn test_s12_b82_math_trig() {
    let code = r#"
import math

def sin_cos(angle: float) -> tuple:
    return (math.sin(angle), math.cos(angle))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sin_cos"), "Got: {}", result);
}

#[test]
fn test_s12_b82_math_pow() {
    let code = r#"
import math

def safe_power(base: float, exp: float) -> float:
    return math.pow(base, exp)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_power"), "Got: {}", result);
}

#[test]
fn test_s12_b82_math_fabs() {
    let code = r#"
import math

def absolute(x: float) -> float:
    return math.fabs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"), "Got: {}", result);
}

// ===== Re module =====

#[test]
fn test_s12_b82_re_compile_match() {
    let code = r#"
import re

def is_valid_email(email: str) -> bool:
    pattern = re.compile(r"[^@]+@[^@]+\.[^@]+")
    return pattern.match(email) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_email"), "Got: {}", result);
}

#[test]
fn test_s12_b82_re_findall() {
    let code = r#"
import re

def extract_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn extract_numbers"), "Got: {}", result);
}

#[test]
fn test_s12_b82_re_sub() {
    let code = r#"
import re

def clean_whitespace(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_whitespace"), "Got: {}", result);
}

#[test]
fn test_s12_b82_re_split() {
    let code = r#"
import re

def split_on_punct(text: str) -> list:
    return re.split(r"[,;.!?]+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_on_punct"), "Got: {}", result);
}

// ===== JSON module =====

#[test]
fn test_s12_b82_json_loads() {
    let code = r#"
import json

def parse_json(text: str) -> dict:
    return json.loads(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_json"), "Got: {}", result);
}

#[test]
fn test_s12_b82_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_json"), "Got: {}", result);
}

// ===== OS module =====

#[test]
fn test_s12_b82_os_path_join() {
    let code = r#"
import os

def make_path(directory: str, filename: str) -> str:
    return os.path.join(directory, filename)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_path"), "Got: {}", result);
}

#[test]
fn test_s12_b82_os_path_exists() {
    let code = r#"
import os

def file_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn file_exists"), "Got: {}", result);
}

#[test]
fn test_s12_b82_os_environ() {
    let code = r#"
import os

def get_env(key: str) -> str:
    return os.environ.get(key, "")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_env"), "Got: {}", result);
}

// ===== Hashlib module =====

#[test]
fn test_s12_b82_hashlib_sha256() {
    let code = r#"
import hashlib

def hash_text(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn hash_text"), "Got: {}", result);
}

#[test]
fn test_s12_b82_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hash(text: str) -> str:
    return hashlib.md5(text.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn md5_hash"), "Got: {}", result);
}

// ===== Complex stdlib combinations =====

#[test]
fn test_s12_b82_re_with_dict() {
    let code = r##"
import re

def extract_key_values(text: str) -> dict:
    result = {}
    for match in re.findall(r"(\w+)=(\w+)", text):
        result[match[0]] = match[1]
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn extract_key_values"), "Got: {}", result);
}

#[test]
fn test_s12_b82_math_statistics() {
    let code = r#"
import math

def geometric_mean(values: list) -> float:
    if not values:
        return 0.0
    product = 1.0
    for v in values:
        product *= v
    return math.pow(product, 1.0 / len(values))
"#;
    let result = transpile(code);
    assert!(result.contains("fn geometric_mean"), "Got: {}", result);
}

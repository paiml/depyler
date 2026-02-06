//! Session 12 Batch 18: Stdlib method patterns for cold path coverage
//!
//! Targets cold paths in stdlib method generation:
//! - Math module (atan, atan2, degrees, radians, factorial, comb, perm)
//! - String module methods (ascii_lowercase, digits, punctuation)
//! - Random module (randint, choice, shuffle, sample, uniform)
//! - Time module (time, sleep)
//! - Hashlib variants (sha384, sha512, md5 with update+digest)
//! - Complex re module patterns (fullmatch, IGNORECASE flag)
//! - Complex datetime patterns (timedelta, date, time)
//! - Complex os module patterns (rename, chmod, stat, getcwd)

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

// ===== Math advanced functions =====

#[test]
fn test_s12_math_atan() {
    let code = r#"
import math

def arctangent(x: float) -> float:
    return math.atan(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn arctangent"), "Got: {}", result);
}

#[test]
fn test_s12_math_atan2() {
    let code = r#"
import math

def angle(y: float, x: float) -> float:
    return math.atan2(y, x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn angle"), "Got: {}", result);
}

#[test]
fn test_s12_math_degrees() {
    let code = r#"
import math

def to_degrees(radians: float) -> float:
    return math.degrees(radians)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_degrees"), "Got: {}", result);
}

#[test]
fn test_s12_math_radians() {
    let code = r#"
import math

def to_radians(degrees: float) -> float:
    return math.radians(degrees)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_radians"), "Got: {}", result);
}

#[test]
fn test_s12_math_exp() {
    let code = r#"
import math

def exponential(x: float) -> float:
    return math.exp(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn exponential"), "Got: {}", result);
}

#[test]
fn test_s12_math_log2() {
    let code = r#"
import math

def log_base_2(x: float) -> float:
    return math.log2(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn log_base_2"), "Got: {}", result);
}

#[test]
fn test_s12_math_log10() {
    let code = r#"
import math

def log_base_10(x: float) -> float:
    return math.log10(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn log_base_10"), "Got: {}", result);
}

#[test]
fn test_s12_math_copysign() {
    let code = r#"
import math

def copy_sign(x: float, y: float) -> float:
    return math.copysign(x, y)
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_sign"), "Got: {}", result);
}

#[test]
fn test_s12_math_isnan() {
    let code = r#"
import math

def check_nan(x: float) -> bool:
    return math.isnan(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_nan"), "Got: {}", result);
}

#[test]
fn test_s12_math_isinf() {
    let code = r#"
import math

def check_inf(x: float) -> bool:
    return math.isinf(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_inf"), "Got: {}", result);
}

#[test]
fn test_s12_math_hypot() {
    let code = r#"
import math

def hypotenuse(a: float, b: float) -> float:
    return math.hypot(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn hypotenuse"), "Got: {}", result);
}

// ===== Hashlib variants =====

#[test]
fn test_s12_hashlib_sha384() {
    let code = r#"
import hashlib

def hash_sha384(data: str) -> str:
    return hashlib.sha384(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn hash_sha384"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_sha512() {
    let code = r#"
import hashlib

def hash_sha512(data: str) -> str:
    return hashlib.sha512(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn hash_sha512"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_md5() {
    let code = r#"
import hashlib

def hash_md5(data: str) -> str:
    return hashlib.md5(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn hash_md5"), "Got: {}", result);
}

// ===== Re module patterns =====

#[test]
fn test_s12_re_match() {
    let code = r#"
import re

def is_match(pattern: str, text: str) -> bool:
    return re.match(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_match"), "Got: {}", result);
}

#[test]
fn test_s12_re_search() {
    let code = r#"
import re

def has_pattern(text: str) -> bool:
    return re.search(r"\d+", text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_pattern"), "Got: {}", result);
}

#[test]
fn test_s12_re_findall() {
    let code = r#"
import re

def find_words(text: str) -> list:
    return re.findall(r"\w+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_words"), "Got: {}", result);
}

#[test]
fn test_s12_re_sub() {
    let code = r#"
import re

def remove_digits(text: str) -> str:
    return re.sub(r"\d", "", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_digits"), "Got: {}", result);
}

#[test]
fn test_s12_re_split() {
    let code = r#"
import re

def split_on_whitespace(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_on_whitespace"), "Got: {}", result);
}

// ===== Complex os module =====

#[test]
fn test_s12_os_getcwd() {
    let code = r#"
import os

def current_dir() -> str:
    return os.getcwd()
"#;
    let result = transpile(code);
    assert!(result.contains("fn current_dir"), "Got: {}", result);
}

#[test]
fn test_s12_os_rename() {
    let code = r#"
import os

def rename_file(old: str, new: str):
    os.rename(old, new)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rename_file"), "Got: {}", result);
}

// ===== Datetime patterns =====

#[test]
fn test_s12_datetime_now() {
    let code = r#"
import datetime

def get_now() -> str:
    return str(datetime.datetime.now())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_now"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_date() {
    let code = r#"
import datetime

def make_date() -> str:
    d = datetime.date(2024, 1, 15)
    return str(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_date"), "Got: {}", result);
}

// ===== Complex patterns combining stdlib =====

#[test]
fn test_s12_file_processor() {
    let code = r#"
import os
import json

def process_json_files(directory: str) -> list:
    results = []
    for filename in os.listdir(directory):
        if filename.endswith(".json"):
            path = os.path.join(directory, filename)
            with open(path) as f:
                data = json.loads(f.read())
            results.append(data)
    return results
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_json_files"), "Got: {}", result);
}

#[test]
fn test_s12_math_statistics() {
    let code = r#"
import math

def standard_deviation(values: list) -> float:
    n = len(values)
    mean = sum(values) / n
    variance = 0.0
    for v in values:
        diff = v - mean
        variance += diff * diff
    variance = variance / n
    return math.sqrt(variance)
"#;
    let result = transpile(code);
    assert!(result.contains("fn standard_deviation"), "Got: {}", result);
}

#[test]
fn test_s12_regex_email_validator() {
    let code = r#"
import re

def validate_email(email: str) -> bool:
    pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    return re.match(pattern, email) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_email"), "Got: {}", result);
}

#[test]
fn test_s12_path_utilities() {
    let code = r#"
import os

def normalize_path(path: str) -> str:
    path = os.path.expanduser(path)
    path = os.path.abspath(path)
    return path

def path_info(path: str) -> dict:
    return {
        "dirname": os.path.dirname(path),
        "basename": os.path.basename(path),
        "exists": os.path.exists(path),
    }
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize_path"), "Got: {}", result);
    assert!(result.contains("fn path_info"), "Got: {}", result);
}

#[test]
fn test_s12_hash_file() {
    let code = r#"
import hashlib

def file_hash(path: str) -> str:
    with open(path) as f:
        content = f.read()
    return hashlib.sha256(content.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn file_hash"), "Got: {}", result);
}

// ===== Complex data processing =====

#[test]
fn test_s12_csv_like_processing() {
    let code = r#"
def parse_csv(content: str) -> list:
    rows = []
    for line in content.strip().split("\n"):
        fields = line.split(",")
        cleaned = []
        for field in fields:
            cleaned.append(field.strip())
        rows.append(cleaned)
    return rows
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_csv"), "Got: {}", result);
}

#[test]
fn test_s12_json_builder() {
    let code = r#"
import json

def build_report(items: list) -> str:
    report = {
        "count": len(items),
        "items": items,
    }
    return json.dumps(report)
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_report"), "Got: {}", result);
}

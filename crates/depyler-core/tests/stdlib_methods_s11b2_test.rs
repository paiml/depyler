//! Session 11 Batch 2: Stdlib method coverage
//!
//! Targets uncovered stdlib conversion paths in:
//! - direct_rules_convert.rs: os.path methods, os.environ, os file operations
//! - direct_rules_convert.rs: datetime, pathlib, collections
//! - expr_gen.rs: hashlib, base64, json, csv, uuid, textwrap
//! - expr_gen.rs: fnmatch, shlex, binascii, urllib

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

// ===== os.path operations =====

#[test]
fn test_s11b2_os_path_join() {
    let code = r#"
import os

def full_path(base: str, name: str) -> str:
    return os.path.join(base, name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn full_path"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_exists() {
    let code = r#"
import os

def check_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_exists"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_isfile() {
    let code = r#"
import os

def is_file(path: str) -> bool:
    return os.path.isfile(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_file"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_isdir() {
    let code = r#"
import os

def is_dir(path: str) -> bool:
    return os.path.isdir(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_dir"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_basename() {
    let code = r#"
import os

def get_name(path: str) -> str:
    return os.path.basename(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_name"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_dirname() {
    let code = r#"
import os

def get_dir(path: str) -> str:
    return os.path.dirname(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_dir"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_splitext() {
    let code = r#"
import os

def get_ext(path: str) -> tuple:
    return os.path.splitext(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_ext"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_path_expanduser() {
    let code = r#"
import os

def expand(path: str) -> str:
    return os.path.expanduser(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand"), "Got: {}", result);
}

// ===== os file operations =====

#[test]
fn test_s11b2_os_makedirs() {
    let code = r#"
import os

def make_dir(path: str):
    os.makedirs(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dir"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_remove() {
    let code = r#"
import os

def delete(path: str):
    os.remove(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn delete"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_rename() {
    let code = r#"
import os

def rename(old: str, new: str):
    os.rename(old, new)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rename"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_listdir() {
    let code = r#"
import os

def list_files(path: str) -> list:
    return os.listdir(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn list_files"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_getcwd() {
    let code = r#"
import os

def current_dir() -> str:
    return os.getcwd()
"#;
    let result = transpile(code);
    assert!(result.contains("fn current_dir"), "Got: {}", result);
}

#[test]
fn test_s11b2_os_getenv() {
    let code = r#"
import os

def get_var(name: str) -> str:
    return os.getenv(name, "default")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_var"), "Got: {}", result);
}

// ===== os.environ =====

#[test]
fn test_s11b2_os_environ_get() {
    let code = r#"
import os

def env_var(key: str) -> str:
    return os.environ.get(key, "")
"#;
    let result = transpile(code);
    assert!(result.contains("fn env_var"), "Got: {}", result);
}

// ===== collections =====

#[test]
fn test_s11b2_deque_basic() {
    let code = r#"
from collections import deque

def make_deque() -> list:
    d = deque()
    d.append(1)
    d.append(2)
    d.appendleft(0)
    return list(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_deque"), "Got: {}", result);
}

#[test]
fn test_s11b2_counter_basic() {
    let code = r#"
from collections import Counter

def count_items(items: list) -> dict:
    return dict(Counter(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_items"), "Got: {}", result);
}

#[test]
fn test_s11b2_defaultdict() {
    let code = r#"
from collections import defaultdict

def group_items(pairs: list) -> dict:
    d = defaultdict(list)
    return dict(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_items"), "Got: {}", result);
}

// ===== json =====

#[test]
fn test_s11b2_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_json"), "Got: {}", result);
}

#[test]
fn test_s11b2_json_loads() {
    let code = r#"
import json

def from_json(text: str) -> dict:
    return json.loads(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn from_json"), "Got: {}", result);
}

// ===== hashlib =====

#[test]
fn test_s11b2_hashlib_sha256() {
    let code = r#"
import hashlib

def sha256_hash(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sha256_hash"), "Got: {}", result);
}

#[test]
fn test_s11b2_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hash(data: str) -> str:
    return hashlib.md5(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn md5_hash"), "Got: {}", result);
}

// ===== base64 =====

#[test]
fn test_s11b2_base64_encode() {
    let code = r#"
import base64

def encode(data: bytes) -> bytes:
    return base64.b64encode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode"), "Got: {}", result);
}

#[test]
fn test_s11b2_base64_decode() {
    let code = r#"
import base64

def decode(data: bytes) -> bytes:
    return base64.b64decode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn decode"), "Got: {}", result);
}

// ===== datetime =====

#[test]
fn test_s11b2_datetime_now() {
    let code = r#"
from datetime import datetime

def current_time():
    return datetime.now()
"#;
    let result = transpile(code);
    assert!(result.contains("fn current_time"), "Got: {}", result);
}

#[test]
fn test_s11b2_datetime_constructor() {
    let code = r#"
from datetime import datetime

def make_date(year: int, month: int, day: int):
    return datetime(year, month, day)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_date"), "Got: {}", result);
}

#[test]
fn test_s11b2_date_today() {
    let code = r#"
from datetime import date

def today_date():
    return date.today()
"#;
    let result = transpile(code);
    assert!(result.contains("fn today_date"), "Got: {}", result);
}

#[test]
fn test_s11b2_datetime_strftime() {
    let code = r#"
from datetime import datetime

def format_date(dt) -> str:
    return dt.strftime("%Y-%m-%d")
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_date"), "Got: {}", result);
}

// ===== pathlib =====

#[test]
fn test_s11b2_pathlib_path() {
    let code = r#"
from pathlib import Path

def make_path(s: str):
    return Path(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_path"), "Got: {}", result);
}

#[test]
fn test_s11b2_pathlib_exists() {
    let code = r#"
from pathlib import Path

def path_exists(s: str) -> bool:
    return Path(s).exists()
"#;
    let result = transpile(code);
    assert!(result.contains("fn path_exists"), "Got: {}", result);
}

#[test]
fn test_s11b2_pathlib_read_text() {
    let code = r#"
from pathlib import Path

def read_file(path: str) -> str:
    return Path(path).read_text()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s11b2_pathlib_write_text() {
    let code = r#"
from pathlib import Path

def write_file(path: str, content: str):
    Path(path).write_text(content)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

// ===== regex =====

#[test]
fn test_s11b2_re_compile_match() {
    let code = r#"
import re

def starts_with_digit(text: str) -> bool:
    pattern = re.compile(r"\d")
    return pattern.match(text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn starts_with_digit"), "Got: {}", result);
}

#[test]
fn test_s11b2_re_escape() {
    let code = r#"
import re

def escape_pattern(text: str) -> str:
    return re.escape(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn escape_pattern"), "Got: {}", result);
}

#[test]
fn test_s11b2_re_fullmatch() {
    let code = r#"
import re

def exact_match(pattern: str, text: str) -> bool:
    return re.fullmatch(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn exact_match"), "Got: {}", result);
}

// ===== type conversions =====

#[test]
fn test_s11b2_int_from_string() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"), "Got: {}", result);
}

#[test]
fn test_s11b2_float_from_string() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_float"), "Got: {}", result);
}

#[test]
fn test_s11b2_str_from_int() {
    let code = r#"
def to_string(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_string"), "Got: {}", result);
}

#[test]
fn test_s11b2_bool_from_value() {
    let code = r#"
def to_bool(n: int) -> bool:
    return bool(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Got: {}", result);
}

// ===== builtins =====

#[test]
fn test_s11b2_chr_ord() {
    let code = r#"
def char_at(n: int) -> str:
    return chr(n)

def code_of(c: str) -> int:
    return ord(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_at"), "Got: {}", result);
    assert!(result.contains("fn code_of"), "Got: {}", result);
}

#[test]
fn test_s11b2_hex_bin_oct() {
    let code = r#"
def to_hex(n: int) -> str:
    return hex(n)

def to_bin(n: int) -> str:
    return bin(n)

def to_oct(n: int) -> str:
    return oct(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_hex"), "Got: {}", result);
    assert!(result.contains("fn to_bin"), "Got: {}", result);
    assert!(result.contains("fn to_oct"), "Got: {}", result);
}

#[test]
fn test_s11b2_isinstance() {
    let code = r#"
def is_int(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_int"), "Got: {}", result);
}

#[test]
fn test_s11b2_print_sep() {
    let code = r#"
def print_csv(a: str, b: str, c: str):
    print(a, b, c, sep=",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_csv"), "Got: {}", result);
}

#[test]
fn test_s11b2_print_end() {
    let code = r#"
def print_no_newline(msg: str):
    print(msg, end="")
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_no_newline"), "Got: {}", result);
}

// ===== map/filter/reduce =====

#[test]
fn test_s11b2_map_lambda() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"), "Got: {}", result);
}

#[test]
fn test_s11b2_filter_lambda() {
    let code = r#"
def positives_only(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn positives_only"), "Got: {}", result);
}

// ===== threading =====

#[test]
fn test_s11b2_threading_lock() {
    let code = r#"
import threading

def make_lock():
    return threading.Lock()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_lock"), "Got: {}", result);
}

// ===== asyncio =====

#[test]
fn test_s11b2_asyncio_sleep() {
    let code = r#"
import asyncio

async def wait(secs: int):
    await asyncio.sleep(secs)
"#;
    let result = transpile(code);
    assert!(result.contains("wait"), "Got: {}", result);
}

// ===== subprocess =====

#[test]
fn test_s11b2_subprocess_run() {
    let code = r#"
import subprocess

def run_cmd(cmd: str):
    subprocess.run(cmd, shell=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn run_cmd"), "Got: {}", result);
}

// ===== math module extended =====

#[test]
fn test_s11b2_math_gcd() {
    let code = r#"
import math

def greatest_common(a: int, b: int) -> int:
    return math.gcd(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greatest_common"), "Got: {}", result);
}

#[test]
fn test_s11b2_math_lcm() {
    let code = r#"
import math

def least_common(a: int, b: int) -> int:
    return math.lcm(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn least_common"), "Got: {}", result);
}

#[test]
fn test_s11b2_math_isfinite() {
    let code = r#"
import math

def check_finite(x: float) -> bool:
    return math.isfinite(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_finite"), "Got: {}", result);
}

#[test]
fn test_s11b2_math_isinf() {
    let code = r#"
import math

def check_inf(x: float) -> bool:
    return math.isinf(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_inf"), "Got: {}", result);
}

#[test]
fn test_s11b2_math_isnan() {
    let code = r#"
import math

def check_nan(x: float) -> bool:
    return math.isnan(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_nan"), "Got: {}", result);
}

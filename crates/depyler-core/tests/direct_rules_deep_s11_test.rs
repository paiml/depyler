//! Session 11: Deep coverage tests for direct_rules_convert.rs
//!
//! Targets the #3 coverage bottleneck (61% covered, 3396 missed regions):
//! - Collections module (deque, Counter, OrderedDict, defaultdict)
//! - OS path operations
//! - OS file system operations
//! - OS environ methods
//! - Re module operations
//! - Base64 operations
//! - Threading/asyncio patterns
//! - Slice operations with step
//! - Dict methods (fromkeys, update)
//! - String rarely-used methods

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

// ============================================================================
// Collections module
// ============================================================================

#[test]
fn test_s11_direct_collections_deque() {
    let code = r#"
from collections import deque

def make_deque() -> deque:
    d = deque([1, 2, 3])
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_deque"),
        "Should transpile deque constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_deque_appendleft() {
    let code = r#"
from collections import deque

def prepend(d: deque, val: int) -> deque:
    d.appendleft(val)
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn prepend"),
        "Should transpile deque appendleft. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_deque_popleft() {
    let code = r#"
from collections import deque

def pop_front(d: deque) -> int:
    return d.popleft()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pop_front"),
        "Should transpile deque popleft. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_collections_counter() {
    let code = r#"
from collections import Counter

def count_chars(s: str) -> dict:
    return Counter(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_chars"),
        "Should transpile Counter. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_collections_ordered_dict() {
    let code = r#"
from collections import OrderedDict

def ordered() -> dict:
    d = OrderedDict()
    d["a"] = 1
    d["b"] = 2
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn ordered"),
        "Should transpile OrderedDict. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_collections_defaultdict() {
    let code = r#"
from collections import defaultdict

def grouped() -> dict:
    d = defaultdict(list)
    d["a"].append(1)
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn grouped"),
        "Should transpile defaultdict. Got: {}",
        result
    );
}

// ============================================================================
// OS path operations
// ============================================================================

#[test]
fn test_s11_direct_os_path_join() {
    let code = r#"
import os

def build_path(base: str, name: str) -> str:
    return os.path.join(base, name)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn build_path"),
        "Should transpile os.path.join. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_exists() {
    let code = r#"
import os

def file_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn file_exists"),
        "Should transpile os.path.exists. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_isfile() {
    let code = r#"
import os

def is_file(path: str) -> bool:
    return os.path.isfile(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_file"),
        "Should transpile os.path.isfile. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_isdir() {
    let code = r#"
import os

def is_dir(path: str) -> bool:
    return os.path.isdir(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_dir"),
        "Should transpile os.path.isdir. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_basename() {
    let code = r#"
import os

def get_name(path: str) -> str:
    return os.path.basename(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_name"),
        "Should transpile os.path.basename. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_dirname() {
    let code = r#"
import os

def get_dir(path: str) -> str:
    return os.path.dirname(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_dir"),
        "Should transpile os.path.dirname. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_splitext() {
    let code = r#"
import os

def get_ext(path: str) -> tuple:
    return os.path.splitext(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_ext"),
        "Should transpile os.path.splitext. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_path_expanduser() {
    let code = r#"
import os

def home_path(rel: str) -> str:
    return os.path.expanduser(rel)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn home_path"),
        "Should transpile os.path.expanduser. Got: {}",
        result
    );
}

// ============================================================================
// OS file system operations
// ============================================================================

#[test]
fn test_s11_direct_os_makedirs() {
    let code = r#"
import os

def ensure_dir(path: str):
    os.makedirs(path, exist_ok=True)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn ensure_dir"),
        "Should transpile os.makedirs. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_remove() {
    let code = r#"
import os

def delete_file(path: str):
    os.remove(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn delete_file"),
        "Should transpile os.remove. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_rename() {
    let code = r#"
import os

def rename_file(old: str, new: str):
    os.rename(old, new)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn rename_file"),
        "Should transpile os.rename. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_listdir() {
    let code = r#"
import os

def list_files(path: str) -> list:
    return os.listdir(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn list_files"),
        "Should transpile os.listdir. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_getcwd() {
    let code = r#"
import os

def current_dir() -> str:
    return os.getcwd()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn current_dir"),
        "Should transpile os.getcwd. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_os_getenv() {
    let code = r#"
import os

def get_home() -> str:
    return os.getenv("HOME", "/tmp")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_home"),
        "Should transpile os.getenv with default. Got: {}",
        result
    );
}

// ============================================================================
// OS environ methods
// ============================================================================

#[test]
fn test_s11_direct_environ_get() {
    let code = r#"
import os

def env_get(key: str) -> str:
    return os.environ.get(key, "")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn env_get"),
        "Should transpile environ.get. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_environ_keys() {
    let code = r#"
import os

def env_keys() -> list:
    return list(os.environ.keys())
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn env_keys"),
        "Should transpile environ.keys. Got: {}",
        result
    );
}

// ============================================================================
// Regular expressions
// ============================================================================

#[test]
fn test_s11_direct_re_compile() {
    let code = r#"
import re

def make_pattern(pat: str):
    return re.compile(pat)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_pattern"),
        "Should transpile re.compile. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_match() {
    let code = r#"
import re

def matches(pattern: str, text: str) -> bool:
    return re.match(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn matches"),
        "Should transpile re.match. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_search() {
    let code = r#"
import re

def find_pattern(pattern: str, text: str) -> bool:
    return re.search(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_pattern"),
        "Should transpile re.search. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_findall() {
    let code = r#"
import re

def find_all(pattern: str, text: str) -> list:
    return re.findall(pattern, text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_all"),
        "Should transpile re.findall. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_sub() {
    let code = r#"
import re

def replace_all(pattern: str, repl: str, text: str) -> str:
    return re.sub(pattern, repl, text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn replace_all"),
        "Should transpile re.sub. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_split() {
    let code = r#"
import re

def split_on(pattern: str, text: str) -> list:
    return re.split(pattern, text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_on"),
        "Should transpile re.split. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_escape() {
    let code = r#"
import re

def safe_pattern(text: str) -> str:
    return re.escape(text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_pattern"),
        "Should transpile re.escape. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_re_fullmatch() {
    let code = r#"
import re

def full_match(pattern: str, text: str) -> bool:
    return re.fullmatch(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn full_match"),
        "Should transpile re.fullmatch. Got: {}",
        result
    );
}

// ============================================================================
// Base64 operations
// ============================================================================

#[test]
fn test_s11_direct_base64_encode() {
    let code = r#"
import base64

def encode_bytes(data: bytes) -> bytes:
    return base64.b64encode(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn encode_bytes"),
        "Should transpile b64encode. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_base64_decode() {
    let code = r#"
import base64

def decode_bytes(data: bytes) -> bytes:
    return base64.b64decode(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn decode_bytes"),
        "Should transpile b64decode. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_base64_urlsafe_encode() {
    let code = r#"
import base64

def url_encode(data: bytes) -> bytes:
    return base64.urlsafe_b64encode(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn url_encode"),
        "Should transpile urlsafe_b64encode. Got: {}",
        result
    );
}

// ============================================================================
// Threading patterns
// ============================================================================

#[test]
fn test_s11_direct_threading_lock() {
    let code = r#"
import threading

def make_lock():
    lock = threading.Lock()
    return lock
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_lock"),
        "Should transpile threading.Lock. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_threading_semaphore() {
    let code = r#"
import threading

def make_sem():
    sem = threading.Semaphore(5)
    return sem
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_sem"),
        "Should transpile threading.Semaphore. Got: {}",
        result
    );
}

// ============================================================================
// Asyncio patterns
// ============================================================================

#[test]
fn test_s11_direct_asyncio_sleep() {
    let code = r#"
import asyncio

async def delay(seconds: float):
    await asyncio.sleep(seconds)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn delay") || result.contains("async fn delay"),
        "Should transpile asyncio.sleep. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_asyncio_queue() {
    let code = r#"
import asyncio

async def make_queue():
    q = asyncio.Queue()
    return q
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_queue"),
        "Should transpile asyncio.Queue. Got: {}",
        result
    );
}

// ============================================================================
// JSON operations
// ============================================================================

#[test]
fn test_s11_direct_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_json"),
        "Should transpile json.dumps. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_json_loads() {
    let code = r#"
import json

def from_json(text: str) -> dict:
    return json.loads(text)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn from_json"),
        "Should transpile json.loads. Got: {}",
        result
    );
}

// ============================================================================
// Hashlib operations
// ============================================================================

#[test]
fn test_s11_direct_hashlib_sha256() {
    let code = r#"
import hashlib

def hash_data(data: str) -> str:
    h = hashlib.sha256(data.encode())
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn hash_data"),
        "Should transpile hashlib.sha256. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hash(data: str) -> str:
    h = hashlib.md5(data.encode())
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn md5_hash"),
        "Should transpile hashlib.md5. Got: {}",
        result
    );
}

// ============================================================================
// Datetime operations
// ============================================================================

#[test]
fn test_s11_direct_datetime_now() {
    let code = r#"
from datetime import datetime

def current_time():
    return datetime.now()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn current_time"),
        "Should transpile datetime.now. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_datetime_constructor() {
    let code = r#"
from datetime import datetime

def make_date():
    return datetime(2024, 1, 15)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_date"),
        "Should transpile datetime constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_date_today() {
    let code = r#"
from datetime import date

def today():
    return date.today()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn today"),
        "Should transpile date.today. Got: {}",
        result
    );
}

// ============================================================================
// Pathlib operations
// ============================================================================

#[test]
fn test_s11_direct_pathlib_path() {
    let code = r#"
from pathlib import Path

def make_path(s: str):
    return Path(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_path"),
        "Should transpile Path constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_path_exists() {
    let code = r#"
from pathlib import Path

def check_path(p: str) -> bool:
    return Path(p).exists()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_path"),
        "Should transpile Path.exists. Got: {}",
        result
    );
}

// ============================================================================
// CSV operations
// ============================================================================

#[test]
fn test_s11_direct_csv_reader() {
    let code = r#"
import csv

def read_csv(filename: str) -> list:
    results: list = []
    with open(filename) as f:
        reader = csv.reader(f)
        for row in reader:
            results.append(row)
    return results
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_csv"),
        "Should transpile csv.reader. Got: {}",
        result
    );
}

// ============================================================================
// Builtin type conversions
// ============================================================================

#[test]
fn test_s11_direct_int_from_string() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn parse_int") && result.contains("parse"),
        "Should transpile int() from string. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_float_from_string() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn parse_float"),
        "Should transpile float() from string. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_bool_from_int() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn to_bool"),
        "Should transpile bool() from int. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_bytes_constructor() {
    let code = r#"
def make_bytes(n: int) -> bytes:
    return bytes(n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_bytes"),
        "Should transpile bytes() constructor. Got: {}",
        result
    );
}

// ============================================================================
// Print variations
// ============================================================================

#[test]
fn test_s11_direct_print_multiple_args() {
    let code = r#"
def print_multi(a: str, b: int, c: float):
    print(a, b, c)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn print_multi"),
        "Should transpile print with multiple args. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_print_sep() {
    let code = r#"
def print_csv(a: str, b: str, c: str):
    print(a, b, c, sep=",")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn print_csv"),
        "Should transpile print with sep. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_print_end() {
    let code = r#"
def print_no_newline(s: str):
    print(s, end="")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn print_no_newline"),
        "Should transpile print with end. Got: {}",
        result
    );
}

// ============================================================================
// Enumerate with index
// ============================================================================

#[test]
fn test_s11_direct_enumerate_loop() {
    let code = r#"
def indexed(items: list) -> list:
    result: list = []
    for i, item in enumerate(items):
        result.append(f"{i}: {item}")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn indexed"),
        "Should transpile enumerate loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_enumerate_start() {
    let code = r#"
def indexed_from(items: list) -> list:
    result: list = []
    for i, item in enumerate(items, 1):
        result.append(f"{i}: {item}")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn indexed_from"),
        "Should transpile enumerate with start. Got: {}",
        result
    );
}

// ============================================================================
// Zip operations
// ============================================================================

#[test]
fn test_s11_direct_zip_two_lists() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result: list = []
    for x, y in zip(a, b):
        result.append((x, y))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pair_up"),
        "Should transpile zip. Got: {}",
        result
    );
}

// ============================================================================
// Sorted with key
// ============================================================================

#[test]
fn test_s11_direct_sorted_basic() {
    let code = r#"
def sort_list(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_list"),
        "Should transpile sorted. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_sorted_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_desc"),
        "Should transpile sorted reverse. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_sorted_key() {
    let code = r#"
def sort_by_len(items: list) -> list:
    return sorted(items, key=len)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_len"),
        "Should transpile sorted with key. Got: {}",
        result
    );
}

// ============================================================================
// Map/Filter/Reduce
// ============================================================================

#[test]
fn test_s11_direct_map_lambda() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn double_all"),
        "Should transpile map with lambda. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_filter_lambda() {
    let code = r#"
def positives(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn positives"),
        "Should transpile filter with lambda. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_functools_reduce() {
    let code = r#"
from functools import reduce

def product(items: list) -> int:
    return reduce(lambda a, b: a * b, items, 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn product"),
        "Should transpile functools.reduce. Got: {}",
        result
    );
}

// ============================================================================
// String formatting edge cases
// ============================================================================

#[test]
fn test_s11_direct_str_format_named() {
    let code = r#"
def greet(name: str, age: int) -> str:
    return "Hello {name}, you are {age}".format(name=name, age=age)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn greet"),
        "Should transpile str.format with named args. Got: {}",
        result
    );
}

// ============================================================================
// Complex nested operations
// ============================================================================

#[test]
fn test_s11_direct_nested_dict_access() {
    let code = r#"
def deep_get(d: dict, k1: str, k2: str) -> int:
    return d[k1][k2]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn deep_get"),
        "Should transpile nested dict access. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_dict_in_check() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn has_key"),
        "Should transpile dict in check. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_string_in_check() {
    let code = r#"
def contains(text: str, sub: str) -> bool:
    return sub in text
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn contains"),
        "Should transpile string in check. Got: {}",
        result
    );
}

// ============================================================================
// Min/Max edge cases
// ============================================================================

#[test]
fn test_s11_direct_min_two_args() {
    let code = r#"
def smaller(a: int, b: int) -> int:
    return min(a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn smaller"),
        "Should transpile min with two args. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_max_two_args() {
    let code = r#"
def larger(a: int, b: int) -> int:
    return max(a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn larger"),
        "Should transpile max with two args. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_min_list() {
    let code = r#"
def smallest(items: list) -> int:
    return min(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn smallest"),
        "Should transpile min with list. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_max_list() {
    let code = r#"
def biggest(items: list) -> int:
    return max(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn biggest"),
        "Should transpile max with list. Got: {}",
        result
    );
}

// ============================================================================
// Abs/Sum/Round
// ============================================================================

#[test]
fn test_s11_direct_abs_int() {
    let code = r#"
def magnitude(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn magnitude") && result.contains("abs"),
        "Should transpile abs(). Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_sum_list() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn total"),
        "Should transpile sum(). Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_round_float() {
    let code = r#"
def round_val(x: float) -> int:
    return round(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn round_val"),
        "Should transpile round(). Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_round_with_digits() {
    let code = r#"
def round_digits(x: float, n: int) -> float:
    return round(x, n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn round_digits"),
        "Should transpile round with digits. Got: {}",
        result
    );
}

// ============================================================================
// Input/Open patterns
// ============================================================================

#[test]
fn test_s11_direct_input_basic() {
    let code = r#"
def get_name() -> str:
    return input("Enter name: ")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_name"),
        "Should transpile input(). Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_file"),
        "Should transpile open/read. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct_open_write() {
    let code = r#"
def write_file(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn write_file"),
        "Should transpile open/write. Got: {}",
        result
    );
}

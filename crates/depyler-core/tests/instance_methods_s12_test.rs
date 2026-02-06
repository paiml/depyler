//! Session 12: Targeted tests for expr_gen_instance_methods.rs uncovered paths
//!
//! Targets:
//! - File I/O methods (read, write, readlines, readline)
//! - Path methods (pathlib)
//! - Datetime methods
//! - Dict advanced methods (setdefault, popitem, update)
//! - List advanced methods (insert, index, remove)
//! - String advanced methods (center, ljust, rjust, maketrans, translate)

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

// ===== Dict advanced methods =====

#[test]
fn test_s12_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str, default: int) -> int:
    return d.setdefault(key, default)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"), "Got: {}", result);
}

#[test]
fn test_s12_dict_update() {
    let code = r#"
def merge(d1: dict, d2: dict) -> dict:
    d1.update(d2)
    return d1
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

#[test]
fn test_s12_dict_pop_default() {
    let code = r#"
def safe_pop(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_pop"), "Got: {}", result);
}

#[test]
fn test_s12_dict_pop_no_default() {
    let code = r#"
def pop_key(d: dict, key: str) -> int:
    return d.pop(key)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_key"), "Got: {}", result);
}

#[test]
fn test_s12_dict_get_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_dict_get_no_default() {
    let code = r#"
def maybe_get(d: dict, key: str) -> int:
    return d.get(key)
"#;
    let result = transpile(code);
    assert!(result.contains("fn maybe_get"), "Got: {}", result);
}

#[test]
fn test_s12_dict_clear() {
    let code = r#"
def empty_dict(d: dict):
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty_dict"), "Got: {}", result);
}

#[test]
fn test_s12_dict_copy() {
    let code = r#"
def clone_dict(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_dict"), "Got: {}", result);
}

// ===== List advanced methods =====

#[test]
fn test_s12_list_insert() {
    let code = r#"
def insert_at(items: list, idx: int, val: int):
    items.insert(idx, val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_at"), "Got: {}", result);
}

#[test]
fn test_s12_list_index() {
    let code = r#"
def find_index(items: list, val: int) -> int:
    return items.index(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_index"), "Got: {}", result);
}

#[test]
fn test_s12_list_remove() {
    let code = r#"
def remove_item(items: list, val: int):
    items.remove(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_item"), "Got: {}", result);
}

#[test]
fn test_s12_list_count() {
    let code = r#"
def count_item(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_item"), "Got: {}", result);
}

#[test]
fn test_s12_list_extend() {
    let code = r#"
def concat(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn concat"), "Got: {}", result);
}

#[test]
fn test_s12_list_sort() {
    let code = r#"
def sort_inplace(items: list):
    items.sort()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_inplace"), "Got: {}", result);
}

#[test]
fn test_s12_list_reverse() {
    let code = r#"
def reverse_inplace(items: list):
    items.reverse()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_inplace"), "Got: {}", result);
}

// ===== String advanced methods =====

#[test]
fn test_s12_string_center() {
    let code = r#"
def center_text(s: str, width: int) -> str:
    return s.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12_string_ljust() {
    let code = r#"
def left_justify(s: str, width: int) -> str:
    return s.ljust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_justify"), "Got: {}", result);
}

#[test]
fn test_s12_string_rjust() {
    let code = r#"
def right_justify(s: str, width: int) -> str:
    return s.rjust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_justify"), "Got: {}", result);
}

#[test]
fn test_s12_string_count() {
    let code = r#"
def count_char(s: str, ch: str) -> int:
    return s.count(ch)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_char"), "Got: {}", result);
}

#[test]
fn test_s12_string_find() {
    let code = r#"
def find_pos(s: str, sub: str) -> int:
    return s.find(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pos"), "Got: {}", result);
}

#[test]
fn test_s12_string_rfind() {
    let code = r#"
def rfind_pos(s: str, sub: str) -> int:
    return s.rfind(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rfind_pos"), "Got: {}", result);
}

#[test]
fn test_s12_string_title() {
    let code = r#"
def titlecase(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn titlecase"), "Got: {}", result);
}

#[test]
fn test_s12_string_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

#[test]
fn test_s12_string_isdigit() {
    let code = r#"
def all_digits(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_digits"), "Got: {}", result);
}

#[test]
fn test_s12_string_isalnum() {
    let code = r#"
def all_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_alnum"), "Got: {}", result);
}

#[test]
fn test_s12_string_isspace() {
    let code = r#"
def all_space(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_space"), "Got: {}", result);
}

#[test]
fn test_s12_string_isupper() {
    let code = r#"
def all_upper(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_upper"), "Got: {}", result);
}

#[test]
fn test_s12_string_islower() {
    let code = r#"
def all_lower(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_lower"), "Got: {}", result);
}

#[test]
fn test_s12_string_lstrip() {
    let code = r#"
def left_strip(s: str) -> str:
    return s.lstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_strip"), "Got: {}", result);
}

#[test]
fn test_s12_string_rstrip() {
    let code = r#"
def right_strip(s: str) -> str:
    return s.rstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_strip"), "Got: {}", result);
}

#[test]
fn test_s12_string_replace() {
    let code = r#"
def fix_spaces(s: str) -> str:
    return s.replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn fix_spaces"), "Got: {}", result);
}

// ===== File I/O patterns =====

#[test]
fn test_s12_with_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_write() {
    let code = r#"
def write_file(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_readlines() {
    let code = r#"
def read_lines(path: str) -> list:
    with open(path, "r") as f:
        return f.readlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_lines"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_readline() {
    let code = r#"
def first_line(path: str) -> str:
    with open(path, "r") as f:
        return f.readline()
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_line"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_append() {
    let code = r#"
def append_file(path: str, line: str):
    with open(path, "a") as f:
        f.write(line)
"#;
    let result = transpile(code);
    assert!(result.contains("fn append_file"), "Got: {}", result);
}

// ===== Pathlib methods =====

#[test]
fn test_s12_pathlib_exists() {
    let code = r#"
from pathlib import Path

def check_path(p: str) -> bool:
    return Path(p).exists()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_path"), "Got: {}", result);
}

#[test]
fn test_s12_pathlib_read_text() {
    let code = r#"
from pathlib import Path

def read(p: str) -> str:
    return Path(p).read_text()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read"), "Got: {}", result);
}

#[test]
fn test_s12_pathlib_write_text() {
    let code = r#"
from pathlib import Path

def write(p: str, content: str):
    Path(p).write_text(content)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write"), "Got: {}", result);
}

#[test]
fn test_s12_pathlib_stem() {
    let code = r#"
from pathlib import Path

def get_stem(p: str) -> str:
    return Path(p).stem
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_stem"), "Got: {}", result);
}

#[test]
fn test_s12_pathlib_suffix() {
    let code = r#"
from pathlib import Path

def get_suffix(p: str) -> str:
    return Path(p).suffix
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_suffix"), "Got: {}", result);
}

#[test]
fn test_s12_pathlib_parent() {
    let code = r#"
from pathlib import Path

def get_parent(p: str) -> str:
    return str(Path(p).parent)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_parent"), "Got: {}", result);
}

#[test]
fn test_s12_pathlib_name() {
    let code = r#"
from pathlib import Path

def get_name(p: str) -> str:
    return Path(p).name
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_name"), "Got: {}", result);
}

// ===== Datetime patterns =====

#[test]
fn test_s12_datetime_now() {
    let code = r#"
from datetime import datetime

def current_time() -> str:
    return str(datetime.now())
"#;
    let result = transpile(code);
    assert!(result.contains("fn current_time"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_strftime() {
    let code = r#"
from datetime import datetime

def format_date(dt) -> str:
    return dt.strftime("%Y-%m-%d")
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_date"), "Got: {}", result);
}

// ===== Regex patterns =====

#[test]
fn test_s12_re_search() {
    let code = r#"
import re

def find_number(text: str) -> bool:
    return re.search(r"\d+", text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_number"), "Got: {}", result);
}

#[test]
fn test_s12_re_findall() {
    let code = r#"
import re

def all_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_numbers"), "Got: {}", result);
}

#[test]
fn test_s12_re_sub() {
    let code = r#"
import re

def remove_digits(text: str) -> str:
    return re.sub(r"\d+", "", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_digits"), "Got: {}", result);
}

#[test]
fn test_s12_re_split() {
    let code = r#"
import re

def split_words(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_words"), "Got: {}", result);
}

#[test]
fn test_s12_re_match() {
    let code = r#"
import re

def starts_with_digit(text: str) -> bool:
    return re.match(r"\d", text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn starts_with_digit"), "Got: {}", result);
}

// ===== collections patterns =====

#[test]
fn test_s12_counter() {
    let code = r#"
from collections import Counter

def count_chars(s: str) -> dict:
    return dict(Counter(s))
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_chars"), "Got: {}", result);
}

#[test]
fn test_s12_defaultdict() {
    let code = r#"
from collections import defaultdict

def group_items(pairs: list) -> dict:
    d = defaultdict(list)
    for k, v in pairs:
        d[k].append(v)
    return dict(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_items"), "Got: {}", result);
}

#[test]
fn test_s12_deque_operations() {
    let code = r#"
from collections import deque

def sliding_window(items: list, size: int) -> list:
    d = deque(maxlen=size)
    for item in items:
        d.append(item)
    return list(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sliding_window"), "Got: {}", result);
}

// ===== json patterns =====

#[test]
fn test_s12_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_json"), "Got: {}", result);
}

#[test]
fn test_s12_json_loads() {
    let code = r#"
import json

def from_json(s: str) -> dict:
    return json.loads(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn from_json"), "Got: {}", result);
}

// ===== hashlib =====

#[test]
fn test_s12_hashlib_sha256() {
    let code = r#"
import hashlib

def sha256_hex(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sha256_hex"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hex(data: str) -> str:
    return hashlib.md5(data.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn md5_hex"), "Got: {}", result);
}

// ===== base64 =====

#[test]
fn test_s12_base64_encode() {
    let code = r#"
import base64

def encode(data: bytes) -> str:
    return base64.b64encode(data).decode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode"), "Got: {}", result);
}

#[test]
fn test_s12_base64_decode() {
    let code = r#"
import base64

def decode(data: str) -> bytes:
    return base64.b64decode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn decode"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_with_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self.radius ** 2

    @property
    def circumference(self) -> float:
        return 2 * 3.14159 * self.radius
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_dunder_len() {
    let code = r#"
class Container:
    def __init__(self):
        self.items = []

    def add(self, item: int):
        self.items.append(item)

    def __len__(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("Container"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_str_repr() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

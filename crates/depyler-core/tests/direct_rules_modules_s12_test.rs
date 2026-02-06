//! Session 12 Batch 14: Module-level patterns in direct_rules_convert.rs
//!
//! Targets the coldest paths in the biggest coverage gap file:
//! - os.path methods (expanduser, realpath, relpath, splitext, isfile, isdir, isabs, abspath)
//! - sys module (exit, argv, platform, version, path)
//! - itertools patterns (chain, product, combinations, permutations)
//! - functools patterns (reduce, partial, lru_cache)
//! - csv module (reader, writer, DictReader)
//! - typing module patterns (Optional, Union, List, Dict, Tuple)
//! - io module (StringIO, BytesIO)
//! - pathlib patterns (Path, PurePath)
//! - subprocess patterns (run, check_output)
//! - copy module (copy, deepcopy)

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

// ===== os.path methods =====

#[test]
fn test_s12_os_path_splitext() {
    let code = r#"
import os

def get_ext(path: str) -> str:
    name, ext = os.path.splitext(path)
    return ext
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_ext"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_isfile() {
    let code = r#"
import os

def is_file(path: str) -> bool:
    return os.path.isfile(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_file"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_isdir() {
    let code = r#"
import os

def is_dir(path: str) -> bool:
    return os.path.isdir(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_dir"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_abspath() {
    let code = r#"
import os

def absolute(path: str) -> str:
    return os.path.abspath(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_expanduser() {
    let code = r#"
import os

def expand_home(path: str) -> str:
    return os.path.expanduser(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand_home"), "Got: {}", result);
}

#[test]
fn test_s12_os_listdir() {
    let code = r#"
import os

def list_files(directory: str) -> list:
    return os.listdir(directory)
"#;
    let result = transpile(code);
    assert!(result.contains("fn list_files"), "Got: {}", result);
}

#[test]
fn test_s12_os_makedirs() {
    let code = r#"
import os

def ensure_dir(path: str):
    os.makedirs(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_dir"), "Got: {}", result);
}

#[test]
fn test_s12_os_remove() {
    let code = r#"
import os

def delete_file(path: str):
    os.remove(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn delete_file"), "Got: {}", result);
}

// ===== sys module =====

#[test]
fn test_s12_sys_exit() {
    let code = r#"
import sys

def bail(code: int):
    sys.exit(code)
"#;
    let result = transpile(code);
    assert!(result.contains("fn bail"), "Got: {}", result);
}

#[test]
fn test_s12_sys_argv() {
    let code = r#"
import sys

def get_args() -> list:
    return sys.argv
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_args"), "Got: {}", result);
}

// ===== functools module =====

#[test]
fn test_s12_functools_reduce() {
    let code = r#"
from functools import reduce

def product(items: list) -> int:
    return reduce(lambda x, y: x * y, items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn product"), "Got: {}", result);
}

// ===== typing module patterns =====

#[test]
fn test_s12_typing_optional() {
    let code = r#"
from typing import Optional

def find_item(items: list, target: int) -> Optional[int]:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_item"), "Got: {}", result);
}

#[test]
fn test_s12_typing_list_annotation() {
    let code = r#"
from typing import List

def double_all(items: List[int]) -> List[int]:
    return [x * 2 for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"), "Got: {}", result);
}

#[test]
fn test_s12_typing_dict_annotation() {
    let code = r#"
from typing import Dict

def count_words(words: List[str]) -> Dict[str, int]:
    result = {}
    for w in words:
        if w in result:
            result[w] += 1
        else:
            result[w] = 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_words"), "Got: {}", result);
}

#[test]
fn test_s12_typing_tuple_annotation() {
    let code = r#"
from typing import Tuple

def swap_pair(a: int, b: int) -> Tuple[int, int]:
    return (b, a)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_pair"), "Got: {}", result);
}

// ===== Complex module interaction patterns =====

#[test]
fn test_s12_os_walk_pattern() {
    let code = r#"
import os

def count_files(directory: str) -> int:
    count = 0
    for root, dirs, files in os.walk(directory):
        count += len(files)
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_files"), "Got: {}", result);
}

#[test]
fn test_s12_os_environ_get() {
    let code = r#"
import os

def get_env(key: str) -> str:
    return os.environ.get(key, "")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_env"), "Got: {}", result);
}

// ===== String module patterns =====

#[test]
fn test_s12_string_ascii_letters() {
    let code = r#"
import string

def is_alpha_only(s: str) -> bool:
    for c in s:
        if c not in string.ascii_letters:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alpha_only"), "Got: {}", result);
}

// ===== Complex type conversion patterns =====

#[test]
fn test_s12_int_with_base_16() {
    let code = r#"
def parse_hex(s: str) -> int:
    return int(s, 16)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_hex"), "Got: {}", result);
}

#[test]
fn test_s12_int_with_base_2() {
    let code = r#"
def parse_binary(s: str) -> int:
    return int(s, 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_binary"), "Got: {}", result);
}

#[test]
fn test_s12_int_with_base_8() {
    let code = r#"
def parse_octal(s: str) -> int:
    return int(s, 8)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_octal"), "Got: {}", result);
}

// ===== Repr/hex/oct/bin builtins =====

#[test]
fn test_s12_hex_builtin() {
    let code = r#"
def to_hex(n: int) -> str:
    return hex(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_hex"), "Got: {}", result);
}

#[test]
fn test_s12_oct_builtin() {
    let code = r#"
def to_octal(n: int) -> str:
    return oct(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_octal"), "Got: {}", result);
}

#[test]
fn test_s12_bin_builtin() {
    let code = r#"
def to_binary(n: int) -> str:
    return bin(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_binary"), "Got: {}", result);
}

#[test]
fn test_s12_ord_builtin() {
    let code = r#"
def char_code(c: str) -> int:
    return ord(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_code"), "Got: {}", result);
}

#[test]
fn test_s12_chr_builtin() {
    let code = r#"
def code_to_char(n: int) -> str:
    return chr(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn code_to_char"), "Got: {}", result);
}

// ===== isinstance/type patterns =====

#[test]
fn test_s12_isinstance_check() {
    let code = r#"
def is_integer(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_integer"), "Got: {}", result);
}

#[test]
fn test_s12_type_check() {
    let code = r#"
def get_type_name(x) -> str:
    return type(x).__name__
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_type_name"), "Got: {}", result);
}

// ===== Complex comprehension patterns =====

#[test]
fn test_s12_dict_comp_from_range() {
    let code = r#"
def make_squares(n: int) -> dict:
    return {i: i * i for i in range(n)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_squares"), "Got: {}", result);
}

#[test]
fn test_s12_list_comp_with_method() {
    let code = r#"
def upper_words(words: list) -> list:
    return [w.upper() for w in words]
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_words"), "Got: {}", result);
}

#[test]
fn test_s12_list_comp_nested_condition() {
    let code = r#"
def divisible_by(items: list, d: int) -> list:
    return [x for x in items if x % d == 0 and x > 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn divisible_by"), "Got: {}", result);
}

// ===== Complex algorithm with multiple features =====

#[test]
fn test_s12_word_count_algorithm() {
    let code = r#"
def word_count(text: str) -> dict:
    words = text.lower().split()
    counts = {}
    for word in words:
        word = word.strip(".,!?;:")
        if word:
            if word in counts:
                counts[word] += 1
            else:
                counts[word] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_count"), "Got: {}", result);
}

#[test]
fn test_s12_caesar_cipher() {
    let code = r#"
def caesar_encrypt(text: str, shift: int) -> str:
    result = ""
    for c in text:
        if c.isalpha():
            base = ord("a") if c.islower() else ord("A")
            shifted = (ord(c) - base + shift) % 26 + base
            result += chr(shifted)
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn caesar_encrypt"), "Got: {}", result);
}

#[test]
fn test_s12_matrix_transpose() {
    let code = r#"
def transpose(matrix: list) -> list:
    if not matrix:
        return []
    rows = len(matrix)
    cols = len(matrix[0])
    result = []
    for j in range(cols):
        row = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transpose"), "Got: {}", result);
}

#[test]
fn test_s12_flatten_nested_list() {
    let code = r#"
def flatten(nested: list) -> list:
    result = []
    for item in nested:
        if isinstance(item, list):
            for sub in item:
                result.append(sub)
        else:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

// ===== Complex error handling with module interaction =====

#[test]
fn test_s12_safe_file_read() {
    let code = r#"
def safe_read(path: str) -> str:
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        return ""
    except PermissionError:
        return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_read"), "Got: {}", result);
}

#[test]
fn test_s12_safe_json_parse() {
    let code = r#"
import json

def safe_parse(s: str) -> dict:
    try:
        return json.loads(s)
    except json.JSONDecodeError:
        return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"), "Got: {}", result);
}

// ===== Complex class with module usage =====

#[test]
fn test_s12_config_class() {
    let code = r#"
import json

class Config:
    def __init__(self, path: str):
        self.path = path
        self.data = {}

    def load(self) -> dict:
        with open(self.path) as f:
            self.data = json.loads(f.read())
        return self.data

    def get(self, key: str) -> str:
        return self.data.get(key, "")
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

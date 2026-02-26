//! Session 12 Batch 35: Builtin constructor and path function cold paths
//!
//! Targets zero-coverage functions in direct_rules_convert.rs:
//! - bytes(), bytearray(), tuple(), dict(), set(), frozenset() constructors
//! - os.path functions (splitext, basename, dirname, exists, isfile, isdir)
//! - open() with different modes
//! - Dynamic function calls
//! - Set operations via operators
//! - Await expressions

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

// ===== Builtin constructors =====

#[test]
fn test_s12_b35_bytes_empty() {
    let code = r#"
def make_bytes() -> bytes:
    return bytes()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_bytes"), "Got: {}", result);
}

#[test]
fn test_s12_b35_bytearray_empty() {
    let code = r#"
def make_ba() -> bytearray:
    return bytearray()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_ba"), "Got: {}", result);
}

#[test]
fn test_s12_b35_bytearray_size() {
    let code = r#"
def make_ba_sized(n: int) -> bytearray:
    return bytearray(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_ba_sized"), "Got: {}", result);
}

#[test]
fn test_s12_b35_tuple_empty() {
    let code = r#"
def make_tuple():
    return tuple()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_tuple"), "Got: {}", result);
}

#[test]
fn test_s12_b35_tuple_from_list() {
    let code = r#"
def list_to_tuple(items: list):
    return tuple(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn list_to_tuple"), "Got: {}", result);
}

#[test]
fn test_s12_b35_dict_empty() {
    let code = r#"
def make_dict() -> dict:
    return dict()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b35_set_empty() {
    let code = r#"
def make_set() -> set:
    return set()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_set"), "Got: {}", result);
}

#[test]
fn test_s12_b35_set_from_list() {
    let code = r#"
def dedupe(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn dedupe"), "Got: {}", result);
}

#[test]
fn test_s12_b35_frozenset_empty() {
    let code = r#"
def make_frozen() -> frozenset:
    return frozenset()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_frozen"), "Got: {}", result);
}

#[test]
fn test_s12_b35_list_from_keys() {
    let code = r#"
def dict_keys_to_list(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_keys_to_list"), "Got: {}", result);
}

#[test]
fn test_s12_b35_list_from_values() {
    let code = r#"
def dict_vals_to_list(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_vals_to_list"), "Got: {}", result);
}

// ===== os.path functions =====

#[test]
fn test_s12_b35_path_exists() {
    let code = r#"
from os.path import exists

def check_path(p: str) -> bool:
    return exists(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_path"), "Got: {}", result);
}

#[test]
fn test_s12_b35_path_isfile() {
    let code = r#"
from os.path import isfile

def is_file(p: str) -> bool:
    return isfile(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_file"), "Got: {}", result);
}

#[test]
fn test_s12_b35_path_isdir() {
    let code = r#"
from os.path import isdir

def is_dir(p: str) -> bool:
    return isdir(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_dir"), "Got: {}", result);
}

#[test]
fn test_s12_b35_path_basename() {
    let code = r#"
from os.path import basename

def get_name(p: str) -> str:
    return basename(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_name"), "Got: {}", result);
}

#[test]
fn test_s12_b35_path_dirname() {
    let code = r#"
from os.path import dirname

def get_dir(p: str) -> str:
    return dirname(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_dir"), "Got: {}", result);
}

#[test]
fn test_s12_b35_path_splitext() {
    let code = r#"
from os.path import splitext

def get_ext(p: str) -> tuple:
    return splitext(p)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_ext"), "Got: {}", result);
}

// ===== File open patterns =====

#[test]
fn test_s12_b35_open_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_b35_open_write() {
    let code = r#"
def write_file(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_b35_open_append() {
    let code = r#"
def append_file(path: str, data: str):
    with open(path, "a") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn append_file"), "Got: {}", result);
}

// ===== Set operations =====

#[test]
fn test_s12_b35_set_or_op() {
    let code = r#"
def union_sets(a: set, b: set) -> set:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("fn union_sets"), "Got: {}", result);
}

#[test]
fn test_s12_b35_set_and_op() {
    let code = r#"
def intersect_sets(a: set, b: set) -> set:
    return a & b
"#;
    let result = transpile(code);
    assert!(result.contains("fn intersect_sets"), "Got: {}", result);
}

#[test]
fn test_s12_b35_set_sub_op() {
    let code = r#"
def diff_sets(a: set, b: set) -> set:
    return a - b
"#;
    let result = transpile(code);
    assert!(result.contains("fn diff_sets"), "Got: {}", result);
}

#[test]
fn test_s12_b35_set_xor_op() {
    let code = r#"
def sym_diff(a: set, b: set) -> set:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(result.contains("fn sym_diff"), "Got: {}", result);
}

// ===== Async/await =====

#[test]
fn test_s12_b35_async_function() {
    let code = r#"
async def fetch_data(url: str) -> str:
    result = await get(url)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn fetch_data"), "Got: {}", result);
}

#[test]
fn test_s12_b35_async_for() {
    let code = r#"
async def process_items(items: list) -> int:
    total = 0
    for item in items:
        result = await process(item)
        total += result
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_items"), "Got: {}", result);
}

// ===== Slice with step =====

#[test]
fn test_s12_b35_slice_reverse() {
    let code = r#"
def reverse_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"), "Got: {}", result);
}

#[test]
fn test_s12_b35_slice_every_other() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"), "Got: {}", result);
}

#[test]
fn test_s12_b35_slice_start_stop() {
    let code = r#"
def middle(items: list, start: int, end: int) -> list:
    return items[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn middle"), "Got: {}", result);
}

#[test]
fn test_s12_b35_slice_from_start() {
    let code = r#"
def tail(items: list, n: int) -> list:
    return items[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn tail"), "Got: {}", result);
}

#[test]
fn test_s12_b35_slice_to_end() {
    let code = r#"
def head(items: list, n: int) -> list:
    return items[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn head"), "Got: {}", result);
}

// ===== isinstance pattern =====

#[test]
fn test_s12_b35_isinstance_check() {
    let code = r#"
def is_int(value) -> bool:
    return isinstance(value, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_int"), "Got: {}", result);
}

// ===== ord/chr builtins =====

#[test]
fn test_s12_b35_ord_call() {
    let code = r#"
def char_code(c: str) -> int:
    return ord(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_code"), "Got: {}", result);
}

#[test]
fn test_s12_b35_chr_call() {
    let code = r#"
def code_to_char(n: int) -> str:
    return chr(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn code_to_char"), "Got: {}", result);
}

// ===== sorted/reversed builtins =====

#[test]
fn test_s12_b35_sorted_call() {
    let code = r#"
def sort_copy(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_copy"), "Got: {}", result);
}

#[test]
fn test_s12_b35_reversed_call() {
    let code = r#"
def rev_copy(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn rev_copy"), "Got: {}", result);
}

// ===== zip builtin =====

#[test]
fn test_s12_b35_zip_two() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append((x, y))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair_up"), "Got: {}", result);
}

// ===== Complex class with builtins =====

#[test]
fn test_s12_b35_class_with_set_ops() {
    let code = r#"
class TagSet:
    def __init__(self):
        self.tags = set()

    def add_tag(self, tag: str):
        self.tags.add(tag)

    def has_tag(self, tag: str) -> bool:
        return tag in self.tags

    def merge(self, other):
        self.tags = self.tags.union(other.tags)

    def common_tags(self, other) -> set:
        return self.tags.intersection(other.tags)
"#;
    let result = transpile(code);
    assert!(result.contains("TagSet"), "Got: {}", result);
}

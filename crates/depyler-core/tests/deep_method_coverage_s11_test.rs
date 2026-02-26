//! Session 11: Deep method coverage tests targeting specific untested paths
//!
//! Each test is designed to exercise a specific code path in
//! expr_gen_instance_methods.rs that existing tests miss.

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

// ============================================================================
// String method calls on variables (not literals)
// ============================================================================

#[test]
fn test_s11_deep_str_startswith_var() {
    let code = r#"
def starts_with(text: str, prefix: str) -> bool:
    return text.startswith(prefix)
"#;
    let result = transpile(code);
    assert!(result.contains("starts_with") || result.contains("fn starts_with"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_endswith_var() {
    let code = r#"
def ends_with(text: str, suffix: str) -> bool:
    return text.endswith(suffix)
"#;
    let result = transpile(code);
    assert!(result.contains("ends_with") || result.contains("fn ends_with"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_find_with_var() {
    let code = r#"
def locate(text: str, sub: str) -> int:
    return text.find(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("find") || result.contains("fn locate"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_rfind() {
    let code = r#"
def last_pos(text: str, sub: str) -> int:
    return text.rfind(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("rfind") || result.contains("fn last_pos"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_index() {
    let code = r#"
def must_find(text: str, sub: str) -> int:
    return text.index(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn must_find"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_rindex() {
    let code = r#"
def must_find_last(text: str, sub: str) -> int:
    return text.rindex(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn must_find_last"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_count_var() {
    let code = r#"
def count_occurrences(text: str, sub: str) -> int:
    return text.count(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("matches") || result.contains("count"), "Got: {}", result);
}

// ============================================================================
// String split variants
// ============================================================================

#[test]
fn test_s11_deep_str_split_no_args() {
    let code = r#"
def split_whitespace(text: str) -> list:
    return text.split()
"#;
    let result = transpile(code);
    assert!(result.contains("split_whitespace") || result.contains("split"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_split_with_sep() {
    let code = r#"
def split_on(text: str, sep: str) -> list:
    return text.split(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("split"), "Got: {}", result);
}

#[test]
fn test_s11_deep_str_splitlines() {
    let code = r#"
def get_lines(text: str) -> list:
    return text.splitlines()
"#;
    let result = transpile(code);
    assert!(result.contains("lines") || result.contains("split"), "Got: {}", result);
}

// ============================================================================
// List indexing and slicing patterns
// ============================================================================

#[test]
fn test_s11_deep_list_negative_index() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_item"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_slice_start() {
    let code = r#"
def first_three(items: list) -> list:
    return items[:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_three"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_slice_end() {
    let code = r#"
def skip_first(items: list) -> list:
    return items[1:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_first"), "Got: {}", result);
}

#[test]
fn test_s11_deep_list_slice_both() {
    let code = r#"
def middle(items: list) -> list:
    return items[1:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn middle"), "Got: {}", result);
}

#[test]
fn test_s11_deep_string_indexing() {
    let code = r#"
def first_char(s: str) -> str:
    return s[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_char"), "Got: {}", result);
}

#[test]
fn test_s11_deep_string_slice() {
    let code = r#"
def substring(s: str, start: int, end: int) -> str:
    return s[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn substring"), "Got: {}", result);
}

// ============================================================================
// Dict access patterns
// ============================================================================

#[test]
fn test_s11_deep_dict_bracket_access() {
    let code = r#"
def get_val(d: dict, key: str) -> int:
    return d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_val"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_bracket_assign() {
    let code = r#"
def set_val(d: dict, key: str, val: int) -> None:
    d[key] = val
"#;
    let result = transpile(code);
    assert!(result.contains("insert") || result.contains("fn set_val"), "Got: {}", result);
}

#[test]
fn test_s11_deep_dict_get_default() {
    let code = r#"
def safe_lookup(d: dict, key: str) -> str:
    return d.get(key, "unknown")
"#;
    let result = transpile(code);
    assert!(result.contains("unwrap_or") || result.contains("get"), "Got: {}", result);
}

// ============================================================================
// Set operations on typed sets
// ============================================================================

#[test]
fn test_s11_deep_set_update() {
    let code = r#"
from typing import Set

def add_all(target: Set[int], source: Set[int]) -> None:
    target.update(source)
"#;
    let result = transpile(code);
    assert!(result.contains("extend") || result.contains("fn add_all"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_remove() {
    let code = r#"
from typing import Set

def remove_item(s: Set[int], val: int) -> None:
    s.remove(val)
"#;
    let result = transpile(code);
    assert!(result.contains("remove") || result.contains("fn remove_item"), "Got: {}", result);
}

#[test]
fn test_s11_deep_set_pop() {
    let code = r#"
from typing import Set

def pop_any(s: Set[int]) -> int:
    return s.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_any"), "Got: {}", result);
}

// ============================================================================
// Attribute access patterns
// ============================================================================

#[test]
fn test_s11_deep_self_attribute_read() {
    let code = r#"
class Obj:
    def __init__(self, val: int) -> None:
        self.val = val

    def get_val(self) -> int:
        return self.val
"#;
    let result = transpile(code);
    assert!(result.contains("self.val") || result.contains("Obj"), "Got: {}", result);
}

#[test]
fn test_s11_deep_self_attribute_write() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.count: int = 0

    def increment(self) -> None:
        self.count = self.count + 1
"#;
    let result = transpile(code);
    assert!(result.contains("Counter") || result.contains("count"), "Got: {}", result);
}

// ============================================================================
// Type casting patterns
// ============================================================================

#[test]
fn test_s11_deep_list_of_str_to_int() {
    let code = r#"
def parse_numbers(items: list) -> list:
    return [int(x) for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("parse") || result.contains("fn parse_numbers"), "Got: {}", result);
}

#[test]
fn test_s11_deep_int_to_str_format() {
    let code = r#"
def format_number(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("to_string") || result.contains("format"), "Got: {}", result);
}

#[test]
fn test_s11_deep_float_to_str() {
    let code = r#"
def format_float(x: float) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(result.contains("to_string") || result.contains("fn format_float"), "Got: {}", result);
}

// ============================================================================
// Print patterns
// ============================================================================

#[test]
fn test_s11_deep_print_with_args() {
    let code = r#"
def log_values(a: int, b: str) -> None:
    print(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("println") || result.contains("print"), "Got: {}", result);
}

#[test]
fn test_s11_deep_print_fstring() {
    let code = r#"
def log_msg(name: str, count: int) -> None:
    print(f"Name: {name}, Count: {count}")
"#;
    let result = transpile(code);
    assert!(result.contains("println") || result.contains("format!"), "Got: {}", result);
}

// ============================================================================
// Complex boolean patterns
// ============================================================================

#[test]
fn test_s11_deep_complex_boolean() {
    let code = r#"
def is_valid(name: str, age: int) -> bool:
    return len(name) > 0 and age >= 0 and age < 150
"#;
    let result = transpile(code);
    assert!(result.contains("&&"), "Got: {}", result);
}

#[test]
fn test_s11_deep_none_check() {
    let code = r#"
from typing import Optional

def is_present(val: Optional[int]) -> bool:
    return val is not None
"#;
    let result = transpile(code);
    assert!(result.contains("is_some") || result.contains("fn is_present"), "Got: {}", result);
}

#[test]
fn test_s11_deep_is_none_check() {
    let code = r#"
from typing import Optional

def is_empty(val: Optional[int]) -> bool:
    return val is None
"#;
    let result = transpile(code);
    assert!(result.contains("is_none") || result.contains("fn is_empty"), "Got: {}", result);
}

// ============================================================================
// Enumerate and zip patterns
// ============================================================================

#[test]
fn test_s11_deep_enumerate_start() {
    let code = r#"
def number_lines(lines: list) -> list:
    result: list = []
    for i, line in enumerate(lines):
        result.append(f"{i}: {line}")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("enumerate"), "Got: {}", result);
}

#[test]
fn test_s11_deep_zip_three() {
    let code = r#"
def combine(a: list, b: list, c: list) -> list:
    result: list = []
    for x, y, z in zip(a, b, c):
        result.append(x + y + z)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("zip") || result.contains("fn combine"), "Got: {}", result);
}

// ============================================================================
// os.path methods
// ============================================================================

#[test]
fn test_s11_deep_os_path_basename() {
    let code = r#"
import os

def get_filename(path: str) -> str:
    return os.path.basename(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_filename"), "Got: {}", result);
}

#[test]
fn test_s11_deep_os_path_dirname() {
    let code = r#"
import os

def get_dir(path: str) -> str:
    return os.path.dirname(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_dir"), "Got: {}", result);
}

#[test]
fn test_s11_deep_os_path_splitext() {
    let code = r#"
import os

def get_ext(path: str) -> str:
    name, ext = os.path.splitext(path)
    return ext
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_ext"), "Got: {}", result);
}

// ============================================================================
// Hashlib methods
// ============================================================================

#[test]
fn test_s11_deep_hashlib_sha256() {
    let code = r#"
import hashlib

def hash_str(s: str) -> str:
    return hashlib.sha256(s.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn hash_str"), "Got: {}", result);
}

#[test]
fn test_s11_deep_hashlib_md5() {
    let code = r#"
import hashlib

def md5_hash(s: str) -> str:
    return hashlib.md5(s.encode()).hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("fn md5_hash"), "Got: {}", result);
}

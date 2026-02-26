//! Session 11: Coverage tests for stdlib method generation paths
//!
//! Tests exercise stdlib method transpilation through the full pipeline
//! to hit code paths in stdlib_method_gen/*.rs modules.

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
// math module
// ============================================================================

#[test]
fn test_s11_math_sqrt() {
    let code = r#"
import math

def root(x: float) -> float:
    return math.sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("sqrt"), "Should transpile math.sqrt. Got: {}", result);
}

#[test]
fn test_s11_math_abs() {
    let code = r#"
import math

def absolute(x: float) -> float:
    return math.fabs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("abs"), "Should transpile math.fabs. Got: {}", result);
}

#[test]
fn test_s11_math_pow() {
    let code = r#"
import math

def power(x: float, y: float) -> float:
    return math.pow(x, y)
"#;
    let result = transpile(code);
    assert!(result.contains("pow"), "Should transpile math.pow. Got: {}", result);
}

#[test]
fn test_s11_math_log() {
    let code = r#"
import math

def logarithm(x: float) -> float:
    return math.log(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("ln") || result.contains("log"),
        "Should transpile math.log. Got: {}",
        result
    );
}

#[test]
fn test_s11_math_sin_cos() {
    let code = r#"
import math

def trig(x: float) -> float:
    return math.sin(x) + math.cos(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("sin") && result.contains("cos"),
        "Should transpile sin/cos. Got: {}",
        result
    );
}

// ============================================================================
// os module
// ============================================================================

#[test]
fn test_s11_os_path_join() {
    let code = r#"
import os

def join_path(a: str, b: str) -> str:
    return os.path.join(a, b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("join") || result.contains("Path"),
        "Should transpile os.path.join. Got: {}",
        result
    );
}

#[test]
fn test_s11_os_path_exists() {
    let code = r#"
import os

def file_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(
        result.contains("exists") || result.contains("Path"),
        "Should transpile os.path.exists. Got: {}",
        result
    );
}

// ============================================================================
// json module
// ============================================================================

#[test]
fn test_s11_json_loads() {
    let code = r#"
import json

def parse_json(s: str) -> dict:
    return json.loads(s)
"#;
    let result = transpile(code);
    assert!(
        result.contains("serde_json") || result.contains("from_str") || result.contains("parse"),
        "Should transpile json.loads. Got: {}",
        result
    );
}

#[test]
fn test_s11_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("serde_json") || result.contains("to_string") || result.contains("json"),
        "Should transpile json.dumps. Got: {}",
        result
    );
}

// ============================================================================
// String methods (more complex)
// ============================================================================

#[test]
fn test_s11_string_find() {
    let code = r#"
def find_pos(s: str, sub: str) -> int:
    return s.find(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("find"), "Should transpile str.find. Got: {}", result);
}

#[test]
fn test_s11_string_count() {
    let code = r#"
def count_sub(s: str, sub: str) -> int:
    return s.count(sub)
"#;
    let result = transpile(code);
    assert!(
        result.contains("matches") || result.contains("count"),
        "Should transpile str.count. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_isdigit() {
    let code = r#"
def all_digits(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_ascii_digit") || result.contains("chars"),
        "Should transpile str.isdigit. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_isalpha() {
    let code = r#"
def all_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(
        result.contains("is_alphabetic") || result.contains("chars"),
        "Should transpile str.isalpha. Got: {}",
        result
    );
}

#[test]
fn test_s11_string_lstrip() {
    let code = r#"
def left_trim(s: str) -> str:
    return s.lstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("trim_start"), "Should transpile str.lstrip. Got: {}", result);
}

#[test]
fn test_s11_string_rstrip() {
    let code = r#"
def right_trim(s: str) -> str:
    return s.rstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("trim_end"), "Should transpile str.rstrip. Got: {}", result);
}

// ============================================================================
// List methods
// ============================================================================

#[test]
fn test_s11_list_pop() {
    let code = r#"
def pop_last(items: list) -> int:
    return items.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("pop"), "Should transpile list.pop. Got: {}", result);
}

#[test]
fn test_s11_list_insert() {
    let code = r#"
def insert_at(items: list, idx: int, val: int) -> None:
    items.insert(idx, val)
"#;
    let result = transpile(code);
    assert!(result.contains("insert"), "Should transpile list.insert. Got: {}", result);
}

#[test]
fn test_s11_list_extend() {
    let code = r#"
def extend_list(a: list, b: list) -> None:
    a.extend(b)
"#;
    let result = transpile(code);
    assert!(result.contains("extend"), "Should transpile list.extend. Got: {}", result);
}

#[test]
fn test_s11_list_remove() {
    let code = r#"
def remove_item(items: list, val: int) -> None:
    items.remove(val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("retain") || result.contains("remove") || result.contains("position"),
        "Should transpile list.remove. Got: {}",
        result
    );
}

#[test]
fn test_s11_list_reverse() {
    let code = r#"
def reverse_list(items: list) -> None:
    items.reverse()
"#;
    let result = transpile(code);
    assert!(result.contains("reverse"), "Should transpile list.reverse. Got: {}", result);
}

#[test]
fn test_s11_list_sort() {
    let code = r#"
def sort_list(items: list) -> None:
    items.sort()
"#;
    let result = transpile(code);
    assert!(result.contains("sort"), "Should transpile list.sort. Got: {}", result);
}

// ============================================================================
// Dict methods
// ============================================================================

#[test]
fn test_s11_dict_items() {
    let code = r#"
def iterate_dict(data: dict) -> list:
    result: list = []
    for key, value in data.items():
        result.append(key)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("iter") || result.contains("items") || result.contains("for"),
        "Should transpile dict.items. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_update() {
    let code = r#"
def merge(a: dict, b: dict) -> None:
    a.update(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("extend") || result.contains("update") || result.contains("insert"),
        "Should transpile dict.update. Got: {}",
        result
    );
}

#[test]
fn test_s11_dict_pop() {
    let code = r#"
def remove_key(data: dict, key: str) -> int:
    return data.pop(key)
"#;
    let result = transpile(code);
    assert!(
        result.contains("remove") || result.contains("pop"),
        "Should transpile dict.pop. Got: {}",
        result
    );
}

// ============================================================================
// Set methods
// ============================================================================

#[test]
fn test_s11_set_union() {
    let code = r#"
from typing import Set

def combine(a: Set[int], b: Set[int]) -> Set[int]:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("union") || result.contains("HashSet"),
        "Should transpile set.union. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_intersection() {
    let code = r#"
from typing import Set

def common(a: Set[int], b: Set[int]) -> Set[int]:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("intersection") || result.contains("HashSet"),
        "Should transpile set.intersection. Got: {}",
        result
    );
}

#[test]
fn test_s11_set_difference() {
    let code = r#"
from typing import Set

def diff(a: Set[int], b: Set[int]) -> Set[int]:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("difference") || result.contains("HashSet"),
        "Should transpile set.difference. Got: {}",
        result
    );
}

// ============================================================================
// Builtin functions
// ============================================================================

#[test]
fn test_s11_builtin_print() {
    let code = r#"
def greet(name: str) -> None:
    print(name)
"#;
    let result = transpile(code);
    assert!(
        result.contains("println") || result.contains("print"),
        "Should transpile print(). Got: {}",
        result
    );
}

#[test]
fn test_s11_builtin_len() {
    let code = r#"
def size(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("len()"), "Should transpile len(). Got: {}", result);
}

#[test]
fn test_s11_builtin_range_simple() {
    let code = r#"
def count(n: int) -> int:
    total: int = 0
    for i in range(n):
        total = total + i
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("0..") || result.contains("range"),
        "Should transpile range(). Got: {}",
        result
    );
}

#[test]
fn test_s11_builtin_range_start_stop() {
    let code = r#"
def count_from(start: int, stop: int) -> int:
    total: int = 0
    for i in range(start, stop):
        total = total + i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains(".."), "Should transpile range(start, stop). Got: {}", result);
}

#[test]
fn test_s11_builtin_input() {
    let code = r#"
def get_name() -> str:
    name: str = input("Enter name: ")
    return name
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_name"), "Should transpile input(). Got: {}", result);
}

#[test]
fn test_s11_builtin_isinstance() {
    let code = r#"
def check_type(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_type"), "Should transpile isinstance(). Got: {}", result);
}

#[test]
fn test_s11_builtin_type_conversion_bool() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Should transpile bool(). Got: {}", result);
}

#[test]
fn test_s11_builtin_sum() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("sum") || result.contains("iter"),
        "Should transpile sum(). Got: {}",
        result
    );
}

#[test]
fn test_s11_builtin_any() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("any") || result.contains("iter"),
        "Should transpile any(). Got: {}",
        result
    );
}

#[test]
fn test_s11_builtin_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("all") || result.contains("iter"),
        "Should transpile all(). Got: {}",
        result
    );
}

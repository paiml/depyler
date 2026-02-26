//! Session 12 Batch 13: Deep coverage tests for stmt_gen.rs cold paths
//!
//! Targets:
//! - Assert with message and NotEqual patterns
//! - Complex nested dict indices
//! - Try/except/finally with variable hoisting
//! - With statement patterns (sync/async, with/without target)
//! - Nested function definitions and closures
//! - Delete statement
//! - Global/nonlocal declarations
//! - Pass/ellipsis statements
//! - Complex return patterns
//! - Tuple unpacking from split
//! - Enumerate with tuple unpacking
//! - String iteration with dict operations

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

// ===== Assert patterns =====

#[test]
fn test_s12_assert_not_equal() {
    let code = r#"
def check_different(a: int, b: int) -> bool:
    assert a != b, "values must differ"
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_different"), "Got: {}", result);
}

#[test]
fn test_s12_assert_isinstance_pattern() {
    let code = r#"
def process(x: int) -> int:
    assert x >= 0
    return x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Delete statement =====

#[test]
fn test_s12_del_dict_key() {
    let code = r#"
def remove_entry(d: dict, key: str) -> dict:
    del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_entry"), "Got: {}", result);
}

// ===== Global declaration =====

#[test]
fn test_s12_global_variable_use() {
    let code = r#"
counter = 0

def increment() -> int:
    global counter
    counter += 1
    return counter
"#;
    let result = transpile(code);
    assert!(result.contains("increment"), "Got: {}", result);
}

// ===== Pass statement =====

#[test]
fn test_s12_pass_in_function() {
    let code = r#"
def placeholder():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn placeholder"), "Got: {}", result);
}

#[test]
fn test_s12_pass_in_except() {
    let code = r#"
def ignore_error(x: int) -> int:
    try:
        return 100 / x
    except ZeroDivisionError:
        pass
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn ignore_error"), "Got: {}", result);
}

// ===== Complex try/except/finally =====

#[test]
fn test_s12_try_except_else() {
    let code = r#"
def try_else(s: str) -> int:
    try:
        val = int(s)
    except ValueError:
        return -1
    else:
        return val
"#;
    let result = transpile(code);
    assert!(result.contains("fn try_else"), "Got: {}", result);
}

#[test]
fn test_s12_try_finally_no_except() {
    let code = r#"
def with_cleanup(x: int) -> int:
    result = 0
    try:
        result = x * 2
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_cleanup"), "Got: {}", result);
}

#[test]
fn test_s12_nested_try_except() {
    let code = r#"
def nested_error_handling(a: int, b: int) -> int:
    try:
        try:
            return a / b
        except ZeroDivisionError:
            return -1
    except Exception:
        return -2
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_error_handling"), "Got: {}", result);
}

// ===== With statement patterns =====

#[test]
fn test_s12_with_no_target() {
    let code = r#"
def critical_section() -> int:
    with open("lock"):
        result = 42
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn critical_section"), "Got: {}", result);
}

// ===== Raise patterns =====

#[test]
fn test_s12_raise_type_error() {
    let code = r#"
def validate_type(x: int) -> int:
    if x < 0:
        raise TypeError("Expected positive integer")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_type"), "Got: {}", result);
}

#[test]
fn test_s12_raise_index_error() {
    let code = r#"
def safe_access(items: list, idx: int) -> int:
    if idx >= len(items):
        raise IndexError("Index out of bounds")
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_access"), "Got: {}", result);
}

#[test]
fn test_s12_raise_key_error() {
    let code = r#"
def require_key(d: dict, key: str) -> int:
    if key not in d:
        raise KeyError(key)
    return d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn require_key"), "Got: {}", result);
}

// ===== Tuple unpacking =====

#[test]
fn test_s12_tuple_unpack_from_function() {
    let code = r#"
def get_pair() -> tuple:
    return (1, 2)

def use_pair() -> int:
    a, b = get_pair()
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn use_pair"), "Got: {}", result);
}

#[test]
fn test_s12_triple_unpack() {
    let code = r#"
def unpack_triple(items: list) -> int:
    a, b, c = items[0], items[1], items[2]
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn unpack_triple"), "Got: {}", result);
}

// ===== String iteration with dict =====

#[test]
fn test_s12_char_count_dict() {
    let code = r#"
def char_frequency(s: str) -> dict:
    freq = {}
    for c in s:
        if c in freq:
            freq[c] += 1
        else:
            freq[c] = 1
    return freq
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_frequency"), "Got: {}", result);
}

// ===== Enumerate with unpacking =====

#[test]
fn test_s12_enumerate_with_index() {
    let code = r#"
def indexed_items(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append((i, item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_items"), "Got: {}", result);
}

#[test]
fn test_s12_enumerate_with_start() {
    let code = r#"
def numbered_lines(lines: list) -> list:
    result = []
    for num, line in enumerate(lines, 1):
        result.append(str(num) + ": " + line)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn numbered_lines"), "Got: {}", result);
}

// ===== Complex return patterns =====

#[test]
fn test_s12_return_dict_literal() {
    let code = r#"
def make_config() -> dict:
    return {"key": "value", "count": 42}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_config"), "Got: {}", result);
}

#[test]
fn test_s12_return_list_comprehension() {
    let code = r#"
def squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Got: {}", result);
}

#[test]
fn test_s12_early_return_in_loop() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first"), "Got: {}", result);
}

// ===== Nested dict operations =====

#[test]
fn test_s12_nested_dict_access() {
    let code = r#"
def get_nested(config: dict) -> str:
    return config["database"]["host"]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_nested"), "Got: {}", result);
}

#[test]
fn test_s12_nested_dict_assign() {
    let code = r#"
def set_nested(config: dict, value: str) -> dict:
    config["database"] = {"host": value}
    return config
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_nested"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_with_multiple_methods() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1

    def decrement(self):
        self.count -= 1

    def get_count(self) -> int:
        return self.count

    def reset(self):
        self.count = 0
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
    assert!(result.contains("increment"), "Got: {}", result);
    assert!(result.contains("get_count"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_comparison() {
    let code = r#"
class Temperature:
    def __init__(self, value: float):
        self.value = value

    def __lt__(self, other) -> bool:
        return self.value < other.value

    def __gt__(self, other) -> bool:
        return self.value > other.value
"#;
    let result = transpile(code);
    assert!(result.contains("Temperature"), "Got: {}", result);
}

// ===== While loop patterns =====

#[test]
fn test_s12_while_with_break() {
    let code = r#"
def find_in_sorted(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_in_sorted"), "Got: {}", result);
}

#[test]
fn test_s12_while_with_continue() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    i = 0
    while i < len(items):
        i += 1
        if items[i - 1] < 0:
            continue
        total += items[i - 1]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positive"), "Got: {}", result);
}

// ===== Multiple assignment =====

#[test]
fn test_s12_chained_assignment() {
    let code = r#"
def init_counters() -> int:
    a = b = c = d = 0
    return a + b + c + d
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_counters"), "Got: {}", result);
}

// ===== String split unpacking =====

#[test]
fn test_s12_split_unpack_two() {
    let code = r#"
def parse_pair(s: str) -> tuple:
    key, value = s.split("=", 1)
    return (key, value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_pair"), "Got: {}", result);
}

// ===== Complex if/elif/else =====

#[test]
fn test_s12_multi_elif() {
    let code = r#"
def classify(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    elif x < 10:
        return "small"
    elif x < 100:
        return "medium"
    else:
        return "large"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

// ===== OS and sys module patterns =====

#[test]
fn test_s12_os_path_join() {
    let code = r#"
import os

def build_path(directory: str, filename: str) -> str:
    return os.path.join(directory, filename)
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_path"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_exists() {
    let code = r#"
import os

def file_exists(path: str) -> bool:
    return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn file_exists"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_basename() {
    let code = r#"
import os

def get_filename(path: str) -> str:
    return os.path.basename(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_filename"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_dirname() {
    let code = r#"
import os

def get_directory(path: str) -> str:
    return os.path.dirname(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_directory"), "Got: {}", result);
}

#[test]
fn test_s12_os_getenv() {
    let code = r#"
import os

def get_home() -> str:
    return os.getenv("HOME", "/tmp")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_home"), "Got: {}", result);
}

// ===== JSON module =====

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

// ===== Itertools-like patterns =====

#[test]
fn test_s12_zip_two_lists() {
    let code = r#"
def pair_up(keys: list, values: list) -> list:
    result = []
    for k, v in zip(keys, values):
        result.append((k, v))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair_up"), "Got: {}", result);
}

#[test]
fn test_s12_enumerate_dict_build() {
    let code = r#"
def make_index(items: list) -> dict:
    index = {}
    for i, item in enumerate(items):
        index[item] = i
    return index
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_index"), "Got: {}", result);
}

// ===== Collections module =====

#[test]
fn test_s12_collections_defaultdict() {
    let code = r#"
from collections import defaultdict

def group_words(words: list) -> dict:
    groups = defaultdict(list)
    for word in words:
        groups[len(word)].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_words"), "Got: {}", result);
}

#[test]
fn test_s12_collections_counter() {
    let code = r#"
from collections import Counter

def count_elements(items: list) -> dict:
    return Counter(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_elements"), "Got: {}", result);
}

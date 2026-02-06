//! Session 12 Batch 22: Deep coverage for builtin function conversions
//!
//! Targets direct_rules_convert.rs cold paths:
//! - bytes(), bytearray(), tuple(), frozenset(), set() builtins
//! - open() with various modes
//! - zeros(), ones(), full() array initializers
//! - list() from dict keys/values
//! - date/datetime constructors
//! - sorted() with various inputs
//! - all()/any()/sum() builtins
//! - reversed()/enumerate()/zip() builtins

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

// ===== bytes() builtin =====

#[test]
fn test_s12_bytes_empty() {
    let code = r#"
class Buffer:
    def __init__(self):
        self.data = bytes()
"#;
    let result = transpile(code);
    assert!(result.contains("Buffer"), "Got: {}", result);
}

#[test]
fn test_s12_bytes_from_size() {
    let code = r#"
class Buffer:
    def __init__(self, size: int):
        self.data = bytes(size)
"#;
    let result = transpile(code);
    assert!(result.contains("Buffer"), "Got: {}", result);
}

// ===== bytearray() builtin =====

#[test]
fn test_s12_bytearray_empty() {
    let code = r#"
class MutableBuffer:
    def __init__(self):
        self.data = bytearray()
"#;
    let result = transpile(code);
    assert!(result.contains("MutableBuffer"), "Got: {}", result);
}

#[test]
fn test_s12_bytearray_from_size() {
    let code = r#"
class MutableBuffer:
    def __init__(self, n: int):
        self.data = bytearray(n)
"#;
    let result = transpile(code);
    assert!(result.contains("MutableBuffer"), "Got: {}", result);
}

// ===== tuple() builtin =====

#[test]
fn test_s12_tuple_empty() {
    let code = r#"
class Container:
    def __init__(self):
        self.items = tuple()
"#;
    let result = transpile(code);
    assert!(result.contains("Container"), "Got: {}", result);
}

#[test]
fn test_s12_tuple_from_list() {
    let code = r#"
class Container:
    def __init__(self, items: list):
        self.items = tuple(items)
"#;
    let result = transpile(code);
    assert!(result.contains("Container"), "Got: {}", result);
}

// ===== frozenset() builtin =====

#[test]
fn test_s12_frozenset_empty() {
    let code = r#"
class ImmutableSet:
    def __init__(self):
        self.items = frozenset()
"#;
    let result = transpile(code);
    assert!(result.contains("ImmutableSet"), "Got: {}", result);
}

#[test]
fn test_s12_frozenset_from_list() {
    let code = r#"
class ImmutableSet:
    def __init__(self, items: list):
        self.items = frozenset(items)
"#;
    let result = transpile(code);
    assert!(result.contains("ImmutableSet"), "Got: {}", result);
}

// ===== set() from iterables in class methods =====

#[test]
fn test_s12_set_empty_in_method() {
    let code = r#"
class UniqueCollector:
    def __init__(self):
        self.seen = set()

    def reset(self):
        self.seen = set()
"#;
    let result = transpile(code);
    assert!(result.contains("UniqueCollector"), "Got: {}", result);
}

#[test]
fn test_s12_set_from_list_in_method() {
    let code = r#"
class UniqueCollector:
    def __init__(self, items: list):
        self.seen = set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("UniqueCollector"), "Got: {}", result);
}

// ===== list() from dict methods =====

#[test]
fn test_s12_list_from_dict_keys() {
    let code = r#"
class Config:
    def __init__(self):
        self.data = {}

    def get_keys(self) -> list:
        return list(self.data.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("get_keys"), "Got: {}", result);
}

#[test]
fn test_s12_list_from_dict_values() {
    let code = r#"
class Config:
    def __init__(self):
        self.data = {}

    def get_values(self) -> list:
        return list(self.data.values())
"#;
    let result = transpile(code);
    assert!(result.contains("get_values"), "Got: {}", result);
}

#[test]
fn test_s12_list_empty_in_method() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = list()

    def clear(self):
        self.items = list()
"#;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

// ===== open() with various modes =====

#[test]
fn test_s12_open_read_mode() {
    let code = r#"
def read_file(path: str) -> str:
    f = open(path, "r")
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_open_write_mode() {
    let code = r#"
def write_file(path: str, data: str):
    f = open(path, "w")
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_open_append_mode() {
    let code = r#"
def append_file(path: str, data: str):
    f = open(path, "a")
"#;
    let result = transpile(code);
    assert!(result.contains("fn append_file"), "Got: {}", result);
}

#[test]
fn test_s12_open_default_mode() {
    let code = r#"
def open_default(path: str):
    f = open(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn open_default"), "Got: {}", result);
}

// ===== zeros/ones/full array initializers =====

#[test]
fn test_s12_zeros_call() {
    let code = r#"
class Matrix:
    def __init__(self, n: int):
        self.data = zeros(n)
"#;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

#[test]
fn test_s12_ones_call() {
    let code = r#"
class Matrix:
    def __init__(self, n: int):
        self.data = ones(n)
"#;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

#[test]
fn test_s12_full_call() {
    let code = r#"
class Matrix:
    def __init__(self, n: int, val: int):
        self.data = full(n, val)
"#;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

// ===== sorted() with various inputs =====

#[test]
fn test_s12_sorted_basic_in_method() {
    let code = r#"
class Sorter:
    def __init__(self, items: list):
        self.items = items

    def get_sorted(self) -> list:
        return sorted(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("get_sorted"), "Got: {}", result);
}

// ===== all()/any() builtins =====

#[test]
fn test_s12_all_builtin_in_method() {
    let code = r#"
class Validator:
    def __init__(self, checks: list):
        self.checks = checks

    def all_pass(self) -> bool:
        return all(self.checks)
"#;
    let result = transpile(code);
    assert!(result.contains("all_pass"), "Got: {}", result);
}

#[test]
fn test_s12_any_builtin_in_method() {
    let code = r#"
class Validator:
    def __init__(self, checks: list):
        self.checks = checks

    def any_pass(self) -> bool:
        return any(self.checks)
"#;
    let result = transpile(code);
    assert!(result.contains("any_pass"), "Got: {}", result);
}

// ===== sum() builtin =====

#[test]
fn test_s12_sum_builtin_in_method() {
    let code = r#"
class Calculator:
    def __init__(self, values: list):
        self.values = values

    def total(self) -> int:
        return sum(self.values)
"#;
    let result = transpile(code);
    assert!(result.contains("total"), "Got: {}", result);
}

// ===== dict() builtin =====

#[test]
fn test_s12_dict_builtin_in_method() {
    let code = r#"
class Registry:
    def __init__(self):
        self.data = dict()
"#;
    let result = transpile(code);
    assert!(result.contains("Registry"), "Got: {}", result);
}

// ===== reversed()/enumerate()/zip() =====

#[test]
fn test_s12_reversed_in_method() {
    let code = r#"
class Sequence:
    def __init__(self, items: list):
        self.items = items

    def backwards(self) -> list:
        result = []
        for item in reversed(self.items):
            result.append(item)
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("backwards"), "Got: {}", result);
}

#[test]
fn test_s12_enumerate_in_method() {
    let code = r#"
class IndexedList:
    def __init__(self, items: list):
        self.items = items

    def with_indices(self) -> list:
        result = []
        for i, item in enumerate(self.items):
            result.append((i, item))
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("with_indices"), "Got: {}", result);
}

#[test]
fn test_s12_zip_in_method() {
    let code = r#"
class Merger:
    def __init__(self, a: list, b: list):
        self.a = a
        self.b = b

    def merge(self) -> list:
        result = []
        for x, y in zip(self.a, self.b):
            result.append((x, y))
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("merge"), "Got: {}", result);
}

// ===== Complex patterns combining builtins =====

#[test]
fn test_s12_class_with_multiple_builtins() {
    let code = r#"
class DataProcessor:
    def __init__(self):
        self.data = list()
        self.cache = dict()
        self.seen = set()

    def add(self, item: int):
        if item not in self.seen:
            self.seen.add(item)
            self.data.append(item)

    def count(self) -> int:
        return len(self.data)

    def total(self) -> int:
        return sum(self.data)

    def sorted_data(self) -> list:
        return sorted(self.data)
"#;
    let result = transpile(code);
    assert!(result.contains("DataProcessor"), "Got: {}", result);
    assert!(result.contains("fn add"), "Got: {}", result);
    assert!(result.contains("fn count"), "Got: {}", result);
}

// ===== os.path functions (direct import) =====

#[test]
fn test_s12_splitext_direct() {
    let code = r#"
def get_extension(path: str) -> str:
    name, ext = splitext(path)
    return ext
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_extension"), "Got: {}", result);
}

#[test]
fn test_s12_basename_direct() {
    let code = r#"
def get_filename(path: str) -> str:
    return basename(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_filename"), "Got: {}", result);
}

#[test]
fn test_s12_dirname_direct() {
    let code = r#"
def get_dir(path: str) -> str:
    return dirname(path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_dir"), "Got: {}", result);
}

#[test]
fn test_s12_exists_isfile_isdir() {
    let code = r#"
def check_path(path: str) -> bool:
    if exists(path):
        if isfile(path):
            return True
        if isdir(path):
            return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_path"), "Got: {}", result);
}

// ===== isinstance check =====

#[test]
fn test_s12_isinstance_in_method() {
    let code = r#"
class TypeChecker:
    def check(self, value) -> bool:
        return isinstance(value, int)
"#;
    let result = transpile(code);
    assert!(result.contains("TypeChecker"), "Got: {}", result);
}

// ===== cls() classmethod pattern =====

#[test]
fn test_s12_classmethod_cls_constructor() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    @classmethod
    def origin(cls):
        return cls(0.0, 0.0)

    @classmethod
    def from_tuple(cls, t: tuple):
        return cls(t[0], t[1])
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

// ===== Error type generation triggers =====

#[test]
fn test_s12_raise_value_error() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s12_raise_index_error() {
    let code = r#"
def safe_get(items: list, idx: int) -> int:
    if idx >= len(items):
        raise IndexError("out of range")
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_raise_runtime_error() {
    let code = r#"
def must_succeed(flag: bool):
    if not flag:
        raise RuntimeError("operation failed")
"#;
    let result = transpile(code);
    assert!(result.contains("fn must_succeed"), "Got: {}", result);
}

#[test]
fn test_s12_raise_file_not_found() {
    let code = r#"
def require_file(path: str):
    if not exists(path):
        raise FileNotFoundError("missing: " + path)
"#;
    let result = transpile(code);
    assert!(result.contains("fn require_file"), "Got: {}", result);
}

#[test]
fn test_s12_raise_type_error() {
    let code = r#"
def check_type(value, expected_type: str):
    if expected_type == "int" and not isinstance(value, int):
        raise TypeError("expected int")
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_type"), "Got: {}", result);
}

// ===== Complex method with multiple builtins =====

#[test]
fn test_s12_complex_method_builtins() {
    let code = r#"
class TextAnalyzer:
    def __init__(self, text: str):
        self.text = text
        self.words = text.split()

    def word_count(self) -> int:
        return len(self.words)

    def unique_words(self) -> int:
        return len(set(self.words))

    def sorted_words(self) -> list:
        return sorted(set(self.words))

    def char_count(self) -> int:
        return sum(len(w) for w in self.words)
"#;
    let result = transpile(code);
    assert!(result.contains("TextAnalyzer"), "Got: {}", result);
    assert!(result.contains("word_count"), "Got: {}", result);
}

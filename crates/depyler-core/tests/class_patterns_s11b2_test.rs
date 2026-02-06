//! Session 11 Batch 2: Class patterns, dunder methods, with-statement, raise
//!
//! Targets:
//! - rust_gen/mod.rs: class codegen, dunder method translation
//! - expr_gen_instance_methods.rs:2658 dunder method dispatch
//! - stmt_gen.rs: with statement, raise patterns
//! - stmt_gen.rs:2401 heterogeneous IO (boxed dyn Write)
//! - stmt_gen_complex.rs: nested function with captures

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

// ===== Class with dunder methods =====

#[test]
fn test_s11b2_class_init_str() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    def __str__(self) -> str:
        return self.name + " age " + str(self.age)
"#;
    let result = transpile(code);
    assert!(result.contains("Person"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_repr() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return "Point(" + str(self.x) + ", " + str(self.y) + ")"
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_eq() {
    let code = r#"
class Vec2:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Vec2"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_len() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def __len__(self) -> int:
        return len(self.items)

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_contains() {
    let code = r#"
class Bag:
    def __init__(self):
        self.items = []

    def __contains__(self, item: int) -> bool:
        return item in self.items

    def add(self, item: int):
        self.items.append(item)
"#;
    let result = transpile(code);
    assert!(result.contains("Bag"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_next() {
    let code = r#"
class Counter:
    def __init__(self, limit: int):
        self.current = 0
        self.limit = limit

    def __next__(self) -> int:
        if self.current >= self.limit:
            raise StopIteration
        val = self.current
        self.current += 1
        return val
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_staticmethod() {
    let code = r#"
class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def mul(a: int, b: int) -> int:
        return a * b
"#;
    let result = transpile(code);
    assert!(result.contains("Math"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self.radius * self.radius
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_inheritance() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return self.name

class Dog(Animal):
    def speak(self) -> str:
        return self.name + " says woof"
"#;
    let result = transpile(code);
    assert!(result.contains("Animal"), "Got: {}", result);
    assert!(result.contains("Dog"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_variables() {
    let code = r#"
class Config:
    MAX_SIZE = 100
    DEFAULT_NAME = "unnamed"

    def __init__(self, name: str):
        self.name = name
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

// ===== With statement patterns =====

#[test]
fn test_s11b2_with_open_read() {
    let code = r#"
def read_all(path: str) -> str:
    with open(path) as f:
        content = f.read()
    return content
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_all"), "Got: {}", result);
}

#[test]
fn test_s11b2_with_open_write() {
    let code = r#"
def write_text(path: str, text: str):
    with open(path, "w") as f:
        f.write(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_text"), "Got: {}", result);
}

#[test]
fn test_s11b2_with_open_readlines() {
    let code = r#"
def get_lines(path: str) -> list:
    with open(path) as f:
        lines = f.readlines()
    return lines
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_lines"), "Got: {}", result);
}

#[test]
fn test_s11b2_with_open_readline() {
    let code = r#"
def first_line(path: str) -> str:
    with open(path) as f:
        line = f.readline()
    return line
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_line"), "Got: {}", result);
}

#[test]
fn test_s11b2_with_open_append() {
    let code = r#"
def append_text(path: str, text: str):
    with open(path, "a") as f:
        f.write(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn append_text"), "Got: {}", result);
}

#[test]
fn test_s11b2_with_open_binary_read() {
    let code = r#"
def read_bytes(path: str) -> bytes:
    with open(path, "rb") as f:
        data = f.read()
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_bytes"), "Got: {}", result);
}

// ===== Raise patterns =====

#[test]
fn test_s11b2_raise_value_error() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be non-negative")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s11b2_raise_type_error() {
    let code = r#"
def check_type(x: int) -> int:
    if x == 0:
        raise TypeError("cannot be zero")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_type"), "Got: {}", result);
}

#[test]
fn test_s11b2_raise_in_try() {
    let code = r#"
def safe_validate(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("negative")
        return x
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_validate"), "Got: {}", result);
}

#[test]
fn test_s11b2_raise_index_error() {
    let code = r#"
def checked_get(items: list, idx: int) -> int:
    if idx >= len(items):
        raise IndexError("out of range")
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn checked_get"), "Got: {}", result);
}

#[test]
fn test_s11b2_raise_key_error() {
    let code = r#"
def required_key(d: dict, key: str) -> int:
    if key not in d:
        raise KeyError("missing key")
    return d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn required_key"), "Got: {}", result);
}

#[test]
fn test_s11b2_raise_not_implemented() {
    let code = r#"
def abstract_method():
    raise NotImplementedError("subclass must implement")
"#;
    let result = transpile(code);
    assert!(result.contains("fn abstract_method"), "Got: {}", result);
}

// ===== Complex patterns =====

#[test]
fn test_s11b2_multiple_return_none() {
    let code = r#"
def find_item(items: list, target: int):
    for item in items:
        if item == target:
            return item
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_item"), "Got: {}", result);
}

#[test]
fn test_s11b2_early_return_guards() {
    let code = r#"
def safe_divide(a: int, b: int) -> float:
    if b == 0:
        return 0.0
    if a == 0:
        return 0.0
    return float(a) / float(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s11b2_nested_if_elif() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        if x > 100:
            return "large"
        elif x > 10:
            return "medium"
        else:
            return "small"
    elif x == 0:
        return "zero"
    else:
        return "negative"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s11b2_assert_with_message() {
    let code = r#"
def check(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"), "Got: {}", result);
}

#[test]
fn test_s11b2_del_dict_key() {
    let code = r#"
def remove_key(d: dict, key: str) -> dict:
    del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

#[test]
fn test_s11b2_global_constant() {
    let code = r#"
MAX_SIZE = 1000
DEFAULT_TIMEOUT = 30

def get_max() -> int:
    return MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE") || result.contains("get_max"), "Got: {}", result);
}

#[test]
fn test_s11b2_multiple_imports() {
    let code = r#"
import math
import os

def compute(x: float) -> float:
    return math.sqrt(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Got: {}", result);
}

#[test]
fn test_s11b2_from_import() {
    let code = r#"
from math import sqrt, floor

def calc(x: float) -> int:
    return floor(sqrt(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn calc"), "Got: {}", result);
}

// ===== Docstring patterns =====

#[test]
fn test_s11b2_function_docstring() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two integers."""
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn add"), "Got: {}", result);
}

#[test]
fn test_s11b2_multiline_docstring() {
    let code = r#"
def compute(x: float) -> float:
    """
    Compute the result.

    Args:
        x: input value

    Returns:
        computed result
    """
    return x * 2.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Got: {}", result);
}

#[test]
fn test_s11b2_class_docstring() {
    let code = r#"
class Node:
    """A simple node."""
    def __init__(self, val: int):
        self.val = val
"#;
    let result = transpile(code);
    assert!(result.contains("Node"), "Got: {}", result);
}

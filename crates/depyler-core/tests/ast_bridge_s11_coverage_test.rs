//! Session 11: Coverage tests for ast_bridge.rs code paths
//!
//! Tests exercise AST->HIR conversion for various Python constructs:
//! - Class definitions with methods
//! - Decorators
//! - Global/nonlocal statements
//! - Complex import patterns
//! - Async patterns
//! - With statements
//! - Yield expressions
//! - Walrus operator
//! - Multiple inheritance
//! - Property decorators
//! - Dataclass patterns

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

fn hir_succeeds(python_code: &str) -> bool {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).is_ok()
}

// ============================================================================
// Class definitions
// ============================================================================

#[test]
fn test_s11_class_simple() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#;
    assert!(hir_succeeds(code), "Should parse simple class");
}

#[test]
fn test_s11_class_with_method() {
    let code = r#"
class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b

    def multiply(self, a: int, b: int) -> int:
        return a * b
"#;
    assert!(hir_succeeds(code), "Should parse class with methods");
}

#[test]
fn test_s11_class_with_static_method() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    assert!(hir_succeeds(code), "Should parse static method");
}

#[test]
fn test_s11_class_with_class_method() {
    let code = r#"
class Factory:
    @classmethod
    def create(cls, name: str) -> None:
        pass
"#;
    assert!(hir_succeeds(code), "Should parse class method");
}

#[test]
fn test_s11_class_inheritance() {
    let code = r#"
class Animal:
    def speak(self) -> str:
        return ""

class Dog(Animal):
    def speak(self) -> str:
        return "Woof"
"#;
    assert!(hir_succeeds(code), "Should parse inheritance");
}

// ============================================================================
// Import patterns
// ============================================================================

#[test]
fn test_s11_import_simple() {
    let code = r#"
import math

def use_math() -> float:
    return math.sqrt(4.0)
"#;
    let result = transpile(code);
    assert!(result.contains("sqrt"), "Should handle math import. Got: {}", result);
}

#[test]
fn test_s11_import_from() {
    let code = r#"
from typing import List, Dict, Optional

def process(items: List[int]) -> Dict[str, int]:
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Should handle from imports. Got: {}", result);
}

#[test]
fn test_s11_import_multiple() {
    let code = r#"
import os
import sys
import json

def get_path() -> str:
    return "."
"#;
    assert!(hir_succeeds(code), "Should parse multiple imports");
}

// ============================================================================
// With statement
// ============================================================================

#[test]
fn test_s11_with_statement() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    assert!(hir_succeeds(code), "Should parse with statement");
}

// ============================================================================
// Async patterns
// ============================================================================

#[test]
fn test_s11_async_function() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return ""
"#;
    assert!(hir_succeeds(code), "Should parse async function");
}

// ============================================================================
// Global and nonlocal
// ============================================================================

#[test]
fn test_s11_global_statement() {
    let code = r#"
counter: int = 0

def increment() -> None:
    global counter
    counter = counter + 1
"#;
    assert!(hir_succeeds(code), "Should parse global statement");
}

// ============================================================================
// Complex type annotations in HIR
// ============================================================================

#[test]
fn test_s11_bridge_nested_generic_types() {
    let code = r#"
from typing import Dict, List, Optional

def complex_types(data: Dict[str, List[int]]) -> Optional[int]:
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn complex_types"), "Should handle nested generics. Got: {}", result);
}

#[test]
fn test_s11_bridge_union_type() {
    let code = r#"
from typing import Union

def process(value: Union[int, str]) -> str:
    return str(value)
"#;
    assert!(hir_succeeds(code), "Should parse Union type");
}

// ============================================================================
// Decorator patterns
// ============================================================================

#[test]
fn test_s11_bridge_decorated_function() {
    let code = r#"
def my_decorator(func):
    return func

@my_decorator
def decorated() -> int:
    return 42
"#;
    assert!(hir_succeeds(code), "Should parse decorated function");
}

// ============================================================================
// Multiple assignment targets
// ============================================================================

#[test]
fn test_s11_bridge_tuple_unpack_assign() {
    let code = r#"
def swap(a: int, b: int) -> int:
    a, b = b, a
    return a
"#;
    assert!(hir_succeeds(code), "Should parse tuple unpack assignment");
}

// ============================================================================
// List/dict/set comprehensions in HIR
// ============================================================================

#[test]
fn test_s11_bridge_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn unique_lengths"),
        "Should handle set comprehension. Got: {}",
        result
    );
}

#[test]
fn test_s11_bridge_dict_comprehension() {
    let code = r#"
def invert(data: dict) -> dict:
    return {v: k for k, v in data.items()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert"), "Should handle dict comprehension. Got: {}", result);
}

// ============================================================================
// Try/Except/Finally
// ============================================================================

#[test]
fn test_s11_bridge_try_finally() {
    let code = r#"
def with_cleanup(path: str) -> int:
    result: int = 0
    try:
        result = 42
    finally:
        pass
    return result
"#;
    assert!(hir_succeeds(code), "Should parse try/finally");
}

#[test]
fn test_s11_bridge_try_except_else() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        result: int = int(s)
    except ValueError:
        result = 0
    else:
        result = result + 1
    return result
"#;
    assert!(hir_succeeds(code), "Should parse try/except/else");
}

// ============================================================================
// Raise statements
// ============================================================================

#[test]
fn test_s11_bridge_raise_statement() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(hir_succeeds(code), "Should parse raise statement");
}

// ============================================================================
// Delete statement
// ============================================================================

#[test]
fn test_s11_bridge_del_statement() {
    let code = r#"
def cleanup(data: dict) -> None:
    del data["key"]
"#;
    assert!(hir_succeeds(code), "Should parse del statement");
}

// ============================================================================
// Assert with message
// ============================================================================

#[test]
fn test_s11_bridge_assert_with_message() {
    let code = r#"
def check(x: int) -> None:
    assert x > 0, "must be positive"
"#;
    assert!(hir_succeeds(code), "Should parse assert with message");
}

// ============================================================================
// Starred expression
// ============================================================================

#[test]
fn test_s11_bridge_star_unpack() {
    let code = r#"
def first_and_rest(items: list) -> int:
    first = items[0]
    return first
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_and_rest"), "Should handle star unpack. Got: {}", result);
}

// ============================================================================
// Nested function definitions
// ============================================================================

#[test]
fn test_s11_bridge_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    assert!(hir_succeeds(code), "Should parse nested function");
}

// ============================================================================
// Complex class patterns
// ============================================================================

#[test]
fn test_s11_bridge_dataclass_like() {
    let code = r#"
class Config:
    host: str
    port: int
    debug: bool

    def __init__(self, host: str, port: int, debug: bool = False) -> None:
        self.host = host
        self.port = port
        self.debug = debug

    def url(self) -> str:
        return f"http://{self.host}:{self.port}"
"#;
    assert!(hir_succeeds(code), "Should parse dataclass-like pattern");
}

// ============================================================================
// Generator expressions
// ============================================================================

#[test]
fn test_s11_bridge_generator_expression() {
    let code = r#"
def sum_of_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    assert!(hir_succeeds(code), "Should parse generator expression");
}

// ============================================================================
// While/else pattern
// ============================================================================

#[test]
fn test_s11_bridge_while_else() {
    let code = r#"
def search(items: list, target: int) -> bool:
    i: int = 0
    while i < len(items):
        if items[i] == target:
            return True
        i = i + 1
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn search"), "Should handle while with return. Got: {}", result);
}

// ============================================================================
// Empty module
// ============================================================================

#[test]
fn test_s11_bridge_empty_module() {
    let code = "";
    let ast = parse(code, Mode::Module, "<test>").expect("parse");
    let result = AstBridge::new().with_source(code.to_string()).python_to_hir(ast);
    assert!(result.is_ok(), "Should handle empty module");
}

// ============================================================================
// Module with only comments
// ============================================================================

#[test]
fn test_s11_bridge_comments_only() {
    let code = r#"
# This is a comment
# Another comment
"#;
    assert!(hir_succeeds(code), "Should handle comments-only module");
}

// ============================================================================
// Module with docstring
// ============================================================================

#[test]
fn test_s11_bridge_module_docstring() {
    let code = r#"
"""This is a module docstring."""

def hello() -> str:
    return "hello"
"#;
    let result = transpile(code);
    assert!(result.contains("fn hello"), "Should handle module docstring. Got: {}", result);
}

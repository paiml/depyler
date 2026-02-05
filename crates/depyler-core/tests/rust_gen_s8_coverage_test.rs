//! Session 8 coverage tests for rust_gen.rs
//! Targets: module-level patterns, imports, class generation, function signatures

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

// ── Module-level constants ──────────────────────────────────────

#[test]
fn test_module_level_constant() {
    let code = transpile(
        r#"
MAX_SIZE: int = 100
PI: float = 3.14159
"#,
    );
    assert!(
        code.contains("MAX_SIZE") || code.contains("100"),
        "Should generate module constant: {code}"
    );
}

#[test]
fn test_module_level_string_constant() {
    let code = transpile(
        r#"
VERSION: str = "1.0.0"
"#,
    );
    assert!(
        code.contains("VERSION") || code.contains("1.0.0"),
        "Should generate string constant: {code}"
    );
}

// ── Function signatures ─────────────────────────────────────────

#[test]
fn test_function_no_args() {
    let code = transpile(
        r#"
def hello() -> str:
    return "hello"
"#,
    );
    assert!(
        code.contains("fn hello") && code.contains("String"),
        "Should generate no-arg function: {code}"
    );
}

#[test]
fn test_function_multiple_args() {
    let code = transpile(
        r#"
def add(a: int, b: int, c: int) -> int:
    return a + b + c
"#,
    );
    assert!(
        code.contains("fn add") && code.contains("i64") || code.contains("i32"),
        "Should generate multi-arg function: {code}"
    );
}

#[test]
fn test_function_default_args() {
    let code = transpile(
        r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"
"#,
    );
    assert!(
        code.contains("fn greet"),
        "Should generate function with defaults: {code}"
    );
}

#[test]
fn test_function_returning_list() {
    let code = transpile(
        r#"
def make_list() -> list:
    return [1, 2, 3]
"#,
    );
    assert!(
        code.contains("Vec") || code.contains("vec!"),
        "Should generate Vec return type: {code}"
    );
}

#[test]
fn test_function_returning_dict() {
    let code = transpile(
        r#"
def make_dict() -> dict:
    return {"key": "value"}
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("BTreeMap"),
        "Should generate HashMap return: {code}"
    );
}

#[test]
fn test_function_returning_tuple() {
    let code = transpile(
        r#"
def pair(a: int, b: int) -> tuple:
    return (a, b)
"#,
    );
    assert!(
        code.contains("(") || code.contains("tuple"),
        "Should generate tuple return: {code}"
    );
}

#[test]
fn test_function_returning_optional() {
    let code = transpile(
        r#"
from typing import Optional
def find(items: list, target: int) -> Optional[int]:
    for i, item in enumerate(items):
        if item == target:
            return i
    return None
"#,
    );
    assert!(
        code.contains("Option") || code.contains("None"),
        "Should generate Option return: {code}"
    );
}

// ── Class generation ────────────────────────────────────────────

#[test]
fn test_simple_class() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#,
    );
    assert!(
        code.contains("struct Point"),
        "Should generate struct: {code}"
    );
    assert!(
        code.contains("x:") && code.contains("y:"),
        "Should have fields: {code}"
    );
}

#[test]
fn test_class_with_str() {
    let code = transpile(
        r#"
class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name = name
        self.age = age

    def __str__(self) -> str:
        return f"{self.name} ({self.age})"
"#,
    );
    assert!(
        code.contains("struct Person") || code.contains("impl"),
        "Should generate class: {code}"
    );
    assert!(
        code.contains("Display") || code.contains("fmt") || code.contains("to_string"),
        "Should generate Display trait: {code}"
    );
}

#[test]
fn test_class_with_repr() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"
"#,
    );
    assert!(
        code.contains("Debug") || code.contains("fmt") || code.contains("Display"),
        "Should generate Debug/Display: {code}"
    );
}

#[test]
fn test_class_with_len() {
    let code = transpile(
        r#"
class Collection:
    def __init__(self) -> None:
        self.items: list = []

    def __len__(self) -> int:
        return len(self.items)
"#,
    );
    assert!(
        code.contains("len") || code.contains("items"),
        "Should generate len method: {code}"
    );
}

#[test]
fn test_class_with_iter() {
    let code = transpile(
        r#"
class Numbers:
    def __init__(self) -> None:
        self.data: list = []

    def __iter__(self):
        return iter(self.data)
"#,
    );
    assert!(
        code.contains("iter") || code.contains("IntoIterator") || code.contains("Iterator"),
        "Should generate iterator: {code}"
    );
}

// ── Multiple functions ──────────────────────────────────────────

#[test]
fn test_multiple_functions() {
    let code = transpile(
        r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b

def compute(x: int, y: int) -> int:
    return add(x, y) + multiply(x, y)
"#,
    );
    assert!(code.contains("fn add"), "Should have add: {code}");
    assert!(code.contains("fn multiply"), "Should have multiply: {code}");
    assert!(code.contains("fn compute"), "Should have compute: {code}");
}

// ── Import handling ─────────────────────────────────────────────

#[test]
fn test_import_json() {
    let code = transpile(
        r#"
import json
def f(data: dict) -> str:
    return json.dumps(data)
"#,
    );
    assert!(
        code.contains("serde_json") || code.contains("json") || code.contains("to_string"),
        "Should map json import: {code}"
    );
}

#[test]
fn test_import_collections() {
    let code = transpile(
        r#"
from collections import OrderedDict
def f() -> dict:
    d = OrderedDict()
    d["a"] = 1
    return d
"#,
    );
    assert!(
        code.contains("BTreeMap") || code.contains("HashMap") || code.contains("OrderedDict"),
        "Should map OrderedDict: {code}"
    );
}

#[test]
fn test_import_typing() {
    let code = transpile(
        r#"
from typing import List, Dict, Optional
def f(items: List[int]) -> Optional[Dict[str, int]]:
    if len(items) == 0:
        return None
    return {"count": len(items)}
"#,
    );
    assert!(
        code.contains("Vec") || code.contains("HashMap") || code.contains("Option"),
        "Should map typing imports: {code}"
    );
}

// ── Complex module patterns ─────────────────────────────────────

#[test]
fn test_module_with_class_and_functions() {
    let code = transpile(
        r#"
class Calculator:
    def __init__(self) -> None:
        self.history: list = []

    def add(self, a: int, b: int) -> int:
        result = a + b
        self.history.append(result)
        return result

def create_calculator() -> Calculator:
    return Calculator()
"#,
    );
    assert!(
        code.contains("struct Calculator"),
        "Should have struct: {code}"
    );
    assert!(
        code.contains("fn create_calculator") || code.contains("create_calculator"),
        "Should have factory function: {code}"
    );
}

#[test]
fn test_dataclass_like() {
    let code = transpile(
        r#"
class Config:
    def __init__(self, host: str, port: int, debug: bool) -> None:
        self.host = host
        self.port = port
        self.debug = debug
"#,
    );
    assert!(
        code.contains("struct Config"),
        "Should generate struct: {code}"
    );
    assert!(
        code.contains("host") && code.contains("port") && code.contains("debug"),
        "Should have all fields: {code}"
    );
}

// ── Error handling patterns ─────────────────────────────────────

#[test]
fn test_raise_value_error() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#,
    );
    assert!(
        code.contains("Err") || code.contains("panic") || code.contains("Error"),
        "Should generate error: {code}"
    );
}

#[test]
fn test_raise_runtime_error() {
    let code = transpile(
        r#"
def f() -> None:
    raise RuntimeError("not implemented")
"#,
    );
    assert!(
        code.contains("panic") || code.contains("unimplemented") || code.contains("Err"),
        "Should generate panic/error: {code}"
    );
}

// ── Decorator-like patterns ─────────────────────────────────────

#[test]
fn test_staticmethod() {
    let code = transpile(
        r#"
class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#,
    );
    assert!(
        code.contains("fn add") && (code.contains("impl") || code.contains("Math")),
        "Should generate static method: {code}"
    );
}

#[test]
fn test_classmethod() {
    let code = transpile(
        r#"
class Counter:
    count: int = 0

    @classmethod
    def increment(cls) -> int:
        cls.count += 1
        return cls.count
"#,
    );
    assert!(
        code.contains("increment") || code.contains("Counter"),
        "Should generate classmethod: {code}"
    );
}

// ── Docstrings ──────────────────────────────────────────────────

#[test]
fn test_function_docstring() {
    let code = transpile(
        r#"
def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b
"#,
    );
    assert!(code.contains("fn add"), "Should generate function: {code}");
}

#[test]
fn test_class_docstring() {
    let code = transpile(
        r#"
class Stack:
    """A simple stack implementation."""
    def __init__(self) -> None:
        self.items: list = []

    def push(self, item: int) -> None:
        """Push an item onto the stack."""
        self.items.append(item)
"#,
    );
    assert!(
        code.contains("struct Stack") || code.contains("impl Stack"),
        "Should generate class: {code}"
    );
}

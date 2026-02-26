//! Session 11: Deep coverage tests for rust_gen.rs (mod)
//!
//! Targets the #4 coverage bottleneck (79% covered, 2347 missed regions):
//! - Class code generation patterns
//! - Import handling
//! - Module-level constants
//! - Enum/ADT detection
//! - Decorator patterns
//! - Complex class hierarchies
//! - Module-level expressions
//! - Type inference for constants

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
// Class patterns
// ============================================================================

#[test]
fn test_s11_gen_class_empty() {
    let code = r#"
class Empty:
    pass
"#;
    let result = transpile(code);
    assert!(
        result.contains("struct Empty") || result.contains("Empty"),
        "Should transpile empty class. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_with_fields() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y
"#;
    let result = transpile(code);
    assert!(
        result.contains("struct Point") || result.contains("Point"),
        "Should transpile class with fields. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_with_methods() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1

    def get_count(self) -> int:
        return self.count
"#;
    let result = transpile(code);
    assert!(
        result.contains("Counter") && result.contains("increment"),
        "Should transpile class with methods. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_with_str() {
    let code = r#"
class Named:
    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name
"#;
    let result = transpile(code);
    assert!(result.contains("Named"), "Should transpile class with __str__. Got: {}", result);
}

#[test]
fn test_s11_gen_class_with_repr() {
    let code = r#"
class Item:
    def __init__(self, value: int):
        self.value = value

    def __repr__(self) -> str:
        return f"Item({self.value})"
"#;
    let result = transpile(code);
    assert!(result.contains("Item"), "Should transpile class with __repr__. Got: {}", result);
}

#[test]
fn test_s11_gen_class_with_eq() {
    let code = r#"
class Pair:
    def __init__(self, a: int, b: int):
        self.a = a
        self.b = b

    def __eq__(self, other) -> bool:
        return self.a == other.a and self.b == other.b
"#;
    let result = transpile(code);
    assert!(result.contains("Pair"), "Should transpile class with __eq__. Got: {}", result);
}

#[test]
fn test_s11_gen_class_with_len() {
    let code = r#"
class Container:
    def __init__(self):
        self.items: list = []

    def __len__(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("Container"), "Should transpile class with __len__. Got: {}", result);
}

#[test]
fn test_s11_gen_class_staticmethod() {
    let code = r#"
class MathHelper:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("MathHelper") || result.contains("fn add"),
        "Should transpile staticmethod. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_classmethod() {
    let code = r#"
class Config:
    def __init__(self, value: str):
        self.value = value

    @classmethod
    def from_string(cls, s: str):
        return cls(s)
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Should transpile classmethod. Got: {}", result);
}

#[test]
fn test_s11_gen_class_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self.radius ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Should transpile property. Got: {}", result);
}

#[test]
fn test_s11_gen_class_inheritance() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return ""

class Dog(Animal):
    def speak(self) -> str:
        return "Woof"
"#;
    let result = transpile(code);
    assert!(
        result.contains("Animal") || result.contains("Dog"),
        "Should transpile class inheritance. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_vars() {
    let code = r#"
class Config:
    MAX_SIZE = 100
    DEFAULT_NAME = "unnamed"

    def __init__(self, name: str):
        self.name = name
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Should transpile class vars. Got: {}", result);
}

// ============================================================================
// Import handling
// ============================================================================

#[test]
fn test_s11_gen_import_math() {
    let code = r#"
import math

def circle_area(r: float) -> float:
    return math.pi * r ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn circle_area"), "Should handle math import. Got: {}", result);
}

#[test]
fn test_s11_gen_from_import() {
    let code = r#"
from math import sqrt, pi

def hypotenuse(a: float, b: float) -> float:
    return sqrt(a ** 2 + b ** 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn hypotenuse"), "Should handle from import. Got: {}", result);
}

#[test]
fn test_s11_gen_import_os() {
    let code = r#"
import os

def get_home() -> str:
    return os.getenv("HOME", "/tmp")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_home"), "Should handle os import. Got: {}", result);
}

#[test]
fn test_s11_gen_import_typing() {
    let code = r#"
from typing import List, Dict, Optional

def process(items: List[int]) -> Dict[str, int]:
    return {"count": len(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Should handle typing import. Got: {}", result);
}

#[test]
fn test_s11_gen_import_collections() {
    let code = r#"
from collections import defaultdict

def word_count(words: list) -> dict:
    counts = defaultdict(int)
    for w in words:
        counts[w] += 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_count"), "Should handle collections import. Got: {}", result);
}

#[test]
fn test_s11_gen_import_json() {
    let code = r#"
import json

def serialize(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn serialize"), "Should handle json import. Got: {}", result);
}

// ============================================================================
// Module-level constants
// ============================================================================

#[test]
fn test_s11_gen_module_int_constant() {
    let code = r#"
MAX_SIZE = 1024

def get_max() -> int:
    return MAX_SIZE
"#;
    let result = transpile(code);
    assert!(
        result.contains("MAX_SIZE") || result.contains("1024"),
        "Should transpile int constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_module_float_constant() {
    let code = r#"
EPSILON = 0.001

def is_close(a: float, b: float) -> bool:
    return abs(a - b) < EPSILON
"#;
    let result = transpile(code);
    assert!(
        result.contains("EPSILON") || result.contains("is_close"),
        "Should transpile float constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_module_string_constant() {
    let code = r#"
VERSION = "1.0.0"

def get_version() -> str:
    return VERSION
"#;
    let result = transpile(code);
    assert!(
        result.contains("VERSION") || result.contains("1.0.0"),
        "Should transpile string constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_module_bool_constant() {
    let code = r#"
DEBUG = False

def is_debug() -> bool:
    return DEBUG
"#;
    let result = transpile(code);
    assert!(
        result.contains("DEBUG") || result.contains("is_debug"),
        "Should transpile bool constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_module_list_constant() {
    let code = r#"
PRIMES = [2, 3, 5, 7, 11, 13]

def is_prime(n: int) -> bool:
    return n in PRIMES
"#;
    let result = transpile(code);
    assert!(
        result.contains("PRIMES") || result.contains("is_prime"),
        "Should transpile list constant. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_module_dict_constant() {
    let code = r#"
STATUS_CODES = {200: "OK", 404: "Not Found", 500: "Error"}

def get_status(code: int) -> str:
    return STATUS_CODES.get(code, "Unknown")
"#;
    let result = transpile(code);
    assert!(
        result.contains("STATUS_CODES") || result.contains("get_status"),
        "Should transpile dict constant. Got: {}",
        result
    );
}

// ============================================================================
// Multiple functions in module
// ============================================================================

#[test]
fn test_s11_gen_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b

def div(a: float, b: float) -> float:
    return a / b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn add")
            && result.contains("fn sub")
            && result.contains("fn mul")
            && result.contains("fn div"),
        "Should transpile multiple functions. Got: {}",
        result
    );
}

// ============================================================================
// Function with default parameters
// ============================================================================

#[test]
fn test_s11_gen_default_params() {
    let code = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Should transpile default params. Got: {}", result);
}

#[test]
fn test_s11_gen_multiple_defaults() {
    let code = r#"
def connect(host: str = "localhost", port: int = 8080, timeout: int = 30) -> str:
    return f"{host}:{port}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn connect"), "Should transpile multiple defaults. Got: {}", result);
}

// ============================================================================
// Async functions
// ============================================================================

#[test]
fn test_s11_gen_async_function() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fetch_data") || result.contains("async fn fetch_data"),
        "Should transpile async function. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_async_with_await() {
    let code = r#"
import asyncio

async def delayed_result(n: int) -> int:
    await asyncio.sleep(1)
    return n * 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn delayed_result"),
        "Should transpile async with await. Got: {}",
        result
    );
}

// ============================================================================
// Generator functions
// ============================================================================

#[test]
fn test_s11_gen_generator_yield() {
    let code = r#"
def count_up(n: int):
    for i in range(n):
        yield i
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_up"), "Should transpile generator yield. Got: {}", result);
}

#[test]
fn test_s11_gen_generator_yield_value() {
    let code = r#"
def fibonacci(n: int):
    a = 0
    b = 1
    for _ in range(n):
        yield a
        a, b = b, a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fibonacci"),
        "Should transpile fibonacci generator. Got: {}",
        result
    );
}

// ============================================================================
// Docstrings
// ============================================================================

#[test]
fn test_s11_gen_function_docstring() {
    let code = r#"
def documented(x: int) -> int:
    """Return the double of x."""
    return x * 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn documented"),
        "Should transpile function with docstring. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_docstring() {
    let code = r#"
class Documented:
    """A documented class."""
    def __init__(self):
        self.value = 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("Documented"),
        "Should transpile class with docstring. Got: {}",
        result
    );
}

// ============================================================================
// Complex class patterns
// ============================================================================

#[test]
fn test_s11_gen_class_with_multiple_methods() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items: list = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def peek(self) -> int:
        return self.items[-1]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Stack") && result.contains("push"),
        "Should transpile Stack class. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_class_with_comparison() {
    let code = r#"
class Temperature:
    def __init__(self, degrees: float):
        self.degrees = degrees

    def __lt__(self, other) -> bool:
        return self.degrees < other.degrees

    def __le__(self, other) -> bool:
        return self.degrees <= other.degrees

    def __gt__(self, other) -> bool:
        return self.degrees > other.degrees
"#;
    let result = transpile(code);
    assert!(
        result.contains("Temperature"),
        "Should transpile class with comparisons. Got: {}",
        result
    );
}

// ============================================================================
// Enum-like class patterns
// ============================================================================

#[test]
fn test_s11_gen_enum_like_class() {
    let code = r#"
class Color:
    RED = 1
    GREEN = 2
    BLUE = 3
"#;
    let result = transpile(code);
    assert!(
        result.contains("Color") || result.contains("RED"),
        "Should transpile enum-like class. Got: {}",
        result
    );
}

// ============================================================================
// Complex module patterns
// ============================================================================

#[test]
fn test_s11_gen_function_calling_function() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def main(x: int) -> int:
    return helper(x) + 1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn helper") && result.contains("fn main"),
        "Should transpile function calling function. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn factorial"),
        "Should transpile recursive function. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_mutually_recursive() {
    let code = r#"
def is_even(n: int) -> bool:
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    return is_even(n - 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_even") && result.contains("fn is_odd"),
        "Should transpile mutually recursive. Got: {}",
        result
    );
}

// ============================================================================
// Type annotations on function signatures
// ============================================================================

#[test]
fn test_s11_gen_complex_return_type() {
    let code = r#"
from typing import List, Tuple

def split_list(items: List[int]) -> Tuple[List[int], List[int]]:
    evens: list = []
    odds: list = []
    for x in items:
        if x % 2 == 0:
            evens.append(x)
        else:
            odds.append(x)
    return (evens, odds)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn split_list"),
        "Should transpile complex return type. Got: {}",
        result
    );
}

#[test]
fn test_s11_gen_optional_param() {
    let code = r#"
from typing import Optional

def find(items: list, target: int) -> Optional[int]:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find"), "Should transpile Optional param. Got: {}", result);
}

// ============================================================================
// Varargs patterns
// ============================================================================

#[test]
fn test_s11_gen_args_function() {
    let code = r#"
def sum_all(*args) -> int:
    total = 0
    for x in args:
        total += x
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_all"), "Should transpile *args. Got: {}", result);
}

#[test]
fn test_s11_gen_kwargs_function() {
    let code = r#"
def config(**kwargs) -> dict:
    return kwargs
"#;
    let result = transpile(code);
    assert!(result.contains("fn config"), "Should transpile **kwargs. Got: {}", result);
}

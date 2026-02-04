//! Coverage tests for rust_gen.rs
//!
//! DEPYLER-99MODE-001: Targets rust_gen.rs (78.48% -> 85%+)
//! Covers: class generation, OOP patterns, exception handling,
//! module-level code, import handling, complex type scenarios.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Class generation - various patterns
// ============================================================================

#[test]
fn test_class_with_multiple_methods() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []
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
    let rust = transpile(code);
    assert!(rust.contains("struct Stack"));
}

#[test]
fn test_class_with_str_fields() {
    let code = r#"
class User:
    def __init__(self, name: str, email: str):
        self.name = name
        self.email = email
    def display(self) -> str:
        return self.name + " <" + self.email + ">"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_inheritance_simple() {
    let code = r#"
class Shape:
    def __init__(self):
        self.color = "black"
    def area(self) -> float:
        return 0.0

class Circle(Shape):
    def __init__(self, radius: float):
        self.radius = radius
        self.color = "red"
    def area(self) -> float:
        return 3.14159 * self.radius * self.radius
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dataclass_simple() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Point:
    x: float
    y: float
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dataclass_with_defaults() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Config:
    host: str = "localhost"
    port: int = 8080
    debug: bool = False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dataclass_with_methods() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Rectangle:
    width: float
    height: float

    def area(self) -> float:
        return self.width * self.height

    def perimeter(self) -> float:
        return 2.0 * (self.width + self.height)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_enum_class() {
    let code = r#"
from enum import Enum

class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_enum_str() {
    let code = r#"
from enum import Enum

class Direction(Enum):
    NORTH = "N"
    SOUTH = "S"
    EAST = "E"
    WEST = "W"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_with_classmethod() {
    let code = r#"
class Counter:
    count = 0

    def __init__(self):
        Counter.count += 1

    @classmethod
    def get_count(cls) -> int:
        return cls.count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_with_staticmethod() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def multiply(a: int, b: int) -> int:
        return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_with_property() {
    let code = r#"
class Temperature:
    def __init__(self, celsius: float):
        self._celsius = celsius

    @property
    def fahrenheit(self) -> float:
        return self._celsius * 1.8 + 32.0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Exception handling patterns
// ============================================================================

#[test]
fn test_try_except_value_error() {
    let code = r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_key_error() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    try:
        return d[key]
    except KeyError:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_multiple_types() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
    except Exception:
        return -99
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_finally() {
    let code = r#"
def process(data: str) -> int:
    result = 0
    try:
        result = int(data)
    except ValueError:
        result = -1
    finally:
        print("done")
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_else() {
    let code = r#"
def divide(a: int, b: int) -> int:
    try:
        result = a // b
    except ZeroDivisionError:
        return 0
    else:
        return result * 2
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_value_error() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be non-negative")
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_runtime_error() {
    let code = r#"
def not_implemented():
    raise RuntimeError("not implemented yet")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_custom_exception_class() {
    let code = r#"
class AppError(Exception):
    def __init__(self, message: str, code: int):
        self.message = message
        self.code = code
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Module-level constants and globals
// ============================================================================

#[test]
fn test_module_constants() {
    let code = r#"
MAX_RETRIES = 3
DEFAULT_TIMEOUT = 30
APP_NAME = "myapp"

def get_name() -> str:
    return APP_NAME
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_module_constant_types() {
    let code = r#"
PI = 3.14159
E = 2.71828
MAX_INT = 2147483647
IS_DEBUG = False

def get_pi() -> float:
    return PI
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multiple_functions() {
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
    let rust = transpile(code);
    assert!(rust.contains("fn add"));
    assert!(rust.contains("fn sub"));
    assert!(rust.contains("fn mul"));
    assert!(rust.contains("fn div"));
}

// ============================================================================
// Import patterns
// ============================================================================

#[test]
fn test_import_typing() {
    let code = r#"
from typing import List, Dict, Optional, Tuple, Set

def f(items: List[int]) -> Dict[str, int]:
    result: Dict[str, int] = {}
    for i, item in enumerate(items):
        result[str(i)] = item
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_import_collections() {
    let code = r#"
from collections import Counter

def count_chars(text: str) -> int:
    c = Counter(text)
    return len(c)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_import_math() {
    let code = r#"
import math

def circle_area(radius: float) -> float:
    return math.pi * radius * radius

def hypotenuse(a: float, b: float) -> float:
    return math.sqrt(a * a + b * b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_import_sys() {
    let code = r#"
import sys

def get_args() -> list:
    return sys.argv
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_import_os() {
    let code = r#"
import os

def get_env(key: str) -> str:
    return os.environ.get(key, "")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_import_json() {
    let code = r#"
import json

def parse(text: str) -> dict:
    return json.loads(text)

def serialize(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex type scenarios
// ============================================================================

#[test]
fn test_optional_parameter() {
    let code = r#"
from typing import Optional

def greet(name: Optional[str] = None) -> str:
    if name is None:
        return "Hello, World!"
    return "Hello, " + name + "!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_of_tuples() {
    let code = r#"
from typing import List, Tuple

def pairs(n: int) -> List[Tuple[int, int]]:
    result = []
    for i in range(n):
        result.append((i, i * i))
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_of_lists() {
    let code = r#"
from typing import Dict, List

def group_by_mod(items: List[int], m: int) -> Dict[int, List[int]]:
    groups: Dict[int, List[int]] = {}
    for item in items:
        key = item % m
        if key not in groups:
            groups[key] = []
        groups[key].append(item)
    return groups
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_optional() {
    let code = r#"
from typing import Optional, Dict

def find_value(data: Dict[str, int], key: str) -> Optional[int]:
    if key in data:
        return data[key]
    return None
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator functions
// ============================================================================

#[test]
fn test_generator_count_up() {
    let code = r#"
def count_up(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_fibonacci() {
    let code = r#"
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_filter() {
    let code = r#"
def evens(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Async functions
// ============================================================================

#[test]
fn test_async_basic() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_async_with_await() {
    let code = r#"
async def process(url: str) -> str:
    data = await fetch(url)
    return data
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested functions and closures
// ============================================================================

#[test]
fn test_nested_function_basic() {
    let code = r#"
def outer() -> int:
    def inner(x: int) -> int:
        return x * 2
    return inner(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_closure_capturing_var() {
    let code = r#"
def make_adder(n: int):
    def add(x: int) -> int:
        return x + n
    return add
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multiple_nested_functions() {
    let code = r#"
def outer() -> int:
    def double(x: int) -> int:
        return x * 2
    def triple(x: int) -> int:
        return x * 3
    return double(5) + triple(3)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Protocol and ABC patterns
// ============================================================================

#[test]
fn test_protocol_class() {
    let code = r#"
from typing import Protocol

class Serializable(Protocol):
    def serialize(self) -> str: ...
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_abstract_class() {
    let code = r#"
class Processor:
    def process(self, data: str) -> str:
        raise NotImplementedError("subclass must implement")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow patterns exercising rust_gen paths
// ============================================================================

#[test]
fn test_nested_for_loops() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total += val
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_with_complex_condition() {
    let code = r#"
def find_threshold(items: list, target: int) -> int:
    i = 0
    total = 0
    while i < len(items) and total < target:
        total += items[i]
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_if_elif_else() {
    let code = r#"
def classify(x: int, y: int) -> str:
    if x > 0 and y > 0:
        return "Q1"
    elif x < 0 and y > 0:
        return "Q2"
    elif x < 0 and y < 0:
        return "Q3"
    elif x > 0 and y < 0:
        return "Q4"
    else:
        return "origin"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns combining multiple features
// ============================================================================

#[test]
fn test_class_with_class_variable() {
    let code = r#"
class Logger:
    level = 0

    def __init__(self, name: str):
        self.name = name

    def log(self, message: str):
        print(self.name + ": " + message)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_function_returning_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def origin() -> Point:
    return Point(0, 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_using_other_class() {
    let code = r#"
class Node:
    def __init__(self, value: int):
        self.value = value
        self.next = None

def create_list(values: list) -> Node:
    head = Node(values[0])
    current = head
    for v in values[1:]:
        current.next = Node(v)
        current = current.next
    return head
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multiple_classes() {
    let code = r#"
class Engine:
    def __init__(self, hp: int):
        self.hp = hp

class Car:
    def __init__(self, name: str, hp: int):
        self.name = name
        self.engine = Engine(hp)

    def describe(self) -> str:
        return self.name
"#;
    let rust = transpile(code);
    assert!(rust.contains("struct Engine"));
    assert!(rust.contains("struct Car"));
}

#[test]
fn test_function_with_many_params() {
    let code = r#"
def configure(
    host: str,
    port: int,
    debug: bool,
    workers: int,
    timeout: float
) -> str:
    return host
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_function_with_defaults() {
    let code = r#"
def connect(
    host: str = "localhost",
    port: int = 5432,
    timeout: int = 30
) -> str:
    return host
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutual_recursion() {
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
    assert!(transpile_ok(code));
}

#[test]
fn test_main_function() {
    let code = r#"
def main():
    print("Hello, World!")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn main"));
}

#[test]
fn test_main_with_args() {
    let code = r#"
import sys

def main():
    args = sys.argv
    if len(args) > 1:
        print(args[1])
    else:
        print("no args")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Decorator patterns
// ============================================================================

#[test]
fn test_simple_decorator() {
    let code = r#"
def my_decorator(func):
    def wrapper():
        return func()
    return wrapper

@my_decorator
def hello() -> str:
    return "hello"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String return type inference
// ============================================================================

#[test]
fn test_string_return_concat() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello, " + name + "!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_fstring() {
    let code = r#"
def greet(name: str, age: int) -> str:
    return f"{name} is {age}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_method() {
    let code = r#"
def normalize(text: str) -> str:
    return text.strip().lower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_conditional() {
    let code = r#"
def label(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable hoisting patterns
// ============================================================================

#[test]
fn test_variable_hoisting_if() {
    let code = r#"
def f(flag: bool) -> int:
    if flag:
        result = 10
    else:
        result = 20
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_variable_hoisting_loop() {
    let code = r#"
def f(items: list) -> int:
    last = 0
    for item in items:
        last = item
    return last
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_variable_hoisting_try() {
    let code = r#"
def f(s: str) -> int:
    try:
        value = int(s)
    except ValueError:
        value = 0
    return value
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Numeric type coercion
// ============================================================================

#[test]
fn test_int_division_returns_float() {
    let code = r#"
def average(items: list) -> float:
    return sum(items) / len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mixed_arithmetic() {
    let code = r#"
def compute(a: int, b: float) -> float:
    return a * b + 1.0
"#;
    assert!(transpile_ok(code));
}

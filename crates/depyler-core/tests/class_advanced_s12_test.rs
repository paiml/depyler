//! Session 12 Batch 75: Advanced class patterns and OOP cold paths
//!
//! Targets class codegen, inheritance, property patterns,
//! and complex class interactions.

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

#[test]
fn test_s12_b75_class_with_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self.radius * self.radius

    @property
    def diameter(self) -> float:
        return 2.0 * self.radius
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_repr() {
    let code = r##"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"##;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_add() {
    let code = r#"
class Vector2D:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vector2D(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Vector2D(self.x - other.x, self.y - other.y)

    def length(self) -> float:
        return (self.x * self.x + self.y * self.y) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("Vector2D"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_len() {
    let code = r#"
class Buffer:
    def __init__(self):
        self.data = []

    def __len__(self) -> int:
        return len(self.data)

    def add(self, item: int):
        self.data.append(item)

    def is_empty(self) -> bool:
        return len(self.data) == 0
"#;
    let result = transpile(code);
    assert!(result.contains("Buffer"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_contains() {
    let code = r#"
class IntSet:
    def __init__(self):
        self.items = set()

    def __contains__(self, item: int) -> bool:
        return item in self.items

    def add(self, item: int):
        self.items.add(item)

    def remove(self, item: int):
        self.items.discard(item)
"#;
    let result = transpile(code);
    assert!(result.contains("IntSet"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_inheritance() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return ""

class Dog(Animal):
    def speak(self) -> str:
        return self.name + " says Woof!"

class Cat(Animal):
    def speak(self) -> str:
        return self.name + " says Meow!"
"#;
    let result = transpile(code);
    assert!(result.contains("Animal"), "Got: {}", result);
    assert!(result.contains("Dog"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_classmethod() {
    let code = r#"
class Color:
    def __init__(self, r: int, g: int, b: int):
        self.r = r
        self.g = g
        self.b = b

    @classmethod
    def red(cls):
        return Color(255, 0, 0)

    @classmethod
    def green(cls):
        return Color(0, 255, 0)

    @classmethod
    def blue(cls):
        return Color(0, 0, 255)
"#;
    let result = transpile(code);
    assert!(result.contains("Color"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_static() {
    let code = r#"
class MathUtils:
    @staticmethod
    def gcd(a: int, b: int) -> int:
        while b > 0:
            a, b = b, a % b
        return a

    @staticmethod
    def lcm(a: int, b: int) -> int:
        return a * b // MathUtils.gcd(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("MathUtils"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_iter() {
    let code = r#"
class Range:
    def __init__(self, start: int, stop: int):
        self.start = start
        self.stop = stop

    def __iter__(self):
        self.current = self.start
        return self

    def __next__(self) -> int:
        if self.current >= self.stop:
            raise StopIteration
        value = self.current
        self.current += 1
        return value
"#;
    let result = transpile(code);
    assert!(result.contains("Range"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_getitem() {
    let code = r#"
class Grid:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data = [0] * (rows * cols)

    def __getitem__(self, key: tuple) -> int:
        row, col = key
        return self.data[row * self.cols + col]

    def __setitem__(self, key: tuple, value: int):
        row, col = key
        self.data[row * self.cols + col] = value
"#;
    let result = transpile(code);
    assert!(result.contains("Grid"), "Got: {}", result);
}

#[test]
fn test_s12_b75_dataclass_like() {
    let code = r#"
class Config:
    def __init__(self, host: str, port: int, debug: bool, timeout: float):
        self.host = host
        self.port = port
        self.debug = debug
        self.timeout = timeout

    def is_development(self) -> bool:
        return self.debug and self.host == "localhost"

    def connection_string(self) -> str:
        return self.host + ":" + str(self.port)
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_with_multiple_types() {
    let code = r#"
class Cache:
    def __init__(self, max_size: int):
        self.max_size = max_size
        self.data = {}
        self.order = []

    def get(self, key: str) -> str:
        return self.data.get(key, "")

    def put(self, key: str, value: str):
        if key in self.data:
            self.order.remove(key)
        elif len(self.data) >= self.max_size:
            oldest = self.order.pop(0)
            del self.data[oldest]
        self.data[key] = value
        self.order.append(key)

    def size(self) -> int:
        return len(self.data)
"#;
    let result = transpile(code);
    assert!(result.contains("Cache"), "Got: {}", result);
}

#[test]
fn test_s12_b75_class_composition() {
    let code = r#"
class Address:
    def __init__(self, street: str, city: str, state: str):
        self.street = street
        self.city = city
        self.state = state

class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        self.address = None

    def set_address(self, street: str, city: str, state: str):
        self.address = Address(street, city, state)
"#;
    let result = transpile(code);
    assert!(result.contains("Address"), "Got: {}", result);
    assert!(result.contains("Person"), "Got: {}", result);
}

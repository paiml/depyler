//! Session 12 Batch 50: Class generation and ADT cold paths
//!
//! Targets cold paths in rust_gen/mod.rs and class codegen:
//! - ADT pattern detection (base class filtering)
//! - Complex class hierarchies
//! - Class with many field types
//! - Dataclass-like patterns
//! - Class with both instance and class methods
//! - Enum-like class patterns
//! - Class with operator overloading

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

// ===== Complex class hierarchies =====

#[test]
fn test_s12_b50_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_methods() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def magnitude(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5

    def scale(self, factor: float):
        self.x *= factor
        self.y *= factor

    def dot(self, other) -> float:
        return self.x * other.x + self.y * other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Vector"), "Got: {}", result);
    assert!(result.contains("magnitude"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_str() {
    let code = r##"
class Color:
    def __init__(self, r: int, g: int, b: int):
        self.r = r
        self.g = g
        self.b = b

    def __str__(self) -> str:
        return f"rgb({self.r}, {self.g}, {self.b})"

    def hex(self) -> str:
        return f"#{self.r:02x}{self.g:02x}{self.b:02x}"
"##;
    let result = transpile(code);
    assert!(result.contains("Color"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_inheritance() {
    let code = r#"
class Shape:
    def __init__(self):
        self.name = "shape"

    def area(self) -> float:
        return 0.0

class Rectangle(Shape):
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height
        self.name = "rectangle"

    def area(self) -> float:
        return self.width * self.height
"#;
    let result = transpile(code);
    assert!(result.contains("Rectangle"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_list_field() {
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
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_dict_field() {
    let code = r##"
class Registry:
    def __init__(self):
        self.entries = {}
        self.count = 0

    def register(self, name: str, value: int):
        self.entries[name] = value
        self.count += 1

    def lookup(self, name: str) -> int:
        return self.entries.get(name, -1)

    def unregister(self, name: str) -> bool:
        if name in self.entries:
            del self.entries[name]
            self.count -= 1
            return True
        return False

    def all_names(self) -> list:
        return list(self.entries.keys())
"##;
    let result = transpile(code);
    assert!(result.contains("Registry"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_bool_fields() {
    let code = r#"
class Toggle:
    def __init__(self):
        self.active = False
        self.locked = False

    def toggle(self):
        if not self.locked:
            self.active = not self.active

    def lock(self):
        self.locked = True

    def unlock(self):
        self.locked = False

    def is_active(self) -> bool:
        return self.active
"#;
    let result = transpile(code);
    assert!(result.contains("Toggle"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_class_method() {
    let code = r#"
class Temperature:
    def __init__(self, celsius: float):
        self.celsius = celsius

    @classmethod
    def from_fahrenheit(cls, f: float):
        return cls((f - 32.0) * 5.0 / 9.0)

    def to_fahrenheit(self) -> float:
        return self.celsius * 9.0 / 5.0 + 32.0
"#;
    let result = transpile(code);
    assert!(result.contains("Temperature"), "Got: {}", result);
}

#[test]
fn test_s12_b50_class_with_static() {
    let code = r#"
class Validator:
    @staticmethod
    def is_email(s: str) -> bool:
        return "@" in s and "." in s

    @staticmethod
    def is_positive(n: int) -> bool:
        return n > 0

    @staticmethod
    def is_nonempty(s: str) -> bool:
        return len(s) > 0
"#;
    let result = transpile(code);
    assert!(result.contains("Validator"), "Got: {}", result);
}

// ===== Enum-like patterns =====

#[test]
fn test_s12_b50_enum_like_class() {
    let code = r##"
class Direction:
    NORTH = "N"
    SOUTH = "S"
    EAST = "E"
    WEST = "W"

    @staticmethod
    def opposite(d: str) -> str:
        if d == "N":
            return "S"
        elif d == "S":
            return "N"
        elif d == "E":
            return "W"
        else:
            return "E"
"##;
    let result = transpile(code);
    assert!(result.contains("Direction"), "Got: {}", result);
}

// ===== Multiple classes in one module =====

#[test]
fn test_s12_b50_two_classes() {
    let code = r#"
class Node:
    def __init__(self, value: int):
        self.value = value
        self.next = None

class LinkedList:
    def __init__(self):
        self.head = None
        self.size = 0

    def push(self, value: int):
        node = Node(value)
        node.next = self.head
        self.head = node
        self.size += 1

    def length(self) -> int:
        return self.size
"#;
    let result = transpile(code);
    assert!(result.contains("Node"), "Got: {}", result);
    assert!(result.contains("LinkedList"), "Got: {}", result);
}

// ===== Class with comparison methods =====

#[test]
fn test_s12_b50_class_with_eq() {
    let code = r#"
class Pair:
    def __init__(self, first: int, second: int):
        self.first = first
        self.second = second

    def __eq__(self, other) -> bool:
        return self.first == other.first and self.second == other.second

    def sum(self) -> int:
        return self.first + self.second
"#;
    let result = transpile(code);
    assert!(result.contains("Pair"), "Got: {}", result);
}

// ===== Dataclass-like pattern =====

#[test]
fn test_s12_b50_data_class() {
    let code = r##"
class Employee:
    def __init__(self, name: str, department: str, salary: float, active: bool):
        self.name = name
        self.department = department
        self.salary = salary
        self.active = active

    def give_raise(self, amount: float):
        self.salary += amount

    def deactivate(self):
        self.active = False

    def to_string(self) -> str:
        return f"{self.name} ({self.department}): ${self.salary}"
"##;
    let result = transpile(code);
    assert!(result.contains("Employee"), "Got: {}", result);
}

// ===== Class with complex method interactions =====

#[test]
fn test_s12_b50_matrix_class() {
    let code = r#"
class Matrix:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data = []
        for i in range(rows):
            row = []
            for j in range(cols):
                row.append(0.0)
            self.data.append(row)

    def set(self, r: int, c: int, val: float):
        self.data[r][c] = val

    def get(self, r: int, c: int) -> float:
        return self.data[r][c]

    def add(self, other):
        result = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            for j in range(self.cols):
                result.data[i][j] = self.data[i][j] + other.data[i][j]
        return result

    def trace(self) -> float:
        total = 0.0
        for i in range(min(self.rows, self.cols)):
            total += self.data[i][i]
        return total
"#;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

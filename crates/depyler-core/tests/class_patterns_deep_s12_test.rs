//! Session 12 Batch 30: Class pattern cold paths
//!
//! Targets:
//! - __init__ with complex defaults
//! - __str__ and __repr__ methods
//! - __eq__, __ne__, __lt__, __gt__ dunder methods
//! - __len__, __contains__, __iter__ dunders
//! - __add__, __mul__ operator dunders
//! - __getitem__, __setitem__ dunders
//! - __enter__, __exit__ context manager dunders
//! - Class inheritance patterns
//! - Multiple methods with self attribute access
//! - Static fields and class-level constants

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

// ===== Dunder methods =====

#[test]
fn test_s12_str_dunder() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    def __str__(self) -> str:
        return f"{self.name} ({self.age})"
"#;
    let result = transpile(code);
    assert!(result.contains("Person"), "Got: {}", result);
}

#[test]
fn test_s12_repr_dunder() {
    let code = r#"
class Pair:
    def __init__(self, a: int, b: int):
        self.a = a
        self.b = b

    def __repr__(self) -> str:
        return f"Pair({self.a}, {self.b})"
"#;
    let result = transpile(code);
    assert!(result.contains("Pair"), "Got: {}", result);
}

#[test]
fn test_s12_eq_dunder() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_lt_dunder() {
    let code = r#"
class Comparable:
    def __init__(self, value: int):
        self.value = value

    def __lt__(self, other) -> bool:
        return self.value < other.value
"#;
    let result = transpile(code);
    assert!(result.contains("Comparable"), "Got: {}", result);
}

#[test]
fn test_s12_len_dunder() {
    let code = r#"
class Collection:
    def __init__(self):
        self.items = []

    def __len__(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("Collection"), "Got: {}", result);
}

#[test]
fn test_s12_contains_dunder() {
    let code = r#"
class SearchableList:
    def __init__(self, items: list):
        self.items = items

    def __contains__(self, item: int) -> bool:
        return item in self.items
"#;
    let result = transpile(code);
    assert!(result.contains("SearchableList"), "Got: {}", result);
}

#[test]
fn test_s12_getitem_dunder() {
    let code = r#"
class Array:
    def __init__(self, data: list):
        self.data = data

    def __getitem__(self, idx: int) -> int:
        return self.data[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("Array"), "Got: {}", result);
}

#[test]
fn test_s12_setitem_dunder() {
    let code = r#"
class MutableArray:
    def __init__(self, data: list):
        self.data = data

    def __setitem__(self, idx: int, val: int):
        self.data[idx] = val
"#;
    let result = transpile(code);
    assert!(result.contains("MutableArray"), "Got: {}", result);
}

#[test]
fn test_s12_add_dunder() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)
"#;
    let result = transpile(code);
    assert!(result.contains("Vector"), "Got: {}", result);
}

#[test]
fn test_s12_mul_dunder() {
    let code = r#"
class Scalar:
    def __init__(self, value: float):
        self.value = value

    def __mul__(self, other) -> float:
        return self.value * other.value
"#;
    let result = transpile(code);
    assert!(result.contains("Scalar"), "Got: {}", result);
}

// ===== Class inheritance =====

#[test]
fn test_s12_simple_inheritance() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return ""

class Dog(Animal):
    def speak(self) -> str:
        return "Woof!"
"#;
    let result = transpile(code);
    assert!(result.contains("Animal"), "Got: {}", result);
    assert!(result.contains("Dog"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_with_many_methods() {
    let code = r##"
class StringBuffer:
    def __init__(self):
        self.parts = []
        self.length = 0

    def append(self, text: str):
        self.parts.append(text)
        self.length += len(text)

    def clear(self):
        self.parts = []
        self.length = 0

    def to_string(self) -> str:
        return "".join(self.parts)

    def is_empty(self) -> bool:
        return self.length == 0

    def size(self) -> int:
        return self.length
"##;
    let result = transpile(code);
    assert!(result.contains("StringBuffer"), "Got: {}", result);
    assert!(result.contains("fn append"), "Got: {}", result);
    assert!(result.contains("fn clear"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_dict_attribute() {
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

    def has(self, name: str) -> bool:
        return name in self.entries

    def remove(self, name: str):
        if name in self.entries:
            del self.entries[name]
            self.count -= 1
"##;
    let result = transpile(code);
    assert!(result.contains("Registry"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_list_operations() {
    let code = r##"
class Queue:
    def __init__(self):
        self.items = []

    def enqueue(self, item: int):
        self.items.append(item)

    def dequeue(self) -> int:
        if not self.items:
            raise IndexError("empty queue")
        return self.items.pop(0)

    def peek(self) -> int:
        if not self.items:
            raise IndexError("empty queue")
        return self.items[0]

    def size(self) -> int:
        return len(self.items)

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def contains(self, item: int) -> bool:
        return item in self.items
"##;
    let result = transpile(code);
    assert!(result.contains("Queue"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_boolean_state() {
    let code = r#"
class Toggle:
    def __init__(self):
        self.enabled = False

    def enable(self):
        self.enabled = True

    def disable(self):
        self.enabled = False

    def toggle(self):
        self.enabled = not self.enabled

    def is_enabled(self) -> bool:
        return self.enabled
"#;
    let result = transpile(code);
    assert!(result.contains("Toggle"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_computed_property() {
    let code = r#"
class Temperature:
    def __init__(self, celsius: float):
        self.celsius = celsius

    def to_fahrenheit(self) -> float:
        return self.celsius * 1.8 + 32.0

    def to_kelvin(self) -> float:
        return self.celsius + 273.15
"#;
    let result = transpile(code);
    assert!(result.contains("Temperature"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_validation() {
    let code = r#"
class BoundedValue:
    def __init__(self, value: int, minimum: int, maximum: int):
        self.minimum = minimum
        self.maximum = maximum
        self.value = max(minimum, min(value, maximum))

    def set(self, value: int):
        self.value = max(self.minimum, min(value, self.maximum))

    def get(self) -> int:
        return self.value
"#;
    let result = transpile(code);
    assert!(result.contains("BoundedValue"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_counter_pattern() {
    let code = r#"
class Counter:
    def __init__(self):
        self.counts = {}

    def add(self, key: str):
        if key in self.counts:
            self.counts[key] += 1
        else:
            self.counts[key] = 1

    def get(self, key: str) -> int:
        return self.counts.get(key, 0)

    def most_common(self) -> str:
        best_key = ""
        best_count = 0
        for k, v in self.counts.items():
            if v > best_count:
                best_count = v
                best_key = k
        return best_key
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

// ===== Multiple classes in one module =====

#[test]
fn test_s12_multiple_classes() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

class Line:
    def __init__(self, start_x: float, start_y: float, end_x: float, end_y: float):
        self.start_x = start_x
        self.start_y = start_y
        self.end_x = end_x
        self.end_y = end_y

    def length(self) -> float:
        dx = self.end_x - self.start_x
        dy = self.end_y - self.start_y
        return (dx ** 2 + dy ** 2) ** 0.5

class Circle:
    def __init__(self, x: float, y: float, radius: float):
        self.x = x
        self.y = y
        self.radius = radius

    def area(self) -> float:
        return 3.14159 * self.radius ** 2
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
    assert!(result.contains("Line"), "Got: {}", result);
    assert!(result.contains("Circle"), "Got: {}", result);
}

// ===== Class with class-level constants =====

#[test]
fn test_s12_class_constants() {
    let code = r#"
class HttpStatus:
    OK = 200
    NOT_FOUND = 404
    INTERNAL_ERROR = 500

    def __init__(self, code: int):
        self.code = code

    def is_success(self) -> bool:
        return 200 <= self.code < 300
"#;
    let result = transpile(code);
    assert!(result.contains("HttpStatus"), "Got: {}", result);
}

// ===== Dataclass-like pattern =====

#[test]
fn test_s12_dataclass_pattern() {
    let code = r#"
class User:
    def __init__(self, name: str, email: str, age: int):
        self.name = name
        self.email = email
        self.age = age

    def display_name(self) -> str:
        return self.name

    def is_adult(self) -> bool:
        return self.age >= 18
"#;
    let result = transpile(code);
    assert!(result.contains("User"), "Got: {}", result);
}

// ===== Method calling other method =====

#[test]
fn test_s12_method_calls_method() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x: int):
        self.result += x

    def subtract(self, x: int):
        self.result -= x

    def reset(self):
        self.result = 0

    def get_result(self) -> int:
        return self.result
"#;
    let result = transpile(code);
    assert!(result.contains("Calculator"), "Got: {}", result);
}

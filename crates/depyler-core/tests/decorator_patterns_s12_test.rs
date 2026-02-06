//! Session 12 Batch 85: Decorator and special method patterns
//!
//! Targets cold paths for decorators (@property, @staticmethod,
//! @classmethod), special methods (__init__, __str__, __repr__,
//! __eq__, __lt__, __hash__), and class-level patterns.

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

// ===== Property decorator =====

#[test]
fn test_s12_b85_property_getter() {
    let code = r#"
class Rectangle:
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height

    @property
    def area(self) -> float:
        return self.width * self.height

    @property
    def perimeter(self) -> float:
        return 2 * (self.width + self.height)
"#;
    let result = transpile(code);
    assert!(result.contains("Rectangle"), "Got: {}", result);
}

// ===== Static method =====

#[test]
fn test_s12_b85_staticmethod() {
    let code = r#"
class Converter:
    @staticmethod
    def celsius_to_fahrenheit(c: float) -> float:
        return c * 9.0 / 5.0 + 32.0

    @staticmethod
    def fahrenheit_to_celsius(f: float) -> float:
        return (f - 32.0) * 5.0 / 9.0
"#;
    let result = transpile(code);
    assert!(result.contains("Converter"), "Got: {}", result);
}

// ===== Classmethod =====

#[test]
fn test_s12_b85_classmethod() {
    let code = r#"
class User:
    def __init__(self, name: str, email: str):
        self.name = name
        self.email = email

    @classmethod
    def from_string(cls, data: str):
        parts = data.split(",")
        return User(parts[0], parts[1])
"#;
    let result = transpile(code);
    assert!(result.contains("User"), "Got: {}", result);
}

// ===== Special methods =====

#[test]
fn test_s12_b85_str_method() {
    let code = r##"
class Fraction:
    def __init__(self, num: int, den: int):
        self.num = num
        self.den = den

    def __str__(self) -> str:
        return f"{self.num}/{self.den}"
"##;
    let result = transpile(code);
    assert!(result.contains("Fraction"), "Got: {}", result);
}

#[test]
fn test_s12_b85_eq_method() {
    let code = r#"
class Pair:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Pair"), "Got: {}", result);
}

#[test]
fn test_s12_b85_lt_method() {
    let code = r#"
class Priority:
    def __init__(self, value: int, priority: int):
        self.value = value
        self.priority = priority

    def __lt__(self, other) -> bool:
        return self.priority < other.priority
"#;
    let result = transpile(code);
    assert!(result.contains("Priority"), "Got: {}", result);
}

#[test]
fn test_s12_b85_len_method() {
    let code = r#"
class Bag:
    def __init__(self):
        self.items = []

    def __len__(self) -> int:
        return len(self.items)

    def add(self, item: int):
        self.items.append(item)
"#;
    let result = transpile(code);
    assert!(result.contains("Bag"), "Got: {}", result);
}

#[test]
fn test_s12_b85_contains_method() {
    let code = r#"
class SearchableList:
    def __init__(self):
        self.data = []

    def __contains__(self, item: int) -> bool:
        return item in self.data

    def add(self, item: int):
        if item not in self.data:
            self.data.append(item)
"#;
    let result = transpile(code);
    assert!(result.contains("SearchableList"), "Got: {}", result);
}

#[test]
fn test_s12_b85_add_method() {
    let code = r#"
class Money:
    def __init__(self, amount: float, currency: str):
        self.amount = amount
        self.currency = currency

    def __add__(self, other):
        return Money(self.amount + other.amount, self.currency)

    def __sub__(self, other):
        return Money(self.amount - other.amount, self.currency)
"#;
    let result = transpile(code);
    assert!(result.contains("Money"), "Got: {}", result);
}

#[test]
fn test_s12_b85_mul_method() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __mul__(self, scalar: float):
        return Vector(self.x * scalar, self.y * scalar)

    def dot(self, other) -> float:
        return self.x * other.x + self.y * other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Vector"), "Got: {}", result);
}

// ===== Complex class hierarchies =====

#[test]
fn test_s12_b85_abstract_like() {
    let code = r#"
class Shape:
    def __init__(self):
        pass

    def area(self) -> float:
        return 0.0

    def perimeter(self) -> float:
        return 0.0

class Square(Shape):
    def __init__(self, side: float):
        self.side = side

    def area(self) -> float:
        return self.side * self.side

    def perimeter(self) -> float:
        return 4.0 * self.side
"#;
    let result = transpile(code);
    assert!(result.contains("Shape"), "Got: {}", result);
    assert!(result.contains("Square"), "Got: {}", result);
}

#[test]
fn test_s12_b85_class_with_many_methods() {
    let code = r##"
class StringHelper:
    def __init__(self, text: str):
        self.text = text

    def is_palindrome(self) -> bool:
        cleaned = self.text.lower().replace(" ", "")
        return cleaned == cleaned[::-1]

    def word_count(self) -> int:
        return len(self.text.split())

    def char_count(self) -> int:
        return len(self.text)

    def first_word(self) -> str:
        words = self.text.split()
        if words:
            return words[0]
        return ""

    def last_word(self) -> str:
        words = self.text.split()
        if words:
            return words[-1]
        return ""
"##;
    let result = transpile(code);
    assert!(result.contains("StringHelper"), "Got: {}", result);
}

#[test]
fn test_s12_b85_class_with_class_vars() {
    let code = r#"
class Counter:
    total = 0

    def __init__(self, name: str):
        self.name = name
        self.count = 0

    def increment(self):
        self.count += 1

    def get_count(self) -> int:
        return self.count
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

//! TDD Tests for Multiple Classes (DEPYLER-0111 Phase 4)
//!
//! Phase 4: Multiple class definitions in the same module
//! Tests class composition, interaction, and proper code organization
//!
//! Test Coverage:
//! 1. Two simple independent classes
//! 2. Class using another class as field type
//! 3. Composition (class contains instance of another)
//! 4. Method returning instance of another class
//! 5. Method accepting another class as parameter
//! 6. Three classes interacting
//! 7. Classes with shared constants/attributes
//! 8. Factory pattern (class creating instances of another)
//! 9. Integration with functions using multiple classes
//! 10. Nested class instantiation

use depyler_core::DepylerPipeline;

#[test]
fn test_two_simple_independent_classes() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

class Color:
    def __init__(self, r: int, g: int, b: int):
        self.r = r
        self.g = g
        self.b = b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have both structs
    assert!(
        rust_code.contains("struct Point"),
        "Should have Point struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Color"),
        "Should have Color struct.\nGot:\n{}",
        rust_code
    );

    // Should have both impl blocks
    assert!(
        rust_code.contains("impl Point"),
        "Should have Point impl.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("impl Color"),
        "Should have Color impl.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_using_another_as_field_type() {
    let python = r#"
class Address:
    def __init__(self, street: str, city: str):
        self.street = street
        self.city = city

class Person:
    def __init__(self, name: str, address: Address):
        self.name = name
        self.address = address
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Address"),
        "Should have Address struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Person"),
        "Should have Person struct.\nGot:\n{}",
        rust_code
    );

    // Person should reference Address type
    let has_address_field = rust_code.contains("address") || rust_code.contains("Address");
    assert!(
        has_address_field,
        "Person should have address field.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_composition_class_contains_another() {
    let python = r#"
class Engine:
    def __init__(self, horsepower: int):
        self.horsepower = horsepower

    def start(self) -> str:
        return "Engine started"

class Car:
    def __init__(self, model: str):
        self.model = model
        self.engine = Engine(200)

    def start_car(self) -> str:
        return self.engine.start()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Engine"),
        "Should have Engine struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Car"),
        "Should have Car struct.\nGot:\n{}",
        rust_code
    );

    // Should have start method in Engine
    assert!(
        rust_code.contains("fn start"),
        "Engine should have start method.\nGot:\n{}",
        rust_code
    );

    // Should have start_car method in Car
    assert!(
        rust_code.contains("fn start_car"),
        "Car should have start_car method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_method_returning_another_class() {
    let python = r#"
class Position:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

class Robot:
    def __init__(self, name: str):
        self.name = name

    def get_position(self) -> Position:
        return Position(0, 0)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Position"),
        "Should have Position struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Robot"),
        "Should have Robot struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn get_position"),
        "Robot should have get_position method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_method_accepting_another_class() {
    let python = r#"
class Message:
    def __init__(self, text: str):
        self.text = text

class Logger:
    def __init__(self, prefix: str):
        self.prefix = prefix

    def log(self, message: Message) -> str:
        return self.prefix + message.text
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Message"),
        "Should have Message struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Logger"),
        "Should have Logger struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn log"),
        "Logger should have log method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_three_classes_interacting() {
    let python = r#"
class Author:
    def __init__(self, name: str):
        self.name = name

class Book:
    def __init__(self, title: str, author: Author):
        self.title = title
        self.author = author

class Library:
    def __init__(self, name: str):
        self.name = name

    def add_book(self, book: Book) -> str:
        return book.title
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have all three structs
    assert!(
        rust_code.contains("struct Author"),
        "Should have Author struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Book"),
        "Should have Book struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Library"),
        "Should have Library struct.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classes_with_shared_constants() {
    let python = r#"
class Config:
    MAX_SIZE: int = 1000

class Buffer:
    SIZE: int = 1000

    def __init__(self):
        self.data = []
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Config"),
        "Should have Config struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Buffer"),
        "Should have Buffer struct.\nGot:\n{}",
        rust_code
    );

    // Both should have their constants
    let has_max_size = rust_code.contains("MAX_SIZE") || rust_code.contains("max_size");
    let has_size = rust_code.contains("SIZE") || rust_code.contains("size");
    assert!(
        has_max_size && has_size,
        "Should have both constants.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_factory_pattern() {
    let python = r#"
class Widget:
    def __init__(self, id: int):
        self.id = id

class WidgetFactory:
    def __init__(self):
        self.counter = 0

    def create_widget(self) -> Widget:
        self.counter = self.counter + 1
        return Widget(self.counter)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Widget"),
        "Should have Widget struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct WidgetFactory"),
        "Should have WidgetFactory struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn create_widget"),
        "WidgetFactory should have create_widget method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_integration_functions_with_multiple_classes() {
    let python = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

def add_vector_to_point(point: Point, vector: Vector) -> Point:
    return Point(point.x + vector.x, point.y + vector.y)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Vector"),
        "Should have Vector struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Point"),
        "Should have Point struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn add_vector_to_point"),
        "Should have add_vector_to_point function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_nested_class_instantiation() {
    let python = r#"
class Inner:
    def __init__(self, value: int):
        self.value = value

class Outer:
    def __init__(self, x: int):
        self.x = x
        self.inner = Inner(x * 2)

    def get_inner_value(self) -> int:
        return self.inner.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Inner"),
        "Should have Inner struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("struct Outer"),
        "Should have Outer struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn get_inner_value"),
        "Outer should have get_inner_value method.\nGot:\n{}",
        rust_code
    );
}

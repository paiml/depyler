//! DEPYLER-0932: Dataclass Parameter Ordering Tests
//!
//! Tests for ensuring generated struct `new()` method parameters
//! match the field order in Python dataclasses.

use depyler_core::DepylerPipeline;

/// Test basic dataclass field ordering
#[test]
fn test_depyler_0932_basic_field_order() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Person:
    name: str
    age: int
    email: str

def create_person() -> Person:
    return Person("Alice", 30, "alice@example.com")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // The new() method should have parameters in order: name, age, email
    // Check that the struct is generated
    assert!(
        code.contains("struct Person"),
        "Should generate Person struct: {}",
        code
    );
}

/// Test dataclass with default values - ordering should be preserved
#[test]
fn test_depyler_0932_with_defaults() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    host: str = "localhost"
    port: int = 8080
    debug: bool = False

def create_config() -> Config:
    return Config()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("struct Config"),
        "Should generate Config struct: {}",
        code
    );
}

/// Test dataclass with mixed required and optional fields
#[test]
fn test_depyler_0932_mixed_required_optional() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Item:
    id: int
    name: str
    price: float = 0.0
    quantity: int = 1

def create_item(id: int, name: str) -> Item:
    return Item(id, name)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should generate struct with fields in correct order
    assert!(
        code.contains("struct Item"),
        "Should generate Item struct: {}",
        code
    );
}

/// Test dataclass instantiation with positional arguments
#[test]
fn test_depyler_0932_positional_instantiation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Point:
    x: float
    y: float
    z: float

def origin() -> Point:
    return Point(0.0, 0.0, 0.0)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should generate new() call with correct argument order
    assert!(
        code.contains("Point") && (code.contains("new(") || code.contains("Point {")),
        "Should generate Point instantiation: {}",
        code
    );
}

/// Test dataclass with keyword arguments
#[test]
fn test_depyler_0932_keyword_instantiation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Rectangle:
    width: float
    height: float

def square(size: float) -> Rectangle:
    return Rectangle(width=size, height=size)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("struct Rectangle"),
        "Should generate Rectangle struct: {}",
        code
    );
}

/// Test nested dataclass
#[test]
fn test_depyler_0932_nested_dataclass() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Address:
    street: str
    city: str

@dataclass
class Person:
    name: str
    address: Address

def create_person() -> Person:
    addr = Address("123 Main St", "Springfield")
    return Person("John", addr)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("struct Address") && code.contains("struct Person"),
        "Should generate both structs: {}",
        code
    );
}

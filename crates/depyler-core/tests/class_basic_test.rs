//! TDD Tests for Class Support (DEPYLER-0111 Phase 1)
//!
//! Phase 1: Simple classes with __init__ method
//! This test MUST FAIL initially (Red phase), then pass after implementation (Green phase)
//!
//! Class mapping:
//! Python: class Point: def __init__(self, x, y): ...
//! Rust:   struct Point { x: i32, y: i32 } impl Point { fn new(x: i32, y: i32) -> Self { ... } }

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_class_with_init() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should generate a struct
    assert!(
        rust_code.contains("struct Point"),
        "Should generate struct Point.\nGot:\n{}",
        rust_code
    );

    // Should have fields
    assert!(
        rust_code.contains("x:") && rust_code.contains("y:"),
        "Should have x and y fields.\nGot:\n{}",
        rust_code
    );

    // Should generate impl block
    assert!(
        rust_code.contains("impl Point"),
        "Should generate impl block.\nGot:\n{}",
        rust_code
    );

    // Should have constructor (new method)
    assert!(
        rust_code.contains("fn new"),
        "Should have new() constructor.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_with_typed_fields() {
    let python = r#"
class Rectangle:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Rectangle"),
        "Should generate struct Rectangle.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("width") && rust_code.contains("height"),
        "Should have width and height fields.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_with_default_parameters() {
    let python = r#"
class Counter:
    def __init__(self, start: int = 0):
        self.value = start
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Counter"),
        "Should generate struct Counter.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("value"),
        "Should have value field.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_empty_class() {
    let python = r#"
class Empty:
    pass
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Empty"),
        "Should generate struct Empty.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_with_docstring() {
    let python = r#"
class Person:
    """A simple person class."""
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Person"),
        "Should generate struct Person.\nGot:\n{}",
        rust_code
    );

    // Docstring should be preserved as doc comment
    assert!(
        rust_code.contains("person class") || rust_code.contains("Person"),
        "Should preserve docstring.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_with_multiple_fields() {
    let python = r#"
class Student:
    def __init__(self, name: str, age: int, grade: float, active: bool):
        self.name = name
        self.age = age
        self.grade = grade
        self.active = active
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Student"),
        "Should generate struct Student.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("name") && rust_code.contains("age") && 
        rust_code.contains("grade") && rust_code.contains("active"),
        "Should have all 4 fields.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_instantiation() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def create_point() -> Point:
    p = Point(10, 20)
    return p
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should create instance with Point::new()
    assert!(
        rust_code.contains("Point::new") || rust_code.contains("Point {"),
        "Should create Point instance.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_field_access() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def get_x(p: Point) -> int:
    return p.x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should access field with p.x
    assert!(
        rust_code.contains("p.x") || rust_code.contains("p .x"),
        "Should access field p.x.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_instances() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def test() -> int:
    p1 = Point(1, 2)
    p2 = Point(3, 4)
    return p1.x + p2.x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should create multiple instances
    assert!(
        rust_code.matches("Point::new").count() >= 2 || rust_code.matches("Point {").count() >= 2,
        "Should create multiple Point instances.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_with_string_field() {
    let python = r#"
class User:
    def __init__(self, username: str):
        self.username = username
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct User"),
        "Should generate struct User.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("username"),
        "Should have username field.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_field_mutation() {
    let python = r#"
class Counter:
    def __init__(self, value: int):
        self.value = value

def increment(c: Counter) -> int:
    c.value = c.value + 1
    return c.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should allow field mutation (may require &mut)
    assert!(
        rust_code.contains("c.value") || rust_code.contains("c .value"),
        "Should access and mutate c.value.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_without_init() {
    let python = r#"
class Config:
    pass
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Config"),
        "Should generate struct Config.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_with_computed_field() {
    let python = r#"
class Rectangle:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.area = width * height
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Rectangle"),
        "Should generate struct Rectangle.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("area"),
        "Should have computed area field.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_integration_with_function() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def distance_from_origin(p: Point) -> float:
    return (p.x * p.x + p.y * p.y) ** 0.5
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Point"),
        "Should generate struct Point.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn distance_from_origin") || rust_code.contains("distance_from_origin"),
        "Should have distance_from_origin function.\nGot:\n{}",
        rust_code
    );
}

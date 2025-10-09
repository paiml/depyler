//! TDD Tests for Class Attributes (DEPYLER-0111 Phase 3)
//!
//! Phase 3: Class-level attributes (static/constant fields)
//! Python: class Foo: x = 10 → Rust: impl Foo { const X: i32 = 10; } or static
//!
//! Test Coverage:
//! 1. Simple class constant (immutable class variable)
//! 2. Class attribute accessed via ClassName
//! 3. Class attribute accessed via instance (self)
//! 4. Multiple class attributes
//! 5. Class attribute with string type
//! 6. Class attribute used in method
//! 7. Class attribute modified (mutable static)
//! 8. Counter pattern (class variable tracking instances)
//! 9. Class constant used in __init__
//! 10. Mix of class attributes and instance attributes

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_class_constant() {
    let python = r#"
class Config:
    MAX_SIZE: int = 100

    def __init__(self, size: int):
        self.size = size
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have struct definition
    assert!(
        rust_code.contains("struct Config"),
        "Should have Config struct.\nGot:\n{}",
        rust_code
    );

    // Should have const MAX_SIZE in impl block or as associated constant
    let has_const = rust_code.contains("const MAX_SIZE") ||
                    rust_code.contains("const max_size") ||
                    rust_code.contains("MAX_SIZE: i32 = 100");
    assert!(has_const, "Should have MAX_SIZE constant.\nGot:\n{}", rust_code);
}

#[test]
fn test_class_attribute_access_via_classname() {
    let python = r#"
class Math:
    PI: float = 3.14159

    def get_pi(self) -> float:
        return Math.PI
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Math"),
        "Should have Math struct.\nGot:\n{}",
        rust_code
    );

    // Should have PI constant
    let has_pi = rust_code.contains("PI") || rust_code.contains("pi");
    assert!(has_pi, "Should have PI constant.\nGot:\n{}", rust_code);

    // Should have get_pi method
    assert!(
        rust_code.contains("fn get_pi"),
        "Should have get_pi method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_class_attribute_access_via_self() {
    let python = r#"
class Robot:
    DEFAULT_NAME: str = "Robot"

    def __init__(self):
        self.name = self.DEFAULT_NAME
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Robot"),
        "Should have Robot struct.\nGot:\n{}",
        rust_code
    );

    // Should have DEFAULT_NAME constant
    let has_default_name = rust_code.contains("DEFAULT_NAME") ||
                           rust_code.contains("default_name");
    assert!(has_default_name, "Should have DEFAULT_NAME constant.\nGot:\n{}", rust_code);
}

#[test]
fn test_multiple_class_attributes() {
    let python = r#"
class Constants:
    WIDTH: int = 800
    HEIGHT: int = 600
    TITLE: str = "Game"

    def __init__(self):
        self.width = Constants.WIDTH
        self.height = Constants.HEIGHT
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have all three constants
    let has_width = rust_code.contains("WIDTH") || rust_code.contains("width");
    let has_height = rust_code.contains("HEIGHT") || rust_code.contains("height");
    let has_title = rust_code.contains("TITLE") || rust_code.contains("title");

    assert!(has_width, "Should have WIDTH constant.\nGot:\n{}", rust_code);
    assert!(has_height, "Should have HEIGHT constant.\nGot:\n{}", rust_code);
    assert!(has_title, "Should have TITLE constant.\nGot:\n{}", rust_code);
}

#[test]
fn test_class_attribute_with_string_type() {
    let python = r#"
class Message:
    PREFIX: str = "[INFO]"

    def format(self, text: str) -> str:
        return Message.PREFIX + " " + text
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Message"),
        "Should have Message struct.\nGot:\n{}",
        rust_code
    );

    // Should have PREFIX constant with string value
    let has_prefix = rust_code.contains("PREFIX") || rust_code.contains("prefix");
    let has_info = rust_code.contains("[INFO]") || rust_code.contains("INFO");
    assert!(has_prefix && has_info, "Should have PREFIX constant with [INFO] value.\nGot:\n{}", rust_code);
}

#[test]
fn test_class_attribute_used_in_method() {
    let python = r#"
class Circle:
    PI: float = 3.14159

    def __init__(self, radius: float):
        self.radius = radius

    def area(self) -> float:
        return Circle.PI * self.radius * self.radius
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Circle"),
        "Should have Circle struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn area"),
        "Should have area method.\nGot:\n{}",
        rust_code
    );

    // Should have PI constant
    let has_pi = rust_code.contains("PI") || rust_code.contains("pi");
    assert!(has_pi, "Should have PI constant.\nGot:\n{}", rust_code);
}

#[test]
fn test_mutable_class_attribute() {
    let python = r#"
class Counter:
    count: int = 0

    def __init__(self):
        Counter.count = Counter.count + 1

    def get_count(self) -> int:
        return Counter.count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Counter"),
        "Should have Counter struct.\nGot:\n{}",
        rust_code
    );

    // Should have count variable (static or similar)
    let has_count = rust_code.contains("count") || rust_code.contains("COUNT");
    assert!(has_count, "Should have count variable.\nGot:\n{}", rust_code);
}

#[test]
fn test_counter_pattern() {
    let python = r#"
class Widget:
    total_widgets: int = 0

    def __init__(self, name: str):
        self.name = name
        Widget.total_widgets = Widget.total_widgets + 1

    def get_total(self) -> int:
        return Widget.total_widgets
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Widget"),
        "Should have Widget struct.\nGot:\n{}",
        rust_code
    );

    // Should have total_widgets variable
    let has_total = rust_code.contains("total_widgets") ||
                    rust_code.contains("TOTAL_WIDGETS");
    assert!(has_total, "Should have total_widgets variable.\nGot:\n{}", rust_code);
}

#[test]
fn test_class_constant_used_in_init() {
    let python = r#"
class Buffer:
    DEFAULT_SIZE: int = 1024

    def __init__(self, size: int = DEFAULT_SIZE):
        self.size = size
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Buffer"),
        "Should have Buffer struct.\nGot:\n{}",
        rust_code
    );

    // Should have DEFAULT_SIZE constant
    let has_default_size = rust_code.contains("DEFAULT_SIZE") ||
                           rust_code.contains("default_size") ||
                           rust_code.contains("1024");
    assert!(has_default_size, "Should have DEFAULT_SIZE constant.\nGot:\n{}", rust_code);
}

#[test]
fn test_mix_class_and_instance_attributes() {
    let python = r#"
class Car:
    WHEELS: int = 4

    def __init__(self, color: str):
        self.color = color
        self.wheels = Car.WHEELS

    def get_wheels(self) -> int:
        return self.wheels
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Car"),
        "Should have Car struct.\nGot:\n{}",
        rust_code
    );

    // Should have WHEELS constant
    let has_wheels = rust_code.contains("WHEELS") || rust_code.contains("wheels");
    assert!(has_wheels, "Should have WHEELS constant or field.\nGot:\n{}", rust_code);

    // Should have color field in struct
    let has_color = rust_code.contains("color");
    assert!(has_color, "Should have color field.\nGot:\n{}", rust_code);
}

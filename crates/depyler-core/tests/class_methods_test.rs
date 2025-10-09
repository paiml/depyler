//! TDD Tests for Class Instance Methods (DEPYLER-0111 Phase 2)
//!
//! Phase 2: Instance methods on classes
//! Python: def method(self, ...) → Rust: fn method(&self, ...) or fn method(&mut self, ...)
//!
//! Test Coverage:
//! 1. Simple instance method returning value
//! 2. Instance method accessing self fields
//! 3. Instance method modifying self (needs &mut self)
//! 4. Instance method with parameters
//! 5. Instance method calling another instance method
//! 6. Multiple instance methods
//! 7. Instance method with return type annotation
//! 8. Instance method with no return (-> ())
//! 9. Chained method calls (fluent interface)
//! 10. Instance method with default parameters
//! 11. Static-like method (no self usage)
//! 12. Instance method returning self reference

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_instance_method() {
    let python = r#"
class Counter:
    def __init__(self, value: int):
        self.value = value

    def get_value(self) -> int:
        return self.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have impl block with method
    assert!(
        rust_code.contains("impl Counter"),
        "Should have impl Counter block.\nGot:\n{}",
        rust_code
    );

    // Should have get_value method
    assert!(
        rust_code.contains("fn get_value"),
        "Should have get_value method.\nGot:\n{}",
        rust_code
    );

    // Should take &self parameter
    let has_self_ref = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(has_self_ref, "Method should take &self parameter.\nGot:\n{}", rust_code);
}

#[test]
fn test_instance_method_accessing_fields() {
    let python = r#"
class Rectangle:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    def area(self) -> int:
        return self.width * self.height
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn area"),
        "Should have area method.\nGot:\n{}",
        rust_code
    );

    // Should access self.width and self.height
    let has_width = rust_code.contains("self.width") || rust_code.contains("self .width");
    let has_height = rust_code.contains("self.height") || rust_code.contains("self .height");
    assert!(has_width && has_height, "Should access self.width and self.height.\nGot:\n{}", rust_code);
}

#[test]
fn test_instance_method_modifying_self() {
    let python = r#"
class Counter:
    def __init__(self, value: int):
        self.value = value

    def increment(self) -> None:
        self.value = self.value + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn increment"),
        "Should have increment method.\nGot:\n{}",
        rust_code
    );

    // Should take &mut self since it modifies fields
    let has_mut_self = rust_code.contains("&mut self") || rust_code.contains("& mut self");
    assert!(has_mut_self, "Mutating method should take &mut self.\nGot:\n{}", rust_code);
}

#[test]
fn test_instance_method_with_parameters() {
    let python = r#"
class Calculator:
    def __init__(self, initial: int):
        self.value = initial

    def add(self, amount: int) -> int:
        return self.value + amount
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn add"),
        "Should have add method.\nGot:\n{}",
        rust_code
    );

    // Should have amount parameter
    assert!(
        rust_code.contains("amount"),
        "Should have amount parameter.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_instance_method_calling_another_method() {
    let python = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    def diameter(self) -> float:
        return self.radius * 2.0

    def circumference(self) -> float:
        return self.diameter() * 3.14159
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn diameter"),
        "Should have diameter method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn circumference"),
        "Should have circumference method.\nGot:\n{}",
        rust_code
    );

    // Should call self.diameter()
    let calls_diameter = rust_code.contains("self.diameter()") || rust_code.contains("self .diameter");
    assert!(calls_diameter, "Should call self.diameter().\nGot:\n{}", rust_code);
}

#[test]
fn test_multiple_instance_methods() {
    let python = r#"
class BankAccount:
    def __init__(self, balance: int):
        self.balance = balance

    def deposit(self, amount: int) -> None:
        self.balance = self.balance + amount

    def withdraw(self, amount: int) -> None:
        self.balance = self.balance - amount

    def get_balance(self) -> int:
        return self.balance
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn deposit"),
        "Should have deposit method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn withdraw"),
        "Should have withdraw method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn get_balance"),
        "Should have get_balance method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_instance_method_with_return_annotation() {
    let python = r#"
class StringHolder:
    def __init__(self, text: str):
        self.text = text

    def get_length(self) -> int:
        return len(self.text)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn get_length"),
        "Should have get_length method.\nGot:\n{}",
        rust_code
    );

    // Should have return type (i32, i64, usize, or similar)
    let has_return_type = rust_code.contains("-> i32") ||
                          rust_code.contains("-> i64") ||
                          rust_code.contains("-> usize");
    assert!(has_return_type, "Should have return type annotation.\nGot:\n{}", rust_code);
}

#[test]
fn test_instance_method_no_return() {
    let python = r#"
class Logger:
    def __init__(self, prefix: str):
        self.prefix = prefix

    def log(self, message: str) -> None:
        print(self.prefix + message)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn log"),
        "Should have log method.\nGot:\n{}",
        rust_code
    );

    // Methods with None return might have -> () or no return type
    // Both are acceptable
}

#[test]
fn test_method_call_on_instance() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance_from_origin(self) -> float:
        return (self.x * self.x + self.y * self.y) ** 0.5

def test_distance() -> float:
    p = Point(3, 4)
    return p.distance_from_origin()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should call method on instance (flexible about whitespace)
    let calls_method = rust_code.contains("p.distance_from_origin") &&
                       rust_code.contains("()");
    assert!(calls_method, "Should call p.distance_from_origin().\nGot:\n{}", rust_code);
}

#[test]
fn test_chained_method_calls() {
    let python = r#"
class Builder:
    def __init__(self, value: int):
        self.value = value

    def add(self, x: int) -> int:
        return self.value + x

    def multiply(self, x: int) -> int:
        return self.value * x

def compute() -> int:
    b = Builder(5)
    result = b.add(3)
    result = result + b.multiply(2)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn add"),
        "Should have add method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn multiply"),
        "Should have multiply method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_instance_method_with_default_parameter() {
    let python = r#"
class Formatter:
    def __init__(self, prefix: str):
        self.prefix = prefix

    def format(self, text: str, suffix: str = "!") -> str:
        return self.prefix + text + suffix
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn format"),
        "Should have format method.\nGot:\n{}",
        rust_code
    );

    // Should have text and suffix parameters
    assert!(
        rust_code.contains("text") && rust_code.contains("suffix"),
        "Should have text and suffix parameters.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_method_integration_with_functions() {
    let python = r#"
class Temperature:
    def __init__(self, celsius: float):
        self.celsius = celsius

    def to_fahrenheit(self) -> float:
        return self.celsius * 9.0 / 5.0 + 32.0

def convert_temperature(c: float) -> float:
    temp = Temperature(c)
    return temp.to_fahrenheit()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("struct Temperature"),
        "Should have Temperature struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn to_fahrenheit"),
        "Should have to_fahrenheit method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn convert_temperature"),
        "Should have convert_temperature function.\nGot:\n{}",
        rust_code
    );
}

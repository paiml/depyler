//! TDD Tests for @property Decorator (DEPYLER-0112 Phase 3)
//!
//! Phase 3: @property decorator support
//! Python: @property def method(self) → Rust: pub fn method(&self) (getter method)
//!
//! Test Coverage:
//! 1. Simple property getter (read-only)
//! 2. Property with computation
//! 3. Property accessing instance fields
//! 4. Property accessing class constants
//! 5. Multiple properties in same class
//! 6. Property calling another method
//! 7. Property with type annotation
//! 8. Mix of properties and regular methods
//! 9. Property returning different types
//! 10. Property with conditional logic

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_property_getter() {
    let python = r#"
class Person:
    def __init__(self, name: str):
        self.name = name

    @property
    def display_name(self):
        return self.name
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
        rust_code.contains("struct Person"),
        "Should have Person struct.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn display_name"),
        "Should have display_name function.\nGot:\n{}",
        rust_code
    );

    // Should have &self parameter (it's a getter)
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(
        has_self,
        "Property should have &self parameter.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_property_with_computation() {
    let python = r#"
class Rectangle:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    @property
    def area(self) -> int:
        return self.width * self.height
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
        rust_code.contains("fn area"),
        "Should have area function.\nGot:\n{}",
        rust_code
    );

    // Should have computation (width * height)
    let has_computation = rust_code.contains("width") && rust_code.contains("height");
    assert!(
        has_computation,
        "Should have computation.\nGot:\n{}",
        rust_code
    );

    // Should have &self
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(has_self, "Property should have &self.\nGot:\n{}", rust_code);
}

#[test]
fn test_property_accessing_instance_fields() {
    let python = r#"
class Person:
    def __init__(self, first: str, last: str):
        self.first = first
        self.last = last

    @property
    def full_name(self) -> str:
        return self.first + " " + self.last
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
        rust_code.contains("fn full_name"),
        "Should have full_name function.\nGot:\n{}",
        rust_code
    );

    // Should access self.first and self.last
    let accesses_fields = rust_code.contains("first") && rust_code.contains("last");
    assert!(
        accesses_fields,
        "Should access instance fields.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_property_accessing_class_constants() {
    let python = r#"
class Circle:
    PI: float = 3.14159

    def __init__(self, radius: float):
        self.radius = radius

    @property
    def circumference(self) -> float:
        return 2 * Circle.PI * self.radius
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
        rust_code.contains("fn circumference"),
        "Should have circumference function.\nGot:\n{}",
        rust_code
    );

    // Should reference PI constant
    let has_pi = rust_code.contains("PI") || rust_code.contains("pi");
    assert!(has_pi, "Should reference PI constant.\nGot:\n{}", rust_code);

    // Should reference radius
    assert!(
        rust_code.contains("radius"),
        "Should reference radius.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_properties() {
    let python = r#"
class Person:
    def __init__(self, first: str, last: str, age: int):
        self.first = first
        self.last = last
        self.age = age

    @property
    def full_name(self) -> str:
        return self.first + " " + self.last

    @property
    def is_adult(self) -> bool:
        return self.age >= 18

    @property
    def display_info(self) -> str:
        return self.full_name + " (" + str(self.age) + ")"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have all three properties
    assert!(
        rust_code.contains("fn full_name"),
        "Should have full_name property.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn is_adult"),
        "Should have is_adult property.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn display_info"),
        "Should have display_info property.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_property_calling_another_method() {
    let python = r#"
class Calculator:
    def __init__(self, value: int):
        self.value = value

    def double(self) -> int:
        return self.value * 2

    @property
    def doubled_value(self) -> int:
        return self.double()
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
        rust_code.contains("fn double"),
        "Should have double method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn doubled_value"),
        "Should have doubled_value property.\nGot:\n{}",
        rust_code
    );

    // Should call double method
    let calls_double = rust_code.contains("double");
    assert!(
        calls_double,
        "Should call double method.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_property_with_type_annotation() {
    let python = r#"
class Counter:
    def __init__(self, count: int):
        self.count = count

    @property
    def is_even(self) -> bool:
        return self.count % 2 == 0
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
        rust_code.contains("fn is_even"),
        "Should have is_even property.\nGot:\n{}",
        rust_code
    );

    // Should have bool return type
    let has_bool_return = rust_code.contains("-> bool") || rust_code.contains("bool");
    assert!(
        has_bool_return,
        "Should have bool return type.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_mix_properties_and_regular_methods() {
    let python = r#"
class Account:
    def __init__(self, balance: float):
        self.balance = balance

    @property
    def formatted_balance(self) -> str:
        return "$" + str(self.balance)

    def deposit(self, amount: float) -> None:
        self.balance = self.balance + amount

    def withdraw(self, amount: float) -> None:
        self.balance = self.balance - amount

    @property
    def is_positive(self) -> bool:
        return self.balance > 0.0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have properties
    assert!(
        rust_code.contains("fn formatted_balance"),
        "Should have formatted_balance property.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn is_positive"),
        "Should have is_positive property.\nGot:\n{}",
        rust_code
    );

    // Should have regular methods
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
}

#[test]
fn test_property_returning_different_types() {
    let python = r#"
class DataHolder:
    def __init__(self, value: int):
        self.value = value

    @property
    def as_int(self) -> int:
        return self.value

    @property
    def as_str(self) -> str:
        return str(self.value)

    @property
    def as_float(self) -> float:
        return float(self.value)
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
        rust_code.contains("fn as_int"),
        "Should have as_int property.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn as_str"),
        "Should have as_str property.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn as_float"),
        "Should have as_float property.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_property_with_conditional_logic() {
    let python = r#"
class User:
    def __init__(self, age: int, premium: bool):
        self.age = age
        self.premium = premium

    @property
    def access_level(self) -> str:
        if self.premium:
            return "premium"
        elif self.age >= 18:
            return "adult"
        else:
            return "basic"
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
        rust_code.contains("fn access_level"),
        "Should have access_level property.\nGot:\n{}",
        rust_code
    );

    // Should have conditional logic
    let has_conditional = rust_code.contains("if") || rust_code.contains("match");
    assert!(
        has_conditional,
        "Should have conditional logic.\nGot:\n{}",
        rust_code
    );

    // Should have &self
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(has_self, "Property should have &self.\nGot:\n{}", rust_code);
}

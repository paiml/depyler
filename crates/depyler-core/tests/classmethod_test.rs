//! TDD Tests for @classmethod Decorator (DEPYLER-0112 Phase 2)
//!
//! Phase 2: @classmethod decorator support
//! Python: @classmethod def method(cls) → Rust: fn method() -> Self (or appropriate factory)
//!
//! Test Coverage:
//! 1. Simple classmethod factory (create instance)
//! 2. Classmethod with parameters
//! 3. Classmethod accessing class constants
//! 4. Multiple classmethods
//! 5. Classmethod calling another classmethod
//! 6. Instance method calling classmethod
//! 7. Classmethod with type annotations
//! 8. Classmethod as alternative constructor
//! 9. Mix of class, static, and instance methods
//! 10. Classmethod with default parameters

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_classmethod_factory() {
    let python = r#"
class Person:
    def __init__(self, name: str):
        self.name = name

    @classmethod
    def create_john(cls):
        return cls("John")
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
        rust_code.contains("fn create_john"),
        "Should have create_john function.\nGot:\n{}",
        rust_code
    );

    // Should NOT have &self or cls parameter
    let has_self_param = rust_code.contains("&self") || rust_code.contains("cls:");
    assert!(
        !has_self_param,
        "Classmethod should not have &self or cls parameter.\nGot:\n{}",
        rust_code
    );

    // Should call Self::new or constructor pattern
    let has_constructor = rust_code.contains("Self::new")
        || rust_code.contains("Person::new")
        || rust_code.contains("Self {");
    assert!(
        has_constructor,
        "Should use Self::new or constructor.\nGot:\n{}",
        rust_code
    );

    // Should NOT have undefined cls variable
    let has_cls_var = rust_code.contains("cls(");
    assert!(
        !has_cls_var,
        "Should not have undefined cls variable.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classmethod_with_parameters() {
    let python = r#"
class User:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    @classmethod
    def create(cls, name: str, age: int):
        return cls(name, age)
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
        rust_code.contains("fn create"),
        "Should have create function.\nGot:\n{}",
        rust_code
    );

    // Should have name and age parameters
    assert!(
        rust_code.contains("name") && rust_code.contains("age"),
        "Should have name and age parameters.\nGot:\n{}",
        rust_code
    );

    // Should NOT have cls parameter
    let has_cls_param = rust_code.contains("cls:");
    assert!(
        !has_cls_param,
        "Should not have cls parameter.\nGot:\n{}",
        rust_code
    );

    // Should call constructor with parameters
    let has_constructor_call = rust_code.contains("Self::new") || rust_code.contains("User::new");
    assert!(
        has_constructor_call,
        "Should call constructor.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classmethod_accessing_class_constants() {
    let python = r#"
class Config:
    DEFAULT_NAME: str = "Unknown"

    def __init__(self, name: str):
        self.name = name

    @classmethod
    def create_default(cls):
        return cls(cls.DEFAULT_NAME)
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
        rust_code.contains("fn create_default"),
        "Should have create_default function.\nGot:\n{}",
        rust_code
    );

    // Should reference DEFAULT_NAME constant
    let has_const_ref = rust_code.contains("DEFAULT_NAME") || rust_code.contains("default_name");
    assert!(
        has_const_ref,
        "Should reference DEFAULT_NAME.\nGot:\n{}",
        rust_code
    );

    // Should use Self::DEFAULT_NAME or Config::DEFAULT_NAME
    let has_qualified_const = rust_code.contains("Self::DEFAULT_NAME")
        || rust_code.contains("Config::DEFAULT_NAME")
        || rust_code.contains("Self::default_name")
        || rust_code.contains("Config::default_name");
    assert!(
        has_qualified_const,
        "Should use qualified constant access.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_classmethods() {
    let python = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    @classmethod
    def origin(cls):
        return cls(0, 0)

    @classmethod
    def unit_x(cls):
        return cls(1, 0)

    @classmethod
    def unit_y(cls):
        return cls(0, 1)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have all three classmethods
    assert!(
        rust_code.contains("fn origin"),
        "Should have origin function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn unit_x"),
        "Should have unit_x function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn unit_y"),
        "Should have unit_y function.\nGot:\n{}",
        rust_code
    );

    // None should have cls parameter
    let cls_count = rust_code.matches("cls:").count();
    assert_eq!(
        cls_count, 0,
        "No function should have cls parameter.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classmethod_calling_another_classmethod() {
    let python = r#"
class Rectangle:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height

    @classmethod
    def square(cls, size: int):
        return cls(size, size)

    @classmethod
    def unit_square(cls):
        return cls.square(1)
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
        rust_code.contains("fn square"),
        "Should have square function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn unit_square"),
        "Should have unit_square function.\nGot:\n{}",
        rust_code
    );

    // Should call Self::square or Rectangle::square
    let calls_square =
        rust_code.contains("Self::square") || rust_code.contains("Rectangle::square");
    assert!(
        calls_square,
        "Should call square classmethod.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_instance_method_calling_classmethod() {
    let python = r#"
class Counter:
    def __init__(self, value: int):
        self.value = value

    @classmethod
    def zero(cls):
        return cls(0)

    def reset(self):
        return Counter.zero()
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
        rust_code.contains("fn zero"),
        "Should have zero function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn reset"),
        "Should have reset method.\nGot:\n{}",
        rust_code
    );

    // reset should have &self
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(has_self, "reset should have &self.\nGot:\n{}", rust_code);

    // Should call Counter::zero or Self::zero
    let calls_zero = rust_code.contains("Counter::zero") || rust_code.contains("Self::zero");
    assert!(
        calls_zero,
        "Should call zero classmethod.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classmethod_with_type_annotations() {
    let python = r#"
class Temperature:
    def __init__(self, celsius: float):
        self.celsius = celsius

    @classmethod
    def from_fahrenheit(cls, fahrenheit: float) -> Temperature:
        celsius = (fahrenheit - 32.0) * 5.0 / 9.0
        return cls(celsius)
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
        rust_code.contains("fn from_fahrenheit"),
        "Should have from_fahrenheit function.\nGot:\n{}",
        rust_code
    );

    // Should have float parameter
    let has_float =
        rust_code.contains("f32") || rust_code.contains("f64") || rust_code.contains("fahrenheit");
    assert!(
        has_float,
        "Should have float parameter.\nGot:\n{}",
        rust_code
    );

    // Should return Self or Temperature
    let has_return_type = rust_code.contains("-> Self")
        || rust_code.contains("-> Temperature")
        || rust_code.contains("Self");
    assert!(
        has_return_type,
        "Should return Self or Temperature.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classmethod_as_alternative_constructor() {
    let python = r#"
class Date:
    def __init__(self, year: int, month: int, day: int):
        self.year = year
        self.month = month
        self.day = day

    @classmethod
    def from_string(cls, date_str: str):
        # Simplified parsing
        return cls(2024, 1, 1)

    @classmethod
    def today(cls):
        return cls(2024, 10, 9)
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
        rust_code.contains("fn from_string"),
        "Should have from_string function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn today"),
        "Should have today function.\nGot:\n{}",
        rust_code
    );

    // Both should call constructor
    let has_constructors = rust_code.contains("Self::new")
        || rust_code.contains("Date::new")
        || rust_code.contains("Self {");
    assert!(
        has_constructors,
        "Should have constructor calls.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_mix_class_static_instance_methods() {
    let python = r#"
class Calculator:
    DEFAULT_OFFSET: int = 10

    def __init__(self, offset: int):
        self.offset = offset

    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @classmethod
    def with_default_offset(cls):
        return cls(cls.DEFAULT_OFFSET)

    def add_with_offset(self, a: int, b: int) -> int:
        return Calculator.add(a, b) + self.offset
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have static method (no &self, no cls)
    assert!(
        rust_code.contains("fn add"),
        "Should have add static function.\nGot:\n{}",
        rust_code
    );

    // Should have classmethod (no &self, no cls parameter)
    assert!(
        rust_code.contains("fn with_default_offset"),
        "Should have with_default_offset classmethod.\nGot:\n{}",
        rust_code
    );

    // Should have instance method (with &self)
    assert!(
        rust_code.contains("fn add_with_offset"),
        "Should have add_with_offset instance method.\nGot:\n{}",
        rust_code
    );

    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(
        has_self,
        "add_with_offset should have &self.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_classmethod_with_default_parameters() {
    let python = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    @classmethod
    def create_anonymous(cls, age: int = 18):
        return cls("Anonymous", age)
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
        rust_code.contains("fn create_anonymous"),
        "Should have create_anonymous function.\nGot:\n{}",
        rust_code
    );

    // Should have age parameter with default or Option
    let has_age_param = rust_code.contains("age");
    assert!(
        has_age_param,
        "Should have age parameter.\nGot:\n{}",
        rust_code
    );

    // Should call constructor with "Anonymous" and age
    let has_constructor = rust_code.contains("Self::new")
        || rust_code.contains("Person::new")
        || rust_code.contains("\"Anonymous\"");
    assert!(
        has_constructor,
        "Should construct with 'Anonymous'.\nGot:\n{}",
        rust_code
    );
}

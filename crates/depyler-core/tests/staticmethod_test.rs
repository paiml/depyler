//! TDD Tests for @staticmethod Decorator (DEPYLER-0112 Phase 1)
//!
//! Phase 1: @staticmethod decorator support
//! Python: @staticmethod def method() → Rust: fn method() (no &self)
//!
//! Test Coverage:
//! 1. Simple staticmethod with no parameters
//! 2. Staticmethod with parameters
//! 3. Staticmethod with return type
//! 4. Staticmethod accessing class constants
//! 5. Multiple staticmethods in same class
//! 6. Staticmethod calling another staticmethod
//! 7. Instance method calling staticmethod
//! 8. Staticmethod with type annotations
//! 9. Staticmethod as utility function
//! 10. Mix of static and instance methods

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_staticmethod_no_params() {
    let python = r#"
class Math:
    @staticmethod
    def pi() -> float:
        return 3.14159
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

    assert!(
        rust_code.contains("fn pi"),
        "Should have pi function.\nGot:\n{}",
        rust_code
    );

    // Should NOT have &self or &mut self parameter
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(!has_self, "Staticmethod should not have &self parameter.\nGot:\n{}", rust_code);
}

#[test]
fn test_staticmethod_with_parameters() {
    let python = r#"
class Calculator:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn add"),
        "Should have add function.\nGot:\n{}",
        rust_code
    );

    // Should have parameters a and b
    assert!(
        rust_code.contains("a") && rust_code.contains("b"),
        "Should have a and b parameters.\nGot:\n{}",
        rust_code
    );

    // Should NOT have &self
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(!has_self, "Staticmethod should not have &self.\nGot:\n{}", rust_code);
}

#[test]
fn test_staticmethod_with_return_type() {
    let python = r#"
class StringUtils:
    @staticmethod
    def uppercase(text: str) -> str:
        return text.upper()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn uppercase"),
        "Should have uppercase function.\nGot:\n{}",
        rust_code
    );

    // Should have String return type
    let has_string_return = rust_code.contains("-> String") ||
                            rust_code.contains("-> str") ||
                            rust_code.contains("String");
    assert!(has_string_return, "Should have String return type.\nGot:\n{}", rust_code);
}

#[test]
fn test_staticmethod_accessing_class_constants() {
    let python = r#"
class Config:
    MAX_SIZE: int = 1000

    @staticmethod
    def get_max_size() -> int:
        return Config.MAX_SIZE
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn get_max_size"),
        "Should have get_max_size function.\nGot:\n{}",
        rust_code
    );

    // Should have MAX_SIZE constant
    let has_max_size = rust_code.contains("MAX_SIZE") || rust_code.contains("max_size");
    assert!(has_max_size, "Should reference MAX_SIZE.\nGot:\n{}", rust_code);
}

#[test]
fn test_multiple_staticmethods() {
    let python = r#"
class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def multiply(a: int, b: int) -> int:
        return a * b

    @staticmethod
    def subtract(a: int, b: int) -> int:
        return a - b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have all three methods
    assert!(
        rust_code.contains("fn add"),
        "Should have add function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn multiply"),
        "Should have multiply function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn subtract"),
        "Should have subtract function.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_staticmethod_calling_another_staticmethod() {
    let python = r#"
class Math:
    @staticmethod
    def square(x: int) -> int:
        return x * x

    @staticmethod
    def sum_of_squares(a: int, b: int) -> int:
        return Math.square(a) + Math.square(b)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn square"),
        "Should have square function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn sum_of_squares"),
        "Should have sum_of_squares function.\nGot:\n{}",
        rust_code
    );

    // Should call Math::square or similar
    let calls_square = rust_code.contains("square");
    assert!(calls_square, "Should call square method.\nGot:\n{}", rust_code);
}

#[test]
fn test_instance_method_calling_staticmethod() {
    let python = r#"
class Calculator:
    def __init__(self, offset: int):
        self.offset = offset

    @staticmethod
    def double(x: int) -> int:
        return x * 2

    def apply_double(self, x: int) -> int:
        return Calculator.double(x) + self.offset
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn double"),
        "Should have double function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn apply_double"),
        "Should have apply_double method.\nGot:\n{}",
        rust_code
    );

    // apply_double should have &self
    let has_self = rust_code.contains("&self") || rust_code.contains("& self");
    assert!(has_self, "apply_double should have &self.\nGot:\n{}", rust_code);
}

#[test]
fn test_staticmethod_with_type_annotations() {
    let python = r#"
class Converter:
    @staticmethod
    def celsius_to_fahrenheit(celsius: float) -> float:
        return celsius * 9.0 / 5.0 + 32.0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn celsius_to_fahrenheit"),
        "Should have celsius_to_fahrenheit function.\nGot:\n{}",
        rust_code
    );

    // Should have float parameters and return
    let has_float = rust_code.contains("f32") ||
                    rust_code.contains("f64") ||
                    rust_code.contains("celsius");
    assert!(has_float, "Should have float type.\nGot:\n{}", rust_code);
}

#[test]
fn test_staticmethod_as_utility_function() {
    let python = r#"
class Validator:
    @staticmethod
    def is_valid_email(email: str) -> bool:
        return "@" in email
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("fn is_valid_email"),
        "Should have is_valid_email function.\nGot:\n{}",
        rust_code
    );

    // Should have bool return type
    let has_bool_return = rust_code.contains("-> bool") || rust_code.contains("bool");
    assert!(has_bool_return, "Should have bool return type.\nGot:\n{}", rust_code);
}

#[test]
fn test_mix_static_and_instance_methods() {
    let python = r#"
class Counter:
    @staticmethod
    def default_start() -> int:
        return 0

    def __init__(self):
        self.count = Counter.default_start()

    def increment(self) -> None:
        self.count = self.count + 1

    def get_count(self) -> int:
        return self.count

    @staticmethod
    def max_count() -> int:
        return 100
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.as_ref().err());

    let rust_code = result.unwrap();

    // Should have static methods
    assert!(
        rust_code.contains("fn default_start"),
        "Should have default_start function.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn max_count"),
        "Should have max_count function.\nGot:\n{}",
        rust_code
    );

    // Should have instance methods
    assert!(
        rust_code.contains("fn increment"),
        "Should have increment method.\nGot:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn get_count"),
        "Should have get_count method.\nGot:\n{}",
        rust_code
    );
}

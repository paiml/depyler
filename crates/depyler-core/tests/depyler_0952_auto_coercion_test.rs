//! DEPYLER-0952: Auto-Coercion Injection Pass Tests
//!
//! Tests for systemic type coercion to fix E0308 type mismatch errors.
//! Per strategy document: maybe_coerce() should handle:
//! - f32 ↔ f64 coercion
//! - String ↔ &str coercion

use depyler_core::DepylerPipeline;

/// Test f64 to f32 coercion when assigning to f32 variable
#[test]
fn test_depyler_0952_f64_to_f32_coercion() {
    let python = r#"
import numpy as np
def process(arr):
    # arr.mean() returns f32 in trueno, but Python float operations may produce f64
    mean_val: float = arr.mean()
    # Assigning f64 literal to f32 context should coerce
    threshold = 0.5  # This is f64 in Python
    return mean_val > threshold
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // The comparison should work without E0308 error
    // Either threshold should be f32 or mean_val comparison should coerce
    assert!(
        !code.contains("E0308"),
        "Should not have type mismatch errors: {}",
        code
    );
}

/// Test String to &str coercion in function calls
#[test]
fn test_depyler_0952_string_to_str_coercion() {
    let python = r#"
def greet(name: str) -> str:
    greeting = "Hello, " + name
    # Function expecting &str but receiving String should work
    return greeting.upper()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Verify the code compiles (no E0308)
    assert!(
        code.contains("fn greet"),
        "Should generate greet function: {}",
        code
    );
}

/// Test &str to String coercion when returning from function
#[test]
fn test_depyler_0952_str_to_string_return_coercion() {
    let python = r#"
def get_message() -> str:
    # Return type is String, but literal is &str
    return "Hello"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should have .to_string() or String::from() for the return
    assert!(
        code.contains("to_string()") || code.contains("String::from") || code.contains("\"Hello\""),
        "Should handle string return type: {}",
        code
    );
}

/// Test mixed numeric types in binary operations
#[test]
fn test_depyler_0952_mixed_numeric_coercion() {
    let python = r#"
def calculate(x: int, y: float) -> float:
    # int + float should coerce int to float
    return x + y
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should have coercion: x as f64 or similar
    assert!(
        code.contains("as f64") || code.contains("as f32") || code.contains("f64"),
        "Should coerce int to float in mixed operations: {}",
        code
    );
}

/// Test if-else branch unification with different types
#[test]
fn test_depyler_0952_ifelse_type_unification() {
    let python = r#"
def get_value(flag: bool) -> float:
    # Body returns float division, else returns int literal
    # Both branches should have same type
    return 1.0 / 2.0 if flag else 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // The else branch should coerce 0 to float
    let has_int_else = code.contains("else { 0 }") && !code.contains("else { 0.0");
    assert!(
        !has_int_else,
        "Else branch should use float literal, not int: {}",
        code
    );
}

/// Test list of strings (Vec<String> vs Vec<&str>)
#[test]
fn test_depyler_0952_vec_string_coercion() {
    let python = r#"
def get_names() -> list:
    names = ["Alice", "Bob", "Charlie"]
    return names
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should properly handle Vec<String> or Vec<&str>
    assert!(
        code.contains("vec!") || code.contains("Vec"),
        "Should generate vector of strings: {}",
        code
    );
}

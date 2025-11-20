//! DEPYLER-0438: F-String Display Formatter Tests
//!
//! Tests verifying that f-strings use {} (Display) instead of {:?} (Debug).
//!
//! Bug: F-strings were using {:?} which prints strings with quotes.
//! Fix: Use {} to match Python semantics.

use depyler_core::DepylerPipeline;

#[test]
fn test_depyler_0438_fstring_simple_variable() {
    let source = r#"
name = "Alice"
message = f"Hello, {name}!"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} not {:?}
    assert!(
        result.contains(r#"format!("Hello, {}!", name)"#),
        "Expected Display formatter {{}}, got:\n{}",
        result
    );

    // Should NOT contain {:?}
    assert!(
        !result.contains("{:?}"),
        "Should not use Debug formatter {{:?}}, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_multiple_expressions() {
    let source = r#"
x = 5
y = 10
result = f"x={x}, y={y}, sum={x+y}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} for all expressions, not {:?}
    assert!(
        result.contains(r#"format!("x={}, y={}, sum={}"#),
        "Expected Display formatter for multiple vars, got:\n{}",
        result
    );

    assert!(
        !result.contains("{:?}"),
        "Should not use Debug formatter, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_print_statement() {
    let source = r#"
name = "Bob"
print(f"Hello, {name}!")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} in format! for print
    assert!(
        result.contains(r#"format!("Hello, {}!", name)"#),
        "Expected Display formatter in print, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_with_expression() {
    let source = r#"
x = 10
y = 20
message = f"Sum is: {x + y}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} for expressions
    assert!(
        result.contains(r#"format!("Sum is: {}"#),
        "Expected Display formatter for expression, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_integer() {
    let source = r#"
count = 42
message = f"Count: {count}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} for integers (they implement Display)
    assert!(
        result.contains(r#"format!("Count: {}",  count)"#) || result.contains(r#"format!("Count: {}", count)"#),
        "Expected Display formatter for integer, got:\n{}",
        result
    );

    assert!(
        !result.contains("{:?}"),
        "Should not use Debug formatter for integer, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_float() {
    let source = r#"
pi = 3.14159
message = f"Pi is approximately {pi}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} for floats
    assert!(
        result.contains(r#"format!("Pi is approximately {}"#),
        "Expected Display formatter for float, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_mixed_types() {
    let source = r#"
name = "Alice"
age = 30
height = 5.6
message = f"{name} is {age} years old and {height} feet tall"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} for all types (String, int, float)
    assert!(
        result.contains(r#"format!("{} is {} years old and {} feet tall"#),
        "Expected Display formatter for mixed types, got:\n{}",
        result
    );

    assert!(
        !result.contains("{:?}"),
        "Should not use Debug formatter for any type, got:\n{}",
        result
    );
}

#[test]
fn test_depyler_0438_fstring_empty_expression() {
    let source = r#"
x = ""
message = f"Value: {x}"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(source).unwrap();

    // Should use {} even for empty string
    assert!(
        result.contains(r#"format!("Value: {}"#),
        "Expected Display formatter for empty string, got:\n{}",
        result
    );
}

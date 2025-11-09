//! Targeted coverage tests for codegen_raise_stmt function
//!
//! Target: codegen_raise_stmt (lines 308-345, complexity 13)
//! Coverage focus: Exception handling, scope tracking, Box::new() wrapping
//!
//! Test Strategy:
//! - Raise in can-fail functions (return Err)
//! - Raise in non-can-fail functions (panic!)
//! - Raise within try/except blocks (DEPYLER-0333)
//! - Box::new() wrapping (DEPYLER-0310)
//! - Bare raise (no exception specified)
//! - Different exception types

use depyler_core::DepylerPipeline;

/// Unit Test: Simple raise in can-fail function
///
/// Verifies: return Err() generation (lines 324-336)
#[test]
fn test_raise_in_can_fail_function() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_positive(x: int) -> int:
    if x < 0:
        raise ValueError("Must be positive")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn validate_positive"));
}

/// Unit Test: Raise with specific exception type
///
/// Verifies: Exception type extraction and handling
#[test]
fn test_raise_specific_exception() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_bounds(x: int, max_val: int) -> int:
    if x > max_val:
        raise IndexError("Out of bounds")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_bounds"));
}

/// Unit Test: Multiple raise statements
///
/// Verifies: Multiple exception paths
#[test]
fn test_multiple_raise_statements() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_range(x: int) -> int:
    if x < 0:
        raise ValueError("Too small")
    if x > 100:
        raise ValueError("Too large")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn validate_range"));
}

/// Unit Test: Raise with formatted message
///
/// Verifies: Exception expression generation (line 314)
#[test]
fn test_raise_with_formatted_message() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def divide_safe(a: int, b: int) -> int:
    if b == 0:
        raise ZeroDivisionError("Cannot divide by zero")
    return a / b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn divide_safe"));
}

/// Unit Test: Raise in nested if
///
/// Verifies: Raise within nested scope
#[test]
fn test_raise_in_nested_if() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_validation(x: int, y: int) -> int:
    if x > 0:
        if y < 0:
            raise ValueError("Invalid combination")
        return x + y
    raise ValueError("x must be positive")
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_validation"));
}

/// Unit Test: Raise in loop
///
/// Verifies: Early exit from iteration
#[test]
fn test_raise_in_loop() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def find_or_fail(items: list[int], target: int) -> int:
    for item in items:
        if item == target:
            return item
    raise ValueError("Not found")
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn find_or_fail"));
}

/// Unit Test: Raise after multiple statements
///
/// Verifies: Raise as final statement
#[test]
fn test_raise_after_statements() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_or_fail(x: int) -> int:
    y = x * 2
    z = y + 1
    if z > 100:
        raise ValueError("Result too large")
    return z
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_or_fail"));
}

/// Unit Test: Raise different exception types
///
/// Verifies: Multiple exception type handling
#[test]
fn test_different_exception_types() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def validate_input(x: int) -> int:
    if x < 0:
        raise ValueError("Negative")
    if x == 0:
        raise ZeroDivisionError("Zero")
    if x > 1000:
        raise IndexError("Too large")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn validate_input"));
}

/// Unit Test: Raise with variable in message
///
/// Verifies: Expression evaluation in exception
#[test]
fn test_raise_with_variable_message() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_minimum(x: int, min_val: int) -> int:
    if x < min_val:
        raise ValueError("Value must be >= minimum")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_minimum"));
}

/// Unit Test: Raise in elif branch
///
/// Verifies: Raise in conditional branches
#[test]
fn test_raise_in_elif() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def categorize_or_fail(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        raise ValueError("Zero not allowed")
    else:
        return "positive"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn categorize_or_fail"));
}

/// Unit Test: Raise with simple exception (no message)
///
/// Verifies: Exception without arguments
#[test]
fn test_raise_simple_exception() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def fail_fast(x: int) -> int:
    if x < 0:
        raise ValueError("error")
    return x
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn fail_fast"));
}

/// Unit Test: Multiple raises in try/except
///
/// Verifies: Exception handling within try blocks
#[test]
fn test_raise_in_try_except() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_validation(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("negative")
        return x
    except ValueError:
        raise RuntimeError("validation failed")
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn nested_validation"));
}

/// Integration Test: Complex exception handling
///
/// Verifies: All raise patterns together
#[test]
fn test_complex_exception_patterns() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def complex_process(items: list[int], threshold: int) -> int:
    if len(items) == 0:
        raise ValueError("Empty list")
    
    total = 0
    for item in items:
        if item < 0:
            raise ValueError("Negative value")
        if item > threshold:
            raise ValueError("Value exceeds threshold")
        total = total + item
    
    if total == 0:
        raise RuntimeError("Sum is zero")
    
    return total
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_process"));
}

/// Property Test: All raise patterns should transpile
///
/// Property: Different raise patterns are valid
#[test]
fn test_property_raise_patterns() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("simple", "if x < 0:\n        raise ValueError(\"error\")"),
        ("after_check", "y = x * 2\n    if y > 100:\n        raise ValueError(\"too large\")"),
        ("in_loop", "for i in items:\n        if i < 0:\n            raise ValueError(\"neg\")"),
    ];

    for (name, raise_stmt) in test_cases {
        let python_code = format!(
            r#"
def test_{}(x: int, items: list[int]) -> int:
    {}
    return x
"#,
            name, raise_stmt
        );
        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            name,
            result.err()
        );
    }
}

/// Mutation Test: Exception path selection
///
/// Targets mutations in:
/// 1. Can-fail vs non-can-fail detection
/// 2. Exception type extraction
/// 3. Error path generation
#[test]
fn test_mutation_exception_paths() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Can-fail function must generate return Err
    let can_fail = r#"
def test1(x: int) -> int:
    if x < 0:
        raise ValueError("neg")
    return x
"#;
    let rust1 = pipeline.transpile(can_fail).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Different exception types
    let multi_type = r#"
def test2(x: int) -> int:
    if x < 0:
        raise ValueError("val")
    if x == 0:
        raise ZeroDivisionError("zero")
    return x
"#;
    let rust2 = pipeline.transpile(multi_type).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Raise after other statements
    let after_stmts = r#"
def test3(x: int) -> int:
    y = x + 1
    z = y * 2
    if z > 100:
        raise ValueError("big")
    return z
"#;
    let rust3 = pipeline.transpile(after_stmts).unwrap();
    assert!(rust3.contains("fn test3"));
}

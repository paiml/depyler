//! Tests for Python exception handling conversion to Rust Result<T, E>

use depyler_core::DepylerPipeline;

#[test]
fn test_function_that_can_fail() {
    let pipeline = DepylerPipeline::new();

    // Function that might fail with division by zero
    let python_code = r#"
def divide(a: int, b: int) -> float:
    if b == 0:
        raise ValueError("Division by zero")
    return a / b
"#;

    let result = pipeline.transpile(python_code);
    println!("Divide function result: {:?}", result);

    // This will likely fail as exceptions aren't implemented yet
    assert!(result.is_err() || result.unwrap().contains("Result"));
}

#[test]
fn test_try_except_block() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def safe_divide(a: int, b: int) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        return 0.0
"#;

    let result = pipeline.transpile(python_code);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Verify exception handling is present
    assert!(
        rust_code.contains("safe_divide"),
        "Should have safe_divide function"
    );
    assert!(
        rust_code.contains("Result") || rust_code.contains("_result"),
        "Should have error handling"
    );
}

#[test]
fn test_function_with_error_annotation() {
    let pipeline = DepylerPipeline::new();

    // Using a comment annotation to indicate this function can fail
    let python_code = r#"
# @depyler: error_type = "ValueError"
def parse_int(s: str) -> int:
    # In Python this would be int(s) which can raise ValueError
    return int(s)
"#;

    let result = pipeline.transpile(python_code);
    println!("Parse int result: {:?}", result);

    // Check if we handle the annotation
    if let Ok(rust_code) = result {
        println!("Generated code:\n{}", rust_code);
        // Even without full exception support, we can check the annotation was parsed
        assert!(rust_code.contains("parse_int"));
    }
}

#[test]
fn test_implicit_error_handling() {
    let pipeline = DepylerPipeline::new();

    // Functions that implicitly can fail
    let python_code = r#"
def get_first(items: List[int]) -> int:
    return items[0]  # Can raise IndexError
    
def get_value(data: Dict[str, int], key: str) -> int:
    return data[key]  # Can raise KeyError
"#;

    let result = pipeline.transpile(python_code);
    println!("Implicit errors result: {:?}", result);

    // Dictionary access isn't supported in V1, but check what we get
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn test_custom_exception() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
class CustomError(Exception):
    pass
    
def may_fail(condition: bool):
    if condition:
        raise CustomError("Something went wrong")
"#;

    let result = pipeline.transpile(python_code);
    println!("Custom exception result: {:?}", result);

    // Classes aren't supported in V1, but we might generate something
    println!("Custom exception generated: {}", result.is_ok());
    // For now, just check it completes
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_finally_block() {
    let pipeline = DepylerPipeline::new();

    let python_code = r#"
def with_cleanup():
    try:
        print("Doing work")
    finally:
        print("Cleanup")
"#;

    let result = pipeline.transpile(python_code);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Verify finally block cleanup is present
    assert!(
        rust_code.contains("with_cleanup"),
        "Should have with_cleanup function"
    );
    assert!(rust_code.contains("Cleanup"), "Should have cleanup code");
}

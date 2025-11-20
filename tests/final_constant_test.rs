/// Test for Python's typing.Final annotation transpiling to Rust const
///
/// This test verifies that:
/// - FIELD_GOAL: Final[int] = 600 transpiles to const FIELD_GOAL: i32 = 600;
/// - The const keyword is used instead of let
/// - The type annotation is preserved (without the Final wrapper)
use depyler_core::DepylerPipeline;

#[test]
fn test_final_int_constant() {
    let python_code = r#"
from typing import Final

FIELD_GOAL: Final[int] = 600
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should succeed. Error: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    // Check that 'const' is used instead of 'let'
    assert!(
        rust_code.contains("const FIELD_GOAL"),
        "Should generate 'const FIELD_GOAL', but got:\n{}",
        rust_code
    );

    // Check that the type annotation is i32 (not Final<i32>)
    assert!(
        rust_code.contains(": i32"),
        "Should have type annotation ': i32', but got:\n{}",
        rust_code
    );

    // Check that the value is 600
    assert!(
        rust_code.contains("= 600"),
        "Should have value '= 600', but got:\n{}",
        rust_code
    );

    // Ensure 'let' is not used for this constant
    assert!(
        !rust_code.contains("let FIELD_GOAL"),
        "Should NOT use 'let' for Final annotation, but got:\n{}",
        rust_code
    );
}

#[test]
fn test_final_string_constant() {
    let python_code = r#"
from typing import Final

API_KEY: Final[str] = "secret_key_123"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should succeed. Error: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    // Check that 'const' is used
    assert!(
        rust_code.contains("const API_KEY"),
        "Should generate 'const API_KEY', but got:\n{}",
        rust_code
    );

    // Check that the type annotation is &str or String
    assert!(
        rust_code.contains(": &str") || rust_code.contains(": String"),
        "Should have string type annotation, but got:\n{}",
        rust_code
    );
}

#[test]
fn test_final_float_constant() {
    let python_code = r#"
from typing import Final

PI: Final[float] = 3.14159
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should succeed. Error: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    // Check that 'const' is used
    assert!(
        rust_code.contains("const PI"),
        "Should generate 'const PI', but got:\n{}",
        rust_code
    );

    // Check that the type annotation is f64
    assert!(
        rust_code.contains(": f64"),
        "Should have type annotation ': f64', but got:\n{}",
        rust_code
    );
}

#[test]
fn test_multiple_final_constants() {
    let python_code = r#"
from typing import Final

MAX_CONNECTIONS: Final[int] = 100
TIMEOUT_MS: Final[int] = 5000
DEFAULT_NAME: Final[str] = "unnamed"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should succeed. Error: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    // Check that all constants use 'const'
    assert!(
        rust_code.contains("const MAX_CONNECTIONS"),
        "Should generate 'const MAX_CONNECTIONS', but got:\n{}",
        rust_code
    );
    assert!(
        rust_code.contains("const TIMEOUT_MS"),
        "Should generate 'const TIMEOUT_MS', but got:\n{}",
        rust_code
    );
    assert!(
        rust_code.contains("const DEFAULT_NAME"),
        "Should generate 'const DEFAULT_NAME', but got:\n{}",
        rust_code
    );
}

#[test]
fn test_final_vs_regular_variable() {
    let python_code = r#"
from typing import Final

def process() -> int:
    MAX_VALUE: Final[int] = 100
    current: int = 50
    return current
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should succeed. Error: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    // Check that Final uses 'const'
    assert!(
        rust_code.contains("const MAX_VALUE"),
        "Should generate 'const MAX_VALUE' for Final annotation, but got:\n{}",
        rust_code
    );

    // Check that regular variable uses 'let'
    assert!(
        rust_code.contains("let current"),
        "Should generate 'let current' for regular variable, but got:\n{}",
        rust_code
    );
}

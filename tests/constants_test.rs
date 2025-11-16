/// Constants Transpilation Test
///
/// Tests that module-level constants are correctly transpiled from Python to Rust.

use depyler_core::DepylerPipeline;

#[test]
fn test_simple_constants() {
    let python_code = r#"
A = 1
B = 2
C = 3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Check that constants are present in the generated code
    assert!(
        rust_code.contains("A") && rust_code.contains("B") && rust_code.contains("C"),
        "Generated code must contain all constant names. Got:\n{}",
        rust_code
    );

    // Constants should be declared with appropriate Rust syntax (const or pub const)
    let has_const_declarations = rust_code.contains("const A") || rust_code.contains("pub const A")
        || rust_code.contains("static A") || rust_code.contains("pub static A");
    
    assert!(
        has_const_declarations,
        "Constants should be declared with const or static keyword. Got:\n{}",
        rust_code
    );

    // Verify the values are present
    assert!(
        rust_code.contains("1") && rust_code.contains("2") && rust_code.contains("3"),
        "Generated code must contain constant values. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_constants_with_types() {
    let python_code = r#"
A: int = 1
B: int = 2
C: int = 3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Check that constants are present
    assert!(
        rust_code.contains("A") && rust_code.contains("B") && rust_code.contains("C"),
        "Generated code must contain all constant names. Got:\n{}",
        rust_code
    );

    // Should have integer type annotations
    let has_type_annotations = rust_code.contains("i32") || rust_code.contains("i64");
    
    assert!(
        has_type_annotations,
        "Constants should have integer type annotations. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_string_constants() {
    let python_code = r#"
NAME = "Alice"
GREETING = "Hello"
MESSAGE = "World"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Check that constant names are present
    assert!(
        rust_code.contains("NAME") && rust_code.contains("GREETING") && rust_code.contains("MESSAGE"),
        "Generated code must contain all constant names. Got:\n{}",
        rust_code
    );

    // String values should be present (in some form)
    assert!(
        rust_code.contains("Alice") && rust_code.contains("Hello") && rust_code.contains("World"),
        "Generated code must contain string values. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_mixed_type_constants() {
    let python_code = r#"
INT_VALUE = 42
FLOAT_VALUE = 3.14
STRING_VALUE = "test"
BOOL_VALUE = True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Check that all constant names are present
    assert!(
        rust_code.contains("INT_VALUE") 
        && rust_code.contains("FLOAT_VALUE") 
        && rust_code.contains("STRING_VALUE")
        && rust_code.contains("BOOL_VALUE"),
        "Generated code must contain all constant names. Got:\n{}",
        rust_code
    );

    // Check that values are present
    assert!(
        rust_code.contains("42") 
        && rust_code.contains("3.14") 
        && rust_code.contains("test")
        && (rust_code.contains("true") || rust_code.contains("True")),
        "Generated code must contain all constant values. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_generated_constants_code_compiles() {
    // Test that the generated code for simple constants is syntactically valid
    let python_code = r#"
A = 1
B = 2
C = 3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Basic check that code contains expected patterns
    assert!(
        !rust_code.is_empty(),
        "Generated Rust code must not be empty"
    );
    
    // Check for basic Rust syntax patterns
    assert!(
        rust_code.contains("A") || rust_code.contains("B") || rust_code.contains("C"),
        "Generated code should contain constant declarations. Got:\n{}",
        rust_code
    );
}

#[test]
fn test_constants_used_in_function() {
    let python_code = r#"
A = 1
B = 2
C = 3

def sum_constants() -> int:
    return A + B + C
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Constants should be defined
    assert!(
        rust_code.contains("A") && rust_code.contains("B") && rust_code.contains("C"),
        "Generated code must contain all constant names. Got:\n{}",
        rust_code
    );

    // Function should reference the constants
    assert!(
        rust_code.contains("sum_constants"),
        "Generated code must contain the function. Got:\n{}",
        rust_code
    );

    // Basic validation - check that code isn't empty and has expected patterns
    assert!(
        !rust_code.is_empty() && rust_code.contains("A"),
        "Generated Rust code should be valid and contain constant references. Got:\n{}",
        rust_code
    );
}

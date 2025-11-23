//! DEPYLER-0269: isinstance() Generates Invalid Rust Code
//!
//! Tests that isinstance() calls are properly handled in transpilation.
//! For statically-typed Rust, isinstance(x, T) where x: T is always true.

use depyler_core::DepylerPipeline;

#[test]
fn test_isinstance_int_removed() {
    // Python: isinstance with type-annotated int parameter
    let python = r#"
def check_int(value: int) -> bool:
    """Check if value is an integer (always True due to type system)."""
    return isinstance(value, int)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // BUG: Should NOT contain isinstance (undefined in Rust)
    assert!(
        !rust.contains("isinstance"),
        "BUG: isinstance should be removed/optimized (undefined in Rust)\nGenerated:\n{}",
        rust
    );

    // Should return true (type system guarantees value: i32 is always int)
    assert!(
        rust.contains("true") || rust.contains("True"),
        "Expected function to return true (type system guarantee)\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_isinstance_str_removed() {
    // Python: isinstance with type-annotated str parameter
    let python = r#"
def check_str(value: str) -> bool:
    """Check if value is a string (always True due to type system)."""
    return isinstance(value, str)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should NOT contain isinstance
    assert!(
        !rust.contains("isinstance"),
        "BUG: isinstance should be removed\nGenerated:\n{}",
        rust
    );

    // Should return true
    assert!(
        rust.contains("true") || rust.contains("True"),
        "Expected function to return true\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_isinstance_list_removed() {
    // Python: isinstance with List type
    let python = r#"
def check_list(items: list) -> bool:
    return isinstance(items, list)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    assert!(
        !rust.contains("isinstance"),
        "BUG: isinstance should be removed\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_isinstance_dict_removed() {
    // Python: isinstance with dict type
    let python = r#"
def check_dict(data: dict) -> bool:
    return isinstance(data, dict)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    assert!(
        !rust.contains("isinstance"),
        "BUG: isinstance should be removed\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_isinstance_compiles() {
    // Critical: Generated code MUST compile
    let python = r#"
def type_check_int(value: int) -> bool:
    """Check if value is an integer (always True due to type system)."""
    return isinstance(value, int)

def type_check_str(value: str) -> bool:
    """Check if value is a string (always True due to type system)."""
    return isinstance(value, str)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Verify code does not contain undefined identifiers
    assert!(
        !rust.contains("isinstance"),
        "Generated code contains undefined 'isinstance'\nGenerated:\n{}",
        rust
    );

    // Note: Actual rustc compilation check would go here in a full integration test
    // For unit tests, we verify the transpiler output is correct
}

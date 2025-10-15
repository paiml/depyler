//! TDD Test for Type Annotation Preservation
//! Testing TRANSPILER_BUG_type_annotations.md fix
//!
//! This test SHOULD FAIL initially, then pass after implementation

use depyler_core::DepylerPipeline;

#[test]
fn test_type_annotation_usize_to_i32() {
    let python = r#"
def test() -> int:
    arr = [1, 2, 3]
    right: int = len(arr) - 1
    return right
    "#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Expected: Type annotation should force conversion to i32
    // Use regex-style check to handle variable whitespace
    assert!(
        rust_code.contains("right: i32"),
        "Type annotation 'int' should produce 'i32' type in Rust.\nGot:\n{}",
        rust_code
    );

    // Should have conversion from usize to i32
    assert!(
        rust_code.contains("as i32") || rust_code.contains(".try_into()"),
        "Should convert usize (from len()) to i32.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_type_annotation_simple_int() {
    let python = r#"
def test(val: int) -> int:
    x: int = val + 1
    return x
    "#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    // Print full error for debugging
    if result.is_err() {
        eprintln!("ERROR: {:#?}", result.as_ref().err());
    }

    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Type annotation should be preserved for non-constant values
    assert!(
        rust_code.contains("x: i32"),
        "Type annotation should be preserved.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_type_annotation_str() {
    let python = r#"
def test() -> str:
    name: str = "hello"
    return name
    "#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    assert!(
        rust_code.contains("String") || rust_code.contains("&str"),
        "Type annotation 'str' should produce String/&str in Rust.\nGot:\n{}",
        rust_code
    );
}

#[test]
fn test_no_annotation_infers_type() {
    let python = r#"
def test(n: int) -> int:
    x = n + 1
    return x
    "#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Without annotation, should still work (type inference)
    // May be optimized to inline, so check function compiles
    assert!(
        rust_code.contains("pub fn test"),
        "Function should transpile successfully.\nGot:\n{}",
        rust_code
    );
}

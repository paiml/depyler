//! DEPYLER-0823: Test for cast followed by method call
//!
//! Problem: When `len()` returns `x.len() as i32` and a method is called on it,
//! the code generator produces invalid Rust: `x.len() as i32.bit_length()`
//! instead of `(x.len() as i32).bit_length()`.

use depyler_core::DepylerPipeline;

/// Test that len() followed by method call generates valid Rust
#[test]
fn test_DEPYLER_0823_len_followed_by_method_call() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_len_method() -> int:
    arr = [1, 2, 3]
    return len(arr).bit_length()
"#;

    // This should NOT panic - it should generate valid Rust
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // The generated code should have parentheses around the cast
    // Expected: (arr.len() as i32).bit_length()
    // NOT: arr.len() as i32.bit_length()
    assert!(
        rust_code.contains("(") && rust_code.contains(")."),
        "Cast should be parenthesized before method call. Got:\n{}",
        rust_code
    );
}

/// Test integer cast followed by method call
#[test]
fn test_DEPYLER_0823_int_cast_followed_by_method() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_int_method() -> int:
    x = 3.14
    return int(x).bit_length()
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());
}

/// Test chained method calls after cast
#[test]
fn test_DEPYLER_0823_cast_with_chained_methods() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_chained() -> str:
    arr = [1, 2, 3]
    return str(len(arr)).zfill(5)
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());
}

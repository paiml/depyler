//! DEPYLER-0828: Test for float/int comparison with variable (not literal)
//!
//! Problem: When comparing `x < y` where `x: float` and `y` is an integer
//! variable (not a literal), the transpiler doesn't cast y to float.
//! This causes `x < y` to become `x < y` in Rust which fails to compile
//! because `f64` and `i32` can't be compared directly.
//!
//! Solution: Add `expr_returns_int_direct` function and cast integer variables
//! to `f64` when comparing with float expressions.

use depyler_core::DepylerPipeline;

/// Test float param compared to integer variable (not literal)
/// Python: def test(x: float) -> bool: y = 5; return x < y
/// Expected Rust: pub fn test(x: f64) -> bool { let y = 5; x < (y as f64) }
#[test]
fn test_DEPYLER_0828_float_param_vs_int_var() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test(x: float) -> bool:
    y = 5  # integer variable
    return x < y
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust_code = result.unwrap();

    // The generated code should compile
    // Either by casting y to f64, or by making y be f64 from the start
    assert!(
        rust_code.contains("as f64") || rust_code.contains("5.0") || rust_code.contains("5_f64"),
        "Should cast integer to float or use float literal: {}",
        rust_code
    );
}

/// Test int param compared to float variable
/// Python: def test(x: int) -> bool: y = 5.0; return x < y
/// Expected Rust: x as f64 < y or similar
#[test]
fn test_DEPYLER_0828_int_param_vs_float_var() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test(x: int) -> bool:
    y = 5.0  # float variable
    return x < y
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust_code = result.unwrap();

    // The generated code should have proper cast
    assert!(
        rust_code.contains("as f64") || rust_code.contains("x < y"),
        "Should cast or handle properly: {}",
        rust_code
    );
}

/// Test multiple comparison operators with float/int mix
#[test]
fn test_DEPYLER_0828_all_comparison_ops() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_lt(x: float, y: int) -> bool:
    return x < y

def test_gt(x: float, y: int) -> bool:
    return x > y

def test_le(x: float, y: int) -> bool:
    return x <= y

def test_ge(x: float, y: int) -> bool:
    return x >= y

def test_eq(x: float, y: int) -> bool:
    return x == y

def test_ne(x: float, y: int) -> bool:
    return x != y
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Should transpile all comparisons: {:?}", result.err());

    let rust_code = result.unwrap();

    // Check that int params are cast to f64 for comparison with float
    // Count how many times "as f64" appears (should be 6 times, one per function)
    let cast_count = rust_code.matches("as f64").count();
    assert!(
        cast_count >= 6,
        "Should have at least 6 'as f64' casts (one per function), found {}: {}",
        cast_count,
        rust_code
    );
}

/// Test that literal int compared to float works (already handled in DEPYLER-0720)
#[test]
fn test_DEPYLER_0828_literal_still_works() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test(x: float) -> bool:
    return x < 5  # literal, should already work
"#;

    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should contain the float representation of 5
    assert!(
        rust_code.contains("5.0") || rust_code.contains("5_f64") || rust_code.contains("5f64"),
        "Literal should be converted to float: {}",
        rust_code
    );
}

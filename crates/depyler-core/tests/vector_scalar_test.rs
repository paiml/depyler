//! DEPYLER-0928: Vector-scalar arithmetic tests
//!
//! Tests for Vector-scalar operations with trueno:
//! - Vector - scalar (subtraction)
//! - Vector * scalar (multiplication with integer)
//! - Variable recognition for floats from numpy methods
//!
//! These tests verify that the transpiler generates correct trueno method calls
//! instead of raw binary operators that don't compile.

use depyler_core::DepylerPipeline;

/// Test Vector subtraction with scalar from numpy min() result
/// Python: arr - min_val where min_val = np.min(arr)
/// Rust: Should generate element-wise subtraction
#[test]
fn test_depyler_0928_vector_sub_scalar_from_min() {
    let python = r#"
import numpy as np

def normalize(a: float, b: float, c: float):
    arr = np.array([a, b, c])
    min_val = np.min(arr)
    return arr - min_val
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should use element-wise subtraction pattern
    // Either as_slice().iter().map() or Vector method
    assert!(
        rust_code.contains("as_slice")
            || rust_code.contains("sub_scalar")
            || rust_code.contains("map(|"),
        "Vector - scalar should use element-wise pattern, not raw operator.\nGot:\n{}",
        rust_code
    );
}

/// Test Vector multiplication with integer 0
/// Python: arr * 0
/// Rust: Should generate arr.scale(0f32).unwrap() or similar
#[test]
fn test_depyler_0928_vector_mul_integer_zero() {
    let python = r#"
import numpy as np

def zero_array(a: float, b: float, c: float):
    arr = np.array([a, b, c])
    return arr * 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should use scale() method for Vector * integer
    // OR convert the integer to f32
    assert!(
        rust_code.contains("scale(")
            || rust_code.contains("0f32")
            || rust_code.contains("0_f32")
            || rust_code.contains("0 as f32"),
        "Vector * integer should use scale() or cast to f32.\nGot:\n{}",
        rust_code
    );
}

/// Test Vector multiplication with integer (non-zero)
/// Python: arr * 2
/// Rust: Should generate arr.scale(2f32).unwrap() or similar
#[test]
fn test_depyler_0928_vector_mul_integer_nonzero() {
    let python = r#"
import numpy as np

def double_array(a: float, b: float, c: float):
    arr = np.array([a, b, c])
    return arr * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should use scale() method for Vector * integer
    assert!(
        rust_code.contains("scale(")
            || rust_code.contains("2f32")
            || rust_code.contains("2_f32")
            || rust_code.contains("2 as f32"),
        "Vector * integer should use scale() or cast to f32.\nGot:\n{}",
        rust_code
    );
}

/// Test minmax normalization pattern (complete pattern from failing example)
/// Python: (arr - min_val) / denom if denom > 0 else arr * 0
/// This is the exact pattern from example_numpy_minmax
#[test]
fn test_depyler_0928_minmax_normalization_pattern() {
    let python = r#"
import numpy as np

def minmax_normalize(a: float, b: float, c: float):
    arr = np.array([a, b, c])
    min_val = np.min(arr)
    max_val = np.max(arr)
    denom = max_val - min_val
    result = (arr - min_val) / denom if denom > 0 else arr * 0
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // The generated code should be valid Rust - no raw Vector ops
    // Check for proper trueno method calls
    let has_proper_vector_ops = rust_code.contains("as_slice")
        || rust_code.contains("scale(")
        || rust_code.contains("map(|");

    assert!(
        has_proper_vector_ops,
        "Minmax pattern should use proper trueno methods.\nGot:\n{}",
        rust_code
    );
}

/// Test that min_val from np.min is recognized as returning float
/// This tests the expr_returns_float recognition
#[test]
fn test_depyler_0928_min_val_recognized_as_float() {
    let python = r#"
import numpy as np

def test_min(a: float, b: float, c: float):
    arr = np.array([a, b, c])
    min_val = np.min(arr)
    # Using min_val in comparison should work with float semantics
    if min_val > 0:
        return min_val
    return 0.0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // min_val should be used in float comparison
    // The comparison should use f32 since trueno returns f32
    assert!(
        rust_code.contains("min_val > 0")
            || rust_code.contains("min_val > 0f32")
            || rust_code.contains("min_val > 0.0"),
        "min_val comparison should work.\nGot:\n{}",
        rust_code
    );
}

/// Test Vector addition with scalar
/// Python: arr + offset where offset is float
#[test]
fn test_depyler_0928_vector_add_scalar() {
    let python = r#"
import numpy as np

def add_offset(a: float, b: float, c: float, offset: float):
    arr = np.array([a, b, c])
    return arr + offset
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should use element-wise addition pattern
    assert!(
        rust_code.contains("as_slice")
            || rust_code.contains("add_scalar")
            || rust_code.contains("map(|"),
        "Vector + scalar should use element-wise pattern.\nGot:\n{}",
        rust_code
    );
}

/// Test Vector division by scalar
/// Python: arr / factor where factor is float
#[test]
fn test_depyler_0928_vector_div_scalar() {
    let python = r#"
import numpy as np

def scale_down(a: float, b: float, c: float, factor: float):
    arr = np.array([a, b, c])
    return arr / factor
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should use element-wise division pattern
    assert!(
        rust_code.contains("as_slice")
            || rust_code.contains("div_scalar")
            || rust_code.contains("map(|"),
        "Vector / scalar should use element-wise pattern.\nGot:\n{}",
        rust_code
    );
}

/// Test Vector multiplication with scalar that's an integer literal
/// This is the exact pattern from arr * 0 in minmax
#[test]
fn test_depyler_0928_vector_mul_integer_literal_in_ternary() {
    let python = r#"
import numpy as np

def conditional_zero(a: float, b: float, condition: bool):
    arr = np.array([a, b])
    result = arr if condition else arr * 0
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should handle arr * 0 in ternary expression
    // Either scale(0f32) or convert 0 to f32
    assert!(
        rust_code.contains("scale(")
            || rust_code.contains("0f32")
            || rust_code.contains("0_f32")
            || rust_code.contains("0 as f32"),
        "Vector * 0 in ternary should use proper Vector operation.\nGot:\n{}",
        rust_code
    );
}

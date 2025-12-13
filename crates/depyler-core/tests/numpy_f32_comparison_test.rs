//! DEPYLER-0927: Tests for f32 comparison coercion in numpy/trueno context
//!
//! Tests that when comparing f32 values (from trueno operations like norm_l2())
//! with integer literals, the literals are correctly coerced to f32.

use depyler_core::DepylerPipeline;

/// Test that norm_a > 0 generates norm_a > 0f32 when norm_a is from norm_l2()
#[test]
fn test_depyler_0927_comparison_f32_coercion_norm_a() {
    let python = r#"
import numpy as np
def check_norm(a):
    norm_a = a.norm_l2()
    return norm_a > 0
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).expect("transpilation failed");

    // Should generate 0f32 or 0.0_f32, not plain 0
    assert!(
        code.contains("0f32") || code.contains("0.0_f32") || code.contains("0.0f32"),
        "Should coerce 0 to f32 when comparing with norm_a (f32 from norm_l2()): {}",
        code
    );
}

/// Test that norm_b > 0 generates norm_b > 0f32
#[test]
fn test_depyler_0927_comparison_f32_coercion_norm_b() {
    let python = r#"
import numpy as np
def check_norm(b):
    norm_b = b.norm_l2()
    return norm_b > 0
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).expect("transpilation failed");

    assert!(
        code.contains("0f32") || code.contains("0.0_f32") || code.contains("0.0f32"),
        "Should coerce 0 to f32 when comparing with norm_b (f32 from norm_l2()): {}",
        code
    );
}

/// Test nested comparison with AND: (norm_a > 0) && (norm_b > 0)
#[test]
fn test_depyler_0927_nested_comparison_and() {
    let python = r#"
import numpy as np
def check_both_norms(a, b):
    norm_a = a.norm_l2()
    norm_b = b.norm_l2()
    return (norm_a > 0) and (norm_b > 0)
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).expect("transpilation failed");

    // Count occurrences of f32 literals - should have 2 (one for each comparison)
    let f32_count = code.matches("0f32").count()
        + code.matches("0.0_f32").count()
        + code.matches("0.0f32").count();

    assert!(
        f32_count >= 2,
        "Should have at least 2 f32 literals for both comparisons: {}",
        code
    );
}

/// Test if-else expression with f32 body and integer else branch
#[test]
fn test_depyler_0927_ifexpr_f32_branch_unification() {
    let python = r#"
import numpy as np
def safe_divide(dot, norm_a, norm_b):
    return dot / (norm_a * norm_b) if (norm_a > 0) and (norm_b > 0) else 0
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).expect("transpilation failed");

    // The else branch should use 0f32 (or 0.0_f32) to match the f32 division result
    // Look for pattern: else { 0f32 } or else { 0.0_f32 } or else { 0.0f32 }
    // NOT: else { 0 }
    let has_else_with_int = code.contains("else { 0 }");

    assert!(
        !has_else_with_int,
        "Else branch should use f32 literal, not integer 0: {}",
        code
    );
}

/// Test cosine similarity pattern (full integration)
/// The comparisons use f32 (for trueno), but the division result is f64,
/// so the else branch should also be f64 for type consistency.
#[test]
fn test_depyler_0927_cosine_similarity_pattern() {
    let python = r#"
import numpy as np
def cosine_sim(a, b):
    dot = a.dot(b)
    norm_a = a.norm_l2()
    norm_b = b.norm_l2()
    result = dot / (norm_a * norm_b) if (norm_a > 0) and (norm_b > 0) else 0
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).expect("transpilation failed");

    // Should have f32 comparisons for both norm checks
    let f32_comparison_count = code.matches("0f32").count()
        + code.matches("0.0_f32").count()
        + code.matches("0.0f32").count();

    // 2 f32 literals for the comparisons (norm_a > 0f32, norm_b > 0f32)
    assert!(
        f32_comparison_count >= 2,
        "Should have f32 literals for comparisons (expected >= 2, got {}): {}",
        f32_comparison_count,
        code
    );

    // The else branch should NOT have integer 0 (should be 0f64 or 0.0)
    let has_else_with_int = code.contains("else { 0 }");
    assert!(
        !has_else_with_int,
        "Else branch should use float literal (f64 to match body), not integer 0: {}",
        code
    );
}

/// Test that dot variable is also recognized as f32 result
#[test]
fn test_depyler_0927_dot_is_f32() {
    let python = r#"
import numpy as np
def check_dot(a, b):
    dot = a.dot(b)
    return dot > 0
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).expect("transpilation failed");

    assert!(
        code.contains("0f32") || code.contains("0.0_f32") || code.contains("0.0f32"),
        "Should coerce 0 to f32 when comparing with dot (f32 from dot()): {}",
        code
    );
}

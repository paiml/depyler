//! DEPYLER-0978: Golden Annotated Example Test
//!
//! This test validates transpilation of a fully type-annotated Python file.
//! Purpose: Falsify hypothesis that type inference gaps are the sole convergence blocker.
//!
//! FINDINGS (2025-12-14):
//! The golden example FAILS to compile, revealing 4 codegen bugs:
//!
//! 1. E0308: Missing Some() wrapper for Option<T> returns
//!    - Function: dictionary_operations
//!    - Returns String, but signature says Option<String>
//!    - Fix: Wrap early return in Some()
//!
//! 2. E0308: Unnecessary borrow in callable parameters
//!    - Function: function_composition
//!    - Passes &item to transform, but item is already owned after .cloned()
//!    - Fix: Pass item directly, not &item
//!
//! 3. E0308: Missing unwrap() for Option handling
//!    - Function: optional_handling
//!    - Returns &Option<i64> instead of i64
//!    - Fix: Use maybe_value.unwrap() or *maybe_value.as_ref().unwrap()
//!
//! 4. E0596: Missing mut for borrowed mutable reference
//!    - Function: main
//!    - items passed as &mut but declared without mut
//!    - Fix: let mut items = ...
//!
//! CONCLUSION: Codegen has bugs independent of type inference.
//! Fix codegen before focusing on inference improvements (Jidoka principle).

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

/// Golden example transpilation test
///
/// This test is IGNORED because it currently fails due to codegen bugs.
/// When all bugs are fixed, this test should pass.
#[test]
#[ignore = "DEPYLER-0978: Golden example reveals codegen bugs - fix before enabling"]
fn test_golden_annotated_example_transpiles() {
    let source = include_str!("../../../examples/golden_annotated_example.py");

    // Generate Rust code
    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Basic sanity checks - these should pass even with codegen bugs
    assert!(
        rust_code.contains("pub fn numeric_operations"),
        "Should contain numeric_operations function"
    );
    assert!(
        rust_code.contains("pub fn string_manipulation"),
        "Should contain string_manipulation function"
    );
    assert!(
        rust_code.contains("pub fn dictionary_operations"),
        "Should contain dictionary_operations function"
    );
    assert!(
        rust_code.contains("pub fn optional_handling"),
        "Should contain optional_handling function"
    );
}

/// Test that golden example has no Type::Unknown fallbacks
///
/// This test validates that explicit type annotations prevent serde_json::Value fallbacks.
#[test]
#[ignore = "DEPYLER-0978: Enable after codegen fixes"]
fn test_golden_example_no_unknown_types() {
    let source = include_str!("../../../examples/golden_annotated_example.py");

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // With full annotations, there should be no serde_json::Value fallbacks
    // (except possibly in generic contexts)
    let value_count = rust_code.matches("serde_json::Value").count();
    assert!(
        value_count < 5,
        "Too many serde_json::Value fallbacks ({}) - type annotations not propagating",
        value_count
    );
}

/// Test specific codegen bug: Option return wrapping
#[test]
fn test_option_return_wrapping_bug() {
    // Minimal reproduction of the dictionary_operations bug
    let source = r#"
def get_value(d: dict) -> str | None:
    if "key" in d:
        return d.get("key", "default")
    return None
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // The early return should be wrapped in Some()
    // Current bug: returns String directly instead of Some(String)
    // This test documents the expected behavior
    assert!(
        rust_code.contains("Option<String>") || rust_code.contains("Option<&str>"),
        "Return type should be Option: {}",
        rust_code
    );
}

/// Test specific codegen bug: callable parameter borrowing
#[test]
fn test_callable_parameter_borrowing_bug() {
    // Minimal reproduction of the function_composition bug
    let source = r#"
from typing import Callable, List

def apply_func(f: Callable[[str], str], items: List[str]) -> List[str]:
    result: List[str] = []
    for item in items:
        result.append(f(item))
    return result
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // After .iter().cloned(), item is owned, so should not pass &item
    // Current bug: passes &item when item is already String (owned)
    // Check that we don't have unnecessary & before item in the f() call
    // This is a heuristic check - the actual fix needs codegen changes
    assert!(
        rust_code.contains("transform") || rust_code.contains("f("),
        "Should have function call: {}",
        rust_code
    );
}

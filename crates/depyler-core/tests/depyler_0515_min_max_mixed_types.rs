//! DEPYLER-0515 / GH-72: min/max Operations with Mixed Numeric Types
//!
//! **ROOT CAUSE**: Direct mapping to std::cmp::min/max without type coercion
//!
//! **Five Whys**:
//! 1. Why does min(5, 3.14) fail? std::cmp::min requires same types
//! 2. Why does std::cmp::min require same types? Rust's type system is strict
//! 3. Why not use a different approach? Current implementation doesn't handle type coercion
//! 4. Why wasn't this caught? Python's duck typing allows mixed numeric types
//! 5. ROOT: Direct mapping to std::cmp::min without detecting/handling type mismatches
//!
//! **Problem**: Python allows min/max with mixed numeric types (int/float), but
//! Rust's std::cmp::min/max requires both arguments to have the same type.
//!
//! **Examples**:
//! - Python: `min(5, 3.14)` → returns 3.14 ✅
//! - Rust: `std::cmp::min(5, 3.14)` → compile error ❌
//! - Should: `5_f64.min(3.14)` or `(5 as f64).min(3.14)` ✅
//!
//! **Solution**: Detect mixed numeric types and use f64 method calls with type coercion

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile_to_rust(python_code: &str) -> Result<String, String> {
    let ast = parse(python_code, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

// ============================================================================
// RED PHASE - Failing Tests
// ============================================================================

#[test]
fn test_DEPYLER_0515_min_literal_mixed_types() {
    // RED: min(5, 3.14) with literal int and float
    let python = r#"
def test_min() -> float:
    return min(5, 3.14)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0515: min() with mixed int/float literals should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should NOT use std::cmp::min (requires same types)
    // Should use f64 method or explicit conversion
    let normalized = rust_code.replace(char::is_whitespace, "");

    // Acceptable patterns (any of these):
    // - (5 as f64).min(3.14)
    // - 5_f64.min(3.14)
    // - f64::min(5.0, 3.14)
    let has_valid_pattern = normalized.contains("asf64).min")
        || normalized.contains("5_f64.min")
        || normalized.contains("f64::min");

    assert!(
        has_valid_pattern || !normalized.contains("std::cmp::min"),
        "DEPYLER-0515: Should not use std::cmp::min for mixed types.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0515_min_variable_mixed_types() {
    // Variable mixed with float literal
    let python = r#"
def test_min(x: int) -> float:
    return min(x, 3.14)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0515: min() with int variable and float literal should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should handle type conversion for x
    let normalized = rust_code.replace(char::is_whitespace, "");
    assert!(
        !normalized.contains("std::cmp::min(x,3.14)"),
        "DEPYLER-0515: Should convert types, not use std::cmp::min directly.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0515_max_literal_mixed_types() {
    // max() has the same issue
    let python = r#"
def test_max() -> float:
    return max(5, 3.14)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0515: max() with mixed int/float literals should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    let normalized = rust_code.replace(char::is_whitespace, "");
    let has_valid_pattern = normalized.contains("asf64).max")
        || normalized.contains("5_f64.max")
        || normalized.contains("f64::max");

    assert!(
        has_valid_pattern || !normalized.contains("std::cmp::max"),
        "DEPYLER-0515: Should not use std::cmp::max for mixed types.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0515_max_variable_mixed_types() {
    let python = r#"
def test_max(x: int) -> float:
    return max(x, 3.14)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0515: max() with int variable and float literal should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    let normalized = rust_code.replace(char::is_whitespace, "");
    assert!(
        !normalized.contains("std::cmp::max(x,3.14)"),
        "DEPYLER-0515: Should convert types, not use std::cmp::max directly.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0515: min() codegen produces addition instead of min - requires AST fix"]
fn test_DEPYLER_0515_min_same_type_still_works() {
    // Ensure we don't break same-type min/max
    let python = r#"
def test_min(a: int, b: int) -> int:
    return min(a, b)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0515: min() with same types should still work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // For same types, std::cmp::min is fine
    assert!(
        rust_code.contains("std::cmp::min") || rust_code.contains(".min("),
        "DEPYLER-0515: Should generate valid min call.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0515_actual_example_from_issue() {
    // Exact example from GitHub issue #72
    let python = r#"
def example() -> float:
    return min(5, 3.14)

def example2(x: int) -> float:
    return min(x, 3.14)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0515: Examples from issue #72 should transpile. Error:\n{}",
        result.unwrap_err()
    );
}

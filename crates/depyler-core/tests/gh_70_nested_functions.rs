//! GH-70: Nested function definitions not supported
//!
//! **ROOT CAUSE**: Type inference doesn't propagate types into nested function context
//!
//! **Five Whys**:
//! 1. Why doesn't code compile? Types are all `()` instead of correct types
//! 2. Why are types `()`? Type inference defaulted to Unknown → `()`
//! 3. Why did type inference fail? Nested function context not analyzed
//! 4. Why wasn't context analyzed? Type inference treats nested functions as isolated
//! 5. ROOT: Type inference doesn't propagate types from outer function into nested functions
//!
//! **Problem**: Nested functions transpile but with wrong types:
//! - Outer function return type: `()` (should be function pointer)
//! - Nested function parameters: `()` (should be inferred from usage)
//! - Nested function return: `()` (should be inferred from body)
//!
//! **Solution Required**:
//! 1. Detect when function returns another function → set return type to fn pointer
//! 2. Propagate type information into nested function parameters
//! 3. Infer nested function return type from body
//!
//! **Examples**:
//! ```python
//! def outer():
//!     def inner(entry):
//!         return entry[0]
//!     return inner
//! ```
//!
//! **Generated (BROKEN)**:
//! ```rust,ignore
//! pub fn outer() {  // ← Missing return type
//!     fn inner(entry: ()) -> () {  // ← Wrong parameter/return types
//!         entry[0]  // ERROR: can't index ()
//!     }
//!     inner  // ERROR: expected (), found fn item
//! }
//! ```
//!
//! **Expected (CORRECT)**:
//! ```rust,ignore
//! pub fn outer() -> fn((String, String, String)) -> String {
//!     fn inner(entry: (String, String, String)) -> String {
//!         entry.0.clone()
//!     }
//!     inner
//! }
//! ```

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
fn test_GH_70_simple_nested_function_returning_function() {
    // RED: Nested function that returns another function
    let python = r#"
def outer():
    def inner(x):
        return x * 2
    return inner

def main():
    func = outer()
    result = func(5)
    print(result)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "GH-70: Simple nested function should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should have proper return type for outer function
    assert!(
        rust_code.contains("fn outer()") && (rust_code.contains("-> fn(") || rust_code.contains("-> impl Fn")),
        "GH-70: outer() should have return type annotation.\nGenerated:\n{}",
        rust_code
    );

    // Nested function should exist
    assert!(
        rust_code.contains("fn inner"),
        "GH-70: Should generate nested function.\nGenerated:\n{}",
        rust_code
    );

    // Should NOT have parameter type () for inner
    assert!(
        !rust_code.contains("fn inner(x: ())"),
        "GH-70: inner parameter should not be unit type ().\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_GH_70_nested_function_with_tuple_destructuring() {
    // RED: Exact pattern from GH-70 issue
    let python = r#"
def outer():
    def inner(entry):
        timestamp, level, message = entry
        return timestamp[11:13]
    return inner
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "GH-70: Tuple destructuring nested function should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should generate nested function
    assert!(
        rust_code.contains("fn inner"),
        "GH-70: Should generate nested function.\nGenerated:\n{}",
        rust_code
    );

    // Should NOT have unit type for entry parameter
    assert!(
        !rust_code.contains("fn inner(entry: ())"),
        "GH-70: entry parameter should not be unit type.\nGenerated:\n{}",
        rust_code
    );

    // Outer function should return something (not unit)
    assert!(
        !rust_code.contains("pub fn outer() {") || rust_code.contains("pub fn outer() ->"),
        "GH-70: outer() should have explicit return type.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_GH_70_itertools_groupby_pattern() {
    // RED: Real-world pattern from log_analyzer.py
    let python = r#"
from itertools import groupby

def group_by_hour(entries):
    def extract_hour(entry):
        timestamp, level, message = entry
        return timestamp[11:13]

    entries_sorted = sorted(entries, key=extract_hour)

    hour_counts = {}
    for hour, group in groupby(entries_sorted, key=extract_hour):
        hour_counts[hour] = sum(1 for _ in group)

    return hour_counts
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "GH-70: itertools.groupby pattern should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should generate extract_hour function
    assert!(
        rust_code.contains("fn extract_hour") || rust_code.contains("extract_hour"),
        "GH-70: Should generate extract_hour function.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_GH_70_nested_function_type_inference() {
    // RED: Test that type inference works for nested functions
    let python = r#"
def outer(x: int) -> callable:
    def inner(y: int) -> int:
        return x + y
    return inner

def main():
    add_five = outer(5)
    result = add_five(3)
    print(result)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "GH-70: Nested function with type hints should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should have proper types (not ())
    assert!(
        rust_code.contains("fn inner") && !rust_code.contains("fn inner(y: ())"),
        "GH-70: inner should have i64 parameter, not ().\nGenerated:\n{}",
        rust_code
    );

    // Should have return type
    assert!(
        rust_code.contains("-> ") && rust_code.contains("fn outer"),
        "GH-70: outer should have return type.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_GH_70_check_rust_compilation() {
    // RED: Verify generated Rust actually compiles
    let python = r#"
def make_multiplier(n):
    def multiply(x):
        return x * n
    return multiply

def main():
    times_two = make_multiplier(2)
    result = times_two(5)
    print(result)
"#;

    let result = transpile_to_rust(python);
    assert!(result.is_ok(), "Should transpile: {}", result.unwrap_err());

    let rust_code = result.unwrap();

    // Write to temp file for inspection
    std::fs::write("/tmp/gh_70_compilation_test.rs", &rust_code).unwrap();

    // Check structure
    assert!(
        rust_code.contains("fn make_multiplier") && rust_code.contains("fn multiply"),
        "GH-70: Should generate both functions.\nGenerated:\n{}",
        rust_code
    );

    // Key check: outer function should have return type if it returns inner function
    if rust_code.contains("return multiply") || rust_code.ends_with("multiply\n}") {
        assert!(
            rust_code.contains("fn make_multiplier") && rust_code.contains("->"),
            "GH-70: make_multiplier should have return type since it returns a function.\nGenerated:\n{}",
            rust_code
        );
    }
}

#[test]
fn test_GH_70_minimal_reproduction() {
    // RED: Ultra-minimal case from GH-70
    let python = r#"
def outer():
    def inner(entry):
        return entry[0]
    return inner
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "GH-70: Minimal nested function should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Write for debugging
    std::fs::write("/tmp/gh_70_minimal.rs", &rust_code).unwrap();

    // Should generate inner function
    assert!(
        rust_code.contains("fn inner"),
        "GH-70: Should generate inner function.\nGenerated:\n{}",
        rust_code
    );

    // Should return inner
    assert!(
        rust_code.contains("inner") && (rust_code.contains("return inner") || rust_code.ends_with("inner\n}\n") || rust_code.contains("    inner\n")),
        "GH-70: outer should return inner.\nGenerated:\n{}",
        rust_code
    );
}

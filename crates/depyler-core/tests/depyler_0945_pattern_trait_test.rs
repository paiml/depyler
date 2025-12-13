//! DEPYLER-0945: Pattern Trait Borrowing Tests
//!
//! Tests for ensuring Pattern trait methods (starts_with, ends_with, find, replace, split)
//! produce compilable code. When params are already &str (from str type hints), no extra
//! borrowing is needed. When params are owned String, we borrow them.

use depyler_core::DepylerPipeline;
use std::process::Command;

/// Helper to check if generated Rust code compiles
fn compiles_with_rustc(code: &str) -> bool {
    let temp_file = "/tmp/depyler_0945_test.rs";
    std::fs::write(temp_file, code).unwrap();

    let output = Command::new("rustc")
        .args(["--edition", "2021", temp_file, "--crate-type", "lib", "-o", "/tmp/depyler_0945_test"])
        .output()
        .expect("Failed to run rustc");

    output.status.success()
}

/// Get compilation errors
fn compile_errors(code: &str) -> String {
    let temp_file = "/tmp/depyler_0945_test.rs";
    std::fs::write(temp_file, code).unwrap();

    let output = Command::new("rustc")
        .args(["--edition", "2021", temp_file, "--crate-type", "lib", "-o", "/tmp/depyler_0945_test"])
        .output()
        .expect("Failed to run rustc");

    String::from_utf8_lossy(&output.stderr).to_string()
}

/// Test startswith with variable pattern - CRITICAL FOR E0277 FIX
#[test]
fn test_depyler_0945_startswith_variable_pattern() {
    let python = r#"
def check_prefix(text: str, pattern: str) -> bool:
    return text.startswith(pattern)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Should NOT have double-borrowing (&&) which would fail compilation
    assert!(
        !code.contains("starts_with(&&"),
        "Should NOT double-borrow: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0277
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile (E0277 fix). Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test endswith with variable suffix
#[test]
fn test_depyler_0945_endswith_variable_suffix() {
    let python = r#"
def check_suffix(text: str, suffix: str) -> bool:
    return text.endswith(suffix)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Should NOT have double-borrowing
    assert!(
        !code.contains("ends_with(&&"),
        "Should NOT double-borrow: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0277
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile (E0277 fix). Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test find with variable substring
#[test]
fn test_depyler_0945_find_variable_substring() {
    let python = r#"
def find_pos(text: str, needle: str) -> int:
    return text.find(needle)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Should NOT have double-borrowing
    assert!(
        !code.contains("find(&&"),
        "Should NOT double-borrow: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0277
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile (E0277 fix). Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test replace with variable arguments
#[test]
fn test_depyler_0945_replace_variable_args() {
    let python = r#"
def replace_text(text: str, old: str, new: str) -> str:
    return text.replace(old, new)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Should NOT have double-borrowing
    assert!(
        !code.contains("replace(&&"),
        "Should NOT double-borrow: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0277
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile (E0277 fix). Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test split with variable delimiter
#[test]
fn test_depyler_0945_split_variable_delimiter() {
    let python = r#"
from typing import List

def split_text(text: str, delimiter: str) -> List[str]:
    return text.split(delimiter)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // Should NOT have double-borrowing
    assert!(
        !code.contains("split(&&"),
        "Should NOT double-borrow: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0277
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile (E0277 fix). Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test string literal patterns still work (no borrowing needed)
#[test]
fn test_depyler_0945_string_literal_patterns() {
    let python = r#"
def check_hello(text: str) -> bool:
    return text.startswith("Hello")

def split_comma(text: str) -> list:
    return text.split(",")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // String literals should be used directly (no & needed)
    // They should appear as "Hello" not &"Hello"
    assert!(
        code.contains(r#"starts_with("Hello")"#) || code.contains("starts_with(\"Hello\")"),
        "String literal should be used directly: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test that OWNED String variables DO get borrowed (not &str params)
#[test]
fn test_depyler_0945_owned_string_gets_borrowed() {
    let python = r#"
def check_dynamic_prefix(text: str) -> bool:
    pattern = "Hello"
    return text.startswith(pattern)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();

    // CRITICAL: Generated code must compile
    if !compiles_with_rustc(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

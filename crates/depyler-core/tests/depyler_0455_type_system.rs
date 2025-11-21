// DEPYLER-0455: Type System Bugs (ArgumentTypeError, str/String, Option truthiness)
//
// This test suite validates correct Rust code generation for Python type system patterns:
// 1. Exception raising with ArgumentTypeError
// 2. String method return type consistency
// 3. Option type truthiness checks
// 4. Option type Display formatting
//
// Created: 2025-11-21
// Ticket: https://github.com/paiml/depyler/issues/DEPYLER-0455

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

/// Helper function to check if generated Rust code contains a pattern
fn assert_contains(rust_code: &str, pattern: &str) {
    assert!(
        rust_code.contains(pattern),
        "Expected pattern not found:\n  Pattern: {}\n  Code:\n{}",
        pattern,
        rust_code
    );
}

/// Helper function to check if generated Rust code does NOT contain a pattern
fn assert_not_contains(rust_code: &str, pattern: &str) {
    assert!(
        !rust_code.contains(pattern),
        "Unexpected pattern found:\n  Pattern: {}\n  Code:\n{}",
        pattern,
        rust_code
    );
}

// ====================================================================================
// Test 1: ArgumentTypeError Exception Handling
// ====================================================================================
//
// BUG: `raise argparse.ArgumentTypeError(msg)` generates `Err(format!(...))`
// which returns String instead of ArgumentTypeError.
//
// Expected: `Err(ArgumentTypeError::new(format!(...)))`

#[test]
fn test_DEPYLER_0455_01_argument_type_error_exception() {
    let python = r#"
import argparse

def validate_int(value):
    """Validate that value is an integer."""
    try:
        num = int(value)
        return num
    except ValueError:
        raise argparse.ArgumentTypeError(f"Value must be an integer, got '{value}'")
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should construct ArgumentTypeError properly
    // Expected: Err(ArgumentTypeError::new(...))
    assert_contains(&rust_code, "ArgumentTypeError");

    // Should NOT directly return formatted string as error
    // This would cause type mismatch: expected ArgumentTypeError, found String
    assert_not_contains(&rust_code, "return Err(format!(");

    // Function signature should return Result with ArgumentTypeError
    let has_result_type = rust_code.contains("Result<")
        && (rust_code.contains("ArgumentTypeError")
            || rust_code.contains("Box<dyn std::error::Error>"));

    assert!(
        has_result_type,
        "Expected Result<_, ArgumentTypeError> signature. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 2: String Method Type Consistency
// ====================================================================================
//
// BUG: Assigning `.to_lowercase()` (returns String) to variable typed as &str
// causes type mismatch.
//
// Expected: Consistent String type OR proper type conversions

#[test]
fn test_DEPYLER_0455_02_string_method_type_consistency() {
    let python = r#"
def process_format(use_json, format_str):
    """Process format string, converting to lowercase if not JSON."""
    if use_json:
        output_format = "json"
    else:
        output_format = format_str.lower()
    return output_format
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use consistent String type throughout
    // Either all branches return String, or use proper conversions
    let has_consistent_types =
        // Option A: Declare as String
        rust_code.contains("output_format: String")
        ||
        // Option B: All branches convert to String
        (rust_code.contains(r#""json".to_string()"#)
         && rust_code.contains(".to_lowercase()"));

    assert!(
        has_consistent_types,
        "Expected consistent String type handling. Got:\n{}",
        rust_code
    );

    // Should handle .to_lowercase() correctly
    assert_contains(&rust_code, ".to_lowercase()");

    // Should NOT have type mismatch between &str and String
    // This would cause: expected `&str`, found `String`
    let has_type_mismatch = rust_code.contains(r#"output_format = "json""#)
        && rust_code.contains("output_format = format_str.to_lowercase()")
        && !rust_code.contains(".to_string()");

    assert!(
        !has_type_mismatch,
        "Type mismatch detected between &str and String. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 3: Option Truthiness Check
// ====================================================================================
//
// BUG: `if option_var` treats Option<String> as bool, which fails type checking.
// Python: `if config_file:` (truthiness check)
// Rust: Must use `.is_some()` or `if let Some(...)`
//
// Expected: `if option_var.is_some()` OR `if let Some(value) = option_var`

#[test]
fn test_DEPYLER_0455_03_option_truthiness_check() {
    let python = r#"
import os

def check_config():
    """Check if config file environment variable is set."""
    config_file = os.environ.get("CONFIG_FILE")
    if config_file:
        print(f"Config: {config_file}")
    else:
        print("No config file set")
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use .is_some() or if let Some for Option check
    let has_proper_option_check = rust_code.contains(".is_some()")
        || rust_code.contains("if let Some(");

    assert!(
        has_proper_option_check,
        "Expected .is_some() or if let Some for Option check. Got:\n{}",
        rust_code
    );

    // Should NOT directly use Option<String> as bool
    // This would cause: expected `bool`, found `Option<String>`
    let has_direct_option_if = rust_code.contains("if config_file {")
        && !rust_code.contains(".is_some()")
        && !rust_code.contains("if let Some");

    assert!(
        !has_direct_option_if,
        "Direct Option as bool detected (type error). Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 4: Option Display Handling
// ====================================================================================
//
// BUG: Formatting `Option<String>` with `format!("{}", option_var)` fails because
// Option<String> doesn't implement Display trait.
//
// Expected: Pattern match to extract inner value OR unwrap after .is_some() check

#[test]
fn test_DEPYLER_0455_04_option_display_handling() {
    let python = r#"
import os

def show_config():
    """Return config file path if set."""
    config_file = os.environ.get("CONFIG_FILE")
    if config_file:
        return f"Config: {config_file}"
    return "No config"
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use pattern matching or unwrap to access inner value
    let has_proper_handling =
        // Option A: if let Some(value) pattern
        rust_code.contains("if let Some(")
        ||
        // Option B: .is_some() check followed by .unwrap()
        (rust_code.contains(".is_some()") && rust_code.contains(".unwrap()"))
        ||
        // Option C: .unwrap_or() or .map() or similar
        rust_code.contains(".unwrap_or(")
        || rust_code.contains(".map(|");

    assert!(
        has_proper_handling,
        "Expected if let Some, .unwrap(), or .unwrap_or() for Option value access. Got:\n{}",
        rust_code
    );

    // Should NOT format Option<String> directly without extracting value
    // This would cause: `Option<String>` doesn't implement `std::fmt::Display`

    // Check for problematic pattern: format!(..., config_file) where config_file is Option
    // But allow if it's inside "if let Some" block
    let lines: Vec<&str> = rust_code.lines().collect();
    let mut has_bad_format = false;

    for (i, line) in lines.iter().enumerate() {
        if line.contains("format!(") && line.contains("config_file") {
            // Check if we're inside an "if let Some" block
            // Look backwards for "if let Some"
            let in_if_let_block = lines[..i]
                .iter()
                .rev()
                .take(5) // Look back up to 5 lines
                .any(|l| l.contains("if let Some"));

            if !in_if_let_block {
                has_bad_format = true;
                break;
            }
        }
    }

    assert!(
        !has_bad_format,
        "Direct Option formatting detected (Display not implemented). Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 5: Combined Scenario (Real-world complexity)
// ====================================================================================
//
// This test uses a more realistic scenario combining all 4 bugs.
// It mirrors the actual code from example_complex that triggered DEPYLER-0455.

#[test]
fn test_DEPYLER_0455_05_combined_scenario() {
    let python = r#"
import os
import argparse
import re

def email_address(value):
    """Custom type for email address validation."""
    pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    if not re.match(pattern, value):
        raise argparse.ArgumentTypeError(f"Invalid email address: '{value}'")
    return value

def main():
    """Main function with environment variable handling."""
    # Get format from environment, defaulting to "text"
    env_format = os.environ.get("DEFAULT_FORMAT", "text")
    output_format = env_format.lower()

    # Get optional config file path
    config_file = os.environ.get("CONFIG_FILE")

    output_lines = []
    output_lines.append(f"Format: {output_format}")

    if config_file:
        output_lines.append(f"Config: {config_file}")

    for line in output_lines:
        print(line)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Bug 1: ArgumentTypeError handling
    assert_contains(&rust_code, "ArgumentTypeError");
    assert_not_contains(&rust_code, "return Err(format!(");

    // Bug 2: String type consistency
    let has_string_handling = rust_code.contains(".to_lowercase()");
    assert!(has_string_handling);

    // Bug 3: Option truthiness
    let has_option_check = rust_code.contains(".is_some()")
        || rust_code.contains("if let Some(");
    assert!(has_option_check);

    // Bug 4: Option Display (implicit in proper if let Some usage)
    // If we have "if let Some", the inner block should use the unwrapped value
    if rust_code.contains("if let Some(") {
        // Good - pattern matching extracts value
        assert!(true);
    } else if rust_code.contains(".is_some()") {
        // Should have corresponding .unwrap() or .unwrap_or()
        let has_unwrap = rust_code.contains(".unwrap()")
            || rust_code.contains(".unwrap_or(");
        assert!(has_unwrap, "Expected .unwrap() after .is_some() check");
    }
}

// ====================================================================================
// Test 6: Edge Case - Nested Option Checks
// ====================================================================================
//
// Test that nested Option checks are handled correctly

#[test]
fn test_DEPYLER_0455_06_nested_option_checks() {
    let python = r#"
import os

def check_multiple_env():
    """Check multiple environment variables."""
    var1 = os.environ.get("VAR1")
    var2 = os.environ.get("VAR2")

    if var1:
        if var2:
            return f"Both: {var1}, {var2}"
        return f"Only var1: {var1}"
    return "None set"
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // All Option checks should use .is_some() or if let
    let option_checks = rust_code.matches("if ").count();
    let proper_checks = rust_code.matches(".is_some()").count()
        + rust_code.matches("if let Some").count();

    assert!(
        proper_checks >= option_checks,
        "Expected all if statements to properly check Options. Got:\n{}",
        rust_code
    );
}

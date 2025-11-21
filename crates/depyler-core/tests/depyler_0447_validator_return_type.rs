//! DEPYLER-0447: Argparse Validator Return Type Inference
//!
//! Tests for correct return type inference when argparse validators:
//! 1. Return the original parameter (identity validators)
//! 2. Return converted types (int, float)
//! 3. Return string method results (upper, lower, strip)
//!
//! Bug: email_address validator incorrectly generated as:
//!   pub fn email_address(value: serde_json::Value) -> Result<i32, ArgumentTypeError>
//!
//! Expected:
//!   pub fn email_address(value: &str) -> Result<String, Box<dyn std::error::Error>>

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

#[test]
fn test_depyler_0447_identity_validator_returns_string() {
    let py = r#"
import argparse
import re

def email_address(value):
    """Validator that returns the original string parameter."""
    if not re.match(r"^[a-z]+@[a-z]+\.com$", value):
        raise argparse.ArgumentTypeError("Invalid email")
    return value

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--email", type=email_address)
    args = parser.parse_args()
"#;

    let rust_code = transpile_python(py).unwrap();

    // Check 1: Parameter should be &str (not serde_json::Value)
    assert!(
        rust_code.contains("pub fn email_address(value: &str)"),
        "email_address should have `value: &str` parameter, got:\n{}",
        extract_function_signature(&rust_code, "email_address")
    );

    // Check 2: Return type should be Result<String, ...> (not Result<i32, ...>)
    assert!(
        rust_code.contains("-> Result<String, Box<dyn std::error::Error>>")
            || rust_code.contains("-> Result<std::string::String, Box<dyn std::error::Error>>"),
        "email_address should return Result<String, Box<dyn Error>>, got:\n{}",
        extract_function_signature(&rust_code, "email_address")
    );

    // Check 3: Return statement should convert &str to String
    assert!(
        rust_code.contains("Ok(value.to_string())")
            || rust_code.contains("Ok(value.into())"),
        "email_address should convert &str to String in return, got:\n{}",
        extract_function_body(&rust_code, "email_address")
    );

    // Check 4: Error should be wrapped in Box::new(ArgumentTypeError::new(...))
    assert!(
        rust_code.contains("Box::new(ArgumentTypeError::new("),
        "Errors should be wrapped in Box::new(ArgumentTypeError::new(...)), got:\n{}",
        extract_function_body(&rust_code, "email_address")
    );
}

#[test]
fn test_depyler_0447_converting_validator_returns_converted_type() {
    let py = r#"
import argparse

def port_number(value):
    """Validator that converts string to int."""
    port = int(value)
    if port < 1:
        raise argparse.ArgumentTypeError("Port must be >= 1")
    return port

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=port_number)
"#;

    let rust_code = transpile_python(py).unwrap();

    // Check 1: Parameter should be &str
    assert!(
        rust_code.contains("pub fn port_number(value: &str)"),
        "port_number should have `value: &str` parameter, got:\n{}",
        extract_function_signature(&rust_code, "port_number")
    );

    // Check 2: Return type should be Result<i32, ...> (converts to int)
    assert!(
        rust_code.contains("-> Result<i32, Box<dyn std::error::Error>>"),
        "port_number should return Result<i32, Box<dyn Error>>, got:\n{}",
        extract_function_signature(&rust_code, "port_number")
    );

    // Check 3: Should use .parse::<i32>() for conversion
    assert!(
        rust_code.contains(".parse::<i32>()"),
        "port_number should use .parse::<i32>() for string to int conversion, got:\n{}",
        extract_function_body(&rust_code, "port_number")
    );
}

#[test]
fn test_depyler_0447_string_method_validator_returns_string() {
    let py = r#"
import argparse

def uppercase_string(value):
    """Validator that transforms string using string method."""
    return value.upper()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=uppercase_string)
"#;

    let rust_code = transpile_python(py).unwrap();

    // Check 1: Parameter should be &str
    assert!(
        rust_code.contains("pub fn uppercase_string(value: &str)"),
        "uppercase_string should have `value: &str` parameter, got:\n{}",
        extract_function_signature(&rust_code, "uppercase_string")
    );

    // Check 2: Return type should be Result<String, ...>
    assert!(
        rust_code.contains("-> Result<String, Box<dyn std::error::Error>>")
            || rust_code.contains("-> Result<std::string::String, Box<dyn std::error::Error>>"),
        "uppercase_string should return Result<String, Box<dyn Error>>, got:\n{}",
        extract_function_signature(&rust_code, "uppercase_string")
    );

    // Check 3: Should use .to_uppercase() for string transformation
    assert!(
        rust_code.contains("Ok(value.to_uppercase())"),
        "uppercase_string should use .to_uppercase(), got:\n{}",
        extract_function_body(&rust_code, "uppercase_string")
    );
}

#[test]
fn test_depyler_0447_lowercase_validator_returns_string() {
    let py = r#"
import argparse

def lowercase_validator(value):
    """Validator that returns lowercase string."""
    return value.lower()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--text", type=lowercase_validator)
"#;

    let rust_code = transpile_python(py).unwrap();

    assert!(
        rust_code.contains("pub fn lowercase_validator(value: &str)"),
        "lowercase_validator should have `value: &str` parameter"
    );

    assert!(
        rust_code.contains("-> Result<String,") || rust_code.contains("-> Result<std::string::String,"),
        "lowercase_validator should return Result<String, ...>"
    );

    assert!(
        rust_code.contains("Ok(value.to_lowercase())"),
        "lowercase_validator should use .to_lowercase()"
    );
}

#[test]
fn test_depyler_0447_strip_validator_returns_string() {
    let py = r#"
import argparse

def strip_whitespace(value):
    """Validator that strips whitespace."""
    return value.strip()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=strip_whitespace)
"#;

    let rust_code = transpile_python(py).unwrap();

    assert!(
        rust_code.contains("pub fn strip_whitespace(value: &str)"),
        "strip_whitespace should have `value: &str` parameter"
    );

    assert!(
        rust_code.contains("-> Result<String,") || rust_code.contains("-> Result<std::string::String,"),
        "strip_whitespace should return Result<String, ...>"
    );

    assert!(
        rust_code.contains("Ok(value.trim().to_string())"),
        "strip_whitespace should use .trim().to_string()"
    );
}

#[test]
fn test_depyler_0447_float_validator_returns_float() {
    let py = r#"
import argparse

def percentage_validator(value):
    """Validator that converts to float and validates range."""
    pct = float(value)
    if pct < 0.0 or pct > 100.0:
        raise argparse.ArgumentTypeError("Percentage must be 0-100")
    return pct

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--percent", type=percentage_validator)
"#;

    let rust_code = transpile_python(py).unwrap();

    assert!(
        rust_code.contains("pub fn percentage_validator(value: &str)"),
        "percentage_validator should have `value: &str` parameter"
    );

    assert!(
        rust_code.contains("-> Result<f64, Box<dyn std::error::Error>>"),
        "percentage_validator should return Result<f64, Box<dyn Error>>"
    );

    assert!(
        rust_code.contains(".parse::<f64>()"),
        "percentage_validator should use .parse::<f64>()"
    );
}

// Helper functions for test assertions

fn extract_function_signature(rust_code: &str, func_name: &str) -> String {
    let pattern = format!("pub fn {}", func_name);
    rust_code
        .lines()
        .skip_while(|line| !line.contains(&pattern))
        .take_while(|line| !line.contains('{'))
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_function_body(rust_code: &str, func_name: &str) -> String {
    let pattern = format!("pub fn {}", func_name);
    let lines: Vec<&str> = rust_code.lines().collect();
    let start_idx = lines.iter().position(|line| line.contains(&pattern));

    if let Some(start) = start_idx {
        lines[start..std::cmp::min(start + 20, lines.len())]
            .join("\n")
    } else {
        String::from("Function not found")
    }
}

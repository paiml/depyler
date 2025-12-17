// DEPYLER-0436: argparse validator parameter type inference
//
// Tests that parameters in argparse custom validators are correctly
// inferred as &str (not serde_json::Value).
//
// Root cause: Type inference doesn't detect argparse validator pattern
// Solution: When function is used as argparse type= validator, first param should be &str
//
// Parent: DEPYLER-0428 (ArgumentTypeError support)

#![allow(non_snake_case)] // Test naming convention

use depyler_core::DepylerPipeline;
use std::sync::atomic::{AtomicU64, Ordering};

// DEPYLER-1028: Use unique temp files to prevent race conditions in parallel tests
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_path() -> (String, String) {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let rs_file = format!("/tmp/depyler_0436_{}_{}.rs", pid, id);
    let rlib_file = format!("/tmp/depyler_0436_{}_{}.rlib", pid, id);
    (rs_file, rlib_file)
}

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

#[test]
fn test_DEPYLER_0436_parameter_type_should_be_str() {
    // Minimal argparse validator
    let python = r#"
def port_validator(value):
    """Argparse custom type validator."""
    port = int(value)
    if port < 1 or port > 65535:
        raise ValueError(f"Port must be 1-65535")
    return port
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // CRITICAL: Parameter should be &str, NOT serde_json::Value
    assert!(
        rust.contains("value: &str"),
        "Parameter 'value' should be &str for argparse validators. Got: {}",
        rust
    );

    // Should NOT use serde_json::Value
    assert!(
        !rust.contains("value: serde_json::Value"),
        "Should not use serde_json::Value for string parameters: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_0436_int_call_should_parse_not_cast() {
    // int() on string parameter should use .parse(), not cast
    let python = r#"
def validator(value):
    num = int(value)
    return num
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Should use .parse() for string to int conversion
    assert!(
        rust.contains(".parse") || rust.contains("parse::<i32>"),
        "int(string_value) should use .parse(), not cast. Got: {}",
        rust
    );

    // Should NOT use cast syntax (value) as i32
    assert!(
        !rust.contains("(value) as i32"),
        "Should not use cast for string parsing: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_0436_parse_error_handling() {
    // DEPYLER-0436: Even in try/except blocks, parameter type should be inferred as &str
    let python = r#"
def validator(value):
    try:
        num = int(value)
        return num
    except ValueError:
        raise ValueError("Invalid number")
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // DEPYLER-0436: Parameter should be &str, NOT serde_json::Value
    assert!(
        rust.contains("value: &str"),
        "Parameter 'value' should be &str even in try/except. Got: {}",
        rust
    );

    // Should NOT use serde_json::Value
    assert!(
        !rust.contains("value: serde_json::Value") && !rust.contains("value: &serde_json::Value"),
        "Should not use serde_json::Value for string parameters: {}",
        rust
    );

    // NOTE: Result return type is DEPYLER-0437's responsibility (try/except control flow)
}

#[test]
fn test_DEPYLER_0436_full_validator_compiles() {
    // Complete argparse validator should compile successfully
    let python = r#"
def port_number(value):
    """Port validator for argparse."""
    try:
        port = int(value)
        if port < 1 or port > 65535:
            raise ValueError(f"Port must be 1-65535, got {port}")
        return port
    except ValueError:
        raise ValueError(f"Invalid port: {value}")
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Write to temp file and try to compile
    let (temp_file, temp_rlib) = unique_temp_path();
    std::fs::write(&temp_file, &rust).unwrap();

    let compile_result = std::process::Command::new("rustc")
        .args([
            "--crate-type",
            "lib",
            "--edition",
            "2021",
            &temp_file,
            "-o",
            &temp_rlib,
        ])
        .output();

    // Cleanup
    let _ = std::fs::remove_file(&temp_file);
    let _ = std::fs::remove_file(&temp_rlib);

    assert!(
        compile_result.is_ok(),
        "rustc should run successfully: {:?}",
        compile_result.err()
    );

    let output = compile_result.unwrap();

    // Check for specific errors we're trying to fix
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !stderr.contains("E0605"),
        "Should not have cast errors (E0605 - non-primitive cast). Stderr: {}",
        stderr
    );

    assert!(
        !stderr.contains("cannot cast"),
        "Should not have cast errors. Stderr: {}",
        stderr
    );

    assert!(
        output.status.success(),
        "Compilation should succeed. Stderr: {}",
        stderr
    );
}

//! DEPYLER-0957: Custom Exception Field Type Inference Test
//!
//! Root Cause: Exception classes with unannotated __init__ parameters
//! get Type::Unknown → serde_json::Value instead of String.
//!
//! Fix: For Exception subclasses, default Unknown field types to String.

use std::process::Command;

/// Test that custom exceptions get String typed fields (not serde_json::Value)
/// SLOW: Requires full cargo compilation - use `cargo test -- --ignored` to run
#[test]
#[ignore = "slow: requires cargo build (200+ seconds)"]
fn test_depyler_0957_custom_exception_string_field() {
    let python_source = r#"
class MyError(Exception):
    def __init__(self, message):
        self.message = message

def test():
    raise MyError("test error")
"#;

    // Transpile
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "depyler",
            "--",
            "transpile",
            "--code",
            python_source,
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        // Clear LLVM coverage env to prevent interference under cargo-llvm-cov
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_SHOW_ENV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("LLVM_PROFILE_FILE")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_INCREMENTAL")
        .env_remove("CARGO_BUILD_JOBS")
        .env_remove("CARGO_TARGET_DIR")
        .output()
        .expect("Failed to run depyler");

    let rust_code = String::from_utf8_lossy(&output.stdout);

    // CRITICAL: Field should be String, NOT serde_json::Value
    assert!(
        rust_code.contains("message: String") || rust_code.contains("pub message: String"),
        "Exception field 'message' should be String, not serde_json::Value.\nGenerated:\n{}",
        rust_code
    );

    // CRITICAL: new() should take String, NOT serde_json::Value
    assert!(
        rust_code.contains("message: String") ||
        rust_code.contains("message: impl Into<String>") ||
        rust_code.contains("fn new(message: String)"),
        "Exception constructor should take String.\nGenerated:\n{}",
        rust_code
    );

    // Should NOT contain serde_json::Value for message field
    assert!(
        !rust_code.contains("message: serde_json::Value"),
        "Exception field 'message' should NOT be serde_json::Value.\nGenerated:\n{}",
        rust_code
    );
}

/// Test that custom exception with explicit str annotation works
/// SLOW: Requires full cargo compilation - use `cargo test -- --ignored` to run
#[test]
#[ignore = "slow: requires cargo build (200+ seconds)"]
fn test_depyler_0957_exception_explicit_str_annotation() {
    let python_source = r#"
class ValidationError(Exception):
    def __init__(self, message: str):
        self.message = message
"#;

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "depyler",
            "--",
            "transpile",
            "--code",
            python_source,
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        // Clear LLVM coverage env to prevent interference under cargo-llvm-cov
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_SHOW_ENV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("LLVM_PROFILE_FILE")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_INCREMENTAL")
        .env_remove("CARGO_BUILD_JOBS")
        .env_remove("CARGO_TARGET_DIR")
        .output()
        .expect("Failed to run depyler");

    let rust_code = String::from_utf8_lossy(&output.stdout);

    // Explicit annotation should definitely produce String
    assert!(
        rust_code.contains("message: String") || rust_code.contains("pub message: String"),
        "Exception with str annotation should have String field.\nGenerated:\n{}",
        rust_code
    );
}

/// Test that exception with multiple fields infers String for message-like names
/// SLOW: Requires full cargo compilation - use `cargo test -- --ignored` to run
#[test]
#[ignore = "slow: requires cargo build (200+ seconds)"]
fn test_depyler_0957_exception_multiple_fields() {
    let python_source = r#"
class APIError(Exception):
    def __init__(self, code, message, details):
        self.code = code
        self.message = message
        self.details = details
"#;

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "depyler",
            "--",
            "transpile",
            "--code",
            python_source,
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/..")
        // Clear LLVM coverage env to prevent interference under cargo-llvm-cov
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_SHOW_ENV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("LLVM_PROFILE_FILE")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_INCREMENTAL")
        .env_remove("CARGO_BUILD_JOBS")
        .env_remove("CARGO_TARGET_DIR")
        .output()
        .expect("Failed to run depyler");

    let rust_code = String::from_utf8_lossy(&output.stdout);

    // message and details should be String, code could be i64 or String
    // At minimum, message should NOT be serde_json::Value
    assert!(
        !rust_code.contains("message: serde_json::Value"),
        "Exception 'message' field should NOT be serde_json::Value.\nGenerated:\n{}",
        rust_code
    );
}

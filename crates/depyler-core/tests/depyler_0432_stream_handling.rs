// DEPYLER-0432: sys.stdin/stdout Stream Handling and File I/O
//
// This test suite validates the transpilation of Python I/O operations
// to Rust, including:
// - sys.stdin/stdout/stderr usage
// - File operations (open, read, write)
// - Type inference for file paths and boolean flags
// - Argument-less argparse subcommands
//
// Created: 2025-11-19
// Ticket: https://github.com/paiml/depyler/issues/DEPYLER-0432

#![allow(non_snake_case)] // Test naming convention

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
// Test 1: sys.stdin iteration
// ====================================================================================

#[test]
fn test_DEPYLER_0432_01_sys_stdin_iteration() {
    let python = r#"
import sys

def process_stdin():
    for line in sys.stdin:
        print(line.rstrip())
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use std::io::stdin().lines()
    assert_contains(&rust_code, "std::io::stdin()");
    assert_contains(&rust_code, ".lines()");

    // Should NOT use sys.stdin directly
    assert_not_contains(&rust_code, "sys.stdin");
}

// ====================================================================================
// Test 2: sys.stderr print
// ====================================================================================

#[test]
fn test_DEPYLER_0432_02_sys_stderr_print() {
    let python = r#"
import sys

def log_error(msg):
    print(f"Error: {msg}", file=sys.stderr)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use eprintln! for stderr (idiomatic Rust)
    // OR writeln!(std::io::stderr(), ...)
    let has_eprintln = rust_code.contains("eprintln!");
    let has_stderr_write = rust_code.contains("std::io::stderr()");

    assert!(
        has_eprintln || has_stderr_write,
        "Expected eprintln! or std::io::stderr(), got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 3: File read (text mode)
// ====================================================================================

#[test]
fn test_DEPYLER_0432_03_file_read_text() {
    let python = r#"
def read_file(filepath):
    with open(filepath) as f:
        content = f.read()
    return content
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use File::open
    assert_contains(&rust_code, "std::fs::File::open");

    // Should use read_to_string for text mode
    assert_contains(&rust_code, "read_to_string");

    // Function should return Result<>
    assert_contains(&rust_code, "Result<");
}

// ====================================================================================
// Test 4: File read (binary mode)
// ====================================================================================

#[test]
#[ignore] // DEPYLER-0432: Binary mode requires mode parameter detection (separate feature)
fn test_DEPYLER_0432_04_file_read_binary() {
    let python = r#"
def read_binary(filepath):
    with open(filepath, "rb") as f:
        data = f.read()
    return data
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use File::open
    assert_contains(&rust_code, "std::fs::File::open");

    // Should use read_to_end for binary mode
    let has_read_to_end = rust_code.contains("read_to_end");
    let has_vec_u8 = rust_code.contains("Vec<u8>");

    assert!(
        has_read_to_end || has_vec_u8,
        "Expected read_to_end or Vec<u8>, got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 5: File iteration (line-by-line)
// ====================================================================================

#[test]
fn test_DEPYLER_0432_05_file_iteration() {
    let python = r#"
def count_lines(filepath):
    count = 0
    with open(filepath) as f:
        for line in f:
            count += 1
    return count
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use BufReader for line iteration
    let has_bufreader = rust_code.contains("BufReader");
    let has_lines_method = rust_code.contains(".lines()");

    assert!(
        has_bufreader || has_lines_method,
        "Expected BufReader or .lines() method, got:\n{}",
        rust_code
    );

    // Should NOT use .iter() on File directly
    if rust_code.contains("File::open") {
        assert_not_contains(&rust_code, ".iter()");
    }
}

// ====================================================================================
// Test 6: Argparse string parameter type inference (file path)
// ====================================================================================

#[test]
fn test_DEPYLER_0432_06_argparse_string_param_inference() {
    let python = r#"
import argparse

def process_file(filepath):
    with open(filepath) as f:
        return f.read()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", help="File to process")
    args = parser.parse_args()
    result = process_file(args.file)
    print(result)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Parameter should be inferred as &str (not serde_json::Value)
    // Check function signature
    let has_str_param =
        rust_code.contains("filepath: &str") || rust_code.contains("filepath: String");

    // Should NOT use serde_json::Value for file paths
    let has_value_param = rust_code.contains("filepath: serde_json::Value");

    assert!(
        has_str_param && !has_value_param,
        "Expected filepath: &str or String, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 7: Argparse boolean parameter type inference
// ====================================================================================

#[test]
fn test_DEPYLER_0432_07_argparse_bool_param_inference() {
    let python = r#"
import argparse

def process(verbose=False):
    if verbose:
        print("Verbose mode enabled")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    process(args.verbose)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Boolean parameter should be inferred as bool (not &serde_json::Value)
    let has_bool_param = rust_code.contains("verbose: bool");

    // Should NOT use serde_json::Value for boolean flags
    let has_value_param = rust_code.contains("verbose: &serde_json::Value")
        || rust_code.contains("verbose: serde_json::Value");

    assert!(
        has_bool_param && !has_value_param,
        "Expected verbose: bool, not Value. Got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Test 8: Argument-less argparse subcommand
// ====================================================================================

#[test]
fn test_DEPYLER_0432_08_argumentless_subcommand() {
    let python = r#"
import argparse
import sys

def read_stdin():
    for line in sys.stdin:
        print(line.rstrip())

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")

    # Argument-less subcommand
    subparsers.add_parser("stdin", help="Read from stdin")

    # Subcommand with arguments
    read_parser = subparsers.add_parser("read", help="Read file")
    read_parser.add_argument("file")

    args = parser.parse_args()

    if args.command == "stdin":
        read_stdin()
    elif args.command == "read":
        print(f"Reading {args.file}")
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should generate enum variant for stdin subcommand
    // Pattern: Stdin, (unit variant) OR Stdin { } (empty struct)
    let has_stdin_variant = rust_code.contains("Stdin,") || rust_code.contains("Stdin {");

    assert!(
        has_stdin_variant,
        "Expected Stdin enum variant, got:\n{}",
        rust_code
    );

    // Should also generate Read variant with file field
    assert_contains(&rust_code, "Read");
    assert_contains(&rust_code, "file");
}

// ====================================================================================
// Test 9: Hex encoding (.hex() method on bytes)
// ====================================================================================

#[test]
#[ignore] // DEPYLER-0432: Hex encoding requires separate implementation (binascii/hex module)
fn test_DEPYLER_0432_09_hex_encoding() {
    let python = r#"
def show_hex(data):
    hex_str = data[:10].hex()
    print(hex_str)
    return hex_str
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use hex::encode() OR manual hex formatting
    let has_hex_encode = rust_code.contains("hex::encode");
    let has_hex_format =
        rust_code.contains("format!(\"{:02x}\"") || rust_code.contains("format!(\"{:x}\"");

    assert!(
        has_hex_encode || has_hex_format,
        "Expected hex::encode() or hex formatting, got:\n{}",
        rust_code
    );

    // Should NOT use .hex() method directly (doesn't exist in Rust)
    assert_not_contains(&rust_code, ".hex()");
}

// ====================================================================================
// Test 10: tempfile.NamedTemporaryFile translation
// ====================================================================================

#[test]
fn test_DEPYLER_0432_10_tempfile_translation() {
    let python = r#"
import tempfile

def create_temp():
    with tempfile.NamedTemporaryFile(mode="w", delete=False) as f:
        temp_path = f.name
        f.write("test content")
    return temp_path
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use tempfile crate
    let has_tempfile = rust_code.contains("tempfile::");
    let has_namedtempfile = rust_code.contains("NamedTempFile");

    assert!(
        has_tempfile || has_namedtempfile,
        "Expected tempfile crate usage, got:\n{}",
        rust_code
    );
}

// ====================================================================================
// Integration Test: Full stream_processor.py example
// ====================================================================================

#[test]
#[ignore] // Run separately due to large file
fn test_DEPYLER_0432_11_stream_processor_integration() {
    // Read the actual stream_processor.py from reprorusted-python-cli
    let python_file =
        "/home/user/reprorusted-python-cli/examples/example_io_streams/stream_processor.py";

    let python_code =
        std::fs::read_to_string(python_file).expect("Failed to read stream_processor.py");

    let result = transpile_python(&python_code);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Write to temp file for manual inspection
    std::fs::write("/tmp/stream_processor_test.rs", &rust_code)
        .expect("Failed to write test output");

    // Verify key patterns are present
    assert_contains(&rust_code, "std::io::stdin()");
    assert_contains(&rust_code, "std::fs::File::open");

    // Should NOT have serde_json::Value for all parameters
    // (At least some should be properly typed)
    let value_count = rust_code.matches("serde_json::Value").count();
    let total_params_estimate = 15; // Rough estimate of parameters in stream_processor

    assert!(
        value_count < total_params_estimate,
        "Too many serde_json::Value types ({}/{}). Type inference not working!",
        value_count,
        total_params_estimate
    );

    // Try to compile (this will fail initially, that's expected for RED phase)
    println!("\n=== Attempting compilation (expected to fail in RED phase) ===");

    let compile_result = std::process::Command::new("rustc")
        .args([
            "--crate-type",
            "bin",
            "--edition",
            "2021",
            "/tmp/stream_processor_test.rs",
            "-o",
            "/tmp/stream_processor_test",
        ])
        .output();

    if let Ok(output) = compile_result {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error_count = stderr.matches("error[E").count();

        println!("Compilation errors: {}", error_count);
        println!(
            "First 50 lines of errors:\n{}",
            stderr.lines().take(50).collect::<Vec<_>>().join("\n")
        );

        // In RED phase, we expect errors. Track progress:
        // Initial: 32 errors
        // After fixes: 0 errors (target)
        assert!(
            error_count > 0,
            "RED phase: Expected compilation errors, but got none! \
             Test may be passing prematurely."
        );
    }
}

// ====================================================================================
// Test 12: sys.stdin.readlines() (DEPYLER-0638)
// ====================================================================================

#[test]
fn test_DEPYLER_0638_01_stdin_readlines() {
    let python = r#"
import sys

def read_all_lines():
    lines = sys.stdin.readlines()
    return lines
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should use std::io::stdin().lock().lines().collect()
    assert_contains(&rust_code, "std::io::stdin()");
    assert_contains(&rust_code, ".lock()");
    assert_contains(&rust_code, ".lines()");
    assert_contains(&rust_code, ".collect");

    // Should NOT use sys.stdin directly
    assert_not_contains(&rust_code, "sys.stdin");
    // Should NOT use readlines method (should be transpiled to .lines().collect())
    assert_not_contains(&rust_code, "readlines");
}

#[test]
fn test_DEPYLER_0638_02_stdin_readlines_process() {
    let python = r#"
import sys

def process_log_lines():
    lines = sys.stdin.readlines()
    for line in lines:
        if line.startswith("ERROR"):
            print(line.strip())
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should properly handle readlines
    assert_contains(&rust_code, "std::io::stdin()");
    assert_contains(&rust_code, ".lines()");
    assert_contains(&rust_code, ".collect");
}

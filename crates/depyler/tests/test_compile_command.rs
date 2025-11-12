//! DEPYLER-0380: Compile Command Tests - GREEN Phase
//!
//! **EXTREME TDD Protocol**
//!
//! Tests verify the `depyler compile` command:
//! - Transpiles Python → Rust
//! - Creates Cargo project structure
//! - Builds executable binary
//! - Binary runs and produces correct output
//!
//! Target Coverage: ≥85%
//! TDG Score: A (≤2.0)
//! Complexity: ≤10 per function

use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to create a temp directory with a Python file
fn setup_python_file(filename: &str, content: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let python_file = temp_dir.path().join(filename);
    fs::write(&python_file, content).expect("Failed to write Python file");
    (temp_dir, python_file)
}

#[test]
fn test_depyler_0380_compile_command_exists() {
    // Test Case: `depyler compile --help` should work
    // Expected: Help text mentioning compile subcommand

    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args(["compile", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("compile"));
}

#[test]
fn test_depyler_0380_compile_hello_world() {
    // Test Case: Compile simple hello world Python script
    // Expected: Binary created and outputs "Hello, World!"
    // Bug: Not implemented yet (RED phase)

    let python_code = r#"
def main():
    print("Hello, World!")

if __name__ == '__main__':
    main()
"#;

    let (_temp_dir, python_file) = setup_python_file("hello.py", python_code);

    // Run depyler compile
    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args(["compile", python_file.to_str().unwrap()])
        .assert()
        .success();

    // Check that binary was created
    let binary_name = if cfg!(windows) { "hello.exe" } else { "hello" };
    let binary_path = python_file.parent().unwrap().join(binary_name);
    assert!(
        binary_path.exists(),
        "Binary should be created at {}",
        binary_path.display()
    );

    // Run the binary and check output
    Command::new(&binary_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn test_depyler_0380_compile_with_args() {
    // Test Case: Compile script that takes command-line arguments
    // Expected: Binary accepts args and processes them correctly

    let python_code = r#"
import sys

def main():
    if len(sys.argv) > 1:
        print(f"Hello, {sys.argv[1]}!")
    else:
        print("Hello, World!")

if __name__ == '__main__':
    main()
"#;

    let (_temp_dir, python_file) = setup_python_file("greet.py", python_code);

    // Compile
    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args(["compile", python_file.to_str().unwrap()])
        .assert()
        .success();

    // Run with argument
    let binary_name = if cfg!(windows) { "greet.exe" } else { "greet" };
    let binary_path = python_file.parent().unwrap().join(binary_name);

    Command::new(&binary_path)
        .arg("Alice")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Alice!"));
}

#[test]
fn test_depyler_0380_compile_with_output_flag() {
    // Test Case: Specify custom output path with -o flag
    // Expected: Binary created at specified location

    let python_code = r#"
def main():
    print("Custom output!")

if __name__ == '__main__':
    main()
"#;

    let (_temp_dir, python_file) = setup_python_file("custom.py", python_code);
    let output_path = python_file.parent().unwrap().join("my_binary");

    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args([
            "compile",
            python_file.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let binary_path = if cfg!(windows) {
        output_path.with_extension("exe")
    } else {
        output_path
    };

    assert!(binary_path.exists(), "Binary should exist at custom path");
}

#[test]
fn test_depyler_0380_compile_with_profile_release() {
    // Test Case: Compile with --profile release flag
    // Expected: Binary optimized for release

    let python_code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def main():
    result = fibonacci(10)
    print(f"fib(10) = {result}")

if __name__ == '__main__':
    main()
"#;

    let (_temp_dir, python_file) = setup_python_file("fib.py", python_code);

    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args([
            "compile",
            python_file.to_str().unwrap(),
            "--profile",
            "release",
        ])
        .assert()
        .success();

    let binary_name = if cfg!(windows) { "fib.exe" } else { "fib" };
    let binary_path = python_file.parent().unwrap().join(binary_name);

    Command::new(&binary_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("fib(10) = 55"));
}

#[test]
fn test_depyler_0380_compile_missing_file_error() {
    // Test Case: Try to compile non-existent file
    // Expected: Clear error message

    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args(["compile", "/nonexistent/file.py"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

#[test]
fn test_depyler_0380_compile_invalid_python_error() {
    // Test Case: Try to compile syntactically invalid Python
    // Expected: Clear parse error

    let invalid_python = r#"
def main(
    print("Missing closing paren")
"#;

    let (_temp_dir, python_file) = setup_python_file("invalid.py", invalid_python);

    Command::cargo_bin("depyler")
        .expect("Failed to find depyler binary")
        .args(["compile", python_file.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("parse").or(predicate::str::contains("syntax")));
}

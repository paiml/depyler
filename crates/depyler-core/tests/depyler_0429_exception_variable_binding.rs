//! DEPYLER-0429: Exception Variable Binding Tests
//!
//! Tests for `except Exception as e:` variable binding in transpiled Rust code.
//!
//! ## Bug Description
//! When Python code uses `except Exception as e:`, the transpiler recognizes the
//! exception variable `e` but does NOT bind it in the generated Rust code. This causes
//! E0425 "cannot find value `e` in this scope" errors.
//!
//! ## Test Strategy
//! 1. Simple exception binding (ValueError)
//! 2. Exception with attribute access (e.returncode)
//! 3. Exception without variable (no binding)
//! 4. Integration test (full task_runner.py)

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

#[test]
fn test_DEPYLER_0429_01_simple_exception_binding() {
    let python_code = r#"
def parse_int(value):
    try:
        x = int(value)
        return x
    except ValueError as e:
        print(f"Parse error: {e}")
        return -1
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile");

    println!("Generated Rust code:\n{}", rust_code);

    // MUST contain match expression with bound variable
    assert!(
        rust_code.contains("Err(e)") || rust_code.contains("Err(ref e)"),
        "Expected exception variable `e` to be bound in Err branch, but found: {}",
        rust_code
    );

    // MUST NOT have unbound variable references
    assert!(
        !rust_code.contains("e.") || rust_code.contains("Err(e)"),
        "Exception variable `e` used but not bound!"
    );

    // Compile check
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile without errors"
    );
}

#[test]
fn test_DEPYLER_0429_02_exception_with_attribute_access() {
    let python_code = r#"
import subprocess
import sys

def run_command(cmd):
    try:
        result = subprocess.run(cmd, check=True)
        return result.returncode
    except subprocess.CalledProcessError as e:
        print(f"Command failed with exit code {e.returncode}", file=sys.stderr)
        sys.exit(e.returncode)
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile");

    println!("Generated Rust code:\n{}", rust_code);

    // MUST bind exception variable
    assert!(
        rust_code.contains("Err(e)") || rust_code.contains("Err(ref e)"),
        "Expected exception variable `e` bound in Err branch"
    );

    // MUST be able to access exception attributes
    // Note: e.returncode must be accessible within Err branch
    assert!(
        rust_code.contains("e.returncode") || rust_code.contains("e).returncode"),
        "Expected access to exception attribute `e.returncode`"
    );

    // Compile check (may fail due to subprocess not implemented, but no E0425 errors)
    let compilation_errors = get_compilation_errors(&rust_code);
    assert!(
        !compilation_errors.iter().any(|err| err.contains("E0425") && err.contains("cannot find value `e`")),
        "Must NOT have E0425 'cannot find value `e`' errors. Found: {:?}",
        compilation_errors
    );
}

#[test]
fn test_DEPYLER_0429_03_exception_without_variable() {
    let python_code = r#"
def check_file(path):
    try:
        f = open(path)
        return True
    except FileNotFoundError:
        print("File not found")
        return False
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile");

    println!("Generated Rust code:\n{}", rust_code);

    // MUST use wildcard pattern (no variable)
    assert!(
        rust_code.contains("Err(_)"),
        "Expected wildcard pattern `Err(_)` for exception without variable"
    );

    // MUST NOT bind unused variable
    assert!(
        !rust_code.contains("Err(e)"),
        "Should not bind exception variable when Python doesn't use one"
    );

    // Compile check
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile without errors"
    );
}

#[test]
fn test_DEPYLER_0429_04_multiple_exceptions_with_variables() {
    let python_code = r#"
def process_value(value):
    try:
        x = int(value)
        result = 100 / x
        return result
    except ValueError as e1:
        print(f"Invalid value: {e1}")
        return -1
    except ZeroDivisionError as e2:
        print(f"Division error: {e2}")
        return -2
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile");

    println!("Generated Rust code:\n{}", rust_code);

    // MUST bind BOTH exception variables (e1 and e2)
    // Note: May be transformed to match arms or nested if-let
    let has_e1_binding = rust_code.contains("e1") &&
        (rust_code.contains("Err(e1)") || rust_code.contains("let e1"));

    let has_e2_binding = rust_code.contains("e2") &&
        (rust_code.contains("Err(e2)") || rust_code.contains("let e2"));

    assert!(
        has_e1_binding,
        "Expected exception variable `e1` to be bound for ValueError handler"
    );

    assert!(
        has_e2_binding,
        "Expected exception variable `e2` to be bound for ZeroDivisionError handler"
    );

    // Compile check
    let compilation_errors = get_compilation_errors(&rust_code);
    assert!(
        !compilation_errors.iter().any(|err| err.contains("E0425")),
        "Must NOT have E0425 errors for exception variables. Found: {:?}",
        compilation_errors
    );
}

#[test]
#[ignore = "Integration test - run separately"]
fn test_DEPYLER_0429_05_task_runner_full_integration() {
    // Full task_runner.py transpilation
    use std::fs;
    use std::path::PathBuf;

    let task_runner_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("reprorusted-python-cli/examples/example_subprocess/task_runner.py");

    if !task_runner_path.exists() {
        println!("Skipping: task_runner.py not found at {:?}", task_runner_path);
        return;
    }

    let python_code = fs::read_to_string(&task_runner_path)
        .expect("Failed to read task_runner.py");

    let rust_code = transpile_python(&python_code)
        .expect("Failed to transpile task_runner.py");

    println!("Generated Rust code (task_runner.rs):\n{}", rust_code);

    // Check exception variable binding
    assert!(
        rust_code.contains("Err(e)") || rust_code.contains("Err(ref e)"),
        "task_runner.py uses `except CalledProcessError as e:` - must bind variable"
    );

    // Check subprocess.run() transpilation
    assert!(
        rust_code.contains("std::process::Command"),
        "subprocess.run() must transpile to std::process::Command"
    );

    // Compile check - MUST have zero errors
    let compilation_errors = get_compilation_errors(&rust_code);

    // Filter out dependency errors (clap, subprocess not in scope, etc.)
    let binding_errors: Vec<_> = compilation_errors
        .iter()
        .filter(|err| err.contains("E0425") && err.contains("cannot find value `e`"))
        .collect();

    assert!(
        binding_errors.is_empty(),
        "task_runner.py MUST compile without exception binding errors. Found: {:?}",
        binding_errors
    );
}

// Helper functions

fn compile_rust_code(rust_code: &str) -> bool {
    use std::process::Command;
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(rust_code.as_bytes()).expect("Failed to write");

    let output = Command::new("rustc")
        .arg("--crate-type").arg("lib")
        .arg("--edition").arg("2021")
        .arg(temp_file.path())
        .output()
        .expect("Failed to run rustc");

    output.status.success()
}

fn get_compilation_errors(rust_code: &str) -> Vec<String> {
    use std::process::Command;
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(rust_code.as_bytes()).expect("Failed to write");

    let output = Command::new("rustc")
        .arg("--crate-type").arg("lib")
        .arg("--edition").arg("2021")
        .arg(temp_file.path())
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    stderr
        .lines()
        .filter(|line| line.contains("error[E"))
        .map(|s| s.to_string())
        .collect()
}

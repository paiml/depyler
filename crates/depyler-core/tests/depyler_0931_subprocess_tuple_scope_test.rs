//! DEPYLER-0931: Subprocess Tuple Scope Tests
//!
//! Tests for fixing E0425 "cannot find value in scope" errors when
//! subprocess.run results are unpacked in try/except blocks.
//! Variables from tuple unpacking must be hoisted with proper initialization.

use depyler_core::DepylerPipeline;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

// DEPYLER-1028: Use unique temp files to prevent race conditions in parallel tests
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_path() -> String {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    format!("/tmp/depyler_0931_{}_{}.rs", pid, id)
}

/// Helper to check if generated Rust code compiles
fn compiles(code: &str) -> bool {
    // Write to temp file with unique name
    let temp_file = unique_temp_path();
    std::fs::write(&temp_file, code).unwrap();

    // Find serde_json rlib for proper extern linking
    let serde_json_rlib = find_serde_json_rlib();

    // Build rustc args
    let mut args = vec![
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        "-L".to_string(),
        "dependency=target/debug/deps".to_string(),
        "-L".to_string(),
        "dependency=../../target/debug/deps".to_string(),
    ];

    // Add serde_json extern if found
    if let Some(rlib) = serde_json_rlib {
        args.push("--extern".to_string());
        args.push(format!("serde_json={}", rlib));
    }

    args.push(temp_file.clone());

    let output = Command::new("rustc")
        .args(&args)
        .output()
        .expect("Failed to run rustc");

    // Cleanup temp file
    let _ = std::fs::remove_file(&temp_file);

    output.status.success()
}

/// Find serde_json rlib in target directories
fn find_serde_json_rlib() -> Option<String> {
    // Check common locations for serde_json rlib
    for base in [
        "target/debug/deps",
        "../../target/debug/deps",
        "target/release/deps",
    ] {
        if let Ok(entries) = std::fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("libserde_json") && name.ends_with(".rlib") {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    None
}

/// Helper to get compilation errors
fn compile_errors(code: &str) -> String {
    let temp_file = unique_temp_path();
    std::fs::write(&temp_file, code).unwrap();

    // Find serde_json rlib for proper extern linking
    let serde_json_rlib = find_serde_json_rlib();

    // Build rustc args
    let mut args = vec![
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        "-L".to_string(),
        "dependency=target/debug/deps".to_string(),
        "-L".to_string(),
        "dependency=../../target/debug/deps".to_string(),
    ];

    // Add serde_json extern if found
    if let Some(rlib) = serde_json_rlib {
        args.push("--extern".to_string());
        args.push(format!("serde_json={}", rlib));
    }

    args.push(temp_file.clone());

    let output = Command::new("rustc")
        .args(&args)
        .output()
        .expect("Failed to run rustc");

    // Cleanup temp file
    let _ = std::fs::remove_file(&temp_file);

    String::from_utf8_lossy(&output.stderr).to_string()
}

/// Test basic tuple unpacking from subprocess in try/except
/// SLOW: Requires rustc compilation validation
#[test]
#[ignore = "slow: requires rustc compilation"]
fn test_depyler_0931_subprocess_tuple_in_try() {
    let python = r#"
import subprocess

def run_command(cmd: list) -> str:
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        stdout = result.stdout
        stderr = result.stderr
        return stdout
    except Exception as e:
        return ""
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Should properly scope stdout/stderr variables
    assert!(
        code.contains("fn run_command"),
        "Should generate run_command function: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0425 errors
    if !compiles(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test tuple unpacking with multiple variables
/// SLOW: Requires rustc compilation validation
#[test]
#[ignore = "slow: requires rustc compilation"]
fn test_depyler_0931_multi_variable_unpacking() {
    let python = r#"
def process_output() -> tuple:
    try:
        a, b, c = (1, 2, 3)
        return (a, b, c)
    except:
        return (0, 0, 0)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Variables should be accessible in both branches
    assert!(
        code.contains("fn process_output"),
        "Should generate function: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test variable hoisting for try/except with assignment
#[test]
fn test_depyler_0931_variable_hoisting_in_try() {
    let python = r#"
def safe_read(path: str) -> str:
    try:
        data = open(path).read()
        return data
    except:
        return ""
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // 'data' should be accessible in both try and except context
    assert!(
        code.contains("fn safe_read"),
        "Should generate safe_read function: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test subprocess.Popen result handling
#[test]
fn test_depyler_0931_subprocess_popen() {
    let python = r#"
import subprocess

def start_process(cmd: str) -> int:
    try:
        proc = subprocess.Popen(cmd, shell=True)
        returncode = proc.wait()
        return returncode
    except Exception:
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Should handle subprocess.Popen
    assert!(
        code.contains("fn start_process"),
        "Should generate start_process function: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test variable used in except block must be hoisted
#[test]
fn test_depyler_0931_variable_used_in_except() {
    let python = r#"
def process_with_fallback(x: int) -> int:
    try:
        result = x * 2
        return result
    except:
        # result should be accessible here as Option<i64>
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    assert!(
        code.contains("fn process_with_fallback"),
        "Should generate function: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test nested try/except with variable scoping
#[test]
fn test_depyler_0931_nested_try_except() {
    let python = r#"
def nested_handler(x: int) -> int:
    try:
        outer = x + 1
        try:
            inner = outer * 2
            return inner
        except:
            return outer
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Both outer and inner should be properly scoped
    assert!(
        code.contains("fn nested_handler"),
        "Should generate function: {}",
        code
    );

    // CRITICAL: Generated code must compile
    if !compiles(&code) {
        let errors = compile_errors(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

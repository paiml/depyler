//! RED Phase Tests for DEPYLER-0497: Format! Macro Display Trait for Option/Result/Vec
//!
//! Problem: Option<T>, Result<T, E>, and Vec<T> don't implement Display trait.
//! When used in format! macros with {} placeholders, compilation fails.
//!
//! Solution: Either use {:?} debug formatting OR unwrap/transform the values.
//!
//! This test MUST FAIL initially (Red phase), then pass after fix (Green phase).

use depyler_core::DepylerPipeline;
use std::process::Command;

/// Helper function to compile generated Rust code and check for errors
fn compile_rust_code(rust_code: &str, test_name: &str) -> Result<(), String> {

    // Write to temporary file with unique name per test
    let temp_file = format!("/tmp/depyler_0497_{}.rs", test_name);
    std::fs::write(&temp_file, rust_code).map_err(|e| format!("Write failed: {}", e))?;

    // Try to compile
    let output = Command::new("rustc")
        .args(&["--crate-type", "lib", "--deny", "warnings", &temp_file])
        .output()
        .map_err(|e| format!("Rustc execution failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Compilation failed:\n{}", stderr))
    }
}

#[test]
fn test_depyler_0497_option_in_fstring_variable() {
    // RED Phase: This test will FAIL because Option<i32> doesn't implement Display
    let python = r#"
def find_value(x: int):
    if x > 5:
        return x * 2
    return None

def main():
    result = find_value(10)
    print(f"Found: {result}")

    not_found = find_value(3)
    print(f"Not found: {not_found}")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Debug: Print generated code
    println!("Generated Rust code:\n{}", rust_code);

    // The generated code should compile without errors
    match compile_rust_code(&rust_code, "option_variable") {
        Ok(_) => {
            // GREEN: Test passes when format! correctly handles Option types
            println!("✅ DEPYLER-0497: Format! macro correctly handles Option types");
        }
        Err(e) => {
            // RED: Test fails with E0277: Option<T> doesn't implement Display
            if e.contains("E0277") && e.contains("std::fmt::Display") {
                panic!(
                    "❌ RED PHASE: Option<T> Display trait error (DEPYLER-0497)\n\
                     Expected: format! should use {{{{:?}}}} or unwrap Option values\n\
                     Actual: Compilation error\n\n\
                     Error:\n{}",
                    e
                );
            } else {
                panic!("Unexpected compilation error:\n{}", e);
            }
        }
    }
}

#[test]
fn test_depyler_0497_option_from_function_call() {
    // RED Phase: Direct function call returning Option in f-string
    let python = r#"
def get_optional() -> int:
    return None

def main():
    print(f"Value: {get_optional()}")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    match compile_rust_code(&rust_code, "option_function") {
        Ok(_) => {
            println!("✅ DEPYLER-0497: Format! correctly handles Option-returning functions");
        }
        Err(e) => {
            if e.contains("E0277") && e.contains("std::fmt::Display") {
                panic!(
                    "❌ RED PHASE: Option<T> Display trait error (DEPYLER-0497)\n\
                     Function call returns Option, needs {{:?}} or unwrapping\n\n\
                     Error:\n{}",
                    e
                );
            } else {
                panic!("Unexpected compilation error:\n{}", e);
            }
        }
    }
}

#[test]
fn test_depyler_0497_result_in_fstring() {
    // Test Result<T, E> in format! macro
    let python = r#"
def safe_divide(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Division by zero")
    return a // b

def main():
    result = safe_divide(10, 2)
    print(f"Result: {result}")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    match compile_rust_code(&rust_code, "result") {
        Ok(_) => {
            println!("✅ DEPYLER-0497: Format! correctly handles Result types");
        }
        Err(e) => {
            if e.contains("E0277") && e.contains("std::fmt::Display") {
                panic!(
                    "❌ RED PHASE: Result<T, E> Display trait error (DEPYLER-0497)\n\
                     Result types need {{:?}} or unwrapping with ?\n\n\
                     Error:\n{}",
                    e
                );
            } else {
                panic!("Unexpected compilation error:\n{}", e);
            }
        }
    }
}

#[test]
fn test_depyler_0497_vec_in_fstring() {
    // Test Vec<T> in format! macro
    let python = r#"
def get_numbers():
    return [1, 2, 3, 4, 5]

def main():
    numbers = get_numbers()
    print(f"Numbers: {numbers}")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();
    println!("Generated Rust code:\n{}", rust_code);

    match compile_rust_code(&rust_code, "vec") {
        Ok(_) => {
            println!("✅ DEPYLER-0497: Format! correctly handles Vec types");
        }
        Err(e) => {
            if e.contains("E0277") && e.contains("std::fmt::Display") {
                panic!(
                    "❌ RED PHASE: Vec<T> Display trait error (DEPYLER-0497)\n\
                     Vec needs {{{{:?}}}} debug formatting\n\n\
                     Error:\n{}",
                    e
                );
            } else {
                panic!("Unexpected compilation error:\n{}", e);
            }
        }
    }
}

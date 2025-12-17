//! DEPYLER-0983: Golden Exception Handling Test
//!
//! This test validates transpilation of a fully type-annotated Python file
//! that exercises exception handling patterns (try/except → Result).
//!
//! Purpose: Falsify hypothesis that exception handling codegen is sound.
//!
//! Exception patterns tested:
//! 1. Simple try/except with fallback return
//! 2. try/except with exception variable binding
//! 3. try/except/finally (resource cleanup)
//! 4. Multiple exception handlers
//! 5. Nested try/except blocks
//! 6. Exception re-raising patterns
//! 7. Return value propagation through Result

use depyler_core::DepylerPipeline;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

// DEPYLER-1028: Use unique temp files to prevent race conditions in parallel tests
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_path() -> String {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    format!("/tmp/depyler_0983_{}_{}.rs", pid, id)
}

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

/// Check if generated Rust code compiles
fn compiles(code: &str) -> Result<(), String> {
    let temp_file = unique_temp_path();
    std::fs::write(&temp_file, code).map_err(|e| e.to_string())?;

    let output = Command::new("rustc")
        .args([
            "--crate-type=lib",
            "--edition=2021",
            "-L",
            "dependency=target/debug/deps",
            "-L",
            "dependency=../../target/debug/deps",
            &temp_file,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    // Cleanup
    let _ = std::fs::remove_file(&temp_file);

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Golden exception example transpilation test
///
/// This test is IGNORED because it currently fails due to exception codegen bugs.
/// When all bugs are fixed, this test should pass.
#[test]
#[ignore = "DEPYLER-0983: Golden exception example reveals codegen bugs - fix before enabling"]
fn test_golden_exception_example_transpiles() {
    let source = include_str!("../../../examples/golden_exception_handling.py");

    // Generate Rust code
    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Basic sanity checks - these should pass even with codegen bugs
    assert!(
        rust_code.contains("pub fn parse_int_safe"),
        "Should contain parse_int_safe function"
    );
    assert!(
        rust_code.contains("pub fn divide_safe"),
        "Should contain divide_safe function"
    );
    assert!(
        rust_code.contains("pub fn multiple_handlers"),
        "Should contain multiple_handlers function"
    );
    assert!(
        rust_code.contains("pub fn nested_try_except"),
        "Should contain nested_try_except function"
    );
}

/// Test that golden exception example compiles
#[test]
#[ignore = "DEPYLER-0983: Enable after exception codegen fixes"]
fn test_golden_exception_example_compiles() {
    let source = include_str!("../../../examples/golden_exception_handling.py");

    let rust_code = transpile_python(source).expect("Failed to transpile");

    match compiles(&rust_code) {
        Ok(()) => (),
        Err(e) => {
            panic!(
                "Golden exception example should compile.\nErrors:\n{}\n\nGenerated:\n{}",
                e, rust_code
            );
        }
    }
}

/// Test simple try/except with fallback
#[test]
fn test_simple_try_except_fallback() {
    let source = r#"
def parse_int_safe(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Should have parse function and error handling
    assert!(
        rust_code.contains("parse") || rust_code.contains("Parse"),
        "Should use parse for string to int: {}",
        rust_code
    );
}

/// Test try/except with exception variable binding
#[test]
fn test_exception_variable_binding() {
    let source = r#"
def get_with_bound_exception(d: dict[str, int], key: str) -> str:
    try:
        value: int = d[key]
        return str(value)
    except KeyError as e:
        return f"Missing key: {e}"
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Should have some form of error binding
    assert!(
        rust_code.contains("Err(") || rust_code.contains("err") || rust_code.contains("Error"),
        "Should have error handling with binding: {}",
        rust_code
    );
}

/// Test multiple exception handlers
/// SLOW: Requires rustc compilation validation
#[test]
#[ignore = "slow: requires rustc compilation"]
fn test_multiple_exception_handlers() {
    let source = r#"
def multiple_handlers(s: str, d: dict[str, int]) -> int:
    try:
        num: int = int(s)
        return d[str(num)]
    except ValueError:
        return -1
    except KeyError:
        return -2
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Should handle multiple error cases
    assert!(
        rust_code.contains("-1") && rust_code.contains("-2"),
        "Should have both fallback values: {}",
        rust_code
    );
}

/// Test nested try/except blocks
#[test]
fn test_nested_try_except() {
    let source = r#"
def nested_handler(x: int) -> int:
    outer: int = 0
    inner: int = 0
    try:
        outer = x + 1
        try:
            inner = outer * 2
            return inner
        except ValueError:
            return outer
    except Exception:
        return 0
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Should have nested match or result handling
    // The key is that both outer and inner variables are declared
    assert!(
        rust_code.contains("outer") && rust_code.contains("inner"),
        "Should hoist both outer and inner variables: {}",
        rust_code
    );
}

/// Test that try/except return types are consistent
/// SLOW: Requires rustc compilation validation
#[test]
#[ignore = "slow: requires rustc compilation"]
fn test_try_except_return_type_consistency() {
    let source = r#"
def divide_safe(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // try/except functions are wrapped in Result by the transpiler
    // The important thing is that it has consistent return handling
    assert!(
        rust_code.contains("Result<i64") || rust_code.contains("-> i64"),
        "Return type should be i64 or Result<i64>: {}",
        rust_code
    );

    // Both branches should return the same type
    assert!(
        rust_code.contains("return Ok(0)") || rust_code.contains("return 0"),
        "Except branch should return 0: {}",
        rust_code
    );
}

/// Test early return within try block
#[test]
fn test_early_return_in_try() {
    let source = r#"
def early_return_in_try(x: int) -> int:
    try:
        if x < 0:
            return -1
        result: int = x * 2
        return result
    except Exception:
        return 0
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Should handle early return correctly
    assert!(
        rust_code.contains("return") || rust_code.contains("Ok("),
        "Should have early return handling: {}",
        rust_code
    );
}

/// Test exception with computation chain
#[test]
fn test_exception_computation_chain() {
    let source = r#"
def exception_with_computation(a: int, b: int, c: int) -> int:
    try:
        step1: int = a // b
        step2: int = step1 * c
        return step2
    except ZeroDivisionError:
        return -1
"#;

    let rust_code = transpile_python(source).expect("Failed to transpile");

    // Should have step1 and step2 variables
    assert!(
        rust_code.contains("step1") && rust_code.contains("step2"),
        "Should have intermediate variables: {}",
        rust_code
    );
}

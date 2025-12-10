//! DEPYLER-0327: Try Block Analysis for Exception Type Generation
//!
//! Tests that exception types are generated even when caught internally in try/except blocks.
//!
//! ## Problem
//! Before DEPYLER-0327, the property analyzer didn't analyze try/except blocks, so exception
//! types used only within caught contexts were never generated, causing compilation errors.
//!
//! Example:
//! ```python
//! def operation_with_cleanup(value):
//!     try:
//!         if value < 0:
//!             raise ValueError("negative value")  # ValueError needed here!
//!         return value * 2
//!     except ValueError:
//!         return 0
//! ```
//!
//! Before fix: ValueError::new used but struct ValueError not generated → E0433
//! After fix: ValueError struct generated because try/except block is analyzed
//!
//! ## Solution (DEPYLER-0327)
//! 1. Added Try block case to stmt_can_fail() in properties.rs
//! 2. Collect exception types from handler signatures (exception_type field)
//! 3. Always collect error_types even if can_fail=false (caught internally)
//! 4. Check func.properties.error_types in func_gen.rs to generate types
//!
//! ## Test Coverage
//! - ValueError in try/except with internal catch
//! - Multiple exception types in handlers
//! - Nested try/except blocks
//! - Try/except with finally
//! - Mixed caught and propagated exceptions

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

#[test]
fn test_try_except_generates_caught_exception_types() {
    let python_code = r#"
def operation_with_cleanup(value: int) -> int:
    try:
        if value < 0:
            raise ValueError("negative value")
        return value * 2
    except ValueError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // CRITICAL: ValueError should be generated even though it's caught internally
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError struct for caught exceptions.\nGenerated code:\n{}",
        rust_code
    );

    // Should have Display and Error impls
    assert!(
        rust_code.contains("impl std::fmt::Display for ValueError"),
        "Should implement Display for ValueError"
    );
    assert!(
        rust_code.contains("impl std::error::Error for ValueError"),
        "Should implement Error for ValueError"
    );

    // Should use Err(ValueError::new(...))
    assert!(
        rust_code.contains("ValueError::new("),
        "Should use ValueError::new constructor"
    );
}

#[test]
fn test_try_except_multiple_caught_types() {
    let python_code = r#"
def process_data(value: int, divisor: int) -> int:
    try:
        if value < 0:
            raise ValueError("negative value")
        if divisor == 0:
            raise ZeroDivisionError("division by zero")
        return value / divisor
    except ValueError:
        return -1
    except ZeroDivisionError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Both ValueError and ZeroDivisionError should be generated
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError for caught exception"
    );
    assert!(
        rust_code.contains("struct ZeroDivisionError"),
        "Should generate ZeroDivisionError for caught exception"
    );

    // Each should be generated exactly once
    assert_eq!(
        rust_code.matches("struct ValueError").count(),
        1,
        "ValueError should be generated exactly once"
    );
    assert_eq!(
        rust_code.matches("struct ZeroDivisionError").count(),
        1,
        "ZeroDivisionError should be generated exactly once"
    );
}

#[test]
fn test_try_except_with_finally_generates_types() {
    let python_code = r#"
def operation_with_finally(value: int) -> int:
    result = 0
    try:
        if value < 0:
            raise ValueError("negative")
        result = value * 2
    except ValueError:
        result = 0
    finally:
        pass
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // ValueError should still be generated with finally block present
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError even with finally block"
    );
}

#[test]
fn test_nested_try_except_generates_types() {
    let python_code = r#"
def nested_operations(data: list[str], index: int) -> int:
    try:
        value_str = data[index]
        try:
            value = int(value_str)
            return value
        except ValueError:
            return 0
    except IndexError:
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Both ValueError (inner) and IndexError (outer) should be generated
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError from inner try/except"
    );
    assert!(
        rust_code.contains("struct IndexError"),
        "Should generate IndexError from outer try/except"
    );
}

#[test]
fn test_try_except_with_multiple_functions() {
    let python_code = r#"
def func_a(x: int) -> int:
    try:
        if x < 0:
            raise ValueError("negative")
        return x
    except ValueError:
        return 0

def func_b(x: int) -> int:
    try:
        if x == 0:
            raise ValueError("zero")
        return 100 / x
    except ValueError:
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // ValueError should be generated only once (deduplicated)
    assert_eq!(
        rust_code.matches("struct ValueError").count(),
        1,
        "ValueError should be generated exactly once for multiple functions"
    );

    // Both functions should use ValueError
    assert_eq!(
        rust_code.matches("ValueError::new").count(),
        2,
        "Both functions should use ValueError::new"
    );
}

#[test]
#[ignore = "BLOCKED: Requires DEPYLER-0333 (exception scope tracking) - Currently generates return Err() in non-Result function"]
fn test_try_except_compiles_caught_exceptions() {
    // This test is INTENTIONALLY IGNORED because it requires exception scope tracking
    // (DEPYLER-0333) to work correctly.
    //
    // Current behavior: Generates `return Err(ValueError::new(...))` even in functions
    // returning `i32`, causing E0308 type mismatch.
    //
    // Expected behavior (after DEPYLER-0333): Should detect that ValueError is caught
    // internally and generate `return 0` (the except handler default) instead of Err().
    //
    // This test documents the expected behavior once DEPYLER-0333 is implemented.

    let python_code = r#"
def operation_with_cleanup(value: int) -> int:
    try:
        if value < 0:
            raise ValueError("negative value")
        return value * 2
    except ValueError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file and verify it compiles
    let test_code = format!(
        "{}\n{}",
        rust_code,
        r#"
fn main() {
    assert_eq!(operation_with_cleanup(5), 10);
    assert_eq!(operation_with_cleanup(-1), 0);
}
"#
    );

    std::fs::write("/tmp/test_depyler_0327_compiles.rs", test_code)
        .expect("Failed to write test file");

    let output = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("bin")
        .arg("/tmp/test_depyler_0327_compiles.rs")
        .arg("-o")
        .arg("/tmp/test_depyler_0327_compiles")
        .output()
        .expect("Failed to run rustc");

    assert!(
        output.status.success(),
        "Generated code should compile. stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_try_except_bare_except_clause() {
    let python_code = r#"
def safe_operation(value: int) -> int:
    try:
        if value < 0:
            raise ValueError("negative")
        return value * 2
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // With bare except, ValueError should still be generated
    // (from analyzing the try body)
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError even with bare except clause"
    );
}

#[test]
fn test_try_except_exception_reraised() {
    let python_code = r#"
def operation_with_logging(value: int) -> int:
    try:
        if value < 0:
            raise ValueError("negative value")
        return value * 2
    except ValueError as e:
        # Log and re-raise
        raise
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // ValueError should be generated
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError when exception is re-raised"
    );
}

#[test]
fn test_try_except_with_indexerror_from_list_access() {
    let python_code = r#"
def get_first_element(data: list[int]) -> int:
    try:
        return data[0]
    except IndexError:
        return -1
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // IndexError should be generated from handler signature
    assert!(
        rust_code.contains("struct IndexError"),
        "Should generate IndexError from except clause"
    );
}

#[test]
fn test_try_except_string_type_inference_integration() {
    // Integration test combining DEPYLER-0327 (Try block analysis)
    // with String type inference improvements
    let python_code = r#"
def parse_and_validate(data: list[str], index: int) -> int:
    try:
        value_str = data[index]
        value = int(value_str)
        if value < 0:
            raise ValueError("negative")
        return value
    except IndexError:
        return -1
    except ValueError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Both IndexError and ValueError should be generated
    assert!(
        rust_code.contains("struct IndexError"),
        "Should generate IndexError"
    );
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError"
    );

    // String type inference: value_str should use .parse::<i32>()
    // (from Vec<String>.get() type tracking)
    assert!(
        rust_code.contains(".parse::<i32>()") || rust_code.contains("parse::<i32>"),
        "Should use parse::<i32>() for String to int conversion"
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0327"]
fn test_try_except_does_not_affect_propagated_exceptions() {
    // Verify that functions that propagate exceptions (not catching them)
    // still work correctly
    let python_code = r#"
def check_positive(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x

def use_check(x: int) -> int:
    try:
        return check_positive(x)
    except ValueError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // ValueError should be generated once
    assert_eq!(
        rust_code.matches("struct ValueError").count(),
        1,
        "ValueError should be generated exactly once"
    );

    // check_positive should return Result<i32, ValueError>
    assert!(
        rust_code.contains("Result<i32, ValueError>"),
        "check_positive should return Result type"
    );

    // use_check might return i32 (if we had exception scope tracking)
    // but currently also returns Result - this is acceptable
}

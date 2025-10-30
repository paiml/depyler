//! DEPYLER-0295: ValueError Type Generation Tests
//!
//! Tests that ValueError is properly generated when Python code raises ValueError.
//!
//! ## Problem
//! When Python code raised ValueError, the transpiler generated code that used
//! ValueError but did not generate the ValueError type definition, causing
//! compilation errors like "cannot find type `ValueError` in this scope".
//!
//! ## Solution
//! - Added `needs_valueerror` flag to CodeGenContext
//! - Generate ValueError type definition when flag is set
//! - Set flag when ValueError is encountered in raise statements
//!
//! ## Test Coverage
//! - Basic ValueError usage
//! - ValueError with custom message
//! - Multiple ValueError raises in different functions
//! - Compilation verification
//! - Behavior correctness

use depyler_core::DepylerPipeline;

#[test]
fn test_valueerror_type_generated() {
    let python_code = r#"
def check_positive(x: int) -> int:
    if x < 0:
        raise ValueError("negative value")
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should generate ValueError struct
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError struct definition"
    );

    // Should generate Display impl
    assert!(
        rust_code.contains("impl std::fmt::Display for ValueError"),
        "Should implement Display for ValueError"
    );

    // Should generate Error impl
    assert!(
        rust_code.contains("impl std::error::Error for ValueError"),
        "Should implement Error trait for ValueError"
    );

    // Should have constructor
    assert!(
        rust_code.contains("pub fn new(message: impl Into<String>)"),
        "Should have ValueError::new constructor"
    );
}

#[test]
fn test_valueerror_return_type() {
    let python_code = r#"
def validate_range(x: int) -> int:
    if x < 0 or x > 100:
        raise ValueError("value must be 0-100")
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Function should return Result<i32, ValueError>
    assert!(
        rust_code.contains("Result<i32, ValueError>"),
        "Function should return Result<i32, ValueError>"
    );

    // Should use Err(ValueError::new(...))
    assert!(
        rust_code.contains("Err(ValueError::new("),
        "Should use Err(ValueError::new(...))"
    );
}

#[test]
fn test_valueerror_multiple_functions() {
    let python_code = r#"
def check_positive(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x

def check_range(x: int) -> int:
    if x < 0 or x > 100:
        raise ValueError("out of range")
    return x

def validate(x: int) -> int:
    if x == 42:
        raise ValueError("forbidden value")
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should generate ValueError only once
    let valueerror_count = rust_code.matches("struct ValueError").count();
    assert_eq!(
        valueerror_count, 1,
        "Should generate ValueError struct exactly once"
    );

    // All functions should use Result<i32, ValueError>
    let result_count = rust_code.matches("Result<i32, ValueError>").count();
    assert_eq!(
        result_count, 3,
        "All three functions should return Result<i32, ValueError>"
    );
}

#[test]
fn test_valueerror_compiles() {
    let python_code = r#"
def check_positive(x: int) -> int:
    if x < 0:
        raise ValueError("negative value")
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file
    let test_code = format!(
        "{}\n{}",
        rust_code,
        r#"
fn main() {
    assert!(check_positive(5).is_ok());
    assert!(check_positive(-1).is_err());
}
"#
    );

    std::fs::write("/tmp/test_depyler_0295_compiles.rs", test_code)
        .expect("Failed to write test file");

    // Attempt compilation
    let output = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("bin")
        .arg("/tmp/test_depyler_0295_compiles.rs")
        .arg("-o")
        .arg("/tmp/test_depyler_0295_compiles")
        .output()
        .expect("Failed to run rustc");

    assert!(
        output.status.success(),
        "Generated code should compile successfully. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_valueerror_behavior() {
    let python_code = r#"
def check_positive(x: int) -> int:
    if x < 0:
        raise ValueError("negative value")
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file
    let test_code = format!(
        "{}\n{}",
        rust_code,
        r#"
fn main() {
    // Positive values should succeed
    let result = check_positive(5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    // Negative values should return Err with ValueError
    let result = check_positive(-1);
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = format!("{}", error);
    assert!(error_msg.contains("value error"));
    assert!(error_msg.contains("negative value"));
}
"#
    );

    std::fs::write("/tmp/test_depyler_0295_behavior.rs", test_code)
        .expect("Failed to write test file");

    // Compile
    let compile_output = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("bin")
        .arg("/tmp/test_depyler_0295_behavior.rs")
        .arg("-o")
        .arg("/tmp/test_depyler_0295_behavior")
        .output()
        .expect("Failed to run rustc");

    assert!(
        compile_output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&compile_output.stderr)
    );

    // Run the test
    let run_output = std::process::Command::new("/tmp/test_depyler_0295_behavior")
        .output()
        .expect("Failed to run test binary");

    assert!(
        run_output.status.success(),
        "Test execution failed. stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&run_output.stdout),
        String::from_utf8_lossy(&run_output.stderr)
    );
}

#[test]
fn test_valueerror_with_different_messages() {
    let python_code = r#"
def validate_age(age: int) -> int:
    if age < 0:
        raise ValueError("age cannot be negative")
    if age > 150:
        raise ValueError("age too high")
    return age
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should generate ValueError
    assert!(
        rust_code.contains("struct ValueError"),
        "Should generate ValueError struct"
    );

    // Should have both error messages in the generated code
    assert!(
        rust_code.contains("age cannot be negative"),
        "Should preserve first error message"
    );
    assert!(
        rust_code.contains("age too high"),
        "Should preserve second error message"
    );
}

#[test]
fn test_valueerror_not_generated_when_not_used() {
    let python_code = r#"
def add(x: int, y: int) -> int:
    return x + y
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should NOT generate ValueError when not used
    assert!(
        !rust_code.contains("struct ValueError"),
        "Should not generate ValueError when not used"
    );
}

#[test]
fn test_valueerror_with_zerodivisionerror() {
    let python_code = r#"
def safe_divide(x: int, y: int) -> int:
    if y == 0:
        raise ZeroDivisionError("division by zero")
    if x < 0:
        raise ValueError("negative dividend")
    return x / y
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // NOTE: When multiple different exception types are raised in a single function,
    // the transpiler currently uses Box<dyn std::error::Error> and doesn't generate
    // specific error type definitions. This is a known limitation, separate from
    // DEPYLER-0295. DEPYLER-0295 specifically fixes single-error-type generation.

    // Should use Box<dyn std::error::Error> for multiple error types
    assert!(
        rust_code.contains("Box<dyn std::error::Error>"),
        "Should use Box<dyn Error> for functions with multiple error types"
    );

    // Should still reference both error types (even if not generating the structs)
    assert!(
        rust_code.contains("ValueError::new"),
        "Should use ValueError::new"
    );
    assert!(
        rust_code.contains("ZeroDivisionError::new"),
        "Should use ZeroDivisionError::new"
    );
}

#[test]
fn test_valueerror_display_format() {
    let python_code = r#"
def check(x: int) -> int:
    if x == 0:
        raise ValueError("zero not allowed")
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Display implementation should use "value error:" prefix
    assert!(
        rust_code.contains(r#"write!(f, "value error: {}", self.message)"#),
        "Display should format with 'value error:' prefix"
    );
}

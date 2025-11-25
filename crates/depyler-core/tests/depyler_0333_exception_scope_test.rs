//! DEPYLER-0333: Exception Scope Tracking Tests
//!
//! **EXTREME TDD Protocol - Phase 1: RED**
//!
//! This test suite validates exception scope tracking functionality:
//! - Try/except blocks with caught exceptions
//! - Nested try/except structures
//! - raise statements in different scopes
//! - Result<T, E> vs panic!() behavior
//!
//! All tests should FAIL initially, then pass after implementation.

use depyler_core::DepylerPipeline;

// ============================================================================
// UNIT TESTS - Basic Exception Scope Tracking
// ============================================================================

#[test]
#[ignore = "DEPYLER-0333: Exception scope tracking not implemented yet - RED phase"]
fn test_0333_01_simple_try_except_caught_exception() {
    // Pattern: Exception is caught internally - should NOT propagate Result
    let python = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return i32, NOT Result<i32, ZeroDivisionError>
    assert!(
        rust_code.contains("fn safe_divide") && rust_code.contains("-> i32"),
        "Should return i32, not Result.\nGot:\n{}",
        rust_code
    );

    // Should NOT have return Err() in exception handler
    assert!(
        !rust_code.contains("return Err(ZeroDivisionError"),
        "Should not return Err in caught exception.\nGot:\n{}",
        rust_code
    );

    // Should return fallback value (0) in exception handler
    assert!(
        rust_code.contains("return 0"),
        "Should return fallback value in exception handler.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_02_nested_try_except_blocks() {
    // Pattern: Nested try blocks with different exception types
    let python = r#"
def nested_operations(data: list[int]) -> int:
    try:
        # Outer try - handles IndexError
        value = data[0]
        try:
            # Inner try - handles ZeroDivisionError
            return 100 // value
        except ZeroDivisionError:
            return -1
    except IndexError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return i32 (all exceptions caught)
    assert!(
        rust_code.contains("-> i32"),
        "Should return i32 with all exceptions caught.\nGot:\n{}",
        rust_code
    );

    // Should have proper exception handling for both levels
    assert!(
        rust_code.contains("-1") && rust_code.contains("-2"),
        "Should have both exception handlers.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_03_try_except_finally() {
    // Pattern: Try/except/finally block
    let python = r#"
def with_finally(x: int) -> int:
    result = 0
    try:
        result = 10 // x
    except ZeroDivisionError:
        result = -1
    finally:
        result = result + 1
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should have finally block logic
    assert!(
        rust_code.contains("result + 1"),
        "Should have finally block logic.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_04_raise_in_try_block_caught() {
    // Pattern: raise inside try block with matching handler
    let python = r#"
def validate_positive(n: int) -> int:
    try:
        if n < 0:
            raise ValueError("must be positive")
        return n * 2
    except ValueError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return i32 (exception is caught)
    assert!(
        rust_code.contains("-> i32"),
        "Should return i32 with exception caught.\nGot:\n{}",
        rust_code
    );

    // Should NOT use return Err() for raise inside caught try block
    // Instead should use control flow to jump to handler
    assert!(
        !rust_code.contains("return Err(ValueError"),
        "Should not return Err for caught exception.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_05_raise_outside_try_block_panic() {
    // Pattern: raise outside try block in non-Result function
    let python = r#"
def validate_positive(n: int) -> int:
    if n < 0:
        raise ValueError("must be positive")
    return n * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should either:
    // Option 1: Use panic!() for uncaught exception in non-Result function
    // Option 2: Change signature to Result<i32, ValueError>
    let uses_panic = rust_code.contains("panic!");
    let uses_result = rust_code.contains("Result<i32, ValueError>");

    assert!(
        uses_panic || uses_result,
        "Should use panic!() or Result for uncaught exception.\nGot:\n{}",
        rust_code
    );

    // If using Result, should have return Err()
    if uses_result {
        assert!(
            rust_code.contains("Err(ValueError"),
            "Result function should use Err for raise.\nGot:\n{}",
            rust_code
        );
    }
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_06_multiple_exception_types_in_handlers() {
    // Pattern: Multiple exception types handled separately
    let python = r#"
def process(data: list[int], divisor: int) -> int:
    try:
        value = data[0]
        return 100 // value
    except IndexError:
        return -1
    except ZeroDivisionError:
        return -2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return i32 (all exceptions caught)
    assert!(
        rust_code.contains("-> i32"),
        "Should return i32 with all exceptions caught.\nGot:\n{}",
        rust_code
    );

    // Should have both exception handlers
    assert!(
        rust_code.contains("-1") && rust_code.contains("-2"),
        "Should have handlers for both exception types.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_07_bare_except_clause() {
    // Pattern: Bare except (catches all exceptions)
    let python = r#"
def safe_operation(x: int, y: int) -> int:
    try:
        return x // y
    except:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return i32 (all exceptions caught by bare except)
    assert!(
        rust_code.contains("-> i32"),
        "Should return i32 with bare except.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_08_exception_reraising() {
    // Pattern: Exception caught and re-raised
    let python = r#"
def logged_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        print("Division by zero!")
        raise
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return Result (exception is re-raised)
    assert!(
        rust_code.contains("Result<i32, ZeroDivisionError>"),
        "Should return Result when exception is re-raised.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_09_function_call_can_raise_in_try() {
    // Pattern: Function call that can raise, inside try block
    let python = r#"
def parse_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should return i32 (ValueError is caught)
    assert!(
        rust_code.contains("-> i32"),
        "Should return i32 with exception caught.\nGot:\n{}",
        rust_code
    );

    // Should use .unwrap_or() or similar for caught exception
    // NOT the ? operator
    assert!(
        !rust_code.contains("parse::<i32>()?"),
        "Should not use ? operator for caught exception.\nGot:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_10_mixed_result_and_panic_functions() {
    // Pattern: Function that can both panic and return Result
    let python = r#"
def complex_operation(x: int, y: int) -> int:
    # This raises but is NOT caught - should change signature or panic
    if x < 0:
        raise ValueError("x must be non-negative")

    # This raises and IS caught - should use unwrap_or
    try:
        result = 100 // y
    except ZeroDivisionError:
        result = 0

    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Should either:
    // 1. Return Result and handle both exceptions consistently
    // 2. panic!() for uncaught ValueError, unwrap_or for caught ZeroDivisionError
    let uses_result = rust_code.contains("Result<i32, ValueError>");
    let uses_panic = rust_code.contains("panic!");

    assert!(
        uses_result || uses_panic,
        "Should handle uncaught exception appropriately.\nGot:\n{}",
        rust_code
    );
}

// ============================================================================
// COMPILATION TESTS - Verify Generated Rust Compiles
// ============================================================================

#[test]
#[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
fn test_0333_compilation_safe_divide() {
    // Integration test: Verify generated code compiles without errors
    let python = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.as_ref().err()
    );

    let rust_code = result.unwrap();

    // Write to temp file and compile with rustc
    let temp_dir = tempfile::tempdir().unwrap();
    let rust_file = temp_dir.path().join("lib.rs");
    std::fs::write(&rust_file, &rust_code).unwrap();

    let output = std::process::Command::new("rustc")
        .args([
            "--crate-type",
            "lib",
            "--edition",
            "2021",
            "--deny",
            "warnings",
            rust_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run rustc");

    assert!(
        output.status.success(),
        "Generated code failed to compile:\nSTDOUT:\n{}\nSTDERR:\n{}\nRust code:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        rust_code
    );
}

// ============================================================================
// PROPERTY TESTS - Validate Invariants
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Property 1: Try blocks always have matching exception handlers
    proptest! {
        #[test]
        #[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
        fn prop_try_blocks_have_handlers(exception_type in "ValueError|ZeroDivisionError|IndexError") {
            let python = format!(r#"
def test_function(x: int) -> int:
    try:
        if x < 0:
            raise {}("error")
        return x
    except {}:
        return 0
"#, exception_type, exception_type);

            let pipeline = DepylerPipeline::new();
            let result = pipeline.transpile(&python);

            // Should transpile successfully
            prop_assert!(result.is_ok(), "Transpilation failed for {}: {:?}", exception_type, result.err());
        }
    }

    // Property 2: Nested try blocks maintain proper scope nesting
    proptest! {
        #[test]
        #[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
        fn prop_nested_try_blocks_well_formed(depth in 1..=3usize) {
            // Generate nested try blocks
            let mut python = String::from("def test_function(x: int) -> int:\n");
            python.push_str("    result = x\n");

            for i in 0..depth {
                python.push_str(&format!("{}try:\n", "    ".repeat(i + 1)));
                python.push_str(&format!("{}result = result // 2\n", "    ".repeat(i + 2)));
            }

            for i in (0..depth).rev() {
                python.push_str(&format!("{}except ZeroDivisionError:\n", "    ".repeat(i + 1)));
                python.push_str(&format!("{}result = 0\n", "    ".repeat(i + 2)));
            }

            python.push_str("    return result\n");

            let pipeline = DepylerPipeline::new();
            let result = pipeline.transpile(&python);

            // Should transpile successfully with nested try blocks
            prop_assert!(result.is_ok(), "Nested try blocks failed at depth {}: {:?}", depth, result.err());
        }
    }

    // Property 3: Caught exceptions never generate ? operator
    proptest! {
        #[test]
        #[ignore = "DEPYLER-0333: Not implemented yet - RED phase"]
        fn prop_caught_exceptions_no_question_mark(operation in "[+\\-*/]") {
            let python = format!(r#"
def test_function(a: int, b: int) -> int:
    try:
        return a {} b
    except:
        return 0
"#, operation);

            let pipeline = DepylerPipeline::new();
            let result = pipeline.transpile(&python);

            if let Ok(rust_code) = result {
                // Should NOT contain ? operator in caught try block
                prop_assert!(
                    !rust_code.contains("?"),
                    "Caught exception should not use ? operator:\n{}",
                    rust_code
                );
            }
        }
    }
}

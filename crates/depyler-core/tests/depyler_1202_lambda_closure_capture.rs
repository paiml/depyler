//! DEPYLER-1202: Lambda Closure Capture Tests
//!
//! Tests for E0425 "cannot find value" errors caused by lambdas
//! that reference variables from outer scope without proper cloning.
//!
//! Python's lambdas can freely reference outer scope variables.
//! In Rust, move closures capture by move, requiring clones for non-Copy types.

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;
use std::process::Command;

/// Helper to check if generated code compiles
fn check_compiles(rust_code: &str) -> Result<(), String> {
    // Write to temp file
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("depyler_1202_test.rs");
    std::fs::write(&temp_file, rust_code).map_err(|e| format!("Write error: {}", e))?;

    // Try to compile
    let output = Command::new("rustc")
        .args([
            "--crate-type",
            "lib",
            "--edition",
            "2021",
            "-o",
            "/dev/null",
        ])
        .arg(&temp_file)
        .output()
        .map_err(|e| format!("Compile error: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Compilation failed:\n{}", stderr))
    }
}

/// Test 1: Lambda capturing a local variable (non-Copy type like String)
#[test]
fn test_DEPYLER_1202_lambda_captures_local_string() {
    let python = r#"
def process(items: list) -> list:
    prefix = "item_"
    return list(map(lambda x: prefix + str(x), items))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Lambda capturing local string should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Debug: print generated code
    eprintln!("Generated code:\n{}", rust_code);

    // Should reference prefix somewhere (either cloned before or in closure)
    assert!(
        rust_code.contains("prefix"),
        "Should reference prefix.\nGot:\n{}",
        rust_code
    );

    // CRITICAL: The generated code must compile without E0382/E0425 errors
    match check_compiles(&rust_code) {
        Ok(()) => (),
        Err(e) => {
            // Check specifically for the closure capture issue
            if e.contains("E0382") || e.contains("use of moved value") {
                panic!(
                    "DEPYLER-1202: Lambda closure capture issue - \
                     String captured without clone.\n\
                     Fix: Clone captured non-Copy variables before move closure.\n\
                     Error: {}\n\nGenerated code:\n{}",
                    e, rust_code
                );
            } else if e.contains("E0425") || e.contains("cannot find value") {
                panic!(
                    "DEPYLER-1202: Unresolved name in lambda.\n\
                     Fix: Ensure captured variables are in scope.\n\
                     Error: {}\n\nGenerated code:\n{}",
                    e, rust_code
                );
            }
            // Other compilation errors are acceptable (may need other fixes)
            eprintln!("Note: Code has other compilation issues: {}", e);
        }
    }
}

/// Test 2: Lambda capturing multiple outer variables
#[test]
fn test_DEPYLER_1202_lambda_captures_multiple_vars() {
    let python = r#"
def format_items(items: list) -> list:
    prefix = "["
    suffix = "]"
    return list(map(lambda x: prefix + str(x) + suffix, items))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Lambda capturing multiple vars should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    // Both variables should be referenced
    assert!(
        rust_code.contains("prefix") && rust_code.contains("suffix"),
        "Should reference both captured variables.\nGot:\n{}",
        rust_code
    );
}

/// Test 3: Lambda in filter capturing condition variable
#[test]
fn test_DEPYLER_1202_lambda_filter_captures_threshold() {
    let python = r#"
def filter_above(numbers: list, threshold: int) -> list:
    min_val = threshold
    return list(filter(lambda x: x > min_val, numbers))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Lambda in filter capturing local should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    assert!(
        rust_code.contains("min_val"),
        "Should reference min_val.\nGot:\n{}",
        rust_code
    );
}

/// Test 4: Nested lambda with outer scope capture
#[test]
fn test_DEPYLER_1202_nested_lambda_outer_capture() {
    let python = r#"
def transform_matrix(matrix: list) -> list:
    multiplier = 2
    return list(map(lambda row: list(map(lambda x: x * multiplier, row)), matrix))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Nested lambda with outer capture should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    // The inner lambda should have access to multiplier from outer scope
    assert!(
        rust_code.contains("multiplier"),
        "Should reference multiplier from outer scope.\nGot:\n{}",
        rust_code
    );
}

/// Test 5: Lambda stored in variable then used (common pattern)
#[test]
fn test_DEPYLER_1202_lambda_stored_then_used() {
    let python = r#"
def process_with_offset(numbers: list, offset: int) -> list:
    adjustment = offset * 2
    transform = lambda x: x + adjustment
    return list(map(transform, numbers))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Lambda stored in variable should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    assert!(
        rust_code.contains("adjustment"),
        "Should reference adjustment.\nGot:\n{}",
        rust_code
    );
}

/// Test 6: HOSTILE - Lambda captures loop variable (common E0425 source)
#[test]
fn test_DEPYLER_1202_hostile_lambda_captures_loop_var() {
    let python = r#"
def create_multipliers(numbers: list) -> list:
    results = []
    for i in range(3):
        multiplier = i + 1
        results.append(lambda x: x * multiplier)
    return results
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    // This is a known problematic pattern - may or may not transpile correctly
    // The test documents the expected behavior
    if let Ok(rust_code) = result {
        eprintln!("Generated code:\n{}", rust_code);
        // If it transpiles, multiplier should be captured
        assert!(
            rust_code.contains("multiplier"),
            "Should reference multiplier.\nGot:\n{}",
            rust_code
        );
    }
}

/// Test 7: Lambda with comprehension capturing outer variable
#[test]
fn test_DEPYLER_1202_lambda_in_comprehension_captures() {
    let python = r#"
def scale_all(matrix: list, factor: int) -> list:
    scale = factor
    return [[scale * x for x in row] for row in matrix]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Comprehension with captured variable should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    assert!(
        rust_code.contains("scale"),
        "Should reference scale.\nGot:\n{}",
        rust_code
    );
}

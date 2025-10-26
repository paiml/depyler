// ============================================================================
// DEPYLER-0265: Iterator Dereferencing Bug in For Loops
// ============================================================================
// BUG: For loops over collections generate .iter() which yields &T, but loop
// body treats values as T, causing type mismatches in comparisons/assignments
//
// ROOT CAUSE: stmt_gen.rs generates `for item in collection.iter()` where
// item is &T, but code uses item as if it were T
//
// FIX: Add automatic dereferencing - either:
//   1. Add `let item = *item_ref;` at start of loop body
//   2. Use `.iter().copied()` for Copy types
//   3. Dereference at usage sites
//
// DISCOVERED: Performance Benchmarking Campaign (compute_intensive.py)
// SEVERITY: P0 BLOCKING - prevents compilation of for loops over collections
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0265_for_loop_comparison_compiles() {
    // DEPYLER-0265: For loop with comparison generates type mismatch
    // RED Phase: This test MUST FAIL initially

    let python_code = r#"
def find_min(numbers: list[int]) -> int:
    """Find minimum value in a list."""
    if not numbers:
        return 0
    min_val = numbers[0]
    for num in numbers:
        if num < min_val:
            min_val = num
    return min_val
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Debugging: Print generated code
    eprintln!("=== DEPYLER-0265: Generated Rust Code (comparison) ===");
    eprintln!("{}", rust_code);

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0265_comparison.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0265: Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0265_comparison.rlib")
        .output()
        .expect("DEPYLER-0265: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0265: rustc stderr ===");
        eprintln!("{}", stderr);

        // ASSERT: Must NOT have type mismatch errors
        assert!(
            !stderr.contains("mismatched types"),
            "DEPYLER-0265 FAILURE: Type mismatch in for loop comparison!\n\
             Expected: Iterator dereferencing handled automatically\n\
             Actual: Generated code has &T vs T type mismatch\n\
             \n\
             Common error patterns:\n\
             - 'expected `&i32`, found `i32`' (comparing &T with T)\n\
             - 'expected `i32`, found `&i32`' (assigning &T to T)\n\
             \n\
             See docs/bugs/DEPYLER-0265.md for details.\n\
             \n\
             Generated Rust code:\n{}\n\
             \n\
             rustc error:\n{}",
            rust_code,
            stderr
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0265: Compilation should succeed\n\
         Generated code:\n{}\n\
         Errors:\n{}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0265_for_loop_arithmetic_compiles() {
    // DEPYLER-0265: For loop with arithmetic generates type mismatch

    let python_code = r#"
def sum_list(numbers: list[int]) -> int:
    """Sum all numbers in a list."""
    total = 0
    for num in numbers:
        total = total + num
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0265_arithmetic.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0265: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0265_arithmetic.rlib")
        .output()
        .expect("DEPYLER-0265: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0265: rustc stderr (arithmetic) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("mismatched types"),
            "DEPYLER-0265: Type mismatch in for loop arithmetic!\n\
             Error: {}\n\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0265: Arithmetic in for loop should compile\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0265_for_loop_assignment_compiles() {
    // DEPYLER-0265: For loop with assignment generates type mismatch

    let python_code = r#"
def find_max(numbers: list[int]) -> int:
    """Find maximum value in a list."""
    if not numbers:
        return 0
    max_val = numbers[0]
    for num in numbers:
        if num > max_val:
            max_val = num
    return max_val
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0265_assignment.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0265: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0265_assignment.rlib")
        .output()
        .expect("DEPYLER-0265: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0265: rustc stderr (assignment) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("mismatched types"),
            "DEPYLER-0265: Type mismatch in for loop assignment!\n\
             Error: {}\n\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0265: Assignment in for loop should compile\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0265_for_loop_string_compiles() {
    // DEPYLER-0265: For loop over strings (non-Copy types)

    let python_code = r#"
def find_longest(words: list[str]) -> str:
    """Find the longest word in a list."""
    if not words:
        return ""
    longest = words[0]
    for word in words:
        if len(word) > len(longest):
            longest = word
    return longest
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0265_string.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0265: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0265_string.rlib")
        .output()
        .expect("DEPYLER-0265: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0265: rustc stderr (string) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("mismatched types"),
            "DEPYLER-0265: Type mismatch in for loop over strings!\n\
             Error: {}\n\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0265: For loop over strings should compile\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

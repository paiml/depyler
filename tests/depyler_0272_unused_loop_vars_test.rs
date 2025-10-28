// DEPYLER-0272: Unused Loop Variables - RED Phase Tests
// Tests verify transpiler prefixes unused loop variables with underscore
//
// Expected Behavior:
// - When for loop variable is declared but not used in body
// - Transpiler should prefix variable name with _ (e.g., i → _i)
// - Generated code should compile with --deny warnings
//
// Bug: Currently generates unused variable warnings with -D warnings

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;
use std::fs;
use std::process::Command;

/// Helper function to verify generated Rust code compiles with --deny warnings
fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_file = format!("/tmp/depyler_0272_{}.rs", test_name);
    fs::write(&temp_file, rust_code).expect("Failed to write temp file");

    let output = Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--deny",
            "warnings",
            &temp_file,
            "-o",
            &format!("/tmp/depyler_0272_{}.rlib", test_name),
        ])
        .output()
        .expect("Failed to run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Generated Rust code failed to compile for {}:\n{}\n\nGenerated code:\n{}",
            test_name, stderr, rust_code
        );
    }

    // Cleanup
    let _ = fs::remove_file(&temp_file);
    let _ = fs::remove_file(format!("/tmp/depyler_0272_{}.rlib", test_name));
}

/// Helper to check if error contains unused variable warning
fn contains_unused_variable_warning(rust_code: &str, var_name: &str) -> bool {
    let temp_file = "/tmp/depyler_0272_check.rs";
    fs::write(temp_file, rust_code).expect("Failed to write temp file");

    let output = Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--deny",
            "warnings",
            temp_file,
        ])
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let has_warning =
        stderr.contains("unused variable") && stderr.contains(&format!("`{}`", var_name));

    let _ = fs::remove_file(temp_file);
    has_warning
}

#[test]
fn test_DEPYLER_0272_range_loop_unused_variable_compiles() {
    // Test Case: Classic Fibonacci pattern - range loop with unused index
    // This is the EXACT pattern from the failing DEPYLER-0271 test
    let python = r#"
def fibonacci(n: int) -> int:
    """Calculate Fibonacci number."""
    if n <= 1:
        return n

    a = 0
    b = 1
    for i in range(2, n + 1):  # i is unused - should become _i
        c = a + b
        a = b
        b = c

    return b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Verify generated code uses _i instead of i
    // Should generate: for _i in 2..n + 1 {
    // Not: for i in 2..n + 1 {

    // Before fix: Fails with "unused variable: `i`"
    // After fix: Compiles successfully with _i
    assert_compiles(&rust_code, "range_loop_unused");
}

#[test]
fn test_DEPYLER_0272_list_loop_unused_variable_compiles() {
    // Test Case: Iterating over list but not using loop variable
    // Common pattern for counting or side effects
    let python = r#"
def count_iterations(items: list[int]) -> int:
    """Count loop iterations without using items."""
    count = 0
    for item in items:  # item is unused - should become _item
        count = count + 1
    return count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: for _item in items.iter().cloned() {
    // Not: for item in items.iter().cloned() {

    assert_compiles(&rust_code, "list_loop_unused");
}

#[test]
fn test_DEPYLER_0272_used_variable_not_prefixed() {
    // Test Case: Loop variable IS used - should NOT add underscore
    // Regression test to ensure we don't break working code
    let python = r#"
def sum_list(numbers: list[int]) -> int:
    """Sum all numbers in list."""
    total = 0
    for num in numbers:  # num IS used - keep as num
        total = total + num
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: for num in numbers.iter().cloned() {
    // NOT: for _num in numbers.iter().cloned() {
    // Because num is actually used in the body (total = total + num)

    assert!(
        rust_code.contains("for num in"),
        "Used variable should NOT be prefixed with underscore"
    );
    assert_compiles(&rust_code, "used_variable_not_prefixed");
}

#[test]
fn test_DEPYLER_0272_multiple_unused_loops_compile() {
    // Test Case: Multiple loops with unused variables
    // Ensure fix works across multiple loops in same function
    let python = r#"
def repeat_operations(n: int) -> int:
    """Perform operations n times."""
    result = 0

    for i in range(n):  # i unused - should be _i
        result = result + 1

    for j in range(n):  # j unused - should be _j
        result = result * 2

    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Both loops should have underscored variables
    // for _i in 0..n { ... }
    // for _j in 0..n { ... }

    assert_compiles(&rust_code, "multiple_unused_loops");
}

#[test]
fn test_DEPYLER_0272_nested_loops_unused_compiles() {
    // Test Case: Nested loops where outer variable is unused
    let python = r#"
def nested_count(outer: list[int], inner: list[int]) -> int:
    """Nested loops with unused outer variable."""
    count = 0
    for x in outer:  # x unused - should be _x
        for y in inner:  # y used - keep as y
            count = count + y
    return count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Outer loop: for _x in outer.iter().cloned() {
    // Inner loop: for y in inner.iter().cloned() {
    //   count = count + y;  // y is used here
    // }

    assert_compiles(&rust_code, "nested_loops_unused");
}

#[test]
fn test_DEPYLER_0272_enumerate_pattern_unused_index() {
    // Test Case: Using enumerate() but only using value, not index
    // Python: for i, val in enumerate(items)
    // If only val is used, i should become _i
    let python = r#"
def process_values(items: list[str]) -> int:
    """Process list values, ignore indices."""
    count = 0
    for i, val in enumerate(items):  # i unused, val used
        if val:
            count = count + 1
    return count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Tuple unpacking: for (_i, val) in items.iter().enumerate() {
    // i is unused, val is used

    assert_compiles(&rust_code, "enumerate_unused_index");
}

#[test]
#[ignore = "Diagnostic test - run manually to verify current bug"]
fn test_DEPYLER_0272_verify_current_bug() {
    // This test verifies the bug exists by checking for unused variable warning
    // Run with: cargo test test_DEPYLER_0272_verify_current_bug -- --ignored --nocapture
    let python = r#"
def simple_loop(n: int) -> int:
    result = 0
    for i in range(n):  # i is not used in body
        result = result + 1
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Check if current code produces unused variable warning
    if contains_unused_variable_warning(&rust_code, "i") {
        println!("\n✅ BUG CONFIRMED: Loop variable 'i' is unused but not prefixed with _");
        println!("Expected error: 'unused variable: `i`'");
        println!("Fix needed: Detect unused loop variables and prefix with _");
    } else {
        println!("\n⚠️ Bug may already be fixed or error message changed");
    }
}

// Property-based test: Verify various unused loop patterns
#[test]
fn test_DEPYLER_0272_various_unused_patterns() {
    let test_cases = vec![
        (
            "simple range",
            r#"
def f(n: int) -> int:
    x = 0
    for i in range(n):
        x = x + 1
    return x
"#,
        ),
        (
            "list iteration",
            r#"
def f(items: list[int]) -> int:
    count = 0
    for item in items:
        count = count + 1
    return count
"#,
        ),
        (
            "complex range",
            r#"
def f(start: int, end: int) -> int:
    result = 0
    for i in range(start, end):
        result = result + 1
    return result
"#,
        ),
    ];

    for (name, python) in test_cases {
        println!("\nTesting pattern: {}", name);
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(python);
        assert!(
            result.is_ok(),
            "Transpilation failed for {}: {:?}",
            name,
            result.err()
        );

        let rust_code = result.unwrap();
        assert_compiles(&rust_code, &format!("pattern_{}", name.replace(' ', "_")));
    }
}

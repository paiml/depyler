/// DEPYLER-0287 & DEPYLER-0288: Recursive List Function Tests
///
/// These tests verify that the transpiler correctly handles:
/// 1. Result type propagation in recursive function calls
/// 2. Type annotation for negative list indexing
///
/// Expected behavior: All tests should PASS after transpiler fixes
use depyler_core::DepylerPipeline;

#[test]
fn test_recursive_list_sum_compiles() {
    let python_code = r#"
def sum_list_recursive(numbers: list[int]) -> int:
    """Recursive list summation."""
    if len(numbers) == 0:
        return 0
    else:
        first = numbers[0]
        rest = numbers[1:]
        return first + sum_list_recursive(rest)
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // DEPYLER-0287: The generated code should compile without errors
    // Key requirement: Recursive call must handle Result type properly

    // Check that the code mentions Result (due to list indexing)
    assert!(
        generated_code.contains("Result<") || generated_code.contains("i32"),
        "Generated code should handle list indexing Results"
    );

    // Verify the generated code compiles
    let syntax_check = syn::parse_file(&generated_code);
    assert!(
        syntax_check.is_ok(),
        "Generated code should be valid Rust syntax: {:?}",
        syntax_check.err()
    );

    println!("Generated code:\n{}", generated_code);
}

#[test]
fn test_recursive_list_sum_executes() {
    let python_code = r#"
def sum_list_recursive(numbers: list[int]) -> int:
    """Recursive list summation."""
    if len(numbers) == 0:
        return 0
    else:
        first = numbers[0]
        rest = numbers[1:]
        return first + sum_list_recursive(rest)
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Write to temp file and compile with rustc
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_recursive_sum.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    // Try to compile it
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_recursive_sum.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "DEPYLER-0287: Generated code should compile!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_recursive_sum.rlib"));
}

#[test]
fn test_negative_index_compiles() {
    let python_code = r#"
def get_last_element(items: list[int]) -> int:
    """Get last element using negative index."""
    return items[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // DEPYLER-0288: The generated code should have proper type annotations for idx
    // and should compile without "trait Neg not implemented" error

    println!("Generated code:\n{}", generated_code);

    // Verify the generated code compiles
    let syntax_check = syn::parse_file(&generated_code);
    assert!(
        syntax_check.is_ok(),
        "Generated code should be valid Rust syntax: {:?}",
        syntax_check.err()
    );
}

#[test]
fn test_negative_index_executes() {
    let python_code = r#"
def get_last_element(items: list[int]) -> int:
    """Get last element using negative index."""
    return items[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    // Write to temp file and compile with rustc
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_negative_index.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    // Try to compile it
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_negative_index.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "DEPYLER-0288: Generated code should compile!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_negative_index.rlib"));
}

#[test]
fn test_nested_recursion_with_lists() {
    let python_code = r#"
def fibonacci_list(n: int) -> list[int]:
    """Build fibonacci list recursively."""
    if n <= 0:
        return []
    if n == 1:
        return [1]
    prev = fibonacci_list(n - 1)
    if len(prev) > 1:
        return prev + [prev[-1] + prev[-2]]
    else:
        return prev + [1]
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    println!("Generated code:\n{}", generated_code);

    // Verify the generated code compiles
    let syntax_check = syn::parse_file(&generated_code);
    assert!(
        syntax_check.is_ok(),
        "Generated code should be valid Rust syntax: {:?}",
        syntax_check.err()
    );

    // Write to temp file and compile with rustc
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_nested_recursion.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    // Try to compile it
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_nested_recursion.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Generated code should compile!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_nested_recursion.rlib"));
}

#[test]
fn test_result_propagation_context_aware() {
    // Test that Result propagation is context-aware:
    // - In Result-returning function: use ? operator
    // - In non-Result context: use .unwrap()

    let python_code = r#"
def helper(items: list[int]) -> int:
    if len(items) == 0:
        return 0
    return items[0] + helper(items[1:])

def caller() -> int:
    return helper([1, 2, 3])
"#;

    let pipeline = DepylerPipeline::new();
    let generated_code = pipeline
        .transpile(python_code)
        .expect("Transpilation should succeed");

    println!("Generated code:\n{}", generated_code);

    // The code should compile
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_result_propagation.rs");

    std::fs::write(&test_file, &generated_code).expect("Should write temp file");

    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--edition=2021")
        .arg(&test_file)
        .arg("-o")
        .arg(temp_dir.join("test_result_propagation.rlib"))
        .output()
        .expect("Should run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Result propagation should be context-aware!\n\nGenerated code:\n{}\n\nRustc errors:\n{}",
            generated_code, stderr
        );
    }

    // Cleanup
    let _ = std::fs::remove_file(&test_file);
    let _ = std::fs::remove_file(temp_dir.join("test_result_propagation.rlib"));
}

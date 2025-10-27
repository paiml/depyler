// DEPYLER-0269: Function Parameter Borrowing - RED Phase Tests
// Tests verify transpiler adds & borrow operator when passing owned values to reference parameters
//
// Expected Behavior:
// - When function signature expects &Vec<T>, &str, &HashMap<K,V>, etc.
// - And caller passes owned value (Vec<T>, String, HashMap<K,V>)
// - Transpiler should automatically insert & to create reference
//
// Bug: Currently generates mismatched types error (expected &T, found T)

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;
use std::fs;
use std::process::Command;

/// Helper function to verify generated Rust code compiles
fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_file = format!("/tmp/depyler_0269_{}.rs", test_name);
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
            &format!("/tmp/depyler_0269_{}.rlib", test_name),
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
    let _ = fs::remove_file(format!("/tmp/depyler_0269_{}.rlib", test_name));
}

/// Helper to check if error message contains expected borrow suggestion
fn contains_borrow_error(rust_code: &str) -> bool {
    let temp_file = "/tmp/depyler_0269_check.rs";
    fs::write(temp_file, rust_code).expect("Failed to write temp file");

    let output = Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", temp_file])
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let has_error = stderr.contains("expected `&Vec<")
        || stderr.contains("expected `&HashMap<")
        || stderr.contains("expected reference")
        || stderr.contains("consider borrowing here");

    let _ = fs::remove_file(temp_file);
    has_error
}

#[test]
fn test_DEPYLER_0269_basic_reference_parameter_compiles() {
    // Test Case: Function accepts &Vec<i32>, caller passes owned Vec<i32>
    // Expected: Transpiler adds & automatically
    let python = r#"
def process(data: list[int]) -> int:
    """Function that expects a reference to a list."""
    return len(data)

def main() -> None:
    """Caller passes owned list."""
    nums = [1, 2, 3]
    result = process(nums)
    print(result)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Verify the generated code includes & in the function call
    // Should generate: process(&nums)
    // Not: process(nums)
    println!("Generated code:\n{}", rust_code);

    // Before fix: This will fail with "expected &Vec<i32>, found Vec<i32>"
    // After fix: This will compile successfully
    assert_compiles(&rust_code, "basic_reference_parameter");
}

#[test]
fn test_DEPYLER_0269_multiple_reference_parameters_compiles() {
    // Test Case: Function with multiple reference parameters
    // Expected: Add & to all arguments that need it
    let python = r#"
def merge(list1: list[int], list2: list[int]) -> list[int]:
    """Merge two lists."""
    return list1 + list2

def main() -> None:
    """Call with two owned lists."""
    a = [1, 2]
    b = [3, 4]
    result = merge(a, b)
    print(result)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should generate: merge(&a, &b)
    // Not: merge(a, b)
    println!("Generated code:\n{}", rust_code);

    // Before fix: Fails with 2 borrow errors (one for each parameter)
    // After fix: Compiles successfully
    assert_compiles(&rust_code, "multiple_reference_parameters");
}

#[test]
fn test_DEPYLER_0269_string_reference_parameter_compiles() {
    // Test Case: String parameter (expects &str or &String)
    // Expected: Add & when passing owned String
    let python = r#"
def process_text(text: str) -> int:
    """Process a string."""
    return len(text)

def main() -> None:
    """Pass owned string."""
    message = "hello world"
    length = process_text(message)
    print(length)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should generate: process_text(&message)
    // Not: process_text(message)
    println!("Generated code:\n{}", rust_code);

    // Before fix: Fails with "expected &String, found String" or similar
    // After fix: Compiles successfully
    assert_compiles(&rust_code, "string_reference_parameter");
}

#[test]
fn test_DEPYLER_0269_dict_reference_parameter_compiles() {
    // Test Case: Dict parameter (expects &HashMap)
    // Expected: Add & when passing owned HashMap
    let python = r#"
def count_keys(data: dict[str, int]) -> int:
    """Count keys in dict."""
    return len(data)

def main() -> None:
    """Pass owned dict."""
    info = {"a": 1, "b": 2, "c": 3}
    count = count_keys(info)
    print(count)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should generate: count_keys(&info)
    // Not: count_keys(info)
    println!("Generated code:\n{}", rust_code);

    // Before fix: Fails with "expected &HashMap<String, i32>, found HashMap<String, i32>"
    // After fix: Compiles successfully
    assert_compiles(&rust_code, "dict_reference_parameter");
}

#[test]
#[ignore = "Diagnostic test - run manually to verify current error messages"]
fn test_DEPYLER_0269_verify_current_bug() {
    // This test verifies the bug exists by checking for borrow error messages
    // Run with: cargo test test_DEPYLER_0269_verify_current_bug -- --ignored --nocapture
    let python = r#"
def process(data: list[int]) -> int:
    return len(data)

def main() -> None:
    nums = [1, 2, 3]
    result = process(nums)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Check if current code produces borrow error
    if contains_borrow_error(&rust_code) {
        println!("\n✅ BUG CONFIRMED: Missing & causes borrow error");
        println!("Expected error: 'expected `&Vec<i32>`, found `Vec<i32>`'");
        println!("Fix needed: Add & in function call generation");
    } else {
        println!("\n⚠️ Bug may already be fixed or error message changed");
    }
}

// Property-based test: Verify ANY reference parameter gets & automatically
#[test]
fn test_DEPYLER_0269_reference_parameter_types() {
    // Test various collection types that should all get & added
    let test_cases = vec![
        (
            "list[int]",
            r#"
def f(x: list[int]) -> int:
    return len(x)
def main() -> None:
    v = [1, 2]
    f(v)
"#,
        ),
        (
            "list[str]",
            r#"
def f(x: list[str]) -> int:
    return len(x)
def main() -> None:
    v = ["a", "b"]
    f(v)
"#,
        ),
        (
            "dict[str, int]",
            r#"
def f(x: dict[str, int]) -> int:
    return len(x)
def main() -> None:
    v = {"a": 1}
    f(v)
"#,
        ),
        (
            "set[int]",
            r#"
def f(x: set[int]) -> int:
    return len(x)
def main() -> None:
    v = {1, 2, 3}
    f(v)
"#,
        ),
    ];

    for (type_name, python) in test_cases {
        println!("\nTesting type: {}", type_name);
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(python);
        assert!(
            result.is_ok(),
            "Transpilation failed for {}: {:?}",
            type_name,
            result.err()
        );

        let rust_code = result.unwrap();
        assert_compiles(
            &rust_code,
            &format!(
                "property_{}",
                type_name
                    .replace('[', "_")
                    .replace([']', ','], "")
                    .replace(' ', "_")
            ),
        );
    }
}

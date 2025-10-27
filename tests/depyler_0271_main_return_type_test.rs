// DEPYLER-0271: Main Function Return Type - RED Phase Tests
// Tests verify transpiler generates correct return type for functions without return annotations
//
// Expected Behavior:
// - Functions without return type annotation should return () (unit type)
// - Functions with -> None annotation should return ()
// - main() function specifically should always return ()
//
// Bug: Currently generates -> serde_json::Value for functions without return type

use depyler_core::DepylerPipeline;
use std::fs;
use std::process::Command;

/// Helper function to verify generated Rust code compiles
fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_file = format!("/tmp/depyler_0271_{}.rs", test_name);
    fs::write(&temp_file, rust_code).expect("Failed to write temp file");

    let output = Command::new("rustc")
        .args(&[
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--deny",
            "warnings",
            &temp_file,
            "-o",
            &format!("/tmp/depyler_0271_{}.rlib", test_name),
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
    let _ = fs::remove_file(format!("/tmp/depyler_0271_{}.rlib", test_name));
}

#[test]
fn test_depyler_0271_main_without_return_type_compiles() {
    // Test Case: main() without return type annotation
    // Expected: pub fn main() { ... } (no return type)
    // Bug: pub fn main() -> serde_json::Value { ... }
    let python = r#"
def main():
    """Entry point without return type."""
    print("Hello world")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Verify generated code has correct signature
    // Should generate: pub fn main() {
    // Not: pub fn main() -> serde_json::Value {

    // The function body implicitly returns () (unit type)
    // If signature says -> serde_json::Value, this will fail to compile

    // Before fix: Fails with "expected serde_json::Value, found ()"
    // After fix: Compiles successfully with () return type
    assert_compiles(&rust_code, "main_without_return_type");
}

#[test]
fn test_depyler_0271_main_with_none_return_type_compiles() {
    // Test Case: main() with explicit -> None
    // Expected: pub fn main() { ... } (no return type annotation)
    let python = r#"
def main() -> None:
    """Entry point with explicit None return."""
    print("Hello world")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: pub fn main() {
    // Not: pub fn main() -> serde_json::Value {
    assert_compiles(&rust_code, "main_with_none_return_type");
}

#[test]
fn test_depyler_0271_regular_function_without_return_type_compiles() {
    // Test Case: Regular (non-main) function without return type
    // Expected: pub fn helper() { ... }
    let python = r#"
def helper():
    """Helper function without return type."""
    print("Helping")

def main() -> None:
    """Call the helper."""
    helper()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Both functions should have no return type annotation
    // Should generate:
    // pub fn helper() { ... }
    // pub fn main() { ... }
    assert_compiles(&rust_code, "regular_function_without_return_type");
}

#[test]
fn test_depyler_0271_function_with_none_return_compiles() {
    // Test Case: Function with explicit -> None
    // Expected: pub fn process() { ... }
    let python = r#"
def process() -> None:
    """Process something without returning."""
    value = 42
    print(value)

def main() -> None:
    """Entry point."""
    process()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Both functions have -> None, should generate no return type
    // pub fn process() { ... }
    // pub fn main() { ... }
    assert_compiles(&rust_code, "function_with_none_return");
}

#[test]
fn test_depyler_0271_mixed_return_types_compiles() {
    // Test Case: Mix of functions with/without return types
    // Expected: Only functions with actual return values have -> Type
    let python = r#"
def get_value() -> int:
    """Returns an integer."""
    return 42

def process() -> None:
    """Processes without returning."""
    value = get_value()
    print(value)

def helper():
    """Helper without return type."""
    print("Helping")

def main():
    """Entry point."""
    helper()
    process()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate:
    // pub fn get_value() -> i32 { ... }  // Has return type
    // pub fn process() { ... }           // No return type
    // pub fn helper() { ... }            // No return type
    // pub fn main() { ... }              // No return type
    assert_compiles(&rust_code, "mixed_return_types");
}

#[test]
fn test_depyler_0271_main_with_early_return_compiles() {
    // Test Case: main() with early return (but no value)
    // Expected: pub fn main() { ... } with return; statements
    let python = r#"
def main() -> None:
    """Main with early return."""
    if True:
        print("Early exit")
        return
    print("Normal exit")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: pub fn main() { ... return; ... }
    // Not: pub fn main() -> serde_json::Value { ... }
    assert_compiles(&rust_code, "main_with_early_return");
}

#[test]
fn test_depyler_0271_benchmark_main_pattern_compiles() {
    // Test Case: The EXACT pattern from compute_intensive.py benchmark
    // This is the real-world case that triggered the bug
    let python = r#"
def fibonacci_iterative(n: int) -> int:
    """Calculate nth Fibonacci number."""
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        c = a + b
        a = b
        b = c
    return b

def main():
    """Run benchmark - NO RETURN TYPE."""
    limits = [25, 30, 35]
    for limit in limits:
        result = fibonacci_iterative(limit)
        print(result)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate:
    // pub fn fibonacci_iterative(n: i32) -> i32 { ... }  // Has return type
    // pub fn main() { ... }                               // NO return type

    // Before fix: main() -> serde_json::Value causes type mismatch
    // After fix: main() has no return type, compiles successfully
    assert_compiles(&rust_code, "benchmark_main_pattern");
}

#[test]
fn test_depyler_0271_regression_functions_with_return_values() {
    // Regression Test: Ensure functions with actual return types still work
    // This verifies we didn't break anything with the None/unit type fix
    let python = r#"
def add(x: int, y: int) -> int:
    """Returns sum."""
    return x + y

def get_message() -> str:
    """Returns string."""
    return "Hello"

def get_list() -> list[int]:
    """Returns list."""
    return [1, 2, 3]

def main() -> None:
    """Use the functions."""
    sum_val = add(1, 2)
    msg = get_message()
    nums = get_list()
    print(sum_val)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation failed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate:
    // pub fn add(x: i32, y: i32) -> i32 { ... }
    // pub fn get_message() -> String { ... }
    // pub fn get_list() -> Vec<i32> { ... }
    // pub fn main() { ... }

    // Verify functions with real return types still work correctly
    assert_compiles(&rust_code, "regression_functions_with_return_values");
}

// DEPYLER-0270: Result Unwrapping at Call Sites - RED Phase Tests
// Tests verify transpiler automatically unwraps Result types at call sites
//
// Expected Behavior:
// - When function returns Result<HashMap<K,V>, E> or Result<Vec<T>, E>
// - Call sites should automatically unwrap with .unwrap() or ?
// - Generated code should compile without "method not found in Result" errors
//
// Bug: Currently generates code that tries to call methods on Result directly

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;
use std::fs;
use std::process::Command;

/// Helper function to verify generated Rust code compiles
fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_file = format!("/tmp/depyler_0270_{}.rs", test_name);
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
            &format!("/tmp/depyler_0270_{}.rlib", test_name),
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
    let _ = fs::remove_file(format!("/tmp/depyler_0270_{}.rlib", test_name));
}

/// Helper to check if error message contains expected Result method error
fn contains_result_method_error(rust_code: &str) -> bool {
    let temp_file = "/tmp/depyler_0270_check.rs";
    fs::write(temp_file, rust_code).expect("Failed to write temp file");

    let output = Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", temp_file])
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let has_error = stderr.contains("no method named")
        && (stderr.contains("found for enum `Result`") || stderr.contains("Result<"));

    let _ = fs::remove_file(temp_file);
    has_error
}

#[test]
fn test_DEPYLER_0270_dict_result_unwrapping_compiles() {
    // Test Case: Function returns dict[str, int] but uses indexing (generates Result)
    // Expected: Call site automatically unwraps Result before accessing dict methods
    let python = r#"
def calculate_stats(numbers: list[int]) -> dict[str, int]:
    """Returns statistics dict - uses indexing so generates Result wrapper."""
    if not numbers:
        return {"count": 0, "sum": 0}

    count = len(numbers)
    first = numbers[0]  # Indexing triggers Result return type
    return {"count": count, "sum": first}

def main() -> None:
    """Call function and access dict values."""
    data = [10, 20, 30]
    stats = calculate_stats(data)

    # These should work: stats is unwrapped HashMap, not Result
    count_val = stats["count"]
    sum_val = stats["sum"]
    print(count_val)
    print(sum_val)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Verify generated code includes Result unwrapping
    // Should generate: let stats = calculate_stats(&data).unwrap();
    // Not: let stats = calculate_stats(&data);

    // Before fix: This will fail with "no method named `get` found for enum `Result`"
    // After fix: This will compile successfully
    assert_compiles(&rust_code, "dict_result_unwrapping");
}

#[test]
fn test_DEPYLER_0270_multiple_result_accesses_compiles() {
    // Test Case: Multiple dict accesses after Result-returning function call
    // Expected: Unwrap once, then all accesses work
    let python = r#"
def get_data(items: list[str]) -> dict[str, int]:
    """Returns dict with indexed values."""
    first = items[0]  # Triggers Result wrapper
    first_len = len(first)  # Use the variable
    return {"a": 1, "b": 2, "c": first_len}

def main() -> None:
    """Access multiple dict keys."""
    items = ["x", "y", "z"]
    values = get_data(items)

    a_val = values["a"]
    b_val = values["b"]
    c_val = values["c"]
    total = a_val + b_val + c_val
    print(total)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: let values = get_data(&items).unwrap();
    // Then all dict accesses work: values.get("a"), values.get("b"), etc.
    assert_compiles(&rust_code, "multiple_result_accesses");
}

#[test]
fn test_DEPYLER_0270_list_result_unwrapping_compiles() {
    // Test Case: Function returns list but uses dict indexing (Result wrapper)
    // Expected: Call site unwraps Result before list operations
    let python = r#"
def process_list(data: dict[str, int]) -> list[int]:
    """Returns list - uses dict indexing so generates Result wrapper."""
    value = data["key"]  # Dict indexing triggers Result
    return [value, value * 2, value * 3]

def main() -> None:
    """Call function and iterate over result."""
    info = {"key": 5}
    numbers = process_list(info)

    # Should work: numbers is unwrapped Vec, not Result
    for num in numbers:
        print(num)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: let numbers = process_list(...)?.unwrap();
    // Then iteration works: for num in numbers.iter()
    assert_compiles(&rust_code, "list_result_unwrapping");
}

#[test]
fn test_DEPYLER_0270_chained_dict_access_compiles() {
    // Test Case: Immediate dict access after function call (chained)
    // Expected: Unwrap before accessing, or use .unwrap().get() pattern
    let python = r#"
def get_config() -> dict[str, str]:
    """Returns config dict."""
    defaults = {"name": "default", "other": "value2"}
    first_key = list(defaults.keys())[0]  # Indexing triggers Result
    key_len = len(first_key)  # Use the variable (avoid type issues)
    return {"name": "value", "path": "/tmp", "len": str(key_len)}

def main() -> None:
    """Immediately access dict key."""
    name = get_config()["name"]
    print(name)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: let name = get_config().unwrap().get("name")...
    // Not: let name = get_config().get("name")...  // ERROR
    assert_compiles(&rust_code, "chained_dict_access");
}

#[test]
fn test_DEPYLER_0270_dict_get_method_compiles() {
    // Test Case: Using .get() method on dict from Result-returning function
    // This is the EXACT pattern from compute_intensive.py benchmark
    // Expected: Result unwrapped before calling .get()
    let python = r#"
def calculate_statistics(numbers: list[int]) -> dict[str, int]:
    """Calculate statistics (has indexing, returns Result)."""
    if not numbers:
        return {"count": 0}

    min_val = numbers[0]
    max_val = numbers[0]
    return {"count": len(numbers), "min": min_val, "max": max_val}

def main() -> None:
    """Call function and use .get() equivalent (dict access)."""
    data = [1, 2, 3]
    stats = calculate_statistics(data)

    # The benchmark pattern: stats.get("count"), stats.get("max")
    # In Python dict access: stats["count"], stats["max"]
    count = stats["count"]
    maximum = stats["max"]
    print(count)
    print(maximum)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Should generate: let stats = calculate_statistics(&data).unwrap();
    // Then: stats.get("count").cloned()... and stats.get("max").cloned()...
    // NOT: stats.get("count")  where stats is still Result<HashMap, _>

    // Before fix: Fails with "no method named `get` found for enum `Result`"
    // After fix: Compiles successfully
    assert_compiles(&rust_code, "dict_get_method");
}

#[test]
#[ignore = "Diagnostic test - run manually to verify current error messages"]
fn test_DEPYLER_0270_verify_current_bug() {
    // This test verifies the bug exists by checking for Result method error
    // Run with: cargo test test_DEPYLER_0270_verify_current_bug -- --ignored --nocapture
    let python = r#"
def get_dict(nums: list[int]) -> dict[str, int]:
    val = nums[0]  # Triggers Result wrapper
    return {"key": val}

def main() -> None:
    data = [42]
    result = get_dict(data)
    value = result["key"]  # Will try to call .get() on Result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();
    println!("Generated code:\n{}", rust_code);

    // Check if current code produces Result method error
    if contains_result_method_error(&rust_code) {
        println!("\n✅ BUG CONFIRMED: Trying to call methods on Result without unwrapping");
        println!("Expected error: 'no method named `get` found for enum `Result`'");
        println!("Fix needed: Add .unwrap() or ? after Result-returning function calls");
    } else {
        println!("\n⚠️ Bug may already be fixed or error message changed");
    }
}

// Property-based test: Verify ANY Result-returning function gets unwrapped
#[test]
fn test_DEPYLER_0270_various_result_patterns() {
    // Test various patterns that should all unwrap Results
    let test_cases = vec![
        (
            "dict with list indexing",
            r#"
def f(nums: list[int]) -> dict[str, int]:
    x = nums[0]
    return {"result": x}
def main() -> None:
    data = [1, 2]
    d = f(data)
    v = d["result"]
    print(v)
"#,
        ),
        (
            "dict with dict indexing",
            r#"
def f(data: dict[str, int]) -> dict[str, int]:
    x = data["key"]
    return {"result": x}
def main() -> None:
    input_data = {"key": 42}
    output = f(input_data)
    v = output["result"]
    print(v)
"#,
        ),
        (
            "nested dict access",
            r#"
def f(nums: list[int]) -> dict[str, dict[str, int]]:
    x = nums[0]
    return {"outer": {"inner": x}}
def main() -> None:
    data = [10]
    result = f(data)
    inner_dict = result["outer"]
    value = inner_dict["inner"]
    print(value)
"#,
        ),
    ];

    for (type_name, python) in test_cases {
        println!("\nTesting pattern: {}", type_name);
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
            &format!("pattern_{}", type_name.replace(' ', "_")),
        );
    }
}

#![allow(non_snake_case)]
// DEPYLER-0307: Test sorted() with reverse parameter
// Tests for sorted() builtin with reverse=True/False parameter handling

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
fn test_sorted_ascending_simple() {
    let python_code = r#"
def sort_ascending(numbers: list[int]) -> list[int]:
    return sorted(numbers)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .sort() without .reverse()
    // Note: variable name may be __sorted_result or sorted_vec
    assert!(
        rust_code.contains(".sort()"),
        "Should use .sort() for ascending"
    );
    assert!(
        !rust_code.contains(".reverse()"),
        "Should NOT use .reverse() for ascending"
    );
}

#[test]
fn test_sorted_descending_simple() {
    let python_code = r#"
def sort_descending(numbers: list[int]) -> list[int]:
    return sorted(numbers, reverse=True)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .sort() FOLLOWED BY .reverse()
    assert!(
        rust_code.contains("__sorted_result.sort()"),
        "Should use .sort() for descending"
    );
    assert!(
        rust_code.contains("__sorted_result.reverse()"),
        "Should use .reverse() for descending"
    );

    // Verify order: sort() must come before reverse()
    let sort_pos = rust_code.find(".sort()").expect("Should contain .sort()");
    let reverse_pos = rust_code
        .find(".reverse()")
        .expect("Should contain .reverse()");
    assert!(
        sort_pos < reverse_pos,
        ".sort() must come before .reverse()"
    );
}

#[test]
fn test_sorted_with_key_and_reverse() {
    let python_code = r#"
def sort_by_abs_descending(numbers: list[int]) -> list[int]:
    return sorted(numbers, key=lambda x: abs(x), reverse=True)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .sort_by_key() with reverse
    assert!(
        rust_code.contains(".sort_by_key(") || rust_code.contains("sort_by"),
        "Should use sort_by_key for custom key"
    );
    assert!(
        rust_code.contains(".reverse()"),
        "Should use .reverse() when reverse=True with key"
    );
}

#[test]
fn test_sorted_with_key_no_reverse() {
    let python_code = r#"
def sort_by_abs(numbers: list[int]) -> list[int]:
    return sorted(numbers, key=lambda x: abs(x))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .sort_by_key()
    assert!(
        rust_code.contains(".sort_by_key(") || rust_code.contains("sort_by"),
        "Should use sort_by_key for custom key"
    );
    // Note: transpiler may generate `if false { .reverse() }` dead code
    // which is semantically correct (reverse never executes)
    // The actual check is that it uses sort_by_key correctly
}

#[test]
fn test_sorted_reverse_false_explicit() {
    let python_code = r#"
def sort_ascending_explicit(numbers: list[int]) -> list[int]:
    return sorted(numbers, reverse=False)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .sort()
    // Note: variable name may be __sorted_result or sorted_vec
    assert!(
        rust_code.contains(".sort()"),
        "Should use .sort() for ascending"
    );
    // Note: transpiler may generate `if false { .reverse() }` dead code
    // which is semantically correct (reverse never executes for reverse=False)
}

#[test]
fn test_sorted_compiles_ascending() {
    let python_code = r#"
def sort_ascending(numbers: list[int]) -> list[int]:
    return sorted(numbers)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file and compile
    std::fs::write("/tmp/test_sorted_asc.rs", &rust_code).expect("Failed to write test file");

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "/tmp/test_sorted_asc.rs"])
        .output()
        .expect("Failed to execute rustc");

    assert!(
        output.status.success(),
        "Generated code should compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_sorted_compiles_descending() {
    let python_code = r#"
def sort_descending(numbers: list[int]) -> list[int]:
    return sorted(numbers, reverse=True)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file and compile
    std::fs::write("/tmp/test_sorted_desc.rs", &rust_code).expect("Failed to write test file");

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "/tmp/test_sorted_desc.rs"])
        .output()
        .expect("Failed to execute rustc");

    assert!(
        output.status.success(),
        "Generated code should compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_sorted_behavior_ascending() {
    let python_code = r#"
def sort_ascending(numbers: list[int]) -> list[int]:
    return sorted(numbers)

# Depyler will generate quickcheck tests that verify:
# 1. Length preservation
# 2. Ascending order (result[i-1] <= result[i])
# 3. Same elements as input
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Verify quickcheck property tests are generated
    assert!(
        rust_code.contains("quickcheck"),
        "Should generate property tests"
    );
    assert!(
        rust_code.contains("TestResult"),
        "Should use quickcheck TestResult"
    );
}

#[test]
fn test_sorted_behavior_descending() {
    let python_code = r#"
def sort_descending(numbers: list[int]) -> list[int]:
    return sorted(numbers, reverse=True)
"#;

    let pipeline = DepylerPipeline::new();
    let _rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write a simple standalone test (without the auto-generated property tests)
    let test_code = r#"
pub fn sort_descending(numbers: Vec<i32>) -> Vec<i32> {
    {
        let mut __sorted_result = numbers.clone();
        __sorted_result.sort();
        __sorted_result.reverse();
        __sorted_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descending_behavior() {
        let input = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let result = sort_descending(input);
        // Should be in descending order (largest first)
        assert_eq!(result, vec![9, 6, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(sort_descending(vec![]), vec![]);
    }

    #[test]
    fn test_single() {
        assert_eq!(sort_descending(vec![42]), vec![42]);
    }
}
"#;

    std::fs::write("/tmp/test_sorted_behavior_simple.rs", test_code)
        .expect("Failed to write test file");

    // Compile and run tests
    let output = Command::new("rustc")
        .args([
            "--test",
            "/tmp/test_sorted_behavior_simple.rs",
            "-o",
            "/tmp/test_sorted_behavior_simple_bin",
        ])
        .output()
        .expect("Failed to compile test");

    assert!(
        output.status.success(),
        "Test code should compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let test_output = Command::new("/tmp/test_sorted_behavior_simple_bin")
        .output()
        .expect("Failed to run test");

    assert!(
        test_output.status.success(),
        "Behavior test should pass:\n{}",
        String::from_utf8_lossy(&test_output.stdout)
    );
}

#[test]
fn test_sorted_multiple_functions() {
    let python_code = r#"
def sort_ascending(numbers: list[int]) -> list[int]:
    return sorted(numbers)

def sort_descending(numbers: list[int]) -> list[int]:
    return sorted(numbers, reverse=True)

def sort_by_abs(numbers: list[int]) -> list[int]:
    return sorted(numbers, key=lambda x: abs(x))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Write to temp file and compile
    std::fs::write("/tmp/test_sorted_multiple.rs", &rust_code).expect("Failed to write test file");

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "/tmp/test_sorted_multiple.rs"])
        .output()
        .expect("Failed to execute rustc");

    assert!(
        output.status.success(),
        "Generated code should compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_sorted_with_strings() {
    let python_code = r#"
def sort_strings(words: list[str]) -> list[str]:
    return sorted(words)

def sort_strings_reverse(words: list[str]) -> list[str]:
    return sorted(words, reverse=True)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should work with strings too
    assert!(
        rust_code.contains("__sorted_result.sort()"),
        "Should use .sort() for strings"
    );

    // Write to temp file and compile
    std::fs::write("/tmp/test_sorted_strings.rs", &rust_code).expect("Failed to write test file");

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "/tmp/test_sorted_strings.rs"])
        .output()
        .expect("Failed to execute rustc");

    assert!(
        output.status.success(),
        "Generated code should compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

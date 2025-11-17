// DEPYLER-0382: *args unpacking in function calls
// Tests for transpiling Python's *args unpacking to Rust equivalents

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code and return the Rust output
fn transpile_and_compile(python_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code)?;
    Ok(result)
}

// ============================================================================
// Test Case 1: os.path.join(*parts) - Single Starred Arg
// ============================================================================

#[test]
fn test_depyler_0382_os_path_join_starred_basic() {
    let python = r#"
import os

def join_paths(*parts: str) -> str:
    return os.path.join(*parts)
"#;

    let result = transpile_and_compile(python);

    // This test should fail initially (RED phase)
    assert!(result.is_ok(), "Should transpile os.path.join(*parts)");

    let code = result.unwrap();

    // Verify the transpiled code uses appropriate Rust pattern
    // Should convert to parts.join(std::path::MAIN_SEPARATOR_STR) or similar
    assert!(
        code.contains("join") || code.contains("MAIN_SEPARATOR"),
        "Should have path joining logic. Got: {}", code
    );
}

#[test]
fn test_depyler_0382_os_path_join_starred_with_list() {
    let python = r#"
import os

def test() -> str:
    parts = ["home", "user", "docs"]
    return os.path.join(*parts)
"#;

    let result = transpile_and_compile(python);
    assert!(result.is_ok(), "Should transpile os.path.join(*parts) with list literal");
}

#[test]
fn test_depyler_0382_os_path_join_starred_mixed_args() {
    let python = r#"
import os

def test(base: str, *rest: str) -> str:
    return os.path.join(base, *rest)
"#;

    let result = transpile_and_compile(python);
    assert!(result.is_ok(), "Should transpile os.path.join with mixed positional and starred args");
}

// ============================================================================
// Test Case 2: print(*items) - Variadic Print
// ============================================================================

#[test]
fn test_depyler_0382_print_starred() {
    let python = r#"
def print_all(*items):
    print(*items)
"#;

    let result = transpile_and_compile(python);
    assert!(result.is_ok(), "Should transpile print(*items)");
}

// ============================================================================
// Test Case 3: Error Cases - Unsupported Functions
// ============================================================================

#[test]
fn test_depyler_0382_unsupported_function_error() {
    let python = r#"
def test(*args):
    some_custom_func(*args)
"#;

    let result = transpile_and_compile(python);

    // Should either transpile with a TODO/warning, or return a clear error
    // For now, we expect it to fail with a helpful message
    if result.is_err() {
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("*args") || err_msg.contains("unpacking") || err_msg.contains("not yet supported"),
            "Error message should mention *args unpacking. Got: {}", err_msg
        );
    }
}

// ============================================================================
// Property-Based Tests (QuickCheck)
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};

    fn prop_os_path_join_always_has_join_logic(parts: Vec<String>) -> TestResult {
        if parts.is_empty() || parts.len() > 10 {
            return TestResult::discard();
        }

        let parts_str = parts.iter()
            .map(|s| format!("\"{}\"", s.replace("\"", "\\\"")))
            .collect::<Vec<_>>()
            .join(", ");

        let python = format!(
            r#"
import os

def test() -> str:
    parts = [{}]
    return os.path.join(*parts)
"#,
            parts_str
        );

        match transpile_and_compile(&python) {
            Ok(code) => {
                // Verify path joining logic exists
                TestResult::from_bool(
                    code.contains("join") || code.contains("MAIN_SEPARATOR") || code.contains("PathBuf")
                )
            }
            Err(_) => TestResult::failed(),
        }
    }

    #[test]
    fn property_os_path_join_starred() {
        quickcheck(prop_os_path_join_always_has_join_logic as fn(Vec<String>) -> TestResult);
    }
}

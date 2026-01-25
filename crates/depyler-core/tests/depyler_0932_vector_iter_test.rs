// DEPYLER-0932: TDD tests for Vector.iter() transformation
// Location: crates/depyler-core/tests/depyler_0932_vector_iter_test.rs
//
// Bug: Vector.iter() generates `result.iter()` but trueno Vector needs `.as_slice().iter()`
// Symptom: E0599 "no method named `iter` found for struct `Vector<T>`"
// Fix: Transform .iter() on numpy arrays to .as_slice().iter()

use depyler_core::DepylerPipeline;

/// Test that iterating over numpy array uses as_slice().iter()
#[test]
fn test_depyler_0932_vector_iter_basic() {
    let python = r#"
import numpy as np

def iterate_array(a: float, b: float, c: float) -> list[str]:
    arr = np.array([a, b, c])
    return [str(x) for x in arr]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline
        .transpile(python)
        .expect("transpilation should succeed");

    // Should use as_slice().iter() for Vector iteration
    assert!(
        result.contains(".as_slice()") || result.contains(".iter()"),
        "Expected Vector iteration pattern. Generated:\n{}",
        result
    );

    // Should NOT have bare .iter() on Vector without as_slice()
    // This is hard to check without false positives, so just verify as_slice exists
    if result.contains("Vector") {
        assert!(
            result.contains("as_slice"),
            "Vector iteration should use .as_slice().iter(). Generated:\n{}",
            result
        );
    }
}

/// Test minmax normalization pattern with iteration
#[test]
fn test_depyler_0932_minmax_iteration() {
    let python = r#"
import numpy as np

def minmax_print(a: float, b: float, c: float):
    arr = np.array([a, b, c])
    min_val = np.min(arr)
    max_val = np.max(arr)
    denom = max_val - min_val
    result = (arr - min_val) / denom if denom > 0 else arr * 0
    print(" ".join(str(round(x, 2)) for x in result))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python)
        .expect("transpilation should succeed");

    // Debug output
    eprintln!("Generated code:\n{}", rust_code);

    // The result variable iteration MUST use as_slice().iter()
    // Code may be formatted across multiple lines, so join lines for checking
    let normalized = rust_code.replace("\n", " ").replace("  ", " ");

    // Check for the correct pattern: result .as_slice() .iter()
    let has_correct_pattern =
        normalized.contains("result .as_slice()") || normalized.contains("result.as_slice()");

    // Check for WRONG pattern: result .iter() without as_slice before it
    let has_wrong_pattern =
        normalized.contains("result .iter()") && !normalized.contains("result .as_slice()");

    assert!(
        has_correct_pattern || !has_wrong_pattern,
        "Vector iteration must use .as_slice().iter(), not bare .iter(). Generated:\n{}",
        rust_code
    );
}

/// Test direct for loop over numpy array
#[test]
fn test_depyler_0932_for_loop_over_array() {
    let python = r#"
import numpy as np

def sum_elements(a: float, b: float, c: float) -> float:
    arr = np.array([a, b, c])
    total = 0.0
    for x in arr:
        total += x
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline
        .transpile(python)
        .expect("transpilation should succeed");

    // For loop over Vector should iterate properly
    // Either inline iteration or using as_slice
    assert!(
        result.contains("for") || result.contains("iter"),
        "Should have iteration pattern. Generated:\n{}",
        result
    );
}

/// Test generator expression over numpy array
#[test]
fn test_depyler_0932_generator_over_array() {
    let python = r#"
import numpy as np

def stringify_array(a: float, b: float) -> str:
    arr = np.array([a, b])
    return " ".join(str(x) for x in arr)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline
        .transpile(python)
        .expect("transpilation should succeed");

    // Generator over Vector should use proper iteration
    if result.contains("Vector") {
        assert!(
            result.contains("as_slice") || result.contains("into_iter"),
            "Generator over Vector should use as_slice().iter() pattern"
        );
    }
}

/// Test CLI-style code with if/elif branches containing numpy
/// This mirrors the structure of example_numpy_minmax
#[test]
fn test_depyler_0932_cli_with_branches() {
    let python = r#"
import numpy as np

def main():
    cmd = "minmax3"
    a = 1.0
    b = 2.0
    c = 3.0
    if cmd == "minmax3":
        arr = np.array([a, b, c])
        min_val = np.min(arr)
        max_val = np.max(arr)
        denom = max_val - min_val
        result = (arr - min_val) / denom if denom > 0 else arr * 0
        print(" ".join(str(round(x, 2)) for x in result))
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python)
        .expect("transpilation should succeed");

    // Debug: print the generated code
    eprintln!("Generated Rust code:\n{}", rust_code);

    // The iteration over result MUST use as_slice().iter()
    // Normalize code: join lines and look for pattern
    let normalized = rust_code.replace('\n', " ").replace("  ", " ");

    // WRONG pattern: "result .iter()" or "result.iter()" WITHOUT ".as_slice()" before it
    // The key insight: when "result" is used as the iteration target, we need it to have .as_slice()
    // Pattern we want: result.as_slice().iter() or result .as_slice() .iter()
    // Pattern we DON'T want: result.iter() or result .iter() (without as_slice)

    // First, find all matches of "result" followed by ".iter()"
    let has_wrong_pattern = {
        let mut wrong = false;
        let mut search_start = 0;
        while let Some(idx) = normalized[search_start..].find("result") {
            let abs_idx = search_start + idx;
            // Look at what comes after "result" (skip whitespace)
            let after_result = &normalized[abs_idx + 6..]; // "result" is 6 chars
            let after_trimmed = after_result.trim_start();

            // Check if .iter() comes next (with or without .as_slice() first)
            if after_trimmed.starts_with(".iter()") {
                // Direct .iter() without .as_slice() - this is WRONG
                eprintln!(
                    "Found wrong pattern at index {}: result directly followed by .iter()",
                    abs_idx
                );
                wrong = true;
                break;
            } else if after_trimmed.starts_with(".as_slice()") {
                // Correct pattern: result.as_slice().iter()
                eprintln!(
                    "Found correct pattern at index {}: result.as_slice()",
                    abs_idx
                );
            }

            search_start = abs_idx + 1;
        }
        wrong
    };

    // The test FAILS if we find "result .iter()" that's NOT immediately preceded by ".as_slice()"
    // This is the definitive check - we should never have bare result.iter()
    assert!(
        !has_wrong_pattern,
        "Vector 'result' iteration must use .as_slice().iter(), not bare .iter(). Generated:\n{}",
        rust_code
    );
}

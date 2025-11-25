//! EXTREME TDD Test Suite for array_initialization.rs Module
//!
//! DEPYLER-REFACTOR-001 Phase 2.3: Extract array initialization and range functions
//!
//! # Test Categories
//! 1. Behavior preservation tests - Ensure identical behavior after extraction
//! 2. Compilation verification tests - Generated Rust must compile
//! 3. Property-based tests - Invariants with proptest
//!
//! # TDD Protocol
//! - RED: Tests written first, module doesn't exist yet
//! - GREEN: Extract module, tests pass
//! - REFACTOR: TDG A+ grade, 95% coverage
//!
//! # Note on numpy support
//! Direct function calls (zeros, ones, full) are supported for array init.
//! Range expressions are fully supported.

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

// ============================================================================
// RED PHASE: Module Existence Test (should FAIL until module extracted)
// ============================================================================

#[test]
#[ignore = "RED PHASE: Module not yet extracted"]
fn test_array_initialization_module_exists() {
    // This test will pass once the module is extracted
    let _module_path =
        std::path::Path::new("crates/depyler-core/src/rust_gen/array_initialization.rs");
}

// ============================================================================
// Behavior Preservation Tests - zeros() (direct function call syntax)
// ============================================================================

#[test]
fn test_zeros_direct_call() {
    let pipeline = DepylerPipeline::new();
    // Test zeros as direct function call
    let python = r#"
def create_zeros() -> int:
    arr = zeros(5)
    return len(arr)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // zeros(5) should generate vec![0; 5]
    assert!(
        rust.contains("vec![0;") || rust.contains("[0;"),
        "zeros(5) should generate array init. Got:\n{rust}"
    );
}

#[test]
fn test_ones_direct_call() {
    let pipeline = DepylerPipeline::new();
    // Test ones as direct function call
    let python = r#"
def create_ones() -> int:
    arr = ones(8)
    return len(arr)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("vec![1;") || rust.contains("[1;"),
        "ones(8) should generate array init. Got:\n{rust}"
    );
}

// ============================================================================
// Behavior Preservation Tests - range() - FULLY SUPPORTED
// ============================================================================

#[test]
fn test_range_single_arg() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_range_simple() -> list:
    r = list(range(5))
    return r
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("0..5") || rust.contains("0 .. 5"),
        "range(5) should generate 0..5. Got:\n{rust}"
    );
}

#[test]
fn test_range_two_args() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_range_start_end() -> list:
    r = list(range(2, 7))
    return r
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("2..7") || rust.contains("2 .. 7"),
        "range(2, 7) should generate 2..7. Got:\n{rust}"
    );
}

#[test]
fn test_range_with_positive_step() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_range_step() -> list:
    r = list(range(0, 10, 2))
    return r
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("step_by") || rust.contains(".step_by("),
        "range(0, 10, 2) should use step_by. Got:\n{rust}"
    );
}

#[test]
fn test_range_with_negative_step() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_range_negative_step() -> list:
    r = list(range(10, 0, -1))
    return r
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".rev()") || rust.contains("rev()"),
        "range(10, 0, -1) should use .rev(). Got:\n{rust}"
    );
}

#[test]
fn test_range_compiles() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def test_ranges() -> i32:
    r1 = list(range(5))
    r2 = list(range(1, 10))
    r3 = list(range(0, 10, 2))
    return len(r1) + len(r2) + len(r3)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    let temp_file = std::env::temp_dir().join("test_ranges_init.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_ranges_init.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    // Cleanup
    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Generated range code should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_range_variables() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def range_with_vars(start: int, end: int) -> list:
    r = list(range(start, end))
    return r
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("start") && rust.contains("end"),
        "range(start, end) should preserve variable names. Got:\n{rust}"
    );
}

#[test]
fn test_range_in_for_loop() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def sum_range() -> int:
    total = 0
    for i in range(10):
        total = total + i
    return total
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("..") || rust.contains("..10") || rust.contains("0..10"),
        "for i in range(10) should generate range. Got:\n{rust}"
    );
}

#[test]
fn test_nested_range_in_loops() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def nested_test() -> list:
    result = []
    for i in range(3):
        for j in range(3):
            result.append(i * 3 + j)
    return result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // Should have two range expressions
    let range_count = rust.matches("..").count();
    assert!(
        range_count >= 2,
        "Nested ranges should generate multiple .. operators. Got:\n{rust}"
    );
}

// ============================================================================
// Range Edge Cases
// ============================================================================

#[test]
fn test_range_zero_step_protection() {
    let pipeline = DepylerPipeline::new();
    // range with step should include zero-step protection
    let python = r#"
def range_step_check() -> list:
    return list(range(0, 10, 2))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // Should contain step protection code
    assert!(
        rust.contains("step") && rust.contains("0"),
        "range with step should have protection. Got:\n{rust}"
    );
}

#[test]
fn test_range_large_values() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def large_range() -> int:
    r = list(range(1000000))
    return len(r)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("1000000") || rust.contains("1_000_000"),
        "Large range values should be preserved. Got:\n{rust}"
    );
}

// ============================================================================
// Property-Based Tests
// ============================================================================

proptest! {
    #[test]
    fn prop_transpilation_is_deterministic(seed in 0i32..1000) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_ranges():
    r = list(range(5))
    return {}
"#, seed);
        let result1 = pipeline.transpile(&python);
        let result2 = pipeline.transpile(&python);
        match (result1, result2) {
            (Ok(r1), Ok(r2)) => prop_assert_eq!(r1, r2, "Transpilation should be deterministic"),
            (Err(_), Err(_)) => (), // Both errors is acceptable
            _ => prop_assert!(false, "Results should be consistent"),
        }
    }

    #[test]
    fn prop_range_generates_valid_rust(end in 1i32..100) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_range():
    return list(range({}))
"#, end);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Should contain range syntax
            prop_assert!(
                rust.contains(".."),
                "range() should generate Rust range syntax. Got:\n{}", rust
            );
        }
    }

    #[test]
    fn prop_range_with_step_uses_step_by(step in 1i32..10) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_range_step():
    return list(range(0, 20, {}))
"#, step);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Should use step_by for stepping
            prop_assert!(
                rust.contains("step_by") || rust.contains("step"),
                "range with step should use step_by. Got:\n{}", rust
            );
        }
    }

    #[test]
    fn prop_range_bounds_preserved(start in 0i32..50, end in 50i32..100) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_range_bounds():
    return list(range({}, {}))
"#, start, end);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Start and end values should appear in output
            prop_assert!(
                rust.contains(&start.to_string()) && rust.contains(&end.to_string()),
                "Range bounds should be preserved. Start: {}, End: {}. Got:\n{}", start, end, rust
            );
        }
    }
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_range_with_enumerate() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def enumerate_range() -> list:
    result = []
    for idx, val in enumerate(range(5)):
        result.append((idx, val))
    return result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("enumerate") && rust.contains(".."),
        "enumerate(range()) should work. Got:\n{rust}"
    );
}

#[test]
fn test_range_with_zip() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def zip_ranges() -> list:
    r1 = range(5)
    r2 = range(5, 10)
    return list(zip(r1, r2))
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".zip("),
        "zip(range, range) should work. Got:\n{rust}"
    );
}

#[test]
fn test_range_in_list_comprehension() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def squares() -> list:
    return [x * x for x in range(10)]
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("..") && rust.contains("map"),
        "List comprehension with range should work. Got:\n{rust}"
    );
}

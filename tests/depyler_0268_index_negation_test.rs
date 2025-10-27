// ============================================================================
// DEPYLER-0268: Index Negation Bug - Cannot Negate `usize`
// ============================================================================
// BUG: Negative index generates `(-idx) as usize` which fails because usize
// doesn't implement Neg trait - must use .unsigned_abs() or .abs() instead
//
// ROOT CAUSE: expr_gen.rs line ~2143 attempts to negate usize directly
// instead of converting signed int to unsigned offset using .unsigned_abs()
//
// FIX: Change `(-idx) as usize` to `idx.unsigned_abs() as usize`
//
// DISCOVERED: DEPYLER-0265 test suite (final failing test with String negative index)
// SEVERITY: P0 BLOCKING - prevents compilation of negative index access patterns
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0268_negative_index_last_item_compiles() {
    // DEPYLER-0268: `items[-1]` generates `(-idx) as usize` which fails
    // RED Phase: This test MUST FAIL initially with "cannot apply unary operator `-`"

    let python_code = r#"
def get_last(items: list[str]) -> str:
    """Get last item from list using negative index."""
    return items[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Debugging: Print generated code
    eprintln!("=== DEPYLER-0268: Generated Rust Code (negative index -1) ===");
    eprintln!("{}", rust_code);

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0268_negative_last.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0268: Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0268_negative_last.rlib")
        .output()
        .expect("DEPYLER-0268: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0268: rustc stderr (negative index -1) ===");
        eprintln!("{}", stderr);

        // ASSERT: Must NOT have "cannot apply unary operator `-`" error
        assert!(
            !stderr.contains("cannot apply unary operator `-`")
                && !stderr.contains("usize: Neg"),
            "DEPYLER-0268 FAILURE: Cannot negate usize!\n\
             Expected: Use .unsigned_abs() or .abs() to convert signed to unsigned offset\n\
             Actual: Generated (-idx) as usize which attempts to negate usize\n\
             \n\
             Error patterns:\n\
             - 'cannot apply unary operator `-` to type `usize`'\n\
             - 'the trait `Neg` is not implemented for `usize`'\n\
             \n\
             See docs/bugs/DEPYLER-0268.md for details.\n\
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
        "DEPYLER-0268: Negative index -1 should compile\n\
         Generated code:\n{}\n\
         Errors:\n{}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0268_negative_index_second_last_compiles() {
    // DEPYLER-0268: `items[-2]` for second-to-last item

    let python_code = r#"
def get_second_last(nums: list[int]) -> int:
    """Get second-to-last item using negative index."""
    return nums[-2]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0268_negative_second_last.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0268: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0268_negative_second_last.rlib")
        .output()
        .expect("DEPYLER-0268: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0268: rustc stderr (negative index -2) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("cannot apply unary operator `-`")
                && !stderr.contains("usize: Neg"),
            "DEPYLER-0268: Cannot negate usize!\n\
             Error: {}\n\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0268: Negative index -2 should compile\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0268_runtime_negative_index_compiles() {
    // DEPYLER-0268: Runtime negative index handling (both positive and negative)

    let python_code = r#"
def get_by_index(items: list[str], idx: int) -> str:
    """Get item by index - handles both positive and negative at runtime."""
    return items[idx]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0268_runtime_index.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0268: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0268_runtime_index.rlib")
        .output()
        .expect("DEPYLER-0268: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0268: rustc stderr (runtime index) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("cannot apply unary operator `-`")
                && !stderr.contains("usize: Neg"),
            "DEPYLER-0268: Runtime index negation error!\n\
             Error: {}\n\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0268: Runtime index should compile\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0268_nested_collection_negative_index_compiles() {
    // DEPYLER-0268: Negative index with nested collections (Vec<Vec<int>>)

    let python_code = r#"
def get_last_row(matrix: list[list[int]]) -> list[int]:
    """Get last row from 2D matrix using negative index."""
    return matrix[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0268_nested_negative.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0268: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0268_nested_negative.rlib")
        .output()
        .expect("DEPYLER-0268: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0268: rustc stderr (nested negative) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("cannot apply unary operator `-`")
                && !stderr.contains("usize: Neg"),
            "DEPYLER-0268: Nested collection negative index error!\n\
             Error: {}\n\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0268: Nested negative index should compile\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0268_positive_index_still_works() {
    // DEPYLER-0268: Ensure positive indices still work after fix (regression test)

    let python_code = r#"
def get_first(items: list[int]) -> int:
    """Get first item - positive index should still work."""
    return items[0]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0268_positive_index.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0268: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0268_positive_index.rlib")
        .output()
        .expect("DEPYLER-0268: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0268: rustc stderr (positive index) ===");
        eprintln!("{}", stderr);
    }

    assert!(
        output.status.success(),
        "DEPYLER-0268: Positive index should still work (regression test)\n\
         Code: {}\nErrors: {}",
        rust_code,
        stderr
    );
}

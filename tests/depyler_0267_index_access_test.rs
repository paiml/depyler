// ============================================================================
// DEPYLER-0267: Index Access Bug - `.copied()` Used for Non-Copy Types
// ============================================================================
// BUG: Index access generates `.copied()` for all types, but String doesn't
// implement Copy trait - should use `.cloned()` instead
//
// ROOT CAUSE: expr_gen.rs lines 2130 and 2146 use `.copied()` unconditionally
// for Vec/List index access, but HashMap access (lines 2102, 2110) correctly
// uses `.cloned()` for String keys
//
// FIX: Change `.copied()` to `.cloned()` for Vec/List index access
//
// DISCOVERED: DEPYLER-0265 test suite (final failing test with String list)
// SEVERITY: P0 BLOCKING - prevents compilation of index access on non-Copy types
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0267_string_list_index_compiles() {
    // DEPYLER-0267: `items[i]` for String list generates `.copied()` which fails
    // RED Phase: This test MUST FAIL initially

    let python_code = r#"
def get_string(items: list[str], index: int) -> str:
    """Get string from list by index."""
    return items[index]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Debugging: Print generated code
    eprintln!("=== DEPYLER-0267: Generated Rust Code (string index) ===");
    eprintln!("{}", rust_code);

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0267_string_index.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0267: Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0267_string_index.rlib")
        .output()
        .expect("DEPYLER-0267: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0267: rustc stderr (string index) ===");
        eprintln!("{}", stderr);

        // ASSERT: Must NOT have "String: Copy" error
        assert!(
            !stderr.contains("String: Copy"),
            "DEPYLER-0267 FAILURE: Cannot use .copied() on String!\\n\\
             Expected: .cloned() for non-Copy types\\n\\
             Actual: Generated .copied() which requires Copy trait\\n\\
             \\n\\
             Error pattern: 'the trait bound `String: Copy` is not satisfied'\\n\\
             \\n\\
             See docs/bugs/DEPYLER-0267.md for details.\\n\\
             \\n\\
             Generated Rust code:\\n{}\\n\\
             \\n\\
             rustc error:\\n{}",
            rust_code,
            stderr
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0267: String index should compile\\n\\
         Generated code:\\n{}\\n\\
         Errors:\\n{}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0267_vec_list_index_compiles() {
    // DEPYLER-0267: Nested list indexing (Vec<Vec<int>> → Vec<int>)

    let python_code = r#"
def get_row(matrix: list[list[int]], row: int) -> list[int]:
    """Get row from 2D matrix."""
    return matrix[row]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0267_vec_index.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0267: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0267_vec_index.rlib")
        .output()
        .expect("DEPYLER-0267: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0267: rustc stderr (vec index) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("Vec<i32>: Copy"),
            "DEPYLER-0267: Vec doesn't implement Copy!\\n\\
             Error: {}\\n\\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0267: Vec index should compile\\n\\
         Code: {}\\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0267_copy_types_still_work() {
    // DEPYLER-0267: Ensure Copy types (int, float) still work with fix

    let python_code = r#"
def get_int(nums: list[int], index: int) -> int:
    """Get int from list - Copy type."""
    return nums[index]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0267_copy_type.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0267: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0267_copy_type.rlib")
        .output()
        .expect("DEPYLER-0267: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0267: rustc stderr (copy type) ===");
        eprintln!("{}", stderr);
    }

    assert!(
        output.status.success(),
        "DEPYLER-0267: Copy type (int) should still work\\n\\
         Code: {}\\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0267_negative_index_string_compiles() {
    // DEPYLER-0267: Negative index with String list
    // NOTE: This also has DEPYLER-0268 bug (index negation), so expect different error

    let python_code = r#"
def get_last_string(items: list[str]) -> str:
    """Get last string from list using negative index."""
    return items[-1]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0267_negative_string.rs";
    std::fs::write(temp_file, &rust_code).expect("DEPYLER-0267: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0267_negative_string.rlib")
        .output()
        .expect("DEPYLER-0267: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0267: rustc stderr (negative string) ===");
        eprintln!("{}", stderr);

        // DEPYLER-0267: Must NOT have String: Copy error
        assert!(
            !stderr.contains("String: Copy"),
            "DEPYLER-0267: Should use .cloned() not .copied()!\\n\\
             Error: {}\\n\\
             Code: {}",
            stderr,
            rust_code
        );
    }

    // NOTE: This test will still fail due to DEPYLER-0268 (index negation bug)
    // But it should NOT fail due to .copied() on String
    // We verify that by checking the error message doesn't contain "String: Copy"
}

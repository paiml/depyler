// ============================================================================
// DEPYLER-0266: Boolean Conversion Bug - Cannot Use `!` on Borrowed Collections
// ============================================================================
// BUG: Python `if not collection:` generates `if !collection` which fails
// because Rust's `!` operator requires bool type, but collections are &Vec<T>
//
// ROOT CAUSE: UnaryOp::Not expression generation doesn't check if operand
// is a collection type - should use .is_empty() instead
//
// FIX: Detect collection types in Not operator and generate .is_empty()
//
// DISCOVERED: DEPYLER-0265 test suite (blocked 3 of 4 tests)
// SEVERITY: P0 BLOCKING - prevents compilation of empty collection checks
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0266_list_empty_check_compiles() {
    // DEPYLER-0266: `if not items:` generates `if !items` which fails
    // RED Phase: This test MUST FAIL initially

    let python_code = r#"
def is_empty_list(items: list[int]) -> bool:
    """Check if list is empty."""
    if not items:
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Debugging: Print generated code
    eprintln!("=== DEPYLER-0266: Generated Rust Code (list) ===");
    eprintln!("{}", rust_code);

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0266_list.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0266: Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0266_list.rlib")
        .output()
        .expect("DEPYLER-0266: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0266: rustc stderr (list) ===");
        eprintln!("{}", stderr);

        // ASSERT: Must NOT have "cannot apply unary operator `!`" error
        assert!(
            !stderr.contains("cannot apply unary operator"),
            "DEPYLER-0266 FAILURE: Cannot use ! operator on collection!\\n\\\
             Expected: `if items.is_empty()` or similar\\n\\\
             Actual: Generated `if !items` which is invalid Rust\\n\\\
             \\n\\\
             Error pattern: 'cannot apply unary operator `!` to type'\\n\\\
             \\n\\\
             See docs/bugs/DEPYLER-0266.md for details.\\n\\\
             \\n\\\
             Generated Rust code:\\n{}\\n\\\
             \\n\\\
             rustc error:\\n{}",
            rust_code,
            stderr
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0266: Compilation should succeed\\n\\\
         Generated code:\\n{}\\n\\\
         Errors:\\n{}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0266_string_empty_check_compiles() {
    // DEPYLER-0266: `if not text:` generates `if !text` for strings

    let python_code = r#"
def is_empty_string(text: str) -> bool:
    """Check if string is empty."""
    if not text:
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0266_string.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0266: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0266_string.rlib")
        .output()
        .expect("DEPYLER-0266: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0266: rustc stderr (string) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("cannot apply unary operator"),
            "DEPYLER-0266: Boolean conversion failed for string!\\n\\\
             Error: {}\\n\\\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0266: String empty check should compile\\n\\\
         Code: {}\\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0266_dict_empty_check_compiles() {
    // DEPYLER-0266: `if not mapping:` for dict types

    let python_code = r#"
def is_empty_dict(mapping: dict[str, int]) -> bool:
    """Check if dict is empty."""
    if not mapping:
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0266_dict.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0266: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0266_dict.rlib")
        .output()
        .expect("DEPYLER-0266: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0266: rustc stderr (dict) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("cannot apply unary operator"),
            "DEPYLER-0266: Boolean conversion failed for dict!\\n\\\
             Error: {}\\n\\\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0266: Dict empty check should compile\\n\\\
         Code: {}\\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0266_guard_clause_pattern_compiles() {
    // DEPYLER-0266: Common guard clause pattern with early return

    let python_code = r#"
def process_items(items: list[int]) -> int:
    """Process items with guard clause."""
    if not items:
        return 0
    total = 0
    for item in items:
        total = total + item
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0266_guard.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0266: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0266_guard.rlib")
        .output()
        .expect("DEPYLER-0266: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0266: rustc stderr (guard) ===");
        eprintln!("{}", stderr);

        assert!(
            !stderr.contains("cannot apply unary operator"),
            "DEPYLER-0266: Guard clause pattern failed!\\n\\\
             Error: {}\\n\\\
             Code: {}",
            stderr,
            rust_code
        );
    }

    assert!(
        output.status.success(),
        "DEPYLER-0266: Guard clause should compile\\n\\\
         Code: {}\\nErrors: {}",
        rust_code,
        stderr
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0266_boolean_not_still_works() {
    // DEPYLER-0266: Ensure regular boolean `not` still works correctly

    let python_code = r#"
def negate_bool(flag: bool) -> bool:
    """Negate a boolean."""
    if not flag:
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust_code = result.unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0266_bool.rs";
    std::fs::write(temp_file, &rust_code)
        .expect("DEPYLER-0266: Failed to write temp file");

    // Attempt to compile
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0266_bool.rlib")
        .output()
        .expect("DEPYLER-0266: Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0266: rustc stderr (bool) ===");
        eprintln!("{}", stderr);
    }

    assert!(
        output.status.success(),
        "DEPYLER-0266: Boolean not should still work\\n\\\
         Code: {}\\nErrors: {}",
        rust_code,
        stderr
    );
}

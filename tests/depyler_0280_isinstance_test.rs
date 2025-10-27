// ============================================================================
// DEPYLER-0269: isinstance() Generates Invalid Rust Code
// ============================================================================
// BUG: isinstance(value, int) generates literal isinstance(value, int)
// which fails to compile in Rust (isinstance and int don't exist)
//
// ROOT CAUSE: expr_gen.rs:366 convert_call() has NO handler for isinstance()
//
// FIX: Add isinstance() handler that returns `true` (type system guarantees)
//
// DISCOVERED: Matrix-Testing Column A → B (01_basic_types)
// SEVERITY: P0 BLOCKING - prevents compilation
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_int_removed() {
    // RED Phase: This test MUST FAIL initially with isinstance in output
    let python = r#"
def check_int(x: int) -> bool:
    return isinstance(x, int)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust = result.unwrap();

    // Debugging: Print generated code
    eprintln!("=== DEPYLER-0269: Generated Rust Code ===");
    eprintln!("{}", rust);

    // ASSERT: Should NOT contain isinstance
    assert!(
        !rust.contains("isinstance"),
        "DEPYLER-0269 FAILURE: isinstance should be removed!\n\
         Expected: true\n\
         Actual: isinstance(x, int)\n\
         \n\
         Generated code:\n{}",
        rust
    );

    // ASSERT: Should return true (type system guarantees)
    assert!(
        rust.contains("true"),
        "DEPYLER-0269 FAILURE: Should return true!\n\
         Type system guarantees that x: i32 is always int\n\
         \n\
         Generated code:\n{}",
        rust
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_str_removed() {
    // RED Phase: isinstance(s, str) should also be removed
    let python = r#"
def check_str(s: str) -> bool:
    return isinstance(s, str)
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("isinstance"), "isinstance should be removed");
    assert!(rust.contains("true"), "Should return true");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_compiles() {
    // RED Phase: Generated code should compile (will fail until fix)
    let python = r#"
def check_type(value: int) -> bool:
    return isinstance(value, int)
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0269.rs";
    std::fs::write(temp_file, &rust).expect("Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0269.rlib")
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\n=== DEPYLER-0269: rustc stderr ===");
        eprintln!("{}", stderr);
    }

    assert!(
        output.status.success(),
        "DEPYLER-0269: Generated code must compile!\n\
         \n\
         Expected: Code compiles without errors\n\
         Actual: Compilation failed\n\
         \n\
         Errors:\n{}\n\
         \n\
         Generated code:\n{}",
        stderr,
        rust
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_multiple_types() {
    // RED Phase: Multiple isinstance calls should all be removed
    let python = r#"
def check_types(x: int, s: str) -> bool:
    return isinstance(x, int) and isinstance(s, str)
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    // Both isinstance calls should be removed
    assert!(!rust.contains("isinstance"), "All isinstance calls should be removed");

    // Should be: true && true (or optimized to true)
    let true_count = rust.matches("true").count();
    assert!(
        true_count >= 2,
        "Should have at least 2 'true' values (one per isinstance)\n\
         Found: {} occurrences\n\
         Generated code:\n{}",
        true_count,
        rust
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_in_if_statement() {
    // RED Phase: isinstance in if statement should work
    let python = r#"
def validate(data: str) -> bool:
    if isinstance(data, str):
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("isinstance"), "isinstance should be removed");
    assert!(rust.contains("true"), "Should contain true");

    // Should compile
    let temp_file = "/tmp/test_depyler_0269_if.rs";
    std::fs::write(temp_file, &rust).unwrap();

    let output = Command::new("rustc")
        .args(&["--crate-type", "lib", "--edition", "2021", temp_file])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Code with isinstance in if statement should compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_with_float() {
    // RED Phase: isinstance with float type
    let python = r#"
def check_float(x: float) -> bool:
    return isinstance(x, float)
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("isinstance"), "isinstance should be removed");
    assert!(rust.contains("true"), "Should return true");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0269_isinstance_with_bool() {
    // RED Phase: isinstance with bool type
    let python = r#"
def check_bool(b: bool) -> bool:
    return isinstance(b, bool)
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("isinstance"), "isinstance should be removed");
    assert!(rust.contains("true"), "Should return true");
}

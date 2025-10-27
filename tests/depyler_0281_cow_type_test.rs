// ============================================================================
// DEPYLER-0270: Cow<'static, str> Type Inference Bug
// ============================================================================
// BUG: String concatenation generates Cow<'static, str> return type
// which doesn't match format!() return type (String)
//
// ROOT CAUSE: func_gen.rs:505-513 codegen_return_type() sets uses_cow_return
// without checking if function body uses format!() or string concatenation
//
// FIX: Detect string concatenation and force String return type
//
// DISCOVERED: Matrix-Testing Column A → B (01_basic_types)
// SEVERITY: P0 BLOCKING - prevents compilation
// ============================================================================

use depyler_core::DepylerPipeline;
use std::process::Command;

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0270_string_concat_returns_string() {
    // RED Phase: This test MUST FAIL initially with Cow in return type
    let python = r#"
def concat(a: str, b: str) -> str:
    return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation should succeed");
    let rust = result.unwrap();

    // Debugging: Print generated code
    eprintln!("=== DEPYLER-0270: Generated Rust Code ===");
    eprintln!("{}", rust);

    // ASSERT: Should NOT contain Cow in return type
    assert!(
        !rust.contains("-> Cow<"),
        "DEPYLER-0270 FAILURE: Should not use Cow for string concatenation!\\n\
         Expected: -> String\\n\
         Actual: -> Cow<'static, str>\\n\
         \\n\
         Generated code:\\n{}",
        rust
    );

    // ASSERT: Should return String
    assert!(
        rust.contains("-> String"),
        "DEPYLER-0270 FAILURE: Should return String!\\n\
         String concatenation always produces owned String\\n\
         \\n\
         Generated code:\\n{}",
        rust
    );

    // ASSERT: Should use &str parameters (not Cow)
    let has_str_params = rust.contains("a: &str") && rust.contains("b: &str");
    assert!(
        has_str_params,
        "DEPYLER-0270 FAILURE: Should use &str parameters!\\n\
         Expected: a: &str, b: &str\\n\
         \\n\
         Generated code:\\n{}",
        rust
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0270_string_concat_compiles() {
    // RED Phase: Generated code should compile (will fail until fix)
    let python = r#"
def concat(a: str, b: str) -> str:
    return a + b
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    // Write to temp file
    let temp_file = "/tmp/test_depyler_0270_concat.rs";
    std::fs::write(temp_file, &rust).expect("Failed to write temp file");

    // Attempt to compile with rustc
    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/test_depyler_0270_concat.rlib")
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("\\n=== DEPYLER-0270: rustc stderr ===");
        eprintln!("{}", stderr);
    }

    assert!(
        output.status.success(),
        "DEPYLER-0270: Generated code must compile!\\n\
         \\n\
         Expected: Code compiles without errors\\n\
         Actual: Compilation failed\\n\
         \\n\
         Errors:\\n{}\\n\
         \\n\
         Generated code:\\n{}",
        stderr,
        rust
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0270_fstring_returns_string() {
    // RED Phase: F-strings should also return String, not Cow
    let python = r#"
def format_name(first: str, last: str) -> str:
    return f"{first} {last}"
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("-> Cow<"), "F-string should not return Cow");
    assert!(rust.contains("-> String"), "F-string should return String");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0270_format_call_returns_string() {
    // RED Phase: Explicit format() calls should return String
    let python = r#"
def format_msg(name: str, count: int) -> str:
    return "{} has {}".format(name, count)
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("-> Cow<"), "format() should not return Cow");
    assert!(rust.contains("-> String"), "format() should return String");
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0270_multiple_concat_returns_string() {
    // RED Phase: Multiple string concatenations should return String
    let python = r#"
def concat_three(a: str, b: str, c: str) -> str:
    return a + b + c
"#;

    let pipeline = DepylerPipeline::new();
    let rust = pipeline.transpile(python).unwrap();

    assert!(!rust.contains("-> Cow<"), "Multi-concat should not return Cow");
    assert!(rust.contains("-> String"), "Multi-concat should return String");
}

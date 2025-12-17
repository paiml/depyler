//! DEPYLER-0970: Return Type Inference from Body Tests
//!
//! This test module validates that return types are correctly inferred
//! from function bodies when no type annotation is provided.
//!
//! Key scenario:
//! - `def run_command(...): return result.returncode, result.stdout, result.stderr`
//! - Expected return type: `(i32, String, String)` not `(serde_json::Value, String, String)`

use depyler_core::DepylerPipeline;

#[test]
fn test_subprocess_return_type_inference() {
    // Simplified version of the subprocess pattern
    let python = r#"
import subprocess

def run_command(cmd: list[str]) -> tuple[int, str, str]:
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.returncode, result.stdout, result.stderr
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // With explicit type annotation, should have correct return type
    assert!(
        rust_code.contains("-> (i32, String, String)") || rust_code.contains("-> (i64, String, String)"),
        "Return type should be (i32, String, String) with explicit annotation\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_subprocess_return_type_inference_no_annotation() {
    // WITHOUT explicit type annotation - this is the failing case
    let python = r#"
import subprocess

def run_command(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.returncode, result.stdout, result.stderr
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    println!("Generated Rust code:\n{}", rust_code);

    // Should NOT have serde_json::Value in return type
    assert!(
        !rust_code.contains("serde_json::Value, String, String"),
        "Return type should NOT contain serde_json::Value\n\nGenerated:\n{}",
        rust_code
    );

    // Should have i32 for returncode (first tuple element)
    assert!(
        rust_code.contains("-> (i32,") || rust_code.contains("-> (i64,"),
        "Return type should start with (i32, or (i64,\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_attribute_type_inference_returncode() {
    // Test that result.returncode is inferred as i32
    let python = r#"
import subprocess

def get_exit_code(cmd: list[str]) -> int:
    result = subprocess.run(cmd)
    return result.returncode
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should have i32 return type, not serde_json::Value
    assert!(
        rust_code.contains("-> i32") || rust_code.contains("-> i64"),
        "Return type should be i32 or i64\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_tuple_return_with_mixed_sources() {
    // Test that tuple returns from different sources are correctly typed
    let python = r#"
def get_stats(n: int) -> tuple[int, str, bool]:
    count = n * 2
    name = "test"
    done = n > 10
    return count, name, done
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should correctly infer tuple type from body
    assert!(
        rust_code.contains("-> (i32, String, bool)") || rust_code.contains("-> (i64, String, bool)"),
        "Return type should be inferred from body expressions\n\nGenerated:\n{}",
        rust_code
    );
}

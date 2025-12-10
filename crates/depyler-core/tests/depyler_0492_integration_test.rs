//! DEPYLER-0492 Integration Test: Verify type inference on real-world code

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

#[test]
fn test_subprocess_type_inference_integration() {
    let python = r#"
import subprocess

def run_command(cmd, capture=False):
    result = subprocess.run(cmd, capture_output=capture)
    return result.returncode
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Type inference should work:
    // - cmd: &Vec<String> (from subprocess.run signature)
    // - capture: bool (from default value False)
    assert!(
        rust.contains("cmd: &Vec<String>") || rust.contains("cmd: Vec<String>"),
        "cmd should be inferred as Vec<String>, got:\n{}",
        rust
    );

    assert!(
        rust.contains("capture: bool"),
        "capture should be inferred as bool, got:\n{}",
        rust
    );

    assert!(
        !rust.contains("serde_json::Value"),
        "Should not contain any serde_json::Value types, got:\n{}",
        rust
    );
}

#[test]
fn test_indexing_type_inference_integration() {
    let python = r#"
def get_first(items):
    return items[0]

def get_rest(items):
    return items[1:]
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Type inference should infer Vec<T> from indexing/slicing
    let has_vec_type = rust.contains("items: &Vec<") || rust.contains("items: Vec<");

    assert!(
        has_vec_type,
        "items should be inferred as Vec<T>, got:\n{}",
        rust
    );
}

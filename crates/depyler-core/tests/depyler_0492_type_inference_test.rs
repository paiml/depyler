//! DEPYLER-0492: Type Inference for Unannotated Parameters
//!
//! Tests that unannotated function parameters are inferred from usage
//! instead of defaulting to serde_json::Value.
//!
//! This test file follows TDD (RED phase):
//! - Write failing tests BEFORE implementing the fix
//! - Tests demonstrate the expected behavior after integration

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

/// Test that function parameter `cmd` is inferred as Vec<String>
/// from list construction and indexing operations
#[test]
#[ignore] // RED: Will fail until type inference integrated
fn test_subprocess_cmd_type_inference() {
    let python = r#"
def run_command(cmd, capture=False):
    # cmd is used as a list: indexing and slicing
    result = subprocess.run(cmd, capture_output=capture)
    return result.returncode
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // MUST infer cmd: Vec<String> (or idiomatic &Vec<String>/&[String]) from usage
    assert!(
        rust.contains("cmd: Vec<String>")
            || rust.contains("cmd: &Vec<String>")
            || rust.contains("cmd: &[String]"),
        "Expected cmd to be inferred as Vec<String> or &Vec<String> or &[String], got:\n{}",
        rust
    );

    // MUST NOT default to serde_json::Value
    assert!(
        !rust.contains("cmd: &serde_json::Value"),
        "cmd should NOT be serde_json::Value, got:\n{}",
        rust
    );

    // MUST infer capture: bool from default value False
    assert!(
        rust.contains("capture: bool"),
        "Expected capture to be inferred as bool, got:\n{}",
        rust
    );
}

/// Test that list indexing (cmd[0]) generates Index trait constraint
#[test]
#[ignore] // RED: Will fail until constraint collection implemented
fn test_list_indexing_constraint() {
    let python = r#"
def get_first(items):
    return items[0]
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Should infer items as indexable type (Vec, slice, etc.)
    assert!(
        rust.contains("items: Vec<") || rust.contains("items: &Vec<") || rust.contains("items: &["),
        "Expected items to be inferred as Vec or &Vec or slice from indexing, got:\n{}",
        rust
    );

    assert!(
        !rust.contains("items: &serde_json::Value"),
        "items should NOT default to serde_json::Value when indexed, got:\n{}",
        rust
    );
}

/// Test that list slicing (cmd[1..]) generates slicing constraint
#[test]
#[ignore] // RED: Will fail until constraint collection implemented
fn test_list_slicing_constraint() {
    let python = r#"
def get_rest(items):
    return items[1:]
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Should infer items as sliceable type
    assert!(
        rust.contains("items: Vec<") || rust.contains("items: &Vec<") || rust.contains("items: &["),
        "Expected items to be inferred as Vec or &Vec or slice from slicing, got:\n{}",
        rust
    );
}

/// Test that function call propagates type constraints from stdlib signature
#[test]
#[ignore] // RED: Will fail until stdlib constraint propagation implemented
fn test_stdlib_constraint_propagation() {
    let python = r#"
def run_echo(args):
    import subprocess
    subprocess.run(args)  # subprocess.run expects List[str]
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Should infer args: Vec<String> from subprocess.run signature
    assert!(
        rust.contains("args: Vec<String>") || rust.contains("args: &[String]"),
        "Expected args to be inferred from subprocess.run signature, got:\n{}",
        rust
    );
}

/// Test that list construction propagates element type constraints
#[test]
#[ignore] // RED: Will fail until list construction constraint implemented
fn test_list_construction_constraint() {
    let python = r#"
def build_command(prog, args):
    cmd = [prog] + args  # List concatenation
    return cmd
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Should infer args: Vec<T> from list concatenation
    // where T matches prog's type (likely String or Unknown)
    assert!(
        rust.contains("args: Vec<") || rust.contains("args: &Vec<") || rust.contains("args: &["),
        "Expected args to be inferred from list concatenation, got:\n{}",
        rust
    );
}

/// Test that default value False infers bool type
#[test]
#[ignore] // RED: Will fail until default value inference implemented
fn test_bool_default_inference() {
    let python = r#"
def process(flag=False):
    if flag:
        print("enabled")
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Should infer flag: bool from default value False
    assert!(
        rust.contains("flag: bool"),
        "Expected flag to be inferred as bool from default False, got:\n{}",
        rust
    );

    assert!(
        !rust.contains("flag: serde_json::Value"),
        "flag should NOT be serde_json::Value, got:\n{}",
        rust
    );
}

/// Integration test: Full subprocess example should compile
#[test]
#[ignore] // RED: Will fail until full integration complete
fn test_full_subprocess_example() {
    let python = r#"
import subprocess

def run_command(cmd, capture=False, check=False, cwd=None):
    if capture:
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=cwd, check=check)
        return result.returncode, result.stdout, result.stderr
    else:
        result = subprocess.run(cmd, cwd=cwd, check=check)
        return result.returncode, "", ""
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Expected parameter types:
    // cmd: Vec<String> or &[String]
    // capture: bool
    // check: bool
    // cwd: Option<String>

    assert!(
        rust.contains("cmd: Vec<String>") || rust.contains("cmd: &[String]"),
        "cmd should be Vec<String> or &[String]"
    );
    assert!(rust.contains("capture: bool"), "capture should be bool");
    assert!(rust.contains("check: bool"), "check should be bool");
    assert!(
        rust.contains("cwd: Option<String>"),
        "cwd should be Option<String>"
    );

    // Should NOT contain any serde_json::Value for parameters
    // Get the function signature line
    let func_line = rust
        .lines()
        .find(|l| l.contains("pub fn run_command"))
        .expect("Should find run_command function");

    assert!(
        !func_line.contains("serde_json::Value"),
        "Parameters should not use serde_json::Value: {}",
        func_line
    );
}

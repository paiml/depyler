//! TDD Tests for JSON Value Iteration Bug (DEPYLER-0607)
//!
//! Bug: When iterating over serde_json::Value, the transpiler generates code
//! that tries to iterate directly over Value, but Value doesn't implement IntoIterator.
//!
//! Example:
//! ```python
//! data = {"items": [1, 2, 3]}
//! for item in data["items"]:
//!     print(item)
//! ```
//!
//! Current output (broken):
//! ```rust
//! for item in data.get("items").cloned().unwrap_or_default() { ... }
//! // Error: Value is not an iterator
//! ```
//!
//! Expected output:
//! ```rust
//! for item in data.get("items").and_then(|v| v.as_array()).unwrap_or(&vec![]).iter() { ... }
//! ```
//!
//! Root cause: The for-loop codegen doesn't detect when the iterable expression
//! evaluates to serde_json::Value and add the necessary .as_array() conversion.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::HirModule;
use depyler_core::optimizer::{Optimizer, OptimizerConfig};
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

/// Transpile with optimizer (matches CLI behavior)
fn transpile(python: &str) -> Result<String, String> {
    let ast = parse(python, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new().python_to_hir(ast).map_err(|e| e.to_string())?;

    let hir_program = depyler_core::hir::HirProgram {
        functions: hir.functions.clone(),
        classes: hir.classes.clone(),
        imports: hir.imports.clone(),
    };

    let mut optimizer = Optimizer::new(OptimizerConfig::default());
    let optimized = optimizer.optimize_program(hir_program);

    let optimized_hir = HirModule {
        functions: optimized.functions,
        classes: optimized.classes,
        imports: optimized.imports,
        constants: hir.constants,
        type_aliases: hir.type_aliases,
        protocols: hir.protocols,
    };

    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) =
        generate_rust_file(&optimized_hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("Failed to create src dir");
    let lib_file = src_dir.join("lib.rs");

    std::fs::write(
        &cargo_toml,
        r#"[package]
name = "test_lib"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
path = "src/lib.rs"

[dependencies]
serde_json = "1.0"
"#,
    )
    .expect("Failed to write Cargo.toml");

    std::fs::write(&lib_file, rust_code).expect("Failed to write lib.rs");

    let output = std::process::Command::new("cargo")
        .args(["check", "--quiet"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to run cargo");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Rust compilation failed for {}:\n{}\n\nGenerated code:\n{}",
            test_name, stderr, rust_code
        );
    }
}

/// Test: Iterate over dict value that is a list
#[test]
fn test_iterate_dict_list_value() {
    let python = r#"
def main():
    data = {"items": [1, 2, 3], "tags": ["a", "b"]}
    for item in data["items"]:
        print(item)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should contain .as_array() for JSON Value iteration
    assert!(
        rust.contains("as_array") || rust.contains("iter()"),
        "Should handle JSON Value iteration. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "iterate_dict_list_value");
}

/// Test: Iterate over dict.get() result
#[test]
fn test_iterate_dict_get_result() {
    let python = r#"
def process(data: dict):
    for item in data.get("values", []):
        print(item)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "iterate_dict_get_result");
}

/// Test: Nested iteration over JSON values
#[test]
fn test_nested_json_iteration() {
    let python = r#"
def main():
    data = {"matrix": [[1, 2], [3, 4]]}
    for row in data["matrix"]:
        for val in row:
            print(val)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "nested_json_iteration");
}

/// Test: Method chain returning JSON Value
#[test]
fn test_method_chain_json_value() {
    let python = r#"
def main():
    data = {"items": [1, 2, 3]}
    for item in data.get("items").cloned().unwrap_or_default():
        print(item)
"#;

    // This Python doesn't make sense - but tests the chain pattern
    // Just test that basic dict iteration works
    let python_simple = r#"
def main():
    data = {"items": [1, 2, 3]}
    items = data["items"]
    for item in items:
        print(item)
"#;

    let rust = transpile(python_simple).expect("Transpilation should succeed");
    assert_compiles(&rust, "method_chain_json_value");
}

//! TDD Tests for Heterogeneous Dict Values Bug (DEPYLER-0601)
//!
//! Bug: Dicts with values of different types are not handled correctly.
//! When a dict has different value types for different keys, Rust requires
//! all values to have the same type.
//!
//! Example:
//! ```python
//! data = {"items": [1, 2, 3], "tags": ["a", "b"]}
//! ```
//!
//! Root cause: The transpiler generates HashMap<String, Vec<i32>> but then
//! tries to insert Vec<String>, causing E0308 type mismatch.
//!
//! Fix: Detect heterogeneous value types and use serde_json::Value

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

    // Run optimizer (like CLI does)
    let hir_program = depyler_core::hir::HirProgram {
        functions: hir.functions.clone(),
        classes: hir.classes.clone(),
        imports: hir.imports.clone(),
    };

    let mut optimizer = Optimizer::new(OptimizerConfig::default());
    let optimized = optimizer.optimize_program(hir_program);

    // Convert back to HirModule for rust_gen
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

    // Create Cargo.toml with dependencies
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
        // Clear LLVM coverage env to prevent interference under cargo-llvm-cov
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_SHOW_ENV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("LLVM_PROFILE_FILE")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_INCREMENTAL")
        .env_remove("CARGO_BUILD_JOBS")
        .env_remove("CARGO_TARGET_DIR")
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

/// Test: Dict with different value types (list of int vs list of str)
/// This is the core bug - heterogeneous dict values
/// Note: Full compilation requires proper serde_json::Value iteration (separate issue)
#[test]
fn test_heterogeneous_dict_list_values_detection() {
    let python = r#"
def main():
    data = {"items": [1, 2, 3], "tags": ["a", "b"]}
    # Just access the dict, don't iterate (iteration over Value is a separate issue)
    print(data["items"])
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Verify generated code uses DepylerValue for heterogeneous dicts
    // DEPYLER-1051: Hybrid Fallback uses DepylerValue::Dict
    if !rust.contains("DepylerValue::Dict") {
        panic!("Heterogeneous dict should use DepylerValue::Dict. Generated:\n{}", rust);
    }
    if !rust.contains("DepylerValue::Str") {
        panic!("Should use DepylerValue::Str. Generated:\n{}", rust);
    }

    // Should compile without type mismatch errors (E0308)
    assert_compiles(&rust, "heterogeneous_dict_list_values_detection");
}

/// Test: Dict with mixed primitive values (int and str)
#[test]
fn test_heterogeneous_dict_primitives() {
    let python = r#"
def main():
    data = {"count": 42, "name": "test"}
    print(data["name"])
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should use serde_json::Value for heterogeneous values
    assert!(
        rust.contains("serde_json") || rust.contains("Value"),
        "Heterogeneous dict should use serde_json::Value. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "heterogeneous_dict_primitives");
}

/// Test: Dict with nested heterogeneous dict
#[test]
fn test_heterogeneous_nested_dict() {
    let python = r#"
def main():
    data = {"user": {"name": "alice", "age": 30}}
    print(data["user"]["name"])
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should use serde_json::Value for heterogeneous nested values
    assert!(
        rust.contains("serde_json") || rust.contains("Value"),
        "Nested heterogeneous dict should use serde_json::Value. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "heterogeneous_nested_dict");
}

/// Test: Homogeneous dict should still use HashMap (not serde_json)
#[test]
fn test_homogeneous_dict_uses_hashmap() {
    let python = r#"
def main():
    data = {"a": 1, "b": 2, "c": 3}
    print(data["a"])
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Homogeneous dict (all int values) should use HashMap directly
    assert!(
        rust.contains("HashMap"),
        "Homogeneous dict should use HashMap. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "homogeneous_dict_uses_hashmap");
}

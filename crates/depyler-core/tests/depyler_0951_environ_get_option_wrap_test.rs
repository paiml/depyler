//! TDD Test for DEPYLER-0951: os.environ.get() should not double-wrap in Some()
//!
//! Bug: `os.environ.get("KEY")` generates `Some(std::env::var("KEY").ok())`
//! Expected: `std::env::var("KEY").ok()` (already returns Option<String>)
//!
//! Root cause: Return statement codegen wraps expression in Some() when return type
//! is Option<T>, but doesn't check if expression ALREADY returns Option.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::HirModule;
use depyler_core::optimizer::{Optimizer, OptimizerConfig};
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

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

/// DEPYLER-0951: os.environ.get() returns Option<String>, should not be wrapped in Some()
/// Python: `return os.environ.get("KEY")`
/// Bad Rust: `Some(std::env::var("KEY").ok())` - Type error: Option<Option<String>>
/// Good Rust: `std::env::var("KEY").ok()` - Correct: Option<String>
#[test]
fn test_environ_get_no_double_wrap() {
    let python = r#"
import os

def test():
    return os.environ.get("KEY")
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain Some(std::env::var - this would be double-wrapping
    assert!(
        !rust.contains("Some(std::env::var"),
        "Should not double-wrap env::var().ok() in Some(). Generated:\n{}",
        rust
    );

    // Should contain std::env::var().ok() directly
    assert!(
        rust.contains("std::env::var") && rust.contains(".ok()"),
        "Should have std::env::var(...).ok(). Generated:\n{}",
        rust
    );

    // Should compile without type errors
    assert_compiles(&rust, "environ_get_no_double_wrap");
}

/// Test that env::var().ok() is recognized as already returning Option
#[test]
fn test_environ_get_returns_option_string() {
    let python = r#"
import os

def get_config() -> str:
    value = os.environ.get("CONFIG")
    if value:
        return value
    return "default"
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should compile - this tests that value is correctly typed as Option<String>
    assert_compiles(&rust, "environ_get_option_string");
}

/// Test dict.get() also returns Option and shouldn't be double-wrapped
/// Note: This test verifies no double-wrapping but doesn't compile due to separate
/// dict value type bug (Items 45-47 in QA checklist)
#[test]
fn test_dict_get_no_double_wrap() {
    let python = r#"
def test():
    d = {"key": "value"}
    return d.get("key")
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain Some(d.get - dict.get() already returns Option
    assert!(
        !rust.contains("Some(d.get") && !rust.contains("Some(map.get"),
        "Should not double-wrap dict.get() in Some(). Generated:\n{}",
        rust
    );

    // Note: Compilation fails due to separate dict value type issue (Items 45-47)
    // The key issue for DEPYLER-0951 (no double-wrapping) is verified above
}

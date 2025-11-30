//! TDD Tests for Issue #200: os module routing errors
//!
//! These tests verify that Python's os module is correctly transpiled to Rust std/crate equivalents:
//! - os.walk() → walkdir crate
//! - os.urandom() → rand crate
//! - os.environ → std::env::vars()
//! - os.path.exists() → std::path::Path::new().exists()

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

[dependencies]
walkdir = "2"
rand = "0.8"
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

/// Test: os.path.exists() should NOT generate raw "os.path.exists"
#[test]
fn test_os_path_exists_not_raw() {
    let python = r#"
def check_file(path: str) -> bool:
    return os.path.exists(path)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain raw os.path.exists
    assert!(
        !rust.contains("os.path.exists"),
        "Should not have raw os.path.exists. Generated:\n{}",
        rust
    );

    // Should contain std::path::Path::new
    assert!(
        rust.contains("Path::new") || rust.contains("std::path"),
        "Should use std::path. Generated:\n{}",
        rust
    );
}

/// Test: os.walk() should be translated to walkdir
#[test]
fn test_os_walk_translation() {
    let python = r#"
def list_files(directory: str) -> list:
    files = []
    for root, dirs, filenames in os.walk(directory):
        for f in filenames:
            files.append(f)
    return files
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain raw os.walk
    assert!(
        !rust.contains("os.walk"),
        "Should not have raw os.walk. Generated:\n{}",
        rust
    );

    // Should use walkdir or std::fs
    assert!(
        rust.contains("walkdir") || rust.contains("WalkDir") || rust.contains("read_dir"),
        "Should use walkdir or std::fs. Generated:\n{}",
        rust
    );
}

/// Test: os.environ should be translated to std::env
#[test]
fn test_os_environ_translation() {
    let python = r#"
def get_all_env() -> dict:
    return dict(os.environ)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain raw os.environ
    assert!(
        !rust.contains("os.environ"),
        "Should not have raw os.environ. Generated:\n{}",
        rust
    );

    // Should use std::env::vars()
    assert!(
        rust.contains("std::env") || rust.contains("env::vars"),
        "Should use std::env. Generated:\n{}",
        rust
    );
}

/// Test: os.environ.insert() should be translated to std::env::set_var()
#[test]
fn test_os_environ_insert_translation() {
    let python = r#"
def set_env(key: str, value: str):
    os.environ[key] = value
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain raw os.environ.insert
    assert!(
        !rust.contains("os.environ.insert") && !rust.contains("os.environ["),
        "Should not have raw os.environ. Generated:\n{}",
        rust
    );

    // Should use std::env::set_var
    assert!(
        rust.contains("set_var") || rust.contains("std::env"),
        "Should use std::env::set_var. Generated:\n{}",
        rust
    );
}

/// Test: os.urandom() should be translated to rand
#[test]
fn test_os_urandom_translation() {
    let python = r#"
def generate_random_bytes(n: int) -> bytes:
    return os.urandom(n)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain raw os.urandom
    assert!(
        !rust.contains("os.urandom"),
        "Should not have raw os.urandom. Generated:\n{}",
        rust
    );

    // Should use rand crate
    assert!(
        rust.contains("rand") || rust.contains("Rng") || rust.contains("thread_rng"),
        "Should use rand crate. Generated:\n{}",
        rust
    );
}

/// Integration test: Combined os module usage should compile
#[test]
fn test_combined_os_module_compiles() {
    let python = r#"
def check_path(path: str) -> bool:
    return os.path.exists(path)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "combined_os_module");
}

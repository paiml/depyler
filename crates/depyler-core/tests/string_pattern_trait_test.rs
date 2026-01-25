//! TDD Tests for Issue #202: String:Pattern trait bound errors
//!
//! When transpiling Python str.rsplit(sep, maxsplit), the separator must use
//! bare string literals (&str) not String for Rust's Pattern trait.
//!
//! Python: text.rsplit("/", 1)
//! Rust WRONG: text.rsplit("/".to_string(), 1)  // String doesn't impl Pattern
//! Rust RIGHT: text.rsplit("/", 1)  // &str implements Pattern

use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::HirModule;
use depyler_core::optimizer::{Optimizer, OptimizerConfig};
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python: &str) -> Result<String, String> {
    let ast = parse(python, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;

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
        top_level_stmts: hir.top_level_stmts,
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

/// Test: rsplit() should NOT generate String for separator (Pattern trait)
#[test]
fn test_rsplit_uses_str_not_string() {
    let python = r#"
def split_path(path: str) -> str:
    parts = path.rsplit("/", 1)
    return parts[-1]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain "/".to_string() for rsplit separator
    assert!(
        !rust.contains(r#""/".to_string()"#) || !rust.contains("rsplit"),
        "rsplit should use bare string literal, not .to_string(). Generated:\n{}",
        rust
    );
}

/// Test: rsplit() with string literal separator should compile
#[test]
fn test_rsplit_compiles() {
    let python = r#"
def split_path(path: str) -> str:
    parts = path.rsplit("/", 1)
    return parts[-1]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "rsplit_pattern");
}

/// Test: startswith() should use bare string literal for Pattern
#[test]
fn test_startswith_uses_str_literal() {
    let python = r#"
def check_prefix(text: str) -> bool:
    return text.startswith("hello")
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT have "hello".to_string() in startswith call
    // (already handled, this is a regression test)
    assert!(
        !rust.contains(r#"startswith("hello".to_string())"#),
        "startswith should use bare string literal. Generated:\n{}",
        rust
    );
}

/// Test: contains() should use bare string literal for Pattern
#[test]
fn test_contains_uses_str_literal() {
    let python = r#"
def has_substring(text: str) -> bool:
    return "world" in text
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "contains_pattern");
}

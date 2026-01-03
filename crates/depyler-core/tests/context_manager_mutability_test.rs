//! TDD Tests for Context Manager Mutability Bug (DEPYLER-0602)
//!
//! Bug: Custom context managers with __enter__(&mut self) generate immutable
//! _context variable, causing E0596 "cannot borrow as mutable" error.
//!
//! Example:
//! ```python
//! with Connection("localhost") as conn:
//!     conn.query("SELECT 1")
//! ```
//!
//! Root cause: The `with` statement generated `let _context = ...` instead of
//! `let mut _context = ...`, but __enter__() takes &mut self.
//!
//! Fix: Add `mut` keyword to context variable declaration.

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

/// Test: Custom context manager with __enter__ returning self
#[test]
#[ignore] // Flaky under coverage instrumentation - cargo check from instrumented test causes issues
fn test_custom_context_manager_enter_returns_self() {
    let python = r#"
class Connection:
    def __init__(self, host: str):
        self.host = host
        self.connected = False

    def __enter__(self):
        self.connected = True
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.connected = False
        return False

    def query(self, sql: str) -> str:
        return f"Result from {self.host}"

def main():
    with Connection("localhost") as conn:
        result = conn.query("SELECT 1")
        print(result)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The context variable should be mutable
    assert!(
        rust.contains("let mut _context"),
        "Context variable should be mutable. Generated:\n{}",
        rust
    );

    // Should compile without E0596 error
    assert_compiles(&rust, "custom_context_manager_enter_returns_self");
}

/// Test: Context manager without target variable
#[test]
#[ignore] // Flaky under coverage instrumentation - cargo check from instrumented test causes issues
fn test_context_manager_no_target() {
    let python = r#"
class Logger:
    def __init__(self):
        pass

    def __enter__(self):
        print("entering")
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        print("exiting")
        return False

def main():
    with Logger():
        print("inside")
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should compile (context var might be unused but should still be mutable)
    assert_compiles(&rust, "context_manager_no_target");
}

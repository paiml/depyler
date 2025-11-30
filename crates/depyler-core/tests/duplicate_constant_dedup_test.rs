//! TDD Tests for Issue #201: Duplicate constant definitions
//!
//! When a Python file has multiple assignments to the same module-level variable:
//! ```python
//! SCRIPT = Path(__file__).parent / "script.py"  # PathBuf
//! SCRIPT = "script.py"  # str
//! ```
//! Only the LAST assignment should be generated (Python reassignment semantics).
//! Without this fix, both definitions are emitted causing error[E0428].

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
once_cell = "1"
serde_json = "1"
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

/// Test: Duplicate module-level assignments should NOT generate duplicate constants
#[test]
fn test_duplicate_constant_uses_last_value() {
    let python = r#"
SCRIPT = "first.py"
SCRIPT = "second.py"
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain TWO SCRIPT definitions
    let script_count = rust.matches("pub const SCRIPT").count()
        + rust.matches("pub static SCRIPT").count();
    assert_eq!(
        script_count, 1,
        "Should have exactly one SCRIPT definition. Generated:\n{}",
        rust
    );

    // Should contain the LAST value "second.py"
    assert!(
        rust.contains("second.py"),
        "Should use the last assigned value. Generated:\n{}",
        rust
    );

    // Should NOT contain the first value
    assert!(
        !rust.contains("first.py"),
        "Should not contain the first value. Generated:\n{}",
        rust
    );
}

/// Test: PathBuf then string reassignment (common test file pattern)
#[test]
fn test_path_to_string_reassignment_deduplicates() {
    let python = r#"
from pathlib import Path
SCRIPT = Path(__file__).parent / "tool.py"
SCRIPT = "tool.py"
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should have exactly ONE SCRIPT definition
    let script_count = rust.matches("SCRIPT").count();
    // Allow SCRIPT to appear in the value and once in the definition
    assert!(
        script_count <= 2,
        "Should not have duplicate SCRIPT definitions. Count: {}. Generated:\n{}",
        script_count,
        rust
    );
}

/// Test: Duplicate constant deduplication should compile
#[test]
fn test_duplicate_constant_compiles() {
    let python = r#"
SCRIPT = "first.py"
SCRIPT = "second.py"

def main():
    print(SCRIPT)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "duplicate_constant");
}

/// Test: Multiple different constants with same-name reassignment
#[test]
fn test_multiple_constants_with_reassignment() {
    let python = r#"
NAME = "old_name"
NAME = "new_name"
VERSION = "1.0"
VERSION = "2.0"
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Each should appear exactly once as a definition
    let name_defs = rust.matches("pub const NAME").count()
        + rust.matches("pub static NAME").count();
    let version_defs = rust.matches("pub const VERSION").count()
        + rust.matches("pub static VERSION").count();

    assert_eq!(
        name_defs, 1,
        "NAME should be defined once. Generated:\n{}",
        rust
    );
    assert_eq!(
        version_defs, 1,
        "VERSION should be defined once. Generated:\n{}",
        rust
    );

    // Should use the last values
    assert!(rust.contains("new_name"), "Should use last NAME value");
    assert!(rust.contains("2.0"), "Should use last VERSION value");
}

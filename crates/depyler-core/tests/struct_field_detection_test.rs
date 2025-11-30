//! TDD Tests for Struct Field Detection Bug (DEPYLER-0603)
//!
//! Bug: Struct fields assigned outside __init__ (e.g., in __enter__, __exit__)
//! are not detected, causing E0609 "no field X on type Y" errors.
//!
//! Example:
//! ```python
//! class Timer:
//!     def __enter__(self):
//!         self.start = 0  # Field not detected!
//!         return self
//! ```
//!
//! Root cause: `infer_fields_from_init` only looks at __init__ method,
//! but Python allows field assignments in any method.
//!
//! Fix: Extend field inference to scan ALL methods for self.field assignments.

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

/// Test: Fields assigned in __enter__ should be detected
#[test]
fn test_field_assigned_in_enter() {
    let python = r#"
class Timer:
    def __enter__(self):
        self.start = 0
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.elapsed = 100
        return False

def main():
    with Timer() as t:
        x = 1 + 1
    print(t.elapsed)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The struct should have 'start' and 'elapsed' fields
    assert!(
        rust.contains("start:") || rust.contains("pub start"),
        "Struct should have 'start' field. Generated:\n{}",
        rust
    );
    assert!(
        rust.contains("elapsed:") || rust.contains("pub elapsed"),
        "Struct should have 'elapsed' field. Generated:\n{}",
        rust
    );

    // Should compile without E0609 error
    assert_compiles(&rust, "field_assigned_in_enter");
}

/// Test: Fields assigned in regular method should be detected
#[test]
fn test_field_assigned_in_regular_method() {
    let python = r#"
class Counter:
    def reset(self):
        self.count = 0

    def increment(self):
        self.count = self.count + 1

def main():
    c = Counter()
    c.reset()
    print(c.count)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The struct should have 'count' field
    assert!(
        rust.contains("count:") || rust.contains("pub count"),
        "Struct should have 'count' field. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "field_assigned_in_regular_method");
}

/// Test: Class with __init__ AND other method field assignments
#[test]
fn test_field_from_init_and_other_methods() {
    let python = r#"
class Connection:
    def __init__(self, host: str):
        self.host = host

    def connect(self):
        self.connected = True

def main():
    c = Connection("localhost")
    c.connect()
    print(c.connected)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should have both 'host' (from __init__) and 'connected' (from connect())
    assert!(
        rust.contains("host:") || rust.contains("pub host"),
        "Struct should have 'host' field. Generated:\n{}",
        rust
    );
    assert!(
        rust.contains("connected:") || rust.contains("pub connected"),
        "Struct should have 'connected' field. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "field_from_init_and_other_methods");
}

/// Test: Deduplication - same field assigned in multiple methods
#[test]
fn test_field_deduplication() {
    let python = r#"
class State:
    def __init__(self):
        self.value = 0

    def reset(self):
        self.value = 0  # Same field

def main():
    s = State()
    s.reset()
    print(s.value)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should have exactly one 'value' field (not duplicated)
    let field_count = rust.matches("value:").count() + rust.matches("pub value").count();
    // The struct definition should only have one field declaration
    // (multiple accesses to self.value in method bodies are fine)
    assert!(
        field_count >= 1,
        "Struct should have at least one 'value' field. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "field_deduplication");
}

//! DEPYLER-0956: os.makedirs() uses ? but function returns ()
//!
//! Bug: When os.makedirs() is used, the generated code used the ? operator
//! but the function return type was () instead of Result<(), Box<dyn Error>>.
//!
//! Fix: Use .unwrap() instead of ? to not require Result return type.
//! This matches Python's semantics where OSError is raised (panics in Rust).

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
    use std::process::Command;

    let test_file = std::env::temp_dir().join(format!("depyler_0956_{}.rs", test_name));
    std::fs::write(&test_file, rust_code).expect("Failed to write test file");

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--edition", "2021"])
        .arg(&test_file)
        .output()
        .expect("Failed to run rustc");

    assert!(
        output.status.success(),
        "Compilation failed for {}: {}",
        test_name,
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test that os.makedirs generates compilable code
#[test]
fn test_makedirs_generates_compilable_code() {
    let python = r#"
import os

def test():
    os.makedirs("foo/bar/baz")
"#;

    let rust = transpile(python).expect("transpilation failed");

    // Must use .unwrap() not ?
    assert!(
        rust.contains(".unwrap()"),
        "Expected .unwrap() but got: {}",
        rust
    );

    // Verify it compiles
    assert_compiles(&rust, "makedirs");
}

/// Test that os.mkdir also generates compilable code
#[test]
fn test_mkdir_generates_compilable_code() {
    let python = r#"
import os

def test():
    os.mkdir("foo")
"#;

    let rust = transpile(python).expect("transpilation failed");

    // Must use .unwrap() not ?
    assert!(
        rust.contains(".unwrap()"),
        "Expected .unwrap() but got: {}",
        rust
    );

    // Verify it compiles
    assert_compiles(&rust, "mkdir");
}

/// Test that os.rmdir also generates compilable code
#[test]
fn test_rmdir_generates_compilable_code() {
    let python = r#"
import os

def test():
    os.rmdir("foo")
"#;

    let rust = transpile(python).expect("transpilation failed");

    // Must use .unwrap() not ?
    assert!(
        rust.contains(".unwrap()"),
        "Expected .unwrap() but got: {}",
        rust
    );

    // Verify it compiles
    assert_compiles(&rust, "rmdir");
}

/// Test that os.rename also generates compilable code
#[test]
fn test_rename_generates_compilable_code() {
    let python = r#"
import os

def test():
    os.rename("old.txt", "new.txt")
"#;

    let rust = transpile(python).expect("transpilation failed");

    // Must use .unwrap() not ?
    assert!(
        rust.contains(".unwrap()"),
        "Expected .unwrap() but got: {}",
        rust
    );

    // Verify it compiles
    assert_compiles(&rust, "rename");
}

//! TDD Test for DEPYLER-0952: Bare None assignment needs type annotation
//!
//! Bug: `x = None` generates `let x = None;` without type annotation
//! Expected: `let x: Option<()> = None;` (Rust needs type annotation for None)
//!
//! Root cause: When there's no Python type annotation on a None assignment,
//! the codegen skips type annotation generation, but Rust can't infer Option<_>

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

/// DEPYLER-0952: Bare `x = None` should generate type annotation `Option<()>`
/// Python: `x = None` (no type hint)
/// Bad Rust: `let x = None;` - Error: type annotations needed for `Option<_>`
/// Good Rust: `let x: Option<()> = None;` - Compiles correctly
#[test]
fn test_none_assignment_has_type_annotation() {
    let python = r#"
def test():
    x = None
    return x
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should contain type annotation for None
    // Either `let x: Option<()>` or `let x : Option < () >` (with spaces from quote!)
    assert!(
        rust.contains("Option<()>") || rust.contains("Option < () >"),
        "None assignment should have type annotation Option<()>. Generated:\n{}",
        rust
    );

    // Should compile without type inference errors
    assert_compiles(&rust, "none_assignment_type_annotation");
}

/// Test that None in mutable variable also gets type annotation
#[test]
fn test_mutable_none_has_type_annotation() {
    let python = r#"
def test():
    x = None
    if True:
        x = "hello"
    return x
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should compile - the None placeholder should have proper typing
    // Note: This uses hoisting pattern, but if mutable_vars skips None,
    // the first real assignment becomes the declaration
    // Either way, it should compile
    assert_compiles(&rust, "mutable_none_type_annotation");
}

/// Test that returning None from a function works
#[test]
fn test_return_none_compiles() {
    let python = r#"
def test():
    return None
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should compile without errors
    assert_compiles(&rust, "return_none");
}

/// Test None in conditional expression
#[test]
fn test_none_in_conditional() {
    let python = r#"
def test(flag: bool) -> str:
    result = None
    if flag:
        result = "yes"
    else:
        result = "no"
    return result
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should compile - None placeholder should work with later string assignments
    assert_compiles(&rust, "none_conditional");
}

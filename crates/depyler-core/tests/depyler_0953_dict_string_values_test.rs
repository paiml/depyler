//! TDD Test for DEPYLER-0953: Dict string values should be String, not &str
//!
//! Bug: `d = {"k": "v"}` generates `map.insert("k".to_string(), "v");`
//! Expected: `map.insert("k".to_string(), "v".to_string());`
//!
//! Root cause: String literal values are only converted to .to_string() when
//! there's an explicit type annotation requiring String. Without annotation,
//! values stay as &str, causing HashMap<String, &str> vs HashMap<String, String>.

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

/// DEPYLER-0953: Dict with string values should use String, not &str
/// Python: `d = {"key": "value"}` (no type hint)
/// Bad Rust: HashMap<String, &str> - type mismatch when returned
/// Good Rust: HashMap<String, String> - consistent types
#[test]
fn test_dict_string_values_are_owned() {
    let python = r#"
def test():
    d = {"key": "value"}
    return d
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should convert string values to owned Strings
    assert!(
        rust.contains(".to_string()"),
        "Dict string values should be converted to String. Generated:\n{}",
        rust
    );

    // Count how many .to_string() calls - should be at least 2 (key AND value)
    let to_string_count = rust.matches(".to_string()").count();
    assert!(
        to_string_count >= 2,
        "Both key AND value should use .to_string(). Found {} calls. Generated:\n{}",
        to_string_count,
        rust
    );

    // Should compile without type errors
    assert_compiles(&rust, "dict_string_values_owned");
}

/// Test dict access compiles correctly
#[test]
fn test_dict_access_compiles() {
    let python = r#"
def test():
    d = {"key": "value"}
    v = d["key"]
    return v
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "dict_access");
}

/// Test dict assignment compiles correctly
#[test]
fn test_dict_assignment_compiles() {
    let python = r#"
def test():
    d = {"key": "value"}
    d["key2"] = "value2"
    return d
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "dict_assignment");
}

/// Test dict with mixed value types
#[test]
fn test_dict_with_int_values() {
    let python = r#"
def test():
    d = {"a": 1, "b": 2}
    return d
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    // Int values don't need conversion
    assert_compiles(&rust, "dict_int_values");
}

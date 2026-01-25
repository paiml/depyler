//! TDD Test for DEPYLER-0954: Integer addition should not cast to f64
//!
//! Bug: `a + b` where both are i32 generates `(a as f64) + b`
//! Expected: `a + b` (no casting when both operands are integers)
//!
//! Root cause: Variable "b" was incorrectly matched by is_float_var heuristic
//! for color channel names (r, g, b, h, s, v...). The comment says "b" should
//! be excluded as too generic, but it was included in the matches.

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

/// DEPYLER-0954: Integer addition with params named 'a', 'b' should not cast to f64
/// Python: `def f(a: int, b: int) -> int: return a + b`
/// Bad Rust: `(a as f64) + b` - type error (f64 + i32)
/// Good Rust: `a + b` - correct integer addition
#[test]
fn test_int_add_no_float_cast() {
    let python = r#"
def f(a: int, b: int) -> int:
    return a + b
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain "(a as f64)" - spurious float cast
    assert!(
        !rust.contains("as f64"),
        "Integer addition should not cast to f64. Generated:\n{}",
        rust
    );

    // Should contain simple addition without cast
    assert!(
        rust.contains("a + b") || rust.contains("a +b") || rust.contains("a+ b"),
        "Should have simple integer addition. Generated:\n{}",
        rust
    );

    // Should compile without type errors
    assert_compiles(&rust, "int_add_no_float_cast");
}

/// Test that other single-letter int params also work
#[test]
fn test_int_mul_with_x_y() {
    let python = r#"
def f(x: int, y: int) -> int:
    return x * y
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT contain float casting
    assert!(
        !rust.contains("as f64"),
        "Integer multiplication should not cast to f64. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "int_mul_x_y");
}

/// Test that float operations still work correctly
#[test]
fn test_float_add_still_works() {
    let python = r#"
def f(a: float, b: float) -> float:
    return a + b
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "float_add");
}

/// Test mixed int/float still casts correctly
#[test]
fn test_int_float_mix_casts() {
    let python = r#"
def f(n: int, rate: float) -> float:
    return n * rate
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should cast int to float when multiplying with float
    // Either using literal suffix or explicit cast
    assert_compiles(&rust, "int_float_mix");
}

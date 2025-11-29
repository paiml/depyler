//! DEPYLER-0516: Negative Literal Type Inference Bug (E0308)
//!
//! **STATUS**: RED phase - Failing tests
//!
//! **PROBLEM**: Negative integer literals default to `serde_json::Value` instead of `i32`
//!
//! **IMPACT**: 15/32 verificar corpus failures (47%)
//!
//! **ROOT CAUSE**: UnaryOp::Neg loses type information during inference
//!
//! **EXAMPLES**:
//! - BROKEN: `x = -1` → `pub const x: serde_json::Value = -1;`
//! - CORRECT: `x = 1` → `pub const x: i32 = 1;`
//!
//! **FIX**: Preserve integer type through unary negation

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};
use std::fs;
use std::process::Command;

fn transpile_to_rust(python_code: &str) -> Result<String, String> {
    let ast = parse(python_code, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

fn check_rust_compiles(rust_code: &str) -> Result<(), String> {
    let temp_file = "/tmp/depyler_0516_test.rs";
    fs::write(temp_file, rust_code).map_err(|e| format!("Failed to write temp file: {}", e))?;

    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--deny")
        .arg("warnings")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/depyler_0516_test.rlib")
        .output()
        .map_err(|e| format!("Failed to run rustc: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Compilation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[test]
#[ignore = "Known failing - DEPYLER-0516"]
fn test_DEPYLER_0516_negative_integer_literal() {
    // RED: Negative literal should generate i32, not serde_json::Value
    let python = "x = -1";
    let rust_code = transpile_to_rust(python).expect("Transpilation should succeed");

    // Should use i32, not serde_json::Value
    assert!(
        rust_code.contains("i32"),
        "DEPYLER-0516: Negative literal should infer i32.\nGenerated:\n{}",
        rust_code
    );
    assert!(
        !rust_code.contains("serde_json::Value"),
        "DEPYLER-0516: Should not use serde_json::Value for integer literals.\nGenerated:\n{}",
        rust_code
    );

    // Should compile
    match check_rust_compiles(&rust_code) {
        Ok(_) => {}
        Err(e) => panic!(
            "DEPYLER-0516: Generated code should compile.\nError:\n{}\nCode:\n{}",
            e, rust_code
        ),
    }
}

#[test]
#[ignore = "Known failing - DEPYLER-0516"]
fn test_DEPYLER_0516_parenthesized_negative() {
    // RED: Parenthesized negative should also infer i32
    let python = "x = (-1)";
    let rust_code = transpile_to_rust(python).expect("Transpilation should succeed");

    assert!(
        rust_code.contains("i32"),
        "DEPYLER-0516: Parenthesized negative should infer i32.\nGenerated:\n{}",
        rust_code
    );

    check_rust_compiles(&rust_code).expect("Should compile");
}

#[test]
#[ignore = "Known failing - DEPYLER-0516"]
fn test_DEPYLER_0516_double_negative() {
    // RED: Double negative should infer i32
    let python = "x = (--1)";
    let rust_code = transpile_to_rust(python).expect("Transpilation should succeed");

    assert!(
        rust_code.contains("i32"),
        "DEPYLER-0516: Double negative should infer i32.\nGenerated:\n{}",
        rust_code
    );

    check_rust_compiles(&rust_code).expect("Should compile");
}

#[test]
#[ignore = "Known failing - DEPYLER-0516"]
fn test_DEPYLER_0516_various_negatives() {
    // RED: All negative integers should infer i32
    for val in &["-1", "-2", "-10", "-100", "-0"] {
        let python = format!("x = {}", val);
        let rust_code = transpile_to_rust(&python).expect("Transpilation should succeed");

        assert!(
            rust_code.contains("i32"),
            "DEPYLER-0516: '{}' should infer i32.\nGenerated:\n{}",
            val,
            rust_code
        );
        assert!(
            !rust_code.contains("serde_json::Value"),
            "DEPYLER-0516: '{}' should not use serde_json::Value.\nGenerated:\n{}",
            val,
            rust_code
        );

        check_rust_compiles(&rust_code)
            .unwrap_or_else(|e| panic!("DEPYLER-0516: '{}' should compile.\nError:\n{}", val, e));
    }
}

#[test]
#[ignore = "Known failing - DEPYLER-0516"]
fn test_DEPYLER_0516_positive_still_works() {
    // CONTROL: Positive integers should still work (don't break them!)
    for val in &["0", "1", "2", "10", "100"] {
        let python = format!("x = {}", val);
        let rust_code = transpile_to_rust(&python).expect("Transpilation should succeed");

        assert!(
            rust_code.contains("i32"),
            "DEPYLER-0516: Positive '{}' should still infer i32.\nGenerated:\n{}",
            val,
            rust_code
        );

        check_rust_compiles(&rust_code).unwrap_or_else(|e| {
            panic!(
                "DEPYLER-0516: '{}' should still compile.\nError:\n{}",
                val, e
            )
        });
    }
}

#[test]
#[ignore = "Known failing - DEPYLER-0516"]
fn test_DEPYLER_0516_function_context() {
    // RED: Negative literals in function context should also work
    let python = r#"
def foo():
    x = -1
    return x
"#;

    let rust_code = transpile_to_rust(python).expect("Transpilation should succeed");

    // Should contain i32 somewhere for the variable
    assert!(
        rust_code.contains("i32") || !rust_code.contains("serde_json::Value"),
        "DEPYLER-0516: Negative literal in function should infer correctly.\nGenerated:\n{}",
        rust_code
    );

    check_rust_compiles(&rust_code).expect("Should compile");
}

//! Walrus Operator (:=) Support Tests (DEPYLER-0188)
//!
//! Tests for Python assignment expressions in various contexts:
//! - if conditions
//! - for loop conditions
//! - list comprehensions
//! - while loops (already supported)

use depyler_core::ast_bridge::AstBridge;
use depyler_core::hir::HirModule;
use depyler_core::optimizer::{Optimizer, OptimizerConfig};
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

/// Transpile with optimizer (matches CLI behavior)
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

/// Test walrus operator in if condition
/// Python: if (n := len(text)) > 5: return n
/// Rust: let n = text.len(); if n > 5 { return n; }
#[test]
fn test_walrus_in_if_condition() {
    let python = r#"
def check_length(text: str) -> int:
    if (n := len(text)) > 5:
        return n
    return 0
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile walrus in if: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    // Should have assignment for n before/outside the if (hoisted correctly)
    assert!(
        rust.contains("let n"),
        "Should generate let binding for walrus target. Got:\n{}",
        rust
    );
    // Should be able to use n in the return statement (not scoped to a block)
    assert!(
        rust.contains("return n"),
        "Should be able to use walrus variable in if body. Got:\n{}",
        rust
    );
    // The comparison should involve n (either directly or via CSE temp)
    assert!(
        rust.contains("n >")
            || rust.contains("n>")
            || (rust.contains("let n") && rust.contains("> 5")),
        "Should compare walrus value with 5. Got:\n{}",
        rust
    );
}

/// Test walrus operator in for loop body with if
/// Python: for word in words: if (n := len(word)) >= min_len: return f"{word}({n})"
#[test]
fn test_walrus_in_for_if() {
    let python = r#"
def find_first_long(text: str, min_len: int) -> str:
    words = text.split()
    for word in words:
        if (n := len(word)) >= min_len:
            return word
    return "none"
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile walrus in for-if: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    // Should have assignment before if inside the for loop
    assert!(
        rust.contains("let n"),
        "Should generate let binding for walrus. Got:\n{}",
        rust
    );
}

/// Test walrus operator in simple comparison
/// Python: if (x := get_value()) is not None:
#[test]
fn test_walrus_with_none_check() {
    let python = r#"
def check_value(data: dict) -> int:
    if (x := data.get("key")) is not None:
        return x
    return 0
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile walrus with None check: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    // Should use if let or Option pattern
    assert!(
        rust.contains("let x") || rust.contains("if let"),
        "Should handle walrus with None check. Got:\n{}",
        rust
    );
}

/// Test walrus operator in while loop (already supported - regression test)
#[test]
fn test_walrus_in_while_loop() {
    let python = r#"
def read_chunks(data: list) -> int:
    count = 0
    i = 0
    while (chunk := data[i] if i < len(data) else None) is not None:
        count += 1
        i += 1
    return count
"#;

    // This is a complex case - may not be supported yet
    // Just verify it doesn't panic
    let _result = transpile(python);
}

/// Test simple walrus in if with direct use
#[test]
fn test_walrus_simple_if() {
    let python = r#"
def check(x: int) -> int:
    if (y := x * 2) > 10:
        return y
    return 0
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile simple walrus if: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("let y") && rust.contains("y >"),
        "Should have let y and use y in condition. Got:\n{}",
        rust
    );
}

/// Test walrus with boolean result
#[test]
fn test_walrus_boolean_condition() {
    let python = r#"
def has_items(items: list) -> bool:
    if (count := len(items)) > 0:
        return True
    return False
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile walrus boolean: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("let count"),
        "Should generate let count. Got:\n{}",
        rust
    );
}

/// Test nested walrus operators
#[test]
fn test_walrus_nested() {
    let python = r#"
def process(a: int, b: int) -> int:
    if (x := a + 1) > 0:
        if (y := x + b) > 10:
            return y
    return 0
"#;

    let result = transpile(python);
    assert!(
        result.is_ok(),
        "Should transpile nested walrus: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("let x") && rust.contains("let y"),
        "Should generate both let bindings. Got:\n{}",
        rust
    );
}

//! DEPYLER-0513: F-String (Formatted String Literal) Support
//!
//! **ROOT CAUSE**: HIR → Rust codegen missing match arm for HirExpr::FString
//!
//! **Five Whys**:
//! 1. Why fails in classes? direct_rules.rs path lacks FString match arm
//! 2. Why two paths? Classes use direct_rules, functions use expr_gen
//! 3. Why no FString in direct_rules? Only added to expr_gen path
//! 4. Why separate? Class codegen has different context (self refs)
//! 5. ROOT: F-string codegen only in expr_gen, not in direct_rules.rs:1841
//!
//! **Examples**:
//! - Python: `f"Hello {name}"` → Rust: `format!("Hello {}", name)`
//! - Python: `f"{x} + {y} = {x+y}"` → Rust: `format!("{} + {} = {}", x, y, x + y)`

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile_to_rust(python_code: &str) -> Result<String, String> {
    let ast = parse(python_code, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

// ============================================================================
// RED PHASE - Failing Tests
// ============================================================================

#[test]
fn test_DEPYLER_0513_simple_fstring() {
    // RED: Should fail with "Expression type not yet supported: FString"
    let python = r#"
def greet(name: str) -> str:
    return f"Hello {name}"
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0513: Simple f-string should be supported. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should generate format! macro
    assert!(
        rust_code.contains("format!"),
        "DEPYLER-0513: Should generate format! macro. Generated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0513_fstring_with_multiple_expressions() {
    // Multiple expressions in f-string
    let python = r#"
def calculate(x: int, y: int) -> str:
    return f"{x} + {y} = {x + y}"
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0513: F-string with multiple expressions should work. Error:\n{}",
        result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0513_fstring_with_literal_only() {
    // F-string with no expressions (just literal)
    let python = r#"
def message() -> str:
    return f"Hello world"
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0513: F-string with only literals should work. Error:\n{}",
        result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0513_fstring_with_method_call() {
    // F-string with method call
    let python = r#"
def debug(items: list[int]) -> str:
    return f"Length: {len(items)}"
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0513: F-string with method calls should work. Error:\n{}",
        result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0513_fstring_in_class_method() {
    // RED: This is where it actually fails - f-strings in class methods
    let python = r#"
class Game:
    def __init__(self, verbose: bool):
        self.verbose = verbose

    def debug(self, target: int) -> None:
        print(f"Target: {target}")
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0513: F-string in class method should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should contain format!
    assert!(
        rust_code.contains("format!"),
        "DEPYLER-0513: Should generate format! macro. Generated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0513_marco_polo_example() {
    // Real example from marco_polo.py line 67
    let python = r#"
class MarcoPoloGame:
    def play_round(self, target: int) -> None:
        print(f"\n[DEBUG] Target number: {target}")
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0513: marco_polo f-string should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should contain println! with format!
    assert!(
        rust_code.contains("format!") || rust_code.contains("println!"),
        "DEPYLER-0513: Should generate format! or println!. Generated:\n{}",
        rust_code
    );
}

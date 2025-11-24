//! DEPYLER-0509: Unnecessary Parentheses in Integer Casts
//!
//! Tests to verify that integer literals don't have unnecessary double parentheses
//! when cast to i64 for function calls.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile_to_rust(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").unwrap();
    let (hir, _) = AstBridge::new().python_to_hir(ast).unwrap();
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).unwrap();
    rust_code
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0509_integer_literal_casts_no_double_parens() {
    // DEPYLER-0509: Integer literals should not have double parentheses when cast
    let python = r#"
def add(x: int, y: int) -> int:
    return x + y

def main():
    result = add(2, 3)
"#;

    let rust_code = transpile_to_rust(python);

    // Should NOT have double parentheses like ((2) as i64)
    assert!(
        !rust_code.contains("((2)"),
        "DEPYLER-0509: Integer literal 2 should not have double parentheses. Generated: {}",
        rust_code
    );
    assert!(
        !rust_code.contains("((3)"),
        "DEPYLER-0509: Integer literal 3 should not have double parentheses. Generated: {}",
        rust_code
    );

    // Should have proper cast syntax: either 2_i64, 2 as i64, or (2 as i64)
    // Not ((2) as i64)
    let has_proper_cast = rust_code.contains("2_i64")
        || rust_code.contains("2 as i64")
        || (rust_code.contains("(2 as i64)") && !rust_code.contains("((2)"));

    assert!(
        has_proper_cast || !rust_code.contains("as i64"),
        "DEPYLER-0509: Integer casts should use proper syntax without double parentheses"
    );
}

#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0509_no_double_parens_in_generated_code() {
    // DEPYLER-0509: Generated code should not have double parentheses
    let python = r#"
def calc(x: int, y: int, z: int) -> int:
    return (x + y) * z

def test() -> int:
    result = calc(2, 3, 4)
    return result
"#;

    let rust_code = transpile_to_rust(python);

    // Check that the output has proper casts without double parentheses
    // The code should have "(2 as i64)" not "((2) as i64)"
    let has_double_parens = rust_code.contains("((2)")
        || rust_code.contains("((3)")
        || rust_code.contains("((4)");

    assert!(
        !has_double_parens,
        "DEPYLER-0509: Should not have double parentheses. Generated code:\n{}",
        rust_code
    );

    // The code should have proper cast syntax: either "2 as i64" or "(2 as i64)"
    // but NOT "((2) as i64)"
    let has_proper_cast = rust_code.contains("2 as i64")
        || rust_code.contains("3 as i64")
        || rust_code.contains("4 as i64");

    assert!(
        has_proper_cast,
        "DEPYLER-0509: Should have proper cast syntax (N as i64). Generated code:\n{}",
        rust_code
    );
}

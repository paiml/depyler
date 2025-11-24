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
fn test_DEPYLER_0509_compiles_without_warnings() {
    // DEPYLER-0509: Generated code should compile without clippy warnings
    let python = r#"
def calc(x: int, y: int, z: int) -> int:
    return (x + y) * z

def test():
    result = calc(2, 3, 4)
    return result
"#;

    let rust_code = transpile_to_rust(python);

    // Write to temp file
    let temp_file = "/tmp/depyler_0509_test.rs";
    std::fs::write(temp_file, &rust_code).unwrap();

    // Compile with warnings as errors
    let output = std::process::Command::new("rustc")
        .args(["--crate-type", "lib", "--deny", "warnings", temp_file])
        .output()
        .expect("Failed to execute rustc");

    // Clean up
    let _ = std::fs::remove_file(temp_file);

    assert!(
        output.status.success(),
        "DEPYLER-0509: Generated code should compile without warnings. Error: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

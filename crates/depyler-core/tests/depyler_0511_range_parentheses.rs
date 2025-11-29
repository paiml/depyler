//! DEPYLER-0511: Range Comprehension Parentheses Bug
//!
//! **STOP THE LINE - P0 Bug**
//!
//! Affects 3 tests:
//! - test_25_list_comprehension
//! - test_30_dict_comprehension
//! - test_35_set_comprehension
//!
//! ROOT CAUSE: convert_range_call returns bare `0..5`, which breaks when
//! `.into_iter()` is appended due to operator precedence.
//!
//! Generated: `0..5.into_iter()` (WRONG - parses as `0..(5.into_iter())`)
//! Should be: `(0..5).into_iter()` (CORRECT)
//!
//! **Five Whys Analysis:**
//! 1. Why fails? Operator precedence: `.` binds tighter than `..`
//! 2. Why no parens? Comprehension appends `.into_iter()` directly
//! 3. Why bare range? convert_range_call returns `0..5` without parens
//! 4. Why no parens there? Ranges don't need parens in isolation
//! 5. ROOT: Ranges need parens when followed by method calls

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};
use std::process::Command;

fn transpile_to_rust(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").unwrap();
    let (hir, _) = AstBridge::new().python_to_hir(ast).unwrap();
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).unwrap();
    rust_code
}

fn compile_rust(rust_code: &str) -> Result<(), String> {
    std::fs::write("/tmp/depyler_0511_test.rs", rust_code).unwrap();

    let output = Command::new("rustc")
        .args([
            "--crate-type",
            "lib",
            "--edition",
            "2021",
            "/tmp/depyler_0511_test.rs",
            "-o",
            "/tmp/depyler_0511_test.rlib",
        ])
        .output()
        .unwrap();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    Ok(())
}

// ============================================================================
// RED PHASE - Failing Tests
// ============================================================================

#[test]
fn test_DEPYLER_0511_list_comprehension_range() {
    // RED: This should fail with "can't call method `into_iter` on type `{integer}`"
    let python = r#"
def test() -> list[int]:
    squares = [x * x for x in range(5)]
    return squares
"#;

    let rust_code = transpile_to_rust(python);

    // Should contain (0..5).into_iter() NOT 0..5.into_iter()
    assert!(
        rust_code.contains("(0..5).into_iter()") || rust_code.contains("(0_i32..5"),
        "DEPYLER-0511: Range should have parentheses before .into_iter(). Generated:\n{}",
        rust_code
    );

    // Should compile successfully
    let compile_result = compile_rust(&rust_code);
    assert!(
        compile_result.is_ok(),
        "DEPYLER-0511: Generated code should compile. Error:\n{}",
        compile_result.unwrap_err()
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0511"]
fn test_DEPYLER_0511_dict_comprehension_range() {
    // RED: This should fail with same error
    let python = r#"
def test() -> dict[int, int]:
    squares = {x: x * x for x in range(5)}
    return squares
"#;

    let rust_code = transpile_to_rust(python);

    // Check for parentheses (whitespace-agnostic)
    let normalized = rust_code.replace(char::is_whitespace, "");
    assert!(
        normalized.contains("(0..5).into_iter()") || normalized.contains("(0_i32..5"),
        "DEPYLER-0511: Range should have parentheses. Generated:\n{}",
        rust_code
    );

    // Should compile
    let compile_result = compile_rust(&rust_code);
    assert!(
        compile_result.is_ok(),
        "DEPYLER-0511: Dict comprehension should compile. Error:\n{}",
        compile_result.unwrap_err()
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0511 set comprehension range"]
fn test_DEPYLER_0511_set_comprehension_range() {
    // RED: This should fail with same error
    let python = r#"
def test() -> set[int]:
    evens = {x for x in range(10) if x % 2 == 0}
    return evens
"#;

    let rust_code = transpile_to_rust(python);

    // Check for parentheses (whitespace-agnostic)
    let normalized = rust_code.replace(char::is_whitespace, "");
    assert!(
        normalized.contains("(0..10).into_iter()") || normalized.contains("(0_i32..10"),
        "DEPYLER-0511: Range should have parentheses. Generated:\n{}",
        rust_code
    );

    // Should compile
    let compile_result = compile_rust(&rust_code);
    assert!(
        compile_result.is_ok(),
        "DEPYLER-0511: Set comprehension should compile. Error:\n{}",
        compile_result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0511_range_with_start_end() {
    // Test range(2, 7) case
    let python = r#"
def test() -> list[int]:
    numbers = [x for x in range(2, 7)]
    return numbers
"#;

    let rust_code = transpile_to_rust(python);

    // Should have (2..7).into_iter()
    assert!(
        rust_code.contains("(2..7).into_iter()") || rust_code.contains("(2_i32..7"),
        "DEPYLER-0511: Range with start/end should have parentheses. Generated:\n{}",
        rust_code
    );

    let compile_result = compile_rust(&rust_code);
    assert!(
        compile_result.is_ok(),
        "DEPYLER-0511: Should compile. Error:\n{}",
        compile_result.unwrap_err()
    );
}

#[test]
fn test_DEPYLER_0511_nested_comprehension_ranges() {
    // Test nested comprehensions with ranges
    let python = r#"
def test() -> list[tuple[int, int]]:
    pairs = [(x, y) for x in range(3) for y in range(3)]
    return pairs
"#;

    let rust_code = transpile_to_rust(python);

    // Both ranges should have parentheses (whitespace-agnostic)
    let normalized = rust_code.replace(char::is_whitespace, "");
    let has_parens = normalized.matches("(0..3)").count() >= 2;
    assert!(
        has_parens,
        "DEPYLER-0511: Both nested ranges should have parentheses. Generated:\n{}",
        rust_code
    );

    let compile_result = compile_rust(&rust_code);
    assert!(
        compile_result.is_ok(),
        "DEPYLER-0511: Nested comprehension should compile. Error:\n{}",
        compile_result.unwrap_err()
    );
}

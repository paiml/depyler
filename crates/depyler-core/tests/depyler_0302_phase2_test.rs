#![allow(non_snake_case)]
// DEPYLER-0302 Phase 2: String Method Medium Wins Test
// Tests for:
// 1. String multiplication (s * n → .repeat())
// 2. title() method with custom implementation

use depyler_core::DepylerPipeline;

// ========== String Multiplication Tests ==========

#[test]
fn test_string_mult_literal_times_int() {
    let python_code = r#"
def repeat_ab() -> str:
    return "ab" * 3
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .repeat()
    assert!(
        rust_code.contains(".repeat("),
        "Should contain .repeat() method"
    );
    assert!(
        rust_code.contains("3 as usize"),
        "Should cast count to usize"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_mult_var_times_int() {
    let python_code = r#"
def repeat_string(s: str, count: int) -> str:
    return s * count
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .repeat()
    assert!(
        rust_code.contains(".repeat("),
        "Should contain .repeat() method"
    );
    assert!(
        rust_code.contains("count as usize"),
        "Should cast count to usize"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_mult_int_times_string() {
    let python_code = r#"
def repeat_reverse(count: int, s: str) -> str:
    return count * s
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .repeat() with reversed order
    assert!(
        rust_code.contains(".repeat("),
        "Should contain .repeat() method"
    );
    assert!(
        rust_code.contains("count as usize"),
        "Should cast count to usize"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_mult_in_expression() {
    let python_code = r#"
def make_border(width: int) -> str:
    return "=" * width + " TITLE " + "=" * width
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .repeat() for both multiplications
    let repeat_count = rust_code.matches(".repeat(").count();
    assert!(
        repeat_count >= 2,
        "Should contain at least 2 .repeat() calls, found {}",
        repeat_count
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== title() Method Tests ==========

#[test]
fn test_title_basic() {
    let python_code = r#"
def to_titlecase(s: str) -> str:
    return s.title()
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use split_whitespace
    assert!(
        rust_code.contains("split_whitespace()"),
        "Should contain split_whitespace()"
    );
    // Should use to_uppercase
    assert!(
        rust_code.contains("to_uppercase()"),
        "Should contain to_uppercase()"
    );
    // Should join with space
    assert!(
        rust_code.contains(".join(\" \")"),
        "Should contain .join(\" \")"
    );
    // Should NOT have literal "title()" method call
    assert!(
        !rust_code.contains(".title()"),
        "Should NOT contain .title() method call"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_title_with_literal() {
    let python_code = r#"
def greet() -> str:
    return "hello world".title()
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use title case implementation
    assert!(
        rust_code.contains("split_whitespace()"),
        "Should contain split_whitespace()"
    );
    assert!(
        rust_code.contains("to_uppercase()"),
        "Should contain to_uppercase()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_title_in_expression() {
    let python_code = r#"
def format_name(first: str, last: str) -> str:
    return first.title() + " " + last.title()
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should have multiple title case implementations
    let title_count = rust_code.matches("split_whitespace()").count();
    assert!(
        title_count >= 2,
        "Should contain at least 2 title case implementations, found {}",
        title_count
    );

    println!("Generated Rust code:\n{}", rust_code);
}

// ========== Integration Tests ==========

#[test]
fn test_phase2_combined() {
    // Test both fixes together
    let python_code = r#"
def make_header(text: str, width: int) -> str:
    title = text.title()
    border = "=" * width
    return border + " " + title + " " + border
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should have both title and repeat
    assert!(
        rust_code.contains("split_whitespace()"),
        "Should contain title() implementation"
    );
    assert!(
        rust_code.contains(".repeat("),
        "Should contain .repeat() for string multiplication"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_mult_does_not_break_array_mult() {
    // Ensure array multiplication still works
    let python_code = r#"
def make_array() -> list[int]:
    return [0] * 5
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use array syntax, NOT .repeat()
    assert!(
        rust_code.contains("[") && rust_code.contains(";"),
        "Should use array syntax [elem; size]"
    );
    // Should NOT use .repeat() for array
    assert!(
        !rust_code.contains(".repeat("),
        "Should NOT use .repeat() for array multiplication"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

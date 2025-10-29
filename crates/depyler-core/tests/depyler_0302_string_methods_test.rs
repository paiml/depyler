// DEPYLER-0302 Phase 1: String Method Quick Wins Test
// Tests for lstrip, rstrip, isalnum methods

use depyler_core::DepylerPipeline;

#[test]
fn test_lstrip_basic() {
    let python_code = r#"
def strip_leading(s: str) -> str:
    return s.lstrip()
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .trim_start()
    assert!(
        rust_code.contains("trim_start()"),
        "Should contain trim_start()"
    );
    assert!(
        !rust_code.contains("lstrip()"),
        "Should not contain lstrip()"
    );

    // Should compile
    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_rstrip_basic() {
    let python_code = r#"
def strip_trailing(s: str) -> str:
    return s.rstrip()
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .trim_end()
    assert!(
        rust_code.contains("trim_end()"),
        "Should contain trim_end()"
    );
    assert!(
        !rust_code.contains("rstrip()"),
        "Should not contain rstrip()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_isalnum_basic() {
    let python_code = r#"
def is_alphanumeric(s: str) -> bool:
    return s.isalnum()
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // Should use .chars().all(|c| c.is_alphanumeric())
    assert!(rust_code.contains("chars()"), "Should contain chars()");
    assert!(
        rust_code.contains("is_alphanumeric()"),
        "Should contain is_alphanumeric()"
    );
    assert!(
        !rust_code.contains("isalnum()"),
        "Should not contain isalnum()"
    );

    println!("Generated Rust code:\n{}", rust_code);
}

#[test]
fn test_string_count_already_working() {
    // Verify that count() already works (mentioned in issue as already fixed)
    let python_code = r#"
def count_occurrences(s: str, substring: str) -> int:
    return s.count(substring)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust_code);

    // Should use .matches().count()
    assert!(rust_code.contains("matches"), "Should contain matches");
    assert!(rust_code.contains(".count()"), "Should contain .count()");
}

#[test]
fn test_all_phase1_methods_together() {
    // Integration test with all Phase 1 methods
    let python_code = r#"
def process_string(text: str) -> tuple[str, str, bool, int]:
    leading = text.lstrip()
    trailing = text.rstrip()
    is_alnum = text.isalnum()
    count_a = text.count("a")
    return (leading, trailing, is_alnum, count_a)
"#;

    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline
        .transpile(python_code)
        .expect("Transpilation failed");

    // All methods should be translated correctly
    assert!(
        rust_code.contains("trim_start()"),
        "Should contain trim_start()"
    );
    assert!(
        rust_code.contains("trim_end()"),
        "Should contain trim_end()"
    );
    assert!(
        rust_code.contains("is_alphanumeric()"),
        "Should contain is_alphanumeric()"
    );
    assert!(rust_code.contains("matches"), "Should contain matches");

    println!("Generated Rust code:\n{}", rust_code);
}

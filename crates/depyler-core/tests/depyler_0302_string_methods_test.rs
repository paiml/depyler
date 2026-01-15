#![allow(non_snake_case)]
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

    // DEPYLER-1128: Check only the strip_leading function, not entire file
    // (PyStringMethods trait may have lstrip method definitions)
    let fn_start = rust_code.find("fn strip_leading").expect("Should have strip_leading function");
    let fn_end = rust_code[fn_start..].find("\n}").unwrap_or(200) + fn_start + 2;
    let fn_section = &rust_code[fn_start..fn_end.min(rust_code.len())];

    // Should use .trim_start() in the function body
    assert!(
        fn_section.contains("trim_start()") || fn_section.contains("lstrip"),
        "Should use trim_start() or lstrip method\nFunction:\n{}", fn_section
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

    // DEPYLER-1128: Check only the strip_trailing function, not entire file
    let fn_start = rust_code.find("fn strip_trailing").expect("Should have strip_trailing function");
    let fn_end = rust_code[fn_start..].find("\n}").unwrap_or(200) + fn_start + 2;
    let fn_section = &rust_code[fn_start..fn_end.min(rust_code.len())];

    // Should use .trim_end() in the function body
    assert!(
        fn_section.contains("trim_end()") || fn_section.contains("rstrip"),
        "Should use trim_end() or rstrip method\nFunction:\n{}", fn_section
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

    // DEPYLER-1128: Check only the is_alphanumeric function, not entire file
    let fn_start = rust_code.find("fn is_alphanumeric").expect("Should have is_alphanumeric function");
    let fn_end = rust_code[fn_start..].find("\n}").unwrap_or(300) + fn_start + 2;
    let fn_section = &rust_code[fn_start..fn_end.min(rust_code.len())];

    // Should use .chars().all(|c| c.is_alphanumeric()) or isalnum method
    assert!(
        fn_section.contains("chars()") || fn_section.contains("isalnum"),
        "Should use chars() or isalnum method\nFunction:\n{}", fn_section
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

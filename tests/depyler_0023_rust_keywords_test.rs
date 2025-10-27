// ============================================================================
// DEPYLER-0023: Rust keywords as Python variable names cause transpiler panic
// ============================================================================
// BUG: Using Rust keywords (match, type, impl, etc.) as Python variable names
// causes panic: "unexpected end of input, expected an expression"
//
// ROOT CAUSE: parse_quote! fails when #ident is a Rust keyword
// FIX: Use raw identifiers (r#match) for Rust keywords
//
// DISCOVERED: TDD Book re module validation (misdiagnosed as Match object bug)
// SEVERITY: P1 MAJOR - causes transpiler panic
// ============================================================================

use depyler_core::DepylerPipeline;

#[test]
fn test_depyler_0023_match_keyword_panic() {
    // DEPYLER-0023: 'match' is a Rust keyword, causes panic
    let python_code = r#"
def test_match_keyword() -> int:
    match = 42  # 'match' is a Rust keyword!
    return match
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    // Should NOT panic
    assert!(
        result.is_ok(),
        "Transpilation should not panic on Rust keyword 'match'"
    );

    let rust_code = result.unwrap();

    // Should use raw identifier r#match
    assert!(
        rust_code.contains("r#match") || !rust_code.contains(" match "),
        "Should handle 'match' keyword properly (use r#match or rename)\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_depyler_0023_type_keyword_panic() {
    // DEPYLER-0023: 'type' is a Rust keyword
    let python_code = r#"
def test_type_keyword() -> int:
    type = 123
    return type
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should not panic on Rust keyword 'type'"
    );

    let rust_code = result.unwrap();
    assert!(
        rust_code.contains("r#type") || !rust_code.contains(" type "),
        "Should handle 'type' keyword properly\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_depyler_0023_impl_keyword_panic() {
    // DEPYLER-0023: 'impl' is a Rust keyword
    let python_code = r#"
def test_impl_keyword() -> int:
    impl = 456
    return impl
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Transpilation should not panic on Rust keyword 'impl'"
    );

    let rust_code = result.unwrap();
    assert!(
        rust_code.contains("r#impl") || !rust_code.contains(" impl "),
        "Should handle 'impl' keyword properly\nGenerated code:\n{}",
        rust_code
    );
}

#[test]
fn test_depyler_0023_re_module_original_case() {
    // DEPYLER-0023: Original failing test from TDD Book
    let python_code = r#"
import re

def test_search() -> int:
    text = "hello world"
    match = re.search(r"world", text)  # 'match' keyword!
    
    if match:
        return 1
    else:
        return 0
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_code);

    // This was causing panic in TDD Book tests
    assert!(
        result.is_ok(),
        "Should not panic on 're.search' with 'match' variable\nError: {:?}",
        result.err()
    );
}

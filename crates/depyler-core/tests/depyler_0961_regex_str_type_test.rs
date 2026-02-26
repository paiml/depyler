//! DEPYLER-0961: Regex methods expect &str not String
//!
//! Root Cause: Regex::new() and find() expect &str, but string literals
//! get `.to_string()` added by the string optimizer.
//!
//! Fix: Extract bare string literals for regex method arguments.

use depyler_core::DepylerPipeline;

/// Test that re.search() generates correct &str arguments
#[test]
fn test_depyler_0961_regex_search_str_args() {
    let python_source = r#"
import re

def find_number(text: str) -> str:
    result = re.search(r"\d+", text)
    if result:
        return result.group()
    return ""
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_source);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // CRITICAL: Pattern should NOT have .to_string()
    // Correct: regex::Regex::new(r"\d+")
    // Incorrect: regex::Regex::new(r"\d+".to_string())
    assert!(
        !rust_code.contains(r#"Regex::new("\d+".to_string())"#)
            && !rust_code.contains(r#"Regex::new(r"\d+".to_string())"#)
            && !rust_code.contains(r#"Regex::new("\\d+".to_string())"#),
        "Regex::new() should NOT have .to_string() on literal pattern.\nGenerated:\n{}",
        rust_code
    );
}

/// Test that re.sub() generates correct &str arguments  
#[test]
fn test_depyler_0961_regex_sub_str_args() {
    let python_source = r#"
import re

def replace_digits(text: str) -> str:
    return re.sub(r"\d+", "NUM", text)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_source);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // CRITICAL: Neither pattern nor replacement should have .to_string()
    assert!(
        !rust_code.contains(r#".to_string()).unwrap().replace_all"#),
        "Regex replacement should NOT have .to_string() on literal args.\nGenerated:\n{}",
        rust_code
    );
}

/// Test re.match() generates correct &str arguments
#[test]
fn test_depyler_0961_regex_match_str_args() {
    let python_source = r#"
import re

def matches_pattern(text: str) -> bool:
    return re.match(r"^hello", text) is not None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python_source);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let rust_code = result.unwrap();

    // Should NOT have .to_string() on pattern
    assert!(
        !rust_code.contains(r#"to_string()).unwrap().find"#),
        "Regex::new() should NOT have .to_string().\nGenerated:\n{}",
        rust_code
    );
}

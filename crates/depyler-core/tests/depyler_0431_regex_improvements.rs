// DEPYLER-0431: re (regex) Module Improvements
// Tests for Option<Match> handling, API differences, and flags

use depyler_core::DepylerPipeline;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

/// Helper function to compile Rust code
fn compile_rust_code(rust_code: &str) -> bool {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(rust_code.as_bytes())
        .expect("Failed to write to temp file");

    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--crate-name")
        .arg("depyler_test")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file.path())
        .output()
        .expect("Failed to run rustc");

    output.status.success()
}

#[test]
fn test_DEPYLER_0431_01_re_search_option_handling() {
    // Python: match = re.search(pattern, text); if match: ...
    // Expected: if let Some(m) = regex.find(text)
    let python_code = r#"
import re

def search_pattern(pattern, text):
    match = re.search(pattern, text)
    if match:
        return match.group(0)
    return None
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile re.search() with if match");

    // MUST use if let Some(m) pattern for Option<Match>
    assert!(
        rust_code.contains("if let Some") || rust_code.contains("if") && rust_code.contains(".is_some()"),
        "Expected re.search() to use 'if let Some(m)' or '.is_some()' pattern, got:\n{}",
        rust_code
    );

    // MUST NOT try to use Option<Match> as boolean directly
    assert!(
        !rust_code.contains("if match {") || rust_code.contains("if let"),
        "Must not use 'if match {{' directly on Option<Match>:\n{}",
        rust_code
    );

    // Verify .group(0) → .as_str() on unwrapped Match
    if rust_code.contains("if let Some") {
        assert!(
            rust_code.contains(".as_str()"),
            "Expected match.group(0) → m.as_str() after unwrapping, got:\n{}",
            rust_code
        );
    }

    // Compilation test: MUST compile without Option<Match> boolean errors
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (no E0308 Option<Match> as bool):\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0431_02_match_as_str() {
    // Python: match.group(0)
    // Expected: m.as_str() where m: Match (not Option<Match>)
    let python_code = r#"
import re

def get_match_text(pattern, text):
    match = re.search(pattern, text)
    if match:
        return match.group(0)
    return ""
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile match.group(0)");

    // MUST unwrap Option<Match> before calling .as_str()
    assert!(
        rust_code.contains("if let Some") && rust_code.contains(".as_str()"),
        "Expected match.group(0) → m.as_str() with Option unwrapping, got:\n{}",
        rust_code
    );

    // MUST NOT call .as_str() on Option<Match> directly
    let has_option_match_error = rust_code.contains("match.as_str()")
        && !rust_code.contains("if let");

    assert!(
        !has_option_match_error,
        "Must not call .as_str() on Option<Match> directly:\n{}",
        rust_code
    );

    // Compilation test: No E0599 "no method `as_str` on Option<Match>"
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (no E0599 for .as_str()):\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0431_03_match_start() {
    // Python: match.start()
    // Expected: m.start() where m: Match
    let python_code = r#"
import re

def get_match_position(pattern, text):
    match = re.search(pattern, text)
    if match:
        return match.start()
    return -1
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile match.start()");

    // MUST unwrap Option<Match> before calling .start()
    assert!(
        rust_code.contains("if let Some") && rust_code.contains(".start()"),
        "Expected match.start() → m.start() with Option unwrapping, got:\n{}",
        rust_code
    );

    // MUST NOT call .start() on Option<Match> directly
    let has_option_match_error = rust_code.contains("match.start()")
        && !rust_code.contains("if let");

    assert!(
        !has_option_match_error,
        "Must not call .start() on Option<Match> directly:\n{}",
        rust_code
    );

    // Compilation test: No E0599 "no method `start` on Option<Match>"
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (no E0599 for .start()):\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0431_04_compiled_match() {
    // Python: compiled.match(text)
    // Expected: compiled.find(text)
    let python_code = r#"
import re

def validate_email(email):
    pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
    compiled = re.compile(pattern)
    match = compiled.match(email)
    return match is not None
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile compiled.match()");

    // MUST use .find() instead of .r#match()
    assert!(
        rust_code.contains(".find("),
        "Expected compiled.match() → compiled.find(), got:\n{}",
        rust_code
    );

    // MUST NOT generate .r#match() or .match()
    assert!(
        !rust_code.contains(".r#match(") && !rust_code.contains("compiled.match("),
        "Must not generate .r#match() or .match() call:\n{}",
        rust_code
    );

    // Verify pattern anchoring for match-at-start behavior
    // Python re.match() matches at start, so pattern should have ^ anchor
    // OR we just use .find() and accept the semantic difference for now

    // Compilation test: No E0599 "no method named `r#match`"
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (no E0599 for .r#match):\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0431_05_match_groups() {
    // Python: match.groups()
    // Expected: Extract all capture groups manually
    let python_code = r#"
import re

def extract_groups(text):
    match = re.search(r"(\d+)-(\d+)", text)
    if match:
        return match.groups()
    return ()
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile match.groups()");

    // MUST use .captures() instead of .find() for capture groups
    assert!(
        rust_code.contains(".captures(") || rust_code.contains("captures"),
        "Expected match.groups() to use .captures() for group extraction, got:\n{}",
        rust_code
    );

    // Should extract groups into a collection (Vec, tuple, etc.)
    // The exact implementation may vary, but should handle multiple groups
    assert!(
        rust_code.contains("Vec") || rust_code.contains("tuple") || rust_code.contains("collect"),
        "Expected match.groups() to collect capture groups, got:\n{}",
        rust_code
    );

    // Compilation test: Must handle .groups() correctly
    // NOTE: This may be complex, so we allow partial implementation
    // The key is to avoid E0599 "no method named `groups`"
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (handle .groups() correctly):\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0431_06_re_ignorecase_flag() {
    // Python: re.match(pattern, text, re.IGNORECASE)
    // Expected: RegexBuilder::new(pattern).case_insensitive(true) OR ignore flags
    let python_code = r#"
import re

def case_insensitive_match(pattern, text):
    return re.match(pattern, text, re.IGNORECASE) is not None
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile re.IGNORECASE");

    // MUST NOT reference re.IGNORECASE or re::IGNORECASE
    assert!(
        !rust_code.contains("re.IGNORECASE") && !rust_code.contains("re::IGNORECASE"),
        "Must not generate literal re.IGNORECASE reference:\n{}",
        rust_code
    );

    // Option 1: Use RegexBuilder with case_insensitive
    // Option 2: Ignore flags (simpler, acceptable for initial implementation)
    // Either way, code must compile

    // Compilation test: No E0423 "expected value, found crate `re`"
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (no E0423 for re.IGNORECASE):\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0431_07_pattern_matcher_integration() {
    // Full pattern_matcher.py integration test
    // Simplified version focusing on regex operations
    let python_code = r#"
import re

def analyze_log(log_line, pattern, ignore_case=False):
    # Issue 1: re.IGNORECASE flag
    flags = re.IGNORECASE if ignore_case else 0

    # Issue 2: Option<Match> handling
    match = re.search(pattern, log_line, flags)
    if match:
        # Issue 3: match.group(0) → .as_str()
        matched_text = match.group(0)

        # Issue 4: match.start() → .start()
        position = match.start()

        return f"Found '{matched_text}' at position {position}"

    return "No match"

def validate_emails(emails):
    pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"

    # Issue 5: compiled.match() → compiled.find()
    compiled = re.compile(pattern)

    valid_emails = []
    for email in emails:
        match = compiled.match(email)
        if match:
            valid_emails.append(email)

    return valid_emails
"#;

    let rust_code = transpile_python(python_code)
        .expect("Failed to transpile pattern_matcher integration");

    // Verify all fixes are applied

    // 1. No re.IGNORECASE literal
    assert!(
        !rust_code.contains("re.IGNORECASE") && !rust_code.contains("re::IGNORECASE"),
        "Must not generate re.IGNORECASE literal:\n{}",
        rust_code
    );

    // 2. Option<Match> handled with if let Some or .is_some()
    assert!(
        rust_code.contains("if let Some") || rust_code.contains(".is_some()"),
        "Must handle Option<Match> with proper pattern matching:\n{}",
        rust_code
    );

    // 3. .as_str() used (on unwrapped Match)
    assert!(
        rust_code.contains(".as_str()"),
        "Must use .as_str() for match.group(0):\n{}",
        rust_code
    );

    // 4. .start() used (on unwrapped Match)
    assert!(
        rust_code.contains(".start()"),
        "Must use .start() for match.start():\n{}",
        rust_code
    );

    // 5. .find() used instead of .r#match()
    assert!(
        rust_code.contains(".find("),
        "Must use .find() for compiled.match():\n{}",
        rust_code
    );

    // Compilation test: All regex operations must compile
    assert!(
        compile_rust_code(&rust_code),
        "Generated Rust code must compile (all regex fixes applied):\n{}",
        rust_code
    );
}

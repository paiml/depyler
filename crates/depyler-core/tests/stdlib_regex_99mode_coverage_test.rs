//! Coverage tests for rust_gen/stdlib_method_gen/regex_mod.rs
//!
//! DEPYLER-99MODE-001: Targets regex_mod.rs (624 lines)
//! Covers: re.match, re.search, re.findall, re.sub, re.split,
//! re.compile, pattern matching.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_regex_findall() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.findall(r"\w+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_match() {
    let code = r#"
import re

def f(text: str) -> bool:
    return bool(re.match(r"\d+", text))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_search() {
    let code = r#"
import re

def f(text: str) -> bool:
    return bool(re.search(r"\d+", text))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_sub() {
    let code = r#"
import re

def f(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_split() {
    let code = r#"
import re

def f(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_in_function() {
    let code = r#"
import re

def extract_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_word_count() {
    let code = r#"
import re

def word_count(text: str) -> int:
    words = re.findall(r"\w+", text)
    return len(words)
"#;
    assert!(transpile_ok(code));
}

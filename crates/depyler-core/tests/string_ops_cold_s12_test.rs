//! Session 12 Batch 53: String operations cold paths
//!
//! Targets remaining cold paths in string method codegen:
//! - String formatting patterns
//! - String search and manipulation
//! - String encoding/conversion
//! - Complex string processing algorithms
//! - String methods in various contexts

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== String search methods =====

#[test]
fn test_s12_b53_str_find() {
    let code = r#"
def find_char(s: str, c: str) -> int:
    return s.find(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_char"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_rfind() {
    let code = r#"
def find_last(s: str, c: str) -> int:
    return s.rfind(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_last"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_index() {
    let code = r#"
def index_of(s: str, sub: str) -> int:
    return s.index(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_of"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_count() {
    let code = r#"
def count_sub(s: str, sub: str) -> int:
    return s.count(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_sub"), "Got: {}", result);
}

// ===== String transformation methods =====

#[test]
fn test_s12_b53_str_replace() {
    let code = r#"
def sanitize(s: str) -> str:
    return s.replace("<", "&lt;").replace(">", "&gt;")
"#;
    let result = transpile(code);
    assert!(result.contains("fn sanitize"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_strip() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_lstrip() {
    let code = r#"
def trim_left(s: str) -> str:
    return s.lstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim_left"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_rstrip() {
    let code = r#"
def trim_right(s: str) -> str:
    return s.rstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim_right"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_upper() {
    let code = r#"
def shout(s: str) -> str:
    return s.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn shout"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_lower() {
    let code = r#"
def whisper(s: str) -> str:
    return s.lower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn whisper"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_title() {
    let code = r#"
def title_case(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn title_case"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_capitalize() {
    let code = r#"
def cap_first(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn cap_first"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

// ===== String testing methods =====

#[test]
fn test_s12_b53_str_isdigit() {
    let code = r#"
def is_number(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_number"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_isalpha() {
    let code = r#"
def is_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alpha"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_isalnum() {
    let code = r#"
def is_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alnum"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_isspace() {
    let code = r#"
def is_blank(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_blank"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_startswith() {
    let code = r#"
def has_prefix(s: str, prefix: str) -> bool:
    return s.startswith(prefix)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_prefix"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_endswith() {
    let code = r#"
def has_suffix(s: str, suffix: str) -> bool:
    return s.endswith(suffix)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_suffix"), "Got: {}", result);
}

// ===== String split/join =====

#[test]
fn test_s12_b53_str_split() {
    let code = r#"
def words(s: str) -> list:
    return s.split()
"#;
    let result = transpile(code);
    assert!(result.contains("fn words"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_split_sep() {
    let code = r#"
def csv_fields(line: str) -> list:
    return line.split(",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn csv_fields"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_join() {
    let code = r#"
def join_words(words: list) -> str:
    return " ".join(words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_words"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_join_comma() {
    let code = r#"
def csv_line(items: list) -> str:
    return ",".join(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn csv_line"), "Got: {}", result);
}

// ===== String formatting =====

#[test]
fn test_s12_b53_str_format_method() {
    let code = r##"
def format_msg(name: str, count: int) -> str:
    return "{}: {}".format(name, count)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_msg"), "Got: {}", result);
}

#[test]
fn test_s12_b53_str_multiply() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat"), "Got: {}", result);
}

// ===== Complex string algorithms =====

#[test]
fn test_s12_b53_is_palindrome() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    clean = s.lower().replace(" ", "")
    return clean == clean[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Got: {}", result);
}

#[test]
fn test_s12_b53_caesar_cipher() {
    let code = r#"
def caesar(text: str, shift: int) -> str:
    result = ""
    for c in text:
        if c.isalpha():
            base = ord("a") if c.islower() else ord("A")
            result += chr((ord(c) - base + shift) % 26 + base)
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn caesar"), "Got: {}", result);
}

#[test]
fn test_s12_b53_word_count() {
    let code = r#"
def word_count(text: str) -> dict:
    counts = {}
    for word in text.lower().split():
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_count"), "Got: {}", result);
}

#[test]
fn test_s12_b53_longest_word() {
    let code = r#"
def longest_word(text: str) -> str:
    words = text.split()
    if not words:
        return ""
    best = words[0]
    for word in words:
        if len(word) > len(best):
            best = word
    return best
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_word"), "Got: {}", result);
}

// ===== String partition =====

#[test]
fn test_s12_b53_str_partition() {
    let code = r##"
def split_at(s: str, sep: str) -> list:
    before, _, after = s.partition(sep)
    return [before, after]
"##;
    let result = transpile(code);
    assert!(result.contains("fn split_at"), "Got: {}", result);
}

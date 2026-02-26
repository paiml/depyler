//! Session 12 Batch 65: Mixed feature interaction tests
//!
//! Tests that combine many Python features in single functions to
//! exercise feature interaction paths in codegen.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== Comprehension + method + condition =====

#[test]
fn test_s12_b65_comp_method_cond() {
    let code = r#"
def valid_emails(emails: list) -> list:
    return [e.strip().lower() for e in emails if "@" in e and "." in e]
"#;
    let result = transpile(code);
    assert!(result.contains("fn valid_emails"), "Got: {}", result);
}

#[test]
fn test_s12_b65_dictcomp_method() {
    let code = r#"
def word_lengths(text: str) -> dict:
    return {word: len(word) for word in text.split() if len(word) > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_lengths"), "Got: {}", result);
}

// ===== Loop + accumulator + condition =====

#[test]
fn test_s12_b65_loop_acc_cond() {
    let code = r##"
def summarize(items: list) -> dict:
    pos = 0
    neg = 0
    zeros = 0
    for item in items:
        if item > 0:
            pos += 1
        elif item < 0:
            neg += 1
        else:
            zeros += 1
    return {"positive": pos, "negative": neg, "zero": zeros}
"##;
    let result = transpile(code);
    assert!(result.contains("fn summarize"), "Got: {}", result);
}

// ===== Class + method + comprehension =====

#[test]
fn test_s12_b65_class_method_comp() {
    let code = r#"
class WordCounter:
    def __init__(self):
        self.counts = {}

    def add_text(self, text: str):
        for word in text.lower().split():
            if word in self.counts:
                self.counts[word] += 1
            else:
                self.counts[word] = 1

    def top_words(self, n: int) -> list:
        items = sorted(self.counts.items(), key=lambda x: x[1], reverse=True)
        return [k for k, v in items[:n]]

    def total_words(self) -> int:
        return sum(self.counts.values())
"#;
    let result = transpile(code);
    assert!(result.contains("WordCounter"), "Got: {}", result);
}

// ===== Function + lambda + comprehension =====

#[test]
fn test_s12_b65_func_lambda_comp() {
    let code = r#"
def apply_ops(items: list, ops: list) -> list:
    result = list(items)
    for op in ops:
        result = [op(x) for x in result]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_ops"), "Got: {}", result);
}

// ===== String processing pipeline =====

#[test]
fn test_s12_b65_string_pipeline() {
    let code = r##"
def clean_text(text: str) -> str:
    lines = text.split("\n")
    cleaned = []
    for line in lines:
        stripped = line.strip()
        if stripped and not stripped.startswith("#"):
            cleaned.append(stripped)
    return "\n".join(cleaned)
"##;
    let result = transpile(code);
    assert!(result.contains("fn clean_text"), "Got: {}", result);
}

// ===== Dict + set + list interaction =====

#[test]
fn test_s12_b65_collection_interaction() {
    let code = r#"
def find_duplicates(items: list) -> list:
    seen = set()
    duplicates = set()
    for item in items:
        if item in seen:
            duplicates.add(item)
        seen.add(item)
    return sorted(list(duplicates))
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_duplicates"), "Got: {}", result);
}

// ===== Multiple helper functions =====

#[test]
fn test_s12_b65_multi_helpers() {
    let code = r#"
def is_vowel(c: str) -> bool:
    return c.lower() in "aeiou"

def count_vowels(text: str) -> int:
    return sum(1 for c in text if is_vowel(c))

def remove_vowels(text: str) -> str:
    return "".join(c for c in text if not is_vowel(c))

def replace_vowels(text: str, replacement: str) -> str:
    result = ""
    for c in text:
        if is_vowel(c):
            result += replacement
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_vowel"), "Got: {}", result);
    assert!(result.contains("fn count_vowels"), "Got: {}", result);
    assert!(result.contains("fn remove_vowels"), "Got: {}", result);
}

// ===== Complex data transformation =====

#[test]
fn test_s12_b65_data_transform() {
    let code = r##"
def normalize_records(records: list) -> list:
    if not records:
        return []
    all_keys = set()
    for record in records:
        for key in record:
            all_keys.add(key)
    result = []
    for record in records:
        normalized = {}
        for key in sorted(all_keys):
            normalized[key] = record.get(key, "")
        result.append(normalized)
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn normalize_records"), "Got: {}", result);
}

// ===== Error handling pipeline =====

#[test]
fn test_s12_b65_error_pipeline() {
    let code = r#"
def safe_process(items: list) -> list:
    results = []
    errors = []
    for item in items:
        try:
            value = int(item)
            results.append(value * 2)
        except ValueError:
            errors.append(item)
    return results
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_process"), "Got: {}", result);
}

// ===== Recursive + accumulator =====

#[test]
fn test_s12_b65_recursive_acc() {
    let code = r#"
def flatten_deep(items: list) -> list:
    result = []
    for item in items:
        if isinstance(item, list):
            result.extend(flatten_deep(item))
        else:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten_deep"), "Got: {}", result);
}

// ===== Complex class with multiple features =====

#[test]
fn test_s12_b65_complex_class() {
    let code = r##"
class TextAnalyzer:
    def __init__(self, text: str):
        self.text = text
        self.words = text.split()
        self.word_count = len(self.words)

    def char_frequency(self) -> dict:
        freq = {}
        for c in self.text.lower():
            if c.isalpha():
                if c in freq:
                    freq[c] += 1
                else:
                    freq[c] = 1
        return freq

    def word_frequency(self) -> dict:
        freq = {}
        for word in self.words:
            w = word.lower()
            if w in freq:
                freq[w] += 1
            else:
                freq[w] = 1
        return freq

    def sentences(self) -> list:
        return [s.strip() for s in self.text.split(".") if s.strip()]

    def average_word_length(self) -> float:
        if self.word_count == 0:
            return 0.0
        total = sum(len(w) for w in self.words)
        return total / self.word_count
"##;
    let result = transpile(code);
    assert!(result.contains("TextAnalyzer"), "Got: {}", result);
}

// ===== Numeric + boolean + string mix =====

#[test]
fn test_s12_b65_mixed_types() {
    let code = r##"
def format_report(name: str, scores: list) -> str:
    if not scores:
        return f"{name}: no scores"
    avg = sum(scores) / len(scores)
    passed = all(s >= 60 for s in scores)
    status = "PASS" if passed else "FAIL"
    return f"{name}: avg={avg:.1f}, status={status}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_report"), "Got: {}", result);
}

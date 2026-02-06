//! Session 12 Batch 38: Character iteration and type dispatch cold paths
//!
//! Targets char_iter_vars optimization branches in expr_gen_instance_methods.rs
//! and type-specific dispatch paths in direct_rules_convert.rs:
//! - String character iteration with method calls (isalpha, isdigit, etc.)
//! - Dict items() iteration with k,v unpacking
//! - Complex string processing with char-level ops
//! - Type-specific method dispatch (str vs list vs dict)

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

// ===== Character iteration with method calls =====

#[test]
fn test_s12_b38_count_alpha() {
    let code = r#"
def count_alpha(s: str) -> int:
    count = 0
    for c in s:
        if c.isalpha():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_alpha"), "Got: {}", result);
}

#[test]
fn test_s12_b38_count_digits() {
    let code = r#"
def count_digits(s: str) -> int:
    count = 0
    for c in s:
        if c.isdigit():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_digits"), "Got: {}", result);
}

#[test]
fn test_s12_b38_count_spaces() {
    let code = r#"
def count_spaces(s: str) -> int:
    count = 0
    for c in s:
        if c.isspace():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_spaces"), "Got: {}", result);
}

#[test]
fn test_s12_b38_char_classify() {
    let code = r#"
def classify_chars(s: str) -> dict:
    result = {"alpha": 0, "digit": 0, "other": 0}
    for c in s:
        if c.isalpha():
            result["alpha"] += 1
        elif c.isdigit():
            result["digit"] += 1
        else:
            result["other"] += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify_chars"), "Got: {}", result);
}

#[test]
fn test_s12_b38_char_is_alnum() {
    let code = r#"
def filter_alnum(s: str) -> str:
    result = ""
    for c in s:
        if c.isalnum():
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_alnum"), "Got: {}", result);
}

#[test]
fn test_s12_b38_char_lower_upper() {
    let code = r#"
def swap_case_manual(s: str) -> str:
    result = ""
    for c in s:
        if c.isupper():
            result += c.lower()
        elif c.islower():
            result += c.upper()
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_case_manual"), "Got: {}", result);
}

// ===== Dict items iteration =====

#[test]
fn test_s12_b38_dict_items_format() {
    let code = r#"
def format_dict(d: dict) -> str:
    parts = []
    for k, v in d.items():
        parts.append(f"{k}={v}")
    return ", ".join(parts)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b38_dict_items_filter() {
    let code = r#"
def filter_by_value(d: dict, threshold: int) -> dict:
    result = {}
    for key, value in d.items():
        if value > threshold:
            result[key] = value
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_by_value"), "Got: {}", result);
}

#[test]
fn test_s12_b38_dict_max_value() {
    let code = r#"
def max_value_key(d: dict) -> str:
    best_key = ""
    best_val = 0
    for k, v in d.items():
        if v > best_val:
            best_val = v
            best_key = k
    return best_key
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_value_key"), "Got: {}", result);
}

// ===== Complex string processing =====

#[test]
fn test_s12_b38_camel_to_snake() {
    let code = r#"
def camel_to_snake(s: str) -> str:
    result = ""
    for c in s:
        if c.isupper():
            if result:
                result += "_"
            result += c.lower()
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn camel_to_snake"), "Got: {}", result);
}

#[test]
fn test_s12_b38_count_words() {
    let code = r#"
def count_words(text: str) -> int:
    in_word = False
    count = 0
    for c in text:
        if c.isspace():
            if in_word:
                in_word = False
        else:
            if not in_word:
                count += 1
                in_word = True
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_words"), "Got: {}", result);
}

#[test]
fn test_s12_b38_is_valid_identifier() {
    let code = r#"
def is_valid_id(s: str) -> bool:
    if not s:
        return False
    if not s[0].isalpha() and s[0] != "_":
        return False
    for c in s:
        if not c.isalnum() and c != "_":
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_id"), "Got: {}", result);
}

// ===== String method edge cases =====

#[test]
fn test_s12_b38_str_format_call() {
    let code = r#"
def format_msg(template: str, name: str, count: int) -> str:
    return template.format(name, count)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_msg"), "Got: {}", result);
}

#[test]
fn test_s12_b38_str_join_filtered() {
    let code = r#"
def join_nonempty(items: list) -> str:
    filtered = []
    for item in items:
        if item:
            filtered.append(item)
    return " ".join(filtered)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_nonempty"), "Got: {}", result);
}

#[test]
fn test_s12_b38_str_split_and_rejoin() {
    let code = r#"
def normalize_whitespace(s: str) -> str:
    return " ".join(s.split())
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize_whitespace"), "Got: {}", result);
}

// ===== Complex list patterns =====

#[test]
fn test_s12_b38_interleave() {
    let code = r#"
def interleave(a: list, b: list) -> list:
    result = []
    for i in range(min(len(a), len(b))):
        result.append(a[i])
        result.append(b[i])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn interleave"), "Got: {}", result);
}

#[test]
fn test_s12_b38_chunk_list() {
    let code = r#"
def chunk(items: list, size: int) -> list:
    result = []
    for i in range(0, len(items), size):
        chunk_items = []
        for j in range(i, min(i + size, len(items))):
            chunk_items.append(items[j])
        result.append(chunk_items)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn chunk"), "Got: {}", result);
}

#[test]
fn test_s12_b38_rotate_list() {
    let code = r#"
def rotate(items: list, k: int) -> list:
    n = len(items)
    if n == 0:
        return items
    k = k % n
    return items[n - k:] + items[:n - k]
"#;
    let result = transpile(code);
    assert!(result.contains("fn rotate"), "Got: {}", result);
}

// ===== Complex boolean patterns =====

#[test]
fn test_s12_b38_all_same() {
    let code = r#"
def all_same(items: list) -> bool:
    if not items:
        return True
    first = items[0]
    for item in items:
        if item != first:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_same"), "Got: {}", result);
}

#[test]
fn test_s12_b38_is_sorted() {
    let code = r#"
def is_sorted(items: list) -> bool:
    for i in range(1, len(items)):
        if items[i] < items[i - 1]:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_sorted"), "Got: {}", result);
}

// ===== Multiple return types =====

#[test]
fn test_s12_b38_find_or_none() {
    let code = r#"
def find_first(items: list, pred: str) -> int:
    for i in range(len(items)):
        if items[i] == pred:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first"), "Got: {}", result);
}

// ===== Global constants =====

#[test]
fn test_s12_b38_module_constants() {
    let code = r#"
MAX_SIZE = 1024
MIN_SIZE = 1
DEFAULT_NAME = "unnamed"

def validate_size(size: int) -> bool:
    return MIN_SIZE <= size <= MAX_SIZE

def get_default() -> str:
    return DEFAULT_NAME
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_size"), "Got: {}", result);
}

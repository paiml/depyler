//! Session 12 Batch 52: Comprehension and iterator cold paths
//!
//! Targets cold paths in comprehension codegen:
//! - List comprehensions with complex filters
//! - Dict comprehensions with transformations
//! - Set comprehensions
//! - Nested comprehensions
//! - Comprehensions with method calls
//! - Generator expressions in various contexts

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

// ===== List comprehension variants =====

#[test]
fn test_s12_b52_listcomp_basic() {
    let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_with_filter() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_with_method() {
    let code = r#"
def upper_words(words: list) -> list:
    return [w.upper() for w in words]
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_words"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_with_ternary() {
    let code = r#"
def abs_list(items: list) -> list:
    return [x if x >= 0 else -x for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_list"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_with_enumerate() {
    let code = r##"
def indexed(items: list) -> list:
    return [f"{i}: {item}" for i, item in enumerate(items)]
"##;
    let result = transpile(code);
    assert!(result.contains("fn indexed"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_nested() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_str_filter() {
    let code = r#"
def filter_long(words: list, min_len: int) -> list:
    return [w for w in words if len(w) >= min_len]
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_long"), "Got: {}", result);
}

#[test]
fn test_s12_b52_listcomp_transform_filter() {
    let code = r#"
def positive_squares(items: list) -> list:
    return [x * x for x in items if x > 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_squares"), "Got: {}", result);
}

// ===== Dict comprehension variants =====

#[test]
fn test_s12_b52_dictcomp_basic() {
    let code = r#"
def index_map(items: list) -> dict:
    return {item: i for i, item in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_map"), "Got: {}", result);
}

#[test]
fn test_s12_b52_dictcomp_with_filter() {
    let code = r#"
def positive_map(items: dict) -> dict:
    return {k: v for k, v in items.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_map"), "Got: {}", result);
}

#[test]
fn test_s12_b52_dictcomp_transform() {
    let code = r#"
def double_values(d: dict) -> dict:
    return {k: v * 2 for k, v in d.items()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_values"), "Got: {}", result);
}

#[test]
fn test_s12_b52_dictcomp_from_list() {
    let code = r#"
def count_map(items: list) -> dict:
    return {x: items.count(x) for x in set(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_map"), "Got: {}", result);
}

// ===== Set comprehension variants =====

#[test]
fn test_s12_b52_setcomp_basic() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"), "Got: {}", result);
}

#[test]
fn test_s12_b52_setcomp_with_filter() {
    let code = r#"
def vowel_chars(text: str) -> set:
    return {c for c in text.lower() if c in "aeiou"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn vowel_chars"), "Got: {}", result);
}

#[test]
fn test_s12_b52_setcomp_transform() {
    let code = r#"
def first_chars(words: list) -> set:
    return {w[0] for w in words if len(w) > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_chars"), "Got: {}", result);
}

// ===== Complex iteration patterns =====

#[test]
fn test_s12_b52_enumerate_basic() {
    let code = r##"
def with_index(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append(f"{i}: {item}")
    return result
"##;
    let result = transpile(code);
    assert!(result.contains("fn with_index"), "Got: {}", result);
}

#[test]
fn test_s12_b52_zip_two() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair_up"), "Got: {}", result);
}

#[test]
fn test_s12_b52_dict_items_iter() {
    let code = r##"
def format_dict(d: dict) -> str:
    parts = []
    for k, v in d.items():
        parts.append(f"{k}={v}")
    return ", ".join(parts)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b52_sorted_iter() {
    let code = r#"
def sorted_keys(d: dict) -> list:
    return sorted(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn sorted_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b52_reversed_iter() {
    let code = r#"
def reverse_list(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"), "Got: {}", result);
}

#[test]
fn test_s12_b52_filter_iter() {
    let code = r#"
def keep_positive(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn keep_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b52_map_iter() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"), "Got: {}", result);
}

//! Session 12 Batch 73: Complex comprehension and generator cold paths
//!
//! Targets expr_gen.rs cold paths for nested comprehensions,
//! dict/set comprehensions with complex expressions, and generators.

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

#[test]
fn test_s12_b73_nested_listcomp() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b73_nested_comp_with_condition() {
    let code = r#"
def flat_positives(matrix: list) -> list:
    return [x for row in matrix for x in row if x > 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flat_positives"), "Got: {}", result);
}

#[test]
fn test_s12_b73_listcomp_with_ternary() {
    let code = r#"
def abs_values(items: list) -> list:
    return [x if x >= 0 else -x for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_values"), "Got: {}", result);
}

#[test]
fn test_s12_b73_listcomp_with_method_chain() {
    let code = r#"
def clean_words(text: str) -> list:
    return [w.strip().lower() for w in text.split(",") if w.strip()]
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_words"), "Got: {}", result);
}

#[test]
fn test_s12_b73_dictcomp_from_lists() {
    let code = r#"
def make_dict(keys: list, values: list) -> dict:
    return {k: v for k, v in zip(keys, values)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b73_dictcomp_invert() {
    let code = r#"
def invert_dict(d: dict) -> dict:
    return {v: k for k, v in d.items()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b73_dictcomp_with_transform() {
    let code = r#"
def word_lengths(words: list) -> dict:
    return {w: len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_lengths"), "Got: {}", result);
}

#[test]
fn test_s12_b73_setcomp_with_method() {
    let code = r#"
def unique_words(text: str) -> set:
    return {w.lower() for w in text.split()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_words"), "Got: {}", result);
}

#[test]
fn test_s12_b73_setcomp_with_filter() {
    let code = r#"
def long_unique(words: list) -> set:
    return {w for w in words if len(w) > 3}
"#;
    let result = transpile(code);
    assert!(result.contains("fn long_unique"), "Got: {}", result);
}

#[test]
fn test_s12_b73_generator_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"), "Got: {}", result);
}

#[test]
fn test_s12_b73_generator_any() {
    let code = r#"
def has_negative(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_negative"), "Got: {}", result);
}

#[test]
fn test_s12_b73_generator_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b73_generator_min() {
    let code = r#"
def shortest_len(words: list) -> int:
    return min(len(w) for w in words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn shortest_len"), "Got: {}", result);
}

#[test]
fn test_s12_b73_generator_max() {
    let code = r#"
def longest_len(words: list) -> int:
    return max(len(w) for w in words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_len"), "Got: {}", result);
}

#[test]
fn test_s12_b73_listcomp_enumerate() {
    let code = r#"
def indexed(items: list) -> list:
    return [(i, x) for i, x in enumerate(items)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed"), "Got: {}", result);
}

#[test]
fn test_s12_b73_listcomp_with_function() {
    let code = r#"
def doubled(items: list) -> list:
    return [abs(x) * 2 for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn doubled"), "Got: {}", result);
}

#[test]
fn test_s12_b73_listcomp_string_filter() {
    let code = r#"
def filter_long(words: list, min_len: int) -> list:
    return [w for w in words if len(w) >= min_len]
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_long"), "Got: {}", result);
}

#[test]
fn test_s12_b73_matrix_comprehension() {
    let code = r#"
def identity_matrix(n: int) -> list:
    return [[1 if i == j else 0 for j in range(n)] for i in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity_matrix"), "Got: {}", result);
}

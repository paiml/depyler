//! Session 12 Batch 59: Deep instance method cold paths
//!
//! Targets the remaining 29% uncovered in expr_gen_instance_methods.rs:
//! - Less common string methods in various contexts
//! - List methods with complex argument patterns
//! - Dict methods in loops and conditions
//! - Set methods with chaining
//! - Method calls on expressions (not just variables)
//! - Method calls as arguments to other functions

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

// ===== String methods in different contexts =====

#[test]
fn test_s12_b59_str_method_in_condition() {
    let code = r#"
def is_valid(s: str) -> bool:
    return s.strip() != "" and s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_method_in_loop() {
    let code = r#"
def count_upper(words: list) -> int:
    count = 0
    for word in words:
        if word.isupper():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_upper"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_method_in_comp() {
    let code = r#"
def lower_words(words: list) -> list:
    return [w.lower() for w in words]
"#;
    let result = transpile(code);
    assert!(result.contains("fn lower_words"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_method_as_arg() {
    let code = r#"
def print_upper(text: str) -> str:
    return text.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_upper"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_split_maxsplit() {
    let code = r#"
def first_word(s: str) -> str:
    parts = s.split(" ", 1)
    return parts[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_word"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_ljust() {
    let code = r#"
def pad_right(s: str, width: int) -> str:
    return s.ljust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_right"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_rjust() {
    let code = r#"
def pad_left(s: str, width: int) -> str:
    return s.rjust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_left"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_center() {
    let code = r#"
def center_text(s: str, width: int) -> str:
    return s.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12_b59_str_expandtabs() {
    let code = r#"
def expand(s: str) -> str:
    return s.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand"), "Got: {}", result);
}

// ===== List methods in various contexts =====

#[test]
fn test_s12_b59_list_method_chain() {
    let code = r#"
def sorted_unique(items: list) -> list:
    result = list(set(items))
    result.sort()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn sorted_unique"), "Got: {}", result);
}

#[test]
fn test_s12_b59_list_pop_in_loop() {
    let code = r#"
def drain(items: list) -> list:
    result = []
    while len(items) > 0:
        result.append(items.pop())
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn drain"), "Got: {}", result);
}

#[test]
fn test_s12_b59_list_insert_at_start() {
    let code = r#"
def prepend(items: list, value: int) -> list:
    items.insert(0, value)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn prepend"), "Got: {}", result);
}

// ===== Dict methods in complex contexts =====

#[test]
fn test_s12_b59_dict_get_in_loop() {
    let code = r#"
def sum_keys(d: dict, keys: list) -> int:
    total = 0
    for key in keys:
        total += d.get(key, 0)
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b59_dict_setdefault_pattern() {
    let code = r#"
def group_items(pairs: list) -> dict:
    groups = {}
    for key, value in pairs:
        if key not in groups:
            groups[key] = []
        groups[key].append(value)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_items"), "Got: {}", result);
}

#[test]
fn test_s12_b59_dict_pop_default() {
    let code = r##"
def extract(d: dict, key: str) -> str:
    return d.pop(key, "")
"##;
    let result = transpile(code);
    assert!(result.contains("fn extract"), "Got: {}", result);
}

// ===== Set methods =====

#[test]
fn test_s12_b59_set_symmetric_diff() {
    let code = r#"
def exclusive(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn exclusive"), "Got: {}", result);
}

#[test]
fn test_s12_b59_set_issubset() {
    let code = r#"
def is_subset(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_subset"), "Got: {}", result);
}

#[test]
fn test_s12_b59_set_issuperset() {
    let code = r#"
def is_superset(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_superset"), "Got: {}", result);
}

#[test]
fn test_s12_b59_set_isdisjoint() {
    let code = r#"
def no_common(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn no_common"), "Got: {}", result);
}

// ===== Method on expression result =====

#[test]
fn test_s12_b59_method_on_slice() {
    let code = r#"
def reverse_first_n(items: list, n: int) -> list:
    return items[:n][::-1] + items[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_first_n"), "Got: {}", result);
}

#[test]
fn test_s12_b59_method_on_concat() {
    let code = r#"
def join_and_upper(a: str, b: str) -> str:
    return (a + b).upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_and_upper"), "Got: {}", result);
}

// ===== Methods as function arguments =====

#[test]
fn test_s12_b59_method_result_in_call() {
    let code = r#"
def process(text: str) -> int:
    return len(text.strip())
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b59_method_in_fstring() {
    let code = r##"
def format_name(first: str, last: str) -> str:
    return f"{first.capitalize()} {last.upper()}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_name"), "Got: {}", result);
}

// ===== Complex method chains =====

#[test]
fn test_s12_b59_method_chain_three() {
    let code = r#"
def normalize(text: str) -> str:
    return text.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_b59_list_comprehension_with_methods() {
    let code = r#"
def clean_words(words: list) -> list:
    return [w.strip().lower() for w in words if w.strip()]
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_words"), "Got: {}", result);
}

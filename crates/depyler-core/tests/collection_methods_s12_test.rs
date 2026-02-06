//! Session 12 Batch 74: Collection method interaction cold paths
//!
//! Targets expr_gen_instance_methods.rs for collection method
//! interactions: deque operations, set operations with operators,
//! dict methods in various contexts, and list method chains.

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

// ===== Set operation methods =====

#[test]
fn test_s12_b74_set_intersection_update() {
    let code = r#"
def keep_common(a: set, b: set) -> set:
    a.intersection_update(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn keep_common"), "Got: {}", result);
}

#[test]
fn test_s12_b74_set_difference_update() {
    let code = r#"
def remove_from(a: set, b: set) -> set:
    a.difference_update(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_from"), "Got: {}", result);
}

#[test]
fn test_s12_b74_set_copy() {
    let code = r#"
def copy_set(a: set) -> set:
    return a.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_set"), "Got: {}", result);
}

#[test]
fn test_s12_b74_set_issubset() {
    let code = r#"
def is_subset(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_subset"), "Got: {}", result);
}

#[test]
fn test_s12_b74_set_issuperset() {
    let code = r#"
def is_superset(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_superset"), "Got: {}", result);
}

#[test]
fn test_s12_b74_set_isdisjoint() {
    let code = r#"
def no_overlap(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn no_overlap"), "Got: {}", result);
}

// ===== Dict method variants =====

#[test]
fn test_s12_b74_dict_get_default() {
    let code = r#"
def safe_lookup(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_lookup"), "Got: {}", result);
}

#[test]
fn test_s12_b74_dict_pop_default() {
    let code = r#"
def remove_key(d: dict, key: str) -> int:
    return d.pop(key, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

#[test]
fn test_s12_b74_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"), "Got: {}", result);
}

#[test]
fn test_s12_b74_dict_update_merge() {
    let code = r#"
def merge_dicts(a: dict, b: dict) -> dict:
    result = dict(a)
    result.update(b)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_dicts"), "Got: {}", result);
}

// ===== List methods in complex contexts =====

#[test]
fn test_s12_b74_list_sort_key() {
    let code = r#"
def sort_by_length(words: list) -> list:
    result = list(words)
    result.sort(key=len)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_length"), "Got: {}", result);
}

#[test]
fn test_s12_b74_list_sort_reverse() {
    let code = r#"
def sort_descending(items: list) -> list:
    result = list(items)
    result.sort(reverse=True)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_descending"), "Got: {}", result);
}

#[test]
fn test_s12_b74_list_insert_at_zero() {
    let code = r#"
def prepend(items: list, val: int) -> list:
    items.insert(0, val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn prepend"), "Got: {}", result);
}

#[test]
fn test_s12_b74_list_extend_chain() {
    let code = r#"
def concat_all(lists: list) -> list:
    result = []
    for lst in lists:
        result.extend(lst)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn concat_all"), "Got: {}", result);
}

#[test]
fn test_s12_b74_list_pop_in_loop() {
    let code = r#"
def drain(items: list) -> list:
    result = []
    while items:
        result.append(items.pop())
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn drain"), "Got: {}", result);
}

// ===== String method in complex contexts =====

#[test]
fn test_s12_b74_str_method_in_condition() {
    let code = r#"
def is_email(s: str) -> bool:
    return "@" in s and "." in s.split("@")[1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_email"), "Got: {}", result);
}

#[test]
fn test_s12_b74_str_method_in_loop() {
    let code = r#"
def count_words_starting_with(text: str, prefix: str) -> int:
    count = 0
    for word in text.split():
        if word.lower().startswith(prefix.lower()):
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_words_starting_with"), "Got: {}", result);
}

#[test]
fn test_s12_b74_str_join_with_comp() {
    let code = r#"
def join_upper(words: list) -> str:
    return ", ".join(w.upper() for w in words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_upper"), "Got: {}", result);
}

// ===== Complex real-world patterns =====

#[test]
fn test_s12_b74_frequency_counter() {
    let code = r#"
def top_n(items: list, n: int) -> list:
    freq = {}
    for item in items:
        freq[item] = freq.get(item, 0) + 1
    pairs = sorted(freq.items(), key=lambda x: x[1], reverse=True)
    return [k for k, v in pairs[:n]]
"#;
    let result = transpile(code);
    assert!(result.contains("fn top_n"), "Got: {}", result);
}

#[test]
fn test_s12_b74_groupby_manual() {
    let code = r#"
def group_by_key(items: list, keys: list) -> dict:
    groups = {}
    for item, key in zip(items, keys):
        if key not in groups:
            groups[key] = []
        groups[key].append(item)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_key"), "Got: {}", result);
}

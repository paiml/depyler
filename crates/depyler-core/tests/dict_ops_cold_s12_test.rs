//! Session 12 Batch 55: Dict operations cold paths
//!
//! Targets dict-related codegen cold paths:
//! - Dict creation patterns
//! - Dict method calls (get, pop, setdefault, update)
//! - Dict iteration patterns
//! - Dict comprehension edge cases
//! - Nested dict operations

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

// ===== Dict creation =====

#[test]
fn test_s12_b55_empty_dict() {
    let code = r#"
def new_dict() -> dict:
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn new_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_literal() {
    let code = r##"
def config() -> dict:
    return {"host": "localhost", "port": 8080, "debug": True}
"##;
    let result = transpile(code);
    assert!(result.contains("fn config"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_from_pairs() {
    let code = r#"
def from_lists(keys: list, values: list) -> dict:
    result = {}
    for i in range(len(keys)):
        result[keys[i]] = values[i]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn from_lists"), "Got: {}", result);
}

// ===== Dict methods =====

#[test]
fn test_s12_b55_dict_get_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_keys() {
    let code = r#"
def all_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_values() {
    let code = r#"
def all_values(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_values"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_items() {
    let code = r#"
def all_items(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append((k, v))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_items"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_pop() {
    let code = r#"
def remove_and_get(d: dict, key: str, default: int) -> int:
    return d.pop(key, default)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_and_get"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_setdefault() {
    let code = r#"
def get_or_set(d: dict, key: str, default: int) -> int:
    return d.setdefault(key, default)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_or_set"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_update() {
    let code = r#"
def merge(d1: dict, d2: dict) -> dict:
    result = {}
    result.update(d1)
    result.update(d2)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

// ===== Dict containment =====

#[test]
fn test_s12_b55_dict_in() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_key"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_not_in() {
    let code = r#"
def missing_key(d: dict, key: str) -> bool:
    return key not in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn missing_key"), "Got: {}", result);
}

// ===== Dict iteration =====

#[test]
fn test_s12_b55_dict_iter_keys() {
    let code = r#"
def key_list(d: dict) -> list:
    result = []
    for k in d:
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn key_list"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_iter_values_sum() {
    let code = r#"
def sum_values(d: dict) -> int:
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_values"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_iter_items_filter() {
    let code = r#"
def filter_positive(d: dict) -> dict:
    result = {}
    for k, v in d.items():
        if v > 0:
            result[k] = v
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_positive"), "Got: {}", result);
}

// ===== Dict algorithms =====

#[test]
fn test_s12_b55_dict_invert() {
    let code = r#"
def invert(d: dict) -> dict:
    result = {}
    for k, v in d.items():
        result[v] = k
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_frequency() {
    let code = r#"
def frequency(items: list) -> dict:
    counts = {}
    for item in items:
        if item in counts:
            counts[item] += 1
        else:
            counts[item] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn frequency"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_group() {
    let code = r#"
def group_by_first(words: list) -> dict:
    groups = {}
    for word in words:
        key = word[0]
        if key not in groups:
            groups[key] = []
        groups[key].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_first"), "Got: {}", result);
}

#[test]
fn test_s12_b55_dict_max_value() {
    let code = r##"
def max_entry(d: dict) -> str:
    best_key = ""
    best_val = 0
    for k, v in d.items():
        if v > best_val:
            best_val = v
            best_key = k
    return best_key
"##;
    let result = transpile(code);
    assert!(result.contains("fn max_entry"), "Got: {}", result);
}

// ===== Dict delete =====

#[test]
fn test_s12_b55_dict_del() {
    let code = r#"
def remove_key(d: dict, key: str) -> dict:
    if key in d:
        del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

// ===== Dict len =====

#[test]
fn test_s12_b55_dict_len() {
    let code = r#"
def size(d: dict) -> int:
    return len(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn size"), "Got: {}", result);
}

// ===== Dict clear =====

#[test]
fn test_s12_b55_dict_clear() {
    let code = r#"
def reset(d: dict):
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reset"), "Got: {}", result);
}

// ===== Dict copy =====

#[test]
fn test_s12_b55_dict_copy() {
    let code = r#"
def clone(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone"), "Got: {}", result);
}

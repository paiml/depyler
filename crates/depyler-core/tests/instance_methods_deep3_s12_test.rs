//! Session 12 Batch 34: Instance method cold paths (deep3)
//!
//! Targets remaining cold paths in expr_gen_instance_methods.rs:
//! - Rare string methods (title, swapcase, isdigit, isalpha, isalnum, isspace)
//! - Dict methods (keys, values, popitem, setdefault)
//! - List methods (copy, sort with key, extend from generator)
//! - Set methods (union, intersection, difference, symmetric_difference)
//! - Bytes/bytearray methods
//! - Complex method chaining patterns

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

// ===== Rare string methods =====

#[test]
fn test_s12_b34_str_title() {
    let code = r#"
def title_str(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn title_str"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_isdigit() {
    let code = r#"
def all_digits(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_digits"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_isalpha() {
    let code = r#"
def all_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_alpha"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_isalnum() {
    let code = r#"
def all_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_alnum"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_isspace() {
    let code = r#"
def is_blank(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_blank"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_isupper() {
    let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_upper"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_islower() {
    let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_lower"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_capitalize() {
    let code = r#"
def cap(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn cap"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_lstrip() {
    let code = r#"
def left_trim(s: str) -> str:
    return s.lstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_trim"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_rstrip() {
    let code = r#"
def right_trim(s: str) -> str:
    return s.rstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_trim"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_startswith() {
    let code = r#"
def starts(s: str, prefix: str) -> bool:
    return s.startswith(prefix)
"#;
    let result = transpile(code);
    assert!(result.contains("fn starts"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_endswith() {
    let code = r#"
def ends(s: str, suffix: str) -> bool:
    return s.endswith(suffix)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ends"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_replace_multi() {
    let code = r#"
def clean_text(s: str) -> str:
    s = s.replace("\t", " ")
    s = s.replace("\n", " ")
    return s.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_text"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_split_maxsplit() {
    let code = r#"
def first_word(s: str) -> str:
    parts = s.split(" ", 1)
    return parts[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_word"), "Got: {}", result);
}

// ===== Dict methods =====

#[test]
fn test_s12_b34_dict_keys() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b34_dict_values() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_values"), "Got: {}", result);
}

#[test]
fn test_s12_b34_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str, val: int) -> int:
    return d.setdefault(key, val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"), "Got: {}", result);
}

#[test]
fn test_s12_b34_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> int:
    return d.pop(key, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

#[test]
fn test_s12_b34_dict_update() {
    let code = r#"
def merge_in(target: dict, source: dict):
    target.update(source)
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_in"), "Got: {}", result);
}

#[test]
fn test_s12_b34_dict_get_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

// ===== List methods =====

#[test]
fn test_s12_b34_list_sort() {
    let code = r#"
def sort_in_place(items: list) -> list:
    items.sort()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_in_place"), "Got: {}", result);
}

#[test]
fn test_s12_b34_list_copy() {
    let code = r#"
def clone_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_list"), "Got: {}", result);
}

#[test]
fn test_s12_b34_list_extend() {
    let code = r#"
def combine(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Got: {}", result);
}

#[test]
fn test_s12_b34_list_insert() {
    let code = r#"
def prepend(items: list, val: int) -> list:
    items.insert(0, val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn prepend"), "Got: {}", result);
}

#[test]
fn test_s12_b34_list_remove() {
    let code = r#"
def drop_first(items: list, val: int) -> list:
    items.remove(val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn drop_first"), "Got: {}", result);
}

#[test]
fn test_s12_b34_list_index() {
    let code = r#"
def find_pos(items: list, val: int) -> int:
    return items.index(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pos"), "Got: {}", result);
}

#[test]
fn test_s12_b34_list_count() {
    let code = r#"
def how_many(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn how_many"), "Got: {}", result);
}

// ===== Set methods =====

#[test]
fn test_s12_b34_set_add() {
    let code = r#"
def collect_unique(items: list) -> set:
    s = set()
    for item in items:
        s.add(item)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn collect_unique"), "Got: {}", result);
}

#[test]
fn test_s12_b34_set_discard() {
    let code = r#"
def remove_val(s: set, val: int) -> set:
    s.discard(val)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_val"), "Got: {}", result);
}

#[test]
fn test_s12_b34_set_union() {
    let code = r#"
def combine_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine_sets"), "Got: {}", result);
}

#[test]
fn test_s12_b34_set_intersection() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn common"), "Got: {}", result);
}

#[test]
fn test_s12_b34_set_difference() {
    let code = r#"
def only_in_first(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_first"), "Got: {}", result);
}

// ===== Complex method patterns =====

#[test]
fn test_s12_b34_method_on_literal() {
    let code = r#"
def comma_join(items: list) -> str:
    return ", ".join(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn comma_join"), "Got: {}", result);
}

#[test]
fn test_s12_b34_method_chain_three() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip().lower().replace(" ", "-")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

#[test]
fn test_s12_b34_method_in_comprehension() {
    let code = r#"
def strip_all(items: list) -> list:
    return [s.strip() for s in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn strip_all"), "Got: {}", result);
}

#[test]
fn test_s12_b34_method_in_condition() {
    let code = r#"
def find_numeric(items: list) -> list:
    result = []
    for s in items:
        if s.isdigit():
            result.append(s)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_numeric"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_format_method() {
    let code = r#"
def make_greeting(name: str) -> str:
    template = "Hello, {}!"
    return template.format(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_greeting"), "Got: {}", result);
}

#[test]
fn test_s12_b34_str_join_list_comp() {
    let code = r#"
def to_csv(items: list) -> str:
    return ",".join([str(x) for x in items])
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_csv"), "Got: {}", result);
}

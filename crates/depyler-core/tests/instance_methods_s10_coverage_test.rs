//! DEPYLER-99MODE-S10: Integration tests targeting expr_gen_instance_methods.rs coverage
//!
//! Tests for dict methods, list methods, string methods, set operations,
//! frozenset, and attribute access through the transpilation pipeline.

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

// ===== Dict Methods =====

#[test]
fn test_s10_dict_get() {
    let code = r#"
def lookup(d: dict, key: str) -> str:
    return d.get(key, "default")
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"));
    assert!(result.contains("get") || result.contains("unwrap_or"));
}

#[test]
fn test_s10_dict_keys() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_keys"));
    assert!(result.contains("keys"));
}

#[test]
fn test_s10_dict_values() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_values"));
    assert!(result.contains("values"));
}

#[test]
fn test_s10_dict_items() {
    let code = r#"
def iterate_items(d: dict):
    for k, v in d.items():
        print(k, v)
"#;
    let result = transpile(code);
    assert!(result.contains("fn iterate_items"));
    assert!(result.contains("iter") || result.contains("items"));
}

#[test]
fn test_s10_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> str:
    return d.pop(key, "missing")
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"));
    assert!(result.contains("remove") || result.contains("pop"));
}

#[test]
fn test_s10_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str):
    d.setdefault(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"));
    assert!(result.contains("entry") || result.contains("setdefault") || result.contains("or_insert"));
}

#[test]
fn test_s10_dict_clear() {
    let code = r#"
def reset_dict(d: dict):
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reset_dict"));
    assert!(result.contains("clear"));
}

#[test]
fn test_s10_dict_copy() {
    let code = r#"
def clone_dict(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_dict"));
    assert!(result.contains("clone"));
}

// ===== List Methods =====

#[test]
fn test_s10_list_append() {
    let code = r#"
def add_item(items: list, x: int):
    items.append(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_item"));
    assert!(result.contains("push"));
}

#[test]
fn test_s10_list_extend() {
    let code = r#"
def merge_lists(a: list, b: list):
    a.extend(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_lists"));
    assert!(result.contains("extend"));
}

#[test]
fn test_s10_list_sort() {
    let code = r#"
def sort_list(items: list):
    items.sort()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_list"));
    assert!(result.contains("sort"));
}

#[test]
fn test_s10_list_reverse() {
    let code = r#"
def reverse_list(items: list):
    items.reverse()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"));
    assert!(result.contains("reverse"));
}

#[test]
fn test_s10_list_count() {
    let code = r#"
def count_item(items: list, x: int) -> int:
    return items.count(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_item"));
    assert!(result.contains("filter") || result.contains("count"));
}

#[test]
fn test_s10_list_clear() {
    let code = r#"
def reset_list(items: list):
    items.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reset_list"));
    assert!(result.contains("clear"));
}

#[test]
fn test_s10_list_copy() {
    let code = r#"
def clone_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_list"));
    assert!(result.contains("clone") || result.contains("to_vec"));
}

// ===== String Methods =====

#[test]
fn test_s10_str_split() {
    let code = r#"
def split_csv(s: str) -> list:
    return s.split(",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_csv"));
    assert!(result.contains("split"));
}

#[test]
fn test_s10_str_rsplit() {
    let code = r#"
def rsplit_path(s: str) -> list:
    return s.rsplit("/")
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_path"));
    assert!(result.contains("rsplit") || result.contains("split"));
}

#[test]
fn test_s10_str_find() {
    let code = r#"
def find_char(s: str, c: str) -> int:
    return s.find(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_char"));
    assert!(result.contains("find"));
}

#[test]
fn test_s10_str_replace() {
    let code = r#"
def fix_spaces(s: str) -> str:
    return s.replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn fix_spaces"));
    assert!(result.contains("replace"));
}

#[test]
fn test_s10_str_startswith() {
    let code = "
def is_comment(s: str) -> bool:
    return s.startswith(\"#\")
";
    let result = transpile(code);
    assert!(result.contains("fn is_comment"));
    assert!(result.contains("starts_with"));
}

#[test]
fn test_s10_str_endswith() {
    let code = r#"
def is_python(s: str) -> bool:
    return s.endswith(".py")
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_python"));
    assert!(result.contains("ends_with"));
}

#[test]
fn test_s10_str_strip() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"));
    assert!(result.contains("trim"));
}

#[test]
fn test_s10_str_lstrip() {
    let code = r#"
def left_clean(s: str) -> str:
    return s.lstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_clean"));
    assert!(result.contains("trim_start"));
}

#[test]
fn test_s10_str_rstrip() {
    let code = r#"
def right_clean(s: str) -> str:
    return s.rstrip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_clean"));
    assert!(result.contains("trim_end"));
}

#[test]
fn test_s10_str_upper() {
    let code = r#"
def shout(s: str) -> str:
    return s.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn shout"));
    assert!(result.contains("to_uppercase"));
}

#[test]
fn test_s10_str_lower() {
    let code = r#"
def whisper(s: str) -> str:
    return s.lower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn whisper"));
    assert!(result.contains("to_lowercase"));
}

#[test]
fn test_s10_str_ljust() {
    let code = r#"
def pad_right(s: str, width: int) -> str:
    return s.ljust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_right"));
}

#[test]
fn test_s10_str_rjust() {
    let code = r#"
def pad_left(s: str, width: int) -> str:
    return s.rjust(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_left"));
}

#[test]
fn test_s10_str_splitlines() {
    let code = r#"
def get_lines(s: str) -> list:
    return s.splitlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_lines"));
    assert!(result.contains("lines") || result.contains("split"));
}

#[test]
fn test_s10_str_isupper() {
    let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_upper"));
    assert!(result.contains("is_uppercase") || result.contains("chars"));
}

#[test]
fn test_s10_str_islower() {
    let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_lower"));
    assert!(result.contains("is_lowercase") || result.contains("chars"));
}

#[test]
fn test_s10_str_isalnum() {
    let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_alnum"));
    assert!(result.contains("is_alphanumeric") || result.contains("chars"));
}

#[test]
fn test_s10_str_isspace() {
    let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_space"));
}

#[test]
fn test_s10_str_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"));
}

// ===== Set Operations =====

#[test]
fn test_s10_set_remove() {
    let code = r#"
def drop_item(s: set, x: int):
    s.remove(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn drop_item"));
    assert!(result.contains("remove"));
}

#[test]
fn test_s10_set_clear() {
    let code = r#"
def clear_set(s: set):
    s.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_set"));
    assert!(result.contains("clear"));
}

#[test]
fn test_s10_set_pop() {
    let code = r#"
def pop_set(s: set) -> int:
    return s.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_set"));
}

#[test]
fn test_s10_set_issubset() {
    let code = r#"
def check_subset(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_subset"));
    assert!(result.contains("is_subset"));
}

#[test]
fn test_s10_set_issuperset() {
    let code = r#"
def check_superset(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_superset"));
    assert!(result.contains("is_superset"));
}

#[test]
fn test_s10_set_symmetric_difference() {
    let code = r#"
def sym_diff(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sym_diff"));
    assert!(result.contains("symmetric_difference"));
}

// ===== Frozenset =====

#[test]
fn test_s10_frozenset_creation() {
    let code = r#"
def make_frozen() -> frozenset:
    return frozenset({1, 2, 3})
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_frozen"));
    assert!(result.contains("HashSet") || result.contains("Arc"));
}

// ===== Attribute Access Patterns =====

#[test]
fn test_s10_len_of_string() {
    let code = r#"
def string_len(s: str) -> int:
    return len(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn string_len"));
    assert!(result.contains("len()"));
}

#[test]
fn test_s10_len_of_list() {
    let code = r#"
def list_len(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn list_len"));
    assert!(result.contains("len()"));
}

#[test]
fn test_s10_len_of_dict() {
    let code = r#"
def dict_len(d: dict) -> int:
    return len(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_len"));
    assert!(result.contains("len()"));
}

// ===== Type Conversion Methods =====

#[test]
fn test_s10_int_to_str() {
    let code = r#"
def num_to_str(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn num_to_str"));
    assert!(result.contains("to_string"));
}

#[test]
fn test_s10_str_to_float() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_float"));
    assert!(result.contains("parse") || result.contains("f64"));
}

// ===== List/Tuple Operations =====

#[test]
fn test_s10_sorted_builtin() {
    let code = r#"
def get_sorted(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_sorted"));
    assert!(result.contains("sort") || result.contains("sorted"));
}

#[test]
fn test_s10_reversed_builtin() {
    let code = r#"
def get_reversed(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_reversed"));
    assert!(result.contains("rev") || result.contains("reverse"));
}

#[test]
fn test_s10_min_max_builtins() {
    let code = r#"
def get_range(items: list) -> int:
    return max(items) - min(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_range"));
    assert!(result.contains("max") || result.contains("min") || result.contains("iter"));
}

#[test]
fn test_s10_sum_builtin() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"));
    assert!(result.contains("sum") || result.contains("iter"));
}

// ===== Membership Testing =====

#[test]
fn test_s10_in_list() {
    let code = r#"
def contains_item(items: list, x: int) -> bool:
    return x in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains_item"));
    assert!(result.contains("contains") || result.contains("any"));
}

#[test]
fn test_s10_in_dict() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_key"));
    assert!(result.contains("contains_key") || result.contains("contains"));
}

#[test]
fn test_s10_in_string() {
    let code = r#"
def has_substring(s: str, sub: str) -> bool:
    return sub in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_substring"));
    assert!(result.contains("contains"));
}

// ===== Enumerate =====

#[test]
fn test_s10_enumerate_basic() {
    let code = r#"
def print_indexed(items: list):
    for i, item in enumerate(items):
        print(i, item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_indexed"));
    assert!(result.contains("enumerate"));
}

// ===== Zip =====

#[test]
fn test_s10_zip_basic() {
    let code = r#"
def combine(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"));
    assert!(result.contains("zip"));
}

// ===== Map/Filter =====

#[test]
fn test_s10_map_builtin() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"));
    assert!(result.contains("map"));
}

#[test]
fn test_s10_filter_builtin() {
    let code = r#"
def positives(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn positives"));
    assert!(result.contains("filter"));
}

// ===== Any/All =====

#[test]
fn test_s10_any_builtin() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_positive"));
    assert!(result.contains("any"));
}

#[test]
fn test_s10_all_builtin() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"));
    assert!(result.contains("all"));
}

// ===== abs/round =====

#[test]
fn test_s10_abs_builtin() {
    let code = r#"
def magnitude(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn magnitude"));
    assert!(result.contains("abs"));
}

#[test]
fn test_s10_round_builtin() {
    let code = r#"
def approx(x: float) -> float:
    return round(x, 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn approx"));
    assert!(result.contains("round") || result.contains("powi"));
}

// ===== isinstance =====

#[test]
fn test_s10_isinstance_check() {
    let code = r#"
def is_string(x) -> bool:
    return isinstance(x, str)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_string"));
}

// ===== Complex String Formatting =====

#[test]
fn test_s10_format_method() {
    let code = r#"
def format_msg(name: str, age: int) -> str:
    return "{} is {} years old".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_msg"));
    assert!(result.contains("format!"));
}

// ===== Chained List Operations =====

#[test]
fn test_s10_list_comprehension_with_condition() {
    let code = r#"
def even_squares(n: int) -> list:
    return [x * x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_squares"));
    assert!(result.contains("filter") || result.contains("map"));
}

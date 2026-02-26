//! DEPYLER-99MODE-S11: Integration tests targeting dict/set/regex method coverage
//!
//! Tests for: dict operations (get, pop, setdefault, update, keys, values, items),
//! set operations (symmetric_difference, issubset, issuperset, comprehensions),
//! regex operations (split, sub, findall), and uncommon string methods.

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

// ===== Dict Operations =====

#[test]
fn test_s11_dict_get_with_default() {
    let code = r#"
def lookup(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"));
    assert!(result.contains("get") || result.contains("unwrap_or"));
}

#[test]
fn test_s11_dict_get_string_default() {
    let code = r#"
def lookup_name(d: dict, key: str) -> str:
    return d.get(key, "unknown")
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup_name"));
}

#[test]
fn test_s11_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"));
}

#[test]
fn test_s11_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"));
}

#[test]
fn test_s11_dict_update() {
    let code = r#"
def merge_dicts(a: dict, b: dict):
    a.update(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_dicts"));
}

#[test]
fn test_s11_dict_keys() {
    let code = r#"
from typing import Dict

def get_keys(d: Dict[str, int]) -> list:
    return list(d.keys())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_keys"));
    assert!(result.contains("keys"));
}

#[test]
fn test_s11_dict_values() {
    let code = r#"
from typing import Dict

def get_values(d: Dict[str, int]) -> list:
    return list(d.values())
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_values"));
    assert!(result.contains("values"));
}

#[test]
fn test_s11_dict_items() {
    let code = r#"
from typing import Dict

def print_items(d: Dict[str, int]):
    for k, v in d.items():
        print(k, v)
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_items"));
}

#[test]
fn test_s11_dict_clear() {
    let code = r#"
def clear_all(d: dict):
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_all"));
    assert!(result.contains("clear"));
}

#[test]
fn test_s11_dict_copy() {
    let code = r#"
def clone_dict(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_dict"));
    assert!(result.contains("clone"));
}

#[test]
fn test_s11_dict_comprehension_from_list() {
    let code = r#"
from typing import Dict, List

def index_items(items: List[str]) -> Dict[int, str]:
    return {i: item for i, item in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_items"));
}

#[test]
fn test_s11_dict_comprehension_with_filter() {
    let code = r#"
from typing import Dict

def positive_only(d: Dict[str, int]) -> Dict[str, int]:
    return {k: v for k, v in d.items() if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_only"));
}

// ===== Set Operations =====

#[test]
fn test_s11_set_symmetric_difference() {
    let code = r#"
def sym_diff(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sym_diff"));
    assert!(result.contains("symmetric_difference") || result.contains("^"));
}

#[test]
fn test_s11_set_issubset() {
    let code = r#"
def is_contained(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_contained"));
    assert!(result.contains("is_subset"));
}

#[test]
fn test_s11_set_issuperset() {
    let code = r#"
def contains_all(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains_all"));
    assert!(result.contains("is_superset"));
}

#[test]
fn test_s11_set_union() {
    let code = r#"
def combine(a: set, b: set) -> set:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"));
    assert!(result.contains("union"));
}

#[test]
fn test_s11_set_intersection() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn common"));
    assert!(result.contains("intersection"));
}

#[test]
fn test_s11_set_difference() {
    let code = r#"
def only_in_a(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_a"));
    assert!(result.contains("difference"));
}

#[test]
fn test_s11_set_add_remove() {
    let code = r#"
def modify_set(s: set, x: int):
    s.add(x)
    s.remove(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn modify_set"));
    assert!(result.contains("insert") || result.contains("add"));
    assert!(result.contains("remove"));
}

#[test]
fn test_s11_set_discard() {
    let code = r#"
def safe_remove(s: set, x: int):
    s.discard(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_remove"));
}

#[test]
fn test_s11_set_comprehension_with_filter() {
    let code = r#"
def even_squares(n: int) -> set:
    return {x * x for x in range(n) if x % 2 == 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_squares"));
}

#[test]
fn test_s11_set_comprehension_from_string() {
    let code = r#"
def unique_chars(s: str) -> set:
    return {c for c in s}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_chars"));
}

// ===== Regex Operations =====

#[test]
fn test_s11_regex_search() {
    let code = r#"
import re

def find_number(text: str) -> bool:
    return re.search(r"\d+", text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_number"));
}

#[test]
fn test_s11_regex_findall() {
    let code = r#"
import re

def find_all_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_all_numbers"));
}

#[test]
fn test_s11_regex_sub() {
    let code = r#"
import re

def replace_digits(text: str) -> str:
    return re.sub(r"\d+", "NUM", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn replace_digits"));
}

#[test]
fn test_s11_regex_split() {
    let code = r#"
import re

def split_words(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_words"));
}

#[test]
fn test_s11_regex_match() {
    let code = r#"
import re

def starts_with_digit(text: str) -> bool:
    return re.match(r"\d", text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn starts_with_digit"));
}

// ===== String Methods (uncommon) =====

#[test]
fn test_s11_str_center() {
    let code = r#"
def pad_center(s: str, width: int) -> str:
    return s.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_center"));
}

#[test]
fn test_s11_str_zfill() {
    let code = r#"
def zero_pad(s: str, width: int) -> str:
    return s.zfill(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_pad"));
}

#[test]
fn test_s11_str_count() {
    let code = r#"
def count_chars(s: str, ch: str) -> int:
    return s.count(ch)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_chars"));
    assert!(result.contains("matches") || result.contains("count"));
}

#[test]
fn test_s11_str_title() {
    let code = r#"
def titlecase(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn titlecase"));
}

#[test]
fn test_s11_str_capitalize() {
    let code = r#"
def capitalize(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn capitalize"));
}

#[test]
fn test_s11_str_isdigit() {
    let code = r#"
def is_numeric(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_numeric"));
}

#[test]
fn test_s11_str_isalpha() {
    let code = r#"
def is_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alpha"));
}

#[test]
fn test_s11_str_replace_multiple() {
    let code = r#"
def sanitize(s: str) -> str:
    result = s.replace("&", "&amp;")
    result = result.replace("<", "&lt;")
    result = result.replace(">", "&gt;")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn sanitize"));
    assert!(result.contains("replace"));
}

#[test]
fn test_s11_str_format_method() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"));
}

#[test]
fn test_s11_str_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bytes"));
}

// ===== List Operations =====

#[test]
fn test_s11_list_sort_with_key() {
    let code = r#"
def sort_by_abs(items: list):
    items.sort(key=abs)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_abs"));
}

#[test]
fn test_s11_sorted_with_reverse() {
    let code = r#"
def sort_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_desc"));
}

#[test]
fn test_s11_list_insert() {
    let code = r#"
def insert_at_front(items: list, x: int):
    items.insert(0, x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_at_front"));
    assert!(result.contains("insert"));
}

#[test]
fn test_s11_list_pop_index() {
    let code = r#"
def remove_first(items: list) -> int:
    return items.pop(0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_first"));
}

#[test]
fn test_s11_list_index() {
    let code = r#"
def find_position(items: list, x: int) -> int:
    return items.index(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_position"));
}

#[test]
fn test_s11_list_extend() {
    let code = r#"
def combine_lists(a: list, b: list):
    a.extend(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine_lists"));
    assert!(result.contains("extend"));
}

// ===== Builtins =====

#[test]
fn test_s11_builtin_abs() {
    let code = r#"
def absolute(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn absolute"));
    assert!(result.contains("abs"));
}

#[test]
fn test_s11_builtin_round() {
    let code = r#"
def round_num(x: float) -> int:
    return round(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_num"));
    assert!(result.contains("round"));
}

#[test]
fn test_s11_builtin_min_max() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return max(lo, min(x, hi))
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"));
    assert!(result.contains("max") || result.contains("min"));
}

#[test]
fn test_s11_builtin_enumerate() {
    let code = r#"
from typing import List

def indexed_items(items: List[str]):
    for i, item in enumerate(items):
        print(i, item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_items"));
    assert!(result.contains("enumerate"));
}

#[test]
fn test_s11_builtin_zip() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    return list(zip(a, b))
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair_up"));
    assert!(result.contains("zip"));
}

#[test]
fn test_s11_builtin_isinstance() {
    let code = r#"
def is_int(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_int"));
}

#[test]
fn test_s11_builtin_any_all() {
    let code = r#"
from typing import List

def has_positive(items: List[int]) -> bool:
    return any(x > 0 for x in items)

def all_positive(items: List[int]) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_positive"));
    assert!(result.contains("fn all_positive"));
    assert!(result.contains("any") || result.contains("iter"));
}

#[test]
fn test_s11_builtin_sum_with_generator() {
    let code = r#"
from typing import List

def sum_squares(items: List[int]) -> int:
    return sum(x * x for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"));
}

#[test]
fn test_s11_builtin_map_filter() {
    let code = r#"
from typing import List

def double_evens(items: List[int]) -> List[int]:
    return list(map(lambda x: x * 2, filter(lambda x: x % 2 == 0, items)))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_evens"));
}

// ===== Complex Patterns =====

#[test]
fn test_s11_nested_dict_access() {
    let code = r#"
from typing import Dict

def deep_get(data: Dict[str, dict], key1: str, key2: str) -> int:
    inner = data.get(key1, {})
    return inner.get(key2, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_get"));
}

#[test]
fn test_s11_dict_iteration_with_modification() {
    let code = r#"
from typing import Dict

def filter_dict(d: Dict[str, int]) -> Dict[str, int]:
    result = {}
    for k, v in d.items():
        if v > 0:
            result[k] = v
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_dict"));
}

#[test]
fn test_s11_set_from_list() {
    let code = r#"
from typing import List, Set

def deduplicate(items: List[int]) -> Set[int]:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn deduplicate"));
}

#[test]
fn test_s11_dict_from_two_lists() {
    let code = r#"
from typing import List, Dict

def make_dict(keys: List[str], values: List[int]) -> Dict[str, int]:
    return dict(zip(keys, values))
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"));
}

#[test]
fn test_s11_membership_testing() {
    let code = r#"
from typing import List, Dict

def contains_key(d: Dict[str, int], key: str) -> bool:
    return key in d

def contains_item(items: List[int], x: int) -> bool:
    return x in items

def contains_char(s: str, c: str) -> bool:
    return c in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains_key"));
    assert!(result.contains("fn contains_item"));
    assert!(result.contains("fn contains_char"));
    assert!(result.contains("contains"));
}

#[test]
fn test_s11_not_in_operator() {
    let code = r#"
from typing import List

def not_in_list(items: List[int], x: int) -> bool:
    return x not in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn not_in_list"));
}

// ===== Slice Operations =====

#[test]
fn test_s11_list_slice_start_end() {
    let code = r#"
from typing import List

def middle(items: List[int]) -> List[int]:
    return items[1:-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn middle"));
}

#[test]
fn test_s11_list_slice_step() {
    let code = r#"
from typing import List

def every_other(items: List[int]) -> List[int]:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"));
}

#[test]
fn test_s11_string_slice() {
    let code = r#"
def first_three(s: str) -> str:
    return s[:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_three"));
}

#[test]
fn test_s11_string_reverse() {
    let code = r#"
def reverse_str(s: str) -> str:
    return s[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_str"));
}

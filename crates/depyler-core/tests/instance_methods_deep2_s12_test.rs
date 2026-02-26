//! Session 12 Batch 15: Deep coverage for expr_gen_instance_methods.rs cold paths
//!
//! Targets the #1 coverage gap file (71% line coverage, 3025 missed lines):
//! - String justification with fillchar (center, ljust, rjust, zfill)
//! - String expandtabs with custom tabsize
//! - List methods (insert, extend, clear, copy, remove, index)
//! - Dict methods (copy, clear, fromkeys)
//! - Set methods (intersection_update, difference_update, symmetric_difference, clear)
//! - Deque methods (appendleft, popleft, rotate)
//! - Bytes methods (decode, hex)
//! - File I/O methods (flush, close)
//! - Complex method chain patterns

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

// ===== String justification methods =====

#[test]
fn test_s12_str_center_basic() {
    let code = r#"
def center_text(s: str) -> str:
    return s.center(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12_str_center_fillchar() {
    let code = r#"
def center_with_dash(s: str) -> str:
    return s.center(20, "-")
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_with_dash"), "Got: {}", result);
}

#[test]
fn test_s12_str_ljust_basic() {
    let code = r#"
def left_pad(s: str) -> str:
    return s.ljust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_pad"), "Got: {}", result);
}

#[test]
fn test_s12_str_ljust_fillchar() {
    let code = r#"
def left_pad_dot(s: str) -> str:
    return s.ljust(20, ".")
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_pad_dot"), "Got: {}", result);
}

#[test]
fn test_s12_str_rjust_basic() {
    let code = r#"
def right_pad(s: str) -> str:
    return s.rjust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_pad"), "Got: {}", result);
}

#[test]
fn test_s12_str_rjust_fillchar() {
    let code = r#"
def right_pad_star(s: str) -> str:
    return s.rjust(20, "*")
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_pad_star"), "Got: {}", result);
}

#[test]
fn test_s12_str_zfill_basic() {
    let code = r#"
def zero_pad(n: str) -> str:
    return n.zfill(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_pad"), "Got: {}", result);
}

#[test]
fn test_s12_str_expandtabs_default() {
    let code = r#"
def expand_tabs(s: str) -> str:
    return s.expandtabs()
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand_tabs"), "Got: {}", result);
}

#[test]
fn test_s12_str_expandtabs_custom() {
    let code = r#"
def expand_tabs_4(s: str) -> str:
    return s.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand_tabs_4"), "Got: {}", result);
}

// ===== String check methods =====

#[test]
fn test_s12_str_isdigit() {
    let code = r#"
def is_numeric_str(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_numeric_str"), "Got: {}", result);
}

#[test]
fn test_s12_str_isalpha() {
    let code = r#"
def is_alpha_str(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alpha_str"), "Got: {}", result);
}

#[test]
fn test_s12_str_isalnum() {
    let code = r#"
def is_alnum_str(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_alnum_str"), "Got: {}", result);
}

#[test]
fn test_s12_str_isupper() {
    let code = r#"
def is_upper_str(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_upper_str"), "Got: {}", result);
}

#[test]
fn test_s12_str_islower() {
    let code = r#"
def is_lower_str(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_lower_str"), "Got: {}", result);
}

#[test]
fn test_s12_str_isspace() {
    let code = r#"
def is_whitespace(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_whitespace"), "Got: {}", result);
}

#[test]
fn test_s12_str_title() {
    let code = r#"
def title_case(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn title_case"), "Got: {}", result);
}

#[test]
fn test_s12_str_swapcase() {
    let code = r#"
def swap_case(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_case"), "Got: {}", result);
}

#[test]
fn test_s12_str_capitalize() {
    let code = r#"
def capitalize_str(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn capitalize_str"), "Got: {}", result);
}

// ===== String partition =====

#[test]
fn test_s12_str_partition() {
    let code = r#"
def split_at_sep(s: str) -> tuple:
    return s.partition("=")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at_sep"), "Got: {}", result);
}

#[test]
fn test_s12_str_rpartition() {
    let code = r#"
def split_at_last_sep(s: str) -> tuple:
    return s.rpartition("/")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at_last_sep"), "Got: {}", result);
}

// ===== List methods =====

#[test]
fn test_s12_list_insert() {
    let code = r#"
def insert_at(items: list, pos: int, val: int) -> list:
    items.insert(pos, val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_at"), "Got: {}", result);
}

#[test]
fn test_s12_list_extend() {
    let code = r#"
def extend_list(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn extend_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_clear() {
    let code = r#"
def clear_list(items: list) -> int:
    items.clear()
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_copy() {
    let code = r#"
def copy_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_remove() {
    let code = r#"
def remove_item(items: list, val: int) -> list:
    items.remove(val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_item"), "Got: {}", result);
}

#[test]
fn test_s12_list_index() {
    let code = r#"
def find_index(items: list, val: int) -> int:
    return items.index(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_index"), "Got: {}", result);
}

#[test]
fn test_s12_list_count() {
    let code = r#"
def count_item(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_item"), "Got: {}", result);
}

#[test]
fn test_s12_list_reverse() {
    let code = r#"
def reverse_in_place(items: list) -> list:
    items.reverse()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_in_place"), "Got: {}", result);
}

// ===== Dict methods =====

#[test]
fn test_s12_dict_copy() {
    let code = r#"
def copy_dict(d: dict) -> dict:
    return d.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_dict"), "Got: {}", result);
}

#[test]
fn test_s12_dict_clear() {
    let code = r#"
def clear_dict(d: dict) -> int:
    d.clear()
    return len(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_dict"), "Got: {}", result);
}

#[test]
fn test_s12_dict_popitem() {
    let code = r#"
def pop_last(d: dict) -> tuple:
    return d.popitem()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_last"), "Got: {}", result);
}

// ===== Set methods =====

#[test]
fn test_s12_set_intersection_update() {
    let code = r#"
def keep_common(s: set, other: set) -> set:
    s.intersection_update(other)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn keep_common"), "Got: {}", result);
}

#[test]
fn test_s12_set_difference_update() {
    let code = r#"
def remove_common(s: set, other: set) -> set:
    s.difference_update(other)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_common"), "Got: {}", result);
}

#[test]
fn test_s12_set_symmetric_difference() {
    let code = r#"
def exclusive_elements(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn exclusive_elements"), "Got: {}", result);
}

#[test]
fn test_s12_set_clear() {
    let code = r#"
def clear_set(s: set) -> int:
    s.clear()
    return len(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_copy() {
    let code = r#"
def copy_set(s: set) -> set:
    return s.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_pop() {
    let code = r#"
def pop_element(s: set) -> int:
    return s.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_element"), "Got: {}", result);
}

#[test]
fn test_s12_set_union() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sets"), "Got: {}", result);
}

#[test]
fn test_s12_set_intersection() {
    let code = r#"
def common_elements(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn common_elements"), "Got: {}", result);
}

#[test]
fn test_s12_set_difference() {
    let code = r#"
def only_in_first(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_first"), "Got: {}", result);
}

// ===== Deque methods =====

#[test]
fn test_s12_deque_appendleft() {
    let code = r#"
from collections import deque

def prepend(items: list, val: int) -> list:
    d = deque(items)
    d.appendleft(val)
    return list(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn prepend"), "Got: {}", result);
}

#[test]
fn test_s12_deque_popleft() {
    let code = r#"
from collections import deque

def pop_front(items: list) -> int:
    d = deque(items)
    return d.popleft()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_front"), "Got: {}", result);
}

#[test]
fn test_s12_deque_rotate() {
    let code = r#"
from collections import deque

def rotate_right(items: list, n: int) -> list:
    d = deque(items)
    d.rotate(n)
    return list(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rotate_right"), "Got: {}", result);
}

// ===== Bytes methods =====

#[test]
fn test_s12_bytes_decode() {
    let code = r#"
def decode_bytes(data: bytes) -> str:
    return data.decode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn decode_bytes"), "Got: {}", result);
}

#[test]
fn test_s12_bytes_decode_utf8() {
    let code = r#"
def decode_utf8(data: bytes) -> str:
    return data.decode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn decode_utf8"), "Got: {}", result);
}

// ===== Complex method chain patterns =====

#[test]
fn test_s12_chain_strip_split_join() {
    let code = r#"
def normalize_whitespace(s: str) -> str:
    return " ".join(s.strip().split())
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize_whitespace"), "Got: {}", result);
}

#[test]
fn test_s12_chain_lower_replace_strip() {
    let code = r#"
def clean_identifier(s: str) -> str:
    return s.lower().replace(" ", "_").strip("_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_identifier"), "Got: {}", result);
}

#[test]
fn test_s12_chain_split_strip_filter() {
    let code = r#"
def parse_csv_line(line: str) -> list:
    parts = line.split(",")
    result = []
    for part in parts:
        cleaned = part.strip()
        if cleaned:
            result.append(cleaned)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_csv_line"), "Got: {}", result);
}

// ===== Complex algorithms using instance methods =====

#[test]
fn test_s12_run_length_encode() {
    let code = r#"
def run_length_encode(s: str) -> str:
    if not s:
        return ""
    result = ""
    count = 1
    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            count += 1
        else:
            result += str(count) + s[i - 1]
            count = 1
    result += str(count) + s[-1]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn run_length_encode"), "Got: {}", result);
}

#[test]
fn test_s12_stack_operations() {
    let code = r#"
def balanced_parens(s: str) -> bool:
    stack = []
    for c in s:
        if c == "(":
            stack.append(c)
        elif c == ")":
            if not stack:
                return False
            stack.pop()
    return len(stack) == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn balanced_parens"), "Got: {}", result);
}

#[test]
fn test_s12_unique_sorted() {
    let code = r#"
def unique_sorted(items: list) -> list:
    seen = set()
    result = []
    for item in items:
        if item not in seen:
            seen.add(item)
            result.append(item)
    result.sort()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_sorted"), "Got: {}", result);
}

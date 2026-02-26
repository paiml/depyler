//! Session 12 Batch 3: String formatting and set operation methods
//!
//! Targets the 354-line uncovered block in expr_gen_instance_methods.rs
//! (lines 1396-1750) plus set operations (lines 1869-1979)

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

// ===== String formatting methods (lines 1396-1750) =====

#[test]
fn test_s12b3_str_center_basic() {
    let code = r#"
def center_text(s: str) -> str:
    return s.center(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_center_with_fill() {
    let code = r#"
def center_dashes(s: str, width: int) -> str:
    return s.center(width, "-")
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_dashes"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_ljust_basic() {
    let code = r#"
def pad_right(s: str) -> str:
    return s.ljust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_right"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_ljust_with_fill() {
    let code = r#"
def pad_right_dots(s: str, width: int) -> str:
    return s.ljust(width, ".")
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_right_dots"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_rjust_basic() {
    let code = r#"
def pad_left(s: str) -> str:
    return s.rjust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_left"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_rjust_with_fill() {
    let code = r#"
def pad_left_zeros(s: str, width: int) -> str:
    return s.rjust(width, "0")
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_left_zeros"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_zfill() {
    let code = r#"
def zero_pad(num_str: str) -> str:
    return num_str.zfill(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_pad"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_zfill_variable() {
    let code = r#"
def zero_pad_var(s: str, width: int) -> str:
    return s.zfill(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_pad_var"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_capitalize() {
    let code = r#"
def cap(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn cap"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_swapcase() {
    let code = r#"
def swap_case(s: str) -> str:
    return s.swapcase()
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_case"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_expandtabs() {
    let code = r#"
def expand_tabs(s: str) -> str:
    return s.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand_tabs"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_expandtabs_variable() {
    let code = r#"
def expand_var(s: str, size: int) -> str:
    return s.expandtabs(size)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand_var"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_splitlines() {
    let code = r#"
def lines(s: str) -> list:
    return s.splitlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn lines"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_partition() {
    let code = r#"
def split_at(s: str, sep: str) -> tuple:
    return s.partition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_rpartition() {
    let code = r#"
def rsplit_at(s: str, sep: str) -> tuple:
    return s.rpartition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_at"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_casefold() {
    let code = r#"
def normalize(s: str) -> str:
    return s.casefold()
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_isprintable() {
    let code = r#"
def is_printable(s: str) -> bool:
    return s.isprintable()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_printable"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_isupper() {
    let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_upper"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_islower() {
    let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_lower"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_istitle() {
    let code = r#"
def check_title(s: str) -> bool:
    return s.istitle()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_title"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_isnumeric() {
    let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_numeric"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_isascii() {
    let code = r#"
def check_ascii(s: str) -> bool:
    return s.isascii()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_ascii"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_isdecimal() {
    let code = r#"
def check_decimal(s: str) -> bool:
    return s.isdecimal()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_decimal"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_isidentifier() {
    let code = r#"
def check_ident(s: str) -> bool:
    return s.isidentifier()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_ident"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_format_single() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello {}!".format(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_format_multiple() {
    let code = r#"
def describe(name: str, age: int) -> str:
    return "{} is {} years old".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_format_positional() {
    let code = r#"
def swap_format(a: str, b: str) -> str:
    return "{1} then {0}".format(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap_format"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_rsplit() {
    let code = r#"
def rsplit_text(s: str) -> list:
    return s.rsplit(",")
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_text"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_rsplit_maxsplit() {
    let code = r#"
def rsplit_last(s: str) -> list:
    return s.rsplit(",", 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_last"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_index() {
    let code = r#"
def find_pos(s: str, sub: str) -> int:
    return s.index(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pos"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_rindex() {
    let code = r#"
def rfind_pos(s: str, sub: str) -> int:
    return s.rindex(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rfind_pos"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bytes"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_encode_utf8() {
    let code = r#"
def to_utf8(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_utf8"), "Got: {}", result);
}

// ===== Set operations (lines 1869-1979) =====

#[test]
fn test_s12b3_set_union_method() {
    let code = r#"
def combine(a: set, b: set) -> set:
    return a.union(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_intersection_method() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn common"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_difference_method() {
    let code = r#"
def only_in_a(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_a"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_symmetric_difference_method() {
    let code = r#"
def xor_sets(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn xor_sets"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_issubset() {
    let code = r#"
def is_subset(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_subset"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_issuperset() {
    let code = r#"
def is_superset(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_superset"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_isdisjoint() {
    let code = r#"
def no_overlap(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn no_overlap"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_add() {
    let code = r#"
def add_item(s: set, item: int) -> set:
    s.add(item)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_item"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_discard() {
    let code = r#"
def remove_safe(s: set, item: int) -> set:
    s.discard(item)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_safe"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_remove() {
    let code = r#"
def remove_item(s: set, item: int) -> set:
    s.remove(item)
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_item"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_clear() {
    let code = r#"
def empty_set(s: set) -> set:
    s.clear()
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty_set"), "Got: {}", result);
}

#[test]
fn test_s12b3_set_update() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    a.update(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sets"), "Got: {}", result);
}

// ===== Dict advanced methods =====

#[test]
fn test_s12b3_dict_setdefault_chained() {
    let code = r#"
def group_words(words: list) -> dict:
    groups = {}
    for word in words:
        first = word[0]
        groups.setdefault(first, []).append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_words"), "Got: {}", result);
}

#[test]
fn test_s12b3_dict_fromkeys() {
    let code = r#"
def init_dict(keys: list) -> dict:
    return dict.fromkeys(keys, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_dict"), "Got: {}", result);
}

// ===== List sort with reverse =====

#[test]
fn test_s12b3_list_sort_reverse() {
    let code = r#"
def sort_desc(items: list):
    items.sort(reverse=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_desc"), "Got: {}", result);
}

#[test]
fn test_s12b3_sorted_reverse() {
    let code = r#"
def sorted_desc(items: list) -> list:
    return sorted(items, reverse=True)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sorted_desc"), "Got: {}", result);
}

// ===== Counter most_common =====

#[test]
fn test_s12b3_counter_most_common() {
    let code = r#"
from collections import Counter

def top_chars(s: str, n: int) -> list:
    return Counter(s).most_common(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn top_chars"), "Got: {}", result);
}

// ===== File I/O edge cases =====

#[test]
fn test_s12b3_file_read_with_size() {
    let code = r#"
def read_chunk(path: str, size: int) -> str:
    with open(path, "r") as f:
        return f.read(size)
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_chunk"), "Got: {}", result);
}

#[test]
fn test_s12b3_file_iteration() {
    let code = r#"
def count_lines(path: str) -> int:
    count = 0
    with open(path, "r") as f:
        for line in f:
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_lines"), "Got: {}", result);
}

#[test]
fn test_s12b3_file_writelines() {
    let code = r#"
def write_all(path: str, lines: list):
    with open(path, "w") as f:
        f.writelines(lines)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_all"), "Got: {}", result);
}

// ===== Bytes methods =====

#[test]
fn test_s12b3_bytes_decode() {
    let code = r#"
def to_str(data: bytes) -> str:
    return data.decode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_str"), "Got: {}", result);
}

#[test]
fn test_s12b3_bytes_decode_utf8() {
    let code = r#"
def to_str_utf8(data: bytes) -> str:
    return data.decode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_str_utf8"), "Got: {}", result);
}

// ===== Complex string operations =====

#[test]
fn test_s12b3_str_maketrans_translate() {
    let code = r#"
def remove_vowels(s: str) -> str:
    table = str.maketrans("", "", "aeiou")
    return s.translate(table)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_vowels"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_title() {
    let code = r#"
def to_title(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_title"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_upper() {
    let code = r#"
def to_upper(s: str) -> str:
    return s.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_upper"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_lower() {
    let code = r#"
def to_lower(s: str) -> str:
    return s.lower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_lower"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_strip() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

#[test]
fn test_s12b3_str_strip_chars() {
    let code = r#"
def clean_chars(s: str, chars: str) -> str:
    return s.strip(chars)
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean_chars"), "Got: {}", result);
}

//! Session 12 Batch 8: Deep coverage tests for direct_rules_convert.rs
//! Targets: dict indexing heuristics, slicing, negative indices,
//! set/deque/list method variants, DepylerValue wrapping, static method detection

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

// ===== Dict indexing heuristics =====

#[test]
fn test_s12_dict_index_by_key_var() {
    let code = r#"
def lookup(data: dict, key: str) -> int:
    return data[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_config_name() {
    let code = r#"
def get_config(config: dict) -> str:
    return config["host"]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_config"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_params() {
    let code = r#"
def get_param(params: dict, name: str) -> str:
    return params[name]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_param"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_options() {
    let code = r#"
def get_option(options: dict, key: str) -> int:
    return options[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_option"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_settings() {
    let code = r#"
def get_setting(settings: dict, key: str) -> str:
    return settings[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_setting"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_env() {
    let code = r#"
def get_env(env: dict, var: str) -> str:
    return env[var]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_env"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_cache() {
    let code = r#"
def get_cached(cache: dict, key: str) -> int:
    return cache[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_cached"), "Got: {}", result);
}

#[test]
fn test_s12_dict_index_result() {
    let code = r#"
def get_result(result: dict, field: str) -> str:
    return result[field]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_result"), "Got: {}", result);
}

// ===== List negative indexing =====

#[test]
fn test_s12_list_negative_one() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_item"), "Got: {}", result);
}

#[test]
fn test_s12_list_negative_two() {
    let code = r#"
def second_to_last(items: list) -> int:
    return items[-2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn second_to_last"), "Got: {}", result);
}

#[test]
fn test_s12_list_variable_index() {
    let code = r#"
def get_at(items: list, idx: int) -> int:
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_at"), "Got: {}", result);
}

// ===== String slicing patterns =====

#[test]
fn test_s12_str_slice_start_stop() {
    let code = r#"
def substr(s: str) -> str:
    return s[2:5]
"#;
    let result = transpile(code);
    assert!(result.contains("fn substr"), "Got: {}", result);
}

#[test]
fn test_s12_str_slice_start_only() {
    let code = r#"
def from_start(s: str) -> str:
    return s[3:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn from_start"), "Got: {}", result);
}

#[test]
fn test_s12_str_slice_stop_only() {
    let code = r#"
def to_stop(s: str) -> str:
    return s[:5]
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_stop"), "Got: {}", result);
}

#[test]
fn test_s12_str_slice_full() {
    let code = r#"
def full_copy(s: str) -> str:
    return s[:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn full_copy"), "Got: {}", result);
}

#[test]
fn test_s12_str_slice_negative_stop() {
    let code = r#"
def trim_end(s: str) -> str:
    return s[:-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn trim_end"), "Got: {}", result);
}

// ===== List slicing patterns =====

#[test]
fn test_s12_list_slice_start_stop() {
    let code = r#"
def sub_list(items: list) -> list:
    return items[1:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn sub_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_slice_start_only() {
    let code = r#"
def tail(items: list) -> list:
    return items[1:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn tail"), "Got: {}", result);
}

#[test]
fn test_s12_list_slice_stop_only() {
    let code = r#"
def head(items: list) -> list:
    return items[:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn head"), "Got: {}", result);
}

#[test]
fn test_s12_list_slice_full() {
    let code = r#"
def clone_list(items: list) -> list:
    return items[:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn clone_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_slice_negative() {
    let code = r#"
def last_two(items: list) -> list:
    return items[-2:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_two"), "Got: {}", result);
}

// ===== Set operations =====

#[test]
fn test_s12_set_issubset() {
    let code = r#"
def check_subset(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_subset"), "Got: {}", result);
}

#[test]
fn test_s12_set_issuperset() {
    let code = r#"
def check_superset(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_superset"), "Got: {}", result);
}

#[test]
fn test_s12_set_isdisjoint() {
    let code = r#"
def check_disjoint(a: set, b: set) -> bool:
    return a.isdisjoint(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_disjoint"), "Got: {}", result);
}

#[test]
fn test_s12_set_pop() {
    let code = r#"
def pop_from_set(s: set) -> int:
    return s.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_from_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_remove() {
    let code = r#"
def remove_from_set(s: set, item: int):
    s.remove(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_from_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_discard() {
    let code = r#"
def discard_from_set(s: set, item: int):
    s.discard(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn discard_from_set"), "Got: {}", result);
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
def unique_to_a(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_to_a"), "Got: {}", result);
}

#[test]
fn test_s12_set_symmetric_difference() {
    let code = r#"
def xor_sets(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn xor_sets"), "Got: {}", result);
}

// ===== Dict methods =====

#[test]
fn test_s12_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str, default: int) -> int:
    return d.setdefault(key, default)
"#;
    let result = transpile(code);
    assert!(result.contains("fn ensure_key"), "Got: {}", result);
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

#[test]
fn test_s12_dict_pop_with_default() {
    let code = r#"
def safe_pop(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_pop"), "Got: {}", result);
}

#[test]
fn test_s12_dict_update() {
    let code = r#"
def merge_dicts(a: dict, b: dict):
    a.update(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_dicts"), "Got: {}", result);
}

#[test]
fn test_s12_dict_clear() {
    let code = r#"
def clear_dict(d: dict):
    d.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_dict"), "Got: {}", result);
}

// ===== List methods =====

#[test]
fn test_s12_list_pop_no_arg() {
    let code = r#"
def pop_last_item(items: list) -> int:
    return items.pop()
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_last_item"), "Got: {}", result);
}

#[test]
fn test_s12_list_pop_with_index() {
    let code = r#"
def pop_at(items: list, idx: int) -> int:
    return items.pop(idx)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_at"), "Got: {}", result);
}

#[test]
fn test_s12_list_insert() {
    let code = r#"
def insert_at(items: list, idx: int, val: int):
    items.insert(idx, val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_at"), "Got: {}", result);
}

#[test]
fn test_s12_list_remove() {
    let code = r#"
def remove_item(items: list, val: int):
    items.remove(val)
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
def reverse_list(items: list):
    items.reverse()
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_sort() {
    let code = r#"
def sort_list(items: list):
    items.sort()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_extend() {
    let code = r#"
def extend_list(items: list, more: list):
    items.extend(more)
"#;
    let result = transpile(code);
    assert!(result.contains("fn extend_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_clear() {
    let code = r#"
def clear_list(items: list):
    items.clear()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_list"), "Got: {}", result);
}

// ===== String methods - justification and formatting =====

#[test]
fn test_s12_str_center() {
    let code = r#"
def center_text(s: str) -> str:
    return s.center(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12_str_center_with_fill() {
    let code = r#"
def center_fill(s: str) -> str:
    return s.center(20, "*")
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_fill"), "Got: {}", result);
}

#[test]
fn test_s12_str_ljust() {
    let code = r#"
def left_justify(s: str) -> str:
    return s.ljust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_justify"), "Got: {}", result);
}

#[test]
fn test_s12_str_ljust_with_fill() {
    let code = r#"
def left_fill(s: str) -> str:
    return s.ljust(20, "-")
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_fill"), "Got: {}", result);
}

#[test]
fn test_s12_str_rjust() {
    let code = r#"
def right_justify(s: str) -> str:
    return s.rjust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_justify"), "Got: {}", result);
}

#[test]
fn test_s12_str_rjust_with_fill() {
    let code = r#"
def right_fill(s: str) -> str:
    return s.rjust(20, "0")
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_fill"), "Got: {}", result);
}

#[test]
fn test_s12_str_zfill() {
    let code = r#"
def zero_fill(s: str) -> str:
    return s.zfill(10)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_fill"), "Got: {}", result);
}

// ===== String case methods =====

#[test]
fn test_s12_str_title() {
    let code = r#"
def to_title(s: str) -> str:
    return s.title()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_title"), "Got: {}", result);
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
def capitalize_text(s: str) -> str:
    return s.capitalize()
"#;
    let result = transpile(code);
    assert!(result.contains("fn capitalize_text"), "Got: {}", result);
}

#[test]
fn test_s12_str_casefold() {
    let code = r#"
def fold_case(s: str) -> str:
    return s.casefold()
"#;
    let result = transpile(code);
    assert!(result.contains("fn fold_case"), "Got: {}", result);
}

// ===== String check methods =====

#[test]
fn test_s12_str_isupper() {
    let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_upper"), "Got: {}", result);
}

#[test]
fn test_s12_str_islower() {
    let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_lower"), "Got: {}", result);
}

#[test]
fn test_s12_str_isspace() {
    let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_space"), "Got: {}", result);
}

#[test]
fn test_s12_str_isnumeric() {
    let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_numeric"), "Got: {}", result);
}

#[test]
fn test_s12_str_isdecimal() {
    let code = r#"
def check_decimal(s: str) -> bool:
    return s.isdecimal()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_decimal"), "Got: {}", result);
}

#[test]
fn test_s12_str_istitle() {
    let code = r#"
def check_title(s: str) -> bool:
    return s.istitle()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_title"), "Got: {}", result);
}

#[test]
fn test_s12_str_isprintable() {
    let code = r#"
def check_printable(s: str) -> bool:
    return s.isprintable()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_printable"), "Got: {}", result);
}

#[test]
fn test_s12_str_isidentifier() {
    let code = r#"
def check_identifier(s: str) -> bool:
    return s.isidentifier()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_identifier"), "Got: {}", result);
}

// ===== String partition =====

#[test]
fn test_s12_str_partition() {
    let code = r#"
def split_at(s: str, sep: str) -> tuple:
    return s.partition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at"), "Got: {}", result);
}

#[test]
fn test_s12_str_rpartition() {
    let code = r#"
def split_at_last(s: str, sep: str) -> tuple:
    return s.rpartition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at_last"), "Got: {}", result);
}

// ===== String encoding =====

#[test]
fn test_s12_str_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bytes"), "Got: {}", result);
}

#[test]
fn test_s12_str_encode_utf8() {
    let code = r#"
def to_utf8(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_utf8"), "Got: {}", result);
}

// ===== String find/count =====

#[test]
fn test_s12_str_find() {
    let code = r#"
def find_sub(s: str, sub: str) -> int:
    return s.find(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_sub"), "Got: {}", result);
}

#[test]
fn test_s12_str_count() {
    let code = r#"
def count_sub(s: str, sub: str) -> int:
    return s.count(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_sub"), "Got: {}", result);
}

// ===== String format =====

#[test]
fn test_s12_str_format_method() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

// ===== sys.stdout/stderr =====

#[test]
fn test_s12_sys_stdout_write() {
    let code = r#"
import sys

def write_out(msg: str):
    sys.stdout.write(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_out"), "Got: {}", result);
}

#[test]
fn test_s12_sys_stderr_write() {
    let code = r#"
import sys

def write_err(msg: str):
    sys.stderr.write(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_err"), "Got: {}", result);
}

// ===== File I/O methods =====

#[test]
fn test_s12_file_readlines() {
    let code = r#"
def read_all_lines(path: str) -> list:
    with open(path) as f:
        return f.readlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_all_lines"), "Got: {}", result);
}

#[test]
fn test_s12_file_write() {
    let code = r#"
def write_data(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_data"), "Got: {}", result);
}

#[test]
fn test_s12_file_writelines() {
    let code = r#"
def write_lines(path: str, lines: list):
    with open(path, "w") as f:
        f.writelines(lines)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_lines"), "Got: {}", result);
}

// ===== Enumerate and zip =====

#[test]
fn test_s12_enumerate_loop() {
    let code = r#"
def indexed_sum(items: list) -> int:
    total = 0
    for i, val in enumerate(items):
        total += i + val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_sum"), "Got: {}", result);
}

#[test]
fn test_s12_zip_loop() {
    let code = r#"
def paired_sum(a: list, b: list) -> int:
    total = 0
    for x, y in zip(a, b):
        total += x + y
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn paired_sum"), "Got: {}", result);
}

// ===== Type conversions =====

#[test]
fn test_s12_int_from_string() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"), "Got: {}", result);
}

#[test]
fn test_s12_float_from_string() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_float"), "Got: {}", result);
}

#[test]
fn test_s12_str_from_int() {
    let code = r#"
def int_to_str(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_to_str"), "Got: {}", result);
}

#[test]
fn test_s12_bool_conversion() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Got: {}", result);
}

// ===== Tuple operations =====

#[test]
fn test_s12_tuple_return() {
    let code = r#"
def make_pair(a: int, b: str) -> tuple:
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_pair"), "Got: {}", result);
}

#[test]
fn test_s12_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

// ===== Complex patterns =====

#[test]
fn test_s12_nested_dict_access() {
    let code = r#"
def nested_get(data: dict, key1: str, key2: str) -> int:
    inner = data[key1]
    return inner[key2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_get"), "Got: {}", result);
}

#[test]
fn test_s12_list_of_dicts() {
    let code = r#"
def get_names(items: list) -> list:
    result = []
    for item in items:
        result.append(item["name"])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_names"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comprehension() {
    let code = r#"
def invert(d: dict) -> dict:
    return {v: k for k, v in d.items()}
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert"), "Got: {}", result);
}

#[test]
fn test_s12_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"), "Got: {}", result);
}

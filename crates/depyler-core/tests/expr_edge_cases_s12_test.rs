//! Session 12 Batch 28: Expression edge cases and cold paths in expr_gen.rs
//!
//! Targets:
//! - Slice operations (start-only, stop-only, full, step)
//! - Negative index access
//! - Chained comparisons
//! - Augmented assignments (+=, -=, *=, /=, //=, %=, **=, &=, |=, ^=, <<=, >>=)
//! - Multiple assignment targets
//! - Tuple unpacking
//! - Walrus operator (:=)
//! - Complex method chains
//! - Nested subscript access
//! - Star unpacking

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

// ===== Slice operations =====

#[test]
fn test_s12_slice_start_only() {
    let code = r#"
def tail(items: list, start: int) -> list:
    return items[start:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn tail"), "Got: {}", result);
}

#[test]
fn test_s12_slice_stop_only() {
    let code = r#"
def head(items: list, n: int) -> list:
    return items[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn head"), "Got: {}", result);
}

#[test]
fn test_s12_slice_start_stop() {
    let code = r#"
def middle(items: list, start: int, end: int) -> list:
    return items[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn middle"), "Got: {}", result);
}

#[test]
fn test_s12_slice_with_step() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"), "Got: {}", result);
}

#[test]
fn test_s12_slice_reverse() {
    let code = r#"
def reverse_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_list"), "Got: {}", result);
}

#[test]
fn test_s12_string_slice() {
    let code = r#"
def first_n_chars(text: str, n: int) -> str:
    return text[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_n_chars"), "Got: {}", result);
}

// ===== Negative index access =====

#[test]
fn test_s12_negative_index() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_item"), "Got: {}", result);
}

#[test]
fn test_s12_negative_index_two() {
    let code = r#"
def second_to_last(items: list) -> int:
    return items[-2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn second_to_last"), "Got: {}", result);
}

// ===== Augmented assignments =====

#[test]
fn test_s12_augmented_add() {
    let code = r#"
def accumulate(items: list) -> int:
    total = 0
    for x in items:
        total += x
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_sub() {
    let code = r#"
def countdown(n: int) -> int:
    while n > 0:
        n -= 1
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn countdown"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_mul() {
    let code = r#"
def factorial(n: int) -> int:
    result = 1
    i = 2
    while i <= n:
        result *= i
        i += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_div() {
    let code = r#"
def halve_repeatedly(x: float, times: int) -> float:
    i = 0
    while i < times:
        x /= 2.0
        i += 1
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve_repeatedly"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_floor_div() {
    let code = r#"
def reduce(n: int) -> int:
    n //= 2
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn reduce"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_mod() {
    let code = r#"
def wrap(n: int, limit: int) -> int:
    n %= limit
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn wrap"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_pow() {
    let code = r#"
def square_in_place(x: int) -> int:
    x **= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn square_in_place"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitand() {
    let code = r#"
def clear_high_bits(n: int) -> int:
    n &= 0xFF
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_high_bits"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitor() {
    let code = r#"
def set_bits(n: int, mask: int) -> int:
    n |= mask
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_bits"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_bitxor() {
    let code = r#"
def flip_bits(n: int, mask: int) -> int:
    n ^= mask
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn flip_bits"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_lshift() {
    let code = r#"
def double_shift(n: int) -> int:
    n <<= 1
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_shift"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_rshift() {
    let code = r#"
def half_shift(n: int) -> int:
    n >>= 1
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn half_shift"), "Got: {}", result);
}

// ===== Multiple assignment / tuple unpacking =====

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

#[test]
fn test_s12_triple_unpack() {
    let code = r#"
def parse_point(data: str) -> tuple:
    x, y, z = data.split(",")
    return (x, y, z)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_point"), "Got: {}", result);
}

// ===== Chained comparisons =====

#[test]
fn test_s12_chained_comparison() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

// ===== Complex method chains =====

#[test]
fn test_s12_method_chain_string() {
    let code = r#"
def clean(text: str) -> str:
    return text.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

// ===== Dict operations =====

#[test]
fn test_s12_dict_get_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_get"), "Got: {}", result);
}

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
fn test_s12_dict_update() {
    let code = r#"
def merge_dicts(a: dict, b: dict) -> dict:
    a.update(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_dicts"), "Got: {}", result);
}

#[test]
fn test_s12_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

#[test]
fn test_s12_dict_items_loop() {
    let code = r#"
def print_pairs(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_pairs"), "Got: {}", result);
}

// ===== List methods =====

#[test]
fn test_s12_list_insert() {
    let code = r#"
def insert_at_front(items: list, val: int) -> list:
    items.insert(0, val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn insert_at_front"), "Got: {}", result);
}

#[test]
fn test_s12_list_remove() {
    let code = r#"
def remove_first(items: list, val: int) -> list:
    items.remove(val)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_first"), "Got: {}", result);
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
def count_occurrences(items: list, val: int) -> int:
    return items.count(val)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_occurrences"), "Got: {}", result);
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

#[test]
fn test_s12_list_clear() {
    let code = r#"
def clear_list(items: list) -> list:
    items.clear()
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn clear_list"), "Got: {}", result);
}

#[test]
fn test_s12_list_extend() {
    let code = r#"
def combine(a: list, b: list) -> list:
    a.extend(b)
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Got: {}", result);
}

#[test]
fn test_s12_list_pop_with_index() {
    let code = r#"
def pop_front(items: list) -> int:
    return items.pop(0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pop_front"), "Got: {}", result);
}

// ===== String methods =====

#[test]
fn test_s12_string_zfill() {
    let code = r#"
def pad_number(n: str) -> str:
    return n.zfill(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_number"), "Got: {}", result);
}

#[test]
fn test_s12_string_center() {
    let code = r#"
def center_text(text: str) -> str:
    return text.center(40)
"#;
    let result = transpile(code);
    assert!(result.contains("fn center_text"), "Got: {}", result);
}

#[test]
fn test_s12_string_ljust() {
    let code = r#"
def left_pad(text: str) -> str:
    return text.ljust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn left_pad"), "Got: {}", result);
}

#[test]
fn test_s12_string_rjust() {
    let code = r#"
def right_pad(text: str) -> str:
    return text.rjust(20)
"#;
    let result = transpile(code);
    assert!(result.contains("fn right_pad"), "Got: {}", result);
}

#[test]
fn test_s12_string_count() {
    let code = r#"
def count_char(text: str, c: str) -> int:
    return text.count(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_char"), "Got: {}", result);
}

#[test]
fn test_s12_string_find() {
    let code = r#"
def find_substr(text: str, sub: str) -> int:
    return text.find(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_substr"), "Got: {}", result);
}

#[test]
fn test_s12_string_rfind() {
    let code = r#"
def rfind_substr(text: str, sub: str) -> int:
    return text.rfind(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rfind_substr"), "Got: {}", result);
}

#[test]
fn test_s12_string_encode() {
    let code = r#"
def to_bytes(text: str) -> bytes:
    return text.encode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bytes"), "Got: {}", result);
}

#[test]
fn test_s12_string_splitlines() {
    let code = r#"
def get_lines(text: str) -> list:
    return text.splitlines()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_lines"), "Got: {}", result);
}

#[test]
fn test_s12_string_expandtabs() {
    let code = r#"
def expand(text: str) -> str:
    return text.expandtabs(4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn expand"), "Got: {}", result);
}

#[test]
fn test_s12_string_partition() {
    let code = r#"
def split_at(text: str, sep: str) -> tuple:
    return text.partition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_at"), "Got: {}", result);
}

#[test]
fn test_s12_string_rpartition() {
    let code = r#"
def rsplit_at(text: str, sep: str) -> tuple:
    return text.rpartition(sep)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rsplit_at"), "Got: {}", result);
}

// ===== Complex real-world patterns =====

#[test]
fn test_s12_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Got: {}", result);
}

#[test]
fn test_s12_matrix_multiply() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    total = 0
    for i in range(len(a)):
        total += a[i] * b[i]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn dot_product"), "Got: {}", result);
}

#[test]
fn test_s12_frequency_counter() {
    let code = r#"
def count_chars(text: str) -> dict:
    counts = {}
    for c in text:
        if c in counts:
            counts[c] += 1
        else:
            counts[c] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_chars"), "Got: {}", result);
}

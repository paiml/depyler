//! Session 12 Batch 88: Iteration and loop pattern cold paths
//!
//! Targets codegen paths for various iteration patterns including
//! enumerate, zip, reversed, sorted, filter, map, and complex loops.

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

#[test]
fn test_s12_b88_enumerate_with_start() {
    let code = r#"
def number_lines(lines: list) -> list:
    result = []
    for num, line in enumerate(lines, 1):
        result.append((num, line))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn number_lines"), "Got: {}", result);
}

#[test]
fn test_s12_b88_zip_two_lists() {
    let code = r#"
def combine(keys: list, values: list) -> dict:
    result = {}
    for k, v in zip(keys, values):
        result[k] = v
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Got: {}", result);
}

#[test]
fn test_s12_b88_reversed_iteration() {
    let code = r#"
def reverse_sum(items: list) -> list:
    result = []
    total = 0
    for item in reversed(items):
        total += item
        result.append(total)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b88_sorted_iteration() {
    let code = r#"
def process_sorted(items: list) -> list:
    result = []
    for item in sorted(items):
        result.append(item * 2)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_sorted"), "Got: {}", result);
}

#[test]
fn test_s12_b88_range_with_step() {
    let code = r#"
def odd_numbers(n: int) -> list:
    return list(range(1, n, 2))
"#;
    let result = transpile(code);
    assert!(result.contains("fn odd_numbers"), "Got: {}", result);
}

#[test]
fn test_s12_b88_range_negative_step() {
    let code = r#"
def countdown(n: int) -> list:
    return list(range(n, 0, -1))
"#;
    let result = transpile(code);
    assert!(result.contains("fn countdown"), "Got: {}", result);
}

#[test]
fn test_s12_b88_while_with_complex_cond() {
    let code = r#"
def collect_while(items: list, limit: int) -> list:
    result = []
    i = 0
    total = 0
    while i < len(items) and total < limit:
        total += items[i]
        result.append(items[i])
        i += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn collect_while"), "Got: {}", result);
}

#[test]
fn test_s12_b88_nested_for_with_break() {
    let code = r#"
def find_in_matrix(matrix: list, target: int) -> tuple:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_in_matrix"), "Got: {}", result);
}

#[test]
fn test_s12_b88_iter_over_string() {
    let code = r#"
def char_frequency(text: str) -> dict:
    freq = {}
    for c in text:
        freq[c] = freq.get(c, 0) + 1
    return freq
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_frequency"), "Got: {}", result);
}

#[test]
fn test_s12_b88_iter_over_dict_keys() {
    let code = r#"
def copy_keys(d: dict) -> list:
    result = []
    for key in d:
        result.append(key)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b88_iter_with_index() {
    let code = r#"
def find_all(items: list, target: int) -> list:
    indices = []
    for i in range(len(items)):
        if items[i] == target:
            indices.append(i)
    return indices
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_all"), "Got: {}", result);
}

#[test]
fn test_s12_b88_while_true_break() {
    let code = r#"
def read_until_zero(items: list) -> list:
    result = []
    i = 0
    while True:
        if i >= len(items) or items[i] == 0:
            break
        result.append(items[i])
        i += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_until_zero"), "Got: {}", result);
}

#[test]
fn test_s12_b88_for_with_continue() {
    let code = r#"
def skip_negatives(items: list) -> list:
    result = []
    for item in items:
        if item < 0:
            continue
        result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_negatives"), "Got: {}", result);
}

#[test]
fn test_s12_b88_sliding_window() {
    let code = r#"
def max_sum_window(items: list, k: int) -> int:
    if len(items) < k:
        return 0
    current = sum(items[:k])
    best = current
    for i in range(k, len(items)):
        current += items[i] - items[i - k]
        if current > best:
            best = current
    return best
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_sum_window"), "Got: {}", result);
}

#[test]
fn test_s12_b88_two_pointer() {
    let code = r#"
def is_palindrome_list(items: list) -> bool:
    left = 0
    right = len(items) - 1
    while left < right:
        if items[left] != items[right]:
            return False
        left += 1
        right -= 1
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome_list"), "Got: {}", result);
}

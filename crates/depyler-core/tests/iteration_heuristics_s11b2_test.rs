//! Session 11 Batch 2: Iteration heuristic coverage
//!
//! Targets:
//! - stmt_gen.rs:2869 is_file_iteration() name heuristics
//! - stmt_gen.rs:2900 is_csv_reader_iteration() name heuristics
//! - stmt_gen.rs:3277 string iteration detection heuristics
//! - stmt_gen.rs:3038 needs_enumerate_index_cast()
//! - stmt_gen.rs:3040 needs_char_to_string_conversion()
//! - expr_gen.rs:1673 containment operations (in/not in)

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

// ===== File iteration heuristics (variable name detection) =====

#[test]
fn test_s11b2_for_line_in_f() {
    let code = r#"
def read_f(path: str) -> list:
    result = []
    f = open(path)
    for line in f:
        result.append(line.strip())
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_f"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_line_in_file() {
    let code = r#"
def read_file(path: str) -> int:
    count = 0
    file = open(path)
    for line in file:
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_line_in_input_file() {
    let code = r#"
def process(path: str) -> list:
    lines = []
    input_file = open(path)
    for line in input_file:
        lines.append(line)
    return lines
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_line_in_output_file() {
    let code = r#"
def copy_lines(path: str) -> list:
    data = []
    output_file = open(path)
    for line in output_file:
        data.append(line)
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn copy_lines"), "Got: {}", result);
}

// ===== CSV reader iteration heuristics =====

#[test]
fn test_s11b2_csv_reader_var() {
    let code = r#"
import csv

def parse_csv(path: str) -> list:
    result = []
    reader = csv.reader(open(path))
    for row in reader:
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_csv"), "Got: {}", result);
}

#[test]
fn test_s11b2_csv_reader_suffix() {
    let code = r#"
import csv

def read_data(path: str) -> list:
    rows = []
    csv_reader = csv.reader(open(path))
    for row in csv_reader:
        rows.append(row)
    return rows
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_data"), "Got: {}", result);
}

// ===== String iteration heuristics =====

#[test]
fn test_s11b2_iterate_str_typed() {
    let code = r#"
def char_count(text: str) -> int:
    count = 0
    for c in text:
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_count"), "Got: {}", result);
}

#[test]
fn test_s11b2_iterate_word_var() {
    let code = r#"
def char_list(word: str) -> list:
    result = []
    for ch in word:
        result.append(ch)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_list"), "Got: {}", result);
}

#[test]
fn test_s11b2_iterate_sentence_var() {
    let code = r#"
def vowel_count(sentence: str) -> int:
    n = 0
    for c in sentence:
        if c in "aeiouAEIOU":
            n += 1
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn vowel_count"), "Got: {}", result);
}

#[test]
fn test_s11b2_iterate_line_var() {
    let code = r#"
def count_spaces(line: str) -> int:
    n = 0
    for c in line:
        if c == " ":
            n += 1
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_spaces"), "Got: {}", result);
}

#[test]
fn test_s11b2_iterate_name_var() {
    let code = r#"
def upper_name(name: str) -> str:
    result = ""
    for c in name:
        result += c.upper()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_name"), "Got: {}", result);
}

// ===== Containment operations =====

#[test]
fn test_s11b2_in_tuple() {
    let code = r#"
def is_vowel(c: str) -> bool:
    return c in ("a", "e", "i", "o", "u")
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_vowel"), "Got: {}", result);
}

#[test]
fn test_s11b2_not_in_tuple() {
    let code = r#"
def is_consonant(c: str) -> bool:
    return c not in ("a", "e", "i", "o", "u")
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_consonant"), "Got: {}", result);
}

#[test]
fn test_s11b2_in_set() {
    let code = r#"
def is_primary(color: str) -> bool:
    return color in {"red", "blue", "yellow"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_primary"), "Got: {}", result);
}

#[test]
fn test_s11b2_in_dict() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_key"), "Got: {}", result);
}

#[test]
fn test_s11b2_not_in_list() {
    let code = r#"
def is_new(items: list, val: int) -> bool:
    return val not in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_new"), "Got: {}", result);
}

#[test]
fn test_s11b2_in_string_literal() {
    let code = r#"
def is_digit_char(c: str) -> bool:
    return c in "0123456789"
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_digit_char"), "Got: {}", result);
}

#[test]
fn test_s11b2_not_in_string() {
    let code = r#"
def has_no_space(s: str) -> bool:
    return " " not in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_no_space"), "Got: {}", result);
}

// ===== Enumerate patterns =====

#[test]
fn test_s11b2_enumerate_basic() {
    let code = r#"
def with_index(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_index"), "Got: {}", result);
}

#[test]
fn test_s11b2_enumerate_with_start() {
    let code = r#"
def numbered(items: list) -> list:
    result = []
    for i, item in enumerate(items, 1):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn numbered"), "Got: {}", result);
}

// ===== Dict iteration =====

#[test]
fn test_s11b2_for_dict_keys() {
    let code = r#"
def all_keys(d: dict) -> list:
    result = []
    for k in d.keys():
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_keys"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_dict_values() {
    let code = r#"
def sum_vals(d: dict) -> int:
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_vals"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_dict_items() {
    let code = r#"
def to_list(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_list"), "Got: {}", result);
}

// ===== Zip patterns =====

#[test]
fn test_s11b2_zip_two() {
    let code = r#"
def pair_sum(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair_sum"), "Got: {}", result);
}

#[test]
fn test_s11b2_zip_three() {
    let code = r#"
def triple_sum(a: list, b: list, c: list) -> list:
    result = []
    for x, y, z in zip(a, b, c):
        result.append(x + y + z)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn triple_sum"), "Got: {}", result);
}

// ===== Reversed/sorted iteration =====

#[test]
fn test_s11b2_for_reversed() {
    let code = r#"
def backward(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn backward"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_sorted() {
    let code = r#"
def ordered(items: list) -> list:
    result = []
    for item in sorted(items):
        result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn ordered"), "Got: {}", result);
}

#[test]
fn test_s11b2_for_sorted_reverse() {
    let code = r#"
def descending(items: list) -> list:
    result = []
    for item in sorted(items, reverse=True):
        result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn descending"), "Got: {}", result);
}

// ===== Range patterns =====

#[test]
fn test_s11b2_range_one_arg() {
    let code = r#"
def count(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn count"), "Got: {}", result);
}

#[test]
fn test_s11b2_range_two_args() {
    let code = r#"
def partial_sum(start: int, end: int) -> int:
    total = 0
    for i in range(start, end):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn partial_sum"), "Got: {}", result);
}

#[test]
fn test_s11b2_range_three_args() {
    let code = r#"
def step_sum(start: int, end: int, step: int) -> int:
    total = 0
    for i in range(start, end, step):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn step_sum"), "Got: {}", result);
}

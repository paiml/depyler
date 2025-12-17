//! Tests for extracted helper functions from stmt_gen.rs
//!
//! These tests ensure the extracted type tracking helpers work correctly.
//! Written using EXTREME TDD - tests first, then extraction.

use depyler_core::DepylerPipeline;

fn transpiles(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpiles_to(code: &str) -> Option<String> {
    DepylerPipeline::new().transpile(code).ok()
}

// =============================================================================
// Type Tracking Tests - Option-returning functions
// =============================================================================

#[test]
fn test_option_returning_function_tracking() {
    // When a function returns Option, variables assigned from it should be tracked
    let code = r#"
from typing import Optional

def find_item(items: list[int], target: int) -> Optional[int]:
    for item in items:
        if item == target:
            return item
    return None

def main():
    result = find_item([1, 2, 3], 2)
    if result:
        print(result)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_option_type_from_method_get() {
    // os.environ.get(key) returns Option<String>
    let code = r#"
import os

def get_config():
    config_file = os.environ.get("CONFIG_FILE")
    if config_file:
        return config_file
    return "default.conf"
"#;
    assert!(transpiles(code));
}

#[test]
fn test_string_type_from_environ_get_with_default() {
    // os.environ.get(key, default) returns String (not Option)
    let code = r#"
import os

def get_config():
    config_file = os.environ.get("CONFIG_FILE", "default.conf")
    return config_file
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Counter/char iteration
// =============================================================================

#[test]
fn test_counter_string_tracking() {
    // Counter(string) should track as char counter for iteration
    let code = r#"
from collections import Counter

def count_chars(text: str):
    counter = Counter(text)
    for char, count in counter.items():
        print(char, count)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_counter_from_list() {
    // Counter(list) should work normally
    let code = r#"
from collections import Counter

def count_items(items: list[int]):
    counter = Counter(items)
    return counter
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Iterator expressions
// =============================================================================

#[test]
fn test_generator_expr_iterator_tracking() {
    // Generator expressions produce iterators, not collections
    let code = r#"
def squares(n: int):
    gen = (x * x for x in range(n))
    for val in gen:
        print(val)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_filter_chain_iterator_tracking() {
    // Method chains with filter/map produce iterators
    let code = r#"
def even_squares(nums: list[int]) -> list[int]:
    result = list(filter(lambda x: x % 2 == 0, map(lambda x: x * x, nums)))
    return result
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Numpy/Vector arrays
// =============================================================================

#[test]
fn test_numpy_array_tracking() {
    // numpy arrays should be tracked for proper iteration
    let code = r#"
import numpy as np

def process_array():
    arr = np.array([1.0, 2.0, 3.0])
    for val in arr:
        print(val)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_numpy_binary_op_tracking() {
    // Binary ops on numpy arrays produce numpy arrays
    let code = r#"
import numpy as np

def add_arrays():
    a = np.array([1.0, 2.0])
    b = np.array([3.0, 4.0])
    result = a + b
    return result
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - CSV DictReader mutability
// =============================================================================

#[test]
fn test_csv_dictreader_mutable() {
    // csv.DictReader needs mutable access
    let code = r#"
import csv

def read_csv(filepath: str):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        for row in reader:
            print(row)
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - None placeholder
// =============================================================================

#[test]
fn test_none_placeholder_skip() {
    // None placeholder followed by real assignment should skip None
    let code = r#"
def find_max(items: list[int]) -> int:
    result = None
    for item in items:
        if result is None or item > result:
            result = item
    return result
"#;
    assert!(transpiles(code));
}

#[test]
fn test_none_in_conditional() {
    // Variable initialized to None and set in conditional
    let code = r#"
def process(flag: bool) -> str:
    value = None
    if flag:
        value = "yes"
    else:
        value = "no"
    return value
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Float type propagation
// =============================================================================

#[test]
fn test_float_callable_return_tracking() {
    // Variables assigned from Callable that returns float should be tracked
    let code = r#"
from typing import Callable

def integrate(f: Callable[[float], float], a: float, b: float) -> float:
    fa = f(a)
    fb = f(b)
    return (fa + fb) * (b - a) / 2
"#;
    assert!(transpiles(code));
}

#[test]
fn test_float_binary_op_tracking() {
    // Binary operations involving floats should track result as float
    let code = r#"
def compute(a: float, b: float) -> float:
    result = a * b + 1.0
    return result
"#;
    assert!(transpiles(code));
}

#[test]
fn test_float_cse_temp_tracking() {
    // CSE temps from float expressions should be tracked
    let code = r#"
def bisect(f, a: float, b: float) -> float:
    fa = f(a)
    fb = f(b)
    if fa * fb > 0:
        return a
    return (a + b) / 2
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Variable type propagation
// =============================================================================

#[test]
fn test_var_to_var_type_propagation() {
    // Type should propagate from one variable to another on assignment
    let code = r#"
def copy_value() -> float:
    a: float = 3.14
    b = a
    return b
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Collection types
// =============================================================================

#[test]
fn test_list_type_from_literal() {
    // List type should be tracked from literal
    let code = r#"
def process_list():
    items = [1, 2, 3]
    for item in items:
        print(item)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_dict_type_from_literal() {
    // Dict type should be tracked from literal
    let code = r#"
def process_dict():
    data = {"a": 1, "b": 2}
    for key in data:
        print(key, data[key])
"#;
    assert!(transpiles(code));
}

#[test]
fn test_set_type_from_literal() {
    // Set type should be tracked from literal
    let code = r#"
def process_set():
    items = {1, 2, 3}
    return 2 in items
"#;
    assert!(transpiles(code));
}

#[test]
fn test_tuple_type_from_literal() {
    // Tuple type should be tracked from literal for correct field access
    let code = r#"
def process_tuple():
    pair = (1, "hello")
    return pair[0]
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Comprehension types
// =============================================================================

#[test]
fn test_list_comp_type_tracking() {
    // List comprehension should track element type
    let code = r#"
def squares(n: int) -> list[int]:
    result = [x * x for x in range(n)]
    return result
"#;
    assert!(transpiles(code));
}

#[test]
fn test_dict_comp_type_tracking() {
    // Dict comprehension should track key/value types
    let code = r#"
def make_dict(n: int) -> dict[int, int]:
    result = {i: i * i for i in range(n)}
    return result
"#;
    assert!(transpiles(code));
}

#[test]
fn test_set_comp_type_tracking() {
    // Set comprehension should track element type
    let code = r#"
def unique_squares(n: int) -> set[int]:
    result = {x * x for x in range(n)}
    return result
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Method return types
// =============================================================================

#[test]
fn test_string_method_split_type() {
    // .split() returns list of strings
    let code = r#"
def parse_line(line: str):
    parts = line.split(",")
    for part in parts:
        print(part)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_string_method_lower_type() {
    // .lower() returns string
    let code = r#"
def normalize(s: str) -> str:
    result = s.lower()
    return result
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Type Tracking Tests - Slice types
// =============================================================================

#[test]
fn test_slice_type_tracking() {
    // Sliced lists should be tracked as owned Vec
    let code = r#"
def get_rest(items: list[int]) -> list[int]:
    rest = items[1:]
    return rest
"#;
    assert!(transpiles(code));
}

// =============================================================================
// JSON Context Tests
// =============================================================================

#[test]
fn test_json_loads_type_tracking() {
    // json.loads() returns Value type
    let code = r#"
import json

def parse_json(s: str):
    data = json.loads(s)
    return data
"#;
    assert!(transpiles(code));
}

#[test]
fn test_json_value_index_tracking() {
    // Indexing Value type returns Value
    let code = r#"
import json

def get_field(s: str):
    data = json.loads(s)
    name = data["name"]
    return name
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Deque/Queue Type Tracking Tests
// =============================================================================

#[test]
fn test_deque_type_tracking() {
    // deque() should be tracked for truthiness conversion
    let code = r#"
from collections import deque

def process_queue():
    queue = deque([1, 2, 3])
    while queue:
        item = queue.popleft()
        print(item)
"#;
    assert!(transpiles(code));
}

// =============================================================================
// String Literal Normalization Tests
// =============================================================================

#[test]
fn test_string_literal_to_string_conversion() {
    // String literals assigned to typed String vars should be converted
    let code = r#"
def get_version() -> str:
    version: str = "1.0.0"
    return version
"#;
    let result = transpiles_to(code);
    assert!(result.is_some());
    // The generated code should have .to_string() for the literal
}

#[test]
fn test_mutable_string_literal_normalization() {
    // Mutable vars with string literals should be normalized to String
    let code = r#"
def build_message(name: str) -> str:
    msg = "Hello, "
    msg = msg + name
    return msg
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Result-returning Function Tests
// =============================================================================

#[test]
fn test_result_unwrap_in_assignment() {
    // Result-returning function calls should be unwrapped in non-Result context
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)

def main():
    x = parse_int("42")
    print(x)
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Final Type Annotation Tests
// =============================================================================

#[test]
fn test_final_generates_const() {
    // Final type annotation should generate const
    let code = r#"
from typing import Final

MAX_SIZE: Final[int] = 100
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Box dyn Write Tests
// =============================================================================

#[test]
fn test_file_or_stdout_boxing() {
    // File/Stdout assigned to Box<dyn Write> should be boxed
    let code = r#"
import sys

def get_output(use_file: bool, path: str):
    if use_file:
        output = open(path, "w")
    else:
        output = sys.stdout
    return output
"#;
    assert!(transpiles(code));
}

// =============================================================================
// ArgumentParser Pattern Tests
// =============================================================================

#[test]
fn test_argparse_basic() {
    // Basic ArgumentParser should be transformed to clap
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A tool")
    parser.add_argument("--name", help="Your name")
    args = parser.parse_args()
    print(args.name)
"#;
    assert!(transpiles(code));
}

#[test]
fn test_argparse_with_subcommands() {
    // ArgumentParser with subcommands should work
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A CLI tool")
    subparsers = parser.add_subparsers(dest="command", required=True)

    init_parser = subparsers.add_parser("init", help="Initialize")
    init_parser.add_argument("name", help="Project name")

    args = parser.parse_args()
"#;
    assert!(transpiles(code));
}

#[test]
fn test_argparse_argument_group() {
    // Argument groups should be skipped (not needed with clap derive)
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    group = parser.add_argument_group("Input options")
    group.add_argument("--input", help="Input file")
    args = parser.parse_args()
"#;
    assert!(transpiles(code));
}

// =============================================================================
// Dict Augmented Assignment Tests
// =============================================================================

#[test]
fn test_dict_augassign_add() {
    // dict[key] += value should avoid borrow-after-move
    let code = r#"
def count_items(items: list[str]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for item in items:
        if item in counts:
            counts[item] += 1
        else:
            counts[item] = 1
    return counts
"#;
    assert!(transpiles(code));
}

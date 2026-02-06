//! Session 12: Targeted tests for stmt_gen.rs and rust_gen/mod.rs uncovered paths
//!
//! Targets:
//! - Complex try/except patterns
//! - Generator/yield patterns
//! - Async/await
//! - While True -> loop conversion
//! - TYPE_CHECKING elision
//! - Complex assignment patterns
//! - Global/nonlocal statements
//! - Assert patterns
//! - Del statements
//! - Complex class patterns with multiple methods

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

// ===== while True -> loop =====

#[test]
fn test_s12_while_true_basic() {
    let code = r#"
def event_loop() -> int:
    count = 0
    while True:
        count += 1
        if count >= 10:
            break
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn event_loop"), "Got: {}", result);
    assert!(result.contains("loop"), "Expected 'loop' keyword, got: {}", result);
}

#[test]
fn test_s12_while_true_with_continue() {
    let code = r#"
def skip_odds() -> list:
    result = []
    i = 0
    while True:
        i += 1
        if i > 10:
            break
        if i % 2 != 0:
            continue
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_odds"), "Got: {}", result);
}

#[test]
fn test_s12_while_true_nested() {
    let code = r#"
def nested_loops() -> int:
    total = 0
    i = 0
    while True:
        if i >= 5:
            break
        j = 0
        while True:
            if j >= 3:
                break
            total += 1
            j += 1
        i += 1
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_loops"), "Got: {}", result);
}

// ===== TYPE_CHECKING elision =====

#[test]
fn test_s12_type_checking_import() {
    let code = r#"
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from os import PathLike

def process(path: str) -> str:
    return path
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Generator/yield =====

#[test]
fn test_s12_simple_generator() {
    let code = r#"
def count_up(n: int):
    for i in range(n):
        yield i
"#;
    let result = transpile(code);
    assert!(result.contains("count_up"), "Got: {}", result);
}

#[test]
fn test_s12_generator_with_condition() {
    let code = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    let result = transpile(code);
    assert!(result.contains("even_numbers"), "Got: {}", result);
}

// ===== Async/await =====

#[test]
fn test_s12_async_function() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(result.contains("fetch_data"), "Got: {}", result);
}

#[test]
fn test_s12_async_with_await() {
    let code = r#"
async def process():
    result = await get_data()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("process"), "Got: {}", result);
}

// ===== Complex assignment patterns =====

#[test]
fn test_s12_tuple_unpack_three() {
    let code = r#"
def unpack(t: tuple) -> int:
    a, b, c = t
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn unpack"), "Got: {}", result);
}

#[test]
fn test_s12_swap_variables() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_string_concat() {
    let code = r#"
def build_string(items: list) -> str:
    result = ""
    for item in items:
        result += str(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_string"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_list_extend() {
    let code = r#"
def accumulate(items: list) -> list:
    result = []
    for item in items:
        result += [item]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"), "Got: {}", result);
}

// ===== Assert patterns =====

#[test]
fn test_s12_assert_basic() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    assert b != 0
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_assert_with_message() {
    let code = r#"
def validate(x: int) -> int:
    assert x >= 0, "must be non-negative"
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

// ===== Del statement =====

#[test]
fn test_s12_del_dict_key() {
    let code = r#"
def remove_key(d: dict, key: str):
    del d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

// ===== Return patterns =====

#[test]
fn test_s12_return_none() {
    let code = r#"
def do_nothing():
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn do_nothing"), "Got: {}", result);
}

#[test]
fn test_s12_return_empty() {
    let code = r#"
def early_exit(x: int):
    if x < 0:
        return
    print(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn early_exit"), "Got: {}", result);
}

#[test]
fn test_s12_return_tuple() {
    let code = r#"
def divmod_result(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divmod_result"), "Got: {}", result);
}

#[test]
fn test_s12_return_dict() {
    let code = r#"
def make_config() -> dict:
    return {"host": "localhost", "port": 8080}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_config"), "Got: {}", result);
}

#[test]
fn test_s12_return_list() {
    let code = r#"
def make_list() -> list:
    return [1, 2, 3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_list"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_multiple_methods() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, n: int):
        self.result += n

    def subtract(self, n: int):
        self.result -= n

    def multiply(self, n: int):
        self.result *= n

    def get_result(self) -> int:
        return self.result

    def reset(self):
        self.result = 0
"#;
    let result = transpile(code);
    assert!(result.contains("Calculator"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_class_variable() {
    let code = r#"
class Counter:
    count = 0

    def __init__(self):
        Counter.count += 1

    def get_count(self) -> int:
        return Counter.count
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_eq() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

// ===== Import patterns =====

#[test]
fn test_s12_import_simple() {
    let code = r#"
import os

def get_cwd() -> str:
    return os.getcwd()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_cwd"), "Got: {}", result);
}

#[test]
fn test_s12_from_import() {
    let code = r#"
from os.path import join

def make_path(a: str, b: str) -> str:
    return join(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_path"), "Got: {}", result);
}

#[test]
fn test_s12_from_import_multiple() {
    let code = r#"
from os.path import join, exists

def safe_join(a: str, b: str) -> str:
    path = join(a, b)
    if exists(path):
        return path
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_join"), "Got: {}", result);
}

// ===== Docstring patterns =====

#[test]
fn test_s12_function_docstring() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn add"), "Got: {}", result);
}

#[test]
fn test_s12_class_docstring() {
    let code = r#"
class Greeter:
    """A simple greeter class."""

    def __init__(self, name: str):
        self.name = name

    def greet(self) -> str:
        """Return a greeting."""
        return "Hello " + self.name
"#;
    let result = transpile(code);
    assert!(result.contains("Greeter"), "Got: {}", result);
}

// ===== Complex algorithms =====

#[test]
fn test_s12_quicksort() {
    let code = r#"
def quicksort(items: list) -> list:
    if len(items) <= 1:
        return items
    pivot = items[0]
    left = [x for x in items[1:] if x <= pivot]
    right = [x for x in items[1:] if x > pivot]
    return quicksort(left) + [pivot] + quicksort(right)
"#;
    let result = transpile(code);
    assert!(result.contains("fn quicksort"), "Got: {}", result);
}

#[test]
fn test_s12_merge_sort() {
    let code = r#"
def merge(left: list, right: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    while i < len(left):
        result.append(left[i])
        i += 1
    while j < len(right):
        result.append(right[j])
        j += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

#[test]
fn test_s12_depth_first_search() {
    let code = r#"
def dfs(graph: dict, start: str) -> list:
    visited = []
    stack = [start]
    while stack:
        node = stack.pop()
        if node not in visited:
            visited.append(node)
            for neighbor in graph.get(node, []):
                stack.append(neighbor)
    return visited
"#;
    let result = transpile(code);
    assert!(result.contains("fn dfs"), "Got: {}", result);
}

#[test]
fn test_s12_two_sum() {
    let code = r#"
def two_sum(nums: list, target: int) -> list:
    seen = {}
    for i in range(len(nums)):
        complement = target - nums[i]
        if complement in seen:
            return [seen[complement], i]
        seen[nums[i]] = i
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn two_sum"), "Got: {}", result);
}

#[test]
fn test_s12_longest_common_prefix() {
    let code = r#"
def longest_prefix(strs: list) -> str:
    if not strs:
        return ""
    prefix = strs[0]
    for s in strs[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""
    return prefix
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_prefix"), "Got: {}", result);
}

#[test]
fn test_s12_is_palindrome() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    cleaned = s.lower().replace(" ", "")
    return cleaned == cleaned[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Got: {}", result);
}

// ===== with statement patterns =====

#[test]
fn test_s12_with_open_binary() {
    let code = r#"
def read_binary(path: str) -> bytes:
    with open(path, "rb") as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_binary"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_write_newline() {
    let code = r#"
def write_lines(path: str, lines: list):
    with open(path, "w") as f:
        for line in lines:
            f.write(line + "\n")
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_lines"), "Got: {}", result);
}

// ===== enumerate patterns =====

#[test]
fn test_s12_enumerate_basic() {
    let code = r#"
def indexed(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append((i, item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed"), "Got: {}", result);
}

#[test]
fn test_s12_enumerate_with_start() {
    let code = r#"
def numbered(items: list) -> list:
    result = []
    for i, item in enumerate(items, 1):
        result.append((i, item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn numbered"), "Got: {}", result);
}

// ===== zip patterns =====

#[test]
fn test_s12_zip_two() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append((x, y))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair_up"), "Got: {}", result);
}

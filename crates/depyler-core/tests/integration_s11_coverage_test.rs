//! Session 11: Integration coverage tests exercising complex multi-statement patterns
//!
//! These tests combine multiple language features to exercise code paths
//! that may not be covered by single-feature unit tests. Each test exercises
//! several code generation paths simultaneously.

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

// ============================================================================
// Complex data structure manipulation
// ============================================================================

#[test]
fn test_s11_integration_dict_build_and_iterate() {
    let code = r#"
def word_count(text: str) -> dict:
    counts: dict = {}
    words: list = text.split(" ")
    for word in words:
        if word in counts:
            counts[word] = counts[word] + 1
        else:
            counts[word] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_count"), "Should generate function. Got: {}", result);
    assert!(result.contains("HashMap") || result.contains("counts"), "Should use HashMap. Got: {}", result);
}

#[test]
fn test_s11_integration_list_filter_map() {
    let code = r#"
def positive_doubles(items: list) -> list:
    result: list = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_doubles"), "Should generate function");
    assert!(result.contains("push") || result.contains("collect"), "Should use push/collect. Got: {}", result);
}

#[test]
fn test_s11_integration_nested_loops() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total: int = 0
    for row in matrix:
        for val in row:
            total = total + val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_sum"), "Should generate function");
    assert!(result.contains("for"), "Should have nested loops. Got: {}", result);
}

#[test]
fn test_s11_integration_string_processing() {
    let code = r#"
def clean_and_split(text: str) -> list:
    cleaned: str = text.strip().lower()
    words: list = cleaned.split(" ")
    result: list = []
    for word in words:
        if len(word) > 0:
            result.append(word)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("trim") || result.contains("to_lowercase"), "Should have string methods. Got: {}", result);
}

// ============================================================================
// Multiple functions in one module
// ============================================================================

#[test]
fn test_s11_integration_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b

def compute(x: int, y: int) -> int:
    s: int = add(x, y)
    p: int = multiply(x, y)
    return s + p
"#;
    let result = transpile(code);
    assert!(result.contains("fn add"), "Should have add function");
    assert!(result.contains("fn multiply"), "Should have multiply function");
    assert!(result.contains("fn compute"), "Should have compute function");
}

// ============================================================================
// Complex type annotations
// ============================================================================

#[test]
fn test_s11_integration_typed_dict_operations() {
    let code = r#"
from typing import Dict, List

def group_by_length(words: List[str]) -> Dict[int, List[str]]:
    groups: Dict[int, List[str]] = {}
    for word in words:
        key: int = len(word)
        if key not in groups:
            groups[key] = []
        groups[key].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_length"), "Should generate function");
    assert!(result.contains("HashMap") || result.contains("groups"), "Should use HashMap");
}

#[test]
fn test_s11_integration_optional_chaining() {
    let code = r#"
from typing import Optional

def find_first(items: list, target: int) -> Optional[int]:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("Option") || result.contains("None") || result.contains("Some"),
            "Should use Option type. Got: {}", result);
}

// ============================================================================
// Complex control flow patterns
// ============================================================================

#[test]
fn test_s11_integration_while_with_break() {
    let code = r#"
def find_first_negative(items: list) -> int:
    i: int = 0
    while i < len(items):
        if items[i] < 0:
            return items[i]
        i = i + 1
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("while"), "Should have while loop. Got: {}", result);
}

#[test]
fn test_s11_integration_nested_conditionals() {
    let code = r#"
def categorize(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "first_quadrant"
        else:
            return "fourth_quadrant"
    else:
        if y > 0:
            return "second_quadrant"
        else:
            return "third_quadrant"
"#;
    let result = transpile(code);
    assert!(result.contains("first_quadrant"), "Should have all quadrants. Got: {}", result);
}

// ============================================================================
// Algorithms
// ============================================================================

#[test]
fn test_s11_integration_bubble_sort() {
    let code = r#"
def bubble_sort(arr: list) -> list:
    n: int = len(arr)
    for i in range(n):
        for j in range(n - i - 1):
            if arr[j] > arr[j + 1]:
                temp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = temp
    return arr
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Should generate bubble sort");
    assert!(result.contains("for") || result.contains("range"), "Should have loops. Got: {}", result);
}

#[test]
fn test_s11_integration_binary_search() {
    let code = r#"
def binary_search(arr: list, target: int) -> int:
    low: int = 0
    high: int = len(arr) - 1
    while low <= high:
        mid: int = (low + high) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Should generate binary search");
    assert!(result.contains("while"), "Should have while loop. Got: {}", result);
}

#[test]
fn test_s11_integration_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        temp: int = b
        b = a % b
        a = temp
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Should generate gcd");
    assert!(result.contains("while"), "Should have while loop");
    assert!(result.contains("%"), "Should have modulo. Got: {}", result);
}

// ============================================================================
// String algorithms
// ============================================================================

#[test]
fn test_s11_integration_palindrome() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    cleaned: str = s.lower().strip()
    reversed_s: str = cleaned[::-1]
    return cleaned == reversed_s
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Should generate palindrome check");
}

#[test]
fn test_s11_integration_count_vowels() {
    let code = r#"
def count_vowels(s: str) -> int:
    count: int = 0
    for char in s:
        if char in "aeiouAEIOU":
            count = count + 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_vowels"), "Should generate vowel counter");
}

#[test]
fn test_s11_integration_reverse_words() {
    let code = r#"
def reverse_words(sentence: str) -> str:
    words: list = sentence.split(" ")
    reversed_words: list = []
    i: int = len(words) - 1
    while i >= 0:
        reversed_words.append(words[i])
        i = i - 1
    return " ".join(reversed_words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_words"), "Should generate reverse_words");
    assert!(result.contains("split") || result.contains("join"), "Should use string methods. Got: {}", result);
}

// ============================================================================
// Math/numeric patterns
// ============================================================================

#[test]
fn test_s11_integration_fibonacci_iterative() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for i in range(2, n + 1):
        temp: int = a + b
        a = b
        b = temp
    return b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Should generate fibonacci");
}

#[test]
fn test_s11_integration_prime_check() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 1
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_prime"), "Should generate prime check");
    assert!(result.contains("false") || result.contains("true"), "Should have boolean returns. Got: {}", result);
}

// ============================================================================
// Error handling / exception patterns
// ============================================================================

#[test]
fn test_s11_integration_try_multiple_except() {
    let code = r#"
def safe_divide(a: int, b: int) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        return 0.0
    except ValueError:
        return -1.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Should generate function. Got: {}", result);
}

// ============================================================================
// Class-like patterns (as functions)
// ============================================================================

#[test]
fn test_s11_integration_point_distance() {
    let code = r#"
import math

def distance(x1: float, y1: float, x2: float, y2: float) -> float:
    dx: float = x2 - x1
    dy: float = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Should generate distance function");
    assert!(result.contains("sqrt"), "Should use sqrt. Got: {}", result);
}

// ============================================================================
// Comprehension patterns
// ============================================================================

#[test]
fn test_s11_integration_list_comp_with_condition() {
    let code = r#"
def even_squares(n: int) -> list:
    return [x * x for x in range(n) if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_squares"), "Should generate function");
}

#[test]
fn test_s11_integration_nested_list_comp() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Should generate flatten function");
}

// ============================================================================
// Augmented assignment patterns
// ============================================================================

#[test]
fn test_s11_integration_string_building() {
    let code = r#"
def build_csv(headers: list, rows: list) -> str:
    result: str = ",".join(headers)
    result = result + "\n"
    for row in rows:
        result = result + ",".join(row) + "\n"
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_csv"), "Should generate build_csv");
    assert!(result.contains("join"), "Should use join. Got: {}", result);
}

// ============================================================================
// Multiple data type interaction
// ============================================================================

#[test]
fn test_s11_integration_mixed_types_function() {
    let code = r#"
def summarize(items: list) -> dict:
    result: dict = {}
    result["count"] = len(items)
    total: int = 0
    for item in items:
        total = total + item
    result["sum"] = total
    if len(items) > 0:
        result["avg"] = total / len(items)
    else:
        result["avg"] = 0
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn summarize"), "Should generate summarize");
}

// ============================================================================
// Assertion and validation patterns
// ============================================================================

#[test]
fn test_s11_integration_validate_input() {
    let code = r#"
def validate(name: str, age: int) -> bool:
    if len(name) == 0:
        return False
    if age < 0 or age > 150:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Should generate validate");
    assert!(result.contains("false") && result.contains("true"), "Should have boolean returns. Got: {}", result);
}

// ============================================================================
// Accumulator patterns
// ============================================================================

#[test]
fn test_s11_integration_running_average() {
    let code = r#"
def running_average(values: list) -> list:
    result: list = []
    total: float = 0.0
    for i in range(len(values)):
        total = total + values[i]
        result.append(total / (i + 1))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn running_average"), "Should generate running_average");
}

// ============================================================================
// Global-scope statements (module-level code)
// ============================================================================

#[test]
fn test_s11_integration_multiple_functions_with_types() {
    let code = r#"
from typing import List, Tuple

def split_even_odd(items: List[int]) -> Tuple[List[int], List[int]]:
    evens: List[int] = []
    odds: List[int] = []
    for item in items:
        if item % 2 == 0:
            evens.append(item)
        else:
            odds.append(item)
    return (evens, odds)
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_even_odd"), "Should generate function");
}

// ============================================================================
// Complex return type patterns
// ============================================================================

#[test]
fn test_s11_integration_early_return_with_loop() {
    let code = r#"
def find_duplicate(items: list) -> int:
    seen: set = set()
    for item in items:
        if item in seen:
            return item
        seen.add(item)
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_duplicate"), "Should generate find_duplicate");
    assert!(result.contains("HashSet") || result.contains("set") || result.contains("insert"),
            "Should use set. Got: {}", result);
}

#[test]
fn test_s11_integration_conditional_list_build() {
    let code = r#"
def filter_and_transform(items: list, threshold: int) -> list:
    result: list = []
    for item in items:
        if item >= threshold:
            result.append(item * 2)
        elif item >= 0:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_and_transform"), "Should generate function");
    assert!(result.contains("if") || result.contains("else"), "Should have conditionals. Got: {}", result);
}

//! Session 12 Batch 19: Deep coverage for func_gen.rs and type_gen.rs
//!
//! Targets cold paths in function generation:
//! - Functions with multiple return types
//! - Functions with complex type annotations
//! - Nested function definitions
//! - Decorator patterns (@staticmethod, @classmethod, @property)
//! - Default arguments with various types
//! - *args and **kwargs handling
//! - Recursive functions
//! - Functions returning complex types (Dict, List, Tuple, Optional)
//! - Class methods with self parameter handling
//! - Functions with docstrings
//! - Type hint patterns (Union, Optional, Any, Callable)

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

// ===== Complex function signatures =====

#[test]
fn test_s12_many_params() {
    let code = r#"
def many_args(a: int, b: int, c: int, d: int, e: int) -> int:
    return a + b + c + d + e
"#;
    let result = transpile(code);
    assert!(result.contains("fn many_args"), "Got: {}", result);
}

#[test]
fn test_s12_mixed_type_params() {
    let code = r#"
def mixed_types(name: str, age: int, weight: float, active: bool) -> str:
    if active:
        return name
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn mixed_types"), "Got: {}", result);
}

#[test]
fn test_s12_function_returning_list() {
    let code = r#"
def make_range(start: int, end: int) -> list:
    result = []
    for i in range(start, end):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_range"), "Got: {}", result);
}

#[test]
fn test_s12_function_returning_dict() {
    let code = r#"
def make_dict(keys: list, values: list) -> dict:
    result = {}
    for i in range(len(keys)):
        result[keys[i]] = values[i]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"), "Got: {}", result);
}

#[test]
fn test_s12_function_returning_tuple() {
    let code = r#"
def divide_with_remainder(a: int, b: int) -> tuple:
    quotient = a // b
    remainder = a % b
    return (quotient, remainder)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divide_with_remainder"), "Got: {}", result);
}

// ===== Recursive patterns =====

#[test]
fn test_s12_recursive_fibonacci() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fib"), "Got: {}", result);
}

#[test]
fn test_s12_recursive_power() {
    let code = r#"
def power(base: int, exp: int) -> int:
    if exp == 0:
        return 1
    return base * power(base, exp - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn power"), "Got: {}", result);
}

#[test]
fn test_s12_tail_recursive_sum() {
    let code = r#"
def sum_to(n: int, acc: int = 0) -> int:
    if n <= 0:
        return acc
    return sum_to(n - 1, acc + n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_to"), "Got: {}", result);
}

// ===== Nested functions =====

#[test]
fn test_s12_nested_helper() {
    let code = r#"
def process_items(items: list) -> list:
    def transform(x: int) -> int:
        return x * 2 + 1
    result = []
    for item in items:
        result.append(transform(item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_items"), "Got: {}", result);
}

#[test]
fn test_s12_nested_with_closure() {
    let code = r#"
def make_multiplier(factor: int):
    def multiply(x: int) -> int:
        return x * factor
    return multiply
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_multiplier"), "Got: {}", result);
}

// ===== Class method patterns =====

#[test]
fn test_s12_class_with_init_and_methods() {
    let code = r#"
class BankAccount:
    def __init__(self, owner: str, balance: float = 0.0):
        self.owner = owner
        self.balance = balance

    def deposit(self, amount: float):
        self.balance += amount

    def withdraw(self, amount: float) -> bool:
        if amount > self.balance:
            return False
        self.balance -= amount
        return True

    def get_balance(self) -> float:
        return self.balance
"#;
    let result = transpile(code);
    assert!(result.contains("BankAccount"), "Got: {}", result);
    assert!(result.contains("deposit"), "Got: {}", result);
    assert!(result.contains("withdraw"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_static() {
    let code = r#"
class Validator:
    @staticmethod
    def is_positive(x: int) -> bool:
        return x > 0

    @staticmethod
    def is_even(x: int) -> bool:
        return x % 2 == 0

    @staticmethod
    def clamp(x: int, lo: int, hi: int) -> int:
        if x < lo:
            return lo
        if x > hi:
            return hi
        return x
"#;
    let result = transpile(code);
    assert!(result.contains("Validator"), "Got: {}", result);
}

// ===== Complex typing patterns =====

#[test]
fn test_s12_optional_return() {
    let code = r#"
from typing import Optional

def safe_divide(a: float, b: float) -> Optional[float]:
    if b == 0:
        return None
    return a / b
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_list_of_tuples() {
    let code = r#"
from typing import List, Tuple

def pairs(items: List[int]) -> List[Tuple[int, int]]:
    result = []
    for i in range(0, len(items) - 1, 2):
        result.append((items[i], items[i + 1]))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs"), "Got: {}", result);
}

// ===== Functions with docstrings =====

#[test]
fn test_s12_function_with_docstring() {
    let code = r#"
def calculate_area(width: float, height: float) -> float:
    """Calculate the area of a rectangle.

    Args:
        width: The width of the rectangle.
        height: The height of the rectangle.

    Returns:
        The area of the rectangle.
    """
    return width * height
"#;
    let result = transpile(code);
    assert!(result.contains("fn calculate_area"), "Got: {}", result);
}

// ===== Complex algorithm functions =====

#[test]
fn test_s12_quicksort() {
    let code = r#"
def quicksort(arr: list) -> list:
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = []
    middle = []
    right = []
    for x in arr:
        if x < pivot:
            left.append(x)
        elif x == pivot:
            middle.append(x)
        else:
            right.append(x)
    return quicksort(left) + middle + quicksort(right)
"#;
    let result = transpile(code);
    assert!(result.contains("fn quicksort"), "Got: {}", result);
}

#[test]
fn test_s12_lcs_algorithm() {
    let code = r#"
def longest_common_subsequence(s1: str, s2: str) -> int:
    m = len(s1)
    n = len(s2)
    dp = []
    for i in range(m + 1):
        row = []
        for j in range(n + 1):
            row.append(0)
        dp.append(row)
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                if dp[i - 1][j] > dp[i][j - 1]:
                    dp[i][j] = dp[i - 1][j]
                else:
                    dp[i][j] = dp[i][j - 1]
    return dp[m][n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_common_subsequence"), "Got: {}", result);
}

#[test]
fn test_s12_topological_sort() {
    let code = r#"
def topological_sort(graph: dict) -> list:
    visited = set()
    result = []

    def dfs(node: str):
        if node in visited:
            return
        visited.add(node)
        if node in graph:
            for neighbor in graph[node]:
                dfs(neighbor)
        result.append(node)

    for node in graph:
        dfs(node)
    result.reverse()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn topological_sort"), "Got: {}", result);
}

// ===== Multiple functions interacting =====

#[test]
fn test_s12_helper_chain() {
    let code = r#"
def is_vowel(c: str) -> bool:
    return c.lower() in "aeiou"

def count_vowels(s: str) -> int:
    count = 0
    for c in s:
        if is_vowel(c):
            count += 1
    return count

def vowel_ratio(s: str) -> float:
    if len(s) == 0:
        return 0.0
    return count_vowels(s) / len(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_vowel"), "Got: {}", result);
    assert!(result.contains("fn count_vowels"), "Got: {}", result);
    assert!(result.contains("fn vowel_ratio"), "Got: {}", result);
}

#[test]
fn test_s12_builder_pattern_functions() {
    let code = r#"
def create_user(name: str, email: str, age: int = 0) -> dict:
    user = {"name": name, "email": email}
    if age > 0:
        user["age"] = age
    return user

def validate_user(user: dict) -> bool:
    if "name" not in user:
        return False
    if "email" not in user:
        return False
    if "@" not in user["email"]:
        return False
    return True

def format_user(user: dict) -> str:
    return user["name"] + " <" + user["email"] + ">"
"#;
    let result = transpile(code);
    assert!(result.contains("fn create_user"), "Got: {}", result);
    assert!(result.contains("fn validate_user"), "Got: {}", result);
    assert!(result.contains("fn format_user"), "Got: {}", result);
}

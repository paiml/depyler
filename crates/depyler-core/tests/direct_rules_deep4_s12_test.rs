//! Session 12 Batch 11: More deep coverage for direct_rules_convert.rs
//!
//! Targets remaining cold paths:
//! - Complex method chains on strings
//! - Dict/list/set comprehension variants
//! - Multiple assignment patterns
//! - Augmented assignment operators
//! - Complex boolean expressions
//! - Nested function calls
//! - Lambda expressions
//! - F-string formatting
//! - Complex class hierarchies
//! - Multiple inheritance
//! - Property patterns
//! - Class methods
//! - Static methods
//! - Abstract classes
//! - Decorator patterns
//! - Context managers
//! - Generator expressions

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

// ===== Complex boolean expressions =====

#[test]
fn test_s12_complex_and_or() {
    let code = r#"
def validate(x: int, y: int, z: int) -> bool:
    return (x > 0 and y > 0) or z == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s12_chained_comparison_three() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 <= x <= 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"), "Got: {}", result);
}

#[test]
fn test_s12_not_in_operator() {
    let code = r#"
def is_excluded(item: str, exclusions: list) -> bool:
    return item not in exclusions
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_excluded"), "Got: {}", result);
}

#[test]
fn test_s12_is_none_check() {
    let code = r#"
def is_missing(value: object) -> bool:
    return value is None
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_missing"), "Got: {}", result);
}

#[test]
fn test_s12_is_not_none_check() {
    let code = r#"
def has_value(value: object) -> bool:
    return value is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_value"), "Got: {}", result);
}

// ===== Augmented assignment operators =====

#[test]
fn test_s12_augmented_mul_assign() {
    let code = r#"
def double_all(items: list) -> int:
    result = 1
    for item in items:
        result *= item
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_div_assign() {
    let code = r#"
def halve(x: float) -> float:
    x /= 2.0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_mod_assign() {
    let code = r#"
def reduce_mod(x: int, m: int) -> int:
    x %= m
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn reduce_mod"), "Got: {}", result);
}

#[test]
fn test_s12_augmented_floor_div_assign() {
    let code = r#"
def floor_half(x: int) -> int:
    x //= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn floor_half"), "Got: {}", result);
}

// ===== F-string formatting =====

#[test]
fn test_s12_fstring_basic() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
    assert!(result.contains("format!"), "Expected format!, got: {}", result);
}

#[test]
fn test_s12_fstring_expression() {
    let code = r#"
def show_result(x: int) -> str:
    return f"Result: {x * 2}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_result"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_multiple_vars() {
    let code = r#"
def coords(x: int, y: int) -> str:
    return f"({x}, {y})"
"#;
    let result = transpile(code);
    assert!(result.contains("fn coords"), "Got: {}", result);
}

// ===== Lambda expressions =====

#[test]
fn test_s12_lambda_in_sort() {
    let code = r#"
def sort_by_second(pairs: list) -> list:
    pairs.sort(key=lambda x: x[1])
    return pairs
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_second"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_simple() {
    let code = r#"
def apply(f, x: int) -> int:
    return f(x)

def double(x: int) -> int:
    f = lambda x: x * 2
    return f(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn double"), "Got: {}", result);
}

// ===== Multiple assignment =====

#[test]
fn test_s12_multi_assign() {
    let code = r#"
def init() -> int:
    a = b = c = 0
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn init"), "Got: {}", result);
}

#[test]
fn test_s12_swap_values() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_inheritance() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return ""

class Dog(Animal):
    def speak(self) -> str:
        return "Woof!"

class Cat(Animal):
    def speak(self) -> str:
        return "Meow!"
"#;
    let result = transpile(code);
    assert!(result.contains("Animal"), "Got: {}", result);
    assert!(result.contains("Dog"), "Got: {}", result);
    assert!(result.contains("Cat"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_class_method() {
    let code = r#"
class Temperature:
    def __init__(self, celsius: float):
        self.celsius = celsius

    @classmethod
    def from_fahrenheit(cls, f: float):
        return cls((f - 32) * 5 / 9)
"#;
    let result = transpile(code);
    assert!(result.contains("Temperature"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_static_method() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def multiply(a: int, b: int) -> int:
        return a * b
"#;
    let result = transpile(code);
    assert!(result.contains("MathUtils"), "Got: {}", result);
}

// ===== Nested data structures =====

#[test]
fn test_s12_nested_list_of_lists() {
    let code = r#"
def transpose(matrix: list) -> list:
    rows = len(matrix)
    cols = len(matrix[0])
    result = []
    for j in range(cols):
        row = []
        for i in range(rows):
            row.append(matrix[i][j])
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transpose"), "Got: {}", result);
}

#[test]
fn test_s12_dict_of_lists() {
    let code = r#"
def group_by_length(words: list) -> dict:
    groups = {}
    for word in words:
        length = len(word)
        if length not in groups:
            groups[length] = []
        groups[length].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_length"), "Got: {}", result);
}

// ===== String operations =====

#[test]
fn test_s12_str_join_list() {
    let code = r#"
def join_words(words: list) -> str:
    return ", ".join(words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_words"), "Got: {}", result);
}

#[test]
fn test_s12_str_split_maxsplit() {
    let code = r#"
def first_word(s: str) -> str:
    parts = s.split(" ", 1)
    return parts[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_word"), "Got: {}", result);
}

#[test]
fn test_s12_str_rsplit() {
    let code = r#"
def get_extension(path: str) -> str:
    parts = path.rsplit(".", 1)
    return parts[-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_extension"), "Got: {}", result);
}

// ===== Builtin functions =====

#[test]
fn test_s12_sorted_builtin() {
    let code = r#"
def sort_copy(items: list) -> list:
    return sorted(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_copy"), "Got: {}", result);
}

#[test]
fn test_s12_reversed_builtin() {
    let code = r#"
def reverse_copy(items: list) -> list:
    return list(reversed(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn reverse_copy"), "Got: {}", result);
}

#[test]
fn test_s12_min_max_builtin() {
    let code = r#"
def get_range(items: list) -> tuple:
    return (min(items), max(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_range"), "Got: {}", result);
}

#[test]
fn test_s12_sum_builtin() {
    let code = r#"
def total(items: list) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

#[test]
fn test_s12_any_all_builtins() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)

def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_positive"), "Got: {}", result);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

#[test]
fn test_s12_abs_builtin() {
    let code = r#"
def magnitude(x: int) -> int:
    return abs(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn magnitude"), "Got: {}", result);
}

#[test]
fn test_s12_round_builtin() {
    let code = r#"
def round_val(x: float) -> int:
    return round(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_val"), "Got: {}", result);
}

#[test]
fn test_s12_map_filter_builtins() {
    let code = r#"
def double_positives(items: list) -> list:
    filtered = list(filter(lambda x: x > 0, items))
    return list(map(lambda x: x * 2, filtered))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_positives"), "Got: {}", result);
}

// ===== Complex slice patterns =====

#[test]
fn test_s12_step_slice() {
    let code = r#"
def every_other(items: list) -> list:
    return items[::2]
"#;
    let result = transpile(code);
    assert!(result.contains("fn every_other"), "Got: {}", result);
}

#[test]
fn test_s12_reverse_slice() {
    let code = r#"
def reversed_list(items: list) -> list:
    return items[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn reversed_list"), "Got: {}", result);
}

// ===== Complex algorithm patterns =====

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
fn test_s12_is_palindrome() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    clean = s.lower().replace(" ", "")
    return clean == clean[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Got: {}", result);
}

#[test]
fn test_s12_fizzbuzz() {
    let code = r#"
def fizzbuzz(n: int) -> list:
    result = []
    for i in range(1, n + 1):
        if i % 15 == 0:
            result.append("FizzBuzz")
        elif i % 3 == 0:
            result.append("Fizz")
        elif i % 5 == 0:
            result.append("Buzz")
        else:
            result.append(str(i))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn fizzbuzz"), "Got: {}", result);
}

#[test]
fn test_s12_prime_check() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_prime"), "Got: {}", result);
}

#[test]
fn test_s12_count_vowels() {
    let code = r#"
def count_vowels(s: str) -> int:
    count = 0
    for c in s.lower():
        if c in "aeiou":
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_vowels"), "Got: {}", result);
}

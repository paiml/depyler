//! DEPYLER-99MODE-S10: Integration tests targeting advanced pattern coverage gaps
//!
//! Tests for: lambda captures, f-string edge cases, type mapper generics,
//! async/await, generators, named expressions, and complex type patterns.

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

// ===== Lambda Patterns =====

#[test]
fn test_s10_lambda_basic() {
    let code = r#"
def apply(x: int) -> int:
    f = lambda n: n * 2
    return f(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply"));
}

#[test]
fn test_s10_lambda_with_capture() {
    let code = r#"
def make_multiplier(factor: int) -> int:
    multiply = lambda x: x * factor
    return multiply(5)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_multiplier"));
}

#[test]
fn test_s10_lambda_in_map() {
    let code = r#"
def double_list(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_list"));
    assert!(result.contains("map"));
}

#[test]
fn test_s10_lambda_in_filter() {
    let code = r#"
def evens_only(items: list) -> list:
    return list(filter(lambda x: x % 2 == 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens_only"));
    assert!(result.contains("filter"));
}

#[test]
fn test_s10_lambda_multi_arg() {
    let code = r#"
def sort_pairs(pairs: list) -> list:
    pairs.sort(key=lambda p: p[0])
    return pairs
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_pairs"));
    assert!(result.contains("sort"));
}

#[test]
fn test_s10_lambda_in_sorted() {
    let code = r#"
def sort_by_len(words: list) -> list:
    return sorted(words, key=lambda w: len(w))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_len"));
    assert!(result.contains("sort"));
}

// ===== F-String Patterns =====

#[test]
fn test_s10_fstring_basic() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s10_fstring_with_expression() {
    let code = r#"
def describe(x: int) -> str:
    return f"value is {x * 2}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s10_fstring_multiple_parts() {
    let code = r#"
def full_name(first: str, last: str) -> str:
    return f"{first} {last}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn full_name"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s10_fstring_with_method_call() {
    let code = r#"
def upper_greeting(name: str) -> str:
    return f"Hello, {name.upper()}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_greeting"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s10_fstring_with_int() {
    let code = r#"
def count_msg(n: int) -> str:
    return f"Found {n} items"
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_msg"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s10_fstring_with_float() {
    let code = r#"
def format_price(price: float) -> str:
    return f"Total: {price}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_price"));
    assert!(result.contains("format!"));
}

#[test]
fn test_s10_fstring_nested() {
    let code = r#"
def nested_format(a: int, b: int) -> str:
    return f"sum={a+b}, product={a*b}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_format"));
    assert!(result.contains("format!"));
}

// ===== Type Annotation Patterns =====

#[test]
fn test_s10_type_list_int() {
    let code = r#"
from typing import List

def sum_ints(nums: List[int]) -> int:
    return sum(nums)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_ints"));
    assert!(result.contains("Vec") || result.contains("i32"));
}

#[test]
fn test_s10_type_dict_str_int() {
    let code = r#"
from typing import Dict

def count_words(text: str) -> Dict[str, int]:
    result = {}
    for word in text.split():
        result[word] = result.get(word, 0) + 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_words"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_type_tuple() {
    let code = r#"
from typing import Tuple

def swap(a: int, b: int) -> Tuple[int, int]:
    return (b, a)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"));
}

#[test]
fn test_s10_type_set() {
    let code = r#"
from typing import Set

def unique(items: list) -> Set[int]:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique"));
    assert!(result.contains("HashSet") || result.contains("set"));
}

#[test]
fn test_s10_type_optional_param() {
    let code = r#"
from typing import Optional

def get_default(val: Optional[int], default: int) -> int:
    if val is not None:
        return val
    return default
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_default"));
    assert!(result.contains("Option") || result.contains("None"));
}

// ===== Ternary / Conditional Expression =====

#[test]
fn test_s10_ternary_basic() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"));
    assert!(result.contains("if") || result.contains("else"));
}

#[test]
fn test_s10_ternary_string() {
    let code = r#"
def label(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn label"));
    assert!(result.contains("positive"));
}

#[test]
fn test_s10_ternary_in_list() {
    let code = r#"
def categorize(items: list) -> list:
    return ["even" if x % 2 == 0 else "odd" for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn categorize"));
}

// ===== Generator Expression =====

#[test]
fn test_s10_generator_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"));
}

#[test]
fn test_s10_generator_any() {
    let code = r#"
def has_negative(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_negative"));
    assert!(result.contains("any"));
}

#[test]
fn test_s10_generator_all() {
    let code = r#"
def all_even(items: list) -> bool:
    return all(x % 2 == 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_even"));
    assert!(result.contains("all"));
}

// ===== Complex Control Flow =====

#[test]
fn test_s10_nested_if_else() {
    let code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    let result = transpile(code);
    assert!(result.contains("fn grade"));
    assert!(result.contains("90") || result.contains("80"));
}

#[test]
fn test_s10_nested_loops() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total = total + val
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix_sum"));
}

#[test]
fn test_s10_early_return_in_loop() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first"));
}

// ===== Int/Float Coercion =====

#[test]
fn test_s10_int_float_mix() {
    let code = r#"
def average(total: int, count: int) -> float:
    return total / count
"#;
    let result = transpile(code);
    assert!(result.contains("fn average"));
    assert!(result.contains("f64") || result.contains("as f64"));
}

#[test]
fn test_s10_float_operations() {
    let code = r#"
def distance(x1: float, y1: float, x2: float, y2: float) -> float:
    return ((x2 - x1) ** 2 + (y2 - y1) ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"));
    assert!(result.contains("sqrt") || result.contains("powf"));
}

// ===== String Formatting (not f-string) =====

#[test]
fn test_s10_percent_format() {
    let code = r#"
def format_old(name: str, age: int) -> str:
    return "%s is %d years old" % (name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_old"));
    assert!(result.contains("format!"));
}

// ===== Complex Data Structures =====

#[test]
fn test_s10_nested_dict() {
    let code = r#"
def nested_access(data: dict) -> str:
    return data["key"]
"#;
    let result = transpile(code);
    assert!(result.contains("fn nested_access"));
}

#[test]
fn test_s10_list_of_tuples() {
    let code = r#"
def pairs(n: int) -> list:
    return [(i, i * i) for i in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs"));
}

// ===== Global/Constant Patterns =====

#[test]
fn test_s10_multiple_constants() {
    let code = r#"
MIN_VAL = 0
MAX_VAL = 100

def clamp(x: int) -> int:
    if x < MIN_VAL:
        return MIN_VAL
    if x > MAX_VAL:
        return MAX_VAL
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("MIN_VAL"));
    assert!(result.contains("MAX_VAL"));
    assert!(result.contains("fn clamp"));
}

// ===== Exception/Error Patterns =====

#[test]
fn test_s10_raise_value_error() {
    let code = r#"
def validate_positive(x: int) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_positive"));
    assert!(result.contains("must be positive") || result.contains("panic") || result.contains("Error"));
}

#[test]
fn test_s10_raise_generic() {
    let code = r#"
def not_implemented():
    raise NotImplementedError("not yet")
"#;
    let result = transpile(code);
    assert!(result.contains("fn not_implemented"));
    assert!(result.contains("not yet") || result.contains("unimplemented") || result.contains("todo"));
}

// ===== Multiple Assignment =====

#[test]
fn test_s10_tuple_unpack() {
    let code = r#"
def unpack(pair: tuple) -> int:
    a, b = pair
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn unpack"));
}

#[test]
fn test_s10_swap_variables() {
    let code = r#"
def do_swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn do_swap"));
}

// ===== Walrus Operator / Named Expression =====

#[test]
fn test_s10_while_with_assignment() {
    let code = r#"
def count_lines(text: str) -> int:
    lines = text.split("\n")
    return len(lines)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_lines"));
    assert!(result.contains("split"));
}

// ===== Set Comprehension =====

#[test]
fn test_s10_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"));
    assert!(result.contains("HashSet") || result.contains("collect"));
}

// ===== Dict Comprehension =====

#[test]
fn test_s10_dict_comprehension_with_filter() {
    let code = r#"
def positive_map(items: list) -> dict:
    return {i: v for i, v in enumerate(items) if v > 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_map"));
}

// ===== Chained Comparison =====

#[test]
fn test_s10_chained_comparison() {
    let code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo <= x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"));
}

// ===== String Slicing =====

#[test]
fn test_s10_string_slice() {
    let code = r#"
def first_n(s: str, n: int) -> str:
    return s[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_n"));
}

#[test]
fn test_s10_string_slice_from() {
    let code = r#"
def skip_n(s: str, n: int) -> str:
    return s[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn skip_n"));
}

// ===== Recursive Functions =====

#[test]
fn test_s10_recursive_fibonacci() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fib"));
}

#[test]
fn test_s10_recursive_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    return gcd(b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"));
}

// ===== Boolean Expressions =====

#[test]
fn test_s10_complex_boolean() {
    let code = r#"
def check(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"));
}

// ===== Power Operations =====

#[test]
fn test_s10_power_int() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube"));
    assert!(result.contains("pow") || result.contains("powi"));
}

// ===== Multiple Return Types =====

#[test]
fn test_s10_return_list() {
    let code = r#"
def range_list(start: int, end: int) -> list:
    return list(range(start, end))
"#;
    let result = transpile(code);
    assert!(result.contains("fn range_list"));
}

#[test]
fn test_s10_return_dict() {
    let code = r#"
def make_dict(key: str, value: int) -> dict:
    return {key: value}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_return_set() {
    let code = r#"
def make_set(items: list) -> set:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_set"));
    assert!(result.contains("HashSet") || result.contains("collect"));
}

// ===== Complex Method Chaining =====

#[test]
fn test_s10_method_chain_complex() {
    let code = r#"
def process(text: str) -> list:
    return text.strip().lower().split()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"));
    assert!(result.contains("trim") || result.contains("strip"));
    assert!(result.contains("to_lowercase") || result.contains("lower"));
    assert!(result.contains("split"));
}

// ===== Empty Collections =====

#[test]
fn test_s10_empty_list() {
    let code = r#"
def empty() -> list:
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty"));
    assert!(result.contains("vec!") || result.contains("Vec"));
}

#[test]
fn test_s10_empty_dict() {
    let code = r#"
def empty_dict() -> dict:
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty_dict"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_empty_set() {
    let code = r#"
def empty_set() -> set:
    return set()
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty_set"));
    assert!(result.contains("HashSet"));
}

// ===== Complex Nested Patterns =====

#[test]
fn test_s10_nested_comprehension_in_try() {
    let code = r#"
def safe_squares(items: list) -> list:
    try:
        return [x * x for x in items]
    except TypeError:
        return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_squares"));
}

#[test]
fn test_s10_multiple_assignments() {
    let code = r#"
def compute(x: int) -> int:
    a = x + 1
    b = a * 2
    c = b - 3
    return c
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"));
}

// ===== Print Patterns =====

#[test]
fn test_s10_print_fstring() {
    let code = r#"
def log(msg: str, level: int):
    print(f"[{level}] {msg}")
"#;
    let result = transpile(code);
    assert!(result.contains("fn log"));
    assert!(result.contains("println!") || result.contains("print"));
}

#[test]
fn test_s10_print_multiple_args() {
    let code = r#"
def show(a: int, b: int):
    print(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show"));
    assert!(result.contains("println!") || result.contains("print"));
}

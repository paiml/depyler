//! Session 12: Targeted tests for expr_gen.rs uncovered paths
//!
//! Targets:
//! - Complex expression patterns
//! - Lambda expressions
//! - Slice operations
//! - Set/frozenset literals
//! - Complex subscript patterns
//! - Generator expressions in various builtins

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

// ===== Lambda expressions =====

#[test]
fn test_s12_lambda_basic() {
    let code = r#"
def apply(f, x: int) -> int:
    return f(x)

def double(n: int) -> int:
    f = lambda x: x * 2
    return f(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn double"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_in_map() {
    let code = r#"
def double_all(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_all"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_in_filter() {
    let code = r#"
def positives(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn positives"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_in_sorted() {
    let code = r#"
def sort_by_abs(items: list) -> list:
    return sorted(items, key=lambda x: abs(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_abs"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_multi_arg() {
    let code = r#"
def make_adder() -> int:
    add = lambda a, b: a + b
    return add(3, 4)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"), "Got: {}", result);
}

// ===== Slice operations =====

#[test]
fn test_s12_slice_start_end() {
    let code = r#"
def mid(items: list) -> list:
    return items[1:3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn mid"), "Got: {}", result);
}

#[test]
fn test_s12_slice_from_start() {
    let code = r#"
def first_n(items: list, n: int) -> list:
    return items[:n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_n"), "Got: {}", result);
}

#[test]
fn test_s12_slice_to_end() {
    let code = r#"
def after_n(items: list, n: int) -> list:
    return items[n:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn after_n"), "Got: {}", result);
}

#[test]
fn test_s12_slice_negative() {
    let code = r#"
def last_two(items: list) -> list:
    return items[-2:]
"#;
    let result = transpile(code);
    assert!(result.contains("fn last_two"), "Got: {}", result);
}

#[test]
fn test_s12_string_slice() {
    let code = r#"
def first_char(s: str) -> str:
    return s[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_char"), "Got: {}", result);
}

#[test]
fn test_s12_string_slice_range() {
    let code = r#"
def substring(s: str, start: int, end: int) -> str:
    return s[start:end]
"#;
    let result = transpile(code);
    assert!(result.contains("fn substring"), "Got: {}", result);
}

// ===== Set/frozenset literals =====

#[test]
fn test_s12_set_literal() {
    let code = r#"
def make_set() -> set:
    return {1, 2, 3}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_set"), "Got: {}", result);
}

#[test]
fn test_s12_set_literal_strings() {
    let code = r#"
def vowels() -> set:
    return {"a", "e", "i", "o", "u"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn vowels"), "Got: {}", result);
}

#[test]
fn test_s12_set_comprehension() {
    let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_squares"), "Got: {}", result);
}

// ===== Dict comprehension =====

#[test]
fn test_s12_dict_comprehension() {
    let code = r#"
def square_map(n: int) -> dict:
    return {i: i * i for i in range(n)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn square_map"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comprehension_with_filter() {
    let code = r#"
def even_squares(n: int) -> dict:
    return {i: i * i for i in range(n) if i % 2 == 0}
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_squares"), "Got: {}", result);
}

// ===== Complex subscript patterns =====

#[test]
fn test_s12_nested_subscript() {
    let code = r#"
def get_cell(matrix: list, i: int, j: int) -> int:
    return matrix[i][j]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_cell"), "Got: {}", result);
}

#[test]
fn test_s12_dict_subscript_string() {
    let code = r#"
def get_name(d: dict) -> str:
    return d["name"]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_name"), "Got: {}", result);
}

#[test]
fn test_s12_dict_subscript_variable() {
    let code = r#"
def get_key(d: dict, key: str) -> int:
    return d[key]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_key"), "Got: {}", result);
}

// ===== Generator expressions in builtins =====

#[test]
fn test_s12_sum_generator() {
    let code = r#"
def sum_even(n: int) -> int:
    return sum(x for x in range(n) if x % 2 == 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_even"), "Got: {}", result);
}

#[test]
fn test_s12_min_generator() {
    let code = r#"
def min_square(items: list) -> int:
    return min(x * x for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_square"), "Got: {}", result);
}

#[test]
fn test_s12_max_generator() {
    let code = r#"
def max_abs(items: list) -> int:
    return max(abs(x) for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_abs"), "Got: {}", result);
}

// ===== Nested list comprehension =====

#[test]
fn test_s12_nested_list_comp() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

// ===== Walrus operator =====

#[test]
fn test_s12_walrus_in_if() {
    let code = r#"
def process(data: list) -> int:
    if n := len(data):
        return n
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_walrus_in_while() {
    let code = r#"
def count_lines(lines: list) -> int:
    total = 0
    idx = 0
    while idx < len(lines):
        line = lines[idx]
        if len(line) > 0:
            total += 1
        idx += 1
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_lines"), "Got: {}", result);
}

// ===== Ternary in various contexts =====

#[test]
fn test_s12_ternary_in_assignment() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    result = lo if x < lo else hi if x > hi else x
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

#[test]
fn test_s12_ternary_in_return() {
    let code = r#"
def sign(x: int) -> int:
    return 1 if x > 0 else -1 if x < 0 else 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

#[test]
fn test_s12_ternary_in_call() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    return a / b if b != 0 else 0.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

// ===== Complex arithmetic =====

#[test]
fn test_s12_mixed_arithmetic() {
    let code = r#"
def compute(a: int, b: int, c: float) -> float:
    return (a + b) * c / 2.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Got: {}", result);
}

#[test]
fn test_s12_modulo_and_divmod() {
    let code = r#"
def check_even(n: int) -> bool:
    return n % 2 == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_even"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_operations() {
    let code = r#"
def set_bit(x: int, pos: int) -> int:
    return x | (1 << pos)

def clear_bit(x: int, pos: int) -> int:
    return x & ~(1 << pos)

def check_bit(x: int, pos: int) -> bool:
    return (x >> pos) & 1 == 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_bit"), "Got: {}", result);
    assert!(result.contains("fn clear_bit"), "Got: {}", result);
    assert!(result.contains("fn check_bit"), "Got: {}", result);
}

// ===== String operations =====

#[test]
fn test_s12_string_join() {
    let code = r#"
def join_words(words: list) -> str:
    return " ".join(words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_words"), "Got: {}", result);
}

#[test]
fn test_s12_string_split_maxsplit() {
    let code = r#"
def first_word(text: str) -> str:
    return text.split(" ", 1)[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_word"), "Got: {}", result);
}

#[test]
fn test_s12_string_format_method() {
    let code = r#"
def format_msg(name: str, count: int) -> str:
    return "{} has {} items".format(name, count)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_msg"), "Got: {}", result);
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
        return "Woof"
"#;
    let result = transpile(code);
    assert!(result.contains("Animal"), "Got: {}", result);
    assert!(result.contains("Dog"), "Got: {}", result);
}

#[test]
fn test_s12_class_static_method() {
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

// ===== Exception handling patterns =====

#[test]
fn test_s12_try_except_finally() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        result = a // b
    except ZeroDivisionError:
        result = 0
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_try_multiple_except() {
    let code = r#"
def safe_access(items: list, idx: int) -> int:
    try:
        return items[idx]
    except IndexError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_access"), "Got: {}", result);
}

#[test]
fn test_s12_raise_value_error() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

// ===== Complex control flow =====

#[test]
fn test_s12_for_else() {
    let code = r#"
def find_item(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_item"), "Got: {}", result);
}

#[test]
fn test_s12_nested_loops_with_break() {
    let code = r#"
def find_pair(matrix: list, target: int) -> tuple:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pair"), "Got: {}", result);
}

#[test]
fn test_s12_continue_in_loop() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    for x in items:
        if x <= 0:
            continue
        total += x
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positive"), "Got: {}", result);
}

// ===== Nested function patterns =====

#[test]
fn test_s12_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x) + 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

#[test]
fn test_s12_nested_function_capture() {
    let code = r#"
def make_adder(n: int) -> int:
    def add(x: int) -> int:
        return x + n
    return add(10)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"), "Got: {}", result);
}

// ===== Global constants =====

#[test]
fn test_s12_global_string_constant() {
    let code = r#"
VERSION = "1.0.0"

def get_version() -> str:
    return VERSION
"#;
    let result = transpile(code);
    assert!(result.contains("VERSION"), "Got: {}", result);
}

#[test]
fn test_s12_global_numeric_constant() {
    let code = r#"
MAX_SIZE = 1000

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"), "Got: {}", result);
}

#[test]
fn test_s12_multiple_global_constants() {
    let code = r#"
WIDTH = 800
HEIGHT = 600
TITLE = "App"

def get_area() -> int:
    return WIDTH * HEIGHT
"#;
    let result = transpile(code);
    assert!(result.contains("WIDTH"), "Got: {}", result);
    assert!(result.contains("HEIGHT"), "Got: {}", result);
}

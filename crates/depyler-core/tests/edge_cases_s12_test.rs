//! Session 12 Batch 16: Edge case tests for coverage gaps
//!
//! Targets unusual but valid Python patterns that exercise cold code paths:
//! - Empty function bodies (pass)
//! - Functions with only docstrings
//! - Multiple decorators
//! - Star args and kwargs
//! - Default parameter values
//! - None comparisons
//! - Boolean algebra edge cases
//! - Empty collections
//! - Single element collections
//! - Nested loops with index manipulation
//! - Complex string interpolation
//! - Recursive data structures
//! - Multiple imports
//! - Complex conditional expressions

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

// ===== Empty and minimal functions =====

#[test]
fn test_s12_empty_function_pass() {
    let code = r#"
def noop():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn noop"), "Got: {}", result);
}

#[test]
fn test_s12_function_with_docstring_only() {
    let code = r#"
def documented():
    """This function does nothing."""
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn documented"), "Got: {}", result);
}

#[test]
fn test_s12_function_returns_none() {
    let code = r#"
def returns_none() -> None:
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn returns_none"), "Got: {}", result);
}

// ===== Default parameter values =====

#[test]
fn test_s12_default_int_param() {
    let code = r#"
def add(a: int, b: int = 0) -> int:
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn add"), "Got: {}", result);
}

#[test]
fn test_s12_default_string_param() {
    let code = r#"
def greet(name: str = "World") -> str:
    return "Hello, " + name
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_default_none_param() {
    let code = r#"
def process(data: list = None) -> list:
    if data is None:
        data = []
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_default_bool_param() {
    let code = r#"
def verbose_func(verbose: bool = False) -> str:
    if verbose:
        return "verbose"
    return "quiet"
"#;
    let result = transpile(code);
    assert!(result.contains("fn verbose_func"), "Got: {}", result);
}

// ===== Star args and kwargs =====

#[test]
fn test_s12_star_args() {
    let code = r#"
def variadic(*args) -> int:
    total = 0
    for arg in args:
        total += arg
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("variadic"), "Got: {}", result);
}

#[test]
fn test_s12_kwargs() {
    let code = r#"
def with_options(**kwargs) -> dict:
    return kwargs
"#;
    let result = transpile(code);
    assert!(result.contains("with_options"), "Got: {}", result);
}

// ===== Empty collections =====

#[test]
fn test_s12_empty_list() {
    let code = r#"
def make_empty_list() -> list:
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_empty_list"), "Got: {}", result);
}

#[test]
fn test_s12_empty_dict() {
    let code = r#"
def make_empty_dict() -> dict:
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_empty_dict"), "Got: {}", result);
}

#[test]
fn test_s12_empty_set() {
    let code = r#"
def make_empty_set() -> set:
    return set()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_empty_set"), "Got: {}", result);
}

#[test]
fn test_s12_empty_tuple() {
    let code = r#"
def make_empty_tuple() -> tuple:
    return ()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_empty_tuple"), "Got: {}", result);
}

// ===== Boolean edge cases =====

#[test]
fn test_s12_bool_short_circuit_and() {
    let code = r#"
def check_both(a: bool, b: bool) -> bool:
    return a and b
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_both"), "Got: {}", result);
}

#[test]
fn test_s12_bool_short_circuit_or() {
    let code = r#"
def check_either(a: bool, b: bool) -> bool:
    return a or b
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_either"), "Got: {}", result);
}

#[test]
fn test_s12_triple_and() {
    let code = r#"
def all_true(a: bool, b: bool, c: bool) -> bool:
    return a and b and c
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_true"), "Got: {}", result);
}

#[test]
fn test_s12_triple_or() {
    let code = r#"
def any_true(a: bool, b: bool, c: bool) -> bool:
    return a or b or c
"#;
    let result = transpile(code);
    assert!(result.contains("fn any_true"), "Got: {}", result);
}

#[test]
fn test_s12_mixed_and_or() {
    let code = r#"
def complex_logic(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or c
"#;
    let result = transpile(code);
    assert!(result.contains("fn complex_logic"), "Got: {}", result);
}

// ===== None comparison patterns =====

#[test]
fn test_s12_none_return() {
    let code = r#"
def maybe_find(items: list, target: int):
    for item in items:
        if item == target:
            return item
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn maybe_find"), "Got: {}", result);
}

#[test]
fn test_s12_none_default_dict_get() {
    let code = r#"
def safe_lookup(d: dict, key: str):
    return d.get(key, None)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_lookup"), "Got: {}", result);
}

// ===== Nested loops =====

#[test]
fn test_s12_nested_for_break() {
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
fn test_s12_triple_nested_loop() {
    let code = r#"
def cube_sum(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            for k in range(n):
                total += i + j + k
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube_sum"), "Got: {}", result);
}

// ===== Multiple imports =====

#[test]
fn test_s12_multiple_math_imports() {
    let code = r#"
import math

def hypotenuse(a: float, b: float) -> float:
    return math.sqrt(a * a + b * b)

def area_circle(r: float) -> float:
    return math.pi * r * r
"#;
    let result = transpile(code);
    assert!(result.contains("fn hypotenuse"), "Got: {}", result);
    assert!(result.contains("fn area_circle"), "Got: {}", result);
}

// ===== Complex data transformations =====

#[test]
fn test_s12_dict_invert() {
    let code = r#"
def invert_dict(d: dict) -> dict:
    result = {}
    for key in d:
        result[d[key]] = key
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert_dict"), "Got: {}", result);
}

#[test]
fn test_s12_list_to_dict() {
    let code = r#"
def list_to_dict(pairs: list) -> dict:
    result = {}
    for pair in pairs:
        result[pair[0]] = pair[1]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn list_to_dict"), "Got: {}", result);
}

#[test]
fn test_s12_group_by_key() {
    let code = r#"
def group_by(items: list, key_func) -> dict:
    groups = {}
    for item in items:
        k = key_func(item)
        if k not in groups:
            groups[k] = []
        groups[k].append(item)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by"), "Got: {}", result);
}

// ===== Arithmetic edge cases =====

#[test]
fn test_s12_floor_division() {
    let code = r#"
def floor_div(a: int, b: int) -> int:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn floor_div"), "Got: {}", result);
}

#[test]
fn test_s12_modulo() {
    let code = r#"
def modulo(a: int, b: int) -> int:
    return a % b
"#;
    let result = transpile(code);
    assert!(result.contains("fn modulo"), "Got: {}", result);
}

#[test]
fn test_s12_divmod_pattern() {
    let code = r#"
def hours_minutes(total_minutes: int) -> tuple:
    hours = total_minutes // 60
    minutes = total_minutes % 60
    return (hours, minutes)
"#;
    let result = transpile(code);
    assert!(result.contains("fn hours_minutes"), "Got: {}", result);
}

// ===== String edge cases =====

#[test]
fn test_s12_multiline_string() {
    let code = r#"
def get_message() -> str:
    return "Hello\nWorld\n"
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_message"), "Got: {}", result);
}

#[test]
fn test_s12_string_multiplication() {
    let code = r#"
def repeat_str(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat_str"), "Got: {}", result);
}

#[test]
fn test_s12_string_in_check() {
    let code = r#"
def contains_sub(s: str, sub: str) -> bool:
    return sub in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains_sub"), "Got: {}", result);
}

// ===== Complex algorithms =====

#[test]
fn test_s12_bubble_sort() {
    let code = r#"
def bubble_sort(arr: list) -> list:
    n = len(arr)
    for i in range(n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"), "Got: {}", result);
}

#[test]
fn test_s12_gcd_algorithm() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"), "Got: {}", result);
}

#[test]
fn test_s12_sieve_of_eratosthenes() {
    let code = r#"
def sieve(n: int) -> list:
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    is_prime[1] = False
    for i in range(2, int(n ** 0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False
    primes = []
    for i in range(n + 1):
        if is_prime[i]:
            primes.append(i)
    return primes
"#;
    let result = transpile(code);
    assert!(result.contains("fn sieve"), "Got: {}", result);
}

#[test]
fn test_s12_binary_search() {
    let code = r#"
def binary_search(arr: list, target: int) -> int:
    left = 0
    right = len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_with_default_init() {
    let code = r#"
class Config:
    def __init__(self, debug: bool = False, verbose: bool = False):
        self.debug = debug
        self.verbose = verbose

    def is_debug(self) -> bool:
        return self.debug
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_class_var() {
    let code = r#"
class Counter:
    count = 0

    def __init__(self):
        Counter.count += 1

    def get_total(self) -> int:
        return Counter.count
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

// ===== Math constants and operations =====

#[test]
fn test_s12_math_constants() {
    let code = r#"
import math

def circle_circumference(r: float) -> float:
    return 2.0 * math.pi * r

def euler_power(x: float) -> float:
    return math.e ** x
"#;
    let result = transpile(code);
    assert!(result.contains("fn circle_circumference"), "Got: {}", result);
    assert!(result.contains("fn euler_power"), "Got: {}", result);
}

// ===== Error recovery patterns =====

#[test]
fn test_s12_try_with_return_in_except() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    try:
        result = a / b
    except ZeroDivisionError:
        return 0.0
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_try_with_multiple_returns() {
    let code = r#"
def parse_number(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        try:
            return int(float(s))
        except ValueError:
            return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_number"), "Got: {}", result);
}

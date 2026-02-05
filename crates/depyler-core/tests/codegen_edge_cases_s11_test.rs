//! DEPYLER-99MODE-S11: Edge case integration tests for code generation
//!
//! Tests for: augmented assignment, walrus operator simulation, global constants,
//! multiple assignment, tuple unpacking, complex expressions, operator precedence,
//! type coercion, and various Python idioms.

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

// ===== Augmented Assignment =====

#[test]
fn test_s11_augmented_add() {
    let code = r#"
def accumulate(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"));
}

#[test]
fn test_s11_augmented_sub() {
    let code = r#"
def countdown(n: int) -> int:
    result = n
    while result > 0:
        result -= 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn countdown"));
}

#[test]
fn test_s11_augmented_mul() {
    let code = r#"
def power_of_two(n: int) -> int:
    result = 1
    for i in range(n):
        result *= 2
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn power_of_two"));
}

#[test]
fn test_s11_augmented_floordiv() {
    let code = r#"
def halve_repeatedly(n: int) -> int:
    while n > 1:
        n //= 2
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve_repeatedly"));
}

#[test]
fn test_s11_augmented_mod() {
    let code = r#"
def reduce_mod(n: int, m: int) -> int:
    n %= m
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn reduce_mod"));
}

#[test]
fn test_s11_augmented_string_concat() {
    let code = r#"
def build_string(items: list) -> str:
    result = ""
    for item in items:
        result += str(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_string"));
}

// ===== Global Constants =====

#[test]
fn test_s11_int_constant() {
    let code = r#"
MAX_SIZE = 100

def is_valid(x: int) -> bool:
    return x <= MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"));
    assert!(result.contains("100"));
}

#[test]
fn test_s11_string_constant() {
    let code = r#"
DEFAULT_NAME = "World"

def greet(name: str = "World") -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("DEFAULT_NAME") || result.contains("World"));
}

#[test]
fn test_s11_float_constant() {
    let code = r#"
EPSILON = 0.001

def is_close(a: float, b: float) -> bool:
    return abs(a - b) < EPSILON
"#;
    let result = transpile(code);
    assert!(result.contains("EPSILON") || result.contains("0.001"));
}

#[test]
fn test_s11_bool_constant() {
    let code = r#"
DEBUG = True

def log(msg: str):
    if DEBUG:
        print(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("DEBUG"));
}

#[test]
fn test_s11_multiple_constants() {
    let code = r#"
WIDTH = 80
HEIGHT = 24
TITLE = "App"

def get_size() -> int:
    return WIDTH * HEIGHT
"#;
    let result = transpile(code);
    assert!(result.contains("WIDTH"));
    assert!(result.contains("HEIGHT"));
    assert!(result.contains("TITLE"));
}

// ===== Multiple Assignment =====

#[test]
fn test_s11_tuple_unpack_simple() {
    let code = r#"
def get_both() -> tuple:
    x, y = 1, 2
    return (x, y)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_both"));
}

#[test]
fn test_s11_swap() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"));
}

#[test]
fn test_s11_unpack_three() {
    let code = r#"
def unpack() -> int:
    a, b, c = 1, 2, 3
    return a + b + c
"#;
    let result = transpile(code);
    assert!(result.contains("fn unpack"));
}

// ===== Operator Precedence =====

#[test]
fn test_s11_complex_arithmetic() {
    let code = r#"
def compute(x: int, y: int, z: int) -> int:
    return (x + y) * z - x // y
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"));
}

#[test]
fn test_s11_boolean_logic() {
    let code = r#"
def check(a: bool, b: bool, c: bool) -> bool:
    return (a and b) or (not c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"));
    assert!(result.contains("&&") || result.contains("||") || result.contains("!"));
}

#[test]
fn test_s11_chained_comparison() {
    let code = r#"
def in_range(x: int) -> bool:
    return 0 <= x and x < 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn in_range"));
}

#[test]
fn test_s11_power_operation() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn cube"));
    assert!(result.contains("pow") || result.contains("powi"));
}

#[test]
fn test_s11_negation() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"));
}

#[test]
fn test_s11_bitwise_ops() {
    let code = r#"
def bitwise(a: int, b: int) -> int:
    return (a & b) | (a ^ b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn bitwise"));
    assert!(result.contains("&") || result.contains("|") || result.contains("^"));
}

// ===== Type Coercion =====

#[test]
fn test_s11_int_to_float() {
    let code = r#"
def to_float(x: int) -> float:
    return float(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_float"));
    assert!(result.contains("f64") || result.contains("as f64"));
}

#[test]
fn test_s11_float_to_int() {
    let code = r#"
def to_int(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_int"));
    assert!(result.contains("as i") || result.contains("i64") || result.contains("i32"));
}

#[test]
fn test_s11_int_to_str() {
    let code = r#"
def to_string(x: int) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_string"));
    assert!(result.contains("to_string") || result.contains("format!"));
}

#[test]
fn test_s11_str_to_int() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"));
    assert!(result.contains("parse") || result.contains("from_str"));
}

#[test]
fn test_s11_bool_to_int() {
    let code = r#"
def bool_as_int(b: bool) -> int:
    return int(b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn bool_as_int"));
}

// ===== Python Idioms =====

#[test]
fn test_s11_list_multiplication() {
    let code = r#"
def repeat(n: int) -> list:
    return [0] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat"));
}

#[test]
fn test_s11_string_multiplication() {
    let code = r#"
def separator(n: int) -> str:
    return "-" * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn separator"));
    assert!(result.contains("repeat"));
}

#[test]
fn test_s11_ternary_expression() {
    let code = r#"
def sign(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn sign"));
}

#[test]
fn test_s11_multiple_return_values() {
    let code = r#"
def divmod_custom(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn divmod_custom"));
}

#[test]
fn test_s11_default_args() {
    let code = r#"
def greet(name: str = "World") -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"));
}

#[test]
fn test_s11_docstring_preserved() {
    let code = r#"
def documented(x: int) -> int:
    """Returns the square of x."""
    return x * x
"#;
    let result = transpile(code);
    assert!(result.contains("fn documented"));
    // Docstring should be converted to Rust doc comment
    assert!(result.contains("///") || result.contains("square"));
}

// ===== Error Handling Patterns =====

#[test]
fn test_s11_raise_value_error() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be non-negative")
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"));
}

#[test]
fn test_s11_raise_not_implemented() {
    let code = r#"
def abstract_method():
    raise NotImplementedError("subclass must implement")
"#;
    let result = transpile(code);
    assert!(result.contains("fn abstract_method"));
    assert!(result.contains("unimplemented!") || result.contains("todo!") || result.contains("panic!"));
}

// ===== Math Operations =====

#[test]
fn test_s11_math_sqrt() {
    let code = r#"
import math

def distance(x: float, y: float) -> float:
    return math.sqrt(x * x + y * y)
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"));
    assert!(result.contains("sqrt"));
}

#[test]
fn test_s11_math_floor_ceil() {
    let code = r#"
import math

def floor_val(x: float) -> int:
    return math.floor(x)

def ceil_val(x: float) -> int:
    return math.ceil(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn floor_val"));
    assert!(result.contains("fn ceil_val"));
    assert!(result.contains("floor") || result.contains("ceil"));
}

#[test]
fn test_s11_math_abs_float() {
    let code = r#"
import math

def abs_diff(a: float, b: float) -> float:
    return math.fabs(a - b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_diff"));
    assert!(result.contains("abs"));
}

// ===== Collection Patterns =====

#[test]
fn test_s11_empty_list() {
    let code = r#"
from typing import List

def make_list() -> List[int]:
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_list"));
}

#[test]
fn test_s11_empty_dict() {
    let code = r#"
from typing import Dict

def make_dict() -> Dict[str, int]:
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_dict"));
}

#[test]
fn test_s11_empty_set() {
    let code = r#"
from typing import Set

def make_set() -> Set[int]:
    return set()
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_set"));
}

#[test]
fn test_s11_list_literal() {
    let code = r#"
from typing import List

def primes() -> List[int]:
    return [2, 3, 5, 7, 11, 13]
"#;
    let result = transpile(code);
    assert!(result.contains("fn primes"));
}

#[test]
fn test_s11_dict_literal() {
    let code = r#"
from typing import Dict

def config() -> Dict[str, int]:
    return {"width": 80, "height": 24, "depth": 8}
"#;
    let result = transpile(code);
    assert!(result.contains("fn config"));
}

#[test]
fn test_s11_set_literal() {
    let code = r#"
from typing import Set

def vowels() -> Set[str]:
    return {"a", "e", "i", "o", "u"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn vowels"));
}

// ===== Recursive Algorithms =====

#[test]
fn test_s11_fibonacci() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"));
}

#[test]
fn test_s11_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    return gcd(b, a % b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn gcd"));
}

#[test]
fn test_s11_binary_search() {
    let code = r#"
from typing import List

def binary_search(items: List[int], target: int) -> int:
    lo = 0
    hi = len(items) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"));
    assert!(result.contains("lo") || result.contains("hi") || result.contains("mid"));
}

#[test]
fn test_s11_bubble_sort() {
    let code = r#"
from typing import List

def bubble_sort(items: List[int]) -> List[int]:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn bubble_sort"));
}

// ===== Print Patterns =====

#[test]
fn test_s11_print_basic() {
    let code = r#"
def hello():
    print("Hello, World!")
"#;
    let result = transpile(code);
    assert!(result.contains("fn hello"));
    assert!(result.contains("println!"));
}

#[test]
fn test_s11_print_multiple_args() {
    let code = r#"
def show(name: str, age: int):
    print(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show"));
}

#[test]
fn test_s11_print_formatted() {
    let code = r#"
def show_score(name: str, score: int):
    print(f"{name}: {score}")
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_score"));
    assert!(result.contains("println!") || result.contains("format!"));
}

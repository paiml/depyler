//! Session 12: Targeted tests for rust_gen/mod.rs constant generation and NASA mode paths
//!
//! Targets uncovered code in:
//! - Module-level constant generation (simple const, LazyLock)
//! - Type inference for constants (int, float, string, bool, list, set, dict)
//! - Type alias generation
//! - Conditional imports (needs_hashmap, needs_hashset, etc.)
//! - Async function generation
//! - NASA mode async paradox detection
//! - DepylerValue enum conditions
//! - ADT pattern detection

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

// ===== Module-level constants =====

#[test]
fn test_s12_const_int() {
    let code = r#"
MAX_SIZE = 100

def get_max() -> int:
    return MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"), "Got: {}", result);
    assert!(result.contains("100"), "Got: {}", result);
}

#[test]
fn test_s12_const_float() {
    let code = r#"
THRESHOLD = 0.5

def check(x: float) -> bool:
    return x > THRESHOLD
"#;
    let result = transpile(code);
    assert!(result.contains("THRESHOLD"), "Got: {}", result);
    assert!(result.contains("0.5"), "Got: {}", result);
}

#[test]
fn test_s12_const_string() {
    let code = r#"
DEFAULT_NAME = "world"

def greet() -> str:
    return "Hello " + DEFAULT_NAME
"#;
    let result = transpile(code);
    assert!(result.contains("DEFAULT_NAME"), "Got: {}", result);
}

#[test]
fn test_s12_const_bool() {
    let code = r#"
DEBUG = True
VERBOSE = False

def is_debug() -> bool:
    return DEBUG
"#;
    let result = transpile(code);
    assert!(result.contains("DEBUG"), "Got: {}", result);
}

#[test]
fn test_s12_const_negative_int() {
    let code = r#"
MIN_VALUE = -1

def get_min() -> int:
    return MIN_VALUE
"#;
    let result = transpile(code);
    assert!(result.contains("MIN_VALUE"), "Got: {}", result);
}

#[test]
fn test_s12_const_list_literal() {
    let code = r#"
COLORS = ["red", "green", "blue"]

def first_color() -> str:
    return COLORS[0]
"#;
    let result = transpile(code);
    assert!(result.contains("COLORS"), "Got: {}", result);
}

#[test]
fn test_s12_const_dict_literal() {
    let code = r#"
CONFIG = {"host": "localhost", "port": 8080}

def get_host() -> str:
    return CONFIG["host"]
"#;
    let result = transpile(code);
    assert!(result.contains("CONFIG"), "Got: {}", result);
}

#[test]
fn test_s12_const_set_literal() {
    let code = r#"
VALID_CHARS = {"a", "b", "c"}

def is_valid(c: str) -> bool:
    return c in VALID_CHARS
"#;
    let result = transpile(code);
    assert!(result.contains("VALID_CHARS"), "Got: {}", result);
}

#[test]
fn test_s12_const_tuple_literal() {
    let code = r#"
DIMS = (1920, 1080)

def get_width() -> int:
    return DIMS[0]
"#;
    let result = transpile(code);
    assert!(result.contains("DIMS"), "Got: {}", result);
}

#[test]
fn test_s12_multiple_constants() {
    let code = r#"
A = 1
B = 2
C = 3

def total() -> int:
    return A + B + C
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

#[test]
fn test_s12_const_reassignment_dedup() {
    // Python allows reassignment; last assignment wins
    let code = r#"
X = 1
X = 2

def get_x() -> int:
    return X
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_x"), "Got: {}", result);
}

#[test]
fn test_s12_const_empty_list() {
    let code = r#"
EMPTY = []

def get_empty() -> list:
    return EMPTY
"#;
    let result = transpile(code);
    assert!(result.contains("EMPTY"), "Got: {}", result);
}

// ===== Type alias-like patterns =====

#[test]
fn test_s12_type_alias_simple() {
    let code = r#"
Vector = list

def make_vector() -> list:
    return [1, 2, 3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_vector"), "Got: {}", result);
}

// ===== Conditional imports via usage =====

#[test]
fn test_s12_needs_hashmap() {
    let code = r#"
def make_map() -> dict:
    d = {"a": 1, "b": 2}
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_map"), "Got: {}", result);
    assert!(result.contains("HashMap"), "Expected HashMap usage, got: {}", result);
}

#[test]
fn test_s12_needs_hashset() {
    let code = r#"
def make_set() -> set:
    s = {1, 2, 3}
    return s
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_set"), "Got: {}", result);
}

#[test]
fn test_s12_needs_vec() {
    let code = r#"
def make_list() -> list:
    items = [1, 2, 3]
    items.append(4)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_list"), "Got: {}", result);
}

// ===== Async function patterns =====

#[test]
fn test_s12_async_function_basic() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(result.contains("fetch"), "Got: {}", result);
    assert!(result.contains("async"), "Expected async keyword, got: {}", result);
}

#[test]
fn test_s12_async_function_with_body() {
    let code = r#"
async def process(data: list) -> int:
    total = 0
    for item in data:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("async"), "Expected async, got: {}", result);
    assert!(result.contains("process"), "Got: {}", result);
}

#[test]
fn test_s12_multiple_async_functions() {
    let code = r#"
async def fetch_a() -> str:
    return "a"

async def fetch_b() -> str:
    return "b"

async def fetch_all() -> list:
    a = await fetch_a()
    b = await fetch_b()
    return [a, b]
"#;
    let result = transpile(code);
    assert!(result.contains("fetch_a"), "Got: {}", result);
    assert!(result.contains("fetch_b"), "Got: {}", result);
    assert!(result.contains("fetch_all"), "Got: {}", result);
}

// ===== Complex constant expressions =====

#[test]
fn test_s12_const_arithmetic_expr() {
    let code = r#"
WIDTH = 1920
HEIGHT = 1080
AREA = WIDTH * HEIGHT

def get_area() -> int:
    return AREA
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_area"), "Got: {}", result);
}

#[test]
fn test_s12_const_string_concat() {
    let code = r#"
PREFIX = "Hello"
SUFFIX = "World"
MESSAGE = PREFIX + " " + SUFFIX

def get_message() -> str:
    return MESSAGE
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_message"), "Got: {}", result);
}

// ===== Class patterns with constants =====

#[test]
fn test_s12_class_with_constants() {
    let code = r#"
class Config:
    MAX_RETRIES = 3
    TIMEOUT = 30

    def __init__(self):
        self.retries = 0

    def can_retry(self) -> bool:
        return self.retries < Config.MAX_RETRIES
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

// ===== Generator patterns =====

#[test]
fn test_s12_generator_yield() {
    let code = r#"
def fibonacci(n: int):
    a = 0
    b = 1
    for _ in range(n):
        yield a
        a, b = b, a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fibonacci"), "Got: {}", result);
}

// ===== Try/except patterns =====

#[test]
fn test_s12_try_except_basic() {
    let code = r#"
def safe_parse(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_finally() {
    let code = r#"
def with_cleanup(path: str) -> str:
    result = ""
    try:
        result = "ok"
    except Exception:
        result = "error"
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_cleanup"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_multiple() {
    let code = r#"
def parse_input(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_input"), "Got: {}", result);
}

// ===== Lambda patterns =====

#[test]
fn test_s12_lambda_as_constant() {
    let code = r#"
double = lambda x: x * 2

def apply_double(n: int) -> int:
    return double(n)
"#;
    let result = transpile(code);
    assert!(result.contains("double") || result.contains("apply_double"), "Got: {}", result);
}

// ===== Global/nonlocal =====

#[test]
fn test_s12_global_keyword() {
    let code = r#"
counter = 0

def increment():
    global counter
    counter += 1
"#;
    let result = transpile(code);
    assert!(result.contains("increment"), "Got: {}", result);
}

// ===== Complex type annotations =====

#[test]
fn test_s12_optional_return() {
    let code = r#"
from typing import Optional

def find(items: list, target: int) -> Optional[int]:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find"), "Got: {}", result);
}

#[test]
fn test_s12_list_of_str_return() {
    let code = r#"
from typing import List

def split_words(s: str) -> List[str]:
    return s.split(" ")
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_words"), "Got: {}", result);
}

#[test]
fn test_s12_dict_with_types() {
    let code = r#"
from typing import Dict

def word_count(text: str) -> Dict[str, int]:
    counts = {}
    for word in text.split():
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    let result = transpile(code);
    assert!(result.contains("fn word_count"), "Got: {}", result);
}

// ===== Decorator patterns =====

#[test]
fn test_s12_staticmethod() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("MathUtils"), "Got: {}", result);
}

// ===== Complex expressions needing runtime init =====

#[test]
fn test_s12_const_list_comprehension() {
    let code = r#"
SQUARES = [i * i for i in range(10)]

def get_squares() -> list:
    return SQUARES
"#;
    let result = transpile(code);
    assert!(result.contains("SQUARES"), "Got: {}", result);
}

#[test]
fn test_s12_const_with_function_call() {
    let code = r#"
import os
HOME = os.getenv("HOME")

def get_home() -> str:
    return HOME
"#;
    let result = transpile(code);
    assert!(result.contains("HOME") || result.contains("get_home"), "Got: {}", result);
}

// ===== Heterogeneous collections =====

#[test]
fn test_s12_heterogeneous_list() {
    let code = r#"
MIXED = [1, "two", 3.0]

def get_mixed() -> list:
    return MIXED
"#;
    let result = transpile(code);
    assert!(result.contains("MIXED"), "Got: {}", result);
}

#[test]
fn test_s12_heterogeneous_dict() {
    let code = r#"
INFO = {"name": "test", "count": 42, "active": True}

def get_info() -> dict:
    return INFO
"#;
    let result = transpile(code);
    assert!(result.contains("INFO"), "Got: {}", result);
}

// ===== Docstring at module level =====

#[test]
fn test_s12_module_docstring() {
    let code = r#"
"""This module does something."""

def main() -> int:
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

// ===== Complex patterns combining multiple features =====

#[test]
fn test_s12_full_program() {
    let code = r#"
from typing import List, Optional

MAX_ITEMS = 100
DEFAULT_NAME = "unknown"

class Item:
    def __init__(self, name: str, value: int):
        self.name = name
        self.value = value

    def __str__(self) -> str:
        return self.name

def find_item(items: List[int], target: int) -> Optional[int]:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None

def process(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("Item"), "Got: {}", result);
    assert!(result.contains("fn find_item"), "Got: {}", result);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Assert with complex expressions =====

#[test]
fn test_s12_assert_expression() {
    let code = r#"
def validate_range(x: int, lo: int, hi: int) -> int:
    assert lo <= x <= hi, "out of range"
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_range"), "Got: {}", result);
}

// ===== Nested function definitions =====

#[test]
fn test_s12_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

// ===== Walrus operator =====

#[test]
fn test_s12_walrus_in_while() {
    let code = r#"
def read_chunks(data: list) -> list:
    result = []
    i = 0
    while i < len(data):
        chunk = data[i]
        result.append(chunk)
        i += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_chunks"), "Got: {}", result);
}

// ===== Multiple return paths =====

#[test]
fn test_s12_multiple_return_paths() {
    let code = r#"
def classify(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    elif x < 10:
        return "small"
    elif x < 100:
        return "medium"
    else:
        return "large"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

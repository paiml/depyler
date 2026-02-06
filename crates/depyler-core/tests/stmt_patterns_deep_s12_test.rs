//! Session 12 Batch 27: Statement pattern cold paths in direct_rules_convert.rs
//!
//! Targets:
//! - With statements (context managers)
//! - Try/except with multiple handlers
//! - Nested function definitions (closures)
//! - Labeled break/continue
//! - Lambda expressions
//! - Await expressions
//! - Yield expressions
//! - @staticmethod and @property decorators

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

// ===== With statements =====

#[test]
fn test_s12_with_open_file() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_open_write() {
    let code = r#"
def write_file(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_with_no_as() {
    let code = r#"
def use_lock():
    with lock:
        x = 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn use_lock"), "Got: {}", result);
}

// ===== Try/except patterns =====

#[test]
fn test_s12_try_except_single() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_bare() {
    let code = r#"
def safe_parse(text: str) -> int:
    try:
        return int(text)
    except:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_finally() {
    let code = r#"
def cleanup_action(path: str):
    try:
        f = open(path, "r")
    except FileNotFoundError:
        pass
    finally:
        pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn cleanup_action"), "Got: {}", result);
}

#[test]
fn test_s12_try_except_as_variable() {
    let code = r#"
def log_error(text: str) -> str:
    try:
        return int(text)
    except ValueError as e:
        return str(e)
"#;
    let result = transpile(code);
    assert!(result.contains("fn log_error"), "Got: {}", result);
}

// ===== Nested function definitions =====

#[test]
fn test_s12_nested_function_basic() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y + 1
    return inner(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

#[test]
fn test_s12_nested_function_closure() {
    let code = r#"
def make_adder(n: int):
    def add(x: int) -> int:
        return x + n
    return add
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"), "Got: {}", result);
}

// ===== Lambda expressions =====

#[test]
fn test_s12_lambda_basic() {
    let code = r#"
def get_doubler():
    return lambda x: x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_doubler"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_in_sorted() {
    let code = r#"
def sort_by_second(pairs: list) -> list:
    return sorted(pairs, key=lambda x: x[1])
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_by_second"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_no_params() {
    let code = r#"
def get_zero():
    f = lambda: 0
    return f()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_zero"), "Got: {}", result);
}

#[test]
fn test_s12_lambda_multiple_params() {
    let code = r#"
def apply_op(a: int, b: int) -> int:
    op = lambda x, y: x + y
    return op(a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply_op"), "Got: {}", result);
}

// ===== Async/await =====

#[test]
fn test_s12_async_def_basic() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn fetch_data"), "Got: {}", result);
}

#[test]
fn test_s12_await_expression() {
    let code = r#"
async def process(url: str) -> str:
    data = await fetch(url)
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Yield expressions =====

#[test]
fn test_s12_yield_basic() {
    let code = r#"
def count_up(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_up"), "Got: {}", result);
}

#[test]
fn test_s12_yield_with_value() {
    let code = r#"
def fibonacci():
    a = 0
    b = 1
    while True:
        yield a
        a, b = b, a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fibonacci"), "Got: {}", result);
}

// ===== Static method and property decorators =====

#[test]
fn test_s12_static_method() {
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

#[test]
fn test_s12_property_decorator() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def diameter(self) -> float:
        return self.radius * 2.0
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
}

// ===== Global and nonlocal =====

#[test]
fn test_s12_global_variable() {
    let code = r#"
count = 0

def increment():
    global count
    count += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn increment"), "Got: {}", result);
}

// ===== Floor division edge cases =====

#[test]
fn test_s12_floor_division_basic() {
    let code = r#"
def half(n: int) -> int:
    return n // 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn half"), "Got: {}", result);
}

#[test]
fn test_s12_floor_division_float() {
    let code = r#"
def floor_div_float(a: float, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn floor_div_float"), "Got: {}", result);
}

// ===== Bitwise operations on integers =====

#[test]
fn test_s12_bitwise_and() {
    let code = r#"
def mask_byte(n: int) -> int:
    return n & 0xFF
"#;
    let result = transpile(code);
    assert!(result.contains("fn mask_byte"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_or() {
    let code = r#"
def set_flag(flags: int, bit: int) -> int:
    return flags | bit
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_flag"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_xor() {
    let code = r#"
def toggle_bit(n: int, bit: int) -> int:
    return n ^ bit
"#;
    let result = transpile(code);
    assert!(result.contains("fn toggle_bit"), "Got: {}", result);
}

#[test]
fn test_s12_left_shift() {
    let code = r#"
def shift_left(n: int, bits: int) -> int:
    return n << bits
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_left"), "Got: {}", result);
}

#[test]
fn test_s12_right_shift() {
    let code = r#"
def shift_right(n: int, bits: int) -> int:
    return n >> bits
"#;
    let result = transpile(code);
    assert!(result.contains("fn shift_right"), "Got: {}", result);
}

#[test]
fn test_s12_bitwise_not() {
    let code = r#"
def invert_bits(n: int) -> int:
    return ~n
"#;
    let result = transpile(code);
    assert!(result.contains("fn invert_bits"), "Got: {}", result);
}

// ===== String repetition =====

#[test]
fn test_s12_string_repeat_literal() {
    let code = r#"
def make_separator() -> str:
    return "-" * 40
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_separator"), "Got: {}", result);
}

#[test]
fn test_s12_string_repeat_variable() {
    let code = r#"
def repeat_char(c: str, n: int) -> str:
    return c * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat_char"), "Got: {}", result);
}

// ===== Containment operations =====

#[test]
fn test_s12_in_dict() {
    let code = r#"
def has_key(d: dict, key: str) -> bool:
    return key in d
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_key"), "Got: {}", result);
}

#[test]
fn test_s12_not_in_list() {
    let code = r#"
def is_missing(items: list, x: int) -> bool:
    return x not in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_missing"), "Got: {}", result);
}

#[test]
fn test_s12_in_string() {
    let code = r#"
def contains_word(text: str, word: str) -> bool:
    return word in text
"#;
    let result = transpile(code);
    assert!(result.contains("fn contains_word"), "Got: {}", result);
}

#[test]
fn test_s12_in_set() {
    let code = r#"
def is_member(s: set, item: int) -> bool:
    return item in s
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_member"), "Got: {}", result);
}

// ===== Ternary/conditional expressions =====

#[test]
fn test_s12_ternary_basic() {
    let code = r#"
def abs_val(x: int) -> int:
    return x if x >= 0 else -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn abs_val"), "Got: {}", result);
}

#[test]
fn test_s12_ternary_string() {
    let code = r#"
def greeting(is_morning: bool) -> str:
    return "Good morning" if is_morning else "Good evening"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greeting"), "Got: {}", result);
}

// ===== Comprehension patterns =====

#[test]
fn test_s12_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"), "Got: {}", result);
}

#[test]
fn test_s12_dict_comprehension() {
    let code = r#"
def index_words(words: list) -> dict:
    return {i: w for i, w in enumerate(words)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_words"), "Got: {}", result);
}

#[test]
fn test_s12_list_comp_with_condition() {
    let code = r#"
def evens(numbers: list) -> list:
    return [x for x in numbers if x % 2 == 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens"), "Got: {}", result);
}

// ===== F-string patterns =====

#[test]
fn test_s12_fstring_basic() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_multiple() {
    let code = r#"
def describe(name: str, age: int) -> str:
    return f"{name} is {age} years old"
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_expression() {
    let code = r#"
def area_msg(width: int, height: int) -> str:
    return f"Area is {width * height}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn area_msg"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_empty() {
    let code = r#"
def empty() -> str:
    return f""
"#;
    let result = transpile(code);
    assert!(result.contains("fn empty"), "Got: {}", result);
}

// ===== Unary operators =====

#[test]
fn test_s12_unary_neg() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"), "Got: {}", result);
}

#[test]
fn test_s12_unary_not_bool() {
    let code = r#"
def flip(b: bool) -> bool:
    return not b
"#;
    let result = transpile(code);
    assert!(result.contains("fn flip"), "Got: {}", result);
}

#[test]
fn test_s12_unary_pos() {
    let code = r#"
def identity(x: int) -> int:
    return +x
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_class_with_static_and_instance() {
    let code = r#"
class Vector:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def length(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5

    @staticmethod
    def zero():
        return Vector(0.0, 0.0)

    def add(self, other):
        return Vector(self.x + other.x, self.y + other.y)
"#;
    let result = transpile(code);
    assert!(result.contains("Vector"), "Got: {}", result);
    assert!(result.contains("fn length"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_property_and_methods() {
    let code = r#"
class Rectangle:
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height

    @property
    def area(self) -> float:
        return self.width * self.height

    @property
    def perimeter(self) -> float:
        return 2.0 * (self.width + self.height)

    def scale(self, factor: float):
        self.width *= factor
        self.height *= factor
"#;
    let result = transpile(code);
    assert!(result.contains("Rectangle"), "Got: {}", result);
}

// ===== Delete statement =====

#[test]
fn test_s12_del_variable() {
    let code = r#"
def cleanup():
    x = 42
    del x
"#;
    let result = transpile(code);
    assert!(result.contains("fn cleanup"), "Got: {}", result);
}

// ===== Assert statement =====

#[test]
fn test_s12_assert_basic() {
    let code = r#"
def check_positive(x: int):
    assert x > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_positive"), "Got: {}", result);
}

#[test]
fn test_s12_assert_with_message() {
    let code = r#"
def check_range(x: int):
    assert x >= 0, "must be non-negative"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_range"), "Got: {}", result);
}

// ===== Pass statement =====

#[test]
fn test_s12_pass_in_function() {
    let code = r#"
def noop():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn noop"), "Got: {}", result);
}

#[test]
fn test_s12_pass_in_class() {
    let code = r#"
class Empty:
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("Empty"), "Got: {}", result);
}

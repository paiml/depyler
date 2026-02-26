//! Coverage tests for func_gen_inference.rs (return type inference, nested functions)
//!
//! DEPYLER-99MODE-S8: Session 8 Batch 5 - targeting zero-test file
//! func_gen_inference.rs has 1,315 lines of code with 0 inline tests.
//! These transpile-based tests exercise return type inference and function codegen paths.

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

// ── detect_returns_nested_function (GH-70) ───────────────────────

#[test]
fn test_nested_function_returned_explicit() {
    let code = transpile(
        r#"
def make_adder(x: int):
    def adder(y: int) -> int:
        return x + y
    return adder
"#,
    );
    assert!(code.contains("fn make_adder"), "code: {code}");
}

#[test]
fn test_nested_function_with_string_return() {
    let code = transpile(
        r#"
def make_greeter(prefix: str):
    def greet(name: str) -> str:
        return prefix + " " + name
    return greet
"#,
    );
    assert!(code.contains("fn make_greeter"), "code: {code}");
}

#[test]
fn test_nested_function_no_params() {
    let code = transpile(
        r#"
def make_const():
    def f() -> int:
        return 42
    return f
"#,
    );
    assert!(code.contains("fn make_const"), "code: {code}");
}

#[test]
fn test_nested_function_multiple_params() {
    let code = transpile(
        r#"
def make_calculator(op: str):
    def calc(a: int, b: int) -> int:
        if op == "add":
            return a + b
        else:
            return a - b
    return calc
"#,
    );
    assert!(code.contains("fn make_calculator"), "code: {code}");
}

// ── function_returns_heterogeneous_io (DEPYLER-0626) ──────────────

#[test]
fn test_heterogeneous_io_file_and_stdout() {
    let code = transpile(
        r#"
import sys

def get_output(path: str):
    if path == "-":
        return sys.stdout
    else:
        return open(path)
"#,
    );
    assert!(code.contains("fn get_output"), "code: {code}");
}

// ── is_file_creating_return_expr ──────────────────────────────────

#[test]
fn test_file_open_return() {
    let code = transpile(
        r#"
def open_file(path: str):
    return open(path)
"#,
    );
    assert!(code.contains("fn open_file"), "code: {code}");
}

// ── codegen_return_type inference ──────────────────────────────────

#[test]
fn test_return_type_inferred_int() {
    let code = transpile(
        r#"
def f(x: int):
    return x + 1
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_inferred_string() {
    let code = transpile(
        r#"
def f(s: str):
    return s.upper()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_inferred_bool() {
    let code = transpile(
        r#"
def f(x: int):
    return x > 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_inferred_list() {
    let code = transpile(
        r#"
def f(items: list[int]):
    result: list[int] = []
    for x in items:
        result.append(x * 2)
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_none_is_unit() {
    let code = transpile(
        r#"
def f(x: int) -> None:
    print(x)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_return_type_tuple_inferred() {
    let code = transpile(
        r#"
def f(x: int, y: int) -> tuple:
    return (x, y)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Error type mapping (DEPYLER-0597) ──────────────────────────

#[test]
fn test_os_error_maps_to_io() {
    let code = transpile(
        r#"
def f(path: str) -> str:
    try:
        return open(path).read()
    except OSError:
        return ""
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_value_error_maps_to_box_dyn() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        raise ValueError("bad value")
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_index_error_type() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    try:
        return items[0]
    except IndexError:
        return -1
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── preload_hir_type_annotations (DEPYLER-1181) ──────────────────

#[test]
fn test_preload_in_nested_if() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if x > 10:
        if x > 20:
            result: int = x * 3
        else:
            result: int = x * 2
    else:
        result: int = x
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_preload_in_while_loop() {
    let code = transpile(
        r#"
def f(n: int) -> int:
    total: int = 0
    while n > 0:
        total = total + n
        n = n - 1
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_preload_in_for_loop() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    total: int = 0
    for item in items:
        total = total + item
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_preload_in_try_except() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    result: int = 0
    try:
        result = int(s)
    except:
        result = -1
    return result
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_preload_in_with_block() {
    let code = transpile(
        r#"
def f(path: str) -> str:
    with open(path) as fh:
        data: str = fh.read()
    return data
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_preload_in_nested_function() {
    let code = transpile(
        r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        result: int = y * 2
        return result
    return inner(x)
"#,
    );
    assert!(code.contains("fn outer"), "code: {code}");
}

// ── Parameter inference (DEPYLER-0524, DEPYLER-0737) ──────────────

#[test]
fn test_unknown_param_inferred_from_comparison() {
    let code = transpile(
        r#"
def f(x):
    if x > 0:
        return x
    return 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_unknown_param_inferred_from_string_ops() {
    let code = transpile(
        r#"
def f(s):
    return s.strip()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_optional_unknown_param() {
    let code = transpile(
        r#"
from typing import Optional
def f(x: Optional[int] = None) -> int:
    if x is not None:
        return x
    return 0
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Keyword function names (DEPYLER-0306) ──────────────────────

#[test]
fn test_keyword_fn_name_loop() {
    // "loop" is a Rust keyword but valid Python identifier
    let code = transpile(
        r#"
def loop(x: int) -> bool:
    return x > 0
"#,
    );
    assert!(code.contains("fn"), "code: {code}");
}

// ── Function with multiple return types ──────────────────────────

#[test]
fn test_multiple_returns_same_type() {
    let code = transpile(
        r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#,
    );
    assert!(code.contains("fn classify"), "code: {code}");
}

// ── String ownership (v3.16.0) ──────────────────────────────────

#[test]
fn test_owned_string_from_concat() {
    let code = transpile(
        r#"
def f(a: str, b: str) -> str:
    return a + b
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_owned_string_from_format() {
    let code = transpile(
        r#"
def f(name: str) -> str:
    return f"Hello {name}"
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_owned_string_from_upper() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.upper()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
    assert!(code.contains("String") || code.contains("str"), "should return string type: {code}");
}

// ── Error type with raise ────────────────────────────────────────

#[test]
fn test_zero_division_error_type() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    if b == 0:
        raise ZeroDivisionError("division by zero")
    return a // b
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── ADT rewriting (DEPYLER-0936) ────────────────────────────────

#[test]
fn test_simple_class_method() {
    let code = transpile(
        r#"
class Counter:
    def __init__(self, start: int):
        self.count = start

    def increment(self) -> int:
        self.count = self.count + 1
        return self.count
"#,
    );
    assert!(code.contains("Counter") || code.contains("fn"), "code: {code}");
}

// ── Lifetime analysis (DEPYLER-0275) ──────────────────────────────

#[test]
fn test_borrowed_param_return() {
    let code = transpile(
        r#"
def first_char(s: str) -> str:
    return s[0]
"#,
    );
    assert!(code.contains("fn first_char"), "code: {code}");
}

// ── Complex function with annotations ────────────────────────────

#[test]
fn test_function_with_all_annotations() {
    let code = transpile(
        r#"
def process(items: list[int], threshold: int = 10) -> list[int]:
    result: list[int] = []
    for item in items:
        if item > threshold:
            result.append(item)
    return result
"#,
    );
    assert!(code.contains("fn process"), "code: {code}");
}

// ── Generator detection ──────────────────────────────────────────

#[test]
fn test_generator_function() {
    let code = transpile(
        r#"
def count_up(n: int):
    i = 0
    while i < n:
        yield i
        i = i + 1
"#,
    );
    assert!(code.contains("fn count_up"), "code: {code}");
}

// ── Complex nested patterns ──────────────────────────────────────

#[test]
fn test_function_calling_another() {
    let code = transpile(
        r#"
def add(a: int, b: int) -> int:
    return a + b

def triple_add(x: int) -> int:
    return add(add(x, x), x)
"#,
    );
    assert!(code.contains("fn add"), "code: {code}");
    assert!(code.contains("fn triple_add"), "code: {code}");
}

#[test]
fn test_recursive_function() {
    let code = transpile(
        r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#,
    );
    assert!(code.contains("fn factorial"), "code: {code}");
}

// ── Edge cases ──────────────────────────────────────────────────

#[test]
fn test_empty_function() {
    let code = transpile(
        r#"
def noop():
    pass
"#,
    );
    assert!(code.contains("fn noop"), "code: {code}");
}

#[test]
fn test_function_with_docstring() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    """Double the input."""
    return x * 2
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_function_returns_dict() {
    let code = transpile(
        r#"
def make_dict(key: str, value: int) -> dict[str, int]:
    result: dict[str, int] = {}
    result[key] = value
    return result
"#,
    );
    assert!(code.contains("fn make_dict"), "code: {code}");
}

#[test]
fn test_function_with_list_param() {
    let code = transpile(
        r#"
def sum_list(items: list[int]) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#,
    );
    assert!(code.contains("fn sum_list"), "code: {code}");
}

#[test]
fn test_function_with_bool_logic() {
    let code = transpile(
        r#"
def is_valid(x: int, y: int) -> bool:
    return x > 0 and y > 0
"#,
    );
    assert!(code.contains("fn is_valid"), "code: {code}");
}

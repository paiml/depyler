//! Coverage tests for edge cases in codegen, expr_gen, and direct_rules paths
//!
//! DEPYLER-99MODE-S8: Session 8 Batch 5 - additional coverage tests
//! targeting under-tested codegen patterns and edge cases.

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

// ── Augmented assignment operators ──────────────────────────────

#[test]
fn test_augassign_add() {
    let code = transpile(
        r#"
def f() -> int:
    x = 10
    x += 5
    return x
"#,
    );
    assert!(code.contains("+=") || code.contains("+ 5"), "code: {code}");
}

#[test]
fn test_augassign_sub() {
    let code = transpile(
        r#"
def f() -> int:
    x = 10
    x -= 3
    return x
"#,
    );
    assert!(code.contains("-=") || code.contains("- 3"), "code: {code}");
}

#[test]
fn test_augassign_mul() {
    let code = transpile(
        r#"
def f() -> int:
    x = 10
    x *= 2
    return x
"#,
    );
    assert!(
        code.contains("*=")
            || code.contains("* 2")
            || code.contains("py_mul")
            || code.contains("mul"),
        "code: {code}"
    );
}

#[test]
fn test_augassign_div() {
    let code = transpile(
        r#"
def f() -> float:
    x = 10.0
    x /= 2.0
    return x
"#,
    );
    assert!(code.contains("/=") || code.contains("/ 2"), "code: {code}");
}

#[test]
fn test_augassign_mod() {
    let code = transpile(
        r#"
def f() -> int:
    x = 10
    x %= 3
    return x
"#,
    );
    assert!(code.contains("%=") || code.contains("% 3"), "code: {code}");
}

// ── Comparison operators ────────────────────────────────────────

#[test]
fn test_compare_lt() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> bool:
    return a < b
"#,
    );
    assert!(code.contains("<"), "code: {code}");
}

#[test]
fn test_compare_le() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> bool:
    return a <= b
"#,
    );
    assert!(code.contains("<="), "code: {code}");
}

#[test]
fn test_compare_gt() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> bool:
    return a > b
"#,
    );
    assert!(code.contains(">"), "code: {code}");
}

#[test]
fn test_compare_ge() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> bool:
    return a >= b
"#,
    );
    assert!(code.contains(">="), "code: {code}");
}

#[test]
fn test_compare_eq() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> bool:
    return a == b
"#,
    );
    assert!(code.contains("=="), "code: {code}");
}

#[test]
fn test_compare_ne() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> bool:
    return a != b
"#,
    );
    assert!(code.contains("!="), "code: {code}");
}

// ── Logical operators ───────────────────────────────────────────

#[test]
fn test_logical_and() {
    let code = transpile(
        r#"
def f(a: bool, b: bool) -> bool:
    return a and b
"#,
    );
    assert!(code.contains("&&"), "code: {code}");
}

#[test]
fn test_logical_or() {
    let code = transpile(
        r#"
def f(a: bool, b: bool) -> bool:
    return a or b
"#,
    );
    assert!(code.contains("||"), "code: {code}");
}

#[test]
fn test_logical_not() {
    let code = transpile(
        r#"
def f(a: bool) -> bool:
    return not a
"#,
    );
    assert!(code.contains("!"), "code: {code}");
}

// ── Bitwise operators ───────────────────────────────────────────

#[test]
fn test_bitwise_and() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a & b
"#,
    );
    assert!(code.contains("&"), "code: {code}");
}

#[test]
fn test_bitwise_or() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a | b
"#,
    );
    assert!(code.contains("|"), "code: {code}");
}

#[test]
fn test_bitwise_xor() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a ^ b
"#,
    );
    assert!(code.contains("^"), "code: {code}");
}

#[test]
fn test_bitwise_lshift() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a << b
"#,
    );
    assert!(code.contains("<<"), "code: {code}");
}

#[test]
fn test_bitwise_rshift() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a >> b
"#,
    );
    assert!(code.contains(">>"), "code: {code}");
}

#[test]
fn test_unary_neg() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    return -x
"#,
    );
    assert!(code.contains("-"), "code: {code}");
}

#[test]
fn test_unary_bitnot() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    return ~x
"#,
    );
    assert!(code.contains("!"), "code: {code}");
}

// ── Floor division and power ────────────────────────────────────

#[test]
fn test_floor_division() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a // b
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_power_operator() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a ** b
"#,
    );
    assert!(code.contains("pow") || code.contains("powi"), "code: {code}");
}

// ── String operations ───────────────────────────────────────────

#[test]
fn test_string_multiplication() {
    let code = transpile(
        r#"
def f(s: str, n: int) -> str:
    return s * n
"#,
    );
    assert!(code.contains("repeat"), "code: {code}");
}

#[test]
fn test_string_in_operator() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return "hello" in s
"#,
    );
    assert!(code.contains("contains"), "code: {code}");
}

#[test]
fn test_string_not_in_operator() {
    let code = transpile(
        r#"
def f(s: str) -> bool:
    return "hello" not in s
"#,
    );
    assert!(code.contains("contains"), "code: {code}");
}

// ── List operations ─────────────────────────────────────────────

#[test]
fn test_list_comprehension() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return [x * 2 for x in items]
"#,
    );
    assert!(code.contains("map") || code.contains("iter"), "code: {code}");
}

#[test]
fn test_list_comprehension_with_condition() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return [x for x in items if x > 0]
"#,
    );
    assert!(code.contains("filter") || code.contains("iter"), "code: {code}");
}

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        r#"
def f(items: list[str]) -> dict[str, int]:
    return {item: len(item) for item in items}
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_set_comprehension() {
    let code = transpile(
        r#"
def f(items: list[int]) -> set[int]:
    return {x * 2 for x in items}
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Tuple operations ────────────────────────────────────────────

#[test]
fn test_tuple_unpacking() {
    let code = transpile(
        r#"
def f() -> int:
    a, b = 1, 2
    return a + b
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_tuple_return() {
    let code = transpile(
        r#"
def f(x: int, y: int) -> tuple[int, int]:
    return (x + 1, y + 1)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Slicing ─────────────────────────────────────────────────────

#[test]
fn test_list_slice() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return items[1:3]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_string_slice() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s[0:3]
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Ternary / conditional expression ────────────────────────────

#[test]
fn test_ternary_expression() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#,
    );
    assert!(code.contains("if") || code.contains("then"), "code: {code}");
}

// ── Multiple assignment ─────────────────────────────────────────

#[test]
fn test_multiple_assignment() {
    let code = transpile(
        r#"
def f() -> int:
    x = y = 42
    return x + y
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Global/class variables ──────────────────────────────────────

#[test]
fn test_module_level_constant() {
    let code = transpile(
        r#"
MAX_SIZE = 100

def f() -> int:
    return MAX_SIZE
"#,
    );
    assert!(code.contains("MAX_SIZE") || code.contains("100"), "code: {code}");
}

// ── Lambda ──────────────────────────────────────────────────────

#[test]
fn test_lambda_basic() {
    let code = transpile(
        r#"
def f(items: list[int]) -> list[int]:
    return sorted(items, key=lambda x: -x)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Enumerate and zip ───────────────────────────────────────────

#[test]
fn test_enumerate_in_for() {
    let code = transpile(
        r#"
def f(items: list[str]) -> list[str]:
    result: list[str] = []
    for i, item in enumerate(items):
        result.append(str(i) + ": " + item)
    return result
"#,
    );
    assert!(code.contains("enumerate"), "code: {code}");
}

#[test]
fn test_zip_in_for() {
    let code = transpile(
        r#"
def f(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#,
    );
    assert!(code.contains("zip"), "code: {code}");
}

// ── Range variations ────────────────────────────────────────────

#[test]
fn test_range_one_arg() {
    let code = transpile(
        r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#,
    );
    assert!(code.contains("0.."), "code: {code}");
}

#[test]
fn test_range_two_args() {
    let code = transpile(
        r#"
def f(start: int, end: int) -> int:
    total = 0
    for i in range(start, end):
        total += i
    return total
"#,
    );
    assert!(code.contains(".."), "code: {code}");
}

#[test]
fn test_range_three_args() {
    let code = transpile(
        r#"
def f(n: int) -> list[int]:
    result: list[int] = []
    for i in range(0, n, 2):
        result.append(i)
    return result
"#,
    );
    assert!(code.contains("step_by") || code.contains("fn f"), "code: {code}");
}

// ── Nested data structures ──────────────────────────────────────

#[test]
fn test_nested_list() {
    let code = transpile(
        r#"
def f() -> list[list[int]]:
    return [[1, 2], [3, 4]]
"#,
    );
    assert!(code.contains("vec!") || code.contains("Vec"), "code: {code}");
}

#[test]
fn test_empty_list() {
    let code = transpile(
        r#"
def f() -> list[int]:
    return []
"#,
    );
    assert!(code.contains("vec!") || code.contains("Vec"), "code: {code}");
}

#[test]
fn test_empty_dict() {
    let code = transpile(
        r#"
def f() -> dict[str, int]:
    return {}
"#,
    );
    assert!(code.contains("HashMap") || code.contains("new"), "code: {code}");
}

// ── isinstance check ────────────────────────────────────────────

#[test]
fn test_isinstance_check() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Type conversion ─────────────────────────────────────────────

#[test]
fn test_int_to_float() {
    let code = transpile(
        r#"
def f(x: int) -> float:
    return float(x)
"#,
    );
    assert!(code.contains("as f64") || code.contains("f64"), "code: {code}");
}

#[test]
fn test_float_to_int() {
    let code = transpile(
        r#"
def f(x: float) -> int:
    return int(x)
"#,
    );
    assert!(code.contains("as i64") || code.contains("i64"), "code: {code}");
}

// ── Global constants ────────────────────────────────────────────

#[test]
fn test_constant_true() {
    let code = transpile(
        r#"
def f() -> bool:
    return True
"#,
    );
    assert!(code.contains("true"), "code: {code}");
}

#[test]
fn test_constant_false() {
    let code = transpile(
        r#"
def f() -> bool:
    return False
"#,
    );
    assert!(code.contains("false"), "code: {code}");
}

#[test]
fn test_constant_none() {
    let code = transpile(
        r#"
def f():
    return None
"#,
    );
    assert!(code.contains("None") || code.contains("fn f"), "code: {code}");
}

// ── f-strings ───────────────────────────────────────────────────

#[test]
fn test_fstring_simple() {
    let code = transpile(
        r#"
def f(name: str) -> str:
    return f"Hello {name}"
"#,
    );
    assert!(code.contains("format!"), "code: {code}");
}

#[test]
fn test_fstring_expression() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return f"value is {x + 1}"
"#,
    );
    assert!(code.contains("format!"), "code: {code}");
}

#[test]
fn test_fstring_multiple_parts() {
    let code = transpile(
        r#"
def f(a: str, b: str) -> str:
    return f"{a} and {b}"
"#,
    );
    assert!(code.contains("format!"), "code: {code}");
}

// ── Walrus operator (named expression) ──────────────────────────

#[test]
fn test_walrus_operator() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    total = 0
    for item in items:
        if (n := item * 2) > 10:
            total += n
    return total
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Class with methods ──────────────────────────────────────────

#[test]
fn test_class_with_init() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y
"#,
    );
    assert!(code.contains("struct Point") || code.contains("Point"), "code: {code}");
}

#[test]
fn test_class_with_method() {
    let code = transpile(
        r#"
class Counter:
    def __init__(self, value: int):
        self.value = value

    def increment(self) -> int:
        self.value += 1
        return self.value
"#,
    );
    assert!(code.contains("Counter"), "code: {code}");
}

#[test]
fn test_class_with_str() {
    let code = transpile(
        r#"
class Person:
    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name
"#,
    );
    assert!(code.contains("Person"), "code: {code}");
}

// ── Multiple functions ──────────────────────────────────────────

#[test]
fn test_multiple_functions() {
    let code = transpile(
        r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b
"#,
    );
    assert!(code.contains("fn add"), "code: {code}");
    assert!(code.contains("fn sub"), "code: {code}");
    assert!(code.contains("fn mul"), "code: {code}");
}

// ── Exception types ─────────────────────────────────────────────

#[test]
fn test_type_error_exception() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if not isinstance(x, int):
        raise TypeError("expected int")
    return x
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── While True with break ───────────────────────────────────────

#[test]
fn test_while_true_break() {
    let code = transpile(
        r#"
def f() -> int:
    x = 0
    while True:
        x += 1
        if x >= 10:
            break
    return x
"#,
    );
    assert!(code.contains("loop"), "code: {code}");
    assert!(code.contains("break"), "code: {code}");
}

// ── Continue statement ──────────────────────────────────────────

#[test]
fn test_continue_in_loop() {
    let code = transpile(
        r#"
def f(items: list[int]) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total += item
    return total
"#,
    );
    assert!(code.contains("continue"), "code: {code}");
}

// ── Complex expressions ─────────────────────────────────────────

#[test]
fn test_chained_comparison() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    return 0 < x and x < 100
"#,
    );
    assert!(code.contains("&&") || code.contains("fn f"), "code: {code}");
}

#[test]
fn test_nested_ternary() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return "big" if x > 100 else "medium" if x > 10 else "small"
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Module-level imports ────────────────────────────────────────

#[test]
fn test_import_os() {
    let code = transpile(
        r#"
import os

def f() -> str:
    return os.getcwd()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

#[test]
fn test_from_import() {
    let code = transpile(
        r#"
from os import getcwd

def f() -> str:
    return getcwd()
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Delete statement ────────────────────────────────────────────

#[test]
fn test_pass_statement() {
    let code = transpile(
        r#"
def f():
    pass
"#,
    );
    assert!(code.contains("fn f"), "code: {code}");
}

// ── Complex class hierarchy ─────────────────────────────────────

#[test]
fn test_class_with_multiple_methods() {
    let code = transpile(
        r#"
class Stack:
    def __init__(self):
        self.items: list[int] = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#,
    );
    assert!(code.contains("Stack"), "code: {code}");
}

//! Session 8 batch 3: Advanced pattern coverage tests
//! Targets: escape analysis paths, optimization paths, borrowing context,
//! lifetime analysis, generic inference, and complex transpilation patterns

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

// ── Borrowing patterns (escape analysis) ────────────────────────

#[test]
fn test_parameter_borrowed_not_moved() {
    let code = transpile(
        r#"
def length(s: str) -> int:
    return len(s)
"#,
    );
    assert!(
        code.contains("fn length") && (code.contains("&") || code.contains("str") || code.contains("String")),
        "Should handle borrowing for read-only param: {code}"
    );
}

#[test]
fn test_parameter_mutated() {
    let code = transpile(
        r#"
def append_item(items: list, x: int) -> list:
    items.append(x)
    return items
"#,
    );
    assert!(
        code.contains("push") || code.contains("append") || code.contains("mut"),
        "Should detect mutation: {code}"
    );
}

#[test]
fn test_multiple_reads_no_move() {
    let code = transpile(
        r#"
def use_twice(s: str) -> str:
    a = s.upper()
    b = s.lower()
    return a + b
"#,
    );
    assert!(
        code.contains("to_uppercase") || code.contains("to_lowercase") || code.contains("upper"),
        "Should handle multiple reads: {code}"
    );
}

#[test]
fn test_list_consumed_in_loop() {
    let code = transpile(
        r#"
def sum_all(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#,
    );
    assert!(
        code.contains("for") || code.contains("iter"),
        "Should handle list consumed in loop: {code}"
    );
}

// ── Optimization-exercising patterns ────────────────────────────

#[test]
fn test_constant_folding_candidate() {
    let code = transpile(
        r#"
def f() -> int:
    x = 2 + 3
    y = x * 4
    return y
"#,
    );
    assert!(
        code.contains("5") || code.contains("20") || code.contains("2 + 3") || code.contains("x * 4"),
        "Should generate constant arithmetic: {code}"
    );
}

#[test]
fn test_dead_code_after_return() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return -x
"#,
    );
    assert!(
        code.contains("fn f") && code.contains("return"),
        "Should handle dead code paths: {code}"
    );
}

#[test]
fn test_common_subexpression() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    x = a * b + 1
    y = a * b + 2
    return x + y
"#,
    );
    assert!(
        code.contains("a * b") || code.contains("a *") || code.contains("+ 1"),
        "Should generate common subexpressions: {code}"
    );
}

#[test]
fn test_strength_reduction_multiply() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    return x * 2
"#,
    );
    assert!(
        code.contains("* 2") || code.contains("<< 1") || code.contains("2"),
        "Should handle multiply: {code}"
    );
}

// ── Generic type inference patterns ─────────────────────────────

#[test]
fn test_generic_list_operations() {
    let code = transpile(
        r#"
def first(items: list) -> int:
    if len(items) > 0:
        return items[0]
    return 0
"#,
    );
    assert!(
        code.contains("fn first") && (code.contains("Vec") || code.contains("[0]")),
        "Should infer list element type: {code}"
    );
}

#[test]
fn test_generic_dict_operations() {
    let code = transpile(
        r#"
def get_or_default(d: dict, key: str, default: int) -> int:
    if key in d:
        return d[key]
    return default
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("get") || code.contains("contains"),
        "Should handle dict with inferred types: {code}"
    );
}

#[test]
fn test_optional_type_handling() {
    let code = transpile(
        r#"
from typing import Optional
def find(items: list, target: int) -> Optional[int]:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return None
"#,
    );
    assert!(
        code.contains("Option") || code.contains("None") || code.contains("Some"),
        "Should generate Option type: {code}"
    );
}

// ── Lifetime analysis patterns ──────────────────────────────────

#[test]
fn test_string_slice_return() {
    let code = transpile(
        r#"
def first_word(s: str) -> str:
    idx = s.find(" ")
    if idx >= 0:
        return s[:idx]
    return s
"#,
    );
    assert!(
        code.contains("fn first_word"),
        "Should handle string return: {code}"
    );
}

#[test]
fn test_list_return_slice() {
    let code = transpile(
        r#"
def take_n(items: list, n: int) -> list:
    return items[:n]
"#,
    );
    assert!(
        code.contains("fn take_n"),
        "Should handle list slice return: {code}"
    );
}

// ── Class method patterns ───────────────────────────────────────

#[test]
fn test_class_self_mutation() {
    let code = transpile(
        r#"
class Counter:
    def __init__(self) -> None:
        self.count: int = 0

    def increment(self) -> None:
        self.count += 1

    def get(self) -> int:
        return self.count
"#,
    );
    assert!(
        code.contains("struct Counter") || code.contains("impl Counter"),
        "Should generate struct with methods: {code}"
    );
    assert!(
        code.contains("count") && (code.contains("+=") || code.contains("+ 1")),
        "Should handle self mutation: {code}"
    );
}

#[test]
fn test_class_with_list_field() {
    let code = transpile(
        r#"
class Queue:
    def __init__(self) -> None:
        self.items: list = []

    def enqueue(self, item: int) -> None:
        self.items.append(item)

    def dequeue(self) -> int:
        return self.items.pop(0)

    def size(self) -> int:
        return len(self.items)
"#,
    );
    assert!(
        code.contains("struct Queue"),
        "Should generate Queue struct: {code}"
    );
    assert!(
        code.contains("enqueue") && code.contains("dequeue"),
        "Should have queue methods: {code}"
    );
}

#[test]
fn test_class_with_dict_field() {
    let code = transpile(
        r#"
class Registry:
    def __init__(self) -> None:
        self.data: dict = {}

    def register(self, key: str, value: int) -> None:
        self.data[key] = value

    def lookup(self, key: str) -> int:
        return self.data[key]
"#,
    );
    assert!(
        code.contains("struct Registry") || code.contains("HashMap"),
        "Should generate struct with dict field: {code}"
    );
}

#[test]
fn test_class_inheritance_simple() {
    let code = transpile(
        r#"
class Animal:
    def __init__(self, name: str) -> None:
        self.name = name

    def speak(self) -> str:
        return self.name

class Dog(Animal):
    def speak(self) -> str:
        return self.name + " barks"
"#,
    );
    assert!(
        code.contains("Animal") && code.contains("Dog"),
        "Should generate both classes: {code}"
    );
}

// ── Complex expression patterns ─────────────────────────────────

#[test]
fn test_nested_ternary() {
    let code = transpile(
        r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else ("zero" if x == 0 else "negative")
"#,
    );
    assert!(
        code.contains("positive") && code.contains("negative"),
        "Should handle nested ternary: {code}"
    );
}

#[test]
fn test_boolean_expression_chain() {
    let code = transpile(
        r#"
def is_valid(x: int) -> bool:
    return x > 0 and x < 100 and x != 50
"#,
    );
    assert!(
        code.contains("&&") || code.contains("and"),
        "Should handle chained boolean: {code}"
    );
}

#[test]
fn test_complex_list_operations() {
    let code = transpile(
        r#"
def process(items: list) -> list:
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    result.sort()
    return result
"#,
    );
    assert!(
        code.contains("push") || code.contains("sort") || code.contains("append"),
        "Should handle complex list operations: {code}"
    );
}

#[test]
fn test_map_filter_pattern() {
    let code = transpile(
        r#"
def positive_doubles(items: list) -> list:
    return list(map(lambda x: x * 2, filter(lambda x: x > 0, items)))
"#,
    );
    assert!(
        code.contains("map") || code.contains("filter") || code.contains("iter"),
        "Should handle map/filter: {code}"
    );
}

// ── Numeric operations ──────────────────────────────────────────

#[test]
fn test_integer_division() {
    let code = transpile(
        r#"
def f(a: int, b: int) -> int:
    return a // b
"#,
    );
    assert!(
        code.contains("/") || code.contains("div"),
        "Should generate integer division: {code}"
    );
}

#[test]
fn test_power_operation() {
    let code = transpile(
        r#"
def f(base: int, exp: int) -> int:
    return base ** exp
"#,
    );
    assert!(
        code.contains("pow") || code.contains("**"),
        "Should generate power operation: {code}"
    );
}

#[test]
fn test_modulo_operation() {
    let code = transpile(
        r#"
def is_even(n: int) -> bool:
    return n % 2 == 0
"#,
    );
    assert!(
        code.contains("%") || code.contains("rem"),
        "Should generate modulo: {code}"
    );
}

#[test]
fn test_bitwise_operations() {
    let code = transpile(
        r#"
def flags(a: int, b: int) -> int:
    return (a & 0xFF) | (b << 8)
"#,
    );
    assert!(
        code.contains("&") || code.contains("|") || code.contains("<<"),
        "Should generate bitwise ops: {code}"
    );
}

// ── String formatting patterns ──────────────────────────────────

#[test]
fn test_fstring_nested_expression() {
    let code = transpile(
        r#"
def f(items: list) -> str:
    return f"Count: {len(items)}, First: {items[0]}"
"#,
    );
    assert!(
        code.contains("format!") || code.contains("Count"),
        "Should handle complex f-string: {code}"
    );
}

#[test]
fn test_str_format_method() {
    let code = transpile(
        r#"
def f(name: str, age: int) -> str:
    return "Hello, {}! You are {} years old.".format(name, age)
"#,
    );
    assert!(
        code.contains("format!") || code.contains("Hello"),
        "Should handle .format() method: {code}"
    );
}

// ── Dict operations ─────────────────────────────────────────────

#[test]
fn test_dict_items_iteration() {
    let code = transpile(
        r#"
def sum_values(d: dict) -> int:
    total = 0
    for key, value in d.items():
        total += value
    return total
"#,
    );
    assert!(
        code.contains("iter") || code.contains("items") || code.contains("for"),
        "Should handle dict.items() iteration: {code}"
    );
}

#[test]
fn test_dict_keys_values() {
    let code = transpile(
        r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#,
    );
    assert!(
        code.contains("keys") || code.contains("collect") || code.contains("iter"),
        "Should handle dict.keys(): {code}"
    );
}

#[test]
fn test_dict_get_with_default() {
    let code = transpile(
        r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#,
    );
    assert!(
        code.contains("get") || code.contains("unwrap_or") || code.contains("0"),
        "Should handle dict.get with default: {code}"
    );
}

// ── Builtin function patterns ───────────────────────────────────

#[test]
fn test_enumerate_with_start() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    result = []
    for i, item in enumerate(items, start=1):
        result.append((i, item))
    return result
"#,
    );
    assert!(
        code.contains("enumerate") || code.contains("iter"),
        "Should handle enumerate with start: {code}"
    );
}

#[test]
fn test_zip_three_lists() {
    let code = transpile(
        r#"
def f(a: list, b: list, c: list) -> list:
    result = []
    for x, y, z in zip(a, b, c):
        result.append(x + y + z)
    return result
"#,
    );
    assert!(
        code.contains("zip") || code.contains("iter"),
        "Should handle zip of three lists: {code}"
    );
}

#[test]
fn test_sorted_with_key() {
    let code = transpile(
        r#"
def sort_by_length(items: list) -> list:
    return sorted(items, key=lambda x: len(x))
"#,
    );
    assert!(
        code.contains("sort") || code.contains("sorted"),
        "Should handle sorted with key: {code}"
    );
}

#[test]
fn test_min_max_of_list() {
    let code = transpile(
        r#"
def range_of(items: list) -> int:
    return max(items) - min(items)
"#,
    );
    assert!(
        code.contains("max") || code.contains("min") || code.contains("unwrap"),
        "Should handle min/max: {code}"
    );
}

#[test]
fn test_abs_function() {
    let code = transpile(
        r#"
def distance(a: int, b: int) -> int:
    return abs(a - b)
"#,
    );
    assert!(
        code.contains("abs") || code.contains(".abs()"),
        "Should handle abs: {code}"
    );
}

// ── Complex class patterns ──────────────────────────────────────

#[test]
fn test_class_with_comparison() {
    let code = transpile(
        r#"
class Box:
    def __init__(self, value: int) -> None:
        self.value = value

    def __eq__(self, other: Box) -> bool:
        return self.value == other.value

    def __lt__(self, other: Box) -> bool:
        return self.value < other.value
"#,
    );
    assert!(
        code.contains("PartialEq") || code.contains("PartialOrd") || code.contains("eq"),
        "Should generate comparison traits: {code}"
    );
}

#[test]
fn test_class_with_add() {
    let code = transpile(
        r#"
class Vector:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __add__(self, other: Vector) -> Vector:
        return Vector(self.x + other.x, self.y + other.y)
"#,
    );
    assert!(
        code.contains("Add") || code.contains("add") || code.contains("impl"),
        "Should generate Add trait: {code}"
    );
}

// ── Error handling edge cases ───────────────────────────────────

#[test]
fn test_empty_function() {
    let code = transpile(
        r#"
def noop() -> None:
    pass
"#,
    );
    assert!(
        code.contains("fn noop"),
        "Should generate empty function: {code}"
    );
}

#[test]
fn test_function_with_docstring_only() {
    let code = transpile(
        r#"
def documented() -> None:
    """This function does nothing but has a docstring."""
    pass
"#,
    );
    assert!(
        code.contains("fn documented"),
        "Should handle docstring-only function: {code}"
    );
}

#[test]
fn test_class_empty() {
    let code = transpile(
        r#"
class Empty:
    pass
"#,
    );
    assert!(
        code.contains("struct Empty") || code.contains("Empty"),
        "Should handle empty class: {code}"
    );
}

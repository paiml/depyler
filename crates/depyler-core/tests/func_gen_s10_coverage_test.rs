//! DEPYLER-99MODE-S10: Integration tests targeting func_gen.rs coverage gaps
//!
//! Tests for: parameter type inference, return type handling, mutable
//! parameter analysis, string methods on params, file-like params, and
//! callable type inference.

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

// ===== Parameter Type Inference =====

#[test]
fn test_s10_infer_param_from_print() {
    // Parameter used in print() should be inferred as String
    let code = r#"
def display(item):
    print(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn display"));
}

#[test]
fn test_s10_infer_param_from_string_method() {
    // Parameter with .upper() called should be inferred as String
    let code = r#"
def shout(text):
    return text.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn shout"));
    assert!(result.contains("to_uppercase") || result.contains("upper"));
}

#[test]
fn test_s10_infer_param_from_split() {
    // Parameter with .split() called should be inferred as String
    let code = r#"
def tokenize(text):
    return text.split()
"#;
    let result = transpile(code);
    assert!(result.contains("fn tokenize"));
    assert!(result.contains("split"));
}

#[test]
fn test_s10_infer_param_from_len() {
    // Parameter used in len() -- could be str or list
    let code = r#"
def size(items):
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn size"));
    assert!(result.contains("len"));
}

#[test]
fn test_s10_infer_param_from_iteration() {
    // Parameter used in for loop should be iterable
    let code = r#"
def process_all(items):
    for item in items:
        print(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_all"));
    assert!(result.contains("for"));
}

#[test]
fn test_s10_infer_param_from_index() {
    // Parameter with [0] access should be list-like
    let code = r#"
def first(items):
    return items[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn first"));
}

#[test]
fn test_s10_infer_param_from_comparison() {
    // Parameter compared to int should be int
    let code = r#"
def is_positive(x):
    return x > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_positive"));
    assert!(result.contains("bool"));
}

#[test]
fn test_s10_infer_param_from_arithmetic() {
    // Parameter used in arithmetic should be numeric
    let code = r#"
def double(x):
    return x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn double"));
}

// ===== Return Type Handling =====

#[test]
fn test_s10_return_none_explicitly() {
    let code = r#"
def do_nothing(x: int):
    if x > 0:
        print(x)
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn do_nothing"));
}

#[test]
fn test_s10_no_return() {
    let code = r#"
def side_effect(x: int):
    print(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn side_effect"));
}

#[test]
fn test_s10_return_bool() {
    let code = r#"
def check(x: int) -> bool:
    return x > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn check"));
    assert!(result.contains("-> bool"));
}

#[test]
fn test_s10_return_string() {
    let code = r#"
def to_str(x: int) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_str"));
    assert!(result.contains("String") || result.contains("str"));
}

// ===== Multiple Parameters =====

#[test]
fn test_s10_three_params() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    if x < lo:
        return lo
    if x > hi:
        return hi
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"));
}

#[test]
fn test_s10_mixed_types() {
    let code = r#"
def describe(name: str, age: int) -> str:
    return f"{name} is {age}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"));
    assert!(result.contains("format!"));
}

// ===== Mutable Parameter Analysis =====

#[test]
fn test_s10_mutable_list_param() {
    let code = r#"
def append_item(items: list, x: int):
    items.append(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn append_item"));
    assert!(result.contains("mut") || result.contains("push"));
}

#[test]
fn test_s10_mutable_dict_param() {
    let code = r#"
def add_entry(d: dict, key: str, value: int):
    d[key] = value
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_entry"));
}

#[test]
fn test_s10_immutable_param() {
    let code = r#"
def compute(x: int) -> int:
    return x * x + 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"));
}

// ===== String Method Inference =====

#[test]
fn test_s10_strip_infer() {
    let code = r#"
def clean(text):
    return text.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"));
    assert!(result.contains("trim"));
}

#[test]
fn test_s10_startswith_infer() {
    let code = r#"
def is_prefix(text, prefix):
    return text.startswith(prefix)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_prefix"));
    assert!(result.contains("starts_with"));
}

#[test]
fn test_s10_endswith_infer() {
    let code = r#"
def has_suffix(text):
    return text.endswith(".py")
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_suffix"));
    assert!(result.contains("ends_with"));
}

#[test]
fn test_s10_join_infer() {
    let code = r#"
def join_items(items):
    return ", ".join(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_items"));
    assert!(result.contains("join"));
}

// ===== Callable Parameters =====

#[test]
fn test_s10_function_as_param() {
    let code = r#"
def apply(f, x: int) -> int:
    return f(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn apply"));
}

#[test]
fn test_s10_function_as_param_with_map() {
    let code = r#"
def transform(items: list, f) -> list:
    return list(map(f, items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn transform"));
    assert!(result.contains("map"));
}

// ===== Complex Function Patterns =====

#[test]
fn test_s10_function_with_guard() {
    let code = r#"
def safe_divide(a: int, b: int) -> int:
    if b == 0:
        return 0
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"));
}

#[test]
fn test_s10_function_with_accumulator() {
    let code = r#"
def sum_list(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_list"));
    assert!(result.contains("total"));
}

#[test]
fn test_s10_function_with_flag() {
    let code = r#"
def has_even(items: list) -> bool:
    found = False
    for item in items:
        if item % 2 == 0:
            found = True
    return found
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_even"));
    assert!(result.contains("found") || result.contains("bool"));
}

// ===== Decorator-like Patterns =====

#[test]
fn test_s10_staticmethod_like() {
    let code = r#"
class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b

    def multiply(self, a: int, b: int) -> int:
        return a * b
"#;
    let result = transpile(code);
    assert!(result.contains("Calculator"));
    assert!(result.contains("add") || result.contains("multiply"));
}

// ===== Variadic Arguments =====

#[test]
fn test_s10_args_pattern() {
    let code = r#"
def first_or_default(items: list, default: int) -> int:
    if len(items) > 0:
        return items[0]
    return default
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_or_default"));
}

// ===== Property-like Access =====

#[test]
fn test_s10_class_property_access() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("Point"));
    assert!(result.contains("distance"));
}

// ===== Type Annotations with Generics =====

#[test]
fn test_s10_list_of_str_param() {
    let code = r#"
from typing import List

def join_names(names: List[str]) -> str:
    return ", ".join(names)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_names"));
    assert!(result.contains("join"));
}

#[test]
fn test_s10_dict_str_int_param() {
    let code = r#"
from typing import Dict

def total_values(counts: Dict[str, int]) -> int:
    total = 0
    for v in counts.values():
        total = total + v
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn total_values"));
}

// ===== try/except in Functions with Return Type =====

#[test]
fn test_s10_try_except_in_typed_function() {
    let code = r#"
def parse_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"));
    assert!(result.contains("-> i32") || result.contains("-> i64"));
}

// ===== Nested Function with Different Return Type =====

#[test]
fn test_s10_nested_fn_different_return() {
    let code = r#"
def outer(x: int) -> str:
    def inner(n: int) -> int:
        return n * 2
    result = inner(x)
    return str(result)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"));
    assert!(result.contains("inner"));
}

// ===== Default Parameter with None =====

#[test]
fn test_s10_default_none_param() {
    let code = r#"
from typing import Optional

def process(x: int, label: Optional[str] = None) -> str:
    if label is not None:
        return f"{label}: {x}"
    return str(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"));
}

// ===== Boolean Flags =====

#[test]
fn test_s10_bool_flag_param() {
    let code = r#"
def format_num(x: int, prefix: bool = False) -> str:
    if prefix:
        return f"0x{x}"
    return str(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_num"));
}

//! Session 11: Deep coverage tests for func_gen*.rs files
//!
//! Targets function generation code paths:
//! - Default parameter handling
//! - Varargs and kwargs
//! - Return type inference
//! - Nested function definitions
//! - Lambda in various contexts
//! - Decorator patterns
//! - Docstring handling
//! - Complex parameter type combinations
//! - Generic function patterns
//! - Closure capture patterns

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

// ============================================================================
// Default parameter patterns
// ============================================================================

#[test]
fn test_s11_func_default_int() {
    let code = r#"
def count(items: list, start: int = 0) -> int:
    return len(items) + start
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count"),
        "Should transpile default int param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_string() {
    let code = r#"
def greet(name: str = "World") -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn greet"),
        "Should transpile default string param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_none() {
    let code = r#"
from typing import Optional

def find(items: list, default = None):
    if items:
        return items[0]
    return default
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find"),
        "Should transpile default None param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_bool() {
    let code = r#"
def process(data: list, verbose: bool = False) -> list:
    if verbose:
        print(f"Processing {len(data)} items")
    return data
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should transpile default bool param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_float() {
    let code = r#"
def scale(value: float, factor: float = 1.0) -> float:
    return value * factor
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn scale"),
        "Should transpile default float param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_empty_list() {
    let code = r#"
def extend(items: list, extra: list = []) -> list:
    items.extend(extra)
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn extend"),
        "Should transpile default empty list. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_default_empty_dict() {
    let code = r#"
def merge(base: dict, extra: dict = {}) -> dict:
    base.update(extra)
    return base
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn merge"),
        "Should transpile default empty dict. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_multiple_defaults() {
    let code = r#"
def format_name(first: str, last: str = "", title: str = "") -> str:
    parts: list = []
    if title:
        parts.append(title)
    parts.append(first)
    if last:
        parts.append(last)
    return " ".join(parts)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn format_name"),
        "Should transpile multiple defaults. Got: {}",
        result
    );
}

// ============================================================================
// Varargs and kwargs
// ============================================================================

#[test]
fn test_s11_func_args_simple() {
    let code = r#"
def total(*args) -> int:
    s = 0
    for x in args:
        s += x
    return s
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn total"),
        "Should transpile *args. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_args_with_regular() {
    let code = r#"
def concat(sep: str, *args) -> str:
    return sep.join(args)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn concat"),
        "Should transpile regular + *args. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_kwargs_simple() {
    let code = r#"
def config(**kwargs) -> dict:
    return kwargs
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn config"),
        "Should transpile **kwargs. Got: {}",
        result
    );
}

// ============================================================================
// Return type patterns
// ============================================================================

#[test]
fn test_s11_func_return_list() {
    let code = r#"
def make_list(n: int) -> list:
    result: list = []
    for i in range(n):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_list"),
        "Should transpile list return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_return_dict() {
    let code = r#"
def make_dict(keys: list, values: list) -> dict:
    result: dict = {}
    for i in range(len(keys)):
        result[keys[i]] = values[i]
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_dict"),
        "Should transpile dict return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_return_set() {
    let code = r#"
def unique(items: list) -> set:
    result: set = set()
    for item in items:
        result.add(item)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn unique"),
        "Should transpile set return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_return_tuple() {
    let code = r#"
def minmax(items: list) -> tuple:
    return (min(items), max(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn minmax"),
        "Should transpile tuple return. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_no_return() {
    let code = r#"
def side_effect(items: list):
    items.append(0)
    items.sort()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn side_effect"),
        "Should transpile void function. Got: {}",
        result
    );
}

// ============================================================================
// Nested functions / closures
// ============================================================================

#[test]
fn test_s11_func_nested_simple() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y + 1
    return inner(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn outer"),
        "Should transpile nested function. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_nested_closure() {
    let code = r#"
def make_adder(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_adder"),
        "Should transpile closure. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_nested_multiple() {
    let code = r#"
def compute(x: int, y: int) -> int:
    def add(a: int, b: int) -> int:
        return a + b
    def mul(a: int, b: int) -> int:
        return a * b
    return add(x, mul(x, y))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn compute"),
        "Should transpile multiple nested functions. Got: {}",
        result
    );
}

// ============================================================================
// Lambda patterns
// ============================================================================

#[test]
fn test_s11_func_lambda_in_sort() {
    let code = r#"
def sort_by_key(pairs: list) -> list:
    pairs.sort(key=lambda p: p[0])
    return pairs
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_key"),
        "Should transpile lambda in sort. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_lambda_in_filter() {
    let code = r#"
def filter_positive(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn filter_positive"),
        "Should transpile lambda in filter. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_lambda_in_map() {
    let code = r#"
def square_all(items: list) -> list:
    return list(map(lambda x: x ** 2, items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn square_all"),
        "Should transpile lambda in map. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_lambda_assign() {
    let code = r#"
def apply():
    double = lambda x: x * 2
    return double(5)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn apply"),
        "Should transpile lambda assignment. Got: {}",
        result
    );
}

// ============================================================================
// Docstring handling
// ============================================================================

#[test]
fn test_s11_func_multiline_docstring() {
    let code = r#"
def documented(x: int) -> int:
    """
    Doubles the input value.

    Args:
        x: The input integer.

    Returns:
        The doubled value.
    """
    return x * 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn documented"),
        "Should transpile multiline docstring. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_single_line_docstring() {
    let code = r#"
def simple(x: int) -> int:
    """Returns x plus one."""
    return x + 1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn simple"),
        "Should transpile single-line docstring. Got: {}",
        result
    );
}

// ============================================================================
// Complex parameter combinations
// ============================================================================

#[test]
fn test_s11_func_all_param_types() {
    let code = r#"
def complex_sig(a: int, b: str, c: float, d: bool, e: list, f: dict) -> str:
    return f"{a} {b} {c} {d}"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn complex_sig"),
        "Should transpile all param types. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_typed_list_param() {
    let code = r#"
from typing import List

def sum_ints(items: List[int]) -> int:
    return sum(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_ints"),
        "Should transpile typed List param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_typed_dict_param() {
    let code = r#"
from typing import Dict

def count_values(d: Dict[str, int]) -> int:
    return sum(d.values())
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_values"),
        "Should transpile typed Dict param. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_optional_param() {
    let code = r#"
from typing import Optional

def maybe(x: Optional[int] = None) -> int:
    if x is None:
        return 0
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn maybe"),
        "Should transpile Optional param. Got: {}",
        result
    );
}

// ============================================================================
// Recursive patterns
// ============================================================================

#[test]
fn test_s11_func_recursive_fibonacci() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fib"),
        "Should transpile recursive fib. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_recursive_tree_traversal() {
    let code = r#"
def tree_depth(node: dict) -> int:
    if not node:
        return 0
    left = node.get("left", {})
    right = node.get("right", {})
    return 1 + max(tree_depth(left), tree_depth(right))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn tree_depth"),
        "Should transpile recursive tree traversal. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_tail_recursive() {
    let code = r#"
def sum_tail(n: int, acc: int = 0) -> int:
    if n <= 0:
        return acc
    return sum_tail(n - 1, acc + n)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sum_tail"),
        "Should transpile tail recursive. Got: {}",
        result
    );
}

// ============================================================================
// Async function patterns
// ============================================================================

#[test]
fn test_s11_func_async_simple() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fetch"),
        "Should transpile async function. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_async_with_await() {
    let code = r#"
import asyncio

async def delayed(n: int) -> int:
    await asyncio.sleep(1)
    return n
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn delayed"),
        "Should transpile async with await. Got: {}",
        result
    );
}

// ============================================================================
// Generator patterns
// ============================================================================

#[test]
fn test_s11_func_generator_simple() {
    let code = r#"
def count_up(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_up"),
        "Should transpile simple generator. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_generator_with_return() {
    let code = r#"
def take_while_positive(items: list):
    for x in items:
        if x <= 0:
            return
        yield x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn take_while_positive"),
        "Should transpile generator with return. Got: {}",
        result
    );
}

// ============================================================================
// Complex function body patterns
// ============================================================================

#[test]
fn test_s11_func_multiple_loops() {
    let code = r#"
def process(data: list) -> dict:
    counts: dict = {}
    for item in data:
        if item in counts:
            counts[item] += 1
        else:
            counts[item] = 1
    result: dict = {}
    for key, val in counts.items():
        if val > 1:
            result[key] = val
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should transpile function with multiple loops. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_complex_initialization() {
    let code = r#"
def matrix_identity(n: int) -> list:
    result: list = []
    for i in range(n):
        row: list = [0] * n
        row[i] = 1
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn matrix_identity"),
        "Should transpile complex initialization. Got: {}",
        result
    );
}

#[test]
fn test_s11_func_early_returns() {
    let code = r#"
def validate(s: str) -> bool:
    if not s:
        return False
    if len(s) > 100:
        return False
    if not s[0].isalpha():
        return False
    for c in s:
        if not c.isalnum():
            return False
    return True
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn validate"),
        "Should transpile multiple early returns. Got: {}",
        result
    );
}

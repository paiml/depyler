//! Session 11 Batch 2: while True→loop, generator/yield, TYPE_CHECKING
//!
//! Targets:
//! - stmt_gen.rs:1395 while True→loop conversion
//! - stmt_gen.rs:2273 TYPE_CHECKING elision
//! - expr_gen.rs:10791 Yield/Await HirExpr variants
//! - stmt_gen.rs:3204 generator iterator state
//! - stmt_gen_complex.rs:1843 recursive nested function detection

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

// ===== while True → loop conversion =====

#[test]
fn test_s11b2_while_true_basic_loop() {
    let code = r#"
def run_forever():
    while True:
        pass
"#;
    let result = transpile(code);
    assert!(result.contains("loop"), "while True should become loop. Got: {}", result);
}

#[test]
fn test_s11b2_while_true_break() {
    let code = r#"
def count_to_ten() -> int:
    n = 0
    while True:
        n += 1
        if n >= 10:
            break
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("loop"), "Got: {}", result);
    assert!(result.contains("break"), "Got: {}", result);
}

#[test]
fn test_s11b2_while_true_return() {
    let code = r#"
def find_positive(items: list) -> int:
    idx = 0
    while True:
        if items[idx] > 0:
            return items[idx]
        idx += 1
"#;
    let result = transpile(code);
    assert!(result.contains("loop"), "Got: {}", result);
}

#[test]
fn test_s11b2_while_true_continue() {
    let code = r#"
def skip_odds() -> list:
    result = []
    i = 0
    while True:
        i += 1
        if i > 20:
            break
        if i % 2 != 0:
            continue
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("loop"), "Got: {}", result);
    assert!(result.contains("continue"), "Got: {}", result);
}

#[test]
fn test_s11b2_while_true_nested() {
    let code = r#"
def nested_loops() -> int:
    total = 0
    while True:
        i = 0
        while True:
            i += 1
            total += 1
            if i >= 5:
                break
        if total >= 25:
            break
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("loop"), "Got: {}", result);
}

// ===== TYPE_CHECKING elision =====

#[test]
fn test_s11b2_type_checking_elision() {
    let code = r#"
from typing import TYPE_CHECKING

def add(a: int, b: int) -> int:
    if TYPE_CHECKING:
        pass
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn add"), "Got: {}", result);
}

#[test]
fn test_s11b2_type_checking_constant() {
    let code = r#"
TYPE_CHECKING = False

def multiply(a: int, b: int) -> int:
    if TYPE_CHECKING:
        pass
    return a * b
"#;
    let result = transpile(code);
    assert!(result.contains("fn multiply"), "Got: {}", result);
}

// ===== Generator/yield patterns =====

#[test]
fn test_s11b2_simple_yield() {
    let code = r#"
def counter(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    let result = transpile(code);
    assert!(result.contains("counter") || result.contains("Counter"), "Got: {}", result);
}

#[test]
fn test_s11b2_fibonacci_generator() {
    let code = r#"
def fibonacci():
    a = 0
    b = 1
    while True:
        yield a
        a, b = b, a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fibonacci") || result.contains("Fibonacci"), "Got: {}", result);
}

#[test]
fn test_s11b2_yield_in_for() {
    let code = r#"
def squares(n: int):
    for i in range(n):
        yield i * i
"#;
    let result = transpile(code);
    assert!(result.contains("squares") || result.contains("Squares"), "Got: {}", result);
}

#[test]
fn test_s11b2_yield_with_condition() {
    let code = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    let result = transpile(code);
    assert!(result.contains("even_numbers") || result.contains("EvenNumbers"), "Got: {}", result);
}

#[test]
fn test_s11b2_generator_with_return() {
    let code = r#"
def limited(n: int):
    for i in range(100):
        if i >= n:
            return
        yield i
"#;
    let result = transpile(code);
    assert!(result.contains("limited") || result.contains("Limited"), "Got: {}", result);
}

// ===== Recursive nested functions =====

#[test]
fn test_s11b2_recursive_inner_fn() {
    let code = r#"
def outer(n: int) -> int:
    def inner(x: int) -> int:
        if x <= 0:
            return 0
        return x + inner(x - 1)
    return inner(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

#[test]
fn test_s11b2_closure_captures() {
    let code = r#"
def make_counter(start: int):
    count = start
    def increment() -> int:
        return count + 1
    return increment
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_counter"), "Got: {}", result);
}

#[test]
fn test_s11b2_multiple_inner_fns() {
    let code = r#"
def pipeline(x: int) -> int:
    def double(n: int) -> int:
        return n * 2
    def add_one(n: int) -> int:
        return n + 1
    return add_one(double(x))
"#;
    let result = transpile(code);
    assert!(result.contains("fn pipeline"), "Got: {}", result);
}

// ===== Async/await =====

#[test]
fn test_s11b2_async_basic() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(result.contains("fetch"), "Got: {}", result);
}

#[test]
fn test_s11b2_async_with_await() {
    let code = r#"
import asyncio

async def delayed_add(a: int, b: int) -> int:
    await asyncio.sleep(1)
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("delayed_add"), "Got: {}", result);
}

// ===== Floor div in try/except =====

#[test]
fn test_s11b2_floor_div_try_except() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_div"), "Got: {}", result);
}

#[test]
fn test_s11b2_try_value_error() {
    let code = r#"
def safe_int(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_int"), "Got: {}", result);
}

#[test]
fn test_s11b2_try_generic_except() {
    let code = r#"
def safe_op(x: int, y: int) -> int:
    try:
        return x // y
    except Exception:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_op"), "Got: {}", result);
}

#[test]
fn test_s11b2_try_finally() {
    let code = r#"
def cleanup(x: int) -> int:
    result = 0
    try:
        result = x * 2
    finally:
        result += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn cleanup"), "Got: {}", result);
}

// ===== Truthiness patterns =====

#[test]
fn test_s11b2_while_list_truthiness() {
    let code = r#"
def drain_list(items: list) -> int:
    total = 0
    while items:
        total += items.pop()
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn drain_list"), "Got: {}", result);
}

#[test]
fn test_s11b2_if_string_truthiness() {
    let code = r#"
def greet(name: str) -> str:
    if name:
        return "Hello " + name
    return "Hello stranger"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s11b2_if_dict_truthiness() {
    let code = r#"
def is_empty(d: dict) -> bool:
    if d:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_empty"), "Got: {}", result);
}

#[test]
fn test_s11b2_or_default_string() {
    let code = r#"
def name_or_default(name: str) -> str:
    return name or "unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("fn name_or_default"), "Got: {}", result);
}

#[test]
fn test_s11b2_and_short_circuit() {
    let code = r#"
def safe_first(items: list) -> bool:
    return bool(items) and items[0] > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_first"), "Got: {}", result);
}

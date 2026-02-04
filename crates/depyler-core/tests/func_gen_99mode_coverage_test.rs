//! Coverage tests for func_gen.rs
//!
//! DEPYLER-99MODE-001: Targets func_gen.rs (83.09% -> 90%+)
//! Covers: decorator patterns, async/await, generators, closures,
//! type inference, variable usage tracking, borrowing strategies,
//! argparse integration, string return detection.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Decorator patterns
// ============================================================================

#[test]
fn test_property_decorator() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self._radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self._radius * self._radius
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_staticmethod_decorator() {
    let code = r#"
class MathHelper:
    @staticmethod
    def gcd(a: int, b: int) -> int:
        while b:
            a, b = b, a % b
        return a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_classmethod_decorator() {
    let code = r#"
class Factory:
    count = 0

    @classmethod
    def create(cls) -> int:
        cls.count += 1
        return cls.count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Async function generation
// ============================================================================

#[test]
fn test_async_function_simple() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return url
"#;
    let rust = transpile(code);
    assert!(rust.contains("async"));
}

#[test]
fn test_async_function_with_computation() {
    let code = r#"
async def compute(x: int) -> int:
    result = x * 2
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_async_function_void() {
    let code = r#"
async def log_event(message: str):
    print(message)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator function generation
// ============================================================================

#[test]
fn test_generator_simple_range() {
    let code = r#"
def numbers(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_condition() {
    let code = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_infinite() {
    let code = r#"
def counter():
    n = 0
    while True:
        yield n
        n += 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_with_state() {
    let code = r#"
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Return type inference
// ============================================================================

#[test]
fn test_infer_return_int() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x + y
"#;
    let rust = transpile(code);
    assert!(rust.contains("i64"));
}

#[test]
fn test_infer_return_float() {
    let code = r#"
def f(x: float, y: float) -> float:
    return x * y
"#;
    let rust = transpile(code);
    assert!(rust.contains("f64"));
}

#[test]
fn test_infer_return_str() {
    let code = r#"
def f(name: str) -> str:
    return "Hello " + name
"#;
    let rust = transpile(code);
    assert!(rust.contains("String"));
}

#[test]
fn test_infer_return_bool() {
    let code = r#"
def f(x: int) -> bool:
    return x > 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("bool"));
}

#[test]
fn test_infer_return_list() {
    let code = r#"
from typing import List
def f() -> List[int]:
    return [1, 2, 3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("Vec"));
}

#[test]
fn test_infer_return_dict() {
    let code = r#"
from typing import Dict
def f() -> Dict[str, int]:
    return {"a": 1}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infer_return_optional() {
    let code = r#"
from typing import Optional
def f(items: list) -> Optional[int]:
    if len(items) > 0:
        return items[0]
    return None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_infer_return_tuple() {
    let code = r#"
def f(x: int) -> tuple:
    return (x, x * 2, x * 3)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function type inference
// ============================================================================

#[test]
fn test_nested_fn_returning_int() {
    let code = r#"
def outer(x: int) -> int:
    def square(n: int) -> int:
        return n * n
    return square(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_capturing_params() {
    let code = r#"
def make_multiplier(factor: int):
    def multiply(x: int) -> int:
        return x * factor
    return multiply
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_fn_with_closure() {
    let code = r#"
def accumulator(initial: int):
    total = initial
    def add(value: int) -> int:
        return total + value
    return add
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Parameter mutability detection
// ============================================================================

#[test]
fn test_mutable_list_param() {
    let code = r#"
def modify(items: list):
    items.append(1)
    items.append(2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutable_dict_param() {
    let code = r#"
def modify(d: dict):
    d["new_key"] = "new_value"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_immutable_int_param() {
    let code = r#"
def double(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_immutable_str_param() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello, " + name
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String return detection patterns
// ============================================================================

#[test]
fn test_string_return_via_fstring() {
    let code = r#"
def format_name(first: str, last: str) -> str:
    return f"{first} {last}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_via_concat() {
    let code = r#"
def build_path(dir: str, file: str) -> str:
    return dir + "/" + file
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_via_method() {
    let code = r#"
def clean(text: str) -> str:
    return text.strip().lower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_via_join() {
    let code = r#"
def csv_line(items: list) -> str:
    return ",".join(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_conditional() {
    let code = r#"
def status(code: int) -> str:
    if code == 200:
        return "OK"
    elif code == 404:
        return "Not Found"
    elif code == 500:
        return "Internal Server Error"
    return "Unknown"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_return_in_try() {
    let code = r#"
def safe_read(path: str) -> str:
    try:
        with open(path) as f:
            return f.read()
    except Exception:
        return ""
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable usage tracking
// ============================================================================

#[test]
fn test_var_used_in_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_after_if() {
    let code = r#"
def f(flag: bool) -> int:
    if flag:
        value = 42
    else:
        value = 0
    return value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_after_try() {
    let code = r#"
def f(s: str) -> int:
    try:
        n = int(s)
    except ValueError:
        n = -1
    return n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_reassigned_in_loop() {
    let code = r#"
def f(items: list) -> int:
    result = 0
    for item in items:
        result = item
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_used_in_nested_function() {
    let code = r#"
def outer() -> int:
    x = 10
    def inner() -> int:
        return x + 1
    return inner()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Default parameter values
// ============================================================================

#[test]
fn test_default_int_param() {
    let code = r#"
def f(x: int = 0) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_str_param() {
    let code = r#"
def f(name: str = "world") -> str:
    return "Hello, " + name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_float_param() {
    let code = r#"
def f(rate: float = 0.05) -> float:
    return 100.0 * rate
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_bool_param() {
    let code = r#"
def f(verbose: bool = False) -> str:
    if verbose:
        return "verbose mode"
    return "quiet mode"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_none_param() {
    let code = r#"
from typing import Optional
def f(value: Optional[int] = None) -> int:
    if value is None:
        return 0
    return value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_multiple_defaults() {
    let code = r#"
def connect(host: str = "localhost", port: int = 5432, ssl: bool = True) -> str:
    return host
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Star args
// ============================================================================

#[test]
fn test_star_args_int() {
    let code = r#"
def sum_all(*args: int) -> int:
    return sum(args)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_star_args_str() {
    let code = r#"
def concat_all(*args: str) -> str:
    return " ".join(args)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex function signatures
// ============================================================================

#[test]
fn test_no_return_type() {
    let code = r#"
def f(x: int):
    print(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_no_params_no_return() {
    let code = r#"
def f():
    print("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_many_params() {
    let code = r#"
def f(a: int, b: int, c: int, d: int, e: int) -> int:
    return a + b + c + d + e
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Numeric type coercion in function bodies
// ============================================================================

#[test]
fn test_int_to_float_coercion() {
    let code = r#"
def f(x: int) -> float:
    return x * 0.5
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_division_returns_float() {
    let code = r#"
def average(total: int, count: int) -> float:
    return total / count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mixed_int_float_params() {
    let code = r#"
def weighted(value: int, weight: float) -> float:
    return value * weight
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex body patterns
// ============================================================================

#[test]
fn test_multiple_return_paths() {
    let code = r#"
def classify(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    return "F"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_early_return_guard() {
    let code = r#"
def process(data: list) -> int:
    if not data:
        return 0
    if len(data) == 1:
        return data[0]
    total = 0
    for item in data:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_with_mutation() {
    let code = r#"
def collatz(n: int) -> int:
    steps = 0
    while n != 1:
        if n % 2 == 0:
            n = n // 2
        else:
            n = 3 * n + 1
        steps += 1
    return steps
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_loops_with_break() {
    let code = r#"
def find_pair(items: list, target: int) -> tuple:
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            if items[i] + items[j] == target:
                return (i, j)
    return (-1, -1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_list_building() {
    let code = r#"
from typing import List
def partition(items: List[int], pivot: int) -> tuple:
    less: List[int] = []
    equal: List[int] = []
    greater: List[int] = []
    for item in items:
        if item < pivot:
            less.append(item)
        elif item == pivot:
            equal.append(item)
        else:
            greater.append(item)
    return (less, equal, greater)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_function_calling_function() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def sum_of_squares(n: int) -> int:
    total = 0
    for i in range(1, n + 1):
        total += square(i)
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn square"));
    assert!(rust.contains("fn sum_of_squares"));
}

#[test]
fn test_recursive_with_accumulator() {
    let code = r#"
def reverse_list(items: list) -> list:
    if len(items) <= 1:
        return items
    return reverse_list(items[1:]) + [items[0]]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pipeline integration patterns
// ============================================================================

#[test]
fn test_pipeline_class_and_function() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def distance(p1: Point, p2: Point) -> float:
    dx = p1.x - p2.x
    dy = p1.y - p2.y
    return (dx * dx + dy * dy) ** 0.5
"#;
    let rust = transpile(code);
    assert!(rust.contains("struct Point"));
    assert!(rust.contains("fn distance"));
}

#[test]
fn test_pipeline_constants_and_functions() {
    let code = r#"
MAX_SIZE = 100
MIN_SIZE = 1

def clamp(value: int) -> int:
    if value > MAX_SIZE:
        return MAX_SIZE
    if value < MIN_SIZE:
        return MIN_SIZE
    return value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pipeline_enum_and_function() {
    let code = r#"
from enum import Enum

class Status(Enum):
    OK = 200
    NOT_FOUND = 404
    ERROR = 500

def is_success(code: int) -> bool:
    return code == 200
"#;
    assert!(transpile_ok(code));
}

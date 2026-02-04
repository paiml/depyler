//! Coverage tests for func_gen_inference.rs and lib.rs
//!
//! DEPYLER-99MODE-001: Targets func_gen_inference.rs (81.22%) and
//! lib.rs (74.41%) coverage improvements.

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
// Nested function type inference
// ============================================================================

#[test]
fn test_nested_function_returns_closure() {
    let code = r#"
def make_multiplier(n: int):
    def multiply(x: int) -> int:
        return x * n
    return multiply
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_function_returns_value() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_function_captures_multiple() {
    let code = r#"
def make_range_checker(low: int, high: int):
    def check(x: int) -> bool:
        return low <= x <= high
    return check
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_factory_function() {
    let code = r#"
def factory(data: list):
    def processor(idx: int) -> int:
        return data[idx]
    return processor
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Return type inference
// ============================================================================

#[test]
fn test_return_type_conditional() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    elif x < 0:
        return -x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_type_str_format() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_type_list() {
    let code = r#"
from typing import List
def doubles(items: List[int]) -> List[int]:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_type_dict() {
    let code = r#"
from typing import Dict
def count_items(items: list) -> Dict[str, int]:
    counts = {}
    for item in items:
        if item in counts:
            counts[item] += 1
        else:
            counts[item] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_type_tuple() {
    let code = r#"
def min_max(items: list) -> tuple:
    return (min(items), max(items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Main function special cases
// ============================================================================

#[test]
fn test_main_function_simple() {
    let code = r#"
def main():
    print("Hello, World!")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn main"));
}

#[test]
fn test_main_with_return_int() {
    let code = r#"
def main() -> int:
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_main_with_sys_argv() {
    let code = r#"
import sys
def main():
    args = sys.argv
    print(len(args))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Generator functions
// ============================================================================

#[test]
fn test_generator_simple() {
    let code = r#"
def count_up(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_generator_fibonacci() {
    let code = r#"
def fibonacci():
    a = 0
    b = 1
    while True:
        yield a
        a, b = b, a + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Parameter mutability
// ============================================================================

#[test]
fn test_mutable_list_param() {
    let code = r#"
def append_value(items: list, value: int) -> list:
    items.append(value)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutable_dict_param() {
    let code = r#"
def update_dict(d: dict, key: str, value: int) -> dict:
    d[key] = value
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optional_param_none_default() {
    let code = r#"
from typing import Optional
def process(value: Optional[int] = None) -> int:
    if value is None:
        return 0
    return value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optional_dict_param() {
    let code = r#"
from typing import Optional, Dict
def process(config: Optional[Dict[str, int]] = None) -> int:
    if config is None:
        return 0
    return config.get("key", 0)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pipeline features
// ============================================================================

#[test]
fn test_pipeline_simple_function() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn add"));
    assert!(rust.contains("i64"));
}

#[test]
fn test_pipeline_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn add"));
    assert!(rust.contains("fn sub"));
    assert!(rust.contains("fn mul"));
}

#[test]
fn test_pipeline_class_generation() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    let rust = transpile(code);
    assert!(rust.contains("struct Point"));
}

#[test]
fn test_pipeline_dataclass() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Config:
    name: str
    value: int
    enabled: bool
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pipeline_dataclass_with_defaults() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Config:
    name: str = "default"
    value: int = 0
    enabled: bool = True
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pipeline_enum() {
    let code = r#"
from enum import Enum

class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pipeline_module_constants() {
    let code = r#"
MAX_SIZE = 100
DEFAULT_NAME = "unknown"

def get_max() -> int:
    return MAX_SIZE
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_class_hierarchy() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return ""

class Dog(Animal):
    def speak(self) -> str:
        return "Woof!"

class Cat(Animal):
    def speak(self) -> str:
        return "Meow!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_recursive_with_memoization() {
    let code = r#"
def fibonacci(n: int) -> int:
    memo = {}
    def fib(k: int) -> int:
        if k in memo:
            return memo[k]
        if k <= 1:
            return k
        result = fib(k - 1) + fib(k - 2)
        memo[k] = result
        return result
    return fib(n)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_protocol_class() {
    let code = r#"
from typing import Protocol

class Comparable(Protocol):
    def compare(self, other: int) -> int:
        ...
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_walrus_in_condition() {
    let code = r#"
def first_match(items: list, threshold: int) -> int:
    for item in items:
        if (val := item * 2) > threshold:
            return val
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_parameter_values() {
    let code = r#"
def greet(name: str = "World", greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_default_values() {
    let code = r#"
def create_config(
    name: str = "default",
    max_retries: int = 3,
    timeout: float = 30.0,
    verbose: bool = False
) -> str:
    return name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_star_args() {
    let code = r#"
def sum_all(*args: int) -> int:
    return sum(args)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_empty_list() {
    let code = r#"
def empty() -> list:
    return []
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_empty_dict() {
    let code = r#"
def empty() -> dict:
    return {}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_comprehension() {
    let code = r#"
def get_squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_async_function() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    assert!(transpile_ok(code));
}

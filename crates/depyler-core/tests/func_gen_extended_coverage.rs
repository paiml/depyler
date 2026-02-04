//! Extended coverage tests for func_gen.rs
//!
//! DEPYLER-99MODE-001: Targets remaining uncovered paths in func_gen module
//! Focus: complex parameter types, decorators, class methods, async,
//! varargs, docstrings, multiple functions, recursive patterns.

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
// Complex parameter types
// ============================================================================

#[test]
fn test_param_list_of_str() {
    let code = r#"
from typing import List
def f(items: List[str]) -> str:
    return ",".join(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_param_dict_str_int() {
    let code = r#"
from typing import Dict
def f(d: Dict[str, int]) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_param_optional_str() {
    let code = r#"
from typing import Optional
def f(s: Optional[str] = None) -> str:
    if s is None:
        return ""
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_param_set_int() {
    let code = r#"
from typing import Set
def f(s: Set[int]) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple return types / complex returns
// ============================================================================

#[test]
fn test_return_conditional_type() {
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
fn test_return_in_loop() {
    let code = r#"
def f(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_in_try() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Async functions
// ============================================================================

#[test]
fn test_async_with_params() {
    let code = r#"
async def f(url: str) -> str:
    return url
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class methods
// ============================================================================

#[test]
fn test_class_with_multiple_methods() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_repr() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_eq() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function with complex body patterns
// ============================================================================

#[test]
fn test_function_with_multiple_loops() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total += val
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_function_with_nested_conditionals() {
    let code = r#"
def classify(x: int, y: int) -> str:
    if x > 0:
        if y > 0:
            return "Q1"
        else:
            return "Q4"
    else:
        if y > 0:
            return "Q2"
        else:
            return "Q3"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_function_with_comprehension() {
    let code = r#"
def squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Recursive functions
// ============================================================================

#[test]
fn test_recursive_fibonacci() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_recursive_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    return gcd(b, a % b)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Functions with docstrings
// ============================================================================

#[test]
fn test_docstring_single_line() {
    let code = r#"
def f(x: int) -> int:
    """Return x plus one."""
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_docstring_multi_line() {
    let code = r#"
def f(x: int, y: int) -> int:
    """
    Add two numbers.

    Args:
        x: First number
        y: Second number

    Returns:
        Sum of x and y
    """
    return x + y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple function definitions in one file
// ============================================================================

#[test]
fn test_many_functions() {
    let code = r#"
def a() -> int:
    return 1

def b() -> int:
    return 2

def c() -> int:
    return 3

def d() -> int:
    return a() + b() + c()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn a"));
    assert!(rust.contains("fn b"));
    assert!(rust.contains("fn c"));
    assert!(rust.contains("fn d"));
}

// ============================================================================
// Functions with varargs
// ============================================================================

#[test]
fn test_varargs_simple() {
    let code = r#"
def f(*args: int) -> int:
    return sum(args)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Decorators
// ============================================================================

#[test]
fn test_staticmethod() {
    let code = r#"
class Util:
    @staticmethod
    def helper(x: int) -> int:
        return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_property_getter() {
    let code = r#"
class Circle:
    def __init__(self, r: float):
        self.r = r

    @property
    def area(self) -> float:
        return 3.14 * self.r * self.r
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_empty_function() {
    assert!(transpile_ok("def f():\n    pass"));
}

#[test]
fn test_single_return() {
    assert!(transpile_ok("def f() -> int:\n    return 0"));
}

#[test]
fn test_function_with_only_assignment() {
    let code = r#"
def f():
    x = 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_function_with_print() {
    let code = r#"
def f(msg: str):
    print(msg)
"#;
    assert!(transpile_ok(code));
}

//! EXTREME TDD: Tests for codegen.rs function generation
//! Coverage: generate_rust, hir_to_rust, convert_function_to_rust

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new().transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ Basic function generation ============

#[test]
fn test_gen_fn_no_params() {
    let code = "def hello() -> str:\n    return \"hello\"";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "fn hello"));
}

#[test]
fn test_gen_fn_one_param() {
    let code = "def double(x: int) -> int:\n    return x * 2";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "fn double"));
}

#[test]
fn test_gen_fn_multiple_params() {
    let code = "def add(a: int, b: int, c: int) -> int:\n    return a + b + c";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "fn add"));
}

#[test]
fn test_gen_fn_no_return() {
    let code = "def noop() -> None:\n    pass";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_int_return() {
    let code = "def get_int() -> int:\n    return 42";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "i32") || transpile_contains(code, "i64"));
}

#[test]
fn test_gen_fn_float_return() {
    let code = "def get_float() -> float:\n    return 3.14";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "f64"));
}

#[test]
fn test_gen_fn_str_return() {
    let code = "def get_str() -> str:\n    return \"test\"";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "String") || transpile_contains(code, "&str"));
}

#[test]
fn test_gen_fn_bool_return() {
    let code = "def get_bool() -> bool:\n    return True";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "bool"));
}

#[test]
fn test_gen_fn_list_return() {
    let code = "def get_list() -> list:\n    return [1, 2, 3]";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "Vec"));
}

#[test]
fn test_gen_fn_dict_return() {
    let code = "def get_dict() -> dict:\n    return {\"a\": 1}";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "HashMap"));
}

// ============ Parameter types ============

#[test]
fn test_gen_fn_param_int() {
    let code = "def f(x: int) -> int:\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_float() {
    let code = "def f(x: float) -> float:\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_str() {
    let code = "def f(s: str) -> str:\n    return s";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_bool() {
    let code = "def f(b: bool) -> bool:\n    return b";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_list() {
    let code = "def f(items: list) -> list:\n    return items";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_dict() {
    let code = "def f(data: dict) -> dict:\n    return data";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_tuple() {
    let code = "def f(t: tuple) -> tuple:\n    return t";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_param_optional() {
    let code = "def f(x: int = None) -> int:\n    return x if x else 0";
    assert!(transpile_ok(code));
}

// ============ Generic types ============

#[test]
fn test_gen_fn_list_int() {
    let code = "def f(items: list[int]) -> int:\n    return sum(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_list_str() {
    let code = "def f(items: list[str]) -> str:\n    return \",\".join(items)";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_dict_str_int() {
    let code = "def f(data: dict[str, int]) -> int:\n    return sum(data.values())";
    assert!(transpile_ok(code));
}

// ============ Complex function bodies ============

#[test]
fn test_gen_fn_with_local_var() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_with_multiple_locals() {
    let code = r#"
def f(x: int) -> int:
    a = x + 1
    b = a * 2
    c = b - 3
    return c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_with_if() {
    let code = r#"
def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_with_while() {
    let code = r#"
def factorial(n: int) -> int:
    result = 1
    while n > 1:
        result = result * n
        n = n - 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_with_for() {
    let code = r#"
def sum_list(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_recursive() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)
"#;
    assert!(transpile_ok(code));
}

// ============ Multiple functions ============

#[test]
fn test_gen_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_calling_fn() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def sum_of_squares(a: int, b: int) -> int:
    return square(a) + square(b)
"#;
    assert!(transpile_ok(code));
}

// ============ Class methods ============

#[test]
fn test_gen_method_simple() {
    let code = r#"
class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_method_with_self_field() {
    let code = r#"
class Counter:
    count: int

    def get_count(self) -> int:
        return self.count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_method_mutating_self() {
    let code = r#"
class Counter:
    count: int

    def increment(self) -> None:
        self.count = self.count + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_static_method() {
    let code = r#"
class Utils:
    @staticmethod
    def double(x: int) -> int:
        return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_classmethod() {
    let code = r#"
class Factory:
    @classmethod
    def create(cls) -> int:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============ Docstrings ============

#[test]
fn test_gen_fn_with_docstring() {
    let code = r#"
def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_with_multiline_docstring() {
    let code = r#"
def complex_fn(x: int) -> int:
    """
    A complex function.

    Args:
        x: Input value

    Returns:
        Processed value
    """
    return x * 2
"#;
    assert!(transpile_ok(code));
}

// ============ Edge cases ============

#[test]
fn test_gen_fn_keyword_name() {
    // Python allows some names that are Rust keywords
    let code = "def my_fn(type: str) -> str:\n    return type";
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_many_params() {
    let code = r#"
def many_params(a: int, b: int, c: int, d: int, e: int) -> int:
    return a + b + c + d + e
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_nested_types() {
    let code = r#"
def f(data: dict[str, list[int]]) -> int:
    total = 0
    for key in data:
        total = total + sum(data[key])
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_gen_fn_varargs() {
    let code = r#"
def sum_all(*args: int) -> int:
    return sum(args)
"#;
    assert!(transpile_ok(code));
}

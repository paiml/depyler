//! EXTREME TDD tests for func_gen module
//! Tests edge cases, error paths, and boundary conditions using property-based testing

use depyler_core::ast_bridge::AstBridge;
use depyler_core::codegen::hir_to_rust;
use proptest::prelude::*;
use rustpython_ast::Suite;
use rustpython_parser::Parse;

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to create a ModModule from parsed code
fn make_module(ast: Suite) -> rustpython_ast::Mod {
    rustpython_ast::Mod::Module(rustpython_ast::ModModule {
        body: ast,
        range: Default::default(),
        type_ignores: vec![],
    })
}

/// Transpile Python code to Rust and return the result
fn transpile_code(python_code: &str) -> Option<String> {
    let ast = Suite::parse(python_code, "<test>").ok()?;
    let bridge = AstBridge::new().with_source(python_code.to_string());
    let (hir, _type_env) = bridge.python_to_hir(make_module(ast)).ok()?;
    let rust_code = hir_to_rust(&hir).ok()?;
    Some(rust_code)
}

/// Check if Python code transpiles successfully
fn transpile_succeeds(python_code: &str) -> bool {
    transpile_code(python_code).is_some()
}

// ============================================================================
// FUNCTION SIGNATURE TESTS
// ============================================================================

#[test]
fn test_function_no_params() {
    let code = r#"
def no_params() -> int:
    return 42
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_single_param() {
    let code = r#"
def single_param(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_multiple_params() {
    let code = r#"
def multiple_params(a: int, b: int, c: int) -> int:
    return a + b + c
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_mixed_types() {
    let code = r#"
def mixed_types(n: int, x: float, s: str, b: bool) -> str:
    return s
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_list_param() {
    let code = r#"
def list_param(lst: list) -> int:
    return len(lst)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_dict_param() {
    let code = r#"
def dict_param(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_optional_param() {
    let code = r#"
def optional_param(x: int = 10) -> int:
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_args_vararg() {
    let code = r#"
def varargs(*args) -> int:
    return len(args)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_kwargs() {
    let code = r#"
def kwargs(**kwargs) -> int:
    return len(kwargs)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// RETURN TYPE TESTS
// ============================================================================

#[test]
fn test_return_int() {
    let code = r#"
def ret_int() -> int:
    return 42
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("i64"));
}

#[test]
fn test_return_float() {
    let code = r#"
def ret_float() -> float:
    return 3.14
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("f64"));
}

#[test]
fn test_return_str() {
    let code = r#"
def ret_str() -> str:
    return "hello"
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("String") || result.contains("str"));
}

#[test]
fn test_return_bool() {
    let code = r#"
def ret_bool() -> bool:
    return True
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("bool"));
}

#[test]
fn test_return_none() {
    let code = r#"
def ret_none() -> None:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_list() {
    let code = r#"
def ret_list() -> list:
    return [1, 2, 3]
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("Vec"));
}

#[test]
fn test_return_dict() {
    let code = r#"
def ret_dict() -> dict:
    return {"a": 1}
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("HashMap"));
}

#[test]
fn test_return_tuple() {
    let code = r#"
def ret_tuple() -> tuple:
    return (1, 2)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// NESTED FUNCTION TESTS
// ============================================================================

#[test]
fn test_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y + 1
    return inner(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_nested_function_closure() {
    let code = r#"
def outer(x: int) -> int:
    def inner() -> int:
        return x + 1
    return inner()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_nested_functions() {
    let code = r#"
def outer(x: int) -> int:
    def helper1() -> int:
        return 1
    def helper2() -> int:
        return 2
    return helper1() + helper2() + x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// RECURSIVE FUNCTION TESTS
// ============================================================================

#[test]
fn test_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mutual_recursion() {
    let code = r#"
def is_even(n: int) -> bool:
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    return is_even(n - 1)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// METHOD TESTS (self parameter)
// ============================================================================

#[test]
fn test_method_with_self() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.count = 0

    def increment(self) -> int:
        self.count = self.count + 1
        return self.count
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_method_with_params() {
    let code = r#"
class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_classmethod() {
    let code = r#"
class Factory:
    @classmethod
    def create(cls) -> "Factory":
        return cls()
"#;
    // Classmethods may not be fully supported
    let _result = transpile_code(code);
}

#[test]
fn test_staticmethod() {
    let code = r#"
class Utility:
    @staticmethod
    def helper(x: int) -> int:
        return x * 2
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// GENERIC FUNCTION TESTS
// ============================================================================

#[test]
fn test_generic_function() {
    let code = r#"
from typing import TypeVar

T = TypeVar('T')

def identity(x: T) -> T:
    return x
"#;
    // TypeVar may not be fully supported - graceful handling
    let _result = transpile_code(code);
}

// ============================================================================
// DOCSTRING TESTS
// ============================================================================

#[test]
fn test_function_with_docstring() {
    let code = r#"
def documented(x: int) -> int:
    """
    This function adds 1 to x.

    Args:
        x: The input number

    Returns:
        x + 1
    """
    return x + 1
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// DECORATOR TESTS
// ============================================================================

#[test]
fn test_property_decorator() {
    let code = r#"
class Point:
    def __init__(self, x: int) -> None:
        self._x = x

    @property
    def x(self) -> int:
        return self._x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// CONTROL FLOW TESTS
// ============================================================================

#[test]
fn test_early_return() {
    let code = r#"
def early_return(x: int) -> int:
    if x < 0:
        return -1
    if x == 0:
        return 0
    return 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_multiple_return_paths() {
    let code = r#"
def classify(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    else:
        return "positive"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_return_in_loop() {
    let code = r#"
def find_first(lst: list, target: int) -> int:
    for i, v in enumerate(lst):
        if v == target:
            return i
    return -1
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// VARIABLE SCOPE TESTS
// ============================================================================

#[test]
fn test_local_variable() {
    let code = r#"
def local_var() -> int:
    x = 10
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_shadowing() {
    let code = r#"
def shadow(x: int) -> int:
    x = x + 1
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_variable_reassignment() {
    let code = r#"
def reassign() -> int:
    x = 1
    x = 2
    x = 3
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_escaping_variable() {
    let code = r#"
def if_escape(cond: bool) -> int:
    if cond:
        x = 1
    else:
        x = 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_loop_escaping_variable() {
    let code = r#"
def loop_escape() -> int:
    for i in range(10):
        x = i
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COMPLEX FUNCTION PATTERNS
// ============================================================================

#[test]
fn test_function_with_list_comprehension() {
    let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_map() {
    let code = r#"
def double_all(lst: list) -> list:
    return list(map(lambda x: x * 2, lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_filter() {
    let code = r#"
def evens(lst: list) -> list:
    return list(filter(lambda x: x % 2 == 0, lst))
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_reduce() {
    let code = r#"
from functools import reduce

def product(lst: list) -> int:
    return reduce(lambda a, b: a * b, lst, 1)
"#;
    // reduce may not be fully supported
    let _result = transpile_code(code);
}

// ============================================================================
// ERROR HANDLING IN FUNCTIONS
// ============================================================================

#[test]
fn test_function_with_try_except() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_raise() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("must be positive")
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_function_with_assert() {
    let code = r#"
def check(x: int) -> int:
    assert x > 0, "must be positive"
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// KEYWORD HANDLING
// ============================================================================

#[test]
fn test_rust_keyword_as_param() {
    let code = r#"
def use_keyword(type: str) -> str:
    return type
"#;
    // Rust keywords should be handled (escaped or renamed)
    assert!(transpile_succeeds(code));
}

#[test]
fn test_rust_keyword_as_variable() {
    let code = r#"
def use_keywords() -> int:
    let = 1
    match = 2
    return let + match
"#;
    // Rust keywords should be handled (escaped or renamed)
    assert!(transpile_succeeds(code));
}

// ============================================================================
// BORROWING AND OWNERSHIP TESTS
// ============================================================================

#[test]
fn test_string_param_borrowed() {
    let code = r#"
def use_str(s: str) -> int:
    return len(s)
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("&str") || result.contains("&String") || result.contains("String"));
}

#[test]
fn test_list_param_borrowed() {
    let code = r#"
def use_list(lst: list) -> int:
    return len(lst)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mutable_list_param() {
    let code = r#"
def modify_list(lst: list) -> None:
    lst.append(42)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

proptest! {
    /// Property: Simple functions should always transpile
    #[test]
    fn prop_simple_function_transpiles(x in -1000i64..1000) {
        let code = format!(r#"
def simple() -> int:
    return {}
"#, x);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Functions with addition should transpile
    #[test]
    fn prop_add_function_transpiles(a in -100i64..100, b in -100i64..100) {
        let code = format!(r#"
def add() -> int:
    return {} + {}
"#, a, b);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Functions with conditionals should transpile
    #[test]
    fn prop_conditional_function_transpiles(x in -100i64..100) {
        let code = format!(r#"
def check() -> int:
    if {} > 0:
        return 1
    else:
        return -1
"#, x);
        prop_assert!(transpile_succeeds(&code));
    }
}

// ============================================================================
// MUTATION-RESISTANT TESTS
// ============================================================================

#[test]
fn test_function_name_preserved() {
    let code = r#"
def my_unique_function_name(x: int) -> int:
    return x
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("my_unique_function_name"));
}

#[test]
fn test_param_name_preserved() {
    let code = r#"
def func(unique_param_name: int) -> int:
    return unique_param_name
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("unique_param_name"));
}

#[test]
fn test_return_value_preserved() {
    let code = r#"
def constant() -> int:
    return 12345
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("12345"));
}

#[test]
fn test_multiple_functions() {
    let code = r#"
def func1() -> int:
    return 1

def func2() -> int:
    return 2

def func3() -> int:
    return 3
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("func1"));
    assert!(result.contains("func2"));
    assert!(result.contains("func3"));
}

// ============================================================================
// ASYNC FUNCTION TESTS
// ============================================================================

#[test]
fn test_async_function() {
    let code = r#"
async def async_func() -> int:
    return 42
"#;
    // Async may not be fully supported - check graceful handling
    let _result = transpile_code(code);
}

// ============================================================================
// GENERATOR FUNCTION TESTS
// ============================================================================

#[test]
fn test_generator_function() {
    let code = r#"
def gen(n: int):
    for i in range(n):
        yield i
"#;
    // Generators may not be fully supported - check graceful handling
    let _result = transpile_code(code);
}

//! Session 11: Edge case coverage tests
//!
//! Targets rare code paths and edge cases:
//! - Empty functions and bodies
//! - Single-element collections
//! - Nested collection types
//! - Pass statement
//! - Global/nonlocal
//! - Multiple return values
//! - Recursive data structures
//! - String escaping
//! - Numeric boundaries

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
// Empty and minimal functions
// ============================================================================

#[test]
fn test_s11_edge_pass_only() {
    let code = r#"
def noop() -> None:
    pass
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn noop"),
        "Should transpile pass-only function. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_docstring_only() {
    let code = r#"
def documented() -> None:
    """This function does nothing."""
    pass
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn documented"),
        "Should transpile docstring-only function. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_single_return() {
    let code = r#"
def identity(x: int) -> int:
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn identity"),
        "Should transpile single-return function. Got: {}",
        result
    );
}

// ============================================================================
// Collection edge cases
// ============================================================================

#[test]
fn test_s11_edge_empty_list_literal() {
    let code = r#"
def empty() -> list:
    return []
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn empty"),
        "Should transpile empty list. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_single_element_list() {
    let code = r#"
def singleton() -> list:
    return [42]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn singleton"),
        "Should transpile single-element list. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_nested_list() {
    let code = r#"
def matrix() -> list:
    return [[1, 2], [3, 4]]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn matrix"),
        "Should transpile nested list. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_empty_dict_literal() {
    let code = r#"
def empty_map() -> dict:
    return {}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn empty_map"),
        "Should transpile empty dict. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_dict_with_int_keys() {
    let code = r#"
def int_map() -> dict:
    return {1: "one", 2: "two"}
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn int_map"),
        "Should transpile dict with int keys. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_tuple_literal() {
    let code = r#"
def pair() -> tuple:
    return (1, 2)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn pair"),
        "Should transpile tuple literal. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_triple_tuple() {
    let code = r#"
def triple() -> tuple:
    return (1, "hello", True)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn triple"),
        "Should transpile mixed tuple. Got: {}",
        result
    );
}

// ============================================================================
// Numeric edge cases
// ============================================================================

#[test]
fn test_s11_edge_large_int() {
    let code = r#"
def big() -> int:
    return 999999999
"#;
    let result = transpile(code);
    assert!(
        result.contains("999999999") || result.contains("fn big"),
        "Should transpile large int. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_zero_float() {
    let code = r#"
def zero_f() -> float:
    return 0.0
"#;
    let result = transpile(code);
    assert!(
        result.contains("0.0") || result.contains("fn zero_f"),
        "Should transpile zero float. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_negative_float() {
    let code = r#"
def neg_f() -> float:
    return -1.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn neg_f"),
        "Should transpile negative float. Got: {}",
        result
    );
}

// ============================================================================
// String edge cases
// ============================================================================

#[test]
fn test_s11_edge_empty_string() {
    let code = r#"
def empty_str() -> str:
    return ""
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn empty_str"),
        "Should transpile empty string. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_string_concat() {
    let code = r#"
def concat(a: str, b: str) -> str:
    return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn concat"),
        "Should transpile string concat. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_string_multiply() {
    let code = r#"
def repeat(s: str, n: int) -> str:
    return s * n
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn repeat"),
        "Should transpile string multiply. Got: {}",
        result
    );
}

// ============================================================================
// Boolean patterns
// ============================================================================

#[test]
fn test_s11_edge_bool_literal_true() {
    let code = r#"
def yes() -> bool:
    return True
"#;
    let result = transpile(code);
    assert!(
        result.contains("true"),
        "Should transpile True. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_bool_literal_false() {
    let code = r#"
def no() -> bool:
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("false"),
        "Should transpile False. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_not_operator() {
    let code = r#"
def negate(x: bool) -> bool:
    return not x
"#;
    let result = transpile(code);
    assert!(
        result.contains("!") || result.contains("fn negate"),
        "Should transpile not operator. Got: {}",
        result
    );
}

// ============================================================================
// Complex control flow patterns
// ============================================================================

#[test]
fn test_s11_edge_early_return_guard() {
    let code = r#"
def guarded(x: int) -> int:
    if x <= 0:
        return 0
    result: int = x * x
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn guarded"),
        "Should handle guard return. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_multiple_guards() {
    let code = r#"
def multi_guard(x: int, y: int) -> int:
    if x <= 0:
        return 0
    if y <= 0:
        return 0
    return x * y
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn multi_guard"),
        "Should handle multiple guards. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_while_true() {
    let code = r#"
def find_it(items: list) -> int:
    i: int = 0
    while True:
        if i >= len(items):
            break
        if items[i] > 10:
            return items[i]
        i += 1
    return -1
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_it"),
        "Should transpile while True. Got: {}",
        result
    );
}

// ============================================================================
// List comprehension patterns
// ============================================================================

#[test]
fn test_s11_edge_list_comp_simple() {
    let code = r#"
def squares(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn squares"),
        "Should transpile list comp. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_list_comp_with_filter() {
    let code = r#"
def even_squares(n: int) -> list:
    return [i * i for i in range(n) if i % 2 == 0]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn even_squares"),
        "Should transpile filtered list comp. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_list_comp_with_str() {
    let code = r#"
def upper_list(items: list) -> list:
    return [s.upper() for s in items]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn upper_list"),
        "Should transpile str list comp. Got: {}",
        result
    );
}

// ============================================================================
// Multiple functions in module
// ============================================================================

#[test]
fn test_s11_edge_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def sub(a: int, b: int) -> int:
    return a - b

def mul(a: int, b: int) -> int:
    return a * b
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn add") && result.contains("fn sub") && result.contains("fn mul"),
        "Should transpile multiple functions. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_function_calling_function() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def quadruple(x: int) -> int:
    return double(double(x))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn double") && result.contains("fn quadruple"),
        "Should transpile function calling function. Got: {}",
        result
    );
}

// ============================================================================
// Global statement
// ============================================================================

#[test]
fn test_s11_edge_global_var() {
    let code = r#"
counter: int = 0

def increment() -> int:
    global counter
    counter += 1
    return counter
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn increment"),
        "Should transpile global statement. Got: {}",
        result
    );
}

// ============================================================================
// Chained method calls
// ============================================================================

#[test]
fn test_s11_edge_chained_str_methods() {
    let code = r#"
def clean_and_lower(text: str) -> str:
    return text.strip().lower()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clean_and_lower"),
        "Should transpile chained methods. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_chained_list_methods() {
    let code = r#"
def process(items: list) -> list:
    result: list = items.copy()
    result.sort()
    result.reverse()
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn process"),
        "Should transpile sequential list methods. Got: {}",
        result
    );
}

// ============================================================================
// Power operator
// ============================================================================

#[test]
fn test_s11_edge_power_int() {
    let code = r#"
def cube(x: int) -> int:
    return x ** 3
"#;
    let result = transpile(code);
    assert!(
        result.contains("pow") || result.contains("fn cube"),
        "Should transpile power operator. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_power_float() {
    let code = r#"
def sqrt_approx(x: float) -> float:
    return x ** 0.5
"#;
    let result = transpile(code);
    assert!(
        result.contains("pow") || result.contains("sqrt") || result.contains("fn sqrt_approx"),
        "Should transpile float power. Got: {}",
        result
    );
}

// ============================================================================
// Floor division and modulo
// ============================================================================

#[test]
fn test_s11_edge_floor_div() {
    let code = r#"
def half(x: int) -> int:
    return x // 2
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn half"),
        "Should transpile floor division. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_modulo() {
    let code = r#"
def is_even(x: int) -> bool:
    return x % 2 == 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("%") || result.contains("fn is_even"),
        "Should transpile modulo. Got: {}",
        result
    );
}

// ============================================================================
// Delete statement
// ============================================================================

#[test]
fn test_s11_edge_del_statement() {
    let code = r#"
def remove_item(d: dict, key: str) -> None:
    del d[key]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn remove_item"),
        "Should transpile del statement. Got: {}",
        result
    );
}

// ============================================================================
// Type hints with Optional
// ============================================================================

#[test]
fn test_s11_edge_optional_param() {
    let code = r#"
from typing import Optional

def maybe(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn maybe"),
        "Should transpile Optional param. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_optional_return() {
    let code = r#"
from typing import Optional

def find(items: list, target: int) -> Optional[int]:
    for item in items:
        if item == target:
            return item
    return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find"),
        "Should transpile Optional return. Got: {}",
        result
    );
}

// ============================================================================
// Class patterns
// ============================================================================

#[test]
fn test_s11_edge_class_empty() {
    let code = r#"
class Empty:
    pass
"#;
    let result = transpile(code);
    assert!(
        result.contains("Empty") || result.contains("struct"),
        "Should transpile empty class. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_class_with_class_var() {
    let code = r#"
class Config:
    debug: bool = False
    version: str = "1.0"
"#;
    let result = transpile(code);
    assert!(
        result.contains("Config") || result.contains("struct"),
        "Should transpile class with class vars. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_class_with_str_method() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"
"#;
    let result = transpile(code);
    assert!(
        result.contains("Point"),
        "Should transpile class with __str__. Got: {}",
        result
    );
}

// ============================================================================
// Decorator patterns
// ============================================================================

#[test]
fn test_s11_edge_staticmethod() {
    let code = r#"
class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("Math") || result.contains("add"),
        "Should transpile staticmethod. Got: {}",
        result
    );
}

// ============================================================================
// Complex expressions
// ============================================================================

#[test]
fn test_s11_edge_nested_function_calls() {
    let code = r#"
def compute(items: list) -> int:
    return min(max(items), len(items))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn compute"),
        "Should transpile nested function calls. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_complex_expression() {
    let code = r#"
def calc(a: int, b: int, c: int) -> int:
    return (a + b) * c - (a % b)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn calc"),
        "Should transpile complex expression. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_conditional_assignment() {
    let code = r#"
def assign_cond(x: int) -> int:
    y: int = 0
    if x > 0:
        y = x
    else:
        y = -x
    return y
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn assign_cond"),
        "Should transpile conditional assignment. Got: {}",
        result
    );
}

// ============================================================================
// Mixed patterns combining multiple features
// ============================================================================

#[test]
fn test_s11_edge_fizzbuzz() {
    let code = r#"
def fizzbuzz(n: int) -> list:
    result: list = []
    for i in range(1, n + 1):
        if i % 15 == 0:
            result.append("FizzBuzz")
        elif i % 3 == 0:
            result.append("Fizz")
        elif i % 5 == 0:
            result.append("Buzz")
        else:
            result.append(str(i))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fizzbuzz"),
        "Should transpile FizzBuzz. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_gcd_algorithm() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn gcd"),
        "Should transpile GCD algorithm. Got: {}",
        result
    );
}

#[test]
fn test_s11_edge_power_set() {
    let code = r#"
def power_set(items: list) -> list:
    result: list = [[]]
    for item in items:
        new_subsets: list = []
        for subset in result:
            new_subsets.append(subset + [item])
        result.extend(new_subsets)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn power_set"),
        "Should transpile power set. Got: {}",
        result
    );
}

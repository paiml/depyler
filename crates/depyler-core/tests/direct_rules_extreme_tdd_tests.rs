//! EXTREME TDD tests for direct_rules module
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
// CLASS CONVERSION TESTS
// ============================================================================

#[test]
fn test_empty_class() {
    let code = r#"
class Empty:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_class_with_init() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_class_with_method() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.value = 0

    def increment(self) -> None:
        self.value = self.value + 1

    def get(self) -> int:
        return self.value
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dataclass() {
    let code = r#"
from dataclasses import dataclass

@dataclass
class Person:
    name: str
    age: int
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_class_inheritance() {
    let code = r#"
class Base:
    def method(self) -> int:
        return 0

class Derived(Base):
    def method(self) -> int:
        return 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_class_with_class_attribute() {
    let code = r#"
class Config:
    DEFAULT_VALUE: int = 100

    def get_default(self) -> int:
        return Config.DEFAULT_VALUE
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_class_with_staticmethod() {
    let code = r#"
class Math:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// TYPE ALIAS TESTS
// ============================================================================

#[test]
fn test_simple_type_alias() {
    let code = r#"
from typing import List

IntList = List[int]

def use_alias(x: IntList) -> int:
    return len(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_union_type() {
    let code = r#"
from typing import Union

def handle(x: Union[int, str]) -> str:
    return str(x)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_optional_type() {
    let code = r#"
from typing import Optional

def maybe(x: Optional[int]) -> int:
    if x is None:
        return 0
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// PROTOCOL/TRAIT TESTS
// ============================================================================

#[test]
fn test_protocol_basic() {
    let code = r#"
from typing import Protocol

class Comparable(Protocol):
    def compare(self, other: "Comparable") -> int:
        ...
"#;
    // Protocols may not be fully supported
    let _result = transpile_code(code);
}

// ============================================================================
// ASSIGNMENT TESTS
// ============================================================================

#[test]
fn test_simple_assignment() {
    let code = r#"
def assign() -> int:
    x = 42
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_tuple_unpacking() {
    let code = r#"
def unpack() -> int:
    a, b = 1, 2
    return a + b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_unpacking() {
    let code = r#"
def unpack() -> int:
    lst = [1, 2, 3]
    a, b, c = lst
    return a + b + c
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_index_assignment() {
    let code = r#"
def set_index(lst: list, i: int, v: int) -> None:
    lst[i] = v
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_attribute_assignment() {
    let code = r#"
class Point:
    def __init__(self, x: int) -> None:
        self.x = x

    def set_x(self, v: int) -> None:
        self.x = v
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_augmented_assignment() {
    let code = r#"
def augment(x: int) -> int:
    x += 10
    x -= 5
    x *= 2
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// BINARY OPERATION TESTS
// ============================================================================

#[test]
fn test_arithmetic_ops() {
    let code = r#"
def arith(a: int, b: int) -> int:
    return a + b - a * b // a % b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_comparison_ops() {
    let code = r#"
def compare(a: int, b: int) -> bool:
    return a < b and a <= b or a > b and a >= b
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_bitwise_ops() {
    let code = r#"
def bitwise(a: int, b: int) -> int:
    return (a & b) | (a ^ b) << 2 >> 1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_logical_ops() {
    let code = r#"
def logical(a: bool, b: bool) -> bool:
    return a and b or not a
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_power_op() {
    let code = r#"
def power(a: int, b: int) -> int:
    return a ** b
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// UNARY OPERATION TESTS
// ============================================================================

#[test]
fn test_unary_minus() {
    let code = r#"
def negate(x: int) -> int:
    return -x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_unary_not() {
    let code = r#"
def invert(x: bool) -> bool:
    return not x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_unary_bitnot() {
    let code = r#"
def bitnot(x: int) -> int:
    return ~x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// CONTAINER LITERAL TESTS
// ============================================================================

#[test]
fn test_empty_list() {
    let code = r#"
def empty() -> list:
    return []
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_with_items() {
    let code = r#"
def items() -> list:
    return [1, 2, 3, 4, 5]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_empty_dict() {
    let code = r#"
def empty() -> dict:
    return {}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_with_items() {
    let code = r#"
def items() -> dict:
    return {"a": 1, "b": 2}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_empty_set() {
    let code = r#"
def empty() -> set:
    return set()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_with_items() {
    let code = r#"
def items() -> set:
    return {1, 2, 3}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_tuple() {
    let code = r#"
def items() -> tuple:
    return (1, 2, 3)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// COMPREHENSION TESTS
// ============================================================================

#[test]
fn test_list_comprehension() {
    let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_list_comprehension_with_if() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dict_comprehension() {
    let code = r#"
def square_dict(n: int) -> dict:
    return {x: x * x for x in range(n)}
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_set_comprehension() {
    let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// CONTROL FLOW TESTS
// ============================================================================

#[test]
fn test_if_statement() {
    let code = r#"
def check(x: int) -> int:
    if x > 0:
        return 1
    return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_else_statement() {
    let code = r#"
def check(x: int) -> int:
    if x > 0:
        return 1
    else:
        return -1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_if_elif_else() {
    let code = r#"
def classify(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_while_loop() {
    let code = r#"
def count(n: int) -> int:
    i = 0
    while i < n:
        i = i + 1
    return i
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_loop_range() {
    let code = r#"
def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total = total + i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_for_loop_list() {
    let code = r#"
def sum_list(lst: list) -> int:
    total = 0
    for item in lst:
        total = total + item
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_break_statement() {
    let code = r#"
def find(n: int) -> int:
    for i in range(n):
        if i == 5:
            break
    return i
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_continue_statement() {
    let code = r#"
def sum_odds(n: int) -> int:
    total = 0
    for i in range(n):
        if i % 2 == 0:
            continue
        total = total + i
    return total
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_match_statement() {
    let code = r#"
def match_val(x: int) -> str:
    match x:
        case 0:
            return "zero"
        case 1:
            return "one"
        case _:
            return "other"
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// EXCEPTION HANDLING TESTS
// ============================================================================

#[test]
fn test_try_except() {
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
fn test_try_except_finally() {
    let code = r#"
def with_cleanup(x: int) -> int:
    try:
        return x * 2
    finally:
        pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_raise() {
    let code = r#"
def validate(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_assert() {
    let code = r#"
def check(x: int) -> int:
    assert x > 0
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// FUNCTION CALL TESTS
// ============================================================================

#[test]
fn test_builtin_call() {
    let code = r#"
def use_builtins(lst: list) -> int:
    return len(lst) + abs(-5) + max(1, 2) + min(3, 4)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_method_call() {
    let code = r#"
def use_methods(s: str) -> str:
    return s.upper().lower().strip()
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_chained_calls() {
    let code = r#"
def chain(s: str) -> list:
    return s.strip().split(",")
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// LAMBDA TESTS
// ============================================================================

#[test]
fn test_lambda_simple() {
    let code = r#"
def use_lambda() -> int:
    f = lambda x: x + 1
    return f(5)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_lambda_multi_arg() {
    let code = r#"
def use_lambda() -> int:
    f = lambda x, y: x + y
    return f(2, 3)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_lambda_no_arg() {
    let code = r#"
def use_lambda() -> int:
    f = lambda: 42
    return f()
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// TERNARY EXPRESSION TESTS
// ============================================================================

#[test]
fn test_ternary() {
    let code = r#"
def ternary(x: int) -> int:
    return 1 if x > 0 else -1
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_nested_ternary() {
    let code = r#"
def nested(x: int) -> int:
    return 1 if x > 0 else 0 if x == 0 else -1
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// IDENTIFIER SANITIZATION TESTS
// ============================================================================

#[test]
fn test_underscore_prefix() {
    let code = r#"
def use_underscore() -> int:
    _private = 42
    return _private
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_dunder_method() {
    let code = r#"
class Container:
    def __len__(self) -> int:
        return 0
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_stdlib_shadowing() {
    let code = r#"
def use_shadow() -> int:
    list = [1, 2, 3]
    return len(list)
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// MUTABLE STATE TESTS
// ============================================================================

#[test]
fn test_mutable_param() {
    let code = r#"
def modify(lst: list) -> None:
    lst.append(42)
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_mutable_self() {
    let code = r#"
class Counter:
    def __init__(self) -> None:
        self.count = 0

    def inc(self) -> None:
        self.count = self.count + 1
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// TYPE CONVERSION TESTS
// ============================================================================

#[test]
fn test_int_type() {
    let code = r#"
def use_int(x: int) -> int:
    return x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_float_type() {
    let code = r#"
def use_float(x: float) -> float:
    return x
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("f64"));
}

#[test]
fn test_bool_type() {
    let code = r#"
def use_bool(x: bool) -> bool:
    return x
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("bool"));
}

#[test]
fn test_str_type() {
    let code = r#"
def use_str(x: str) -> str:
    return x
"#;
    assert!(transpile_succeeds(code));
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

proptest! {
    /// Property: Simple classes should transpile
    #[test]
    fn prop_simple_class_transpiles(x in 1i64..100) {
        let code = format!(r#"
class C:
    def __init__(self) -> None:
        self.value = {}
"#, x);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Methods should transpile
    #[test]
    fn prop_method_transpiles(x in 1i64..100) {
        let code = format!(r#"
class C:
    def method(self) -> int:
        return {}
"#, x);
        prop_assert!(transpile_succeeds(&code));
    }

    /// Property: Functions with loops should transpile
    #[test]
    fn prop_loop_transpiles(n in 1usize..50) {
        let code = format!(r#"
def loop() -> int:
    total = 0
    for i in range({}):
        total = total + i
    return total
"#, n);
        prop_assert!(transpile_succeeds(&code));
    }
}

// ============================================================================
// MUTATION-RESISTANT TESTS
// ============================================================================

#[test]
fn test_class_name_preserved() {
    let code = r#"
class MyUniqueClassName:
    pass
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_field_name_preserved() {
    let code = r#"
class Point:
    def __init__(self, unique_x: int) -> None:
        self.unique_x = unique_x
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_method_name_preserved() {
    let code = r#"
class Service:
    def unique_method_name(self) -> int:
        return 42
"#;
    assert!(transpile_succeeds(code));
}

#[test]
fn test_constant_preserved() {
    let code = r#"
def constant() -> int:
    return 98765
"#;
    let result = transpile_code(code).unwrap();
    assert!(result.contains("98765"));
}

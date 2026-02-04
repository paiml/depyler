//! Coverage tests for ast_bridge.rs
//!
//! DEPYLER-99MODE-001: Targets ast_bridge.rs (4,042 lines)
//! Covers: module conversion, class/protocol detection, function conversion,
//! statement conversion, expression conversion, type extraction,
//! parameter inference, operator mapping.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Module-level conversion
// ============================================================================

#[test]
fn test_ast_bridge_single_function() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_multiple_functions() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_function_with_docstring() {
    let code = r#"
def greet(name: str) -> str:
    """Return a greeting message."""
    return "Hello, " + name
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class definitions
// ============================================================================

#[test]
fn test_ast_bridge_simple_class() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_class_with_methods() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x: int) -> int:
        self.result += x
        return self.result

    def reset(self):
        self.result = 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_class_str_method() {
    let code = r#"
class Dog:
    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function parameters
// ============================================================================

#[test]
fn test_ast_bridge_typed_params() {
    let code = "def f(x: int, y: str, z: float) -> str:\n    return str(x)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_default_params() {
    let code = r#"
def f(x: int = 0, y: str = "default") -> str:
    return y + str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_untyped_params() {
    let code = "def f(x, y):\n    return x + y\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_mixed_typed_untyped() {
    let code = "def f(x: int, y) -> int:\n    return x\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Type annotations
// ============================================================================

#[test]
fn test_ast_bridge_return_int() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_str() {
    let code = "def f() -> str:\n    return \"hello\"\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_float() {
    let code = "def f() -> float:\n    return 3.14\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_bool() {
    let code = "def f() -> bool:\n    return True\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_list() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_dict() {
    let code = "def f() -> dict:\n    return {\"a\": 1}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_tuple() {
    let code = "def f() -> tuple:\n    return (1, 2)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Statement conversion
// ============================================================================

#[test]
fn test_ast_bridge_assign() {
    let code = "def f() -> int:\n    x = 42\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_ann_assign() {
    let code = "def f() -> int:\n    x: int = 42\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_aug_assign() {
    let code = "def f(x: int) -> int:\n    x += 1\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_if_stmt() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "pos"
    return "non-pos"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_if_elif_else() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_while_stmt() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    i = 0
    while i < n:
        total += i
        i += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_for_stmt() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_for_list() {
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
fn test_ast_bridge_break_stmt() {
    let code = r#"
def f() -> int:
    for i in range(100):
        if i > 10:
            break
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_continue_stmt() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        if i % 2 == 0:
            continue
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_stmt() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_return_none() {
    let code = "def f():\n    return\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_pass_stmt() {
    let code = "def f():\n    pass\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_raise_stmt() {
    let code = r#"
def f():
    raise ValueError("error")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_assert_stmt() {
    let code = "def f(x: int):\n    assert x > 0\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_try_except() {
    let code = r#"
def f() -> int:
    try:
        return 42
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_try_except_finally() {
    let code = r#"
def f() -> int:
    x = 0
    try:
        x = 42
    except:
        x = -1
    finally:
        x += 1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_with_stmt() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as file:
        return file.read()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Expression conversion
// ============================================================================

#[test]
fn test_ast_bridge_literal_int() {
    let code = "def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_literal_neg_int() {
    let code = "def f() -> int:\n    return -42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_literal_float() {
    let code = "def f() -> float:\n    return 3.14\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_literal_string() {
    let code = "def f() -> str:\n    return \"hello\"\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_literal_bool() {
    let code = "def f() -> bool:\n    return True\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_literal_none() {
    let code = "def f():\n    return None\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_add() {
    let code = "def f(a: int, b: int) -> int:\n    return a + b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_sub() {
    let code = "def f(a: int, b: int) -> int:\n    return a - b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_mul() {
    let code = "def f(a: int, b: int) -> int:\n    return a * b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_div() {
    let code = "def f(a: float, b: float) -> float:\n    return a / b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_floor_div() {
    let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_mod() {
    let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_binop_pow() {
    let code = "def f(a: int, b: int) -> int:\n    return a ** b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_unaryop_neg() {
    let code = "def f(x: int) -> int:\n    return -x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_unaryop_not() {
    let code = "def f(x: bool) -> bool:\n    return not x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_compare_chain() {
    let code = "def f(x: int) -> bool:\n    return 0 < x < 100\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_boolop_and() {
    let code = "def f(a: bool, b: bool) -> bool:\n    return a and b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_boolop_or() {
    let code = "def f(a: bool, b: bool) -> bool:\n    return a or b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_call_expr() {
    let code = "def f(x: int) -> int:\n    return abs(x)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_subscript_expr() {
    let code = "def f(items: list) -> int:\n    return items[0]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_slice_expr() {
    let code = "def f(items: list) -> list:\n    return items[1:3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_list_expr() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_dict_expr() {
    let code = "def f() -> dict:\n    return {\"a\": 1}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_tuple_expr() {
    let code = "def f() -> tuple:\n    return (1, 2)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_set_expr() {
    let code = "def f() -> set:\n    return {1, 2, 3}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_list_comp_expr() {
    let code = "def f() -> list:\n    return [x * 2 for x in range(10)]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_dict_comp_expr() {
    let code = "def f() -> dict:\n    return {str(i): i for i in range(5)}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_set_comp_expr() {
    let code = "def f(items: list) -> set:\n    return {x for x in items}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_lambda_expr() {
    let code = r#"
def f() -> list:
    return sorted([3, 1, 2], key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_fstring_expr() {
    let code = r#"
def f(name: str) -> str:
    return f"Hello {name}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_ifexp() {
    let code = r#"
def f(x: int) -> str:
    return "yes" if x > 0 else "no"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_attribute_expr() {
    let code = r#"
class Foo:
    def __init__(self):
        self.val = 0

    def get(self) -> int:
        return self.val
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Async patterns
// ============================================================================

#[test]
fn test_ast_bridge_async_function() {
    let code = "async def f() -> int:\n    return 42\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_async_await() {
    let code = r#"
async def fetch() -> str:
    return "data"

async def main() -> str:
    result = await fetch()
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Operator conversion
// ============================================================================

#[test]
fn test_ast_bridge_op_in() {
    let code = "def f(x: int, items: list) -> bool:\n    return x in items\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_not_in() {
    let code = "def f(x: int, items: list) -> bool:\n    return x not in items\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_is() {
    let code = "def f(x) -> bool:\n    return x is None\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_is_not() {
    let code = "def f(x) -> bool:\n    return x is not None\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_bitwise_and() {
    let code = "def f(a: int, b: int) -> int:\n    return a & b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_bitwise_or() {
    let code = "def f(a: int, b: int) -> int:\n    return a | b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_bitwise_xor() {
    let code = "def f(a: int, b: int) -> int:\n    return a ^ b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_lshift() {
    let code = "def f(a: int, b: int) -> int:\n    return a << b\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_op_rshift() {
    let code = "def f(a: int, b: int) -> int:\n    return a >> b\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration patterns
// ============================================================================

#[test]
fn test_ast_bridge_gcd() {
    let code = r#"
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_is_prime() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_flatten_list() {
    let code = r#"
def flatten(items: list) -> list:
    result = []
    for item in items:
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_multi_class_module() {
    let code = r#"
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return self.name

class Dog:
    def __init__(self, name: str):
        self.name = name

    def bark(self) -> str:
        return "Woof!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_bridge_mixed_functions_classes() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

class Wrapper:
    def __init__(self, val: int):
        self.val = val

    def doubled(self) -> int:
        return helper(self.val)
"#;
    assert!(transpile_ok(code));
}

//! Coverage tests for type_system/hindley_milner.rs
//!
//! DEPYLER-99MODE-001: Targets hindley_milner.rs (932 lines)
//! Covers: Algorithm W type inference, Robinson's unification,
//! occurs check, substitution, constraint solving.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Primitive type unification
// ============================================================================

#[test]
fn test_hm_int_inference() {
    let code = "def f() -> int:\n    x = 42\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_float_inference() {
    let code = "def f() -> float:\n    x = 3.14\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_string_inference() {
    let code = "def f() -> str:\n    x = \"hello\"\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_bool_inference() {
    let code = "def f() -> bool:\n    x = True\n    return x\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Type variable binding (Instance constraints)
// ============================================================================

#[test]
fn test_hm_param_binding() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_mixed_param_binding() {
    let code = r#"
def f(x: int, s: str) -> str:
    return s + str(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection type unification
// ============================================================================

#[test]
fn test_hm_list_type() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_dict_type() {
    let code = r#"
def f() -> dict:
    d = {"key": 1}
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_set_type() {
    let code = r#"
def f() -> set:
    s = {1, 2, 3}
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_tuple_type() {
    let code = "def f() -> tuple:\n    return (1, 2)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Equality constraints from expressions
// ============================================================================

#[test]
fn test_hm_arithmetic_unify() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    z = y * 2
    return z
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_comparison_unify() {
    let code = r#"
def f(x: int) -> bool:
    return x > 0 and x < 100
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function call type inference
// ============================================================================

#[test]
fn test_hm_function_call_inference() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def f() -> int:
    result = double(21)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_chained_function_calls() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def f() -> int:
    x = add(1, 2)
    y = add(x, 3)
    return y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop type inference
// ============================================================================

#[test]
fn test_hm_for_loop_inference() {
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
fn test_hm_range_inference() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Conditional type inference
// ============================================================================

#[test]
fn test_hm_if_else_inference() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        result = x
    else:
        result = -x
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_ternary_inference() {
    let code = r#"
def f(x: int) -> int:
    return x if x > 0 else -x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex type inference patterns
// ============================================================================

#[test]
fn test_hm_recursive_function() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_list_comprehension() {
    let code = r#"
def f(n: int) -> list:
    return [i * i for i in range(n)]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_dict_comprehension() {
    let code = r#"
def f(keys: list) -> dict:
    return {k: len(k) for k in keys}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hm_class_type_inference() {
    let code = r#"
class Counter:
    def __init__(self, start: int):
        self.value = start

    def increment(self) -> int:
        self.value += 1
        return self.value
"#;
    assert!(transpile_ok(code));
}

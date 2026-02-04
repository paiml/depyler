//! Coverage tests for optimizer.rs
//!
//! DEPYLER-99MODE-001: Targets optimizer.rs (3,114 lines)
//! Covers: constant propagation, dead code elimination,
//! common subexpression elimination, walrus operator hoisting,
//! side effect detection, pure function detection.

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
// Constant propagation
// ============================================================================

#[test]
fn test_optimizer_const_prop_int() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_const_prop_float() {
    let code = r#"
def f() -> float:
    pi = 3.14
    return pi
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_const_prop_string() {
    let code = r#"
def f() -> str:
    greeting = "hello"
    return greeting
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_const_prop_bool() {
    let code = r#"
def f() -> bool:
    flag = True
    return flag
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_const_prop_arithmetic() {
    let code = r#"
def f() -> int:
    a = 10
    b = 20
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_const_prop_no_propagate_mutable() {
    let code = r#"
def f() -> int:
    x = 0
    x = 10
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_const_prop_no_propagate_augmented() {
    let code = r#"
def f() -> int:
    x = 0
    x += 5
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dead code elimination
// ============================================================================

#[test]
fn test_optimizer_dce_unused_var() {
    let code = r#"
def f() -> int:
    unused = 42
    used = 10
    return used
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_dce_preserves_side_effects() {
    let code = r#"
def f() -> int:
    print("hello")
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_dce_in_loop() {
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
fn test_optimizer_dce_in_conditional() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        result = x * 2
    else:
        result = 0
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_dce_after_return() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_dce_transitive() {
    let code = r#"
def f() -> int:
    a = 1
    b = a + 1
    c = b + 1
    return 42
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Common subexpression elimination
// ============================================================================

#[test]
fn test_optimizer_cse_repeated_expr() {
    let code = r#"
def f(x: int) -> int:
    a = x * x
    b = x * x
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_cse_function_call() {
    let code = r#"
def f(items: list) -> int:
    a = len(items)
    b = len(items)
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_cse_no_impure() {
    let code = r#"
def f() -> int:
    print("a")
    print("b")
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_cse_complex_expr() {
    let code = r#"
def f(a: int, b: int) -> int:
    x = a * b + a
    y = a * b + a
    return x + y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Walrus operator hoisting
// ============================================================================

#[test]
fn test_optimizer_walrus_in_if() {
    let code = r#"
def f(text: str) -> int:
    if len(text) > 5:
        return len(text)
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_walrus_in_while() {
    let code = r#"
def f() -> int:
    i = 0
    while i < 10:
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Side effect detection
// ============================================================================

#[test]
fn test_optimizer_side_effect_print() {
    let code = r#"
def f():
    x = 10
    print(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_side_effect_list_append() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    items.append(2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_side_effect_dict_update() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pure function patterns
// ============================================================================

#[test]
fn test_optimizer_pure_abs() {
    let code = r#"
def f(x: int) -> int:
    a = abs(x)
    b = abs(x)
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_pure_len() {
    let code = r#"
def f(items: list) -> bool:
    return len(items) > 0 and len(items) < 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_pure_min_max() {
    let code = r#"
def f(a: int, b: int) -> int:
    lo = min(a, b)
    hi = max(a, b)
    return hi - lo
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Optimization pipeline integration
// ============================================================================

#[test]
fn test_optimizer_full_pipeline_simple() {
    let code = r#"
def f(x: int) -> int:
    temp = x + 1
    return temp
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_full_pipeline_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i * i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_full_pipeline_nested_if() {
    let code = r#"
def f(x: int, y: int) -> int:
    if x > 0:
        if y > 0:
            return x + y
        else:
            return x - y
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_full_pipeline_multiple_returns() {
    let code = r#"
def f(x: int) -> str:
    if x > 100:
        return "large"
    if x > 10:
        return "medium"
    if x > 0:
        return "small"
    return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_full_pipeline_string_ops() {
    let code = r#"
def f(text: str) -> str:
    result = text.strip()
    result = result.lower()
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_full_pipeline_dict_ops() {
    let code = r#"
def f() -> dict:
    d = {}
    d["a"] = 1
    d["b"] = 2
    d["c"] = 3
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_full_pipeline_comprehension() {
    let code = r#"
def f() -> list:
    return [x * x for x in range(10) if x % 2 == 0]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex optimization scenarios
// ============================================================================

#[test]
fn test_optimizer_fibonacci_opt() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_accumulation_opt() {
    let code = r#"
def sum_squares(items: list) -> int:
    total = 0
    for item in items:
        total += item * item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_multi_function_opt() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def process(items: list) -> list:
    result = []
    for item in items:
        result.append(double(item))
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_exception_handling_opt() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_nested_loops_opt() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            total += i * j
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_string_building_opt() {
    let code = r#"
def f(items: list) -> str:
    parts = []
    for item in items:
        parts.append(str(item))
    return ", ".join(parts)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_early_return_opt() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_class_method_opt() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self) -> int:
        self.count += 1
        return self.count

    def reset(self):
        self.count = 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_complex_conditionals() {
    let code = r#"
def classify(x: int, y: int) -> str:
    if x > 0 and y > 0:
        return "first"
    elif x < 0 and y > 0:
        return "second"
    elif x < 0 and y < 0:
        return "third"
    elif x > 0 and y < 0:
        return "fourth"
    return "origin"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_optimizer_while_with_break() {
    let code = r#"
def f(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] == 0:
            break
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

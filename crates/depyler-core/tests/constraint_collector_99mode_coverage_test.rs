//! Coverage tests for constraint_collector.rs
//!
//! DEPYLER-99MODE-001: Targets type_system/constraint_collector.rs (2,442 lines)
//! Covers: arithmetic constraints, comparison/bool constraints,
//! bitwise constraints, collection constraints, method-based inference,
//! function call propagation, return type constraints.

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
// Arithmetic operator constraints (Add/Sub/Mul/Div)
// ============================================================================

#[test]
fn test_constraint_add_ints() {
    let code = r#"
def f(x, y):
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_sub_ints() {
    let code = r#"
def f(x, y):
    return x - y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_mul() {
    let code = r#"
def f(x, y):
    return x * y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_div() {
    let code = r#"
def f(x, y):
    return x / y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_floor_div() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x // y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_modulo() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x % y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_power() {
    let code = r#"
def f(x: int, n: int) -> float:
    return x ** n
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comparison operator constraints (Bool result)
// ============================================================================

#[test]
fn test_constraint_lt() {
    let code = r#"
def f(a, b):
    return a < b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_gt() {
    let code = r#"
def f(a, b):
    return a > b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_eq() {
    let code = r#"
def f(a, b):
    return a == b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_neq() {
    let code = r#"
def f(a, b):
    return a != b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_lte() {
    let code = r#"
def f(a, b):
    return a <= b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_gte() {
    let code = r#"
def f(a, b):
    return a >= b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Boolean operator constraints
// ============================================================================

#[test]
fn test_constraint_bool_and() {
    let code = r#"
def f(a, b):
    return a and b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_bool_or() {
    let code = r#"
def f(a, b):
    return a or b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_bool_not() {
    let code = r#"
def f(a) -> bool:
    return not a
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Bitwise operator constraints (Int required)
// ============================================================================

#[test]
fn test_constraint_bitand() {
    let code = r#"
def f(a, b):
    return a & b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_bitor() {
    let code = r#"
def f(a, b):
    return a | b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_bitxor() {
    let code = r#"
def f(a, b):
    return a ^ b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_lshift() {
    let code = r#"
def f(a, b):
    return a << b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_rshift() {
    let code = r#"
def f(a, b):
    return a >> b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection constraints (element type unification)
// ============================================================================

#[test]
fn test_constraint_list_homogeneous() {
    let code = r#"
def f():
    return [1, 2, 3]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_list_empty() {
    let code = r#"
def f() -> list:
    return []
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_dict_literal() {
    let code = r#"
def f():
    return {"a": 1, "b": 2}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_dict_empty() {
    let code = r#"
def f() -> dict:
    return {}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_tuple_types() {
    let code = r#"
def f():
    return (1, "hello", True)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_set_literal() {
    let code = r#"
def f():
    return {1, 2, 3}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method-based type inference
// ============================================================================

#[test]
fn test_constraint_string_method_upper() {
    let code = r#"
def f(text):
    return text.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_string_method_split() {
    let code = r#"
def f(text):
    return text.split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_list_method_append() {
    let code = r#"
def f(items):
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_list_method_extend() {
    let code = r#"
def f(items):
    items.extend([4, 5, 6])
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_dict_method_keys() {
    let code = r#"
def f(mapping):
    return list(mapping.keys())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_dict_method_values() {
    let code = r#"
def f(mapping):
    return list(mapping.values())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_dict_method_items() {
    let code = r#"
def f(mapping):
    result = []
    for k, v in mapping.items():
        result.append(k)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Function call parameter constraint propagation
// ============================================================================

#[test]
fn test_constraint_call_known_func() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b

def caller(x, y):
    return add(x, y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_call_chain() {
    let code = r#"
def double(x: int) -> int:
    return x * 2

def quadruple(x):
    return double(double(x))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_len_call() {
    let code = r#"
def f(items):
    return len(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Return type constraints
// ============================================================================

#[test]
fn test_constraint_return_annotated() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_return_multiple() {
    let code = r#"
def f(flag: bool) -> str:
    if flag:
        return "yes"
    return "no"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_return_expression() {
    let code = r#"
def f(x: int) -> int:
    return x * 2 + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable assignment constraints
// ============================================================================

#[test]
fn test_constraint_assign_simple() {
    let code = r#"
def f():
    x = 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_assign_with_annotation() {
    let code = r#"
def f():
    x: int = 5
    y: str = "hello"
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_augmented_assign() {
    let code = r#"
def f():
    x = 0
    x += 1
    x *= 2
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow condition constraints (Bool enforcement)
// ============================================================================

#[test]
fn test_constraint_if_condition_bool() {
    let code = r#"
def f(x):
    if x:
        return 1
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_elif_condition() {
    let code = r#"
def f(x):
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_while_condition() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    while n > 0:
        total += n
        n -= 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Slice constraints (Int indices, String default)
// ============================================================================

#[test]
fn test_constraint_slice_string() {
    let code = r#"
def f(text: str) -> str:
    return text[1:4]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_slice_list() {
    let code = r#"
def f(items: list) -> list:
    return items[1:3]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// For loop constraints
// ============================================================================

#[test]
fn test_constraint_for_range() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_for_collection() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Try/except constraints
// ============================================================================

#[test]
fn test_constraint_try_except() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex multi-constraint patterns
// ============================================================================

#[test]
fn test_constraint_complex_function() {
    let code = r#"
def process(data, threshold):
    result = []
    count = 0
    for item in data:
        if item > threshold:
            result.append(item * 2)
            count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_constraint_multi_function() {
    let code = r#"
def square(x: int) -> int:
    return x * x

def sum_squares(items: list) -> int:
    total = 0
    for item in items:
        total += square(item)
    return total
"#;
    assert!(transpile_ok(code));
}

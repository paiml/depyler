//! Coverage tests for const_generic_inference.rs
//!
//! DEPYLER-99MODE-001: Targets const_generic_inference.rs (1,248 lines)
//! Covers: fixed-size array detection, list multiplication patterns,
//! const value inference, length checks, index access bounds.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Fixed-size literal lists
// ============================================================================

#[test]
fn test_const_generic_literal_list() {
    let code = r#"
def f() -> list:
    arr = [1, 2, 3]
    return arr
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_literal_list_5() {
    let code = r#"
def f() -> list:
    arr = [10, 20, 30, 40, 50]
    return arr
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_empty_list() {
    let code = r#"
def f() -> list:
    arr = []
    return arr
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// List multiplication patterns
// ============================================================================

#[test]
fn test_const_generic_list_multiply() {
    let code = r#"
def f() -> list:
    arr = [0] * 5
    return arr
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_list_multiply_reverse() {
    let code = r#"
def f() -> list:
    arr = 10 * [0]
    return arr
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_list_multiply_large() {
    let code = r#"
def f() -> list:
    arr = [0] * 100
    return arr
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Length check patterns
// ============================================================================

#[test]
fn test_const_generic_len_check() {
    let code = r#"
def f(arr: list) -> int:
    if len(arr) == 5:
        return arr[0]
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_len_compare() {
    let code = r#"
def f(arr: list) -> bool:
    return len(arr) > 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Index access patterns
// ============================================================================

#[test]
fn test_const_generic_index_access() {
    let code = r#"
def f(arr: list) -> int:
    return arr[4]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_index_access_negative() {
    let code = r#"
def f(arr: list) -> int:
    return arr[-1]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_multiple_index_access() {
    let code = r#"
def f(arr: list) -> int:
    return arr[0] + arr[1] + arr[2]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow with list assignments
// ============================================================================

#[test]
fn test_const_generic_if_else_lists() {
    let code = r#"
def f(flag: bool) -> list:
    if flag:
        x = [1, 2, 3]
    else:
        x = [4, 5, 6]
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_loop_list() {
    let code = r#"
def f() -> int:
    arr = [10, 20, 30]
    total = 0
    for val in arr:
        total += val
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_while_list() {
    let code = r#"
def f() -> list:
    result = [0] * 3
    i = 0
    while i < 3:
        result[i] = i * i
        i += 1
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Return value inference
// ============================================================================

#[test]
fn test_const_generic_return_literal() {
    let code = r#"
def f() -> list:
    return [1, 2, 3, 4]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_return_assigned() {
    let code = r#"
def f() -> list:
    result = [10, 20, 30]
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_const_generic_matrix() {
    let code = r#"
def f() -> list:
    matrix = [[1, 2], [3, 4], [5, 6]]
    return matrix
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_with_mutation() {
    let code = r#"
def f() -> list:
    arr = [0] * 5
    arr[0] = 1
    arr[2] = 3
    return arr
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_with_append() {
    let code = r#"
def f() -> list:
    arr = [1, 2, 3]
    arr.append(4)
    return arr
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_const_generic_function_param() {
    let code = r#"
def process(data: list) -> int:
    if len(data) >= 3:
        return data[0] + data[1] + data[2]
    return 0
"#;
    assert!(transpile_ok(code));
}

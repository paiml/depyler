//! Coverage tests for rust_gen/stdlib_method_gen/builtin_functions.rs
//!
//! DEPYLER-99MODE-001: Targets builtin_functions.rs (1,305 lines)
//! Covers: print, range, sorted, enumerate, zip, map, filter,
//! abs, min, max, sum, any, all, reversed, isinstance, type.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// print
// ============================================================================

#[test]
fn test_stdlib_print() {
    let code = r#"
def f(x: int):
    print(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_print_string() {
    let code = r#"
def f(s: str):
    print(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// range
// ============================================================================

#[test]
fn test_stdlib_range_one_arg() {
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
fn test_stdlib_range_two_args() {
    let code = r#"
def f(start: int, stop: int) -> int:
    total = 0
    for i in range(start, stop):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_range_three_args() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(0, n, 2):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// sorted
// ============================================================================

#[test]
fn test_stdlib_sorted() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// enumerate
// ============================================================================

#[test]
fn test_stdlib_enumerate() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, item in enumerate(items):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// zip
// ============================================================================

#[test]
fn test_stdlib_zip() {
    let code = r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// abs, min, max, sum
// ============================================================================

#[test]
fn test_stdlib_abs() {
    let code = r#"
def f(x: int) -> int:
    return abs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_min() {
    let code = r#"
def f(a: int, b: int) -> int:
    return min(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_max() {
    let code = r#"
def f(a: int, b: int) -> int:
    return max(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_sum() {
    let code = r#"
def f(items: list) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// any, all
// ============================================================================

#[test]
fn test_stdlib_any() {
    let code = r#"
def f(items: list) -> bool:
    return any(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_all() {
    let code = r#"
def f(items: list) -> bool:
    return all(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// reversed
// ============================================================================

#[test]
fn test_stdlib_reversed() {
    let code = r#"
def f(items: list) -> list:
    return list(reversed(items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// map, filter
// ============================================================================

#[test]
fn test_stdlib_map() {
    let code = r#"
def f(items: list) -> list:
    return list(map(str, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdlib_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// isinstance
// ============================================================================

#[test]
fn test_stdlib_isinstance() {
    let code = r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Combined builtins
// ============================================================================

#[test]
fn test_stdlib_builtins_combined() {
    let code = r#"
def f(items: list) -> dict:
    n = len(items)
    total = sum(items)
    smallest = min(items[0], items[-1])
    largest = max(items[0], items[-1])
    return {"n": n, "total": total, "min": smallest, "max": largest}
"#;
    assert!(transpile_ok(code));
}

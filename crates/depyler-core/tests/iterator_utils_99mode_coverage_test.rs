//! Coverage tests for rust_gen/iterator_utils.rs
//!
//! DEPYLER-99MODE-001: Targets iterator_utils.rs (1,199 lines)
//! Covers: iterator detection, generator expressions, dict methods,
//! built-in iterators, method chains producing iterators.

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
// Range iteration
// ============================================================================

#[test]
fn test_iter_range_basic() {
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
fn test_iter_range_start_stop() {
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
fn test_iter_range_with_step() {
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
// Enumerate
// ============================================================================

#[test]
fn test_iter_enumerate() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i, item in enumerate(items):
        total += i + item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Zip
// ============================================================================

#[test]
fn test_iter_zip() {
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
// Map and filter
// ============================================================================

#[test]
fn test_iter_map() {
    let code = r#"
def f(items: list) -> list:
    return list(map(str, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dict iteration methods
// ============================================================================

#[test]
fn test_iter_dict_keys() {
    let code = r#"
def f(d: dict) -> list:
    result = []
    for key in d:
        result.append(key)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_dict_values() {
    let code = r#"
def f(d: dict) -> list:
    result = []
    for val in d.values():
        result.append(val)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_dict_items() {
    let code = r#"
def f(d: dict) -> list:
    result = []
    for k, v in d.items():
        result.append(k)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Reversed
// ============================================================================

#[test]
fn test_iter_reversed() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String iteration
// ============================================================================

#[test]
fn test_iter_string_chars() {
    let code = r#"
def f(s: str) -> int:
    count = 0
    for c in s:
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_string_split() {
    let code = r#"
def f(s: str) -> list:
    return s.split()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// List comprehensions (iterator consumers)
// ============================================================================

#[test]
fn test_iter_list_comp_basic() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_list_comp_filtered() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_list_comp_with_transform() {
    let code = r#"
def f(items: list) -> list:
    return [str(x) for x in items]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex iterator patterns
// ============================================================================

#[test]
fn test_iter_chained_methods() {
    let code = r#"
def f(s: str) -> list:
    words = s.strip().split()
    return words
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_nested_iteration() {
    let code = r#"
def f(matrix: list) -> int:
    total = 0
    for row in matrix:
        for item in row:
            total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_sum_of_range() {
    let code = r#"
def f(n: int) -> int:
    return sum(range(n))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_iter_len_of_list() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

//! Coverage tests for performance_warnings.rs
//!
//! DEPYLER-99MODE-001: Targets performance_warnings.rs (1,767 lines)
//! Covers: string concatenation in loops, nested loop detection,
//! sorting in loops, range(len()) antipattern, collection usage,
//! memory allocation patterns, redundant computation detection.

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
// String performance patterns
// ============================================================================

#[test]
fn test_perf_string_concat_in_loop() {
    let code = r#"
def f(n: int) -> str:
    result = ""
    for i in range(n):
        result = result + str(i)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_string_augmented_concat_loop() {
    let code = r#"
def f(items: list) -> str:
    result = ""
    for item in items:
        result += str(item)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested loop detection
// ============================================================================

#[test]
fn test_perf_double_nested_loop() {
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
fn test_perf_triple_nested_loop() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        for j in range(n):
            for k in range(n):
                total += i + j + k
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_while_nested_in_for() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        i = 0
        while i < item:
            total += i
            i += 1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Sorting in loop detection
// ============================================================================

#[test]
fn test_perf_sort_in_loop() {
    let code = r#"
def f(matrix: list) -> list:
    result = []
    for row in matrix:
        result.append(sorted(row))
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Range(len()) antipattern
// ============================================================================

#[test]
fn test_perf_range_len_pattern() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for i in range(len(items)):
        total += items[i]
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection usage patterns
// ============================================================================

#[test]
fn test_perf_append_in_loop() {
    let code = r#"
def f(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i * i)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_list_creation_in_loop() {
    let code = r#"
def f(n: int) -> list:
    result = []
    for i in range(n):
        temp = [0] * 10
        result.append(temp)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Redundant computation
// ============================================================================

#[test]
fn test_perf_len_in_loop_condition() {
    let code = r#"
def f(items: list) -> int:
    i = 0
    while i < len(items):
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_sum_in_nested_loop() {
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

// ============================================================================
// Complex performance-relevant patterns
// ============================================================================

#[test]
fn test_perf_linear_search_loop() {
    let code = r#"
def f(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_dict_operations_in_loop() {
    let code = r#"
def f(text: str) -> dict:
    counts = {}
    for word in text.split():
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_comprehension_vs_loop() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_perf_multiple_iterations() {
    let code = r#"
def f(items: list) -> int:
    s = sum(items)
    m = max(items)
    n = min(items)
    return s + m + n
"#;
    assert!(transpile_ok(code));
}

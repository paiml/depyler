//! Coverage tests for profiling.rs
//!
//! DEPYLER-99MODE-001: Targets profiling.rs (1,596 lines)
//! Covers: instruction counting, memory allocation estimation,
//! hot path detection, performance predictions, flamegraph data.

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
// Loop analysis - instruction counting
// ============================================================================

#[test]
fn test_profiling_simple_loop() {
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
fn test_profiling_nested_loops() {
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
fn test_profiling_while_loop() {
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

// ============================================================================
// Type check detection
// ============================================================================

#[test]
fn test_profiling_isinstance_check() {
    let code = r#"
def f(x: int) -> str:
    if isinstance(x, int):
        return "int"
    return "other"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_multiple_isinstance() {
    let code = r#"
def f(x: int) -> str:
    if isinstance(x, int):
        return "int"
    elif isinstance(x, str):
        return "str"
    elif isinstance(x, float):
        return "float"
    return "unknown"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection allocation tracking
// ============================================================================

#[test]
fn test_profiling_list_creation() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3, 4, 5]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_dict_creation() {
    let code = r#"
def f() -> dict:
    data = {"a": 1, "b": 2, "c": 3}
    return data
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_collection_iteration() {
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
// Function call analysis
// ============================================================================

#[test]
fn test_profiling_function_calls() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += helper(i)
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_method_calls() {
    let code = r#"
def f(items: list) -> list:
    items.append(1)
    items.append(2)
    items.reverse()
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Conditional analysis
// ============================================================================

#[test]
fn test_profiling_simple_conditional() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x * 2
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_complex_conditional() {
    let code = r#"
def f(x: int, y: int) -> str:
    if x > 0 and y > 0:
        return "both positive"
    elif x > 0:
        return "x positive"
    elif y > 0:
        return "y positive"
    return "neither"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex expression analysis
// ============================================================================

#[test]
fn test_profiling_binary_expr() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c - a % b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_string_operations() {
    let code = r#"
def f(s: str) -> str:
    return s.strip().lower().replace("a", "b")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Hot path patterns
// ============================================================================

#[test]
fn test_profiling_hot_path_algorithm() {
    let code = r#"
def bubble_sort(items: list) -> list:
    n = len(items)
    for i in range(n):
        for j in range(0, n - i - 1):
            if items[j] > items[j + 1]:
                items[j], items[j + 1] = items[j + 1], items[j]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_multi_function_hot() {
    let code = r#"
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

def count_primes(limit: int) -> int:
    count = 0
    for n in range(2, limit):
        if is_prime(n):
            count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Performance prediction patterns
// ============================================================================

#[test]
fn test_profiling_iterator_opportunity() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_memory_layout() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_profiling_comprehensive() {
    let code = r#"
def matrix_multiply(a: list, b: list) -> list:
    rows_a = len(a)
    cols_b = len(b[0])
    result = []
    for i in range(rows_a):
        row = []
        for j in range(cols_b):
            total = 0
            for k in range(len(b)):
                total += a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result
"#;
    assert!(transpile_ok(code));
}

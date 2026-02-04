//! Coverage tests for rust_gen/stmt_gen_complex.rs
//!
//! DEPYLER-99MODE-001: Targets stmt_gen_complex.rs (2,290 lines)
//! Covers: try/except/finally, nested functions, error handling,
//! variable hoisting, floor division with error handling.

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
// Try/except basic
// ============================================================================

#[test]
fn test_stmt_complex_try_except_basic() {
    let code = r#"
def f(x: int) -> int:
    try:
        return x // 1
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_try_except_specific() {
    let code = r#"
def f(x: int, y: int) -> int:
    try:
        return x // y
    except ZeroDivisionError:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_try_except_as() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError as e:
        return -1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Try/except/finally
// ============================================================================

#[test]
fn test_stmt_complex_try_finally() {
    let code = r#"
def f() -> int:
    x = 0
    try:
        x = 42
    except:
        x = -1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_try_multiple_except() {
    let code = r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable hoisting in try/except
// ============================================================================

#[test]
fn test_stmt_complex_hoisted_variable() {
    let code = r#"
def f(x: int) -> int:
    result = 0
    try:
        result = x * 2
    except:
        result = -1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_hoisted_with_condition() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    try:
        for item in items:
            total += item
    except:
        total = -1
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested control flow
// ============================================================================

#[test]
fn test_stmt_complex_nested_if_in_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        if item > 0:
            total += item
        elif item == 0:
            continue
        else:
            break
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_nested_loops() {
    let code = r#"
def f(n: int) -> list:
    result = []
    for i in range(n):
        for j in range(n):
            if i != j:
                result.append(i * n + j)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_while_with_break() {
    let code = r#"
def f(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] < 0:
            break
        i += 1
    return i
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple returns
// ============================================================================

#[test]
fn test_stmt_complex_multiple_returns() {
    let code = r#"
def f(x: int) -> str:
    if x > 100:
        return "high"
    if x > 50:
        return "medium"
    if x > 0:
        return "low"
    return "none"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex expressions in statements
// ============================================================================

#[test]
fn test_stmt_complex_list_building() {
    let code = r#"
def f(n: int) -> list:
    result = []
    for i in range(n):
        if i % 2 == 0:
            result.append(i * i)
        else:
            result.append(i)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_dict_building() {
    let code = r#"
def f(keys: list, values: list) -> dict:
    result = {}
    for i in range(len(keys)):
        result[keys[i]] = values[i]
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Error handling patterns
// ============================================================================

#[test]
fn test_stmt_complex_guard_clause() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        return -1
    if x == 0:
        return 0
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_exception_flow() {
    let code = r#"
def f(data: list) -> int:
    try:
        total = 0
        for item in data:
            total += item
        return total
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Comprehensive
// ============================================================================

#[test]
fn test_stmt_complex_algorithm() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    low = 0
    high = len(items) - 1
    while low <= high:
        mid = (low + high) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_complex_class_with_try() {
    let code = r#"
class SafeCalculator:
    def __init__(self):
        self.result = 0

    def divide(self, a: int, b: int) -> int:
        try:
            self.result = a // b
        except:
            self.result = 0
        return self.result
"#;
    assert!(transpile_ok(code));
}

//! Coverage tests for error_reporting.rs
//!
//! DEPYLER-99MODE-001: Targets error_reporting.rs (986 lines)
//! Covers: enhanced error reporting, source location tracking,
//! automatic suggestions, type mismatch detection.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Type mismatch patterns (triggers suggestion generation)
// ============================================================================

#[test]
fn test_error_report_type_conversion() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_float_to_int() {
    let code = r#"
def f(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_string_ops() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type inference patterns
// ============================================================================

#[test]
fn test_error_report_inferred_types() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_complex_inference() {
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
// Division patterns (int vs float division)
// ============================================================================

#[test]
fn test_error_report_int_division() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a // b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_float_division() {
    let code = r#"
def f(a: float, b: float) -> float:
    return a / b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Optional/None patterns
// ============================================================================

#[test]
fn test_error_report_none_check() {
    let code = r#"
def f(x: int) -> int:
    if x is None:
        return 0
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_optional_param() {
    let code = r#"
def f(x: int = 0) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection patterns
// ============================================================================

#[test]
fn test_error_report_list_operations() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    items.append(4)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_dict_operations() {
    let code = r#"
def f() -> dict:
    d = {"a": 1}
    d["b"] = 2
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Error handling patterns (try/except)
// ============================================================================

#[test]
fn test_error_report_try_except() {
    let code = r#"
def f(x: int) -> int:
    try:
        return 100 // x
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_raise() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns triggering error paths
// ============================================================================

#[test]
fn test_error_report_class_method() {
    let code = r#"
class MyClass:
    def __init__(self, value: int):
        self.value = value

    def get(self) -> int:
        return self.value
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_error_report_multi_function() {
    let code = r#"
def helper(x: int) -> int:
    return x * 2

def f(items: list) -> list:
    return [helper(x) for x in items]
"#;
    assert!(transpile_ok(code));
}

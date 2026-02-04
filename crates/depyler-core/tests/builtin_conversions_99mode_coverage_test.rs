//! Coverage tests for rust_gen/builtin_conversions.rs
//!
//! DEPYLER-99MODE-001: Targets builtin_conversions.rs (1,152 lines)
//! Covers: len(), int(), float(), str(), bool() builtin type conversions.

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
// len() builtin
// ============================================================================

#[test]
fn test_builtin_len_list() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_len_string() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_len_dict() {
    let code = r#"
def f(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_len_in_loop() {
    let code = r#"
def f(items: list) -> int:
    for i in range(len(items)):
        pass
    return len(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// int() builtin
// ============================================================================

#[test]
fn test_builtin_int_from_float() {
    let code = r#"
def f(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_int_from_str() {
    let code = r#"
def f(s: str) -> int:
    return int(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_int_from_bool() {
    let code = r#"
def f(b: bool) -> int:
    return int(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_int_literal() {
    let code = r#"
def f() -> int:
    return int(3.14)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// float() builtin
// ============================================================================

#[test]
fn test_builtin_float_from_int() {
    let code = r#"
def f(x: int) -> float:
    return float(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_float_from_str() {
    let code = r#"
def f(s: str) -> float:
    return float(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_float_literal() {
    let code = r#"
def f() -> float:
    return float(42)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// str() builtin
// ============================================================================

#[test]
fn test_builtin_str_from_int() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_str_from_float() {
    let code = r#"
def f(x: float) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_str_from_bool() {
    let code = r#"
def f(b: bool) -> str:
    return str(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_str_concat() {
    let code = r#"
def f(x: int) -> str:
    return "Value: " + str(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// bool() builtin
// ============================================================================

#[test]
fn test_builtin_bool_from_int() {
    let code = r#"
def f(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_bool_from_str() {
    let code = r#"
def f(s: str) -> bool:
    return bool(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Combined builtins
// ============================================================================

#[test]
fn test_builtin_chain_conversions() {
    let code = r#"
def f(x: int) -> str:
    return str(int(float(x)))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_in_expression() {
    let code = r#"
def f(items: list) -> int:
    return len(items) * 2 + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_len_comparison() {
    let code = r#"
def f(items: list) -> bool:
    return len(items) > 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_comprehensive() {
    let code = r#"
def f(items: list) -> str:
    n = len(items)
    total = 0
    for i in range(n):
        total += int(items[i])
    return str(total) + " from " + str(n) + " items"
"#;
    assert!(transpile_ok(code));
}

//! Coverage tests for rust_gen/string_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets string_analysis.rs (761 lines)
//! Covers: string type detection, owned string returns,
//! string method classification, f-string patterns.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_string_upper() {
    let code = "def f(s: str) -> str:\n    return s.upper()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_string_lower() {
    let code = "def f(s: str) -> str:\n    return s.lower()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_string_strip() {
    let code = "def f(s: str) -> str:\n    return s.strip()\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_string_replace() {
    let code = r#"
def f(s: str) -> str:
    return s.replace("a", "b")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_split() {
    let code = r#"
def f(s: str) -> list:
    return s.split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_join() {
    let code = r#"
def f(items: list) -> str:
    return ",".join(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_startswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.startswith("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_endswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.endswith(".py")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_find() {
    let code = r#"
def f(s: str) -> int:
    return s.find("x")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_isdigit() {
    let code = r#"
def f(s: str) -> bool:
    return s.isdigit()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_literal_return() {
    let code = r#"
def f() -> str:
    return "hello world"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_concatenation() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + " " + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_fstring() {
    let code = r#"
def f(name: str) -> str:
    return f"Hello, {name}!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_conditional_return() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_loop_build() {
    let code = r#"
def f(items: list) -> str:
    result = ""
    for item in items:
        result += str(item)
    return result
"#;
    assert!(transpile_ok(code));
}

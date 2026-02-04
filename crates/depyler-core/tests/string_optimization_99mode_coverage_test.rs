//! Coverage tests for string_optimization.rs
//!
//! DEPYLER-99MODE-001: Targets string_optimization.rs (1,302 lines)
//! Covers: static str detection, borrowed str, owned string,
//! string concatenation, interning, Cow patterns, string in collections.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Static string (read-only literal)
// ============================================================================

#[test]
fn test_stropt_static_literal() {
    let code = r#"
def f():
    print("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_static_return() {
    let code = r#"
def f() -> str:
    return "result"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_multiple_literals() {
    let code = r#"
def f():
    print("a")
    print("b")
    print("c")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Borrowed string (immutable parameter)
// ============================================================================

#[test]
fn test_stropt_borrowed_param() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_borrowed_method() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_borrowed_comparison() {
    let code = r#"
def f(s: str) -> bool:
    return s == "target"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Owned string (concatenation/mutation)
// ============================================================================

#[test]
fn test_stropt_owned_concat() {
    let code = r#"
def f(a: str, b: str) -> str:
    return a + " " + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_owned_mutation() {
    let code = r#"
def f(s: str) -> str:
    s = "new value"
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_owned_fstring() {
    let code = r#"
def f(name: str, age: int) -> str:
    return f"Hello {name}, age {age}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_owned_loop_concat() {
    let code = r#"
def f(items: list) -> str:
    result = ""
    for item in items:
        result += str(item) + ", "
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String in collections
// ============================================================================

#[test]
fn test_stropt_string_in_list() {
    let code = r#"
def f() -> list:
    return ["hello", "world"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_string_in_dict() {
    let code = r#"
def f() -> dict:
    return {"key": "value", "name": "test"}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_string_in_set() {
    let code = r#"
def f() -> int:
    words = {"hello", "world", "test"}
    return len(words)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String method chains
// ============================================================================

#[test]
fn test_stropt_method_chain() {
    let code = r#"
def f(text: str) -> list:
    return text.strip().lower().split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_replace_chain() {
    let code = r#"
def f(text: str) -> str:
    return text.replace("old", "new").strip()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Conditional string patterns
// ============================================================================

#[test]
fn test_stropt_conditional_string() {
    let code = r#"
def f(flag: bool) -> str:
    if flag:
        return "yes"
    else:
        return "no"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_ternary_string() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String in loop patterns
// ============================================================================

#[test]
fn test_stropt_string_in_for() {
    let code = r#"
def f(items: list):
    for item in items:
        print(str(item))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_string_join() {
    let code = r#"
def f(items: list) -> str:
    return ", ".join(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex string patterns
// ============================================================================

#[test]
fn test_stropt_word_processing() {
    let code = r#"
def f(text: str) -> list:
    words = text.split()
    result = []
    for word in words:
        if len(word) > 3:
            result.append(word.upper())
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stropt_string_builder() {
    let code = r#"
def f(parts: list) -> str:
    result = ""
    for i in range(len(parts)):
        if i > 0:
            result += ", "
        result += parts[i]
    return result
"#;
    assert!(transpile_ok(code));
}

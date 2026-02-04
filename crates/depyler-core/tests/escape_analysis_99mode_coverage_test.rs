//! Coverage tests for escape_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets escape_analysis.rs (1,280 lines)
//! Covers: ownership inference, use-after-move detection, aliasing,
//! mutability requirements, strategic cloning, escape through return.

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
// Simple ownership patterns
// ============================================================================

#[test]
fn test_escape_simple_return() {
    let code = r#"
def f(x: int) -> int:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_no_escape() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_string_return() {
    let code = r#"
def f(s: str) -> str:
    return s
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable reassignment and aliasing
// ============================================================================

#[test]
fn test_escape_reassignment() {
    let code = r#"
def f() -> int:
    x = 1
    x = 2
    x = 3
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_alias_int() {
    let code = r#"
def f() -> int:
    a = 42
    b = a
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_alias_list() {
    let code = r#"
def f() -> list:
    a = [1, 2, 3]
    b = a
    return b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Mutability requirements
// ============================================================================

#[test]
fn test_escape_mutable_list() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    items.append(2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_mutable_dict() {
    let code = r#"
def f() -> dict:
    d = {}
    d["a"] = 1
    d["b"] = 2
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_mutable_counter() {
    let code = r#"
def f(n: int) -> int:
    total = 0
    for i in range(n):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Escape through collections
// ============================================================================

#[test]
fn test_escape_in_list_literal() {
    let code = r#"
def f(a: int, b: int) -> list:
    return [a, b, a + b]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_in_dict_literal() {
    let code = r#"
def f(key: str, val: int) -> dict:
    return {key: val}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_in_tuple() {
    let code = r#"
def f(x: int, y: int) -> tuple:
    return (x, y)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Use-after-move patterns
// ============================================================================

#[test]
fn test_escape_multi_use() {
    let code = r#"
def f(items: list) -> int:
    total = sum(items)
    count = len(items)
    return total + count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_loop_use() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total + len(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method chain patterns
// ============================================================================

#[test]
fn test_escape_method_chain() {
    let code = r#"
def f(s: str) -> str:
    return s.strip().lower().replace("a", "b")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_method_on_param() {
    let code = r#"
def f(items: list) -> list:
    items.sort()
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex escape patterns
// ============================================================================

#[test]
fn test_escape_conditional_return() {
    let code = r#"
def f(items: list, flag: bool) -> list:
    if flag:
        return items
    return []
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_loop_accumulate() {
    let code = r#"
def f(data: list) -> list:
    result = []
    for item in data:
        if item > 0:
            result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_class_fields() {
    let code = r#"
class Container:
    def __init__(self):
        self.items = []

    def add(self, item: int):
        self.items.append(item)

    def get_items(self) -> list:
        return self.items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_complex_algorithm() {
    let code = r#"
def merge(a: list, b: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    return result
"#;
    assert!(transpile_ok(code));
}

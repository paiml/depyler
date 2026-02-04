//! Coverage tests for lifetime_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets lifetime_analysis.rs (1,757 lines)
//! Covers: lifetime inference, borrowing analysis, parameter mutation,
//! escape detection, elision rules, comprehension captures, loop usage.

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
// Basic lifetime inference - read-only parameters
// ============================================================================

#[test]
fn test_lifetime_read_only_int() {
    let code = r#"
def f(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_read_only_string() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_read_only_list() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Mutation detection
// ============================================================================

#[test]
fn test_lifetime_mutated_list_append() {
    let code = r#"
def f(items: list) -> list:
    items.append(42)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_mutated_assignment() {
    let code = r#"
def f(x: int) -> int:
    x = x + 1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_mutated_index() {
    let code = r#"
def f(items: list) -> list:
    items[0] = 99
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_swap_pattern() {
    let code = r#"
def f(items: list) -> list:
    items[0], items[1] = items[1], items[0]
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Escape detection
// ============================================================================

#[test]
fn test_lifetime_escape_through_return() {
    let code = r#"
def f(s: str) -> str:
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_escape_in_collection() {
    let code = r#"
def f(a: int, b: int) -> list:
    return [a, b]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_escape_in_tuple() {
    let code = r#"
def f(x: int, y: int) -> tuple:
    return (x, y)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop usage
// ============================================================================

#[test]
fn test_lifetime_used_in_for_loop() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_used_in_while_loop() {
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

#[test]
fn test_lifetime_loop_with_index() {
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
// Comprehension captures
// ============================================================================

#[test]
fn test_lifetime_list_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_filtered_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method calls on parameters
// ============================================================================

#[test]
fn test_lifetime_method_chain() {
    let code = r#"
def f(s: str) -> str:
    return s.strip().lower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_string_methods() {
    let code = r#"
def f(text: str) -> str:
    return text.upper()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Elision rules
// ============================================================================

#[test]
fn test_lifetime_single_param_elision() {
    let code = r#"
def f(x: int) -> int:
    return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_no_reference_elision() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_self_elision() {
    let code = r#"
class MyClass:
    def __init__(self):
        self.data = 0

    def get_data(self) -> int:
        return self.data
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_lifetime_multiple_params() {
    let code = r#"
def f(a: str, b: str, c: int) -> str:
    if c > 0:
        return a
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_nested_function_calls() {
    let code = r#"
def f(items: list) -> int:
    return sum(items) + len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_conditional_mutation() {
    let code = r#"
def f(items: list, flag: bool) -> list:
    if flag:
        items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_complex_control_flow() {
    let code = r#"
def f(data: list) -> int:
    total = 0
    for item in data:
        if item > 0:
            total += item
        elif item < -10:
            break
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lifetime_dict_parameter() {
    let code = r#"
def f(d: dict) -> list:
    keys = []
    for k in d:
        keys.append(k)
    return keys
"#;
    assert!(transpile_ok(code));
}

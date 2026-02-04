//! Coverage tests for rust_gen/control_flow_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets control_flow_analysis.rs (1,008 lines)
//! Covers: return path analysis, nested function detection,
//! escaping variable collection, variable usage tracking.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Return path analysis - always returns
// ============================================================================

#[test]
fn test_cfa_always_returns_simple() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_always_returns_if_else() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return 1
    else:
        return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_always_returns_elif() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_always_returns_raise() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nested function detection
// ============================================================================

#[test]
fn test_cfa_nested_function() {
    let code = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_nested_in_if() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        def helper(y: int) -> int:
            return y + 1
        return helper(x)
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// If-escaping variables (DEPYLER-0834)
// ============================================================================

#[test]
fn test_cfa_if_escaping_var() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        result = x * 2
    else:
        result = 0
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_if_escaping_multiple() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        a = x
        b = x * 2
    else:
        a = 0
        b = 0
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_if_escaping_nested() {
    let code = r#"
def f(x: int) -> str:
    if x > 100:
        label = "large"
    elif x > 10:
        label = "medium"
    elif x > 0:
        label = "small"
    else:
        label = "non-positive"
    return label
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Loop-escaping variables (DEPYLER-0762)
// ============================================================================

#[test]
fn test_cfa_loop_escaping_var() {
    let code = r#"
def f(items: list) -> int:
    last = 0
    for item in items:
        last = item
    return last
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_loop_escaping_accumulator() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    count = 0
    for item in items:
        total += item
        count += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_loop_escaping_best() {
    let code = r#"
def f(items: list) -> int:
    best = 0
    for item in items:
        if item > best:
            best = item
    return best
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Variable usage tracking
// ============================================================================

#[test]
fn test_cfa_var_used_in_expr() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x * y + x - y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_var_used_in_condition() {
    let code = r#"
def f(x: int) -> bool:
    return x > 0 and x < 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_var_used_in_method_call() {
    let code = r#"
def f(items: list) -> int:
    items.append(42)
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_var_used_in_index() {
    let code = r#"
def f(items: list, idx: int) -> int:
    return items[idx]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex control flow patterns
// ============================================================================

#[test]
fn test_cfa_try_except_variable() {
    let code = r#"
def f(x: int) -> int:
    try:
        result = 100 // x
    except:
        result = 0
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_while_with_break() {
    let code = r#"
def f(items: list, target: int) -> int:
    idx = -1
    i = 0
    while i < len(items):
        if items[i] == target:
            idx = i
            break
        i += 1
    return idx
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_for_with_continue() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total += item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_cfa_complex_escaping() {
    let code = r#"
def f(data: list) -> int:
    found = False
    result = 0
    for item in data:
        if item > 0:
            found = True
            result = item
            break
    if found:
        return result
    return -1
"#;
    assert!(transpile_ok(code));
}

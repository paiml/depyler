//! Coverage tests for ast_bridge/converters.rs
//!
//! DEPYLER-99MODE-001: Targets ast_bridge/converters.rs (1,289 lines)
//! Covers: Python AST→HIR conversion, statement types, assignment,
//! augmented assign, control flow, exception handling, context managers.

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
// Variable assignment
// ============================================================================

#[test]
fn test_ast_conv_simple_assign() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_type_annotated_assign() {
    let code = r#"
def f() -> int:
    x: int = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_multiple_assign() {
    let code = r#"
def f() -> int:
    x = 1
    y = 2
    z = 3
    return x + y + z
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Augmented assignment
// ============================================================================

#[test]
fn test_ast_conv_augmented_add() {
    let code = r#"
def f() -> int:
    x = 0
    x += 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_augmented_sub() {
    let code = r#"
def f() -> int:
    x = 10
    x -= 3
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_augmented_mul() {
    let code = r#"
def f() -> int:
    x = 2
    x *= 5
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_augmented_div() {
    let code = r#"
def f() -> float:
    x = 10.0
    x /= 3.0
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - if/elif/else
// ============================================================================

#[test]
fn test_ast_conv_if_else() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "non-positive"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_elif() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_nested_if() {
    let code = r#"
def f(x: int, y: int) -> int:
    if x > 0:
        if y > 0:
            return x + y
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Control flow - loops
// ============================================================================

#[test]
fn test_ast_conv_for_range() {
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
fn test_ast_conv_for_list() {
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
fn test_ast_conv_while() {
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
fn test_ast_conv_break() {
    let code = r#"
def f(items: list) -> int:
    for item in items:
        if item < 0:
            break
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_continue() {
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

// ============================================================================
// Return statements
// ============================================================================

#[test]
fn test_ast_conv_return_value() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_return_expr() {
    let code = r#"
def f(x: int) -> int:
    return x * 2 + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_early_return() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        return 0
    return x
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Exception handling
// ============================================================================

#[test]
fn test_ast_conv_try_except() {
    let code = r#"
def f(x: int) -> int:
    try:
        return x // 1
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// List operations
// ============================================================================

#[test]
fn test_ast_conv_list_append() {
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
fn test_ast_conv_dict_assign() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Tuple unpacking
// ============================================================================

#[test]
fn test_ast_conv_tuple_unpack() {
    let code = r#"
def f() -> int:
    a, b = 1, 2
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_ast_conv_swap() {
    let code = r#"
def f(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_ast_conv_nested_loops() {
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
fn test_ast_conv_comprehensive() {
    let code = r#"
def process(items: list) -> dict:
    result = {}
    count = 0
    for item in items:
        if item > 0:
            key = str(item)
            result[key] = item * 2
            count += 1
    return result
"#;
    assert!(transpile_ok(code));
}

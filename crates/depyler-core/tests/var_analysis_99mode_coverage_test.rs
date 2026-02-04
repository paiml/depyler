//! Coverage tests for rust_gen/var_analysis.rs
//!
//! DEPYLER-99MODE-001: Targets var_analysis.rs (2,711 lines)
//! Covers: variable mutability analysis, usage tracking,
//! scope analysis, reassignment detection, collection mutation.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Variable mutability detection
// ============================================================================

#[test]
fn test_var_analysis_immutable_var() {
    let code = "def f() -> int:\n    x = 42\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_mutable_reassign() {
    let code = "def f() -> int:\n    x = 0\n    x = 10\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_mutable_augmented() {
    let code = "def f() -> int:\n    x = 0\n    x += 5\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_mutable_in_loop() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_mutable_list_append() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_mutable_dict_assign() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_mutable_set_add() {
    let code = r#"
def f() -> set:
    s = set()
    s.add(1)
    return s
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Scope analysis
// ============================================================================

#[test]
fn test_var_analysis_if_scope() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        y = x * 2
    else:
        y = 0
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_loop_scope() {
    let code = r#"
def f() -> int:
    result = 0
    for i in range(5):
        temp = i * 2
        result += temp
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_nested_scope() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(5):
        for j in range(5):
            total += i * j
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Usage tracking
// ============================================================================

#[test]
fn test_var_analysis_used_once() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_used_multiple() {
    let code = "def f(x: int) -> int:\n    return x + x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_unused_var() {
    let code = r#"
def f() -> int:
    unused = 42
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_param_used() {
    let code = r#"
def f(a: int, b: int) -> int:
    return a + b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns
// ============================================================================

#[test]
fn test_var_analysis_swap() {
    let code = r#"
def f() -> int:
    a, b = 1, 2
    a, b = b, a
    return a
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_accumulator() {
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
fn test_var_analysis_conditional_assign() {
    let code = r#"
def f(x: int) -> str:
    if x > 0:
        result = "positive"
    elif x < 0:
        result = "negative"
    else:
        result = "zero"
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_list_building() {
    let code = r#"
def f(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i * i)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_dict_building() {
    let code = r#"
def f(items: list) -> dict:
    counts = {}
    for item in items:
        counts[item] = counts.get(item, 0) + 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_var_analysis_class_self() {
    let code = r#"
class Obj:
    def __init__(self, val: int):
        self.val = val

    def update(self, new_val: int):
        self.val = new_val

    def get(self) -> int:
        return self.val
"#;
    assert!(transpile_ok(code));
}

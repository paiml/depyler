//! Coverage tests for borrowing_context.rs
//!
//! DEPYLER-99MODE-001: Targets borrowing_context.rs (1,733 lines)
//! Covers: borrowing analysis, ownership inference, parameter usage patterns,
//! escape analysis, mutation detection, borrow strategy selection.

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
// Ownership - immutable borrow patterns
// ============================================================================

#[test]
fn test_borrow_read_only_param() {
    let code = "def f(x: int) -> int:\n    return x + 1\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_list() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_string() {
    let code = r#"
def f(s: str) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_read_only_dict() {
    let code = r#"
def f(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_multiple_reads() {
    let code = r#"
def f(x: int, y: int) -> int:
    return x + y + x * y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - mutable borrow patterns
// ============================================================================

#[test]
fn test_borrow_mutable_list_append() {
    let code = r#"
def f(items: list) -> list:
    items.append(42)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutable_dict_assign() {
    let code = r#"
def f(d: dict) -> dict:
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutable_augmented_assign() {
    let code = r#"
def f(x: int) -> int:
    x += 10
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mutable_index_assign() {
    let code = r#"
def f(items: list, idx: int) -> list:
    items[idx] = 99
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - escape through return
// ============================================================================

#[test]
fn test_borrow_escape_return_param() {
    let code = r#"
def f(items: list) -> list:
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_escape_return_string() {
    let code = r#"
def f(s: str) -> str:
    return s + "!"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_escape_conditional_return() {
    let code = r#"
def f(a: list, b: list, flag: bool) -> list:
    if flag:
        return a
    return b
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - loop context usage
// ============================================================================

#[test]
fn test_borrow_param_in_for_loop() {
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
fn test_borrow_param_in_while_loop() {
    let code = r#"
def f(items: list) -> int:
    i = 0
    total = 0
    while i < len(items):
        total += items[i]
        i += 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_param_in_nested_loops() {
    let code = r#"
def f(data: list) -> int:
    total = 0
    for i in range(len(data)):
        for j in range(i + 1, len(data)):
            total += data[i] * data[j]
    return total
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - comprehension capture
// ============================================================================

#[test]
fn test_borrow_param_in_list_comp() {
    let code = r#"
def f(items: list, threshold: int) -> list:
    return [x for x in items if x > threshold]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_param_in_dict_comp() {
    let code = r#"
def f(keys: list) -> dict:
    return {k: len(k) for k in keys}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - copy vs move types
// ============================================================================

#[test]
fn test_borrow_copy_type_int() {
    let code = r#"
def f(x: int) -> int:
    y = x
    z = x
    return y + z
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_copy_type_float() {
    let code = r#"
def f(x: float) -> float:
    y = x
    z = x
    return y + z
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_copy_type_bool() {
    let code = r#"
def f(flag: bool) -> bool:
    a = flag
    b = flag
    return a and b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_mixed_copy_noncopy() {
    let code = r#"
def f(x: int, s: str, items: list) -> int:
    return x + len(s) + len(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - method call patterns
// ============================================================================

#[test]
fn test_borrow_string_methods() {
    let code = r#"
def f(s: str) -> str:
    return s.upper().strip()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_list_methods_read() {
    let code = r#"
def f(items: list) -> int:
    return items.index(42)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_list_methods_mutate() {
    let code = r#"
def f(items: list) -> list:
    items.sort()
    items.reverse()
    return items
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Ownership - class field patterns
// ============================================================================

#[test]
fn test_borrow_class_field_read() {
    let code = r#"
class Container:
    def __init__(self, data: list):
        self.data = data

    def size(self) -> int:
        return len(self.data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_class_field_mutate() {
    let code = r#"
class Buffer:
    def __init__(self):
        self.items = []

    def add(self, item: int):
        self.items.append(item)

    def clear(self):
        self.items = []
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex borrowing scenarios
// ============================================================================

#[test]
fn test_borrow_param_passed_to_function() {
    let code = r#"
def helper(items: list) -> int:
    return sum(items)

def f(data: list) -> int:
    return helper(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_multiple_mutation_paths() {
    let code = r#"
def f(items: list, flag: bool) -> list:
    if flag:
        items.append(1)
    else:
        items.append(2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_loop_with_early_return() {
    let code = r#"
def f(items: list, target: int) -> bool:
    for item in items:
        if item == target:
            return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_complex_nested_access() {
    let code = r#"
def f(matrix: list) -> int:
    total = 0
    for row in matrix:
        for val in row:
            total += val
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_borrow_string_concatenation() {
    let code = r#"
def f(parts: list) -> str:
    result = ""
    for part in parts:
        result += str(part)
    return result
"#;
    assert!(transpile_ok(code));
}

//! EXTREME TDD: Tests for direct_rules_convert body conversion functions
//! Coverage target: convert_body, find_mutable_vars_in_body, convert_body_with_context

use depyler_core::hir::{AssignTarget, BinOp, HirExpr, HirStmt, Literal, Type};
use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ convert_body tests ============

#[test]
fn test_body_single_return() {
    assert!(transpile_ok("def f() -> int: return 42"));
}

#[test]
fn test_body_multiple_statements() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    z = y * 2
    return z
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_if() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_while() {
    let code = r#"
def f(n: int) -> int:
    result = 0
    while n > 0:
        result = result + n
        n = n - 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_for() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_nested_if() {
    let code = r#"
def f(x: int, y: int) -> int:
    if x > 0:
        if y > 0:
            return x + y
        else:
            return x - y
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_pass() {
    let code = r#"
def f() -> None:
    pass
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_break() {
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
fn test_body_with_continue() {
    let code = r#"
def f(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total = total + item
    return total
"#;
    assert!(transpile_ok(code));
}

// ============ find_mutable_vars_in_body tests ============

#[test]
fn test_mutable_vars_simple_assignment() {
    let code = r#"
def f(x: int) -> int:
    y = x + 1
    y = y * 2
    return y
"#;
    assert!(transpile_contains(code, "mut"));
}

#[test]
fn test_mutable_vars_loop_counter() {
    let code = r#"
def countdown(n: int) -> int:
    while n > 0:
        n = n - 1
    return n
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutable_vars_accumulator() {
    let code = r#"
def sum_to_n(n: int) -> int:
    total = 0
    i = 0
    while i < n:
        total = total + i
        i = i + 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutable_vars_in_if_branches() {
    let code = r#"
def f(x: int) -> int:
    result = 0
    if x > 0:
        result = x
    else:
        result = -x
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mutable_vars_augmented_assign() {
    let code = r#"
def f(x: int) -> int:
    x += 1
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ convert_body_with_context tests ============

#[test]
fn test_body_context_method_self() {
    let code = r#"
class Counter:
    count: int

    def increment(self) -> None:
        self.count = self.count + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_context_class_field_access() {
    let code = r#"
class Point:
    x: int
    y: int

    def distance_from_origin(self) -> float:
        return (self.x * self.x + self.y * self.y) ** 0.5
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_context_method_params() {
    let code = r#"
class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_context_static_method() {
    let code = r#"
class Utils:
    @staticmethod
    def double(x: int) -> int:
        return x * 2
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_context_complex_method() {
    let code = r#"
class BankAccount:
    balance: float

    def deposit(self, amount: float) -> None:
        if amount > 0:
            self.balance = self.balance + amount

    def withdraw(self, amount: float) -> bool:
        if amount <= self.balance:
            self.balance = self.balance - amount
            return True
        return False
"#;
    assert!(transpile_ok(code));
}

// ============ Edge cases ============

#[test]
fn test_body_empty_function() {
    let code = r#"
def f() -> None:
    pass
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_only_docstring() {
    let code = r#"
def f() -> None:
    """This is a docstring."""
    pass
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_multiple_returns() {
    let code = r#"
def abs_val(x: int) -> int:
    if x >= 0:
        return x
    return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_deeply_nested() {
    let code = r#"
def f(x: int, y: int, z: int) -> int:
    if x > 0:
        if y > 0:
            if z > 0:
                return x + y + z
            else:
                return x + y
        else:
            return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_mixed_control_flow() {
    let code = r#"
def process(items: list) -> int:
    result = 0
    for item in items:
        if item < 0:
            continue
        if item > 100:
            break
        while item > 0:
            result = result + 1
            item = item - 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_list_operations() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in items:
        result.append(item * 2)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_dict_operations() {
    let code = r#"
def f(data: dict) -> int:
    total = 0
    for key in data:
        total = total + data[key]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_string_operations() {
    let code = r#"
def f(s: str) -> str:
    result = s.upper()
    return result.strip()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_tuple_unpacking() {
    let code = r#"
def f(point: tuple) -> int:
    x, y = point
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_body_with_boolean_logic() {
    let code = r#"
def f(a: bool, b: bool) -> bool:
    if a and b:
        return True
    if a or b:
        return False
    return not a
"#;
    assert!(transpile_ok(code));
}

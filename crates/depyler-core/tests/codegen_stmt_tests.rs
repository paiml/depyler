//! EXTREME TDD: Tests for codegen.rs statement generation
//! Coverage: stmt_to_rust_tokens, handle_assign_target, handle_if_stmt, etc.

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new()
        .transpile(code)
        .map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ Assignment to tokens ============

#[test]
fn test_stmt_assign_simple() {
    let code = r#"
def f() -> int:
    x = 42
    return x
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "let"));
}

#[test]
fn test_stmt_assign_mutable() {
    let code = r#"
def f() -> int:
    x = 0
    x = x + 1
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_assign_typed() {
    let code = r#"
def f() -> int:
    x: int = 42
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_assign_tuple_unpack() {
    let code = r#"
def f(pair: tuple) -> int:
    a, b = pair
    return a + b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_assign_list_index() {
    let code = r#"
def f(items: list) -> list:
    items[0] = 100
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_assign_dict_key() {
    let code = r#"
def f(data: dict) -> dict:
    data["key"] = "value"
    return data
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_assign_attr() {
    let code = r#"
class Point:
    x: int

    def set_x(self, val: int) -> None:
        self.x = val
"#;
    assert!(transpile_ok(code));
}

// ============ If statement to tokens ============

#[test]
fn test_stmt_if_simple() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "if"));
}

#[test]
fn test_stmt_if_else() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return 1
    else:
        return -1
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "else"));
}

#[test]
fn test_stmt_if_elif() {
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
fn test_stmt_if_nested() {
    let code = r#"
def f(x: int, y: int) -> int:
    if x > 0:
        if y > 0:
            return 1
        return 2
    return 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_if_complex_condition() {
    let code = r#"
def f(a: int, b: int, c: int) -> bool:
    if a > 0 and (b < 0 or c == 0):
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

// ============ While statement to tokens ============

#[test]
fn test_stmt_while_simple() {
    let code = r#"
def countdown(n: int) -> int:
    while n > 0:
        n = n - 1
    return n
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "while"));
}

#[test]
fn test_stmt_while_with_break() {
    let code = r#"
def f(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] < 0:
            break
        i = i + 1
    return i
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "break"));
}

#[test]
fn test_stmt_while_with_continue() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    i = 0
    while i < len(items):
        if items[i] < 0:
            i = i + 1
            continue
        total = total + items[i]
        i = i + 1
    return total
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "continue"));
}

#[test]
fn test_stmt_while_true() {
    let code = r#"
def f() -> int:
    x = 0
    while True:
        x = x + 1
        if x > 10:
            break
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ For statement to tokens ============

#[test]
fn test_stmt_for_list() {
    let code = r#"
def sum_list(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "for"));
}

#[test]
fn test_stmt_for_range() {
    let code = r#"
def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total = total + i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_enumerate() {
    let code = r#"
def indexed_sum(items: list) -> int:
    total = 0
    for i, item in enumerate(items):
        total = total + i * item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_dict_keys() {
    let code = r#"
def sum_dict_values(data: dict) -> int:
    total = 0
    for key in data:
        total = total + data[key]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_dict_items() {
    let code = r#"
def sum_values(data: dict) -> int:
    total = 0
    for key, value in data.items():
        total = total + value
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_string() {
    let code = r#"
def count_a(s: str) -> int:
    count = 0
    for c in s:
        if c == "a":
            count = count + 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_nested() {
    let code = r#"
def matrix_sum(matrix: list) -> int:
    total = 0
    for row in matrix:
        for cell in row:
            total = total + cell
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_for_with_continue() {
    let code = r#"
def filter_positive(items: list) -> list:
    result = []
    for item in items:
        if item < 0:
            continue
        result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}

// ============ Return statement to tokens ============

#[test]
fn test_stmt_return_none() {
    let code = r#"
def f() -> None:
    return
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_return_literal() {
    let code = r#"
def f() -> int:
    return 42
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "return") || transpile_contains(code, "42"));
}

#[test]
fn test_stmt_return_expr() {
    let code = r#"
def f(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_return_call() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_return_tuple() {
    let code = r#"
def f(x: int) -> tuple:
    return (x, x + 1)
"#;
    assert!(transpile_ok(code));
}

// ============ Pass statement to tokens ============

#[test]
fn test_stmt_pass() {
    let code = r#"
def f() -> None:
    pass
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_pass_in_if() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        pass
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ Assert statement to tokens ============

#[test]
fn test_stmt_assert() {
    let code = r#"
def f(x: int) -> int:
    assert x > 0
    return x
"#;
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "assert"));
}

#[test]
fn test_stmt_assert_with_msg() {
    let code = r#"
def f(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ Raise statement to tokens ============

#[test]
fn test_stmt_raise() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ Expression statement to tokens ============

#[test]
fn test_stmt_expr_call() {
    let code = r#"
def f(items: list) -> list:
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_expr_method_chain() {
    let code = r#"
def f(items: list) -> list:
    items.append(1)
    items.append(2)
    items.append(3)
    return items
"#;
    assert!(transpile_ok(code));
}

// ============ Complex statement combinations ============

#[test]
fn test_stmt_complex_flow() {
    let code = r#"
def process(items: list, threshold: int) -> int:
    result = 0
    for item in items:
        if item < 0:
            continue
        if item > threshold:
            break
        result = result + item
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_nested_loops_with_conditions() {
    let code = r#"
def find_pair(matrix: list, target: int) -> tuple:
    for i, row in enumerate(matrix):
        for j, cell in enumerate(row):
            if cell == target:
                return (i, j)
    return (-1, -1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stmt_while_with_for() {
    let code = r#"
def mixed_loops(n: int) -> int:
    total = 0
    while n > 0:
        for i in range(n):
            total = total + i
        n = n - 1
    return total
"#;
    assert!(transpile_ok(code));
}

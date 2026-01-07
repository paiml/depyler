//! EXTREME TDD: Tests for direct_rules_convert statement functions
//! Coverage: convert_stmt, convert_stmt_with_mutable_vars, convert_stmt_with_context

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new().transpile(code).map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ Return statement tests ============

#[test]
fn test_return_none() {
    assert!(transpile_ok("def f() -> None:\n    return"));
}

#[test]
fn test_return_literal_int() {
    assert!(transpile_ok("def f() -> int:\n    return 42"));
}

#[test]
fn test_return_literal_float() {
    assert!(transpile_ok("def f() -> float:\n    return 3.14"));
}

#[test]
fn test_return_literal_string() {
    assert!(transpile_ok("def f() -> str:\n    return \"hello\""));
}

#[test]
fn test_return_literal_bool() {
    assert!(transpile_ok("def f() -> bool:\n    return True"));
}

#[test]
fn test_return_expression() {
    assert!(transpile_ok("def f(x: int) -> int:\n    return x + 1"));
}

#[test]
fn test_return_variable() {
    let code = r#"
def f(x: int) -> int:
    y = x * 2
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_return_call() {
    assert!(transpile_ok("def f(s: str) -> str:\n    return s.upper()"));
}

#[test]
fn test_return_list() {
    assert!(transpile_ok("def f() -> list:\n    return [1, 2, 3]"));
}

#[test]
fn test_return_dict() {
    assert!(transpile_ok("def f() -> dict:\n    return {\"a\": 1}"));
}

#[test]
fn test_return_tuple() {
    assert!(transpile_ok("def f() -> tuple:\n    return (1, 2)"));
}

// ============ If statement tests ============

#[test]
fn test_if_simple() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_if_else() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    else:
        return -x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_if_elif() {
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
fn test_if_multiple_elif() {
    let code = r#"
def grade(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    elif score >= 60:
        return "D"
    else:
        return "F"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_if_nested() {
    let code = r#"
def f(x: int, y: int) -> int:
    if x > 0:
        if y > 0:
            return 1
        else:
            return 2
    else:
        return 3
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_if_with_and() {
    let code = r#"
def f(x: int, y: int) -> bool:
    if x > 0 and y > 0:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_if_with_or() {
    let code = r#"
def f(x: int, y: int) -> bool:
    if x > 0 or y > 0:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_if_with_not() {
    let code = r#"
def f(x: bool) -> bool:
    if not x:
        return True
    return False
"#;
    assert!(transpile_ok(code));
}

// ============ While statement tests ============

#[test]
fn test_while_simple() {
    let code = r#"
def countdown(n: int) -> None:
    while n > 0:
        n = n - 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_with_accumulator() {
    let code = r#"
def sum_to(n: int) -> int:
    total = 0
    i = 1
    while i <= n:
        total = total + i
        i = i + 1
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_with_break() {
    let code = r#"
def find_first_negative(items: list) -> int:
    i = 0
    while i < len(items):
        if items[i] < 0:
            break
        i = i + 1
    return i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_while_with_continue() {
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
}

#[test]
fn test_while_true() {
    let code = r#"
def infinite_until_break(x: int) -> int:
    while True:
        if x > 100:
            break
        x = x + 1
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ For statement tests ============

#[test]
fn test_for_over_list() {
    let code = r#"
def sum_list(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_over_range() {
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
fn test_for_over_string() {
    let code = r#"
def count_chars(s: str) -> int:
    count = 0
    for c in s:
        count = count + 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_with_enumerate() {
    let code = r#"
def indexed_sum(items: list) -> int:
    total = 0
    for i, item in enumerate(items):
        total = total + i + item
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_for_nested() {
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
fn test_for_with_break() {
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
fn test_for_with_continue() {
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

// ============ Pass statement tests ============

#[test]
fn test_pass_in_function() {
    assert!(transpile_ok("def f() -> None:\n    pass"));
}

#[test]
fn test_pass_in_if() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        pass
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pass_in_else() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    else:
        pass
    return 0
"#;
    assert!(transpile_ok(code));
}

// ============ Expression statement tests ============

#[test]
fn test_expr_stmt_call() {
    let code = r#"
def f(items: list) -> None:
    items.append(1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_expr_stmt_method_chain() {
    let code = r#"
def f(s: str) -> None:
    s.upper().strip()
"#;
    assert!(transpile_ok(code));
}

// ============ Assert statement tests ============

#[test]
fn test_assert_simple() {
    let code = r#"
def f(x: int) -> int:
    assert x > 0
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_assert_with_message() {
    let code = r#"
def f(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ Raise statement tests ============

#[test]
fn test_raise_exception() {
    let code = r#"
def f(x: int) -> int:
    if x < 0:
        raise ValueError("negative value")
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_raise_generic() {
    let code = r#"
def f() -> None:
    raise Exception("error")
"#;
    assert!(transpile_ok(code));
}

// ============ Try/Except tests ============

#[test]
fn test_try_except_simple() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except:
        return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_try_except_finally() {
    let code = r#"
def f() -> int:
    x = 0
    try:
        x = 1
    except:
        x = -1
    finally:
        pass
    return x
"#;
    assert!(transpile_ok(code));
}

// ============ Complex statement combinations ============

#[test]
fn test_combined_control_flow() {
    let code = r#"
def process(items: list, threshold: int) -> int:
    result = 0
    for item in items:
        if item < 0:
            continue
        if item > threshold:
            break
        while item > 0:
            result = result + 1
            item = item - 1
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_everything() {
    let code = r#"
def complex_fn(data: list, limit: int) -> int:
    total = 0
    for row in data:
        for cell in row:
            if cell > 0:
                while cell > 0 and total < limit:
                    total = total + 1
                    cell = cell - 1
                    if total >= limit:
                        break
    return total
"#;
    assert!(transpile_ok(code));
}

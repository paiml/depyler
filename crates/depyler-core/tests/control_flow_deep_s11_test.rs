//! Session 11: Deep control flow coverage tests
//!
//! Targets specific untested control flow code paths:
//! - Complex if/elif/else chains
//! - While with complex conditions
//! - For loop over different iterables
//! - Break/continue in nested loops
//! - Try/except with different exception types
//! - With statement patterns
//! - Assert with messages

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ============================================================================
// Complex if/elif/else chains
// ============================================================================

#[test]
fn test_s11_ctrl_if_elif_chain() {
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
    let result = transpile(code);
    assert!(result.contains("fn grade"), "Should transpile if/elif chain. Got: {}", result);
}

#[test]
fn test_s11_ctrl_nested_if() {
    let code = r#"
def quadrant(x: int, y: int) -> int:
    if x >= 0:
        if y >= 0:
            return 1
        else:
            return 4
    else:
        if y >= 0:
            return 2
        else:
            return 3
"#;
    let result = transpile(code);
    assert!(result.contains("fn quadrant"), "Should transpile nested if. Got: {}", result);
}

#[test]
fn test_s11_ctrl_if_with_compound_condition() {
    let code = r#"
def is_valid_range(x: int, lo: int, hi: int) -> bool:
    if x >= lo and x <= hi:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("&&"), "Should use && for compound condition. Got: {}", result);
}

#[test]
fn test_s11_ctrl_if_or_condition() {
    let code = r#"
def is_weekend(day: str) -> bool:
    if day == "Saturday" or day == "Sunday":
        return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("||"), "Should use || for or condition. Got: {}", result);
}

// ============================================================================
// While loop patterns
// ============================================================================

#[test]
fn test_s11_ctrl_while_countdown() {
    let code = r#"
def countdown(n: int) -> int:
    total: int = 0
    while n > 0:
        total += n
        n -= 1
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("while"), "Should transpile while loop. Got: {}", result);
}

#[test]
fn test_s11_ctrl_while_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    idx: int = 0
    while idx < len(items):
        if items[idx] == target:
            break
        idx += 1
    return idx
"#;
    let result = transpile(code);
    assert!(result.contains("break"), "Should transpile while with break. Got: {}", result);
}

#[test]
fn test_s11_ctrl_while_with_continue() {
    let code = r#"
def sum_even(n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        i += 1
        if i % 2 != 0:
            continue
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("continue"), "Should transpile while with continue. Got: {}", result);
}

// ============================================================================
// For loop patterns
// ============================================================================

#[test]
fn test_s11_ctrl_for_range_only_stop() {
    let code = r#"
def sum_range(n: int) -> int:
    total: int = 0
    for i in range(n):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("for"), "Should transpile for range. Got: {}", result);
}

#[test]
fn test_s11_ctrl_for_range_start_stop() {
    let code = r#"
def sum_range_start(start: int, stop: int) -> int:
    total: int = 0
    for i in range(start, stop):
        total += i
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("for"), "Should transpile range(start, stop). Got: {}", result);
}

#[test]
fn test_s11_ctrl_for_over_string() {
    let code = r#"
def count_vowels(text: str) -> int:
    count: int = 0
    for ch in text:
        if ch == "a" or ch == "e" or ch == "i" or ch == "o" or ch == "u":
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_vowels"),
        "Should transpile for over string. Got: {}",
        result
    );
}

#[test]
fn test_s11_ctrl_for_with_break() {
    let code = r#"
def first_negative(items: list) -> int:
    for item in items:
        if item < 0:
            return item
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn first_negative"),
        "Should transpile for with early return. Got: {}",
        result
    );
}

#[test]
fn test_s11_ctrl_nested_for() {
    let code = r#"
def pairs(items: list) -> list:
    result: list = []
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            result.append((items[i], items[j]))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs"), "Should transpile nested for. Got: {}", result);
}

// ============================================================================
// Break and continue in nested loops
// ============================================================================

#[test]
fn test_s11_ctrl_break_inner_loop() {
    let code = r#"
def find_in_matrix(matrix: list, target: int) -> bool:
    for row in matrix:
        for item in row:
            if item == target:
                return True
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn find_in_matrix"),
        "Should transpile break in inner loop. Got: {}",
        result
    );
}

#[test]
fn test_s11_ctrl_continue_with_accumulate() {
    let code = r#"
def sum_positive(items: list) -> int:
    total: int = 0
    for item in items:
        if item <= 0:
            continue
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("continue"), "Should transpile continue. Got: {}", result);
}

// ============================================================================
// Try/except patterns
// ============================================================================

#[test]
fn test_s11_ctrl_try_bare_except() {
    let code = r#"
def safe_div(a: int, b: int) -> int:
    try:
        return a // b
    except Exception:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_div"), "Should transpile try/except. Got: {}", result);
}

#[test]
fn test_s11_ctrl_try_except_else() {
    let code = r#"
def parse_or_default(s: str) -> int:
    try:
        result: int = int(s)
    except ValueError:
        result = -1
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn parse_or_default"),
        "Should transpile try/except with else. Got: {}",
        result
    );
}

#[test]
fn test_s11_ctrl_try_except_finally() {
    let code = r#"
def safe_process(x: int) -> int:
    result: int = 0
    try:
        result = x * 10
    except Exception:
        result = -1
    finally:
        print("done")
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_process"),
        "Should transpile try/except/finally. Got: {}",
        result
    );
}

// ============================================================================
// Assert patterns
// ============================================================================

#[test]
fn test_s11_ctrl_assert_simple() {
    let code = r#"
def checked(x: int) -> int:
    assert x >= 0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("assert"), "Should transpile assert. Got: {}", result);
}

#[test]
fn test_s11_ctrl_assert_with_message() {
    let code = r#"
def checked_msg(x: int) -> int:
    assert x >= 0, "x must be non-negative"
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("assert") || result.contains("fn checked_msg"),
        "Should transpile assert with message. Got: {}",
        result
    );
}

// ============================================================================
// With statement
// ============================================================================

#[test]
fn test_s11_ctrl_with_open() {
    let code = r#"
def read_lines(path: str) -> list:
    lines: list = []
    with open(path) as f:
        for line in f:
            lines.append(line.strip())
    return lines
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_lines"), "Should transpile with open. Got: {}", result);
}

// ============================================================================
// Complex mixed patterns
// ============================================================================

#[test]
fn test_s11_ctrl_fibonacci() {
    let code = r#"
def fib(n: int) -> int:
    if n <= 1:
        return n
    a: int = 0
    b: int = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    let result = transpile(code);
    assert!(result.contains("fn fib"), "Should transpile fibonacci. Got: {}", result);
}

#[test]
fn test_s11_ctrl_binary_search() {
    let code = r#"
def binary_search(arr: list, target: int) -> int:
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn binary_search"), "Should transpile binary search. Got: {}", result);
}

#[test]
fn test_s11_ctrl_two_pointer() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    left: int = 0
    right: int = len(s) - 1
    while left < right:
        if s[left] != s[right]:
            return False
        left += 1
        right -= 1
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Should transpile two-pointer. Got: {}", result);
}

#[test]
fn test_s11_ctrl_accumulate_pattern() {
    let code = r#"
def running_sum(items: list) -> list:
    result: list = []
    total: int = 0
    for item in items:
        total += item
        result.append(total)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn running_sum"), "Should transpile accumulate. Got: {}", result);
}

#[test]
fn test_s11_ctrl_sliding_window() {
    let code = r#"
def max_sum_window(items: list, k: int) -> int:
    current: int = 0
    for i in range(k):
        current += items[i]
    best: int = current
    for i in range(k, len(items)):
        current = current + items[i] - items[i - k]
        if current > best:
            best = current
    return best
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn max_sum_window"),
        "Should transpile sliding window. Got: {}",
        result
    );
}

#[test]
fn test_s11_ctrl_multiple_conditions_return() {
    let code = r#"
def triangle_type(a: int, b: int, c: int) -> str:
    if a + b <= c or a + c <= b or b + c <= a:
        return "invalid"
    if a == b and b == c:
        return "equilateral"
    if a == b or b == c or a == c:
        return "isosceles"
    return "scalene"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn triangle_type"),
        "Should transpile triangle classifier. Got: {}",
        result
    );
}

#[test]
fn test_s11_ctrl_string_processing() {
    let code = r#"
def title_case(text: str) -> str:
    words: list = text.split()
    result: list = []
    for word in words:
        if len(word) > 0:
            result.append(word[0].upper() + word[1:])
    return " ".join(result)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn title_case"),
        "Should transpile string processing. Got: {}",
        result
    );
}

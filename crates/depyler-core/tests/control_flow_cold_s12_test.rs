//! Session 12 Batch 57: Control flow cold paths
//!
//! Targets control flow codegen cold paths:
//! - Complex while loop patterns
//! - Multiple nested loops with break/continue
//! - Switch-like elif chains
//! - Recursive patterns
//! - Complex conditional expressions

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== While loop patterns =====

#[test]
fn test_s12_b57_while_countdown() {
    let code = r#"
def countdown(n: int) -> list:
    result = []
    while n > 0:
        result.append(n)
        n -= 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn countdown"), "Got: {}", result);
}

#[test]
fn test_s12_b57_while_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    i = 0
    while i < len(items):
        if items[i] == target:
            break
        i += 1
    return i
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first"), "Got: {}", result);
}

#[test]
fn test_s12_b57_while_with_continue() {
    let code = r#"
def sum_positive_while(items: list) -> int:
    total = 0
    i = 0
    while i < len(items):
        i += 1
        if items[i - 1] <= 0:
            continue
        total += items[i - 1]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positive_while"), "Got: {}", result);
}

#[test]
fn test_s12_b57_while_true_break() {
    let code = r#"
def read_until_done(items: list) -> list:
    result = []
    i = 0
    while True:
        if i >= len(items):
            break
        result.append(items[i])
        i += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_until_done"), "Got: {}", result);
}

// ===== Nested loop patterns =====

#[test]
fn test_s12_b57_nested_for() {
    let code = r#"
def pairs(n: int) -> list:
    result = []
    for i in range(n):
        for j in range(i + 1, n):
            result.append((i, j))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b57_triple_nested() {
    let code = r#"
def triplets(n: int) -> list:
    result = []
    for i in range(n):
        for j in range(n):
            for k in range(n):
                if i + j + k == n:
                    result.append((i, j, k))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn triplets"), "Got: {}", result);
}

// ===== Complex elif chains =====

#[test]
fn test_s12_b57_grade_calculator() {
    let code = r##"
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
"##;
    let result = transpile(code);
    assert!(result.contains("fn grade"), "Got: {}", result);
}

#[test]
fn test_s12_b57_day_name() {
    let code = r##"
def day_name(n: int) -> str:
    if n == 0:
        return "Monday"
    elif n == 1:
        return "Tuesday"
    elif n == 2:
        return "Wednesday"
    elif n == 3:
        return "Thursday"
    elif n == 4:
        return "Friday"
    elif n == 5:
        return "Saturday"
    elif n == 6:
        return "Sunday"
    else:
        return "Invalid"
"##;
    let result = transpile(code);
    assert!(result.contains("fn day_name"), "Got: {}", result);
}

// ===== Recursive patterns =====

#[test]
fn test_s12_b57_recursive_factorial() {
    let code = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn factorial"), "Got: {}", result);
}

#[test]
fn test_s12_b57_recursive_sum() {
    let code = r#"
def rec_sum(items: list) -> int:
    if len(items) == 0:
        return 0
    return items[0] + rec_sum(items[1:])
"#;
    let result = transpile(code);
    assert!(result.contains("fn rec_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b57_recursive_power() {
    let code = r#"
def fast_pow(base: int, exp: int) -> int:
    if exp == 0:
        return 1
    if exp % 2 == 0:
        half = fast_pow(base, exp // 2)
        return half * half
    return base * fast_pow(base, exp - 1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn fast_pow"), "Got: {}", result);
}

#[test]
fn test_s12_b57_recursive_flatten() {
    let code = r#"
def tree_depth(node: dict) -> int:
    if not node:
        return 0
    left = tree_depth(node.get("left", {}))
    right = tree_depth(node.get("right", {}))
    return 1 + max(left, right)
"#;
    let result = transpile(code);
    assert!(result.contains("fn tree_depth"), "Got: {}", result);
}

// ===== Complex conditional expressions =====

#[test]
fn test_s12_b57_nested_ternary() {
    let code = r##"
def sign(n: int) -> str:
    return "positive" if n > 0 else ("negative" if n < 0 else "zero")
"##;
    let result = transpile(code);
    assert!(result.contains("fn sign"), "Got: {}", result);
}

#[test]
fn test_s12_b57_complex_and_or() {
    let code = r#"
def classify_char(c: str) -> str:
    if c.isalpha() and c.isupper():
        return "uppercase"
    elif c.isalpha() and c.islower():
        return "lowercase"
    elif c.isdigit():
        return "digit"
    else:
        return "other"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify_char"), "Got: {}", result);
}

// ===== Early return patterns =====

#[test]
fn test_s12_b57_guard_clauses() {
    let code = r#"
def process(items: list) -> int:
    if not items:
        return 0
    if len(items) == 1:
        return items[0]
    if len(items) == 2:
        return items[0] + items[1]
    return sum(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b57_validation_chain() {
    let code = r##"
def validate_email(email: str) -> str:
    if not email:
        return "empty"
    if "@" not in email:
        return "missing @"
    if "." not in email:
        return "missing domain"
    if email.startswith("@"):
        return "missing local part"
    return "valid"
"##;
    let result = transpile(code);
    assert!(result.contains("fn validate_email"), "Got: {}", result);
}

// ===== Loop with accumulator patterns =====

#[test]
fn test_s12_b57_sliding_window() {
    let code = r#"
def max_subarray_sum(items: list, k: int) -> int:
    if len(items) < k:
        return 0
    window_sum = 0
    for i in range(k):
        window_sum += items[i]
    best = window_sum
    for i in range(k, len(items)):
        window_sum += items[i] - items[i - k]
        if window_sum > best:
            best = window_sum
    return best
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_subarray_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b57_two_pointer() {
    let code = r#"
def two_sum_sorted(items: list, target: int) -> list:
    lo = 0
    hi = len(items) - 1
    while lo < hi:
        s = items[lo] + items[hi]
        if s == target:
            return [lo, hi]
        elif s < target:
            lo += 1
        else:
            hi -= 1
    return [-1, -1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn two_sum_sorted"), "Got: {}", result);
}

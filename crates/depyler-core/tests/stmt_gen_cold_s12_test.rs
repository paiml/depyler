//! Session 12 Batch 47: Statement generation cold paths
//!
//! Targets cold paths in stmt_gen.rs and stmt_gen_complex.rs:
//! - Assert with various comparison operators
//! - Raise bare and with cause
//! - With statement variants (no target, async)
//! - Try/except with multiple handlers
//! - For-else and while-else patterns
//! - Complex augmented assignment targets
//! - Break/continue in nested loops
//! - Complex return type propagation

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

// ===== Assert with various comparison operators =====

#[test]
fn test_s12_b47_assert_eq() {
    let code = r#"
def check_equal(a: int, b: int):
    assert a == b, "values must be equal"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_equal"), "Got: {}", result);
}

#[test]
fn test_s12_b47_assert_ne() {
    let code = r##"
def check_not_equal(a: int, b: int):
    assert a != b, f"expected different but got {a}"
"##;
    let result = transpile(code);
    assert!(result.contains("fn check_not_equal"), "Got: {}", result);
}

#[test]
fn test_s12_b47_assert_gt() {
    let code = r#"
def check_positive(n: int):
    assert n > 0, "must be positive"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b47_assert_lt() {
    let code = r#"
def check_small(n: int):
    assert n < 100, "must be less than 100"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_small"), "Got: {}", result);
}

#[test]
fn test_s12_b47_assert_no_message() {
    let code = r#"
def validate(x: int):
    assert x >= 0
    assert x <= 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s12_b47_assert_bool_expr() {
    let code = r#"
def check_valid(items: list):
    assert len(items) > 0, "list cannot be empty"
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_valid"), "Got: {}", result);
}

// ===== Raise patterns =====

#[test]
fn test_s12_b47_raise_value_error() {
    let code = r##"
def validate_age(age: int) -> int:
    if age < 0:
        raise ValueError(f"Invalid age: {age}")
    return age
"##;
    let result = transpile(code);
    assert!(result.contains("fn validate_age"), "Got: {}", result);
}

#[test]
fn test_s12_b47_raise_type_error() {
    let code = r#"
def process(value: str) -> str:
    if not isinstance(value, str):
        raise TypeError("expected string")
    return value.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b47_raise_runtime_error() {
    let code = r#"
def divide(a: float, b: float) -> float:
    if b == 0.0:
        raise RuntimeError("division by zero")
    return a / b
"#;
    let result = transpile(code);
    assert!(result.contains("fn divide"), "Got: {}", result);
}

#[test]
fn test_s12_b47_raise_bare() {
    let code = r#"
def safe_op(x: int) -> int:
    try:
        return x * 2
    except Exception:
        raise
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_op"), "Got: {}", result);
}

// ===== With statement patterns =====

#[test]
fn test_s12_b47_with_file_read() {
    let code = r#"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        content = f.read()
    return content
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_b47_with_file_write() {
    let code = r#"
def write_file(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_b47_with_no_as() {
    let code = r#"
def locked_op(x: int) -> int:
    with lock():
        return x * 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn locked_op"), "Got: {}", result);
}

// ===== Try/except complex patterns =====

#[test]
fn test_s12_b47_try_multiple_except() {
    let code = r##"
def safe_convert(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
    except Exception:
        return -3
"##;
    let result = transpile(code);
    assert!(result.contains("fn safe_convert"), "Got: {}", result);
}

#[test]
fn test_s12_b47_try_except_finally() {
    let code = r#"
def read_data(path: str) -> str:
    result = ""
    try:
        with open(path) as f:
            result = f.read()
    except IOError:
        result = "error"
    finally:
        pass
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_data"), "Got: {}", result);
}

#[test]
fn test_s12_b47_try_except_else() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    try:
        result = a / b
    except ZeroDivisionError:
        return 0.0
    else:
        return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_b47_nested_try() {
    let code = r##"
def robust_parse(data: str) -> int:
    try:
        try:
            return int(data)
        except ValueError:
            return int(float(data))
    except Exception:
        return 0
"##;
    let result = transpile(code);
    assert!(result.contains("fn robust_parse"), "Got: {}", result);
}

// ===== For-else and while-else =====

#[test]
fn test_s12_b47_for_else_search() {
    let code = r#"
def find_item(items: list, target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    else:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_item"), "Got: {}", result);
}

#[test]
fn test_s12_b47_while_else() {
    let code = r#"
def find_factor(n: int) -> int:
    i = 2
    while i * i <= n:
        if n % i == 0:
            return i
        i += 1
    else:
        return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_factor"), "Got: {}", result);
}

// ===== Complex augmented assignment =====

#[test]
fn test_s12_b47_augassign_dict_value() {
    let code = r##"
def count_chars(s: str) -> dict:
    counts = {}
    for c in s:
        if c in counts:
            counts[c] += 1
        else:
            counts[c] = 1
    return counts
"##;
    let result = transpile(code);
    assert!(result.contains("fn count_chars"), "Got: {}", result);
}

#[test]
fn test_s12_b47_augassign_list_element() {
    let code = r#"
def scale_in_place(items: list, factor: float):
    for i in range(len(items)):
        items[i] *= factor
"#;
    let result = transpile(code);
    assert!(result.contains("fn scale_in_place"), "Got: {}", result);
}

// ===== Nested break/continue =====

#[test]
fn test_s12_b47_nested_break() {
    let code = r#"
def find_pair(matrix: list, target: int) -> list:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return [i, j]
    return [-1, -1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pair"), "Got: {}", result);
}

#[test]
fn test_s12_b47_continue_with_condition() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b47_break_from_inner() {
    let code = r##"
def first_duplicate(items: list) -> str:
    seen = set()
    for item in items:
        if item in seen:
            return item
        seen.add(item)
    return ""
"##;
    let result = transpile(code);
    assert!(result.contains("fn first_duplicate"), "Got: {}", result);
}

// ===== Complex return patterns =====

#[test]
fn test_s12_b47_multi_return_paths() {
    let code = r##"
def classify(n: int) -> str:
    if n < 0:
        return "negative"
    elif n == 0:
        return "zero"
    elif n < 10:
        return "small"
    elif n < 100:
        return "medium"
    else:
        return "large"
"##;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s12_b47_return_from_nested() {
    let code = r#"
def deep_search(data: list, target: int) -> bool:
    for group in data:
        for item in group:
            if item == target:
                return True
    return False
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_search"), "Got: {}", result);
}

#[test]
fn test_s12_b47_return_conditional_expr() {
    let code = r#"
def max_of_three(a: int, b: int, c: int) -> int:
    m = a if a > b else b
    return m if m > c else c
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_of_three"), "Got: {}", result);
}

// ===== Pass statement =====

#[test]
fn test_s12_b47_pass_in_if() {
    let code = r#"
def noop_check(x: int) -> int:
    if x > 0:
        pass
    else:
        x = -x
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn noop_check"), "Got: {}", result);
}

#[test]
fn test_s12_b47_pass_in_except() {
    let code = r#"
def suppress_error(x: int) -> int:
    try:
        return x // 0
    except ZeroDivisionError:
        pass
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn suppress_error"), "Got: {}", result);
}

//! Session 12 Batch 40: stmt_gen cold paths
//!
//! Targets remaining cold paths in stmt_gen.rs:
//! - Complex assignment patterns (augmented assign to dict/list elements)
//! - For-else and while-else patterns
//! - Nested function definitions
//! - Complex class method bodies
//! - Multi-line expressions in statements
//! - Assert with complex conditions
//! - Global/nonlocal in nested context

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

// ===== Augmented assignment to container elements =====

#[test]
fn test_s12_b40_aug_assign_dict() {
    let code = r#"
def increment(d: dict, key: str):
    d[key] += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn increment"), "Got: {}", result);
}

#[test]
fn test_s12_b40_aug_assign_list() {
    let code = r#"
def double_at(items: list, idx: int):
    items[idx] *= 2
"#;
    let result = transpile(code);
    assert!(result.contains("fn double_at"), "Got: {}", result);
}

#[test]
fn test_s12_b40_assign_to_slice() {
    let code = r#"
def zero_range(items: list, start: int, end: int):
    for i in range(start, end):
        items[i] = 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_range"), "Got: {}", result);
}

// ===== For-else pattern =====

#[test]
fn test_s12_b40_for_else_found() {
    let code = r#"
def find_first_even(items: list) -> int:
    for item in items:
        if item % 2 == 0:
            return item
    else:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first_even"), "Got: {}", result);
}

#[test]
fn test_s12_b40_for_else_search() {
    let code = r#"
def find_target(items: list, target: int) -> bool:
    for item in items:
        if item == target:
            break
    else:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_target"), "Got: {}", result);
}

// ===== While-else pattern =====

#[test]
fn test_s12_b40_while_else() {
    let code = r#"
def find_power_of_two(n: int) -> int:
    x = 1
    while x < n:
        x *= 2
    else:
        return x
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_power_of_two"), "Got: {}", result);
}

// ===== Nested function definitions =====

#[test]
fn test_s12_b40_helper_function() {
    let code = r#"
def process(items: list) -> list:
    def helper(x: int) -> int:
        return x * 2 + 1
    result = []
    for item in items:
        result.append(helper(item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b40_nested_with_closure() {
    let code = r#"
def make_multiplier(factor: int):
    def multiply(x: int) -> int:
        return x * factor
    return multiply
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_multiplier"), "Got: {}", result);
}

// ===== Complex class patterns =====

#[test]
fn test_s12_b40_class_with_property() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def area(self) -> float:
        return 3.14159 * self.radius ** 2

    @property
    def circumference(self) -> float:
        return 2.0 * 3.14159 * self.radius
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
}

#[test]
fn test_s12_b40_class_method_complex() {
    let code = r##"
class Matrix:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data = []
        for i in range(rows):
            row = []
            for j in range(cols):
                row.append(0)
            self.data.append(row)

    def get(self, r: int, c: int) -> int:
        return self.data[r][c]

    def set_val(self, r: int, c: int, val: int):
        self.data[r][c] = val

    def add(self, other):
        result = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            for j in range(self.cols):
                result.data[i][j] = self.data[i][j] + other.data[i][j]
        return result

    def scale(self, factor: int):
        for i in range(self.rows):
            for j in range(self.cols):
                self.data[i][j] *= factor
"##;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

// ===== Complex control flow =====

#[test]
fn test_s12_b40_early_return_chain() {
    let code = r#"
def validate_password(pw: str) -> str:
    if len(pw) < 8:
        return "too short"
    has_upper = False
    has_lower = False
    has_digit = False
    for c in pw:
        if c.isupper():
            has_upper = True
        if c.islower():
            has_lower = True
        if c.isdigit():
            has_digit = True
    if not has_upper:
        return "needs uppercase"
    if not has_lower:
        return "needs lowercase"
    if not has_digit:
        return "needs digit"
    return "valid"
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_password"), "Got: {}", result);
}

#[test]
fn test_s12_b40_nested_loops_break() {
    let code = r#"
def find_pair(matrix: list, target: int) -> tuple:
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                return (i, j)
    return (-1, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pair"), "Got: {}", result);
}

#[test]
fn test_s12_b40_continue_with_condition() {
    let code = r#"
def sum_valid(items: list) -> int:
    total = 0
    for item in items:
        if item < 0:
            continue
        if item > 100:
            continue
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_valid"), "Got: {}", result);
}

// ===== Complex assert patterns =====

#[test]
fn test_s12_b40_assert_complex() {
    let code = r#"
def divide(a: int, b: int) -> float:
    assert b != 0, f"cannot divide by zero: {a}/{b}"
    return a / b
"#;
    let result = transpile(code);
    assert!(result.contains("fn divide"), "Got: {}", result);
}

// ===== Multiple except handlers =====

#[test]
fn test_s12_b40_multi_except() {
    let code = r#"
def safe_parse(text: str) -> int:
    try:
        return int(text)
    except ValueError:
        return 0
    except TypeError:
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_parse"), "Got: {}", result);
}

#[test]
fn test_s12_b40_try_finally() {
    let code = r#"
def with_cleanup(items: list) -> int:
    total = 0
    try:
        for item in items:
            total += item
    finally:
        total = max(total, 0)
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn with_cleanup"), "Got: {}", result);
}

// ===== String multiplication =====

#[test]
fn test_s12_b40_string_repeat() {
    let code = r#"
def banner(char: str, width: int) -> str:
    return char * width
"#;
    let result = transpile(code);
    assert!(result.contains("fn banner"), "Got: {}", result);
}

// ===== List multiplication =====

#[test]
fn test_s12_b40_list_repeat() {
    let code = r#"
def zeros(n: int) -> list:
    return [0] * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn zeros"), "Got: {}", result);
}

// ===== Complex function signatures =====

#[test]
fn test_s12_b40_many_params() {
    let code = r#"
def create_config(host: str, port: int, debug: bool, timeout: int, retries: int) -> dict:
    return {
        "host": host,
        "port": port,
        "debug": debug,
        "timeout": timeout,
        "retries": retries
    }
"#;
    let result = transpile(code);
    assert!(result.contains("fn create_config"), "Got: {}", result);
}

#[test]
fn test_s12_b40_star_args() {
    let code = r#"
def total(*args) -> int:
    result = 0
    for arg in args:
        result += arg
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn total"), "Got: {}", result);
}

// ===== Chained operations =====

#[test]
fn test_s12_b40_pipeline() {
    let code = r#"
def pipeline(text: str) -> list:
    words = text.lower().strip().split()
    unique = list(set(words))
    unique.sort()
    return unique
"#;
    let result = transpile(code);
    assert!(result.contains("fn pipeline"), "Got: {}", result);
}

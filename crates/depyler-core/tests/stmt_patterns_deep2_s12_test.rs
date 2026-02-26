//! Session 12 Batch 80: Statement patterns deep cold paths 2
//!
//! Targets stmt_gen.rs cold paths for complex statement patterns:
//! delete, assert variations, complex with blocks, yield patterns,
//! and deeply nested control flow.

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

// ===== Assert patterns =====

#[test]
fn test_s12_b80_assert_len() {
    let code = r#"
def check_nonempty(items: list):
    assert len(items) > 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_nonempty"), "Got: {}", result);
}

#[test]
fn test_s12_b80_assert_isinstance() {
    let code = r#"
def check_type(x):
    assert isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_type"), "Got: {}", result);
}

#[test]
fn test_s12_b80_assert_in() {
    let code = r#"
def check_contains(items: list, target: int):
    assert target in items
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_contains"), "Got: {}", result);
}

#[test]
fn test_s12_b80_assert_not_none() {
    let code = r#"
def check_not_none(x):
    assert x is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_not_none"), "Got: {}", result);
}

// ===== With statement patterns =====

#[test]
fn test_s12_b80_with_open_read() {
    let code = r##"
def read_file(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"##;
    let result = transpile(code);
    assert!(result.contains("fn read_file"), "Got: {}", result);
}

#[test]
fn test_s12_b80_with_open_write() {
    let code = r##"
def write_file(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)
"##;
    let result = transpile(code);
    assert!(result.contains("fn write_file"), "Got: {}", result);
}

#[test]
fn test_s12_b80_with_no_as() {
    let code = r##"
def touch_file(path: str):
    with open(path, "w"):
        pass
"##;
    let result = transpile(code);
    assert!(result.contains("fn touch_file"), "Got: {}", result);
}

// ===== Complex control flow =====

#[test]
fn test_s12_b80_for_else() {
    let code = r#"
def find_first_negative(items: list) -> int:
    for item in items:
        if item < 0:
            return item
    else:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first_negative"), "Got: {}", result);
}

#[test]
fn test_s12_b80_while_else() {
    let code = r#"
def find_power_of_two(n: int) -> int:
    val = 1
    while val < n:
        val *= 2
    else:
        return val
    return val
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_power_of_two"), "Got: {}", result);
}

#[test]
fn test_s12_b80_nested_break_continue() {
    let code = r#"
def find_pair_sum(items: list, target: int) -> tuple:
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            if items[i] + items[j] == target:
                return (i, j)
    return (-1, -1)
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_pair_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b80_multi_return() {
    let code = r#"
def classify(n: int) -> str:
    if n < 0:
        return "negative"
    if n == 0:
        return "zero"
    if n % 2 == 0:
        return "even"
    return "odd"
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

// ===== Augmented assignment patterns =====

#[test]
fn test_s12_b80_aug_assign_multiply() {
    let code = r#"
def scale_in_place(items: list, factor: int) -> list:
    for i in range(len(items)):
        items[i] *= factor
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn scale_in_place"), "Got: {}", result);
}

#[test]
fn test_s12_b80_aug_assign_subtract() {
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
fn test_s12_b80_aug_assign_divide() {
    let code = r#"
def halve_until_small(n: float) -> int:
    count = 0
    while n > 1.0:
        n /= 2.0
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve_until_small"), "Got: {}", result);
}

#[test]
fn test_s12_b80_aug_assign_string() {
    let code = r#"
def build_string(parts: list) -> str:
    result = ""
    for part in parts:
        result += str(part)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_string"), "Got: {}", result);
}

// ===== Async patterns =====

#[test]
fn test_s12_b80_async_function() {
    let code = r#"
async def fetch_data(url: str) -> str:
    return url
"#;
    let result = transpile(code);
    assert!(result.contains("async"), "Got: {}", result);
}

#[test]
fn test_s12_b80_async_with_await() {
    let code = r#"
async def process(url: str) -> str:
    data = await fetch(url)
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("async"), "Got: {}", result);
}

// ===== Docstrings =====

#[test]
fn test_s12_b80_function_docstring() {
    let code = r##"
def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b
"##;
    let result = transpile(code);
    assert!(result.contains("fn add"), "Got: {}", result);
}

#[test]
fn test_s12_b80_class_docstring() {
    let code = r##"
class Calculator:
    """A simple calculator class."""

    def __init__(self):
        self.result = 0

    def add(self, x: int):
        """Add x to result."""
        self.result += x
"##;
    let result = transpile(code);
    assert!(result.contains("Calculator"), "Got: {}", result);
}

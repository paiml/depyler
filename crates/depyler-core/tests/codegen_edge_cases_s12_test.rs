//! Session 12 Batch 39: Codegen edge cases targeting remaining cold paths
//!
//! Targets combined coverage of multiple codegen files:
//! - complex type annotations
//! - multi-value returns
//! - nested data structures
//! - complex string operations
//! - numeric edge cases
//! - collection comprehension variants

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

// ===== Complex type annotations =====

#[test]
fn test_s12_b39_typed_params() {
    let code = r#"
def process(items: list, threshold: float, prefix: str) -> list:
    result = []
    for item in items:
        if item > threshold:
            result.append(f"{prefix}{item}")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b39_optional_param() {
    let code = r#"
def find(items: list, target: int, start: int = 0) -> int:
    for i in range(start, len(items)):
        if items[i] == target:
            return i
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find"), "Got: {}", result);
}

// ===== Multi-value returns =====

#[test]
fn test_s12_b39_return_three() {
    let code = r#"
def min_max_avg(items: list) -> tuple:
    mn = items[0]
    mx = items[0]
    total = 0
    for item in items:
        if item < mn:
            mn = item
        if item > mx:
            mx = item
        total += item
    return (mn, mx, total / len(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max_avg"), "Got: {}", result);
}

#[test]
fn test_s12_b39_return_named() {
    let code = r#"
def analyze(text: str) -> dict:
    words = text.split()
    return {
        "count": len(words),
        "unique": len(set(words)),
        "first": words[0] if words else ""
    }
"#;
    let result = transpile(code);
    assert!(result.contains("fn analyze"), "Got: {}", result);
}

// ===== Nested data structures =====

#[test]
fn test_s12_b39_list_of_dicts() {
    let code = r#"
def get_names(records: list) -> list:
    names = []
    for record in records:
        names.append(record["name"])
    return names
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_names"), "Got: {}", result);
}

#[test]
fn test_s12_b39_dict_of_lists() {
    let code = r#"
def group_by_first_letter(words: list) -> dict:
    groups = {}
    for word in words:
        key = word[0]
        if key not in groups:
            groups[key] = []
        groups[key].append(word)
    return groups
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_first_letter"), "Got: {}", result);
}

#[test]
fn test_s12_b39_nested_dict_access() {
    let code = r#"
def deep_get(data: dict, keys: list):
    current = data
    for key in keys:
        current = current[key]
    return current
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_get"), "Got: {}", result);
}

// ===== Complex string operations =====

#[test]
fn test_s12_b39_format_table() {
    let code = r#"
def format_row(items: list, widths: list) -> str:
    parts = []
    for i in range(len(items)):
        parts.append(str(items[i]).ljust(widths[i]))
    return " | ".join(parts)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_row"), "Got: {}", result);
}

#[test]
fn test_s12_b39_wrap_text() {
    let code = r#"
def wrap(text: str, width: int) -> list:
    lines = []
    current = ""
    for word in text.split():
        if current and len(current) + 1 + len(word) > width:
            lines.append(current)
            current = word
        elif current:
            current += " " + word
        else:
            current = word
    if current:
        lines.append(current)
    return lines
"#;
    let result = transpile(code);
    assert!(result.contains("fn wrap"), "Got: {}", result);
}

// ===== Numeric operations =====

#[test]
fn test_s12_b39_float_comparison() {
    let code = r#"
def approx_equal(a: float, b: float, eps: float) -> bool:
    return abs(a - b) < eps
"#;
    let result = transpile(code);
    assert!(result.contains("fn approx_equal"), "Got: {}", result);
}

#[test]
fn test_s12_b39_clamp() {
    let code = r#"
def clamp(x: float, lo: float, hi: float) -> float:
    return max(lo, min(x, hi))
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

#[test]
fn test_s12_b39_lerp() {
    let code = r#"
def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * t
"#;
    let result = transpile(code);
    assert!(result.contains("fn lerp"), "Got: {}", result);
}

#[test]
fn test_s12_b39_mean_std() {
    let code = r#"
def mean(items: list) -> float:
    return sum(items) / len(items)

def variance(items: list) -> float:
    avg = mean(items)
    total = 0.0
    for x in items:
        total += (x - avg) ** 2
    return total / len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn mean"), "Got: {}", result);
    assert!(result.contains("fn variance"), "Got: {}", result);
}

// ===== Complex boolean logic =====

#[test]
fn test_s12_b39_leap_year() {
    let code = r#"
def is_leap(year: int) -> bool:
    return year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_leap"), "Got: {}", result);
}

#[test]
fn test_s12_b39_validate_date() {
    let code = r#"
def valid_date(year: int, month: int, day: int) -> bool:
    if month < 1 or month > 12:
        return False
    if day < 1 or day > 31:
        return False
    if month in [4, 6, 9, 11] and day > 30:
        return False
    if month == 2:
        if day > 29:
            return False
        if day == 29 and year % 4 != 0:
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn valid_date"), "Got: {}", result);
}

// ===== Complex comprehension patterns =====

#[test]
fn test_s12_b39_comp_enumerate() {
    let code = r#"
def indexed(items: list) -> list:
    return [(i, item) for i, item in enumerate(items)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed"), "Got: {}", result);
}

#[test]
fn test_s12_b39_comp_zip() {
    let code = r#"
def sum_pairs(a: list, b: list) -> list:
    return [x + y for x, y in zip(a, b)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b39_comp_ternary() {
    let code = r#"
def classify(items: list) -> list:
    return ["pos" if x > 0 else "neg" for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s12_b39_comp_transform_filter() {
    let code = r#"
def positive_squares(items: list) -> list:
    return [x * x for x in items if x > 0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn positive_squares"), "Got: {}", result);
}

// ===== Pattern matching on types =====

#[test]
fn test_s12_b39_type_check_pattern() {
    let code = r#"
def stringify(value) -> str:
    if isinstance(value, int):
        return str(value)
    elif isinstance(value, float):
        return f"{value:.2f}"
    elif isinstance(value, str):
        return value
    else:
        return "unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("fn stringify"), "Got: {}", result);
}

// ===== Complex class with properties =====

#[test]
fn test_s12_b39_bank_account() {
    let code = r##"
class BankAccount:
    def __init__(self, owner: str, balance: float):
        self.owner = owner
        self.balance = balance
        self.transactions = []

    def deposit(self, amount: float) -> float:
        if amount <= 0.0:
            raise ValueError("deposit must be positive")
        self.balance += amount
        self.transactions.append(amount)
        return self.balance

    def withdraw(self, amount: float) -> float:
        if amount <= 0.0:
            raise ValueError("withdrawal must be positive")
        if amount > self.balance:
            raise ValueError("insufficient funds")
        self.balance -= amount
        self.transactions.append(-amount)
        return self.balance

    def get_balance(self) -> float:
        return self.balance

    def get_transactions(self) -> list:
        return self.transactions

    def total_deposits(self) -> float:
        total = 0.0
        for t in self.transactions:
            if t > 0.0:
                total += t
        return total
"##;
    let result = transpile(code);
    assert!(result.contains("BankAccount"), "Got: {}", result);
}

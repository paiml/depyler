//! Session 12 Batch 32: Advanced patterns targeting remaining cold paths

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

#[test]
fn test_s12_generator_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_squares"), "Got: {}", result);
}

#[test]
fn test_s12_generator_any() {
    let code = r#"
def has_negative(items: list) -> bool:
    return any(x < 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_negative"), "Got: {}", result);
}

#[test]
fn test_s12_generator_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_positive"), "Got: {}", result);
}

#[test]
fn test_s12_generator_min() {
    let code = r#"
def shortest(words: list) -> int:
    return min(len(w) for w in words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn shortest"), "Got: {}", result);
}

#[test]
fn test_s12_generator_max() {
    let code = r#"
def longest(words: list) -> int:
    return max(len(w) for w in words)
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_method() {
    let code = r#"
def format_name(first: str, last: str) -> str:
    return f"{first.upper()} {last.lower()}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_name"), "Got: {}", result);
}

#[test]
fn test_s12_fstring_index() {
    let code = r#"
def first_char(name: str) -> str:
    return f"Initial: {name[0]}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_char"), "Got: {}", result);
}

#[test]
fn test_s12_comp_upper() {
    let code = r#"
def upper_words(text: str) -> list:
    return [w.upper() for w in text.split()]
"#;
    let result = transpile(code);
    assert!(result.contains("fn upper_words"), "Got: {}", result);
}

#[test]
fn test_s12_comp_cast() {
    let code = r#"
def parse_nums(items: list) -> list:
    return [int(x) for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_nums"), "Got: {}", result);
}

#[test]
fn test_s12_comp_filter() {
    let code = r#"
def long_words(text: str, n: int) -> list:
    return [w for w in text.split() if len(w) >= n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn long_words"), "Got: {}", result);
}

#[test]
fn test_s12_and_chain() {
    let code = r#"
def validate(x: int, y: int) -> bool:
    return x > 0 and y > 0 and x < 100 and y < 100
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate"), "Got: {}", result);
}

#[test]
fn test_s12_or_chain() {
    let code = r#"
def is_ws(c: str) -> bool:
    return c == " " or c == "\t" or c == "\n"
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_ws"), "Got: {}", result);
}

#[test]
fn test_s12_is_none() {
    let code = r#"
def default_val(value, d: int) -> int:
    if value is None:
        return d
    return value
"#;
    let result = transpile(code);
    assert!(result.contains("fn default_val"), "Got: {}", result);
}

#[test]
fn test_s12_is_not_none() {
    let code = r#"
def has_val(value) -> bool:
    return value is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn has_val"), "Got: {}", result);
}

#[test]
fn test_s12_enum_start() {
    let code = r#"
def number_lines(lines: list) -> list:
    result = []
    for i, line in enumerate(lines, 1):
        result.append(f"{i}: {line}")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn number_lines"), "Got: {}", result);
}

#[test]
fn test_s12_range_step() {
    let code = r#"
def evens(n: int) -> list:
    result = []
    for i in range(0, n, 2):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn evens"), "Got: {}", result);
}

#[test]
fn test_s12_range_rev() {
    let code = r#"
def countdown(n: int) -> list:
    result = []
    for i in range(n, 0, -1):
        result.append(i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn countdown"), "Got: {}", result);
}

#[test]
fn test_s12_quadratic() {
    let code = r#"
def quadratic(a: float, b: float, c: float) -> float:
    d = b * b - 4.0 * a * c
    return (-b + d ** 0.5) / (2.0 * a)
"#;
    let result = transpile(code);
    assert!(result.contains("fn quadratic"), "Got: {}", result);
}

#[test]
fn test_s12_normalize_list() {
    let code = r#"
def normalize(items: list) -> list:
    total = sum(items)
    if total == 0:
        return items
    return [x / total for x in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

#[test]
fn test_s12_return_tuple() {
    let code = r#"
def min_max(items: list) -> tuple:
    return (min(items), max(items))
"#;
    let result = transpile(code);
    assert!(result.contains("fn min_max"), "Got: {}", result);
}

#[test]
fn test_s12_return_dict_lit() {
    let code = r#"
def make_pt(x: int, y: int) -> dict:
    return {"x": x, "y": y}
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_pt"), "Got: {}", result);
}

#[test]
fn test_s12_return_list_lit() {
    let code = r#"
def make_pair(a: int, b: int) -> list:
    return [a, b]
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_pair"), "Got: {}", result);
}

#[test]
fn test_s12_percent_format() {
    let code = r#"
def fmt_float(x: float) -> str:
    return "Value: %.2f" % x
"#;
    let result = transpile(code);
    assert!(result.contains("fn fmt_float"), "Got: {}", result);
}

#[test]
fn test_s12_join_gen() {
    let code = r#"
def csv_line(values: list) -> str:
    return ",".join(str(v) for v in values)
"#;
    let result = transpile(code);
    assert!(result.contains("fn csv_line"), "Got: {}", result);
}

#[test]
fn test_s12_build_str() {
    let code = r#"
def build(parts: list) -> str:
    result = ""
    for p in parts:
        result += p
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build"), "Got: {}", result);
}

#[test]
fn test_s12_cls_factory() {
    let code = r#"
class Config:
    def __init__(self, host: str, port: int):
        self.host = host
        self.port = port

    @classmethod
    def default(cls):
        return cls("localhost", 8080)
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s12_sorted_class() {
    let code = r##"
class SortedList:
    def __init__(self):
        self.items = []

    def add(self, item: int):
        pos = 0
        for i in range(len(self.items)):
            if self.items[i] > item:
                break
            pos = i + 1
        self.items.insert(pos, item)

    def contains(self, item: int) -> bool:
        lo = 0
        hi = len(self.items) - 1
        while lo <= hi:
            mid = (lo + hi) // 2
            if self.items[mid] == item:
                return True
            elif self.items[mid] < item:
                lo = mid + 1
            else:
                hi = mid - 1
        return False
"##;
    let result = transpile(code);
    assert!(result.contains("SortedList"), "Got: {}", result);
}

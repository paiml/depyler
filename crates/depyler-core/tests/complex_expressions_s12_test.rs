//! Session 12 Batch 95: Complex expression combination patterns
//!
//! Targets expr_gen.rs cold paths by combining multiple expression
//! types in single functions to exercise interaction paths.

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
fn test_s12_b95_complex_arithmetic() {
    let code = r#"
def polynomial(x: float, coeffs: list) -> float:
    result = 0.0
    power = 1.0
    for coeff in coeffs:
        result += coeff * power
        power *= x
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn polynomial"), "Got: {}", result);
}

#[test]
fn test_s12_b95_matrix_determinant() {
    let code = r#"
def det2x2(m: list) -> float:
    return m[0][0] * m[1][1] - m[0][1] * m[1][0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn det2x2"), "Got: {}", result);
}

#[test]
fn test_s12_b95_string_encode() {
    let code = r#"
def caesar_encode(text: str, shift: int) -> str:
    result = ""
    for c in text:
        if c.isalpha():
            base = ord("a") if c.islower() else ord("A")
            result += chr((ord(c) - base + shift) % 26 + base)
        else:
            result += c
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn caesar_encode"), "Got: {}", result);
}

#[test]
fn test_s12_b95_complex_filter() {
    let code = r#"
def filter_and_transform(items: list, min_val: int, max_val: int) -> list:
    return [x * 2 for x in items if min_val <= x <= max_val]
"#;
    let result = transpile(code);
    assert!(result.contains("fn filter_and_transform"), "Got: {}", result);
}

#[test]
fn test_s12_b95_nested_dict_build() {
    let code = r##"
def build_index(records: list) -> dict:
    index = {}
    for record in records:
        key = record["type"]
        if key not in index:
            index[key] = []
        index[key].append(record["name"])
    return index
"##;
    let result = transpile(code);
    assert!(result.contains("fn build_index"), "Got: {}", result);
}

#[test]
fn test_s12_b95_multiple_generators() {
    let code = r#"
def stats_summary(data: list) -> dict:
    total = sum(x for x in data)
    count = len(data)
    pos = sum(1 for x in data if x > 0)
    neg = sum(1 for x in data if x < 0)
    return {"total": total, "count": count, "positive": pos, "negative": neg}
"#;
    let result = transpile(code);
    assert!(result.contains("fn stats_summary"), "Got: {}", result);
}

#[test]
fn test_s12_b95_complex_boolean_chain() {
    let code = r#"
def validate_input(name: str, age: int, email: str) -> bool:
    if not name or len(name) < 2:
        return False
    if age < 0 or age > 150:
        return False
    if "@" not in email or "." not in email:
        return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_input"), "Got: {}", result);
}

#[test]
fn test_s12_b95_complex_string_build() {
    let code = r##"
def format_table(headers: list, rows: list) -> str:
    widths = [len(h) for h in headers]
    for row in rows:
        for i, cell in enumerate(row):
            if len(str(cell)) > widths[i]:
                widths[i] = len(str(cell))
    lines = []
    header_line = " | ".join(h.ljust(widths[i]) for i, h in enumerate(headers))
    lines.append(header_line)
    sep = "-+-".join("-" * w for w in widths)
    lines.append(sep)
    for row in rows:
        line = " | ".join(str(cell).ljust(widths[i]) for i, cell in enumerate(row))
        lines.append(line)
    return "\n".join(lines)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_table"), "Got: {}", result);
}

#[test]
fn test_s12_b95_recursive_flatten() {
    let code = r#"
def deep_flatten(data) -> list:
    result = []
    if isinstance(data, list):
        for item in data:
            result.extend(deep_flatten(item))
    else:
        result.append(data)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_flatten"), "Got: {}", result);
}

#[test]
fn test_s12_b95_complex_dict_merge() {
    let code = r#"
def deep_merge(a: dict, b: dict) -> dict:
    result = dict(a)
    for key in b:
        if key in result and isinstance(result[key], dict) and isinstance(b[key], dict):
            result[key] = deep_merge(result[key], b[key])
        else:
            result[key] = b[key]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn deep_merge"), "Got: {}", result);
}

#[test]
fn test_s12_b95_complex_sort() {
    let code = r#"
def multi_key_sort(records: list) -> list:
    return sorted(records, key=lambda r: (r["priority"], r["name"]))
"#;
    let result = transpile(code);
    assert!(result.contains("fn multi_key_sort"), "Got: {}", result);
}

#[test]
fn test_s12_b95_number_formatter() {
    let code = r##"
def format_number(n: int) -> str:
    if n < 0:
        return "-" + format_number(-n)
    s = str(n)
    parts = []
    while len(s) > 3:
        parts.append(s[-3:])
        s = s[:-3]
    parts.append(s)
    parts.reverse()
    return ",".join(parts)
"##;
    let result = transpile(code);
    assert!(result.contains("fn format_number"), "Got: {}", result);
}

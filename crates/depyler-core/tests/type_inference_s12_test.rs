//! Session 12 Batch 41: Type inference cold paths
//!
//! Targets type inference branches that need specific type contexts:
//! - Functions returning different types based on conditions
//! - Type narrowing from isinstance checks
//! - Complex variable type evolution (reassignment)
//! - Mixed numeric types (int/float coercion)
//! - Collection type inference from operations
//! - Optional/None type handling patterns

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

// ===== Mixed numeric types =====

#[test]
fn test_s12_b41_int_float_mix() {
    let code = r#"
def convert(x: int) -> float:
    return float(x) / 3.0
"#;
    let result = transpile(code);
    assert!(result.contains("fn convert"), "Got: {}", result);
}

#[test]
fn test_s12_b41_float_int_cast() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn truncate"), "Got: {}", result);
}

#[test]
fn test_s12_b41_mixed_arithmetic() {
    let code = r#"
def weighted_avg(values: list, weights: list) -> float:
    total = 0.0
    weight_sum = 0.0
    for i in range(len(values)):
        total += values[i] * weights[i]
        weight_sum += weights[i]
    return total / weight_sum
"#;
    let result = transpile(code);
    assert!(result.contains("fn weighted_avg"), "Got: {}", result);
}

// ===== Variable type evolution =====

#[test]
fn test_s12_b41_var_reassign_type() {
    let code = r#"
def accumulate(items: list) -> str:
    result = 0
    for item in items:
        result += item
    return str(result)
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"), "Got: {}", result);
}

#[test]
fn test_s12_b41_var_conditional_init() {
    let code = r#"
def init_value(flag: bool) -> int:
    if flag:
        x = 1
    else:
        x = 0
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_value"), "Got: {}", result);
}

// ===== Collection type inference =====

#[test]
fn test_s12_b41_list_append_infer() {
    let code = r#"
def build_list(n: int) -> list:
    result = []
    for i in range(n):
        result.append(i * i)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_list"), "Got: {}", result);
}

#[test]
fn test_s12_b41_dict_build_infer() {
    let code = r#"
def build_dict(keys: list, default: int) -> dict:
    result = {}
    for key in keys:
        result[key] = default
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_dict"), "Got: {}", result);
}

#[test]
fn test_s12_b41_set_build_infer() {
    let code = r#"
def unique_chars(s: str) -> set:
    result = set()
    for c in s:
        result.add(c)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_chars"), "Got: {}", result);
}

// ===== None/Optional patterns =====

#[test]
fn test_s12_b41_optional_chain() {
    let code = r#"
def first_or_none(items: list):
    if len(items) > 0:
        return items[0]
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_or_none"), "Got: {}", result);
}

#[test]
fn test_s12_b41_none_guard() {
    let code = r#"
def safe_length(s) -> int:
    if s is None:
        return 0
    return len(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_length"), "Got: {}", result);
}

// ===== Complex return type inference =====

#[test]
fn test_s12_b41_conditional_return() {
    let code = r#"
def process(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    return "zero"
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b41_return_from_try() {
    let code = r#"
def parse_or_zero(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_or_zero"), "Got: {}", result);
}

// ===== String type operations =====

#[test]
fn test_s12_b41_str_to_int() {
    let code = r#"
def str_to_num(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn str_to_num"), "Got: {}", result);
}

#[test]
fn test_s12_b41_int_to_str() {
    let code = r#"
def num_to_str(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn num_to_str"), "Got: {}", result);
}

#[test]
fn test_s12_b41_float_to_str() {
    let code = r#"
def float_to_str(f: float) -> str:
    return str(f)
"#;
    let result = transpile(code);
    assert!(result.contains("fn float_to_str"), "Got: {}", result);
}

#[test]
fn test_s12_b41_bool_to_int() {
    let code = r#"
def count_true(flags: list) -> int:
    total = 0
    for flag in flags:
        total += int(flag)
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_true"), "Got: {}", result);
}

// ===== Complex expression typing =====

#[test]
fn test_s12_b41_ternary_type() {
    let code = r#"
def max_or_zero(x: int) -> int:
    return x if x > 0 else 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn max_or_zero"), "Got: {}", result);
}

#[test]
fn test_s12_b41_comp_type_infer() {
    let code = r#"
def str_lengths(items: list) -> list:
    return [len(s) for s in items]
"#;
    let result = transpile(code);
    assert!(result.contains("fn str_lengths"), "Got: {}", result);
}

#[test]
fn test_s12_b41_dict_comp_type() {
    let code = r#"
def count_chars(s: str) -> dict:
    freq = {}
    for c in s:
        freq[c] = freq.get(c, 0) + 1
    return freq
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_chars"), "Got: {}", result);
}

// ===== Bool operations =====

#[test]
fn test_s12_b41_not_operator() {
    let code = r#"
def negate(flag: bool) -> bool:
    return not flag
"#;
    let result = transpile(code);
    assert!(result.contains("fn negate"), "Got: {}", result);
}

#[test]
fn test_s12_b41_bool_from_comparison() {
    let code = r#"
def is_between(x: int, lo: int, hi: int) -> bool:
    return x >= lo and x <= hi
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_between"), "Got: {}", result);
}

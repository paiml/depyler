//! Session 12 Batch 70: Tuple unpacking and assignment cold paths
//!
//! Targets direct_rules_convert.rs cold paths for tuple unpacking,
//! starred expressions, and complex assignment patterns.

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
fn test_s12_b70_simple_tuple_unpack() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Got: {}", result);
}

#[test]
fn test_s12_b70_triple_unpack() {
    let code = r#"
def split_triple(t: tuple) -> int:
    x, y, z = t
    return x + y + z
"#;
    let result = transpile(code);
    assert!(result.contains("fn split_triple"), "Got: {}", result);
}

#[test]
fn test_s12_b70_unpack_from_function() {
    let code = r#"
def get_pair() -> tuple:
    return (10, 20)

def use_pair() -> int:
    a, b = get_pair()
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn use_pair"), "Got: {}", result);
}

#[test]
fn test_s12_b70_unpack_in_loop() {
    let code = r#"
def sum_pairs(pairs: list) -> int:
    total = 0
    for a, b in pairs:
        total += a + b
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_pairs"), "Got: {}", result);
}

#[test]
fn test_s12_b70_dict_items_unpack() {
    let code = r#"
def dict_to_list(d: dict) -> list:
    result = []
    for key, value in d.items():
        result.append(key)
        result.append(value)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn dict_to_list"), "Got: {}", result);
}

#[test]
fn test_s12_b70_enumerate_unpack() {
    let code = r#"
def indexed_items(items: list) -> list:
    result = []
    for idx, item in enumerate(items):
        result.append((idx, item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn indexed_items"), "Got: {}", result);
}

#[test]
fn test_s12_b70_zip_unpack() {
    let code = r#"
def zip_sum(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn zip_sum"), "Got: {}", result);
}

#[test]
fn test_s12_b70_nested_index_assign() {
    let code = r#"
def update_matrix(m: list, i: int, j: int, val: int):
    m[i][j] = val
"#;
    let result = transpile(code);
    assert!(result.contains("fn update_matrix"), "Got: {}", result);
}

#[test]
fn test_s12_b70_dict_of_dict_assign() {
    let code = r#"
def set_nested(data: dict, outer: str, inner: str, val: int):
    if outer not in data:
        data[outer] = {}
    data[outer][inner] = val
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_nested"), "Got: {}", result);
}

#[test]
fn test_s12_b70_augmented_index_assign() {
    let code = r#"
def increment_at(items: list, idx: int):
    items[idx] += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn increment_at"), "Got: {}", result);
}

#[test]
fn test_s12_b70_augmented_dict_assign() {
    let code = r#"
def add_to_key(d: dict, key: str, amount: int):
    d[key] += amount
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_to_key"), "Got: {}", result);
}

#[test]
fn test_s12_b70_multiple_assign() {
    let code = r#"
def init_vars() -> int:
    x = y = z = 0
    x += 1
    y += 2
    z += 3
    return x + y + z
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_vars"), "Got: {}", result);
}

#[test]
fn test_s12_b70_chained_attribute_assign() {
    let code = r#"
class Config:
    def __init__(self):
        self.debug = False
        self.verbose = False
        self.timeout = 30

    def enable_debug(self):
        self.debug = True
        self.verbose = True
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s12_b70_list_slice_assign() {
    let code = r#"
def replace_middle(items: list, start: int, end: int, new: list) -> list:
    result = list(items)
    result[start:end] = new
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn replace_middle"), "Got: {}", result);
}

#[test]
fn test_s12_b70_conditional_assign() {
    let code = r#"
def safe_divide(a: float, b: float) -> float:
    result = a / b if b != 0.0 else 0.0
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_divide"), "Got: {}", result);
}

#[test]
fn test_s12_b70_walrus_in_while() {
    let code = r#"
def read_chunks(data: list) -> list:
    result = []
    i = 0
    while (chunk := data[i:i+3]) and i < len(data):
        result.append(chunk)
        i += 3
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn read_chunks"), "Got: {}", result);
}

//! Session 12 Batch 48: Function generation cold paths
//!
//! Targets cold paths in func_gen.rs:
//! - Parameter type inference from body usage
//! - Return type inference from multiple paths
//! - Class method return type propagation
//! - Unused parameter handling
//! - Complex parameter combinations
//! - String method return type detection
//! - Mixed return type inference with Optional

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

// ===== Parameter type inference from usage =====

#[test]
fn test_s12_b48_param_infer_string_methods() {
    let code = r#"
def process_text(content):
    if content.endswith("\n"):
        return content.strip()
    return content.upper()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_text"), "Got: {}", result);
}

#[test]
fn test_s12_b48_param_infer_list_methods() {
    let code = r#"
def add_item(items, value):
    items.append(value)
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_item"), "Got: {}", result);
}

#[test]
fn test_s12_b48_param_infer_dict_methods() {
    let code = r#"
def get_keys(data):
    result = []
    for key in data.keys():
        result.append(key)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b48_param_infer_numeric() {
    let code = r#"
def compute(x, y):
    return x * y + x - y
"#;
    let result = transpile(code);
    assert!(result.contains("fn compute"), "Got: {}", result);
}

#[test]
fn test_s12_b48_param_infer_bool() {
    let code = r#"
def gate(flag, value: int) -> int:
    if flag:
        return value
    return 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn gate"), "Got: {}", result);
}

// ===== Return type inference =====

#[test]
fn test_s12_b48_return_infer_from_literal() {
    let code = r#"
def get_greeting():
    return "hello"
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_greeting"), "Got: {}", result);
}

#[test]
fn test_s12_b48_return_infer_from_arithmetic() {
    let code = r#"
def square(n: int):
    return n * n
"#;
    let result = transpile(code);
    assert!(result.contains("fn square"), "Got: {}", result);
}

#[test]
fn test_s12_b48_return_infer_from_bool() {
    let code = r#"
def is_even(n: int):
    return n % 2 == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_even"), "Got: {}", result);
}

#[test]
fn test_s12_b48_return_infer_from_list() {
    let code = r#"
def make_list(a: int, b: int, c: int):
    return [a, b, c]
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_list"), "Got: {}", result);
}

#[test]
fn test_s12_b48_return_infer_from_dict() {
    let code = r##"
def make_record(name: str, age: int):
    return {"name": name, "age": age}
"##;
    let result = transpile(code);
    assert!(result.contains("fn make_record"), "Got: {}", result);
}

// ===== Multiple return paths =====

#[test]
fn test_s12_b48_mixed_return_types() {
    let code = r#"
def find_or_default(items: list, target: int) -> int:
    for item in items:
        if item == target:
            return item
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_or_default"), "Got: {}", result);
}

#[test]
fn test_s12_b48_optional_return() {
    let code = r#"
def first_positive(items: list):
    for item in items:
        if item > 0:
            return item
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn first_positive"), "Got: {}", result);
}

#[test]
fn test_s12_b48_conditional_return_types() {
    let code = r#"
def safe_index(items: list, idx: int):
    if idx < 0 or idx >= len(items):
        return None
    return items[idx]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_index"), "Got: {}", result);
}

// ===== Class method patterns =====

#[test]
fn test_s12_b48_class_method_return_type() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self) -> int:
        self.count += 1
        return self.count

    def get(self) -> int:
        return self.count

    def reset(self):
        self.count = 0
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

#[test]
fn test_s12_b48_class_with_static_method() {
    let code = r#"
class MathUtils:
    @staticmethod
    def add(a: int, b: int) -> int:
        return a + b

    @staticmethod
    def multiply(a: int, b: int) -> int:
        return a * b
"#;
    let result = transpile(code);
    assert!(result.contains("MathUtils"), "Got: {}", result);
}

#[test]
fn test_s12_b48_class_with_properties() {
    let code = r#"
class Circle:
    def __init__(self, radius: float):
        self.radius = radius

    @property
    def diameter(self) -> float:
        return 2.0 * self.radius

    @property
    def circumference(self) -> float:
        return 2.0 * self.radius * 3.14159
"#;
    let result = transpile(code);
    assert!(result.contains("Circle"), "Got: {}", result);
}

// ===== Complex parameter patterns =====

#[test]
fn test_s12_b48_many_params() {
    let code = r#"
def create_rect(x: int, y: int, width: int, height: int, color: str, filled: bool) -> dict:
    return {"x": x, "y": y}
"#;
    let result = transpile(code);
    assert!(result.contains("fn create_rect"), "Got: {}", result);
}

#[test]
fn test_s12_b48_default_params_mixed() {
    let code = r#"
def format_number(n: float, precision: int = 2, prefix: str = "", suffix: str = "") -> str:
    return prefix + str(round(n, precision)) + suffix
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_number"), "Got: {}", result);
}

#[test]
fn test_s12_b48_star_args() {
    let code = r#"
def sum_all(*args) -> int:
    total = 0
    for arg in args:
        total += arg
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_all"), "Got: {}", result);
}

// ===== String method return type detection =====

#[test]
fn test_s12_b48_string_method_owned() {
    let code = r#"
def process(text: str) -> str:
    return text.replace("old", "new")
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b48_string_method_chain() {
    let code = r#"
def clean(text: str) -> str:
    return text.strip().lower().replace(" ", "_")
"#;
    let result = transpile(code);
    assert!(result.contains("fn clean"), "Got: {}", result);
}

#[test]
fn test_s12_b48_string_center_pad() {
    let code = r#"
def pad_center(text: str, width: int) -> str:
    return text.center(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn pad_center"), "Got: {}", result);
}

#[test]
fn test_s12_b48_string_zfill() {
    let code = r#"
def zero_pad(n: int, width: int) -> str:
    return str(n).zfill(width)
"#;
    let result = transpile(code);
    assert!(result.contains("fn zero_pad"), "Got: {}", result);
}

// ===== Nested function patterns =====

#[test]
fn test_s12_b48_nested_helper() {
    let code = r#"
def outer(items: list) -> list:
    def double(x: int) -> int:
        return x * 2
    result = []
    for item in items:
        result.append(double(item))
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn outer"), "Got: {}", result);
}

#[test]
fn test_s12_b48_nested_with_closure() {
    let code = r#"
def make_adder(n: int):
    def add(x: int) -> int:
        return x + n
    return add
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_adder"), "Got: {}", result);
}

// ===== Generator / yield patterns =====

#[test]
fn test_s12_b48_simple_generator() {
    let code = r#"
def count_up(n: int):
    i = 0
    while i < n:
        yield i
        i += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_up"), "Got: {}", result);
}

#[test]
fn test_s12_b48_filtered_generator() {
    let code = r#"
def even_numbers(n: int):
    for i in range(n):
        if i % 2 == 0:
            yield i
"#;
    let result = transpile(code);
    assert!(result.contains("fn even_numbers"), "Got: {}", result);
}

// ===== Async function patterns =====

#[test]
fn test_s12_b48_async_with_return() {
    let code = r#"
async def fetch_data(url: str) -> str:
    response = await get(url)
    return response
"#;
    let result = transpile(code);
    assert!(result.contains("fn fetch_data"), "Got: {}", result);
}

#[test]
fn test_s12_b48_async_with_loop() {
    let code = r#"
async def fetch_all(urls: list) -> list:
    results = []
    for url in urls:
        data = await get(url)
        results.append(data)
    return results
"#;
    let result = transpile(code);
    assert!(result.contains("fn fetch_all"), "Got: {}", result);
}

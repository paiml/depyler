//! Session 12 Batch 44: Decorator, async, and special pattern cold paths
//!
//! Tests patterns that are harder to exercise through typical code:
//! - Multiple decorators
//! - Async patterns (async def, await, async for, async with)
//! - Property decorators
//! - Classmethod/staticmethod patterns
//! - Complex import patterns
//! - Global statements with complex usage
//! - Complex delete operations
//! - Multiple inheritance
//! - Abstract-like patterns

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

// ===== Multiple decorators =====

#[test]
fn test_s12_b44_double_decorator() {
    let code = r#"
class API:
    @staticmethod
    def version() -> str:
        return "1.0"

    @classmethod
    def create(cls):
        return cls()
"#;
    let result = transpile(code);
    assert!(result.contains("API"), "Got: {}", result);
}

// ===== Async patterns =====

#[test]
fn test_s12_b44_async_simple() {
    let code = r#"
async def fetch(url: str) -> str:
    data = await get_data(url)
    return data
"#;
    let result = transpile(code);
    assert!(result.contains("fn fetch"), "Got: {}", result);
}

#[test]
fn test_s12_b44_async_with_try() {
    let code = r#"
async def safe_fetch(url: str) -> str:
    try:
        result = await get_data(url)
        return result
    except Exception:
        return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_fetch"), "Got: {}", result);
}

#[test]
fn test_s12_b44_async_loop() {
    let code = r#"
async def process_all(urls: list) -> list:
    results = []
    for url in urls:
        data = await fetch(url)
        results.append(data)
    return results
"#;
    let result = transpile(code);
    assert!(result.contains("fn process_all"), "Got: {}", result);
}

// ===== Property patterns =====

#[test]
fn test_s12_b44_property_getter() {
    let code = r#"
class Square:
    def __init__(self, side: float):
        self.side = side

    @property
    def area(self) -> float:
        return self.side * self.side

    @property
    def perimeter(self) -> float:
        return 4.0 * self.side
"#;
    let result = transpile(code);
    assert!(result.contains("Square"), "Got: {}", result);
}

// ===== Global with complex usage =====

#[test]
fn test_s12_b44_global_counter() {
    let code = r#"
_counter = 0

def next_id() -> int:
    global _counter
    _counter += 1
    return _counter

def reset():
    global _counter
    _counter = 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn next_id"), "Got: {}", result);
}

// ===== Multiple inheritance =====

#[test]
fn test_s12_b44_multi_inherit() {
    let code = r#"
class Printable:
    def to_string(self) -> str:
        return ""

class Comparable:
    def compare(self, other) -> int:
        return 0

class Item(Printable, Comparable):
    def __init__(self, name: str, value: int):
        self.name = name
        self.value = value

    def to_string(self) -> str:
        return f"{self.name}: {self.value}"

    def compare(self, other) -> int:
        return self.value - other.value
"#;
    let result = transpile(code);
    assert!(result.contains("Item"), "Got: {}", result);
}

// ===== Complex delete operations =====

#[test]
fn test_s12_b44_del_list_item() {
    let code = r#"
def remove_at(items: list, idx: int) -> list:
    del items[idx]
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_at"), "Got: {}", result);
}

#[test]
fn test_s12_b44_del_dict_key() {
    let code = r#"
def remove_key(d: dict, key: str) -> dict:
    if key in d:
        del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_key"), "Got: {}", result);
}

// ===== Complex string formatting =====

#[test]
fn test_s12_b44_fstring_complex_expr() {
    let code = r#"
def summary(items: list) -> str:
    n = len(items)
    total = sum(items)
    return f"Count: {n}, Sum: {total}, Avg: {total / n if n > 0 else 0}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn summary"), "Got: {}", result);
}

#[test]
fn test_s12_b44_fstring_multiline() {
    let code = r#"
def make_report(name: str, score: int, rank: int) -> str:
    return f"{name} scored {score} (rank {rank})"
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_report"), "Got: {}", result);
}

// ===== Complex iteration patterns =====

#[test]
fn test_s12_b44_enumerate_with_start() {
    let code = r#"
def numbered(items: list, start: int) -> list:
    result = []
    for i, item in enumerate(items, start):
        result.append(f"{i}. {item}")
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn numbered"), "Got: {}", result);
}

#[test]
fn test_s12_b44_zip_three() {
    let code = r#"
def combine_three(a: list, b: list, c: list) -> list:
    result = []
    for x, y, z in zip(a, b, c):
        result.append(x + y + z)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine_three"), "Got: {}", result);
}

// ===== Complex list operations =====

#[test]
fn test_s12_b44_list_slice_assign() {
    let code = r#"
def replace_middle(items: list, start: int, end: int, new_items: list) -> list:
    result = items[:start] + new_items + items[end:]
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn replace_middle"), "Got: {}", result);
}

#[test]
fn test_s12_b44_list_concat() {
    let code = r#"
def merge_sorted(a: list, b: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    while i < len(a):
        result.append(a[i])
        i += 1
    while j < len(b):
        result.append(b[j])
        j += 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sorted"), "Got: {}", result);
}

// ===== Nested class pattern =====

#[test]
fn test_s12_b44_nested_class() {
    let code = r#"
class Outer:
    def __init__(self):
        self.value = 0

    class Inner:
        def __init__(self, x: int):
            self.x = x

        def get(self) -> int:
            return self.x

    def create_inner(self, x: int):
        return Outer.Inner(x)
"#;
    let result = transpile(code);
    assert!(result.contains("Outer"), "Got: {}", result);
}

// ===== Complex numeric patterns =====

#[test]
fn test_s12_b44_complex_math() {
    let code = r#"
def distance(x1: float, y1: float, x2: float, y2: float) -> float:
    dx = x2 - x1
    dy = y2 - y1
    return (dx ** 2 + dy ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Got: {}", result);
}

#[test]
fn test_s12_b44_compound_interest() {
    let code = r#"
def compound_interest(principal: float, rate: float, years: int) -> float:
    amount = principal
    for i in range(years):
        amount *= (1.0 + rate)
    return amount - principal
"#;
    let result = transpile(code);
    assert!(result.contains("fn compound_interest"), "Got: {}", result);
}

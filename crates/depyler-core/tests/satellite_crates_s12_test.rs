//! Session 12 Batch 29: Additional transpile patterns for coverage
//!
//! Targets more cold paths in codegen:
//! - Complex class hierarchies
//! - Multiple return paths
//! - Nested data structures
//! - Error propagation patterns
//! - Global variable patterns
//! - Multiple decorators
//! - Complex loop patterns

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

// ===== Complex class patterns =====

#[test]
fn test_s12_class_multiple_init_defaults() {
    let code = r#"
class Config:
    def __init__(self):
        self.host = "localhost"
        self.port = 8080
        self.debug = False
        self.workers = 4
        self.timeout = 30.0
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

#[test]
fn test_s12_class_with_class_variable() {
    let code = r#"
class Counter:
    count = 0

    def __init__(self):
        self.value = 0

    def increment(self):
        self.value += 1
"#;
    let result = transpile(code);
    assert!(result.contains("Counter"), "Got: {}", result);
}

#[test]
fn test_s12_class_repr_method() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_class_eq_method() {
    let code = r#"
class Color:
    def __init__(self, r: int, g: int, b: int):
        self.r = r
        self.g = g
        self.b = b

    def __eq__(self, other) -> bool:
        return self.r == other.r and self.g == other.g and self.b == other.b
"#;
    let result = transpile(code);
    assert!(result.contains("Color"), "Got: {}", result);
}

// ===== Multiple return paths =====

#[test]
fn test_s12_multiple_returns() {
    let code = r#"
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
"#;
    let result = transpile(code);
    assert!(result.contains("fn classify"), "Got: {}", result);
}

#[test]
fn test_s12_early_return_guard() {
    let code = r#"
def process(items: list) -> int:
    if not items:
        return 0
    if len(items) == 1:
        return items[0]
    total = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

// ===== Nested data structures =====

#[test]
fn test_s12_list_of_dicts() {
    let code = r#"
def find_by_name(records: list, name: str) -> dict:
    for record in records:
        if record["name"] == name:
            return record
    return {}
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_by_name"), "Got: {}", result);
}

#[test]
fn test_s12_dict_of_lists() {
    let code = r#"
def add_to_group(groups: dict, key: str, value: int):
    if key not in groups:
        groups[key] = []
    groups[key].append(value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn add_to_group"), "Got: {}", result);
}

// ===== Complex loop patterns =====

#[test]
fn test_s12_nested_for_loops() {
    let code = r#"
def flatten(matrix: list) -> list:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"), "Got: {}", result);
}

#[test]
fn test_s12_while_with_break() {
    let code = r#"
def find_first(items: list, target: int) -> int:
    i = 0
    while i < len(items):
        if items[i] == target:
            break
        i += 1
    return i
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_first"), "Got: {}", result);
}

#[test]
fn test_s12_for_with_continue() {
    let code = r#"
def sum_positive(items: list) -> int:
    total = 0
    for x in items:
        if x < 0:
            continue
        total += x
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_positive"), "Got: {}", result);
}

#[test]
fn test_s12_for_else() {
    let code = r#"
def find_or_default(items: list, target: int) -> int:
    for x in items:
        if x == target:
            return x
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_or_default"), "Got: {}", result);
}

// ===== Multiple assignments =====

#[test]
fn test_s12_multi_assign_same_value() {
    let code = r#"
def init_counters() -> tuple:
    a = b = c = 0
    return (a, b, c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn init_counters"), "Got: {}", result);
}

// ===== Math operations =====

#[test]
fn test_s12_modulo_operator() {
    let code = r#"
def is_even(n: int) -> bool:
    return n % 2 == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_even"), "Got: {}", result);
}

#[test]
fn test_s12_abs_builtin() {
    let code = r#"
def distance(a: int, b: int) -> int:
    return abs(a - b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance"), "Got: {}", result);
}

#[test]
fn test_s12_min_max_builtins() {
    let code = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return max(lo, min(x, hi))
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp"), "Got: {}", result);
}

#[test]
fn test_s12_divmod_builtin() {
    let code = r#"
def hours_minutes(total_minutes: int) -> tuple:
    return divmod(total_minutes, 60)
"#;
    let result = transpile(code);
    assert!(result.contains("fn hours_minutes"), "Got: {}", result);
}

#[test]
fn test_s12_round_builtin() {
    let code = r#"
def round_to_cents(amount: float) -> float:
    return round(amount, 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn round_to_cents"), "Got: {}", result);
}

// ===== Type conversion builtins =====

#[test]
fn test_s12_int_from_string() {
    let code = r#"
def parse_int(s: str) -> int:
    return int(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_int"), "Got: {}", result);
}

#[test]
fn test_s12_float_from_string() {
    let code = r#"
def parse_float(s: str) -> float:
    return float(s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_float"), "Got: {}", result);
}

#[test]
fn test_s12_str_from_int() {
    let code = r#"
def int_to_str(n: int) -> str:
    return str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn int_to_str"), "Got: {}", result);
}

#[test]
fn test_s12_bool_builtin() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Got: {}", result);
}

#[test]
fn test_s12_chr_builtin() {
    let code = r#"
def code_to_char(n: int) -> str:
    return chr(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn code_to_char"), "Got: {}", result);
}

#[test]
fn test_s12_ord_builtin() {
    let code = r#"
def char_to_code(c: str) -> int:
    return ord(c)
"#;
    let result = transpile(code);
    assert!(result.contains("fn char_to_code"), "Got: {}", result);
}

#[test]
fn test_s12_hex_builtin() {
    let code = r#"
def to_hex(n: int) -> str:
    return hex(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_hex"), "Got: {}", result);
}

#[test]
fn test_s12_oct_builtin() {
    let code = r#"
def to_oct(n: int) -> str:
    return oct(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_oct"), "Got: {}", result);
}

#[test]
fn test_s12_bin_builtin() {
    let code = r#"
def to_bin(n: int) -> str:
    return bin(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bin"), "Got: {}", result);
}

// ===== Print patterns =====

#[test]
fn test_s12_print_basic() {
    let code = r#"
def greet(name: str):
    print("Hello, " + name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_print_multiple_args() {
    let code = r#"
def show_pair(key: str, value: int):
    print(key, value)
"#;
    let result = transpile(code);
    assert!(result.contains("fn show_pair"), "Got: {}", result);
}

// ===== Complex real-world patterns =====

#[test]
fn test_s12_stack_class() {
    let code = r##"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        if not self.items:
            raise IndexError("stack empty")
        return self.items.pop()

    def peek(self) -> int:
        if not self.items:
            raise IndexError("stack empty")
        return self.items[-1]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"##;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
    assert!(result.contains("fn push"), "Got: {}", result);
    assert!(result.contains("fn pop"), "Got: {}", result);
}

#[test]
fn test_s12_lru_cache_class() {
    let code = r##"
class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = {}
        self.order = []

    def get(self, key: str) -> int:
        if key in self.cache:
            return self.cache[key]
        return -1

    def put(self, key: str, value: int):
        if key in self.cache:
            self.cache[key] = value
        else:
            if len(self.cache) >= self.capacity:
                oldest = self.order.pop(0)
                del self.cache[oldest]
            self.cache[key] = value
            self.order.append(key)
"##;
    let result = transpile(code);
    assert!(result.contains("LRUCache"), "Got: {}", result);
}

#[test]
fn test_s12_event_emitter_class() {
    let code = r##"
class EventEmitter:
    def __init__(self):
        self.listeners = {}

    def on(self, event: str, callback):
        if event not in self.listeners:
            self.listeners[event] = []
        self.listeners[event].append(callback)

    def emit(self, event: str):
        if event in self.listeners:
            for callback in self.listeners[event]:
                callback()
"##;
    let result = transpile(code);
    assert!(result.contains("EventEmitter"), "Got: {}", result);
}

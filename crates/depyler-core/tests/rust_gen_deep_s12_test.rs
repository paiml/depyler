//! Session 12 Batch 83: Rust gen module deep cold paths
//!
//! Targets rust_gen/mod.rs cold paths for module-level codegen:
//! constant generation, type inference for constants, homogeneous
//! list type detection, and module structure patterns.

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

// ===== Module-level constant patterns =====

#[test]
fn test_s12_b83_module_int_constant() {
    let code = r#"
MAX_SIZE = 100

def check_size(n: int) -> bool:
    return n <= MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"), "Got: {}", result);
}

#[test]
fn test_s12_b83_module_float_constant() {
    let code = r#"
EPSILON = 0.001

def is_close(a: float, b: float) -> bool:
    return abs(a - b) < EPSILON
"#;
    let result = transpile(code);
    assert!(result.contains("EPSILON"), "Got: {}", result);
}

#[test]
fn test_s12_b83_module_string_constant() {
    let code = r##"
DEFAULT_NAME = "Unknown"

def get_name(name: str) -> str:
    if not name:
        return DEFAULT_NAME
    return name
"##;
    let result = transpile(code);
    assert!(result.contains("DEFAULT_NAME"), "Got: {}", result);
}

#[test]
fn test_s12_b83_module_bool_constant() {
    let code = r#"
DEBUG = False

def log(msg: str):
    if DEBUG:
        print(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("DEBUG"), "Got: {}", result);
}

#[test]
fn test_s12_b83_module_list_constant() {
    let code = r#"
VOWELS = ["a", "e", "i", "o", "u"]

def is_vowel(c: str) -> bool:
    return c.lower() in VOWELS
"#;
    let result = transpile(code);
    assert!(result.contains("VOWELS"), "Got: {}", result);
}

#[test]
fn test_s12_b83_module_dict_constant() {
    let code = r##"
ERROR_CODES = {"not_found": 404, "ok": 200, "error": 500}

def get_status_code(name: str) -> int:
    return ERROR_CODES.get(name, 0)
"##;
    let result = transpile(code);
    assert!(result.contains("ERROR_CODES"), "Got: {}", result);
}

#[test]
fn test_s12_b83_multiple_constants() {
    let code = r#"
MIN_VALUE = 0
MAX_VALUE = 100
STEP = 5

def generate_range() -> list:
    return list(range(MIN_VALUE, MAX_VALUE, STEP))
"#;
    let result = transpile(code);
    assert!(result.contains("MIN_VALUE"), "Got: {}", result);
    assert!(result.contains("MAX_VALUE"), "Got: {}", result);
}

// ===== Module structure patterns =====

#[test]
fn test_s12_b83_constants_with_functions() {
    let code = r##"
VERSION = "1.0.0"
AUTHOR = "Test"

def get_version() -> str:
    return VERSION

def get_info() -> str:
    return AUTHOR + " v" + VERSION
"##;
    let result = transpile(code);
    assert!(result.contains("VERSION"), "Got: {}", result);
    assert!(result.contains("fn get_version"), "Got: {}", result);
}

#[test]
fn test_s12_b83_class_with_constants() {
    let code = r#"
DEFAULT_CAPACITY = 10

class Buffer:
    def __init__(self):
        self.data = []
        self.max_size = DEFAULT_CAPACITY

    def add(self, item: int) -> bool:
        if len(self.data) >= self.max_size:
            return False
        self.data.append(item)
        return True
"#;
    let result = transpile(code);
    assert!(result.contains("DEFAULT_CAPACITY"), "Got: {}", result);
    assert!(result.contains("Buffer"), "Got: {}", result);
}

#[test]
fn test_s12_b83_computed_constant() {
    let code = r#"
BASE = 2
POWERS = [BASE ** i for i in range(10)]

def is_power_of_two(n: int) -> bool:
    return n in POWERS
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_power_of_two"), "Got: {}", result);
}

// ===== Complex module patterns =====

#[test]
fn test_s12_b83_enum_like_constants() {
    let code = r#"
RED = 0
GREEN = 1
BLUE = 2

def color_name(code: int) -> str:
    if code == RED:
        return "red"
    if code == GREEN:
        return "green"
    if code == BLUE:
        return "blue"
    return "unknown"
"#;
    let result = transpile(code);
    assert!(result.contains("RED"), "Got: {}", result);
    assert!(result.contains("fn color_name"), "Got: {}", result);
}

#[test]
fn test_s12_b83_tuple_constant() {
    let code = r#"
ORIGIN = (0, 0)

def distance_from_origin(x: int, y: int) -> float:
    return ((x - ORIGIN[0]) ** 2 + (y - ORIGIN[1]) ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("fn distance_from_origin"), "Got: {}", result);
}

#[test]
fn test_s12_b83_set_constant() {
    let code = r#"
RESERVED = {"if", "else", "while", "for", "return"}

def is_reserved(word: str) -> bool:
    return word in RESERVED
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_reserved"), "Got: {}", result);
}

#[test]
fn test_s12_b83_negative_constant() {
    let code = r#"
NOT_FOUND = -1

def search(items: list, target: int) -> int:
    for i, item in enumerate(items):
        if item == target:
            return i
    return NOT_FOUND
"#;
    let result = transpile(code);
    assert!(result.contains("NOT_FOUND"), "Got: {}", result);
}

#[test]
fn test_s12_b83_multi_class_module() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

class Line:
    def __init__(self, start: Point, end: Point):
        self.start = start
        self.end = end

    def length(self) -> float:
        dx = self.end.x - self.start.x
        dy = self.end.y - self.start.y
        return (dx * dx + dy * dy) ** 0.5

def make_line(x1: float, y1: float, x2: float, y2: float) -> Line:
    return Line(Point(x1, y1), Point(x2, y2))
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
    assert!(result.contains("Line"), "Got: {}", result);
}

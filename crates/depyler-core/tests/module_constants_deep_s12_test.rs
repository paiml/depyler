//! Session 12 Batch 46: Module-level constants and rust_gen cold paths
//!
//! Targets cold paths in rust_gen/mod.rs:
//! - Module-level constant inference (lazy constants)
//! - Empty list/dict/tuple constants
//! - Mixed-type list constants
//! - Binary expression type inference for constants
//! - Comprehension element type inference
//! - TypeVar declarations (skipped in codegen)
//! - Module-level function assignments

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

// ===== Module-level constant type inference =====

#[test]
fn test_s12_b46_module_int_constant() {
    let code = r#"
MAX_SIZE = 100

def get_max() -> int:
    return MAX_SIZE
"#;
    let result = transpile(code);
    assert!(result.contains("MAX_SIZE"), "Got: {}", result);
    assert!(result.contains("fn get_max"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_float_constant() {
    let code = r#"
EPSILON = 0.001

def is_close(a: float, b: float) -> bool:
    return abs(a - b) < EPSILON
"#;
    let result = transpile(code);
    assert!(result.contains("EPSILON"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_string_constant() {
    let code = r##"
VERSION = "1.0.0"
PREFIX = "depyler"

def version_string() -> str:
    return f"{PREFIX}-{VERSION}"
"##;
    let result = transpile(code);
    assert!(result.contains("VERSION"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_bool_constant() {
    let code = r#"
DEBUG = False
VERBOSE = True

def should_log() -> bool:
    return DEBUG or VERBOSE
"#;
    let result = transpile(code);
    assert!(result.contains("DEBUG"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_list_constant() {
    let code = r#"
PRIMES = [2, 3, 5, 7, 11, 13]

def is_small_prime(n: int) -> bool:
    return n in PRIMES
"#;
    let result = transpile(code);
    assert!(result.contains("PRIMES"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_empty_list_constant() {
    let code = r#"
CACHE = []

def add_to_cache(item: int):
    CACHE.append(item)
"#;
    let result = transpile(code);
    assert!(result.contains("CACHE"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_dict_constant() {
    let code = r##"
COLORS = {"red": 1, "green": 2, "blue": 3}

def color_code(name: str) -> int:
    return COLORS.get(name, 0)
"##;
    let result = transpile(code);
    assert!(result.contains("COLORS"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_empty_dict_constant() {
    let code = r#"
REGISTRY = {}

def register(name: str, value: int):
    REGISTRY[name] = value
"#;
    let result = transpile(code);
    assert!(result.contains("REGISTRY"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_tuple_constant() {
    let code = r#"
ORIGIN = (0, 0)

def distance_from_origin(x: int, y: int) -> float:
    dx = x - ORIGIN[0]
    dy = y - ORIGIN[1]
    return (dx * dx + dy * dy) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("ORIGIN"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_set_constant() {
    let code = r#"
VOWELS = {"a", "e", "i", "o", "u"}

def count_vowels(s: str) -> int:
    count = 0
    for c in s:
        if c.lower() in VOWELS:
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("VOWELS"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_nested_binary_expr() {
    let code = r#"
WIDTH = 80
HEIGHT = 24
AREA = WIDTH * HEIGHT

def fits(w: int, h: int) -> bool:
    return w * h <= AREA
"#;
    let result = transpile(code);
    assert!(result.contains("AREA"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_string_list_constant() {
    let code = r#"
KEYWORDS = ["if", "else", "for", "while", "return"]

def is_keyword(word: str) -> bool:
    return word in KEYWORDS
"#;
    let result = transpile(code);
    assert!(result.contains("KEYWORDS"), "Got: {}", result);
}

#[test]
fn test_s12_b46_module_float_list_constant() {
    let code = r#"
WEIGHTS = [0.1, 0.2, 0.3, 0.4]

def weighted_sum(values: list) -> float:
    total = 0.0
    for i in range(len(values)):
        total += values[i] * WEIGHTS[i]
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("WEIGHTS"), "Got: {}", result);
}

#[test]
fn test_s12_b46_multiple_constants() {
    let code = r##"
NAME = "app"
VERSION = "2.0"
MAX_RETRIES = 3
TIMEOUT = 30.0
ENABLED = True

def config_string() -> str:
    return f"{NAME} v{VERSION}"
"##;
    let result = transpile(code);
    assert!(result.contains("NAME"), "Got: {}", result);
    assert!(result.contains("MAX_RETRIES"), "Got: {}", result);
}

#[test]
fn test_s12_b46_constant_with_function_call() {
    let code = r#"
SEPARATOR = "-" * 40

def print_header(title: str) -> str:
    return SEPARATOR + "\n" + title + "\n" + SEPARATOR
"#;
    let result = transpile(code);
    assert!(result.contains("SEPARATOR"), "Got: {}", result);
}

#[test]
fn test_s12_b46_constant_binary_string_ops() {
    let code = r##"
BASE_URL = "https://api.example.com"
API_VERSION = "/v2"
ENDPOINT = BASE_URL + API_VERSION

def make_url(path: str) -> str:
    return f"{ENDPOINT}/{path}"
"##;
    let result = transpile(code);
    assert!(result.contains("ENDPOINT"), "Got: {}", result);
}

// ===== Module with functions and constants interleaved =====

#[test]
fn test_s12_b46_interleaved_funcs_constants() {
    let code = r#"
DEFAULT_SEP = ","

def join_items(items: list) -> str:
    return DEFAULT_SEP.join(items)

MAX_ITEMS = 100

def can_add(current: int) -> bool:
    return current < MAX_ITEMS
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_items"), "Got: {}", result);
    assert!(result.contains("fn can_add"), "Got: {}", result);
}

// ===== Complex constant expressions =====

#[test]
fn test_s12_b46_constant_arithmetic() {
    let code = r#"
BITS = 8
MAX_VAL = 2 ** BITS - 1

def clamp_byte(v: int) -> int:
    if v < 0:
        return 0
    if v > MAX_VAL:
        return MAX_VAL
    return v
"#;
    let result = transpile(code);
    assert!(result.contains("fn clamp_byte"), "Got: {}", result);
}

#[test]
fn test_s12_b46_negative_constant() {
    let code = r#"
MIN_TEMP = -273
MAX_TEMP = 1000

def is_valid_temp(t: int) -> bool:
    return MIN_TEMP <= t <= MAX_TEMP
"#;
    let result = transpile(code);
    assert!(result.contains("MIN_TEMP"), "Got: {}", result);
}

#[test]
fn test_s12_b46_dict_with_int_values() {
    let code = r##"
HTTP_CODES = {
    "ok": 200,
    "created": 201,
    "bad_request": 400,
    "not_found": 404,
    "server_error": 500
}

def get_code(status: str) -> int:
    return HTTP_CODES.get(status, 0)
"##;
    let result = transpile(code);
    assert!(result.contains("HTTP_CODES"), "Got: {}", result);
}

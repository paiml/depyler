//! DEPYLER-99MODE-S10: Integration tests targeting type_mapper.rs coverage gaps
//!
//! Tests for: bare list/dict/set types, generic type annotations,
//! custom type names, nested types, unknown type fallbacks, and
//! array/tuple type handling through the transpilation pipeline.

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

// ===== Bare Types (No Generics) =====

#[test]
fn test_s10_bare_list_type() {
    let code = r#"
def get_items() -> list:
    return [1, 2, 3]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_items"));
    assert!(result.contains("Vec") || result.contains("vec!"));
}

#[test]
fn test_s10_bare_dict_type() {
    let code = r#"
def get_data() -> dict:
    return {"key": "value"}
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_data"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_bare_set_type() {
    let code = r#"
def get_unique() -> set:
    return {1, 2, 3}
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_unique"));
    assert!(result.contains("HashSet"));
}

#[test]
fn test_s10_bare_tuple_type() {
    let code = r#"
def get_pair() -> tuple:
    return (1, 2)
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_pair"));
}

// ===== Generic Type Annotations =====

#[test]
fn test_s10_list_int_type() {
    let code = r#"
from typing import List

def sum_nums(nums: List[int]) -> int:
    return sum(nums)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_nums"));
    assert!(result.contains("Vec") || result.contains("i32") || result.contains("i64"));
}

#[test]
fn test_s10_list_str_type() {
    let code = r#"
from typing import List

def join_all(items: List[str]) -> str:
    return " ".join(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn join_all"));
}

#[test]
fn test_s10_dict_str_int_type() {
    let code = r#"
from typing import Dict

def lookup(data: Dict[str, int], key: str) -> int:
    return data.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn lookup"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_dict_int_str_type() {
    let code = r#"
from typing import Dict

def index_to_name(mapping: Dict[int, str], idx: int) -> str:
    return mapping.get(idx, "unknown")
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_to_name"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_set_int_type() {
    let code = r#"
from typing import Set

def unique_nums(items: list) -> Set[int]:
    return set(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_nums"));
    assert!(result.contains("HashSet"));
}

#[test]
fn test_s10_tuple_int_str_type() {
    let code = r#"
from typing import Tuple

def make_pair(x: int, s: str) -> Tuple[int, str]:
    return (x, s)
"#;
    let result = transpile(code);
    assert!(result.contains("fn make_pair"));
}

#[test]
fn test_s10_optional_int_type() {
    let code = r#"
from typing import Optional

def find_index(items: list, target: int) -> Optional[int]:
    for i, item in enumerate(items):
        if item == target:
            return i
    return None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_index"));
    assert!(result.contains("Option"));
}

#[test]
fn test_s10_optional_str_type() {
    let code = r#"
from typing import Optional

def get_name(data: dict) -> Optional[str]:
    return data.get("name")
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_name"));
    assert!(result.contains("Option"));
}

// ===== Nested Generic Types =====

#[test]
fn test_s10_list_of_list() {
    let code = r#"
from typing import List

def flatten(matrix: List[List[int]]) -> List[int]:
    result = []
    for row in matrix:
        for item in row:
            result.append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn flatten"));
    assert!(result.contains("Vec"));
}

#[test]
fn test_s10_dict_of_list() {
    let code = r#"
from typing import Dict, List

def group_by_key(items: list) -> Dict[str, List[int]]:
    result = {}
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn group_by_key"));
    assert!(result.contains("HashMap"));
}

#[test]
fn test_s10_optional_list() {
    let code = r#"
from typing import Optional, List

def safe_first(items: Optional[List[int]]) -> int:
    if items is None:
        return 0
    return items[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn safe_first"));
    assert!(result.contains("Option"));
}

// ===== Custom Type Names =====

#[test]
fn test_s10_class_type() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def distance(p1: Point, p2: Point) -> float:
    return ((p2.x - p1.x) ** 2 + (p2.y - p1.y) ** 2) ** 0.5
"#;
    let result = transpile(code);
    assert!(result.contains("Point"));
    assert!(result.contains("distance"));
}

// ===== Primitive Type Annotations =====

#[test]
fn test_s10_int_type() {
    let code = r#"
def identity_int(x: int) -> int:
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity_int"));
    assert!(result.contains("i32") || result.contains("i64"));
}

#[test]
fn test_s10_float_type() {
    let code = r#"
def identity_float(x: float) -> float:
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity_float"));
    assert!(result.contains("f64"));
}

#[test]
fn test_s10_bool_type() {
    let code = r#"
def identity_bool(x: bool) -> bool:
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity_bool"));
    assert!(result.contains("bool"));
}

#[test]
fn test_s10_str_type() {
    let code = r#"
def identity_str(x: str) -> str:
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn identity_str"));
    assert!(result.contains("str") || result.contains("String"));
}

// ===== Return Type Inference =====

#[test]
fn test_s10_infer_return_int() {
    let code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    let result = transpile(code);
    assert!(result.contains("fn add"));
    assert!(result.contains("i32") || result.contains("i64"));
}

#[test]
fn test_s10_infer_return_float() {
    let code = r#"
def divide(a: float, b: float) -> float:
    return a / b
"#;
    let result = transpile(code);
    assert!(result.contains("fn divide"));
    assert!(result.contains("f64"));
}

#[test]
fn test_s10_infer_return_bool() {
    let code = r#"
def is_even(n: int) -> bool:
    return n % 2 == 0
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_even"));
    assert!(result.contains("bool"));
}

// ===== Mixed Types in Function Signatures =====

#[test]
fn test_s10_mixed_param_types() {
    let code = r#"
from typing import List, Dict

def process(items: List[int], config: Dict[str, bool]) -> str:
    if config.get("verbose", False):
        return str(len(items))
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"));
    assert!(result.contains("Vec") || result.contains("HashMap"));
}

// ===== Complex Nested Patterns =====

#[test]
fn test_s10_list_comprehension_typed() {
    let code = r#"
from typing import List

def squares(n: int) -> List[int]:
    return [x * x for x in range(n)]
"#;
    let result = transpile(code);
    assert!(result.contains("fn squares"));
}

#[test]
fn test_s10_dict_comprehension_typed() {
    let code = r#"
from typing import Dict

def enumerate_dict(items: list) -> Dict[int, str]:
    return {i: str(v) for i, v in enumerate(items)}
"#;
    let result = transpile(code);
    assert!(result.contains("fn enumerate_dict"));
    assert!(result.contains("HashMap"));
}

// ===== bytes Type =====

#[test]
fn test_s10_bytes_type() {
    let code = r#"
def encode(s: str) -> bytes:
    return s.encode()
"#;
    let result = transpile(code);
    assert!(result.contains("fn encode"));
    assert!(result.contains("Vec") || result.contains("u8") || result.contains("bytes"));
}

// ===== None Return Type =====

#[test]
fn test_s10_none_return() {
    let code = r#"
def print_items(items: list):
    for item in items:
        print(item)
"#;
    let result = transpile(code);
    assert!(result.contains("fn print_items"));
    // Should have no explicit return type or return ()
}

// ===== Type Aliases =====

#[test]
fn test_s10_constant_type_inference() {
    let code = r#"
THRESHOLD = 42
RATE = 3.14

def check(x: int) -> bool:
    return x > THRESHOLD
"#;
    let result = transpile(code);
    assert!(result.contains("THRESHOLD"));
    assert!(result.contains("RATE") || result.contains("3.14"));
    assert!(result.contains("fn check"));
}

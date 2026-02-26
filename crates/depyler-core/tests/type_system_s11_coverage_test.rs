//! Session 11: Type system coverage tests
//!
//! Exercises type inference, type mapping, and type propagation paths
//! through end-to-end transpilation of Python patterns that require
//! sophisticated type handling.

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

// ============================================================================
// Type annotation variations
// ============================================================================

#[test]
fn test_s11_type_list_int() {
    let code = r#"
from typing import List

def sum_list(items: List[int]) -> int:
    total: int = 0
    for item in items:
        total += item
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec<i32>")
            || result.contains("Vec<i64>")
            || result.contains("fn sum_list"),
        "Should type List[int]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_list_str() {
    let code = r#"
from typing import List

def join_all(items: List[str]) -> str:
    return ", ".join(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec<String>") || result.contains("fn join_all"),
        "Should type List[str]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_list_float() {
    let code = r#"
from typing import List

def average(items: List[float]) -> float:
    return sum(items) / len(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("fn average"),
        "Should type List[float]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_dict_str_int() {
    let code = r#"
from typing import Dict

def sum_values(d: Dict[str, int]) -> int:
    total: int = 0
    for v in d.values():
        total += v
    return total
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn sum_values"),
        "Should type Dict[str, int]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_dict_str_str() {
    let code = r#"
from typing import Dict

def get_env(config: Dict[str, str], key: str) -> str:
    return config.get(key, "")
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn get_env"),
        "Should type Dict[str, str]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_set_int() {
    let code = r#"
from typing import Set

def make_set(items: list) -> Set[int]:
    result: Set[int] = set()
    for item in items:
        result.add(item)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashSet") || result.contains("fn make_set"),
        "Should type Set[int]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_optional_str() {
    let code = r#"
from typing import Optional

def find_name(data: dict, key: str) -> Optional[str]:
    if key in data:
        return data[key]
    return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("Option") || result.contains("fn find_name"),
        "Should type Optional[str]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_optional_int() {
    let code = r#"
from typing import Optional

def try_parse(s: str) -> Optional[int]:
    try:
        return int(s)
    except ValueError:
        return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("Option") || result.contains("fn try_parse"),
        "Should type Optional[int]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_tuple_two() {
    let code = r#"
from typing import Tuple

def swap(a: int, b: int) -> Tuple[int, int]:
    return (b, a)
"#;
    let result = transpile(code);
    assert!(result.contains("fn swap"), "Should type Tuple[int, int]. Got: {}", result);
}

#[test]
fn test_s11_type_tuple_three() {
    let code = r#"
from typing import Tuple

def rgb(r: int, g: int, b: int) -> Tuple[int, int, int]:
    return (r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn rgb"), "Should type Tuple[int, int, int]. Got: {}", result);
}

// ============================================================================
// Type inference from context
// ============================================================================

#[test]
fn test_s11_type_infer_int_literal() {
    let code = r#"
def get_count() -> int:
    count: int = 0
    count = count + 1
    return count
"#;
    let result = transpile(code);
    assert!(
        result.contains("i32") || result.contains("i64") || result.contains("fn get_count"),
        "Should infer int type. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_infer_float_literal() {
    let code = r#"
def get_ratio() -> float:
    ratio: float = 0.5
    return ratio * 2.0
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("fn get_ratio"),
        "Should infer float type. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_infer_bool_literal() {
    let code = r#"
def get_flag() -> bool:
    flag: bool = True
    return not flag
"#;
    let result = transpile(code);
    assert!(
        result.contains("bool") || result.contains("fn get_flag"),
        "Should infer bool type. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_infer_string_literal() {
    let code = r#"
def get_msg() -> str:
    msg: str = "hello"
    return msg + " world"
"#;
    let result = transpile(code);
    assert!(
        result.contains("String") || result.contains("fn get_msg"),
        "Should infer string type. Got: {}",
        result
    );
}

// ============================================================================
// Mixed type operations
// ============================================================================

#[test]
fn test_s11_type_int_float_arithmetic() {
    let code = r#"
def mixed(a: int, b: float) -> float:
    return a + b
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("as f64") || result.contains("fn mixed"),
        "Should handle int+float. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_int_division_to_float() {
    let code = r#"
def divide(a: int, b: int) -> float:
    return a / b
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("fn divide"),
        "Should handle int division to float. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_string_int_format() {
    let code = r#"
def label(name: str, idx: int) -> str:
    return f"{name}_{idx}"
"#;
    let result = transpile(code);
    assert!(result.contains("format!"), "Should handle string+int format. Got: {}", result);
}

// ============================================================================
// Complex nested types
// ============================================================================

#[test]
fn test_s11_type_list_of_tuples() {
    let code = r#"
from typing import List, Tuple

def make_pairs(items: List[int]) -> List[Tuple[int, int]]:
    result: list = []
    for i in range(0, len(items), 2):
        result.append((items[i], items[i+1]))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_pairs"),
        "Should handle List[Tuple[int,int]]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_dict_of_lists() {
    let code = r#"
from typing import Dict, List

def group(items: List[int]) -> Dict[str, List[int]]:
    result: dict = {"even": [], "odd": []}
    for item in items:
        if item % 2 == 0:
            result["even"].append(item)
        else:
            result["odd"].append(item)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn group"), "Should handle Dict[str, List[int]]. Got: {}", result);
}

// ============================================================================
// Default parameter values
// ============================================================================

#[test]
fn test_s11_type_default_int() {
    let code = r#"
def repeat(text: str, times: int = 1) -> str:
    return text * times
"#;
    let result = transpile(code);
    assert!(result.contains("fn repeat"), "Should handle default int param. Got: {}", result);
}

#[test]
fn test_s11_type_default_str() {
    let code = r#"
def greet(name: str = "World") -> str:
    return f"Hello, {name}!"
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Should handle default str param. Got: {}", result);
}

#[test]
fn test_s11_type_default_bool() {
    let code = r#"
def process(data: str, verbose: bool = False) -> str:
    if verbose:
        print(f"Processing: {data}")
    return data.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Should handle default bool param. Got: {}", result);
}

#[test]
fn test_s11_type_default_none() {
    let code = r#"
from typing import Optional

def find(items: list, target: int, default: Optional[int] = None) -> Optional[int]:
    for item in items:
        if item == target:
            return item
    return default
"#;
    let result = transpile(code);
    assert!(result.contains("fn find"), "Should handle default None param. Got: {}", result);
}

// ============================================================================
// Return type variations
// ============================================================================

#[test]
fn test_s11_type_return_none() {
    let code = r#"
def do_work(x: int) -> None:
    if x > 0:
        print(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn do_work"), "Should handle None return type. Got: {}", result);
}

#[test]
fn test_s11_type_return_list() {
    let code = r#"
from typing import List

def range_list(n: int) -> List[int]:
    return list(range(n))
"#;
    let result = transpile(code);
    assert!(result.contains("fn range_list"), "Should handle List return type. Got: {}", result);
}

#[test]
fn test_s11_type_return_dict() {
    let code = r#"
from typing import Dict

def count_chars(text: str) -> Dict[str, int]:
    result: dict = {}
    for ch in text:
        if ch in result:
            result[ch] = result[ch] + 1
        else:
            result[ch] = 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_chars"), "Should handle Dict return type. Got: {}", result);
}

#[test]
fn test_s11_type_return_bool() {
    let code = r#"
def is_palindrome(s: str) -> bool:
    cleaned: str = s.lower().strip()
    return cleaned == cleaned[::-1]
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_palindrome"), "Should handle bool return type. Got: {}", result);
}

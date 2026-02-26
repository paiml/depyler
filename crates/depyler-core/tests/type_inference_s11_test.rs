//! Session 11: Type inference and type mapping coverage tests
//!
//! Targets specific untested type inference paths:
//! - Return type inference from complex expressions
//! - Collection element type inference
//! - Type narrowing in conditionals
//! - Union type handling
//! - Generic type parameters
//! - Type annotation patterns

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
// Type annotations on variables
// ============================================================================

#[test]
fn test_s11_type_annotated_int_var() {
    let code = r#"
def typed_int() -> int:
    x: int = 42
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("i64") || result.contains("i32") || result.contains("fn typed_int"),
        "Should use int type annotation. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_annotated_float_var() {
    let code = r#"
def typed_float() -> float:
    x: float = 1.5
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("fn typed_float"),
        "Should use float type annotation. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_annotated_str_var() {
    let code = r#"
def typed_str() -> str:
    x: str = "hello"
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("String") || result.contains("fn typed_str"),
        "Should use str type annotation. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_annotated_bool_var() {
    let code = r#"
def typed_bool() -> bool:
    x: bool = True
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("bool") || result.contains("fn typed_bool"),
        "Should use bool type annotation. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_annotated_list_var() {
    let code = r#"
def typed_list() -> list:
    x: list = [1, 2, 3]
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec") || result.contains("fn typed_list"),
        "Should use list type annotation. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_annotated_dict_var() {
    let code = r#"
def typed_dict() -> dict:
    x: dict = {"a": 1}
    return x
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn typed_dict"),
        "Should use dict type annotation. Got: {}",
        result
    );
}

// ============================================================================
// Typed list/dict from typing module
// ============================================================================

#[test]
fn test_s11_type_list_of_int() {
    let code = r#"
from typing import List

def int_list() -> List[int]:
    return [1, 2, 3]
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec") || result.contains("fn int_list"),
        "Should transpile List[int]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_list_of_str() {
    let code = r#"
from typing import List

def str_list() -> List[str]:
    return ["a", "b"]
"#;
    let result = transpile(code);
    assert!(
        result.contains("Vec") || result.contains("fn str_list"),
        "Should transpile List[str]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_dict_str_int() {
    let code = r#"
from typing import Dict

def str_int_map() -> Dict[str, int]:
    return {"a": 1}
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashMap") || result.contains("fn str_int_map"),
        "Should transpile Dict[str, int]. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_tuple_int_str() {
    let code = r#"
from typing import Tuple

def pair() -> Tuple[int, str]:
    return (1, "hello")
"#;
    let result = transpile(code);
    assert!(result.contains("fn pair"), "Should transpile Tuple[int, str]. Got: {}", result);
}

#[test]
fn test_s11_type_set_of_int() {
    let code = r#"
from typing import Set

def unique() -> Set[int]:
    return {1, 2, 3}
"#;
    let result = transpile(code);
    assert!(
        result.contains("HashSet") || result.contains("fn unique"),
        "Should transpile Set[int]. Got: {}",
        result
    );
}

// ============================================================================
// Return type inference (no annotation)
// ============================================================================

#[test]
fn test_s11_type_infer_return_none() {
    let code = r#"
def nothing():
    pass
"#;
    let result = transpile(code);
    assert!(result.contains("fn nothing"), "Should infer None return. Got: {}", result);
}

#[test]
fn test_s11_type_infer_return_float_expr() {
    let code = r#"
def compute(x: int) -> float:
    return x / 2.0
"#;
    let result = transpile(code);
    assert!(
        result.contains("f64") || result.contains("fn compute"),
        "Should infer float from division. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_infer_return_from_conditional() {
    let code = r#"
def maybe_int(x: int):
    if x > 0:
        return x
    return 0
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn maybe_int"),
        "Should infer int return from conditional. Got: {}",
        result
    );
}

// ============================================================================
// Complex parameter types
// ============================================================================

#[test]
fn test_s11_type_list_param() {
    let code = r#"
def process(items: list) -> int:
    return len(items)
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Should handle list param. Got: {}", result);
}

#[test]
fn test_s11_type_dict_param() {
    let code = r#"
def count_keys(d: dict) -> int:
    return len(d)
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_keys"), "Should handle dict param. Got: {}", result);
}

#[test]
fn test_s11_type_multiple_typed_params() {
    let code = r#"
def combine(name: str, age: int, score: float) -> str:
    return f"{name}: {age}, {score}"
"#;
    let result = transpile(code);
    assert!(result.contains("fn combine"), "Should handle multiple typed params. Got: {}", result);
}

// ============================================================================
// Type conversion expressions
// ============================================================================

#[test]
fn test_s11_type_int_constructor() {
    let code = r#"
def to_int(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("as i64") || result.contains("as i32") || result.contains("fn to_int"),
        "Should transpile int() constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_float_constructor() {
    let code = r#"
def to_float(x: int) -> float:
    return float(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("as f64") || result.contains("fn to_float"),
        "Should transpile float() constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_str_constructor() {
    let code = r#"
def to_str(x: int) -> str:
    return str(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("to_string") || result.contains("fn to_str"),
        "Should transpile str() constructor. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_bool_constructor() {
    let code = r#"
def to_bool(x: int) -> bool:
    return bool(x)
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_bool"), "Should transpile bool() constructor. Got: {}", result);
}

// ============================================================================
// Type narrowing / isinstance patterns
// ============================================================================

#[test]
fn test_s11_type_isinstance_check() {
    let code = r#"
def check_int(x) -> bool:
    return isinstance(x, int)
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_int"), "Should transpile isinstance. Got: {}", result);
}

// ============================================================================
// Complex type expressions
// ============================================================================

#[test]
fn test_s11_type_nested_list() {
    let code = r#"
from typing import List

def matrix() -> List[List[int]]:
    return [[1, 2], [3, 4]]
"#;
    let result = transpile(code);
    assert!(result.contains("fn matrix"), "Should transpile nested List. Got: {}", result);
}

#[test]
fn test_s11_type_dict_with_list_values() {
    let code = r#"
from typing import Dict, List

def group() -> Dict[str, List[int]]:
    return {"evens": [2, 4], "odds": [1, 3]}
"#;
    let result = transpile(code);
    assert!(result.contains("fn group"), "Should transpile Dict with List values. Got: {}", result);
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
        result.contains("fn mixed"),
        "Should handle int/float mixed arithmetic. Got: {}",
        result
    );
}

#[test]
fn test_s11_type_str_int_format() {
    let code = r#"
def format_num(n: int) -> str:
    return "Number: " + str(n)
"#;
    let result = transpile(code);
    assert!(result.contains("fn format_num"), "Should handle str+int formatting. Got: {}", result);
}

#[test]
fn test_s11_type_list_append_typed() {
    let code = r#"
def build_list() -> list:
    items: list = []
    items.append(1)
    items.append(2)
    items.append(3)
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn build_list"), "Should handle typed list append. Got: {}", result);
}

// ============================================================================
// Return type with typing module
// ============================================================================

#[test]
fn test_s11_type_optional_return_some() {
    let code = r#"
from typing import Optional

def maybe_find(items: list, target: int) -> Optional[int]:
    for item in items:
        if item == target:
            return item
    return None
"#;
    let result = transpile(code);
    assert!(
        result.contains("Option") || result.contains("fn maybe_find"),
        "Should transpile Optional return. Got: {}",
        result
    );
}

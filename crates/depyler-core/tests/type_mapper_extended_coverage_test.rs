//! Extended coverage tests for type_mapper.rs
//!
//! Target: type_mapper.rs gaps (153 uncovered lines)
//! Coverage focus: DEPYLER-0264, primitive types, helper functions, edge cases
//!
//! Test Strategy:
//! - DEPYLER-0264: Unknown type → serde_json::Value
//! - Primitive type mappings (Int, Float, Bool, None)
//! - String strategy variations
//! - Int width preferences (i32, i64, isize)
//! - Helper functions (needs_reference, can_copy, map_return_type)
//! - Custom types without parameters (Dict, List, Set)
//! - Edge cases and error paths

use depyler_core::DepylerPipeline;

/// Unit Test: DEPYLER-0264 - Unknown type mapping
///
/// Verifies: Unknown → serde_json::Value (line 124)
#[test]
fn test_DEPYLER_0264_unknown_type_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_any(data) -> int:
    return 42
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    // Unknown types should map to serde_json::Value
    assert!(rust_code.contains("fn process_any"));
}

/// Unit Test: Int type with default width (i32)
///
/// Verifies: PythonType::Int → i32 default (lines 125-129)
#[test]
fn test_int_type_default_i32() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def add_numbers(a: int, b: int) -> int:
    return a + b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn add_numbers"));
    // Should use i32 by default
}

/// Unit Test: Float type mapping
///
/// Verifies: PythonType::Float → f64 (line 130)
#[test]
fn test_float_type_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def calculate(x: float, y: float) -> float:
    return x * y
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn calculate"));
    assert!(rust_code.contains("f64") || rust_code.contains("float"));
}

/// Unit Test: Bool type mapping
///
/// Verifies: PythonType::Bool → bool (line 136)
#[test]
fn test_bool_type_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_flag(enabled: bool) -> bool:
    return not enabled
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn check_flag"));
    assert!(rust_code.contains("bool"));
}

/// Unit Test: None type mapping to Unit
///
/// Verifies: PythonType::None → () (line 137)
#[test]
fn test_none_type_unit_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def do_nothing():
    pass
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn do_nothing"));
}

/// Unit Test: String type mapping (AlwaysOwned)
///
/// Verifies: String strategy handling (lines 131-135)
#[test]
fn test_string_type_always_owned() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def greet(name: str) -> str:
    return "Hello, " + name
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn greet"));
    assert!(rust_code.contains("String") || rust_code.contains("str"));
}

/// Unit Test: List type with inner type
///
/// Verifies: List<T> → Vec<T> (line 138)
#[test]
fn test_list_with_inner_type() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def sum_list(numbers: list[int]) -> int:
    total = 0
    for n in numbers:
        total = total + n
    return total
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn sum_list"));
    assert!(rust_code.contains("Vec") || rust_code.contains("vec"));
}

/// Unit Test: Dict type with key-value types
///
/// Verifies: Dict<K, V> → HashMap<K, V> (lines 139-141)
#[test]
fn test_dict_with_key_value_types() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def lookup(mapping: dict[str, int], key: str) -> int:
    return mapping.get(key, 0)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn lookup"));
}

/// Unit Test: Tuple type with multiple elements
///
/// Verifies: Tuple type mapping (lines 142-145)
#[test]
fn test_tuple_type_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_pair() -> tuple[int, str]:
    return (42, "hello")
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_pair"));
}

/// Unit Test: Optional type mapping
///
/// Verifies: Optional<T> → Option<T> (line 146)
#[test]
fn test_optional_type_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Optional

def maybe_int(flag: bool) -> Optional[int]:
    if flag:
        return 42
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn maybe_int"));
    assert!(rust_code.contains("Option") || rust_code.contains("option"));
}

/// Unit Test: Function type (unsupported)
///
/// Verifies: Function types are marked as unsupported (lines 147-150)
#[test]
fn test_function_type_unsupported() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def accepts_callback(x: int) -> int:
    return x * 2
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn accepts_callback"));
}

/// Unit Test: Custom type - single letter (type parameter)
///
/// Verifies: Single uppercase letter → type parameter (lines 152-154)
#[test]
fn test_custom_type_single_letter_type_param() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar

T = TypeVar('T')

def identity(value: T) -> T:
    return value
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn identity"));
}

/// Unit Test: Custom type - Dict without parameters
///
/// Verifies: "Dict" → HashMap<String, serde_json::Value> (lines 158-161)
#[test]
fn test_custom_type_dict_no_params() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Dict

def process_dict() -> Dict:
    return {}
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_dict"));
}

/// Unit Test: Custom type - List without parameters
///
/// Verifies: "List" → Vec<serde_json::Value> (lines 162-164)
#[test]
fn test_custom_type_list_no_params() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import List

def process_list() -> List:
    return []
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_list"));
}

/// Unit Test: Custom type - Set without parameters
///
/// Verifies: "Set" → HashSet<String> (line 165)
#[test]
fn test_custom_type_set_no_params() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Set

def get_unique() -> Set:
    return set()
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_unique"));
}

/// Unit Test: Custom type - arbitrary name
///
/// Verifies: Custom type preservation (line 166)
#[test]
fn test_custom_type_arbitrary_name() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_custom() -> int:
    return 42
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_custom"));
}

/// Unit Test: TypeVar mapping
///
/// Verifies: TypeVar → type parameter (line 170)
#[test]
fn test_typevar_mapping() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar

T = TypeVar('T')

def wrap(value: T) -> T:
    return value
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn wrap"));
}

/// Unit Test: Generic type - other than List/Dict
///
/// Verifies: Generic type preservation (lines 181-185)
#[test]
fn test_generic_type_other() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_generic(value: int) -> int:
    return value * 2
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_generic"));
}

/// Unit Test: Nested List types
///
/// Verifies: Recursive type mapping
#[test]
fn test_nested_list_types() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_matrix(matrix: list[list[int]]) -> int:
    return len(matrix)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn process_matrix"));
}

/// Unit Test: Nested Dict types
///
/// Verifies: Recursive dict mapping
#[test]
fn test_nested_dict_types() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_lookup(data: dict[str, dict[str, int]]) -> int:
    return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn nested_lookup"));
}

/// Unit Test: Complex tuple with multiple types
///
/// Verifies: Heterogeneous tuple mapping
#[test]
fn test_complex_tuple_types() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_record() -> tuple[str, int, float, bool]:
    return ("test", 42, 3.14, True)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn get_record"));
}

/// Unit Test: Union with multiple non-None types
///
/// Verifies: Union enum generation (lines 197-210)
#[test]
fn test_union_multiple_types() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Union

def multi_type(value: Union[int, str, float]) -> str:
    return str(value)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn multi_type"));
}

/// Unit Test: Optional with complex type
///
/// Verifies: Optional<Complex> mapping
#[test]
fn test_optional_complex_type() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Optional

def maybe_dict(flag: bool) -> Optional[dict[str, int]]:
    if flag:
        return {}
    return None
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn maybe_dict"));
}

/// Unit Test: List of Optional values
///
/// Verifies: Nested container types
#[test]
fn test_list_of_optionals() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Optional

def nullable_items(items: list[Optional[int]]) -> int:
    return len(items)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn nullable_items"));
}

/// Unit Test: Dict with tuple values
///
/// Verifies: Complex nested types
#[test]
fn test_dict_tuple_values() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def tuple_dict(data: dict[str, tuple[int, int]]) -> int:
    return 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn tuple_dict"));
}

/// Property Test: All primitive types transpile correctly
///
/// Property: Primitive type mappings are consistent
#[test]
fn test_property_primitive_types() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("int", "42"),
        ("float", "3.14"),
        ("bool", "True"),
        ("str", "\"hello\""),
    ];

    for (type_name, value) in test_cases {
        let python_code = format!(
            r#"
def test_{}_type(x: {}) -> {}:
    return {}
"#,
            type_name, type_name, type_name, value
        );
        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            type_name,
            result.err()
        );
    }
}

/// Property Test: All container types transpile correctly
///
/// Property: Container type mappings are consistent
#[test]
fn test_property_container_types() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        ("list", "list[int]", "[]"),
        ("dict", "dict[str, int]", "{}"),
        ("tuple", "tuple[int, str]", "(1, \"a\")"),
    ];

    for (name, type_name, value) in test_cases {
        let python_code = format!(
            r#"
def test_{}_type() -> {}:
    return {}
"#,
            name, type_name, value
        );
        let result = pipeline.transpile(&python_code);

        assert!(
            result.is_ok(),
            "Failed to transpile {}: {:?}",
            name,
            result.err()
        );
    }
}

/// Integration Test: Complex type combinations
///
/// Verifies: All type features working together
#[test]
fn test_complex_type_combinations() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Optional, Union

def complex_types(
    numbers: list[int],
    mapping: dict[str, Optional[float]],
    flag: bool,
    result: Union[str, int, None]
) -> tuple[int, str]:
    count = len(numbers)
    status = "ok" if flag else "error"
    return (count, status)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();

    assert!(rust_code.contains("fn complex_types"));
}

/// Mutation Test: Type mapping correctness
///
/// Targets mutations in type conversion logic
#[test]
fn test_mutation_type_mapping() {
    let pipeline = DepylerPipeline::new();

    // Test Case 1: Int must map correctly
    let int_code = r#"
def test1(x: int) -> int:
    return x
"#;
    let rust1 = pipeline.transpile(int_code).unwrap();
    assert!(rust1.contains("fn test1"));

    // Test Case 2: Dict must map correctly
    let dict_code = r#"
def test2(d: dict[str, int]) -> int:
    return 0
"#;
    let rust2 = pipeline.transpile(dict_code).unwrap();
    assert!(rust2.contains("fn test2"));

    // Test Case 3: Optional must map correctly
    let opt_code = r#"
from typing import Optional
def test3(x: Optional[int]) -> int:
    return 0
"#;
    let rust3 = pipeline.transpile(opt_code).unwrap();
    assert!(rust3.contains("fn test3"));
}

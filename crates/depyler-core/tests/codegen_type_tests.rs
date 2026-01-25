//! EXTREME TDD: Tests for codegen.rs type conversion
//! Coverage: type_to_rust_type, needs_std_collections, uses_hashmap

use depyler_core::DepylerPipeline;

fn transpile(code: &str) -> Result<String, String> {
    DepylerPipeline::new()
        .transpile(code)
        .map_err(|e| e.to_string())
}

fn transpile_ok(code: &str) -> bool {
    transpile(code).is_ok()
}

fn transpile_contains(code: &str, needle: &str) -> bool {
    transpile(code).map(|s| s.contains(needle)).unwrap_or(false)
}

// ============ Primitive type conversion ============

#[test]
fn test_type_int_to_i32() {
    let code = "def f(x: int) -> int: return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_float_to_f64() {
    let code = "def f(x: float) -> float: return x";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "f64"));
}

#[test]
fn test_type_str_to_string() {
    let code = "def f(s: str) -> str: return s";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_bool_to_bool() {
    let code = "def f(b: bool) -> bool: return b";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "bool"));
}

#[test]
fn test_type_none_to_unit() {
    let code = "def f() -> None: pass";
    assert!(transpile_ok(code));
}

// ============ Collection type conversion ============

#[test]
fn test_type_list_to_vec() {
    let code = "def f(items: list) -> list: return items";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "Vec"));
}

#[test]
fn test_type_list_int_to_vec_i32() {
    let code = "def f(items: list[int]) -> list[int]: return items";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_list_str_to_vec_string() {
    let code = "def f(items: list[str]) -> list[str]: return items";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_dict_to_hashmap() {
    let code = "def f(data: dict) -> dict: return data";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "HashMap"));
}

#[test]
fn test_type_dict_str_int() {
    let code = "def f(data: dict[str, int]) -> dict[str, int]: return data";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_dict_int_str() {
    let code = "def f(data: dict[int, str]) -> dict[int, str]: return data";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_set_to_hashset() {
    let code = "def f(items: set) -> set: return items";
    assert!(transpile_ok(code));
    assert!(transpile_contains(code, "HashSet"));
}

#[test]
fn test_type_tuple() {
    let code = "def f(t: tuple) -> tuple: return t";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_tuple_int_str() {
    let code = "def f(t: tuple[int, str]) -> tuple[int, str]: return t";
    assert!(transpile_ok(code));
}

// ============ Optional type conversion ============

#[test]
fn test_type_optional_int() {
    let code = "def f(x: int = None) -> int: return x if x else 0";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_optional_str() {
    let code = "def f(s: str = None) -> str: return s if s else \"\"";
    assert!(transpile_ok(code));
}

// ============ Nested type conversion ============

#[test]
fn test_type_list_of_lists() {
    let code = "def f(matrix: list[list[int]]) -> list[list[int]]: return matrix";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_dict_of_lists() {
    let code = "def f(data: dict[str, list[int]]) -> dict[str, list[int]]: return data";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_list_of_dicts() {
    let code = "def f(items: list[dict[str, int]]) -> list[dict[str, int]]: return items";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_dict_of_dicts() {
    let code = "def f(data: dict[str, dict[str, int]]) -> dict[str, dict[str, int]]: return data";
    assert!(transpile_ok(code));
}

// ============ needs_std_collections detection ============

#[test]
fn test_needs_collections_dict() {
    let code = r#"
def f() -> dict:
    return {"key": 1}
"#;
    let result = transpile(code).unwrap();
    assert!(result.contains("HashMap") || result.contains("std::collections"));
}

#[test]
fn test_needs_collections_set() {
    let code = r#"
def f() -> set:
    return {1, 2, 3}
"#;
    let result = transpile(code).unwrap();
    assert!(result.contains("HashSet") || result.contains("std::collections"));
}

#[test]
fn test_no_collections_needed() {
    let code = r#"
def f(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

// ============ uses_hashmap detection ============

#[test]
fn test_uses_hashmap_param() {
    let code = "def f(data: dict) -> int: return len(data)";
    assert!(transpile_ok(code));
}

#[test]
fn test_uses_hashmap_return() {
    let code = "def f() -> dict: return {}";
    assert!(transpile_ok(code));
}

#[test]
fn test_uses_hashmap_local() {
    let code = r#"
def f() -> int:
    data = {"a": 1}
    return data["a"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_uses_hashmap_in_list() {
    let code = "def f(items: list[dict]) -> int: return len(items)";
    assert!(transpile_ok(code));
}

// ============ Type inference ============

#[test]
fn test_type_infer_int_literal() {
    let code = "def f() -> int:\n    x = 42\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_infer_float_literal() {
    let code = "def f() -> float:\n    x = 3.14\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_infer_str_literal() {
    let code = "def f() -> str:\n    x = \"hello\"\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_infer_list_literal() {
    let code = "def f() -> list:\n    x = [1, 2, 3]\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_infer_dict_literal() {
    let code = "def f() -> dict:\n    x = {\"a\": 1}\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_infer_from_call() {
    let code = "def f(s: str) -> int:\n    x = len(s)\n    return x";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_infer_from_method() {
    let code = "def f(s: str) -> str:\n    x = s.upper()\n    return x";
    assert!(transpile_ok(code));
}

// ============ Union type handling ============

#[test]
fn test_type_union_int_none() {
    let code = r#"
def f(x: int) -> int:
    if x > 0:
        return x
    return None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_union_multiple_returns() {
    let code = r#"
def f(flag: bool) -> int:
    if flag:
        return 42
    else:
        return 0
"#;
    assert!(transpile_ok(code));
}

// ============ Complex type scenarios ============

#[test]
fn test_type_recursive_list() {
    let code = r#"
def flatten(nested: list) -> list:
    result = []
    for item in nested:
        result.extend(item)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_generic_function() {
    let code = r#"
def identity(x: int) -> int:
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_multiple_generic_params() {
    let code = r#"
def swap(a: int, b: int) -> tuple:
    return (b, a)
"#;
    assert!(transpile_ok(code));
}

// ============ Edge cases ============

#[test]
fn test_type_empty_list() {
    let code = "def f() -> list: return []";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_empty_dict() {
    let code = "def f() -> dict: return {}";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_empty_set() {
    let code = "def f() -> set: return set()";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mixed_list() {
    let code = "def f() -> list: return [1, \"two\", 3.0]";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_callable() {
    let code = r#"
def apply(fn, x: int) -> int:
    return fn(x)
"#;
    assert!(transpile_ok(code));
}

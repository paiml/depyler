//! Coverage tests for escape_analysis.rs and type_gen.rs
//!
//! DEPYLER-99MODE-001: Targets escape_analysis.rs (1,280 lines) + type_gen.rs (1,407 lines)
//! Covers: escape detection, ownership inference, type generation,
//! generic type construction, collection type patterns.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Escape analysis - ownership patterns
// ============================================================================

#[test]
fn test_escape_local_only() {
    let code = r#"
def f() -> int:
    x = 42
    y = x + 1
    return y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_returned_value() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_passed_to_function() {
    let code = r#"
def process(items: list) -> int:
    return len(items)

def f() -> int:
    data = [1, 2, 3]
    return process(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_stored_in_collection() {
    let code = r#"
def f() -> list:
    result = []
    for i in range(5):
        result.append(i * 2)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_class_field() {
    let code = r#"
class Container:
    def __init__(self, items: list):
        self.items = items

    def get_items(self) -> list:
        return self.items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_closure_capture() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_escape_conditional_ownership() {
    let code = r#"
def f(x: int) -> list:
    if x > 0:
        result = [1, 2, 3]
    else:
        result = [4, 5, 6]
    return result
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type generation patterns
// ============================================================================

#[test]
fn test_typegen_primitive_int() {
    let code = "def f(x: int) -> int:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_primitive_float() {
    let code = "def f(x: float) -> float:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_primitive_str() {
    let code = "def f(x: str) -> str:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_primitive_bool() {
    let code = "def f(x: bool) -> bool:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_list_generic() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_dict_generic() {
    let code = "def f() -> dict:\n    return {\"a\": 1}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_set_generic() {
    let code = "def f() -> set:\n    return {1, 2, 3}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_tuple_generic() {
    let code = "def f() -> tuple:\n    return (1, \"a\")\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection type construction
// ============================================================================

#[test]
fn test_typegen_vec_from_range() {
    let code = "def f() -> list:\n    return list(range(10))\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_empty_list() {
    let code = r#"
def f() -> list:
    items = []
    items.append(1)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_empty_dict() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = 1
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_nested_collection() {
    let code = r#"
def f() -> dict:
    d = {"items": [1, 2, 3]}
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex type patterns
// ============================================================================

#[test]
fn test_typegen_function_return_inference() {
    let code = r#"
def get_items() -> list:
    return [1, 2, 3]

def f() -> int:
    items = get_items()
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_mixed_types_function() {
    let code = r#"
def f(x: int, y: float, s: str, flag: bool) -> str:
    if flag:
        return s + str(x)
    return str(y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_class_with_typed_fields() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    def greet(self) -> str:
        return "Hello, " + self.name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_comprehension_type() {
    let code = "def f() -> list:\n    return [x * 2 for x in range(10)]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_typegen_lambda_type() {
    let code = r#"
def f(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    assert!(transpile_ok(code));
}

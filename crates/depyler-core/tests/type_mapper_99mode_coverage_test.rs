//! Coverage tests for type_mapper.rs
//!
//! DEPYLER-99MODE-001: Targets type_mapper.rs (2,038 lines)
//! Covers: TypeMapper, map_type, map_return_type, needs_reference,
//! can_copy, string strategies, integer width, NASA mode.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Basic type mapping
// ============================================================================

#[test]
fn test_type_mapper_int() {
    let rust = transpile("def f(x: int) -> int:\n    return x\n");
    assert!(rust.contains("i32") || rust.contains("i64"));
}

#[test]
fn test_type_mapper_float() {
    let rust = transpile("def f(x: float) -> float:\n    return x\n");
    assert!(rust.contains("f64"));
}

#[test]
fn test_type_mapper_str() {
    let rust = transpile("def f(x: str) -> str:\n    return x\n");
    assert!(rust.contains("String") || rust.contains("str"));
}

#[test]
fn test_type_mapper_bool() {
    let rust = transpile("def f(x: bool) -> bool:\n    return x\n");
    assert!(rust.contains("bool"));
}

#[test]
fn test_type_mapper_list() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_dict() {
    let code = "def f() -> dict:\n    return {\"a\": 1}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_tuple() {
    let code = "def f() -> tuple:\n    return (1, 2)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_set() {
    let code = "def f() -> set:\n    return {1, 2, 3}\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Return type mapping
// ============================================================================

#[test]
fn test_type_mapper_return_int() {
    let rust = transpile("def f() -> int:\n    return 42\n");
    assert!(rust.contains("i32") || rust.contains("i64"));
}

#[test]
fn test_type_mapper_return_str() {
    let rust = transpile("def f() -> str:\n    return \"hello\"\n");
    assert!(rust.contains("String") || rust.contains("str"));
}

#[test]
fn test_type_mapper_return_float() {
    let rust = transpile("def f() -> float:\n    return 3.14\n");
    assert!(rust.contains("f64"));
}

#[test]
fn test_type_mapper_return_bool() {
    let rust = transpile("def f() -> bool:\n    return True\n");
    assert!(rust.contains("bool"));
}

#[test]
fn test_type_mapper_return_none() {
    let code = "def f():\n    return None\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Reference/borrow needs
// ============================================================================

#[test]
fn test_type_mapper_list_param() {
    let code = r#"
def f(items: list) -> int:
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_dict_param() {
    let code = r#"
def f(d: dict) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_str_param() {
    let code = "def f(s: str) -> int:\n    return len(s)\n";
    assert!(transpile_ok(code));
}

// ============================================================================
// Copy type detection
// ============================================================================

#[test]
fn test_type_mapper_copy_int() {
    let code = r#"
def f(x: int) -> int:
    y = x
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_copy_float() {
    let code = r#"
def f(x: float) -> float:
    y = x
    return x + y
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_copy_bool() {
    let code = r#"
def f(x: bool) -> bool:
    y = x
    return x and y
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Collection type mapping
// ============================================================================

#[test]
fn test_type_mapper_list_of_int() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_list_of_str() {
    let code = r#"
def f() -> list:
    items = ["a", "b", "c"]
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_dict_str_int() {
    let code = r#"
def f() -> dict:
    d = {"x": 1, "y": 2}
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_set_of_int() {
    let code = r#"
def f() -> set:
    s = {1, 2, 3}
    return s
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex type patterns
// ============================================================================

#[test]
fn test_type_mapper_mixed_types() {
    let code = r#"
def f(x: int, y: float, s: str) -> str:
    return s + str(x) + str(y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_function_chain() {
    let code = r#"
def to_int(s: str) -> int:
    return int(s)

def to_str(n: int) -> str:
    return str(n)

def f(s: str) -> str:
    return to_str(to_int(s) + 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_class_fields() {
    let code = r#"
class Config:
    def __init__(self, name: str, value: int):
        self.name = name
        self.value = value

    def display(self) -> str:
        return self.name + ": " + str(self.value)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_default_values() {
    let code = r#"
def f(x: int = 0, s: str = "default", flag: bool = True) -> str:
    if flag:
        return s + str(x)
    return ""
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_list_operations() {
    let code = r#"
def f(items: list) -> list:
    result = []
    for item in items:
        result.append(item * 2)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_type_mapper_dict_operations() {
    let code = r#"
def f() -> dict:
    d = {}
    d["key"] = "value"
    return d
"#;
    assert!(transpile_ok(code));
}

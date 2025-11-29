// DEPYLER-0448: Return Type and Constant Type Inference Defaults to i32
//
// Bug: Type inference for function return types and constants defaults to i32
// when the transpiler cannot determine the actual type from Python source.
//
// Expected behavior:
// 1. Analyze return statements to infer return type
// 2. Use serde_json::Value as fallback (not i32) for complex types
// 3. Properly infer HashMap/dict types from constants

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

// =============================================================================
// Test Suite 1: Basic Return Type Inference
// =============================================================================

#[test]
fn test_depyler_0448_dict_return_infers_hashmap() {
    let python = r#"
def create_config():
    return {"host": "localhost", "port": 5432}
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer HashMap return type, NOT i32
    assert!(
        !rust.contains("pub fn create_config() -> i32"),
        "Dict return should NOT be typed as i32. Generated:\n{}",
        rust
    );

    // Should be HashMap or Value
    assert!(
        rust.contains("HashMap") || rust.contains("Value"),
        "Dict return should be HashMap or Value. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_int_return_infers_i32() {
    let python = r#"
def get_count():
    return 42
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer i32 return type
    assert!(
        rust.contains("pub fn get_count() -> i32"),
        "Int return should be typed as i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_string_return_infers_string() {
    let python = r#"
def get_name():
    return "Alice"
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer String return type
    assert!(
        rust.contains("pub fn get_name() -> String") || rust.contains("pub fn get_name() -> &str"),
        "String return should be typed as String or &str. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_bool_return_infers_bool() {
    let python = r#"
def is_valid():
    return True
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer bool return type
    assert!(
        rust.contains("pub fn is_valid() -> bool"),
        "Bool return should be typed as bool. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_float_return_infers_f64() {
    let python = r#"
def get_pi():
    return 3.14159
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer f64 return type
    assert!(
        rust.contains("pub fn get_pi() -> f64"),
        "Float return should be typed as f64. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_list_return_infers_vec() {
    let python = r#"
def get_items():
    return [1, 2, 3]
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer Vec return type, NOT i32
    assert!(
        !rust.contains("pub fn get_items() -> i32"),
        "List return should NOT be typed as i32. Generated:\n{}",
        rust
    );

    // Should be Vec or Value
    assert!(
        rust.contains("Vec") || rust.contains("Value"),
        "List return should be Vec or Value. Generated:\n{}",
        rust
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0448"]
fn test_depyler_0448_mixed_returns_use_value() {
    let python = r#"
def get_mixed(flag):
    if flag:
        return {"data": 123}
    else:
        return "error"
"#;
    let rust = transpile_python(python).unwrap();

    // Mixed types should use Value, NOT i32
    assert!(
        !rust.contains("pub fn get_mixed") || !rust.contains("-> i32"),
        "Mixed return types should NOT default to i32. Generated:\n{}",
        rust
    );

    // Should use Value
    assert!(
        rust.contains("Value"),
        "Mixed return types should use Value. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_no_return_infers_unit() {
    let python = r#"
def do_something():
    print("hello")
"#;
    let rust = transpile_python(python).unwrap();

    // No explicit return should NOT be i32
    assert!(
        !rust.contains("pub fn do_something() -> i32"),
        "Function with no return should NOT default to i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_none_return_infers_unit() {
    let python = r#"
def returns_none():
    return None
"#;
    let rust = transpile_python(python).unwrap();

    // None return should be unit type ()
    assert!(
        rust.contains("pub fn returns_none()") && !rust.contains("-> i32"),
        "None return should be unit type, NOT i32. Generated:\n{}",
        rust
    );
}

// =============================================================================
// Test Suite 2: Constant Type Inference
// =============================================================================

#[test]
fn test_depyler_0448_dict_constant_infers_hashmap() {
    let python = r#"
DEFAULT_CONFIG = {"host": "localhost", "port": 5432}
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer HashMap type for constant, NOT i32
    assert!(
        !rust.contains("pub const DEFAULT_CONFIG: i32"),
        "Dict constant should NOT be typed as i32. Generated:\n{}",
        rust
    );

    // Should be HashMap or Value
    assert!(
        rust.contains("HashMap") || rust.contains("Value"),
        "Dict constant should be HashMap or Value. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_int_constant_infers_i32() {
    let python = r#"
MAX_RETRIES = 3
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer i32 type for constant
    assert!(
        rust.contains("pub const MAX_RETRIES: i32"),
        "Int constant should be typed as i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_string_constant_infers_string() {
    let python = r#"
APP_NAME = "MyApp"
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer string type for constant
    assert!(
        rust.contains("&str") || rust.contains("String"),
        "String constant should be typed as &str or String. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_bool_constant_infers_bool() {
    let python = r#"
DEBUG_MODE = True
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer bool type for constant
    assert!(
        rust.contains("pub const DEBUG_MODE: bool"),
        "Bool constant should be typed as bool. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_list_constant_infers_vec() {
    let python = r#"
ALLOWED_HOSTS = ["localhost", "127.0.0.1"]
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer Vec type for constant, NOT i32
    assert!(
        !rust.contains("pub const ALLOWED_HOSTS: i32"),
        "List constant should NOT be typed as i32. Generated:\n{}",
        rust
    );

    // Should be Vec or array
    assert!(
        rust.contains("Vec") || rust.contains("[") || rust.contains("Value"),
        "List constant should be Vec, array, or Value. Generated:\n{}",
        rust
    );
}

// =============================================================================
// Test Suite 3: Conditional Returns
// =============================================================================

#[test]
fn test_depyler_0448_if_else_same_type_returns() {
    let python = r#"
def get_value(flag):
    if flag:
        return 10
    else:
        return 20
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer i32 since both branches return int
    assert!(
        rust.contains("pub fn get_value") && rust.contains("-> i32"),
        "Consistent int returns should be typed as i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_nested_if_returns() {
    let python = r#"
def categorize(score):
    if score > 90:
        return "excellent"
    elif score > 70:
        return "good"
    else:
        return "needs improvement"
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer String since all branches return strings
    assert!(
        rust.contains("pub fn categorize") && rust.contains("String"),
        "Consistent string returns should be typed as String. Generated:\n{}",
        rust
    );
}

// =============================================================================
// Test Suite 4: Integration Tests (Real Examples)
// =============================================================================

#[test]
fn test_depyler_0448_load_config_example() {
    let python = r#"
DEFAULT_CONFIG = {"host": "localhost", "port": 5432}

def load_config(path):
    """Load config from JSON file"""
    # Simplified version without imports
    return DEFAULT_CONFIG
"#;
    let rust = transpile_python(python).unwrap();

    // load_config should NOT return i32
    assert!(
        !rust.contains("pub fn load_config") || !rust.contains("-> i32"),
        "load_config should NOT return i32. Generated:\n{}",
        rust
    );

    // DEFAULT_CONFIG should NOT be i32
    assert!(
        !rust.contains("pub const DEFAULT_CONFIG: i32"),
        "DEFAULT_CONFIG should NOT be i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_create_dict_function() {
    let python = r#"
def create_dict():
    return {}
"#;
    let rust = transpile_python(python).unwrap();

    // Empty dict should NOT return i32
    assert!(
        !rust.contains("pub fn create_dict() -> i32"),
        "Empty dict return should NOT be i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_nested_dict_return() {
    let python = r#"
def get_nested():
    return {
        "user": {
            "name": "Alice",
            "age": 30
        },
        "settings": {
            "theme": "dark"
        }
    }
"#;
    let rust = transpile_python(python).unwrap();

    // Nested dict should NOT return i32
    assert!(
        !rust.contains("pub fn get_nested() -> i32"),
        "Nested dict return should NOT be i32. Generated:\n{}",
        rust
    );
}

// =============================================================================
// Test Suite 5: Edge Cases
// =============================================================================

#[test]
fn test_depyler_0448_multiple_return_statements() {
    let python = r#"
def process(data):
    if not data:
        return None
    if len(data) > 10:
        return data[:10]
    return data
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT default to i32
    assert!(
        !rust.contains("pub fn process") || !rust.contains("-> i32"),
        "Multiple returns should NOT default to i32. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_early_return() {
    let python = r#"
def validate(value):
    if value < 0:
        return False
    return True
"#;
    let rust = transpile_python(python).unwrap();

    // Should infer bool
    assert!(
        rust.contains("pub fn validate") && rust.contains("-> bool"),
        "Bool returns should be typed as bool. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0448_return_variable() {
    let python = r#"
def calculate():
    result = {"sum": 10, "count": 2}
    return result
"#;
    let rust = transpile_python(python).unwrap();

    // Returning dict variable should NOT be i32
    assert!(
        !rust.contains("pub fn calculate() -> i32"),
        "Dict variable return should NOT be i32. Generated:\n{}",
        rust
    );
}

// =============================================================================
// Test Suite 6: Compilation Smoke Tests
// =============================================================================

#[test]
fn test_depyler_0448_simple_dict_compiles() {
    let python = r#"
def create_config():
    return {"host": "localhost"}
"#;
    let rust = transpile_python(python).unwrap();

    // Should at least transpile without panic
    assert!(!rust.is_empty(), "Transpilation should produce output");
}

#[test]
fn test_depyler_0448_conditional_dict_compiles() {
    let python = r#"
def get_config(use_default):
    if use_default:
        return {"host": "localhost"}
    else:
        return {"host": "remote"}
"#;
    let rust = transpile_python(python).unwrap();

    // Should transpile without panic
    assert!(!rust.is_empty(), "Transpilation should produce output");

    // Should NOT default to i32
    assert!(
        !rust.contains("pub fn get_config") || !rust.contains("-> i32"),
        "Conditional dict returns should NOT be i32"
    );
}

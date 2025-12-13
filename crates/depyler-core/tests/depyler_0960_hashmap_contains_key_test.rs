//! DEPYLER-0960: HashMap contains() → contains_key() Tests
//!
//! Tests for fixing E0599 "no method named `contains` found for `HashMap`"
//! Python dict `in` operator and `__contains__` method should generate
//! `contains_key()` for HashMap, not `contains()`.

use depyler_core::DepylerPipeline;

/// Test that dict `in` operator generates contains_key()
#[test]
fn test_depyler_0960_dict_in_operator() {
    let python = r#"
def check_key(data: dict, key: str) -> bool:
    return key in data
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should use contains_key() for dict, not contains()
    assert!(
        code.contains("contains_key") || code.contains(".get("),
        "Dict 'in' operator should use contains_key or get: {}",
        code
    );
}

/// Test dict.__contains__ method generates contains_key()
#[test]
fn test_depyler_0960_dict_contains_method() {
    let python = r#"
def has_setting(config: dict, name: str) -> bool:
    return config.__contains__(name)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should use contains_key() for dict
    assert!(
        code.contains("contains_key"),
        "Dict __contains__ should use contains_key: {}",
        code
    );
}

/// Test that dict variable patterns trigger contains_key()
#[test]
fn test_depyler_0960_dict_pattern_names() {
    let python = r#"
def check_map(mymap: dict, k: str) -> bool:
    return k in mymap

def check_data(data: dict, k: str) -> bool:
    return k in data

def check_config(config: dict, k: str) -> bool:
    return k in config
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // All dict-like names should use contains_key
    // Count occurrences - should have multiple contains_key calls
    let contains_key_count = code.matches("contains_key").count();
    assert!(
        contains_key_count >= 3,
        "Expected at least 3 contains_key calls for dict patterns, got {}: {}",
        contains_key_count,
        code
    );
}

/// Test string.contains() still works correctly
#[test]
fn test_depyler_0960_string_contains_unchanged() {
    let python = r#"
def has_substring(text: str, sub: str) -> bool:
    return sub in text
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // String should use .contains() method (not contains_key)
    assert!(
        code.contains(".contains("),
        "String 'in' operator should use .contains(): {}",
        code
    );
    assert!(
        !code.contains("contains_key"),
        "String should NOT use contains_key: {}",
        code
    );
}

/// Test explicit contains() call on dict
#[test]
fn test_depyler_0960_explicit_contains_on_dict() {
    let python = r#"
def lookup(settings: dict, key: str) -> bool:
    return settings.contains(key)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    // This might fail to transpile since Python dict doesn't have contains()
    // but if it does, it should generate contains_key
    if let Ok(code) = result {
        assert!(
            code.contains("contains_key"),
            "Dict.contains should become contains_key: {}",
            code
        );
    }
}

/// Test nested dict access with in operator
#[test]
fn test_depyler_0960_nested_dict_in() {
    let python = r#"
def has_nested_key(env: dict, section: str, key: str) -> bool:
    if section in env:
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("contains_key"),
        "Nested dict 'in' should use contains_key: {}",
        code
    );
}

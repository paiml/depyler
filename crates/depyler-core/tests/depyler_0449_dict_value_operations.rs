// DEPYLER-0449: Dict Operations Generate Wrong Method Calls on serde_json::Value
//
// Bug: Dict operations (in, [], .get()) on serde_json::Value generate HashMap
// method calls that don't exist on the Value type.
//
// Expected behavior:
// 1. "k in dict" → value.get(k).is_some() or value.as_object()
// 2. "dict[k]" → &value[k] or value.get(k)
// 3. Dict methods → proper Value API usage

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

// =============================================================================
// Test Suite 1: Dict Membership Tests ("in" operator)
// =============================================================================

#[test]
fn test_depyler_0449_dict_contains_key_on_value() {
    let python = r#"
config = {"host": "localhost", "port": 5432}

def has_host(config):
    return "host" in config
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT use .contains_key() directly on Value (doesn't exist)
    assert!(
        !rust.contains("config.contains_key("),
        "Should not call contains_key directly on Value. Generated:\n{}",
        rust
    );

    // Should use Value methods: .get() or .as_object()
    assert!(
        rust.contains(".get(") || rust.contains(".as_object()"),
        "Should use Value methods for membership test. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0449_dict_in_loop() {
    let python = r#"
def check_keys(data, keys):
    for key in keys:
        if key in data:
            return True
    return False
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT use HashMap methods on Value
    assert!(
        !rust.contains("data.contains_key("),
        "Should not use HashMap methods on Value. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0449_isinstance_and_in() {
    let python = r#"
def get_value(data, key):
    if isinstance(data, dict) and key in data:
        return data[key]
    return None
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile without E0599 errors
    assert!(!rust.is_empty());

    // Should not use .contains_key() on Value
    assert!(!rust.contains("data.contains_key("));
}

#[test]
fn test_depyler_0449_not_in_operator() {
    let python = r#"
def is_missing(data, key):
    return key not in data
"#;
    let rust = transpile_python(python).unwrap();

    // Should handle "not in" correctly
    assert!(!rust.contains("data.contains_key("));
}

// =============================================================================
// Test Suite 2: Dict Indexing ("[]" operator)
// =============================================================================

#[test]
fn test_depyler_0449_dict_indexing_on_value() {
    let python = r#"
config = {"host": "localhost"}

def get_host(config):
    return config["host"]
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT cast string to usize
    assert!(
        !rust.contains("as usize"),
        "Should not cast string to usize for indexing. Generated:\n{}",
        rust
    );

    // Should use &value[key] or value.get(key)
    assert!(
        rust.contains("&config[") || rust.contains("config.get(") || rust.contains("config["),
        "Should use proper Value indexing. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0449_nested_dict_access() {
    let python = r#"
def get_nested(config, key):
    keys = key.split(".")
    value = config
    for k in keys:
        value = value[k]
    return value
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT have type cast errors
    assert!(
        !rust.contains("as usize"),
        "Should not cast to usize. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0449_dict_subscript_assign() {
    let python = r#"
def set_value(data, key, value):
    data[key] = value
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile (may need mutable reference)
    assert!(!rust.is_empty());

    // Should not use invalid methods
    assert!(!rust.contains("as usize"));
}

#[test]
fn test_depyler_0449_dict_get_or_default() {
    let python = r#"
config = {"host": "localhost"}

def get_port(config):
    return config.get("port", 8080)
"#;
    let rust = transpile_python(python).unwrap();

    // Should handle .get() method
    assert!(rust.contains("config.get("));
}

// =============================================================================
// Test Suite 3: Dict Iteration
// =============================================================================

#[test]
fn test_depyler_0449_iterate_dict_keys() {
    let python = r#"
def print_keys(data):
    for key in data:
        print(key)
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_iterate_dict_values() {
    let python = r#"
def sum_values(data):
    total = 0
    for value in data.values():
        total += value
    return total
"#;
    let rust = transpile_python(python).unwrap();

    // Should handle .values()
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_iterate_dict_items() {
    let python = r#"
def print_items(data):
    for key, value in data.items():
        print(f"{key}: {value}")
"#;
    let rust = transpile_python(python).unwrap();

    // Should handle .items()
    assert!(!rust.is_empty());
}

// =============================================================================
// Test Suite 4: Dict Methods
// =============================================================================

#[test]
fn test_depyler_0449_dict_get_method() {
    let python = r#"
def safe_get(data, key, default):
    return data.get(key, default)
"#;
    let rust = transpile_python(python).unwrap();

    // Should use Value.get()
    assert!(
        rust.contains("data.get("),
        "Should use .get() method. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0449_dict_keys_method() {
    let python = r#"
def get_keys(data):
    return list(data.keys())
"#;
    let rust = transpile_python(python).unwrap();

    // Should handle .keys() on Value
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_dict_update() {
    let python = r#"
def merge_dicts(target, source):
    target.update(source)
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile (may need special handling for mutable Value)
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_dict_pop() {
    let python = r#"
def remove_key(data, key):
    return data.pop(key, None)
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

// =============================================================================
// Test Suite 5: Complex Real-World Examples
// =============================================================================

#[test]
fn test_depyler_0449_config_get_nested() {
    let python = r#"
DEFAULT_CONFIG = {"host": "localhost", "port": 5432}

def get_nested_value(config, key):
    keys = key.split(".")
    value = config
    for k in keys:
        if isinstance(value, dict) and k in value:
            value = value[k]
        else:
            return None
    return value
"#;
    let rust = transpile_python(python).unwrap();

    // Should NOT use .contains_key() on Value
    assert!(
        !rust.contains("value.contains_key("),
        "Should not use contains_key on Value. Generated:\n{}",
        rust
    );

    // Should NOT cast to usize
    assert!(
        !rust.contains("as usize"),
        "Should not cast to usize. Generated:\n{}",
        rust
    );
}

#[test]
fn test_depyler_0449_config_set_nested() {
    let python = r#"
def set_nested_value(config, key, new_value):
    keys = key.split(".")
    current = config
    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]
    current[keys[-1]] = new_value
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should not use invalid methods
    assert!(!rust.contains("current.contains_key("));
    assert!(!rust.contains("current.insert(") || rust.contains("as_object_mut"));
}

#[test]
fn test_depyler_0449_dict_comprehension_filter() {
    let python = r#"
def filter_dict(data, keys):
    return {k: data[k] for k in keys if k in data}
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());

    // Should not use .contains_key() directly
    assert!(!rust.contains("data.contains_key("));
}

// =============================================================================
// Test Suite 6: Edge Cases
// =============================================================================

#[test]
fn test_depyler_0449_empty_dict() {
    let python = r#"
def is_empty(data):
    return len(data) == 0
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_dict_bool_context() {
    let python = r#"
def has_data(data):
    if data:
        return True
    return False
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_dict_comparison() {
    let python = r#"
def dicts_equal(a, b):
    return a == b
"#;
    let rust = transpile_python(python).unwrap();

    // Should compile
    assert!(!rust.is_empty());
}

// =============================================================================
// Test Suite 7: Integration Test (config_manager compilation)
// =============================================================================

#[test]
#[ignore] // Ignore until GREEN phase is complete
fn test_depyler_0449_config_manager_compiles() {
    // This test requires the actual config_manager.py file
    // Will be enabled when we verify the fix works on real examples
    let python = r#"
DEFAULT_CONFIG = {
    "database": {"host": "localhost", "port": 5432},
    "logging": {"level": "INFO", "file": "app.log"}
}

def get_nested_value(config, key):
    keys = key.split(".")
    value = config
    for k in keys:
        if isinstance(value, dict) and k in value:
            value = value[k]
        else:
            return None
    return value

def set_nested_value(config, key, new_value):
    keys = key.split(".")
    current = config
    for k in keys[:-1]:
        if k not in current:
            current[k] = {}
        current = current[k]
    current[keys[-1]] = new_value
"#;

    let rust = transpile_python(python).unwrap();

    // Should not have E0599 errors
    assert!(!rust.contains("value.contains_key("));
    assert!(!rust.contains("as usize"));

    // Should use proper Value methods
    assert!(rust.contains(".get(") || rust.contains(".as_object()"));
}

// =============================================================================
// Test Suite 8: Compilation Smoke Tests
// =============================================================================

#[test]
fn test_depyler_0449_simple_lookup_compiles() {
    let python = r#"
def get_value(data, key):
    if key in data:
        return data[key]
    return None
"#;
    let rust = transpile_python(python).unwrap();

    // Should transpile without panic
    assert!(!rust.is_empty());
}

#[test]
fn test_depyler_0449_dict_mutation_compiles() {
    let python = r#"
def add_item(data, key, value):
    data[key] = value
"#;
    let rust = transpile_python(python).unwrap();

    // Should transpile
    assert!(!rust.is_empty());
}

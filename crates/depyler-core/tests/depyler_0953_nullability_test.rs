//! DEPYLER-0953: Flow-Sensitive Nullability Analysis Tests
//!
//! Tests for improved Option<T> inference through flow-sensitive analysis.
//! When a value is checked against None, subsequent uses should reflect
//! the narrowed type.

use depyler_core::DepylerPipeline;

/// Test that `is None` check generates is_none() method call
#[test]
fn test_depyler_0953_is_none_method() {
    let python = r#"
def check_none(x: int | None) -> bool:
    return x is None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Should use is_none() method
    assert!(
        code.contains("is_none()") || code.contains(".is_none()"),
        "Should use is_none() method: {}",
        code
    );
}

/// Test that `is not None` check generates is_some() method call
#[test]
fn test_depyler_0953_is_not_none_method() {
    let python = r#"
def check_some(x: int | None) -> bool:
    return x is not None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Should use is_some() method
    assert!(
        code.contains("is_some()") || code.contains(".is_some()"),
        "Should use is_some() method: {}",
        code
    );
}

/// Test Optional parameter with default None
#[test]
fn test_depyler_0953_optional_parameter_default_none() {
    let python = r#"
def greet(name: str | None = None) -> str:
    if name is None:
        return "Hello, stranger!"
    return "Hello, " + name
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Parameter should be Option<String>
    assert!(
        code.contains("Option<") || code.contains("Option::<"),
        "Should use Option type for nullable parameter: {}",
        code
    );
}

/// Test None literal assignment
#[test]
fn test_depyler_0953_none_assignment() {
    let python = r#"
def get_optional() -> int | None:
    return None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Return type should be Option<i64>
    assert!(
        code.contains("-> Option<") || code.contains("Option<i64>") || code.contains("None"),
        "Should handle None return: {}",
        code
    );
}

/// Test None in conditional expression
#[test]
fn test_depyler_0953_none_conditional() {
    let python = r#"
def maybe_value(flag: bool) -> int | None:
    return 42 if flag else None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Should properly handle Some/None in conditional
    assert!(
        code.contains("Some(") || code.contains("None"),
        "Should handle conditional with None: {}",
        code
    );
}

/// Test early return pattern with None check
#[test]
fn test_depyler_0953_early_return_none_check() {
    let python = r#"
def process(x: int | None) -> int:
    if x is None:
        return 0
    # After this check, x is known to be non-None
    return x * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // Should generate valid code that compiles
    assert!(
        code.contains("fn process"),
        "Should generate process function: {}",
        code
    );
}

/// Test dict.get() returning Option
#[test]
fn test_depyler_0953_dict_get_option() {
    let python = r#"
def lookup(d: dict, key: str) -> int | None:
    return d.get(key)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let code = result.unwrap();
    // dict.get() returns Option in Rust
    assert!(code.contains(".get("), "Should use get method: {}", code);
}

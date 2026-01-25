//! DEPYLER-1203: Dict DepylerValue Boundary Enforcement Tests
//!
//! Tests for E0308 type mismatch errors when Dict values need DepylerValue wrapping.
//! Follows DEPYLER-1201 pattern for Vec<DepylerValue>.
//!
//! When target type is `HashMap<String, DepylerValue>` (from Dict[str, Any]):
//! - `1` → `DepylerValue::Int(1 as i64)`
//! - `3.14` → `DepylerValue::Float(3.14)`
//! - `"hello"` → `DepylerValue::Str("hello".to_string())`
//! - Variables → `DepylerValue::from(var)`

use depyler_core::DepylerPipeline;
use std::process::Command;

/// Helper to check if generated code compiles
fn check_compiles(rust_code: &str) -> Result<(), String> {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("depyler_1203_test.rs");
    std::fs::write(&temp_file, rust_code).map_err(|e| format!("Write error: {}", e))?;

    let output = Command::new("rustc")
        .args([
            "--crate-type",
            "lib",
            "--edition",
            "2021",
            "-o",
            "/dev/null",
        ])
        .arg(&temp_file)
        .output()
        .map_err(|e| format!("Compile error: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Compilation failed:\n{}", stderr))
    }
}

/// Test 1: Dict subscript assignment with mixed types
/// d["int"] = 42, d["str"] = "hello" where d: Dict[str, Any]
#[test]
fn test_depyler_1203_dict_subscript_mixed_types() {
    let python = r#"
def build_config() -> dict:
    d = {}
    d["count"] = 42
    d["name"] = "test"
    d["enabled"] = True
    return d
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Dict with mixed subscript values should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    // Check for DepylerValue wrapping in subscript assignments
    match check_compiles(&rust_code) {
        Ok(()) => (),
        Err(e) => {
            if e.contains("E0308") || e.contains("mismatched types") {
                panic!(
                    "DEPYLER-1203: Dict subscript boundary issue - \
                     values not wrapped in DepylerValue.\n\
                     Error: {}\n\nGenerated code:\n{}",
                    e, rust_code
                );
            }
            eprintln!("Note: Other compilation issues: {}", e);
        }
    }
}

/// Test 2: Dict.update() with another dict
/// d.update({"key": value}) where d: Dict[str, Any]
#[test]
fn test_depyler_1203_dict_update_boundary() {
    let python = r#"
def merge_configs(base: dict, extra: dict) -> dict:
    result = {"version": 1}
    result.update({"name": "merged"})
    result.update(base)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Dict.update() should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    match check_compiles(&rust_code) {
        Ok(()) => (),
        Err(e) => {
            if e.contains("E0308") || e.contains("mismatched types") {
                panic!(
                    "DEPYLER-1203: Dict.update() boundary issue - \
                     values not wrapped in DepylerValue.\n\
                     Error: {}\n\nGenerated code:\n{}",
                    e, rust_code
                );
            }
            eprintln!("Note: Other compilation issues: {}", e);
        }
    }
}

/// Test 3: Dict literal with DepylerValue target type
#[test]
fn test_depyler_1203_dict_literal_boundary() {
    let python = r#"
def create_record(name: str, age: int) -> dict:
    return {"name": name, "age": age, "active": True}
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Dict literal with mixed values should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    match check_compiles(&rust_code) {
        Ok(()) => (),
        Err(e) => {
            if e.contains("E0308") || e.contains("mismatched types") {
                panic!(
                    "DEPYLER-1203: Dict literal boundary issue.\n\
                     Error: {}\n\nGenerated code:\n{}",
                    e, rust_code
                );
            }
            eprintln!("Note: Other compilation issues: {}", e);
        }
    }
}

/// Test 4: Nested dict assignment
#[test]
fn test_depyler_1203_nested_dict_boundary() {
    let python = r#"
def build_nested() -> dict:
    outer = {}
    outer["inner"] = {"a": 1, "b": 2}
    outer["value"] = 42
    return outer
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Nested dict assignment should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    match check_compiles(&rust_code) {
        Ok(()) => (),
        Err(e) => {
            if e.contains("E0308") || e.contains("mismatched types") {
                panic!(
                    "DEPYLER-1203: Nested dict boundary issue.\n\
                     Error: {}\n\nGenerated code:\n{}",
                    e, rust_code
                );
            }
            eprintln!("Note: Other compilation issues: {}", e);
        }
    }
}

/// Test 5: Dict with int literal values requiring wrap
#[test]
fn test_depyler_1203_dict_int_value_boundary() {
    let python = r#"
def count_items(items: list) -> dict:
    counts = {}
    for item in items:
        counts[item] = 1
    return counts
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Dict with int values should transpile: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();
    eprintln!("Generated code:\n{}", rust_code);

    // Should have DepylerValue wrapping for int literal
    if rust_code.contains("DepylerValue") {
        assert!(
            rust_code.contains("DepylerValue::Int") || rust_code.contains("DepylerValue::from"),
            "Int values should use DepylerValue::Int or DepylerValue::from.\n\nGenerated code:\n{}",
            rust_code
        );
    }
}

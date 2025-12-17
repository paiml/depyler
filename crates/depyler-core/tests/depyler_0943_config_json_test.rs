//! DEPYLER-0943: Config/JSON Handling Tests
//!
//! Tests for fixing E0308 type mismatches when working with
//! JSON/config data and serde_json::Value types.

use depyler_core::DepylerPipeline;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

// DEPYLER-1028: Use unique temp files to prevent race conditions in parallel tests
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir() -> String {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    format!("/tmp/depyler_0943_{}_{}", pid, id)
}

/// Helper to check if generated Rust code compiles
fn compiles_with_cargo(code: &str) -> bool {
    // Create temp directory with Cargo.toml
    let temp_dir = unique_temp_dir();
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(format!("{}/src", temp_dir)).unwrap();
    std::fs::write(
        format!("{}/Cargo.toml", temp_dir),
        r#"[package]
name = "test_0943"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
"#,
    )
    .unwrap();
    std::fs::write(format!("{}/src/lib.rs", temp_dir), code).unwrap();

    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run cargo");

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);

    output.status.success()
}

/// Get compilation errors
fn compile_errors_cargo(code: &str) -> String {
    let temp_dir = unique_temp_dir();
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(format!("{}/src", temp_dir)).unwrap();
    std::fs::write(
        format!("{}/Cargo.toml", temp_dir),
        r#"[package]
name = "test_0943"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
"#,
    )
    .unwrap();
    std::fs::write(format!("{}/src/lib.rs", temp_dir), code).unwrap();

    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run cargo");

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);

    String::from_utf8_lossy(&output.stderr).to_string()
}

/// Test JSON dict literal creation
#[test]
fn test_depyler_0943_json_dict_literal() {
    let python = r#"
def create_config() -> dict:
    return {"name": "test", "value": 42}
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("fn create_config"),
        "Should generate function: {}",
        code
    );
}

/// Test JSON dict key access - CRITICAL TEST FOR DEPYLER-0943
/// This verifies that dict["key"] with str return type compiles correctly.
/// Previously generated: config.get("name").cloned().unwrap_or_default()
/// Which returns serde_json::Value, not String (E0308 type mismatch).
#[test]
fn test_depyler_0943_dict_key_access() {
    let python = r#"
def get_name(config: dict) -> str:
    return config["name"]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("fn get_name"),
        "Should generate function: {}",
        code
    );
    // Should use .get() or indexing for dict access
    assert!(
        code.contains(".get(") || code.contains("["),
        "Should generate dict access: {}",
        code
    );

    // CRITICAL: Must convert serde_json::Value to String
    // The fix adds .as_str().unwrap_or("").to_string()
    assert!(
        code.contains(".as_str()"),
        "Should convert Value to String: {}",
        code
    );

    // CRITICAL: Generated code must compile without E0308
    if !compiles_with_cargo(&code) {
        let errors = compile_errors_cargo(&code);
        panic!(
            "Generated code should compile. Errors:\n{}\n\nGenerated code:\n{}",
            errors, code
        );
    }
}

/// Test JSON dict with nested values
#[test]
fn test_depyler_0943_nested_dict() {
    let python = r#"
def nested_config() -> dict:
    return {
        "database": {
            "host": "localhost",
            "port": 5432
        },
        "debug": True
    }
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("fn nested_config"),
        "Should generate function: {}",
        code
    );
}

/// Test dict.get() with default value
#[test]
fn test_depyler_0943_dict_get_with_default() {
    let python = r#"
def safe_get(config: dict, key: str) -> str:
    return config.get(key, "default")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should use .get().unwrap_or() pattern
    assert!(
        code.contains(".get(") || code.contains("unwrap_or"),
        "Should generate get with default: {}",
        code
    );
}

/// Test JSON-like dict iteration
#[test]
fn test_depyler_0943_dict_iteration() {
    let python = r#"
def print_config(config: dict) -> None:
    for key in config:
        print(key)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("fn print_config"),
        "Should generate function: {}",
        code
    );
}

/// Test dict.keys() method
#[test]
fn test_depyler_0943_dict_keys() {
    let python = r#"
def get_keys(config: dict) -> list:
    return list(config.keys())
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should generate .keys() call
    assert!(
        code.contains(".keys()"),
        "Should use keys() method: {}",
        code
    );
}

/// Test dict.values() method
#[test]
fn test_depyler_0943_dict_values() {
    let python = r#"
def get_values(config: dict) -> list:
    return list(config.values())
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should generate .values() call
    assert!(
        code.contains(".values()"),
        "Should use values() method: {}",
        code
    );
}

/// Test dict.items() method
#[test]
fn test_depyler_0943_dict_items() {
    let python = r#"
def get_items(config: dict) -> list:
    return list(config.items())
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should generate .iter() or items equivalent
    assert!(
        code.contains(".iter()") || code.contains(".items()"),
        "Should use iter/items method: {}",
        code
    );
}

/// Test dict update/merge
#[test]
fn test_depyler_0943_dict_update() {
    let python = r#"
def merge_config(base: dict, override: dict) -> dict:
    result = base.copy()
    result.update(override)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    assert!(
        code.contains("fn merge_config"),
        "Should generate function: {}",
        code
    );
}

/// Test dict len()
#[test]
fn test_depyler_0943_dict_len() {
    let python = r#"
def config_size(config: dict) -> int:
    return len(config)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok(), "Transpilation should succeed: {:?}", result.err());

    let code = result.unwrap();
    // Should use .len()
    assert!(
        code.contains(".len()"),
        "Should use len() method: {}",
        code
    );
}

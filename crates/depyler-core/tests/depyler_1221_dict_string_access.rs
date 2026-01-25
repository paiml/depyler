//! TDD Test for DEPYLER-1221: Dict String Access Double Unwrapping Bug
//!
//! Bug: When accessing a dict with a string key, the transpiler incorrectly generates
//! `.as_str().unwrap_or("")` calls even when the dict value type is already String.
//!
//! Example:
//! ```python
//! def make_config() -> Dict[str, str]:
//!     return {"key": "value"}
//!
//! def get_key() -> str:
//!     config = make_config()
//!     return config["key"]  # Bug: generates .as_str().unwrap_or("") on String
//! ```
//!
//! Root cause: `is_dict_index_access()` returns true for any dict access without checking
//! if the VALUE type is serde_json::Value (where the conversion is needed) or already String.
//!
//! Fix: Check the dict's value type in ctx.var_types before applying the conversion.
//! Only add `.as_str().unwrap_or("")` when value type is serde_json::Value or Unknown.

use depyler_core::DepylerPipeline;

fn transpile(python: &str) -> Result<String, String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).map_err(|e| e.to_string())
}

fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("Failed to create src dir");
    let lib_file = src_dir.join("lib.rs");

    std::fs::write(
        &cargo_toml,
        r#"[package]
name = "test_lib"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
path = "src/lib.rs"
"#,
    )
    .expect("Failed to write Cargo.toml");

    std::fs::write(&lib_file, rust_code).expect("Failed to write lib.rs");

    let output = std::process::Command::new("cargo")
        .args(["check", "--quiet"])
        .current_dir(temp_dir.path())
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_SHOW_ENV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("LLVM_PROFILE_FILE")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_INCREMENTAL")
        .env_remove("CARGO_BUILD_JOBS")
        .env_remove("CARGO_TARGET_DIR")
        .output()
        .expect("Failed to run cargo");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Rust compilation failed for {}:\n{}\n\nGenerated code:\n{}",
            test_name, stderr, rust_code
        );
    }
}

/// DEPYLER-1221: Dict[str, str] access should NOT have .as_str().unwrap_or("")
/// The value type is already String, no conversion needed
#[test]
fn test_dict_string_string_access() {
    let python = r#"
from typing import Dict

def make_config() -> Dict[str, str]:
    return {"key": "value"}

def get_key() -> str:
    config = make_config()
    return config["key"]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT have .as_str().unwrap_or("") because value type is String
    assert!(
        !rust.contains(r#".as_str().unwrap_or("")"#),
        "Should not have .as_str().unwrap_or(\"\") for Dict[str, str]. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "dict_string_string_access");
}

/// DEPYLER-1221: Dict[str, int] access should NOT have .as_str().unwrap_or("")
#[test]
fn test_dict_string_int_access() {
    let python = r#"
from typing import Dict

def make_counts() -> Dict[str, int]:
    return {"a": 1, "b": 2}

def get_count() -> int:
    counts = make_counts()
    return counts["a"]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT have .as_str().unwrap_or("") for dict access because value type is int
    // Note: .as_str() may appear elsewhere (e.g., in regex helpers)
    assert!(
        !rust.contains(r#".as_str().unwrap_or("")"#),
        "Should not have .as_str().unwrap_or(\"\") for Dict[str, int]. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "dict_string_int_access");
}

/// DEPYLER-1221: Dict inferred from literal should have correct value type
#[test]
fn test_dict_literal_inferred_type() {
    let python = r#"
def get_name() -> str:
    data = {"name": "Alice", "city": "NYC"}
    return data["name"]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Dict literal {"name": "Alice"} should be inferred as Dict[str, str]
    // So no .as_str().unwrap_or("") should be generated
    assert!(
        !rust.contains(r#".as_str().unwrap_or("")"#),
        "Literal dict should infer string value type. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "dict_literal_inferred_type");
}

/// DEPYLER-1221: Nested dict access with string values
#[test]
fn test_nested_dict_string_access() {
    let python = r#"
from typing import Dict

def make_nested() -> Dict[str, Dict[str, str]]:
    return {"outer": {"inner": "value"}}

def get_inner() -> str:
    data = make_nested()
    inner = data["outer"]
    return inner["inner"]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "nested_dict_string_access");
}

/// DEPYLER-1221: Dict access in conditional
#[test]
fn test_dict_access_in_conditional() {
    let python = r#"
from typing import Dict

def check_config() -> str:
    config: Dict[str, str] = {"mode": "debug"}
    if config["mode"] == "debug":
        return "debugging"
    return "production"
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "dict_access_in_conditional");
}

/// DEPYLER-1221: Dict with Any value type uses DepylerValue
/// This test verifies the code compiles correctly
#[test]
fn test_dict_any_value_compiles() {
    let python = r#"
from typing import Dict, Any

def process_data(data: Dict[str, Any]) -> str:
    name = data["name"]
    return str(name)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Dict[str, Any] maps to DepylerValue in NASA mode
    // Just verify it compiles - the exact conversion depends on mode
    assert_compiles(&rust, "dict_any_value_compiles");
}

/// DEPYLER-1221: Dict access with variable key
#[test]
fn test_dict_access_variable_key() {
    let python = r#"
from typing import Dict

def get_value(config: Dict[str, str], key: str) -> str:
    return config[key]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "dict_access_variable_key");
}

/// DEPYLER-1221: Dict from function with inferred return type
#[test]
fn test_dict_inferred_return_type() {
    let python = r#"
def make_settings():
    return {"verbose": "true", "level": "info"}

def get_setting() -> str:
    settings = make_settings()
    return settings["verbose"]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // make_settings() infers Dict[str, str] from the literal
    // So settings["verbose"] should NOT have .as_str().unwrap_or("")
    assert!(
        !rust.contains(r#".as_str().unwrap_or("")"#),
        "Inferred Dict[str, str] should not have .as_str().unwrap_or(\"\"). Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "dict_inferred_return_type");
}

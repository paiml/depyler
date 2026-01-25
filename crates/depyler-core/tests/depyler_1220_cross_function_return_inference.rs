//! TDD Test for DEPYLER-1220: Cross-function return type inference
//!
//! Bug: When a function lacks an explicit return type annotation, its return type
//! is not propagated to callers even if it can be inferred from return statements.
//!
//! Example:
//! ```python
//! def get_value():  # No return type annotation
//!     return 42     # Returns int
//!
//! def caller() -> int:
//!     x = get_value()  # x should be inferred as int
//!     return x
//! ```
//!
//! Root cause: `func_return_types` in `type_propagation.rs` only collects
//! functions with non-Unknown return types, missing inference from return stmts.

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

/// DEPYLER-1220: Function with no return type annotation but int return
/// The return type should be inferred from the return statement
#[test]
fn test_infer_return_type_from_literal() {
    let python = r#"
def get_value():
    return 42

def use_value() -> int:
    x = get_value()
    return x
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // get_value should return i64 (or equivalent int type)
    // use_value should compile - x should be inferred as i64
    assert_compiles(&rust, "infer_return_type_from_literal");
}

/// DEPYLER-1220: Function returning string literal without annotation
#[test]
fn test_infer_return_type_string() {
    let python = r#"
def get_name():
    return "Alice"

def greet() -> str:
    name = get_name()
    return "Hello " + name
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "infer_return_type_string");
}

/// DEPYLER-1220: Function returning list without annotation
#[test]
fn test_infer_return_type_list() {
    let python = r#"
from typing import List

def get_numbers():
    return [1, 2, 3]

def sum_numbers() -> int:
    nums = get_numbers()
    total = 0
    for n in nums:
        total = total + n
    return total
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // nums should be Vec<i64>, n should be i64
    assert_compiles(&rust, "infer_return_type_list");
}

/// DEPYLER-1220: Function returning bool from comparison without annotation
#[test]
fn test_infer_return_type_bool() {
    let python = r#"
def is_positive(x: int):
    return x > 0

def check_value(val: int) -> bool:
    result = is_positive(val)
    return result
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "infer_return_type_bool");
}

/// DEPYLER-1220: Chain of unannotated functions
/// Each function's return type must propagate to the next
#[test]
fn test_chained_unannotated_functions() {
    let python = r#"
def step1():
    return 10

def step2():
    x = step1()
    return x + 5

def step3() -> int:
    y = step2()
    return y * 2
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "chained_unannotated_functions");
}

/// DEPYLER-1220: Function with multiple return statements (same type)
#[test]
fn test_multiple_returns_same_type() {
    let python = r#"
def get_sign(x: int):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0

def compute(val: int) -> int:
    sign = get_sign(val)
    return sign * val
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "multiple_returns_same_type");
}

/// DEPYLER-1220: Function returning dict without annotation
/// DEPYLER-1221: Fixed dict string access codegen - now we can access dict values directly
#[test]
fn test_infer_return_type_dict() {
    let python = r#"
from typing import Dict

def make_config():
    return {"key": "value"}

def get_key() -> str:
    config = make_config()
    return config["key"]
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should NOT have .as_str().unwrap_or("") because config has type Dict[str, str]
    // The value is already a String, not serde_json::Value
    assert!(
        !rust.contains(r#".as_str().unwrap_or("")"#),
        "Should not have .as_str().unwrap_or(\"\") conversion for Dict[str, str]. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "infer_return_type_dict");
}

/// DEPYLER-1220: Function returning None (no return statement)
#[test]
fn test_infer_return_type_none() {
    let python = r#"
def do_nothing():
    x = 1

def caller():
    do_nothing()
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "infer_return_type_none");
}

/// DEPYLER-1220: Recursive function without annotation
#[test]
fn test_recursive_function_inference() {
    let python = r#"
def factorial(n: int):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def compute_factorial(x: int) -> int:
    result = factorial(x)
    return result
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "recursive_function_inference");
}

/// DEPYLER-1220: Function returning tuple without annotation
#[test]
fn test_infer_return_type_tuple() {
    let python = r#"
from typing import Tuple

def get_coords():
    return (10, 20)

def use_coords() -> int:
    x, y = get_coords()
    return x + y
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "infer_return_type_tuple");
}

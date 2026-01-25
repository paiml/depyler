//! TDD Test for DEPYLER-1219: Recursive deep generic inference for nested empty literals
//!
//! Bug: When a variable has a deep generic type like `Dict[str, Dict[int, List[str]]]`,
//! empty literals inside nested positions don't inherit the proper type context.
//!
//! Example:
//! ```python
//! data: Dict[str, Dict[int, List[str]]] = {"key": {1: []}}
//! ```
//! The `[]` should become `Vec<String>`, not `Vec<DepylerValue>` or `Vec<String>` (default).
//!
//! Root cause: Type context (current_assign_type) is set at the outer assignment level
//! but not recursively propagated to nested dict/list literal values.

use depyler_core::DepylerPipeline;

fn transpile(python: &str) -> Result<String, String> {
    // Use the production pipeline - same path as CLI
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

/// DEPYLER-1219: Deep nested dict with inner empty list should infer correct type
/// Python: `data: Dict[str, Dict[int, List[str]]] = {"key": {1: []}}`
/// The inner `[]` should become `Vec<String>`, inheriting from the deep generic annotation
#[test]
fn test_deep_dict_with_empty_list() {
    let python = r#"
from typing import Dict, List

def create_deep_dict() -> Dict[str, Dict[int, List[str]]]:
    data: Dict[str, Dict[int, List[str]]] = {"key": {1: []}}
    return data
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The empty list [] should be typed as Vec<String> (from List[str])
    // Should NOT be Vec<DepylerValue> or untyped
    assert!(
        rust.contains("Vec<String>") || rust.contains("Vec<&str>"),
        "Deep nested empty list should have concrete String element type. Generated:\n{}",
        rust
    );

    // Should compile without type errors
    assert_compiles(&rust, "deep_dict_with_empty_list");
}

/// DEPYLER-1219: Nested dict inside dict with type annotation
/// Python: `outer: Dict[str, Dict[str, int]] = {"a": {}}`
/// The inner `{}` should become `HashMap<String, i32>`
#[test]
fn test_nested_dict_with_empty_inner() {
    let python = r#"
from typing import Dict

def create_nested() -> Dict[str, Dict[str, int]]:
    outer: Dict[str, Dict[str, int]] = {"a": {}}
    return outer
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The inner empty dict {} should be typed as HashMap<String, i32>
    // Look for evidence of proper nested typing
    assert!(
        rust.contains("HashMap<String, i32>") || rust.contains("HashMap<String, i64>"),
        "Nested empty dict should have concrete value type. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "nested_dict_with_empty_inner");
}

/// DEPYLER-1219: Triple nesting - Dict of Dict of List
/// Python: `d: Dict[str, Dict[str, List[int]]] = {"a": {"b": []}}`
#[test]
fn test_triple_nested_empty_list() {
    let python = r#"
from typing import Dict, List

def triple_nest() -> Dict[str, Dict[str, List[int]]]:
    d: Dict[str, Dict[str, List[int]]] = {"a": {"b": []}}
    return d
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The innermost [] should be Vec<i32> or Vec<i64>
    assert!(
        rust.contains("Vec<i32>") || rust.contains("Vec<i64>"),
        "Triple nested empty list should have concrete int type. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "triple_nested_empty_list");
}

/// DEPYLER-1219: Optional wrapping with deep nesting
/// Python: `memo: Optional[Dict[int, Dict[int, int]]] = {}`
/// The outer {} should become HashMap<i32, HashMap<i32, i32>>
#[test]
fn test_optional_deep_dict() {
    let python = r#"
from typing import Dict, Optional

def with_memo() -> Optional[Dict[int, Dict[int, int]]]:
    memo: Optional[Dict[int, Dict[int, int]]] = {}
    return memo
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should have HashMap with i32 key type
    assert!(
        rust.contains("HashMap<i32") || rust.contains("HashMap<i64"),
        "Optional-wrapped deep dict should have concrete key type. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "optional_deep_dict");
}

/// DEPYLER-1219: List of Dict with empty dict element
/// Python: `items: List[Dict[str, int]] = [{}]`
/// The inner {} should become HashMap<String, i32>
#[test]
fn test_list_of_dict_with_empty() {
    let python = r#"
from typing import Dict, List

def list_of_dicts() -> List[Dict[str, int]]:
    items: List[Dict[str, int]] = [{}]
    return items
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The empty dict inside the list should be HashMap<String, i32>
    assert!(
        rust.contains("HashMap<String, i32>") || rust.contains("HashMap<String, i64>"),
        "Empty dict in List[Dict] should have concrete types. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "list_of_dict_with_empty");
}

/// DEPYLER-1219: Deep generic with memoization pattern (common algorithm pattern)
/// Python: Recursive fibonacci with memoization uses Dict[int, int]
#[test]
fn test_memoization_dict_pattern() {
    let python = r#"
from typing import Dict

def fib_memo(n: int, memo: Dict[int, int] = None) -> int:
    if memo is None:
        memo = {}
    if n in memo:
        return memo[n]
    if n <= 1:
        return n
    result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
    memo[n] = result
    return result
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // memo = {} should be HashMap<i32, i32> (from Dict[int, int] parameter type)
    // This tests that we propagate from Optional[Dict[int, int]] annotation
    assert!(
        rust.contains("HashMap<i32") || rust.contains("HashMap<i64"),
        "Memoization dict should have concrete int key type. Generated:\n{}",
        rust
    );

    assert_compiles(&rust, "memoization_dict_pattern");
}

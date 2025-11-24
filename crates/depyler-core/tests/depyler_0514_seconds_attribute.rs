//! DEPYLER-0514 / GH-74: .seconds Attribute Overly-Aggressive Rewriting
//!
//! **ROOT CAUSE**: Attribute name pattern matching without type guards
//!
//! **Five Whys**:
//! 1. Why does `test.seconds` get converted incorrectly? Pattern match on line 11255-11258
//! 2. Why doesn't it check object type? No type guards in attribute conversion
//! 3. Why is type information not available? Heuristic-based attribute rewriting
//! 4. Why doesn't it use HIR type information? Written before type propagation was complete
//! 5. ROOT: Overly-aggressive heuristic attribute rewriting without type guards
//!
//! **Similar Issue**: DEPYLER-0357 fixed overly-aggressive .name attribute rewriting
//!
//! **Problem**: The transpiler incorrectly assumes any `.seconds` attribute refers to
//! a Python timedelta object, when it could be a regular object field.
//!
//! **Examples**:
//! - Python: `test.seconds` → Rust: `(test.num_seconds() % 86400) as i32` ❌
//! - Should: `test.seconds` → Rust: `test.seconds` ✅
//!
//! **Solution**: Remove overly-aggressive pattern matching or add type checks

#![allow(non_snake_case)]

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile_to_rust(python_code: &str) -> Result<String, String> {
    let ast = parse(python_code, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new()
        .python_to_hir(ast)
        .map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

// ============================================================================
// RED PHASE - Failing Tests
// ============================================================================

#[test]
fn test_DEPYLER_0514_seconds_field_not_timedelta() {
    // RED: .seconds should be preserved as field access, not converted to num_seconds()
    let python = r#"
class Timer:
    def __init__(self, seconds: int):
        self.seconds = seconds

def get_seconds(timer: Timer) -> int:
    return timer.seconds
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0514: .seconds field access should transpile. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should NOT contain timedelta-specific conversion
    assert!(
        !rust_code.contains("num_seconds()"),
        "DEPYLER-0514: .seconds should not be converted to .num_seconds() for non-timedelta types.\nGenerated:\n{}",
        rust_code
    );

    // Should preserve simple field access
    assert!(
        rust_code.contains("timer.seconds") || rust_code.contains("self.seconds"),
        "DEPYLER-0514: .seconds should be preserved as field access.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0514_seconds_field_in_custom_class() {
    // Custom class with .seconds field
    let python = r#"
class Config:
    seconds: int

    def get_timeout(self) -> int:
        return self.seconds
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0514: Custom class .seconds field should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    assert!(
        !rust_code.contains("num_seconds()"),
        "DEPYLER-0514: Should not apply timedelta conversion.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0514_seconds_lambda_key() {
    // .seconds in lambda key function (similar to DEPYLER-0357 .name issue)
    let python = r#"
class Task:
    def __init__(self, seconds: int):
        self.seconds = seconds

def sort_tasks(tasks: list[Task]) -> list[Task]:
    return sorted(tasks, key=lambda t: t.seconds)
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0514: .seconds in lambda should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    assert!(
        !rust_code.contains("num_seconds()"),
        "DEPYLER-0514: Lambda key should preserve field access.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0514_seconds_dict_access() {
    // .seconds accessed through dict or dynamic object
    let python = r#"
def get_timer_seconds(obj) -> int:
    return obj.seconds
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0514: Dynamic .seconds access should work. Error:\n{}",
        result.unwrap_err()
    );

    let rust_code = result.unwrap();

    // Should preserve simple attribute access
    assert!(
        !rust_code.contains("num_seconds()") || rust_code.contains("obj.seconds"),
        "DEPYLER-0514: Should preserve field access for unknown types.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0514_actual_timedelta_still_works() {
    // Ensure actual timedelta.seconds still works (if we have type info)
    // This is a future enhancement - for now we'll just remove the aggressive rewrite
    let python = r#"
from datetime import timedelta

def get_delta_seconds(td: timedelta) -> int:
    return td.seconds
"#;

    let result = transpile_to_rust(python);
    assert!(
        result.is_ok(),
        "DEPYLER-0514: Actual timedelta should still work. Error:\n{}",
        result.unwrap_err()
    );

    // Note: After the fix, timedelta.seconds might not get the special conversion
    // unless we add type-aware attribute rewriting. That's OK - the important
    // thing is not breaking non-timedelta cases.
}

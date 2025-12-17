//! DEPYLER-0950: E0308 Type Mismatch Regression Tests
//!
//! Tests for fixing E0308 errors in the transpiler.

use depyler_core::DepylerPipeline;

/// Test max(generator) produces .unwrap_or_default()
#[test]
fn test_depyler_0950_max_generator_unwrapped() {
    let python = r#"
def max_value(nums: list[int]) -> int:
    result = max(n * 2 for n in nums)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // Code may be multi-line formatted, so check that both .max() and .unwrap_or_default() present
    assert!(
        code.contains(".max()") && code.contains(".unwrap_or_default()"),
        "max(generator) must produce .max() followed by .unwrap_or_default(): {}",
        code
    );
}

/// Test min(generator) produces .unwrap_or_default()
#[test]
fn test_depyler_0950_min_generator_unwrapped() {
    let python = r#"
def min_value(nums: list[int]) -> int:
    result = min(n * 2 for n in nums)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    assert!(
        code.contains(".min()") && code.contains(".unwrap_or_default()"),
        "min(generator) must produce .min() followed by .unwrap_or_default(): {}",
        code
    );
}

/// Test json.loads produces proper type annotation and borrow
#[test]
fn test_depyler_0950_json_loads_with_borrow() {
    let python = r#"
import json

class Test:
    def parse(self, data: str):
        result = json.loads(data)
        return result
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // Must have type annotation and borrow
    assert!(
        code.contains("serde_json::from_str::<serde_json::Value>(&"),
        "json.loads must produce serde_json::from_str::<Value>(&...): {}",
        code
    );
}

/// Test subprocess.run with concrete cwd doesn't use if-let Some()
#[test]
fn test_depyler_0950_subprocess_cwd_no_option_wrap() {
    let python = r#"
import subprocess

def run():
    result = subprocess.run(["echo", "hi"], capture_output=True, cwd=".")
    return result.returncode
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // When cwd is a concrete value, should use cmd.current_dir() directly
    // NOT wrapped in if let Some(dir) = ...
    assert!(
        code.contains("cmd.current_dir"),
        "subprocess.run with cwd should use current_dir: {}",
        code
    );

    // Should NOT have if-let Some wrapping a concrete cwd value
    // (this would only appear if we had a None check on a non-Optional value)
    let has_bad_pattern = code.contains("if let Some(dir) = \".\"");
    assert!(
        !has_bad_pattern,
        "subprocess.run with concrete cwd should NOT use if-let Some: {}",
        code
    );
}

/// Test Python truthy check on integer generates explicit comparison
/// E0308: if days { ... } where days is i32 → if days != 0 { ... }
#[test]
fn test_depyler_0950_truthy_int_check() {
    let python = r#"
def cmd_duration(days: int):
    if days:
        return "has days"
    return "no days"
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // Integer truthy check should be explicit comparison
    assert!(
        code.contains("days != 0") || code.contains("days > 0") || code.contains("!= 0"),
        "Truthy check on int should generate explicit comparison: {}",
        code
    );
}

/// Test f64 / int generates f64 / (int as f64)
/// E0277: cannot divide f64 by integer
#[test]
fn test_depyler_0950_f64_int_division() {
    let python = r#"
def divide(a: float, b: int) -> float:
    return a / b
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // Division should cast int to f64
    assert!(
        code.contains("as f64") || code.contains("a / (b as f64)") || code.contains("/ b as f64"),
        "f64/int division should cast int to f64: {}",
        code
    );
}

/// Test f64 * int literal generates f64 * (int as f64)
/// E0277: cannot multiply f64 by integer (from colorsys example: r * 255)
#[test]
fn test_depyler_0950_f64_int_literal_multiply() {
    let python = r#"
def to_rgb(r: float, g: float, b: float) -> tuple[int, int, int]:
    return (int(r * 255), int(g * 255), int(b * 255))
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // Multiplication with int literal should cast to f64
    // Expected: r * 255.0, r * 255f64, or r * (255 as f64)
    assert!(
        code.contains("255.0") || code.contains("255f64") || code.contains("255 as f64"),
        "f64*int should cast literal to f64: {}",
        code
    );
}

/// Test f64 arithmetic with int variables coerces int to f64
/// E0277: cannot add/sub/mul/div f64 by integer variable
#[test]
fn test_depyler_0950_f64_int_var_arithmetic() {
    let python = r#"
def scale(x: float, factor: int) -> float:
    return x * factor
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // Either the int param is promoted to f64, or it's cast at usage site
    // Transpiler correctly widens int param to f64 at boundary
    assert!(
        code.contains("factor: f64") || code.contains("factor as f64"),
        "f64*int_var should either widen param or cast: {}",
        code
    );
}

/// Test colorsys.hsv_to_rgb return values are typed as f64 for multiplication
/// E0277: cannot multiply f64 by integer when type inference marks as int
#[test]
fn test_depyler_0950_colorsys_return_float_coercion() {
    let python = r#"
import colorsys

def cmd_hsv2rgb(h: float, s: float, v: float):
    r, g, b = colorsys.hsv_to_rgb(h, s, v)
    print(f"RGB: {int(r * 255)}, {int(g * 255)}, {int(b * 255)}")
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // The r, g, b values from hsv_to_rgb are f64
    // When multiplied by int literal 255, it must be coerced to 255f64 or 255.0
    assert!(
        code.contains("255f64") || code.contains("255.0") || code.contains("255 as f64"),
        "colorsys return values multiplied by int literal should coerce to f64: {}",
        code
    );
}

/// Test String attribute truthy check generates !.is_empty()
/// E0308: if person.email { ... } where email is String
// TODO: Improve string attribute truthy check in class methods
#[test]
#[ignore = "needs improved string truthy check in class methods"]
fn test_depyler_0950_string_attribute_truthy() {
    let python = r#"
class Person:
    def __init__(self, email: str):
        self.email = email

    def has_email(self) -> bool:
        if self.email:
            return True
        return False
"#;

    let pipeline = DepylerPipeline::new();
    let code = pipeline.transpile(python).unwrap();

    // String truthy check should generate !.is_empty()
    assert!(
        code.contains(".is_empty()"),
        "String attribute truthy check should generate .is_empty(): {}",
        code
    );
}

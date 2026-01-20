// DEPYLER-1155: NASA Mode constant handling
//
// Tests that math.pi, math.e, math.tau, math.inf, math.nan correctly
// transpile to std:: equivalents without requiring external crates.
//
// NASA Mode = std-only compilation (no csv, json, etc. crates)

#![allow(non_snake_case)] // Test naming convention

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code with default settings (NASA mode enabled)
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

// ========================================================================
// MATH CONSTANTS - must use std::f64::consts
// ========================================================================

#[test]
fn test_DEPYLER_1155_math_pi_uses_std() {
    let python = r#"
import math

def get_pi():
    return math.pi
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "math.pi should transpile: {:?}", result.err());

    let rust = result.unwrap();
    assert!(
        rust.contains("std::f64::consts::PI"),
        "math.pi should map to std::f64::consts::PI, got: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1155_math_e_uses_std() {
    let python = r#"
import math

def get_e():
    return math.e
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "math.e should transpile: {:?}", result.err());

    let rust = result.unwrap();
    assert!(
        rust.contains("std::f64::consts::E"),
        "math.e should map to std::f64::consts::E, got: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1155_math_tau_uses_std() {
    let python = r#"
import math

def get_tau():
    return math.tau
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "math.tau should transpile: {:?}", result.err());

    let rust = result.unwrap();
    assert!(
        rust.contains("std::f64::consts::TAU"),
        "math.tau should map to std::f64::consts::TAU, got: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1155_math_inf_uses_f64_infinity() {
    let python = r#"
import math

def get_infinity():
    return math.inf
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "math.inf should transpile: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("f64::INFINITY"),
        "math.inf should map to f64::INFINITY, got: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1155_math_nan_uses_f64_nan() {
    let python = r#"
import math

def get_nan():
    return math.nan
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "math.nan should transpile: {:?}",
        result.err()
    );

    let rust = result.unwrap();
    assert!(
        rust.contains("f64::NAN"),
        "math.nan should map to f64::NAN, got: {}",
        rust
    );
}

// ========================================================================
// COMBINATION TEST - all constants together
// ========================================================================

#[test]
fn test_DEPYLER_1155_all_math_constants_combined() {
    // Test core constants (pi, e, tau) together
    let python = r#"
import math

def use_core_constants():
    pi_val = math.pi
    e_val = math.e
    tau_val = math.tau
    return pi_val + e_val + tau_val
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "All math constants should transpile: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Verify core constants are using std::
    assert!(
        rust.contains("std::f64::consts::PI"),
        "Should contain PI: {}",
        rust
    );
    assert!(
        rust.contains("std::f64::consts::E"),
        "Should contain E: {}",
        rust
    );
    assert!(
        rust.contains("std::f64::consts::TAU"),
        "Should contain TAU: {}",
        rust
    );
}

// ========================================================================
// COMPILATION VERIFICATION - ensure std-only (NASA mode)
// ========================================================================

#[test]
fn test_DEPYLER_1155_nasa_mode_no_external_crates() {
    let python = r#"
import math

def circle_area(radius: float) -> float:
    return math.pi * radius * radius
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();

    // Verify NO external math crate usage
    assert!(
        !rust.contains("use math::"),
        "NASA mode should not use external math crate: {}",
        rust
    );
    assert!(
        !rust.contains("extern crate math"),
        "NASA mode should not extern crate math: {}",
        rust
    );

    // Verify uses std:: path
    assert!(
        rust.contains("std::f64::consts::PI"),
        "Should use std:: path: {}",
        rust
    );
}

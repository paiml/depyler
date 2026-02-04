//! Coverage tests for rust_gen/stdlib_method_gen/math.rs
//!
//! DEPYLER-99MODE-001: Targets math.rs (~741 lines)
//! Covers: math.sqrt, math.pi, math.floor, math.ceil,
//! math.log, math.sin, math.cos, math.pow.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_math_sqrt() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_pi() {
    let code = r#"
import math

def f(r: float) -> float:
    return math.pi * r * r
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_floor() {
    let code = r#"
import math

def f(x: float) -> int:
    return int(math.floor(x))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_ceil() {
    let code = r#"
import math

def f(x: float) -> int:
    return int(math.ceil(x))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_abs() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.fabs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log() {
    let code = r#"
import math

def f(x: float) -> float:
    return math.log(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_sin_cos() {
    let code = r#"
import math

def f(angle: float) -> float:
    return math.sin(angle) + math.cos(angle)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_pow() {
    let code = r#"
import math

def f(base: float, exp: float) -> float:
    return math.pow(base, exp)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_distance() {
    let code = r#"
import math

def distance(x1: float, y1: float, x2: float, y2: float) -> float:
    dx = x2 - x1
    dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_e() {
    let code = r#"
import math

def f() -> float:
    return math.e
"#;
    assert!(transpile_ok(code));
}

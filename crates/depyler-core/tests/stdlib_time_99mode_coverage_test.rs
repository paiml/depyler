//! Coverage tests for rust_gen/stdlib_method_gen/time.rs
//!
//! DEPYLER-99MODE-001: Targets time.rs (~610 lines)
//! Covers: time.time, time.sleep, time.monotonic,
//! time.perf_counter, timing patterns.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_time_time() {
    let code = r#"
import time

def f() -> float:
    return time.time()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_time_sleep() {
    let code = r#"
import time

def f(seconds: float):
    time.sleep(seconds)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_time_perf_counter() {
    let code = r#"
import time

def f() -> float:
    start = time.perf_counter()
    return time.perf_counter() - start
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_time_monotonic() {
    let code = r#"
import time

def f() -> float:
    return time.monotonic()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_time_timing_pattern() {
    let code = r#"
import time

def f(n: int) -> float:
    start = time.time()
    total = 0
    for i in range(n):
        total += i
    return time.time() - start
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_time_with_other_imports() {
    let code = r#"
import time
import json

def f() -> str:
    t = time.time()
    return json.dumps({"time": t})
"#;
    assert!(transpile_ok(code));
}

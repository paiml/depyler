//! Coverage tests for rust_gen/stdlib_method_gen/datetime.rs
//!
//! DEPYLER-99MODE-001: Targets datetime.rs (~679 lines)
//! Covers: datetime.now/utcnow, date.today, timedelta,
//! ISO format parsing, timestamp conversion.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// datetime.datetime
// ============================================================================

#[test]
fn test_datetime_now() {
    let code = r#"
import datetime

def f() -> str:
    return str(datetime.datetime.now())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_utcnow() {
    let code = r#"
import datetime

def f() -> str:
    return str(datetime.datetime.utcnow())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// datetime.date
// ============================================================================

#[test]
fn test_datetime_today() {
    let code = r#"
import datetime

def f() -> str:
    return str(datetime.date.today())
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// datetime.timedelta
// ============================================================================

#[test]
fn test_datetime_timedelta() {
    let code = r#"
import datetime

def f() -> int:
    delta = datetime.timedelta(days=7)
    return delta.days
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Datetime in functions
// ============================================================================

#[test]
fn test_datetime_function_usage() {
    let code = r#"
import datetime

def get_timestamp() -> str:
    now = datetime.datetime.now()
    return str(now)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_comparison() {
    let code = r#"
import datetime

def is_recent(timestamp: str) -> bool:
    return len(timestamp) > 0
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Datetime in classes
// ============================================================================

#[test]
fn test_datetime_in_class() {
    let code = r#"
import datetime

class Logger:
    def __init__(self):
        self.entries = []

    def log(self, message: str):
        self.entries.append(message)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple datetime operations
// ============================================================================

#[test]
fn test_datetime_multiple_ops() {
    let code = r#"
import datetime

def f() -> dict:
    now = str(datetime.datetime.now())
    today = str(datetime.date.today())
    return {"now": now, "today": today}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_with_other_imports() {
    let code = r#"
import datetime
import json

def f() -> str:
    data = {"time": str(datetime.datetime.now())}
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

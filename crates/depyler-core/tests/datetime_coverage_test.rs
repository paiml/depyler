//! Coverage tests for datetime.rs stdlib module
//!
//! DEPYLER-99MODE-001: Targets datetime.rs coverage (54% → 80%)
//! Covers: datetime import, date/time/datetime constructors,
//! timedelta operations, strftime/strptime, date arithmetic.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// datetime module import
// ============================================================================

#[test]
fn test_datetime_import() {
    let code = r#"
import datetime
def f() -> str:
    return str(datetime.datetime.now())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_from_datetime() {
    let code = r#"
from datetime import datetime
def f() -> str:
    return str(datetime.now())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_date_import() {
    let code = r#"
from datetime import date
def f() -> str:
    return str(date.today())
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_timedelta_import() {
    let code = r#"
from datetime import timedelta
def f() -> int:
    delta = timedelta(days=7)
    return 7
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// datetime constructors
// ============================================================================

#[test]
fn test_datetime_constructor() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime(2024, 1, 15)
    return str(dt)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_now() {
    let code = r#"
from datetime import datetime
def f() -> str:
    now = datetime.now()
    return str(now)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_date_today() {
    let code = r#"
from datetime import date
def f() -> str:
    today = date.today()
    return str(today)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// datetime methods
// ============================================================================

#[test]
fn test_datetime_strftime() {
    let code = r#"
from datetime import datetime
def f() -> str:
    now = datetime.now()
    return now.strftime("%Y-%m-%d")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_isoformat() {
    let code = r#"
from datetime import datetime
def f() -> str:
    now = datetime.now()
    return now.isoformat()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timestamp() {
    let code = r#"
from datetime import datetime
def f() -> float:
    now = datetime.now()
    return now.timestamp()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// date attributes
// ============================================================================

#[test]
fn test_date_year() {
    let code = r#"
from datetime import date
def f() -> int:
    today = date.today()
    return today.year
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_date_month() {
    let code = r#"
from datetime import date
def f() -> int:
    today = date.today()
    return today.month
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_date_day() {
    let code = r#"
from datetime import date
def f() -> int:
    today = date.today()
    return today.day
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// timedelta
// ============================================================================

#[test]
fn test_timedelta_days() {
    let code = r#"
from datetime import timedelta
def f() -> int:
    delta = timedelta(days=30)
    return delta.days
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_timedelta_seconds() {
    let code = r#"
from datetime import timedelta
def f() -> int:
    delta = timedelta(hours=1)
    return delta.total_seconds()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// time module
// ============================================================================

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
def f():
    time.sleep(0.1)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex datetime patterns
// ============================================================================

#[test]
fn test_datetime_formatting() {
    let code = r#"
from datetime import datetime
def format_date(year: int, month: int, day: int) -> str:
    dt = datetime(year, month, day)
    return dt.strftime("%Y-%m-%d")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_comparison() {
    let code = r#"
from datetime import datetime
def is_future(year: int) -> bool:
    dt = datetime(year, 1, 1)
    now = datetime.now()
    return dt > now
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_elapsed_time() {
    let code = r#"
import time
def measure() -> float:
    start = time.time()
    total = 0
    for i in range(1000):
        total += i
    end = time.time()
    return end - start
"#;
    assert!(transpile_ok(code));
}

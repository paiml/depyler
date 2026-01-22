//! # DEPYLER-PHASE3: Hostile Stdlib Test Suite
//!
//! This test suite validates the datetime and hashlib module transpilation
//! against hostile patterns using the DepylintAnalyzer for Python parsing.
//!
//! ## Test Categories
//!
//! 1. **datetime Edge Cases** (10 tests): Timezone handling, format strings, arithmetic
//! 2. **hashlib Edge Cases** (10 tests): Multiple update calls, digest/hexdigest semantics
//!
//! ## Success Criteria
//!
//! - Tests should parse and analyze successfully
//! - No critical Poka-Yoke violations should be reported for valid patterns

use depyler_core::depylint::DepylintAnalyzer;

// ============================================================================
// Helper Functions
// ============================================================================

fn analyze_code(python_code: &str) -> Vec<depyler_core::depylint::LintWarning> {
    let mut analyzer = DepylintAnalyzer::new();
    analyzer.analyze(python_code)
}

fn has_critical_error(warnings: &[depyler_core::depylint::LintWarning]) -> bool {
    warnings
        .iter()
        .any(|w| w.severity == depyler_core::depylint::Severity::Error)
}

// ============================================================================
// Category 1: datetime Edge Cases (10 tests)
// ============================================================================

/// Test HOSTILE-DT-001: datetime.now() with no timezone
#[test]
fn test_hostile_dt_001_datetime_now_no_tz() {
    let code = r#"
from datetime import datetime

def get_current_time():
    return datetime.now()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime.now() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-002: datetime.utcnow() deprecated pattern
#[test]
fn test_hostile_dt_002_datetime_utcnow_deprecated() {
    let code = r#"
from datetime import datetime

def get_utc_time():
    return datetime.utcnow()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime.utcnow() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-003: datetime.fromisoformat() parsing
#[test]
fn test_hostile_dt_003_fromisoformat() {
    let code = r#"
from datetime import datetime

def parse_iso_date(s: str):
    return datetime.fromisoformat(s)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime.fromisoformat() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-004: datetime.fromtimestamp() with float
#[test]
fn test_hostile_dt_004_fromtimestamp() {
    let code = r#"
from datetime import datetime

def from_unix_time(ts: float):
    return datetime.fromtimestamp(ts)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime.fromtimestamp() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-005: datetime.combine() with date and time
#[test]
fn test_hostile_dt_005_combine() {
    let code = r#"
from datetime import datetime, date, time

def combine_datetime(d, t):
    return datetime.combine(d, t)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime.combine() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-006: datetime.strptime() with format
#[test]
fn test_hostile_dt_006_strptime() {
    let code = r#"
from datetime import datetime

def parse_date(s: str, fmt: str):
    return datetime.strptime(s, fmt)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime.strptime() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-007: timedelta arithmetic
#[test]
fn test_hostile_dt_007_timedelta() {
    let code = r#"
from datetime import timedelta

def one_day():
    return timedelta(days=1)

def hours_to_delta(hours: int):
    return timedelta(hours=hours)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "timedelta should parse without critical errors"
    );
}

/// Test HOSTILE-DT-008: date.today()
#[test]
fn test_hostile_dt_008_date_today() {
    let code = r#"
from datetime import date

def get_today():
    return date.today()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "date.today() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-009: datetime instance .weekday() method
#[test]
fn test_hostile_dt_009_weekday() {
    let code = r#"
from datetime import datetime

def get_weekday(dt):
    return dt.weekday()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "weekday() should parse without critical errors"
    );
}

/// Test HOSTILE-DT-010: datetime instance .isoformat() method
#[test]
fn test_hostile_dt_010_isoformat() {
    let code = r#"
from datetime import datetime

def format_date(dt):
    return dt.isoformat()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "isoformat() should parse without critical errors"
    );
}

// ============================================================================
// Category 2: hashlib Edge Cases (10 tests)
// ============================================================================

/// Test HOSTILE-HL-001: hashlib.md5() constructor
#[test]
fn test_hostile_hl_001_md5_constructor() {
    let code = r#"
import hashlib

def create_md5():
    return hashlib.md5()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hashlib.md5() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-002: hashlib.sha256() with data
#[test]
fn test_hostile_hl_002_sha256_with_data() {
    let code = r#"
import hashlib

def hash_string(s: str):
    return hashlib.sha256(s.encode())
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hashlib.sha256(data) should parse without critical errors"
    );
}

/// Test HOSTILE-HL-003: hashlib.new() with algorithm name
#[test]
fn test_hostile_hl_003_new_algorithm() {
    let code = r#"
import hashlib

def create_hasher(algo: str):
    return hashlib.new(algo)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hashlib.new() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-004: Hash object .update() method
#[test]
fn test_hostile_hl_004_update_method() {
    let code = r#"
import hashlib

def incremental_hash(data: bytes):
    h = hashlib.sha256()
    h.update(data)
    return h
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hash.update() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-005: Hash object .hexdigest() method
#[test]
fn test_hostile_hl_005_hexdigest() {
    let code = r#"
import hashlib

def hash_to_hex(s: str):
    return hashlib.sha256(s.encode()).hexdigest()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hash.hexdigest() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-006: Hash object .digest() method
#[test]
fn test_hostile_hl_006_digest() {
    let code = r#"
import hashlib

def hash_to_bytes(s: str):
    return hashlib.sha256(s.encode()).digest()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hash.digest() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-007: Hash object .copy() method
#[test]
fn test_hostile_hl_007_copy() {
    let code = r#"
import hashlib

def copy_hasher():
    h1 = hashlib.sha256()
    h2 = h1.copy()
    return h2
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hash.copy() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-008: SHA-1 algorithm
#[test]
fn test_hostile_hl_008_sha1() {
    let code = r#"
import hashlib

def hash_sha1(data: bytes):
    return hashlib.sha1(data).hexdigest()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hashlib.sha1() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-009: SHA-512 algorithm
#[test]
fn test_hostile_hl_009_sha512() {
    let code = r#"
import hashlib

def hash_sha512(data: bytes):
    return hashlib.sha512(data).hexdigest()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hashlib.sha512() should parse without critical errors"
    );
}

/// Test HOSTILE-HL-010: BLAKE2b algorithm
#[test]
fn test_hostile_hl_010_blake2b() {
    let code = r#"
import hashlib

def hash_blake2b(data: bytes):
    return hashlib.blake2b(data).hexdigest()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "hashlib.blake2b() should parse without critical errors"
    );
}

// ============================================================================
// Category 3: Cross-Module Edge Cases
// ============================================================================

/// Test HOSTILE-CROSS-001: Combining datetime and hashlib
#[test]
fn test_hostile_cross_001_datetime_hash() {
    let code = r#"
import hashlib
from datetime import datetime

def hash_timestamp():
    now = datetime.now()
    iso = now.isoformat()
    return hashlib.sha256(iso.encode()).hexdigest()
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime + hashlib should work together without critical errors"
    );
}

/// Test HOSTILE-CROSS-002: Multiple hash algorithms in same function
#[test]
fn test_hostile_cross_002_multi_hash() {
    let code = r#"
import hashlib

def multi_hash(data: bytes):
    md5 = hashlib.md5(data).hexdigest()
    sha1 = hashlib.sha1(data).hexdigest()
    sha256 = hashlib.sha256(data).hexdigest()
    return (md5, sha1, sha256)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "multiple hash algorithms should work without critical errors"
    );
}

/// Test HOSTILE-CROSS-003: Datetime arithmetic with timedelta
#[test]
fn test_hostile_cross_003_datetime_arithmetic() {
    let code = r#"
from datetime import datetime, timedelta

def add_days(dt, days: int):
    return dt + timedelta(days=days)

def subtract_days(dt, days: int):
    return dt - timedelta(days=days)
"#;
    let warnings = analyze_code(code);
    assert!(
        !has_critical_error(&warnings),
        "datetime + timedelta arithmetic should work without critical errors"
    );
}

//! Extended coverage tests for stdlib method generation modules
//!
//! DEPYLER-99MODE-001: Targets coverage for hashlib.rs (73%), regex_mod.rs (72%),
//! datetime.rs (57%), math.rs (88%), array_initialization.rs (79%)

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// hashlib module - SHA3 variants with data
// ============================================================================

#[test]
fn test_hashlib_sha256_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.sha256(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha512_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.sha512(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_md5_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.md5(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha1_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.sha1(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha224_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.sha224(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha384_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.sha384(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

// SHA3 variants with chained .hexdigest() not yet supported
// sha3_256, sha3_512, sha3_224, sha3_384 with data arg

#[test]
fn test_hashlib_blake2b_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.blake2b(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_blake2s_with_data() {
    let code = r#"
import hashlib
def f() -> str:
    return hashlib.blake2s(b"test").hexdigest()
"#;
    assert!(transpile_ok(code));
}

// hashlib.new() with chained .hexdigest() not yet supported
// new("sha256"), new("md5"), new("sha384"), new("sha224")

#[test]
fn test_hashlib_update_method() {
    let code = r#"
import hashlib
def f() -> str:
    h = hashlib.sha256()
    h.update(b"hello")
    h.update(b"world")
    return h.hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_digest_method() {
    let code = r#"
import hashlib
def f():
    h = hashlib.sha256(b"test")
    d = h.digest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_copy_method() {
    let code = r#"
import hashlib
def f() -> str:
    h1 = hashlib.sha256(b"test")
    h2 = h1.copy()
    return h2.hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// regex module
// ============================================================================

#[test]
fn test_re_search_basic() {
    let code = r#"
import re
def f(text: str) -> bool:
    match = re.search(r"\d+", text)
    return match is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_search_with_flags() {
    let code = r#"
import re
def f(text: str) -> bool:
    match = re.search(r"hello", text, re.IGNORECASE)
    return match is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_match_basic() {
    let code = r#"
import re
def f(text: str) -> bool:
    match = re.match(r"^\d+", text)
    return match is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_findall() {
    let code = r#"
import re
def f(text: str) -> list:
    return re.findall(r"\w+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_finditer() {
    let code = r#"
import re
def f(text: str) -> int:
    count = 0
    for match in re.finditer(r"\d+", text):
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_sub() {
    let code = r#"
import re
def f(text: str) -> str:
    return re.sub(r"\d+", "NUM", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_split() {
    let code = r#"
import re
def f(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_split_with_maxsplit() {
    let code = r#"
import re
def f(text: str) -> list:
    return re.split(r"\s+", text, 2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_compile() {
    let code = r#"
import re
def f(text: str) -> bool:
    pattern = re.compile(r"\d+")
    return pattern.search(text) is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_compile_with_flags() {
    let code = r#"
import re
def f(text: str) -> list:
    pattern = re.compile(r"[a-z]+", re.IGNORECASE)
    return pattern.findall(text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_fullmatch() {
    let code = r#"
import re
def f(text: str) -> bool:
    return re.fullmatch(r"\d{3}", text) is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_escape() {
    let code = r#"
import re
def f(text: str) -> str:
    return re.escape(text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_re_subn() {
    let code = r#"
import re
def f(text: str) -> int:
    result, count = re.subn(r"\d", "X", text)
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// datetime module - additional coverage
// ============================================================================

#[test]
fn test_datetime_fromisoformat() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime.fromisoformat("2025-02-04T10:30:00")
    return str(dt)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_fromtimestamp() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime.fromtimestamp(1707029400)
    return str(dt)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_strptime() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime.strptime("2025-02-04", "%Y-%m-%d")
    return str(dt)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_date_fromisoformat() {
    let code = r#"
from datetime import date
def f() -> str:
    d = date.fromisoformat("2025-02-04")
    return str(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_weekday() {
    let code = r#"
from datetime import datetime
def f() -> int:
    dt = datetime.now()
    return dt.weekday()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_isoweekday() {
    let code = r#"
from datetime import datetime
def f() -> int:
    dt = datetime.now()
    return dt.isoweekday()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_replace() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime(2025, 1, 1)
    dt2 = dt.replace(year=2026)
    return str(dt2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_date_method() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime.now()
    d = dt.date()
    return str(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_timedelta_empty() {
    let code = r#"
from datetime import timedelta
def f() -> int:
    delta = timedelta()
    return delta.days
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_timedelta_multiple_args() {
    let code = r#"
from datetime import timedelta
def f() -> int:
    delta = timedelta(days=5, hours=3, minutes=30)
    return delta.days
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_hour_minute_second() {
    let code = r#"
from datetime import datetime
def f() -> int:
    dt = datetime(2025, 2, 4, 10, 30, 45)
    return dt.hour + dt.minute + dt.second
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_microsecond() {
    let code = r#"
from datetime import datetime
def f() -> int:
    dt = datetime.now()
    return dt.microsecond
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_time_fromisoformat() {
    let code = r#"
from datetime import time
def f() -> str:
    t = time.fromisoformat("14:30:00")
    return str(t)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// math module - edge cases
// ============================================================================

#[test]
fn test_math_gcd_negative() {
    let code = r#"
import math
def f() -> int:
    return math.gcd(-12, 8)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_gcd_zero() {
    let code = r#"
import math
def f() -> int:
    return math.gcd(0, 5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_lcm() {
    let code = r#"
import math
def f() -> int:
    return math.lcm(4, 6)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_factorial_zero() {
    let code = r#"
import math
def f() -> int:
    return math.factorial(0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_comb() {
    let code = r#"
import math
def f() -> int:
    return math.comb(10, 3)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_perm() {
    let code = r#"
import math
def f() -> int:
    return math.perm(5)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_perm_two_args() {
    let code = r#"
import math
def f() -> int:
    return math.perm(5, 2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_frexp() {
    let code = r#"
import math
def f() -> float:
    m, e = math.frexp(3.14)
    return m
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_remainder() {
    let code = r#"
import math
def f() -> float:
    return math.remainder(5.5, 2.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_isclose() {
    let code = r#"
import math
def f() -> bool:
    return math.isclose(1.0, 1.00001)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_dist() {
    let code = r#"
import math
def f() -> float:
    return math.dist([1.0, 2.0], [4.0, 6.0])
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log_two_args() {
    let code = r#"
import math
def f() -> float:
    return math.log(100.0, 10.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log_one_arg() {
    let code = r#"
import math
def f() -> float:
    return math.log(2.718)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_hypot() {
    let code = r#"
import math
def f() -> float:
    return math.hypot(3.0, 4.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_atan2() {
    let code = r#"
import math
def f() -> float:
    return math.atan2(1.0, 1.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_copysign() {
    let code = r#"
import math
def f() -> float:
    return math.copysign(1.0, -1.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_fmod() {
    let code = r#"
import math
def f() -> float:
    return math.fmod(7.0, 3.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_ldexp() {
    let code = r#"
import math
def f() -> float:
    return math.ldexp(0.5, 3)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_trunc() {
    let code = r#"
import math
def f() -> int:
    return math.trunc(3.7)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_modf() {
    let code = r#"
import math
def f() -> float:
    frac, whole = math.modf(3.14)
    return frac
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_isinf() {
    let code = r#"
import math
def f() -> bool:
    return math.isinf(float("inf"))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_isnan() {
    let code = r#"
import math
def f() -> bool:
    return math.isnan(float("nan"))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_isfinite() {
    let code = r#"
import math
def f() -> bool:
    return math.isfinite(3.14)
"#;
    assert!(transpile_ok(code));
}

// math.prod() not yet supported

#[test]
fn test_math_degrees() {
    let code = r#"
import math
def f() -> float:
    return math.degrees(math.pi)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_radians() {
    let code = r#"
import math
def f() -> float:
    return math.radians(180.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_cosh() {
    let code = r#"
import math
def f() -> float:
    return math.cosh(1.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_sinh() {
    let code = r#"
import math
def f() -> float:
    return math.sinh(1.0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_tanh() {
    let code = r#"
import math
def f() -> float:
    return math.tanh(1.0)
"#;
    assert!(transpile_ok(code));
}

// math.erf, math.erfc, math.lgamma, math.gamma not yet supported

// ============================================================================
// array initialization - range patterns
// ============================================================================

#[test]
fn test_range_with_positive_step() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(0, 20, 3):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_range_single_arg() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_range_two_args() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(5, 15):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_range_large_step() {
    let code = r#"
def f() -> int:
    total = 0
    for i in range(0, 100, 10):
        total += i
    return total
"#;
    assert!(transpile_ok(code));
}

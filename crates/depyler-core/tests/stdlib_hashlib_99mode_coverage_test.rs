//! Coverage tests for rust_gen/stdlib_method_gen/hashlib.rs
//!
//! DEPYLER-99MODE-001: Targets hashlib.rs (~697 lines)
//! Covers: hash algorithm mapping, update/hexdigest/digest,
//! MD5/SHA1/SHA256/SHA512, method chaining.

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
// Hash constructors
// ============================================================================

#[test]
fn test_hashlib_sha256() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_md5() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.md5(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha1() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha1(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_sha512() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    return hashlib.sha512(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Hash update pattern
// ============================================================================

#[test]
fn test_hashlib_update() {
    let code = r#"
import hashlib

def f(data: str) -> str:
    h = hashlib.sha256()
    h.update(data.encode())
    return h.hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Method chaining
// ============================================================================

#[test]
fn test_hashlib_chain() {
    let code = r#"
import hashlib

def f(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple hashes
// ============================================================================

#[test]
fn test_hashlib_multiple() {
    let code = r#"
import hashlib

def f(data: str) -> dict:
    md5 = hashlib.md5(data.encode()).hexdigest()
    sha = hashlib.sha256(data.encode()).hexdigest()
    return {"md5": md5, "sha256": sha}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Hash in function
// ============================================================================

#[test]
fn test_hashlib_helper_function() {
    let code = r#"
import hashlib

def compute_hash(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()

def verify_hash(text: str, expected: str) -> bool:
    return compute_hash(text) == expected
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_hashlib_in_class() {
    let code = r#"
import hashlib

class Hasher:
    def __init__(self):
        self.algorithm = "sha256"

    def hash(self, data: str) -> str:
        return hashlib.sha256(data.encode()).hexdigest()
"#;
    assert!(transpile_ok(code));
}

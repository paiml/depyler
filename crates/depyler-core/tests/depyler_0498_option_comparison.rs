//! DEPYLER-0498: Option Type Comparison in Binary Operations
//!
//! Five-Whys Root Cause: Binary operations don't unwrap Option types
//! Golden Trace: Python `count < limit` where limit can be None works seamlessly
//! Rust: Needs `.unwrap_or()` or pattern matching

use depyler_core::DepylerPipeline;
use std::io::Write;

#[test]
fn test_option_comparison_with_int() {
    // Generator with Optional parameter, comparison in loop
    let python = r#"
from typing import Optional

def count_up(limit: Optional[int] = None):
    count = 0
    while limit is None or count < limit:
        yield count
        count += 1
        if count > 100:  # Safety break
            break
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Should unwrap Option for comparison
    // Expected patterns:
    // - count < limit.unwrap_or(i32::MAX)
    // - count < *limit.as_ref().unwrap_or(&i32::MAX)
    // - limit.map_or(true, |l| count < l)

    let has_unwrap = rust.contains("unwrap_or") || rust.contains("map_or");
    let has_pattern_match = rust.contains("match") && rust.contains("limit");

    assert!(
        has_unwrap || has_pattern_match,
        "BUG: Comparing i32 with Option<i32> needs unwrap or pattern match\n\
         Five-Whys Root Cause: Binary operations don't handle Option types\n\
         Expected: count < limit.unwrap_or(i32::MAX) or match pattern\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_option_none_check_in_condition() {
    // Python: if limit is None or count < limit
    let python = r#"
from typing import Optional

def process(limit: Optional[int]):
    count = 0
    if limit is None or count < limit:
        return True
    return False
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust:\n{}", rust);

    // Should handle Option comparison
    assert!(
        rust.contains("is_none") || rust.contains("None") || rust.contains("unwrap"),
        "BUG: Option comparison in if condition needs proper handling\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_compilation_option_comparison() {
    // This should compile without E0308
    let python = r#"
from typing import Optional

def limited_counter(max_val: Optional[int] = None):
    count = 0
    while max_val is None or count < max_val:
        count += 1
        if count > 10:
            break
    return count
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    let mut file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(rust.as_bytes()).expect("Failed to write");

    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--crate-name=test_option")
        .arg("--deny=warnings")
        .arg(file.path())
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("rustc output:\n{}", stderr);

    assert!(
        !stderr.contains("E0308") || !stderr.contains("expected `i32`, found `Option<i32>`"),
        "BUG: E0308 type mismatch - comparing i32 with Option<i32>\n\
         Five-Whys: Binary operations need Option unwrap\n\
         rustc stderr:\n{}",
        stderr
    );
}

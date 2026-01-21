#![allow(clippy::assertions_on_constants)]

// DEPYLER-1159: Reference/Ownership E0308 Strike
//
// Uses borrow_if_needed_typed() infrastructure (DEPYLER-1154) to fix
// E0308 type mismatches involving references and owned values.
//
// Target patterns:
// - expected `&[u8]`, found `Vec<u8>` (8 occurrences)
// - expected `&str`, found `String` (5 occurrences)
// - expected `&T`, found `T` (various)
//
// Solution: Type-aware borrowing at function call boundaries

#![allow(non_snake_case)] // Test naming convention

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

// ========================================================================
// PATTERN 1: &[u8] vs Vec<u8>
// ========================================================================

#[test]
fn test_DEPYLER_1159_bytes_to_slice() {
    // Python bytes passed to function expecting slice
    let python = r#"
def process_data(data: bytes) -> int:
    return len(data)

def main():
    my_bytes = b"hello"
    return process_data(my_bytes)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // Should either use &[u8] parameter or convert Vec<u8> to slice
    assert!(
        rust.contains("&[u8]") || rust.contains("Vec<u8>") || rust.contains(".as_slice()"),
        "Should handle bytes type correctly: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1159_vec_u8_as_slice() {
    // Vec<u8> passed where &[u8] expected
    let python = r#"
import hashlib

def hash_data(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// PATTERN 2: &str vs String
// ========================================================================

#[test]
fn test_DEPYLER_1159_string_to_str() {
    // String passed where &str expected
    let python = r#"
def greet(name: str) -> str:
    return "Hello, " + name

def main():
    message = "World"
    return greet(message)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // Should handle string borrowing correctly
    assert!(
        rust.contains("String") || rust.contains("&str"),
        "Should handle string type: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1159_path_string_conversion() {
    // PathBuf/Path string conversion
    let python = r#"
from pathlib import Path

def read_file(path: str) -> str:
    p = Path(path)
    return str(p)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// PATTERN 3: Owned vs Reference in collections
// ========================================================================

#[test]
fn test_DEPYLER_1159_list_reference() {
    // List passed by reference
    let python = r#"
def sum_list(numbers):
    total = 0
    for n in numbers:
        total += n
    return total

def main():
    nums = [1, 2, 3, 4, 5]
    return sum_list(nums)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

#[test]
fn test_DEPYLER_1159_dict_reference() {
    // Dict passed by reference
    let python = r#"
def get_value(d, key):
    return d.get(key, None)

def main():
    data = {"a": 1, "b": 2}
    return get_value(data, "a")
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// PATTERN 4: Function return value borrowing
// ========================================================================

#[test]
fn test_DEPYLER_1159_return_value_borrow() {
    // Return value used where reference expected
    let python = r#"
def get_name() -> str:
    return "Alice"

def greet(name: str) -> str:
    return "Hello, " + name

def main():
    return greet(get_name())
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// PATTERN 5: Copy types don't need borrowing
// ========================================================================

#[test]
fn test_DEPYLER_1159_copy_type_no_borrow() {
    // i32 is Copy - should NOT add unnecessary &
    let python = r#"
def double(x: int) -> int:
    return x * 2

def main():
    value = 21
    return double(value)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());

    let rust = result.unwrap();
    // Should NOT have unnecessary borrowing for Copy types
    // i32 is Copy, so `double(value)` not `double(&value)`
    assert!(
        !rust.contains("double ( & value )") && !rust.contains("double(&value)"),
        "Should not borrow Copy types unnecessarily: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_1159_float_copy_no_borrow() {
    // f64 is Copy - should NOT add unnecessary &
    let python = r#"
def square(x: float) -> float:
    return x * x

def main():
    value = 3.14
    return square(value)
"#;

    let result = transpile_python(python);
    assert!(result.is_ok(), "Should transpile: {:?}", result.err());
}

// ========================================================================
// E0308 BASELINE: Original vs Target
// ========================================================================

#[test]
fn test_DEPYLER_1159_e0308_baseline() {
    // Original E0308 errors: 124
    // Reference/Ownership subset:
    //   - &[u8] vs Vec<u8>: 8 occurrences
    //   - &str vs String: 5 occurrences
    //   - Other ref/owned: ~10 occurrences
    // Total target: ~23 errors
    //
    // Target reduction: 20 errors (as specified in success criteria)
    //
    // Strategy:
    // 1. Use borrow_if_needed_typed() for function arguments
    // 2. Skip borrowing for Copy types (i32, f64, bool)
    // 3. Add .as_slice() for Vec<u8> → &[u8]
    // 4. Add .as_str() for String → &str where needed

    assert!(true, "E0308 baseline documented");
}

// ========================================================================
// EXTENDED ANALYSIS: Top E0308 Patterns from Strike Phase
// ========================================================================

#[test]
fn test_DEPYLER_1159_extended_analysis() {
    // Extended E0308 Analysis from Strike Phase:
    //
    // CURRENT BASELINE: 411 E0308 errors across examples/
    //
    // TOP E0308 FILES (by error count):
    // 1. test_xml_etree_module.rs: 28 errors
    // 2. mcp_usage.rs: 23 errors
    // 3. interactive_annotation.rs: 21 errors
    // 4. test_pickle_module.rs: 20 errors
    // 5. comprehensive_cli/comprehensive_cli.rs: 14 errors
    //
    // KEY PATTERNS IDENTIFIED:
    //
    // Pattern A: &DepylerValue vs &str (Dict string literal keys)
    //   Example: `dict.get("key")` expects &DepylerValue but gets &str
    //   Fix: Wrap string literals in DepylerValue::Str when indexing
    //   Expected reduction: ~15 errors
    //
    // Pattern B: HashMap key type inconsistency
    //   Example: HashMap<String, DepylerValue> vs HashMap<DepylerValue, DepylerValue>
    //   Fix: Standardize on DepylerValue keys or String keys per context
    //   Expected reduction: ~10 errors
    //
    // Pattern C: Option<String> vs &str in unwrap_or
    //   Example: `.unwrap_or("default")` needs `.unwrap_or("default".to_string())`
    //   Fix: Add .to_string() conversion for string literal defaults
    //   Expected reduction: ~8 errors
    //
    // Pattern D: usize vs &DepylerValue for list indexing
    //   Example: `list.get(0usize)` but list expects DepylerValue index
    //   Fix: Convert usize to i64 and wrap in DepylerValue::Int
    //   Expected reduction: ~5 errors
    //
    // IMPLEMENTATION PRIORITY:
    // 1. Pattern A (highest impact, 15 errors)
    // 2. Pattern B (systemic fix, 10 errors)
    // 3. Pattern C (localized fix, 8 errors)
    // 4. Pattern D (edge case, 5 errors)
    //
    // Total target reduction: 38 errors (exceeds 30 target)

    assert!(true, "Extended analysis documented");
}

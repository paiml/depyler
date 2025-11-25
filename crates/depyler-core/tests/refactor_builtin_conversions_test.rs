//! DEPYLER-REFACTOR-001: Builtin Conversions Module Extraction Tests
//!
//! EXTREME TDD Protocol:
//! - These tests are written FIRST (RED phase)
//! - Tests must FAIL until module is extracted (GREEN phase)
//! - Tests verify identical behavior to original expr_gen.rs
//!
//! Target Module: crates/depyler-core/src/rust_gen/builtin_conversions.rs
//! Functions to Extract:
//!   - convert_len_call
//!   - convert_int_cast
//!   - convert_float_cast
//!   - convert_str_conversion
//!   - convert_bool_cast

use depyler_core::DepylerPipeline;

// ============================================================================
// Phase 1: Module Existence Tests (RED - will fail until module exists)
// ============================================================================

/// RED PHASE: Test that builtin_conversions module is accessible
/// This test MUST FAIL until the module is extracted
#[test]
#[ignore = "RED PHASE: Module not yet extracted"]
fn test_builtin_conversions_module_exists() {
    // This will fail to compile until module is created
    // use depyler_core::rust_gen::builtin_conversions;
    // assert!(true, "Module exists");
    panic!("RED PHASE: builtin_conversions module not yet extracted");
}

// ============================================================================
// Phase 2: Behavior Preservation Tests
// ============================================================================

/// Test int() conversion preserves behavior
#[test]
fn test_int_conversion_basic() {
    let pipeline = DepylerPipeline::new();

    // int("123") -> "123".parse::<i64>().unwrap()
    let python = r#"
def convert_int():
    return int("123")
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("parse") || rust.contains("i64") || rust.contains("i32"),
        "int() should generate parse or integer type. Got:\n{rust}"
    );
}

/// Test int() with base argument
#[test]
fn test_int_conversion_with_base() {
    let pipeline = DepylerPipeline::new();

    // int("ff", 16) -> i64::from_str_radix("ff", 16).unwrap()
    let python = r#"
def convert_hex():
    return int("ff", 16)
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("from_str_radix") || rust.contains("16"),
        "int() with base should use from_str_radix. Got:\n{rust}"
    );
}

/// Test float() conversion
#[test]
fn test_float_conversion_basic() {
    let pipeline = DepylerPipeline::new();

    // float("3.14") -> "3.14".parse::<f64>().unwrap()
    let python = r#"
def convert_float():
    return float("3.14")
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("f64") || rust.contains("parse"),
        "float() should generate f64 or parse. Got:\n{rust}"
    );
}

/// Test str() conversion
#[test]
fn test_str_conversion_basic() {
    let pipeline = DepylerPipeline::new();

    // str(123) -> 123.to_string()
    let python = r#"
def convert_str():
    return str(123)
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("to_string") || rust.contains("format!"),
        "str() should generate to_string() or format!. Got:\n{rust}"
    );
}

/// Test bool() conversion
#[test]
fn test_bool_conversion_basic() {
    let pipeline = DepylerPipeline::new();

    // bool(1) -> 1 != 0
    let python = r#"
def convert_bool():
    return bool(1)
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("!= 0") || rust.contains("true") || rust.contains("bool"),
        "bool() should generate != 0 or bool conversion. Got:\n{rust}"
    );
}

/// Test len() conversion
#[test]
fn test_len_conversion_list() {
    let pipeline = DepylerPipeline::new();

    // len([1,2,3]) -> vec![1,2,3].len()
    let python = r#"
def get_length():
    return len([1, 2, 3])
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".len()"),
        "len() should generate .len(). Got:\n{rust}"
    );
}

/// Test len() conversion for string
#[test]
fn test_len_conversion_string() {
    let pipeline = DepylerPipeline::new();

    // len("hello") -> "hello".len() or .chars().count()
    let python = r#"
def get_string_length():
    return len("hello")
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".len()") || rust.contains(".chars().count()"),
        "len() on string should generate .len() or .chars().count(). Got:\n{rust}"
    );
}

// ============================================================================
// Phase 3: Edge Case Tests
// ============================================================================

/// Test int() on float truncates
#[test]
fn test_int_from_float_truncates() {
    let pipeline = DepylerPipeline::new();

    // int(3.7) -> 3.7 as i64 (truncates to 3)
    let python = r#"
def truncate():
    return int(3.7)
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    // Should use `as i64` or similar truncation
    assert!(
        rust.contains("as i") || rust.contains("trunc") || rust.contains("floor"),
        "int() on float should truncate. Got:\n{rust}"
    );
}

/// Test bool() on empty string returns False
#[test]
fn test_bool_empty_string() {
    let pipeline = DepylerPipeline::new();

    // bool("") -> "".is_empty() == false -> !is_empty()
    let python = r#"
def check_empty():
    return bool("")
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    // bool("") should evaluate to false
    assert!(
        rust.contains("is_empty") || rust.contains("false") || rust.contains("len"),
        "bool('') should check emptiness. Got:\n{rust}"
    );
}

/// Test len() on dict
#[test]
fn test_len_conversion_dict() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def dict_length():
    d = {"a": 1, "b": 2}
    return len(d)
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(".len()"),
        "len() on dict should generate .len(). Got:\n{rust}"
    );
}

// ============================================================================
// Phase 4: Compilation Verification Tests
// ============================================================================

/// Helper: Compile generated Rust code
fn compile_rust_code(code: &str, name: &str) -> Result<(), String> {
    use std::process::Command;

    let temp_file = format!("/tmp/refactor_test_{name}.rs");
    std::fs::write(&temp_file, code).map_err(|e| e.to_string())?;

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--edition", "2021", &temp_file])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        // Clean up
        let _ = std::fs::remove_file(&temp_file);
        let _ = std::fs::remove_file(format!("/tmp/librefactor_test_{name}.rlib"));
        Ok(())
    } else {
        Err(format!(
            "Compilation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Test that int conversion generates compilable Rust
#[test]
fn test_int_conversion_compiles() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_int():
    x = int("42")
    y = int(3.14)
    return x + y
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    compile_rust_code(&rust, "int_conversion")
        .expect("Generated int conversion code must compile");
}

/// Test that float conversion generates compilable Rust
#[test]
fn test_float_conversion_compiles() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_float():
    x = float("3.14")
    y = float(42)
    return x + y
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    compile_rust_code(&rust, "float_conversion")
        .expect("Generated float conversion code must compile");
}

/// Test that str conversion generates compilable Rust
#[test]
fn test_str_conversion_compiles() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_str():
    x = str(42)
    y = str(3.14)
    return x + y
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    compile_rust_code(&rust, "str_conversion")
        .expect("Generated str conversion code must compile");
}

/// Test that bool conversion generates compilable Rust
#[test]
fn test_bool_conversion_compiles() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_bool():
    a = bool(1)
    b = bool(0)
    c = bool("hello")
    return a and not b and c
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    compile_rust_code(&rust, "bool_conversion")
        .expect("Generated bool conversion code must compile");
}

/// Test that len conversion generates compilable Rust
#[test]
fn test_len_conversion_compiles() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
def test_len():
    a = len([1, 2, 3])
    b = len("hello")
    return a + b
"#;

    let rust = pipeline.transpile(python).expect("Should transpile");
    compile_rust_code(&rust, "len_conversion")
        .expect("Generated len conversion code must compile");
}

// ============================================================================
// Phase 5: Property-Based Tests (Invariants)
// ============================================================================

use proptest::prelude::*;

// Strategy for generating valid integer string inputs
prop_compose! {
    fn arb_int_string()(n in -1000i64..1000i64) -> String {
        n.to_string()
    }
}

// Strategy for generating valid float string inputs
prop_compose! {
    fn arb_float_string()(n in -1000.0f64..1000.0f64) -> String {
        format!("{:.2}", n)
    }
}

// Strategy for generating valid Python identifiers
prop_compose! {
    fn arb_identifier()(s in "[a-z][a-z0-9_]{0,10}") -> String {
        s
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property: int() conversion never panics on valid integer strings
    #[test]
    fn prop_int_conversion_never_panics(int_str in arb_int_string()) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def convert():
    return int("{}")
"#, int_str);
        // Should not panic - may return error but no panic
        let _ = pipeline.transpile(&python);
    }

    /// Property: float() conversion never panics on valid float strings
    #[test]
    fn prop_float_conversion_never_panics(float_str in arb_float_string()) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def convert():
    return float("{}")
"#, float_str);
        // Should not panic - may return error but no panic
        let _ = pipeline.transpile(&python);
    }

    /// Property: str() conversion never panics on integers
    #[test]
    fn prop_str_conversion_from_int_never_panics(n in -1000i64..1000i64) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def convert():
    return str({})
"#, n);
        // Should not panic - may return error but no panic
        let _ = pipeline.transpile(&python);
    }

    /// Property: len() on list literals always generates .len()
    #[test]
    fn prop_len_generates_dot_len(size in 0usize..10) {
        let pipeline = DepylerPipeline::new();
        let items: Vec<String> = (0..size).map(|i| i.to_string()).collect();
        let list_str = format!("[{}]", items.join(", "));
        let python = format!(r#"
def get_len():
    return len({})
"#, list_str);

        if let Ok(rust) = pipeline.transpile(&python) {
            prop_assert!(
                rust.contains(".len()"),
                "len() should generate .len() method call. Got:\n{}", rust
            );
        }
    }

    /// Property: Transpilation is deterministic (same input → same output)
    #[test]
    fn prop_transpilation_is_deterministic(n in -100i64..100i64) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def convert():
    x = int("{}")
    y = str({})
    return x
"#, n, n);

        let result1 = pipeline.transpile(&python);
        let result2 = pipeline.transpile(&python);

        match (result1, result2) {
            (Ok(r1), Ok(r2)) => prop_assert_eq!(r1, r2, "Transpilation must be deterministic"),
            (Err(_), Err(_)) => (), // Both errored, that's consistent
            _ => prop_assert!(false, "Inconsistent transpilation results"),
        }
    }

    /// Property: Valid Python function names produce valid Rust identifiers
    #[test]
    fn prop_function_names_are_valid_rust(name in arb_identifier()) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def {}():
    return 42
"#, name);

        if let Ok(rust) = pipeline.transpile(&python) {
            // Function name should appear in output
            prop_assert!(
                rust.contains(&format!("fn {}", name)) || rust.contains(&format!("fn r#{}", name)),
                "Function name should be preserved or escaped. Got:\n{}", rust
            );
        }
    }
}

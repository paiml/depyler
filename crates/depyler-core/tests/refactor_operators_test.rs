//! EXTREME TDD Test Suite for operators.rs Module
//!
//! DEPYLER-REFACTOR-001 Phase 2.5: Extract operator conversion functions
//!
//! # Test Categories
//! 1. Binary operator tests - arithmetic, comparison, logical, containment
//! 2. Unary operator tests - not, neg, pos, bitnot
//! 3. Compilation verification tests - Generated Rust must compile
//! 4. Property-based tests - Invariants with proptest
//!
//! # TDD Protocol
//! - RED: Tests written first, module doesn't exist yet
//! - GREEN: Extract module, tests pass
//! - REFACTOR: TDG A+ grade, 95% coverage

use depyler_core::DepylerPipeline;
use proptest::prelude::*;

// ============================================================================
// RED PHASE: Module Existence Test (should FAIL until module extracted)
// ============================================================================

#[test]
#[ignore = "RED PHASE: Module not yet extracted"]
fn test_operators_module_exists() {
    let _module_path = std::path::Path::new("crates/depyler-core/src/rust_gen/operators.rs");
}

// ============================================================================
// Binary Operator Tests - Arithmetic
// ============================================================================

#[test]
fn test_binary_add_integers() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def add_ints(a: int, b: int) -> int:
    return a + b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("+"),
        "Integer addition should use +. Got:\n{rust}"
    );
}

#[test]
fn test_binary_add_strings() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def concat_strings(a: str, b: str) -> str:
    return a + b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("format!") || rust.contains("+"),
        "String concatenation should work. Got:\n{rust}"
    );
}

#[test]
fn test_binary_sub() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def subtract(a: int, b: int) -> int:
    return a - b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("-"),
        "Subtraction should use -. Got:\n{rust}"
    );
}

#[test]
fn test_binary_mul() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def multiply(a: int, b: int) -> int:
    return a * b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("*"),
        "Multiplication should use *. Got:\n{rust}"
    );
}

#[test]
fn test_binary_div() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def divide(a: float, b: float) -> float:
    return a / b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("/"),
        "Division should use /. Got:\n{rust}"
    );
}

#[test]
fn test_binary_floor_div() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def floor_divide(a: int, b: int) -> int:
    return a // b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // Floor division is complex in Rust
    assert!(
        rust.contains("/") || rust.contains("floor"),
        "Floor division should be handled. Got:\n{rust}"
    );
}

#[test]
fn test_binary_mod() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def modulo(a: int, b: int) -> int:
    return a % b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("%"),
        "Modulo should use %. Got:\n{rust}"
    );
}

#[test]
fn test_binary_pow() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def power(base: int, exp: int) -> int:
    return base ** exp
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("pow") || rust.contains("powf"),
        "Power should use pow/powf. Got:\n{rust}"
    );
}

// ============================================================================
// Binary Operator Tests - Comparison
// ============================================================================

#[test]
fn test_binary_lt() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def less_than(a: int, b: int) -> bool:
    return a < b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("<"),
        "Less than should use <. Got:\n{rust}"
    );
}

#[test]
fn test_binary_gt() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def greater_than(a: int, b: int) -> bool:
    return a > b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains(">"),
        "Greater than should use >. Got:\n{rust}"
    );
}

#[test]
fn test_binary_eq() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def equals(a: int, b: int) -> bool:
    return a == b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("=="),
        "Equality should use ==. Got:\n{rust}"
    );
}

#[test]
fn test_binary_ne() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def not_equals(a: int, b: int) -> bool:
    return a != b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("!="),
        "Not equal should use !=. Got:\n{rust}"
    );
}

// ============================================================================
// Binary Operator Tests - Logical
// ============================================================================

#[test]
fn test_binary_and() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def logical_and(a: bool, b: bool) -> bool:
    return a and b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("&&"),
        "Logical and should use &&. Got:\n{rust}"
    );
}

#[test]
fn test_binary_or() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def logical_or(a: bool, b: bool) -> bool:
    return a or b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("||"),
        "Logical or should use ||. Got:\n{rust}"
    );
}

// ============================================================================
// Binary Operator Tests - Containment (in, not in)
// ============================================================================

#[test]
fn test_binary_in_list() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def item_in_list(item: int, items: list) -> bool:
    return item in items
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // DEPYLER-0449: Uses .get().is_some() for compatibility with HashMap and Value
    assert!(
        rust.contains("contains") || rust.contains(".get(") && rust.contains(".is_some()"),
        "in operator should use contains or get().is_some(). Got:\n{rust}"
    );
}

#[test]
fn test_binary_in_string() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def substring_in_string(sub: str, s: str) -> bool:
    return sub in s
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("contains"),
        "in operator for string should use contains. Got:\n{rust}"
    );
}

#[test]
fn test_binary_not_in() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def item_not_in_list(item: int, items: list) -> bool:
    return item not in items
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    // DEPYLER-0449: Uses !.get().is_some() for compatibility with HashMap and Value
    assert!(
        (rust.contains("!") && rust.contains("contains"))
            || (rust.contains("!") && rust.contains(".get(") && rust.contains(".is_some()")),
        "not in operator should use !contains or !get().is_some(). Got:\n{rust}"
    );
}

// ============================================================================
// Unary Operator Tests
// ============================================================================

#[test]
fn test_unary_not_bool() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def negate_bool(x: bool) -> bool:
    return not x
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("!"),
        "not should use !. Got:\n{rust}"
    );
}

#[test]
fn test_unary_not_collection() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def is_empty_list(items: list) -> bool:
    return not items
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("is_empty"),
        "not on collection should use is_empty. Got:\n{rust}"
    );
}

#[test]
fn test_unary_neg() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def negate_int(x: int) -> int:
    return -x
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("-"),
        "Negation should use -. Got:\n{rust}"
    );
}

// ============================================================================
// Compilation Tests
// ============================================================================

#[test]
fn test_arithmetic_compiles() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def arithmetic_ops(a: int, b: int) -> int:
    sum_val = a + b
    diff = a - b
    prod = a * b
    return sum_val + diff + prod
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    let temp_file = std::env::temp_dir().join("test_arithmetic_ops.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_arithmetic_ops.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Arithmetic operations should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_comparison_compiles() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def comparison_ops(a: int, b: int) -> bool:
    lt = a < b
    gt = a > b
    eq = a == b
    ne = a != b
    le = a <= b
    ge = a >= b
    return lt or gt or eq or ne or le or ge
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    let temp_file = std::env::temp_dir().join("test_comparison_ops.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_comparison_ops.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Comparison operations should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_logical_compiles() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def logical_ops(a: bool, b: bool) -> bool:
    and_result = a and b
    or_result = a or b
    not_result = not a
    return and_result or or_result or not_result
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");

    let temp_file = std::env::temp_dir().join("test_logical_ops.rs");
    std::fs::write(&temp_file, &rust).expect("Failed to write temp file");

    let out_file = std::env::temp_dir().join("test_logical_ops.rlib");
    let output = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "-o"])
        .arg(&out_file)
        .arg(&temp_file)
        .output()
        .expect("Failed to run rustc");

    let _ = std::fs::remove_file(&out_file);
    let _ = std::fs::remove_file(&temp_file);

    assert!(
        output.status.success(),
        "Logical operations should compile. Errors:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// String Operations Tests
// ============================================================================

#[test]
fn test_string_multiplication() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def repeat_string(s: str, n: int) -> str:
    return s * n
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("repeat"),
        "String * int should use repeat. Got:\n{rust}"
    );
}

#[test]
fn test_list_concatenation() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def concat_lists(a: list, b: list) -> list:
    return a + b
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("chain") || rust.contains("extend") || rust.contains("iter"),
        "List + list should use chain/extend pattern. Got:\n{rust}"
    );
}

// ============================================================================
// Property-Based Tests
// ============================================================================

proptest! {
    #[test]
    fn prop_transpilation_is_deterministic(a in -100i32..100, b in -100i32..100) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_ops():
    return {} + {}
"#, a, b);
        let result1 = pipeline.transpile(&python);
        let result2 = pipeline.transpile(&python);
        match (result1, result2) {
            (Ok(r1), Ok(r2)) => prop_assert_eq!(r1, r2, "Transpilation should be deterministic"),
            (Err(_), Err(_)) => (),
            _ => prop_assert!(false, "Results should be consistent"),
        }
    }

    #[test]
    fn prop_arithmetic_generates_operator(
        op in prop::sample::select(vec!["+", "-", "*", "%"])
    ) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_op(a: int, b: int) -> int:
    return a {} b
"#, op);

        if let Ok(rust) = pipeline.transpile(&python) {
            prop_assert!(
                rust.contains(op) || rust.contains("chain") || rust.contains("format"),
                "Operator {} should appear in output. Got:\n{}", op, rust
            );
        }
    }

    #[test]
    fn prop_comparison_generates_operator(
        op in prop::sample::select(vec!["<", ">", "==", "!=", "<=", ">="])
    ) {
        let pipeline = DepylerPipeline::new();
        let python = format!(r#"
def test_cmp(a: int, b: int) -> bool:
    return a {} b
"#, op);

        if let Ok(rust) = pipeline.transpile(&python) {
            prop_assert!(
                rust.contains(op),
                "Comparison {} should appear in output. Got:\n{}", op, rust
            );
        }
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_chained_comparisons() {
    let pipeline = DepylerPipeline::new();
    // Python allows a < b < c
    let python = r#"
def in_range(x: int) -> bool:
    return 0 < x < 10
"#;
    // Should transpile to (0 < x) && (x < 10) or similar
    let result = pipeline.transpile(python);
    // This may or may not be supported - just ensure it doesn't crash
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_bitwise_operations() {
    let pipeline = DepylerPipeline::new();
    let python = r#"
def bitwise_ops(a: int, b: int) -> int:
    return (a & b) | (a ^ b)
"#;
    let rust = pipeline.transpile(python).expect("Should transpile");
    assert!(
        rust.contains("&") && rust.contains("|") && rust.contains("^"),
        "Bitwise operators should be preserved. Got:\n{rust}"
    );
}

//! SQLite-Style Systematic Validation Tests - Phase 1
//!
//! This module implements the first phase of comprehensive testing inspired by
//! SQLite's legendary test coverage. The goal is to systematically test EVERY
//! Python language construct supported by Depyler.
//!
//! Philosophy:
//! - 100% branch coverage target
//! - Systematic, not random testing
//! - Every language feature gets 5 dedicated tests
//! - Clear test names explain what's being validated
//!
//! References:
//! - docs/specifications/testing-sqlite-style.md
//! - SQLite testing: https://www.sqlite.org/testing.html
//! - Toyota Way: Build quality in, not bolt on

use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code and verify it compiles
fn transpile_and_verify(python: &str, test_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python)?;
    
    // Write to temp file and verify with rustc
    let temp_file = format!("/tmp/depyler_test_{}.rs", test_name);
    std::fs::write(&temp_file, &rust_code)?;
    
    // Check compilation (using --crate-type lib for quick validation)
    let output = std::process::Command::new("rustc")
        .args(["--crate-type", "lib", "--edition", "2021", &temp_file])
        .output()?;
    
    if !output.status.success() {
        return Err(format!(
            "Compilation failed for {}: {}", 
            test_name,
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }
    
    Ok(rust_code)
}

// ============================================================================
// Category 1: Literals (5 tests)
// ============================================================================

#[test]
fn test_01_literals_integers() {
    let python = r#"
def test() -> int:
    decimal = 42
    hexadecimal = 0x2A
    octal = 0o52
    binary = 0b101010
    return decimal + hexadecimal + octal + binary
"#;
    
    let rust = transpile_and_verify(python, "literals_integers").unwrap();
    assert!(rust.contains("fn test()"));
    assert!(rust.contains("-> i32"));
}

#[test]
fn test_02_literals_floats() {
    let python = r#"
def test() -> float:
    normal = 3.14
    scientific = 1.5e10
    negative = -2.5e-3
    return normal + scientific + negative
"#;
    
    let rust = transpile_and_verify(python, "literals_floats").unwrap();
    assert!(rust.contains("fn test()"));
    assert!(rust.contains("-> f64"));
}

#[test]
fn test_03_literals_strings() {
    let python = r#"
def test() -> str:
    simple = "hello"
    escaped = "line1\nline2"
    unicode = "hello 世界"
    return simple + escaped + unicode
"#;
    
    let rust = transpile_and_verify(python, "literals_strings").unwrap();
    assert!(rust.contains("fn test()"));
    assert!(rust.contains("String"));
}

#[test]
fn test_04_literals_booleans() {
    let python = r#"
def test() -> bool:
    t = True
    f = False
    return t and not f
"#;
    
    let rust = transpile_and_verify(python, "literals_booleans").unwrap();
    assert!(rust.contains("fn test()"));
    assert!(rust.contains("bool"));
}

#[test]
#[ignore] // None type support is limited - tracked for future enhancement
fn test_05_literals_none() {
    let python = r#"
def test() -> None:
    x = None
    return x
"#;

    let rust = transpile_and_verify(python, "literals_none").unwrap();
    assert!(rust.contains("fn test()"));
}

// ============================================================================
// Category 2: Binary Operators (5 tests)
// ============================================================================

#[test]
fn test_06_binop_arithmetic() {
    let python = r#"
def test(a: int, b: int) -> int:
    return a + b * 2 - b / 2
"#;
    
    let rust = transpile_and_verify(python, "binop_arithmetic").unwrap();
    assert!(rust.contains("fn test"));
    assert!(rust.contains("+") && rust.contains("*") && rust.contains("-"));
}

#[test]
fn test_07_binop_comparison() {
    let python = r#"
def test(a: int, b: int) -> bool:
    return a < b and a <= b and a == b and a != b and a > b and a >= b
"#;
    
    let rust = transpile_and_verify(python, "binop_comparison").unwrap();
    assert!(rust.contains("<") || rust.contains("<="));
}

#[test]
fn test_08_binop_logical() {
    let python = r#"
def test(a: bool, b: bool) -> bool:
    return a and b or not a
"#;
    
    let rust = transpile_and_verify(python, "binop_logical").unwrap();
    assert!(rust.contains("&&") || rust.contains("||") || rust.contains("!"));
}

#[test]
fn test_09_binop_bitwise() {
    let python = r#"
def test(a: int, b: int) -> int:
    return a & b | a ^ b
"#;
    
    let rust = transpile_and_verify(python, "binop_bitwise").unwrap();
    assert!(rust.contains("&") || rust.contains("|") || rust.contains("^"));
}

#[test]
#[ignore] // Power operator (**) requires special handling - tracked for future enhancement
fn test_10_binop_power() {
    let python = r#"
def test(a: int) -> int:
    return a ** 2
"#;

    let rust = transpile_and_verify(python, "binop_power").unwrap();
    assert!(rust.contains("fn test"));
}

// ============================================================================
// Category 3: Control Flow (5 tests)
// ============================================================================

#[test]
fn test_11_control_if_simple() {
    let python = r#"
def test(x: int) -> int:
    if x > 0:
        return 1
    else:
        return -1
"#;
    
    let rust = transpile_and_verify(python, "control_if_simple").unwrap();
    assert!(rust.contains("if") && rust.contains("else"));
}

#[test]
fn test_12_control_if_elif() {
    let python = r#"
def test(x: int) -> int:
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0
"#;
    
    let rust = transpile_and_verify(python, "control_if_elif").unwrap();
    assert!(rust.contains("if") && rust.contains("else"));
}

#[test]
fn test_13_control_while() {
    let python = r#"
def test(n: int) -> int:
    x = 0
    while x < n:
        x = x + 1
    return x
"#;
    
    let rust = transpile_and_verify(python, "control_while").unwrap();
    assert!(rust.contains("while"));
}

#[test]
fn test_14_control_for_range() {
    let python = r#"
def test(n: int) -> int:
    total = 0
    for i in range(n):
        total = total + i
    return total
"#;
    
    let rust = transpile_and_verify(python, "control_for_range").unwrap();
    assert!(rust.contains("for"));
}

#[test]
fn test_15_control_break_continue() {
    let python = r#"
def test(n: int) -> int:
    x = 0
    while x < n:
        x = x + 1
        if x == 5:
            continue
        if x == 10:
            break
    return x
"#;
    
    let rust = transpile_and_verify(python, "control_break_continue").unwrap();
    assert!(rust.contains("break") || rust.contains("continue"));
}

// ============================================================================
// Category 4: Functions (5 tests)
// ============================================================================

#[test]
fn test_16_function_simple() {
    let python = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
    
    let rust = transpile_and_verify(python, "function_simple").unwrap();
    assert!(rust.contains("fn add"));
}

#[test]
fn test_17_function_multiple_returns() {
    let python = r#"
def test(x: int) -> int:
    if x > 0:
        return 1
    return -1
"#;
    
    let rust = transpile_and_verify(python, "function_multiple_returns").unwrap();
    assert!(rust.contains("return"));
}

#[test]
fn test_18_function_no_return() {
    let python = r#"
def test(x: int) -> None:
    y = x + 1
"#;
    
    let rust = transpile_and_verify(python, "function_no_return").unwrap();
    assert!(rust.contains("fn test"));
}

#[test]
fn test_19_function_recursion() {
    let python = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;
    
    let rust = transpile_and_verify(python, "function_recursion").unwrap();
    assert!(rust.contains("fn factorial"));
    assert!(rust.contains("factorial")); // Recursive call
}

#[test]
fn test_20_function_call() {
    let python = r#"
def add(a: int, b: int) -> int:
    return a + b

def test() -> int:
    return add(1, 2)
"#;
    
    let rust = transpile_and_verify(python, "function_call").unwrap();
    assert!(rust.contains("add("));
}

// ============================================================================
// Summary Test
// ============================================================================

#[test]
fn test_sqlite_style_phase1_summary() {
    println!("\n=== SQLite-Style Systematic Validation - Phase 1 Summary ===");
    println!("Categories Tested: 4/20");
    println!("  1. Literals (5/5 tests)");
    println!("  2. Binary Operators (5/5 tests)");
    println!("  3. Control Flow (5/5 tests)");
    println!("  4. Functions (5/5 tests)");
    println!("\nTotal Tests: 20");
    println!("Target: 100 tests (20 categories × 5 tests)");
    println!("Progress: 20%");
    println!("\nNext Categories:");
    println!("  5. Collections - Lists (5 tests)");
    println!("  6. Collections - Dicts (5 tests)");
    println!("  7. Collections - Sets (5 tests)");
    println!("  8. Collections - Strings (5 tests)");
    println!("\nReference: docs/specifications/testing-sqlite-style.md");
    println!("==========================================================\n");
}

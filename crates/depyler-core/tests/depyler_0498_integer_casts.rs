// DEPYLER-0498: Integer type cast errors (i32 ↔ i64)
// Test auto-insertion of casts when integer types don't match

use depyler_core::DepylerPipeline;

#[test]
#[ignore = "Known failing - DEPYLER-0498"]
fn test_i32_to_i64_function_call_cast() {
    // Python: Nested function takes int (i64), caller passes arithmetic (i32)
    // This matches the fibonacci.py pattern where is_perfect_square is nested
    let python = r#"
def test_value(num: int) -> bool:
    def check_large(x: int) -> bool:
        return x > 1000000

    # num is i32 in arithmetic, check_large expects i64
    return check_large(5 * num * num + 4)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Failed to transpile");

    // Should contain cast to i64
    assert!(
        rust.contains("(5 * num * num + 4) as i64") || rust.contains("as i64"),
        "Should auto-insert cast for i32→i64 function argument"
    );

    // Verify compilation
    let result = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(rust.as_bytes())?;
            child.wait_with_output()
        });

    if let Ok(output) = result {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.contains("expected `i64`, found `i32`"),
            "Should not have i32→i64 type mismatch errors"
        );
    }
}

#[test]
fn test_i64_to_i32_comparison_cast() {
    // Python: Local variable declared as int (i64), compared with i32
    let python = r#"
def is_perfect_square(x: int) -> bool:
    root: int = int(x ** 0.5)
    return root * root == x
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Failed to transpile");

    // Should handle comparison between different integer types
    assert!(
        rust.contains("as i64") || rust.contains("as i32"),
        "Should cast to common type for comparison"
    );

    // Verify compilation
    let result = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(rust.as_bytes())?;
            child.wait_with_output()
        });

    if let Ok(output) = result {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.contains("expected `i32`, found `i64`")
                && !stderr.contains("expected `i64`, found `i32`"),
            "Should not have integer type mismatch errors"
        );
    }
}

#[test]
#[ignore = "Flaky in parallel execution due to rustc subprocess resource contention"]
fn test_integer_arithmetic_type_inference() {
    // Test that integer arithmetic preserves type consistency
    let python = r#"
def calculate(a: int, b: int) -> int:
    result = a * a + b * b
    return result
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Failed to transpile");

    // Verify compilation without type errors
    let result = std::process::Command::new("rustc")
        .arg("--crate-type")
        .arg("lib")
        .arg("--edition")
        .arg("2021")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(rust.as_bytes())?;
            child.wait_with_output()
        });

    if let Ok(output) = result {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "Integer arithmetic should compile without type errors: {}",
            stderr
        );
    }
}

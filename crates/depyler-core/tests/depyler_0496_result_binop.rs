//! DEPYLER-0496: Binary Operations on Result-Returning Expressions Missing ? Operator
//!
//! Tests that binary operations on Result-returning function calls correctly use
//! the ? operator for error propagation.
//!
//! BUG: fibonacci_memoized(n-1) + fibonacci_memoized(n-2) tries to add Result types
//! Expected: fibonacci_memoized(n-1)? + fibonacci_memoized(n-2)?

use depyler_core::DepylerPipeline;
use std::io::Write;

#[test]
fn test_result_returning_binop_addition() {
    // Simple case: two Result-returning calls with addition
    let python = r#"
def divide(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Division by zero")
    return a // b

def add_divisions(a: int, b: int, c: int, d: int) -> int:
    return divide(a, b) + divide(c, d)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use ? operator on Result-returning calls
    // Pattern: divide(...)? + divide(...)?
    let has_question_marks = rust.matches("divide(").zip(rust.matches(")?")).count() >= 2;

    assert!(
        has_question_marks || rust.contains("divide(a, b)? + divide(c, d)?"),
        "BUG: Binary operation on Result-returning calls must use ? operator\n\
         Expected pattern: divide(a, b)? + divide(c, d)?\n\
         Generated:\n{}",
        rust
    );

    // Should NOT try to add Result types directly
    assert!(
        !rust.contains("Result<") || rust.contains(")?"),
        "BUG: Cannot add Result types without ? operator\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_result_returning_binop_subtraction() {
    // Result-returning calls with subtraction
    let python = r#"
def safe_get(items: list, index: int) -> int:
    if index < 0 or index >= len(items):
        raise IndexError("Index out of bounds")
    return items[index]

def diff(items: list, i: int, j: int) -> int:
    return safe_get(items, i) - safe_get(items, j)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use ? operator for error propagation
    assert!(
        rust.contains("safe_get(") && rust.contains(")?"),
        "BUG: Result-returning calls in subtraction must use ? operator\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_result_returning_binop_multiplication() {
    // Result-returning calls with multiplication
    let python = r#"
def validate_positive(n: int) -> int:
    if n < 0:
        raise ValueError("Must be positive")
    return n

def multiply_validated(a: int, b: int) -> int:
    return validate_positive(a) * validate_positive(b)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use ? operator for error propagation
    assert!(
        rust.contains("validate_positive(") && rust.contains(")?"),
        "BUG: Result-returning calls in multiplication must use ? operator\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_fibonacci_memoized_specific() {
    // The actual fibonacci_memoized case from fibonacci.py
    let python = r#"
from typing import Dict, Optional

def fibonacci_memoized(n: int, memo: Optional[Dict[int, int]] = None) -> int:
    if memo is None:
        memo = {}

    if n in memo:
        return memo[n]

    if n <= 0:
        return 0
    elif n == 1:
        return 1

    result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    memo[n] = result
    return result
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // The critical line: result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    // Should generate: fibonacci_memoized(n - 1, &memo)? + fibonacci_memoized(n - 2, &memo)?

    // Check for ? operator on fibonacci_memoized calls
    let fib_call_count = rust.matches("fibonacci_memoized(n - 1").count();
    let question_after_call = rust
        .matches("fibonacci_memoized(n - 1")
        .filter(|_| rust.contains(")?"))
        .count();

    assert!(
        fib_call_count > 0 && question_after_call > 0,
        "BUG: fibonacci_memoized recursive calls must use ? operator\n\
         Expected: fibonacci_memoized(n - 1, ...)? + fibonacci_memoized(n - 2, ...)?\n\
         Generated:\n{}",
        rust
    );
}

#[test]
fn test_nested_result_binops() {
    // Nested binary operations with Result-returning calls
    let python = r#"
def compute(x: int) -> int:
    if x < 0:
        raise ValueError("Negative value")
    return x * 2

def complex_calc(a: int, b: int, c: int) -> int:
    return (compute(a) + compute(b)) * compute(c)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // All three compute() calls should have ? operator
    let compute_calls = rust.matches("compute(").count();
    assert!(
        compute_calls >= 3,
        "Expected at least 3 compute() calls, found {}\nGenerated:\n{}",
        compute_calls,
        rust
    );

    // Should have ? operators
    assert!(
        rust.contains("compute(a)?") || rust.contains(")? +") || rust.contains(")? *"),
        "BUG: Nested Result binops must use ? operator\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_compilation_no_e0369() {
    // Verify generated code compiles without E0369 error
    let python = r#"
def safe_div(a: int, b: int) -> int:
    if b == 0:
        raise ValueError("Division by zero")
    return a // b

def combined(a: int, b: int, c: int, d: int) -> int:
    return safe_div(a, b) + safe_div(c, d)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Write to temp file
    let mut file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(rust.as_bytes()).expect("Failed to write");

    // Try to compile with rustc
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--crate-name=test_result_binop")
        .arg("--deny=warnings")
        .arg(file.path())
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("rustc output:\n{}", stderr);

    // Should NOT have E0369 error (cannot add Result + Result)
    assert!(
        !stderr.contains("E0369"),
        "BUG: Generated code has E0369 error (cannot add Result types)\n\
         This means Result-returning calls in binary operations are missing ? operator\n\
         rustc stderr:\n{}",
        stderr
    );
}

#[test]
fn test_result_binop_with_literals() {
    // Mix of Result-returning call and literal
    let python = r#"
def get_value() -> int:
    raise NotImplementedError()

def add_ten() -> int:
    return get_value() + 10
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // get_value() returns Result, so needs ? before adding literal
    // Expected: get_value()? + 10
    assert!(
        rust.contains("get_value()?") || (rust.contains("get_value()") && rust.contains(")?")),
        "BUG: Result-returning call must use ? before binop with literal\n\
         Expected: get_value()? + 10\n\
         Generated:\n{}",
        rust
    );
}

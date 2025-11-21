// DEPYLER-0437: Try/except control flow - exception handler branching
//
// Tests that try/except blocks generate proper match expressions with
// exception-specific branches, not sequential code.
//
// Root cause: Try blocks transpiled sequentially without exception handlers
// Solution: Generate match expressions for fallible operations in try blocks
//
// Parent: DEPYLER-0428 (ArgumentTypeError support)

use depyler_core::DepylerPipeline;

/// Helper to transpile Python code
fn transpile_python(python: &str) -> anyhow::Result<String> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python)
}

#[test]
fn test_DEPYLER_0437_try_except_generates_match() {
    // Try/except should generate match expression, not sequential code
    let python = r#"
def parse_int(value):
    try:
        num = int(value)
        return num
    except ValueError:
        return -1
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // CRITICAL: Should use match expression for error handling
    assert!(
        rust.contains("match") && rust.contains(".parse"),
        "Should generate match expression for int(value). Got: {}",
        rust
    );

    // Should NOT use unwrap_or_default (hides the exception handler)
    assert!(
        !rust.contains("unwrap_or_default"),
        "Should not use unwrap_or_default - defeats try/except purpose: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_0437_except_handler_in_err_branch() {
    // Except handler code should be in Err(_) branch
    let python = r#"
def validator(value):
    try:
        num = int(value)
        return num
    except ValueError:
        return -1
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Should have Err branch with except handler code
    assert!(
        rust.contains("Err(") && rust.contains("-1"),
        "Except handler (return -1) should be in Err branch: {}",
        rust
    );

    // Should have Ok branch with try body continuation
    assert!(
        rust.contains("Ok("),
        "Should have Ok branch for successful parse: {}",
        rust
    );
}

#[test]
fn test_DEPYLER_0437_multiple_statements_in_try() {
    // Try block with multiple statements should all be in Ok branch
    let python = r#"
def port_validator(value):
    try:
        port = int(value)
        if port < 1 or port > 65535:
            raise ValueError("bad port")
        return port
    except ValueError:
        return -1
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Match should wrap the int() call
    assert!(
        rust.contains("match") && rust.contains(".parse"),
        "Should match on parse result: {}",
        rust
    );

    // Validation (if port < 1) should be in Ok branch
    assert!(
        rust.contains("Ok(port)") || rust.contains("Ok("),
        "Port validation should be in Ok branch: {}",
        rust
    );

    // Should NOT have unreachable code (current bug)
    // Note: This is checked by rustc, we just verify structure
}

#[test]
fn test_DEPYLER_0437_compiles_without_warnings() {
    // Generated code should compile without unreachable warnings
    let python = r#"
def parse_number(text):
    try:
        result = int(text)
        return result
    except ValueError:
        return 0
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Write to temp file and compile
    std::fs::write("/tmp/depyler_0437_test.rs", &rust).unwrap();

    let compile_result = std::process::Command::new("rustc")
        .args(&[
            "--crate-type",
            "lib",
            "--edition",
            "2021",
            "/tmp/depyler_0437_test.rs",
            "-o",
            "/tmp/depyler_0437_test.rlib",
        ])
        .output();

    assert!(
        compile_result.is_ok(),
        "rustc should run successfully: {:?}",
        compile_result.err()
    );

    let output = compile_result.unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should NOT have unreachable code warnings
    assert!(
        !stderr.contains("unreachable"),
        "Should not have unreachable code warnings. Stderr: {}",
        stderr
    );

    assert!(
        output.status.success(),
        "Compilation should succeed without warnings. Stderr: {}",
        stderr
    );
}

#[test]
fn test_DEPYLER_0437_nested_try_in_ok_branch() {
    // Nested operations in try block should all be in Ok branch
    let python = r#"
def validator(value):
    try:
        num = int(value)
        if num < 0:
            raise ValueError("negative")
        result = num * 2
        return result
    except ValueError:
        return None
"#;

    let result = transpile_python(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust = result.unwrap();

    // Should generate match for the parse
    assert!(
        rust.contains("match"),
        "Should use match for exception handling: {}",
        rust
    );

    // Multiple statements (if num < 0, result = num * 2) should be in Ok branch
    // This ensures they don't run if int() fails
    assert!(
        rust.contains("Ok(") || rust.contains("Ok(num)"),
        "Try block continuations should be in Ok branch: {}",
        rust
    );
}

#[test]
#[ignore] // This test requires building the binary which can timeout in CI
fn test_cli_functionality() {
    use std::fs;
    use std::process::Command;

    println!("🧪 Testing CLI Functionality");

    // Create a test Python file
    let test_code = "def multiply(x: int, y: int) -> int:\n    return x * y";
    fs::write("test_cli.py", test_code).expect("Failed to write test file");

    // Test CLI transpile command - use cargo run instead of direct binary
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "transpile", "test_cli.py"])
        .output()
        .expect("Failed to execute CLI command");

    // Check that the command succeeded
    assert!(output.status.success(), "CLI transpile should succeed");

    // Check that output file was created
    assert!(
        std::path::Path::new("test_cli.rs").exists(),
        "Output Rust file should be created"
    );

    // Test CLI check command
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "check", "test_cli.py"])
        .output()
        .expect("Failed to execute CLI check command");

    assert!(output.status.success(), "CLI check should succeed");

    // Test CLI analyze command
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "analyze", "test_cli.py"])
        .output()
        .expect("Failed to execute CLI analyze command");

    assert!(output.status.success(), "CLI analyze should succeed");

    // Cleanup
    fs::remove_file("test_cli.py").ok();
    fs::remove_file("test_cli.rs").ok();

    println!("✅ All CLI functionality tests passed!");
}

#[test]
fn test_integration_transpilation_pipeline() {
    use depyler_core::DepylerPipeline;

    println!("🧪 Testing Core Transpilation Pipeline");

    let pipeline = DepylerPipeline::new();

    // Test simple function
    let python_code = "def square(n: int) -> int:\n    return n * n";
    let result = pipeline.transpile(python_code);

    assert!(
        result.is_ok(),
        "Simple function should transpile successfully"
    );

    if let Ok(rust_code) = result {
        assert!(rust_code.contains("square"), "Should contain function name");
        assert!(
            rust_code.contains("i32"),
            "Should contain Rust integer type"
        );
        // DEPYLER-0271: Transpiler uses implicit return for final statements (idiomatic Rust)
        // Accept either "return" (early returns) or "n * n" (implicit return)
        assert!(
            rust_code.contains("return") || rust_code.contains("n * n"),
            "Should contain return statement (explicit or implicit)"
        );
    }

    println!("✅ Core transpilation pipeline test passed!");
}

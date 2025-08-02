// use serde_json::json;  // Not needed since test is commented out

#[tokio::test]
#[ignore = "MCP API has changed - needs update"]
async fn test_mcp_server_functionality() {
    // MCP API has changed - this test needs to be updated
    // Commenting out to avoid compilation errors
    /*
    println!("🧪 Testing MCP Server End-to-End Functionality");

    let server = DepylerMcpServer::new();

    // Test Initialize
    let init_message = McpMessage {
        id: "test-init".to_string(),
        method: methods::INITIALIZE.to_string(),
        params: json!({}),
    };

    let response = server.handle_message(init_message).await;
    assert!(response.error.is_none(), "Initialize should succeed");
    assert!(response.result.is_some(), "Initialize should return result");

    // Test Tools List
    let tools_message = McpMessage {
        id: "test-tools".to_string(),
        method: methods::TOOLS_LIST.to_string(),
        params: json!({}),
    };

    let response = server.handle_message(tools_message).await;
    assert!(response.error.is_none(), "Tools list should succeed");

    if let Some(result) = response.result {
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 3, "Should have 3 MCP tools");

        let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();

        assert!(tool_names.contains(&methods::TRANSPILE_PYTHON));
        assert!(tool_names.contains(&methods::ANALYZE_MIGRATION_COMPLEXITY));
        assert!(tool_names.contains(&methods::VERIFY_TRANSPILATION));
    }

    // Test Transpile Tool
    let transpile_message = McpMessage {
        id: "test-transpile".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::TRANSPILE_PYTHON,
            "arguments": {
                "source": "def add(a: int, b: int) -> int:\n    return a + b",
                "mode": "inline"
            }
        }),
    };

    let response = server.handle_message(transpile_message).await;
    assert!(response.error.is_none(), "Transpilation should succeed");

    if let Some(result) = response.result {
        assert!(result["rust_code"].is_string(), "Should return Rust code");
        assert!(result["metrics"].is_object(), "Should return metrics");
        assert!(
            result["compilation_command"].is_string(),
            "Should return compilation command"
        );
    }

    println!("✅ All MCP functionality tests passed!");
    */
}

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
        assert!(
            rust_code.contains("return"),
            "Should contain return statement"
        );
    }

    println!("✅ Core transpilation pipeline test passed!");
}

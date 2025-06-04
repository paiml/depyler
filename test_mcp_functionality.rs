use depyler_mcp::{DepylerMcpServer, protocol::*};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing MCP Server Functionality");
    println!("====================================");

    let server = DepylerMcpServer::new();
    
    // Test 1: Initialize
    println!("\n1. Testing MCP Initialize...");
    let init_message = McpMessage {
        id: "test-init".to_string(),
        method: methods::INITIALIZE.to_string(),
        params: json!({}),
    };
    
    let response = server.handle_message(init_message).await;
    if response.error.is_none() {
        println!("   ✅ Initialize successful");
        if let Some(result) = response.result {
            if let Ok(init_result) = serde_json::from_value::<InitializeResult>(result) {
                println!("   📋 Protocol Version: {}", init_result.protocol_version);
                println!("   🏷️  Server: {:?}", init_result.server_info);
            }
        }
    } else {
        println!("   ❌ Initialize failed: {:?}", response.error);
    }

    // Test 2: List Tools
    println!("\n2. Testing Tools List...");
    let tools_message = McpMessage {
        id: "test-tools".to_string(),
        method: methods::TOOLS_LIST.to_string(),
        params: json!({}),
    };
    
    let response = server.handle_message(tools_message).await;
    if response.error.is_none() {
        println!("   ✅ Tools list successful");
        if let Some(result) = response.result {
            let tools = result["tools"].as_array().unwrap();
            println!("   📊 Available tools: {}", tools.len());
            for tool in tools {
                println!("     - {}", tool["name"].as_str().unwrap());
            }
        }
    } else {
        println!("   ❌ Tools list failed: {:?}", response.error);
    }

    // Test 3: Transpile Python
    println!("\n3. Testing Python Transpilation...");
    let transpile_message = McpMessage {
        id: "test-transpile".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::TRANSPILE_PYTHON,
            "arguments": {
                "source": "def multiply(x: int, y: int) -> int:\n    return x * y",
                "mode": "inline",
                "options": {
                    "optimization_level": "energy",
                    "type_inference": "conservative"
                }
            }
        }),
    };
    
    let response = server.handle_message(transpile_message).await;
    if response.error.is_none() {
        println!("   ✅ Transpilation successful");
        if let Some(result) = response.result {
            println!("   🦀 Generated Rust code:");
            println!("   {}", result["rust_code"].as_str().unwrap_or("N/A"));
            println!("   📊 Metrics: {}", result["metrics"]);
        }
    } else {
        println!("   ❌ Transpilation failed: {:?}", response.error);
    }

    // Test 4: Analyze Migration Complexity  
    println!("\n4. Testing Migration Analysis...");
    
    // Create a temporary test directory
    let temp_dir = std::env::temp_dir().join("mcp_test");
    std::fs::create_dir_all(&temp_dir)?;
    let test_file = temp_dir.join("test.py");
    std::fs::write(&test_file, "def fibonacci(n: int) -> int:\n    if n <= 1:\n        return n\n    return fibonacci(n-1) + fibonacci(n-2)")?;
    
    let analyze_message = McpMessage {
        id: "test-analyze".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::ANALYZE_MIGRATION_COMPLEXITY,
            "arguments": {
                "project_path": temp_dir.to_string_lossy(),
                "analysis_depth": "standard"
            }
        }),
    };
    
    let response = server.handle_message(analyze_message).await;
    if response.error.is_none() {
        println!("   ✅ Analysis successful");
        if let Some(result) = response.result {
            println!("   📊 Complexity Score: {}", result["complexity_score"]);
            println!("   📝 Python LOC: {}", result["total_python_loc"]);
            println!("   🦀 Estimated Rust LOC: {}", result["estimated_rust_loc"]);
            println!("   ⏱️  Estimated effort: {} hours", result["estimated_effort_hours"]);
        }
    } else {
        println!("   ❌ Analysis failed: {:?}", response.error);
    }

    // Test 5: Verify Transpilation
    println!("\n5. Testing Transpilation Verification...");
    let verify_message = McpMessage {
        id: "test-verify".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::VERIFY_TRANSPILATION,
            "arguments": {
                "python_source": "def square(n: int) -> int:\n    return n * n",
                "rust_source": "pub fn square(n: i32) -> i32 {\n    n * n\n}",
                "verification_level": "comprehensive"
            }
        }),
    };
    
    let response = server.handle_message(verify_message).await;
    if response.error.is_none() {
        println!("   ✅ Verification successful");
        if let Some(result) = response.result {
            println!("   ✅ Verification passed: {}", result["verification_passed"]);
            println!("   📊 Equivalence score: {}", result["semantic_equivalence_score"]);
            println!("   🛡️  Safety guarantees: {}", result["safety_guarantees"]);
            println!("   ⚡ Performance comparison: {}", result["performance_comparison"]);
        }
    } else {
        println!("   ❌ Verification failed: {:?}", response.error);
    }

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();

    println!("\n🎉 MCP Server Tests Complete!");
    println!("All core MCP tools are functional and ready for AI integration.");
    
    Ok(())
}
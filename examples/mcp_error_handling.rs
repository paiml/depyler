//! Example demonstrating MCP error handling patterns
//! 
//! This example shows how to use the various error types in the MCP module
//! and how to handle them properly in a real application.

use depyler_mcp::error::DepylerMcpError;
use pmcp::error::Error as McpError;

fn main() {
    println!("=== MCP Error Handling Examples ===\n");
    
    // Example 1: Type inference errors
    handle_type_inference();
    
    // Example 2: Unsafe pattern detection
    handle_unsafe_patterns();
    
    // Example 3: Timeout handling
    handle_timeouts();
    
    // Example 4: Error conversion
    handle_error_conversion();
}

/// Demonstrates handling type inference errors
fn handle_type_inference() {
    println!("1. Type Inference Errors:");
    
    let err = DepylerMcpError::type_inference("Cannot infer type for lambda expression");
    
    match err {
        DepylerMcpError::TypeInferenceError(msg) => {
            println!("  Type inference failed: {}", msg);
            println!("  Suggestion: Add explicit type annotations");
        }
        _ => unreachable!(),
    }
    
    println!();
}

/// Demonstrates handling unsafe pattern errors
fn handle_unsafe_patterns() {
    println!("2. Unsafe Pattern Detection:");
    
    let patterns = vec![
        ("eval", "main.py:42:10"),
        ("exec", "utils.py:15:5"),
        ("__import__", "loader.py:8:12"),
    ];
    
    for (pattern, location) in patterns {
        let err = DepylerMcpError::unsafe_pattern(pattern, location);
        println!("  Detected: {} at {}", pattern, location);
        
        // Convert to MCP error for protocol
        let mcp_err: McpError = err.into();
        match mcp_err {
            McpError::Internal(msg) => {
                println!("  MCP message: {}", msg);
            }
            _ => {}
        }
    }
    
    println!();
}

/// Demonstrates timeout handling
fn handle_timeouts() {
    println!("3. Timeout Handling:");
    
    let timeouts = vec![30, 60, 120];
    
    for timeout in timeouts {
        let err = DepylerMcpError::TranspilationTimeout(timeout);
        println!("  {}", err);
        
        // Suggest remediation
        if timeout > 60 {
            println!("  Suggestion: Consider breaking up the code into smaller modules");
        }
    }
    
    println!();
}

/// Demonstrates error conversion and chaining
fn handle_error_conversion() {
    println!("4. Error Conversion:");
    
    // IO error conversion
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "config.toml not found");
    let mcp_err: DepylerMcpError = io_err.into();
    println!("  IO Error: {}", mcp_err);
    
    // JSON error conversion
    let json_str = "{invalid json}";
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Err(json_err) => {
            let mcp_err: DepylerMcpError = json_err.into();
            println!("  JSON Error: {}", mcp_err);
        }
        Ok(_) => {}
    }
    
    // Chain multiple error types
    let result: Result<(), DepylerMcpError> = process_python_code("def broken(");
    if let Err(e) = result {
        println!("  Processing failed: {}", e);
    }
}

/// Simulates processing Python code that might fail
fn process_python_code(code: &str) -> Result<(), DepylerMcpError> {
    if code.contains("def ") && !code.contains(":") {
        return Err(DepylerMcpError::InvalidInput(
            "Function definition missing colon".into()
        ));
    }
    
    if code.contains("eval") || code.contains("exec") {
        return Err(DepylerMcpError::unsafe_pattern(
            "eval/exec", 
            "input:1:1"
        ));
    }
    
    Ok(())
}
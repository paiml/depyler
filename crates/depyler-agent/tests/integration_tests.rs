use depyler_agent::{Agent, AgentConfig};
use depyler_mcp::McpServer;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;

#[tokio::test]
async fn test_agent_startup_and_shutdown() {
    let config = AgentConfig {
        port: 3100,
        host: "127.0.0.1".to_string(),
        auto_transpile: false,
        ..Default::default()
    };

    let agent = Agent::new(config).unwrap();
    
    // Start agent
    agent.start().await.unwrap();
    
    // Verify status
    let status = agent.status().await.unwrap();
    assert_eq!(status.running, true);
    assert_eq!(status.port, 3100);
    
    // Stop agent
    agent.stop().await.unwrap();
    
    // Verify stopped
    let status = agent.status().await.unwrap();
    assert_eq!(status.running, false);
}

#[tokio::test]
async fn test_project_monitoring() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().to_str().unwrap();
    
    // Create test Python file
    let test_file = temp_dir.path().join("test.py");
    fs::write(&test_file, "def hello(): return 'world'").unwrap();
    
    let config = AgentConfig {
        port: 3101,
        auto_transpile: true,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    // Add project to monitor
    agent.add_project(project_path, vec!["*.py".to_string()])
        .await
        .unwrap();
    
    // Verify project is monitored
    let projects = agent.list_projects().await.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].path, project_path);
    
    // Modify file to trigger transpilation
    fs::write(&test_file, "def hello(): return 'modified'").unwrap();
    
    // Wait for debounce
    sleep(Duration::from_millis(600)).await;
    
    // Check transpilation occurred
    let rust_file = temp_dir.path().join("test.rs");
    assert!(rust_file.exists());
    
    // Remove project
    agent.remove_project(project_path).await.unwrap();
    
    let projects = agent.list_projects().await.unwrap();
    assert_eq!(projects.len(), 0);
    
    agent.stop().await.unwrap();
}

#[tokio::test]
async fn test_mcp_server_integration() {
    let config = AgentConfig {
        port: 3102,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    // Create MCP server
    let mcp_server = McpServer::new(agent.clone()).unwrap();
    
    // Start MCP server in background
    let server_handle = tokio::spawn(async move {
        mcp_server.listen("127.0.0.1:3102").await
    });
    
    // Give server time to start
    sleep(Duration::from_millis(100)).await;
    
    // Test MCP connection
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:3102")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "tools/list",
            "id": 1
        }))
        .send()
        .await;
    
    assert!(response.is_ok());
    
    // Stop agent
    agent.stop().await.unwrap();
    
    // Cleanup
    server_handle.abort();
}

#[tokio::test]
async fn test_transpilation_with_verification() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("fibonacci.py");
    
    fs::write(&test_file, r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#).unwrap();
    
    let config = AgentConfig {
        port: 3103,
        auto_transpile: false,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    // Transpile with verification
    let result = agent.transpile_file(
        test_file.to_str().unwrap(),
        true,  // verify
        false  // optimize
    ).await.unwrap();
    
    assert!(result.success);
    assert!(result.rust_code.contains("fn fibonacci"));
    assert!(result.verification_passed.unwrap_or(false));
    
    agent.stop().await.unwrap();
}

#[tokio::test]
async fn test_batch_transpilation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple Python files
    for i in 0..5 {
        let file = temp_dir.path().join(format!("module{}.py", i));
        fs::write(&file, format!("def func{}(): return {}", i, i)).unwrap();
    }
    
    let config = AgentConfig {
        port: 3104,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    // Transpile directory
    let result = agent.transpile_directory(
        temp_dir.path().to_str().unwrap(),
        vec!["*.py".to_string()],
        false,  // verify
        false   // optimize
    ).await.unwrap();
    
    assert_eq!(result.files_processed, 5);
    assert_eq!(result.files_succeeded, 5);
    assert_eq!(result.files_failed, 0);
    
    // Verify Rust files created
    for i in 0..5 {
        let rust_file = temp_dir.path().join(format!("module{}.rs", i));
        assert!(rust_file.exists());
    }
    
    agent.stop().await.unwrap();
}

#[tokio::test]
async fn test_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_file = temp_dir.path().join("invalid.py");
    
    // Write invalid Python code
    fs::write(&invalid_file, "def broken(: return").unwrap();
    
    let config = AgentConfig {
        port: 3105,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    // Attempt transpilation
    let result = agent.transpile_file(
        invalid_file.to_str().unwrap(),
        false,
        false
    ).await;
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("ParseError") || 
            error.to_string().contains("SyntaxError"));
    
    agent.stop().await.unwrap();
}

#[tokio::test]
async fn test_concurrent_transpilation() {
    let temp_dir = TempDir::new().unwrap();
    let mut handles = vec![];
    
    // Create files
    for i in 0..10 {
        let file = temp_dir.path().join(format!("concurrent{}.py", i));
        fs::write(&file, format!("def concurrent{}(): return {}", i, i)).unwrap();
    }
    
    let config = AgentConfig {
        port: 3106,
        max_workers: 4,  // Limit concurrency
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    // Spawn concurrent transpilation tasks
    for i in 0..10 {
        let file = temp_dir.path().join(format!("concurrent{}.py", i));
        let agent_clone = agent.clone();
        
        let handle = tokio::spawn(async move {
            agent_clone.transpile_file(
                file.to_str().unwrap(),
                false,
                false
            ).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
    
    // Verify all Rust files created
    for i in 0..10 {
        let rust_file = temp_dir.path().join(format!("concurrent{}.rs", i));
        assert!(rust_file.exists());
    }
    
    agent.stop().await.unwrap();
}

#[tokio::test]
async fn test_agent_restart() {
    let config = AgentConfig {
        port: 3107,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    
    // Start
    agent.start().await.unwrap();
    assert!(agent.status().await.unwrap().running);
    
    // Restart
    agent.restart().await.unwrap();
    assert!(agent.status().await.unwrap().running);
    
    // Stop
    agent.stop().await.unwrap();
    assert!(!agent.status().await.unwrap().running);
}

#[tokio::test]
async fn test_file_watch_debouncing() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("debounce.py");
    
    fs::write(&test_file, "def test(): return 1").unwrap();
    
    let config = AgentConfig {
        port: 3108,
        auto_transpile: true,
        watch_debounce: Duration::from_millis(500),
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    agent.add_project(
        temp_dir.path().to_str().unwrap(),
        vec!["*.py".to_string()]
    ).await.unwrap();
    
    // Make rapid changes
    for i in 0..5 {
        fs::write(&test_file, format!("def test(): return {}", i)).unwrap();
        sleep(Duration::from_millis(50)).await;
    }
    
    // Wait for debounce period
    sleep(Duration::from_millis(600)).await;
    
    // Check that only one transpilation occurred
    let rust_file = temp_dir.path().join("debounce.rs");
    assert!(rust_file.exists());
    
    let content = fs::read_to_string(&rust_file).unwrap();
    assert!(content.contains("return 4"));  // Last value
    
    agent.stop().await.unwrap();
}

#[tokio::test]
async fn test_python_compatibility_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("compat.py");
    
    fs::write(&test_file, r#"
def supported(): return 42
async def unsupported(): pass
class MyClass: pass
"#).unwrap();
    
    let config = AgentConfig {
        port: 3109,
        ..Default::default()
    };
    
    let agent = Agent::new(config).unwrap();
    agent.start().await.unwrap();
    
    let analysis = agent.analyze_compatibility(
        test_file.to_str().unwrap()
    ).await.unwrap();
    
    assert!(analysis.supported_features.contains(&"function_def".to_string()));
    assert!(analysis.unsupported_features.contains(&"async_function".to_string()));
    assert!(analysis.unsupported_features.contains(&"class_def".to_string()));
    assert!(analysis.compatibility_score < 1.0);
    
    agent.stop().await.unwrap();
}
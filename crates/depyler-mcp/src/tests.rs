use crate::protocol::*;
use crate::server::DepylerMcpServer;
use crate::tools::*;
use serde_json::json;

#[tokio::test]
async fn test_initialize() {
    let server = DepylerMcpServer::new();
    let message = McpMessage {
        id: "1".to_string(),
        method: methods::INITIALIZE.to_string(),
        params: json!({}),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_none());
    assert!(response.result.is_some());

    let result: InitializeResult = serde_json::from_value(response.result.unwrap()).unwrap();
    assert_eq!(result.protocol_version, MCP_VERSION);
    assert!(result.server_info.is_some());
    assert_eq!(result.server_info.unwrap().name, "depyler-mcp");
}

#[tokio::test]
async fn test_tools_list() {
    let server = DepylerMcpServer::new();
    let message = McpMessage {
        id: "2".to_string(),
        method: methods::TOOLS_LIST.to_string(),
        params: json!({}),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_none());
    assert!(response.result.is_some());

    let result = response.result.unwrap();
    let tools = result["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 3);

    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&methods::TRANSPILE_PYTHON));
    assert!(tool_names.contains(&methods::ANALYZE_MIGRATION_COMPLEXITY));
    assert!(tool_names.contains(&methods::VERIFY_TRANSPILATION));
}

#[tokio::test]
async fn test_transpile_python_inline() {
    let server = DepylerMcpServer::new();
    let message = McpMessage {
        id: "3".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::TRANSPILE_PYTHON,
            "arguments": {
                "source": "def add(a: int, b: int) -> int:\n    return a + b",
                "mode": "inline"
            }
        }),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_none());
    assert!(response.result.is_some());

    let result = response.result.unwrap();
    assert!(result["rust_code"].is_string());
    assert!(result["metrics"].is_object());
    assert!(result["compilation_command"].is_string());
}

#[tokio::test]
async fn test_analyze_migration_complexity() {
    let server = DepylerMcpServer::new();

    // Create a temporary directory with a Python file for testing
    let temp_dir = std::env::temp_dir().join("depyler_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let test_file = temp_dir.join("test.py");
    std::fs::write(&test_file, "def hello():\n    print('Hello, world!')").unwrap();

    let message = McpMessage {
        id: "4".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::ANALYZE_MIGRATION_COMPLEXITY,
            "arguments": {
                "project_path": temp_dir.to_string_lossy()
            }
        }),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_none());
    assert!(response.result.is_some());

    let result = response.result.unwrap();
    assert!(result["complexity_score"].is_number());
    assert!(result["total_python_loc"].is_number());
    assert!(result["migration_strategy"].is_object());

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[tokio::test]
async fn test_verify_transpilation() {
    let server = DepylerMcpServer::new();
    let message = McpMessage {
        id: "5".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": methods::VERIFY_TRANSPILATION,
            "arguments": {
                "python_source": "def add(a: int, b: int) -> int:\n    return a + b",
                "rust_source": "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}"
            }
        }),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_none());
    assert!(response.result.is_some());

    let result = response.result.unwrap();
    assert!(result["verification_passed"].is_boolean());
    assert!(result["semantic_equivalence_score"].is_number());
    assert!(result["test_results"].is_object());
    assert!(result["safety_guarantees"].is_object());
}

#[tokio::test]
async fn test_invalid_tool_call() {
    let server = DepylerMcpServer::new();
    let message = McpMessage {
        id: "6".to_string(),
        method: methods::TOOLS_CALL.to_string(),
        params: json!({
            "name": "nonexistent_tool",
            "arguments": {}
        }),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_some());
    assert!(response.result.is_none());
}

#[tokio::test]
async fn test_invalid_method() {
    let server = DepylerMcpServer::new();
    let message = McpMessage {
        id: "7".to_string(),
        method: "invalid_method".to_string(),
        params: json!({}),
    };

    let response = server.handle_message(message).await;
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, error_codes::METHOD_NOT_FOUND);
}

#[test]
fn test_transpile_request_deserialization() {
    let json_data = json!({
        "source": "def test(): pass",
        "mode": "inline",
        "options": {
            "optimization_level": "energy",
            "type_inference": "conservative"
        }
    });

    let request: TranspileRequest = serde_json::from_value(json_data).unwrap();
    assert_eq!(request.source, "def test(): pass");
    assert_eq!(request.mode, Mode::Inline);
}

#[test]
fn test_analyze_request_deserialization() {
    let json_data = json!({
        "project_path": "/path/to/project",
        "analysis_depth": "deep"
    });

    let request: AnalyzeRequest = serde_json::from_value(json_data).unwrap();
    assert_eq!(request.project_path, "/path/to/project");
    assert_eq!(request.analysis_depth, AnalysisDepth::Deep);
}

#[test]
fn test_verify_request_deserialization() {
    let json_data = json!({
        "python_source": "def test(): pass",
        "rust_source": "fn test() {}",
        "verification_level": "comprehensive"
    });

    let request: VerifyRequest = serde_json::from_value(json_data).unwrap();
    assert_eq!(request.python_source, "def test(): pass");
    assert_eq!(request.rust_source, "fn test() {}");
    assert_eq!(request.verification_level, VerificationLevel::Comprehensive);
}

// Include validator tests
mod validator_tests {
    use crate::validator::McpValidator;
    use crate::McpTranspilationResponse;

    #[tokio::test]
    async fn test_validator_creation() {
        let validator = McpValidator::new();
        assert_eq!(validator, McpValidator);
    }

    #[tokio::test]
    async fn test_validate_valid_rust_code() {
        let validator = McpValidator::new();
        let response = McpTranspilationResponse {
            rust_code: "fn hello() -> String { \"hello\".to_string() }".to_string(),
            explanation:
                "This function returns a greeting because it demonstrates simple Rust syntax"
                    .to_string(),
            test_cases: vec![],
            confidence: 0.9,
            alternative_approaches: vec![],
        };

        let result = validator.validate_response(&response).await.unwrap();
        assert!(result.syntactically_valid);
        assert!(result.type_checks);
        assert!(result.tests_pass);
        assert!(result.complexity_acceptable);
        assert!(result.explanation_quality > 0.5);
    }

    #[tokio::test]
    async fn test_validate_invalid_rust_code() {
        let validator = McpValidator::new();
        let response = McpTranspilationResponse {
            rust_code: "fn broken() { let x =".to_string(),
            explanation: "Short".to_string(),
            test_cases: vec![],
            confidence: 0.1,
            alternative_approaches: vec![],
        };

        let result = validator.validate_response(&response).await.unwrap();
        assert!(!result.syntactically_valid);
        assert!(!result.type_checks);
        // Note: tests_pass is based on empty test cases OR confidence > 0.8,
        // so with empty test cases and confidence 0.1, it should pass in the validator logic
        // but since syntactically_valid is false, overall validation should fail
    }
}

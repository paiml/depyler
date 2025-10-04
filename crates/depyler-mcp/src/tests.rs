use crate::server::*;
use crate::tools::*;
use pmcp::server::ToolHandler;
use pmcp::RequestHandlerExtra;
use serde_json::json;

#[tokio::test]
async fn test_server_creation() {
    let server_result = DepylerMcpServer::create_server().await;
    assert!(server_result.is_ok());
}

#[tokio::test]
async fn test_transpile_tool_handler() {
    let transpiler = std::sync::Arc::new(depyler_core::DepylerPipeline::new());
    let tool = TranspileTool::new(transpiler);

    let args = json!({
        "source": "def add(a: int, b: int) -> int:\n    return a + b",
        "mode": "inline"
    });

    let extra = RequestHandlerExtra {
        session_id: Some("test-session".to_string()),
        auth_info: None,
        auth_context: None,
        request_id: "test".to_string(),
        cancellation_token: tokio_util::sync::CancellationToken::new(),
    };

    let result = tool.handle(args, extra).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response["rust_code"].is_string());
    assert!(response["metrics"].is_object());
    assert!(response["compilation_command"].is_string());
}

#[tokio::test]
async fn test_analyze_tool_handler() {
    let transpiler = std::sync::Arc::new(depyler_core::DepylerPipeline::new());
    let tool = AnalyzeTool::new(transpiler);

    // Create a temporary directory with a Python file for testing
    let temp_dir = std::env::temp_dir().join("depyler_test_analyze");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let test_file = temp_dir.join("test.py");
    std::fs::write(&test_file, "def hello():\n    print('Hello, world!')").unwrap();

    let args = json!({
        "project_path": temp_dir.to_string_lossy()
    });

    let extra = RequestHandlerExtra {
        session_id: Some("test-session".to_string()),
        auth_info: None,
        auth_context: None,
        request_id: "test".to_string(),
        cancellation_token: tokio_util::sync::CancellationToken::new(),
    };

    let result = tool.handle(args, extra).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response["complexity_score"].is_number());
    assert!(response["total_python_loc"].is_number());
    assert!(response["migration_strategy"].is_object());

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[tokio::test]
async fn test_verify_tool_handler() {
    let transpiler = std::sync::Arc::new(depyler_core::DepylerPipeline::new());
    let tool = VerifyTool::new(transpiler);

    let args = json!({
        "python_source": "def add(a: int, b: int) -> int:\n    return a + b",
        "rust_source": "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}"
    });

    let extra = RequestHandlerExtra {
        session_id: Some("test-session".to_string()),
        auth_info: None,
        auth_context: None,
        request_id: "test".to_string(),
        cancellation_token: tokio_util::sync::CancellationToken::new(),
    };

    let result = tool.handle(args, extra).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response["verification_passed"].is_boolean());
    assert!(response["semantic_equivalence_score"].is_number());
    assert!(response["test_results"].is_object());
    assert!(response["safety_guarantees"].is_object());
}

#[tokio::test]
async fn test_transpile_tool_invalid_args() {
    let transpiler = std::sync::Arc::new(depyler_core::DepylerPipeline::new());
    let tool = TranspileTool::new(transpiler);

    let args = json!({
        "invalid_field": "value"
    });

    let extra = RequestHandlerExtra {
        session_id: Some("test-session".to_string()),
        auth_info: None,
        auth_context: None,
        request_id: "test".to_string(),
        cancellation_token: tokio_util::sync::CancellationToken::new(),
    };

    let result = tool.handle(args, extra).await;
    assert!(result.is_err());
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

// Client tests
mod client_tests {
    use crate::McpClient;

    #[test]
    fn test_client_creation() {
        let client = McpClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_availability() {
        let client = McpClient::new().unwrap();
        assert!(!client.is_available()); // Should be false when no transport is configured
    }
}

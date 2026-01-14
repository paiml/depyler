pub mod error;
pub mod pmat_integration;
pub mod server;
pub mod tools;
pub mod transport;
pub mod validator;

#[cfg(test)]
mod tests;

pub use error::DepylerMcpError;
pub use pmat_integration::{PmatIntegration, PmatQualityReport};
pub use server::{AnalyzeTool, DepylerMcpServer, PmatQualityTool, TranspileTool, VerifyTool};
pub use tools::{
    AnalyzeRequest, CrateRecommendation, MigrationPhase, PerformanceComparison, SafetyGuarantees,
    TestResults, TranspileMetrics, TranspileRequest, TranspileResponse, VerifyRequest,
};
pub use transport::TransportFactory;

// Re-export pmcp types for convenience
pub use pmcp::{
    Error as McpError, RequestHandlerExtra, Server, ServerBuilder, ToolHandler, Transport,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct McpClient {
    client: Option<Box<dyn std::any::Any + Send>>,
}

#[derive(Debug, Serialize)]
pub struct McpTranspilationRequest {
    pub version: &'static str,
    pub python_ast: serde_json::Value,
    pub error_context: ErrorContext,
    pub quality_hints: QualityHints,
}

#[derive(Debug, Serialize)]
pub struct ErrorContext {
    pub error_message: String,
    pub error_location: Option<Location>,
    pub attempted_approach: String,
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Serialize)]
pub struct QualityHints {
    pub target_complexity: u32,
    pub preferred_types: Vec<String>,
    pub style_level: StyleLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StyleLevel {
    Basic,
    Idiomatic,
    Optimized,
}

#[derive(Debug, Deserialize)]
pub struct McpTranspilationResponse {
    pub rust_code: String,
    pub explanation: String,
    pub test_cases: Vec<TestCase>,
    pub confidence: f64,
    pub alternative_approaches: Vec<AlternativeApproach>,
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub input: serde_json::Value,
    pub expected_output: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct AlternativeApproach {
    pub name: String,
    pub description: String,
    pub trade_offs: String,
}

impl McpClient {
    pub fn new() -> Result<Self> {
        Ok(Self { client: None })
    }

    pub async fn with_stdio() -> Result<Self> {
        // Initialize with stdio transport when needed
        Ok(Self { client: None })
    }

    pub fn is_available(&self) -> bool {
        self.client.is_some()
    }

    pub async fn transpile_fallback(
        &mut self,
        _request: McpTranspilationRequest,
    ) -> Result<McpTranspilationResponse> {
        // For now, always return fallback response since client integration needs API updates
        self.fallback_response().await
    }

    async fn fallback_response(&self) -> Result<McpTranspilationResponse> {
        let mock_response = McpTranspilationResponse {
            rust_code: "// MCP client not initialized - fallback response".to_string(),
            explanation: "This construct requires MCP assistance for proper transpilation"
                .to_string(),
            test_cases: vec![],
            confidence: 0.1,
            alternative_approaches: vec![],
        };

        Ok(mock_response)
    }

    pub async fn transpile_fallback_sync(
        &mut self,
        request: McpTranspilationRequest,
    ) -> Result<McpTranspilationResponse> {
        self.transpile_fallback(request).await
    }
}

impl Default for McpClient {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            eprintln!("Warning: Failed to create MCP client: {}", e);
            McpClient { client: None }
        })
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_mcp_transpilation_request_creation() {
        let request = McpTranspilationRequest {
            version: "1.0",
            python_ast: serde_json::json!({"type": "Module"}),
            error_context: ErrorContext {
                error_message: "test error".to_string(),
                error_location: None,
                attempted_approach: "direct transpilation".to_string(),
            },
            quality_hints: QualityHints {
                target_complexity: 10,
                preferred_types: vec!["String".to_string()],
                style_level: StyleLevel::Idiomatic,
            },
        };

        assert_eq!(request.version, "1.0");
        assert_eq!(request.quality_hints.target_complexity, 10);
    }

    #[test]
    fn test_error_context_with_location() {
        let ctx = ErrorContext {
            error_message: "type mismatch".to_string(),
            error_location: Some(Location { line: 10, column: 5 }),
            attempted_approach: "type inference".to_string(),
        };

        assert_eq!(ctx.error_message, "type mismatch");
        assert!(ctx.error_location.is_some());
        let loc = ctx.error_location.unwrap();
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
    }

    #[test]
    fn test_location_creation() {
        let loc = Location { line: 1, column: 0 };
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 0);
    }

    #[test]
    fn test_quality_hints_creation() {
        let hints = QualityHints {
            target_complexity: 5,
            preferred_types: vec!["i64".to_string(), "String".to_string()],
            style_level: StyleLevel::Basic,
        };

        assert_eq!(hints.target_complexity, 5);
        assert_eq!(hints.preferred_types.len(), 2);
    }

    #[test]
    fn test_style_level_variants() {
        assert!(matches!(StyleLevel::Basic, StyleLevel::Basic));
        assert!(matches!(StyleLevel::Idiomatic, StyleLevel::Idiomatic));
        assert!(matches!(StyleLevel::Optimized, StyleLevel::Optimized));
    }

    #[test]
    fn test_style_level_clone() {
        let level = StyleLevel::Idiomatic;
        let cloned = level; // Copy type, clone() unnecessary
        assert!(matches!(cloned, StyleLevel::Idiomatic));
    }

    #[test]
    fn test_style_level_copy() {
        let level = StyleLevel::Optimized;
        let copied: StyleLevel = level;
        assert!(matches!(copied, StyleLevel::Optimized));
        assert!(matches!(level, StyleLevel::Optimized));
    }

    #[test]
    fn test_style_level_serialization() {
        let level = StyleLevel::Basic;
        let json = serde_json::to_string(&level).unwrap();
        assert!(!json.is_empty());

        let deserialized: StyleLevel = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, StyleLevel::Basic));
    }

    #[test]
    fn test_mcp_transpilation_response_creation() {
        let response = McpTranspilationResponse {
            rust_code: "fn test() {}".to_string(),
            explanation: "A test function".to_string(),
            test_cases: vec![TestCase {
                name: "test_one".to_string(),
                input: serde_json::json!(1),
                expected_output: serde_json::json!(1),
            }],
            confidence: 0.9,
            alternative_approaches: vec![],
        };

        assert_eq!(response.rust_code, "fn test() {}");
        assert_eq!(response.confidence, 0.9);
        assert_eq!(response.test_cases.len(), 1);
    }

    #[test]
    fn test_test_case_creation() {
        let test_case = TestCase {
            name: "test_add".to_string(),
            input: serde_json::json!({"a": 1, "b": 2}),
            expected_output: serde_json::json!(3),
        };

        assert_eq!(test_case.name, "test_add");
    }

    #[test]
    fn test_alternative_approach_creation() {
        let approach = AlternativeApproach {
            name: "Functional style".to_string(),
            description: "Uses map/filter instead of loops".to_string(),
            trade_offs: "More idiomatic but potentially slower".to_string(),
        };

        assert_eq!(approach.name, "Functional style");
        assert!(!approach.description.is_empty());
    }

    #[test]
    fn test_mcp_client_new() {
        let client = McpClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_mcp_client_default() {
        let client = McpClient::default();
        assert!(!client.is_available());
    }

    #[test]
    fn test_mcp_client_is_available() {
        let client = McpClient::new().unwrap();
        assert!(!client.is_available());
    }

    #[tokio::test]
    async fn test_mcp_client_with_stdio() {
        let client = McpClient::with_stdio().await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_mcp_client_transpile_fallback() {
        let mut client = McpClient::new().unwrap();
        let request = McpTranspilationRequest {
            version: "1.0",
            python_ast: serde_json::json!({}),
            error_context: ErrorContext {
                error_message: "test".to_string(),
                error_location: None,
                attempted_approach: "test".to_string(),
            },
            quality_hints: QualityHints {
                target_complexity: 10,
                preferred_types: vec![],
                style_level: StyleLevel::Basic,
            },
        };

        let result = client.transpile_fallback(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.rust_code.contains("MCP client not initialized"));
        assert!(response.confidence < 0.5);
    }

    #[tokio::test]
    async fn test_mcp_client_transpile_fallback_sync() {
        let mut client = McpClient::new().unwrap();
        let request = McpTranspilationRequest {
            version: "1.0",
            python_ast: serde_json::json!({}),
            error_context: ErrorContext {
                error_message: "test".to_string(),
                error_location: None,
                attempted_approach: "test".to_string(),
            },
            quality_hints: QualityHints {
                target_complexity: 10,
                preferred_types: vec![],
                style_level: StyleLevel::Basic,
            },
        };

        let result = client.transpile_fallback_sync(request).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_mcp_client_debug() {
        let client = McpClient::new().unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("McpClient"));
    }
}

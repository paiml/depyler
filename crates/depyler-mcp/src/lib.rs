mod error;
pub mod protocol;
mod server;
mod tools;
pub mod validator;

#[cfg(test)]
mod tests;

pub use error::DepylerMcpError;
pub use server::DepylerMcpServer;
pub use tools::{AnalyzeRequest, TranspileRequest, VerifyRequest};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct McpClient {
    endpoint: Option<String>,
    runtime: Runtime,
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
        Ok(Self {
            endpoint: None,
            runtime: Runtime::new()?,
        })
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn is_available(&self) -> bool {
        self.endpoint.is_some()
    }

    pub async fn transpile_fallback(
        &self,
        _request: McpTranspilationRequest,
    ) -> Result<McpTranspilationResponse> {
        // V1: Stub implementation
        // In a real implementation, this would make an HTTP request to the MCP service

        let mock_response = McpTranspilationResponse {
            rust_code: "// MCP transpilation not yet implemented".to_string(),
            explanation: "This construct requires MCP assistance for proper transpilation"
                .to_string(),
            test_cases: vec![],
            confidence: 0.5,
            alternative_approaches: vec![],
        };

        Ok(mock_response)
    }

    pub fn transpile_fallback_sync(
        &self,
        request: McpTranspilationRequest,
    ) -> Result<McpTranspilationResponse> {
        self.runtime.block_on(self.transpile_fallback(request))
    }
}

impl Default for McpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create MCP client")
    }
}

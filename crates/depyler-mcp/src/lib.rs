mod error;
mod server;
mod tools;
pub mod transport;
pub mod validator;

#[cfg(test)]
mod tests;

pub use error::DepylerMcpError;
pub use server::DepylerMcpServer;
pub use tools::{AnalyzeRequest, TranspileRequest, VerifyRequest};
pub use transport::TransportFactory;

// Re-export pmcp types for convenience
pub use pmcp::{
    Client, Error as McpError, RequestHandlerExtra, Server, StdioTransport, ToolHandler, Transport,
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
        // For now, create a disabled client since the API has changed significantly
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
        Self::new().expect("Failed to create MCP client")
    }
}

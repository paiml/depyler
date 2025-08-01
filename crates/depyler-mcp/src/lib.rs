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
    client::Client,
    error::Error as McpError,
    server::{Server, ToolHandler},
    transport::Transport,
    RequestHandlerExtra,
    StdioTransport,
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
        Ok(Self {
            client: None,
        })
    }

    pub fn is_available(&self) -> bool {
        self.client.is_some()
    }

    pub async fn transpile_fallback(
        &mut self,
        request: McpTranspilationRequest,
    ) -> Result<McpTranspilationResponse> {
        if let Some(client) = &mut self.client {
            // Try to call the transpile tool via MCP
            let tool_args = serde_json::json!({
                "source": request.python_ast.to_string(),
                "mode": "inline",
                "options": {
                    "optimization_level": match request.quality_hints.style_level {
                        StyleLevel::Basic => "size",
                        StyleLevel::Idiomatic => "energy",
                        StyleLevel::Optimized => "speed",
                    },
                    "type_inference": "conservative",
                    "memory_model": "stack_preferred"
                }
            });

            match client
                .call_tool("transpile_python".to_string(), tool_args)
                .await
            {
                Ok(result) => {
                    // Parse the MCP tool result into our response format
                    let rust_code = result
                        .get("rust_code")
                        .and_then(|v| v.as_str())
                        .unwrap_or("// MCP transpilation returned invalid format")
                        .to_string();

                    let explanation = result
                        .get("explanation")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Transpiled via MCP")
                        .to_string();

                    Ok(McpTranspilationResponse {
                        rust_code,
                        explanation,
                        test_cases: vec![],
                        confidence: 0.9,
                        alternative_approaches: vec![],
                    })
                }
                Err(e) => {
                    tracing::warn!("MCP tool call failed: {}", e);
                    self.fallback_response(request).await
                }
            }
        } else {
            self.fallback_response(request).await
        }
    }

    async fn fallback_response(
        &self,
        _request: McpTranspilationRequest,
    ) -> Result<McpTranspilationResponse> {
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

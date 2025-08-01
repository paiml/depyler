use pmcp::error::Error as McpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepylerMcpError {
    #[error("Type inference failed: {0}")]
    TypeInferenceError(String),

    #[error("Unsafe pattern detected: {pattern} at {location}")]
    UnsafePatternError { pattern: String, location: String },

    #[error("Dynamic feature not supported: {0}")]
    UnsupportedDynamicFeature(String),

    #[error("Transpilation timeout after {0} seconds")]
    TranspilationTimeout(u64),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("MCP error: {0}")]
    Mcp(#[from] McpError),
}

impl From<DepylerMcpError> for McpError {
    fn from(err: DepylerMcpError) -> Self {
        match err {
            DepylerMcpError::Mcp(mcp_err) => mcp_err,
            DepylerMcpError::TypeInferenceError(msg) => {
                McpError::Internal(format!("Type inference failed: {}", msg))
            }
            DepylerMcpError::UnsafePatternError { pattern, location } => McpError::Internal(
                format!("Unsafe pattern detected: {} at {}", pattern, location),
            ),
            DepylerMcpError::UnsupportedDynamicFeature(msg) => {
                McpError::Internal(format!("Dynamic feature not supported: {}", msg))
            }
            DepylerMcpError::TranspilationTimeout(secs) => {
                McpError::Internal(format!("Transpilation timeout after {} seconds", secs))
            }
            DepylerMcpError::InvalidInput(msg) => McpError::Internal(msg),
            DepylerMcpError::Internal(msg) => McpError::Internal(msg),
            DepylerMcpError::Io(err) => McpError::Internal(format!("IO error: {}", err)),
            DepylerMcpError::Json(err) => McpError::Internal(format!("JSON error: {}", err)),
        }
    }
}

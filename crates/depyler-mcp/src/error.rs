use crate::protocol::McpError;
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
}

impl From<DepylerMcpError> for McpError {
    fn from(err: DepylerMcpError) -> Self {
        McpError {
            code: match &err {
                DepylerMcpError::TypeInferenceError(_) => -32001,
                DepylerMcpError::UnsafePatternError { .. } => -32002,
                DepylerMcpError::UnsupportedDynamicFeature(_) => -32003,
                DepylerMcpError::TranspilationTimeout(_) => -32004,
                DepylerMcpError::InvalidInput(_) => -32005,
                DepylerMcpError::Internal(_) => -32006,
                DepylerMcpError::Io(_) => -32007,
                DepylerMcpError::Json(_) => -32008,
            },
            message: err.to_string(),
            data: None,
        }
    }
}

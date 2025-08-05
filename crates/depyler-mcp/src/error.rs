use pmcp::error::Error as McpError;
use thiserror::Error;

/// Error types for the Depyler MCP server
/// 
/// # Example
/// ```
/// use depyler_mcp::error::DepylerMcpError;
/// 
/// // Create a type inference error
/// let err = DepylerMcpError::TypeInferenceError("unknown type".into());
/// println!("Error: {}", err);
/// 
/// // Convert to MCP error for protocol compatibility
/// let mcp_err: pmcp::error::Error = err.into();
/// ```
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

impl DepylerMcpError {
    /// Creates a type inference error
    /// 
    /// # Example
    /// ```
    /// use depyler_mcp::error::DepylerMcpError;
    /// 
    /// let err = DepylerMcpError::type_inference("cannot infer type");
    /// assert!(err.to_string().contains("Type inference failed"));
    /// ```
    pub fn type_inference(msg: impl Into<String>) -> Self {
        Self::TypeInferenceError(msg.into())
    }

    /// Creates an unsafe pattern error
    /// 
    /// # Example
    /// ```
    /// use depyler_mcp::error::DepylerMcpError;
    /// 
    /// let err = DepylerMcpError::unsafe_pattern("eval", "main.py:42");
    /// assert!(err.to_string().contains("eval"));
    /// assert!(err.to_string().contains("main.py:42"));
    /// ```
    pub fn unsafe_pattern(pattern: impl Into<String>, location: impl Into<String>) -> Self {
        Self::UnsafePatternError {
            pattern: pattern.into(),
            location: location.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_type_inference_error() {
        let err = DepylerMcpError::TypeInferenceError("unable to infer type for variable x".into());
        assert_eq!(err.to_string(), "Type inference failed: unable to infer type for variable x");
    }

    #[test]
    fn test_unsafe_pattern_error() {
        let err = DepylerMcpError::UnsafePatternError {
            pattern: "eval()".into(),
            location: "line 42, column 10".into(),
        };
        assert_eq!(err.to_string(), "Unsafe pattern detected: eval() at line 42, column 10");
    }

    #[test]
    fn test_unsupported_dynamic_feature() {
        let err = DepylerMcpError::UnsupportedDynamicFeature("metaclass manipulation".into());
        assert_eq!(err.to_string(), "Dynamic feature not supported: metaclass manipulation");
    }

    #[test]
    fn test_transpilation_timeout() {
        let err = DepylerMcpError::TranspilationTimeout(30);
        assert_eq!(err.to_string(), "Transpilation timeout after 30 seconds");
    }

    #[test]
    fn test_invalid_input() {
        let err = DepylerMcpError::InvalidInput("expected Python source code".into());
        assert_eq!(err.to_string(), "Invalid input: expected Python source code");
    }

    #[test]
    fn test_internal_error() {
        let err = DepylerMcpError::Internal("unexpected state".into());
        assert_eq!(err.to_string(), "Internal error: unexpected state");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: DepylerMcpError = io_err.into();
        assert!(matches!(err, DepylerMcpError::Io(_)));
        assert!(err.to_string().contains("IO error"));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
        let err: DepylerMcpError = json_err.into();
        assert!(matches!(err, DepylerMcpError::Json(_)));
        assert!(err.to_string().contains("JSON error"));
    }

    #[test]
    fn test_type_inference_to_mcp() {
        let err = DepylerMcpError::TypeInferenceError("test".into());
        let mcp_err: McpError = err.into();
        match mcp_err {
            McpError::Internal(msg) => assert!(msg.contains("Type inference failed")),
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_unsafe_pattern_to_mcp() {
        let err = DepylerMcpError::UnsafePatternError {
            pattern: "exec".into(),
            location: "main.py:10".into(),
        };
        let mcp_err: McpError = err.into();
        match mcp_err {
            McpError::Internal(msg) => {
                assert!(msg.contains("Unsafe pattern detected"));
                assert!(msg.contains("exec"));
                assert!(msg.contains("main.py:10"));
            }
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_helper_methods() {
        let err1 = DepylerMcpError::type_inference("test message");
        assert!(matches!(err1, DepylerMcpError::TypeInferenceError(_)));
        
        let err2 = DepylerMcpError::unsafe_pattern("eval", "file.py:10");
        assert!(matches!(err2, DepylerMcpError::UnsafePatternError { .. }));
    }
}

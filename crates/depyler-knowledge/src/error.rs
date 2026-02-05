//! Error types for the Sovereign Type Database.

use thiserror::Error;

/// Result type alias for knowledge operations.
pub type Result<T> = std::result::Result<T, KnowledgeError>;

/// Errors that can occur during type extraction and database operations.
#[derive(Error, Debug)]
pub enum KnowledgeError {
    /// Failed to execute uv command
    #[error("uv command failed: {0}")]
    UvCommandFailed(String),

    /// Package not found or installation failed
    #[error("package not found: {0}")]
    PackageNotFound(String),

    /// Failed to parse Python stub file
    #[error("stub parse error in {file}: {message}")]
    StubParseError { file: String, message: String },

    /// Failed to read/write Parquet database
    #[error("database error: {0}")]
    DatabaseError(String),

    /// Invalid TypeFactKind string
    #[error("invalid type fact kind: {0}")]
    InvalidKind(String),

    /// Query returned no results
    #[error("symbol not found: {module}.{symbol}")]
    SymbolNotFound { module: String, symbol: String },

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Arrow/Parquet error (when parquet-storage feature is enabled)
    #[error("storage error: {0}")]
    Storage(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uv_command_failed_display() {
        let err = KnowledgeError::UvCommandFailed("connection timeout".to_string());
        assert_eq!(err.to_string(), "uv command failed: connection timeout");
    }

    #[test]
    fn test_package_not_found_display() {
        let err = KnowledgeError::PackageNotFound("nonexistent-pkg".to_string());
        assert_eq!(err.to_string(), "package not found: nonexistent-pkg");
    }

    #[test]
    fn test_stub_parse_error_display() {
        let err = KnowledgeError::StubParseError {
            file: "mod.pyi".to_string(),
            message: "syntax error".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "stub parse error in mod.pyi: syntax error"
        );
    }

    #[test]
    fn test_database_error_display() {
        let err = KnowledgeError::DatabaseError("corrupt file".to_string());
        assert_eq!(err.to_string(), "database error: corrupt file");
    }

    #[test]
    fn test_invalid_kind_display() {
        let err = KnowledgeError::InvalidKind("foobar".to_string());
        assert_eq!(err.to_string(), "invalid type fact kind: foobar");
    }

    #[test]
    fn test_symbol_not_found_display() {
        let err = KnowledgeError::SymbolNotFound {
            module: "os".to_string(),
            symbol: "missing".to_string(),
        };
        assert_eq!(err.to_string(), "symbol not found: os.missing");
    }

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let knowledge_err = KnowledgeError::from(io_err);
        assert!(matches!(knowledge_err, KnowledgeError::Io(_)));
        assert!(knowledge_err.to_string().contains("file not found"));
    }

    #[test]
    fn test_storage_error_display() {
        let err = KnowledgeError::Storage("parquet write failed".to_string());
        assert_eq!(err.to_string(), "storage error: parquet write failed");
    }

    #[test]
    fn test_error_is_debug() {
        let err = KnowledgeError::InvalidKind("test".to_string());
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("InvalidKind"));
    }

    #[test]
    fn test_result_type_alias_ok() {
        let result: Result<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_alias_err() {
        let result: Result<i32> = Err(KnowledgeError::DatabaseError("fail".to_string()));
        assert!(result.is_err());
    }
}

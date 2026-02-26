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
        assert_eq!(err.to_string(), "stub parse error in mod.pyi: syntax error");
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
        assert!(result.is_ok());
        assert_eq!(*result.as_ref().unwrap(), 42);
    }

    #[test]
    fn test_result_type_alias_err() {
        let result: Result<i32> = Err(KnowledgeError::DatabaseError("fail".to_string()));
        assert!(result.is_err());
    }

    // ========================================================================
    // S9B7: Coverage tests for error types
    // ========================================================================

    #[test]
    fn test_s9b7_uv_command_failed_debug() {
        let err = KnowledgeError::UvCommandFailed("timeout".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("UvCommandFailed"));
        assert!(debug.contains("timeout"));
    }

    #[test]
    fn test_s9b7_package_not_found_debug() {
        let err = KnowledgeError::PackageNotFound("fake-pkg".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("PackageNotFound"));
    }

    #[test]
    fn test_s9b7_stub_parse_error_fields() {
        let err = KnowledgeError::StubParseError {
            file: "test.pyi".to_string(),
            message: "unexpected token".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("test.pyi"));
        assert!(display.contains("unexpected token"));
    }

    #[test]
    fn test_s9b7_symbol_not_found_fields() {
        let err = KnowledgeError::SymbolNotFound {
            module: "collections".to_string(),
            symbol: "OrderedDict".to_string(),
        };
        assert_eq!(err.to_string(), "symbol not found: collections.OrderedDict");
    }

    #[test]
    fn test_s9b7_storage_error_debug() {
        let err = KnowledgeError::Storage("corruption detected".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("Storage"));
        assert!(debug.contains("corruption detected"));
    }

    #[test]
    fn test_s9b7_io_error_kind_preserved() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let knowledge_err = KnowledgeError::from(io_err);
        match knowledge_err {
            KnowledgeError::Io(ref e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::PermissionDenied);
            }
            _ => panic!("expected Io variant"),
        }
    }

    #[test]
    fn test_s9b7_result_type_with_complex_value() {
        let result: Result<Vec<String>> = Ok(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(result.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_s9b7_error_is_std_error() {
        let err: Box<dyn std::error::Error> =
            Box::new(KnowledgeError::InvalidKind("bad".to_string()));
        assert!(err.to_string().contains("bad"));
    }
}

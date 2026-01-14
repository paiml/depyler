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

    /// Arrow/Parquet error
    #[error("arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    /// Parquet error
    #[error("parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),
}

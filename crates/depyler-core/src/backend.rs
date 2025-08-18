//! Backend trait definitions for multiple transpilation targets

use crate::error::TranspileError;
use crate::hir::HirModule;
use anyhow::Result;
use std::fmt;

/// Validation error types
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}

/// Trait for different transpilation backends
pub trait TranspilationBackend: Send + Sync {
    /// Transpile HIR to target language
    fn transpile(&self, hir: &HirModule) -> Result<String, TranspileError>;
    
    /// Validate generated code
    fn validate_output(&self, code: &str) -> Result<(), ValidationError>;
    
    /// Optimize HIR before transpilation
    fn optimize(&self, hir: &HirModule) -> HirModule {
        hir.clone()
    }
    
    /// Get target name
    fn target_name(&self) -> &str;
    
    /// Get file extension for target language
    fn file_extension(&self) -> &str;
}

/// Transpilation target enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranspilationTarget {
    /// Generate idiomatic Rust code (default)
    Rust,
    
    /// Generate Ruchy script format
    #[cfg(feature = "ruchy")]
    Ruchy,
}

impl Default for TranspilationTarget {
    fn default() -> Self {
        Self::Rust
    }
}

impl fmt::Display for TranspilationTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rust => write!(f, "rust"),
            #[cfg(feature = "ruchy")]
            Self::Ruchy => write!(f, "ruchy"),
        }
    }
}

impl TranspilationTarget {
    /// Parse target from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Some(Self::Rust),
            #[cfg(feature = "ruchy")]
            "ruchy" | "ruc" => Some(Self::Ruchy),
            _ => None,
        }
    }
    
    /// Get file extension for target
    pub fn file_extension(&self) -> &str {
        match self {
            Self::Rust => "rs",
            #[cfg(feature = "ruchy")]
            Self::Ruchy => "ruchy",
        }
    }
}

/// Extended error types for backend operations
impl TranspileError {
    /// Create backend-specific error
    pub fn backend_error(msg: impl Into<String>) -> Self {
        Self::new(crate::error::ErrorKind::CodeGenerationError(msg.into()))
    }
    
    /// Create transformation error
    pub fn transform_error(msg: impl Into<String>) -> Self {
        Self::new(crate::error::ErrorKind::CodeGenerationError(
            format!("Transformation failed: {}", msg.into())
        ))
    }
    
    /// Create optimization error
    pub fn optimization_error(msg: impl Into<String>) -> Self {
        Self::new(crate::error::ErrorKind::InternalError(
            format!("Optimization failed: {}", msg.into())
        ))
    }
}

// Re-export for convenience
pub use crate::error::TranspileError as BackendError;
//! Backend trait definitions for multiple transpilation targets
//!
//! This module provides the extensible backend system introduced in v3.0.0,
//! allowing Depyler to target multiple output languages beyond Rust.
//!
//! # Example Implementation
//!
//! ```rust,ignore
//! use depyler_core::{TranspilationBackend, HirModule, TranspileError, ValidationError};
//!
//! struct MyCustomBackend;
//!
//! impl TranspilationBackend for MyCustomBackend {
//!     fn transpile(&self, hir: &HirModule) -> Result<String, TranspileError> {
//!         // Transform HIR to your target language
//!         Ok("// Generated code".to_string())
//!     }
//!     
//!     fn validate_output(&self, code: &str) -> Result<(), ValidationError> {
//!         // Validate generated code
//!         Ok(())
//!     }
//!     
//!     fn target_name(&self) -> &str {
//!         "custom"
//!     }
//!     
//!     fn file_extension(&self) -> &str {
//!         "custom"
//!     }
//! }
//! ```

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
    #[allow(clippy::result_large_err)]
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
    /// Get file extension for target
    pub fn file_extension(&self) -> &str {
        match self {
            Self::Rust => "rs",
            #[cfg(feature = "ruchy")]
            Self::Ruchy => "ruchy",
        }
    }
}

impl std::str::FromStr for TranspilationTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            #[cfg(feature = "ruchy")]
            "ruchy" | "ruc" => Ok(Self::Ruchy),
            _ => Err(format!("Unknown transpilation target: {}", s)),
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
        Self::new(crate::error::ErrorKind::CodeGenerationError(format!(
            "Transformation failed: {}",
            msg.into()
        )))
    }

    /// Create optimization error
    pub fn optimization_error(msg: impl Into<String>) -> Self {
        Self::new(crate::error::ErrorKind::InternalError(format!(
            "Optimization failed: {}",
            msg.into()
        )))
    }
}

// Re-export for convenience
pub use crate::error::TranspileError as BackendError;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // ============================================================================
    // ValidationError Tests
    // ============================================================================

    #[test]
    fn test_validation_error_invalid_syntax() {
        let err = ValidationError::InvalidSyntax("missing semicolon".to_string());
        assert_eq!(err.to_string(), "Invalid syntax: missing semicolon");
    }

    #[test]
    fn test_validation_error_type_error() {
        let err = ValidationError::TypeError("expected i32, found String".to_string());
        assert_eq!(err.to_string(), "Type error: expected i32, found String");
    }

    #[test]
    fn test_validation_error_unsupported_feature() {
        let err = ValidationError::UnsupportedFeature("async generators".to_string());
        assert_eq!(err.to_string(), "Unsupported feature: async generators");
    }

    // ============================================================================
    // TranspilationTarget Tests
    // ============================================================================

    #[test]
    fn test_transpilation_target_default() {
        let target = TranspilationTarget::default();
        assert_eq!(target, TranspilationTarget::Rust);
    }

    #[test]
    fn test_transpilation_target_display_rust() {
        let target = TranspilationTarget::Rust;
        assert_eq!(target.to_string(), "rust");
    }

    #[test]
    #[cfg(feature = "ruchy")]
    fn test_transpilation_target_display_ruchy() {
        let target = TranspilationTarget::Ruchy;
        assert_eq!(target.to_string(), "ruchy");
    }

    #[test]
    fn test_transpilation_target_file_extension_rust() {
        let target = TranspilationTarget::Rust;
        assert_eq!(target.file_extension(), "rs");
    }

    #[test]
    #[cfg(feature = "ruchy")]
    fn test_transpilation_target_file_extension_ruchy() {
        let target = TranspilationTarget::Ruchy;
        assert_eq!(target.file_extension(), "ruchy");
    }

    #[test]
    fn test_transpilation_target_from_str_rust() {
        let target = TranspilationTarget::from_str("rust").unwrap();
        assert_eq!(target, TranspilationTarget::Rust);
    }

    #[test]
    fn test_transpilation_target_from_str_rs() {
        let target = TranspilationTarget::from_str("rs").unwrap();
        assert_eq!(target, TranspilationTarget::Rust);
    }

    #[test]
    fn test_transpilation_target_from_str_rust_uppercase() {
        let target = TranspilationTarget::from_str("RUST").unwrap();
        assert_eq!(target, TranspilationTarget::Rust);
    }

    #[test]
    #[cfg(feature = "ruchy")]
    fn test_transpilation_target_from_str_ruchy() {
        let target = TranspilationTarget::from_str("ruchy").unwrap();
        assert_eq!(target, TranspilationTarget::Ruchy);
    }

    #[test]
    #[cfg(feature = "ruchy")]
    fn test_transpilation_target_from_str_ruc() {
        let target = TranspilationTarget::from_str("ruc").unwrap();
        assert_eq!(target, TranspilationTarget::Ruchy);
    }

    #[test]
    fn test_transpilation_target_from_str_invalid() {
        let result = TranspilationTarget::from_str("python");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown transpilation target: python");
    }

    #[test]
    fn test_transpilation_target_from_str_empty() {
        let result = TranspilationTarget::from_str("");
        assert!(result.is_err());
    }

    // ============================================================================
    // TranspileError Extension Tests
    // ============================================================================

    #[test]
    fn test_backend_error() {
        let err = TranspileError::backend_error("backend initialization failed");
        // Check error kind
        match err.kind {
            crate::error::ErrorKind::CodeGenerationError(ref msg) => {
                assert_eq!(msg, "backend initialization failed");
            }
            _ => panic!("Expected CodeGenerationError"),
        }
    }

    #[test]
    fn test_transform_error() {
        let err = TranspileError::transform_error("AST transformation failed");
        // Check error kind and message format
        match err.kind {
            crate::error::ErrorKind::CodeGenerationError(ref msg) => {
                assert!(msg.contains("Transformation failed"));
                assert!(msg.contains("AST transformation failed"));
            }
            _ => panic!("Expected CodeGenerationError"),
        }
    }

    #[test]
    fn test_optimization_error() {
        let err = TranspileError::optimization_error("constant folding failed");
        // Check error kind and message format
        match err.kind {
            crate::error::ErrorKind::InternalError(ref msg) => {
                assert!(msg.contains("Optimization failed"));
                assert!(msg.contains("constant folding failed"));
            }
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_backend_error_with_string() {
        let err = TranspileError::backend_error(String::from("dynamic error message"));
        // Check error kind
        match err.kind {
            crate::error::ErrorKind::CodeGenerationError(ref msg) => {
                assert_eq!(msg, "dynamic error message");
            }
            _ => panic!("Expected CodeGenerationError"),
        }
    }

    // ============================================================================
    // TranspilationTarget Trait Coverage
    // ============================================================================

    #[test]
    fn test_transpilation_target_clone() {
        let target = TranspilationTarget::Rust;
        let cloned = target;
        assert_eq!(target, cloned);
    }

    #[test]
    fn test_transpilation_target_debug() {
        let target = TranspilationTarget::Rust;
        let debug_str = format!("{:?}", target);
        assert!(debug_str.contains("Rust"));
    }

    #[test]
    fn test_transpilation_target_eq() {
        let target1 = TranspilationTarget::Rust;
        let target2 = TranspilationTarget::Rust;
        assert_eq!(target1, target2);
    }

    #[test]
    #[cfg(feature = "ruchy")]
    fn test_transpilation_target_ne() {
        let target1 = TranspilationTarget::Rust;
        let target2 = TranspilationTarget::Ruchy;
        assert_ne!(target1, target2);
    }
}

use std::fmt;
use thiserror::Error;

/// Source location information for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Types of transpilation errors
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("Python parse error")]
    ParseError,

    #[error("Unsupported Python feature")]
    UnsupportedFeature(String),

    #[error("Type inference error")]
    TypeInferenceError(String),

    #[error("Invalid type annotation")]
    InvalidTypeAnnotation(String),

    #[error("Type mismatch")]
    TypeMismatch {
        expected: String,
        found: String,
        context: String,
    },

    #[error("Code generation error")]
    CodeGenerationError(String),

    #[error("Verification failed")]
    VerificationError(String),

    #[error("Internal error")]
    InternalError(String),
}

/// Context-aware transpilation error
#[derive(Debug, Error)]
pub struct TranspileError {
    pub kind: ErrorKind,
    pub location: Option<SourceLocation>,
    pub context: Vec<String>,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl TranspileError {
    /// Create a new error with the given kind
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            location: None,
            context: Vec::new(),
            source: None,
        }
    }

    /// Add location information to the error
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Add context to the error
    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context.push(ctx.into());
        self
    }

    /// Add source error
    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for TranspileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write main error message
        write!(f, "{}", self.kind)?;

        // Add location if available
        self.format_location(f)?;

        // Add context if available
        self.format_context_list(f)?;

        Ok(())
    }
}

impl TranspileError {
    /// Format location information if available
    #[inline]
    fn format_location(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, " at {loc}")?;
        }
        Ok(())
    }

    /// Format context list if not empty
    #[inline]
    fn format_context_list(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.context.is_empty() {
            write!(f, "\n\nContext:")?;
            for (i, ctx) in self.context.iter().enumerate() {
                write!(f, "\n  {}. {}", i + 1, ctx)?;
            }
        }
        Ok(())
    }
}

/// Result type alias for transpilation operations
pub type TranspileResult<T> = Result<T, TranspileError>;

/// Extension trait for adding context to Results
pub trait ResultExt<T> {
    #[allow(clippy::result_large_err)]
    fn with_context(self, ctx: impl Into<String>) -> TranspileResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<TranspileError>,
{
    fn with_context(self, ctx: impl Into<String>) -> TranspileResult<T> {
        self.map_err(|e| e.into().with_context(ctx))
    }
}

/// Convert anyhow errors to TranspileError
impl From<anyhow::Error> for TranspileError {
    fn from(err: anyhow::Error) -> Self {
        TranspileError::new(ErrorKind::InternalError(err.to_string()))
    }
}

/// Helper macro for creating errors with context
#[macro_export]
macro_rules! transpile_error {
    ($kind:expr) => {
        $crate::error::TranspileError::new($kind)
    };

    ($kind:expr, $($ctx:expr),+) => {{
        let mut err = $crate::error::TranspileError::new($kind);
        $(
            err = err.with_context($ctx);
        )+
        err
    }};
}

/// Helper macro for bailing with a transpile error
#[macro_export]
macro_rules! transpile_bail {
    ($kind:expr) => {
        return Err($crate::transpile_error!($kind))
    };

    ($kind:expr, $($ctx:expr),+) => {
        return Err($crate::transpile_error!($kind, $($ctx),+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // === SourceLocation tests ===

    #[test]
    fn test_source_location_new() {
        let loc = SourceLocation {
            file: "test.py".to_string(),
            line: 10,
            column: 5,
        };
        assert_eq!(loc.file, "test.py");
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
    }

    #[test]
    fn test_source_location_display() {
        let loc = SourceLocation {
            file: "example.py".to_string(),
            line: 42,
            column: 8,
        };
        assert_eq!(format!("{}", loc), "example.py:42:8");
    }

    #[test]
    fn test_source_location_display_edge_cases() {
        let loc = SourceLocation {
            file: "".to_string(),
            line: 0,
            column: 0,
        };
        assert_eq!(format!("{}", loc), ":0:0");
    }

    #[test]
    fn test_source_location_clone() {
        let loc = SourceLocation {
            file: "test.py".to_string(),
            line: 1,
            column: 1,
        };
        let cloned = loc.clone();
        assert_eq!(loc, cloned);
    }

    #[test]
    fn test_source_location_partial_eq() {
        let loc1 = SourceLocation {
            file: "a.py".to_string(),
            line: 1,
            column: 1,
        };
        let loc2 = SourceLocation {
            file: "a.py".to_string(),
            line: 1,
            column: 1,
        };
        let loc3 = SourceLocation {
            file: "b.py".to_string(),
            line: 1,
            column: 1,
        };
        assert_eq!(loc1, loc2);
        assert_ne!(loc1, loc3);
    }

    #[test]
    fn test_source_location_debug() {
        let loc = SourceLocation {
            file: "test.py".to_string(),
            line: 5,
            column: 10,
        };
        let debug = format!("{:?}", loc);
        assert!(debug.contains("SourceLocation"));
        assert!(debug.contains("test.py"));
    }

    // === ErrorKind tests ===

    #[test]
    fn test_error_kind_parse_error() {
        let err = ErrorKind::ParseError;
        assert_eq!(format!("{}", err), "Python parse error");
    }

    #[test]
    fn test_error_kind_unsupported_feature() {
        let err = ErrorKind::UnsupportedFeature("async generators".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Unsupported Python feature"));
    }

    #[test]
    fn test_error_kind_type_inference_error() {
        let err = ErrorKind::TypeInferenceError("cannot infer type".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Type inference error"));
    }

    #[test]
    fn test_error_kind_invalid_type_annotation() {
        let err = ErrorKind::InvalidTypeAnnotation("List[Unknown]".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Invalid type annotation"));
    }

    #[test]
    fn test_error_kind_type_mismatch() {
        let err = ErrorKind::TypeMismatch {
            expected: "int".to_string(),
            found: "str".to_string(),
            context: "function return".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("Type mismatch"));
    }

    #[test]
    fn test_error_kind_code_generation_error() {
        let err = ErrorKind::CodeGenerationError("failed to generate".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Code generation error"));
    }

    #[test]
    fn test_error_kind_verification_error() {
        let err = ErrorKind::VerificationError("verification failed".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Verification failed"));
    }

    #[test]
    fn test_error_kind_internal_error() {
        let err = ErrorKind::InternalError("unexpected state".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Internal error"));
    }

    #[test]
    fn test_error_kind_debug() {
        let err = ErrorKind::ParseError;
        let debug = format!("{:?}", err);
        assert!(debug.contains("ParseError"));
    }

    // === TranspileError tests ===

    #[test]
    fn test_error_creation() {
        let err = TranspileError::new(ErrorKind::UnsupportedFeature("async/await".to_string()));
        assert!(matches!(err.kind, ErrorKind::UnsupportedFeature(_)));
        assert!(err.location.is_none());
        assert!(err.context.is_empty());
    }

    #[test]
    fn test_error_with_location() {
        let loc = SourceLocation {
            file: "test.py".to_string(),
            line: 10,
            column: 5,
        };

        let err = TranspileError::new(ErrorKind::ParseError).with_location(loc.clone());

        assert_eq!(err.location.unwrap(), loc);
    }

    #[test]
    fn test_error_with_context() {
        let err = TranspileError::new(ErrorKind::TypeInferenceError("unknown type".to_string()))
            .with_context("in function 'add'")
            .with_context("while processing parameter 'x'");

        assert_eq!(err.context.len(), 2);
        assert_eq!(err.context[0], "in function 'add'");
        assert_eq!(err.context[1], "while processing parameter 'x'");
    }

    #[test]
    fn test_error_with_source() {
        let source_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = TranspileError::new(ErrorKind::InternalError("io error".to_string()))
            .with_source(source_err);
        assert!(err.source.is_some());
    }

    #[test]
    fn test_error_display() {
        let loc = SourceLocation {
            file: "example.py".to_string(),
            line: 25,
            column: 10,
        };

        let err = TranspileError::new(ErrorKind::UnsupportedFeature("decorators".to_string()))
            .with_location(loc)
            .with_context("in function 'my_func'")
            .with_context("processing @decorator syntax");

        let display = format!("{err}");
        assert!(display.contains("Unsupported Python feature"));
        assert!(display.contains("example.py:25:10"));
        assert!(display.contains("in function 'my_func'"));
    }

    #[test]
    fn test_error_display_no_location() {
        let err = TranspileError::new(ErrorKind::ParseError);
        let display = format!("{}", err);
        assert!(display.contains("Python parse error"));
        assert!(!display.contains("at "));
    }

    #[test]
    fn test_error_display_no_context() {
        let err = TranspileError::new(ErrorKind::ParseError);
        let display = format!("{}", err);
        assert!(!display.contains("Context:"));
    }

    #[test]
    fn test_error_display_with_numbered_context() {
        let err = TranspileError::new(ErrorKind::ParseError)
            .with_context("first")
            .with_context("second")
            .with_context("third");
        let display = format!("{}", err);
        assert!(display.contains("1. first"));
        assert!(display.contains("2. second"));
        assert!(display.contains("3. third"));
    }

    #[test]
    fn test_error_builder_chain() {
        let loc = SourceLocation {
            file: "chain.py".to_string(),
            line: 1,
            column: 1,
        };
        let source_err = std::io::Error::other("test");

        let err = TranspileError::new(ErrorKind::InternalError("test".to_string()))
            .with_location(loc.clone())
            .with_context("ctx1")
            .with_source(source_err);

        assert_eq!(err.location, Some(loc));
        assert_eq!(err.context.len(), 1);
        assert!(err.source.is_some());
    }

    #[test]
    fn test_error_debug() {
        let err = TranspileError::new(ErrorKind::ParseError);
        let debug = format!("{:?}", err);
        assert!(debug.contains("TranspileError"));
        assert!(debug.contains("ParseError"));
    }

    // === ResultExt tests ===

    #[test]
    fn test_result_ext_ok() {
        let result: Result<i32, TranspileError> = Ok(42);
        let with_ctx = result.with_context("extra context");
        assert_eq!(with_ctx.unwrap(), 42);
    }

    #[test]
    fn test_result_ext_err() {
        let result: Result<i32, TranspileError> =
            Err(TranspileError::new(ErrorKind::ParseError));
        let with_ctx = result.with_context("added context");
        let err = with_ctx.unwrap_err();
        assert_eq!(err.context.len(), 1);
        assert_eq!(err.context[0], "added context");
    }

    // === From<anyhow::Error> tests ===

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("something went wrong");
        let err: TranspileError = anyhow_err.into();
        assert!(matches!(err.kind, ErrorKind::InternalError(_)));
        let display = format!("{}", err);
        assert!(display.contains("Internal error"));
    }

    // === Macro tests ===

    #[test]
    fn test_transpile_error_macro() {
        let err1 = transpile_error!(ErrorKind::ParseError);
        assert!(matches!(err1.kind, ErrorKind::ParseError));

        let err2 = transpile_error!(
            ErrorKind::TypeInferenceError("test".to_string()),
            "context 1",
            "context 2"
        );
        assert_eq!(err2.context.len(), 2);
    }

    #[test]
    fn test_transpile_error_macro_single_context() {
        let err = transpile_error!(ErrorKind::ParseError, "single context");
        assert_eq!(err.context.len(), 1);
        assert_eq!(err.context[0], "single context");
    }

    #[test]
    fn test_transpile_error_macro_many_contexts() {
        let err = transpile_error!(
            ErrorKind::InternalError("test".to_string()),
            "ctx1",
            "ctx2",
            "ctx3",
            "ctx4"
        );
        assert_eq!(err.context.len(), 4);
    }

    #[allow(clippy::result_large_err)]
    fn bail_test_helper() -> TranspileResult<()> {
        transpile_bail!(ErrorKind::ParseError);
    }

    #[test]
    fn test_transpile_bail_macro() {
        let result = bail_test_helper();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err.kind, ErrorKind::ParseError));
    }

    #[allow(clippy::result_large_err)]
    fn bail_test_with_context() -> TranspileResult<()> {
        transpile_bail!(ErrorKind::ParseError, "context1", "context2");
    }

    #[test]
    fn test_transpile_bail_macro_with_context() {
        let result = bail_test_with_context();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.context.len(), 2);
    }

    // === Type alias tests ===

    #[test]
    fn test_transpile_result_ok() {
        let result: TranspileResult<i32> = Ok(42);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_result_err() {
        let result: TranspileResult<i32> = Err(TranspileError::new(ErrorKind::ParseError));
        assert!(result.is_err());
    }

    // === Edge cases ===

    #[test]
    fn test_empty_strings_in_error_kind() {
        let err = ErrorKind::UnsupportedFeature("".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Unsupported Python feature"));
    }

    #[test]
    fn test_type_mismatch_all_empty() {
        let err = ErrorKind::TypeMismatch {
            expected: "".to_string(),
            found: "".to_string(),
            context: "".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("Type mismatch"));
    }

    #[test]
    fn test_context_with_special_chars() {
        let err = TranspileError::new(ErrorKind::ParseError)
            .with_context("context with 'quotes' and \"double quotes\"")
            .with_context("context\nwith\nnewlines");
        assert_eq!(err.context.len(), 2);
    }

    #[test]
    fn test_long_context_chain() {
        let mut err = TranspileError::new(ErrorKind::ParseError);
        for i in 0..100 {
            err = err.with_context(format!("context {}", i));
        }
        assert_eq!(err.context.len(), 100);
    }
}

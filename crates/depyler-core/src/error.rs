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
        if let Some(loc) = &self.location {
            write!(f, " at {}", loc)?;
        }

        // Add context if available
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

        let display = format!("{}", err);
        assert!(display.contains("Unsupported Python feature"));
        assert!(display.contains("example.py:25:10"));
        assert!(display.contains("in function 'my_func'"));
    }

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
}


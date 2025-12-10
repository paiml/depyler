//! DEPYLER-0349: error.rs Coverage Tests
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: error.rs 65-70% → 90%+ coverage
//! TDG Score: 95.45 (A+) - Excellent quality error handling infrastructure
//!
//! This test suite validates error handling functionality:
//! - SourceLocation Display implementation
//! - All ErrorKind variants
//! - with_source() method
//! - ResultExt trait
//! - From<anyhow::Error> conversion
//! - transpile_bail! macro
//! - TypeMismatch variant
//! - Edge cases

#![allow(non_snake_case)]

use depyler_core::error::*;

// ============================================================================
// SOURCE LOCATION TESTS
// ============================================================================

#[test]
fn test_depyler_0349_source_location_display() {
    let loc = SourceLocation {
        file: "main.py".to_string(),
        line: 42,
        column: 15,
    };

    let display = format!("{}", loc);
    assert_eq!(display, "main.py:42:15");
}

#[test]
fn test_depyler_0349_source_location_clone_eq() {
    let loc1 = SourceLocation {
        file: "test.py".to_string(),
        line: 10,
        column: 5,
    };

    let loc2 = loc1.clone();
    assert_eq!(loc1, loc2);
}

#[test]
fn test_depyler_0349_source_location_debug() {
    let loc = SourceLocation {
        file: "example.py".to_string(),
        line: 1,
        column: 1,
    };

    let debug = format!("{:?}", loc);
    assert!(debug.contains("SourceLocation"));
    assert!(debug.contains("example.py"));
}

// ============================================================================
// ERROR KIND VARIANT TESTS
// ============================================================================

#[test]
fn test_depyler_0349_error_kind_parse_error() {
    let err = TranspileError::new(ErrorKind::ParseError);
    let display = format!("{}", err);
    assert!(display.contains("Python parse error"));
}

#[test]
fn test_depyler_0349_error_kind_unsupported_feature() {
    let err = TranspileError::new(ErrorKind::UnsupportedFeature("generators".to_string()));
    let display = format!("{}", err);
    assert!(display.contains("Unsupported Python feature"));
}

#[test]
fn test_depyler_0349_error_kind_type_inference() {
    let err = TranspileError::new(ErrorKind::TypeInferenceError(
        "cannot infer type for variable 'x'".to_string(),
    ));
    let display = format!("{}", err);
    assert!(display.contains("Type inference error"));
}

#[test]
fn test_depyler_0349_error_kind_invalid_type_annotation() {
    let err = TranspileError::new(ErrorKind::InvalidTypeAnnotation(
        "List[int, str]".to_string(),
    ));
    let display = format!("{}", err);
    assert!(display.contains("Invalid type annotation"));
}

#[test]
fn test_depyler_0349_error_kind_type_mismatch() {
    let err = TranspileError::new(ErrorKind::TypeMismatch {
        expected: "int".to_string(),
        found: "str".to_string(),
        context: "function parameter".to_string(),
    });
    let display = format!("{}", err);
    assert!(display.contains("Type mismatch"));
}

#[test]
fn test_depyler_0349_error_kind_code_generation() {
    let err = TranspileError::new(ErrorKind::CodeGenerationError(
        "failed to generate function body".to_string(),
    ));
    let display = format!("{}", err);
    assert!(display.contains("Code generation error"));
}

#[test]
fn test_depyler_0349_error_kind_verification() {
    let err = TranspileError::new(ErrorKind::VerificationError(
        "generated code does not compile".to_string(),
    ));
    let display = format!("{}", err);
    assert!(display.contains("Verification failed"));
}

#[test]
fn test_depyler_0349_error_kind_internal() {
    let err = TranspileError::new(ErrorKind::InternalError(
        "unexpected compiler state".to_string(),
    ));
    let display = format!("{}", err);
    assert!(display.contains("Internal error"));
}

// ============================================================================
// WITH_SOURCE METHOD TESTS
// ============================================================================

#[test]
fn test_depyler_0349_with_source_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = TranspileError::new(ErrorKind::ParseError).with_source(io_err);

    assert!(err.source.is_some());
}

#[test]
fn test_depyler_0349_with_source_custom_error() {
    #[derive(Debug)]
    struct CustomError(String);

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Custom: {}", self.0)
        }
    }

    impl std::error::Error for CustomError {}

    let custom_err = CustomError("test error".to_string());
    let err = TranspileError::new(ErrorKind::InternalError("wrapped".to_string()))
        .with_source(custom_err);

    assert!(err.source.is_some());
}

#[test]
fn test_depyler_0349_with_source_and_context() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let err = TranspileError::new(ErrorKind::ParseError)
        .with_source(io_err)
        .with_context("reading source file")
        .with_context("in module 'main'");

    assert!(err.source.is_some());
    assert_eq!(err.context.len(), 2);
}

// ============================================================================
// RESULT EXT TRAIT TESTS
// ============================================================================

#[test]
fn test_depyler_0349_result_ext_ok() {
    let result: Result<i32, TranspileError> = Ok(42);
    let with_ctx = result.with_context("test context");

    assert!(with_ctx.is_ok());
    assert_eq!(with_ctx.unwrap(), 42);
}

#[test]
fn test_depyler_0349_result_ext_err() {
    let result: Result<i32, TranspileError> = Err(TranspileError::new(ErrorKind::ParseError));
    let with_ctx = result.with_context("while parsing function");

    assert!(with_ctx.is_err());
    let err = with_ctx.unwrap_err();
    assert_eq!(err.context.len(), 1);
    assert_eq!(err.context[0], "while parsing function");
}

#[test]
fn test_depyler_0349_result_ext_chain_context() {
    let result: Result<String, TranspileError> = Err(TranspileError::new(
        ErrorKind::TypeInferenceError("unknown".to_string()),
    ));

    let with_ctx = result
        .with_context("in function 'foo'")
        .with_context("processing parameter 'x'")
        .with_context("at module level");

    assert!(with_ctx.is_err());
    let err = with_ctx.unwrap_err();
    assert_eq!(err.context.len(), 3);
}

// ============================================================================
// FROM ANYHOW ERROR TESTS
// ============================================================================

#[test]
fn test_depyler_0349_from_anyhow_error() {
    let anyhow_err = anyhow::anyhow!("something went wrong");
    let transpile_err: TranspileError = anyhow_err.into();

    assert!(matches!(transpile_err.kind, ErrorKind::InternalError(_)));
}

#[test]
fn test_depyler_0349_from_anyhow_with_context() {
    let anyhow_err = anyhow::anyhow!("database connection failed");
    let transpile_err: TranspileError = anyhow_err.into();
    let with_ctx = transpile_err.with_context("loading configuration");

    let display = format!("{}", with_ctx);
    assert!(display.contains("Internal error"));
    assert!(display.contains("loading configuration"));
}

// ============================================================================
// TRANSPILE_BAIL MACRO TESTS
// ============================================================================

#[test]
fn test_depyler_0349_transpile_bail_basic() {
    #[allow(clippy::result_large_err)]
    fn test_fn() -> TranspileResult<i32> {
        depyler_core::transpile_bail!(ErrorKind::ParseError);
    }

    let result = test_fn();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err().kind, ErrorKind::ParseError));
}

#[test]
fn test_depyler_0349_transpile_bail_with_context() {
    #[allow(clippy::result_large_err)]
    fn test_fn() -> TranspileResult<String> {
        depyler_core::transpile_bail!(
            ErrorKind::TypeInferenceError("type mismatch".to_string()),
            "in function 'main'",
            "processing return type"
        );
    }

    let result = test_fn();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.context.len(), 2);
    assert_eq!(err.context[0], "in function 'main'");
}

// ============================================================================
// TYPE MISMATCH VARIANT TESTS
// ============================================================================

#[test]
fn test_depyler_0349_type_mismatch_all_fields() {
    let err = TranspileError::new(ErrorKind::TypeMismatch {
        expected: "List[int]".to_string(),
        found: "Dict[str, int]".to_string(),
        context: "assignment statement".to_string(),
    });

    let display = format!("{}", err);
    assert!(display.contains("Type mismatch"));
}

#[test]
fn test_depyler_0349_type_mismatch_with_location() {
    let loc = SourceLocation {
        file: "types.py".to_string(),
        line: 15,
        column: 8,
    };

    let err = TranspileError::new(ErrorKind::TypeMismatch {
        expected: "int".to_string(),
        found: "float".to_string(),
        context: "arithmetic operation".to_string(),
    })
    .with_location(loc);

    let display = format!("{}", err);
    assert!(display.contains("types.py:15:8"));
    assert!(display.contains("Type mismatch"));
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_depyler_0349_empty_context_not_displayed() {
    let err = TranspileError::new(ErrorKind::ParseError);
    let display = format!("{}", err);

    // Should not contain "Context:" section when context is empty
    assert!(!display.contains("Context:"));
}

#[test]
fn test_depyler_0349_location_none_not_displayed() {
    let err = TranspileError::new(ErrorKind::CodeGenerationError("test".to_string()));
    let display = format!("{}", err);

    // Should not show location info when None
    assert!(!display.contains(" at "));
}

#[test]
fn test_depyler_0349_multiple_context_items() {
    let err = TranspileError::new(ErrorKind::VerificationError("test".to_string()))
        .with_context("context 1")
        .with_context("context 2")
        .with_context("context 3")
        .with_context("context 4");

    let display = format!("{}", err);
    assert!(display.contains("Context:"));
    assert!(display.contains("1. context 1"));
    assert!(display.contains("2. context 2"));
    assert!(display.contains("3. context 3"));
    assert!(display.contains("4. context 4"));
}

#[test]
fn test_depyler_0349_full_error_all_features() {
    let loc = SourceLocation {
        file: "complex.py".to_string(),
        line: 100,
        column: 50,
    };

    let io_err = std::io::Error::new(std::io::ErrorKind::InvalidData, "malformed input");

    let err = TranspileError::new(ErrorKind::TypeInferenceError("ambiguous type".to_string()))
        .with_location(loc)
        .with_context("in class 'MyClass'")
        .with_context("in method 'process'")
        .with_source(io_err);

    let display = format!("{}", err);
    assert!(display.contains("Type inference error"));
    assert!(display.contains("complex.py:100:50"));
    assert!(display.contains("Context:"));
    assert!(display.contains("in class 'MyClass'"));
    assert!(display.contains("in method 'process'"));
}

// ============================================================================
// PROPERTY TESTS - Error Handling Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_source_location_display_format(
            line in 1usize..10000,
            column in 1usize..200,
        ) {
            let loc = SourceLocation {
                file: "test.py".to_string(),
                line,
                column,
            };

            let display = format!("{}", loc);
            assert!(display.contains(&format!("{}:{}", line, column)));
        }

        #[test]
        fn prop_context_always_appends(
            ctx_count in 1usize..20,
        ) {
            let mut err = TranspileError::new(ErrorKind::ParseError);

            for i in 0..ctx_count {
                err = err.with_context(format!("context {}", i));
            }

            assert_eq!(err.context.len(), ctx_count);
        }

        #[test]
        fn prop_error_display_never_panics(
            line in 0usize..1000,
            column in 0usize..100,
        ) {
            let loc = SourceLocation {
                file: "prop.py".to_string(),
                line,
                column,
            };

            let err = TranspileError::new(ErrorKind::ParseError)
                .with_location(loc)
                .with_context("property test");

            // Should never panic on display
            let _display = format!("{}", err);
        }
    }
}

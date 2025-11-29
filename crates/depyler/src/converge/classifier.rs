//! Error classification for convergence loop
//!
//! Wraps the oracle to classify compilation errors by category and root cause.

use super::compiler::{CompilationError, CompilationResult};

/// Category of compilation error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Gap in transpiler (missing feature, incorrect codegen)
    TranspilerGap,
    /// Gap in model (incorrect pattern matching)
    ModelGap,
    /// User code issue (not transpiler's fault)
    UserError,
    /// Unknown category
    Unknown,
}

/// Classification result for a single error
#[derive(Debug, Clone)]
pub struct ErrorClassification {
    /// The original error
    pub error: CompilationError,
    /// Category of the error
    pub category: ErrorCategory,
    /// Subcategory for more specific classification
    pub subcategory: String,
    /// Confidence of classification (0.0-1.0)
    pub confidence: f64,
}

/// Classifier for compilation errors
pub struct ErrorClassifier {
    // Will hold oracle configuration
}

impl ErrorClassifier {
    /// Create a new error classifier
    pub fn new() -> Self {
        Self {}
    }

    /// Classify a single compilation error
    pub fn classify(&self, error: &CompilationError) -> ErrorClassification {
        let (category, subcategory, confidence) = match error.code.as_str() {
            // E0599: No method found - usually missing stdlib mapping
            "E0599" => (
                ErrorCategory::TranspilerGap,
                "missing_method".to_string(),
                0.9,
            ),

            // E0308: Type mismatch - type inference issue
            "E0308" => (
                ErrorCategory::TranspilerGap,
                "type_inference".to_string(),
                0.85,
            ),

            // E0277: Trait not implemented - missing trait bound
            "E0277" => (
                ErrorCategory::TranspilerGap,
                "missing_trait".to_string(),
                0.8,
            ),

            // E0425: Cannot find value - undefined variable
            "E0425" => (
                ErrorCategory::TranspilerGap,
                "undefined_variable".to_string(),
                0.75,
            ),

            // E0433: Failed to resolve - missing import
            "E0433" => (
                ErrorCategory::TranspilerGap,
                "missing_import".to_string(),
                0.85,
            ),

            // E0382: Use after move - borrow checker issue
            "E0382" => (
                ErrorCategory::TranspilerGap,
                "borrow_checker".to_string(),
                0.7,
            ),

            // E0502: Cannot borrow - borrow checker issue
            "E0502" => (
                ErrorCategory::TranspilerGap,
                "borrow_checker".to_string(),
                0.7,
            ),

            // E0507: Cannot move out of borrowed - borrow checker issue
            "E0507" => (
                ErrorCategory::TranspilerGap,
                "borrow_checker".to_string(),
                0.7,
            ),

            // Unknown error codes
            _ => (ErrorCategory::Unknown, "unknown".to_string(), 0.5),
        };

        ErrorClassification {
            error: error.clone(),
            category,
            subcategory,
            confidence,
        }
    }

    /// Classify all errors from compilation results
    pub fn classify_all(&self, results: &[CompilationResult]) -> Vec<ErrorClassification> {
        results
            .iter()
            .flat_map(|r| r.errors.iter())
            .map(|e| self.classify(e))
            .collect()
    }
}

impl Default for ErrorClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_classify_e0599() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "no method named `contains_key`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 10,
            column: 5,
        };

        let classification = classifier.classify(&error);
        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert_eq!(classification.subcategory, "missing_method");
        assert!(classification.confidence > 0.8);
    }

    #[test]
    fn test_classify_e0308() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "expected `i32`, found `i64`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 20,
            column: 10,
        };

        let classification = classifier.classify(&error);
        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert_eq!(classification.subcategory, "type_inference");
    }
}

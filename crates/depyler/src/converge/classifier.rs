//! Error classification for convergence loop (Issue #172)
//!
//! Integrates depyler_oracle for ML-based error classification and
//! OracleQueryLoop for pattern-based fix suggestions.

use super::compiler::{CompilationError, CompilationResult};
use depyler_oracle::{
    ErrorCategory as OracleCategory, Oracle, OracleQueryLoop, OracleSuggestion,
    QueryLoopConfig, RustErrorCode, ErrorContext,
};
use std::sync::OnceLock;

/// Lazily initialized Oracle singleton for ML classification
static ORACLE: OnceLock<Option<Oracle>> = OnceLock::new();

/// Get or initialize the Oracle singleton
fn get_oracle() -> Option<&'static Oracle> {
    ORACLE
        .get_or_init(|| {
            match Oracle::load_or_train() {
                Ok(oracle) => Some(oracle),
                Err(e) => {
                    tracing::warn!("Failed to load oracle: {e}. Using fallback classification.");
                    None
                }
            }
        })
        .as_ref()
}

/// Category of compilation error (converge-level taxonomy)
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

/// Map Oracle's specific category to converge's higher-level taxonomy
fn map_oracle_category(oracle_cat: OracleCategory) -> (ErrorCategory, String) {
    match oracle_cat {
        OracleCategory::TypeMismatch => (ErrorCategory::TranspilerGap, "type_inference".into()),
        OracleCategory::BorrowChecker => (ErrorCategory::TranspilerGap, "borrow_checker".into()),
        OracleCategory::MissingImport => (ErrorCategory::TranspilerGap, "missing_import".into()),
        OracleCategory::SyntaxError => (ErrorCategory::TranspilerGap, "syntax".into()),
        OracleCategory::LifetimeError => (ErrorCategory::TranspilerGap, "lifetime".into()),
        OracleCategory::TraitBound => (ErrorCategory::TranspilerGap, "trait_bound".into()),
        OracleCategory::Other => (ErrorCategory::Unknown, "unknown".into()),
    }
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
    /// Suggested fix from Oracle (if available)
    pub suggested_fix: Option<String>,
}

/// Classifier for compilation errors using ML Oracle
pub struct ErrorClassifier {
    /// Optional OracleQueryLoop for pattern-based fixes
    query_loop: Option<OracleQueryLoop>,
}

impl ErrorClassifier {
    /// Create a new error classifier with Oracle integration
    pub fn new() -> Self {
        // Try to load the query loop with patterns
        let query_loop = Self::init_query_loop();
        Self { query_loop }
    }

    /// Initialize OracleQueryLoop with default patterns
    fn init_query_loop() -> Option<OracleQueryLoop> {
        let config = QueryLoopConfig {
            threshold: 0.7,
            max_suggestions: 3,
            boost_recent: true,
            max_retries: 3,
            llm_fallback: false,
        };
        let mut loop_instance = OracleQueryLoop::with_config(config);

        // Try to load patterns from default path
        let pattern_path = OracleQueryLoop::default_pattern_path();
        if pattern_path.exists() {
            if let Err(e) = loop_instance.load(&pattern_path) {
                tracing::debug!("No patterns loaded: {e}");
            }
        }

        Some(loop_instance)
    }

    /// Classify a single compilation error using ML Oracle
    pub fn classify(&self, error: &CompilationError) -> ErrorClassification {
        // Try ML classification first
        if let Some(oracle) = get_oracle() {
            if let Ok(result) = oracle.classify_message(&error.message) {
                let (category, subcategory) = map_oracle_category(result.category);
                return ErrorClassification {
                    error: error.clone(),
                    category,
                    subcategory,
                    confidence: result.confidence as f64,
                    suggested_fix: result.suggested_fix,
                };
            }
        }

        // Fallback to rule-based classification
        self.classify_fallback(error)
    }

    /// Fallback rule-based classification (original hardcoded logic)
    fn classify_fallback(&self, error: &CompilationError) -> ErrorClassification {
        let (category, subcategory, confidence) = match error.code.as_str() {
            "E0599" => (ErrorCategory::TranspilerGap, "missing_method".into(), 0.9),
            "E0308" => (ErrorCategory::TranspilerGap, "type_inference".into(), 0.85),
            "E0277" => (ErrorCategory::TranspilerGap, "trait_bound".into(), 0.8),
            "E0425" => (ErrorCategory::TranspilerGap, "undefined_variable".into(), 0.75),
            "E0433" => (ErrorCategory::TranspilerGap, "missing_import".into(), 0.85),
            "E0432" => (ErrorCategory::TranspilerGap, "unresolved_import".into(), 0.85),
            "E0382" => (ErrorCategory::TranspilerGap, "borrow_checker".into(), 0.7),
            "E0502" => (ErrorCategory::TranspilerGap, "borrow_checker".into(), 0.7),
            "E0507" => (ErrorCategory::TranspilerGap, "borrow_checker".into(), 0.7),
            "E0597" => (ErrorCategory::TranspilerGap, "lifetime".into(), 0.7),
            "E0716" => (ErrorCategory::TranspilerGap, "lifetime".into(), 0.7),
            _ => (ErrorCategory::Unknown, "unknown".into(), 0.5),
        };

        ErrorClassification {
            error: error.clone(),
            category,
            subcategory,
            confidence,
            suggested_fix: None,
        }
    }

    /// Get fix suggestions from OracleQueryLoop for an error
    pub fn get_suggestions(&mut self, error: &CompilationError) -> Vec<OracleSuggestion> {
        let query_loop = match &mut self.query_loop {
            Some(ql) => ql,
            None => return Vec::new(),
        };

        // Parse error code
        let error_code = match error.code.parse::<RustErrorCode>() {
            Ok(code) => code,
            Err(_) => return Vec::new(),
        };

        // Build error context
        let context = ErrorContext {
            file: error.file.clone(),
            line: error.line,
            column: error.column,
            source_snippet: String::new(), // Could extract from file
            surrounding_lines: Vec::new(),
        };

        query_loop.suggest(error_code, &error.message, &context)
    }

    /// Classify all errors from compilation results
    pub fn classify_all(&self, results: &[CompilationResult]) -> Vec<ErrorClassification> {
        results
            .iter()
            .flat_map(|r| r.errors.iter())
            .map(|e| self.classify(e))
            .collect()
    }

    /// Get Oracle statistics (if query loop is active)
    pub fn stats(&self) -> Option<&depyler_oracle::OracleStats> {
        self.query_loop.as_ref().map(|ql| ql.stats())
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
    fn test_classify_e0599_fallback() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "no method named `contains_key`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 10,
            column: 5,
        };

        // Use fallback directly to test rule-based logic
        let classification = classifier.classify_fallback(&error);
        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert_eq!(classification.subcategory, "missing_method");
        assert!(classification.confidence > 0.8);
    }

    #[test]
    fn test_classify_e0308_fallback() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "expected `i32`, found `i64`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 20,
            column: 10,
        };

        let classification = classifier.classify_fallback(&error);
        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert_eq!(classification.subcategory, "type_inference");
    }

    #[test]
    fn test_classify_e0277_fallback() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E0277".to_string(),
            message: "the trait bound `Foo: Clone` is not satisfied".to_string(),
            file: PathBuf::from("test.rs"),
            line: 30,
            column: 15,
        };

        let classification = classifier.classify_fallback(&error);
        assert_eq!(classification.category, ErrorCategory::TranspilerGap);
        assert_eq!(classification.subcategory, "trait_bound");
    }

    #[test]
    fn test_classify_unknown_code() {
        let classifier = ErrorClassifier::new();
        let error = CompilationError {
            code: "E9999".to_string(),
            message: "unknown error".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
        };

        let classification = classifier.classify_fallback(&error);
        assert_eq!(classification.category, ErrorCategory::Unknown);
    }

    #[test]
    fn test_map_oracle_category() {
        assert_eq!(
            map_oracle_category(OracleCategory::TypeMismatch),
            (ErrorCategory::TranspilerGap, "type_inference".into())
        );
        assert_eq!(
            map_oracle_category(OracleCategory::BorrowChecker),
            (ErrorCategory::TranspilerGap, "borrow_checker".into())
        );
        assert_eq!(
            map_oracle_category(OracleCategory::Other),
            (ErrorCategory::Unknown, "unknown".into())
        );
    }

    #[test]
    fn test_classifier_default() {
        let classifier = ErrorClassifier::default();
        // Just verify it creates without panic
        assert!(classifier.query_loop.is_some());
    }
}

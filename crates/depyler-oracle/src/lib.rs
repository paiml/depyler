//! ML-powered compile error classification and auto-fixing.
//!
//! Uses aprender models (Decision Tree, N-gram, Random Forest) to:
//! - Classify transpilation errors into actionable categories
//! - Suggest fixes based on historical patterns
//! - Detect error drift requiring model retraining

use std::collections::HashMap;
use std::path::Path;

use aprender::format::{self, ModelType, SaveOptions};
use aprender::metrics::drift::{DriftConfig, DriftDetector, DriftStatus};
use aprender::primitives::Matrix;
use aprender::tree::DecisionTreeClassifier;
use serde::{Deserialize, Serialize};

pub mod classifier;
pub mod features;
pub mod ngram;
pub mod patterns;
pub mod tfidf;
pub mod training;

#[cfg(test)]
mod proptests;

pub use classifier::{ErrorClassifier, ErrorCategory};
pub use features::ErrorFeatures;
pub use ngram::{FixPattern, FixSuggestion, NgramFixPredictor};
pub use patterns::{CodeTransform, FixTemplate, FixTemplateRegistry};
pub use tfidf::{CombinedFeatureExtractor, TfidfConfig, TfidfFeatureExtractor};
pub use training::{TrainingDataset, TrainingSample};

/// Error types for the oracle.
#[derive(Debug, thiserror::Error)]
pub enum OracleError {
    /// Model loading/saving error
    #[error("Model error: {0}")]
    Model(String),
    /// Feature extraction error
    #[error("Feature extraction error: {0}")]
    Feature(String),
    /// Classification error
    #[error("Classification error: {0}")]
    Classification(String),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for oracle operations.
pub type Result<T> = std::result::Result<T, OracleError>;

/// Classification result with confidence and suggested fix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// Predicted error category
    pub category: ErrorCategory,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Suggested fix template
    pub suggested_fix: Option<String>,
    /// Related error patterns
    pub related_patterns: Vec<String>,
}

/// Oracle for compile error prediction and fixing.
///
/// # Examples
///
/// ```ignore
/// use depyler_oracle::{Oracle, ErrorFeatures};
///
/// let oracle = Oracle::load("model.apr")?;
/// let features = ErrorFeatures::from_error_message("type mismatch: expected i32, found &str");
/// let result = oracle.classify(&features)?;
/// println!("Category: {:?}, Confidence: {}", result.category, result.confidence);
/// ```
pub struct Oracle {
    /// Decision tree classifier
    classifier: DecisionTreeClassifier,
    /// Category mappings
    categories: Vec<ErrorCategory>,
    /// Fix templates per category
    fix_templates: HashMap<ErrorCategory, Vec<String>>,
    /// Drift detector for retraining triggers
    drift_detector: DriftDetector,
    /// Historical performance scores
    performance_history: Vec<f32>,
}

impl Oracle {
    /// Create a new oracle with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            classifier: DecisionTreeClassifier::new()
                .with_max_depth(10),
            categories: vec![
                ErrorCategory::TypeMismatch,
                ErrorCategory::BorrowChecker,
                ErrorCategory::MissingImport,
                ErrorCategory::SyntaxError,
                ErrorCategory::LifetimeError,
                ErrorCategory::TraitBound,
                ErrorCategory::Other,
            ],
            fix_templates: Self::default_fix_templates(),
            drift_detector: DriftDetector::new(
                DriftConfig::default()
                    .with_min_samples(10)
                    .with_window_size(50),
            ),
            performance_history: Vec::new(),
        }
    }

    /// Default fix templates for each category.
    fn default_fix_templates() -> HashMap<ErrorCategory, Vec<String>> {
        let mut templates = HashMap::new();

        templates.insert(
            ErrorCategory::TypeMismatch,
            vec![
                "Convert type using `.into()` or `as`".to_string(),
                "Check function signature for expected type".to_string(),
                "Use type annotation to clarify".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::BorrowChecker,
            vec![
                "Clone the value instead of borrowing".to_string(),
                "Use a reference (&) instead of moving".to_string(),
                "Introduce a scope to limit borrow lifetime".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::MissingImport,
            vec![
                "Add `use` statement for the missing type".to_string(),
                "Check crate dependencies in Cargo.toml".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::SyntaxError,
            vec![
                "Check for missing semicolons or braces".to_string(),
                "Verify function/struct syntax".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::LifetimeError,
            vec![
                "Add explicit lifetime annotation".to_string(),
                "Use 'static lifetime for owned data".to_string(),
                "Consider using Rc/Arc for shared ownership".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::TraitBound,
            vec![
                "Implement the required trait".to_string(),
                "Add trait bound to generic parameter".to_string(),
                "Use a wrapper type that implements the trait".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::Other,
            vec!["Review the full error message for specifics".to_string()],
        );

        templates
    }

    /// Train the oracle on labeled error data.
    ///
    /// # Errors
    ///
    /// Returns error if training fails.
    pub fn train(&mut self, features: &Matrix<f32>, labels: &[usize]) -> Result<()> {
        self.classifier
            .fit(features, labels)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(())
    }

    /// Classify an error based on its features.
    pub fn classify(&self, features: &ErrorFeatures) -> Result<ClassificationResult> {
        let feature_matrix = features.to_matrix();
        let predictions = self.classifier.predict(&feature_matrix);

        if predictions.is_empty() {
            return Err(OracleError::Classification(
                "No prediction produced".to_string(),
            ));
        }

        let pred_idx = predictions.as_slice()[0];
        let category = self.categories.get(pred_idx).copied().unwrap_or(ErrorCategory::Other);

        let suggested_fix = self
            .fix_templates
            .get(&category)
            .and_then(|fixes| fixes.first().cloned());

        let related = self
            .fix_templates
            .get(&category)
            .map(|fixes| fixes.iter().skip(1).cloned().collect())
            .unwrap_or_default();

        Ok(ClassificationResult {
            category,
            confidence: 0.85, // TODO: Extract from tree probabilities
            suggested_fix,
            related_patterns: related,
        })
    }

    /// Check if the model needs retraining based on performance drift.
    pub fn check_drift(&mut self, recent_accuracy: f32) -> DriftStatus {
        self.performance_history.push(recent_accuracy);

        if self.performance_history.len() < 10 {
            return DriftStatus::NoDrift;
        }

        let baseline: Vec<f32> = self.performance_history[..self.performance_history.len() / 2].to_vec();
        let current: Vec<f32> = self.performance_history[self.performance_history.len() / 2..].to_vec();

        self.drift_detector.detect_performance_drift(&baseline, &current)
    }

    /// Save the oracle model to a file.
    ///
    /// # Errors
    ///
    /// Returns error if saving fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let options = SaveOptions::default()
            .with_name("depyler-oracle")
            .with_description("Error classification model for Depyler transpiler");

        format::save(&self.classifier, ModelType::DecisionTree, path, options)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(())
    }

    /// Load an oracle model from a file.
    ///
    /// # Errors
    ///
    /// Returns error if loading fails.
    pub fn load(path: &Path) -> Result<Self> {
        let classifier: DecisionTreeClassifier =
            format::load(path, ModelType::DecisionTree)
                .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(Self {
            classifier,
            ..Self::new()
        })
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_creation() {
        let oracle = Oracle::new();
        assert_eq!(oracle.categories.len(), 7);
    }

    #[test]
    fn test_fix_templates() {
        let oracle = Oracle::new();
        assert!(oracle.fix_templates.contains_key(&ErrorCategory::TypeMismatch));
        assert!(oracle.fix_templates.contains_key(&ErrorCategory::BorrowChecker));
    }

    #[test]
    fn test_drift_detection_insufficient_data() {
        let mut oracle = Oracle::new();
        let status = oracle.check_drift(0.95);
        assert!(matches!(status, DriftStatus::NoDrift));
    }
}

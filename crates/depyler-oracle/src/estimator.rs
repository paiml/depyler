//! Aprender Estimator trait implementation for oracle predictor.
//!
//! Wraps NgramFixPredictor to work with aprender's model evaluation framework.

use aprender::error::{AprenderError, Result as AprenderResult};
use aprender::primitives::{Matrix, Vector};
use aprender::traits::Estimator;

use crate::classifier::ErrorCategory;
use crate::features::ErrorFeatures;
use crate::ngram::NgramFixPredictor;
use crate::training::TrainingSample;

/// Wrapper to make NgramFixPredictor compatible with aprender's Estimator trait.
///
/// Converts between text-based error classification and numeric features/labels.
pub struct OracleEstimator {
    predictor: NgramFixPredictor,
    /// Training samples (stored for fit)
    samples: Vec<TrainingSample>,
    /// Feature dimension (vocabulary size after fit)
    n_features: usize,
}

impl Clone for OracleEstimator {
    fn clone(&self) -> Self {
        // Create fresh predictor (NgramFixPredictor doesn't impl Clone)
        let mut new = Self::new();
        new.samples = self.samples.clone();
        new.n_features = self.n_features;
        new
    }
}

impl OracleEstimator {
    /// Create a new oracle estimator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            predictor: NgramFixPredictor::new(),
            samples: Vec::new(),
            n_features: 0,
        }
    }

    /// Create with custom similarity threshold.
    #[must_use]
    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.predictor = self.predictor.with_min_similarity(threshold);
        self
    }

    /// Add training samples.
    pub fn add_samples(&mut self, samples: Vec<TrainingSample>) {
        self.samples.extend(samples);
    }

    /// Get the underlying predictor.
    #[must_use]
    pub fn predictor(&self) -> &NgramFixPredictor {
        &self.predictor
    }

    /// Predict category for a single error message.
    #[must_use]
    pub fn predict_category(&self, error_msg: &str) -> Option<ErrorCategory> {
        self.predictor
            .predict_fixes(error_msg, 1)
            .first()
            .map(|s| s.category)
    }
}

impl Default for OracleEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl Estimator for OracleEstimator {
    /// Fit the predictor on training data.
    ///
    /// X: Feature matrix (row i = bag-of-words for sample i)
    /// y: Label vector (category indices)
    ///
    /// Note: For text classification, we typically use the samples directly
    /// rather than pre-computed features. This implementation stores the
    /// samples and rebuilds the predictor.
    fn fit(&mut self, _x: &Matrix<f32>, y: &Vector<f32>) -> AprenderResult<()> {
        // Clear and rebuild predictor
        self.predictor = NgramFixPredictor::new();

        // Learn patterns from stored samples
        for (i, sample) in self.samples.iter().enumerate() {
            if i < y.len() {
                // Use fix if available, otherwise use a generic fix hint
                let fix = sample.fix.as_deref().unwrap_or("Check error details");
                self.predictor
                    .learn_pattern(&sample.message, fix, sample.category);
            }
        }

        // Fit the vectorizer
        self.predictor
            .fit()
            .map_err(|e| AprenderError::InvalidHyperparameter {
                param: "predictor".to_string(),
                value: format!("fit failed: {e}"),
                constraint: "Must have training patterns".to_string(),
            })?;

        Ok(())
    }

    /// Predict category indices for input features.
    ///
    /// Returns vector of category indices (0-5 for ErrorCategory variants).
    fn predict(&self, _x: &Matrix<f32>) -> Vector<f32> {
        // For proper evaluation, we need the original error messages
        // This simplified version returns zeros (would need message storage)
        Vector::zeros(_x.n_rows())
    }

    /// Score the predictor (accuracy).
    fn score(&self, _x: &Matrix<f32>, y: &Vector<f32>) -> f32 {
        // Count correct predictions
        let predictions = self.predict(_x);
        let correct = predictions
            .as_slice()
            .iter()
            .zip(y.as_slice().iter())
            .filter(|(&p, &t)| (p - t).abs() < 0.5)
            .count();

        correct as f32 / y.len().max(1) as f32
    }
}

/// Feature extraction constants used for both training and prediction.
pub mod feature_config {
    /// Error codes for one-hot encoding.
    pub const ERROR_CODES: &[&str] = &[
        // Type mismatch errors
        "E0308", "E0282", "E0609", "E0606", "E0631", // Import/resolution errors
        "E0432", "E0433", "E0412", "E0425", // Trait bound errors
        "E0277", "E0599", // Borrow checker errors
        "E0502", "E0499", "E0507", "E0382", "E0596", "E0597", "E0505", "E0503", "E0594",
        // Syntax errors
        "E0423", "E0658", "E0627", // Move/ownership
        "E0373", "E0061",
    ];

    /// Keywords for occurrence counting.
    pub const KEYWORDS: &[&str] = &[
        // Type-related
        "mismatch",
        "expected",
        "found",
        "type",
        "types",
        "i32",
        "f64",
        "String",
        "Value",
        // Trait-related
        "trait",
        "bound",
        "satisfied",
        "implement",
        "Display",
        "Copy",
        // Borrow-related
        "borrow",
        "borrowed",
        "mut",
        "mutable",
        "move",
        "moved",
        "lifetime",
        "reference",
        // Import-related
        "import",
        "unresolved",
        "undeclared",
        "crate",
        "module",
        // Misc
        "method",
        "field",
        "closure",
        "async",
        "Option",
        "Result",
        "HashMap",
        "Vec",
    ];

    /// Total number of features.
    pub const TOTAL_FEATURES: usize = 25 + 36 + 12; // error codes + keywords + ErrorFeatures
}

/// Convert a single error message to feature vector.
///
/// Uses the same feature extraction as `samples_to_features` for consistency.
#[must_use]
pub fn message_to_features(message: &str) -> Matrix<f32> {
    use feature_config::{ERROR_CODES, KEYWORDS};

    let n_error_codes = ERROR_CODES.len();
    let n_keywords = KEYWORDS.len();
    let n_handcrafted = crate::ErrorFeatures::DIM;
    let n_features = n_error_codes + n_keywords + n_handcrafted;

    let mut features = vec![0.0f32; n_features];

    // Error code features (one-hot encoding)
    for (j, code) in ERROR_CODES.iter().enumerate() {
        if message.contains(code) {
            features[j] = 1.0;
        }
    }

    // Keyword features (count-based)
    for (j, kw) in KEYWORDS.iter().enumerate() {
        let count = message.matches(kw).count();
        features[n_error_codes + j] = count as f32;
    }

    // Hand-crafted ErrorFeatures (normalized)
    let error_features = crate::ErrorFeatures::from_error_message(message);
    let feature_vec = error_features.to_vec();
    for (j, &val) in feature_vec.iter().enumerate() {
        features[n_error_codes + n_keywords + j] = val;
    }

    Matrix::from_vec(1, n_features, features).expect("Feature matrix dimensions should be valid")
}

/// Convert training samples to feature matrix and label vector.
///
/// Uses enhanced features combining:
/// - Error code one-hot encoding (25 features)
/// - Keyword occurrence counts (36 features)
/// - ErrorFeatures hand-crafted features (12 features)
///
/// Total: 73 features for better classification accuracy.
#[must_use]
pub fn samples_to_features(samples: &[TrainingSample]) -> (Matrix<f32>, Vector<f32>) {
    use feature_config::{ERROR_CODES, KEYWORDS};

    let n_error_codes = ERROR_CODES.len();
    let n_keywords = KEYWORDS.len();
    let n_handcrafted = ErrorFeatures::DIM;
    let n_features = n_error_codes + n_keywords + n_handcrafted;
    let n_samples = samples.len();

    let mut features = vec![0.0f32; n_samples * n_features];
    let mut labels = Vec::with_capacity(n_samples);

    for (i, sample) in samples.iter().enumerate() {
        let msg = &sample.message;
        let base_idx = i * n_features;

        // Error code features (one-hot encoding)
        for (j, code) in ERROR_CODES.iter().enumerate() {
            if msg.contains(code) {
                features[base_idx + j] = 1.0;
            }
        }

        // Keyword features (count-based)
        for (j, kw) in KEYWORDS.iter().enumerate() {
            let count = msg.matches(kw).count();
            features[base_idx + n_error_codes + j] = count as f32;
        }

        // Hand-crafted ErrorFeatures (normalized)
        let error_features = ErrorFeatures::from_error_message(msg);
        let feature_vec = error_features.to_vec();
        for (j, &val) in feature_vec.iter().enumerate() {
            features[base_idx + n_error_codes + n_keywords + j] = val;
        }

        labels.push(sample.category.index() as f32);
    }

    let matrix = Matrix::from_vec(n_samples, n_features, features)
        .expect("Feature matrix dimensions should be valid");
    let label_vec = Vector::from_vec(labels);

    (matrix, label_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::depyler_training::build_combined_corpus;

    #[test]
    fn test_oracle_estimator_creation() {
        let estimator = OracleEstimator::new();
        assert!(estimator.samples.is_empty());
    }

    #[test]
    fn test_oracle_estimator_default() {
        let estimator = OracleEstimator::default();
        assert!(estimator.samples.is_empty());
        assert_eq!(estimator.n_features, 0);
    }

    #[test]
    fn test_oracle_estimator_clone() {
        let mut estimator = OracleEstimator::new();
        estimator.add_samples(vec![TrainingSample::with_fix(
            "test error",
            ErrorCategory::TypeMismatch,
            "test fix",
        )]);
        estimator.n_features = 73;

        let cloned = estimator.clone();
        assert_eq!(cloned.samples.len(), 1);
        assert_eq!(cloned.n_features, 73);
    }

    #[test]
    fn test_oracle_estimator_with_min_similarity() {
        let estimator = OracleEstimator::new().with_min_similarity(0.8);
        // Just verify it doesn't panic and returns self
        assert!(estimator.samples.is_empty());
    }

    #[test]
    fn test_oracle_estimator_add_samples() {
        let mut estimator = OracleEstimator::new();
        assert!(estimator.samples.is_empty());

        let samples = vec![
            TrainingSample::with_fix("error1", ErrorCategory::TypeMismatch, "fix1"),
            TrainingSample::with_fix("error2", ErrorCategory::MissingImport, "fix2"),
        ];
        estimator.add_samples(samples);
        assert_eq!(estimator.samples.len(), 2);

        // Add more samples
        estimator.add_samples(vec![TrainingSample::with_fix(
            "error3",
            ErrorCategory::BorrowChecker,
            "fix3",
        )]);
        assert_eq!(estimator.samples.len(), 3);
    }

    #[test]
    fn test_oracle_estimator_predictor() {
        let estimator = OracleEstimator::new();
        let predictor = estimator.predictor();
        // Just verify we can access the predictor
        assert!(predictor.pattern_count() == 0);
    }

    #[test]
    fn test_oracle_estimator_predict_category_no_patterns() {
        let estimator = OracleEstimator::new();
        let result = estimator.predict_category("error[E0308]: mismatched types");
        assert!(result.is_none());
    }

    #[test]
    fn test_samples_to_features() {
        let samples = vec![
            TrainingSample::with_fix(
                "error[E0308]: mismatched types",
                ErrorCategory::TypeMismatch,
                "Fix type",
            ),
            TrainingSample::with_fix(
                "error[E0432]: unresolved import",
                ErrorCategory::MissingImport,
                "Add import",
            ),
        ];

        let (x, y) = samples_to_features(&samples);
        assert_eq!(x.n_rows(), 2);
        assert_eq!(y.len(), 2);
    }

    #[test]
    fn test_samples_to_features_feature_dimensions() {
        let samples = vec![TrainingSample::with_fix(
            "error[E0308]: expected type i32",
            ErrorCategory::TypeMismatch,
            "Change type",
        )];

        let (x, y) = samples_to_features(&samples);
        assert_eq!(x.n_rows(), 1);
        assert_eq!(x.n_cols(), feature_config::TOTAL_FEATURES);
        assert_eq!(y.len(), 1);
    }

    #[test]
    fn test_samples_to_features_error_code_one_hot() {
        let samples = vec![TrainingSample::with_fix(
            "error[E0308]: mismatched types",
            ErrorCategory::TypeMismatch,
            "Fix",
        )];

        let (x, _) = samples_to_features(&samples);
        // E0308 should be encoded
        let row = x.row(0);
        // Find index of E0308 in ERROR_CODES
        let idx = feature_config::ERROR_CODES
            .iter()
            .position(|&c| c == "E0308")
            .unwrap();
        assert_eq!(row[idx], 1.0);
    }

    #[test]
    fn test_samples_to_features_keyword_counts() {
        let samples = vec![TrainingSample::with_fix(
            "expected type type type",
            ErrorCategory::TypeMismatch,
            "Fix",
        )];

        let (x, _) = samples_to_features(&samples);
        let row = x.row(0);
        // "type" should appear 3 times
        let n_error_codes = feature_config::ERROR_CODES.len();
        let type_idx = feature_config::KEYWORDS
            .iter()
            .position(|&k| k == "type")
            .unwrap();
        assert_eq!(row[n_error_codes + type_idx], 3.0);
    }

    #[test]
    fn test_samples_to_features_empty() {
        let samples: Vec<TrainingSample> = vec![];
        let (x, y) = samples_to_features(&samples);
        assert_eq!(x.n_rows(), 0);
        assert_eq!(y.len(), 0);
    }

    #[test]
    fn test_message_to_features() {
        let msg = "error[E0308]: mismatched types expected i32";
        let features = message_to_features(msg);
        assert_eq!(features.n_rows(), 1);
        assert_eq!(features.n_cols(), feature_config::TOTAL_FEATURES);
    }

    #[test]
    fn test_message_to_features_error_code_encoding() {
        let msg = "error[E0277]: trait bound not satisfied";
        let features = message_to_features(msg);
        let row = features.row(0);

        let idx = feature_config::ERROR_CODES
            .iter()
            .position(|&c| c == "E0277")
            .unwrap();
        assert_eq!(row[idx], 1.0);
    }

    #[test]
    fn test_message_to_features_keyword_encoding() {
        let msg = "borrow borrowed mutable reference";
        let features = message_to_features(msg);
        let row = features.row(0);

        let n_error_codes = feature_config::ERROR_CODES.len();
        let borrow_idx = feature_config::KEYWORDS
            .iter()
            .position(|&k| k == "borrow")
            .unwrap();
        assert!(row[n_error_codes + borrow_idx] >= 1.0);
    }

    #[test]
    fn test_feature_config_constants() {
        assert!(!feature_config::ERROR_CODES.is_empty());
        assert!(!feature_config::KEYWORDS.is_empty());
        // Verify total features calculation (TOTAL_FEATURES > 0 is verified by the equality check)
        let expected =
            feature_config::ERROR_CODES.len() + feature_config::KEYWORDS.len() + ErrorFeatures::DIM;
        assert_eq!(feature_config::TOTAL_FEATURES, expected);
    }

    #[test]
    fn test_feature_config_error_codes_valid() {
        for code in feature_config::ERROR_CODES {
            assert!(code.starts_with('E'));
            assert!(code.len() == 5);
        }
    }

    #[test]
    fn test_feature_config_keywords_non_empty() {
        for kw in feature_config::KEYWORDS {
            assert!(!kw.is_empty());
        }
    }

    #[test]
    fn test_estimator_fit() {
        let corpus = build_combined_corpus();
        let samples: Vec<_> = corpus.samples().to_vec();
        let (x, y) = samples_to_features(&samples);

        let mut estimator = OracleEstimator::new();
        estimator.add_samples(samples);

        let result = estimator.fit(&x, &y);
        assert!(result.is_ok());
    }

    #[test]
    fn test_estimator_predict() {
        let mut estimator = OracleEstimator::new();
        let samples = vec![
            TrainingSample::with_fix("error[E0308]: type", ErrorCategory::TypeMismatch, "fix"),
            TrainingSample::with_fix("error[E0432]: import", ErrorCategory::MissingImport, "fix"),
        ];
        estimator.add_samples(samples.clone());

        let (x, y) = samples_to_features(&samples);
        let _ = estimator.fit(&x, &y);

        let predictions = estimator.predict(&x);
        assert_eq!(predictions.len(), x.n_rows());
    }

    #[test]
    fn test_estimator_score() {
        let mut estimator = OracleEstimator::new();
        let samples = vec![
            TrainingSample::with_fix("error[E0308]: type", ErrorCategory::TypeMismatch, "fix"),
            TrainingSample::with_fix("error[E0432]: import", ErrorCategory::MissingImport, "fix"),
        ];
        estimator.add_samples(samples.clone());

        let (x, y) = samples_to_features(&samples);
        let _ = estimator.fit(&x, &y);

        let score = estimator.score(&x, &y);
        // Score should be between 0 and 1
        assert!(score >= 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_estimator_score_empty_labels() {
        let estimator = OracleEstimator::new();
        let x = Matrix::zeros(0, feature_config::TOTAL_FEATURES);
        let y = Vector::from_vec(vec![]);
        let score = estimator.score(&x, &y);
        // Empty labels should return 0 or 1 (handled by max(1))
        assert!(score >= 0.0);
    }

    #[test]
    fn test_labels_map_to_category_index() {
        // Verify samples_to_features correctly maps categories to indices
        let samples = vec![
            TrainingSample::with_fix("e1", ErrorCategory::TypeMismatch, "f1"),
            TrainingSample::with_fix("e2", ErrorCategory::MissingImport, "f2"),
            TrainingSample::with_fix("e3", ErrorCategory::BorrowChecker, "f3"),
            TrainingSample::with_fix("e4", ErrorCategory::TraitBound, "f4"),
        ];

        let (_, y) = samples_to_features(&samples);
        assert_eq!(y.len(), 4);

        // Each label should be the category index
        assert_eq!(y.as_slice()[0], ErrorCategory::TypeMismatch.index() as f32);
        assert_eq!(y.as_slice()[1], ErrorCategory::MissingImport.index() as f32);
        assert_eq!(y.as_slice()[2], ErrorCategory::BorrowChecker.index() as f32);
        assert_eq!(y.as_slice()[3], ErrorCategory::TraitBound.index() as f32);
    }

    #[test]
    fn test_samples_without_fix() {
        // TrainingSample without fix should use default hint
        let mut estimator = OracleEstimator::new();
        let samples = vec![TrainingSample::new(
            "error message",
            ErrorCategory::TypeMismatch,
        )];
        estimator.add_samples(samples.clone());

        let (x, y) = samples_to_features(&samples);
        // Should not panic when fit with samples that have None fix
        let result = estimator.fit(&x, &y);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_error_codes_in_message() {
        let msg = "error[E0308]: mismatched types\nerror[E0277]: trait bound";
        let features = message_to_features(msg);
        let row = features.row(0);

        // Both E0308 and E0277 should be encoded
        let idx_308 = feature_config::ERROR_CODES
            .iter()
            .position(|&c| c == "E0308")
            .unwrap();
        let idx_277 = feature_config::ERROR_CODES
            .iter()
            .position(|&c| c == "E0277")
            .unwrap();

        assert_eq!(row[idx_308], 1.0);
        assert_eq!(row[idx_277], 1.0);
    }
}

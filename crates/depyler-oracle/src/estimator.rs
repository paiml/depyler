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

/// Convert training samples to feature matrix and label vector.
///
/// Uses enhanced features combining:
/// - Error code one-hot encoding (25 features)
/// - Keyword occurrence counts (28 features)
/// - ErrorFeatures hand-crafted features (12 features)
///
/// Total: 65 features for better classification accuracy.
#[must_use]
pub fn samples_to_features(samples: &[TrainingSample]) -> (Matrix<f32>, Vector<f32>) {
    // Feature extraction: error codes + keywords + hand-crafted
    // Extended error codes for better discrimination (Issue #106)
    let error_codes = [
        // Type mismatch errors
        "E0308", "E0282", "E0609", "E0606", "E0631", // Import/resolution errors
        "E0432", "E0433", "E0412", "E0425", // Trait bound errors
        "E0277", "E0599", // Borrow checker errors
        "E0502", "E0499", "E0507", "E0382", "E0596", "E0597", "E0505", "E0503", "E0594",
        // Syntax errors
        "E0423", "E0658", "E0627", // Move/ownership
        "E0373", "E0061",
    ];
    // Extended keywords for better category discrimination
    let keywords = [
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

    // Total features: error codes (25) + keywords (28) + ErrorFeatures (12) = 65
    let n_error_codes = error_codes.len();
    let n_keywords = keywords.len();
    let n_handcrafted = ErrorFeatures::DIM;
    let n_features = n_error_codes + n_keywords + n_handcrafted;
    let n_samples = samples.len();

    let mut features = vec![0.0f32; n_samples * n_features];
    let mut labels = Vec::with_capacity(n_samples);

    for (i, sample) in samples.iter().enumerate() {
        let msg = &sample.message;
        let base_idx = i * n_features;

        // Error code features (one-hot encoding)
        for (j, code) in error_codes.iter().enumerate() {
            if msg.contains(code) {
                features[base_idx + j] = 1.0;
            }
        }

        // Keyword features (count-based)
        for (j, kw) in keywords.iter().enumerate() {
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
    fn test_estimator_fit() {
        let corpus = build_combined_corpus();
        let samples: Vec<_> = corpus.samples().to_vec();
        let (x, y) = samples_to_features(&samples);

        let mut estimator = OracleEstimator::new();
        estimator.add_samples(samples);

        let result = estimator.fit(&x, &y);
        assert!(result.is_ok());
    }
}

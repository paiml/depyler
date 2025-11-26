//! Feature extraction from error messages.

use aprender::primitives::Matrix;
use serde::{Deserialize, Serialize};

/// Features extracted from an error message for ML classification.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ErrorFeatures {
    /// Message length (normalized)
    pub message_length: f32,
    /// Number of type-related keywords
    pub type_keywords: f32,
    /// Number of borrow-related keywords
    pub borrow_keywords: f32,
    /// Number of import-related keywords
    pub import_keywords: f32,
    /// Number of lifetime-related keywords
    pub lifetime_keywords: f32,
    /// Number of trait-related keywords
    pub trait_keywords: f32,
    /// Contains line number
    pub has_line_number: f32,
    /// Contains column number
    pub has_column: f32,
    /// Contains backticks (code snippets)
    pub has_code_snippets: f32,
    /// Contains arrow indicators
    pub has_arrows: f32,
    /// Error code present (e.g., E0308)
    pub has_error_code: f32,
    /// Number of suggestions in message
    pub suggestion_count: f32,
}

impl ErrorFeatures {
    /// Feature dimension.
    pub const DIM: usize = 12;

    /// Extract features from an error message.
    #[must_use]
    pub fn from_error_message(message: &str) -> Self {
        let lower = message.to_lowercase();

        Self {
            message_length: (message.len() as f32 / 500.0).min(1.0),

            type_keywords: count_keywords(&lower, &[
                "expected", "found", "mismatched", "type",
                "cannot coerce", "incompatible",
            ]),

            borrow_keywords: count_keywords(&lower, &[
                "borrow", "borrowed", "move", "moved",
                "ownership", "cannot move",
            ]),

            import_keywords: count_keywords(&lower, &[
                "not found", "unresolved", "cannot find",
                "undefined", "undeclared",
            ]),

            lifetime_keywords: count_keywords(&lower, &[
                "lifetime", "'a", "'static", "live long enough",
                "dangling", "borrowed value",
            ]),

            trait_keywords: count_keywords(&lower, &[
                "trait", "impl", "not implemented", "bound",
                "doesn't implement",
            ]),

            has_line_number: if message.contains(':') && message.chars().any(|c| c.is_ascii_digit()) {
                1.0
            } else {
                0.0
            },

            has_column: if message.matches(':').count() > 1 {
                1.0
            } else {
                0.0
            },

            has_code_snippets: (message.matches('`').count() as f32 / 10.0).min(1.0),

            has_arrows: if message.contains("-->") || message.contains("^^^") {
                1.0
            } else {
                0.0
            },

            has_error_code: if message.contains("E0") || message.contains("[E") {
                1.0
            } else {
                0.0
            },

            suggestion_count: count_keywords(&lower, &[
                "help:", "suggestion:", "consider", "try", "perhaps",
            ]),
        }
    }

    /// Convert features to a row matrix for ML model.
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        Matrix::from_vec(1, Self::DIM, self.to_vec())
            .expect("Feature dimensions are correct")
    }

    /// Convert features to a vector.
    #[must_use]
    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.message_length,
            self.type_keywords,
            self.borrow_keywords,
            self.import_keywords,
            self.lifetime_keywords,
            self.trait_keywords,
            self.has_line_number,
            self.has_column,
            self.has_code_snippets,
            self.has_arrows,
            self.has_error_code,
            self.suggestion_count,
        ]
    }

    /// Create features from a vector.
    ///
    /// # Panics
    ///
    /// Panics if vector length doesn't match DIM.
    #[must_use]
    pub fn from_vec(v: &[f32]) -> Self {
        assert_eq!(v.len(), Self::DIM, "Feature vector must have {} elements", Self::DIM);

        Self {
            message_length: v[0],
            type_keywords: v[1],
            borrow_keywords: v[2],
            import_keywords: v[3],
            lifetime_keywords: v[4],
            trait_keywords: v[5],
            has_line_number: v[6],
            has_column: v[7],
            has_code_snippets: v[8],
            has_arrows: v[9],
            has_error_code: v[10],
            suggestion_count: v[11],
        }
    }
}

/// Count keyword occurrences (normalized).
fn count_keywords(text: &str, keywords: &[&str]) -> f32 {
    let count = keywords.iter().filter(|k| text.contains(*k)).count();
    (count as f32 / keywords.len() as f32).min(1.0)
}

/// Batch feature extraction for training data.
pub struct FeatureExtractor;

impl FeatureExtractor {
    /// Extract features from multiple error messages.
    #[must_use]
    pub fn extract_batch(messages: &[&str]) -> Matrix<f32> {
        let features: Vec<f32> = messages
            .iter()
            .flat_map(|msg| ErrorFeatures::from_error_message(msg).to_vec())
            .collect();

        Matrix::from_vec(messages.len(), ErrorFeatures::DIM, features)
            .expect("Feature batch dimensions are correct")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_extraction() {
        let msg = "error[E0308]: mismatched types\n  --> src/main.rs:10:5\n   |\n10 |     foo(bar)\n   |         ^^^ expected `i32`, found `&str`";

        let features = ErrorFeatures::from_error_message(msg);

        assert!(features.message_length > 0.0);
        assert!(features.type_keywords > 0.0);
        assert!(features.has_error_code > 0.0);
        assert!(features.has_line_number > 0.0);
        assert!(features.has_arrows > 0.0);
    }

    #[test]
    fn test_borrow_features() {
        let msg = "error: cannot move out of borrowed content";
        let features = ErrorFeatures::from_error_message(msg);

        assert!(features.borrow_keywords > 0.0);
        assert!((features.type_keywords - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_to_matrix() {
        let msg = "error: expected i32";
        let features = ErrorFeatures::from_error_message(msg);
        let matrix = features.to_matrix();

        assert_eq!(matrix.n_rows(), 1);
        assert_eq!(matrix.n_cols(), ErrorFeatures::DIM);
    }

    #[test]
    fn test_vec_roundtrip() {
        let msg = "error: mismatched types";
        let features = ErrorFeatures::from_error_message(msg);
        let vec = features.to_vec();
        let restored = ErrorFeatures::from_vec(&vec);

        assert!((features.type_keywords - restored.type_keywords).abs() < 1e-6);
    }

    #[test]
    fn test_batch_extraction() {
        let messages = vec![
            "error: expected i32",
            "error: cannot move",
            "error: not found",
        ];

        let matrix = FeatureExtractor::extract_batch(&messages);

        assert_eq!(matrix.n_rows(), 3);
        assert_eq!(matrix.n_cols(), ErrorFeatures::DIM);
    }

    #[test]
    fn test_lifetime_features() {
        let msg = "error: `x` does not live long enough";
        let features = ErrorFeatures::from_error_message(msg);

        assert!(features.lifetime_keywords > 0.0);
    }

    #[test]
    fn test_trait_features() {
        let msg = "error: the trait bound `T: Clone` is not satisfied";
        let features = ErrorFeatures::from_error_message(msg);

        assert!(features.trait_keywords > 0.0);
    }

    #[test]
    fn test_suggestion_count() {
        let msg = "error: type mismatch\nhelp: try using `.into()`\nhelp: consider adding type annotation";
        let features = ErrorFeatures::from_error_message(msg);

        assert!(features.suggestion_count > 0.0);
    }
}

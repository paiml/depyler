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

            type_keywords: count_keywords(
                &lower,
                &[
                    "expected",
                    "found",
                    "mismatched",
                    "type",
                    "cannot coerce",
                    "incompatible",
                ],
            ),

            borrow_keywords: count_keywords(
                &lower,
                &[
                    "borrow",
                    "borrowed",
                    "move",
                    "moved",
                    "ownership",
                    "cannot move",
                ],
            ),

            import_keywords: count_keywords(
                &lower,
                &[
                    "not found",
                    "unresolved",
                    "cannot find",
                    "undefined",
                    "undeclared",
                ],
            ),

            lifetime_keywords: count_keywords(
                &lower,
                &[
                    "lifetime",
                    "'a",
                    "'static",
                    "live long enough",
                    "dangling",
                    "borrowed value",
                ],
            ),

            trait_keywords: count_keywords(
                &lower,
                &[
                    "trait",
                    "impl",
                    "not implemented",
                    "bound",
                    "doesn't implement",
                ],
            ),

            has_line_number: if message.contains(':') && message.chars().any(|c| c.is_ascii_digit())
            {
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

            suggestion_count: count_keywords(
                &lower,
                &["help:", "suggestion:", "consider", "try", "perhaps"],
            ),
        }
    }

    /// Convert features to a row matrix for ML model.
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        Matrix::from_vec(1, Self::DIM, self.to_vec()).expect("Feature dimensions are correct")
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
        assert_eq!(
            v.len(),
            Self::DIM,
            "Feature vector must have {} elements",
            Self::DIM
        );

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

// GH-210: Enhanced feature extraction for Code2Vec & GNN upgrade
// Expands from 12 to 73 dimensions for better ML classification

/// Top 25 Rust error codes for one-hot encoding
/// Based on frequency analysis from reprorusted-python-cli corpus
pub const ERROR_CODES: [&str; 25] = [
    "E0308", // mismatched types (most common)
    "E0425", // cannot find value
    "E0433", // failed to resolve
    "E0277", // trait bound not satisfied
    "E0599", // no method named
    "E0382", // use of moved value
    "E0502", // cannot borrow as mutable
    "E0503", // cannot use while mutably borrowed
    "E0505", // cannot move out of borrowed
    "E0506", // cannot assign to borrowed
    "E0507", // cannot move out of
    "E0106", // missing lifetime specifier
    "E0495", // cannot infer lifetime
    "E0621", // explicit lifetime required
    "E0282", // type annotations needed
    "E0283", // type annotations required
    "E0412", // cannot find type
    "E0432", // unresolved import
    "E0603", // private item
    "E0609", // no field
    "E0614", // cannot be dereferenced
    "E0615", // attempted to take value
    "E0616", // field is private
    "E0618", // expected function
    "E0620", // cast to unsized type
];

/// Extended keyword categories for detailed feature extraction
pub const KEYWORD_CATEGORIES: [(&str, &[&str]); 9] = [
    ("type_coercion", &["as", "into", "from", "convert", "cast"]),
    ("ownership", &["owned", "clone", "copy", "drop", "take"]),
    ("reference", &["ref", "&", "deref", "borrow"]),
    ("mutability", &["mut", "immutable", "mutable"]),
    ("generic", &["generic", "parameter", "constraint", "where"]),
    ("async", &["async", "await", "future", "poll"]),
    ("closure", &["closure", "capture", "fn", "move"]),
    ("derive", &["derive", "debug", "clone", "default"]),
    (
        "result_option",
        &["result", "option", "some", "none", "ok", "err", "unwrap"],
    ),
];

/// GH-210: Enhanced error features with 73 dimensions
/// Combines base features (12) + error code one-hot (25) + keyword counts (36)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnhancedErrorFeatures {
    /// Base 12 features from ErrorFeatures
    pub base: ErrorFeatures,
    /// One-hot encoding for top 25 error codes
    pub error_code_onehot: Vec<f32>,
    /// Detailed keyword occurrence counts (9 categories × 4 normalized features)
    pub keyword_counts: Vec<f32>,
}

impl Default for EnhancedErrorFeatures {
    fn default() -> Self {
        Self {
            base: ErrorFeatures::default(),
            error_code_onehot: vec![0.0; 25],
            keyword_counts: vec![0.0; 36],
        }
    }
}

impl EnhancedErrorFeatures {
    /// Enhanced feature dimension: 12 + 25 + 36 = 73
    pub const DIM: usize = 73;

    /// Extract enhanced features from error message
    #[must_use]
    pub fn from_error_message(message: &str) -> Self {
        let lower = message.to_lowercase();

        // Base features
        let base = ErrorFeatures::from_error_message(message);

        // Error code one-hot encoding
        let mut error_code_onehot = vec![0.0f32; 25];
        for (i, code) in ERROR_CODES.iter().enumerate() {
            if message.contains(code) {
                error_code_onehot[i] = 1.0;
                break; // Only one error code per message
            }
        }

        // Extended keyword counts (9 categories × 4 features each)
        let mut keyword_counts = vec![0.0f32; 36];
        for (i, (_name, keywords)) in KEYWORD_CATEGORIES.iter().enumerate() {
            let base_idx = i * 4;
            // Feature 1: presence (0 or 1)
            let present = keywords.iter().any(|k| lower.contains(k));
            keyword_counts[base_idx] = if present { 1.0 } else { 0.0 };

            // Feature 2: count ratio (normalized)
            let count = keywords.iter().filter(|k| lower.contains(*k)).count();
            keyword_counts[base_idx + 1] = (count as f32 / keywords.len() as f32).min(1.0);

            // Feature 3: first occurrence position (normalized by message length)
            let first_pos = keywords
                .iter()
                .filter_map(|k| lower.find(k))
                .min()
                .unwrap_or(lower.len());
            keyword_counts[base_idx + 2] = 1.0 - (first_pos as f32 / lower.len().max(1) as f32);

            // Feature 4: keyword density (occurrences per 100 chars)
            let total_occurrences: usize = keywords.iter().map(|k| lower.matches(k).count()).sum();
            keyword_counts[base_idx + 3] =
                (total_occurrences as f32 * 100.0 / lower.len().max(1) as f32).min(1.0);
        }

        Self {
            base,
            error_code_onehot,
            keyword_counts,
        }
    }

    /// Convert to feature vector for ML model
    #[must_use]
    pub fn to_vec(&self) -> Vec<f32> {
        let mut vec = Vec::with_capacity(Self::DIM);
        vec.extend(self.base.to_vec());
        vec.extend(self.error_code_onehot.iter());
        vec.extend(self.keyword_counts.iter());
        vec
    }

    /// Convert to matrix for ML model
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<f32> {
        Matrix::from_vec(1, Self::DIM, self.to_vec()).expect("Feature dimensions are correct")
    }
}

/// Batch extraction for enhanced features
pub struct EnhancedFeatureExtractor;

impl EnhancedFeatureExtractor {
    /// Extract enhanced features from multiple error messages
    #[must_use]
    pub fn extract_batch(messages: &[&str]) -> Matrix<f32> {
        let features: Vec<f32> = messages
            .iter()
            .flat_map(|msg| EnhancedErrorFeatures::from_error_message(msg).to_vec())
            .collect();

        Matrix::from_vec(messages.len(), EnhancedErrorFeatures::DIM, features)
            .expect("Feature batch dimensions are correct")
    }
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

    // GH-210: Tests for enhanced features

    #[test]
    fn test_enhanced_feature_dimension() {
        let msg = "error[E0308]: mismatched types";
        let features = EnhancedErrorFeatures::from_error_message(msg);
        let vec = features.to_vec();

        assert_eq!(vec.len(), EnhancedErrorFeatures::DIM);
        assert_eq!(vec.len(), 73);
    }

    #[test]
    fn test_enhanced_error_code_onehot() {
        let msg = "error[E0308]: mismatched types\n  --> src/main.rs:10:5";
        let features = EnhancedErrorFeatures::from_error_message(msg);

        // E0308 is at index 0
        assert_eq!(features.error_code_onehot[0], 1.0);
        // All others should be 0
        assert_eq!(features.error_code_onehot[1..].iter().sum::<f32>(), 0.0);
    }

    #[test]
    fn test_enhanced_e0425_onehot() {
        let msg = "error[E0425]: cannot find value `foo` in this scope";
        let features = EnhancedErrorFeatures::from_error_message(msg);

        // E0425 is at index 1
        assert_eq!(features.error_code_onehot[1], 1.0);
    }

    #[test]
    fn test_enhanced_keyword_categories() {
        let msg = "error: cannot convert `&str` into `String`";
        let features = EnhancedErrorFeatures::from_error_message(msg);

        // type_coercion category (index 0-3) should have hits
        // "into" and "convert" are both present
        assert!(features.keyword_counts[0] > 0.0, "type_coercion presence");
        assert!(
            features.keyword_counts[1] > 0.0,
            "type_coercion count ratio"
        );
    }

    #[test]
    fn test_enhanced_result_option_keywords() {
        let msg = "error: cannot call `.unwrap()` on `Result<T, E>`";
        let features = EnhancedErrorFeatures::from_error_message(msg);

        // result_option category is index 8 (8 * 4 = 32-35)
        assert!(features.keyword_counts[32] > 0.0, "result_option presence");
    }

    #[test]
    fn test_enhanced_batch_extraction() {
        let messages = vec![
            "error[E0308]: expected i32, found &str",
            "error[E0382]: use of moved value",
            "error[E0277]: trait bound not satisfied",
        ];

        let matrix = EnhancedFeatureExtractor::extract_batch(&messages);

        assert_eq!(matrix.n_rows(), 3);
        assert_eq!(matrix.n_cols(), EnhancedErrorFeatures::DIM);
    }

    #[test]
    fn test_enhanced_to_matrix() {
        let msg = "error[E0599]: no method named `foo` found";
        let features = EnhancedErrorFeatures::from_error_message(msg);
        let matrix = features.to_matrix();

        assert_eq!(matrix.n_rows(), 1);
        assert_eq!(matrix.n_cols(), EnhancedErrorFeatures::DIM);
    }
}

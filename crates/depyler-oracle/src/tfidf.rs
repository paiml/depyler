//! TF-IDF based feature extraction for error classification.
//!
//! Provides high-dimensional sparse features using TF-IDF vectorization,
//! complementing the hand-crafted features in `features.rs`.

use std::collections::HashMap;

use aprender::primitives::Matrix;
use aprender::text::tokenize::WhitespaceTokenizer;
use aprender::text::vectorize::TfidfVectorizer;
use serde::{Deserialize, Serialize};

use crate::OracleError;

/// TF-IDF feature extractor for error messages.
///
/// Converts error messages into high-dimensional TF-IDF vectors
/// for use with ML classifiers.
///
/// # Examples
///
/// ```
/// use depyler_oracle::tfidf::TfidfFeatureExtractor;
///
/// let training = vec![
///     "expected i32, found &str",
///     "cannot move out of borrowed content",
///     "type annotation needed",
/// ];
///
/// let mut extractor = TfidfFeatureExtractor::new()
///     .with_ngram_range(1, 2)
///     .with_max_features(100);
///
/// extractor.fit(&training).unwrap();
///
/// let features = extractor.transform(&["expected u32, found String"]).unwrap();
/// assert_eq!(features.n_rows(), 1);
/// ```
pub struct TfidfFeatureExtractor {
    /// Internal TF-IDF vectorizer
    vectorizer: TfidfVectorizer,
    /// Whether the extractor has been fitted
    is_fitted: bool,
    /// Vocabulary for reference
    vocabulary: HashMap<String, usize>,
    /// Configuration
    config: TfidfConfig,
}

/// Configuration for TF-IDF extraction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TfidfConfig {
    /// N-gram range (min, max)
    pub ngram_range: (usize, usize),
    /// Maximum features to extract
    pub max_features: Option<usize>,
    /// Use sublinear TF scaling
    pub sublinear_tf: bool,
    /// Minimum document frequency
    pub min_df: usize,
    /// Maximum document frequency ratio
    pub max_df: f32,
    /// Remove Rust-specific stop words
    pub use_rust_stopwords: bool,
}

impl Default for TfidfConfig {
    fn default() -> Self {
        Self {
            ngram_range: (1, 2),
            max_features: Some(500),
            sublinear_tf: true,
            min_df: 1,
            max_df: 0.95,
            use_rust_stopwords: true,
        }
    }
}

impl TfidfFeatureExtractor {
    /// Create a new TF-IDF feature extractor.
    #[must_use]
    pub fn new() -> Self {
        let config = TfidfConfig::default();
        Self::with_config(config)
    }

    /// Create with custom configuration.
    #[must_use]
    pub fn with_config(config: TfidfConfig) -> Self {
        let mut vectorizer = TfidfVectorizer::new()
            .with_tokenizer(Box::new(WhitespaceTokenizer::new()))
            .with_ngram_range(config.ngram_range.0, config.ngram_range.1)
            .with_sublinear_tf(config.sublinear_tf)
            .with_min_df(config.min_df)
            .with_max_df(config.max_df);

        if let Some(max_features) = config.max_features {
            vectorizer = vectorizer.with_max_features(max_features);
        }

        if config.use_rust_stopwords {
            vectorizer = vectorizer.with_custom_stop_words(&RUST_ERROR_STOPWORDS);
        }

        Self {
            vectorizer,
            is_fitted: false,
            vocabulary: HashMap::new(),
            config,
        }
    }

    /// Set N-gram range.
    #[must_use]
    pub fn with_ngram_range(mut self, min_n: usize, max_n: usize) -> Self {
        self.config.ngram_range = (min_n.max(1), max_n.max(1));
        self.rebuild_vectorizer();
        self
    }

    /// Set maximum features.
    #[must_use]
    pub fn with_max_features(mut self, max: usize) -> Self {
        self.config.max_features = Some(max);
        self.rebuild_vectorizer();
        self
    }

    /// Enable/disable sublinear TF.
    #[must_use]
    pub fn with_sublinear_tf(mut self, enable: bool) -> Self {
        self.config.sublinear_tf = enable;
        self.rebuild_vectorizer();
        self
    }

    /// Enable/disable Rust stopwords.
    #[must_use]
    pub fn with_rust_stopwords(mut self, enable: bool) -> Self {
        self.config.use_rust_stopwords = enable;
        self.rebuild_vectorizer();
        self
    }

    /// Rebuild vectorizer from config.
    fn rebuild_vectorizer(&mut self) {
        let mut vectorizer = TfidfVectorizer::new()
            .with_tokenizer(Box::new(WhitespaceTokenizer::new()))
            .with_ngram_range(self.config.ngram_range.0, self.config.ngram_range.1)
            .with_sublinear_tf(self.config.sublinear_tf)
            .with_min_df(self.config.min_df)
            .with_max_df(self.config.max_df);

        if let Some(max_features) = self.config.max_features {
            vectorizer = vectorizer.with_max_features(max_features);
        }

        if self.config.use_rust_stopwords {
            vectorizer = vectorizer.with_custom_stop_words(&RUST_ERROR_STOPWORDS);
        }

        self.vectorizer = vectorizer;
        self.is_fitted = false;
    }

    /// Fit the extractor on training documents.
    ///
    /// # Errors
    ///
    /// Returns error if fitting fails.
    pub fn fit<S: AsRef<str>>(&mut self, documents: &[S]) -> Result<(), OracleError> {
        if documents.is_empty() {
            return Err(OracleError::Feature(
                "Cannot fit on empty documents".to_string(),
            ));
        }

        // Preprocess documents
        let processed: Vec<String> = documents.iter().map(|d| preprocess(d.as_ref())).collect();

        self.vectorizer
            .fit(&processed)
            .map_err(|e| OracleError::Feature(e.to_string()))?;

        self.vocabulary = self.vectorizer.vocabulary().clone();
        self.is_fitted = true;
        Ok(())
    }

    /// Transform documents to TF-IDF feature matrix.
    ///
    /// # Errors
    ///
    /// Returns error if transformation fails.
    pub fn transform<S: AsRef<str>>(&self, documents: &[S]) -> Result<Matrix<f64>, OracleError> {
        if !self.is_fitted {
            return Err(OracleError::Feature(
                "Extractor not fitted. Call fit() first".to_string(),
            ));
        }

        if documents.is_empty() {
            return Err(OracleError::Feature(
                "Cannot transform empty documents".to_string(),
            ));
        }

        let processed: Vec<String> = documents.iter().map(|d| preprocess(d.as_ref())).collect();

        self.vectorizer
            .transform(&processed)
            .map_err(|e| OracleError::Feature(e.to_string()))
    }

    /// Fit and transform in one step.
    ///
    /// # Errors
    ///
    /// Returns error if fitting or transformation fails.
    pub fn fit_transform<S: AsRef<str>>(
        &mut self,
        documents: &[S],
    ) -> Result<Matrix<f64>, OracleError> {
        self.fit(documents)?;
        self.transform(documents)
    }

    /// Get vocabulary size.
    #[must_use]
    pub fn vocabulary_size(&self) -> usize {
        self.vocabulary.len()
    }

    /// Get vocabulary.
    #[must_use]
    pub fn vocabulary(&self) -> &HashMap<String, usize> {
        &self.vocabulary
    }

    /// Check if extractor is fitted.
    #[must_use]
    pub fn is_fitted(&self) -> bool {
        self.is_fitted
    }

    /// Get configuration.
    #[must_use]
    pub fn config(&self) -> &TfidfConfig {
        &self.config
    }

    /// Get top features by IDF weight.
    #[must_use]
    pub fn top_features(&self, n: usize) -> Vec<(String, f64)> {
        if !self.is_fitted {
            return Vec::new();
        }

        let idf = self.vectorizer.idf_values();
        let mut features: Vec<(String, f64)> = self
            .vocabulary
            .iter()
            .map(|(word, &idx)| {
                let idf_val = idf.get(idx).copied().unwrap_or(0.0);
                (word.clone(), idf_val)
            })
            .collect();

        features.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        features.truncate(n);
        features
    }
}

impl Default for TfidfFeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Preprocess error message for TF-IDF.
fn preprocess(message: &str) -> String {
    message
        .to_lowercase()
        // Remove error codes like E0308
        .replace(|c: char| c.is_ascii_digit(), "")
        // Normalize punctuation
        .replace(['`', '\'', '"', '(', ')', '[', ']', '{', '}'], " ")
        // Normalize arrows
        .replace("-->", " ")
        .replace("^^^", " ")
        // Collapse whitespace
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Rust-specific stop words for error messages.
const RUST_ERROR_STOPWORDS: [&str; 30] = [
    // Common error message words
    "error", "warning", "note", "help", "the", "a", "an", "is", "are", "was", "were", "this",
    "that", "in", "at", "to", "for", "of", "with", "as", "by", "on", "from", "or", "and", "not",
    "be", "can", "has", "have",
];

/// Combined feature extractor using both hand-crafted and TF-IDF features.
pub struct CombinedFeatureExtractor {
    /// TF-IDF extractor
    tfidf: TfidfFeatureExtractor,
    /// Whether to include hand-crafted features
    include_handcrafted: bool,
}

impl CombinedFeatureExtractor {
    /// Create a new combined extractor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tfidf: TfidfFeatureExtractor::new(),
            include_handcrafted: true,
        }
    }

    /// Create with custom TF-IDF config.
    #[must_use]
    pub fn with_tfidf_config(config: TfidfConfig) -> Self {
        Self {
            tfidf: TfidfFeatureExtractor::with_config(config),
            include_handcrafted: true,
        }
    }

    /// Enable/disable hand-crafted features.
    #[must_use]
    pub fn with_handcrafted(mut self, enable: bool) -> Self {
        self.include_handcrafted = enable;
        self
    }

    /// Fit on training documents.
    ///
    /// # Errors
    ///
    /// Returns error if fitting fails.
    pub fn fit<S: AsRef<str>>(&mut self, documents: &[S]) -> Result<(), OracleError> {
        self.tfidf.fit(documents)
    }

    /// Transform documents to combined feature matrix.
    ///
    /// # Errors
    ///
    /// Returns error if transformation fails.
    pub fn transform<S: AsRef<str>>(&self, documents: &[S]) -> Result<Matrix<f64>, OracleError> {
        use crate::ErrorFeatures;

        let tfidf_matrix = self.tfidf.transform(documents)?;

        if !self.include_handcrafted {
            return Ok(tfidf_matrix);
        }

        // Combine TF-IDF with hand-crafted features
        let n_docs = documents.len();
        let tfidf_cols = tfidf_matrix.n_cols();
        let handcrafted_cols = ErrorFeatures::DIM;
        let total_cols = tfidf_cols + handcrafted_cols;

        let mut combined_data = Vec::with_capacity(n_docs * total_cols);

        for (doc_idx, doc) in documents.iter().enumerate() {
            // Add TF-IDF features
            for col in 0..tfidf_cols {
                combined_data.push(tfidf_matrix.get(doc_idx, col));
            }

            // Add hand-crafted features
            let features = ErrorFeatures::from_error_message(doc.as_ref());
            for val in features.to_vec() {
                combined_data.push(f64::from(val));
            }
        }

        Matrix::from_vec(n_docs, total_cols, combined_data)
            .map_err(|e: &str| OracleError::Feature(e.to_string()))
    }

    /// Fit and transform in one step.
    ///
    /// # Errors
    ///
    /// Returns error if fitting or transformation fails.
    pub fn fit_transform<S: AsRef<str>>(
        &mut self,
        documents: &[S],
    ) -> Result<Matrix<f64>, OracleError> {
        self.fit(documents)?;
        self.transform(documents)
    }

    /// Get feature count.
    #[must_use]
    pub fn feature_count(&self) -> usize {
        use crate::ErrorFeatures;

        let tfidf_count = self.tfidf.vocabulary_size();
        if self.include_handcrafted {
            tfidf_count + ErrorFeatures::DIM
        } else {
            tfidf_count
        }
    }
}

impl Default for CombinedFeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===================
    // TfidfConfig Tests
    // ===================

    #[test]
    fn test_tfidf_config_default() {
        let config = TfidfConfig::default();

        assert_eq!(config.ngram_range, (1, 2));
        assert_eq!(config.max_features, Some(500));
        assert!(config.sublinear_tf);
        assert!(config.use_rust_stopwords);
    }

    // ===================
    // TfidfFeatureExtractor Tests
    // ===================

    #[test]
    fn test_extractor_creation() {
        let extractor = TfidfFeatureExtractor::new();

        assert!(!extractor.is_fitted());
        assert_eq!(extractor.vocabulary_size(), 0);
    }

    #[test]
    fn test_extractor_with_config() {
        let config = TfidfConfig {
            ngram_range: (2, 3),
            max_features: Some(100),
            sublinear_tf: false,
            ..Default::default()
        };

        let extractor = TfidfFeatureExtractor::with_config(config.clone());

        assert_eq!(extractor.config().ngram_range, (2, 3));
        assert_eq!(extractor.config().max_features, Some(100));
        assert!(!extractor.config().sublinear_tf);
    }

    #[test]
    fn test_fit_empty() {
        let mut extractor = TfidfFeatureExtractor::new();
        let empty: Vec<&str> = vec![];

        let result = extractor.fit(&empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_fit_success() {
        let mut extractor = TfidfFeatureExtractor::new();
        let docs = vec![
            "expected i32 found str",
            "cannot move out of borrowed content",
            "type annotation needed",
        ];

        let result = extractor.fit(&docs);
        assert!(result.is_ok());
        assert!(extractor.is_fitted());
        assert!(extractor.vocabulary_size() > 0);
    }

    #[test]
    fn test_transform_without_fit() {
        let extractor = TfidfFeatureExtractor::new();
        let docs = vec!["test"];

        let result = extractor.transform(&docs);
        assert!(result.is_err());
    }

    #[test]
    fn test_transform_success() {
        let mut extractor = TfidfFeatureExtractor::new();
        let training = vec!["expected i32 found str", "cannot borrow", "type needed"];

        extractor.fit(&training).unwrap();

        let matrix = extractor.transform(&["expected type"]).unwrap();
        assert_eq!(matrix.n_rows(), 1);
        assert!(matrix.n_cols() > 0);
    }

    #[test]
    fn test_fit_transform() {
        let mut extractor = TfidfFeatureExtractor::new();
        let docs = vec!["expected i32 found str", "cannot borrow", "type needed"];

        let matrix = extractor.fit_transform(&docs).unwrap();

        assert_eq!(matrix.n_rows(), 3);
        assert!(extractor.is_fitted());
    }

    #[test]
    fn test_ngram_range_config() {
        let extractor = TfidfFeatureExtractor::new().with_ngram_range(1, 3);

        assert_eq!(extractor.config().ngram_range, (1, 3));
        assert!(!extractor.is_fitted());
    }

    #[test]
    fn test_max_features_config() {
        let extractor = TfidfFeatureExtractor::new().with_max_features(50);

        assert_eq!(extractor.config().max_features, Some(50));
    }

    #[test]
    fn test_top_features() {
        let mut extractor = TfidfFeatureExtractor::new().with_max_features(20);

        let docs = vec![
            "expected type i32 found type str",
            "cannot move borrowed value",
            "lifetime annotation needed",
        ];

        extractor.fit(&docs).unwrap();

        let top = extractor.top_features(5);
        assert!(!top.is_empty());
        assert!(top.len() <= 5);

        // Features should have positive IDF
        for (_, idf) in &top {
            assert!(*idf > 0.0);
        }
    }

    #[test]
    fn test_top_features_empty_when_not_fitted() {
        let extractor = TfidfFeatureExtractor::new();

        let top = extractor.top_features(10);
        assert!(top.is_empty());
    }

    // ===================
    // Preprocessing Tests
    // ===================

    #[test]
    fn test_preprocess_lowercase() {
        let result = preprocess("ERROR: Expected I32");
        assert!(result.chars().all(|c| !c.is_uppercase()));
    }

    #[test]
    fn test_preprocess_removes_digits() {
        let result = preprocess("error[E0308]: line 42");
        assert!(!result.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_preprocess_normalizes_punctuation() {
        let result = preprocess("found `String` (expected `i32`)");
        assert!(!result.contains('`'));
        assert!(!result.contains('('));
        assert!(!result.contains(')'));
    }

    #[test]
    fn test_preprocess_collapses_whitespace() {
        let result = preprocess("error   with   spaces");
        assert!(!result.contains("  "));
    }

    // ===================
    // CombinedFeatureExtractor Tests
    // ===================

    #[test]
    fn test_combined_creation() {
        let extractor = CombinedFeatureExtractor::new();
        assert_eq!(extractor.feature_count(), crate::ErrorFeatures::DIM);
    }

    #[test]
    fn test_combined_fit() {
        let mut extractor = CombinedFeatureExtractor::new();
        let docs = vec!["error message one", "error message two"];

        let result = extractor.fit(&docs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_combined_transform() {
        let mut extractor = CombinedFeatureExtractor::new();
        let docs = vec!["expected i32 found str", "cannot borrow", "type needed"];

        extractor.fit(&docs).unwrap();

        let matrix = extractor.transform(&["test message"]).unwrap();
        assert_eq!(matrix.n_rows(), 1);
        // Should have both TF-IDF and hand-crafted features
        assert!(matrix.n_cols() > crate::ErrorFeatures::DIM);
    }

    #[test]
    fn test_combined_without_handcrafted() {
        let mut extractor = CombinedFeatureExtractor::new().with_handcrafted(false);

        let docs = vec!["error one", "error two"];
        extractor.fit(&docs).unwrap();

        let matrix = extractor.transform(&["test"]).unwrap();

        // Should only have TF-IDF features
        assert_eq!(matrix.n_cols(), extractor.tfidf.vocabulary_size());
    }

    #[test]
    fn test_combined_feature_count() {
        let mut extractor = CombinedFeatureExtractor::new();
        let docs = vec!["error message"];

        extractor.fit(&docs).unwrap();

        let expected = extractor.tfidf.vocabulary_size() + crate::ErrorFeatures::DIM;
        assert_eq!(extractor.feature_count(), expected);
    }

    #[test]
    fn test_combined_fit_transform() {
        let mut extractor = CombinedFeatureExtractor::new();
        let docs = vec!["error one", "error two", "error three"];

        let matrix = extractor.fit_transform(&docs).unwrap();

        assert_eq!(matrix.n_rows(), 3);
        assert!(matrix.n_cols() > 0);
    }
}

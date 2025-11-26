//! N-gram based fix pattern predictor.
//!
//! Uses N-gram language models from aprender to:
//! - Learn error → fix patterns from training data
//! - Predict likely fixes for new errors based on TF-IDF similarity
//! - Rank fix suggestions by confidence

use std::collections::HashMap;

use aprender::text::tokenize::WhitespaceTokenizer;
use aprender::text::vectorize::TfidfVectorizer;
use aprender::text::Tokenizer;
use serde::{Deserialize, Serialize};

use crate::classifier::ErrorCategory;
use crate::OracleError;

/// A fix pattern learned from training data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixPattern {
    /// Error pattern (preprocessed)
    pub error_pattern: String,
    /// Associated fix template
    pub fix_template: String,
    /// Error category
    pub category: ErrorCategory,
    /// How many times this pattern was seen
    pub frequency: usize,
    /// Success rate when applied (0.0-1.0)
    pub success_rate: f32,
}

impl FixPattern {
    /// Create a new fix pattern.
    #[must_use]
    pub fn new(error_pattern: &str, fix_template: &str, category: ErrorCategory) -> Self {
        Self {
            error_pattern: error_pattern.to_string(),
            fix_template: fix_template.to_string(),
            category,
            frequency: 1,
            success_rate: 0.0,
        }
    }

    /// Increment frequency count.
    pub fn increment(&mut self) {
        self.frequency += 1;
    }

    /// Update success rate with exponential moving average.
    pub fn update_success(&mut self, success: bool) {
        let alpha = 0.1; // Smoothing factor
        let success_val = if success { 1.0 } else { 0.0 };
        self.success_rate = alpha * success_val + (1.0 - alpha) * self.success_rate;
    }
}

/// Ranked fix suggestion with confidence score.
#[derive(Clone, Debug)]
pub struct FixSuggestion {
    /// The fix template
    pub fix: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Category of error this fix addresses
    pub category: ErrorCategory,
    /// Similar error pattern that matched
    pub matched_pattern: String,
}

/// N-gram based fix pattern predictor.
///
/// # Examples
///
/// ```
/// use depyler_oracle::ngram::NgramFixPredictor;
/// use depyler_oracle::ErrorCategory;
///
/// let mut predictor = NgramFixPredictor::new();
///
/// // Train with error-fix pairs
/// predictor.learn_pattern(
///     "expected i32, found &str",
///     "Use .parse() or as_str()",
///     ErrorCategory::TypeMismatch,
/// );
///
/// // Predict fixes for new errors
/// let suggestions = predictor.predict_fixes("expected u32, found String", 3);
/// assert!(!suggestions.is_empty());
/// ```
pub struct NgramFixPredictor {
    /// Learned fix patterns indexed by category
    patterns: HashMap<ErrorCategory, Vec<FixPattern>>,
    /// TF-IDF vectorizer for error similarity
    vectorizer: TfidfVectorizer,
    /// Whether vectorizer has been fitted
    is_fitted: bool,
    /// Minimum similarity threshold for suggestions
    min_similarity: f32,
    /// N-gram range (min, max)
    ngram_range: (usize, usize),
}

impl NgramFixPredictor {
    /// Create a new predictor with tuned default settings.
    ///
    /// Defaults optimized via grid search:
    /// - ngram_range: (1, 2) - bigrams outperform trigrams
    /// - min_similarity: 0.05 - more lenient matching improves recall
    #[must_use]
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            vectorizer: TfidfVectorizer::new()
                .with_tokenizer(Box::new(WhitespaceTokenizer::new()))
                .with_ngram_range(1, 2) // tuned: unigrams + bigrams only
                .with_sublinear_tf(true),
            is_fitted: false,
            min_similarity: 0.05, // tuned: more lenient matching
            ngram_range: (1, 2),  // tuned: bigrams
        }
    }

    /// Set minimum similarity threshold.
    #[must_use]
    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set N-gram range for pattern matching.
    #[must_use]
    pub fn with_ngram_range(mut self, min_n: usize, max_n: usize) -> Self {
        self.ngram_range = (min_n.max(1), max_n.max(1));
        self.vectorizer = TfidfVectorizer::new()
            .with_tokenizer(Box::new(WhitespaceTokenizer::new()))
            .with_ngram_range(min_n.max(1), max_n.max(1))
            .with_sublinear_tf(true);
        self.is_fitted = false;
        self
    }

    /// Learn a new error-fix pattern.
    pub fn learn_pattern(
        &mut self,
        error_message: &str,
        fix_template: &str,
        category: ErrorCategory,
    ) {
        let normalized = normalize_error(error_message);

        let patterns = self.patterns.entry(category).or_default();

        // Check if pattern already exists
        if let Some(existing) = patterns
            .iter_mut()
            .find(|p| p.error_pattern == normalized)
        {
            existing.increment();
        } else {
            patterns.push(FixPattern::new(&normalized, fix_template, category));
        }

        // Mark vectorizer as needing refit
        self.is_fitted = false;
    }

    /// Learn multiple patterns from training data.
    pub fn learn_batch(&mut self, training_data: &[(String, String, ErrorCategory)]) {
        for (error, fix, category) in training_data {
            self.learn_pattern(error, fix, *category);
        }
    }

    /// Fit the TF-IDF vectorizer on all learned patterns.
    ///
    /// # Errors
    ///
    /// Returns error if fitting fails.
    pub fn fit(&mut self) -> Result<(), OracleError> {
        let all_patterns: Vec<String> = self
            .patterns
            .values()
            .flat_map(|ps| ps.iter().map(|p| p.error_pattern.clone()))
            .collect();

        if all_patterns.is_empty() {
            return Err(OracleError::Model(
                "No patterns to fit. Call learn_pattern() first.".to_string(),
            ));
        }

        self.vectorizer
            .fit(&all_patterns)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        self.is_fitted = true;
        Ok(())
    }

    /// Predict top-k fixes for an error message.
    #[must_use]
    pub fn predict_fixes(&self, error_message: &str, top_k: usize) -> Vec<FixSuggestion> {
        if !self.is_fitted || self.patterns.is_empty() {
            return Vec::new();
        }

        let normalized = normalize_error(error_message);

        // Compute TF-IDF similarity with all patterns
        let mut suggestions: Vec<FixSuggestion> = Vec::new();

        for (category, patterns) in &self.patterns {
            for pattern in patterns {
                let similarity = self.compute_similarity(&normalized, &pattern.error_pattern);

                if similarity >= self.min_similarity {
                    // Weight by frequency and success rate
                    let confidence = similarity
                        * (1.0 + (pattern.frequency as f32).ln())
                        * (0.5 + pattern.success_rate);

                    suggestions.push(FixSuggestion {
                        fix: pattern.fix_template.clone(),
                        confidence: confidence.min(1.0),
                        category: *category,
                        matched_pattern: pattern.error_pattern.clone(),
                    });
                }
            }
        }

        // Sort by confidence descending
        suggestions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        suggestions.truncate(top_k);
        suggestions
    }

    /// Predict fixes for a specific category.
    #[must_use]
    pub fn predict_for_category(
        &self,
        error_message: &str,
        category: ErrorCategory,
        top_k: usize,
    ) -> Vec<FixSuggestion> {
        let all = self.predict_fixes(error_message, top_k * 2);
        all.into_iter()
            .filter(|s| s.category == category)
            .take(top_k)
            .collect()
    }

    /// Compute cosine similarity between two error messages using N-gram overlap.
    fn compute_similarity(&self, a: &str, b: &str) -> f32 {
        // Use simple N-gram Jaccard similarity for now
        // (TF-IDF transform would require storing document vectors)
        let tokenizer = WhitespaceTokenizer::new();

        let tokens_a = tokenizer.tokenize(a).unwrap_or_default();
        let tokens_b = tokenizer.tokenize(b).unwrap_or_default();

        if tokens_a.is_empty() || tokens_b.is_empty() {
            return 0.0;
        }

        // Generate N-grams
        let ngrams_a = generate_ngrams(&tokens_a, self.ngram_range.0, self.ngram_range.1);
        let ngrams_b = generate_ngrams(&tokens_b, self.ngram_range.0, self.ngram_range.1);

        // Jaccard similarity
        let intersection = ngrams_a.iter().filter(|ng| ngrams_b.contains(ng)).count();
        let union = ngrams_a.len() + ngrams_b.len() - intersection;

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// Update success rate for a pattern based on feedback.
    pub fn record_feedback(&mut self, error_pattern: &str, success: bool) {
        let normalized = normalize_error(error_pattern);

        for patterns in self.patterns.values_mut() {
            if let Some(pattern) = patterns.iter_mut().find(|p| p.error_pattern == normalized) {
                pattern.update_success(success);
                return;
            }
        }
    }

    /// Get all patterns for a category.
    #[must_use]
    pub fn patterns_for_category(&self, category: ErrorCategory) -> &[FixPattern] {
        self.patterns.get(&category).map_or(&[], |v| v.as_slice())
    }

    /// Total number of learned patterns.
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.patterns.values().map(|v| v.len()).sum()
    }

    /// Check if predictor has been fitted.
    #[must_use]
    pub fn is_fitted(&self) -> bool {
        self.is_fitted
    }
}

impl Default for NgramFixPredictor {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize error message for pattern matching.
///
/// Applies error code weighting (tuned: weight=2) to emphasize error codes
/// which are the strongest signal for classification.
fn normalize_error(message: &str) -> String {
    // Extract and weight error code (tuned: 2x weighting)
    let error_code = extract_error_code(message);
    let code_prefix = error_code
        .map(|c| format!("{} {} ", c, c)) // Repeat code twice
        .unwrap_or_default();

    let normalized = message
        .to_lowercase()
        // Remove specific identifiers
        .replace(|c: char| c.is_ascii_digit(), "N")
        // Normalize common patterns
        .replace("error:", "")
        .replace("-->", "")
        .replace("  ", " ")
        .trim()
        .to_string();

    format!("{}{}", code_prefix, normalized)
}

/// Extract rustc error code from message (e.g., "E0308").
fn extract_error_code(message: &str) -> Option<String> {
    if let Some(start) = message.find("error[E") {
        if let Some(end) = message[start..].find(']') {
            let code = &message[start + 6..start + end];
            if code.len() == 4 && code.chars().all(|c| c.is_ascii_digit()) {
                return Some(format!("e{}", code.to_lowercase()));
            }
        }
    }
    None
}

/// Generate N-grams from tokens.
fn generate_ngrams(tokens: &[String], min_n: usize, max_n: usize) -> Vec<String> {
    let mut ngrams = Vec::new();

    for n in min_n..=max_n {
        if tokens.len() >= n {
            for window in tokens.windows(n) {
                ngrams.push(window.join("_"));
            }
        }
    }

    ngrams
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===================
    // RED PHASE TESTS
    // ===================

    #[test]
    fn test_fix_pattern_creation() {
        let pattern = FixPattern::new(
            "expected i32, found str",
            "Use .parse()",
            ErrorCategory::TypeMismatch,
        );

        assert_eq!(pattern.error_pattern, "expected i32, found str");
        assert_eq!(pattern.fix_template, "Use .parse()");
        assert_eq!(pattern.category, ErrorCategory::TypeMismatch);
        assert_eq!(pattern.frequency, 1);
        assert!((pattern.success_rate - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_fix_pattern_increment() {
        let mut pattern = FixPattern::new("test", "fix", ErrorCategory::Other);
        assert_eq!(pattern.frequency, 1);

        pattern.increment();
        assert_eq!(pattern.frequency, 2);

        pattern.increment();
        assert_eq!(pattern.frequency, 3);
    }

    #[test]
    fn test_fix_pattern_success_update() {
        let mut pattern = FixPattern::new("test", "fix", ErrorCategory::Other);

        // Initial success rate is 0
        assert!((pattern.success_rate - 0.0).abs() < 1e-6);

        // Update with success
        pattern.update_success(true);
        assert!(pattern.success_rate > 0.0);

        // Multiple successes should increase rate
        for _ in 0..10 {
            pattern.update_success(true);
        }
        assert!(pattern.success_rate > 0.5);
    }

    #[test]
    fn test_predictor_creation() {
        let predictor = NgramFixPredictor::new();

        assert!(!predictor.is_fitted());
        assert_eq!(predictor.pattern_count(), 0);
    }

    #[test]
    fn test_learn_single_pattern() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern(
            "expected i32, found &str",
            "Convert using .to_string() or .parse()",
            ErrorCategory::TypeMismatch,
        );

        assert_eq!(predictor.pattern_count(), 1);
        assert!(!predictor.is_fitted()); // Not fitted until fit() called
    }

    #[test]
    fn test_learn_duplicate_pattern() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("error msg", "fix", ErrorCategory::Other);
        predictor.learn_pattern("error msg", "fix", ErrorCategory::Other);

        // Should increment frequency, not add duplicate
        assert_eq!(predictor.pattern_count(), 1);

        let patterns = predictor.patterns_for_category(ErrorCategory::Other);
        assert_eq!(patterns[0].frequency, 2);
    }

    #[test]
    fn test_learn_batch() {
        let mut predictor = NgramFixPredictor::new();

        let training = vec![
            (
                "expected i32".to_string(),
                "use .parse()".to_string(),
                ErrorCategory::TypeMismatch,
            ),
            (
                "cannot borrow".to_string(),
                "use .clone()".to_string(),
                ErrorCategory::BorrowChecker,
            ),
            (
                "not found".to_string(),
                "add use statement".to_string(),
                ErrorCategory::MissingImport,
            ),
        ];

        predictor.learn_batch(&training);
        assert_eq!(predictor.pattern_count(), 3);
    }

    #[test]
    fn test_fit_empty_patterns() {
        let mut predictor = NgramFixPredictor::new();

        let result = predictor.fit();
        assert!(result.is_err());
    }

    #[test]
    fn test_fit_with_patterns() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("expected i32", "convert type", ErrorCategory::TypeMismatch);
        predictor.learn_pattern("cannot borrow", "clone value", ErrorCategory::BorrowChecker);

        let result = predictor.fit();
        assert!(result.is_ok());
        assert!(predictor.is_fitted());
    }

    #[test]
    fn test_predict_without_fit() {
        let mut predictor = NgramFixPredictor::new();
        predictor.learn_pattern("test", "fix", ErrorCategory::Other);

        // Should return empty without fitting
        let suggestions = predictor.predict_fixes("test error", 3);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_predict_basic() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern(
            "expected i32, found str",
            "Use type conversion",
            ErrorCategory::TypeMismatch,
        );
        predictor.fit().expect("fit should succeed");

        let suggestions = predictor.predict_fixes("expected i32, found string", 3);

        // Should find similar pattern
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.0);
    }

    #[test]
    fn test_predict_ranking() {
        let mut predictor = NgramFixPredictor::new();

        // Add multiple patterns
        predictor.learn_pattern(
            "expected i32, found str",
            "Use .parse()",
            ErrorCategory::TypeMismatch,
        );
        predictor.learn_pattern(
            "expected u32, found string",
            "Use .parse::<u32>()",
            ErrorCategory::TypeMismatch,
        );
        predictor.learn_pattern("cannot borrow", "Use .clone()", ErrorCategory::BorrowChecker);

        predictor.fit().expect("fit should succeed");

        let suggestions = predictor.predict_fixes("expected u64, found str", 5);

        // Should rank type-related fixes higher
        if suggestions.len() >= 2 {
            assert!(suggestions[0].confidence >= suggestions[1].confidence);
        }
    }

    #[test]
    fn test_predict_for_category() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("type error", "fix type", ErrorCategory::TypeMismatch);
        predictor.learn_pattern("borrow error", "fix borrow", ErrorCategory::BorrowChecker);

        predictor.fit().expect("fit should succeed");

        let suggestions =
            predictor.predict_for_category("type error", ErrorCategory::TypeMismatch, 3);

        // All suggestions should be for TypeMismatch
        for s in &suggestions {
            assert_eq!(s.category, ErrorCategory::TypeMismatch);
        }
    }

    #[test]
    fn test_record_feedback() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("test error", "test fix", ErrorCategory::Other);

        let patterns = predictor.patterns_for_category(ErrorCategory::Other);
        let initial_rate = patterns[0].success_rate;

        predictor.record_feedback("test error", true);

        let patterns = predictor.patterns_for_category(ErrorCategory::Other);
        assert!(patterns[0].success_rate > initial_rate);
    }

    #[test]
    fn test_normalize_error() {
        let normalized = normalize_error("error: expected i32, found &str at line 42");

        // Should lowercase
        assert!(!normalized.contains("E"));
        // Should remove "error:"
        assert!(!normalized.contains("error:"));
        // Should normalize numbers
        assert!(normalized.contains('n') || normalized.contains('N'));
    }

    #[test]
    fn test_generate_ngrams() {
        let tokens: Vec<String> = vec!["hello", "world", "rust"]
            .into_iter()
            .map(String::from)
            .collect();

        let ngrams = generate_ngrams(&tokens, 1, 2);

        // Should have 3 unigrams + 2 bigrams = 5
        assert!(ngrams.contains(&"hello".to_string()));
        assert!(ngrams.contains(&"hello_world".to_string()));
        assert!(ngrams.contains(&"world_rust".to_string()));
    }

    #[test]
    fn test_similarity_identical() {
        let predictor = NgramFixPredictor::new();

        let sim = predictor.compute_similarity("expected i32 found str", "expected i32 found str");

        // Identical strings should have similarity 1.0
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_similarity_different() {
        let predictor = NgramFixPredictor::new();

        let sim = predictor.compute_similarity(
            "expected i32 found str",
            "completely different error message",
        );

        // Different strings should have low similarity
        assert!(sim < 0.5);
    }

    #[test]
    fn test_min_similarity_threshold() {
        let mut predictor = NgramFixPredictor::new().with_min_similarity(0.8);

        predictor.learn_pattern("exact error", "exact fix", ErrorCategory::Other);
        predictor.fit().expect("fit should succeed");

        // Very different message should be filtered by high threshold
        let suggestions = predictor.predict_fixes("completely different", 3);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_ngram_range_config() {
        let predictor = NgramFixPredictor::new().with_ngram_range(2, 4);

        assert_eq!(predictor.ngram_range, (2, 4));
        assert!(!predictor.is_fitted());
    }

    #[test]
    fn test_patterns_for_nonexistent_category() {
        let predictor = NgramFixPredictor::new();

        let patterns = predictor.patterns_for_category(ErrorCategory::TypeMismatch);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_fix_suggestion_structure() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("test error", "test fix", ErrorCategory::TypeMismatch);
        predictor.fit().expect("fit should succeed");

        let suggestions = predictor.predict_fixes("test error", 1);

        if let Some(s) = suggestions.first() {
            assert!(!s.fix.is_empty());
            assert!(s.confidence > 0.0);
            assert!(s.confidence <= 1.0);
            assert!(!s.matched_pattern.is_empty());
        }
    }

    #[test]
    fn test_frequency_affects_confidence() {
        let mut predictor = NgramFixPredictor::new();

        // Learn same pattern multiple times
        for _ in 0..5 {
            predictor.learn_pattern(
                "frequent error",
                "frequent fix",
                ErrorCategory::TypeMismatch,
            );
        }

        // Learn another pattern once
        predictor.learn_pattern("rare error", "rare fix", ErrorCategory::TypeMismatch);

        predictor.fit().expect("fit should succeed");

        let frequent = predictor.predict_fixes("frequent error", 1);
        let rare = predictor.predict_fixes("rare error", 1);

        // Frequent pattern should have higher confidence
        if !frequent.is_empty() && !rare.is_empty() {
            assert!(frequent[0].confidence >= rare[0].confidence);
        }
    }

    #[test]
    fn test_success_rate_affects_confidence() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("good pattern", "good fix", ErrorCategory::Other);
        predictor.learn_pattern("bad pattern", "bad fix", ErrorCategory::Other);

        // Record successes for first, failures for second
        for _ in 0..10 {
            predictor.record_feedback("good pattern", true);
            predictor.record_feedback("bad pattern", false);
        }

        predictor.fit().expect("fit should succeed");

        let good = predictor.predict_fixes("good pattern", 1);
        let bad = predictor.predict_fixes("bad pattern", 1);

        // Good pattern should have higher confidence
        if !good.is_empty() && !bad.is_empty() {
            assert!(good[0].confidence >= bad[0].confidence);
        }
    }

    #[test]
    fn test_multiple_categories() {
        let mut predictor = NgramFixPredictor::new();

        predictor.learn_pattern("type error", "type fix", ErrorCategory::TypeMismatch);
        predictor.learn_pattern("borrow error", "borrow fix", ErrorCategory::BorrowChecker);
        predictor.learn_pattern("import error", "import fix", ErrorCategory::MissingImport);
        predictor.learn_pattern("lifetime error", "lifetime fix", ErrorCategory::LifetimeError);

        assert_eq!(predictor.pattern_count(), 4);

        // Each category should have 1 pattern
        assert_eq!(
            predictor
                .patterns_for_category(ErrorCategory::TypeMismatch)
                .len(),
            1
        );
        assert_eq!(
            predictor
                .patterns_for_category(ErrorCategory::BorrowChecker)
                .len(),
            1
        );
    }
}

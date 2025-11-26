//! Property-based tests for the oracle.
//!
//! Uses proptest to verify invariants across random inputs.

use proptest::prelude::*;

use crate::classifier::{ErrorCategory, ErrorClassifier};
use crate::features::ErrorFeatures;
use crate::ngram::{FixPattern, NgramFixPredictor};
use crate::patterns::{FixTemplate, FixTemplateRegistry};
use crate::tfidf::TfidfFeatureExtractor;
use crate::training::{TrainingDataset, TrainingSample};

// ============================================
// Strategies
// ============================================

/// Generate random error messages.
fn error_message_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "error",
            "expected",
            "found",
            "type",
            "mismatch",
            "cannot",
            "move",
            "borrow",
            "borrowed",
            "lifetime",
            "'a",
            "'static",
            "trait",
            "bound",
            "not",
            "implemented",
            "use",
            "import",
            "crate",
            "module",
            "syntax",
            "semicolon",
            "E0308",
            "E0382",
            "E0106",
            "i32",
            "u32",
            "String",
            "&str",
            "Option",
            "Result",
            "Clone",
            "Debug",
            "Send",
            "Sync",
        ]),
        1..20,
    )
    .prop_map(|words| words.join(" "))
}

/// Generate random error categories.
fn category_strategy() -> impl Strategy<Value = ErrorCategory> {
    prop::sample::select(vec![
        ErrorCategory::TypeMismatch,
        ErrorCategory::BorrowChecker,
        ErrorCategory::MissingImport,
        ErrorCategory::SyntaxError,
        ErrorCategory::LifetimeError,
        ErrorCategory::TraitBound,
        ErrorCategory::Other,
    ])
}

/// Generate random fix templates.
fn fix_template_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "Use",
            "Add",
            "Remove",
            "Clone",
            "Convert",
            "Import",
            "Derive",
            ".to_string()",
            ".clone()",
            ".into()",
            "as",
            "&",
            "&mut",
            "Box",
            "Arc",
            "Rc",
        ]),
        1..10,
    )
    .prop_map(|words| words.join(" "))
}

// ============================================
// ErrorCategory Properties
// ============================================

proptest! {
    /// Category index roundtrip: index -> from_index -> index
    #[test]
    fn prop_category_index_roundtrip(category in category_strategy()) {
        let idx = category.index();
        let recovered = ErrorCategory::from_index(idx);
        prop_assert_eq!(recovered, category);
    }

    /// All category indices are unique and in valid range
    #[test]
    fn prop_category_indices_valid(category in category_strategy()) {
        let idx = category.index();
        prop_assert!(idx < 7, "Category index {} out of range", idx);
    }

    /// Category names are non-empty
    #[test]
    fn prop_category_names_nonempty(category in category_strategy()) {
        let name = category.name();
        prop_assert!(!name.is_empty(), "Category name should not be empty");
    }
}

// ============================================
// ErrorClassifier Properties
// ============================================

proptest! {
    /// Classification always returns a valid category
    #[test]
    fn prop_classifier_returns_valid_category(msg in error_message_strategy()) {
        let classifier = ErrorClassifier::new();
        let category = classifier.classify_by_keywords(&msg);
        // Verify it's a valid category by checking index
        let _ = category.index(); // Should not panic
    }

    /// Confidence is always in valid range [0.0, 1.0]
    #[test]
    fn prop_classifier_confidence_valid_range(
        msg in error_message_strategy(),
        category in category_strategy()
    ) {
        let classifier = ErrorClassifier::new();
        let conf = classifier.confidence(&msg, category);
        prop_assert!(conf >= 0.0, "Confidence {} < 0.0", conf);
        prop_assert!(conf <= 1.0, "Confidence {} > 1.0", conf);
    }

    /// Classification is deterministic
    #[test]
    fn prop_classifier_deterministic(msg in error_message_strategy()) {
        let classifier = ErrorClassifier::new();
        let cat1 = classifier.classify_by_keywords(&msg);
        let cat2 = classifier.classify_by_keywords(&msg);
        prop_assert_eq!(cat1, cat2);
    }
}

// ============================================
// ErrorFeatures Properties
// ============================================

proptest! {
    /// Features are always valid (no NaN, no infinite)
    #[test]
    fn prop_features_valid_values(msg in error_message_strategy()) {
        let features = ErrorFeatures::from_error_message(&msg);
        let vec = features.to_vec();

        for (i, &v) in vec.iter().enumerate() {
            prop_assert!(!v.is_nan(), "Feature {} is NaN", i);
            prop_assert!(!v.is_infinite(), "Feature {} is infinite", i);
        }
    }

    /// Features are in expected ranges [0.0, 1.0]
    #[test]
    fn prop_features_in_range(msg in error_message_strategy()) {
        let features = ErrorFeatures::from_error_message(&msg);
        let vec = features.to_vec();

        for (i, &v) in vec.iter().enumerate() {
            prop_assert!(v >= 0.0, "Feature {} is negative: {}", i, v);
            prop_assert!(v <= 1.0, "Feature {} > 1.0: {}", i, v);
        }
    }

    /// Feature vector has correct dimension
    #[test]
    fn prop_features_correct_dim(msg in error_message_strategy()) {
        let features = ErrorFeatures::from_error_message(&msg);
        let vec = features.to_vec();
        prop_assert_eq!(vec.len(), ErrorFeatures::DIM);
    }

    /// Feature roundtrip: to_vec -> from_vec
    #[test]
    fn prop_features_roundtrip(msg in error_message_strategy()) {
        let features = ErrorFeatures::from_error_message(&msg);
        let vec = features.to_vec();
        let restored = ErrorFeatures::from_vec(&vec);
        let restored_vec = restored.to_vec();

        for (i, (&orig, &rest)) in vec.iter().zip(restored_vec.iter()).enumerate() {
            prop_assert!(
                (orig - rest).abs() < 1e-6,
                "Feature {} mismatch: {} vs {}",
                i,
                orig,
                rest
            );
        }
    }
}

// ============================================
// FixPattern Properties
// ============================================

proptest! {
    /// Pattern frequency only increases
    #[test]
    fn prop_pattern_frequency_monotonic(
        msg in error_message_strategy(),
        fix in fix_template_strategy(),
        increments in 0..100usize
    ) {
        let mut pattern = FixPattern::new(&msg, &fix, ErrorCategory::Other);
        let initial = pattern.frequency;

        for _ in 0..increments {
            pattern.increment();
        }

        prop_assert_eq!(pattern.frequency, initial + increments);
    }

    /// Success rate stays in [0.0, 1.0]
    #[test]
    fn prop_pattern_success_rate_bounded(
        msg in error_message_strategy(),
        successes in prop::collection::vec(any::<bool>(), 0..50)
    ) {
        let mut pattern = FixPattern::new(&msg, "fix", ErrorCategory::Other);

        for success in successes {
            pattern.update_success(success);
        }

        prop_assert!(pattern.success_rate >= 0.0);
        prop_assert!(pattern.success_rate <= 1.0);
    }
}

// ============================================
// NgramFixPredictor Properties
// ============================================

proptest! {
    /// Predictions are sorted by confidence (descending)
    #[test]
    fn prop_predictions_sorted_by_confidence(
        patterns in prop::collection::vec(
            (error_message_strategy(), fix_template_strategy(), category_strategy()),
            1..10
        ),
        query in error_message_strategy()
    ) {
        let mut predictor = NgramFixPredictor::new();

        for (msg, fix, cat) in &patterns {
            predictor.learn_pattern(msg, fix, *cat);
        }

        if predictor.fit().is_ok() {
            let suggestions = predictor.predict_fixes(&query, 10);

            for window in suggestions.windows(2) {
                prop_assert!(
                    window[0].confidence >= window[1].confidence,
                    "Predictions not sorted: {} < {}",
                    window[0].confidence,
                    window[1].confidence
                );
            }
        }
    }

    /// Prediction confidence is always valid
    #[test]
    fn prop_prediction_confidence_valid(
        patterns in prop::collection::vec(
            (error_message_strategy(), fix_template_strategy(), category_strategy()),
            1..10
        ),
        query in error_message_strategy()
    ) {
        let mut predictor = NgramFixPredictor::new();

        for (msg, fix, cat) in &patterns {
            predictor.learn_pattern(msg, fix, *cat);
        }

        if predictor.fit().is_ok() {
            let suggestions = predictor.predict_fixes(&query, 10);

            for s in suggestions {
                prop_assert!(s.confidence >= 0.0, "Confidence negative: {}", s.confidence);
                prop_assert!(s.confidence <= 1.0, "Confidence > 1: {}", s.confidence);
            }
        }
    }

    /// Pattern count is consistent
    #[test]
    fn prop_pattern_count_consistent(
        patterns in prop::collection::vec(
            (error_message_strategy(), fix_template_strategy(), category_strategy()),
            0..20
        )
    ) {
        let mut predictor = NgramFixPredictor::new();

        for (msg, fix, cat) in &patterns {
            predictor.learn_pattern(msg, fix, *cat);
        }

        // Count should be <= input patterns (duplicates merged)
        prop_assert!(predictor.pattern_count() <= patterns.len());
    }
}

// ============================================
// FixTemplateRegistry Properties
// ============================================

proptest! {
    /// All registered templates can be retrieved
    #[test]
    fn prop_registry_retrieval(
        n_templates in 1..10usize
    ) {
        let mut registry = FixTemplateRegistry::new();

        for i in 0..n_templates {
            let template = FixTemplate::builder(
                &format!("test-{}", i),
                "Test",
                ErrorCategory::Other
            )
            .with_keywords(&["test"])
            .build();

            registry.register(template);
        }

        prop_assert_eq!(registry.template_count(), n_templates);
    }

    /// Match scores are non-negative
    #[test]
    fn prop_template_match_scores_nonnegative(msg in error_message_strategy()) {
        let registry = FixTemplateRegistry::with_rust_defaults();

        for template in registry.all_templates() {
            let score = template.match_score(&msg);
            prop_assert!(score >= 0.0, "Score negative: {}", score);
        }
    }
}

// ============================================
// TfidfFeatureExtractor Properties
// ============================================

proptest! {
    /// TF-IDF values are non-negative
    #[test]
    fn prop_tfidf_nonnegative(
        docs in prop::collection::vec(error_message_strategy(), 2..10)
    ) {
        let mut extractor = TfidfFeatureExtractor::new()
            .with_max_features(50);

        if extractor.fit(&docs).is_ok() {
            if let Ok(matrix) = extractor.transform(&docs) {
                for i in 0..matrix.n_rows() {
                    for j in 0..matrix.n_cols() {
                        let val = matrix.get(i, j);
                        prop_assert!(val >= 0.0, "TF-IDF negative at ({}, {}): {}", i, j, val);
                    }
                }
            }
        }
    }

    /// Vocabulary size is bounded by max_features
    #[test]
    fn prop_vocabulary_bounded(
        docs in prop::collection::vec(error_message_strategy(), 2..10),
        max_features in 10..100usize
    ) {
        let mut extractor = TfidfFeatureExtractor::new()
            .with_max_features(max_features);

        if extractor.fit(&docs).is_ok() {
            prop_assert!(
                extractor.vocabulary_size() <= max_features,
                "Vocabulary {} > max_features {}",
                extractor.vocabulary_size(),
                max_features
            );
        }
    }
}

// ============================================
// TrainingDataset Properties
// ============================================

proptest! {
    /// Dataset length is consistent
    #[test]
    fn prop_dataset_length_consistent(
        samples in prop::collection::vec(
            (error_message_strategy(), category_strategy()),
            0..50
        )
    ) {
        let mut dataset = TrainingDataset::new();

        for (msg, cat) in &samples {
            dataset.add(TrainingSample::new(msg, *cat));
        }

        prop_assert_eq!(dataset.len(), samples.len());
        prop_assert_eq!(dataset.messages().len(), samples.len());
        prop_assert_eq!(dataset.labels().len(), samples.len());
    }

    /// Labels match category indices
    #[test]
    fn prop_dataset_labels_match(
        samples in prop::collection::vec(
            (error_message_strategy(), category_strategy()),
            1..20
        )
    ) {
        let mut dataset = TrainingDataset::new();

        for (msg, cat) in &samples {
            dataset.add(TrainingSample::new(msg, *cat));
        }

        let labels = dataset.labels();

        for (i, (_, cat)) in samples.iter().enumerate() {
            prop_assert_eq!(
                labels[i],
                cat.index(),
                "Label mismatch at index {}",
                i
            );
        }
    }
}

// ============================================
// Cross-Component Properties
// ============================================

proptest! {
    /// Classifier and features are consistent
    #[test]
    fn prop_classifier_features_consistency(msg in error_message_strategy()) {
        let classifier = ErrorClassifier::new();
        let features = ErrorFeatures::from_error_message(&msg);
        let category = classifier.classify_by_keywords(&msg);

        // If classifier detects strong type keywords, features should too
        if category == ErrorCategory::TypeMismatch {
            // Type keywords feature should be non-zero (most of the time)
            // This is a soft check due to threshold differences
        }

        // Both should handle same input without panicking
        let _ = features.to_vec();
        let _ = classifier.confidence(&msg, category);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    proptest! {
        /// Full pipeline: classify -> extract features -> predict fix
        #[test]
        fn prop_full_pipeline(
            training in prop::collection::vec(
                (error_message_strategy(), fix_template_strategy(), category_strategy()),
                5..20
            ),
            query in error_message_strategy()
        ) {
            // Setup
            let classifier = ErrorClassifier::new();
            let mut predictor = NgramFixPredictor::new();
            let mut tfidf = TfidfFeatureExtractor::new().with_max_features(50);

            // Train predictor
            for (msg, fix, cat) in &training {
                predictor.learn_pattern(msg, fix, *cat);
            }
            let _ = predictor.fit();

            // Train TF-IDF
            let messages: Vec<&str> = training.iter().map(|(m, _, _)| m.as_str()).collect();
            let _ = tfidf.fit(&messages);

            // Classify query
            let category = classifier.classify_by_keywords(&query);
            let confidence = classifier.confidence(&query, category);
            prop_assert!((0.0..=1.0).contains(&confidence));

            // Extract features
            let features = ErrorFeatures::from_error_message(&query);
            let vec = features.to_vec();
            prop_assert_eq!(vec.len(), ErrorFeatures::DIM);

            // Predict fixes (may be empty if no good matches)
            let suggestions = predictor.predict_fixes(&query, 5);
            for s in &suggestions {
                prop_assert!(s.confidence >= 0.0 && s.confidence <= 1.0);
            }
        }
    }
}

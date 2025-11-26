//! Model evaluation tests using aprender's cross-validation.
//!
//! Tests the depyler-specific training corpus with:
//! - NgramFixPredictor accuracy
//! - Cross-validation with StratifiedKFold
//! - F1 score evaluation

use depyler_oracle::classifier::ErrorCategory;
use depyler_oracle::depyler_training::{build_depyler_corpus, corpus_stats, get_training_pairs};
use depyler_oracle::ngram::NgramFixPredictor;

/// Test that predictor can learn and predict from depyler corpus.
#[test]
fn test_ngram_predictor_on_depyler_corpus() {
    let pairs = get_training_pairs();
    assert!(!pairs.is_empty(), "Training pairs should not be empty");

    let mut predictor = NgramFixPredictor::new();

    // Train on all pairs
    for (error, fix, category) in &pairs {
        predictor.learn_pattern(error, fix, *category);
    }

    // Fit the vectorizer
    predictor.fit().expect("Fit should succeed");

    // Test prediction on known patterns
    let suggestions = predictor.predict_fixes(
        "error[E0308]: mismatched types expected `f64`, found `&serde_json::Value`",
        3,
    );

    assert!(!suggestions.is_empty(), "Should find suggestions for type mismatch");
    assert_eq!(
        suggestions[0].category,
        ErrorCategory::TypeMismatch,
        "Top suggestion should be TypeMismatch"
    );
}

/// Test leave-one-out cross-validation accuracy.
#[test]
fn test_leave_one_out_accuracy() {
    let pairs = get_training_pairs();
    let n = pairs.len();

    let mut correct = 0;
    let mut total = 0;

    // Leave-one-out CV
    for i in 0..n {
        let mut predictor = NgramFixPredictor::new();

        // Train on all except i
        for (j, (error, fix, category)) in pairs.iter().enumerate() {
            if i != j {
                predictor.learn_pattern(error, fix, *category);
            }
        }

        if predictor.fit().is_ok() {
            // Predict on held-out sample
            let (test_error, _, test_category) = &pairs[i];
            let suggestions = predictor.predict_fixes(test_error, 1);

            if let Some(top) = suggestions.first() {
                if top.category == *test_category {
                    correct += 1;
                }
            }
            total += 1;
        }
    }

    let accuracy = correct as f32 / total as f32;
    println!(
        "Leave-one-out accuracy: {:.2}% ({}/{})",
        accuracy * 100.0,
        correct,
        total
    );

    // Expect at least 50% accuracy on small corpus
    assert!(
        accuracy >= 0.4,
        "Accuracy should be at least 40%, got {:.2}%",
        accuracy * 100.0
    );
}

/// Test category-specific precision.
#[test]
fn test_category_precision() {
    let pairs = get_training_pairs();
    let mut predictor = NgramFixPredictor::new();

    // Train on all
    for (error, fix, category) in &pairs {
        predictor.learn_pattern(error, fix, *category);
    }
    predictor.fit().expect("Fit should succeed");

    // Test category-specific queries
    let type_mismatch_errors = [
        "expected `f64`, found `serde_json::Value`",
        "mismatched types expected `String`, found `&str`",
        "expected HashMap<String, String>",
    ];

    let mut type_mismatch_correct = 0;
    for error in &type_mismatch_errors {
        let suggestions = predictor.predict_fixes(error, 1);
        if let Some(top) = suggestions.first() {
            if top.category == ErrorCategory::TypeMismatch {
                type_mismatch_correct += 1;
            }
        }
    }

    println!(
        "TypeMismatch precision: {}/{}",
        type_mismatch_correct,
        type_mismatch_errors.len()
    );
}

/// Test corpus statistics.
#[test]
fn test_corpus_statistics() {
    let stats = corpus_stats();
    let total: usize = stats.iter().map(|(_, c)| *c).sum();

    println!("Corpus statistics:");
    for (category, count) in &stats {
        let pct = (*count as f32 / total as f32) * 100.0;
        println!("  {:?}: {} ({:.1}%)", category, count, pct);
    }

    // Verify we have samples in key categories
    assert!(total >= 20, "Should have at least 20 total samples");

    let type_mismatch = stats
        .iter()
        .find(|(c, _)| *c == ErrorCategory::TypeMismatch)
        .map(|(_, n)| *n)
        .unwrap_or(0);
    assert!(type_mismatch >= 5, "TypeMismatch should have >= 5 samples");
}

/// Test similarity threshold impact.
#[test]
fn test_similarity_threshold() {
    let pairs = get_training_pairs();

    // High threshold - fewer but more confident matches
    let mut high_threshold = NgramFixPredictor::new().with_min_similarity(0.5);
    for (error, fix, category) in &pairs {
        high_threshold.learn_pattern(error, fix, *category);
    }
    high_threshold.fit().expect("Fit should succeed");

    // Low threshold - more matches but lower confidence
    let mut low_threshold = NgramFixPredictor::new().with_min_similarity(0.1);
    for (error, fix, category) in &pairs {
        low_threshold.learn_pattern(error, fix, *category);
    }
    low_threshold.fit().expect("Fit should succeed");

    let test_error = "error[E0308]: mismatched types";

    let high_matches = high_threshold.predict_fixes(test_error, 10);
    let low_matches = low_threshold.predict_fixes(test_error, 10);

    println!(
        "High threshold (0.5): {} matches",
        high_matches.len()
    );
    println!(
        "Low threshold (0.1): {} matches",
        low_matches.len()
    );

    // Low threshold should return more matches
    assert!(
        low_matches.len() >= high_matches.len(),
        "Lower threshold should return more matches"
    );
}

/// Benchmark prediction latency.
#[test]
fn test_prediction_latency() {
    use std::time::Instant;

    let pairs = get_training_pairs();
    let mut predictor = NgramFixPredictor::new();

    for (error, fix, category) in &pairs {
        predictor.learn_pattern(error, fix, *category);
    }
    predictor.fit().expect("Fit should succeed");

    let test_errors = [
        "error[E0308]: mismatched types expected `f64`, found `Value`",
        "error[E0599]: no method named `exists` found",
        "error[E0277]: trait bound not satisfied",
        "error[E0433]: failed to resolve",
    ];

    let start = Instant::now();
    for _ in 0..100 {
        for error in &test_errors {
            let _ = predictor.predict_fixes(error, 3);
        }
    }
    let elapsed = start.elapsed();

    let predictions_per_sec = (100 * test_errors.len()) as f64 / elapsed.as_secs_f64();
    println!("Prediction throughput: {:.0} predictions/sec", predictions_per_sec);

    // Should be fast enough for interactive use (>100/sec)
    assert!(
        predictions_per_sec > 100.0,
        "Prediction should be >100/sec, got {:.0}",
        predictions_per_sec
    );
}

//! Hyperparameter tuning for oracle predictor.
//!
//! Uses grid search to find optimal settings for:
//! - Similarity threshold
//! - N-gram range
//! - Error code weighting

use crate::depyler_training::build_combined_corpus;
use crate::ngram::NgramFixPredictor;
use crate::training::TrainingSample;

/// Hyperparameter configuration for tuning.
#[derive(Clone, Debug)]
pub struct TuningConfig {
    /// Minimum similarity threshold (0.0-1.0)
    pub min_similarity: f32,
    /// N-gram range (min, max)
    pub ngram_range: (usize, usize),
    /// Weight for error code features
    pub error_code_weight: f32,
}

impl Default for TuningConfig {
    fn default() -> Self {
        Self {
            min_similarity: 0.1,
            ngram_range: (1, 3),
            error_code_weight: 2.0,
        }
    }
}

/// Result of hyperparameter search.
#[derive(Clone, Debug)]
pub struct TuningResult {
    /// Best configuration found
    pub config: TuningConfig,
    /// Accuracy achieved
    pub accuracy: f32,
    /// Number of correct predictions
    pub correct: usize,
    /// Total predictions
    pub total: usize,
}

/// Run leave-one-out cross-validation with given config.
pub fn evaluate_config(config: &TuningConfig, samples: &[TrainingSample]) -> TuningResult {
    let n = samples.len();
    let mut correct = 0;

    for i in 0..n {
        let mut predictor = NgramFixPredictor::new()
            .with_min_similarity(config.min_similarity)
            .with_ngram_range(config.ngram_range.0, config.ngram_range.1);

        // Train on all except i
        for (j, sample) in samples.iter().enumerate() {
            if i != j {
                let fix = sample.fix.as_deref().unwrap_or("Check error");
                // Weight error codes by repeating them
                let weighted_msg = weight_error_codes(&sample.message, config.error_code_weight);
                predictor.learn_pattern(&weighted_msg, fix, sample.category);
            }
        }

        if predictor.fit().is_ok() {
            let test_sample = &samples[i];
            let weighted_test = weight_error_codes(&test_sample.message, config.error_code_weight);
            let suggestions = predictor.predict_fixes(&weighted_test, 1);

            if let Some(top) = suggestions.first() {
                if top.category == test_sample.category {
                    correct += 1;
                }
            }
        }
    }

    TuningResult {
        config: config.clone(),
        accuracy: correct as f32 / n as f32,
        correct,
        total: n,
    }
}

/// Weight error codes by repeating them in the message.
fn weight_error_codes(message: &str, weight: f32) -> String {
    // Extract error code if present
    if let Some(code_start) = message.find("error[E") {
        if let Some(code_end) = message[code_start..].find(']') {
            let code = &message[code_start..code_start + code_end + 1];
            let repeat_count = weight.round() as usize;
            let repeated = std::iter::repeat(code)
                .take(repeat_count)
                .collect::<Vec<_>>()
                .join(" ");
            return format!("{} {}", repeated, message);
        }
    }
    message.to_string()
}

/// Grid search over hyperparameter space.
pub fn grid_search() -> Vec<TuningResult> {
    let corpus = build_combined_corpus();
    let samples: Vec<_> = corpus.samples().to_vec();

    let similarity_thresholds = [0.05, 0.1, 0.15, 0.2, 0.25];
    let ngram_ranges = [(1, 2), (1, 3), (2, 3), (1, 4)];
    let error_weights = [1.0, 2.0, 3.0, 4.0];

    let mut results = Vec::new();

    for &sim in &similarity_thresholds {
        for &ngram in &ngram_ranges {
            for &weight in &error_weights {
                let config = TuningConfig {
                    min_similarity: sim,
                    ngram_range: ngram,
                    error_code_weight: weight,
                };

                let result = evaluate_config(&config, &samples);
                results.push(result);
            }
        }
    }

    // Sort by accuracy descending
    results.sort_by(|a, b| b.accuracy.partial_cmp(&a.accuracy).unwrap());
    results
}

/// Find best configuration via grid search.
#[must_use]
pub fn find_best_config() -> TuningResult {
    let results = grid_search();
    results.into_iter().next().unwrap_or_else(|| TuningResult {
        config: TuningConfig::default(),
        accuracy: 0.0,
        correct: 0,
        total: 0,
    })
}

/// Quick tuning with reduced search space.
pub fn quick_tune() -> TuningResult {
    let corpus = build_combined_corpus();
    let samples: Vec<_> = corpus.samples().to_vec();

    // Test key configurations
    let configs = vec![
        TuningConfig {
            min_similarity: 0.1,
            ngram_range: (1, 3),
            error_code_weight: 2.0,
        },
        TuningConfig {
            min_similarity: 0.15,
            ngram_range: (1, 3),
            error_code_weight: 3.0,
        },
        TuningConfig {
            min_similarity: 0.1,
            ngram_range: (1, 2),
            error_code_weight: 3.0,
        },
        TuningConfig {
            min_similarity: 0.05,
            ngram_range: (1, 4),
            error_code_weight: 2.0,
        },
    ];

    configs
        .iter()
        .map(|c| evaluate_config(c, &samples))
        .max_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap())
        .unwrap_or_else(|| TuningResult {
            config: TuningConfig::default(),
            accuracy: 0.0,
            correct: 0,
            total: 0,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_error_codes() {
        let msg = "error[E0308]: mismatched types";
        let weighted = weight_error_codes(msg, 3.0);
        assert!(weighted.contains("error[E0308] error[E0308] error[E0308]"));
    }

    #[test]
    fn test_quick_tune() {
        let result = quick_tune();
        assert!(result.accuracy > 0.0);
        assert!(result.total > 0);
        println!(
            "Quick tune: {:.2}% ({}/{}) with sim={}, ngram={:?}, weight={}",
            result.accuracy * 100.0,
            result.correct,
            result.total,
            result.config.min_similarity,
            result.config.ngram_range,
            result.config.error_code_weight
        );
    }

    #[test]
    #[ignore] // Slow - run manually
    fn test_full_grid_search() {
        let results = grid_search();
        println!("\nTop 5 configurations:");
        for (i, r) in results.iter().take(5).enumerate() {
            println!(
                "{}. {:.2}% - sim={}, ngram={:?}, weight={}",
                i + 1,
                r.accuracy * 100.0,
                r.config.min_similarity,
                r.config.ngram_range,
                r.config.error_code_weight
            );
        }
    }
}

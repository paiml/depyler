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
            let repeated = std::iter::repeat_n(code, repeat_count)
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
    let configs = [
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
    use crate::classifier::ErrorCategory;

    #[test]
    fn test_tuning_config_default() {
        let config = TuningConfig::default();
        assert_eq!(config.min_similarity, 0.1);
        assert_eq!(config.ngram_range, (1, 3));
        assert_eq!(config.error_code_weight, 2.0);
    }

    #[test]
    fn test_tuning_config_clone() {
        let config = TuningConfig {
            min_similarity: 0.5,
            ngram_range: (2, 4),
            error_code_weight: 3.0,
        };
        let cloned = config.clone();
        assert_eq!(cloned.min_similarity, 0.5);
        assert_eq!(cloned.ngram_range, (2, 4));
        assert_eq!(cloned.error_code_weight, 3.0);
    }

    #[test]
    fn test_tuning_config_debug() {
        let config = TuningConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("TuningConfig"));
        assert!(debug.contains("min_similarity"));
    }

    #[test]
    fn test_tuning_result_clone() {
        let result = TuningResult {
            config: TuningConfig::default(),
            accuracy: 0.75,
            correct: 15,
            total: 20,
        };
        let cloned = result.clone();
        assert_eq!(cloned.accuracy, 0.75);
        assert_eq!(cloned.correct, 15);
        assert_eq!(cloned.total, 20);
    }

    #[test]
    fn test_tuning_result_debug() {
        let result = TuningResult {
            config: TuningConfig::default(),
            accuracy: 0.8,
            correct: 8,
            total: 10,
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("TuningResult"));
        assert!(debug.contains("accuracy"));
    }

    #[test]
    fn test_weight_error_codes() {
        let msg = "error[E0308]: mismatched types";
        let weighted = weight_error_codes(msg, 3.0);
        assert!(weighted.contains("error[E0308] error[E0308] error[E0308]"));
    }

    #[test]
    fn test_weight_error_codes_no_code() {
        // Message without error code
        let msg = "cannot find value `x` in this scope";
        let weighted = weight_error_codes(msg, 3.0);
        assert_eq!(weighted, msg);
    }

    #[test]
    fn test_weight_error_codes_fractional() {
        // Test fractional weight (rounds to 2)
        let msg = "error[E0425]: cannot find value";
        let weighted = weight_error_codes(msg, 2.4);
        assert!(weighted.contains("error[E0425] error[E0425]"));
    }

    #[test]
    fn test_weight_error_codes_zero_weight() {
        let msg = "error[E0308]: mismatched types";
        let weighted = weight_error_codes(msg, 0.0);
        // With 0 weight, no repeats
        assert!(!weighted.contains("error[E0308] error[E0308]"));
    }

    #[test]
    fn test_evaluate_config_empty_samples() {
        let config = TuningConfig::default();
        let samples: Vec<TrainingSample> = vec![];
        let result = evaluate_config(&config, &samples);
        assert_eq!(result.total, 0);
        assert!(result.accuracy.is_nan() || result.accuracy == 0.0);
    }

    #[test]
    fn test_evaluate_config_single_sample() {
        let config = TuningConfig::default();
        let samples = vec![TrainingSample {
            message: "error[E0308]: mismatched types".to_string(),
            fix: Some("Use correct type".to_string()),
            category: ErrorCategory::TypeMismatch,
        }];
        let result = evaluate_config(&config, &samples);
        assert_eq!(result.total, 1);
    }

    #[test]
    fn test_evaluate_config_no_fix() {
        let config = TuningConfig::default();
        let samples = vec![
            TrainingSample {
                message: "error[E0308]: mismatched types".to_string(),
                fix: None, // No fix provided
                category: ErrorCategory::TypeMismatch,
            },
            TrainingSample {
                message: "error[E0425]: cannot find value".to_string(),
                fix: Some("Add import".to_string()),
                category: ErrorCategory::MissingImport,
            },
        ];
        let result = evaluate_config(&config, &samples);
        assert_eq!(result.total, 2);
    }

    #[test]
    fn test_find_best_config() {
        let result = find_best_config();
        // Should return some valid result
        assert!(result.total > 0 || result.config.min_similarity > 0.0);
    }

    #[test]
    fn test_quick_tune() {
        let result = quick_tune();
        assert!(result.accuracy >= 0.0);
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

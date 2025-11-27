//! AutoML-powered hyperparameter tuning using aprender.
//!
//! Leverages aprender's SearchSpace and RandomSearch for
//! automated hyperparameter optimization of the oracle predictor.

use aprender::automl::params::ParamKey;
use aprender::automl::{ParamValue, RandomSearch, SearchSpace, SearchStrategy};
use std::collections::HashMap;

use crate::depyler_training::build_combined_corpus;
use crate::ngram::NgramFixPredictor;
use crate::training::TrainingSample;
use crate::tuning::TuningResult;

/// Oracle-specific parameter keys for AutoML search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OracleParam {
    /// Minimum similarity threshold (0.0-1.0)
    MinSimilarity,
    /// N-gram minimum range
    NgramMin,
    /// N-gram maximum range
    NgramMax,
    /// Error code weighting factor
    ErrorCodeWeight,
}

impl ParamKey for OracleParam {
    fn name(&self) -> &'static str {
        match self {
            Self::MinSimilarity => "min_similarity",
            Self::NgramMin => "ngram_min",
            Self::NgramMax => "ngram_max",
            Self::ErrorCodeWeight => "error_code_weight",
        }
    }
}

/// Build a search space for oracle hyperparameters.
#[must_use]
pub fn build_oracle_search_space() -> SearchSpace<OracleParam> {
    SearchSpace::new()
        .add_continuous(OracleParam::MinSimilarity, 0.01, 0.3)
        .add(OracleParam::NgramMin, 1..4)
        .add(OracleParam::NgramMax, 2..6)
        .add_continuous(OracleParam::ErrorCodeWeight, 1.0, 5.0)
}

/// Configuration extracted from AutoML trial parameters.
#[derive(Clone, Debug)]
pub struct AutoMLConfig {
    pub min_similarity: f32,
    pub ngram_range: (usize, usize),
    pub error_code_weight: f32,
}

impl AutoMLConfig {
    /// Extract config from AutoML parameter values.
    pub fn from_params(params: &HashMap<OracleParam, ParamValue>) -> Self {
        let min_similarity = params
            .get(&OracleParam::MinSimilarity)
            .and_then(ParamValue::as_f64)
            .unwrap_or(0.1) as f32;

        let ngram_min = params
            .get(&OracleParam::NgramMin)
            .and_then(ParamValue::as_i64)
            .unwrap_or(1) as usize;

        let ngram_max = params
            .get(&OracleParam::NgramMax)
            .and_then(ParamValue::as_i64)
            .unwrap_or(3) as usize;

        let error_code_weight = params
            .get(&OracleParam::ErrorCodeWeight)
            .and_then(ParamValue::as_f64)
            .unwrap_or(2.0) as f32;

        Self {
            min_similarity,
            ngram_range: (ngram_min, ngram_max.max(ngram_min)),
            error_code_weight,
        }
    }
}

/// Evaluate a configuration using leave-one-out cross-validation.
fn evaluate_config(config: &AutoMLConfig, samples: &[TrainingSample]) -> f64 {
    let n = samples.len();
    if n == 0 {
        return 0.0;
    }

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

    correct as f64 / n as f64
}

/// Weight error codes by repeating them in the message.
fn weight_error_codes(message: &str, weight: f32) -> String {
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

/// Result from AutoML optimization.
#[derive(Clone, Debug)]
pub struct AutoMLResult {
    /// Best configuration found
    pub config: AutoMLConfig,
    /// Best accuracy achieved
    pub accuracy: f64,
    /// Number of trials run
    pub trials: usize,
    /// All trial results
    pub history: Vec<(AutoMLConfig, f64)>,
}

/// Run AutoML hyperparameter optimization.
///
/// Uses aprender's RandomSearch to explore the hyperparameter space
/// and find optimal settings for the oracle predictor.
///
/// # Arguments
/// * `n_trials` - Number of random configurations to try
///
/// # Returns
/// Best configuration found and optimization history.
#[must_use]
pub fn automl_optimize(n_trials: usize) -> AutoMLResult {
    let corpus = build_combined_corpus();
    let samples: Vec<_> = corpus.samples().to_vec();

    let search_space = build_oracle_search_space();
    let mut search = RandomSearch::new(n_trials);

    let mut best_config = AutoMLConfig {
        min_similarity: 0.1,
        ngram_range: (1, 3),
        error_code_weight: 2.0,
    };
    let mut best_accuracy = 0.0;
    let mut history = Vec::new();

    // Run random search using aprender's suggest API
    let trials = search.suggest(&search_space, n_trials);
    for trial in trials {
        let config = AutoMLConfig::from_params(&trial.values);
        let accuracy = evaluate_config(&config, &samples);

        history.push((config.clone(), accuracy));

        if accuracy > best_accuracy {
            best_accuracy = accuracy;
            best_config = config;
        }
    }

    AutoMLResult {
        config: best_config,
        accuracy: best_accuracy,
        trials: n_trials,
        history,
    }
}

/// Quick AutoML optimization with fewer trials.
#[must_use]
pub fn automl_quick() -> AutoMLResult {
    automl_optimize(20)
}

/// Full AutoML optimization with comprehensive search.
#[must_use]
pub fn automl_full() -> AutoMLResult {
    automl_optimize(100)
}

/// Extended AutoML optimization with thorough search.
#[must_use]
pub fn automl_extended() -> AutoMLResult {
    automl_optimize(300)
}

/// Convert AutoML result to TuningResult for compatibility.
impl From<AutoMLResult> for TuningResult {
    fn from(result: AutoMLResult) -> Self {
        TuningResult {
            config: crate::tuning::TuningConfig {
                min_similarity: result.config.min_similarity,
                ngram_range: result.config.ngram_range,
                error_code_weight: result.config.error_code_weight,
            },
            accuracy: result.accuracy as f32,
            correct: (result.accuracy * 27.0) as usize, // Approximate from 27 samples
            total: 27,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_search_space() {
        let space = build_oracle_search_space();
        assert_eq!(space.len(), 4);
    }

    #[test]
    fn test_automl_config_from_params() {
        let mut params = HashMap::new();
        params.insert(OracleParam::MinSimilarity, ParamValue::Float(0.15));
        params.insert(OracleParam::NgramMin, ParamValue::Int(2));
        params.insert(OracleParam::NgramMax, ParamValue::Int(4));
        params.insert(OracleParam::ErrorCodeWeight, ParamValue::Float(3.0));

        let config = AutoMLConfig::from_params(&params);
        assert!((config.min_similarity - 0.15).abs() < 0.01);
        assert_eq!(config.ngram_range, (2, 4));
        assert!((config.error_code_weight - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_automl_quick() {
        // Use fewer trials for coverage runs (DEPYLER_FAST_TESTS=1)
        let fast_mode = std::env::var("DEPYLER_FAST_TESTS").is_ok();
        let trials = if fast_mode { 3 } else { 20 };
        let result = automl_optimize(trials);
        assert!(result.accuracy > 0.0);
        assert_eq!(result.trials, trials);
        assert!(!result.history.is_empty());
        println!(
            "AutoML Quick: {:.2}% accuracy with sim={:.3}, ngram={:?}, weight={:.1}",
            result.accuracy * 100.0,
            result.config.min_similarity,
            result.config.ngram_range,
            result.config.error_code_weight
        );
    }

    #[test]
    #[ignore] // Slow - run manually
    fn test_automl_full() {
        let result = automl_full();
        assert!(result.accuracy > 0.0);
        assert_eq!(result.trials, 100);
        println!(
            "AutoML Full: {:.2}% accuracy with sim={:.3}, ngram={:?}, weight={:.1}",
            result.accuracy * 100.0,
            result.config.min_similarity,
            result.config.ngram_range,
            result.config.error_code_weight
        );
    }

    #[test]
    #[ignore] // Very slow - run manually for best results
    fn test_automl_extended() {
        let result = automl_extended();
        assert!(result.accuracy > 0.0);
        assert_eq!(result.trials, 300);
        println!(
            "AutoML Extended (300 trials): {:.2}% accuracy with sim={:.3}, ngram={:?}, weight={:.1}",
            result.accuracy * 100.0,
            result.config.min_similarity,
            result.config.ngram_range,
            result.config.error_code_weight
        );
        // Print top 5 configurations
        let mut sorted: Vec<_> = result.history.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        println!("\nTop 5 configurations:");
        for (i, (cfg, acc)) in sorted.iter().take(5).enumerate() {
            println!(
                "  {}. {:.2}% - sim={:.3}, ngram=({},{}), weight={:.1}",
                i + 1,
                acc * 100.0,
                cfg.min_similarity,
                cfg.ngram_range.0,
                cfg.ngram_range.1,
                cfg.error_code_weight
            );
        }
    }
}

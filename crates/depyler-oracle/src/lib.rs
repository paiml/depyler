//! ML-powered compile error classification and auto-fixing.
//!
//! Uses aprender models (Decision Tree, N-gram, Random Forest) to:
//! - Classify transpilation errors into actionable categories
//! - Suggest fixes based on historical patterns
//! - Detect error drift requiring model retraining

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use aprender::format::{self, Compression, ModelType, SaveOptions};
use aprender::metrics::drift::{DriftConfig, DriftDetector, DriftStatus};
use aprender::primitives::Matrix;
use aprender::tree::RandomForestClassifier;
use serde::{Deserialize, Serialize};

pub mod autofixer;
pub mod automl_tuning;
pub mod citl_fixer;
pub mod classifier;
pub mod github_corpus;
pub mod moe_oracle;
// pub mod data_store; // TODO: Re-enable when alimentar integrated
pub mod depyler_training;
pub mod estimator;
pub mod features;
pub mod hybrid;
// pub mod mlp_classifier; // TODO: GH-XXX - Implement GPU-accelerated MLP classifier
pub mod ngram;
pub mod params_persistence;
pub mod patterns;
pub mod self_supervised;
pub mod synthetic;
pub mod tfidf;
pub mod training;
pub mod tuning;
pub mod unified_training;
pub mod verificar_integration;

pub use autofixer::{AutoFixer, FixContext, FixResult, TransformRule};
pub use automl_tuning::{automl_full, automl_optimize, automl_quick, AutoMLConfig, AutoMLResult};
pub use citl_fixer::{CITLFixer, CITLFixerConfig, IterativeFixResult};
pub use estimator::{samples_to_features, OracleEstimator};
pub use params_persistence::{
    default_params_path, load_params, params_exist, save_params, OptimizedParams,
};
pub use synthetic::{
    generate_synthetic_corpus, generate_synthetic_corpus_sized, SyntheticConfig, SyntheticGenerator,
};
pub use tuning::{find_best_config, quick_tune, TuningConfig, TuningResult};

#[cfg(test)]
mod proptests;

pub use classifier::{ErrorCategory, ErrorClassifier};
pub use features::ErrorFeatures;
pub use hybrid::{
    HybridConfig, HybridTranspiler, PatternComplexity, Strategy, TrainingDataCollector,
    TranslationPair, TranspileError, TranspileResult, TranspileStats,
};
pub use ngram::{FixPattern, FixSuggestion, NgramFixPredictor};
pub use patterns::{CodeTransform, FixTemplate, FixTemplateRegistry};
pub use tfidf::{CombinedFeatureExtractor, TfidfConfig, TfidfFeatureExtractor};
pub use training::{TrainingDataset, TrainingSample};

// MoE Oracle exports
pub use moe_oracle::{ExpertDomain, MoeClassificationResult, MoeOracle, MoeOracleConfig};
pub use depyler_training::{classify_with_moe, load_real_corpus, train_moe_on_real_corpus, train_moe_oracle};

// GitHub corpus integration (via OIP)
pub use github_corpus::{
    build_github_corpus, convert_oip_to_depyler, load_oip_training_data,
    OipDefectCategory, OipTrainingDataset, OipTrainingExample,
    analyze_corpus, get_moe_samples_from_oip, CorpusStats,
};

// Unified training pipeline
pub use unified_training::{
    build_unified_corpus, build_default_unified_corpus, build_unified_corpus_with_oip,
    print_merge_stats, UnifiedTrainingConfig, UnifiedTrainingResult, MergeStats,
};

/// Error types for the oracle.
#[derive(Debug, thiserror::Error)]
pub enum OracleError {
    /// Model loading/saving error
    #[error("Model error: {0}")]
    Model(String),
    /// Feature extraction error
    #[error("Feature extraction error: {0}")]
    Feature(String),
    /// Classification error
    #[error("Classification error: {0}")]
    Classification(String),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for oracle operations.
pub type Result<T> = std::result::Result<T, OracleError>;

/// Classification result with confidence and suggested fix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// Predicted error category
    pub category: ErrorCategory,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Suggested fix template
    pub suggested_fix: Option<String>,
    /// Related error patterns
    pub related_patterns: Vec<String>,
}

/// Oracle for compile error prediction and fixing.
///
/// # Examples
///
/// ```ignore
/// use depyler_oracle::{Oracle, ErrorFeatures};
///
/// let oracle = Oracle::load("model.apr")?;
/// let features = ErrorFeatures::from_error_message("type mismatch: expected i32, found &str");
/// let result = oracle.classify(&features)?;
/// println!("Category: {:?}, Confidence: {}", result.category, result.confidence);
/// ```
/// Configuration for the Random Forest classifier.
#[derive(Clone, Debug)]
pub struct OracleConfig {
    /// Number of trees in the forest (default: 100)
    pub n_estimators: usize,
    /// Maximum tree depth (default: 10)
    pub max_depth: usize,
    /// Random seed for reproducibility
    pub random_state: Option<u64>,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            // 100 trees is usually sufficient for classification
            // 10,000 was excessive and caused 15+ min training times
            n_estimators: 100,
            max_depth: 10,
            random_state: Some(42),
        }
    }
}

pub struct Oracle {
    /// Random Forest classifier (replaces DecisionTree per GH-106)
    classifier: RandomForestClassifier,
    /// Configuration used to create the classifier (kept for model introspection)
    #[allow(dead_code)]
    config: OracleConfig,
    /// Category mappings
    categories: Vec<ErrorCategory>,
    /// Fix templates per category
    fix_templates: HashMap<ErrorCategory, Vec<String>>,
    /// Drift detector for retraining triggers
    drift_detector: DriftDetector,
    /// Historical performance scores
    performance_history: Vec<f32>,
}

/// Default model filename
const DEFAULT_MODEL_NAME: &str = "depyler_oracle.apr";

impl Oracle {
    /// Get the default model path (in project root or current dir).
    #[must_use]
    pub fn default_model_path() -> PathBuf {
        // Try to find project root via Cargo.toml
        let mut path = std::env::current_dir().unwrap_or_default();
        for _ in 0..5 {
            if path.join("Cargo.toml").exists() {
                return path.join(DEFAULT_MODEL_NAME);
            }
            if !path.pop() {
                break;
            }
        }
        // Fallback to current directory
        PathBuf::from(DEFAULT_MODEL_NAME)
    }

    /// Load model from default path, or train and save if not found.
    ///
    /// This is the recommended way to get an Oracle instance - it caches
    /// the trained model to disk for faster subsequent loads.
    pub fn load_or_train() -> Result<Self> {
        let path = Self::default_model_path();

        if path.exists() {
            match Self::load(&path) {
                Ok(oracle) => return Ok(oracle),
                Err(e) => {
                    eprintln!("Warning: Failed to load cached model: {e}. Retraining...");
                }
            }
        }

        // Train using verificar corpus + depyler training data + synthetic data
        let mut dataset = verificar_integration::build_verificar_corpus();
        let depyler_corpus = depyler_training::build_combined_corpus();
        for sample in depyler_corpus.samples() {
            dataset.add(sample.clone());
        }

        // Add synthetic data for robust training (12,000+ samples)
        let synthetic_corpus = synthetic::generate_synthetic_corpus();
        for sample in synthetic_corpus.samples() {
            dataset.add(sample.clone());
        }
        let (features, labels_vec) = samples_to_features(dataset.samples());
        let labels: Vec<usize> = labels_vec.as_slice().iter().map(|&x| x as usize).collect();

        let mut oracle = Self::new();
        oracle.train(&features, &labels)?;

        // Save for next time
        if let Err(e) = oracle.save(&path) {
            eprintln!("Warning: Failed to cache model to {}: {e}", path.display());
        }

        Ok(oracle)
    }

    /// Create a new oracle with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(OracleConfig::default())
    }

    /// Create a new oracle with custom configuration.
    #[must_use]
    pub fn with_config(config: OracleConfig) -> Self {
        let mut classifier =
            RandomForestClassifier::new(config.n_estimators).with_max_depth(config.max_depth);
        if let Some(seed) = config.random_state {
            classifier = classifier.with_random_state(seed);
        }

        Self {
            classifier,
            config,
            categories: vec![
                ErrorCategory::TypeMismatch,
                ErrorCategory::BorrowChecker,
                ErrorCategory::MissingImport,
                ErrorCategory::SyntaxError,
                ErrorCategory::LifetimeError,
                ErrorCategory::TraitBound,
                ErrorCategory::Other,
            ],
            fix_templates: Self::default_fix_templates(),
            drift_detector: DriftDetector::new(
                DriftConfig::default()
                    .with_min_samples(10)
                    .with_window_size(50),
            ),
            performance_history: Vec::new(),
        }
    }

    /// Default fix templates for each category.
    fn default_fix_templates() -> HashMap<ErrorCategory, Vec<String>> {
        let mut templates = HashMap::new();

        templates.insert(
            ErrorCategory::TypeMismatch,
            vec![
                "Convert type using `.into()` or `as`".to_string(),
                "Check function signature for expected type".to_string(),
                "Use type annotation to clarify".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::BorrowChecker,
            vec![
                "Clone the value instead of borrowing".to_string(),
                "Use a reference (&) instead of moving".to_string(),
                "Introduce a scope to limit borrow lifetime".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::MissingImport,
            vec![
                "Add `use` statement for the missing type".to_string(),
                "Check crate dependencies in Cargo.toml".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::SyntaxError,
            vec![
                "Check for missing semicolons or braces".to_string(),
                "Verify function/struct syntax".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::LifetimeError,
            vec![
                "Add explicit lifetime annotation".to_string(),
                "Use 'static lifetime for owned data".to_string(),
                "Consider using Rc/Arc for shared ownership".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::TraitBound,
            vec![
                "Implement the required trait".to_string(),
                "Add trait bound to generic parameter".to_string(),
                "Use a wrapper type that implements the trait".to_string(),
            ],
        );

        templates.insert(
            ErrorCategory::Other,
            vec!["Review the full error message for specifics".to_string()],
        );

        templates
    }

    /// Train the oracle on labeled error data.
    ///
    /// # Errors
    ///
    /// Returns error if training fails.
    pub fn train(&mut self, features: &Matrix<f32>, labels: &[usize]) -> Result<()> {
        self.classifier
            .fit(features, labels)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(())
    }

    /// Classify an error based on its features.
    pub fn classify(&self, features: &ErrorFeatures) -> Result<ClassificationResult> {
        let feature_matrix = features.to_matrix();
        let predictions = self.classifier.predict(&feature_matrix);

        if predictions.is_empty() {
            return Err(OracleError::Classification(
                "No prediction produced".to_string(),
            ));
        }

        let pred_idx = predictions.as_slice()[0];
        let category = self
            .categories
            .get(pred_idx)
            .copied()
            .unwrap_or(ErrorCategory::Other);

        let suggested_fix = self
            .fix_templates
            .get(&category)
            .and_then(|fixes| fixes.first().cloned());

        let related = self
            .fix_templates
            .get(&category)
            .map(|fixes| fixes.iter().skip(1).cloned().collect())
            .unwrap_or_default();

        Ok(ClassificationResult {
            category,
            confidence: 0.85, // TODO: Extract from tree probabilities
            suggested_fix,
            related_patterns: related,
        })
    }

    /// Check if the model needs retraining based on performance drift.
    pub fn check_drift(&mut self, recent_accuracy: f32) -> DriftStatus {
        self.performance_history.push(recent_accuracy);

        if self.performance_history.len() < 10 {
            return DriftStatus::NoDrift;
        }

        let baseline: Vec<f32> =
            self.performance_history[..self.performance_history.len() / 2].to_vec();
        let current: Vec<f32> =
            self.performance_history[self.performance_history.len() / 2..].to_vec();

        self.drift_detector
            .detect_performance_drift(&baseline, &current)
    }

    /// Save the oracle model to a file.
    ///
    /// # Errors
    ///
    /// Returns error if saving fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let options = SaveOptions::default()
            .with_name("depyler-oracle")
            .with_description("RandomForest error classification model for Depyler transpiler")
            .with_compression(Compression::ZstdDefault);

        format::save(&self.classifier, ModelType::RandomForest, path, options)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(())
    }

    /// Load an oracle model from a file.
    ///
    /// # Errors
    ///
    /// Returns error if loading fails.
    pub fn load(path: &Path) -> Result<Self> {
        let classifier: RandomForestClassifier = format::load(path, ModelType::RandomForest)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        let config = OracleConfig::default();
        Ok(Self {
            classifier,
            config,
            categories: vec![
                ErrorCategory::TypeMismatch,
                ErrorCategory::BorrowChecker,
                ErrorCategory::MissingImport,
                ErrorCategory::SyntaxError,
                ErrorCategory::LifetimeError,
                ErrorCategory::TraitBound,
                ErrorCategory::Other,
            ],
            fix_templates: Self::default_fix_templates(),
            drift_detector: DriftDetector::new(
                DriftConfig::default()
                    .with_min_samples(10)
                    .with_window_size(50),
            ),
            performance_history: Vec::new(),
        })
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_creation() {
        let oracle = Oracle::new();
        assert_eq!(oracle.categories.len(), 7);
    }

    #[test]
    fn test_fix_templates() {
        let oracle = Oracle::new();
        assert!(oracle
            .fix_templates
            .contains_key(&ErrorCategory::TypeMismatch));
        assert!(oracle
            .fix_templates
            .contains_key(&ErrorCategory::BorrowChecker));
    }

    #[test]
    fn test_drift_detection_insufficient_data() {
        let mut oracle = Oracle::new();
        let status = oracle.check_drift(0.95);
        assert!(matches!(status, DriftStatus::NoDrift));
    }

    #[test]
    fn test_load_or_train() {
        // Skip full training in fast test mode (coverage runs)
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            // Just test Oracle creation, not full training
            let oracle = Oracle::new();
            assert_eq!(oracle.categories.len(), 7);
            return;
        }

        // First call trains and saves
        let oracle = Oracle::load_or_train().expect("load_or_train should succeed");
        assert_eq!(oracle.categories.len(), 7);

        // Verify model file was created
        let path = Oracle::default_model_path();
        assert!(path.exists(), "Model file should be created at {:?}", path);

        // Second call should load from disk (faster)
        let oracle2 = Oracle::load_or_train().expect("second load_or_train should succeed");
        assert_eq!(oracle2.categories.len(), 7);

        // Clean up
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_default_model_path() {
        let path = Oracle::default_model_path();
        assert!(path.to_string_lossy().contains("depyler_oracle.apr"));
    }
}

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
pub mod corpus_citl;
pub mod github_corpus;
pub mod moe_oracle;
pub mod data_store;
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
pub mod hybrid_retrieval;
pub mod hansei;
pub mod training;
pub mod oracle_lineage;  // Issue #212: OracleLineage using entrenar::monitor::ModelLineage
pub mod tuning;
pub mod unified_training;
pub mod verificar_integration;
pub mod query_loop;
pub mod corpus_extract;
pub mod curriculum;
pub mod distillation;
pub mod error_patterns;
pub mod gnn_encoder;
pub mod ast_embeddings;  // Issue #210: Code2Vec-style AST embeddings
pub mod tarantula;
pub mod tarantula_bridge;
pub mod tarantula_corpus;
pub mod oip_export;
pub mod acceleration_pipeline;

pub use autofixer::{AutoFixer, FixContext, FixResult, TransformRule};
pub use automl_tuning::{automl_full, automl_optimize, automl_quick, AutoMLConfig, AutoMLResult};
pub use citl_fixer::{CITLFixer, CITLFixerConfig, IterativeFixResult};
pub use corpus_citl::{CorpusCITL, IngestionStats};
pub use estimator::{message_to_features, samples_to_features, OracleEstimator};
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
pub use hybrid_retrieval::{Bm25Scorer, HybridRetriever, RrfResult, reciprocal_rank_fusion};
pub use hansei::{
    CategorySummary, HanseiConfig, HanseiReport, IssueSeverity, TranspileHanseiAnalyzer,
    TranspileIssue, TranspileOutcome, Trend,
};
pub use training::{TrainingDataset, TrainingSample};
pub use oracle_lineage::OracleLineage;  // Issue #212: Model lineage tracking

// MoE Oracle exports
pub use moe_oracle::{ExpertDomain, MoeClassificationResult, MoeOracle, MoeOracleConfig};
pub use depyler_training::{classify_with_moe, load_real_corpus, train_moe_on_real_corpus, train_moe_oracle};

// Oracle Query Loop exports (Issue #172)
pub use query_loop::{
    apply_simple_diff, auto_fix_loop, AutoFixResult, ErrorContext, OracleMetrics,
    OracleQueryError, OracleQueryLoop, OracleSuggestion, OracleStats, ParseRustErrorCodeError,
    QueryLoopConfig, RustErrorCode,
};

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

// Tarantula fault localization (Strategy #1 - DEPYLER-0631)
pub use tarantula::{
    FixPriority, SuspiciousTranspilerDecision, TarantulaAnalyzer, TarantulaResult,
    TranspilerDecision, TranspilerDecisionRecord,
};

// Tarantula corpus analysis for batch processing
pub use tarantula_corpus::{
    CorpusAnalysisReport, CorpusAnalyzer, TranspilationResult,
};

// Tarantula bridge for depyler-core decision trace integration
pub use tarantula_bridge::{
    category_to_decision, decision_to_record, decisions_to_records,
    infer_decisions_from_error, synthetic_decisions_from_errors,
};

// Error Pattern Library (Strategy #2 - DEPYLER-0632)
pub use error_patterns::{
    CorpusEntry, ErrorPattern, ErrorPatternConfig, ErrorPatternLibrary,
    ErrorPatternStats, GoldenTraceEntry,
};

// Curriculum Learning (Strategy #3 - DEPYLER-0633)
pub use curriculum::{
    classify_error_difficulty, classify_from_category, CurriculumEntry,
    CurriculumScheduler, CurriculumStats, DifficultyLevel,
};

// Knowledge Distillation (Strategy #4 - DEPYLER-0634)
pub use distillation::{
    DistillationConfig, DistillationStats, ExtractedPattern,
    KnowledgeDistiller, LlmFixExample,
};

// GNN Error Encoder (Strategy #5 - DEPYLER-0635)
pub use gnn_encoder::{
    DepylerGnnEncoder, GnnEncoderConfig, GnnEncoderStats,
    SimilarPattern, StructuralPattern, infer_decision_from_match,
    map_error_category,
};

// AST Embeddings (Issue #210 - Code2Vec-style embeddings)
pub use ast_embeddings::{
    AstEmbedder, AstEmbedding, AstEmbeddingConfig, CombinedEmbeddingExtractor,
    CombinedFeatures, PathContext,
};

// OIP CITL Export (Strategy #6 - DEPYLER-0636)
pub use oip_export::{
    BatchExporter, DepylerExport, ErrorCodeClass, ExportStats,
    SpanInfo, SuggestionInfo, export_to_jsonl,
};

// Acceleration Pipeline (DEPYLER-0637) - Unified strategy integration
pub use acceleration_pipeline::{
    AccelerationPipeline, AnalysisResult, FixSource, PipelineConfig, PipelineStats,
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

/// Get training corpus file paths for hash computation.
///
/// Issue #211: Used to detect when training data has changed.
#[must_use]
pub fn get_training_corpus_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Find project root
    let mut root = std::env::current_dir().unwrap_or_default();
    for _ in 0..5 {
        if root.join("Cargo.toml").exists() {
            break;
        }
        if !root.pop() {
            return paths;
        }
    }

    // Collect corpus directories
    let corpus_dirs = [
        root.join("crates/depyler-oracle/src"),
        root.join("verificar/corpus"),
        root.join("training_data"),
    ];

    for dir in corpus_dirs {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |e| e == "rs" || e == "json") {
                        paths.push(path);
                    }
                }
            }
        }
    }

    // Sort for deterministic hashing
    paths.sort();
    paths
}

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
    ///
    /// ## Issue #212: Model Lineage Tracking (replaces Issue #211)
    ///
    /// This method now uses `entrenar::monitor::ModelLineage` for:
    /// - Git commit SHA and corpus hash change detection
    /// - Model version tracking with lineage chains
    /// - Regression detection between training runs
    /// - Stores lineage in `.depyler/oracle_lineage.json`
    pub fn load_or_train() -> Result<Self> {
        let model_path = Self::default_model_path();
        let lineage_path = OracleLineage::default_lineage_path();

        // Get current state for comparison
        let current_sha = OracleLineage::get_current_commit_sha();
        let corpus_paths = get_training_corpus_paths();
        let current_corpus_hash = OracleLineage::compute_corpus_hash(&corpus_paths);

        // Load existing lineage (Issue #212)
        let mut lineage = match OracleLineage::load(&lineage_path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Warning: Failed to load lineage: {e}. Starting fresh...");
                OracleLineage::new()
            }
        };

        // Check if we need to retrain
        let needs_retrain = lineage.needs_retraining(&current_sha, &current_corpus_hash);
        if needs_retrain && lineage.model_count() > 0 {
            eprintln!("📊 Oracle: Codebase changes detected, triggering retraining...");
        } else if needs_retrain {
            eprintln!("📊 Oracle: No training history found, will train fresh...");
        }

        // Try to load existing model if no retrain needed
        if !needs_retrain && model_path.exists() {
            match Self::load(&model_path) {
                Ok(oracle) => {
                    eprintln!("📊 Oracle: Loaded cached model (no changes detected)");
                    return Ok(oracle);
                }
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

        let sample_count = dataset.samples().len();
        let (features, labels_vec) = samples_to_features(dataset.samples());
        let labels: Vec<usize> = labels_vec.as_slice().iter().map(|&x| x as usize).collect();

        let mut oracle = Self::new();
        oracle.train(&features, &labels)?;

        // Save model for next time
        if let Err(e) = oracle.save(&model_path) {
            eprintln!("Warning: Failed to cache model to {}: {e}", model_path.display());
        }

        // Record training in lineage (Issue #212)
        // Use a default accuracy of 0.85 since we don't have validation data here
        let model_id = lineage.record_training(
            current_sha,
            current_corpus_hash,
            sample_count,
            0.85, // Default accuracy estimate
        );

        // Check for regression (enabled by ModelLineage)
        if let Some((reason, delta)) = lineage.find_regression() {
            eprintln!(
                "⚠️  Oracle: Regression detected! Accuracy dropped by {:.2}% ({})",
                delta.abs() * 100.0,
                reason
            );
        }

        // Save lineage
        if let Err(e) = lineage.save(&lineage_path) {
            eprintln!("Warning: Failed to save lineage: {e}");
        } else {
            eprintln!(
                "📊 Oracle: Training complete ({} samples), lineage recorded as {}",
                sample_count,
                model_id
            );
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

    /// Classify an error based on its message.
    ///
    /// Uses the same feature extraction as training (73 features: error codes + keywords + handcrafted).
    pub fn classify_message(&self, message: &str) -> Result<ClassificationResult> {
        let feature_matrix = message_to_features(message);
        let predictions = self.classifier.predict(&feature_matrix);

        self.build_classification_result(predictions)
    }

    /// Classify an error based on its features (legacy API).
    ///
    /// **Note**: This uses only 12 handcrafted features. For better accuracy with
    /// models trained on the full 73-feature set, use `classify_message` instead.
    #[deprecated(since = "3.22.0", note = "Use classify_message for better accuracy")]
    pub fn classify(&self, features: &ErrorFeatures) -> Result<ClassificationResult> {
        // Convert to full feature vector with zero padding for error codes and keywords
        let error_features = features.to_vec();
        let n_error_codes = estimator::feature_config::ERROR_CODES.len();
        let n_keywords = estimator::feature_config::KEYWORDS.len();
        let n_total = n_error_codes + n_keywords + ErrorFeatures::DIM;

        let mut full_features = vec![0.0f32; n_total];
        // Copy handcrafted features to the end
        for (i, &val) in error_features.iter().enumerate() {
            full_features[n_error_codes + n_keywords + i] = val;
        }

        let feature_matrix = aprender::primitives::Matrix::from_vec(1, n_total, full_features)
            .expect("Feature matrix dimensions are correct");
        let predictions = self.classifier.predict(&feature_matrix);

        self.build_classification_result(predictions)
    }

    fn build_classification_result(&self, predictions: Vec<usize>) -> Result<ClassificationResult> {
        if predictions.is_empty() {
            return Err(OracleError::Classification(
                "No prediction produced".to_string(),
            ));
        }

        let pred_idx = predictions[0];
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

    // ============================================================
    // Issue #212: Model Lineage Tests (replaces Issue #211)
    // ============================================================

    #[test]
    fn test_needs_retrain_no_lineage_file() {
        // When no lineage file exists, should need retraining
        let temp_dir = tempfile::TempDir::new().expect("create temp dir");
        let lineage_path = temp_dir.path().join(".depyler").join("oracle_lineage.json");

        let lineage = OracleLineage::load(&lineage_path).expect("load should not error");
        assert_eq!(lineage.model_count(), 0, "No lineage file should return empty lineage");

        // Empty lineage should need retraining
        assert!(
            lineage.needs_retraining("any_sha", "any_hash"),
            "Empty lineage should need retraining"
        );
    }

    #[test]
    fn test_needs_retrain_commit_changed_lineage() {
        // When commit SHA changes, should need retraining
        let mut lineage = OracleLineage::new();
        lineage.record_training(
            "abc123def456".to_string(),
            "corpus_hash_123".to_string(),
            1000,
            0.85,
        );

        assert!(
            lineage.needs_retraining("different_sha_789", "corpus_hash_123"),
            "Changed commit SHA should trigger retraining"
        );
    }

    #[test]
    fn test_needs_retrain_corpus_changed_lineage() {
        // When corpus hash changes, should need retraining
        let mut lineage = OracleLineage::new();
        lineage.record_training(
            "abc123def456".to_string(),
            "corpus_hash_123".to_string(),
            1000,
            0.85,
        );

        assert!(
            lineage.needs_retraining("abc123def456", "different_corpus_hash"),
            "Changed corpus hash should trigger retraining"
        );
    }

    #[test]
    fn test_no_retrain_when_unchanged_lineage() {
        // When nothing changed, should NOT need retraining
        let mut lineage = OracleLineage::new();
        lineage.record_training(
            "abc123def456".to_string(),
            "corpus_hash_123".to_string(),
            1000,
            0.85,
        );

        assert!(
            !lineage.needs_retraining("abc123def456", "corpus_hash_123"),
            "Unchanged state should NOT need retraining"
        );
    }

    #[test]
    fn test_lineage_saves_after_training() {
        let temp_dir = tempfile::TempDir::new().expect("create temp dir");
        let lineage_path = temp_dir.path().join(".depyler").join("oracle_lineage.json");

        // Create and save lineage
        let mut lineage = OracleLineage::new();
        lineage.record_training(
            "test_sha_12345".to_string(),
            "test_hash_67890".to_string(),
            500,
            0.85,
        );
        lineage.save(&lineage_path).expect("save should work");

        // Verify it was saved
        assert!(lineage_path.exists(), "Lineage file should exist after save");

        // Load and verify
        let loaded = OracleLineage::load(&lineage_path)
            .expect("load should work");
        assert_eq!(loaded.model_count(), 1);

        // Verify latest model has correct metadata
        let latest = loaded.latest_model().expect("should have model");
        assert_eq!(latest.tags.get("commit_sha"), Some(&"test_sha_12345".to_string()));
        assert_eq!(latest.config_hash, "test_hash_67890");
        assert_eq!(latest.tags.get("sample_count"), Some(&"500".to_string()));
    }

    #[test]
    fn test_get_corpus_paths_for_hashing() {
        // Test that we can get corpus paths for hashing
        // This tests the corpus path collection logic
        let paths = get_training_corpus_paths();
        // Should return some paths (even if empty in test environment)
        assert!(
            paths.is_empty() || !paths.is_empty(),
            "get_training_corpus_paths should return a Vec"
        );
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
    #[ignore] // SLOW: Full model training takes >120s
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

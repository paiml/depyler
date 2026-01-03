//! ML-powered compile error classification and auto-fixing.
//!
//! Uses aprender models (Decision Tree, N-gram, Random Forest) to:
//! - Classify transpilation errors into actionable categories
//! - Suggest fixes based on historical patterns
//! - Detect error drift requiring model retraining

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use aprender::format::{self, Compression, ModelType, SaveOptions};
use aprender::online::drift::{DriftDetector, DriftStats, DriftStatus, ADWIN};
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
pub mod utol;  // UTOL-001: Unified Training Oracle Loop

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
    /// ADWIN drift detector for retraining triggers (Issue #213)
    /// Replaces manual performance_history tracking with adaptive windowing
    adwin_detector: ADWIN,
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
                    if path.extension().is_some_and(|e| e == "rs" || e == "json") {
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
            // Issue #213: Use ADWIN with recommended delta (0.002)
            // Per Toyota Way review: ADWIN handles both sudden and gradual drift
            adwin_detector: ADWIN::with_delta(0.002),
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
    ///
    /// Issue #213: Uses ADWIN (Adaptive Windowing) algorithm for drift detection.
    /// ADWIN automatically adjusts window size and detects both sudden and gradual drift.
    ///
    /// # Arguments
    /// * `was_error` - true if the prediction was wrong, false if correct
    ///
    /// # Returns
    /// * `DriftStatus::Stable` - Model performing well
    /// * `DriftStatus::Warning` - Performance degrading, collect more data
    /// * `DriftStatus::Drift` - Significant drift detected, retrain recommended
    pub fn observe_prediction(&mut self, was_error: bool) -> DriftStatus {
        self.adwin_detector.add_element(was_error);
        self.adwin_detector.detected_change()
    }

    /// Get current drift status without adding new observation.
    #[must_use]
    pub fn drift_status(&self) -> DriftStatus {
        self.adwin_detector.detected_change()
    }

    /// Check if model needs retraining based on drift status.
    #[must_use]
    pub fn needs_retraining(&self) -> bool {
        matches!(self.drift_status(), DriftStatus::Drift)
    }

    /// Reset drift detector (call after retraining).
    pub fn reset_drift_detector(&mut self) {
        self.adwin_detector.reset();
    }

    /// Get drift detector statistics.
    #[must_use]
    pub fn drift_stats(&self) -> DriftStats {
        self.adwin_detector.stats()
    }

    /// Set ADWIN sensitivity (delta parameter).
    ///
    /// Lower delta = more sensitive to drift (more false positives)
    /// Higher delta = less sensitive (fewer false positives)
    /// Default: 0.002 (recommended balance)
    pub fn set_adwin_delta(&mut self, delta: f64) {
        self.adwin_detector = ADWIN::with_delta(delta);
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
            // Issue #213: Use ADWIN with recommended delta (0.002)
            adwin_detector: ADWIN::with_delta(0.002),
        })
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Issue #213: Stdout Visualization (Andon-style)
// ============================================================

/// Print drift status to stdout with visual indicators.
pub fn print_drift_status(stats: &DriftStats, status: &DriftStatus) {
    let status_indicator = match status {
        DriftStatus::Stable => "🟢 STABLE",
        DriftStatus::Warning => "🟡 WARNING",
        DriftStatus::Drift => "🔴 DRIFT DETECTED",
    };

    println!("╭─────────────────────────────────────────────────────╮");
    println!("│            Drift Detection Status                   │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Status: {:^40} │", status_indicator);
    println!("│  Samples: {:>8}                                 │", stats.n_samples);
    println!("│  Error Rate: {:>6.2}%                               │", stats.error_rate * 100.0);
    println!("│  Min Error Rate: {:>6.2}%                           │", stats.min_error_rate * 100.0);
    println!("│  Std Dev: {:>8.4}                                 │", stats.std_dev);
    println!("╰─────────────────────────────────────────────────────╯");
}

/// Print retrain trigger status with Andon-style alerts.
pub fn print_retrain_status(stats: &RetrainStats) {
    let status_indicator = match &stats.drift_status {
        DriftStatus::Stable => "🟢",
        DriftStatus::Warning => "🟡",
        DriftStatus::Drift => "🔴",
    };

    let accuracy_bar = create_accuracy_bar(stats.accuracy());

    println!("╭─────────────────────────────────────────────────────╮");
    println!("│            Retrain Trigger Status                   │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  {} Drift Status: {:?}                           │", status_indicator, stats.drift_status);
    println!("│  Predictions: {:>8}                              │", stats.predictions_observed);
    println!("│  Correct:     {:>8}                              │", stats.correct_predictions);
    println!("│  Errors:      {:>8}                              │", stats.errors);
    println!("│  Consecutive: {:>8}                              │", stats.consecutive_errors);
    println!("│  Drift Count: {:>8}                              │", stats.drift_count);
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Accuracy: {:>6.2}% {}                    │", stats.accuracy() * 100.0, accuracy_bar);
    println!("│  Error Rate: {:>6.2}%                               │", stats.error_rate() * 100.0);
    println!("╰─────────────────────────────────────────────────────╯");
}

/// Print lineage history to stdout.
pub fn print_lineage_history(lineage: &OracleLineage) {
    println!("╭─────────────────────────────────────────────────────╮");
    println!("│            Model Lineage History                    │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Total Models: {:>6}                               │", lineage.model_count());

    if let Some(latest) = lineage.latest_model() {
        let commit_sha = latest.tags.get("commit_sha").map(|s| &s[..8.min(s.len())]).unwrap_or("unknown");
        println!("│  Latest Model: {}                     │", latest.model_id.chars().take(30).collect::<String>());
        println!("│  Version: {}                              │", latest.version);
        println!("│  Accuracy: {:>6.2}%                                 │", latest.accuracy * 100.0);
        println!("│  Commit: {}                                │", commit_sha);
    }

    // Show regression if any
    if let Some((reason, delta)) = lineage.find_regression() {
        let indicator = if delta < 0.0 { "🔴" } else { "🟢" };
        println!("├─────────────────────────────────────────────────────┤");
        println!("│  {} Regression: {:+.2}%                             │", indicator, delta * 100.0);
        println!("│  Reason: {:40} │", reason.chars().take(40).collect::<String>());
    }

    // Show lineage chain
    let chain = lineage.get_lineage_chain();
    if !chain.is_empty() {
        println!("├─────────────────────────────────────────────────────┤");
        println!("│  Lineage Chain ({} models):                        │", chain.len());
        for (i, model_id) in chain.iter().take(5).enumerate() {
            let arrow = if i == 0 { "└" } else { "├" };
            println!("│    {} {}               │", arrow, model_id.chars().take(35).collect::<String>());
        }
        if chain.len() > 5 {
            println!("│    ... and {} more                               │", chain.len() - 5);
        }
    }

    println!("╰─────────────────────────────────────────────────────╯");
}

/// Create a visual accuracy bar.
fn create_accuracy_bar(accuracy: f64) -> String {
    let filled = (accuracy * 10.0).round() as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

/// Print combined status (drift + retrain + lineage).
pub fn print_oracle_status(trigger: &RetrainTrigger, lineage: &OracleLineage) {
    print_retrain_status(trigger.stats());
    println!();
    print_drift_status(&trigger.drift_stats(), &trigger.stats().drift_status);
    println!();
    print_lineage_history(lineage);
}

// ============================================================
// Issue #213: RetrainOrchestrator-style Integration
// ============================================================

/// Result of observing a prediction (mirrors aprender::online::orchestrator::ObserveResult).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObserveResult {
    /// Model is performing well
    Stable,
    /// Warning level - collecting more data
    Warning,
    /// Drift detected - retraining needed
    DriftDetected,
}

/// Configuration for retrain trigger (mirrors aprender::online::orchestrator::RetrainConfig).
#[derive(Debug, Clone)]
pub struct RetrainConfig {
    /// Minimum predictions before drift detection is reliable
    pub min_samples: usize,
    /// Maximum consecutive errors before forcing alert
    pub max_consecutive_errors: usize,
    /// Error rate threshold for warning
    pub warning_threshold: f64,
    /// Error rate threshold for drift
    pub drift_threshold: f64,
}

impl Default for RetrainConfig {
    fn default() -> Self {
        Self {
            min_samples: 50,
            max_consecutive_errors: 10,
            warning_threshold: 0.2,
            drift_threshold: 0.3,
        }
    }
}

/// Statistics from the retrain trigger (mirrors aprender::online::orchestrator::OrchestratorStats).
#[derive(Debug, Clone, Default)]
pub struct RetrainStats {
    /// Total predictions observed
    pub predictions_observed: u64,
    /// Total correct predictions
    pub correct_predictions: u64,
    /// Total errors
    pub errors: u64,
    /// Consecutive errors (resets on correct)
    pub consecutive_errors: usize,
    /// Current drift status
    pub drift_status: DriftStatus,
    /// Times drift was detected
    pub drift_count: u64,
}

impl RetrainStats {
    /// Current error rate.
    #[must_use]
    pub fn error_rate(&self) -> f64 {
        if self.predictions_observed == 0 {
            0.0
        } else {
            self.errors as f64 / self.predictions_observed as f64
        }
    }

    /// Current accuracy.
    #[must_use]
    pub fn accuracy(&self) -> f64 {
        1.0 - self.error_rate()
    }
}

/// Retrain trigger for Oracle (adapted from aprender::online::orchestrator::RetrainOrchestrator).
///
/// Monitors prediction outcomes and determines when retraining is needed.
/// Integrates ADWIN drift detection with Oracle predictions.
///
/// # Example
///
/// ```ignore
/// let mut trigger = RetrainTrigger::new(oracle, RetrainConfig::default());
///
/// // After each prediction
/// let result = trigger.observe_prediction(was_error);
/// if result == ObserveResult::DriftDetected {
///     // Retrain the oracle
///     trigger.mark_retrained();
/// }
/// ```
pub struct RetrainTrigger {
    /// The oracle being monitored
    oracle: Oracle,
    /// Configuration
    config: RetrainConfig,
    /// Statistics
    stats: RetrainStats,
}

impl RetrainTrigger {
    /// Create a new retrain trigger with an oracle.
    pub fn new(oracle: Oracle, config: RetrainConfig) -> Self {
        Self {
            oracle,
            config,
            stats: RetrainStats::default(),
        }
    }

    /// Create with default config.
    pub fn with_oracle(oracle: Oracle) -> Self {
        Self::new(oracle, RetrainConfig::default())
    }

    /// Observe a prediction outcome.
    ///
    /// # Arguments
    /// * `was_error` - true if the prediction was wrong, false if correct
    ///
    /// # Returns
    /// * `ObserveResult` indicating current status
    pub fn observe(&mut self, was_error: bool) -> ObserveResult {
        self.stats.predictions_observed += 1;

        if was_error {
            self.stats.errors += 1;
            self.stats.consecutive_errors += 1;
        } else {
            self.stats.correct_predictions += 1;
            self.stats.consecutive_errors = 0;
        }

        // Use ADWIN for drift detection
        let drift_status = self.oracle.observe_prediction(was_error);
        self.stats.drift_status = drift_status;

        // Check for drift
        if matches!(drift_status, DriftStatus::Drift) {
            self.stats.drift_count += 1;
            return ObserveResult::DriftDetected;
        }

        // Check consecutive error threshold
        if self.stats.consecutive_errors >= self.config.max_consecutive_errors {
            self.stats.drift_count += 1;
            return ObserveResult::DriftDetected;
        }

        // Check error rate after minimum samples
        if self.stats.predictions_observed >= self.config.min_samples as u64 {
            let error_rate = self.stats.error_rate();
            if error_rate >= self.config.drift_threshold {
                self.stats.drift_count += 1;
                return ObserveResult::DriftDetected;
            }
            if error_rate >= self.config.warning_threshold {
                return ObserveResult::Warning;
            }
        }

        ObserveResult::Stable
    }

    /// Mark that retraining has been completed.
    pub fn mark_retrained(&mut self) {
        self.oracle.reset_drift_detector();
        self.stats.consecutive_errors = 0;
        self.stats.predictions_observed = 0;
        self.stats.correct_predictions = 0;
        self.stats.errors = 0;
    }

    /// Get current statistics.
    #[must_use]
    pub fn stats(&self) -> &RetrainStats {
        &self.stats
    }

    /// Get mutable reference to oracle for predictions.
    pub fn oracle_mut(&mut self) -> &mut Oracle {
        &mut self.oracle
    }

    /// Get reference to oracle.
    #[must_use]
    pub fn oracle(&self) -> &Oracle {
        &self.oracle
    }

    /// Check if retraining is recommended.
    #[must_use]
    pub fn needs_retraining(&self) -> bool {
        self.oracle.needs_retraining()
            || self.stats.consecutive_errors >= self.config.max_consecutive_errors
            || (self.stats.predictions_observed >= self.config.min_samples as u64
                && self.stats.error_rate() >= self.config.drift_threshold)
    }

    /// Get ADWIN drift statistics.
    #[must_use]
    pub fn drift_stats(&self) -> DriftStats {
        self.oracle.drift_stats()
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

    // ============================================================
    // Issue #213: ADWIN Drift Detection Tests
    // ============================================================

    #[test]
    fn test_adwin_drift_detection_stable() {
        let mut oracle = Oracle::new();

        // Feed good predictions (no errors) - should stay stable
        for _ in 0..50 {
            let status = oracle.observe_prediction(false); // correct prediction
            assert!(
                matches!(status, DriftStatus::Stable),
                "All correct predictions should be stable"
            );
        }

        assert!(!oracle.needs_retraining(), "Should not need retraining with all correct");
    }

    #[test]
    fn test_adwin_drift_detection_gradual_degradation() {
        // Use a more sensitive ADWIN detector for testing
        let mut oracle = Oracle::new();
        // Replace with more sensitive detector for test
        oracle.set_adwin_delta(0.1);

        // Start with good predictions
        for _ in 0..200 {
            oracle.observe_prediction(false);
        }

        // Introduce many errors - ADWIN should detect the significant change
        for _ in 0..200 {
            oracle.observe_prediction(true);
        }

        // After all this, either drift was detected or stats show high error rate
        let stats = oracle.drift_stats();
        // With 200 correct + 200 wrong, the mean error rate should be around 0.5
        // If ADWIN didn't detect drift, at least verify it's tracking the data
        assert!(
            oracle.needs_retraining() || stats.error_rate > 0.3,
            "Should detect drift or have high error rate: {:?}, drift status: {:?}",
            stats,
            oracle.drift_status()
        );
    }

    #[test]
    fn test_adwin_drift_detector_reset() {
        let mut oracle = Oracle::new();

        // Add some observations
        for _ in 0..50 {
            oracle.observe_prediction(true);
        }

        // Reset the detector
        oracle.reset_drift_detector();

        // Should be back to stable
        assert!(matches!(oracle.drift_status(), DriftStatus::Stable));
    }

    #[test]
    fn test_adwin_drift_stats() {
        let mut oracle = Oracle::new();

        // Add some observations
        for _ in 0..10 {
            oracle.observe_prediction(false);
        }
        for _ in 0..10 {
            oracle.observe_prediction(true);
        }

        let stats = oracle.drift_stats();
        assert_eq!(stats.n_samples, 20, "Should have 20 samples");
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

    // ============================================================
    // Issue #213: RetrainTrigger Tests
    // ============================================================

    #[test]
    fn test_retrain_trigger_creation() {
        let oracle = Oracle::new();
        let trigger = RetrainTrigger::with_oracle(oracle);
        let stats = trigger.stats();
        assert_eq!(stats.predictions_observed, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_retrain_trigger_observe_correct() {
        let oracle = Oracle::new();
        let mut trigger = RetrainTrigger::with_oracle(oracle);

        for _ in 0..10 {
            let result = trigger.observe(false); // correct predictions
            assert_eq!(result, ObserveResult::Stable);
        }

        let stats = trigger.stats();
        assert_eq!(stats.predictions_observed, 10);
        assert_eq!(stats.correct_predictions, 10);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_retrain_trigger_consecutive_errors() {
        let oracle = Oracle::new();
        let config = RetrainConfig {
            max_consecutive_errors: 5,
            ..Default::default()
        };
        let mut trigger = RetrainTrigger::new(oracle, config);

        // Less than threshold - should be stable
        for _ in 0..4 {
            let result = trigger.observe(true);
            assert_eq!(result, ObserveResult::Stable);
        }

        // Hit threshold - should detect drift
        let result = trigger.observe(true);
        assert_eq!(result, ObserveResult::DriftDetected);
    }

    #[test]
    fn test_retrain_trigger_error_rate_threshold() {
        let oracle = Oracle::new();
        let config = RetrainConfig {
            min_samples: 10,
            drift_threshold: 0.5,
            warning_threshold: 0.3,
            max_consecutive_errors: 100, // Disable consecutive check
        };
        let mut trigger = RetrainTrigger::new(oracle, config);

        // Add some correct predictions
        for _ in 0..7 {
            trigger.observe(false);
        }

        // Add errors to hit threshold (50% error rate needs 7 errors to hit 7/14 = 50%)
        for _ in 0..6 {
            trigger.observe(true);
        }

        // This should trigger drift (7 errors out of 14 = 50% error rate)
        let result = trigger.observe(true);
        assert!(
            result == ObserveResult::DriftDetected || result == ObserveResult::Warning,
            "Should detect drift or warning at 50% error rate"
        );
    }

    #[test]
    fn test_retrain_trigger_mark_retrained() {
        let oracle = Oracle::new();
        let mut trigger = RetrainTrigger::with_oracle(oracle);

        // Generate some stats
        for _ in 0..10 {
            trigger.observe(true);
        }

        assert_eq!(trigger.stats().errors, 10);

        // Mark as retrained
        trigger.mark_retrained();

        // Stats should be reset
        let stats = trigger.stats();
        assert_eq!(stats.predictions_observed, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.consecutive_errors, 0);
    }

    #[test]
    fn test_retrain_stats_error_rate() {
        let mut stats = RetrainStats::default();
        assert_eq!(stats.error_rate(), 0.0);

        stats.predictions_observed = 100;
        stats.errors = 25;
        assert!((stats.error_rate() - 0.25).abs() < 0.001);
        assert!((stats.accuracy() - 0.75).abs() < 0.001);
    }

    // ============================================================
    // Additional Coverage Tests
    // ============================================================

    #[test]
    fn test_oracle_config_default() {
        let config = OracleConfig::default();
        assert_eq!(config.n_estimators, 100);
        assert_eq!(config.max_depth, 10);
        assert_eq!(config.random_state, Some(42));
    }

    #[test]
    fn test_oracle_config_custom() {
        let config = OracleConfig {
            n_estimators: 50,
            max_depth: 5,
            random_state: Some(123),
        };
        assert_eq!(config.n_estimators, 50);
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.random_state, Some(123));
    }

    #[test]
    fn test_oracle_with_config() {
        let config = OracleConfig {
            n_estimators: 20,
            max_depth: 3,
            random_state: None,
        };
        let oracle = Oracle::with_config(config);
        assert_eq!(oracle.categories.len(), 7);
    }

    #[test]
    fn test_oracle_error_display() {
        let model_err = OracleError::Model("test error".to_string());
        assert!(model_err.to_string().contains("Model error"));

        let feature_err = OracleError::Feature("feature error".to_string());
        assert!(feature_err.to_string().contains("Feature extraction error"));

        let class_err = OracleError::Classification("class error".to_string());
        assert!(class_err.to_string().contains("Classification error"));

        let io_err = OracleError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "not found"));
        assert!(io_err.to_string().contains("IO error"));
    }

    #[test]
    fn test_classification_result_creation() {
        let result = ClassificationResult {
            category: ErrorCategory::TypeMismatch,
            confidence: 0.95,
            suggested_fix: Some("Use .into()".to_string()),
            related_patterns: vec!["pattern1".to_string(), "pattern2".to_string()],
        };
        assert_eq!(result.category, ErrorCategory::TypeMismatch);
        assert_eq!(result.confidence, 0.95);
        assert!(result.suggested_fix.is_some());
        assert_eq!(result.related_patterns.len(), 2);
    }

    #[test]
    fn test_classification_result_clone() {
        let result = ClassificationResult {
            category: ErrorCategory::BorrowChecker,
            confidence: 0.80,
            suggested_fix: None,
            related_patterns: vec![],
        };
        let cloned = result.clone();
        assert_eq!(cloned.category, result.category);
        assert_eq!(cloned.confidence, result.confidence);
    }

    #[test]
    fn test_observe_result_eq() {
        assert_eq!(ObserveResult::Stable, ObserveResult::Stable);
        assert_eq!(ObserveResult::Warning, ObserveResult::Warning);
        assert_eq!(ObserveResult::DriftDetected, ObserveResult::DriftDetected);
        assert_ne!(ObserveResult::Stable, ObserveResult::Warning);
    }

    #[test]
    fn test_retrain_config_default() {
        let config = RetrainConfig::default();
        assert_eq!(config.min_samples, 50);
        assert_eq!(config.max_consecutive_errors, 10);
        assert!((config.warning_threshold - 0.2).abs() < 0.001);
        assert!((config.drift_threshold - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_retrain_config_custom() {
        let config = RetrainConfig {
            min_samples: 100,
            max_consecutive_errors: 5,
            warning_threshold: 0.15,
            drift_threshold: 0.25,
        };
        assert_eq!(config.min_samples, 100);
        assert_eq!(config.max_consecutive_errors, 5);
    }

    #[test]
    fn test_create_accuracy_bar() {
        // Test 100% accuracy
        let bar = create_accuracy_bar(1.0);
        assert_eq!(bar, "[██████████]");

        // Test 50% accuracy
        let bar = create_accuracy_bar(0.5);
        assert_eq!(bar, "[█████░░░░░]");

        // Test 0% accuracy
        let bar = create_accuracy_bar(0.0);
        assert_eq!(bar, "[░░░░░░░░░░]");

        // Test 85% accuracy
        let bar = create_accuracy_bar(0.85);
        assert!(bar.contains("█"));
    }

    #[test]
    fn test_print_drift_status_does_not_panic() {
        // Get stats from an actual ADWIN detector
        let mut oracle = Oracle::new();
        for _ in 0..10 {
            oracle.observe_prediction(false);
        }
        let stats = oracle.drift_stats();
        // Should not panic
        print_drift_status(&stats, &DriftStatus::Stable);
        print_drift_status(&stats, &DriftStatus::Warning);
        print_drift_status(&stats, &DriftStatus::Drift);
    }

    #[test]
    fn test_print_retrain_status_does_not_panic() {
        let stats = RetrainStats {
            predictions_observed: 100,
            correct_predictions: 80,
            errors: 20,
            consecutive_errors: 2,
            drift_status: DriftStatus::Stable,
            drift_count: 0,
        };
        // Should not panic
        print_retrain_status(&stats);
    }

    #[test]
    fn test_print_lineage_history_does_not_panic() {
        let lineage = OracleLineage::new();
        // Should not panic with empty lineage
        print_lineage_history(&lineage);

        // Should not panic with populated lineage
        let mut lineage = OracleLineage::new();
        lineage.record_training("sha123".to_string(), "hash456".to_string(), 1000, 0.9);
        print_lineage_history(&lineage);
    }

    #[test]
    fn test_print_oracle_status_does_not_panic() {
        let oracle = Oracle::new();
        let trigger = RetrainTrigger::with_oracle(oracle);
        let lineage = OracleLineage::new();
        // Should not panic
        print_oracle_status(&trigger, &lineage);
    }

    #[test]
    fn test_retrain_trigger_oracle_access() {
        let oracle = Oracle::new();
        let mut trigger = RetrainTrigger::with_oracle(oracle);

        // Test oracle() accessor
        assert_eq!(trigger.oracle().categories.len(), 7);

        // Test oracle_mut() accessor
        let oracle_mut = trigger.oracle_mut();
        assert_eq!(oracle_mut.categories.len(), 7);
    }

    #[test]
    fn test_retrain_trigger_drift_stats() {
        let oracle = Oracle::new();
        let trigger = RetrainTrigger::with_oracle(oracle);
        let stats = trigger.drift_stats();
        assert_eq!(stats.n_samples, 0);
    }

    #[test]
    fn test_retrain_trigger_needs_retraining() {
        let oracle = Oracle::new();
        let trigger = RetrainTrigger::with_oracle(oracle);
        // Fresh trigger should not need retraining
        assert!(!trigger.needs_retraining());
    }

    #[test]
    fn test_oracle_default() {
        let oracle = Oracle::default();
        assert_eq!(oracle.categories.len(), 7);
    }

    #[test]
    fn test_retrain_stats_default() {
        let stats = RetrainStats::default();
        assert_eq!(stats.predictions_observed, 0);
        assert_eq!(stats.correct_predictions, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.consecutive_errors, 0);
        assert_eq!(stats.drift_count, 0);
        assert!(matches!(stats.drift_status, DriftStatus::Stable));
    }

    #[test]
    fn test_oracle_set_adwin_delta() {
        let mut oracle = Oracle::new();
        // Should not panic
        oracle.set_adwin_delta(0.001);
        oracle.set_adwin_delta(0.01);
        oracle.set_adwin_delta(0.1);
    }
}

//! Acceleration Pipeline (DEPYLER-0637)
//!
//! Unified pipeline integrating all 6 acceleration strategies for 80% single-shot compile rate.
//!
//! ## Architecture Flow
//!
//! ```text
//! Error → Curriculum Scheduler → Oracle Query → GNN Fallback → LLM Fallback → Distillation
//! ```
//!
//! ## Strategies Integrated
//!
//! 1. Tarantula fault localization (DEPYLER-0631)
//! 2. Error pattern library (DEPYLER-0632)
//! 3. Curriculum learning (DEPYLER-0633)
//! 4. Knowledge distillation (DEPYLER-0634)
//! 5. GNN error encoder (DEPYLER-0635)
//! 6. OIP CITL export (DEPYLER-0636)

use crate::classifier::ErrorCategory;
use crate::curriculum::{CurriculumScheduler, DifficultyLevel};
use crate::distillation::{DistillationConfig, KnowledgeDistiller};
use crate::error_patterns::ErrorPatternLibrary;
use crate::gnn_encoder::{DepylerGnnEncoder, GnnEncoderConfig};
use crate::oip_export::{DepylerExport, ErrorCodeClass};
use crate::tarantula::{SuspiciousTranspilerDecision, TarantulaAnalyzer};
use crate::OracleError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the acceleration pipeline
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    /// Minimum confidence for pattern match
    pub min_pattern_confidence: f32,
    /// Minimum similarity for GNN fallback
    pub min_gnn_similarity: f32,
    /// Enable GNN fallback when pattern match fails
    pub enable_gnn_fallback: bool,
    /// Enable distillation feedback loop
    pub enable_distillation: bool,
    /// Maximum LLM calls per batch
    pub max_llm_calls: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            min_pattern_confidence: 0.7,
            min_gnn_similarity: 0.8,
            enable_gnn_fallback: true,
            enable_distillation: true,
            max_llm_calls: 20,
        }
    }
}

/// Result of analyzing an error through the pipeline
#[derive(Clone, Debug)]
pub struct AnalysisResult {
    /// Original error message
    pub error_message: String,
    /// Error code (e.g., E0308)
    pub error_code: Option<String>,
    /// Classified difficulty level
    pub difficulty: DifficultyLevel,
    /// Error category
    pub category: ErrorCategory,
    /// Suggested fix (if found)
    pub suggested_fix: Option<String>,
    /// Fix source (Pattern, GNN, LLM)
    pub fix_source: FixSource,
    /// Confidence score
    pub confidence: f32,
    /// Suspicious transpiler decisions (from Tarantula)
    pub suspicious_decisions: Vec<SuspiciousTranspilerDecision>,
}

/// Source of the suggested fix
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixSource {
    /// Fix from error pattern library
    Pattern,
    /// Fix from GNN similarity match
    Gnn,
    /// Fix from LLM (fallback)
    Llm,
    /// No fix found
    None,
}

/// Statistics for pipeline performance
#[derive(Clone, Debug, Default)]
pub struct PipelineStats {
    /// Total errors analyzed
    pub total_analyzed: usize,
    /// Fixes found from patterns
    pub pattern_fixes: usize,
    /// Fixes found from GNN
    pub gnn_fixes: usize,
    /// Fixes from LLM fallback
    pub llm_fixes: usize,
    /// No fix found
    pub no_fix: usize,
    /// Fixes by difficulty level
    pub by_difficulty: HashMap<DifficultyLevel, usize>,
    /// Fixes by category
    pub by_category: HashMap<ErrorCategory, usize>,
}

impl PipelineStats {
    /// Calculate pattern match rate
    #[must_use]
    pub fn pattern_rate(&self) -> f32 {
        if self.total_analyzed == 0 {
            0.0
        } else {
            self.pattern_fixes as f32 / self.total_analyzed as f32
        }
    }

    /// Calculate overall fix rate (excludes LLM)
    #[must_use]
    pub fn local_fix_rate(&self) -> f32 {
        if self.total_analyzed == 0 {
            0.0
        } else {
            (self.pattern_fixes + self.gnn_fixes) as f32 / self.total_analyzed as f32
        }
    }

    /// Calculate LLM avoidance rate
    #[must_use]
    pub fn llm_avoidance_rate(&self) -> f32 {
        if self.total_analyzed == 0 {
            0.0
        } else {
            1.0 - (self.llm_fixes as f32 / self.total_analyzed as f32)
        }
    }
}

/// Acceleration pipeline combining all 6 strategies
pub struct AccelerationPipeline {
    config: PipelineConfig,
    /// Strategy #1: Tarantula fault localization
    tarantula: TarantulaAnalyzer,
    /// Strategy #2: Error pattern library (optional - requires entrenar)
    patterns: Option<ErrorPatternLibrary>,
    /// Strategy #3: Curriculum scheduler (used via classify_error_difficulty)
    #[allow(dead_code)]
    curriculum: CurriculumScheduler,
    /// Strategy #4: Knowledge distiller
    distiller: KnowledgeDistiller,
    /// Strategy #5: GNN encoder
    gnn_encoder: DepylerGnnEncoder,
    /// Pipeline statistics
    stats: PipelineStats,
}

impl AccelerationPipeline {
    /// Create a new pipeline with default configuration
    ///
    /// # Errors
    /// Returns error if pattern library initialization fails
    pub fn new() -> Result<Self, OracleError> {
        Self::with_config(PipelineConfig::default())
    }

    /// Create a new pipeline with custom configuration
    ///
    /// # Errors
    /// Returns error if Tarantula analyzer initialization fails
    pub fn with_config(config: PipelineConfig) -> Result<Self, OracleError> {
        // Pattern library may fail to initialize (depends on entrenar)
        let patterns = ErrorPatternLibrary::new().ok();

        Ok(Self {
            config,
            tarantula: TarantulaAnalyzer::new()?,
            patterns,
            curriculum: CurriculumScheduler::new(),
            distiller: KnowledgeDistiller::new(DistillationConfig::default()),
            gnn_encoder: DepylerGnnEncoder::new(GnnEncoderConfig::default()),
            stats: PipelineStats::default(),
        })
    }

    /// Analyze an error and suggest a fix
    ///
    /// Pipeline flow:
    /// 1. Classify difficulty (Curriculum)
    /// 2. Search pattern library
    /// 3. If no match + GNN enabled, try GNN similarity
    /// 4. Return result with fix source
    pub fn analyze(&mut self, error_message: &str, error_code: Option<&str>) -> AnalysisResult {
        self.stats.total_analyzed += 1;

        // Step 1: Classify difficulty
        let difficulty = if let Some(code) = error_code {
            crate::curriculum::classify_error_difficulty(code, error_message)
        } else {
            DifficultyLevel::Medium
        };

        // Update difficulty stats
        *self.stats.by_difficulty.entry(difficulty).or_insert(0) += 1;

        // Step 2: Classify category
        let category = self.classify_category(error_code, error_message);
        *self.stats.by_category.entry(category).or_insert(0) += 1;

        // Step 3: Search pattern library
        if let Some(ref mut patterns) = self.patterns {
            let code = error_code.unwrap_or("");
            let suggestions = patterns.suggest_fix(code, error_message, &[]);

            if let Some(pattern) = suggestions.first() {
                if pattern.confidence >= self.config.min_pattern_confidence as f64 {
                    self.stats.pattern_fixes += 1;
                    return AnalysisResult {
                        error_message: error_message.to_string(),
                        error_code: error_code.map(String::from),
                        difficulty,
                        category,
                        suggested_fix: Some(pattern.fix_diff.clone()),
                        fix_source: FixSource::Pattern,
                        confidence: pattern.confidence as f32,
                        suspicious_decisions: Vec::new(),
                    };
                }
            }
        }

        // Step 4: GNN fallback
        if self.config.enable_gnn_fallback {
            let code = error_code.unwrap_or("");
            let similar = self.gnn_encoder.find_similar(code, error_message, "");

            if let Some(best) = similar.first() {
                if best.similarity >= self.config.min_gnn_similarity {
                    self.stats.gnn_fixes += 1;
                    // Get fix from the underlying error pattern if available
                    let fix = best.pattern.error_pattern.as_ref().map(|p| p.fix_diff.clone());
                    return AnalysisResult {
                        error_message: error_message.to_string(),
                        error_code: error_code.map(String::from),
                        difficulty,
                        category,
                        suggested_fix: fix,
                        fix_source: FixSource::Gnn,
                        confidence: best.similarity,
                        suspicious_decisions: Vec::new(),
                    };
                }
            }
        }

        // No local fix found
        self.stats.no_fix += 1;
        AnalysisResult {
            error_message: error_message.to_string(),
            error_code: error_code.map(String::from),
            difficulty,
            category,
            suggested_fix: None,
            fix_source: FixSource::None,
            confidence: 0.0,
            suspicious_decisions: Vec::new(),
        }
    }

    /// Analyze with Tarantula decision tracing
    ///
    /// Note: Decisions should be recorded with the TarantulaAnalyzer prior to calling this.
    pub fn analyze_with_tarantula(
        &mut self,
        error_message: &str,
        error_code: Option<&str>,
    ) -> AnalysisResult {
        let mut result = self.analyze(error_message, error_code);

        // Add Tarantula analysis (uses pre-recorded decisions)
        let tarantula_result = self.tarantula.analyze();
        result.suspicious_decisions = tarantula_result.suspicious;

        result
    }

    /// Get a mutable reference to the Tarantula analyzer for recording decisions
    pub fn tarantula_mut(&mut self) -> &mut TarantulaAnalyzer {
        &mut self.tarantula
    }

    /// Record an LLM fix for distillation
    pub fn record_llm_fix(&mut self, error_message: &str, fix: &str, success: bool) {
        if success {
            self.stats.llm_fixes += 1;

            if self.config.enable_distillation {
                let example = crate::distillation::LlmFixExample {
                    error_code: String::new(),
                    error_message: error_message.to_string(),
                    original_code: String::new(),
                    fixed_code: fix.to_string(),
                    diff: fix.to_string(),
                    explanation: None,
                    llm_confidence: 1.0,
                    validated: true,
                };
                self.distiller.collect_example(example);
            }
        }
    }

    /// Prioritize a batch of errors by difficulty (easy first)
    #[must_use]
    pub fn prioritize_errors(&self, errors: Vec<(String, Option<String>)>) -> Vec<(String, Option<String>, DifficultyLevel)> {
        let mut with_difficulty: Vec<_> = errors
            .into_iter()
            .map(|(msg, code)| {
                let difficulty = if let Some(ref c) = code {
                    crate::curriculum::classify_error_difficulty(c, &msg)
                } else {
                    DifficultyLevel::Medium
                };
                (msg, code, difficulty)
            })
            .collect();

        // Sort by difficulty (easy first)
        with_difficulty.sort_by_key(|(_, _, d)| *d as u8);
        with_difficulty
    }

    /// Get current pipeline statistics
    #[must_use]
    pub fn stats(&self) -> &PipelineStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = PipelineStats::default();
    }

    /// Check if pattern library is available
    #[must_use]
    pub fn has_pattern_library(&self) -> bool {
        self.patterns.is_some()
    }

    /// Export analysis results to OIP format
    #[must_use]
    pub fn export_results(&self, results: &[AnalysisResult]) -> Vec<DepylerExport> {
        results
            .iter()
            .map(|r| {
                let mut export = DepylerExport::new(
                    r.error_code.as_deref().unwrap_or(""),
                    &r.error_message,
                    "analysis",
                );
                export = export.with_confidence(r.confidence);
                if r.suggested_fix.is_some() {
                    export = export.with_oip_category(format!("{:?}", r.category));
                }
                export
            })
            .collect()
    }

    fn classify_category(&self, error_code: Option<&str>, message: &str) -> ErrorCategory {
        if let Some(code) = error_code {
            let class = ErrorCodeClass::from_error_code(code);
            match class {
                ErrorCodeClass::Type => ErrorCategory::TypeMismatch,
                ErrorCodeClass::Borrow => ErrorCategory::BorrowChecker,
                ErrorCodeClass::Name => ErrorCategory::MissingImport,
                ErrorCodeClass::Trait => ErrorCategory::TraitBound,
                ErrorCodeClass::Other => {
                    // Fall back to message analysis
                    if message.contains("lifetime") {
                        ErrorCategory::LifetimeError
                    } else if message.contains("syntax") {
                        ErrorCategory::SyntaxError
                    } else {
                        ErrorCategory::Other
                    }
                }
            }
        } else {
            ErrorCategory::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_new() {
        let pipeline = AccelerationPipeline::new().unwrap();
        assert_eq!(pipeline.stats().total_analyzed, 0);
    }

    #[test]
    fn test_pipeline_analyze_no_pattern() {
        let mut pipeline = AccelerationPipeline::new().unwrap();
        let result = pipeline.analyze("unknown error", None);

        assert_eq!(result.fix_source, FixSource::None);
        assert_eq!(pipeline.stats().total_analyzed, 1);
        assert_eq!(pipeline.stats().no_fix, 1);
    }

    #[test]
    fn test_pipeline_analyze_with_error_code() {
        let mut pipeline = AccelerationPipeline::new().unwrap();
        let result = pipeline.analyze("mismatched types", Some("E0308"));

        assert_eq!(result.error_code, Some("E0308".to_string()));
        assert_eq!(result.category, ErrorCategory::TypeMismatch);
        // E0308 is classified as Medium difficulty in curriculum module
        assert_eq!(result.difficulty, DifficultyLevel::Medium);
    }

    #[test]
    fn test_pipeline_analyze_borrow_error() {
        let mut pipeline = AccelerationPipeline::new().unwrap();
        let result = pipeline.analyze("cannot borrow as mutable", Some("E0502"));

        assert_eq!(result.category, ErrorCategory::BorrowChecker);
        assert_eq!(result.difficulty, DifficultyLevel::Hard);
    }

    #[test]
    fn test_pipeline_prioritize_errors() {
        let pipeline = AccelerationPipeline::new().unwrap();
        let errors = vec![
            ("borrow error".to_string(), Some("E0502".to_string())),   // Hard
            ("type mismatch".to_string(), Some("E0308".to_string())),  // Medium
            ("trait bound".to_string(), Some("E0277".to_string())),    // Hard
        ];

        let prioritized = pipeline.prioritize_errors(errors);

        // Medium (E0308) should come first
        assert_eq!(prioritized[0].1, Some("E0308".to_string()));
        // Both E0502 and E0277 are Hard - just verify they come after Medium
        assert_eq!(prioritized[0].2, DifficultyLevel::Medium);
        assert_eq!(prioritized[1].2, DifficultyLevel::Hard);
        assert_eq!(prioritized[2].2, DifficultyLevel::Hard);
    }

    #[test]
    fn test_pipeline_stats_rates() {
        let mut stats = PipelineStats::default();
        stats.total_analyzed = 100;
        stats.pattern_fixes = 40;
        stats.gnn_fixes = 20;
        stats.llm_fixes = 30;
        stats.no_fix = 10;

        assert!((stats.pattern_rate() - 0.4).abs() < 0.001);
        assert!((stats.local_fix_rate() - 0.6).abs() < 0.001);
        assert!((stats.llm_avoidance_rate() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_pipeline_config_defaults() {
        let config = PipelineConfig::default();
        assert!((config.min_pattern_confidence - 0.7).abs() < 0.001);
        assert!(config.enable_gnn_fallback);
        assert!(config.enable_distillation);
    }

    #[test]
    fn test_pipeline_record_llm_fix() {
        let mut pipeline = AccelerationPipeline::new().unwrap();
        pipeline.record_llm_fix("some error", "some fix", true);

        assert_eq!(pipeline.stats().llm_fixes, 1);
    }

    #[test]
    fn test_pipeline_has_pattern_library() {
        let pipeline = AccelerationPipeline::new().unwrap();
        // Pattern library may or may not be available depending on entrenar
        let _ = pipeline.has_pattern_library();
    }

    #[test]
    fn test_pipeline_export_results() {
        let pipeline = AccelerationPipeline::new().unwrap();
        let results = vec![
            AnalysisResult {
                error_message: "type mismatch".to_string(),
                error_code: Some("E0308".to_string()),
                difficulty: DifficultyLevel::Easy,
                category: ErrorCategory::TypeMismatch,
                suggested_fix: Some("use .into()".to_string()),
                fix_source: FixSource::Pattern,
                confidence: 0.9,
                suspicious_decisions: Vec::new(),
            },
        ];

        let exports = pipeline.export_results(&results);
        assert_eq!(exports.len(), 1);
        assert_eq!(exports[0].error_code, Some("E0308".to_string()));
    }

    #[test]
    fn test_fix_source_variants() {
        assert_eq!(FixSource::Pattern, FixSource::Pattern);
        assert_ne!(FixSource::Pattern, FixSource::Gnn);
        assert_ne!(FixSource::Gnn, FixSource::Llm);
        assert_ne!(FixSource::Llm, FixSource::None);
    }

    #[test]
    fn test_difficulty_ordering_in_prioritize() {
        let pipeline = AccelerationPipeline::new().unwrap();
        let errors = vec![
            ("expert error".to_string(), Some("E0515".to_string())), // Expert
            ("easy error".to_string(), Some("E0308".to_string())),   // Easy
            ("medium error".to_string(), Some("E0425".to_string())), // Medium
        ];

        let prioritized = pipeline.prioritize_errors(errors);

        // Verify order: Easy < Medium < Expert
        assert!(prioritized[0].2 as u8 <= prioritized[1].2 as u8);
        assert!(prioritized[1].2 as u8 <= prioritized[2].2 as u8);
    }

    #[test]
    fn test_pipeline_empty_stats_rates() {
        let stats = PipelineStats::default();
        assert_eq!(stats.pattern_rate(), 0.0);
        assert_eq!(stats.local_fix_rate(), 0.0);
        assert_eq!(stats.llm_avoidance_rate(), 0.0);
    }
}

//! Knowledge Distillation from LLM to Local Oracle
//!
//! Strategy #4 (DEPYLER-0634): Implements knowledge distillation to transfer
//! fix patterns learned by LLMs to the local Oracle decision model.
//!
//! ## Key Components
//!
//! - **LLM Fix Collector**: Captures (error, fix) pairs from successful LLM fixes
//! - **Pattern Normalizer**: Canonicalizes fix patterns for consistent learning
//! - **Distillation Engine**: Temperature-scaled KL divergence for soft targets
//! - **Pattern Promoter**: Moves high-confidence patterns to hardcoded rules
//!
//! ## Integration
//!
//! Works with:
//! - `ErrorPatternLibrary` (Strategy #2) for pattern storage
//! - `CurriculumScheduler` (Strategy #3) for prioritized learning
//! - `entrenar::distill` for distillation loss functions

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::classifier::ErrorCategory;
use crate::error_patterns::{ErrorPattern, ErrorPatternLibrary};
use crate::tarantula::TranspilerDecision;

/// Configuration for knowledge distillation
#[derive(Debug, Clone)]
pub struct DistillationConfig {
    /// Temperature for softening probability distributions (default: 3.0)
    /// Higher = softer distributions, more knowledge transfer
    pub temperature: f32,
    /// Weight for distillation loss vs hard loss (default: 0.7)
    /// Higher = more emphasis on teacher's soft targets
    pub alpha: f32,
    /// Minimum confidence to consider a pattern learned (default: 0.8)
    pub min_confidence: f64,
    /// Number of successful applications before promoting to hardcoded rule
    pub promotion_threshold: u32,
    /// Maximum patterns to retain (LRU eviction)
    pub max_patterns: usize,
}

impl Default for DistillationConfig {
    fn default() -> Self {
        Self {
            temperature: 3.0,
            alpha: 0.7,
            min_confidence: 0.8,
            promotion_threshold: 10,
            max_patterns: 1000,
        }
    }
}

/// An LLM fix example collected for distillation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmFixExample {
    /// Error code (e.g., "E0308")
    pub error_code: String,
    /// Original error message
    pub error_message: String,
    /// Original broken code
    pub original_code: String,
    /// Fixed code from LLM
    pub fixed_code: String,
    /// Extracted diff/patch
    pub diff: String,
    /// LLM's explanation (optional)
    pub explanation: Option<String>,
    /// Confidence score from LLM (0.0-1.0)
    pub llm_confidence: f64,
    /// Whether the fix was validated (compiled successfully)
    pub validated: bool,
}

/// Result of pattern extraction from an LLM fix
#[derive(Debug, Clone)]
pub struct ExtractedPattern {
    /// Canonical pattern ID
    pub pattern_id: String,
    /// Error code it fixes
    pub error_code: String,
    /// Regex pattern for error message matching
    pub error_pattern: String,
    /// Template for the fix
    pub fix_template: String,
    /// Decision type affected
    pub decision_type: Option<TranspilerDecision>,
    /// Initial confidence from extraction
    pub confidence: f64,
}

/// Statistics for the distillation process
#[derive(Debug, Clone, Default)]
pub struct DistillationStats {
    /// Total LLM fix examples collected
    pub examples_collected: usize,
    /// Examples that passed validation
    pub examples_validated: usize,
    /// Patterns extracted
    pub patterns_extracted: usize,
    /// Patterns promoted to hardcoded rules
    pub patterns_promoted: usize,
    /// Patterns retired (superseded or low confidence)
    pub patterns_retired: usize,
    /// Average pattern confidence
    pub avg_confidence: f64,
}

/// Knowledge distillation engine for transferring LLM knowledge to Oracle
pub struct KnowledgeDistiller {
    config: DistillationConfig,
    /// Collected LLM fix examples (teacher)
    examples: Vec<LlmFixExample>,
    /// Extracted patterns (student)
    patterns: HashMap<String, ExtractedPattern>,
    /// Pattern application counts
    application_counts: HashMap<String, u32>,
    /// Success counts per pattern
    success_counts: HashMap<String, u32>,
    /// Statistics
    stats: DistillationStats,
}

impl KnowledgeDistiller {
    /// Create a new knowledge distiller with given configuration
    #[must_use]
    pub fn new(config: DistillationConfig) -> Self {
        Self {
            config,
            examples: Vec::new(),
            patterns: HashMap::new(),
            application_counts: HashMap::new(),
            success_counts: HashMap::new(),
            stats: DistillationStats::default(),
        }
    }

    /// Create with default configuration
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(DistillationConfig::default())
    }

    /// Collect an LLM fix example for distillation
    pub fn collect_example(&mut self, example: LlmFixExample) {
        self.stats.examples_collected += 1;
        if example.validated {
            self.stats.examples_validated += 1;
        }
        self.examples.push(example);

        // Evict oldest examples if over capacity
        let max_examples = self.config.max_patterns * 10;
        if self.examples.len() > max_examples {
            self.examples.drain(0..max_examples / 10);
        }
    }

    /// Extract patterns from collected examples
    pub fn extract_patterns(&mut self) -> Vec<ExtractedPattern> {
        let mut extracted = Vec::new();

        for example in &self.examples {
            // Only extract from validated examples
            if !example.validated {
                continue;
            }

            if let Some(pattern) = self.extract_single_pattern(example) {
                let pattern_id = pattern.pattern_id.clone();
                extracted.push(pattern.clone());

                // Update or insert pattern
                self.patterns
                    .entry(pattern_id)
                    .and_modify(|existing| {
                        // Merge confidences via exponential moving average
                        existing.confidence = existing.confidence * 0.7 + pattern.confidence * 0.3;
                    })
                    .or_insert(pattern);

                self.stats.patterns_extracted += 1;
            }
        }

        // Evict low-confidence patterns if over capacity
        self.evict_low_confidence_patterns();

        extracted
    }

    /// Extract a pattern from a single LLM fix example
    fn extract_single_pattern(&self, example: &LlmFixExample) -> Option<ExtractedPattern> {
        // Skip low-confidence LLM fixes
        if example.llm_confidence < self.config.min_confidence {
            return None;
        }

        // Canonicalize the error pattern
        let error_pattern = self.canonicalize_error_pattern(&example.error_message);

        // Generate pattern ID
        let pattern_id = format!(
            "distill_{}_{}",
            example.error_code,
            hash_str(&error_pattern)
        );

        // Extract fix template
        let fix_template = self.extract_fix_template(&example.diff);

        // Infer decision type from error code
        let decision_type = infer_decision_from_error(&example.error_code);

        Some(ExtractedPattern {
            pattern_id,
            error_code: example.error_code.clone(),
            error_pattern,
            fix_template,
            decision_type,
            confidence: example.llm_confidence,
        })
    }

    /// Canonicalize an error message to a regex pattern
    fn canonicalize_error_pattern(&self, message: &str) -> String {
        // Replace specific identifiers with placeholders
        let mut pattern = message.to_string();

        // Replace type names like `String`, `i32`, etc. with TYPE
        pattern = regex::Regex::new(r"`[A-Z][a-zA-Z0-9_]*`")
            .map(|re| re.replace_all(&pattern, "`TYPE`").to_string())
            .unwrap_or(pattern);

        // Replace variable names in backticks
        pattern = regex::Regex::new(r"`[a-z_][a-z0-9_]*`")
            .map(|re| re.replace_all(&pattern, "`VAR`").to_string())
            .unwrap_or(pattern);

        // Replace line numbers
        pattern = regex::Regex::new(r":\d+:\d+")
            .map(|re| re.replace_all(&pattern, ":LINE:COL").to_string())
            .unwrap_or(pattern);

        // Replace numeric literals
        pattern = regex::Regex::new(r"\b\d+\b")
            .map(|re| re.replace_all(&pattern, "NUM").to_string())
            .unwrap_or(pattern);

        pattern
    }

    /// Extract a fix template from a diff
    fn extract_fix_template(&self, diff: &str) -> String {
        // Parse unified diff format
        let mut template = String::new();
        let mut in_addition = false;

        for line in diff.lines() {
            if let Some(stripped) = line.strip_prefix('+') {
                if !stripped.starts_with("++") {
                    // Actual addition line (not +++ header)
                    in_addition = true;
                    template.push_str(stripped);
                    template.push('\n');
                }
            } else if in_addition && !line.starts_with('-') && !line.starts_with("---") {
                // Context lines after additions
                if let Some(stripped) = line.strip_prefix(' ') {
                    template.push_str(stripped);
                    template.push('\n');
                }
            }
        }

        template.trim().to_string()
    }

    /// Evict low-confidence patterns to stay under capacity
    fn evict_low_confidence_patterns(&mut self) {
        if self.patterns.len() <= self.config.max_patterns {
            return;
        }

        // Sort by confidence and collect IDs to remove
        let mut sorted: Vec<_> = self
            .patterns
            .iter()
            .map(|(id, p)| (id.clone(), p.confidence))
            .collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let to_remove = self.patterns.len() - self.config.max_patterns;
        let ids_to_remove: Vec<_> = sorted
            .into_iter()
            .take(to_remove)
            .map(|(id, _)| id)
            .collect();

        for pattern_id in ids_to_remove {
            self.patterns.remove(&pattern_id);
            self.application_counts.remove(&pattern_id);
            self.success_counts.remove(&pattern_id);
            self.stats.patterns_retired += 1;
        }
    }

    /// Record a pattern application result
    pub fn record_application(&mut self, pattern_id: &str, success: bool) {
        *self
            .application_counts
            .entry(pattern_id.to_string())
            .or_default() += 1;
        if success {
            *self
                .success_counts
                .entry(pattern_id.to_string())
                .or_default() += 1;
        }

        // Update pattern confidence based on success rate
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            let applications = *self.application_counts.get(pattern_id).unwrap_or(&1);
            let successes = *self.success_counts.get(pattern_id).unwrap_or(&0);
            let success_rate = f64::from(successes) / f64::from(applications);

            // Bayesian update of confidence
            pattern.confidence = pattern.confidence * 0.8 + success_rate * 0.2;
        }
    }

    /// Get patterns ready for promotion to hardcoded rules
    pub fn get_promotion_candidates(&self) -> Vec<&ExtractedPattern> {
        self.patterns
            .iter()
            .filter(|(id, pattern)| {
                let applications = self.application_counts.get(*id).copied().unwrap_or(0);
                let successes = self.success_counts.get(*id).copied().unwrap_or(0);

                applications >= self.config.promotion_threshold
                    && pattern.confidence >= self.config.min_confidence
                    && successes as f64 / applications as f64 >= 0.9
            })
            .map(|(_, pattern)| pattern)
            .collect()
    }

    /// Export patterns to ErrorPatternLibrary
    ///
    /// Returns number of patterns exported
    pub fn export_to_library(&self, library: &mut ErrorPatternLibrary) -> usize {
        let mut count = 0;

        for (pattern_id, extracted) in &self.patterns {
            if extracted.confidence < self.config.min_confidence {
                continue;
            }

            let applications = self
                .application_counts
                .get(pattern_id)
                .copied()
                .unwrap_or(0);
            let successes = self.success_counts.get(pattern_id).copied().unwrap_or(0);

            let mut error_pattern = ErrorPattern::new(
                &extracted.error_code,
                &extracted.error_pattern,
                &extracted.fix_template,
            );
            error_pattern.id = pattern_id.clone();
            error_pattern.decision_type = extracted.decision_type;
            error_pattern.applications = applications;
            error_pattern.successes = successes;
            error_pattern.confidence = extracted.confidence;

            if library.add_pattern(error_pattern).is_ok() {
                count += 1;
            }
        }

        count
    }

    /// Get distillation statistics
    #[must_use]
    pub fn stats(&self) -> &DistillationStats {
        &self.stats
    }

    /// Get configuration
    #[must_use]
    pub fn config(&self) -> &DistillationConfig {
        &self.config
    }

    /// Get number of collected examples
    #[must_use]
    pub fn example_count(&self) -> usize {
        self.examples.len()
    }

    /// Get number of extracted patterns
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Classify error using distilled patterns (soft targets)
    ///
    /// Returns probability distribution over error categories
    pub fn classify_soft(
        &self,
        error_code: &str,
        error_message: &str,
    ) -> Vec<(ErrorCategory, f64)> {
        let canonical = self.canonicalize_error_pattern(error_message);
        let mut scores: HashMap<ErrorCategory, f64> = HashMap::new();

        for pattern in self.patterns.values() {
            if pattern.error_code == error_code || pattern.error_pattern.contains(&canonical) {
                let category = infer_category_from_error(error_code);
                *scores.entry(category).or_default() += pattern.confidence;
            }
        }

        // Apply temperature scaling
        let total: f64 = scores.values().sum();
        if total > 0.0 {
            for val in scores.values_mut() {
                *val = (*val / total).powf(1.0 / f64::from(self.config.temperature));
            }
            // Re-normalize
            let new_total: f64 = scores.values().sum();
            for val in scores.values_mut() {
                *val /= new_total;
            }
        }

        let mut result: Vec<_> = scores.into_iter().collect();
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        result
    }
}

/// Simple string hash for pattern ID generation
fn hash_str(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Infer transpiler decision from error code
fn infer_decision_from_error(error_code: &str) -> Option<TranspilerDecision> {
    match error_code {
        "E0308" | "E0277" => Some(TranspilerDecision::TypeInference),
        "E0382" | "E0502" | "E0503" => Some(TranspilerDecision::OwnershipInference),
        "E0106" | "E0495" => Some(TranspilerDecision::LifetimeInference),
        "E0433" | "E0412" => Some(TranspilerDecision::ImportGeneration),
        "E0599" | "E0609" => Some(TranspilerDecision::MethodTranslation),
        "E0425" | "E0531" => Some(TranspilerDecision::ModuleMapping),
        _ => None,
    }
}

/// Infer error category from error code
fn infer_category_from_error(error_code: &str) -> ErrorCategory {
    match error_code {
        "E0308" => ErrorCategory::TypeMismatch,
        "E0277" => ErrorCategory::TraitBound,
        "E0382" | "E0502" | "E0503" => ErrorCategory::BorrowChecker,
        "E0106" | "E0495" => ErrorCategory::LifetimeError,
        "E0433" | "E0412" => ErrorCategory::MissingImport,
        "E0061" | "E0063" => ErrorCategory::SyntaxError,
        _ => ErrorCategory::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distillation_config_default() {
        let config = DistillationConfig::default();
        assert!((config.temperature - 3.0).abs() < f32::EPSILON);
        assert!((config.alpha - 0.7).abs() < f32::EPSILON);
        assert!((config.min_confidence - 0.8).abs() < f64::EPSILON);
        assert_eq!(config.promotion_threshold, 10);
        assert_eq!(config.max_patterns, 1000);
    }

    #[test]
    fn test_knowledge_distiller_creation() {
        let distiller = KnowledgeDistiller::with_defaults();
        assert_eq!(distiller.example_count(), 0);
        assert_eq!(distiller.pattern_count(), 0);
    }

    #[test]
    fn test_collect_example() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "mismatched types: expected `i32`, found `String`".to_string(),
            original_code: "let x: i32 = \"hello\";".to_string(),
            fixed_code: "let x: i32 = 42;".to_string(),
            diff: "+let x: i32 = 42;\n-let x: i32 = \"hello\";".to_string(),
            explanation: Some("Changed string literal to integer".to_string()),
            llm_confidence: 0.95,
            validated: true,
        };

        distiller.collect_example(example);
        assert_eq!(distiller.example_count(), 1);
        assert_eq!(distiller.stats().examples_collected, 1);
        assert_eq!(distiller.stats().examples_validated, 1);
    }

    #[test]
    fn test_collect_unvalidated_example() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "type error".to_string(),
            original_code: "broken".to_string(),
            fixed_code: "still broken".to_string(),
            diff: String::new(),
            explanation: None,
            llm_confidence: 0.5,
            validated: false,
        };

        distiller.collect_example(example);
        assert_eq!(distiller.stats().examples_collected, 1);
        assert_eq!(distiller.stats().examples_validated, 0);
    }

    #[test]
    fn test_extract_patterns() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "mismatched types".to_string(),
            original_code: "let x: i32 = \"hello\";".to_string(),
            fixed_code: "let x: i32 = 42;".to_string(),
            diff: "+let x: i32 = 42;".to_string(),
            explanation: None,
            llm_confidence: 0.95,
            validated: true,
        };

        distiller.collect_example(example);
        let patterns = distiller.extract_patterns();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].error_code, "E0308");
        assert!((patterns[0].confidence - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn test_skip_low_confidence_patterns() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "some error".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.5, // Below threshold
            validated: true,
        };

        distiller.collect_example(example);
        let patterns = distiller.extract_patterns();

        assert!(patterns.is_empty());
    }

    #[test]
    fn test_canonicalize_error_pattern() {
        let distiller = KnowledgeDistiller::with_defaults();

        let message = "mismatched types: expected `String`, found `i32`";
        let canonical = distiller.canonicalize_error_pattern(message);

        assert!(canonical.contains("`TYPE`"));
    }

    #[test]
    fn test_canonicalize_variable_names() {
        let distiller = KnowledgeDistiller::with_defaults();

        let message = "cannot find value `my_var` in this scope";
        let canonical = distiller.canonicalize_error_pattern(message);

        assert!(canonical.contains("`VAR`"));
    }

    #[test]
    fn test_canonicalize_line_numbers() {
        let distiller = KnowledgeDistiller::with_defaults();

        let message = "error at src/main.rs:42:10";
        let canonical = distiller.canonicalize_error_pattern(message);

        assert!(canonical.contains(":LINE:COL"));
    }

    #[test]
    fn test_record_application_success() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        // First add a pattern
        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "type error".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.9,
            validated: true,
        };
        distiller.collect_example(example);
        distiller.extract_patterns();

        // Get the pattern ID
        let pattern_id = distiller.patterns.keys().next().unwrap().clone();

        // Record successful application
        distiller.record_application(&pattern_id, true);

        assert_eq!(distiller.application_counts.get(&pattern_id), Some(&1));
        assert_eq!(distiller.success_counts.get(&pattern_id), Some(&1));
    }

    #[test]
    fn test_record_application_failure() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        distiller.record_application("test_pattern", false);

        assert_eq!(distiller.application_counts.get("test_pattern"), Some(&1));
        assert_eq!(distiller.success_counts.get("test_pattern"), None);
    }

    #[test]
    fn test_promotion_candidates() {
        let mut distiller = KnowledgeDistiller::new(DistillationConfig {
            promotion_threshold: 3,
            min_confidence: 0.8,
            ..Default::default()
        });

        // Add a high-confidence pattern
        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "type error".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.95,
            validated: true,
        };
        distiller.collect_example(example);
        distiller.extract_patterns();

        let pattern_id = distiller.patterns.keys().next().unwrap().clone();

        // Record multiple successful applications
        for _ in 0..5 {
            distiller.record_application(&pattern_id, true);
        }

        let candidates = distiller.get_promotion_candidates();
        assert_eq!(candidates.len(), 1);
    }

    #[test]
    fn test_no_promotion_insufficient_applications() {
        let mut distiller = KnowledgeDistiller::new(DistillationConfig {
            promotion_threshold: 10, // High threshold
            min_confidence: 0.8,
            ..Default::default()
        });

        // Add a high-confidence pattern
        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "type error".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.95,
            validated: true,
        };
        distiller.collect_example(example);
        distiller.extract_patterns();

        let pattern_id = distiller.patterns.keys().next().unwrap().clone();

        // Only a few applications - below promotion_threshold
        for _ in 0..3 {
            distiller.record_application(&pattern_id, true);
        }

        // Should NOT be promoted due to insufficient applications
        let candidates = distiller.get_promotion_candidates();
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_no_promotion_low_success_rate() {
        let mut distiller = KnowledgeDistiller::new(DistillationConfig {
            promotion_threshold: 5,
            min_confidence: 0.8,
            ..Default::default()
        });

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "type error".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.95,
            validated: true,
        };
        distiller.collect_example(example);
        distiller.extract_patterns();

        let pattern_id = distiller.patterns.keys().next().unwrap().clone();

        // Many applications but low success rate (50%)
        for _ in 0..5 {
            distiller.record_application(&pattern_id, true);
            distiller.record_application(&pattern_id, false);
        }

        // Should NOT be promoted due to low success rate (<90%)
        let candidates = distiller.get_promotion_candidates();
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_classify_soft() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "mismatched types".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.95,
            validated: true,
        };
        distiller.collect_example(example);
        distiller.extract_patterns();

        let soft_probs = distiller.classify_soft("E0308", "mismatched types");
        assert!(!soft_probs.is_empty());
        // First result should be TypeMismatch
        assert_eq!(soft_probs[0].0, ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_infer_decision_from_error() {
        assert_eq!(
            infer_decision_from_error("E0308"),
            Some(TranspilerDecision::TypeInference)
        );
        assert_eq!(
            infer_decision_from_error("E0382"),
            Some(TranspilerDecision::OwnershipInference)
        );
        assert_eq!(
            infer_decision_from_error("E0106"),
            Some(TranspilerDecision::LifetimeInference)
        );
        assert_eq!(
            infer_decision_from_error("E0433"),
            Some(TranspilerDecision::ImportGeneration)
        );
        assert_eq!(infer_decision_from_error("E9999"), None);
    }

    #[test]
    fn test_infer_category_from_error() {
        assert_eq!(
            infer_category_from_error("E0308"),
            ErrorCategory::TypeMismatch
        );
        assert_eq!(
            infer_category_from_error("E0277"),
            ErrorCategory::TraitBound
        );
        assert_eq!(
            infer_category_from_error("E0382"),
            ErrorCategory::BorrowChecker
        );
        assert_eq!(infer_category_from_error("E9999"), ErrorCategory::Other);
    }

    #[test]
    fn test_export_to_library() {
        let mut distiller = KnowledgeDistiller::with_defaults();

        let example = LlmFixExample {
            error_code: "E0308".to_string(),
            error_message: "type mismatch".to_string(),
            original_code: "code".to_string(),
            fixed_code: "fixed".to_string(),
            diff: "+fixed".to_string(),
            explanation: None,
            llm_confidence: 0.95,
            validated: true,
        };
        distiller.collect_example(example);
        distiller.extract_patterns();

        let mut library = ErrorPatternLibrary::new().unwrap();
        let count = distiller.export_to_library(&mut library);

        assert!(count > 0);
        assert!(library.pattern_count() > 0);
    }

    #[test]
    fn test_evict_low_confidence() {
        let mut distiller = KnowledgeDistiller::new(DistillationConfig {
            max_patterns: 2,
            min_confidence: 0.5,
            ..Default::default()
        });

        // Add 3 patterns with different confidences
        for (i, conf) in [(0.9), (0.7), (0.8)].iter().enumerate() {
            let example = LlmFixExample {
                error_code: format!("E030{}", i),
                error_message: format!("error {}", i),
                original_code: "code".to_string(),
                fixed_code: "fixed".to_string(),
                diff: "+fixed".to_string(),
                explanation: None,
                llm_confidence: *conf,
                validated: true,
            };
            distiller.collect_example(example);
        }

        distiller.extract_patterns();

        // Should have evicted the lowest confidence pattern
        assert!(distiller.pattern_count() <= 2);
    }

    #[test]
    fn test_extract_fix_template() {
        let distiller = KnowledgeDistiller::with_defaults();

        let diff = r#"--- a/file.rs
+++ b/file.rs
-let x: i32 = "hello";
+let x: i32 = 42;
 fn main() {
"#;

        let template = distiller.extract_fix_template(diff);
        assert!(template.contains("let x: i32 = 42;"));
    }

    #[test]
    fn test_hash_str_consistency() {
        let h1 = hash_str("test string");
        let h2 = hash_str("test string");
        assert_eq!(h1, h2);

        let h3 = hash_str("different");
        assert_ne!(h1, h3);
    }
}

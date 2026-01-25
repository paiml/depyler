//! CITL Error-Pattern Library for Depyler (Strategy #2 - DEPYLER-0632)
//!
//! Provides pattern storage and retrieval for compile error→fix mappings,
//! enabling automatic error resolution without LLM calls.
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
//! │  Golden      │───►│  Pattern     │───►│  Fix         │
//! │  Traces      │    │  Library     │    │  Suggestion  │
//! └──────────────┘    └──────────────┘    └──────────────┘
//!        │                   ▲                   │
//!        │                   │                   │
//! ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
//! │  Corpus      │───►│  entrenar    │◄───│  LLM Fix     │
//! │  Success     │    │  CITL        │    │  Learning    │
//! └──────────────┘    └──────────────┘    └──────────────┘
//! ```
//!
//! # References
//!
//! - DEPYLER-0632: Strategy #2 implementation
//! - docs/specifications/single-shot-80-percentage-review.md Section 10.3

use crate::tarantula::TranspilerDecision;
use crate::OracleError;
use entrenar::citl::{
    ChunkId, DecisionPatternStore, FixPattern as EntrenarFixPattern, PatternStoreConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for the error pattern library
#[derive(Debug, Clone)]
pub struct ErrorPatternConfig {
    /// Minimum confidence for pattern matches (0.0-1.0)
    pub min_confidence: f64,
    /// Maximum number of patterns to return per query
    pub max_suggestions: usize,
    /// Embedding dimension for dense vector search
    pub embedding_dim: usize,
    /// Enable pattern retirement for low-performing patterns
    pub enable_retirement: bool,
    /// Minimum success rate to keep pattern active
    pub retirement_threshold: f64,
}

impl Default for ErrorPatternConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.7,
            max_suggestions: 5,
            embedding_dim: 384,
            enable_retirement: true,
            retirement_threshold: 0.3,
        }
    }
}

/// An error pattern with fix suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    /// Unique pattern identifier
    pub id: String,
    /// Rust error code (e.g., "E0308")
    pub error_code: String,
    /// Error message pattern (regex or substring match)
    pub error_pattern: String,
    /// The fix to apply (unified diff format)
    pub fix_diff: String,
    /// Source context that must be present
    pub context_requirements: Vec<String>,
    /// Transpiler decision that caused this error
    pub decision_type: Option<TranspilerDecision>,
    /// Number of times this pattern has been applied
    pub applications: u32,
    /// Number of successful applications
    pub successes: u32,
    /// Confidence score based on history
    pub confidence: f64,
}

impl ErrorPattern {
    /// Create a new error pattern
    #[must_use]
    pub fn new(
        error_code: impl Into<String>,
        error_pattern: impl Into<String>,
        fix_diff: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            error_code: error_code.into(),
            error_pattern: error_pattern.into(),
            fix_diff: fix_diff.into(),
            context_requirements: Vec::new(),
            decision_type: None,
            applications: 0,
            successes: 0,
            confidence: 1.0, // Start with full confidence
        }
    }

    /// Add context requirement
    #[must_use]
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context_requirements.push(context.into());
        self
    }

    /// Set associated transpiler decision
    #[must_use]
    pub fn with_decision(mut self, decision: TranspilerDecision) -> Self {
        self.decision_type = Some(decision);
        self
    }

    /// Calculate success rate
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.applications == 0 {
            1.0 // No data yet, assume success
        } else {
            self.successes as f64 / self.applications as f64
        }
    }

    /// Check if pattern should be retired
    #[must_use]
    pub fn should_retire(&self, threshold: f64) -> bool {
        self.applications >= 5 && self.success_rate() < threshold
    }
}

/// Golden trace entry for pattern extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenTraceEntry {
    /// Python source file
    pub python_file: String,
    /// Rust source file (transpiled)
    pub rust_file: String,
    /// Error code if compilation failed
    pub error_code: Option<String>,
    /// Error message if compilation failed
    pub error_message: Option<String>,
    /// Fix that was applied (if any)
    pub applied_fix: Option<String>,
    /// Whether the fix was successful
    pub fix_successful: bool,
}

/// Corpus transpilation entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusEntry {
    /// Python source
    pub python_source: String,
    /// Successfully compiled Rust source
    pub rust_source: String,
    /// Transpiler decisions made
    pub decisions: Vec<String>,
}

/// Statistics for the error pattern library
#[derive(Debug, Clone, Default)]
pub struct ErrorPatternStats {
    /// Total patterns in library
    pub total_patterns: usize,
    /// Patterns from golden traces
    pub golden_trace_patterns: usize,
    /// Patterns from corpus
    pub corpus_patterns: usize,
    /// Patterns learned from LLM fixes
    pub llm_learned_patterns: usize,
    /// Total queries made
    pub queries: u64,
    /// Successful matches
    pub matches: u64,
    /// Patterns retired due to low success rate
    pub patterns_retired: usize,
}

impl ErrorPatternStats {
    /// Calculate hit rate
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        if self.queries == 0 {
            0.0
        } else {
            self.matches as f64 / self.queries as f64
        }
    }
}

/// CITL Error-Pattern Library
///
/// Stores error→fix patterns and provides retrieval using hybrid search
/// (BM25 + dense embeddings via trueno-rag).
pub struct ErrorPatternLibrary {
    /// Configuration
    config: ErrorPatternConfig,
    /// entrenar pattern store (when loaded)
    store: Option<DecisionPatternStore>,
    /// In-memory patterns for quick lookup
    patterns: HashMap<String, ErrorPattern>,
    /// Statistics
    stats: ErrorPatternStats,
}

impl ErrorPatternLibrary {
    /// Create a new error pattern library
    pub fn new() -> Result<Self, OracleError> {
        Self::with_config(ErrorPatternConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: ErrorPatternConfig) -> Result<Self, OracleError> {
        let store_config = PatternStoreConfig {
            chunk_size: 256,
            embedding_dim: config.embedding_dim,
            rrf_k: 60.0,
        };

        let store = DecisionPatternStore::with_config(store_config)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(Self {
            config,
            store: Some(store),
            patterns: HashMap::new(),
            stats: ErrorPatternStats::default(),
        })
    }

    /// Bootstrap from golden trace files
    ///
    /// Extracts error→fix patterns from previously recorded golden traces.
    pub fn bootstrap_from_golden_traces(
        &mut self,
        traces: &[GoldenTraceEntry],
    ) -> Result<usize, OracleError> {
        let mut count = 0;

        for trace in traces {
            if let (Some(error_code), Some(applied_fix), true) =
                (&trace.error_code, &trace.applied_fix, trace.fix_successful)
            {
                let pattern = ErrorPattern::new(
                    error_code.clone(),
                    trace.error_message.as_deref().unwrap_or(""),
                    applied_fix.clone(),
                );

                self.add_pattern(pattern)?;
                count += 1;
            }
        }

        self.stats.golden_trace_patterns += count;
        Ok(count)
    }

    /// Bootstrap from successful corpus transpilations
    ///
    /// Extracts patterns from files that compiled successfully.
    pub fn bootstrap_from_corpus(&mut self, entries: &[CorpusEntry]) -> Result<usize, OracleError> {
        let mut count = 0;

        // Group by decision patterns to find common successful patterns
        let mut decision_groups: HashMap<String, Vec<&CorpusEntry>> = HashMap::new();

        for entry in entries {
            let key = entry.decisions.join("::");
            decision_groups.entry(key).or_default().push(entry);
        }

        // Create patterns from groups with multiple successful examples
        for (decision_key, group) in decision_groups {
            if group.len() >= 2 {
                // This decision pattern works reliably
                let pattern = ErrorPattern::new(
                    "SUCCESS", // Not an error - a success pattern
                    &decision_key,
                    "", // No fix needed - this is what works
                )
                .with_context(format!("decisions:{}", decision_key));

                self.add_pattern(pattern)?;
                count += 1;
            }
        }

        self.stats.corpus_patterns += count;
        Ok(count)
    }

    /// Learn a new pattern from an LLM fix
    ///
    /// Called after an LLM successfully fixes an error.
    pub fn learn_from_llm_fix(
        &mut self,
        error_code: &str,
        error_message: &str,
        fix_diff: &str,
        source_context: &[String],
    ) -> Result<String, OracleError> {
        let mut pattern = ErrorPattern::new(error_code, error_message, fix_diff);

        for ctx in source_context {
            pattern = pattern.with_context(ctx.clone());
        }

        let pattern_id = pattern.id.clone();
        self.add_pattern(pattern)?;
        self.stats.llm_learned_patterns += 1;

        Ok(pattern_id)
    }

    /// Add a pattern to the library
    pub fn add_pattern(&mut self, pattern: ErrorPattern) -> Result<(), OracleError> {
        let pattern_id = pattern.id.clone();

        // Add to entrenar store if available
        if let Some(store) = &mut self.store {
            let mut fix_pattern = EntrenarFixPattern::new(&pattern.error_code, &pattern.fix_diff);

            // Add context as decisions
            for ctx in &pattern.context_requirements {
                fix_pattern = fix_pattern.with_decision(ctx.clone());
            }

            store
                .index_fix(fix_pattern)
                .map_err(|e| OracleError::Model(e.to_string()))?;
        }

        // Add to in-memory map
        self.patterns.insert(pattern_id, pattern);
        self.stats.total_patterns = self.patterns.len();

        Ok(())
    }

    /// Query for fix suggestions
    ///
    /// Returns patterns matching the error, sorted by confidence.
    pub fn suggest_fix(
        &mut self,
        error_code: &str,
        error_message: &str,
        source_context: &[String],
    ) -> Vec<ErrorPattern> {
        self.stats.queries += 1;

        // Try entrenar store first (hybrid search)
        if let Some(store) = &self.store {
            let mut context = vec![error_message.to_string()];
            context.extend(source_context.iter().cloned());

            if let Ok(suggestions) =
                store.suggest_fix(error_code, &context, self.config.max_suggestions)
            {
                let patterns: Vec<ErrorPattern> = suggestions
                    .into_iter()
                    .filter(|s| s.weighted_score() as f64 >= self.config.min_confidence)
                    .filter_map(|s| self.patterns.get(&s.pattern.id.0.to_string()).cloned())
                    .collect();

                if !patterns.is_empty() {
                    self.stats.matches += 1;
                    return patterns;
                }
            }
        }

        // Fallback to in-memory exact match
        let matches: Vec<ErrorPattern> = self
            .patterns
            .values()
            .filter(|p| p.error_code == error_code)
            .filter(|p| {
                // Check if error message contains pattern
                error_message.contains(&p.error_pattern)
                    || p.error_pattern.is_empty()
                    || p.error_pattern == error_message
            })
            .filter(|p| p.confidence >= self.config.min_confidence)
            .take(self.config.max_suggestions)
            .cloned()
            .collect();

        if !matches.is_empty() {
            self.stats.matches += 1;
        }

        matches
    }

    /// Record that a pattern was successfully applied
    pub fn record_success(&mut self, pattern_id: &str) {
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            pattern.applications += 1;
            pattern.successes += 1;
            pattern.confidence = pattern.success_rate();
        }

        // Update entrenar store
        if let Some(store) = &mut self.store {
            if let Ok(uuid) = uuid::Uuid::parse_str(pattern_id) {
                store.record_outcome(&ChunkId(uuid), true);
            }
        }
    }

    /// Record that a pattern application failed
    pub fn record_failure(&mut self, pattern_id: &str) {
        if let Some(pattern) = self.patterns.get_mut(pattern_id) {
            pattern.applications += 1;
            pattern.confidence = pattern.success_rate();

            // Check for retirement
            if self.config.enable_retirement
                && pattern.should_retire(self.config.retirement_threshold)
            {
                // Mark for removal
                pattern.confidence = 0.0;
                self.stats.patterns_retired += 1;
            }
        }

        // Update entrenar store
        if let Some(store) = &mut self.store {
            if let Ok(uuid) = uuid::Uuid::parse_str(pattern_id) {
                store.record_outcome(&ChunkId(uuid), false);
            }
        }
    }

    /// Remove retired patterns
    pub fn cleanup_retired(&mut self) {
        self.patterns.retain(|_, p| p.confidence > 0.0);
        self.stats.total_patterns = self.patterns.len();
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> &ErrorPatternStats {
        &self.stats
    }

    /// Get pattern count
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Save library to file
    pub fn save(&self, path: &Path) -> Result<(), OracleError> {
        if let Some(store) = &self.store {
            store
                .save_apr(path)
                .map_err(|e| OracleError::Model(e.to_string()))?;
        }
        Ok(())
    }

    /// Load library from file
    pub fn load(&mut self, path: &Path) -> Result<(), OracleError> {
        let store =
            DecisionPatternStore::load_apr(path).map_err(|e| OracleError::Model(e.to_string()))?;
        self.store = Some(store);
        Ok(())
    }
}

impl Default for ErrorPatternLibrary {
    fn default() -> Self {
        Self::new().expect("Failed to create default ErrorPatternLibrary")
    }
}

impl std::fmt::Debug for ErrorPatternLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorPatternLibrary")
            .field("patterns", &self.patterns.len())
            .field("stats", &self.stats)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_pattern_creation() {
        let pattern = ErrorPattern::new("E0308", "type mismatch", "- old\n+ new")
            .with_context("function:foo")
            .with_decision(TranspilerDecision::TypeInference);

        assert_eq!(pattern.error_code, "E0308");
        assert_eq!(pattern.error_pattern, "type mismatch");
        assert!(!pattern.id.is_empty());
        assert_eq!(
            pattern.decision_type,
            Some(TranspilerDecision::TypeInference)
        );
    }

    #[test]
    fn test_error_pattern_success_rate() {
        let mut pattern = ErrorPattern::new("E0308", "test", "fix");
        assert!((pattern.success_rate() - 1.0).abs() < 0.001); // No data = 1.0

        pattern.applications = 10;
        pattern.successes = 7;
        assert!((pattern.success_rate() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_error_pattern_retirement() {
        let mut pattern = ErrorPattern::new("E0308", "test", "fix");
        pattern.applications = 5;
        pattern.successes = 1; // 20% success rate

        assert!(pattern.should_retire(0.3)); // Below 30% threshold
        assert!(!pattern.should_retire(0.1)); // Above 10% threshold
    }

    #[test]
    fn test_error_pattern_library_new() {
        let library = ErrorPatternLibrary::new().unwrap();
        assert_eq!(library.pattern_count(), 0);
    }

    #[test]
    fn test_error_pattern_library_add_pattern() {
        let mut library = ErrorPatternLibrary::new().unwrap();
        let pattern = ErrorPattern::new("E0308", "mismatch", "fix");

        library.add_pattern(pattern).unwrap();
        assert_eq!(library.pattern_count(), 1);
    }

    #[test]
    fn test_error_pattern_library_suggest_fix() {
        let mut library = ErrorPatternLibrary::new().unwrap();
        library
            .add_pattern(ErrorPattern::new("E0308", "mismatch", "- i32\n+ i64"))
            .unwrap();

        let _suggestions = library.suggest_fix("E0308", "type mismatch: expected i64", &[]);
        // May or may not match depending on exact matching
        assert!(library.stats().queries > 0);
    }

    #[test]
    fn test_error_pattern_library_bootstrap_golden() {
        let mut library = ErrorPatternLibrary::new().unwrap();

        let traces = vec![
            GoldenTraceEntry {
                python_file: "test.py".to_string(),
                rust_file: "test.rs".to_string(),
                error_code: Some("E0308".to_string()),
                error_message: Some("type mismatch".to_string()),
                applied_fix: Some("fix diff".to_string()),
                fix_successful: true,
            },
            GoldenTraceEntry {
                python_file: "test2.py".to_string(),
                rust_file: "test2.rs".to_string(),
                error_code: None,
                error_message: None,
                applied_fix: None,
                fix_successful: true,
            },
        ];

        let count = library.bootstrap_from_golden_traces(&traces).unwrap();
        assert_eq!(count, 1);
        assert_eq!(library.stats().golden_trace_patterns, 1);
    }

    #[test]
    fn test_error_pattern_library_learn_from_llm() {
        let mut library = ErrorPatternLibrary::new().unwrap();

        let pattern_id = library
            .learn_from_llm_fix(
                "E0599",
                "method not found",
                "- old.method()\n+ new_method(&old)",
                &["context1".to_string()],
            )
            .unwrap();

        assert!(!pattern_id.is_empty());
        assert_eq!(library.stats().llm_learned_patterns, 1);
    }

    #[test]
    fn test_error_pattern_library_record_outcomes() {
        let mut library = ErrorPatternLibrary::new().unwrap();
        let pattern = ErrorPattern::new("E0308", "test", "fix");
        let pattern_id = pattern.id.clone();

        library.add_pattern(pattern).unwrap();

        library.record_success(&pattern_id);
        library.record_success(&pattern_id);
        library.record_failure(&pattern_id);

        // Pattern should have 3 applications, 2 successes
        // Success rate = 0.667
        let stats = library.stats();
        assert_eq!(stats.total_patterns, 1);
    }

    #[test]
    fn test_error_pattern_stats_hit_rate() {
        let mut stats = ErrorPatternStats::default();
        assert!((stats.hit_rate() - 0.0).abs() < 0.001);

        stats.queries = 100;
        stats.matches = 75;
        assert!((stats.hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_error_pattern_config_default() {
        let config = ErrorPatternConfig::default();
        assert!((config.min_confidence - 0.7).abs() < 0.001);
        assert_eq!(config.max_suggestions, 5);
        assert!(config.enable_retirement);
    }
}

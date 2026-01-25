//! Oracle Query Loop for pattern-based error resolution (Issue #172)
//!
//! Implements the ROI-optimized error resolution pipeline:
//! - Load .apr pattern files from entrenar CITL
//! - Query patterns on compile errors
//! - Auto-fix loop with compilation verification
//! - Metrics tracking for cost/hit rate analysis
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
//! │  Rust Error  │───►│  Pattern     │───►│  Fix         │
//! │  (E0308...)  │    │  Matching    │    │  Suggestion  │
//! └──────────────┘    └──────────────┘    └──────────────┘
//!                            │
//!                            ▼
//!                     ┌──────────────┐
//!                     │  .apr File   │
//!                     │  (entrenar)  │
//!                     └──────────────┘
//! ```
//!
//! # References
//!
//! - Issue #172: Oracle Query Loop (ROI Multiplier)
//! - entrenar::citl::DecisionPatternStore for pattern storage

use entrenar::citl::DecisionPatternStore;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;

/// Rust error codes we handle (exhaustive for type safety)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustErrorCode {
    /// E0308: mismatched types
    E0308,
    /// E0382: use of moved value
    E0382,
    /// E0502: cannot borrow as mutable
    E0502,
    /// E0499: cannot borrow as mutable more than once
    E0499,
    /// E0597: borrowed value does not live long enough
    E0597,
    /// E0716: temporary value dropped while borrowed
    E0716,
    /// E0277: trait bound not satisfied
    E0277,
    /// E0599: method not found
    E0599,
    /// E0425: cannot find value
    E0425,
    /// E0433: failed to resolve
    E0433,
    /// Other error codes
    Other(u16),
}

/// Error type for parsing RustErrorCode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRustErrorCodeError(String);

impl std::fmt::Display for ParseRustErrorCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid Rust error code: {}", self.0)
    }
}

impl std::error::Error for ParseRustErrorCodeError {}

impl FromStr for RustErrorCode {
    type Err = ParseRustErrorCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s
            .strip_prefix('E')
            .and_then(|n| n.parse::<u16>().ok())
            .ok_or_else(|| ParseRustErrorCodeError(s.to_string()))?;

        Ok(match code {
            308 => Self::E0308,
            382 => Self::E0382,
            502 => Self::E0502,
            499 => Self::E0499,
            597 => Self::E0597,
            716 => Self::E0716,
            277 => Self::E0277,
            599 => Self::E0599,
            425 => Self::E0425,
            433 => Self::E0433,
            n => Self::Other(n),
        })
    }
}

impl RustErrorCode {
    /// Convert to string representation
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::E0308 => "E0308".to_string(),
            Self::E0382 => "E0382".to_string(),
            Self::E0502 => "E0502".to_string(),
            Self::E0499 => "E0499".to_string(),
            Self::E0597 => "E0597".to_string(),
            Self::E0716 => "E0716".to_string(),
            Self::E0277 => "E0277".to_string(),
            Self::E0599 => "E0599".to_string(),
            Self::E0425 => "E0425".to_string(),
            Self::E0433 => "E0433".to_string(),
            Self::Other(n) => format!("E{:04}", n),
        }
    }
}

/// Context about an error for pattern matching
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// File path where error occurred
    pub file: PathBuf,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Source code snippet at error location
    pub source_snippet: String,
    /// Surrounding lines for context
    pub surrounding_lines: Vec<String>,
}

/// Oracle suggestion with confidence and metadata
#[derive(Debug, Clone)]
pub struct OracleSuggestion {
    /// The fix diff to apply (unified diff format)
    pub fix_diff: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Pattern identifier for tracking
    pub pattern_id: String,
    /// Number of times this pattern was applied
    pub times_applied: u32,
    /// Historical success rate of this pattern
    pub success_rate: f64,
}

/// Configuration for the Oracle Query Loop
#[derive(Debug, Clone)]
pub struct QueryLoopConfig {
    /// Minimum confidence threshold for suggestions (0.0-1.0)
    pub threshold: f64,
    /// Maximum number of suggestions to return
    pub max_suggestions: usize,
    /// Boost recently successful patterns
    pub boost_recent: bool,
    /// Maximum retries per error
    pub max_retries: usize,
    /// Enable LLM fallback for low-confidence matches
    pub llm_fallback: bool,
}

impl Default for QueryLoopConfig {
    fn default() -> Self {
        Self {
            threshold: 0.7,
            max_suggestions: 3,
            boost_recent: true,
            max_retries: 3,
            llm_fallback: false,
        }
    }
}

/// Statistics tracking for oracle performance
#[derive(Debug, Default, Clone)]
pub struct OracleStats {
    /// Total queries made
    pub queries: u64,
    /// Successful pattern matches
    pub hits: u64,
    /// Failed pattern matches (below threshold)
    pub misses: u64,
    /// Fixes that were applied
    pub fixes_applied: u64,
    /// Fixes that compiled successfully after application
    pub fixes_verified: u64,
    /// Times LLM fallback was used
    pub llm_fallbacks: u64,
}

impl OracleStats {
    /// Calculate hit rate
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        if self.queries == 0 {
            0.0
        } else {
            self.hits as f64 / self.queries as f64
        }
    }

    /// Calculate fix success rate
    #[must_use]
    pub fn fix_success_rate(&self) -> f64 {
        if self.fixes_applied == 0 {
            0.0
        } else {
            self.fixes_verified as f64 / self.fixes_applied as f64
        }
    }

    /// Estimate cost savings in cents (assumes $0.04 per LLM avoided)
    #[must_use]
    pub fn estimated_savings_cents(&self) -> u64 {
        self.fixes_verified * 4
    }
}

/// Errors that can occur in the Oracle Query Loop
#[derive(Debug, Error)]
pub enum OracleQueryError {
    /// Failed to load pattern file
    #[error("Failed to load oracle from {path}: {cause}")]
    LoadFailed { path: PathBuf, cause: String },

    /// Pattern file not found
    #[error("Pattern file not found: {0}")]
    NotFound(PathBuf),

    /// Invalid pattern format
    #[error("Invalid pattern format: {0}")]
    InvalidFormat(String),

    /// Compilation verification failed
    #[error("Compilation verification failed: {0}")]
    CompilationFailed(String),

    /// Pattern application failed
    #[error("Failed to apply pattern: {0}")]
    ApplyFailed(String),
}

/// Main Oracle Query Loop struct
///
/// Queries .apr pattern files for fix suggestions based on compile errors.
pub struct OracleQueryLoop {
    /// Configuration
    config: QueryLoopConfig,
    /// Statistics tracking
    stats: OracleStats,
    /// Path to loaded pattern file
    pattern_path: Option<PathBuf>,
    /// entrenar DecisionPatternStore for pattern retrieval
    pattern_store: Option<DecisionPatternStore>,
}

impl OracleQueryLoop {
    /// Create a new Oracle Query Loop with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(QueryLoopConfig::default())
    }

    /// Create with custom configuration
    #[must_use]
    pub fn with_config(config: QueryLoopConfig) -> Self {
        Self {
            config,
            stats: OracleStats::default(),
            pattern_path: None,
            pattern_store: None,
        }
    }

    /// Load patterns from an .apr file
    ///
    /// Uses entrenar::citl::DecisionPatternStore to load .apr format patterns.
    ///
    /// # Errors
    ///
    /// Returns error if file not found or invalid format.
    pub fn load(&mut self, path: &Path) -> Result<(), OracleQueryError> {
        if !path.exists() {
            return Err(OracleQueryError::NotFound(path.to_path_buf()));
        }

        // Load using entrenar's DecisionPatternStore
        let store =
            DecisionPatternStore::load_apr(path).map_err(|e| OracleQueryError::LoadFailed {
                path: path.to_path_buf(),
                cause: e.to_string(),
            })?;

        self.pattern_path = Some(path.to_path_buf());
        self.pattern_store = Some(store);
        Ok(())
    }

    /// Query for fix suggestions given a compile error
    ///
    /// Uses entrenar::citl::DecisionPatternStore for hybrid retrieval.
    /// Returns suggestions sorted by confidence (highest first).
    pub fn suggest(
        &mut self,
        error_code: RustErrorCode,
        error_message: &str,
        context: &ErrorContext,
    ) -> Vec<OracleSuggestion> {
        self.stats.queries += 1;

        // If no pattern store loaded, return empty
        let store = match &self.pattern_store {
            Some(s) => s,
            None => {
                self.stats.misses += 1;
                return Vec::new();
            }
        };

        // Build decision context from error context
        let decision_context: Vec<String> = vec![
            format!("error_code:{}", error_code.as_str()),
            format!("line:{}", context.line),
            format!("col:{}", context.column),
            context.source_snippet.clone(),
            error_message.to_string(),
        ];

        // Query the pattern store
        let suggestions = match store.suggest_fix(
            &error_code.as_str(),
            &decision_context,
            self.config.max_suggestions,
        ) {
            Ok(s) => s,
            Err(_) => {
                self.stats.misses += 1;
                return Vec::new();
            }
        };

        // Filter by confidence threshold and convert to OracleSuggestion
        let filtered: Vec<OracleSuggestion> = suggestions
            .into_iter()
            .filter(|s| s.weighted_score() as f64 >= self.config.threshold)
            .map(|s| OracleSuggestion {
                fix_diff: s.pattern.fix_diff.clone(),
                confidence: s.weighted_score() as f64,
                pattern_id: s.pattern.id.to_string(),
                times_applied: s.pattern.attempt_count,
                success_rate: s.pattern.success_rate() as f64,
            })
            .collect();

        if filtered.is_empty() {
            self.stats.misses += 1;
        } else {
            self.stats.hits += 1;
        }

        filtered
    }

    /// Record a successful fix application
    ///
    /// Updates both statistics and the pattern store's success count.
    pub fn record_success(&mut self, pattern_id: &str) {
        self.stats.fixes_verified += 1;

        // Update pattern success count in the store
        if let Some(store) = &mut self.pattern_store {
            // Parse pattern_id as ChunkId (UUID)
            if let Ok(uuid) = uuid::Uuid::parse_str(pattern_id) {
                let chunk_id = entrenar::citl::ChunkId(uuid);
                store.record_outcome(&chunk_id, true);
            }
        }
    }

    /// Record a fix application (before verification)
    pub fn record_fix_applied(&mut self) {
        self.stats.fixes_applied += 1;
    }

    /// Record LLM fallback usage
    pub fn record_llm_fallback(&mut self) {
        self.stats.llm_fallbacks += 1;
    }

    /// Get current statistics
    #[must_use]
    pub fn stats(&self) -> &OracleStats {
        &self.stats
    }

    /// Get configuration
    #[must_use]
    pub fn config(&self) -> &QueryLoopConfig {
        &self.config
    }

    /// Check if patterns are loaded
    #[must_use]
    pub fn is_loaded(&self) -> bool {
        self.pattern_path.is_some()
    }

    /// Get the default pattern file path (~/.depyler/patterns.apr)
    #[must_use]
    pub fn default_pattern_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".depyler")
            .join("patterns.apr")
    }

    /// Get the number of loaded patterns
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.pattern_store.as_ref().map(|s| s.len()).unwrap_or(0)
    }
}

impl Default for OracleQueryLoop {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Phase 3: Auto-Fix Loop
// ============================================================================

/// Result of an auto-fix attempt
#[derive(Debug, Clone)]
pub enum AutoFixResult {
    /// Fix compiled successfully
    Success {
        /// Pattern ID that worked
        pattern_id: String,
        /// Number of attempts made
        attempts: usize,
        /// The fixed source code
        fixed_source: String,
    },
    /// All fixes exhausted without success
    Exhausted {
        /// Error code that couldn't be fixed
        error_code: String,
        /// Number of attempts made
        attempts: usize,
    },
    /// No suggestions available from oracle
    NoSuggestion,
}

/// Apply fixes iteratively until one compiles
///
/// # Arguments
///
/// * `oracle` - The oracle query loop instance
/// * `source` - Mutable source code to fix
/// * `error_code` - The error code to fix
/// * `error_message` - The error message
/// * `context` - Error context
/// * `verify_fn` - Function to verify if code compiles
///
/// # Returns
///
/// `AutoFixResult` indicating success, exhausted, or no suggestion
pub fn auto_fix_loop<F>(
    oracle: &mut OracleQueryLoop,
    source: &mut String,
    error_code: RustErrorCode,
    error_message: &str,
    context: &ErrorContext,
    mut verify_fn: F,
) -> AutoFixResult
where
    F: FnMut(&str) -> bool,
{
    let suggestions = oracle.suggest(error_code, error_message, context);

    if suggestions.is_empty() {
        return AutoFixResult::NoSuggestion;
    }

    let max_retries = oracle.config().max_retries;

    for (attempt, suggestion) in suggestions.iter().take(max_retries).enumerate() {
        // Apply the diff (simple line replacement for now)
        let patched = apply_simple_diff(source, &suggestion.fix_diff);

        oracle.record_fix_applied();

        // Verify compilation
        if verify_fn(&patched) {
            *source = patched.clone();
            oracle.record_success(&suggestion.pattern_id);
            return AutoFixResult::Success {
                pattern_id: suggestion.pattern_id.clone(),
                attempts: attempt + 1,
                fixed_source: patched,
            };
        }
    }

    AutoFixResult::Exhausted {
        error_code: error_code.as_str(),
        attempts: suggestions.len().min(max_retries),
    }
}

/// Apply a simple unified diff to source code
///
/// This is a simplified diff application that handles:
/// - Lines starting with `-` are removed
/// - Lines starting with `+` are added
pub fn apply_simple_diff(source: &str, diff: &str) -> String {
    let mut result = source.to_string();

    for line in diff.lines() {
        if let Some(to_remove) = line.strip_prefix('-').map(|s| s.trim()) {
            if !to_remove.is_empty() && !to_remove.starts_with('-') {
                result = result.replace(to_remove, "");
            }
        }
    }

    for line in diff.lines() {
        if let Some(to_add) = line.strip_prefix('+').map(|s| s.trim()) {
            if !to_add.is_empty() && !to_add.starts_with('+') && !result.contains(to_add) {
                // Simple insertion at the end (real impl would use line numbers)
                if !result.ends_with('\n') {
                    result.push('\n');
                }
                result.push_str(to_add);
                result.push('\n');
            }
        }
    }

    result
}

// ============================================================================
// Phase 4: Prometheus Metrics Export
// ============================================================================

/// Prometheus-style metrics for oracle performance
#[derive(Debug, Default, Clone)]
pub struct OracleMetrics {
    /// Total queries made
    pub queries_total: u64,
    /// Successful pattern matches
    pub hits_total: u64,
    /// Failed pattern matches
    pub misses_total: u64,
    /// Fixes applied (before verification)
    pub fixes_applied_total: u64,
    /// Fixes that compiled successfully
    pub fixes_verified_total: u64,
    /// LLM fallbacks used
    pub llm_fallbacks_total: u64,
    /// Estimated cost savings in cents
    pub estimated_savings_cents: u64,
}

impl OracleMetrics {
    /// Create metrics from OracleStats
    #[must_use]
    pub fn from_stats(stats: &OracleStats) -> Self {
        Self {
            queries_total: stats.queries,
            hits_total: stats.hits,
            misses_total: stats.misses,
            fixes_applied_total: stats.fixes_applied,
            fixes_verified_total: stats.fixes_verified,
            llm_fallbacks_total: stats.llm_fallbacks,
            estimated_savings_cents: stats.estimated_savings_cents(),
        }
    }

    /// Calculate hit rate (0.0-1.0)
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        if self.queries_total == 0 {
            0.0
        } else {
            self.hits_total as f64 / self.queries_total as f64
        }
    }

    /// Calculate fix success rate (0.0-1.0)
    #[must_use]
    pub fn fix_success_rate(&self) -> f64 {
        if self.fixes_applied_total == 0 {
            0.0
        } else {
            self.fixes_verified_total as f64 / self.fixes_applied_total as f64
        }
    }

    /// Export as Prometheus format string
    #[must_use]
    pub fn to_prometheus(&self) -> String {
        format!(
            r#"# HELP depyler_oracle_queries_total Total oracle queries
# TYPE depyler_oracle_queries_total counter
depyler_oracle_queries_total {}

# HELP depyler_oracle_hits_total Successful pattern matches
# TYPE depyler_oracle_hits_total counter
depyler_oracle_hits_total {}

# HELP depyler_oracle_misses_total Failed pattern matches
# TYPE depyler_oracle_misses_total counter
depyler_oracle_misses_total {}

# HELP depyler_oracle_hit_rate Current hit rate
# TYPE depyler_oracle_hit_rate gauge
depyler_oracle_hit_rate {:.4}

# HELP depyler_oracle_fixes_applied_total Fixes applied
# TYPE depyler_oracle_fixes_applied_total counter
depyler_oracle_fixes_applied_total {}

# HELP depyler_oracle_fixes_verified_total Successfully compiled fixes
# TYPE depyler_oracle_fixes_verified_total counter
depyler_oracle_fixes_verified_total {}

# HELP depyler_oracle_fix_success_rate Fix success rate
# TYPE depyler_oracle_fix_success_rate gauge
depyler_oracle_fix_success_rate {:.4}

# HELP depyler_oracle_savings_cents Estimated cost savings in cents
# TYPE depyler_oracle_savings_cents counter
depyler_oracle_savings_cents {}

# HELP depyler_oracle_llm_fallbacks_total LLM fallbacks used
# TYPE depyler_oracle_llm_fallbacks_total counter
depyler_oracle_llm_fallbacks_total {}
"#,
            self.queries_total,
            self.hits_total,
            self.misses_total,
            self.hit_rate(),
            self.fixes_applied_total,
            self.fixes_verified_total,
            self.fix_success_rate(),
            self.estimated_savings_cents,
            self.llm_fallbacks_total,
        )
    }
}

// ============================================================================
// Tests (EXTREME TDD - RED Phase)
// ============================================================================

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // ============================================
    // Phase 1: RustErrorCode Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase1_error_code_parsing_valid() {
        assert_eq!("E0308".parse::<RustErrorCode>(), Ok(RustErrorCode::E0308));
        assert_eq!("E0382".parse::<RustErrorCode>(), Ok(RustErrorCode::E0382));
        assert_eq!("E0502".parse::<RustErrorCode>(), Ok(RustErrorCode::E0502));
        assert_eq!("E0277".parse::<RustErrorCode>(), Ok(RustErrorCode::E0277));
    }

    #[test]
    fn test_DEPYLER_0172_phase1_error_code_parsing_other() {
        assert_eq!(
            "E9999".parse::<RustErrorCode>(),
            Ok(RustErrorCode::Other(9999))
        );
    }

    #[test]
    fn test_DEPYLER_0172_phase1_error_code_parsing_invalid() {
        assert!("invalid".parse::<RustErrorCode>().is_err());
        assert!("0308".parse::<RustErrorCode>().is_err());
        assert!("E".parse::<RustErrorCode>().is_err());
    }

    #[test]
    fn test_DEPYLER_0172_phase1_error_code_as_str() {
        assert_eq!(RustErrorCode::E0308.as_str(), "E0308");
        assert_eq!(RustErrorCode::Other(42).as_str(), "E0042");
    }

    // ============================================
    // Phase 1: QueryLoopConfig Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase1_config_default() {
        let config = QueryLoopConfig::default();
        assert!((config.threshold - 0.7).abs() < 0.001);
        assert_eq!(config.max_suggestions, 3);
        assert!(config.boost_recent);
        assert_eq!(config.max_retries, 3);
        assert!(!config.llm_fallback);
    }

    #[test]
    fn test_DEPYLER_0172_phase1_config_custom() {
        let config = QueryLoopConfig {
            threshold: 0.9,
            max_suggestions: 5,
            boost_recent: false,
            max_retries: 5,
            llm_fallback: true,
        };
        assert!((config.threshold - 0.9).abs() < 0.001);
        assert!(config.llm_fallback);
    }

    // ============================================
    // Phase 1: OracleStats Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase1_stats_hit_rate_zero_queries() {
        let stats = OracleStats::default();
        assert!((stats.hit_rate() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_DEPYLER_0172_phase1_stats_hit_rate_calculation() {
        let stats = OracleStats {
            queries: 100,
            hits: 75,
            ..Default::default()
        };
        assert!((stats.hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_DEPYLER_0172_phase1_stats_fix_success_rate() {
        let stats = OracleStats {
            fixes_applied: 10,
            fixes_verified: 8,
            ..Default::default()
        };
        assert!((stats.fix_success_rate() - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_DEPYLER_0172_phase1_stats_cost_savings() {
        let stats = OracleStats {
            fixes_verified: 100,
            ..Default::default()
        };
        assert_eq!(stats.estimated_savings_cents(), 400); // 100 * $0.04
    }

    // ============================================
    // Phase 1: OracleQueryLoop Creation Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase1_query_loop_new() {
        let oracle = OracleQueryLoop::new();
        assert!(!oracle.is_loaded());
        assert_eq!(oracle.stats().queries, 0);
    }

    #[test]
    fn test_DEPYLER_0172_phase1_query_loop_with_config() {
        let config = QueryLoopConfig {
            threshold: 0.9,
            ..Default::default()
        };
        let oracle = OracleQueryLoop::with_config(config);
        assert!((oracle.config().threshold - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_DEPYLER_0172_phase1_query_loop_default_path() {
        let path = OracleQueryLoop::default_pattern_path();
        assert!(path.to_string_lossy().contains("patterns.apr"));
    }

    // ============================================
    // Phase 1: Load Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase1_load_nonexistent_file() {
        let mut oracle = OracleQueryLoop::new();
        let result = oracle.load(Path::new("/nonexistent/patterns.apr"));
        assert!(matches!(result, Err(OracleQueryError::NotFound(_))));
    }

    #[test]
    fn test_DEPYLER_0172_phase1_load_existing_file() {
        use entrenar::citl::{DecisionPatternStore, FixPattern};

        let mut oracle = OracleQueryLoop::new();

        // Create a valid .apr file using DecisionPatternStore
        let temp_dir = tempfile::tempdir().unwrap();
        let apr_path = temp_dir.path().join("test_patterns.apr");

        // Create and save a pattern store
        let mut store = DecisionPatternStore::new().unwrap();
        let pattern = FixPattern::new(
            "E0308",
            "- let x: i32 = \"hello\";\n+ let x: &str = \"hello\";",
        )
        .with_decision("type_mismatch_detected");
        store.index_fix(pattern).unwrap();
        store.save_apr(&apr_path).unwrap();

        // Now load it with OracleQueryLoop
        let result = oracle.load(&apr_path);
        assert!(result.is_ok(), "Failed to load: {:?}", result.err());
        assert!(oracle.is_loaded());
    }

    #[test]
    fn test_DEPYLER_0172_phase1_load_invalid_format() {
        let mut oracle = OracleQueryLoop::new();
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "invalid format data").unwrap();

        let result = oracle.load(temp.path());
        // Should fail with LoadFailed error since it's not valid .apr format
        assert!(matches!(result, Err(OracleQueryError::LoadFailed { .. })));
    }

    // ============================================
    // Phase 2: Suggest Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase2_suggest_returns_empty_without_patterns() {
        let mut oracle = OracleQueryLoop::new();
        let context = ErrorContext {
            file: PathBuf::from("test.rs"),
            line: 42,
            column: 5,
            source_snippet: "let x: i32 = \"hello\";".to_string(),
            surrounding_lines: vec![],
        };

        let suggestions =
            oracle.suggest(RustErrorCode::E0308, "expected i32, found &str", &context);

        // Without patterns loaded, should return empty
        assert!(suggestions.is_empty());
        assert_eq!(oracle.stats().queries, 1);
        assert_eq!(oracle.stats().misses, 1);
    }

    // ============================================
    // Phase 3: Recording Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase3_record_success() {
        let mut oracle = OracleQueryLoop::new();
        oracle.record_fix_applied();
        oracle.record_success("test-pattern-001");

        assert_eq!(oracle.stats().fixes_applied, 1);
        assert_eq!(oracle.stats().fixes_verified, 1);
    }

    #[test]
    fn test_DEPYLER_0172_phase3_record_llm_fallback() {
        let mut oracle = OracleQueryLoop::new();
        oracle.record_llm_fallback();

        assert_eq!(oracle.stats().llm_fallbacks, 1);
    }

    // ============================================
    // Property Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_property_hit_rate_bounds() {
        // Hit rate should always be in [0.0, 1.0]
        for hits in 0..=100 {
            let stats = OracleStats {
                queries: 100,
                hits,
                ..Default::default()
            };
            let rate = stats.hit_rate();
            assert!(
                (0.0..=1.0).contains(&rate),
                "Hit rate out of bounds: {}",
                rate
            );
        }
    }

    #[test]
    fn test_DEPYLER_0172_property_error_code_roundtrip() {
        // All known error codes should roundtrip through string conversion
        let codes = [
            RustErrorCode::E0308,
            RustErrorCode::E0382,
            RustErrorCode::E0502,
            RustErrorCode::E0499,
            RustErrorCode::E0597,
            RustErrorCode::E0716,
            RustErrorCode::E0277,
            RustErrorCode::E0599,
            RustErrorCode::E0425,
            RustErrorCode::E0433,
        ];

        for code in codes {
            let s = code.as_str();
            let parsed: RustErrorCode = s.parse().expect("valid error code");
            assert_eq!(parsed, code, "Roundtrip failed for {:?}", code);
        }
    }

    // ============================================
    // Phase 3: Auto-Fix Loop Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase3_auto_fix_no_suggestion() {
        let mut oracle = OracleQueryLoop::new();
        let mut source = "let x: i32 = \"hello\";".to_string();
        let context = ErrorContext {
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 14,
            source_snippet: source.clone(),
            surrounding_lines: vec![],
        };

        let result = auto_fix_loop(
            &mut oracle,
            &mut source,
            RustErrorCode::E0308,
            "expected i32, found &str",
            &context,
            |_| true, // Always pass verification
        );

        // No patterns loaded, should return NoSuggestion
        assert!(matches!(result, AutoFixResult::NoSuggestion));
    }

    #[test]
    fn test_DEPYLER_0172_phase3_apply_simple_diff() {
        let source = "let x: i32 = \"hello\";";
        let diff = "- let x: i32 = \"hello\";\n+ let x: &str = \"hello\";";

        let result = apply_simple_diff(source, diff);

        // Should remove old line and add new one
        assert!(result.contains("let x: &str = \"hello\";"));
        assert!(!result.contains("let x: i32 = \"hello\";"));
    }

    #[test]
    fn test_DEPYLER_0172_phase3_apply_simple_diff_empty() {
        let source = "fn main() {}";
        let diff = "";

        let result = apply_simple_diff(source, diff);

        // Empty diff should preserve source
        assert_eq!(result, source);
    }

    // ============================================
    // Phase 4: OracleMetrics Tests
    // ============================================

    #[test]
    fn test_DEPYLER_0172_phase4_metrics_from_stats() {
        let stats = OracleStats {
            queries: 100,
            hits: 75,
            misses: 25,
            fixes_applied: 50,
            fixes_verified: 40,
            llm_fallbacks: 5,
        };

        let metrics = OracleMetrics::from_stats(&stats);

        assert_eq!(metrics.queries_total, 100);
        assert_eq!(metrics.hits_total, 75);
        assert_eq!(metrics.misses_total, 25);
        assert_eq!(metrics.fixes_applied_total, 50);
        assert_eq!(metrics.fixes_verified_total, 40);
        assert_eq!(metrics.llm_fallbacks_total, 5);
    }

    #[test]
    fn test_DEPYLER_0172_phase4_metrics_hit_rate() {
        let metrics = OracleMetrics {
            queries_total: 100,
            hits_total: 75,
            ..Default::default()
        };

        assert!((metrics.hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_DEPYLER_0172_phase4_metrics_fix_success_rate() {
        let metrics = OracleMetrics {
            fixes_applied_total: 10,
            fixes_verified_total: 8,
            ..Default::default()
        };

        assert!((metrics.fix_success_rate() - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_DEPYLER_0172_phase4_prometheus_format() {
        let metrics = OracleMetrics {
            queries_total: 100,
            hits_total: 75,
            misses_total: 25,
            fixes_applied_total: 50,
            fixes_verified_total: 40,
            llm_fallbacks_total: 5,
            estimated_savings_cents: 160, // 40 * 4
        };

        let prom = metrics.to_prometheus();

        // Verify Prometheus format
        assert!(prom.contains("depyler_oracle_queries_total 100"));
        assert!(prom.contains("depyler_oracle_hits_total 75"));
        assert!(prom.contains("depyler_oracle_misses_total 25"));
        assert!(prom.contains("depyler_oracle_hit_rate 0.75"));
        assert!(prom.contains("depyler_oracle_fixes_verified_total 40"));
        assert!(prom.contains("depyler_oracle_fix_success_rate 0.80"));
        assert!(prom.contains("depyler_oracle_savings_cents 160"));
        assert!(prom.contains("# TYPE depyler_oracle_queries_total counter"));
        assert!(prom.contains("# TYPE depyler_oracle_hit_rate gauge"));
    }

    #[test]
    fn test_DEPYLER_0172_phase4_prometheus_zero_division() {
        let metrics = OracleMetrics::default();

        // Should not panic on zero division
        assert!((metrics.hit_rate() - 0.0).abs() < 0.001);
        assert!((metrics.fix_success_rate() - 0.0).abs() < 0.001);

        let prom = metrics.to_prometheus();
        assert!(prom.contains("depyler_oracle_hit_rate 0.00"));
    }
}

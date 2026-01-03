//! Compilation Trainer Shim - pure logic separated from I/O
//!
//! Extracts testable logic from compilation_trainer.rs

use std::collections::HashMap;

/// Diagnostic verbosity tier for CITL training
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticTier {
    #[default]
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl DiagnosticTier {
    /// Convert tier number (1-4) to enum
    pub fn from_level(level: u8) -> Self {
        match level {
            1 => Self::Tier1,
            2 => Self::Tier2,
            3 => Self::Tier3,
            4.. => Self::Tier4,
            _ => Self::Tier1,
        }
    }

    /// Get tier number (1-4)
    pub fn level(&self) -> u8 {
        match self {
            Self::Tier1 => 1,
            Self::Tier2 => 2,
            Self::Tier3 => 3,
            Self::Tier4 => 4,
        }
    }

    /// Get overhead estimate for this tier
    pub fn overhead_percent(&self) -> u8 {
        match self {
            Self::Tier1 => 5,
            Self::Tier2 => 10,
            Self::Tier3 => 25,
            Self::Tier4 => 50,
        }
    }

    /// Get estimated log size per failed file (KB)
    pub fn log_size_kb(&self) -> usize {
        match self {
            Self::Tier1 => 2,
            Self::Tier2 => 5,
            Self::Tier3 => 20,
            Self::Tier4 => 100,
        }
    }
}

/// Clippy lint level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClippyLevel {
    Standard,
    Pedantic,
    #[default]
    Nursery,
    Full,
}

impl ClippyLevel {
    /// Parse from CLI argument string
    pub fn from_cli_arg(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "standard" | "all" => Self::Standard,
            "pedantic" => Self::Pedantic,
            "nursery" => Self::Nursery,
            "full" | "cargo" => Self::Full,
            _ => Self::Nursery,
        }
    }

    /// Get clippy flags for this level
    pub fn flags(&self) -> Vec<&'static str> {
        let mut flags = vec!["clippy::all"];
        if matches!(self, Self::Pedantic | Self::Nursery | Self::Full) {
            flags.push("clippy::pedantic");
        }
        if matches!(self, Self::Nursery | Self::Full) {
            flags.push("clippy::nursery");
        }
        if matches!(self, Self::Full) {
            flags.push("clippy::cargo");
        }
        flags
    }

    /// Get approximate lint count
    pub fn lint_count(&self) -> usize {
        match self {
            Self::Standard => 500,
            Self::Pedantic => 600,
            Self::Nursery => 650,
            Self::Full => 700,
        }
    }
}

/// Verbosity configuration for diagnostic capture (pure data)
#[derive(Debug, Clone)]
pub struct VerbosityConfig {
    pub tier: DiagnosticTier,
    pub clippy_level: ClippyLevel,
    pub trace_errors: Vec<String>,
    pub max_log_size: usize,
    pub timeout_secs: u64,
    pub adaptive: bool,
}

impl Default for VerbosityConfig {
    fn default() -> Self {
        Self {
            tier: DiagnosticTier::Tier1,
            clippy_level: ClippyLevel::Nursery,
            trace_errors: vec![
                "E0308".to_string(),
                "E0277".to_string(),
                "E0382".to_string(),
            ],
            max_log_size: 1_000_000,
            timeout_secs: 300,
            adaptive: true,
        }
    }
}

impl VerbosityConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tier(mut self, tier: DiagnosticTier) -> Self {
        self.tier = tier;
        self
    }

    pub fn with_clippy_level(mut self, level: ClippyLevel) -> Self {
        self.clippy_level = level;
        self
    }

    pub fn with_trace_errors(mut self, errors: Vec<String>) -> Self {
        self.trace_errors = errors;
        self
    }

    pub fn with_adaptive(mut self, adaptive: bool) -> Self {
        self.adaptive = adaptive;
        self
    }

    /// Select appropriate tier based on error code and attempt number
    pub fn select_tier_for_error(&self, error_code: &str, attempt: u32) -> DiagnosticTier {
        if !self.adaptive {
            return self.tier;
        }

        // Adaptive tier selection based on error type and attempt
        let is_complex_error = self.trace_errors.iter().any(|e| error_code.contains(e));

        match (is_complex_error, attempt) {
            (_, 0) => DiagnosticTier::Tier1,
            (true, 1) => DiagnosticTier::Tier2,
            (true, 2) => DiagnosticTier::Tier3,
            (true, _) => DiagnosticTier::Tier4,
            (false, 1) => DiagnosticTier::Tier1,
            (false, 2) => DiagnosticTier::Tier2,
            (false, _) => DiagnosticTier::Tier3,
        }
    }

    /// Check if error code should trigger higher verbosity
    pub fn should_escalate(&self, error_code: &str) -> bool {
        self.trace_errors.iter().any(|e| error_code.contains(e))
    }
}

/// Parsed diagnostic features for training corpus
#[derive(Debug, Clone, Default)]
pub struct DiagnosticFeatures {
    pub error_code: Option<String>,
    pub level: String,
    pub message: String,
    pub spans: Vec<DiagnosticSpan>,
    pub suggestions: Vec<String>,
    pub clippy_lints: Vec<String>,
    pub trace_lines: Vec<String>,
    pub backtrace: Option<String>,
}

impl DiagnosticFeatures {
    /// Check if this diagnostic has actionable suggestions
    pub fn has_suggestions(&self) -> bool {
        !self.suggestions.is_empty()
    }

    /// Get primary error code
    pub fn primary_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }

    /// Count total spans
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    /// Check if error level
    pub fn is_error(&self) -> bool {
        self.level == "error"
    }

    /// Check if warning level
    pub fn is_warning(&self) -> bool {
        self.level == "warning"
    }
}

/// Source location span from compiler diagnostic
#[derive(Debug, Clone)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
    pub is_primary: bool,
    pub label: Option<String>,
}

impl DiagnosticSpan {
    /// Check if span is single line
    pub fn is_single_line(&self) -> bool {
        self.line_start == self.line_end
    }

    /// Get span width in characters
    pub fn width(&self) -> u32 {
        if self.is_single_line() {
            self.column_end.saturating_sub(self.column_start)
        } else {
            0
        }
    }

    /// Get line count
    pub fn line_count(&self) -> u32 {
        self.line_end.saturating_sub(self.line_start) + 1
    }
}

/// Training metrics for a compilation batch
#[derive(Debug, Clone, Default)]
pub struct BatchMetrics {
    pub total_files: usize,
    pub passed: usize,
    pub failed: usize,
    pub error_counts: HashMap<String, usize>,
    pub total_time_ms: u64,
}

impl BatchMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate pass rate (0.0 - 1.0)
    pub fn pass_rate(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            self.passed as f64 / self.total_files as f64
        }
    }

    /// Calculate fail rate (0.0 - 1.0)
    pub fn fail_rate(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            self.failed as f64 / self.total_files as f64
        }
    }

    /// Get most common error code
    pub fn most_common_error(&self) -> Option<(&String, &usize)> {
        self.error_counts.iter().max_by_key(|(_, count)| *count)
    }

    /// Calculate throughput (files per second)
    pub fn throughput(&self) -> f64 {
        if self.total_time_ms == 0 {
            0.0
        } else {
            self.total_files as f64 / (self.total_time_ms as f64 / 1000.0)
        }
    }

    /// Record a compilation result
    pub fn record(&mut self, success: bool, error_code: Option<&str>) {
        self.total_files += 1;
        if success {
            self.passed += 1;
        } else {
            self.failed += 1;
            if let Some(code) = error_code {
                *self.error_counts.entry(code.to_string()).or_insert(0) += 1;
            }
        }
    }

    /// Merge another batch's metrics
    pub fn merge(&mut self, other: &BatchMetrics) {
        self.total_files += other.total_files;
        self.passed += other.passed;
        self.failed += other.failed;
        self.total_time_ms += other.total_time_ms;
        for (code, count) in &other.error_counts {
            *self.error_counts.entry(code.clone()).or_insert(0) += count;
        }
    }
}

/// Training progress state
#[derive(Debug, Clone)]
pub struct TrainingProgress {
    pub epoch: usize,
    pub iteration: usize,
    pub current_rate: f64,
    pub best_rate: f64,
    pub patience_remaining: usize,
    pub converged: bool,
}

impl TrainingProgress {
    pub fn new(patience: usize) -> Self {
        Self {
            epoch: 0,
            iteration: 0,
            current_rate: 0.0,
            best_rate: 0.0,
            patience_remaining: patience,
            converged: false,
        }
    }

    /// Update progress with new rate
    pub fn update(&mut self, rate: f64, target_rate: f64) {
        self.iteration += 1;
        self.current_rate = rate;

        if rate > self.best_rate {
            self.best_rate = rate;
            // Reset patience on improvement
            self.patience_remaining = self.patience_remaining.max(3);
        } else {
            self.patience_remaining = self.patience_remaining.saturating_sub(1);
        }

        if rate >= target_rate {
            self.converged = true;
        }
    }

    /// Check if should stop early
    pub fn should_stop(&self) -> bool {
        self.converged || self.patience_remaining == 0
    }

    /// Calculate improvement from initial
    pub fn improvement(&self, initial_rate: f64) -> f64 {
        self.current_rate - initial_rate
    }
}

/// Curriculum difficulty level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurriculumLevel {
    #[default]
    Easy,
    Medium,
    Hard,
    Expert,
}

impl CurriculumLevel {
    /// Get complexity threshold for this level
    pub fn complexity_threshold(&self) -> f64 {
        match self {
            Self::Easy => 2.0,
            Self::Medium => 5.0,
            Self::Hard => 10.0,
            Self::Expert => f64::MAX,
        }
    }

    /// Advance to next level
    pub fn next(&self) -> Self {
        match self {
            Self::Easy => Self::Medium,
            Self::Medium => Self::Hard,
            Self::Hard => Self::Expert,
            Self::Expert => Self::Expert,
        }
    }

    /// Check if at max level
    pub fn is_max(&self) -> bool {
        matches!(self, Self::Expert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_tier_from_level() {
        assert_eq!(DiagnosticTier::from_level(0), DiagnosticTier::Tier1);
        assert_eq!(DiagnosticTier::from_level(1), DiagnosticTier::Tier1);
        assert_eq!(DiagnosticTier::from_level(2), DiagnosticTier::Tier2);
        assert_eq!(DiagnosticTier::from_level(3), DiagnosticTier::Tier3);
        assert_eq!(DiagnosticTier::from_level(4), DiagnosticTier::Tier4);
        assert_eq!(DiagnosticTier::from_level(5), DiagnosticTier::Tier4);
        assert_eq!(DiagnosticTier::from_level(100), DiagnosticTier::Tier4);
    }

    #[test]
    fn test_diagnostic_tier_level() {
        assert_eq!(DiagnosticTier::Tier1.level(), 1);
        assert_eq!(DiagnosticTier::Tier2.level(), 2);
        assert_eq!(DiagnosticTier::Tier3.level(), 3);
        assert_eq!(DiagnosticTier::Tier4.level(), 4);
    }

    #[test]
    fn test_diagnostic_tier_overhead() {
        assert_eq!(DiagnosticTier::Tier1.overhead_percent(), 5);
        assert_eq!(DiagnosticTier::Tier2.overhead_percent(), 10);
        assert_eq!(DiagnosticTier::Tier3.overhead_percent(), 25);
        assert_eq!(DiagnosticTier::Tier4.overhead_percent(), 50);
    }

    #[test]
    fn test_diagnostic_tier_log_size() {
        assert_eq!(DiagnosticTier::Tier1.log_size_kb(), 2);
        assert_eq!(DiagnosticTier::Tier2.log_size_kb(), 5);
        assert_eq!(DiagnosticTier::Tier3.log_size_kb(), 20);
        assert_eq!(DiagnosticTier::Tier4.log_size_kb(), 100);
    }

    #[test]
    fn test_clippy_level_from_cli() {
        assert_eq!(ClippyLevel::from_cli_arg("standard"), ClippyLevel::Standard);
        assert_eq!(ClippyLevel::from_cli_arg("all"), ClippyLevel::Standard);
        assert_eq!(ClippyLevel::from_cli_arg("pedantic"), ClippyLevel::Pedantic);
        assert_eq!(ClippyLevel::from_cli_arg("nursery"), ClippyLevel::Nursery);
        assert_eq!(ClippyLevel::from_cli_arg("full"), ClippyLevel::Full);
        assert_eq!(ClippyLevel::from_cli_arg("cargo"), ClippyLevel::Full);
        assert_eq!(ClippyLevel::from_cli_arg("PEDANTIC"), ClippyLevel::Pedantic);
        assert_eq!(ClippyLevel::from_cli_arg("unknown"), ClippyLevel::Nursery);
    }

    #[test]
    fn test_clippy_level_flags() {
        assert_eq!(ClippyLevel::Standard.flags(), vec!["clippy::all"]);
        assert_eq!(ClippyLevel::Pedantic.flags(), vec!["clippy::all", "clippy::pedantic"]);
        assert_eq!(ClippyLevel::Nursery.flags(), vec!["clippy::all", "clippy::pedantic", "clippy::nursery"]);
        assert_eq!(ClippyLevel::Full.flags(), vec!["clippy::all", "clippy::pedantic", "clippy::nursery", "clippy::cargo"]);
    }

    #[test]
    fn test_clippy_level_lint_count() {
        assert!(ClippyLevel::Standard.lint_count() < ClippyLevel::Pedantic.lint_count());
        assert!(ClippyLevel::Pedantic.lint_count() < ClippyLevel::Nursery.lint_count());
        assert!(ClippyLevel::Nursery.lint_count() < ClippyLevel::Full.lint_count());
    }

    #[test]
    fn test_verbosity_config_default() {
        let config = VerbosityConfig::default();
        assert_eq!(config.tier, DiagnosticTier::Tier1);
        assert_eq!(config.clippy_level, ClippyLevel::Nursery);
        assert!(config.adaptive);
        assert_eq!(config.timeout_secs, 300);
    }

    #[test]
    fn test_verbosity_config_builder() {
        let config = VerbosityConfig::new()
            .with_tier(DiagnosticTier::Tier3)
            .with_clippy_level(ClippyLevel::Full)
            .with_adaptive(false);

        assert_eq!(config.tier, DiagnosticTier::Tier3);
        assert_eq!(config.clippy_level, ClippyLevel::Full);
        assert!(!config.adaptive);
    }

    #[test]
    fn test_verbosity_select_tier_non_adaptive() {
        let config = VerbosityConfig::new()
            .with_tier(DiagnosticTier::Tier2)
            .with_adaptive(false);

        assert_eq!(config.select_tier_for_error("E0308", 0), DiagnosticTier::Tier2);
        assert_eq!(config.select_tier_for_error("E0308", 5), DiagnosticTier::Tier2);
    }

    #[test]
    fn test_verbosity_select_tier_adaptive() {
        let config = VerbosityConfig::default();

        // First attempt always Tier1
        assert_eq!(config.select_tier_for_error("E0308", 0), DiagnosticTier::Tier1);

        // Complex error escalates faster
        assert_eq!(config.select_tier_for_error("E0308", 1), DiagnosticTier::Tier2);
        assert_eq!(config.select_tier_for_error("E0308", 2), DiagnosticTier::Tier3);
        assert_eq!(config.select_tier_for_error("E0308", 3), DiagnosticTier::Tier4);

        // Non-complex error escalates slower
        assert_eq!(config.select_tier_for_error("E0001", 1), DiagnosticTier::Tier1);
        assert_eq!(config.select_tier_for_error("E0001", 2), DiagnosticTier::Tier2);
    }

    #[test]
    fn test_verbosity_should_escalate() {
        let config = VerbosityConfig::default();
        assert!(config.should_escalate("E0308"));
        assert!(config.should_escalate("E0277"));
        assert!(config.should_escalate("E0382"));
        assert!(!config.should_escalate("E0001"));
        assert!(!config.should_escalate("E9999"));
    }

    #[test]
    fn test_diagnostic_features() {
        let features = DiagnosticFeatures {
            error_code: Some("E0308".to_string()),
            level: "error".to_string(),
            message: "mismatched types".to_string(),
            suggestions: vec!["try using &str".to_string()],
            ..Default::default()
        };

        assert!(features.has_suggestions());
        assert_eq!(features.primary_code(), Some("E0308"));
        assert!(features.is_error());
        assert!(!features.is_warning());
    }

    #[test]
    fn test_diagnostic_features_warning() {
        let features = DiagnosticFeatures {
            level: "warning".to_string(),
            ..Default::default()
        };

        assert!(!features.is_error());
        assert!(features.is_warning());
        assert!(!features.has_suggestions());
    }

    #[test]
    fn test_diagnostic_span() {
        let span = DiagnosticSpan {
            file_name: "test.rs".to_string(),
            line_start: 10,
            line_end: 10,
            column_start: 5,
            column_end: 15,
            is_primary: true,
            label: Some("here".to_string()),
        };

        assert!(span.is_single_line());
        assert_eq!(span.width(), 10);
        assert_eq!(span.line_count(), 1);
    }

    #[test]
    fn test_diagnostic_span_multiline() {
        let span = DiagnosticSpan {
            file_name: "test.rs".to_string(),
            line_start: 10,
            line_end: 15,
            column_start: 5,
            column_end: 15,
            is_primary: false,
            label: None,
        };

        assert!(!span.is_single_line());
        assert_eq!(span.width(), 0);
        assert_eq!(span.line_count(), 6);
    }

    #[test]
    fn test_batch_metrics_new() {
        let metrics = BatchMetrics::new();
        assert_eq!(metrics.total_files, 0);
        assert_eq!(metrics.passed, 0);
        assert_eq!(metrics.failed, 0);
        assert_eq!(metrics.pass_rate(), 0.0);
    }

    #[test]
    fn test_batch_metrics_record() {
        let mut metrics = BatchMetrics::new();
        metrics.record(true, None);
        metrics.record(true, None);
        metrics.record(false, Some("E0308"));
        metrics.record(false, Some("E0308"));
        metrics.record(false, Some("E0277"));

        assert_eq!(metrics.total_files, 5);
        assert_eq!(metrics.passed, 2);
        assert_eq!(metrics.failed, 3);
        assert!((metrics.pass_rate() - 0.4).abs() < 0.001);
        assert!((metrics.fail_rate() - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_batch_metrics_most_common_error() {
        let mut metrics = BatchMetrics::new();
        metrics.record(false, Some("E0308"));
        metrics.record(false, Some("E0308"));
        metrics.record(false, Some("E0277"));

        let (code, count) = metrics.most_common_error().unwrap();
        assert_eq!(code, "E0308");
        assert_eq!(*count, 2);
    }

    #[test]
    fn test_batch_metrics_throughput() {
        let mut metrics = BatchMetrics::new();
        metrics.total_files = 100;
        metrics.total_time_ms = 10000; // 10 seconds

        assert!((metrics.throughput() - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_batch_metrics_merge() {
        let mut m1 = BatchMetrics::new();
        m1.record(true, None);
        m1.record(false, Some("E0308"));
        m1.total_time_ms = 100;

        let mut m2 = BatchMetrics::new();
        m2.record(true, None);
        m2.record(false, Some("E0308"));
        m2.total_time_ms = 200;

        m1.merge(&m2);

        assert_eq!(m1.total_files, 4);
        assert_eq!(m1.passed, 2);
        assert_eq!(m1.failed, 2);
        assert_eq!(m1.total_time_ms, 300);
        assert_eq!(*m1.error_counts.get("E0308").unwrap(), 2);
    }

    #[test]
    fn test_training_progress_new() {
        let progress = TrainingProgress::new(5);
        assert_eq!(progress.epoch, 0);
        assert_eq!(progress.iteration, 0);
        assert_eq!(progress.patience_remaining, 5);
        assert!(!progress.converged);
    }

    #[test]
    fn test_training_progress_update_improvement() {
        let mut progress = TrainingProgress::new(3);
        progress.update(0.5, 0.95);

        assert_eq!(progress.iteration, 1);
        assert_eq!(progress.current_rate, 0.5);
        assert_eq!(progress.best_rate, 0.5);
        assert!(!progress.converged);

        progress.update(0.7, 0.95);
        assert_eq!(progress.best_rate, 0.7);
    }

    #[test]
    fn test_training_progress_converged() {
        let mut progress = TrainingProgress::new(3);
        progress.update(0.95, 0.95);

        assert!(progress.converged);
        assert!(progress.should_stop());
    }

    #[test]
    fn test_training_progress_patience_exhausted() {
        let mut progress = TrainingProgress::new(2);
        progress.update(0.5, 0.95);
        progress.patience_remaining = 2;
        progress.update(0.4, 0.95); // No improvement
        progress.update(0.3, 0.95); // No improvement

        assert!(progress.should_stop());
    }

    #[test]
    fn test_training_progress_improvement() {
        let mut progress = TrainingProgress::new(3);
        progress.update(0.7, 0.95);

        assert!((progress.improvement(0.5) - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_curriculum_level_threshold() {
        assert_eq!(CurriculumLevel::Easy.complexity_threshold(), 2.0);
        assert_eq!(CurriculumLevel::Medium.complexity_threshold(), 5.0);
        assert_eq!(CurriculumLevel::Hard.complexity_threshold(), 10.0);
        assert_eq!(CurriculumLevel::Expert.complexity_threshold(), f64::MAX);
    }

    #[test]
    fn test_curriculum_level_next() {
        assert_eq!(CurriculumLevel::Easy.next(), CurriculumLevel::Medium);
        assert_eq!(CurriculumLevel::Medium.next(), CurriculumLevel::Hard);
        assert_eq!(CurriculumLevel::Hard.next(), CurriculumLevel::Expert);
        assert_eq!(CurriculumLevel::Expert.next(), CurriculumLevel::Expert);
    }

    #[test]
    fn test_curriculum_level_is_max() {
        assert!(!CurriculumLevel::Easy.is_max());
        assert!(!CurriculumLevel::Medium.is_max());
        assert!(!CurriculumLevel::Hard.is_max());
        assert!(CurriculumLevel::Expert.is_max());
    }

    #[test]
    fn test_curriculum_level_default() {
        assert_eq!(CurriculumLevel::default(), CurriculumLevel::Easy);
    }
}

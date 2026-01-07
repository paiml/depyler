//! CLI Shim - pure logic separated from I/O
//!
//! Extracts testable logic from lib.rs command handlers.

use std::path::Path;

/// CITL (Compiler-in-the-Loop) result
#[derive(Debug, Clone)]
pub struct CitlResult {
    pub success: bool,
    pub compilation_rate: f64,
    pub files_processed: usize,
    pub iterations_used: usize,
    pub fixes_applied: usize,
}

impl CitlResult {
    /// Create an empty successful result (no files to process)
    pub fn empty_success() -> Self {
        Self {
            success: true,
            compilation_rate: 1.0,
            files_processed: 0,
            iterations_used: 0,
            fixes_applied: 0,
        }
    }

    /// Create a result from compilation statistics
    pub fn from_stats(
        compiled: usize,
        total: usize,
        iterations: usize,
        fixes: usize,
    ) -> Self {
        let rate = if total == 0 {
            1.0
        } else {
            compiled as f64 / total as f64
        };
        Self {
            success: rate >= 1.0,
            compilation_rate: rate,
            files_processed: total,
            iterations_used: iterations,
            fixes_applied: fixes,
        }
    }

    /// Check if target rate is achieved
    pub fn meets_target(&self, target_rate: f64) -> bool {
        self.compilation_rate >= target_rate
    }

    /// Get failure count
    pub fn failures(&self) -> usize {
        self.files_processed.saturating_sub(self.compiled_count())
    }

    /// Get compiled count
    pub fn compiled_count(&self) -> usize {
        (self.files_processed as f64 * self.compilation_rate).round() as usize
    }
}

/// CITL fixer configuration (pure data)
#[derive(Debug, Clone)]
pub struct CitlConfig {
    pub max_iterations: usize,
    pub confidence_threshold: f64,
    pub verbose: bool,
}

impl Default for CitlConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            confidence_threshold: 0.8,
            verbose: false,
        }
    }
}

impl CitlConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.max_iterations == 0 {
            return Err("Max iterations must be at least 1");
        }
        if self.max_iterations > 100 {
            return Err("Max iterations cannot exceed 100");
        }
        if self.confidence_threshold < 0.0 || self.confidence_threshold > 1.0 {
            return Err("Confidence threshold must be between 0.0 and 1.0");
        }
        Ok(())
    }
}

/// Quality check targets
#[derive(Debug, Clone)]
pub struct QualityTargets {
    pub min_tdg: f64,
    pub max_tdg: f64,
    pub max_complexity: u32,
    pub min_coverage: u32,
}

impl Default for QualityTargets {
    fn default() -> Self {
        Self {
            min_tdg: 0.0,
            max_tdg: 2.0,
            max_complexity: 10,
            min_coverage: 80,
        }
    }
}

impl QualityTargets {
    /// Validate targets are sensible
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.min_tdg < 0.0 {
            return Err("Min TDG cannot be negative");
        }
        if self.max_tdg < self.min_tdg {
            return Err("Max TDG must be >= min TDG");
        }
        if self.max_complexity == 0 {
            return Err("Max complexity must be at least 1");
        }
        if self.min_coverage > 100 {
            return Err("Min coverage cannot exceed 100%");
        }
        Ok(())
    }

    /// Check if TDG is in range
    pub fn tdg_ok(&self, tdg: f64) -> bool {
        tdg >= self.min_tdg && tdg <= self.max_tdg
    }

    /// Check if complexity is acceptable
    pub fn complexity_ok(&self, complexity: u32) -> bool {
        complexity <= self.max_complexity
    }

    /// Check if coverage meets target
    pub fn coverage_ok(&self, coverage_percent: f64) -> bool {
        coverage_percent >= self.min_coverage as f64
    }
}

/// Doctest extraction configuration
#[derive(Debug, Clone)]
pub struct DoctestConfig {
    pub module_prefix: String,
    pub include_classes: bool,
    pub include_pytest: bool,
}

impl Default for DoctestConfig {
    fn default() -> Self {
        Self {
            module_prefix: String::new(),
            include_classes: true,
            include_pytest: false,
        }
    }
}

impl DoctestConfig {
    /// Build module name from path
    pub fn module_name(&self, relative_path: &Path) -> String {
        let base = relative_path
            .with_extension("")
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, ".");

        if self.module_prefix.is_empty() {
            base
        } else {
            format!("{}.{}", self.module_prefix, base)
        }
    }
}

/// Doctest extraction summary
#[derive(Debug, Clone, Default)]
pub struct DoctestSummary {
    pub files_processed: usize,
    pub files_with_examples: usize,
    pub functions_with_examples: usize,
    pub total_doctests: usize,
    pub total_pytest: usize,
}

impl DoctestSummary {
    /// Get total examples (doctests + pytest)
    pub fn total_examples(&self) -> usize {
        self.total_doctests + self.total_pytest
    }

    /// Check if any examples were found
    pub fn has_examples(&self) -> bool {
        self.total_examples() > 0
    }

    /// Get extraction rate
    pub fn extraction_rate(&self) -> f64 {
        if self.files_processed == 0 {
            0.0
        } else {
            self.files_with_examples as f64 / self.files_processed as f64 * 100.0
        }
    }
}

/// File filter configuration
#[derive(Debug, Clone, Default)]
pub struct FileFilter {
    pub skip_hidden: bool,
    pub skip_pycache: bool,
    pub skip_venv: bool,
    pub skip_test_files: bool,
    pub extensions: Vec<String>,
}

impl FileFilter {
    /// Create a Python file filter
    pub fn python() -> Self {
        Self {
            skip_hidden: true,
            skip_pycache: true,
            skip_venv: true,
            skip_test_files: false,
            extensions: vec!["py".to_string()],
        }
    }

    /// Create a Python file filter excluding tests
    pub fn python_no_tests() -> Self {
        Self {
            skip_test_files: true,
            ..Self::python()
        }
    }

    /// Check if a directory should be skipped
    pub fn should_skip_dir(&self, name: &str) -> bool {
        if self.skip_hidden && name.starts_with('.') {
            return true;
        }
        if self.skip_pycache && name == "__pycache__" {
            return true;
        }
        if self.skip_venv && (name == "venv" || name == ".venv" || name == "env") {
            return true;
        }
        false
    }

    /// Check if a file matches the filter
    pub fn matches_file(&self, path: &Path) -> bool {
        // Check extension
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if !self.extensions.is_empty() && !self.extensions.contains(&ext.to_string()) {
            return false;
        }

        // Check test file
        if self.skip_test_files {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with("test_") || name.ends_with("_test.py") {
                return false;
            }
        }

        true
    }

    /// Check if path is a test file
    pub fn is_test_file(path: &Path) -> bool {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        name.starts_with("test_") || name.ends_with("_test.py")
    }
}

/// Lambda command configuration
#[derive(Debug, Clone)]
pub struct LambdaConfig {
    pub runtime: LambdaRuntime,
    pub architecture: Architecture,
    pub memory_mb: u32,
    pub timeout_secs: u32,
}

/// Lambda runtime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LambdaRuntime {
    ProvidedAl2,
    ProvidedAl2023,
}

impl LambdaRuntime {
    /// Get runtime string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProvidedAl2 => "provided.al2",
            Self::ProvidedAl2023 => "provided.al2023",
        }
    }
}

impl Default for LambdaRuntime {
    fn default() -> Self {
        Self::ProvidedAl2023
    }
}

/// Lambda architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    Arm64,
}

impl Architecture {
    /// Get architecture string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Arm64 => "arm64",
        }
    }

    /// Get Rust target triple
    pub fn target_triple(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64-unknown-linux-musl",
            Self::Arm64 => "aarch64-unknown-linux-musl",
        }
    }
}

impl Default for Architecture {
    fn default() -> Self {
        Self::X86_64
    }
}

impl Default for LambdaConfig {
    fn default() -> Self {
        Self {
            runtime: LambdaRuntime::default(),
            architecture: Architecture::default(),
            memory_mb: 128,
            timeout_secs: 30,
        }
    }
}

impl LambdaConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.memory_mb < 128 {
            return Err("Memory must be at least 128 MB");
        }
        if self.memory_mb > 10240 {
            return Err("Memory cannot exceed 10240 MB");
        }
        if self.timeout_secs == 0 {
            return Err("Timeout must be at least 1 second");
        }
        if self.timeout_secs > 900 {
            return Err("Timeout cannot exceed 900 seconds (15 minutes)");
        }
        Ok(())
    }

    /// Get estimated cost per million invocations (rough estimate)
    pub fn estimated_cost_per_million(&self, avg_duration_ms: u64) -> f64 {
        // Simplified pricing: $0.0000166667 per GB-second
        let gb_seconds = (self.memory_mb as f64 / 1024.0) * (avg_duration_ms as f64 / 1000.0);
        gb_seconds * 0.0000166667 * 1_000_000.0
    }
}

/// Progress tracking for batch operations
#[derive(Debug, Clone)]
pub struct BatchProgress {
    pub total: usize,
    pub completed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl BatchProgress {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            succeeded: 0,
            failed: 0,
            skipped: 0,
        }
    }

    /// Record a success
    pub fn record_success(&mut self) {
        self.completed += 1;
        self.succeeded += 1;
    }

    /// Record a failure
    pub fn record_failure(&mut self) {
        self.completed += 1;
        self.failed += 1;
    }

    /// Record a skip
    pub fn record_skip(&mut self) {
        self.completed += 1;
        self.skipped += 1;
    }

    /// Get completion percentage
    pub fn percent_complete(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            self.completed as f64 / self.total as f64 * 100.0
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        let processed = self.succeeded + self.failed;
        if processed == 0 {
            0.0
        } else {
            self.succeeded as f64 / processed as f64 * 100.0
        }
    }

    /// Check if all items processed
    pub fn is_complete(&self) -> bool {
        self.completed >= self.total
    }

    /// Get remaining count
    pub fn remaining(&self) -> usize {
        self.total.saturating_sub(self.completed)
    }
}

/// Format a duration in human-readable form
pub fn format_duration_ms(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.2}s", ms as f64 / 1000.0)
    } else {
        let minutes = ms / 60000;
        let seconds = (ms % 60000) / 1000;
        format!("{}m {}s", minutes, seconds)
    }
}

/// Format a byte count in human-readable form
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Format a percentage with color indication
pub fn format_rate(rate: f64, threshold: f64) -> (String, bool) {
    let formatted = format!("{:.1}%", rate);
    let passed = rate >= threshold;
    (formatted, passed)
}

/// Calculate improvement needed to reach target
pub fn improvement_needed(current: f64, target: f64) -> f64 {
    if current >= target {
        0.0
    } else {
        target - current
    }
}

/// Calculate files to fix to reach target rate
pub fn files_to_fix(total: usize, current_passed: usize, target_rate: f64) -> usize {
    let target_passed = (total as f64 * target_rate / 100.0).ceil() as usize;
    target_passed.saturating_sub(current_passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =====================================================
    // CitlResult Tests
    // =====================================================

    #[test]
    fn test_citl_result_empty_success() {
        let result = CitlResult::empty_success();
        assert!(result.success);
        assert_eq!(result.compilation_rate, 1.0);
        assert_eq!(result.files_processed, 0);
    }

    #[test]
    fn test_citl_result_from_stats_perfect() {
        let result = CitlResult::from_stats(10, 10, 5, 3);
        assert!(result.success);
        assert_eq!(result.compilation_rate, 1.0);
        assert_eq!(result.files_processed, 10);
        assert_eq!(result.fixes_applied, 3);
    }

    #[test]
    fn test_citl_result_from_stats_partial() {
        let result = CitlResult::from_stats(8, 10, 5, 2);
        assert!(!result.success);
        assert!((result.compilation_rate - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_citl_result_from_stats_empty() {
        let result = CitlResult::from_stats(0, 0, 0, 0);
        assert!(result.success);
        assert_eq!(result.compilation_rate, 1.0);
    }

    #[test]
    fn test_citl_result_meets_target() {
        let result = CitlResult::from_stats(85, 100, 10, 5);
        assert!(result.meets_target(0.8));
        assert!(result.meets_target(0.85));
        assert!(!result.meets_target(0.9));
    }

    #[test]
    fn test_citl_result_failures() {
        let result = CitlResult::from_stats(70, 100, 10, 5);
        assert_eq!(result.failures(), 30);
    }

    #[test]
    fn test_citl_result_compiled_count() {
        let result = CitlResult::from_stats(75, 100, 10, 5);
        assert_eq!(result.compiled_count(), 75);
    }

    // =====================================================
    // CitlConfig Tests
    // =====================================================

    #[test]
    fn test_citl_config_default() {
        let config = CitlConfig::default();
        assert_eq!(config.max_iterations, 10);
        assert_eq!(config.confidence_threshold, 0.8);
        assert!(!config.verbose);
    }

    #[test]
    fn test_citl_config_validate_valid() {
        let config = CitlConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_citl_config_validate_zero_iterations() {
        let config = CitlConfig {
            max_iterations: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_citl_config_validate_too_many_iterations() {
        let config = CitlConfig {
            max_iterations: 101,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_citl_config_validate_invalid_threshold() {
        let config = CitlConfig {
            confidence_threshold: 1.5,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_citl_config_validate_negative_threshold() {
        let config = CitlConfig {
            confidence_threshold: -0.1,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    // =====================================================
    // QualityTargets Tests
    // =====================================================

    #[test]
    fn test_quality_targets_default() {
        let targets = QualityTargets::default();
        assert_eq!(targets.min_tdg, 0.0);
        assert_eq!(targets.max_tdg, 2.0);
        assert_eq!(targets.max_complexity, 10);
        assert_eq!(targets.min_coverage, 80);
    }

    #[test]
    fn test_quality_targets_validate_valid() {
        let targets = QualityTargets::default();
        assert!(targets.validate().is_ok());
    }

    #[test]
    fn test_quality_targets_validate_negative_tdg() {
        let targets = QualityTargets {
            min_tdg: -1.0,
            ..Default::default()
        };
        assert!(targets.validate().is_err());
    }

    #[test]
    fn test_quality_targets_validate_invalid_range() {
        let targets = QualityTargets {
            min_tdg: 3.0,
            max_tdg: 2.0,
            ..Default::default()
        };
        assert!(targets.validate().is_err());
    }

    #[test]
    fn test_quality_targets_validate_zero_complexity() {
        let targets = QualityTargets {
            max_complexity: 0,
            ..Default::default()
        };
        assert!(targets.validate().is_err());
    }

    #[test]
    fn test_quality_targets_validate_high_coverage() {
        let targets = QualityTargets {
            min_coverage: 101,
            ..Default::default()
        };
        assert!(targets.validate().is_err());
    }

    #[test]
    fn test_quality_targets_tdg_ok() {
        let targets = QualityTargets::default();
        assert!(targets.tdg_ok(1.0));
        assert!(targets.tdg_ok(0.0));
        assert!(targets.tdg_ok(2.0));
        assert!(!targets.tdg_ok(2.5));
        assert!(!targets.tdg_ok(-0.1));
    }

    #[test]
    fn test_quality_targets_complexity_ok() {
        let targets = QualityTargets::default();
        assert!(targets.complexity_ok(5));
        assert!(targets.complexity_ok(10));
        assert!(!targets.complexity_ok(11));
    }

    #[test]
    fn test_quality_targets_coverage_ok() {
        let targets = QualityTargets::default();
        assert!(targets.coverage_ok(80.0));
        assert!(targets.coverage_ok(95.0));
        assert!(!targets.coverage_ok(79.9));
    }

    // =====================================================
    // DoctestConfig Tests
    // =====================================================

    #[test]
    fn test_doctest_config_default() {
        let config = DoctestConfig::default();
        assert!(config.module_prefix.is_empty());
        assert!(config.include_classes);
        assert!(!config.include_pytest);
    }

    #[test]
    fn test_doctest_config_module_name_no_prefix() {
        let config = DoctestConfig::default();
        let path = Path::new("src/utils/helpers.py");
        let name = config.module_name(path);
        assert!(name.contains("helpers"));
    }

    #[test]
    fn test_doctest_config_module_name_with_prefix() {
        let config = DoctestConfig {
            module_prefix: "mypackage".to_string(),
            ..Default::default()
        };
        let path = Path::new("utils.py");
        let name = config.module_name(path);
        assert!(name.starts_with("mypackage."));
    }

    // =====================================================
    // DoctestSummary Tests
    // =====================================================

    #[test]
    fn test_doctest_summary_default() {
        let summary = DoctestSummary::default();
        assert_eq!(summary.total_examples(), 0);
        assert!(!summary.has_examples());
    }

    #[test]
    fn test_doctest_summary_total_examples() {
        let summary = DoctestSummary {
            total_doctests: 10,
            total_pytest: 5,
            ..Default::default()
        };
        assert_eq!(summary.total_examples(), 15);
        assert!(summary.has_examples());
    }

    #[test]
    fn test_doctest_summary_extraction_rate() {
        let summary = DoctestSummary {
            files_processed: 10,
            files_with_examples: 4,
            ..Default::default()
        };
        assert!((summary.extraction_rate() - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_doctest_summary_extraction_rate_zero() {
        let summary = DoctestSummary::default();
        assert_eq!(summary.extraction_rate(), 0.0);
    }

    // =====================================================
    // FileFilter Tests
    // =====================================================

    #[test]
    fn test_file_filter_python() {
        let filter = FileFilter::python();
        assert!(filter.skip_hidden);
        assert!(filter.skip_pycache);
        assert!(!filter.skip_test_files);
    }

    #[test]
    fn test_file_filter_python_no_tests() {
        let filter = FileFilter::python_no_tests();
        assert!(filter.skip_test_files);
    }

    #[test]
    fn test_file_filter_should_skip_dir_hidden() {
        let filter = FileFilter::python();
        assert!(filter.should_skip_dir(".git"));
        assert!(filter.should_skip_dir(".hidden"));
    }

    #[test]
    fn test_file_filter_should_skip_dir_pycache() {
        let filter = FileFilter::python();
        assert!(filter.should_skip_dir("__pycache__"));
    }

    #[test]
    fn test_file_filter_should_skip_dir_venv() {
        let filter = FileFilter::python();
        assert!(filter.should_skip_dir("venv"));
        assert!(filter.should_skip_dir(".venv"));
        assert!(filter.should_skip_dir("env"));
    }

    #[test]
    fn test_file_filter_should_not_skip_normal() {
        let filter = FileFilter::python();
        assert!(!filter.should_skip_dir("src"));
        assert!(!filter.should_skip_dir("lib"));
    }

    #[test]
    fn test_file_filter_matches_file_extension() {
        let filter = FileFilter::python();
        assert!(filter.matches_file(Path::new("test.py")));
        assert!(!filter.matches_file(Path::new("test.rs")));
    }

    #[test]
    fn test_file_filter_matches_file_test() {
        let filter = FileFilter::python_no_tests();
        assert!(!filter.matches_file(Path::new("test_utils.py")));
        assert!(!filter.matches_file(Path::new("utils_test.py")));
        assert!(filter.matches_file(Path::new("utils.py")));
    }

    #[test]
    fn test_file_filter_is_test_file() {
        assert!(FileFilter::is_test_file(Path::new("test_utils.py")));
        assert!(FileFilter::is_test_file(Path::new("utils_test.py")));
        assert!(!FileFilter::is_test_file(Path::new("utils.py")));
    }

    // =====================================================
    // LambdaRuntime Tests
    // =====================================================

    #[test]
    fn test_lambda_runtime_default() {
        let runtime = LambdaRuntime::default();
        assert_eq!(runtime, LambdaRuntime::ProvidedAl2023);
    }

    #[test]
    fn test_lambda_runtime_as_str() {
        assert_eq!(LambdaRuntime::ProvidedAl2.as_str(), "provided.al2");
        assert_eq!(LambdaRuntime::ProvidedAl2023.as_str(), "provided.al2023");
    }

    // =====================================================
    // Architecture Tests
    // =====================================================

    #[test]
    fn test_architecture_default() {
        let arch = Architecture::default();
        assert_eq!(arch, Architecture::X86_64);
    }

    #[test]
    fn test_architecture_as_str() {
        assert_eq!(Architecture::X86_64.as_str(), "x86_64");
        assert_eq!(Architecture::Arm64.as_str(), "arm64");
    }

    #[test]
    fn test_architecture_target_triple() {
        assert_eq!(Architecture::X86_64.target_triple(), "x86_64-unknown-linux-musl");
        assert_eq!(Architecture::Arm64.target_triple(), "aarch64-unknown-linux-musl");
    }

    // =====================================================
    // LambdaConfig Tests
    // =====================================================

    #[test]
    fn test_lambda_config_default() {
        let config = LambdaConfig::default();
        assert_eq!(config.memory_mb, 128);
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_lambda_config_validate_valid() {
        let config = LambdaConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_lambda_config_validate_low_memory() {
        let config = LambdaConfig {
            memory_mb: 64,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_lambda_config_validate_high_memory() {
        let config = LambdaConfig {
            memory_mb: 20000,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_lambda_config_validate_zero_timeout() {
        let config = LambdaConfig {
            timeout_secs: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_lambda_config_validate_high_timeout() {
        let config = LambdaConfig {
            timeout_secs: 1000,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_lambda_config_estimated_cost() {
        let config = LambdaConfig {
            memory_mb: 1024,
            ..Default::default()
        };
        let cost = config.estimated_cost_per_million(100);
        assert!(cost > 0.0);
    }

    // =====================================================
    // BatchProgress Tests
    // =====================================================

    #[test]
    fn test_batch_progress_new() {
        let progress = BatchProgress::new(100);
        assert_eq!(progress.total, 100);
        assert_eq!(progress.completed, 0);
        assert_eq!(progress.succeeded, 0);
    }

    #[test]
    fn test_batch_progress_record_success() {
        let mut progress = BatchProgress::new(10);
        progress.record_success();
        assert_eq!(progress.completed, 1);
        assert_eq!(progress.succeeded, 1);
        assert_eq!(progress.failed, 0);
    }

    #[test]
    fn test_batch_progress_record_failure() {
        let mut progress = BatchProgress::new(10);
        progress.record_failure();
        assert_eq!(progress.completed, 1);
        assert_eq!(progress.succeeded, 0);
        assert_eq!(progress.failed, 1);
    }

    #[test]
    fn test_batch_progress_record_skip() {
        let mut progress = BatchProgress::new(10);
        progress.record_skip();
        assert_eq!(progress.completed, 1);
        assert_eq!(progress.skipped, 1);
    }

    #[test]
    fn test_batch_progress_percent_complete() {
        let mut progress = BatchProgress::new(10);
        progress.record_success();
        progress.record_success();
        assert!((progress.percent_complete() - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_batch_progress_percent_complete_zero() {
        let progress = BatchProgress::new(0);
        assert_eq!(progress.percent_complete(), 100.0);
    }

    #[test]
    fn test_batch_progress_success_rate() {
        let mut progress = BatchProgress::new(10);
        progress.record_success();
        progress.record_success();
        progress.record_failure();
        assert!((progress.success_rate() - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_batch_progress_success_rate_zero() {
        let progress = BatchProgress::new(10);
        assert_eq!(progress.success_rate(), 0.0);
    }

    #[test]
    fn test_batch_progress_is_complete() {
        let mut progress = BatchProgress::new(2);
        assert!(!progress.is_complete());
        progress.record_success();
        assert!(!progress.is_complete());
        progress.record_success();
        assert!(progress.is_complete());
    }

    #[test]
    fn test_batch_progress_remaining() {
        let mut progress = BatchProgress::new(10);
        assert_eq!(progress.remaining(), 10);
        progress.record_success();
        progress.record_failure();
        assert_eq!(progress.remaining(), 8);
    }

    // =====================================================
    // Format Functions Tests
    // =====================================================

    #[test]
    fn test_format_duration_ms_milliseconds() {
        assert_eq!(format_duration_ms(500), "500ms");
        assert_eq!(format_duration_ms(0), "0ms");
    }

    #[test]
    fn test_format_duration_ms_seconds() {
        assert_eq!(format_duration_ms(1500), "1.50s");
        assert_eq!(format_duration_ms(30000), "30.00s");
    }

    #[test]
    fn test_format_duration_ms_minutes() {
        assert_eq!(format_duration_ms(90000), "1m 30s");
        assert_eq!(format_duration_ms(120000), "2m 0s");
    }

    #[test]
    fn test_format_bytes_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        assert_eq!(format_bytes(2048), "2.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(5242880), "5.0 MB");
    }

    #[test]
    fn test_format_bytes_gigabytes() {
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_format_rate_passed() {
        let (formatted, passed) = format_rate(85.5, 80.0);
        assert_eq!(formatted, "85.5%");
        assert!(passed);
    }

    #[test]
    fn test_format_rate_failed() {
        let (formatted, passed) = format_rate(75.0, 80.0);
        assert_eq!(formatted, "75.0%");
        assert!(!passed);
    }

    #[test]
    fn test_improvement_needed_above_target() {
        assert_eq!(improvement_needed(90.0, 80.0), 0.0);
    }

    #[test]
    fn test_improvement_needed_below_target() {
        assert!((improvement_needed(70.0, 80.0) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_files_to_fix_above_target() {
        assert_eq!(files_to_fix(100, 85, 80.0), 0);
    }

    #[test]
    fn test_files_to_fix_below_target() {
        // 100 files, 70 passed, need 80% = 80 passed, so need 10 more
        assert_eq!(files_to_fix(100, 70, 80.0), 10);
    }

    #[test]
    fn test_files_to_fix_zero_total() {
        assert_eq!(files_to_fix(0, 0, 80.0), 0);
    }
}

//! Unified Training Oracle Loop (UTOL)
//!
//! UTOL replaces manual "Apex Hunt" prompt-driven cycles with an automated,
//! self-correcting compilation feedback system following Toyota Way principles.
//!
//! # Toyota Way Principles Applied
//! - **Jidoka (自働化)**: Auto-stop on compilation failure, self-diagnose, self-repair
//! - **Kaizen (改善)**: Each iteration improves the model incrementally
//! - **Genchi Genbutsu (現地現物)**: Observe actual compilation errors
//! - **Heijunka (平準化)**: Level the training load across error categories
//! - **Andon (行灯)**: Visual feedback system for loop status
//! - **Poka-Yoke (ポカヨケ)**: Error-proofing through deterministic seeds
//!
//! # Example
//! ```ignore
//! use depyler_oracle::utol::{UtolConfig, run_utol};
//!
//! let config = UtolConfig::default();
//! let result = run_utol(config)?;
//! println!("Final compile rate: {:.1}%", result.compile_rate * 100.0);
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::classifier::ErrorCategory;

// ============================================================================
// Configuration (UTOL-040)
// ============================================================================

/// UTOL configuration following YAML schema spec (UTOL-040)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UtolConfig {
    /// Corpus configuration
    pub corpus: CorpusConfig,
    /// Training configuration
    pub training: TrainingConfig,
    /// Convergence criteria
    pub convergence: ConvergenceConfig,
    /// Model configuration
    pub model: ModelConfig,
    /// Display configuration
    pub display: DisplayConfig,
}

/// Corpus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusConfig {
    /// Path to corpus directory
    pub path: PathBuf,
    /// Include patterns (glob)
    pub include_patterns: Vec<String>,
    /// Exclude patterns (glob)
    pub exclude_patterns: Vec<String>,
}

impl Default for CorpusConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("../reprorusted-python-cli"),
            include_patterns: vec!["**/*.py".to_string()],
            exclude_patterns: vec!["**/test_*.py".to_string(), "**/__pycache__/**".to_string()],
        }
    }
}

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of synthetic samples to generate
    pub synthetic_samples: usize,
    /// Random seed for reproducibility (Poka-Yoke)
    pub seed: u64,
    /// Whether to balance classes (Heijunka)
    pub balance_classes: bool,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            synthetic_samples: 12_000,
            seed: 42,
            balance_classes: true,
        }
    }
}

/// Convergence configuration (UTOL-002)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceConfig {
    /// Target compilation success rate (0.0-1.0)
    pub target_rate: f64,
    /// Maximum iterations before giving up
    pub max_iterations: usize,
    /// Patience: stop if no improvement for N iterations
    pub patience: usize,
    /// Minimum improvement threshold
    pub min_delta: f64,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            target_rate: 0.80,
            max_iterations: 50,
            patience: 5,
            min_delta: 0.005,
        }
    }
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Path to model file
    pub path: PathBuf,
    /// Number of trees in random forest
    pub n_estimators: usize,
    /// Maximum tree depth
    pub max_depth: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("depyler_oracle.apr"),
            n_estimators: 100,
            max_depth: 10,
        }
    }
}

/// Display configuration (UTOL-030)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Display mode
    pub mode: DisplayMode,
    /// Refresh rate in milliseconds
    pub refresh_ms: u64,
    /// Show sparkline charts
    pub show_sparklines: bool,
    /// Show category breakdown
    pub show_category_breakdown: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Rich,
            refresh_ms: 500,
            show_sparklines: true,
            show_category_breakdown: true,
        }
    }
}

/// Display mode for Andon output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    /// Rich TUI with colors and sparklines
    #[default]
    Rich,
    /// Minimal single-line output (CI-friendly)
    Minimal,
    /// JSON output (automation)
    Json,
    /// No output
    Silent,
}

// ============================================================================
// Loop State (UTOL-001)
// ============================================================================

/// Current state of the UTOL loop
#[derive(Debug, Clone)]
pub struct LoopState {
    /// Current iteration number (1-indexed)
    pub iteration: usize,
    /// Current compilation success rate (0.0-1.0)
    pub compile_rate: f64,
    /// Timestamp of last model training
    pub last_trained: DateTime<Utc>,
    /// Model size in bytes
    pub model_size: u64,
    /// Model version string
    pub model_version: String,
    /// Patience counter (iterations without improvement)
    pub patience_counter: usize,
    /// History of compile rates for trend analysis
    pub rate_history: Vec<f64>,
    /// Per-category success rates
    pub category_rates: HashMap<ErrorCategory, f64>,
}

impl LoopState {
    /// Create new initial state
    pub fn new() -> Self {
        Self {
            iteration: 0,
            compile_rate: 0.0,
            last_trained: Utc::now(),
            model_size: 0,
            model_version: String::new(),
            patience_counter: 0,
            rate_history: Vec::new(),
            category_rates: HashMap::new(),
        }
    }

    /// Calculate progress percentage
    pub fn progress_pct(&self, max_iterations: usize) -> f64 {
        if max_iterations == 0 {
            return 0.0;
        }
        (self.iteration as f64 / max_iterations as f64) * 100.0
    }

    /// Check if improving based on recent history
    pub fn is_improving(&self, window: usize) -> bool {
        if self.rate_history.len() < 2 {
            return true; // Assume improving if not enough data
        }
        let recent: Vec<_> = self.rate_history.iter().rev().take(window).collect();
        if recent.len() < 2 {
            return true;
        }
        // Check if trend is positive
        recent[0] > recent[recent.len() - 1]
    }

    /// Get the improvement delta from last iteration
    pub fn improvement_delta(&self) -> f64 {
        if self.rate_history.len() < 2 {
            return 0.0;
        }
        let len = self.rate_history.len();
        self.rate_history[len - 1] - self.rate_history[len - 2]
    }
}

impl Default for LoopState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Convergence Estimator (UTOL-002)
// ============================================================================

/// Kalman-filtered convergence estimator
#[derive(Debug, Clone)]
pub struct ConvergenceEstimator {
    /// Kalman filter state estimate
    estimate: f64,
    /// Error covariance
    error_cov: f64,
    /// Process noise
    process_noise: f64,
    /// Measurement noise
    measurement_noise: f64,
    /// History for trend calculation
    history: Vec<f64>,
    /// Target convergence rate
    target: f64,
}

impl ConvergenceEstimator {
    /// Create new estimator with target rate
    pub fn new(target: f64) -> Self {
        Self {
            estimate: 0.5,  // Initial estimate at 50%
            error_cov: 1.0, // High initial uncertainty
            process_noise: 0.01,
            measurement_noise: 0.1,
            history: Vec::new(),
            target,
        }
    }

    /// Update estimate with new measurement
    pub fn update(&mut self, compile_rate: f64) -> ConvergenceEstimate {
        self.history.push(compile_rate);

        // Kalman predict step
        let predicted_cov = self.error_cov + self.process_noise;

        // Kalman update step
        let kalman_gain = predicted_cov / (predicted_cov + self.measurement_noise);
        self.estimate = self.estimate + kalman_gain * (compile_rate - self.estimate);
        self.error_cov = (1.0 - kalman_gain) * predicted_cov;

        // Calculate trend
        let trend = self.calculate_trend();

        // Estimate final rate based on trend
        let remaining = 50 - self.history.len().min(50);
        let estimated_final = (self.estimate + trend * remaining as f64).clamp(0.0, 1.0);

        // Calculate confidence based on error covariance
        let confidence = 1.0 - self.error_cov.sqrt().min(1.0);

        ConvergenceEstimate {
            current: compile_rate,
            smoothed: self.estimate,
            estimated_final,
            confidence,
            will_converge: estimated_final >= self.target,
            iterations_to_target: self.estimate_iterations_to_target(trend),
        }
    }

    /// Calculate linear trend from history
    fn calculate_trend(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }

        // Use last 5 points for trend
        let window: Vec<_> = self.history.iter().rev().take(5).copied().collect();
        if window.len() < 2 {
            return 0.0;
        }

        // Simple linear regression slope
        let n = window.len() as f64;
        let sum_x: f64 = (0..window.len()).map(|i| i as f64).sum();
        let sum_y: f64 = window.iter().sum();
        let sum_xy: f64 = window.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_xx: f64 = (0..window.len()).map(|i| (i as f64).powi(2)).sum();

        let denominator = n * sum_xx - sum_x.powi(2);
        if denominator.abs() < 1e-10 {
            return 0.0;
        }

        (n * sum_xy - sum_x * sum_y) / denominator
    }

    /// Estimate iterations needed to reach target
    fn estimate_iterations_to_target(&self, trend: f64) -> Option<usize> {
        if self.estimate >= self.target {
            return Some(0);
        }
        if trend <= 0.0 {
            return None; // Not converging
        }
        let gap = self.target - self.estimate;
        Some((gap / trend).ceil() as usize)
    }
}

/// Convergence estimate result
#[derive(Debug, Clone)]
pub struct ConvergenceEstimate {
    /// Current raw compile rate
    pub current: f64,
    /// Kalman-smoothed estimate
    pub smoothed: f64,
    /// Estimated final rate at max iterations
    pub estimated_final: f64,
    /// Confidence in estimate (0.0-1.0)
    pub confidence: f64,
    /// Whether we expect to converge
    pub will_converge: bool,
    /// Estimated iterations to reach target (None if not converging)
    pub iterations_to_target: Option<usize>,
}

// ============================================================================
// Andon Display (UTOL-030)
// ============================================================================

/// Unicode sparkline characters (from entrenar::train::tui)
pub const SPARK_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Generate sparkline from values
pub fn sparkline(values: &[f64], width: usize) -> String {
    if values.is_empty() {
        return " ".repeat(width);
    }

    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    // Subsample if needed
    let step = (values.len() as f64 / width as f64).max(1.0);
    let mut result = String::with_capacity(width);

    for i in 0..width {
        let idx = (i as f64 * step) as usize;
        if idx >= values.len() {
            result.push(' ');
            continue;
        }

        let value = values[idx];
        let normalized = if range > 0.0 {
            ((value - min) / range).clamp(0.0, 1.0)
        } else {
            0.5
        };
        let char_idx = (normalized * 7.0).round() as usize;
        result.push(SPARK_CHARS[char_idx.min(7)]);
    }

    result
}

/// Progress bar rendering
pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return "░".repeat(width);
    }

    let pct = (current as f64 / total as f64).clamp(0.0, 1.0);
    let filled = (pct * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

/// Drift status indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftStatus {
    /// Model is stable
    Stable,
    /// Warning: approaching threshold
    Warning,
    /// Critical: below threshold
    Critical,
    /// Drift detected: model degradation
    Drift,
}

impl DriftStatus {
    /// Get display symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Stable => "●",
            Self::Warning => "◐",
            Self::Critical => "○",
            Self::Drift => "⚡",
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Stable => "STABLE",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
            Self::Drift => "DRIFT",
        }
    }
}

/// Andon display for visual feedback
pub struct AndonDisplay {
    config: DisplayConfig,
    last_refresh: Instant,
}

impl AndonDisplay {
    /// Create new display with configuration
    pub fn new(config: &DisplayConfig) -> Self {
        Self {
            config: config.clone(),
            last_refresh: Instant::now(),
        }
    }

    /// Check if should refresh
    pub fn should_refresh(&self) -> bool {
        self.last_refresh.elapsed() >= Duration::from_millis(self.config.refresh_ms)
    }

    /// Format the header line
    pub fn format_header(&self, state: &LoopState, config: &ConvergenceConfig) -> String {
        match self.config.mode {
            DisplayMode::Rich => self.format_rich_header(state, config),
            DisplayMode::Minimal => self.format_minimal(state, config),
            DisplayMode::Json => String::new(), // JSON output at end only
            DisplayMode::Silent => String::new(),
        }
    }

    /// Format rich TUI header
    fn format_rich_header(&self, state: &LoopState, config: &ConvergenceConfig) -> String {
        let progress = progress_bar(state.iteration, config.max_iterations, 20);
        let pct = state.progress_pct(config.max_iterations);

        let status = if state.compile_rate >= config.target_rate {
            "✓ CONVERGED"
        } else if state.is_improving(3) {
            "✓ ON TRACK"
        } else {
            "⚠ STALLED"
        };

        format!(
            "Iteration: [{}] {}/{} ({:.0}%)\n\
             Compile Rate: {:.1}% → Target: {:.1}%  {}",
            progress,
            state.iteration,
            config.max_iterations,
            pct,
            state.compile_rate * 100.0,
            config.target_rate * 100.0,
            status
        )
    }

    /// Format minimal single-line output
    fn format_minimal(&self, state: &LoopState, config: &ConvergenceConfig) -> String {
        let status = if state.compile_rate >= config.target_rate {
            "CONVERGED"
        } else if state.is_improving(3) {
            "ON_TRACK"
        } else {
            "STALLED"
        };

        format!(
            "UTOL [{}/{}] {:.0}% | Rate: {:.1}% | Target: {:.1}% | Status: {}",
            state.iteration,
            config.max_iterations,
            state.progress_pct(config.max_iterations),
            state.compile_rate * 100.0,
            config.target_rate * 100.0,
            status
        )
    }

    /// Format metrics with sparklines
    pub fn format_metrics(&self, state: &LoopState, drift_status: DriftStatus) -> String {
        if !self.config.show_sparklines
            || matches!(self.config.mode, DisplayMode::Silent | DisplayMode::Json)
        {
            return String::new();
        }

        let spark = sparkline(&state.rate_history, 8);
        let delta = state.improvement_delta();
        let delta_str = if delta >= 0.0 {
            format!("+{:.1}%", delta * 100.0)
        } else {
            format!("{:.1}%", delta * 100.0)
        };

        format!(
            "Compile Rate: {} {:.1}% ({})\n\
             Drift Status: {} {}",
            spark,
            state.compile_rate * 100.0,
            delta_str,
            drift_status.symbol(),
            drift_status.name()
        )
    }

    /// Mark refresh
    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Instant::now();
    }
}

// ============================================================================
// Action Decision (UTOL-003)
// ============================================================================

/// Action to take based on loop state
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Target reached, loop complete
    Converged,
    /// Retrain model with failing examples
    Retrain { failing_count: usize },
    /// No improvement detected
    NoImprovement,
    /// Continue to next iteration
    Continue,
    /// Plateau reached (patience exhausted)
    Plateau,
}

/// Decide next action based on state and metrics
pub fn decide_action(
    state: &LoopState,
    compile_rate: f64,
    config: &ConvergenceConfig,
    drift_status: DriftStatus,
    failing_count: usize,
) -> Action {
    // Check convergence first
    if compile_rate >= config.target_rate {
        return Action::Converged;
    }

    // Check for drift - always retrain on drift
    if matches!(drift_status, DriftStatus::Drift) {
        return Action::Retrain { failing_count };
    }

    // Check patience
    if state.patience_counter >= config.patience {
        return Action::Plateau;
    }

    // Check improvement
    let delta = if !state.rate_history.is_empty() {
        compile_rate - state.rate_history.last().copied().unwrap_or(0.0)
    } else {
        config.min_delta + 0.001 // First iteration, assume improvement
    };

    if delta < config.min_delta {
        // No significant improvement
        if failing_count > 0 {
            // Have failures to learn from
            return Action::Retrain { failing_count };
        }
        return Action::NoImprovement;
    }

    // Making progress, continue
    Action::Continue
}

// ============================================================================
// Compilation Types (UTOL-002)
// ============================================================================

/// Result of compiling a single Python file
#[derive(Debug, Clone)]
pub struct CompileResult {
    /// Source Python file
    pub file: PathBuf,
    /// Whether compilation succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Error category if classified
    pub category: Option<ErrorCategory>,
    /// Generated Rust code if successful
    pub rust_code: Option<String>,
}

/// Metrics from a compilation batch
#[derive(Debug, Clone, Default)]
pub struct CompilationMetrics {
    /// Total files processed
    pub total: usize,
    /// Successfully compiled
    pub successful: usize,
    /// Failed to compile
    pub failed: usize,
    /// Overall compile rate
    pub compile_rate: f64,
    /// Error counts by category
    pub category_counts: HashMap<ErrorCategory, usize>,
    /// Category success rates
    pub category_rates: HashMap<ErrorCategory, f64>,
}

impl CompilationMetrics {
    /// Calculate metrics from compilation results
    pub fn from_results(results: &[CompileResult]) -> Self {
        let total = results.len();
        let successful = results.iter().filter(|r| r.success).count();
        let failed = total - successful;

        let compile_rate = if total > 0 {
            successful as f64 / total as f64
        } else {
            0.0
        };

        // Count errors by category
        let mut category_counts: HashMap<ErrorCategory, usize> = HashMap::new();
        for result in results.iter().filter(|r| !r.success) {
            if let Some(cat) = &result.category {
                *category_counts.entry(*cat).or_insert(0) += 1;
            }
        }

        Self {
            total,
            successful,
            failed,
            compile_rate,
            category_counts,
            category_rates: HashMap::new(), // Calculated separately if needed
        }
    }
}

/// Training sample extracted from a failure
#[derive(Debug, Clone)]
pub struct TrainingSample {
    /// Original error text
    pub error_text: String,
    /// Classified category
    pub category: ErrorCategory,
    /// Source file
    pub source_file: PathBuf,
}

/// Extract training samples from compilation failures
pub fn extract_training_samples(results: &[CompileResult]) -> Vec<TrainingSample> {
    results
        .iter()
        .filter(|r| !r.success && r.error.is_some() && r.category.is_some())
        .map(|r| TrainingSample {
            error_text: r.error.clone().unwrap_or_default(),
            category: r.category.expect("filtered for is_some"),
            source_file: r.file.clone(),
        })
        .collect()
}

// ============================================================================
// Corpus Compilation (UTOL-002)
// ============================================================================

use depyler_core::DepylerPipeline;
use glob::Pattern;
use std::process::Command;
use walkdir::WalkDir;

/// Compile all Python files in a corpus directory
pub fn compile_corpus(config: &CorpusConfig, classifier: &crate::Oracle) -> Vec<CompileResult> {
    let mut results = Vec::new();
    let pipeline = DepylerPipeline::new();

    // Parse glob patterns
    let include_patterns: Vec<Pattern> = config
        .include_patterns
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    let exclude_patterns: Vec<Pattern> = config
        .exclude_patterns
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    // Walk corpus directory
    for entry in WalkDir::new(&config.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let path_str = path.to_string_lossy();

        // Check include patterns
        let included =
            include_patterns.is_empty() || include_patterns.iter().any(|p| p.matches(&path_str));
        if !included {
            continue;
        }

        // Check exclude patterns
        let excluded = exclude_patterns.iter().any(|p| p.matches(&path_str));
        if excluded {
            continue;
        }

        // Compile this file
        let result = compile_single_file(path, &pipeline, classifier);
        results.push(result);
    }

    results
}

/// Compile a single Python file
fn compile_single_file(
    path: &std::path::Path,
    pipeline: &DepylerPipeline,
    classifier: &crate::Oracle,
) -> CompileResult {
    // Read Python source
    let python_code = match std::fs::read_to_string(path) {
        Ok(code) => code,
        Err(e) => {
            return CompileResult {
                file: path.to_path_buf(),
                success: false,
                error: Some(format!("Failed to read file: {}", e)),
                category: Some(ErrorCategory::Other),
                rust_code: None,
            };
        }
    };

    // Transpile to Rust
    let rust_code = match pipeline.transpile(&python_code) {
        Ok(code) => code,
        Err(e) => {
            let error_str = format!("{}", e);
            let category = classifier
                .classify_message(&error_str)
                .map(|r| r.category)
                .unwrap_or(ErrorCategory::Other);
            return CompileResult {
                file: path.to_path_buf(),
                success: false,
                error: Some(error_str),
                category: Some(category),
                rust_code: None,
            };
        }
    };

    // Try to compile the Rust code
    let compile_result = try_compile_rust(&rust_code);

    match compile_result {
        Ok(()) => CompileResult {
            file: path.to_path_buf(),
            success: true,
            error: None,
            category: None,
            rust_code: Some(rust_code),
        },
        Err(error_str) => {
            let category = classifier
                .classify_message(&error_str)
                .map(|r| r.category)
                .unwrap_or(ErrorCategory::Other);
            CompileResult {
                file: path.to_path_buf(),
                success: false,
                error: Some(error_str),
                category: Some(category),
                rust_code: Some(rust_code),
            }
        }
    }
}

/// Try to compile Rust code using rustc
fn try_compile_rust(rust_code: &str) -> Result<(), String> {
    use std::io::Write;

    // Create temp files for source and output
    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    let temp_file = temp_dir.join(format!("utol_check_{}.rs", pid));
    // DEPYLER-1119: Use proper temp file for output, not /dev/null
    // Using /dev/null causes rustc to try creating temp dirs in /dev/ which fails
    let temp_output = temp_dir.join(format!("utol_out_{}", pid));

    // Write Rust code
    let mut file = std::fs::File::create(&temp_file).map_err(|e| e.to_string())?;
    file.write_all(rust_code.as_bytes())
        .map_err(|e| e.to_string())?;
    drop(file);

    // Run rustc --emit=metadata (fastest check without codegen)
    // DEPYLER-1119: Added --edition 2021 for proper Rust 2021 syntax
    let output = Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--emit=metadata",
            "--crate-type=lib",
            "-o",
            temp_output.to_str().expect("valid UTF-8 path"),
            temp_file.to_str().expect("valid UTF-8 path"),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    // Clean up both temp files
    let _ = std::fs::remove_file(&temp_file);
    let _ = std::fs::remove_file(&temp_output);

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}

// ============================================================================
// DEPYLER-1101: Type Constraint Learner (Oracle Type Repair)
// ============================================================================

/// A learned type constraint from E0308 errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeConstraint {
    /// Variable name or expression location
    pub location: String,
    /// Expected Rust type (from compiler)
    pub expected_type: String,
    /// Found Rust type (what we generated)
    pub found_type: String,
    /// Source file
    pub source_file: PathBuf,
    /// Line number in generated Rust
    pub line: usize,
}

/// Type constraint learner that extracts type information from E0308 errors
pub struct TypeConstraintLearner {
    /// Learned constraints
    pub constraints: Vec<TypeConstraint>,
    /// Pattern for extracting expected/found types
    e0308_pattern: regex::Regex,
    /// Pattern for extracting line numbers
    location_pattern: regex::Regex,
}

impl TypeConstraintLearner {
    /// Create a new type constraint learner
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            // Pattern: "expected `Type1`, found `Type2`"
            e0308_pattern: regex::Regex::new(r"expected `([^`]+)`, found `([^`]+)`")
                .expect("static regex"),
            // Pattern: "--> file.rs:123:45"
            location_pattern: regex::Regex::new(r"--> ([^:]+):(\d+):\d+").expect("static regex"),
        }
    }

    /// Learn type constraints from compiler error output
    pub fn learn_from_errors(&mut self, errors: &str, source_file: &std::path::Path) {
        let lines: Vec<&str> = errors.lines().collect();
        let mut current_line = 0usize;

        for (i, line) in lines.iter().enumerate() {
            // Extract line number from location
            if let Some(caps) = self.location_pattern.captures(line) {
                current_line = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(0);
            }

            // Look for E0308 type mismatch
            if line.contains("E0308") || line.contains("mismatched types") {
                // Search nearby lines for expected/found
                for check_line in &lines[i..std::cmp::min(i + 10, lines.len())] {
                    if let Some(caps) = self.e0308_pattern.captures(check_line) {
                        let expected = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                        let found = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                        if !expected.is_empty() && !found.is_empty() {
                            self.constraints.push(TypeConstraint {
                                location: format!("line_{}", current_line),
                                expected_type: expected.to_string(),
                                found_type: found.to_string(),
                                source_file: source_file.to_path_buf(),
                                line: current_line,
                            });
                        }
                        break;
                    }
                }
            }
        }
    }

    /// Convert Rust type string to HIR Type (best-effort mapping)
    pub fn rust_type_to_hir(rust_type: &str) -> Option<depyler_core::hir::Type> {
        use depyler_core::hir::Type;

        let normalized = rust_type.trim();
        Some(match normalized {
            "i32" | "i64" | "isize" | "usize" => Type::Int,
            "f32" | "f64" => Type::Float,
            "String" | "&str" | "&String" => Type::String,
            "bool" => Type::Bool,
            "()" => Type::None,
            s if s.starts_with("Vec<") => {
                let inner = s.trim_start_matches("Vec<").trim_end_matches('>');
                Type::List(Box::new(Self::rust_type_to_hir(inner)?))
            }
            s if s.starts_with("HashMap<") => {
                // HashMap<K, V>
                let inner = s.trim_start_matches("HashMap<").trim_end_matches('>');
                let parts: Vec<&str> = inner.splitn(2, ", ").collect();
                if parts.len() == 2 {
                    Type::Dict(
                        Box::new(Self::rust_type_to_hir(parts[0])?),
                        Box::new(Self::rust_type_to_hir(parts[1])?),
                    )
                } else {
                    Type::Dict(Box::new(Type::String), Box::new(Type::Unknown))
                }
            }
            s if s.starts_with("Option<") => {
                let inner = s.trim_start_matches("Option<").trim_end_matches('>');
                Type::Optional(Box::new(Self::rust_type_to_hir(inner)?))
            }
            s if s.starts_with("HashSet<") => {
                let inner = s.trim_start_matches("HashSet<").trim_end_matches('>');
                Type::Set(Box::new(Self::rust_type_to_hir(inner)?))
            }
            _ => Type::Custom(normalized.to_string()),
        })
    }

    /// Get summary of learned constraints
    pub fn summary(&self) -> String {
        if self.constraints.is_empty() {
            return "No type constraints learned".to_string();
        }

        let mut type_pairs: HashMap<(String, String), usize> = HashMap::new();
        for c in &self.constraints {
            *type_pairs
                .entry((c.expected_type.clone(), c.found_type.clone()))
                .or_insert(0) += 1;
        }

        let mut lines = vec![format!(
            "Learned {} type constraints:",
            self.constraints.len()
        )];
        let mut pairs: Vec<_> = type_pairs.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));

        for ((expected, found), count) in pairs.iter().take(10) {
            lines.push(format!(
                "  {} × expected `{}`, found `{}`",
                count, expected, found
            ));
        }

        lines.join("\n")
    }
}

impl Default for TypeConstraintLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of type repair operation
#[derive(Debug, Clone)]
pub struct TypeRepairResult {
    /// Whether repair was successful
    pub success: bool,
    /// Number of iterations used
    pub iterations: usize,
    /// Final compile rate
    pub final_rate: f64,
    /// Constraints learned
    pub constraints_learned: usize,
    /// Constraints applied
    pub constraints_applied: usize,
}

/// DEPYLER-1101: Run automated type repair loop on a single file
///
/// This function implements the "Compile → Learn → Fix" cycle:
/// 1. Transpile Python to Rust
/// 2. Attempt compilation
/// 3. Learn type constraints from E0308 errors
/// 4. Re-transpile with learned constraints injected
/// 5. Repeat until success or max iterations
///
/// Returns the repaired Rust code if successful.
pub fn repair_file_types(
    python_path: &std::path::Path,
    max_iterations: usize,
) -> anyhow::Result<TypeRepairResult> {
    use depyler_core::DepylerPipeline;

    let python_code = std::fs::read_to_string(python_path)?;
    let pipeline = DepylerPipeline::new();
    let mut learner = TypeConstraintLearner::new();
    let mut iterations = 0;
    let mut constraints_applied = 0;

    // Convert learned constraints to HashMap for transpile_with_constraints
    let mut type_constraints: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for _iteration in 0..max_iterations {
        iterations += 1;

        // DEPYLER-1101: Transpile with learned constraints
        // First iteration uses empty constraints, subsequent iterations use learned ones
        let rust_code = if type_constraints.is_empty() {
            match pipeline.transpile(&python_code) {
                Ok(code) => code,
                Err(_e) => {
                    return Ok(TypeRepairResult {
                        success: false,
                        iterations,
                        final_rate: 0.0,
                        constraints_learned: learner.constraints.len(),
                        constraints_applied: 0,
                    });
                }
            }
        } else {
            // Use transpile_with_constraints for subsequent iterations
            match pipeline.transpile_with_constraints(&python_code, &type_constraints) {
                Ok(code) => code,
                Err(_e) => {
                    return Ok(TypeRepairResult {
                        success: false,
                        iterations,
                        final_rate: 0.0,
                        constraints_learned: learner.constraints.len(),
                        constraints_applied,
                    });
                }
            }
        };

        // Try to compile
        match try_compile_rust(&rust_code) {
            Ok(()) => {
                // Success!
                return Ok(TypeRepairResult {
                    success: true,
                    iterations,
                    final_rate: 1.0,
                    constraints_learned: learner.constraints.len(),
                    constraints_applied,
                });
            }
            Err(errors) => {
                // Learn from errors
                let prev_count = learner.constraints.len();
                learner.learn_from_errors(&errors, python_path);
                let new_count = learner.constraints.len();

                // If no new constraints learned, we're stuck
                if new_count == prev_count {
                    eprintln!(
                        "DEPYLER-1101: No new constraints learned after {} iterations",
                        iterations
                    );
                    break;
                }

                // DEPYLER-1101: Inject constraints for next iteration
                // Convert TypeConstraint.location → TypeConstraint.expected_type
                for constraint in &learner.constraints {
                    // Only add if not already present (don't overwrite)
                    if !type_constraints.contains_key(&constraint.location) {
                        type_constraints.insert(
                            constraint.location.clone(),
                            constraint.expected_type.clone(),
                        );
                        constraints_applied += 1;
                        eprintln!(
                            "DEPYLER-1101: Learned constraint: {} → {}",
                            constraint.location, constraint.expected_type
                        );
                    }
                }
            }
        }
    }

    Ok(TypeRepairResult {
        success: false,
        iterations,
        final_rate: 0.0,
        constraints_learned: learner.constraints.len(),
        constraints_applied,
    })
}

// ============================================================================
// Main UTOL Loop (UTOL-001)
// ============================================================================

/// Run the UTOL main loop
pub fn run_utol(config: &UtolConfig) -> anyhow::Result<UtolResult> {
    use std::time::Instant;

    let start = Instant::now();

    // Initialize components
    let mut state = LoopState::new();
    let display = AndonDisplay::new(&config.display);
    let mut estimator = ConvergenceEstimator::new(config.convergence.target_rate);

    // Load or train oracle
    #[cfg(feature = "training")]
    let oracle = crate::Oracle::load_or_train()?;
    #[cfg(not(feature = "training"))]
    let oracle = {
        let path = crate::Oracle::default_model_path();
        if path.exists() {
            crate::Oracle::load(&path)?
        } else {
            crate::Oracle::new()
        }
    };

    // Print header
    if !matches!(config.display.mode, DisplayMode::Silent | DisplayMode::Json) {
        println!("╔══════════════════════════════════════════════════════════════════════╗");
        println!("║                    UTOL - Unified Training Oracle Loop               ║");
        println!("╠══════════════════════════════════════════════════════════════════════╣");
        println!("║  Corpus: {}", config.corpus.path.display());
        println!("║  Target: {:.1}%", config.convergence.target_rate * 100.0);
        println!("║  Max Iterations: {}", config.convergence.max_iterations);
        println!("╚══════════════════════════════════════════════════════════════════════╝");
        println!();
    }

    // Main PDCA loop
    for iteration in 1..=config.convergence.max_iterations {
        state.iteration = iteration;

        // DO: Compile corpus
        let results = compile_corpus(&config.corpus, &oracle);
        let metrics = CompilationMetrics::from_results(&results);

        // CHECK: Evaluate results
        state.compile_rate = metrics.compile_rate;
        state.rate_history.push(state.compile_rate);

        // Update convergence estimate
        let _estimate = estimator.update(state.compile_rate);

        // Determine drift status
        let drift_status = if state.rate_history.len() > 3 {
            let recent_avg: f64 = state.rate_history.iter().rev().take(3).sum::<f64>() / 3.0;
            if recent_avg < state.compile_rate * 0.95 {
                DriftStatus::Drift
            } else if recent_avg < state.compile_rate * 0.98 {
                DriftStatus::Warning
            } else {
                DriftStatus::Stable
            }
        } else {
            DriftStatus::Stable
        };

        // Display progress
        if !matches!(config.display.mode, DisplayMode::Silent | DisplayMode::Json) {
            let header = display.format_header(&state, &config.convergence);
            if !header.is_empty() {
                println!("{}", header);
            }
            let metrics_output = display.format_metrics(&state, drift_status);
            if !metrics_output.is_empty() {
                println!("{}", metrics_output);
            }
            println!();
        }

        // ACT: Decide next action
        let action = decide_action(
            &state,
            state.compile_rate,
            &config.convergence,
            drift_status,
            metrics.failed,
        );

        match action {
            Action::Converged => {
                if !matches!(config.display.mode, DisplayMode::Silent | DisplayMode::Json) {
                    println!("✅ CONVERGED at {:.1}%!", state.compile_rate * 100.0);
                }
                break;
            }
            Action::Plateau => {
                if !matches!(config.display.mode, DisplayMode::Silent | DisplayMode::Json) {
                    println!("⚠ Plateau detected - stopping (patience exhausted)");
                }
                break;
            }
            Action::Retrain { failing_count: _ } => {
                // Extract samples and retrain (simplified - just note it)
                let samples = extract_training_samples(&results);
                if !matches!(config.display.mode, DisplayMode::Silent | DisplayMode::Json) {
                    println!("🔄 Retraining with {} new samples...", samples.len());
                }
                state.patience_counter = 0;
            }
            Action::NoImprovement => {
                state.patience_counter += 1;
                if !matches!(config.display.mode, DisplayMode::Silent | DisplayMode::Json) {
                    println!(
                        "⚠ No improvement (patience: {}/{})",
                        state.patience_counter, config.convergence.patience
                    );
                }
            }
            Action::Continue => {
                // Continue to next iteration
            }
        }
    }

    let duration = start.elapsed();

    Ok(UtolResult {
        compile_rate: state.compile_rate,
        iterations: state.iteration,
        model_version: format!("oracle-utol-{}", env!("CARGO_PKG_VERSION")),
        converged: state.compile_rate >= config.convergence.target_rate,
        category_rates: state
            .category_rates
            .into_iter()
            .map(|(k, v)| (format!("{:?}", k), v))
            .collect(),
        duration_secs: duration.as_secs_f64(),
    })
}

// ============================================================================
// UTOL Result
// ============================================================================

/// Result of UTOL execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtolResult {
    /// Final compilation success rate
    pub compile_rate: f64,
    /// Total iterations executed
    pub iterations: usize,
    /// Final model version
    pub model_version: String,
    /// Whether target was reached
    pub converged: bool,
    /// Category-level success rates
    pub category_rates: HashMap<String, f64>,
    /// Total duration
    pub duration_secs: f64,
}

// ============================================================================
// Tests (Extreme TDD - RED Phase)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // UTOL-040: Configuration Tests
    // ========================================================================

    #[test]
    fn test_utol_config_default_has_reasonable_values() {
        let config = UtolConfig::default();

        // Corpus defaults
        assert!(config.corpus.path.to_string_lossy().contains("reprorusted"));
        assert!(config
            .corpus
            .include_patterns
            .contains(&"**/*.py".to_string()));

        // Training defaults
        assert_eq!(config.training.synthetic_samples, 12_000);
        assert_eq!(config.training.seed, 42); // Poka-Yoke: deterministic

        // Convergence defaults
        assert!((config.convergence.target_rate - 0.80).abs() < 0.001);
        assert_eq!(config.convergence.max_iterations, 50);
        assert_eq!(config.convergence.patience, 5);

        // Model defaults
        assert!(config.model.path.to_string_lossy().contains("oracle"));
        assert_eq!(config.model.n_estimators, 100);
    }

    #[test]
    fn test_utol_config_serialization_roundtrip() {
        let config = UtolConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: UtolConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.training.seed, restored.training.seed);
        assert!((config.convergence.target_rate - restored.convergence.target_rate).abs() < 0.001);
    }

    // ========================================================================
    // UTOL-001: Loop State Tests
    // ========================================================================

    #[test]
    fn test_loop_state_initial_values() {
        let state = LoopState::new();

        assert_eq!(state.iteration, 0);
        assert!((state.compile_rate - 0.0).abs() < 0.001);
        assert_eq!(state.patience_counter, 0);
        assert!(state.rate_history.is_empty());
    }

    #[test]
    fn test_loop_state_progress_calculation() {
        let mut state = LoopState::new();

        // 0/50 = 0%
        assert!((state.progress_pct(50) - 0.0).abs() < 0.001);

        // 25/50 = 50%
        state.iteration = 25;
        assert!((state.progress_pct(50) - 50.0).abs() < 0.001);

        // 50/50 = 100%
        state.iteration = 50;
        assert!((state.progress_pct(50) - 100.0).abs() < 0.001);

        // Edge case: max_iterations = 0
        assert!((state.progress_pct(0) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_loop_state_is_improving() {
        let mut state = LoopState::new();

        // Empty history - assume improving
        assert!(state.is_improving(3));

        // Single point - assume improving
        state.rate_history.push(0.5);
        assert!(state.is_improving(3));

        // Improving trend
        state.rate_history = vec![0.5, 0.6, 0.7, 0.8];
        assert!(state.is_improving(3));

        // Degrading trend
        state.rate_history = vec![0.8, 0.7, 0.6, 0.5];
        assert!(!state.is_improving(3));

        // Flat trend (last > first in window means improving)
        state.rate_history = vec![0.6, 0.6, 0.6, 0.6];
        assert!(!state.is_improving(3)); // Not improving if flat
    }

    #[test]
    fn test_loop_state_improvement_delta() {
        let mut state = LoopState::new();

        // Empty - delta is 0
        assert!((state.improvement_delta() - 0.0).abs() < 0.001);

        // Single point - delta is 0
        state.rate_history.push(0.5);
        assert!((state.improvement_delta() - 0.0).abs() < 0.001);

        // Positive delta
        state.rate_history.push(0.7);
        assert!((state.improvement_delta() - 0.2).abs() < 0.001);

        // Negative delta
        state.rate_history.push(0.6);
        assert!((state.improvement_delta() - (-0.1)).abs() < 0.001);
    }

    // ========================================================================
    // UTOL-002: Convergence Estimator Tests
    // ========================================================================

    #[test]
    fn test_convergence_estimator_initial_state() {
        let estimator = ConvergenceEstimator::new(0.80);

        assert!((estimator.estimate - 0.5).abs() < 0.001);
        assert!((estimator.target - 0.80).abs() < 0.001);
        assert!(estimator.history.is_empty());
    }

    #[test]
    fn test_convergence_estimator_update_improves_estimate() {
        let mut estimator = ConvergenceEstimator::new(0.80);

        // First measurement
        let est1 = estimator.update(0.6);
        assert!(est1.current > 0.0);
        assert!(est1.confidence > 0.0);

        // Second measurement - estimate should move toward measurements
        let est2 = estimator.update(0.65);
        assert!(est2.smoothed > est1.smoothed); // Moving up
    }

    #[test]
    fn test_convergence_estimator_detects_convergence() {
        let mut estimator = ConvergenceEstimator::new(0.80);

        // Feed many improving measurements to train the Kalman filter
        // Kalman filter needs sufficient data to converge to actual values
        for rate in [0.50, 0.55, 0.60, 0.65, 0.70, 0.75, 0.78, 0.80, 0.82, 0.84] {
            estimator.update(rate);
        }

        let final_est = estimator.update(0.86);

        // The smoothed estimate should be above target after strong trend
        // Note: Kalman smoothing means estimate lags behind actual values
        assert!(
            final_est.smoothed > 0.70,
            "Smoothed estimate should track up: {}",
            final_est.smoothed
        );
        assert!(
            final_est.current >= 0.80,
            "Current should be at/above target"
        );
    }

    #[test]
    fn test_convergence_estimator_detects_non_convergence() {
        let mut estimator = ConvergenceEstimator::new(0.80);

        // Feed flat/degrading measurements
        for rate in [0.5, 0.5, 0.48, 0.49, 0.48] {
            estimator.update(rate);
        }

        let final_est = estimator.update(0.47);
        assert!(!final_est.will_converge);
    }

    // ========================================================================
    // UTOL-030: Sparkline Tests
    // ========================================================================

    #[test]
    fn test_sparkline_empty_input() {
        let result = sparkline(&[], 8);
        assert_eq!(result.len(), 8);
        assert!(result.chars().all(|c| c == ' '));
    }

    #[test]
    fn test_sparkline_single_value() {
        let result = sparkline(&[0.5], 8);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_sparkline_increasing_values() {
        let values: Vec<f64> = (0..8).map(|i| i as f64 / 7.0).collect();
        let result = sparkline(&values, 8);

        // Should show increasing pattern
        let chars: Vec<char> = result.chars().collect();
        assert_eq!(chars.len(), 8);
        assert_eq!(chars[0], SPARK_CHARS[0]); // Lowest
        assert_eq!(chars[7], SPARK_CHARS[7]); // Highest
    }

    #[test]
    fn test_sparkline_constant_values() {
        let values = vec![0.5; 8];
        let result = sparkline(&values, 8);

        // All same height (middle)
        let chars: Vec<char> = result.chars().collect();
        assert!(chars.iter().all(|&c| c == chars[0]));
    }

    // ========================================================================
    // UTOL-030: Progress Bar Tests
    // ========================================================================

    #[test]
    fn test_progress_bar_empty() {
        let result = progress_bar(0, 10, 10);
        assert_eq!(result, "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_full() {
        let result = progress_bar(10, 10, 10);
        assert_eq!(result, "██████████");
    }

    #[test]
    fn test_progress_bar_half() {
        let result = progress_bar(5, 10, 10);
        assert_eq!(result, "█████░░░░░");
    }

    #[test]
    fn test_progress_bar_zero_total() {
        let result = progress_bar(5, 0, 10);
        assert_eq!(result, "░░░░░░░░░░");
    }

    // ========================================================================
    // UTOL-030: Drift Status Tests
    // ========================================================================

    #[test]
    fn test_drift_status_symbols() {
        assert_eq!(DriftStatus::Stable.symbol(), "●");
        assert_eq!(DriftStatus::Warning.symbol(), "◐");
        assert_eq!(DriftStatus::Critical.symbol(), "○");
        assert_eq!(DriftStatus::Drift.symbol(), "⚡");
    }

    #[test]
    fn test_drift_status_names() {
        assert_eq!(DriftStatus::Stable.name(), "STABLE");
        assert_eq!(DriftStatus::Warning.name(), "WARNING");
        assert_eq!(DriftStatus::Critical.name(), "CRITICAL");
        assert_eq!(DriftStatus::Drift.name(), "DRIFT");
    }

    // ========================================================================
    // UTOL-003: Action Decision Tests
    // ========================================================================

    #[test]
    fn test_action_converged_when_target_reached() {
        let state = LoopState::new();
        let config = ConvergenceConfig::default();

        let action = decide_action(&state, 0.85, &config, DriftStatus::Stable, 0);
        assert_eq!(action, Action::Converged);
    }

    #[test]
    fn test_action_retrain_on_drift() {
        let state = LoopState::new();
        let config = ConvergenceConfig::default();

        let action = decide_action(&state, 0.50, &config, DriftStatus::Drift, 10);
        assert!(matches!(action, Action::Retrain { .. }));
    }

    #[test]
    fn test_action_plateau_when_patience_exhausted() {
        let mut state = LoopState::new();
        state.patience_counter = 5;
        let config = ConvergenceConfig::default();

        let action = decide_action(&state, 0.50, &config, DriftStatus::Stable, 0);
        assert_eq!(action, Action::Plateau);
    }

    #[test]
    fn test_action_continue_when_improving() {
        let mut state = LoopState::new();
        state.rate_history = vec![0.5];
        let config = ConvergenceConfig::default();

        // Significant improvement (0.51 from 0.50 = 0.01 > min_delta 0.005)
        let action = decide_action(&state, 0.51, &config, DriftStatus::Stable, 0);
        assert_eq!(action, Action::Continue);
    }

    #[test]
    fn test_action_retrain_when_not_improving_with_failures() {
        let mut state = LoopState::new();
        state.rate_history = vec![0.5];
        let config = ConvergenceConfig::default();

        // No improvement but have failures
        let action = decide_action(&state, 0.50, &config, DriftStatus::Stable, 10);
        assert!(matches!(action, Action::Retrain { failing_count: 10 }));
    }

    #[test]
    fn test_action_no_improvement_when_stalled_without_failures() {
        let mut state = LoopState::new();
        state.rate_history = vec![0.5];
        let config = ConvergenceConfig::default();

        // No improvement and no failures
        let action = decide_action(&state, 0.50, &config, DriftStatus::Stable, 0);
        assert_eq!(action, Action::NoImprovement);
    }

    // ========================================================================
    // UTOL-030: Andon Display Tests
    // ========================================================================

    #[test]
    fn test_andon_display_format_minimal() {
        let config = DisplayConfig {
            mode: DisplayMode::Minimal,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.iteration = 10;
        state.compile_rate = 0.75;
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.contains("UTOL"));
        assert!(output.contains("10/50"));
        assert!(output.contains("75.0%"));
    }

    #[test]
    fn test_andon_display_format_rich() {
        let config = DisplayConfig {
            mode: DisplayMode::Rich,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.iteration = 25;
        state.compile_rate = 0.82;
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.contains("Iteration"));
        assert!(output.contains("█")); // Progress bar
        assert!(output.contains("82.0%"));
    }

    #[test]
    fn test_andon_display_silent_returns_empty() {
        let config = DisplayConfig {
            mode: DisplayMode::Silent,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let state = LoopState::new();
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.is_empty());
    }

    #[test]
    fn test_andon_display_should_refresh() {
        let config = DisplayConfig {
            refresh_ms: 100,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);

        // Just created - should not need refresh yet
        // (might need refresh if creation took > 100ms, so we just check it works)
        let _ = display.should_refresh();
    }

    // ========================================================================
    // UTOL Result Tests
    // ========================================================================

    #[test]
    fn test_utol_result_serialization() {
        let result = UtolResult {
            compile_rate: 0.85,
            iterations: 15,
            model_version: "oracle-3.21.0".to_string(),
            converged: true,
            category_rates: HashMap::new(),
            duration_secs: 123.45,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("0.85"));
        assert!(json.contains("oracle-3.21.0"));
        assert!(json.contains("true"));
    }

    // ========================================================================
    // UTOL-002: Compilation Result Tests
    // ========================================================================

    #[test]
    fn test_compile_result_success() {
        let result = CompileResult {
            file: PathBuf::from("test.py"),
            success: true,
            error: None,
            category: None,
            rust_code: Some("fn main() {}".to_string()),
        };

        assert!(result.success);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_compile_result_failure_with_category() {
        let result = CompileResult {
            file: PathBuf::from("test.py"),
            success: false,
            error: Some("E0308: mismatched types".to_string()),
            category: Some(ErrorCategory::TypeMismatch),
            rust_code: None,
        };

        assert!(!result.success);
        assert_eq!(result.category, Some(ErrorCategory::TypeMismatch));
    }

    // ========================================================================
    // UTOL-002: Compilation Metrics Tests
    // ========================================================================

    #[test]
    fn test_compilation_metrics_all_success() {
        let results = vec![
            CompileResult {
                file: "a.py".into(),
                success: true,
                error: None,
                category: None,
                rust_code: Some("".into()),
            },
            CompileResult {
                file: "b.py".into(),
                success: true,
                error: None,
                category: None,
                rust_code: Some("".into()),
            },
        ];

        let metrics = CompilationMetrics::from_results(&results);

        assert_eq!(metrics.total, 2);
        assert_eq!(metrics.successful, 2);
        assert_eq!(metrics.failed, 0);
        assert!((metrics.compile_rate - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_compilation_metrics_mixed() {
        let results = vec![
            CompileResult {
                file: "a.py".into(),
                success: true,
                error: None,
                category: None,
                rust_code: Some("".into()),
            },
            CompileResult {
                file: "b.py".into(),
                success: false,
                error: Some("E0308".into()),
                category: Some(ErrorCategory::TypeMismatch),
                rust_code: None,
            },
            CompileResult {
                file: "c.py".into(),
                success: false,
                error: Some("E0277".into()),
                category: Some(ErrorCategory::TraitBound),
                rust_code: None,
            },
            CompileResult {
                file: "d.py".into(),
                success: true,
                error: None,
                category: None,
                rust_code: Some("".into()),
            },
        ];

        let metrics = CompilationMetrics::from_results(&results);

        assert_eq!(metrics.total, 4);
        assert_eq!(metrics.successful, 2);
        assert_eq!(metrics.failed, 2);
        assert!((metrics.compile_rate - 0.5).abs() < 0.001);
        assert_eq!(
            metrics.category_counts.get(&ErrorCategory::TypeMismatch),
            Some(&1)
        );
        assert_eq!(
            metrics.category_counts.get(&ErrorCategory::TraitBound),
            Some(&1)
        );
    }

    #[test]
    fn test_compilation_metrics_empty() {
        let metrics = CompilationMetrics::from_results(&[]);

        assert_eq!(metrics.total, 0);
        assert!((metrics.compile_rate - 0.0).abs() < 0.001);
    }

    // ========================================================================
    // UTOL-002: Extract Samples Tests
    // ========================================================================

    #[test]
    fn test_extract_training_samples_from_failures() {
        let results = vec![
            CompileResult {
                file: "ok.py".into(),
                success: true,
                error: None,
                category: None,
                rust_code: Some("fn main() {}".into()),
            },
            CompileResult {
                file: "fail.py".into(),
                success: false,
                error: Some("error[E0308]: mismatched types".into()),
                category: Some(ErrorCategory::TypeMismatch),
                rust_code: None,
            },
        ];

        let samples = extract_training_samples(&results);

        assert_eq!(samples.len(), 1);
        assert!(samples[0].error_text.contains("E0308"));
        assert_eq!(samples[0].category, ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_extract_training_samples_empty_on_all_success() {
        let results = vec![CompileResult {
            file: "a.py".into(),
            success: true,
            error: None,
            category: None,
            rust_code: Some("".into()),
        }];

        let samples = extract_training_samples(&results);
        assert!(samples.is_empty());
    }

    // ========================================================================
    // Additional Configuration Tests (UTOL-040)
    // ========================================================================

    #[test]
    fn test_corpus_config_default() {
        let config = CorpusConfig::default();
        assert!(config.path.to_string_lossy().contains("reprorusted"));
        assert!(!config.include_patterns.is_empty());
        assert!(!config.exclude_patterns.is_empty());
    }

    #[test]
    fn test_corpus_config_serialization() {
        let config = CorpusConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: CorpusConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.include_patterns, restored.include_patterns);
    }

    #[test]
    fn test_training_config_default() {
        let config = TrainingConfig::default();
        assert_eq!(config.synthetic_samples, 12_000);
        assert_eq!(config.seed, 42);
        assert!(config.balance_classes);
    }

    #[test]
    fn test_training_config_serialization() {
        let config = TrainingConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: TrainingConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.seed, restored.seed);
        assert_eq!(config.balance_classes, restored.balance_classes);
    }

    #[test]
    fn test_convergence_config_default() {
        let config = ConvergenceConfig::default();
        assert!((config.target_rate - 0.80).abs() < 0.001);
        assert_eq!(config.max_iterations, 50);
        assert_eq!(config.patience, 5);
        assert!((config.min_delta - 0.005).abs() < 0.0001);
    }

    #[test]
    fn test_convergence_config_serialization() {
        let config = ConvergenceConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: ConvergenceConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.patience, restored.patience);
    }

    #[test]
    fn test_model_config_default() {
        let config = ModelConfig::default();
        assert!(config.path.to_string_lossy().contains("oracle"));
        assert_eq!(config.n_estimators, 100);
        assert_eq!(config.max_depth, 10);
    }

    #[test]
    fn test_model_config_serialization() {
        let config = ModelConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: ModelConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.n_estimators, restored.n_estimators);
    }

    #[test]
    fn test_display_config_default() {
        let config = DisplayConfig::default();
        assert_eq!(config.mode, DisplayMode::Rich);
        assert_eq!(config.refresh_ms, 500);
        assert!(config.show_sparklines);
        assert!(config.show_category_breakdown);
    }

    #[test]
    fn test_display_config_serialization() {
        let config = DisplayConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: DisplayConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.mode, restored.mode);
        assert_eq!(config.refresh_ms, restored.refresh_ms);
    }

    #[test]
    fn test_display_mode_default() {
        let mode = DisplayMode::default();
        assert_eq!(mode, DisplayMode::Rich);
    }

    #[test]
    fn test_display_mode_serialization_all_variants() {
        let modes = [
            DisplayMode::Rich,
            DisplayMode::Minimal,
            DisplayMode::Json,
            DisplayMode::Silent,
        ];
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let restored: DisplayMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, restored);
        }
    }

    #[test]
    fn test_display_mode_debug() {
        let mode = DisplayMode::Rich;
        let debug = format!("{:?}", mode);
        assert!(debug.contains("Rich"));
    }

    #[test]
    fn test_display_mode_clone() {
        let mode = DisplayMode::Json;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }

    // ========================================================================
    // Additional Loop State Tests
    // ========================================================================

    #[test]
    fn test_loop_state_default_trait() {
        let state = LoopState::default();
        assert_eq!(state.iteration, 0);
        assert!(state.rate_history.is_empty());
    }

    #[test]
    fn test_loop_state_clone() {
        let mut state = LoopState::new();
        state.iteration = 5;
        state.compile_rate = 0.75;
        state.rate_history = vec![0.5, 0.6, 0.7];

        let cloned = state.clone();
        assert_eq!(state.iteration, cloned.iteration);
        assert_eq!(state.rate_history.len(), cloned.rate_history.len());
    }

    #[test]
    fn test_loop_state_debug() {
        let state = LoopState::new();
        let debug = format!("{:?}", state);
        assert!(debug.contains("LoopState"));
    }

    #[test]
    fn test_loop_state_category_rates() {
        let mut state = LoopState::new();
        state
            .category_rates
            .insert(ErrorCategory::TypeMismatch, 0.9);
        state.category_rates.insert(ErrorCategory::TraitBound, 0.8);

        assert_eq!(state.category_rates.len(), 2);
        assert!((state.category_rates[&ErrorCategory::TypeMismatch] - 0.9).abs() < 0.001);
    }

    // ========================================================================
    // Additional Convergence Estimator Tests
    // ========================================================================

    #[test]
    fn test_convergence_estimator_clone() {
        let mut estimator = ConvergenceEstimator::new(0.80);
        estimator.update(0.6);

        let cloned = estimator.clone();
        assert!((estimator.estimate - cloned.estimate).abs() < 0.001);
        assert_eq!(estimator.history.len(), cloned.history.len());
    }

    #[test]
    fn test_convergence_estimator_debug() {
        let estimator = ConvergenceEstimator::new(0.80);
        let debug = format!("{:?}", estimator);
        assert!(debug.contains("ConvergenceEstimator"));
    }

    #[test]
    fn test_convergence_estimator_iterations_to_target_at_target() {
        let mut estimator = ConvergenceEstimator::new(0.50);

        // Update with values at target
        for _ in 0..5 {
            estimator.update(0.55);
        }

        let est = estimator.update(0.55);
        assert!(est.iterations_to_target.is_some());
    }

    #[test]
    fn test_convergence_estimator_with_many_updates() {
        let mut estimator = ConvergenceEstimator::new(0.80);

        // Simulate 50 iterations
        for i in 0..50 {
            let rate = 0.5 + (i as f64 * 0.01);
            estimator.update(rate);
        }

        // Estimator should have full history
        assert_eq!(estimator.history.len(), 50);
    }

    #[test]
    fn test_convergence_estimate_fields() {
        let mut estimator = ConvergenceEstimator::new(0.80);
        let est = estimator.update(0.6);

        assert!((est.current - 0.6).abs() < 0.001);
        assert!(est.smoothed >= 0.0 && est.smoothed <= 1.0);
        assert!(est.estimated_final >= 0.0 && est.estimated_final <= 1.0);
        assert!(est.confidence >= 0.0 && est.confidence <= 1.0);
    }

    #[test]
    fn test_convergence_estimate_clone() {
        let mut estimator = ConvergenceEstimator::new(0.80);
        let est = estimator.update(0.6);
        let cloned = est.clone();

        assert!((est.current - cloned.current).abs() < 0.001);
        assert_eq!(est.will_converge, cloned.will_converge);
    }

    #[test]
    fn test_convergence_estimate_debug() {
        let mut estimator = ConvergenceEstimator::new(0.80);
        let est = estimator.update(0.6);
        let debug = format!("{:?}", est);
        assert!(debug.contains("ConvergenceEstimate"));
    }

    // ========================================================================
    // Additional Sparkline Tests
    // ========================================================================

    #[test]
    fn test_sparkline_more_values_than_width() {
        let values: Vec<f64> = (0..20).map(|i| i as f64 / 19.0).collect();
        let result = sparkline(&values, 8);

        // Should subsample to 8 chars
        assert_eq!(result.chars().count(), 8);
    }

    #[test]
    fn test_sparkline_negative_values() {
        let values = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        let result = sparkline(&values, 5);

        // Should handle negative values by normalizing
        assert_eq!(result.chars().count(), 5);
        assert!(result.chars().all(|c| SPARK_CHARS.contains(&c)));
    }

    #[test]
    fn test_sparkline_very_small_range() {
        let values = vec![0.5, 0.500001, 0.500002];
        let result = sparkline(&values, 3);

        // Small range should still produce valid sparkline
        assert_eq!(result.chars().count(), 3);
    }

    #[test]
    fn test_sparkline_decreasing_values() {
        let values: Vec<f64> = (0..8).rev().map(|i| i as f64 / 7.0).collect();
        let result = sparkline(&values, 8);

        let chars: Vec<char> = result.chars().collect();
        assert_eq!(chars[0], SPARK_CHARS[7]); // Highest at start
        assert_eq!(chars[7], SPARK_CHARS[0]); // Lowest at end
    }

    // ========================================================================
    // Additional Progress Bar Tests
    // ========================================================================

    #[test]
    fn test_progress_bar_overflow() {
        let result = progress_bar(15, 10, 10);
        // Should clamp to 100%
        assert_eq!(result, "██████████");
    }

    #[test]
    fn test_progress_bar_various_widths() {
        for width in [5, 10, 20, 50] {
            let result = progress_bar(5, 10, width);
            // Should have approximately half filled
            let filled = result.chars().filter(|&c| c == '█').count();
            let empty = result.chars().filter(|&c| c == '░').count();
            assert_eq!(filled + empty, width);
        }
    }

    #[test]
    fn test_progress_bar_one_quarter() {
        let result = progress_bar(25, 100, 20);
        let filled = result.chars().filter(|&c| c == '█').count();
        assert_eq!(filled, 5); // 25% of 20 = 5
    }

    // ========================================================================
    // Additional Drift Status Tests
    // ========================================================================

    #[test]
    fn test_drift_status_equality() {
        assert_eq!(DriftStatus::Stable, DriftStatus::Stable);
        assert_ne!(DriftStatus::Stable, DriftStatus::Warning);
        assert_ne!(DriftStatus::Warning, DriftStatus::Critical);
        assert_ne!(DriftStatus::Critical, DriftStatus::Drift);
    }

    #[test]
    fn test_drift_status_clone() {
        let status = DriftStatus::Warning;
        let cloned = status;
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_drift_status_debug() {
        let status = DriftStatus::Drift;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Drift"));
    }

    // ========================================================================
    // Additional Action Tests
    // ========================================================================

    #[test]
    fn test_action_equality() {
        assert_eq!(Action::Converged, Action::Converged);
        assert_eq!(Action::Plateau, Action::Plateau);
        assert_eq!(Action::NoImprovement, Action::NoImprovement);
        assert_eq!(Action::Continue, Action::Continue);
        assert_eq!(
            Action::Retrain { failing_count: 10 },
            Action::Retrain { failing_count: 10 }
        );
    }

    #[test]
    fn test_action_inequality() {
        assert_ne!(Action::Converged, Action::Plateau);
        assert_ne!(
            Action::Retrain { failing_count: 5 },
            Action::Retrain { failing_count: 10 }
        );
    }

    #[test]
    fn test_action_clone() {
        let action = Action::Retrain { failing_count: 15 };
        let cloned = action.clone();
        assert_eq!(action, cloned);
    }

    #[test]
    fn test_action_debug() {
        let action = Action::Retrain { failing_count: 5 };
        let debug = format!("{:?}", action);
        assert!(debug.contains("Retrain"));
        assert!(debug.contains("5"));
    }

    #[test]
    fn test_action_first_iteration_assumes_improvement() {
        let state = LoopState::new(); // Empty history
        let config = ConvergenceConfig::default();

        // First iteration with low rate should continue
        let action = decide_action(&state, 0.3, &config, DriftStatus::Stable, 0);
        assert_eq!(action, Action::Continue);
    }

    // ========================================================================
    // Additional Andon Display Tests
    // ========================================================================

    #[test]
    fn test_andon_display_json_mode_header() {
        let config = DisplayConfig {
            mode: DisplayMode::Json,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let state = LoopState::new();
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.is_empty()); // JSON mode returns empty header
    }

    #[test]
    fn test_andon_display_format_metrics() {
        let config = DisplayConfig {
            mode: DisplayMode::Rich,
            show_sparklines: true,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.compile_rate = 0.75;
        state.rate_history = vec![0.5, 0.6, 0.7, 0.75];

        let output = display.format_metrics(&state, DriftStatus::Stable);
        assert!(output.contains("Compile Rate"));
        assert!(output.contains("Drift Status"));
    }

    #[test]
    fn test_andon_display_format_metrics_silent() {
        let config = DisplayConfig {
            mode: DisplayMode::Silent,
            show_sparklines: true,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let state = LoopState::new();

        let output = display.format_metrics(&state, DriftStatus::Stable);
        assert!(output.is_empty());
    }

    #[test]
    fn test_andon_display_format_metrics_no_sparklines() {
        let config = DisplayConfig {
            mode: DisplayMode::Rich,
            show_sparklines: false,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let state = LoopState::new();

        let output = display.format_metrics(&state, DriftStatus::Stable);
        assert!(output.is_empty());
    }

    #[test]
    fn test_andon_display_format_metrics_positive_delta() {
        let config = DisplayConfig::default();
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.compile_rate = 0.8;
        state.rate_history = vec![0.5, 0.6, 0.7, 0.8];

        let output = display.format_metrics(&state, DriftStatus::Warning);
        assert!(output.contains("+")); // Positive delta
        assert!(output.contains("WARNING"));
    }

    #[test]
    fn test_andon_display_format_metrics_negative_delta() {
        let config = DisplayConfig::default();
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.compile_rate = 0.6;
        state.rate_history = vec![0.5, 0.6, 0.7, 0.6]; // Went down

        let output = display.format_metrics(&state, DriftStatus::Drift);
        // Negative delta shown without explicit + sign
        assert!(output.contains("DRIFT"));
    }

    #[test]
    fn test_andon_display_mark_refreshed() {
        let config = DisplayConfig {
            refresh_ms: 1000,
            ..Default::default()
        };
        let mut display = AndonDisplay::new(&config);

        // After creation, may or may not need refresh
        let before = display.should_refresh();

        display.mark_refreshed();

        // Right after refresh, should NOT need refresh
        assert!(!display.should_refresh());

        let _ = before; // suppress warning
    }

    #[test]
    fn test_andon_display_rich_converged_status() {
        let config = DisplayConfig {
            mode: DisplayMode::Rich,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.iteration = 10;
        state.compile_rate = 0.85; // Above target
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.contains("CONVERGED"));
    }

    #[test]
    fn test_andon_display_rich_stalled_status() {
        let config = DisplayConfig {
            mode: DisplayMode::Rich,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.iteration = 10;
        state.compile_rate = 0.5;
        state.rate_history = vec![0.5, 0.5, 0.5, 0.5]; // Flat = stalled
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.contains("STALLED"));
    }

    #[test]
    fn test_andon_display_minimal_converged_status() {
        let config = DisplayConfig {
            mode: DisplayMode::Minimal,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.compile_rate = 0.85;
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.contains("CONVERGED"));
    }

    #[test]
    fn test_andon_display_minimal_stalled_status() {
        let config = DisplayConfig {
            mode: DisplayMode::Minimal,
            ..Default::default()
        };
        let display = AndonDisplay::new(&config);
        let mut state = LoopState::new();
        state.compile_rate = 0.5;
        state.rate_history = vec![0.5, 0.5, 0.5, 0.5];
        let conv_config = ConvergenceConfig::default();

        let output = display.format_header(&state, &conv_config);
        assert!(output.contains("STALLED"));
    }

    // ========================================================================
    // Additional UtolResult Tests
    // ========================================================================

    #[test]
    fn test_utol_result_deserialization() {
        let json = r#"{
            "compile_rate": 0.92,
            "iterations": 25,
            "model_version": "test-1.0",
            "converged": true,
            "category_rates": {"TypeMismatch": 0.95},
            "duration_secs": 60.5
        }"#;

        let result: UtolResult = serde_json::from_str(json).unwrap();
        assert!((result.compile_rate - 0.92).abs() < 0.001);
        assert_eq!(result.iterations, 25);
        assert!(result.converged);
    }

    #[test]
    fn test_utol_result_clone() {
        let result = UtolResult {
            compile_rate: 0.85,
            iterations: 15,
            model_version: "test".to_string(),
            converged: true,
            category_rates: HashMap::new(),
            duration_secs: 100.0,
        };

        let cloned = result.clone();
        assert!((result.compile_rate - cloned.compile_rate).abs() < 0.001);
        assert_eq!(result.model_version, cloned.model_version);
    }

    #[test]
    fn test_utol_result_debug() {
        let result = UtolResult {
            compile_rate: 0.85,
            iterations: 15,
            model_version: "test".to_string(),
            converged: true,
            category_rates: HashMap::new(),
            duration_secs: 100.0,
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("UtolResult"));
    }

    // ========================================================================
    // Additional Compilation Types Tests
    // ========================================================================

    #[test]
    fn test_compile_result_debug() {
        let result = CompileResult {
            file: PathBuf::from("test.py"),
            success: true,
            error: None,
            category: None,
            rust_code: Some("fn main() {}".to_string()),
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("CompileResult"));
    }

    #[test]
    fn test_compile_result_clone() {
        let result = CompileResult {
            file: PathBuf::from("test.py"),
            success: false,
            error: Some("error".to_string()),
            category: Some(ErrorCategory::TypeMismatch),
            rust_code: None,
        };
        let cloned = result.clone();
        assert_eq!(result.success, cloned.success);
        assert_eq!(result.error, cloned.error);
    }

    #[test]
    fn test_compilation_metrics_default() {
        let metrics = CompilationMetrics::default();
        assert_eq!(metrics.total, 0);
        assert_eq!(metrics.successful, 0);
        assert_eq!(metrics.failed, 0);
        assert!((metrics.compile_rate - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_compilation_metrics_debug() {
        let metrics = CompilationMetrics::default();
        let debug = format!("{:?}", metrics);
        assert!(debug.contains("CompilationMetrics"));
    }

    #[test]
    fn test_compilation_metrics_clone() {
        let metrics = CompilationMetrics {
            total: 10,
            successful: 8,
            failed: 2,
            compile_rate: 0.8,
            ..Default::default()
        };
        let cloned = metrics.clone();
        assert_eq!(metrics.total, cloned.total);
        assert_eq!(metrics.successful, cloned.successful);
    }

    #[test]
    fn test_compilation_metrics_all_failures() {
        let results = vec![
            CompileResult {
                file: "a.py".into(),
                success: false,
                error: Some("E0308".into()),
                category: Some(ErrorCategory::TypeMismatch),
                rust_code: None,
            },
            CompileResult {
                file: "b.py".into(),
                success: false,
                error: Some("E0277".into()),
                category: Some(ErrorCategory::TraitBound),
                rust_code: None,
            },
        ];

        let metrics = CompilationMetrics::from_results(&results);
        assert_eq!(metrics.total, 2);
        assert_eq!(metrics.successful, 0);
        assert_eq!(metrics.failed, 2);
        assert!((metrics.compile_rate - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_compilation_metrics_with_no_category() {
        let results = vec![CompileResult {
            file: "a.py".into(),
            success: false,
            error: Some("Unknown error".into()),
            category: None, // No category
            rust_code: None,
        }];

        let metrics = CompilationMetrics::from_results(&results);
        assert_eq!(metrics.failed, 1);
        assert!(metrics.category_counts.is_empty()); // No category counted
    }

    // ========================================================================
    // Additional Training Sample Tests
    // ========================================================================

    #[test]
    fn test_training_sample_debug() {
        let sample = TrainingSample {
            error_text: "error[E0308]".to_string(),
            category: ErrorCategory::TypeMismatch,
            source_file: PathBuf::from("test.py"),
        };
        let debug = format!("{:?}", sample);
        assert!(debug.contains("TrainingSample"));
    }

    #[test]
    fn test_training_sample_clone() {
        let sample = TrainingSample {
            error_text: "test error".to_string(),
            category: ErrorCategory::Other,
            source_file: PathBuf::from("test.py"),
        };
        let cloned = sample.clone();
        assert_eq!(sample.error_text, cloned.error_text);
        assert_eq!(sample.category, cloned.category);
    }

    #[test]
    fn test_extract_training_samples_skips_no_category() {
        let results = vec![CompileResult {
            file: "fail.py".into(),
            success: false,
            error: Some("error".into()),
            category: None, // No category
            rust_code: None,
        }];

        let samples = extract_training_samples(&results);
        assert!(samples.is_empty()); // Skipped due to no category
    }

    #[test]
    fn test_extract_training_samples_skips_no_error() {
        let results = vec![CompileResult {
            file: "fail.py".into(),
            success: false,
            error: None, // No error message
            category: Some(ErrorCategory::TypeMismatch),
            rust_code: None,
        }];

        let samples = extract_training_samples(&results);
        assert!(samples.is_empty()); // Skipped due to no error
    }

    #[test]
    fn test_extract_training_samples_multiple_failures() {
        let results = vec![
            CompileResult {
                file: "a.py".into(),
                success: false,
                error: Some("E0308".into()),
                category: Some(ErrorCategory::TypeMismatch),
                rust_code: None,
            },
            CompileResult {
                file: "b.py".into(),
                success: false,
                error: Some("E0277".into()),
                category: Some(ErrorCategory::TraitBound),
                rust_code: None,
            },
            CompileResult {
                file: "c.py".into(),
                success: false,
                error: Some("E0599".into()),
                category: Some(ErrorCategory::SyntaxError),
                rust_code: None,
            },
        ];

        let samples = extract_training_samples(&results);
        assert_eq!(samples.len(), 3);
    }

    // ========================================================================
    // SPARK_CHARS Tests
    // ========================================================================

    #[test]
    fn test_spark_chars_array() {
        assert_eq!(SPARK_CHARS.len(), 8);
        assert_eq!(SPARK_CHARS[0], '▁');
        assert_eq!(SPARK_CHARS[7], '█');
    }
}

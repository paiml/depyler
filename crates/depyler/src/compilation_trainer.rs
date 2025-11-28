//! CompilationTrainer - Training loop abstraction for compilation improvement
//!
//! Mirrors entrenar's Trainer API but designed for compilation loops.
//! Uses entrenar callback system for early stopping and monitoring.
//!
//! ## Diagnostic Tiers
//!
//! This module implements a tiered approach to capturing verbose compiler diagnostics
//! for CITL (Compiler-in-the-Loop) training:
//!
//! - **Tier 1 (Baseline)**: JSON diagnostics + clippy lints
//! - **Tier 2 (Verbose)**: + verbose build output (-v flag)
//! - **Tier 3 (Traces)**: + RUSTC_LOG traces for specific modules
//! - **Tier 4 (Debug)**: + full debug output with backtraces
//!
//! See `docs/specifications/verbose-compiler-diagnostics-citl-spec.md` for details.

use anyhow::Result;
use chrono::Utc;
use depyler_core::cargo_toml_gen;
use depyler_core::DepylerPipeline;
use entrenar::train::{
    efficiency_score, AdaptiveCurriculum, CallbackAction, CallbackContext, CallbackManager,
    CurriculumScheduler, EarlyStopping, MonitorCallback, TieredCurriculum,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

// ============================================================================
// Diagnostic Tier Configuration (DEPYLER-0598)
// ============================================================================

/// Diagnostic verbosity tier for CITL training
///
/// Each tier captures progressively more compiler information:
/// - Higher tiers provide richer signal for oracle training
/// - Higher tiers incur greater compilation overhead
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticTier {
    /// Tier 1: JSON diagnostics + clippy (baseline)
    /// - Error codes and messages
    /// - Clippy lints
    /// - Suggested fixes
    /// - ~5% compilation overhead, ~2KB per failed file
    #[default]
    Tier1,

    /// Tier 2: + verbose build output (-v flag)
    /// - Full rustc command lines
    /// - Dependency resolution order
    /// - ~10% compilation overhead, ~5KB per failed file
    Tier2,

    /// Tier 3: + RUSTC_LOG traces
    /// - Name resolution attempts
    /// - Type inference steps
    /// - Trait bound traces
    /// - ~25% compilation overhead, ~20KB per failed file
    Tier3,

    /// Tier 4: Full debug output
    /// - Full type unification traces
    /// - Borrow checker details
    /// - Complete stack traces
    /// - ~50% compilation overhead, ~100KB per failed file
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
}

/// Clippy lint level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClippyLevel {
    /// clippy::all only (~500 lints)
    Standard,

    /// + clippy::pedantic (~100 additional stricter lints)
    Pedantic,

    /// + clippy::nursery (~50 experimental lints)
    #[default]
    Nursery,

    /// + clippy::cargo (manifest lints)
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
            _ => Self::Nursery, // default
        }
    }
}

/// Verbosity configuration for diagnostic capture
#[derive(Debug, Clone)]
pub struct VerbosityConfig {
    /// Diagnostic tier level (1-4)
    pub tier: DiagnosticTier,

    /// Clippy lint levels to enable
    pub clippy_level: ClippyLevel,

    /// Error codes that trigger higher verbosity on retry
    pub trace_errors: Vec<String>,

    /// Maximum log size per file (bytes)
    pub max_log_size: usize,

    /// Timeout for compilation (seconds)
    pub timeout_secs: u64,

    /// Enable adaptive tier escalation
    pub adaptive: bool,
}

impl Default for VerbosityConfig {
    fn default() -> Self {
        Self {
            tier: DiagnosticTier::Tier1,
            clippy_level: ClippyLevel::Nursery,
            trace_errors: vec![
                "E0308".to_string(), // type mismatch
                "E0277".to_string(), // trait not satisfied
                "E0382".to_string(), // use after move
            ],
            max_log_size: 1_000_000, // 1MB
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

    /// Build cargo clippy command with appropriate verbosity
    pub fn build_command(&self, manifest_path: &std::path::Path) -> Command {
        let mut cmd = Command::new("cargo");

        cmd.arg("clippy");

        // Verbosity flags
        match self.tier {
            DiagnosticTier::Tier1 => {}
            DiagnosticTier::Tier2 => {
                cmd.arg("-v");
            }
            DiagnosticTier::Tier3 => {
                cmd.arg("-v");
            }
            DiagnosticTier::Tier4 => {
                cmd.arg("-vv");
            }
        }

        cmd.arg("--manifest-path").arg(manifest_path);
        cmd.arg("--message-format=json");

        // Environment variables for traces (Tier 3+)
        match self.tier {
            DiagnosticTier::Tier1 | DiagnosticTier::Tier2 => {}
            DiagnosticTier::Tier3 => {
                cmd.env("RUSTC_LOG", "rustc_resolve=info,rustc_typeck=info");
                cmd.env("RUST_BACKTRACE", "1");
            }
            DiagnosticTier::Tier4 => {
                cmd.env(
                    "RUSTC_LOG",
                    "rustc_resolve=debug,rustc_typeck=debug,rustc_borrowck=debug",
                );
                cmd.env("RUST_BACKTRACE", "full");
            }
        }

        // Clippy configuration
        cmd.arg("--");
        cmd.arg("-W").arg("clippy::all");

        if matches!(
            self.clippy_level,
            ClippyLevel::Pedantic | ClippyLevel::Nursery | ClippyLevel::Full
        ) {
            cmd.arg("-W").arg("clippy::pedantic");
        }
        if matches!(self.clippy_level, ClippyLevel::Nursery | ClippyLevel::Full) {
            cmd.arg("-W").arg("clippy::nursery");
        }
        if matches!(self.clippy_level, ClippyLevel::Full) {
            cmd.arg("-W").arg("clippy::cargo");
        }

        cmd.arg("-D").arg("warnings");

        cmd
    }

    /// Select appropriate tier based on error code and attempt number
    ///
    /// Uses entrenar's AdaptiveCurriculum for tier selection (per CITL spec).
    /// Implements adaptive tier escalation for curriculum learning:
    /// - First attempt: baseline tier
    /// - Subsequent attempts: escalate based on error type
    pub fn select_tier_for_error(&self, error_code: &str, attempt: u32) -> DiagnosticTier {
        if !self.adaptive {
            return self.tier;
        }

        // Use entrenar's AdaptiveCurriculum for tier selection
        let curriculum = AdaptiveCurriculum::new();
        let tier_num = curriculum.tier_for_error(error_code, attempt as usize);

        DiagnosticTier::from_level(tier_num as u8)
    }

    /// Get long-tail sample weight for error class (per Feldman 2020)
    ///
    /// Rare error classes get higher weights to improve learning.
    pub fn weight_for_error_class(&self, error_code: &str) -> f32 {
        let curriculum = AdaptiveCurriculum::new();
        curriculum.weight_for_class(error_code)
    }
}

// ============================================================================
// Parsed Diagnostic Features
// ============================================================================

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

/// Source location span from compiler diagnostic
#[derive(Debug, Clone)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
    pub text: String,
    pub label: Option<String>,
}

impl DiagnosticFeatures {
    /// Parse JSON diagnostic output from rustc/clippy
    pub fn parse_json_diagnostics(stdout: &str) -> Vec<Self> {
        stdout
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter_map(|json| {
                let message = json.get("message")?;
                let level = message.get("level")?.as_str()?;

                if level != "error" && level != "warning" {
                    return None;
                }

                Some(DiagnosticFeatures {
                    error_code: message
                        .get("code")
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                        .map(|s| s.to_string()),
                    level: level.to_string(),
                    message: message
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("")
                        .to_string(),
                    spans: Self::parse_spans(message.get("spans")),
                    suggestions: Self::parse_suggestions(message.get("children")),
                    clippy_lints: vec![],
                    trace_lines: vec![],
                    backtrace: None,
                })
            })
            .collect()
    }

    fn parse_spans(spans: Option<&serde_json::Value>) -> Vec<DiagnosticSpan> {
        spans
            .and_then(|s| s.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|span| {
                        Some(DiagnosticSpan {
                            file_name: span.get("file_name")?.as_str()?.to_string(),
                            line_start: span.get("line_start")?.as_u64()? as u32,
                            line_end: span.get("line_end")?.as_u64()? as u32,
                            column_start: span.get("column_start")?.as_u64()? as u32,
                            column_end: span.get("column_end")?.as_u64()? as u32,
                            text: span
                                .get("text")
                                .and_then(|t| t.as_array())
                                .and_then(|arr| arr.first())
                                .and_then(|t| t.get("text"))
                                .and_then(|t| t.as_str())
                                .unwrap_or("")
                                .to_string(),
                            label: span.get("label").and_then(|l| l.as_str()).map(String::from),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn parse_suggestions(children: Option<&serde_json::Value>) -> Vec<String> {
        children
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|child| {
                        let level = child.get("level")?.as_str()?;
                        if level == "help" || level == "note" {
                            child.get("message")?.as_str().map(String::from)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Parse RUSTC_LOG output for trace signals
    pub fn parse_traces(stderr: &str, error_codes: &[String]) -> Vec<String> {
        stderr
            .lines()
            .filter(|line| {
                line.contains("rustc_resolve")
                    || line.contains("rustc_typeck")
                    || line.contains("rustc_borrowck")
                    || error_codes.iter().any(|code| line.contains(code))
            })
            .map(|s| s.to_string())
            .collect()
    }
}

// ============================================================================
// Compilation Result and Config
// ============================================================================

/// Result of a compilation training run (mirrors entrenar::TrainResult)
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Final epoch reached
    pub final_epoch: usize,
    /// Final compilation rate (0.0-1.0)
    pub final_rate: f64,
    /// Best compilation rate achieved
    pub best_rate: f64,
    /// Whether training was stopped early
    pub stopped_early: bool,
    /// Total training time in seconds
    pub elapsed_secs: f64,
    /// Number of files processed
    pub files_processed: usize,
    /// Number of files that compiled successfully
    pub files_compiled: usize,
    /// Number of files that transpiled successfully
    pub files_transpiled: usize,
    /// Target rate achieved
    pub target_achieved: bool,
}

/// Configuration for compilation training
#[derive(Debug, Clone)]
pub struct CompilationConfig {
    /// Target compilation rate (0.0-1.0)
    pub target_rate: f64,
    /// Maximum iterations
    pub max_epochs: usize,
    /// Early stopping patience
    pub patience: usize,
    /// Minimum delta for improvement
    pub min_delta: f64,
    /// Enable verbose output
    pub verbose: bool,
    /// Enable monitoring output
    pub monitor: bool,
    /// Directory for reports
    pub report_dir: PathBuf,
    /// Directory for exporting error corpus
    pub export_corpus: Option<PathBuf>,
    /// Verbosity configuration for diagnostic capture (DEPYLER-0598)
    pub verbosity: VerbosityConfig,
    /// Sample reweight factor for curriculum learning (per Feldman 2020)
    /// Values >1.0 emphasize rare error classes
    pub reweight: f32,
}

impl Default for CompilationConfig {
    fn default() -> Self {
        Self {
            target_rate: 1.0,
            max_epochs: 10,
            patience: 3,
            min_delta: 0.001,
            verbose: false,
            monitor: false,
            report_dir: PathBuf::from(".depyler-improve"),
            export_corpus: None,
            verbosity: VerbosityConfig::default(),
            reweight: 1.0, // No reweighting by default
        }
    }
}

impl CompilationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_target_rate(mut self, rate: f64) -> Self {
        self.target_rate = rate;
        self
    }

    pub fn with_max_epochs(mut self, epochs: usize) -> Self {
        self.max_epochs = epochs;
        self
    }

    pub fn with_patience(mut self, patience: usize) -> Self {
        self.patience = patience;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_monitor(mut self, monitor: bool) -> Self {
        self.monitor = monitor;
        self
    }

    pub fn with_report_dir(mut self, dir: PathBuf) -> Self {
        self.report_dir = dir;
        self
    }

    pub fn with_export_corpus(mut self, dir: PathBuf) -> Self {
        self.export_corpus = Some(dir);
        self
    }

    /// Set diagnostic verbosity tier (1-4)
    pub fn with_verbosity_tier(mut self, tier: u8) -> Self {
        self.verbosity.tier = DiagnosticTier::from_level(tier);
        self
    }

    /// Set clippy lint level
    pub fn with_clippy_level(mut self, level: &str) -> Self {
        self.verbosity.clippy_level = ClippyLevel::from_cli_arg(level);
        self
    }

    /// Set full verbosity configuration
    pub fn with_verbosity(mut self, verbosity: VerbosityConfig) -> Self {
        self.verbosity = verbosity;
        self
    }

    /// Enable/disable adaptive tier escalation
    pub fn with_adaptive_verbosity(mut self, adaptive: bool) -> Self {
        self.verbosity.adaptive = adaptive;
        self
    }

    /// Set sample reweight factor for curriculum learning (per Feldman 2020)
    /// Values >1.0 emphasize rare error classes
    pub fn with_reweight(mut self, reweight: f32) -> Self {
        self.reweight = reweight;
        self
    }
}

/// CompilationTrainer - Orchestrates compilation improvement loop
///
/// Mirrors entrenar's Trainer API for compilation loops:
/// - Uses callback system for early stopping and monitoring
/// - Uses TieredCurriculum for automatic tier advancement (CITL spec)
/// - Returns standardized CompilationResult
/// - Handles transpilation, compilation, and error tracking
pub struct CompilationTrainer {
    /// Python files to process
    files: Vec<PathBuf>,
    /// Configuration
    config: CompilationConfig,
    /// Callback manager
    callbacks: CallbackManager,
    /// Best rate achieved
    best_rate: Option<f64>,
    /// Training start time
    start_time: Option<Instant>,
    /// Previous pass count (for delta calculation)
    prev_pass_count: usize,
    /// Error corpus for training
    error_corpus: Vec<(String, String, String)>,
    /// Tiered curriculum for automatic tier advancement (CITL spec)
    curriculum: TieredCurriculum,
    /// Corpus size tracking for efficiency scoring
    corpus_bytes: usize,
}

impl CompilationTrainer {
    /// Create a new compilation trainer
    pub fn new(files: Vec<PathBuf>, config: CompilationConfig) -> Self {
        let mut callbacks = CallbackManager::new();
        callbacks.add(EarlyStopping::new(config.patience, config.min_delta as f32));
        callbacks.add(MonitorCallback::new());

        // Initialize CITL-style tiered curriculum (60%→70%→80% accuracy thresholds)
        let curriculum = TieredCurriculum::citl_default();

        Self {
            files,
            config,
            callbacks,
            best_rate: None,
            start_time: None,
            prev_pass_count: 0,
            error_corpus: Vec::new(),
            curriculum,
            corpus_bytes: 0,
        }
    }

    /// Add a callback to the trainer
    pub fn add_callback<C: entrenar::train::TrainerCallback + 'static>(&mut self, callback: C) {
        self.callbacks.add(callback);
    }

    /// Build callback context from current state
    fn build_context(&self, epoch: usize, loss: f32) -> CallbackContext {
        CallbackContext {
            epoch,
            max_epochs: self.config.max_epochs,
            loss,
            best_loss: self.best_rate.map(|r| (1.0 - r) as f32),
            elapsed_secs: self.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0),
            ..Default::default()
        }
    }

    /// Run training loop
    pub fn train(&mut self) -> Result<CompilationResult> {
        self.start_time = Some(Instant::now());
        self.best_rate = None;
        let total_files = self.files.len();

        // Create directories
        fs::create_dir_all(&self.config.report_dir)?;
        let temp_base = self.config.report_dir.join("cargo_projects");
        fs::create_dir_all(&temp_base)?;

        // Fire train_begin
        let ctx = self.build_context(0, 1.0);
        if self.callbacks.on_train_begin(&ctx) == CallbackAction::Stop {
            return Ok(self.build_result(0, 0.0, 0, 0, true, false));
        }

        let mut final_epoch = 0;
        let mut final_rate = 0.0;
        let mut stopped_early = false;
        let mut transpile_ok = 0;
        let mut compile_ok = 0;

        for epoch in 0..self.config.max_epochs {
            final_epoch = epoch;

            // Fire epoch_begin
            let ctx = self.build_context(epoch, (1.0 - final_rate) as f32);
            match self.callbacks.on_epoch_begin(&ctx) {
                CallbackAction::Stop => {
                    stopped_early = true;
                    break;
                }
                CallbackAction::SkipEpoch => continue,
                CallbackAction::Continue => {}
            }

            // Step 1: Transpile all files
            let transpile_results = self.transpile_epoch(&temp_base)?;
            transpile_ok = transpile_results.values().filter(|r| r.is_ok()).count();

            // Step 2: Compile all files
            let compile_results = self.compile_epoch(&transpile_results)?;
            compile_ok = compile_results.values().filter(|r| r.is_ok()).count();

            final_rate = compile_ok as f64 / total_files as f64;
            let delta = compile_ok as i32 - self.prev_pass_count as i32;

            // Update best rate
            if self.best_rate.is_none() || final_rate > self.best_rate.unwrap() {
                self.best_rate = Some(final_rate);
            }

            // CITL: Step curriculum and check for tier advancement (60%→70%→80% thresholds)
            let prev_tier = self.curriculum.tier();
            self.curriculum.step(epoch, final_rate as f32);
            let new_tier = self.curriculum.tier();

            // Update verbosity tier if curriculum advanced (adaptive mode)
            if self.config.verbosity.adaptive && new_tier != prev_tier {
                self.config.verbosity.tier = DiagnosticTier::from_level(new_tier as u8);
                println!(
                    "       📈 Curriculum advanced: Tier {} → {} (accuracy: {:.1}%)",
                    prev_tier,
                    new_tier,
                    final_rate * 100.0
                );
            }

            // Track corpus size for efficiency scoring
            let epoch_corpus_bytes: usize = self
                .error_corpus
                .iter()
                .map(|(_, _, json)| json.len())
                .sum();
            self.corpus_bytes += epoch_corpus_bytes;

            // Display progress
            self.display_progress(epoch, final_rate, transpile_ok, compile_ok, delta);

            // Fire epoch_end
            let error_rate = 1.0 - final_rate;
            let ctx = self.build_context(epoch, error_rate as f32);
            if self.callbacks.on_epoch_end(&ctx) == CallbackAction::Stop {
                println!("{}", "─".repeat(70));
                println!("🛑 Training stopped by callback (early stopping or andon alert)");
                stopped_early = true;
                break;
            }

            // Check target achieved
            if final_rate >= self.config.target_rate {
                println!("{}", "─".repeat(70));
                println!("🎉 Target achieved: {:.1}% compilation rate", final_rate * 100.0);
                self.callbacks.on_train_end(&ctx);
                return Ok(self.build_result(epoch + 1, final_rate, transpile_ok, compile_ok, false, true));
            }

            // Verbose output
            if self.config.verbose {
                self.display_errors();
            }

            // Monitor output
            if self.config.monitor {
                self.write_monitor_json(epoch, transpile_ok, compile_ok, final_rate, delta, &compile_results)?;
            }

            // Export corpus
            if let Some(ref corpus_path) = self.config.export_corpus {
                self.export_corpus(epoch, corpus_path)?;
            }

            self.prev_pass_count = compile_ok;
        }

        // Fire train_end
        let ctx = self.build_context(final_epoch, (1.0 - final_rate) as f32);
        self.callbacks.on_train_end(&ctx);

        // Final summary
        self.display_final_summary(compile_ok, final_rate);

        Ok(self.build_result(
            final_epoch + 1,
            final_rate,
            transpile_ok,
            compile_ok,
            stopped_early,
            final_rate >= self.config.target_rate,
        ))
    }

    fn build_result(
        &self,
        final_epoch: usize,
        final_rate: f64,
        transpile_ok: usize,
        compile_ok: usize,
        stopped_early: bool,
        target_achieved: bool,
    ) -> CompilationResult {
        CompilationResult {
            final_epoch,
            final_rate,
            best_rate: self.best_rate.unwrap_or(final_rate),
            stopped_early,
            elapsed_secs: self.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0),
            files_processed: self.files.len(),
            files_compiled: compile_ok,
            files_transpiled: transpile_ok,
            target_achieved,
        }
    }

    fn transpile_epoch(&self, temp_base: &Path) -> Result<HashMap<PathBuf, Result<PathBuf, String>>> {
        let pb = ProgressBar::new(self.files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:20}] {pos}/{len}")
                .unwrap(),
        );

        let mut results: HashMap<PathBuf, Result<PathBuf, String>> = HashMap::new();

        for py_file in &self.files {
            let file_stem = py_file.file_stem().unwrap_or_default().to_string_lossy();
            let project_dir = temp_base.join(format!("proj_{}", file_stem));

            let source_result = fs::read_to_string(py_file);
            let transpile_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                match source_result {
                    Ok(source) => {
                        let pipeline = DepylerPipeline::new();
                        match pipeline.transpile_with_dependencies(&source) {
                            Ok((rust_code, dependencies)) => {
                                fs::create_dir_all(&project_dir).map_err(|e| format!("mkdir: {}", e))?;
                                let rs_file = project_dir.join("lib.rs");
                                fs::write(&rs_file, &rust_code).map_err(|e| format!("Write: {}", e))?;
                                let cargo_toml = cargo_toml_gen::generate_cargo_toml(&file_stem, "lib.rs", &dependencies);
                                fs::write(project_dir.join("Cargo.toml"), &cargo_toml)
                                    .map_err(|e| format!("Cargo.toml: {}", e))?;
                                Ok(project_dir.clone())
                            }
                            Err(e) => Err(format!("Transpile error: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Read error: {}", e)),
                }
            }));

            match transpile_result {
                Ok(Ok(path)) => results.insert(py_file.clone(), Ok(path)),
                Ok(Err(e)) => results.insert(py_file.clone(), Err(e)),
                Err(_) => results.insert(py_file.clone(), Err("Panic during transpilation".to_string())),
            };
            pb.inc(1);
        }
        pb.finish_and_clear();

        Ok(results)
    }

    fn compile_epoch(
        &mut self,
        transpile_results: &HashMap<PathBuf, Result<PathBuf, String>>,
    ) -> Result<HashMap<PathBuf, Result<(), String>>> {
        let mut results: HashMap<PathBuf, Result<(), String>> = HashMap::new();
        self.error_corpus.clear();

        for (py_file, transpile_result) in transpile_results {
            if let Ok(project_dir) = transpile_result {
                let manifest_path = project_dir.join("Cargo.toml");

                // Use VerbosityConfig to build the command (DEPYLER-0598)
                let mut cmd = self.config.verbosity.build_command(&manifest_path);
                let output = cmd.output();

                match output {
                    Ok(result) if result.status.success() => {
                        results.insert(py_file.clone(), Ok(()));
                    }
                    Ok(result) => {
                        // JSON diagnostics come on stdout, human-readable on stderr
                        let stdout = String::from_utf8_lossy(&result.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&result.stderr).to_string();
                        let full_output = format!("{}\n{}", stdout, stderr);
                        results.insert(py_file.clone(), Err(full_output.clone()));

                        let file_name = py_file
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // Parse JSON diagnostic lines for richer signal
                        let diagnostics = DiagnosticFeatures::parse_json_diagnostics(&stdout);
                        for diag in &diagnostics {
                            let code = diag.error_code.as_deref().unwrap_or("no_code");
                            let diagnostic = format!("[{}] {}: {}", diag.level, code, diag.message);

                            // For Tier 3+, also capture trace lines
                            let trace_json = if matches!(
                                self.config.verbosity.tier,
                                DiagnosticTier::Tier3 | DiagnosticTier::Tier4
                            ) {
                                let traces =
                                    DiagnosticFeatures::parse_traces(&stderr, &self.config.verbosity.trace_errors);
                                if !traces.is_empty() {
                                    format!(
                                        "{{\"diagnostic\":{},\"traces\":{}}}",
                                        serde_json::to_string(&diagnostic).unwrap_or_default(),
                                        serde_json::to_string(&traces).unwrap_or_default()
                                    )
                                } else {
                                    diagnostic.clone()
                                }
                            } else {
                                diagnostic.clone()
                            };

                            self.error_corpus.push((file_name.clone(), diagnostic, trace_json));
                        }
                    }
                    Err(e) => {
                        results.insert(py_file.clone(), Err(format!("Command error: {}", e)));
                    }
                }
            }
        }

        Ok(results)
    }

    fn display_progress(&self, epoch: usize, rate: f64, transpile_ok: usize, compile_ok: usize, delta: i32) {
        let total = self.files.len();
        let bar_width = 20;
        let filled = (rate * bar_width as f64) as usize;
        let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);

        let delta_str = if delta > 0 {
            format!("+{}", delta)
        } else if delta == 0 {
            "─".to_string()
        } else {
            format!("{}", delta)
        };

        let status = if rate >= self.config.target_rate {
            "✓"
        } else if delta > 0 {
            "↑"
        } else if delta == 0 {
            "→"
        } else {
            "↓"
        };

        print!(
            "\rEpoch {}/{} [{}] {:>5.1}% | trans: {}/{} | comp: {}/{} | Δ: {:>3} {}",
            epoch + 1,
            self.config.max_epochs,
            bar,
            rate * 100.0,
            transpile_ok,
            total,
            compile_ok,
            total,
            delta_str,
            status
        );
        std::io::Write::flush(&mut std::io::stdout()).ok();
        println!();
    }

    fn display_errors(&self) {
        let mut error_categories: HashMap<String, usize> = HashMap::new();
        for (_, error_code, _) in &self.error_corpus {
            if let Some(code) = error_code.split(']').next() {
                let code = code.trim_start_matches("error[").to_string();
                *error_categories.entry(code).or_insert(0) += 1;
            }
        }
        let mut sorted: Vec<_> = error_categories.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        print!("       Errors: ");
        for (i, (code, count)) in sorted.iter().take(5).enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}:{}", code, count);
        }
        println!();
    }

    fn write_monitor_json(
        &self,
        epoch: usize,
        transpile_ok: usize,
        compile_ok: usize,
        rate: f64,
        delta: i32,
        compile_results: &HashMap<PathBuf, Result<(), String>>,
    ) -> Result<()> {
        let monitor_file = self.config.report_dir.join("monitor.json");
        let mut error_categories: HashMap<String, usize> = HashMap::new();
        let mut failed_files: Vec<String> = Vec::new();

        for (_, error_code, _) in &self.error_corpus {
            if let Some(code) = error_code.split(']').next() {
                let code = code.trim_start_matches("error[").to_string();
                *error_categories.entry(code).or_insert(0) += 1;
            }
        }

        for (py_file, result) in compile_results {
            if result.is_err() {
                if let Some(name) = py_file.file_name() {
                    failed_files.push(name.to_string_lossy().to_string());
                }
            }
        }

        let mut sorted_errors: Vec<_> = error_categories.into_iter().collect();
        sorted_errors.sort_by(|a, b| b.1.cmp(&a.1));

        let error_json: String = sorted_errors
            .iter()
            .map(|(code, count)| format!("\"{}\":{}", code, count))
            .collect::<Vec<_>>()
            .join(",");

        let failed_json: String = failed_files
            .iter()
            .map(|f| format!("\"{}\"", f))
            .collect::<Vec<_>>()
            .join(",");

        let monitor_json = format!(
            r#"{{
  "epoch": {},
  "max_epochs": {},
  "transpile_ok": {},
  "compile_ok": {},
  "total_files": {},
  "compile_rate": {:.4},
  "target_rate": {:.4},
  "delta": {},
  "error_rate": {:.4},
  "error_distribution": {{{}}},
  "failed_files": [{}],
  "timestamp": "{}"
}}"#,
            epoch + 1,
            self.config.max_epochs,
            transpile_ok,
            compile_ok,
            self.files.len(),
            rate,
            self.config.target_rate,
            delta,
            1.0 - rate,
            error_json,
            failed_json,
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
        );
        fs::write(&monitor_file, monitor_json)?;
        Ok(())
    }

    fn export_corpus(&self, epoch: usize, corpus_path: &PathBuf) -> Result<()> {
        let corpus_file = corpus_path.join(format!("epoch_{}.jsonl", epoch));
        let mut output = String::new();
        for (file, code, _msg) in &self.error_corpus {
            output.push_str(&format!(
                "{{\"file\":\"{}\",\"error\":\"{}\"}}\n",
                file,
                code.replace('\"', "\\\"")
            ));
        }
        fs::create_dir_all(corpus_path)?;
        fs::write(&corpus_file, &output)?;
        Ok(())
    }

    fn display_final_summary(&self, compile_ok: usize, final_rate: f64) {
        let total = self.files.len();
        println!("{}", "─".repeat(70));
        println!(
            "\n📊 Final: {}/{} ({:.1}%) | Target: {:.1}% | {}",
            compile_ok,
            total,
            final_rate * 100.0,
            self.config.target_rate * 100.0,
            if final_rate >= self.config.target_rate { "✅ PASS" } else { "❌ FAIL" }
        );

        // CITL: Calculate efficiency score E(T) = Accuracy / log(CorpusSize)
        let eff_score = if self.corpus_bytes > 0 {
            efficiency_score(final_rate as f32, self.corpus_bytes)
        } else {
            0.0
        };

        // Generate hansei report with CITL metrics
        let training_id = format!("depyler-improve-{}", Utc::now().format("%Y%m%d-%H%M%S"));
        let hansei_report = format!(
            "\n🎯 Hansei Report: {}\n{}\nFinal Rate: {:.1}%\nBest Rate: {:.1}%\nElapsed: {:.1}s\nFinal Tier: {}\nCorpus Size: {} bytes\nEfficiency Score: {:.4}",
            training_id,
            "─".repeat(50),
            final_rate * 100.0,
            self.best_rate.unwrap_or(final_rate) * 100.0,
            self.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0),
            self.curriculum.tier(),
            self.corpus_bytes,
            eff_score
        );
        println!("{}", hansei_report);

        // Display efficiency interpretation
        let efficiency_grade = match eff_score {
            e if e > 0.15 => "🟢 Excellent",
            e if e > 0.10 => "🟡 Good",
            e if e > 0.05 => "🟠 Acceptable",
            _ => "🔴 Needs improvement",
        };
        println!("Efficiency Grade: {} ({:.4})", efficiency_grade, eff_score);

        // Write report
        let report_file = self.config.report_dir.join("hansei_report.txt");
        fs::write(&report_file, &hansei_report).ok();
        println!("📄 Reports written to {}", self.config.report_dir.display());
    }

    /// Get reference to error corpus
    pub fn error_corpus(&self) -> &[(String, String, String)] {
        &self.error_corpus
    }

    /// Get weighted error corpus with Feldman long-tail weighting
    ///
    /// Applies reweight factor to emphasize rare error classes.
    /// Returns corpus entries with weights attached.
    pub fn weighted_error_corpus(&self) -> Vec<(String, String, String, f32)> {
        weight_corpus_entries(&self.error_corpus, self.config.reweight)
    }

    /// Export weighted corpus to JSONL format
    pub fn export_weighted_corpus_jsonl(&self) -> String {
        export_weighted_corpus_jsonl(&self.error_corpus, self.config.reweight)
    }
}

// ============================================================================
// Weighted Corpus Functions (DEPYLER-0599)
// ============================================================================

/// Apply Feldman long-tail weighting to corpus entries
///
/// Uses entrenar's AdaptiveCurriculum.weight_for_class() to compute
/// class-specific weights, then applies the reweight factor.
///
/// # Arguments
/// * `corpus` - (filename, error_code, json_diagnostic) tuples
/// * `reweight` - Scaling factor for weights (1.0 = no change, >1.0 = emphasize rare)
///
/// # Returns
/// Corpus entries with weights: (filename, error_code, json_diagnostic, weight)
pub fn weight_corpus_entries(
    corpus: &[(String, String, String)],
    reweight: f32,
) -> Vec<(String, String, String, f32)> {
    if (reweight - 1.0).abs() < 0.001 {
        // No reweighting - return uniform weights
        return corpus
            .iter()
            .map(|(f, e, j)| (f.clone(), e.clone(), j.clone(), 1.0))
            .collect();
    }

    // Count error class frequencies
    let mut class_counts: HashMap<String, usize> = HashMap::new();
    for (_, error_code, _) in corpus {
        let class = extract_error_class(error_code);
        *class_counts.entry(class).or_insert(0) += 1;
    }

    let total_samples = corpus.len() as f32;
    let num_classes = class_counts.len() as f32;

    // Use AdaptiveCurriculum for base weights
    let curriculum = AdaptiveCurriculum::new();

    corpus
        .iter()
        .map(|(f, e, j)| {
            let class = extract_error_class(e);
            let class_count = *class_counts.get(&class).unwrap_or(&1) as f32;

            // Inverse frequency weighting: rare classes get higher weight
            let base_weight = curriculum.weight_for_class(&class);

            // Apply reweight factor with inverse frequency
            // Formula: weight = base_weight * (total / (num_classes * class_count)) ^ (reweight - 1)
            let inv_freq = total_samples / (num_classes * class_count);
            let weight = base_weight * inv_freq.powf(reweight - 1.0);

            (f.clone(), e.clone(), j.clone(), weight)
        })
        .collect()
}

/// Export weighted corpus to JSONL format for external training
///
/// Each line is a JSON object with:
/// - file: source filename
/// - error: error code/message
/// - diagnostic: full JSON diagnostic
/// - weight: sample weight for training
pub fn export_weighted_corpus_jsonl(corpus: &[(String, String, String)], reweight: f32) -> String {
    let weighted = weight_corpus_entries(corpus, reweight);

    weighted
        .iter()
        .map(|(file, error, diagnostic, weight)| {
            format!(
                r#"{{"file":"{}","error":"{}","diagnostic":{},"weight":{:.4}}}"#,
                file.replace('\"', "\\\""),
                error.replace('\"', "\\\""),
                if diagnostic.starts_with('{') {
                    diagnostic.clone()
                } else {
                    format!("\"{}\"", diagnostic.replace('\"', "\\\""))
                },
                weight
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Apply reweight sampling to error class counts
///
/// Returns adjusted counts that oversample rare error classes.
pub fn apply_reweight_sampling(
    samples: &[(&str, usize)],
    reweight: f32,
) -> HashMap<String, usize> {
    if samples.is_empty() {
        return HashMap::new();
    }

    let total: usize = samples.iter().map(|(_, c)| c).sum();
    let num_classes = samples.len() as f32;

    samples
        .iter()
        .map(|(class, count)| {
            let class_freq = *count as f32 / total as f32;
            // Inverse frequency boosting
            let boost = (1.0 / (num_classes * class_freq)).powf(reweight - 1.0);
            let adjusted_count = ((*count as f32) * boost).ceil() as usize;
            (class.to_string(), adjusted_count.max(1))
        })
        .collect()
}

/// Extract error class from error message (e.g., "E0308" from "[error] E0308: ...")
fn extract_error_class(error_msg: &str) -> String {
    // Try to extract error code like E0308, ICE-0001, clippy::xxx
    if let Some(start) = error_msg.find("E0") {
        if let Some(end) = error_msg[start..].find(|c: char| !c.is_alphanumeric()) {
            return error_msg[start..start + end].to_string();
        }
        return error_msg[start..].chars().take(5).collect();
    }
    if let Some(start) = error_msg.find("ICE") {
        return error_msg[start..].chars().take_while(|c| c.is_alphanumeric() || *c == '-').collect();
    }
    if let Some(start) = error_msg.find("clippy::") {
        return error_msg[start..].chars().take_while(|c| c.is_alphanumeric() || *c == ':').collect();
    }
    // Default: use first word after bracket
    error_msg
        .split(']')
        .nth(1)
        .and_then(|s| s.split(':').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

// ============================================================================
// OIP Export Functions (GitHub #156)
// ============================================================================

use alimentar::ArrowDataset;
use arrow::array::{Float32Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// OIP training example with compiler diagnostic signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OipTrainingExample {
    pub source_file: String,
    pub rust_file: String,
    pub error_code: Option<String>,
    pub clippy_lint: Option<String>,
    pub level: String,
    pub message: String,
    pub oip_category: String,
    pub confidence: f64,
    pub line_start: i64,
    pub line_end: i64,
    pub suggestion: Option<String>,
    pub python_construct: Option<String>,
    pub timestamp: i64,
    pub depyler_version: String,
    pub weight: f32,
}

/// Map Rust error code to OIP DefectCategory
///
/// Based on the taxonomy in verbose-compiler-diagnostics-citl-spec.md §11.2
pub fn map_error_to_oip_category(error_code: &str) -> (&'static str, f64) {
    match error_code {
        // Type errors - high confidence
        "E0308" => ("TypeErrors", 0.95),
        "E0277" => ("TraitBounds", 0.95),

        // Ownership/borrow - high confidence
        "E0502" | "E0503" | "E0505" => ("OwnershipBorrow", 0.95),
        "E0382" | "E0507" => ("MemorySafety", 0.90),

        // Name resolution - medium-high confidence
        "E0425" | "E0433" => ("StdlibMapping", 0.85),
        "E0412" => ("TypeAnnotationGaps", 0.85),

        // AST/operator issues - medium confidence
        "E0599" | "E0615" => ("ASTTransform", 0.80),
        "E0614" => ("OperatorPrecedence", 0.80),

        // Configuration - lower confidence
        "E0658" => ("ConfigurationErrors", 0.75),

        // Clippy lints
        c if c.starts_with("clippy::unwrap") => ("ApiMisuse", 0.85),
        c if c.starts_with("clippy::expect") => ("ApiMisuse", 0.85),
        c if c.starts_with("clippy::panic") => ("ApiMisuse", 0.85),
        c if c.starts_with("clippy::todo") => ("LogicErrors", 0.80),
        c if c.starts_with("clippy::unreachable") => ("LogicErrors", 0.80),
        c if c.starts_with("clippy::cognitive") => ("PerformanceIssues", 0.75),
        c if c.starts_with("clippy::needless") => ("IteratorChain", 0.80),
        c if c.starts_with("clippy::manual") => ("ComprehensionBugs", 0.80),

        // Internal compiler errors
        c if c.starts_with("ICE") => ("InternalError", 0.99),

        // Default
        _ => ("LogicErrors", 0.60),
    }
}

/// Export statistics for OIP corpus export
#[derive(Debug, Default)]
pub struct OipExportStats {
    /// Total samples processed
    pub total_samples: usize,
    /// Samples that passed confidence threshold
    pub exported_samples: usize,
    /// Samples filtered due to low confidence
    pub filtered_low_confidence: usize,
    /// Number of unique Rust error codes seen
    pub unique_error_codes: usize,
    /// Number of unique OIP categories assigned
    pub unique_oip_categories: usize,
    /// Distribution of samples by OIP category
    pub category_distribution: HashMap<String, usize>,
    /// Total weight (for reweighting verification)
    pub total_weight: f32,
}

/// Export format for OIP corpus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OipExportFormat {
    Parquet,
    JsonL,
}

impl OipExportFormat {
    /// Parse format from CLI argument string
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "parquet" | "pq" => Self::Parquet,
            "jsonl" | "json" | "ndjson" => Self::JsonL,
            _ => Self::Parquet, // default
        }
    }
}

/// Build Arrow schema for OIP training examples
fn oip_arrow_schema() -> Schema {
    Schema::new(vec![
        Field::new("source_file", DataType::Utf8, false),
        Field::new("rust_file", DataType::Utf8, false),
        Field::new("error_code", DataType::Utf8, true),
        Field::new("clippy_lint", DataType::Utf8, true),
        Field::new("level", DataType::Utf8, false),
        Field::new("message", DataType::Utf8, false),
        Field::new("oip_category", DataType::Utf8, false),
        Field::new("confidence", DataType::Float32, false),
        Field::new("line_start", DataType::Int64, false),
        Field::new("line_end", DataType::Int64, false),
        Field::new("suggestion", DataType::Utf8, true),
        Field::new("python_construct", DataType::Utf8, true),
        Field::new("timestamp", DataType::Int64, false),
        Field::new("depyler_version", DataType::Utf8, false),
        Field::new("weight", DataType::Float32, false),
    ])
}

/// Convert corpus entries to OIP training examples
pub fn corpus_to_oip_examples(
    corpus: &[(String, String, String)],
    reweight: f32,
    min_confidence: f64,
    include_clippy: bool,
) -> Vec<OipTrainingExample> {
    let weighted = weight_corpus_entries(corpus, reweight);
    let timestamp = Utc::now().timestamp();
    let version = env!("CARGO_PKG_VERSION").to_string();

    weighted
        .into_iter()
        .filter_map(|(file, error, diagnostic, weight)| {
            let error_code = extract_error_code(&error);
            let (oip_category, confidence) = map_error_to_oip_category(&error_code);

            // Filter by confidence
            if confidence < min_confidence {
                return None;
            }

            // Filter clippy if not included
            if !include_clippy && error_code.starts_with("clippy::") {
                return None;
            }

            Some(OipTrainingExample {
                source_file: file.clone(),
                rust_file: format!("{}.rs", file.trim_end_matches(".py")),
                error_code: if error_code.starts_with("clippy::") { None } else { Some(error_code.clone()) },
                clippy_lint: if error_code.starts_with("clippy::") { Some(error_code) } else { None },
                level: if error.contains("[error]") { "error".to_string() } else { "warning".to_string() },
                message: extract_message(&error),
                oip_category: oip_category.to_string(),
                confidence,
                line_start: extract_line(&diagnostic).unwrap_or(0),
                line_end: extract_line(&diagnostic).unwrap_or(0),
                suggestion: extract_suggestion_text(&diagnostic),
                python_construct: None, // Could be inferred from AST
                timestamp,
                depyler_version: version.clone(),
                weight,
            })
        })
        .collect()
}

/// Extract error code from error string
fn extract_error_code(error: &str) -> String {
    extract_error_class(error)
}

/// Extract message from error string
fn extract_message(error: &str) -> String {
    error
        .split(':')
        .skip(1)
        .collect::<Vec<_>>()
        .join(":")
        .trim()
        .to_string()
}

/// Extract line number from diagnostic JSON
fn extract_line(diagnostic: &str) -> Option<i64> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(diagnostic) {
        json.get("spans")
            .and_then(|s| s.get(0))
            .and_then(|s| s.get("line_start"))
            .and_then(|l| l.as_i64())
    } else {
        None
    }
}

/// Extract suggestion text from diagnostic JSON
fn extract_suggestion_text(diagnostic: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(diagnostic) {
        json.get("children")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
    } else {
        None
    }
}

/// Convert OIP examples to Arrow RecordBatch
fn examples_to_record_batch(examples: &[OipTrainingExample]) -> Result<RecordBatch> {
    let schema = Arc::new(oip_arrow_schema());

    let source_files: Vec<&str> = examples.iter().map(|e| e.source_file.as_str()).collect();
    let rust_files: Vec<&str> = examples.iter().map(|e| e.rust_file.as_str()).collect();
    let error_codes: Vec<Option<&str>> = examples.iter().map(|e| e.error_code.as_deref()).collect();
    let clippy_lints: Vec<Option<&str>> = examples.iter().map(|e| e.clippy_lint.as_deref()).collect();
    let levels: Vec<&str> = examples.iter().map(|e| e.level.as_str()).collect();
    let messages: Vec<&str> = examples.iter().map(|e| e.message.as_str()).collect();
    let categories: Vec<&str> = examples.iter().map(|e| e.oip_category.as_str()).collect();
    let confidences: Vec<f32> = examples.iter().map(|e| e.confidence as f32).collect();
    let line_starts: Vec<i64> = examples.iter().map(|e| e.line_start).collect();
    let line_ends: Vec<i64> = examples.iter().map(|e| e.line_end).collect();
    let suggestions: Vec<Option<&str>> = examples.iter().map(|e| e.suggestion.as_deref()).collect();
    let constructs: Vec<Option<&str>> = examples.iter().map(|e| e.python_construct.as_deref()).collect();
    let timestamps: Vec<i64> = examples.iter().map(|e| e.timestamp).collect();
    let versions: Vec<&str> = examples.iter().map(|e| e.depyler_version.as_str()).collect();
    let weights: Vec<f32> = examples.iter().map(|e| e.weight).collect();

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(StringArray::from(source_files)),
            Arc::new(StringArray::from(rust_files)),
            Arc::new(StringArray::from(error_codes)),
            Arc::new(StringArray::from(clippy_lints)),
            Arc::new(StringArray::from(levels)),
            Arc::new(StringArray::from(messages)),
            Arc::new(StringArray::from(categories)),
            Arc::new(Float32Array::from(confidences)),
            Arc::new(Int64Array::from(line_starts)),
            Arc::new(Int64Array::from(line_ends)),
            Arc::new(StringArray::from(suggestions)),
            Arc::new(StringArray::from(constructs)),
            Arc::new(Int64Array::from(timestamps)),
            Arc::new(StringArray::from(versions)),
            Arc::new(Float32Array::from(weights)),
        ],
    )?;

    Ok(batch)
}

/// Export OIP corpus using alimentar
///
/// Supports Parquet (recommended) and JSONL formats.
/// Uses alimentar for efficient Arrow-based serialization.
pub fn export_oip_corpus(
    corpus: &[(String, String, String)],
    output_path: &Path,
    format: OipExportFormat,
    min_confidence: f64,
    include_clippy: bool,
    reweight: f32,
) -> Result<OipExportStats> {
    let examples = corpus_to_oip_examples(corpus, reweight, min_confidence, include_clippy);

    let mut stats = OipExportStats {
        total_samples: corpus.len(),
        exported_samples: examples.len(),
        filtered_low_confidence: corpus.len().saturating_sub(examples.len()),
        ..Default::default()
    };

    if examples.is_empty() {
        return Ok(stats);
    }

    // Collect unique error codes and categories
    let mut error_codes = std::collections::HashSet::new();
    let mut oip_categories = std::collections::HashSet::new();

    for ex in &examples {
        if let Some(ref code) = ex.error_code {
            error_codes.insert(code.clone());
        }
        oip_categories.insert(ex.oip_category.clone());
        *stats.category_distribution.entry(ex.oip_category.clone()).or_insert(0) += 1;
        stats.total_weight += ex.weight;
    }

    stats.unique_error_codes = error_codes.len();
    stats.unique_oip_categories = oip_categories.len();

    match format {
        OipExportFormat::Parquet => {
            // Use alimentar for Parquet export
            let batch = examples_to_record_batch(&examples)?;
            let dataset = ArrowDataset::new(vec![batch])?;
            dataset.to_parquet(output_path)?;
        }
        OipExportFormat::JsonL => {
            // JSONL export
            let mut output = String::new();
            for ex in &examples {
                let json = serde_json::to_string(&ex)?;
                output.push_str(&json);
                output.push('\n');
            }
            fs::write(output_path, output)?;
        }
    }

    Ok(stats)
}

/// Load corpus from cache file (JSONL format)
///
/// The cache is generated by `oracle improve` command and contains
/// tuples of (source_file, rust_code, diagnostics).
pub fn load_corpus_cache(cache_path: &Path) -> Result<Vec<(String, String, String)>> {
    let content = fs::read_to_string(cache_path)?;
    let mut corpus = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
            let source = entry.get("source_file")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let rust_code = entry.get("rust_code")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let diagnostics = entry.get("diagnostics")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            corpus.push((source, rust_code, diagnostics));
        }
    }

    Ok(corpus)
}

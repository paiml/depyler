//! Progress reporting for convergence loop
//!
//! Provides formatted output for tracking convergence progress.
//! Supports multiple display modes: rich (TUI), minimal (CI), json, silent.

use super::clusterer::ErrorCluster;
use super::roi_metrics::{EscapeRateTracker, ESCAPE_RATE_FALSIFICATION_THRESHOLD};
use super::state::{ConvergenceState, DisplayMode};

/// Report for a single iteration
#[derive(Debug, Clone)]
pub struct IterationReport {
    /// Iteration number
    pub iteration: usize,
    /// Current compilation rate
    pub compilation_rate: f64,
    /// Target compilation rate
    pub target_rate: f64,
    /// Number of examples
    pub total_examples: usize,
    /// Number of passing examples
    pub passing_examples: usize,
    /// Top error cluster
    pub top_cluster: Option<ErrorClusterSummary>,
}

/// Summary of an error cluster for reporting
#[derive(Debug, Clone)]
pub struct ErrorClusterSummary {
    /// Error code
    pub error_code: String,
    /// Number of examples blocked
    pub examples_blocked: usize,
    /// Fix confidence
    pub fix_confidence: f64,
    /// Root cause description
    pub root_cause_description: String,
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

/// Error frequency entry for failure analysis
#[derive(Debug, Clone)]
pub struct ErrorFrequency {
    /// Error code
    pub code: String,
    /// Number of files affected
    pub count: usize,
    /// Representative error message
    pub sample_message: String,
}

/// Reporter for convergence progress
pub struct ConvergenceReporter {
    /// Verbose mode
    verbose: bool,
    /// Display mode (rich, minimal, json, silent)
    display_mode: DisplayMode,
}

impl ConvergenceReporter {
    /// Create a new reporter
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            display_mode: DisplayMode::Rich,
        }
    }

    /// Create a reporter with specific display mode
    pub fn with_display_mode(display_mode: DisplayMode) -> Self {
        Self {
            verbose: !matches!(display_mode, DisplayMode::Silent),
            display_mode,
        }
    }

    /// Check if output should be shown
    fn should_output(&self) -> bool {
        !matches!(self.display_mode, DisplayMode::Silent | DisplayMode::Json)
    }

    /// Report start of convergence loop
    pub fn report_start(&self, state: &ConvergenceState) {
        if !self.should_output() {
            return;
        }

        match self.display_mode {
            DisplayMode::Rich => {
                println!("╔══════════════════════════════════════════════════════════════╗");
                println!("║               DEPYLER CONVERGENCE LOOP                       ║");
                println!("╠══════════════════════════════════════════════════════════════╣");
                println!(
                    "║ Input Directory: {:43} ║",
                    truncate_path(&state.config.input_dir.display().to_string(), 43)
                );
                println!(
                    "║ Target Rate:     {:6.1}%                                     ║",
                    state.config.target_rate
                );
                println!(
                    "║ Max Iterations:  {:6}                                      ║",
                    state.config.max_iterations
                );
                println!(
                    "║ Auto-fix:        {:6}                                      ║",
                    if state.config.auto_fix { "ON" } else { "OFF" }
                );
                println!("╚══════════════════════════════════════════════════════════════╝");
                println!();
            }
            DisplayMode::Minimal => {
                println!(
                    "CONVERGE | Dir: {} | Target: {:.1}% | Max: {}",
                    truncate_path(&state.config.input_dir.display().to_string(), 30),
                    state.config.target_rate,
                    state.config.max_iterations
                );
            }
            DisplayMode::Json | DisplayMode::Silent => {}
        }
    }

    /// Report iteration progress
    pub fn report_iteration(&self, state: &ConvergenceState, top_cluster: &ErrorCluster) {
        if !self.should_output() {
            return;
        }

        let passing = state.examples.iter().filter(|e| e.compiles).count();
        let total = state.examples.len();

        match self.display_mode {
            DisplayMode::Rich => {
                let prog_bar = progress_bar(state.iteration, state.config.max_iterations, 20);
                let prog_pct = if state.config.max_iterations > 0 {
                    (state.iteration as f64 / state.config.max_iterations as f64) * 100.0
                } else {
                    0.0
                };

                println!("┌──────────────────────────────────────────────────────────────┐");
                println!(
                    "│ [{}] {}/{} ({:.0}%)                              │",
                    prog_bar, state.iteration, state.config.max_iterations, prog_pct
                );
                println!(
                    "│ Rate: {:5.1}% │ Passing: {:4}/{:<4}                           │",
                    state.compilation_rate, passing, total
                );
                println!("├──────────────────────────────────────────────────────────────┤");
                println!(
                    "│ Top Cluster: {} ({} blocked, {:.0}% confidence){}│",
                    top_cluster.error_code,
                    top_cluster.examples_blocked.len(),
                    top_cluster.fix_confidence * 100.0,
                    " ".repeat(20 - top_cluster.error_code.len().min(20))
                );

                if self.verbose {
                    // Show root cause
                    let root_cause_str = match &top_cluster.root_cause {
                        super::clusterer::RootCause::TranspilerGap { gap_type, location } => {
                            format!("{} @ {}", gap_type, location)
                        }
                        super::clusterer::RootCause::Unknown => "unknown".to_string(),
                    };
                    println!("│ Root Cause: {:50} │", truncate(&root_cause_str, 50));

                    // Show sample errors
                    if !top_cluster.sample_errors.is_empty() {
                        println!(
                            "│ Sample Errors:                                               │"
                        );
                        for (i, error) in top_cluster.sample_errors.iter().take(3).enumerate() {
                            println!("│   {}. {}│", i + 1, truncate(&error.message, 55));
                        }
                    }
                }

                println!("└──────────────────────────────────────────────────────────────┘");
                println!();
            }
            DisplayMode::Minimal => {
                // CI-friendly single line with progress
                println!(
                    "[{}/{}] {:.1}% | {}/{} passing | Top: {} ({} blocked)",
                    state.iteration,
                    state.config.max_iterations,
                    state.compilation_rate,
                    passing,
                    total,
                    top_cluster.error_code,
                    top_cluster.examples_blocked.len()
                );
            }
            DisplayMode::Json | DisplayMode::Silent => {}
        }
    }

    /// Format iteration data for external use
    pub fn format_iteration(&self, state: &ConvergenceState, cluster: &ErrorCluster) -> String {
        let passing = state.examples.iter().filter(|e| e.compiles).count();
        let total = state.examples.len();

        format!(
            "Iteration {} | Rate: {:.1}% ({}/{}) | Top: {} ({} blocked)",
            state.iteration,
            state.compilation_rate,
            passing,
            total,
            cluster.error_code,
            cluster.examples_blocked.len()
        )
    }

    /// Report finish of convergence loop
    pub fn report_finish(&self, state: &ConvergenceState) {
        if !self.should_output() {
            return;
        }

        let passing = state.examples.iter().filter(|e| e.compiles).count();
        let total = state.examples.len();
        let reached_target = state.compilation_rate >= state.config.target_rate;

        match self.display_mode {
            DisplayMode::Rich => {
                println!();
                println!("╔══════════════════════════════════════════════════════════════╗");
                println!("║                    CONVERGENCE COMPLETE                      ║");
                println!("╠══════════════════════════════════════════════════════════════╣");
                println!(
                    "║ Status:          {:43} ║",
                    if reached_target {
                        "TARGET REACHED"
                    } else {
                        "TARGET NOT REACHED"
                    }
                );
                println!(
                    "║ Final Rate:      {:5.1}% ({}/{})                              ║",
                    state.compilation_rate, passing, total
                );
                println!(
                    "║ Iterations:      {:6}                                      ║",
                    state.iteration
                );
                println!(
                    "║ Fixes Applied:   {:6}                                      ║",
                    state.fixes_applied.len()
                );
                println!("╚══════════════════════════════════════════════════════════════╝");

                if self.verbose && !state.fixes_applied.is_empty() {
                    println!();
                    println!("Applied Fixes:");
                    for (i, fix) in state.fixes_applied.iter().enumerate() {
                        println!(
                            "  {}. [{}] {} ({})",
                            i + 1,
                            fix.error_code,
                            fix.description,
                            if fix.verified {
                                "verified"
                            } else {
                                "unverified"
                            }
                        );
                    }
                }
            }
            DisplayMode::Minimal => {
                let status = if reached_target {
                    "CONVERGED"
                } else {
                    "NOT_CONVERGED"
                };
                println!(
                    "DONE | {} | {:.1}% ({}/{}) | {} iterations | {} fixes",
                    status,
                    state.compilation_rate,
                    passing,
                    total,
                    state.iteration,
                    state.fixes_applied.len()
                );
            }
            DisplayMode::Json | DisplayMode::Silent => {}
        }

        // Show remaining error clusters (rich mode only)
        if matches!(self.display_mode, DisplayMode::Rich)
            && !reached_target
            && !state.error_clusters.is_empty()
        {
            println!();
            println!("Remaining Error Clusters:");
            for (i, cluster) in state.error_clusters.iter().take(5).enumerate() {
                println!(
                    "  {}. {} - {} examples blocked (confidence: {:.0}%)",
                    i + 1,
                    cluster.error_code,
                    cluster.examples_blocked.len(),
                    cluster.fix_confidence * 100.0
                );
            }
        }
    }

    /// DEPYLER-1321 (Popper): Report escape rate metrics
    /// This implements Popper's falsification criterion for the type system.
    /// If escape_rate > 20%, the type inference is immunizing against falsification.
    pub fn report_escape_rate(&self, tracker: &EscapeRateTracker) {
        if !self.should_output() {
            return;
        }

        let (concrete, dv) = tracker.counts();
        let escape_rate = tracker.escape_rate();
        let total = concrete + dv;
        let is_falsified = tracker.is_falsified();

        match self.display_mode {
            DisplayMode::Rich => {
                println!();
                println!("╔══════════════════════════════════════════════════════════════╗");
                println!("║          DEPYLER-1321: ESCAPE RATE ANALYSIS (Popper)         ║");
                println!("╠══════════════════════════════════════════════════════════════╣");
                println!(
                    "║ Concrete Types:  {:6} ({:5.1}%)                              ║",
                    concrete,
                    if total > 0 {
                        (concrete as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    }
                );
                println!(
                    "║ DepylerValue:    {:6} ({:5.1}%)                              ║",
                    dv,
                    escape_rate * 100.0
                );
                println!(
                    "║ Escape Rate:     {:5.1}%                                       ║",
                    escape_rate * 100.0
                );
                println!(
                    "║ Threshold:       {:5.1}%                                       ║",
                    ESCAPE_RATE_FALSIFICATION_THRESHOLD * 100.0
                );
                println!("╠══════════════════════════════════════════════════════════════╣");

                if is_falsified {
                    println!("║ ⚠️  STATUS: FALSIFIED                                        ║");
                    println!("║                                                              ║");
                    println!("║ Per Karl Popper: Type inference is evading, not solving.    ║");
                    println!("║ DepylerValue acts as a 'protective belt' absorbing type     ║");
                    println!("║ mismatches rather than genuinely inferring correct types.   ║");
                } else {
                    println!("║ ✅  STATUS: OK                                               ║");
                    println!("║                                                              ║");
                    println!("║ Type inference is working within acceptable bounds.         ║");
                }

                println!("╚══════════════════════════════════════════════════════════════╝");
            }
            DisplayMode::Minimal => {
                let status = if is_falsified { "FALSIFIED" } else { "OK" };
                println!(
                    "ESCAPE_RATE | {:.1}% ({}/{}) | Threshold: {:.0}% | Status: {}",
                    escape_rate * 100.0,
                    dv,
                    total,
                    ESCAPE_RATE_FALSIFICATION_THRESHOLD * 100.0,
                    status
                );
            }
            DisplayMode::Json | DisplayMode::Silent => {}
        }
    }

    /// Report failure analysis with Error Frequency Table
    /// This provides automated root cause hinting without requiring log analysis
    pub fn report_failure_analysis(&self, results: &[super::compiler::CompilationResult]) {
        if !self.should_output() {
            return;
        }

        // Aggregate error frequencies
        let frequencies = Self::compute_error_frequencies(results);
        if frequencies.is_empty() {
            return;
        }

        match self.display_mode {
            DisplayMode::Rich => {
                println!();
                println!("╔══════════════════════════════════════════════════════════════╗");
                println!("║                    FAILURES ANALYSIS                         ║");
                println!("╠══════════════════════════════════════════════════════════════╣");

                for (i, freq) in frequencies.iter().take(10).enumerate() {
                    let bar = Self::mini_bar(freq.count, frequencies[0].count, 10);
                    println!(
                        "║ {:2}. {} {:>4} files  [{}]                     ║",
                        i + 1,
                        freq.code,
                        freq.count,
                        bar
                    );
                }

                println!("╠══════════════════════════════════════════════════════════════╣");

                // Top blocker with representative error
                if let Some(top) = frequencies.first() {
                    println!(
                        "║ TOP BLOCKER: {} ({} files)                          ║",
                        top.code, top.count
                    );
                    println!("╠──────────────────────────────────────────────────────────────╣");
                    // Wrap the sample message
                    let msg = &top.sample_message;
                    for line in Self::wrap_text(msg, 58) {
                        println!("║ {}║", Self::pad_right(&line, 60));
                    }
                }

                println!("╚══════════════════════════════════════════════════════════════╝");
            }
            DisplayMode::Minimal => {
                println!();
                println!("=== FAILURES ANALYSIS ===");
                for (i, freq) in frequencies.iter().take(5).enumerate() {
                    println!(
                        "{}. {} ({}): {} files",
                        i + 1,
                        freq.code,
                        Self::error_description(&freq.code),
                        freq.count
                    );
                }
                if let Some(top) = frequencies.first() {
                    println!("Top Blocker: {}", top.code);
                    println!("  Sample: {}", truncate(&top.sample_message, 70));
                }
            }
            DisplayMode::Json | DisplayMode::Silent => {}
        }
    }

    /// Compute error frequencies from compilation results
    pub fn compute_error_frequencies(
        results: &[super::compiler::CompilationResult],
    ) -> Vec<ErrorFrequency> {
        use std::collections::HashMap;

        let mut code_counts: HashMap<String, (usize, String)> = HashMap::new();

        for result in results {
            if result.success {
                continue;
            }
            for error in &result.errors {
                let entry = code_counts
                    .entry(error.code.clone())
                    .or_insert((0, error.message.clone()));
                entry.0 += 1;
            }
        }

        let mut frequencies: Vec<ErrorFrequency> = code_counts
            .into_iter()
            .map(|(code, (count, sample_message))| ErrorFrequency {
                code,
                count,
                sample_message,
            })
            .collect();

        // Sort by count descending
        frequencies.sort_by(|a, b| b.count.cmp(&a.count));
        frequencies
    }

    /// Get human-readable description for error code
    fn error_description(code: &str) -> &'static str {
        match code {
            "E0308" => "Type Mismatch",
            "E0425" => "Undefined Value",
            "E0599" => "Missing Method",
            "E0277" => "Trait Bound",
            "E0412" => "Undefined Type",
            "E0061" => "Arg Count",
            "E0282" => "Type Inference",
            "E0433" => "Unresolved Module",
            "E0562" => "Impl Trait",
            "E0609" => "No Field",
            "TRANSPILE" => "Transpile Error",
            "CARGO" => "Cargo Error",
            "UNKNOWN" => "Unknown",
            _ => "Other",
        }
    }

    /// Mini progress bar for frequencies
    fn mini_bar(value: usize, max: usize, width: usize) -> String {
        if max == 0 {
            return "░".repeat(width);
        }
        let pct = (value as f64 / max as f64).clamp(0.0, 1.0);
        let filled = (pct * width as f64).round() as usize;
        let empty = width.saturating_sub(filled);
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    /// Wrap text to specified width
    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Pad string to right
    fn pad_right(s: &str, width: usize) -> String {
        if s.len() >= width {
            s[..width].to_string()
        } else {
            format!("{:width$}", s, width = width)
        }
    }
}

/// Truncate a string to max length with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Truncate a path string
fn truncate_path(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short     ");
        assert_eq!(
            truncate("this is a very long string", 15),
            "this is a ve..."
        );
    }

    #[test]
    fn test_truncate_exact_length() {
        assert_eq!(truncate("exact", 5), "exact");
    }

    #[test]
    fn test_truncate_path() {
        assert_eq!(truncate_path("short", 10), "short");
        let long = "/very/long/path/to/file.py";
        let truncated = truncate_path(long, 15);
        assert!(truncated.starts_with("..."));
        assert!(truncated.len() <= 15);
    }

    #[test]
    fn test_truncate_path_exact_length() {
        assert_eq!(truncate_path("exact", 5), "exact");
    }

    fn make_test_config() -> super::super::state::ConvergenceConfig {
        super::super::state::ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 100.0,
            max_iterations: 50,
            auto_fix: false,
            dry_run: false,
            verbose: true,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: super::super::state::DisplayMode::default(),
            oracle: false,
            explain: false,
            use_cache: true,
            patch_transpiler: false,
            apr_file: None,
        }
    }

    fn make_test_cluster() -> ErrorCluster {
        ErrorCluster {
            root_cause: super::super::clusterer::RootCause::TranspilerGap {
                gap_type: "missing_method".to_string(),
                location: "expr_gen.rs".to_string(),
            },
            error_code: "E0599".to_string(),
            examples_blocked: vec![PathBuf::from("a.py"), PathBuf::from("b.py")],
            sample_errors: vec![],
            fix_confidence: 0.9,
            suggested_fix: None,
        }
    }

    #[test]
    fn test_reporter_new() {
        let reporter = ConvergenceReporter::new(true);
        assert!(reporter.verbose);
        let reporter2 = ConvergenceReporter::new(false);
        assert!(!reporter2.verbose);
    }

    #[test]
    fn test_reporter_format_iteration() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 5;
        state.compilation_rate = 75.0;

        let cluster = make_test_cluster();

        let report = reporter.format_iteration(&state, &cluster);
        assert!(report.contains("Iteration 5"));
        assert!(report.contains("75.0%"));
        assert!(report.contains("E0599"));
    }

    #[test]
    fn test_iteration_report_struct() {
        let report = IterationReport {
            iteration: 1,
            compilation_rate: 50.0,
            target_rate: 100.0,
            total_examples: 10,
            passing_examples: 5,
            top_cluster: None,
        };
        assert_eq!(report.iteration, 1);
        assert_eq!(report.compilation_rate, 50.0);
    }

    #[test]
    fn test_iteration_report_with_cluster() {
        let summary = ErrorClusterSummary {
            error_code: "E0308".to_string(),
            examples_blocked: 3,
            fix_confidence: 0.85,
            root_cause_description: "type mismatch".to_string(),
        };
        let report = IterationReport {
            iteration: 2,
            compilation_rate: 70.0,
            target_rate: 100.0,
            total_examples: 10,
            passing_examples: 7,
            top_cluster: Some(summary.clone()),
        };
        assert!(report.top_cluster.is_some());
        let cluster = report.top_cluster.unwrap();
        assert_eq!(cluster.error_code, "E0308");
        assert_eq!(cluster.examples_blocked, 3);
    }

    #[test]
    fn test_error_cluster_summary_clone() {
        let summary = ErrorClusterSummary {
            error_code: "E0277".to_string(),
            examples_blocked: 5,
            fix_confidence: 0.9,
            root_cause_description: "trait not implemented".to_string(),
        };
        let cloned = summary.clone();
        assert_eq!(summary.error_code, cloned.error_code);
        assert_eq!(summary.examples_blocked, cloned.examples_blocked);
    }

    #[test]
    fn test_reporter_report_start() {
        let reporter = ConvergenceReporter::new(false);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        // Just verify it doesn't panic
        reporter.report_start(&state);
    }

    #[test]
    fn test_reporter_report_iteration_verbose() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 1;
        let cluster = make_test_cluster();
        // Just verify it doesn't panic
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_report_iteration_non_verbose() {
        let reporter = ConvergenceReporter::new(false);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 1;
        let cluster = make_test_cluster();
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_report_finish_target_reached() {
        let reporter = ConvergenceReporter::new(false);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.compilation_rate = 100.0;
        reporter.report_finish(&state);
    }

    #[test]
    fn test_reporter_report_finish_target_not_reached() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.compilation_rate = 50.0;
        state.error_clusters.push(make_test_cluster());
        reporter.report_finish(&state);
    }

    #[test]
    fn test_reporter_report_finish_with_fixes() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.compilation_rate = 100.0;
        state.fixes_applied.push(super::super::state::AppliedFix {
            iteration: 1,
            error_code: "E0599".to_string(),
            description: "Added method".to_string(),
            file_modified: PathBuf::from("expr_gen.rs"),
            commit_hash: None,
            verified: true,
        });
        reporter.report_finish(&state);
    }

    #[test]
    fn test_reporter_report_iteration_with_sample_errors() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 1;
        let mut cluster = make_test_cluster();
        cluster.sample_errors = vec![super::super::compiler::CompilationError {
            code: "E0599".to_string(),
            message: "method not found".to_string(),
            file: PathBuf::from("test.py"),
            line: 10,
            column: 5,
            ..Default::default()
        }];
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_report_iteration_unknown_root_cause() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 1;
        let cluster = ErrorCluster {
            root_cause: super::super::clusterer::RootCause::Unknown,
            error_code: "E0599".to_string(),
            examples_blocked: vec![],
            sample_errors: vec![],
            fix_confidence: 0.5,
            suggested_fix: None,
        };
        reporter.report_iteration(&state, &cluster);
    }

    // ============================================================================
    // Progress Bar Tests
    // ============================================================================

    #[test]
    fn test_progress_bar_zero_total() {
        let bar = super::progress_bar(0, 0, 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_empty() {
        let bar = super::progress_bar(0, 10, 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_full() {
        let bar = super::progress_bar(10, 10, 10);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_progress_bar_half() {
        let bar = super::progress_bar(5, 10, 10);
        assert_eq!(bar, "█████░░░░░");
    }

    #[test]
    fn test_progress_bar_quarter() {
        let bar = super::progress_bar(25, 100, 20);
        assert_eq!(bar, "█████░░░░░░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_overflow() {
        // current > total should clamp to 100%
        let bar = super::progress_bar(15, 10, 10);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_progress_bar_width_1() {
        let bar = super::progress_bar(5, 10, 1);
        // 50% of width 1 rounds to 1, but chars are UTF-8 (█ = 3 bytes)
        assert_eq!(bar.chars().count(), 1);
    }

    // ============================================================================
    // Display Mode Tests
    // ============================================================================

    #[test]
    fn test_reporter_with_display_mode_rich() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Rich);
        assert!(reporter.verbose);
        assert!(reporter.should_output());
    }

    #[test]
    fn test_reporter_with_display_mode_minimal() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Minimal);
        assert!(reporter.verbose);
        assert!(reporter.should_output());
    }

    #[test]
    fn test_reporter_with_display_mode_json() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Json);
        assert!(reporter.verbose);
        assert!(!reporter.should_output());
    }

    #[test]
    fn test_reporter_with_display_mode_silent() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Silent);
        assert!(!reporter.verbose);
        assert!(!reporter.should_output());
    }

    #[test]
    fn test_reporter_silent_report_start() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Silent);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        // Should not panic and should not output
        reporter.report_start(&state);
    }

    #[test]
    fn test_reporter_silent_report_iteration() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Silent);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        let cluster = make_test_cluster();
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_silent_report_finish() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Silent);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        reporter.report_finish(&state);
    }

    #[test]
    fn test_reporter_json_no_output() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Json);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        let cluster = make_test_cluster();
        // JSON mode should not produce text output
        reporter.report_start(&state);
        reporter.report_iteration(&state, &cluster);
        reporter.report_finish(&state);
    }

    #[test]
    fn test_reporter_minimal_report_start() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Minimal);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        reporter.report_start(&state);
    }

    #[test]
    fn test_reporter_minimal_report_iteration() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Minimal);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        let cluster = make_test_cluster();
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_minimal_report_finish() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Minimal);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        reporter.report_finish(&state);
    }

    #[test]
    fn test_reporter_minimal_report_finish_not_converged() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Minimal);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.compilation_rate = 50.0;
        reporter.report_finish(&state);
    }

    // ============================================================================
    // DEPYLER-UX: Error Frequency / Failure Analysis Tests
    // ============================================================================

    #[test]
    fn test_compute_error_frequencies_empty() {
        let results: Vec<super::super::compiler::CompilationResult> = vec![];
        let frequencies = ConvergenceReporter::compute_error_frequencies(&results);
        assert!(frequencies.is_empty());
    }

    #[test]
    fn test_compute_error_frequencies_all_success() {
        let results = vec![super::super::compiler::CompilationResult {
            source_file: PathBuf::from("a.py"),
            success: true,
            errors: vec![],
            rust_file: Some(PathBuf::from("a.rs")),
        }];
        let frequencies = ConvergenceReporter::compute_error_frequencies(&results);
        assert!(frequencies.is_empty());
    }

    #[test]
    fn test_compute_error_frequencies_single_error() {
        let results = vec![super::super::compiler::CompilationResult {
            source_file: PathBuf::from("a.py"),
            success: false,
            errors: vec![super::super::compiler::CompilationError {
                code: "E0308".to_string(),
                message: "mismatched types".to_string(),
                file: PathBuf::from("a.rs"),
                line: 10,
                column: 5,
                ..Default::default()
            }],
            rust_file: Some(PathBuf::from("a.rs")),
        }];
        let frequencies = ConvergenceReporter::compute_error_frequencies(&results);
        assert_eq!(frequencies.len(), 1);
        assert_eq!(frequencies[0].code, "E0308");
        assert_eq!(frequencies[0].count, 1);
    }

    #[test]
    fn test_compute_error_frequencies_sorted_by_count() {
        let results = vec![
            super::super::compiler::CompilationResult {
                source_file: PathBuf::from("a.py"),
                success: false,
                errors: vec![super::super::compiler::CompilationError {
                    code: "E0599".to_string(),
                    message: "no method".to_string(),
                    file: PathBuf::from("a.rs"),
                    line: 1,
                    column: 1,
                    ..Default::default()
                }],
                rust_file: None,
            },
            super::super::compiler::CompilationResult {
                source_file: PathBuf::from("b.py"),
                success: false,
                errors: vec![
                    super::super::compiler::CompilationError {
                        code: "E0308".to_string(),
                        message: "type mismatch".to_string(),
                        file: PathBuf::from("b.rs"),
                        line: 1,
                        column: 1,
                        ..Default::default()
                    },
                    super::super::compiler::CompilationError {
                        code: "E0308".to_string(),
                        message: "another mismatch".to_string(),
                        file: PathBuf::from("b.rs"),
                        line: 2,
                        column: 1,
                        ..Default::default()
                    },
                ],
                rust_file: None,
            },
        ];
        let frequencies = ConvergenceReporter::compute_error_frequencies(&results);
        assert_eq!(frequencies.len(), 2);
        // E0308 should be first (2 occurrences)
        assert_eq!(frequencies[0].code, "E0308");
        assert_eq!(frequencies[0].count, 2);
        // E0599 should be second (1 occurrence)
        assert_eq!(frequencies[1].code, "E0599");
        assert_eq!(frequencies[1].count, 1);
    }

    #[test]
    fn test_error_frequency_struct() {
        let freq = ErrorFrequency {
            code: "E0308".to_string(),
            count: 42,
            sample_message: "mismatched types".to_string(),
        };
        assert_eq!(freq.code, "E0308");
        assert_eq!(freq.count, 42);
        assert_eq!(freq.sample_message, "mismatched types");
    }

    #[test]
    fn test_error_frequency_clone() {
        let freq = ErrorFrequency {
            code: "E0599".to_string(),
            count: 10,
            sample_message: "method not found".to_string(),
        };
        let cloned = freq.clone();
        assert_eq!(freq.code, cloned.code);
        assert_eq!(freq.count, cloned.count);
    }

    #[test]
    fn test_mini_bar_full() {
        let bar = ConvergenceReporter::mini_bar(100, 100, 10);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_mini_bar_empty() {
        let bar = ConvergenceReporter::mini_bar(0, 100, 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_mini_bar_half() {
        let bar = ConvergenceReporter::mini_bar(50, 100, 10);
        assert_eq!(bar, "█████░░░░░");
    }

    #[test]
    fn test_mini_bar_zero_max() {
        let bar = ConvergenceReporter::mini_bar(50, 0, 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_wrap_text_short() {
        let lines = ConvergenceReporter::wrap_text("short text", 50);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "short text");
    }

    #[test]
    fn test_wrap_text_long() {
        let lines = ConvergenceReporter::wrap_text("this is a very long text that should wrap", 20);
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_wrap_text_empty() {
        let lines = ConvergenceReporter::wrap_text("", 50);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].is_empty());
    }

    #[test]
    fn test_pad_right_short() {
        let padded = ConvergenceReporter::pad_right("short", 10);
        assert_eq!(padded.len(), 10);
        assert!(padded.starts_with("short"));
    }

    #[test]
    fn test_pad_right_exact() {
        let padded = ConvergenceReporter::pad_right("exact", 5);
        assert_eq!(padded, "exact");
    }

    #[test]
    fn test_pad_right_long() {
        let padded = ConvergenceReporter::pad_right("verylongtext", 5);
        assert_eq!(padded.len(), 5);
    }

    #[test]
    fn test_error_description_known_codes() {
        assert_eq!(
            ConvergenceReporter::error_description("E0308"),
            "Type Mismatch"
        );
        assert_eq!(
            ConvergenceReporter::error_description("E0425"),
            "Undefined Value"
        );
        assert_eq!(
            ConvergenceReporter::error_description("E0599"),
            "Missing Method"
        );
        assert_eq!(
            ConvergenceReporter::error_description("E0277"),
            "Trait Bound"
        );
    }

    #[test]
    fn test_error_description_unknown_code() {
        assert_eq!(ConvergenceReporter::error_description("E9999"), "Other");
    }

    #[test]
    fn test_report_failure_analysis_silent_mode() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Silent);
        let results: Vec<super::super::compiler::CompilationResult> = vec![];
        // Should not panic
        reporter.report_failure_analysis(&results);
    }

    #[test]
    fn test_report_failure_analysis_json_mode() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Json);
        let results: Vec<super::super::compiler::CompilationResult> = vec![];
        // Should not panic
        reporter.report_failure_analysis(&results);
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[test]
    fn test_truncate_empty() {
        assert_eq!(super::truncate("", 10), "          ");
    }

    #[test]
    fn test_truncate_path_empty() {
        assert_eq!(super::truncate_path("", 10), "");
    }

    #[test]
    fn test_iteration_report_debug() {
        let report = IterationReport {
            iteration: 1,
            compilation_rate: 50.0,
            target_rate: 100.0,
            total_examples: 10,
            passing_examples: 5,
            top_cluster: None,
        };
        let debug = format!("{:?}", report);
        assert!(debug.contains("iteration"));
        assert!(debug.contains("compilation_rate"));
    }

    #[test]
    fn test_iteration_report_clone() {
        let report = IterationReport {
            iteration: 1,
            compilation_rate: 50.0,
            target_rate: 100.0,
            total_examples: 10,
            passing_examples: 5,
            top_cluster: None,
        };
        let cloned = report.clone();
        assert_eq!(report.iteration, cloned.iteration);
        assert_eq!(report.compilation_rate, cloned.compilation_rate);
    }

    #[test]
    fn test_error_cluster_summary_debug() {
        let summary = ErrorClusterSummary {
            error_code: "E0308".to_string(),
            examples_blocked: 3,
            fix_confidence: 0.85,
            root_cause_description: "type mismatch".to_string(),
        };
        let debug = format!("{:?}", summary);
        assert!(debug.contains("E0308"));
        assert!(debug.contains("examples_blocked"));
    }

    #[test]
    fn test_reporter_rich_with_zero_max_iterations() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Rich);
        let mut config = make_test_config();
        config.max_iterations = 0;
        // Can't create state with invalid config, so use a valid one
        // and manually set iteration to test edge case
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 0;
        // Set max_iterations to 0 after creation for testing
        state.config.max_iterations = 0;
        let cluster = make_test_cluster();
        // Should handle division by zero gracefully
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_rich_with_long_error_code() {
        let reporter = ConvergenceReporter::with_display_mode(DisplayMode::Rich);
        let state = super::super::state::ConvergenceState::new(make_test_config());
        let cluster = ErrorCluster {
            root_cause: super::super::clusterer::RootCause::TranspilerGap {
                gap_type: "very_long_gap_type_name_that_exceeds_typical_lengths".to_string(),
                location: "very_long_location_path_for_testing.rs".to_string(),
            },
            error_code: "E0599_EXTENDED_ERROR_CODE".to_string(),
            examples_blocked: vec![],
            sample_errors: vec![],
            fix_confidence: 0.5,
            suggested_fix: None,
        };
        reporter.report_iteration(&state, &cluster);
    }

    #[test]
    fn test_reporter_format_iteration_with_examples() {
        let reporter = ConvergenceReporter::new(true);
        let mut state = super::super::state::ConvergenceState::new(make_test_config());
        state.iteration = 3;
        state.compilation_rate = 80.0;
        state.examples = vec![
            super::super::state::ExampleState::new(PathBuf::from("a.py"), true),
            super::super::state::ExampleState::new(PathBuf::from("b.py"), true),
            super::super::state::ExampleState::new(PathBuf::from("c.py"), false),
        ];
        let cluster = make_test_cluster();
        let report = reporter.format_iteration(&state, &cluster);
        assert!(report.contains("Iteration 3"));
        assert!(report.contains("2/3"));
    }
}

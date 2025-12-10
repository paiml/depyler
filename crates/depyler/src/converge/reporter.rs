//! Progress reporting for convergence loop
//!
//! Provides formatted output for tracking convergence progress.
//! Supports multiple display modes: rich (TUI), minimal (CI), json, silent.

use super::clusterer::ErrorCluster;
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
                println!("║ Target Rate:     {:6.1}%                                     ║", state.config.target_rate);
                println!("║ Max Iterations:  {:6}                                      ║", state.config.max_iterations);
                println!("║ Auto-fix:        {:6}                                      ║", if state.config.auto_fix { "ON" } else { "OFF" });
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
                        println!("│ Sample Errors:                                               │");
                        for (i, error) in top_cluster.sample_errors.iter().take(3).enumerate() {
                            println!(
                                "│   {}. {}│",
                                i + 1,
                                truncate(&error.message, 55)
                            );
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
                    if reached_target { "TARGET REACHED" } else { "TARGET NOT REACHED" }
                );
                println!("║ Final Rate:      {:5.1}% ({}/{})                              ║",
                    state.compilation_rate, passing, total);
                println!("║ Iterations:      {:6}                                      ║", state.iteration);
                println!("║ Fixes Applied:   {:6}                                      ║", state.fixes_applied.len());
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
                            if fix.verified { "verified" } else { "unverified" }
                        );
                    }
                }
            }
            DisplayMode::Minimal => {
                let status = if reached_target { "CONVERGED" } else { "NOT_CONVERGED" };
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
        if matches!(self.display_mode, DisplayMode::Rich) && !reached_target && !state.error_clusters.is_empty() {
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
        assert_eq!(truncate("this is a very long string", 15), "this is a ve...");
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
        cluster.sample_errors = vec![
            super::super::compiler::CompilationError {
                code: "E0599".to_string(),
                message: "method not found".to_string(),
                file: PathBuf::from("test.py"),
                line: 10,
                column: 5,
            },
        ];
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
}

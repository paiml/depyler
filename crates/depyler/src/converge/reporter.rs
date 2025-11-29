//! Progress reporting for convergence loop
//!
//! Provides formatted output for tracking convergence progress.

use super::clusterer::ErrorCluster;
use super::state::ConvergenceState;

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

/// Reporter for convergence progress
pub struct ConvergenceReporter {
    /// Verbose mode
    verbose: bool,
}

impl ConvergenceReporter {
    /// Create a new reporter
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Report start of convergence loop
    pub fn report_start(&self, state: &ConvergenceState) {
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

    /// Report iteration progress
    pub fn report_iteration(&self, state: &ConvergenceState, top_cluster: &ErrorCluster) {
        let passing = state.examples.iter().filter(|e| e.compiles).count();
        let total = state.examples.len();

        println!("┌──────────────────────────────────────────────────────────────┐");
        println!(
            "│ Iteration {:3} │ Rate: {:5.1}% │ Passing: {:4}/{:<4}           │",
            state.iteration, state.compilation_rate, passing, total
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
        let passing = state.examples.iter().filter(|e| e.compiles).count();
        let total = state.examples.len();
        let reached_target = state.compilation_rate >= state.config.target_rate;

        println!();
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                    CONVERGENCE COMPLETE                      ║");
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!(
            "║ Status:          {:43} ║",
            if reached_target { "✅ TARGET REACHED" } else { "⚠️  TARGET NOT REACHED" }
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

        // Show remaining error clusters
        if !reached_target && !state.error_clusters.is_empty() {
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
    fn test_reporter_format_iteration() {
        let reporter = ConvergenceReporter::new(true);
        let config = super::super::state::ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 100.0,
            max_iterations: 50,
            auto_fix: false,
            dry_run: false,
            verbose: true,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
        };

        let mut state = super::super::state::ConvergenceState::new(config);
        state.iteration = 5;
        state.compilation_rate = 75.0;

        let cluster = ErrorCluster {
            root_cause: super::super::clusterer::RootCause::TranspilerGap {
                gap_type: "missing_method".to_string(),
                location: "expr_gen.rs".to_string(),
            },
            error_code: "E0599".to_string(),
            examples_blocked: vec![PathBuf::from("a.py"), PathBuf::from("b.py")],
            sample_errors: vec![],
            fix_confidence: 0.9,
            suggested_fix: None,
        };

        let report = reporter.format_iteration(&state, &cluster);
        assert!(report.contains("Iteration 5"));
        assert!(report.contains("75.0%"));
        assert!(report.contains("E0599"));
    }
}

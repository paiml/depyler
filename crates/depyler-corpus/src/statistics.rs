//! Statistical analysis module (Phase 4b).
//!
//! Computes statistical metrics for corpus analysis.
//! Based on software metrics research [Basili et al., 1996].

use crate::compiler::CompilationResult;
use crate::taxonomy::ErrorTaxonomy;
use serde::{Deserialize, Serialize};

/// Statistical analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    /// Total files analyzed.
    pub total_files: usize,
    /// Files that compiled successfully.
    pub passed_files: usize,
    /// Files that failed to compile.
    pub failed_files: usize,
    /// Single-shot compilation rate (percentage).
    pub single_shot_rate: f64,
    /// 95% confidence interval (lower bound).
    pub ci_95_lower: f64,
    /// 95% confidence interval (upper bound).
    pub ci_95_upper: f64,
    /// Mean errors per failed file.
    pub mean_errors_per_file: f64,
    /// Standard deviation of errors.
    pub std_deviation: f64,
    /// Median errors per failed file.
    pub median_errors: f64,
    /// Total errors across all files.
    pub total_errors: usize,
}

impl StatisticalAnalysis {
    /// Compute statistics from compilation results and taxonomy.
    pub fn compute(results: &[CompilationResult], taxonomy: &ErrorTaxonomy) -> Self {
        let total_files = results.len();
        let passed_files = results.iter().filter(|r| r.success).count();
        let failed_files = total_files - passed_files;

        let single_shot_rate = if total_files > 0 {
            (passed_files as f64 / total_files as f64) * 100.0
        } else {
            0.0
        };

        // Wilson score interval for binomial proportion (95% CI)
        let (ci_95_lower, ci_95_upper) = Self::wilson_score_interval(
            passed_files,
            total_files,
            1.96, // z-score for 95% CI
        );

        // Error statistics
        let total_errors = taxonomy.errors.len();
        let errors_per_file = Self::compute_errors_per_file(results, taxonomy);

        let mean_errors_per_file = if !errors_per_file.is_empty() {
            errors_per_file.iter().sum::<f64>() / errors_per_file.len() as f64
        } else {
            0.0
        };

        let std_deviation = Self::standard_deviation(&errors_per_file, mean_errors_per_file);
        let median_errors = Self::median(&errors_per_file);

        Self {
            total_files,
            passed_files,
            failed_files,
            single_shot_rate,
            ci_95_lower: ci_95_lower * 100.0,
            ci_95_upper: ci_95_upper * 100.0,
            mean_errors_per_file,
            std_deviation,
            median_errors,
            total_errors,
        }
    }

    /// Wilson score interval for binomial proportion.
    /// More accurate than normal approximation for small samples.
    fn wilson_score_interval(successes: usize, total: usize, z: f64) -> (f64, f64) {
        if total == 0 {
            return (0.0, 0.0);
        }

        let n = total as f64;
        let p = successes as f64 / n;
        let z2 = z * z;

        let denominator = 1.0 + z2 / n;
        let center = p + z2 / (2.0 * n);
        let margin = z * (p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt();

        let lower = (center - margin) / denominator;
        let upper = (center + margin) / denominator;

        (lower.max(0.0), upper.min(1.0))
    }

    /// Compute errors per file for failed compilations.
    fn compute_errors_per_file(
        results: &[CompilationResult],
        taxonomy: &ErrorTaxonomy,
    ) -> Vec<f64> {
        // Group errors by file
        let mut error_counts = std::collections::HashMap::new();

        for error in &taxonomy.errors {
            *error_counts.entry(error.file.clone()).or_insert(0usize) += 1;
        }

        // For failed files, count errors
        results
            .iter()
            .filter(|r| !r.success)
            .map(|r| {
                let file = r.rust_file.to_string_lossy().to_string();
                error_counts.get(&file).copied().unwrap_or(1) as f64
            })
            .collect()
    }

    /// Calculate standard deviation.
    fn standard_deviation(values: &[f64], mean: f64) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;

        variance.sqrt()
    }

    /// Calculate median.
    fn median(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<f64> = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mid = sorted.len() / 2;
        if sorted.len().is_multiple_of(2) {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    /// Check if rate meets target (Andon alert).
    pub fn meets_target(&self, target: f64) -> bool {
        self.single_shot_rate >= target
    }

    /// Get Andon status based on rate.
    pub fn andon_status(&self, target: f64) -> AndonStatus {
        if self.single_shot_rate >= target {
            AndonStatus::Green
        } else if self.single_shot_rate >= target * 0.5 {
            AndonStatus::Yellow
        } else {
            AndonStatus::Red
        }
    }
}

/// Andon (アンドン) status for quality alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AndonStatus {
    /// Target met.
    Green,
    /// Below target but above 50%.
    Yellow,
    /// Critical - below 50% of target.
    Red,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_results(pass: usize, fail: usize) -> Vec<CompilationResult> {
        let mut results = Vec::new();

        for i in 0..pass {
            results.push(CompilationResult {
                rust_file: std::path::PathBuf::from(format!("pass_{i}.rs")),
                python_file: std::path::PathBuf::from(format!("pass_{i}.py")),
                success: true,
                exit_code: Some(0),
                stderr: None,
                stdout: None,
                duration: std::time::Duration::from_millis(100),
                cargo_first: true,
            });
        }

        for i in 0..fail {
            results.push(CompilationResult {
                rust_file: std::path::PathBuf::from(format!("fail_{i}.rs")),
                python_file: std::path::PathBuf::from(format!("fail_{i}.py")),
                success: false,
                exit_code: Some(1),
                stderr: Some("error[E0308]: mismatched types".to_string()),
                stdout: None,
                duration: std::time::Duration::from_millis(50),
                cargo_first: true,
            });
        }

        results
    }

    fn create_test_taxonomy() -> ErrorTaxonomy {
        ErrorTaxonomy {
            errors: vec![],
            by_category: std::collections::HashMap::new(),
            by_code: std::collections::HashMap::new(),
            blockers: vec![],
        }
    }

    #[test]
    fn test_single_shot_rate_calculation() {
        let results = create_test_results(84, 160);
        let taxonomy = create_test_taxonomy();
        let stats = StatisticalAnalysis::compute(&results, &taxonomy);

        assert_eq!(stats.total_files, 244);
        assert_eq!(stats.passed_files, 84);
        assert_eq!(stats.failed_files, 160);

        // 84/244 = 34.43%
        assert!((stats.single_shot_rate - 34.43).abs() < 0.5);
    }

    #[test]
    fn test_wilson_score_interval() {
        let (lower, upper) = StatisticalAnalysis::wilson_score_interval(84, 244, 1.96);

        // 95% CI for p=0.344 should be approximately [0.285, 0.407]
        assert!(lower > 0.25);
        assert!(upper < 0.45);
        assert!(lower < upper);
    }

    #[test]
    fn test_wilson_score_interval_edge_cases() {
        // Empty case
        let (lower, upper) = StatisticalAnalysis::wilson_score_interval(0, 0, 1.96);
        assert_eq!(lower, 0.0);
        assert_eq!(upper, 0.0);

        // All success
        let (lower, upper) = StatisticalAnalysis::wilson_score_interval(100, 100, 1.96);
        assert!(upper <= 1.0);
        assert!(lower > 0.9);

        // All failure
        let (lower, upper) = StatisticalAnalysis::wilson_score_interval(0, 100, 1.96);
        assert!(lower >= 0.0);
        assert!(upper < 0.1);
    }

    #[test]
    fn test_median_odd() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let median = StatisticalAnalysis::median(&values);
        assert_eq!(median, 3.0);
    }

    #[test]
    fn test_median_even() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let median = StatisticalAnalysis::median(&values);
        assert_eq!(median, 2.5);
    }

    #[test]
    fn test_standard_deviation() {
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let mean = 5.0;
        let std = StatisticalAnalysis::standard_deviation(&values, mean);

        // σ ≈ 2.0
        assert!((std - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_andon_status() {
        let results = create_test_results(84, 160);
        let taxonomy = create_test_taxonomy();
        let stats = StatisticalAnalysis::compute(&results, &taxonomy);

        // Target 80%, rate ~34% → Red (< 40% of target)
        assert_eq!(stats.andon_status(80.0), AndonStatus::Red);

        // Target 30%, rate ~34% → Green
        assert_eq!(stats.andon_status(30.0), AndonStatus::Green);

        // Target 50%, rate ~34% → Yellow (68% of target)
        assert_eq!(stats.andon_status(50.0), AndonStatus::Yellow);
    }

    #[test]
    fn test_meets_target() {
        let results = create_test_results(84, 160);
        let taxonomy = create_test_taxonomy();
        let stats = StatisticalAnalysis::compute(&results, &taxonomy);

        assert!(!stats.meets_target(80.0));
        assert!(stats.meets_target(30.0));
        assert!(stats.meets_target(34.0));
    }
}

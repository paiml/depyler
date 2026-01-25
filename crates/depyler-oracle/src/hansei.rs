//! Hansei (反省) Post-Transpilation Analysis
//!
//! Toyota Way principle: Reflection and continuous improvement through
//! systematic analysis of transpilation outcomes.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────┐     ┌──────────────────┐
//! │  Transpilation      │────►│  HanseiAnalyzer  │
//! │  Results            │     │  (Analysis)      │
//! └─────────────────────┘     └──────────────────┘
//!                                      │
//!                                      ▼
//!                              ┌──────────────────┐
//!                              │  HanseiReport    │
//!                              │  (Issues/Recs)   │
//!                              └──────────────────┘
//! ```
//!
//! # References
//!
//! - Liker, J.K. (2004). The Toyota Way: 14 Management Principles
//! - entrenar `monitor/report.rs` - HanseiAnalyzer pattern

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Severity level for identified issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Informational - minor optimization opportunity
    Info,
    /// Warning - potential issue, not blocking
    Warning,
    /// Error - significant issue affecting functionality
    Error,
    /// Critical - blocking issue requiring immediate attention
    Critical,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Info => write!(f, "INFO"),
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Error => write!(f, "ERROR"),
            IssueSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Trend direction for a metric or category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    /// Metric is improving (errors decreasing, success rate increasing)
    Improving,
    /// Metric is degrading (errors increasing, success rate decreasing)
    Degrading,
    /// Metric is stable (minimal change)
    Stable,
    /// Metric is oscillating (high variance)
    Oscillating,
}

impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trend::Improving => write!(f, "↑ Improving"),
            Trend::Degrading => write!(f, "↓ Degrading"),
            Trend::Stable => write!(f, "→ Stable"),
            Trend::Oscillating => write!(f, "~ Oscillating"),
        }
    }
}

/// An identified issue from transpilation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspileIssue {
    /// Severity level
    pub severity: IssueSeverity,
    /// Issue category (e.g., "async_await", "type_inference")
    pub category: String,
    /// Human-readable description
    pub description: String,
    /// Actionable recommendation
    pub recommendation: String,
    /// Number of occurrences
    pub occurrences: usize,
    /// Suspiciousness score from CITL (0.0-1.0)
    pub suspiciousness: f32,
}

/// Summary statistics for an error category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    /// Total occurrences
    pub count: usize,
    /// Success rate (0.0-1.0)
    pub success_rate: f32,
    /// Suspiciousness score from Tarantula
    pub suspiciousness: f32,
    /// Trend over time
    pub trend: Trend,
    /// Percentage of total failures
    pub failure_share: f32,
}

/// Result of a single transpilation attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspileOutcome {
    /// Python feature category
    pub category: String,
    /// Whether transpilation succeeded
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Python features detected in source
    pub python_features: Vec<String>,
}

/// Post-transpilation analysis report (Hansei)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HanseiReport {
    /// Report identifier
    pub report_id: String,
    /// Total transpilation attempts analyzed
    pub total_attempts: usize,
    /// Successful transpilations
    pub successes: usize,
    /// Failed transpilations
    pub failures: usize,
    /// Overall success rate
    pub success_rate: f32,
    /// Category summaries
    pub category_summaries: HashMap<String, CategorySummary>,
    /// Identified issues sorted by severity
    pub issues: Vec<TranspileIssue>,
    /// General recommendations
    pub recommendations: Vec<String>,
    /// Pareto analysis: categories causing 80% of failures
    pub pareto_categories: Vec<String>,
}

impl HanseiReport {
    /// Get issues filtered by minimum severity
    #[must_use]
    pub fn issues_by_severity(&self, min_severity: IssueSeverity) -> Vec<&TranspileIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity >= min_severity)
            .collect()
    }

    /// Get critical and error issues only
    #[must_use]
    pub fn blocking_issues(&self) -> Vec<&TranspileIssue> {
        self.issues_by_severity(IssueSeverity::Error)
    }

    /// Format report as human-readable text
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("# Hansei Report: {}\n\n", self.report_id));
        output.push_str(&format!(
            "## Summary\n- Total: {}\n- Success: {} ({:.1}%)\n- Failures: {}\n\n",
            self.total_attempts,
            self.successes,
            self.success_rate * 100.0,
            self.failures
        ));

        if !self.pareto_categories.is_empty() {
            output.push_str("## Pareto Analysis (80% of failures)\n");
            for cat in &self.pareto_categories {
                if let Some(summary) = self.category_summaries.get(cat) {
                    output.push_str(&format!(
                        "- {}: {:.1}% of failures (suspiciousness: {:.3})\n",
                        cat,
                        summary.failure_share * 100.0,
                        summary.suspiciousness
                    ));
                }
            }
            output.push('\n');
        }

        if !self.issues.is_empty() {
            output.push_str("## Issues\n");
            for issue in &self.issues {
                output.push_str(&format!(
                    "[{}] {}: {} ({} occurrences)\n  → {}\n",
                    issue.severity,
                    issue.category,
                    issue.description,
                    issue.occurrences,
                    issue.recommendation
                ));
            }
            output.push('\n');
        }

        if !self.recommendations.is_empty() {
            output.push_str("## Recommendations\n");
            for (i, rec) in self.recommendations.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, rec));
            }
        }

        output
    }
}

/// Configuration for Hansei analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HanseiConfig {
    /// Suspiciousness threshold for Critical severity (default: 0.9)
    pub critical_threshold: f32,
    /// Suspiciousness threshold for Error severity (default: 0.7)
    pub error_threshold: f32,
    /// Suspiciousness threshold for Warning severity (default: 0.5)
    pub warning_threshold: f32,
    /// Minimum occurrences to report an issue (default: 3)
    pub min_occurrences: usize,
    /// Coefficient of variation threshold for Oscillating trend (default: 0.5)
    pub oscillation_threshold: f32,
    /// Change threshold for Stable trend (default: 0.05)
    pub stability_threshold: f32,
}

impl Default for HanseiConfig {
    fn default() -> Self {
        Self {
            critical_threshold: 0.9,
            error_threshold: 0.7,
            warning_threshold: 0.5,
            min_occurrences: 3,
            oscillation_threshold: 0.5,
            stability_threshold: 0.05,
        }
    }
}

/// Hansei analyzer for transpilation results
///
/// Analyzes transpilation outcomes to identify patterns, categorize issues,
/// and generate actionable recommendations.
pub struct TranspileHanseiAnalyzer {
    config: HanseiConfig,
}

impl TranspileHanseiAnalyzer {
    /// Create a new analyzer with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(HanseiConfig::default())
    }

    /// Create with custom configuration
    #[must_use]
    pub fn with_config(config: HanseiConfig) -> Self {
        Self { config }
    }

    /// Analyze transpilation outcomes and generate a Hansei report
    ///
    /// # Arguments
    ///
    /// * `report_id` - Identifier for this report
    /// * `outcomes` - Vector of transpilation outcomes to analyze
    /// * `suspiciousness_scores` - Optional CITL suspiciousness scores per category
    ///
    /// # Returns
    ///
    /// A comprehensive HanseiReport with issues and recommendations.
    pub fn analyze(
        &self,
        report_id: &str,
        outcomes: &[TranspileOutcome],
        suspiciousness_scores: Option<&HashMap<String, f32>>,
    ) -> HanseiReport {
        if outcomes.is_empty() {
            return self.empty_report(report_id);
        }

        // Count successes and failures per category
        let mut category_stats: HashMap<String, (usize, usize)> = HashMap::new();
        let mut feature_failures: HashMap<String, usize> = HashMap::new();

        for outcome in outcomes {
            let entry = category_stats
                .entry(outcome.category.clone())
                .or_insert((0, 0));
            if outcome.success {
                entry.0 += 1;
            } else {
                entry.1 += 1;
                // Track which Python features appear in failures
                for feature in &outcome.python_features {
                    *feature_failures.entry(feature.clone()).or_insert(0) += 1;
                }
            }
        }

        let total_attempts = outcomes.len();
        let successes = outcomes.iter().filter(|o| o.success).count();
        let failures = total_attempts - successes;
        let success_rate = successes as f32 / total_attempts as f32;

        // Build category summaries
        let category_summaries =
            self.build_category_summaries(&category_stats, failures, suspiciousness_scores);

        // Perform Pareto analysis
        let pareto_categories = self.pareto_analysis(&category_summaries, failures);

        // Identify issues
        let issues = self.identify_issues(
            &category_summaries,
            &feature_failures,
            suspiciousness_scores,
        );

        // Generate recommendations
        let recommendations = self.generate_recommendations(&issues, success_rate);

        HanseiReport {
            report_id: report_id.to_string(),
            total_attempts,
            successes,
            failures,
            success_rate,
            category_summaries,
            issues,
            recommendations,
            pareto_categories,
        }
    }

    fn empty_report(&self, report_id: &str) -> HanseiReport {
        HanseiReport {
            report_id: report_id.to_string(),
            total_attempts: 0,
            successes: 0,
            failures: 0,
            success_rate: 0.0,
            category_summaries: HashMap::new(),
            issues: Vec::new(),
            recommendations: vec!["No transpilation data to analyze".to_string()],
            pareto_categories: Vec::new(),
        }
    }

    fn build_category_summaries(
        &self,
        category_stats: &HashMap<String, (usize, usize)>,
        total_failures: usize,
        suspiciousness_scores: Option<&HashMap<String, f32>>,
    ) -> HashMap<String, CategorySummary> {
        let mut summaries = HashMap::new();

        for (category, (successes, failures)) in category_stats {
            let total = successes + failures;
            let success_rate = *successes as f32 / total as f32;
            let failure_share = if total_failures > 0 {
                *failures as f32 / total_failures as f32
            } else {
                0.0
            };

            let suspiciousness = suspiciousness_scores
                .and_then(|scores| scores.get(category))
                .copied()
                .unwrap_or_else(|| self.estimate_suspiciousness(success_rate));

            // Determine trend (simplified - would need historical data for real trend)
            let trend = self.determine_trend(success_rate, suspiciousness);

            summaries.insert(
                category.clone(),
                CategorySummary {
                    count: total,
                    success_rate,
                    suspiciousness,
                    trend,
                    failure_share,
                },
            );
        }

        summaries
    }

    fn estimate_suspiciousness(&self, success_rate: f32) -> f32 {
        // Simple heuristic: lower success rate = higher suspiciousness
        1.0 - success_rate
    }

    fn determine_trend(&self, success_rate: f32, suspiciousness: f32) -> Trend {
        // Simplified trend detection (real implementation would track history)
        if success_rate > 0.9 {
            Trend::Improving
        } else if suspiciousness > self.config.critical_threshold {
            Trend::Degrading
        } else if (success_rate - 0.5).abs() < self.config.stability_threshold {
            Trend::Oscillating
        } else {
            Trend::Stable
        }
    }

    fn pareto_analysis(
        &self,
        summaries: &HashMap<String, CategorySummary>,
        total_failures: usize,
    ) -> Vec<String> {
        if total_failures == 0 {
            return Vec::new();
        }

        // Sort categories by failure share descending
        let mut categories: Vec<_> = summaries
            .iter()
            .filter(|(_, s)| s.failure_share > 0.0)
            .collect();
        categories.sort_by(|a, b| {
            b.1.failure_share
                .partial_cmp(&a.1.failure_share)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Collect categories until we reach 80% of failures
        let mut pareto = Vec::new();
        let mut cumulative_share = 0.0;

        for (category, summary) in categories {
            pareto.push(category.clone());
            cumulative_share += summary.failure_share;
            if cumulative_share >= 0.8 {
                break;
            }
        }

        pareto
    }

    fn identify_issues(
        &self,
        summaries: &HashMap<String, CategorySummary>,
        feature_failures: &HashMap<String, usize>,
        suspiciousness_scores: Option<&HashMap<String, f32>>,
    ) -> Vec<TranspileIssue> {
        let mut issues = Vec::new();

        // Issues from category summaries
        for (category, summary) in summaries {
            if summary.count < self.config.min_occurrences {
                continue;
            }

            let failure_count = ((1.0 - summary.success_rate) * summary.count as f32) as usize;
            if failure_count == 0 {
                continue;
            }

            let severity = self.severity_from_suspiciousness(summary.suspiciousness);
            let recommendation = self.recommendation_for_category(category, summary);

            issues.push(TranspileIssue {
                severity,
                category: category.clone(),
                description: format!(
                    "{} has {:.1}% failure rate",
                    category,
                    (1.0 - summary.success_rate) * 100.0
                ),
                recommendation,
                occurrences: failure_count,
                suspiciousness: summary.suspiciousness,
            });
        }

        // Issues from Python features
        for (feature, &count) in feature_failures {
            if count < self.config.min_occurrences {
                continue;
            }

            let suspiciousness = suspiciousness_scores
                .and_then(|s| s.get(feature))
                .copied()
                .unwrap_or(0.5);

            let severity = self.severity_from_suspiciousness(suspiciousness);

            issues.push(TranspileIssue {
                severity,
                category: feature.clone(),
                description: format!("Python feature '{}' appears in {} failures", feature, count),
                recommendation: format!("Implement or improve support for '{}'", feature),
                occurrences: count,
                suspiciousness,
            });
        }

        // Sort by severity (Critical first) then by occurrences
        issues.sort_by(|a, b| {
            b.severity
                .cmp(&a.severity)
                .then_with(|| b.occurrences.cmp(&a.occurrences))
        });

        // Deduplicate by category (keep highest severity)
        let mut seen_categories = std::collections::HashSet::new();
        issues.retain(|issue| seen_categories.insert(issue.category.clone()));

        issues
    }

    fn severity_from_suspiciousness(&self, suspiciousness: f32) -> IssueSeverity {
        if suspiciousness >= self.config.critical_threshold {
            IssueSeverity::Critical
        } else if suspiciousness >= self.config.error_threshold {
            IssueSeverity::Error
        } else if suspiciousness >= self.config.warning_threshold {
            IssueSeverity::Warning
        } else {
            IssueSeverity::Info
        }
    }

    fn recommendation_for_category(&self, category: &str, summary: &CategorySummary) -> String {
        let lower = category.to_lowercase();

        if lower.contains("async") || lower.contains("await") {
            return "Implement async/await transpilation using tokio runtime".to_string();
        }
        if lower.contains("generator") || lower.contains("yield") {
            return "Add Iterator trait implementation for generator patterns".to_string();
        }
        if lower.contains("lambda") {
            return "Map lambda expressions to Rust closures".to_string();
        }
        if lower.contains("class") {
            return "Improve struct generation from class definitions".to_string();
        }
        if lower.contains("context") || lower.contains("with") {
            return "Implement Drop trait for context manager patterns".to_string();
        }
        if lower.contains("stdin") || lower.contains("io") {
            return "Add std::io integration for I/O operations".to_string();
        }

        // Default recommendation based on trend
        match summary.trend {
            Trend::Degrading => {
                format!("URGENT: {} is regressing. Review recent changes.", category)
            }
            Trend::Oscillating => {
                format!("Stabilize {} implementation to reduce variance.", category)
            }
            _ => format!(
                "Improve {} support (current success rate: {:.1}%)",
                category,
                summary.success_rate * 100.0
            ),
        }
    }

    fn generate_recommendations(
        &self,
        issues: &[TranspileIssue],
        success_rate: f32,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Overall success rate recommendation
        if success_rate < 0.5 {
            recommendations.push(
                "CRITICAL: Overall success rate below 50%. Focus on high-suspiciousness features."
                    .to_string(),
            );
        } else if success_rate < 0.8 {
            recommendations
                .push("Target 80% success rate by addressing Pareto categories.".to_string());
        }

        // Count issues by severity
        let critical_count = issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Critical)
            .count();
        let error_count = issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .count();

        if critical_count > 0 {
            recommendations.push(format!(
                "Address {} critical issues before adding new features.",
                critical_count
            ));
        }

        if error_count > 3 {
            recommendations
                .push("Consider batch-fixing related error categories together.".to_string());
        }

        // Feature-specific recommendations from top issues
        for issue in issues.iter().take(3) {
            if issue.severity >= IssueSeverity::Error {
                recommendations.push(issue.recommendation.clone());
            }
        }

        recommendations
    }
}

impl Default for TranspileHanseiAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// EXTREME TDD Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // IssueSeverity Tests
    // ========================================================================

    #[test]
    fn test_severity_ordering() {
        assert!(IssueSeverity::Critical > IssueSeverity::Error);
        assert!(IssueSeverity::Error > IssueSeverity::Warning);
        assert!(IssueSeverity::Warning > IssueSeverity::Info);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(IssueSeverity::Critical.to_string(), "CRITICAL");
        assert_eq!(IssueSeverity::Error.to_string(), "ERROR");
        assert_eq!(IssueSeverity::Warning.to_string(), "WARNING");
        assert_eq!(IssueSeverity::Info.to_string(), "INFO");
    }

    // ========================================================================
    // Trend Tests
    // ========================================================================

    #[test]
    fn test_trend_display() {
        assert!(Trend::Improving.to_string().contains("Improving"));
        assert!(Trend::Degrading.to_string().contains("Degrading"));
        assert!(Trend::Stable.to_string().contains("Stable"));
        assert!(Trend::Oscillating.to_string().contains("Oscillating"));
    }

    // ========================================================================
    // HanseiConfig Tests
    // ========================================================================

    #[test]
    fn test_config_default() {
        let config = HanseiConfig::default();
        assert_eq!(config.critical_threshold, 0.9);
        assert_eq!(config.error_threshold, 0.7);
        assert_eq!(config.warning_threshold, 0.5);
        assert_eq!(config.min_occurrences, 3);
    }

    // ========================================================================
    // TranspileHanseiAnalyzer Construction Tests
    // ========================================================================

    #[test]
    fn test_analyzer_new() {
        let analyzer = TranspileHanseiAnalyzer::new();
        assert_eq!(analyzer.config.critical_threshold, 0.9);
    }

    #[test]
    fn test_analyzer_with_config() {
        let config = HanseiConfig {
            critical_threshold: 0.95,
            ..Default::default()
        };
        let analyzer = TranspileHanseiAnalyzer::with_config(config);
        assert_eq!(analyzer.config.critical_threshold, 0.95);
    }

    // ========================================================================
    // Empty Analysis Tests
    // ========================================================================

    #[test]
    fn test_analyze_empty_outcomes() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let report = analyzer.analyze("empty-test", &[], None);

        assert_eq!(report.total_attempts, 0);
        assert_eq!(report.successes, 0);
        assert_eq!(report.failures, 0);
        assert!(report.issues.is_empty());
        assert!(!report.recommendations.is_empty()); // Should have "no data" message
    }

    // ========================================================================
    // Basic Analysis Tests
    // ========================================================================

    #[test]
    fn test_analyze_all_successes() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let outcomes = vec![
            TranspileOutcome {
                category: "simple".to_string(),
                success: true,
                error_message: None,
                python_features: vec![],
            },
            TranspileOutcome {
                category: "simple".to_string(),
                success: true,
                error_message: None,
                python_features: vec![],
            },
        ];

        let report = analyzer.analyze("success-test", &outcomes, None);

        assert_eq!(report.total_attempts, 2);
        assert_eq!(report.successes, 2);
        assert_eq!(report.failures, 0);
        assert_eq!(report.success_rate, 1.0);
    }

    #[test]
    fn test_analyze_all_failures() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let outcomes: Vec<TranspileOutcome> = (0..5)
            .map(|_| TranspileOutcome {
                category: "async_test".to_string(),
                success: false,
                error_message: Some("async not supported".to_string()),
                python_features: vec!["async_await".to_string()],
            })
            .collect();

        let report = analyzer.analyze("failure-test", &outcomes, None);

        assert_eq!(report.total_attempts, 5);
        assert_eq!(report.failures, 5);
        assert_eq!(report.success_rate, 0.0);
        assert!(!report.issues.is_empty());
    }

    #[test]
    fn test_analyze_mixed_results() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let mut outcomes = Vec::new();

        // 7 successes
        for _ in 0..7 {
            outcomes.push(TranspileOutcome {
                category: "simple".to_string(),
                success: true,
                error_message: None,
                python_features: vec![],
            });
        }

        // 3 failures
        for _ in 0..3 {
            outcomes.push(TranspileOutcome {
                category: "complex".to_string(),
                success: false,
                error_message: Some("error".to_string()),
                python_features: vec!["lambda".to_string()],
            });
        }

        let report = analyzer.analyze("mixed-test", &outcomes, None);

        assert_eq!(report.total_attempts, 10);
        assert_eq!(report.successes, 7);
        assert_eq!(report.failures, 3);
        assert!((report.success_rate - 0.7).abs() < 0.001);
    }

    // ========================================================================
    // Severity Classification Tests
    // ========================================================================

    #[test]
    fn test_severity_from_suspiciousness_critical() {
        let analyzer = TranspileHanseiAnalyzer::new();
        assert_eq!(
            analyzer.severity_from_suspiciousness(0.95),
            IssueSeverity::Critical
        );
    }

    #[test]
    fn test_severity_from_suspiciousness_error() {
        let analyzer = TranspileHanseiAnalyzer::new();
        assert_eq!(
            analyzer.severity_from_suspiciousness(0.75),
            IssueSeverity::Error
        );
    }

    #[test]
    fn test_severity_from_suspiciousness_warning() {
        let analyzer = TranspileHanseiAnalyzer::new();
        assert_eq!(
            analyzer.severity_from_suspiciousness(0.55),
            IssueSeverity::Warning
        );
    }

    #[test]
    fn test_severity_from_suspiciousness_info() {
        let analyzer = TranspileHanseiAnalyzer::new();
        assert_eq!(
            analyzer.severity_from_suspiciousness(0.3),
            IssueSeverity::Info
        );
    }

    // ========================================================================
    // Pareto Analysis Tests
    // ========================================================================

    #[test]
    fn test_pareto_analysis_single_category() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let mut summaries = HashMap::new();
        summaries.insert(
            "only_category".to_string(),
            CategorySummary {
                count: 10,
                success_rate: 0.0,
                suspiciousness: 1.0,
                trend: Trend::Degrading,
                failure_share: 1.0,
            },
        );

        let pareto = analyzer.pareto_analysis(&summaries, 10);
        assert_eq!(pareto.len(), 1);
        assert_eq!(pareto[0], "only_category");
    }

    #[test]
    fn test_pareto_analysis_80_20() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let mut summaries = HashMap::new();

        // Category A: 80% of failures
        summaries.insert(
            "category_a".to_string(),
            CategorySummary {
                count: 80,
                success_rate: 0.0,
                suspiciousness: 0.9,
                trend: Trend::Degrading,
                failure_share: 0.8,
            },
        );

        // Category B: 15% of failures
        summaries.insert(
            "category_b".to_string(),
            CategorySummary {
                count: 15,
                success_rate: 0.0,
                suspiciousness: 0.7,
                trend: Trend::Stable,
                failure_share: 0.15,
            },
        );

        // Category C: 5% of failures
        summaries.insert(
            "category_c".to_string(),
            CategorySummary {
                count: 5,
                success_rate: 0.0,
                suspiciousness: 0.5,
                trend: Trend::Stable,
                failure_share: 0.05,
            },
        );

        let pareto = analyzer.pareto_analysis(&summaries, 100);

        // Should only include category_a (80% >= 80% threshold)
        assert_eq!(pareto.len(), 1);
        assert_eq!(pareto[0], "category_a");
    }

    // ========================================================================
    // Suspiciousness Score Integration Tests
    // ========================================================================

    #[test]
    fn test_analyze_with_suspiciousness_scores() {
        let analyzer = TranspileHanseiAnalyzer::new();

        let outcomes: Vec<TranspileOutcome> = (0..10)
            .map(|i| TranspileOutcome {
                category: "async_feature".to_string(),
                success: i < 1, // Only 1 success
                error_message: if i >= 1 {
                    Some("async not supported".to_string())
                } else {
                    None
                },
                python_features: vec!["async_await".to_string()],
            })
            .collect();

        let mut scores = HashMap::new();
        scores.insert("async_await".to_string(), 0.95);

        let report = analyzer.analyze("suspiciousness-test", &outcomes, Some(&scores));

        // Should have critical issue due to high suspiciousness
        let critical_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Critical)
            .collect();

        assert!(!critical_issues.is_empty());
    }

    // ========================================================================
    // Report Output Tests
    // ========================================================================

    #[test]
    fn test_report_to_text() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let outcomes: Vec<TranspileOutcome> = (0..5)
            .map(|_| TranspileOutcome {
                category: "test_cat".to_string(),
                success: false,
                error_message: Some("error".to_string()),
                python_features: vec!["feature".to_string()],
            })
            .collect();

        let report = analyzer.analyze("text-test", &outcomes, None);
        let text = report.to_text();

        assert!(text.contains("Hansei Report"));
        assert!(text.contains("text-test"));
        assert!(text.contains("Summary"));
    }

    #[test]
    fn test_report_issues_by_severity() {
        let report = HanseiReport {
            report_id: "test".to_string(),
            total_attempts: 10,
            successes: 5,
            failures: 5,
            success_rate: 0.5,
            category_summaries: HashMap::new(),
            issues: vec![
                TranspileIssue {
                    severity: IssueSeverity::Critical,
                    category: "critical_cat".to_string(),
                    description: "Critical issue".to_string(),
                    recommendation: "Fix immediately".to_string(),
                    occurrences: 5,
                    suspiciousness: 0.95,
                },
                TranspileIssue {
                    severity: IssueSeverity::Warning,
                    category: "warning_cat".to_string(),
                    description: "Warning issue".to_string(),
                    recommendation: "Consider fixing".to_string(),
                    occurrences: 2,
                    suspiciousness: 0.55,
                },
            ],
            recommendations: Vec::new(),
            pareto_categories: Vec::new(),
        };

        let critical_only = report.issues_by_severity(IssueSeverity::Critical);
        assert_eq!(critical_only.len(), 1);

        let error_and_above = report.issues_by_severity(IssueSeverity::Error);
        assert_eq!(error_and_above.len(), 1); // Only Critical >= Error

        let all_issues = report.issues_by_severity(IssueSeverity::Info);
        assert_eq!(all_issues.len(), 2);
    }

    #[test]
    fn test_report_blocking_issues() {
        let report = HanseiReport {
            report_id: "test".to_string(),
            total_attempts: 10,
            successes: 5,
            failures: 5,
            success_rate: 0.5,
            category_summaries: HashMap::new(),
            issues: vec![
                TranspileIssue {
                    severity: IssueSeverity::Critical,
                    category: "critical".to_string(),
                    description: "Critical".to_string(),
                    recommendation: "Fix".to_string(),
                    occurrences: 5,
                    suspiciousness: 0.95,
                },
                TranspileIssue {
                    severity: IssueSeverity::Error,
                    category: "error".to_string(),
                    description: "Error".to_string(),
                    recommendation: "Fix".to_string(),
                    occurrences: 3,
                    suspiciousness: 0.75,
                },
                TranspileIssue {
                    severity: IssueSeverity::Warning,
                    category: "warning".to_string(),
                    description: "Warning".to_string(),
                    recommendation: "Consider".to_string(),
                    occurrences: 2,
                    suspiciousness: 0.55,
                },
            ],
            recommendations: Vec::new(),
            pareto_categories: Vec::new(),
        };

        let blocking = report.blocking_issues();
        assert_eq!(blocking.len(), 2); // Critical and Error
    }

    // ========================================================================
    // Recommendation Generation Tests
    // ========================================================================

    #[test]
    fn test_recommendation_for_async() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let summary = CategorySummary {
            count: 10,
            success_rate: 0.1,
            suspiciousness: 0.9,
            trend: Trend::Degrading,
            failure_share: 0.5,
        };

        let rec = analyzer.recommendation_for_category("async_handler", &summary);
        assert!(rec.contains("async") || rec.contains("tokio"));
    }

    #[test]
    fn test_recommendation_for_generator() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let summary = CategorySummary {
            count: 10,
            success_rate: 0.1,
            suspiciousness: 0.9,
            trend: Trend::Degrading,
            failure_share: 0.5,
        };

        let rec = analyzer.recommendation_for_category("generator_test", &summary);
        assert!(rec.contains("Iterator") || rec.contains("generator"));
    }

    #[test]
    fn test_generate_recommendations_low_success() {
        let analyzer = TranspileHanseiAnalyzer::new();
        let issues = vec![TranspileIssue {
            severity: IssueSeverity::Critical,
            category: "test".to_string(),
            description: "Test".to_string(),
            recommendation: "Fix it".to_string(),
            occurrences: 10,
            suspiciousness: 0.95,
        }];

        let recs = analyzer.generate_recommendations(&issues, 0.3);

        // Should have recommendation about low success rate
        assert!(recs
            .iter()
            .any(|r| r.contains("50%") || r.contains("CRITICAL")));
    }

    // ========================================================================
    // Property Tests
    // ========================================================================

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_success_rate_bounded(
            successes in 0usize..100,
            failures in 0usize..100
        ) {
            let total = successes + failures;
            if total == 0 {
                return Ok(());
            }

            let analyzer = TranspileHanseiAnalyzer::new();
            let mut outcomes = Vec::new();

            for _ in 0..successes {
                outcomes.push(TranspileOutcome {
                    category: "test".to_string(),
                    success: true,
                    error_message: None,
                    python_features: vec![],
                });
            }

            for _ in 0..failures {
                outcomes.push(TranspileOutcome {
                    category: "test".to_string(),
                    success: false,
                    error_message: Some("error".to_string()),
                    python_features: vec![],
                });
            }

            let report = analyzer.analyze("prop-test", &outcomes, None);

            prop_assert!(report.success_rate >= 0.0);
            prop_assert!(report.success_rate <= 1.0);
            prop_assert_eq!(report.successes + report.failures, report.total_attempts);
        }

        #[test]
        fn prop_pareto_categories_subset(
            n_categories in 1usize..10
        ) {
            let analyzer = TranspileHanseiAnalyzer::new();
            let mut outcomes = Vec::new();

            for i in 0..n_categories {
                // Each category has some failures
                for _ in 0..5 {
                    outcomes.push(TranspileOutcome {
                        category: format!("cat_{}", i),
                        success: false,
                        error_message: Some("error".to_string()),
                        python_features: vec![],
                    });
                }
            }

            let report = analyzer.analyze("pareto-test", &outcomes, None);

            // Pareto categories should be subset of all categories
            for cat in &report.pareto_categories {
                prop_assert!(report.category_summaries.contains_key(cat));
            }
        }

        #[test]
        fn prop_issues_sorted_by_severity(
            n_outcomes in 1usize..50
        ) {
            let analyzer = TranspileHanseiAnalyzer::new();
            let outcomes: Vec<TranspileOutcome> = (0..n_outcomes)
                .map(|i| TranspileOutcome {
                    category: format!("cat_{}", i % 5),
                    success: i % 3 == 0,
                    error_message: if i % 3 != 0 { Some("error".to_string()) } else { None },
                    python_features: vec![format!("feature_{}", i % 3)],
                })
                .collect();

            let report = analyzer.analyze("sorted-test", &outcomes, None);

            // Verify issues are sorted by severity (descending)
            for window in report.issues.windows(2) {
                prop_assert!(window[0].severity >= window[1].severity);
            }
        }

        #[test]
        fn prop_suspiciousness_bounded(suspiciousness in 0.0f32..1.0f32) {
            let analyzer = TranspileHanseiAnalyzer::new();
            let severity = analyzer.severity_from_suspiciousness(suspiciousness);

            // Should always return a valid severity
            prop_assert!(matches!(
                severity,
                IssueSeverity::Info
                    | IssueSeverity::Warning
                    | IssueSeverity::Error
                    | IssueSeverity::Critical
            ));
        }
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_full_analysis_pipeline() {
        let analyzer = TranspileHanseiAnalyzer::new();

        // Simulate corpus similar to reprorusted
        let mut outcomes = Vec::new();

        // async_await: 5% success (high suspiciousness)
        for i in 0..20 {
            outcomes.push(TranspileOutcome {
                category: "async_example".to_string(),
                success: i == 0,
                error_message: if i > 0 {
                    Some("async not supported".to_string())
                } else {
                    None
                },
                python_features: vec!["async_await".to_string()],
            });
        }

        // generator: 10% success
        for i in 0..10 {
            outcomes.push(TranspileOutcome {
                category: "generator_example".to_string(),
                success: i == 0,
                error_message: if i > 0 {
                    Some("yield not supported".to_string())
                } else {
                    None
                },
                python_features: vec!["generator".to_string()],
            });
        }

        // simple: 90% success
        for i in 0..10 {
            outcomes.push(TranspileOutcome {
                category: "simple_example".to_string(),
                success: i < 9,
                error_message: if i >= 9 {
                    Some("edge case".to_string())
                } else {
                    None
                },
                python_features: vec![],
            });
        }

        // Provide suspiciousness scores from CITL
        let mut scores = HashMap::new();
        scores.insert("async_await".to_string(), 0.946);
        scores.insert("generator".to_string(), 0.927);

        let report = analyzer.analyze("full-pipeline", &outcomes, Some(&scores));

        // Verify report structure
        assert_eq!(report.total_attempts, 40);
        assert!(report.success_rate < 0.5); // Should be around 27.5%

        // Verify Pareto analysis caught high-failure categories
        assert!(!report.pareto_categories.is_empty());

        // Verify issues include critical ones for high-suspiciousness features
        let critical_issues: Vec<_> = report
            .issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Critical)
            .collect();
        assert!(!critical_issues.is_empty());

        // Verify text output
        let text = report.to_text();
        assert!(text.contains("Hansei Report"));
        assert!(text.contains("CRITICAL") || text.contains("ERROR"));
    }
}

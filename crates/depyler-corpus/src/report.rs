//! Report generation module (Phase 5).
//!
//! Generates deterministic reports in multiple formats.

use crate::compiler::CompilationResult;
use crate::config::CorpusConfig;
use crate::statistics::{AndonStatus, StatisticalAnalysis};
use crate::taxonomy::{BlockerPriority, ErrorTaxonomy};
use crate::transpiler::TranspilationResult;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;

/// Output format for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    /// JSON format (deterministic, machine-readable).
    Json,
    /// Markdown format (human-readable).
    Markdown,
    /// Terminal output (ASCII/ANSI).
    Terminal,
}

/// The complete corpus analysis report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusReport {
    /// Report metadata.
    pub metadata: ReportMetadata,
    /// Summary statistics.
    pub summary: ReportSummary,
    /// Error distribution.
    pub error_distribution: ErrorDistribution,
    /// Blocker analysis.
    pub blocker_analysis: BlockerAnalysis,
    /// Statistical analysis.
    pub statistical_analysis: StatisticalAnalysis,
    /// Toyota Way metrics.
    pub toyota_way_metrics: ToyotaWayMetrics,
}

/// Report metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Generation timestamp.
    pub generated_at: String,
    /// Corpus name.
    pub corpus_name: String,
    /// Corpus hash.
    pub corpus_hash: String,
    /// Depyler version.
    pub depyler_version: String,
    /// Report hash.
    pub report_hash: String,
}

/// Report summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    /// Total Python files.
    pub total_python_files: usize,
    /// Transpilation results.
    pub transpilation: TranspilationSummary,
    /// Compilation results.
    pub compilation: CompilationSummary,
    /// Single-shot rate.
    pub single_shot_rate: f64,
    /// 95% confidence interval.
    pub confidence_interval_95: (f64, f64),
}

/// Transpilation summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilationSummary {
    pub success: usize,
    pub failure: usize,
    pub rate: f64,
}

/// Compilation summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationSummary {
    pub success: usize,
    pub failure: usize,
    pub rate: f64,
}

/// Error distribution data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDistribution {
    /// Errors by category.
    pub by_category: Vec<CategoryCount>,
    /// Errors by code.
    pub by_error_code: Vec<ErrorCodeCount>,
}

/// Category count entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: usize,
    pub percentage: f64,
}

/// Error code count entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCodeCount {
    pub code: String,
    pub count: usize,
    pub description: String,
}

/// Blocker analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockerAnalysis {
    pub p0_critical: Vec<BlockerEntry>,
    pub p1_high: Vec<BlockerEntry>,
    pub p2_medium: Vec<BlockerEntry>,
    pub p3_low: Vec<BlockerEntry>,
}

/// Single blocker entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockerEntry {
    pub error_code: String,
    pub count: usize,
    pub root_cause: String,
    pub recommended_fix: String,
}

/// Toyota Way metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToyotaWayMetrics {
    /// Number of Jidoka alerts triggered.
    pub jidoka_alerts: usize,
    /// Number of Andon triggers.
    pub andon_triggers: usize,
    /// Number of Kaizen opportunities identified.
    pub kaizen_opportunities: usize,
    /// Hansei items (lessons learned).
    pub hansei_items: Vec<String>,
}

impl CorpusReport {
    /// Create a new corpus report from analysis results.
    pub fn new(
        config: &CorpusConfig,
        transpile_results: Vec<TranspilationResult>,
        compile_results: Vec<CompilationResult>,
        taxonomy: ErrorTaxonomy,
        statistics: StatisticalAnalysis,
    ) -> Self {
        let transpile_success = transpile_results.iter().filter(|r| r.success).count();
        let compile_success = compile_results.iter().filter(|r| r.success).count();

        Self {
            metadata: ReportMetadata {
                generated_at: Utc::now().to_rfc3339(),
                corpus_name: config.corpus_name.clone(),
                corpus_hash: String::new(), // Computed later
                depyler_version: env!("CARGO_PKG_VERSION").to_string(),
                report_hash: String::new(), // Computed at serialization
            },
            summary: ReportSummary {
                total_python_files: transpile_results.len(),
                transpilation: TranspilationSummary {
                    success: transpile_success,
                    failure: transpile_results.len() - transpile_success,
                    rate: if !transpile_results.is_empty() {
                        (transpile_success as f64 / transpile_results.len() as f64) * 100.0
                    } else {
                        0.0
                    },
                },
                compilation: CompilationSummary {
                    success: compile_success,
                    failure: compile_results.len() - compile_success,
                    rate: statistics.single_shot_rate,
                },
                single_shot_rate: statistics.single_shot_rate,
                confidence_interval_95: (statistics.ci_95_lower, statistics.ci_95_upper),
            },
            error_distribution: Self::build_error_distribution(&taxonomy),
            blocker_analysis: Self::build_blocker_analysis(&taxonomy),
            statistical_analysis: statistics.clone(),
            toyota_way_metrics: Self::build_toyota_metrics(&statistics, &taxonomy, config.target_rate),
        }
    }

    fn build_error_distribution(taxonomy: &ErrorTaxonomy) -> ErrorDistribution {
        let total_errors: usize = taxonomy.by_category.values().sum();

        let by_category: Vec<CategoryCount> = taxonomy
            .by_category
            .iter()
            .map(|(cat, &count)| CategoryCount {
                category: format!("{cat:?}"),
                count,
                percentage: if total_errors > 0 {
                    (count as f64 / total_errors as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        let by_error_code: Vec<ErrorCodeCount> = taxonomy
            .by_code
            .iter()
            .map(|(code, &count)| ErrorCodeCount {
                code: code.clone(),
                count,
                description: Self::error_code_description(code),
            })
            .collect();

        ErrorDistribution {
            by_category,
            by_error_code,
        }
    }

    fn build_blocker_analysis(taxonomy: &ErrorTaxonomy) -> BlockerAnalysis {
        let mut p0 = Vec::new();
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        let mut p3 = Vec::new();

        for blocker in &taxonomy.blockers {
            let entry = BlockerEntry {
                error_code: blocker.error_code.clone(),
                count: blocker.count,
                root_cause: blocker.root_cause.clone(),
                recommended_fix: blocker.recommended_fix.clone(),
            };

            match blocker.priority {
                BlockerPriority::P0Critical => p0.push(entry),
                BlockerPriority::P1High => p1.push(entry),
                BlockerPriority::P2Medium => p2.push(entry),
                BlockerPriority::P3Low => p3.push(entry),
            }
        }

        BlockerAnalysis {
            p0_critical: p0,
            p1_high: p1,
            p2_medium: p2,
            p3_low: p3,
        }
    }

    fn build_toyota_metrics(
        stats: &StatisticalAnalysis,
        taxonomy: &ErrorTaxonomy,
        target_rate: f64,
    ) -> ToyotaWayMetrics {
        let andon_status = stats.andon_status(target_rate);

        ToyotaWayMetrics {
            jidoka_alerts: taxonomy
                .blockers
                .iter()
                .filter(|b| b.priority == BlockerPriority::P0Critical)
                .count(),
            andon_triggers: if andon_status == AndonStatus::Red { 1 } else { 0 },
            kaizen_opportunities: taxonomy
                .blockers
                .iter()
                .filter(|b| matches!(b.priority, BlockerPriority::P1High | BlockerPriority::P2Medium))
                .count(),
            hansei_items: Self::generate_hansei_items(taxonomy),
        }
    }

    fn generate_hansei_items(taxonomy: &ErrorTaxonomy) -> Vec<String> {
        let mut items = Vec::new();

        if taxonomy.by_code.get("E0308").copied().unwrap_or(0) > 10 {
            items.push("Type inference needs improvement".to_string());
        }
        if taxonomy.by_code.get("E0412").copied().unwrap_or(0) > 10 {
            items.push("Generic type resolution needs work".to_string());
        }
        if taxonomy.by_code.get("E0425").copied().unwrap_or(0) > 10 {
            items.push("Import/binding generation needs review".to_string());
        }

        items
    }

    fn error_code_description(code: &str) -> String {
        match code {
            "E0308" => "mismatched types".to_string(),
            "E0412" => "cannot find type".to_string(),
            "E0425" => "cannot find value".to_string(),
            "E0282" => "type annotations needed".to_string(),
            "E0277" => "trait not implemented".to_string(),
            _ => "other error".to_string(),
        }
    }

    /// Get the single-shot compilation rate.
    pub fn single_shot_rate(&self) -> f64 {
        self.summary.single_shot_rate
    }

    /// Export to JSON format.
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Export to Markdown format.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str(&format!(
            "# Corpus Analysis Report: {}\n\n",
            self.metadata.corpus_name
        ));
        md.push_str(&format!(
            "**Generated**: {}\n",
            self.metadata.generated_at
        ));
        md.push_str(&format!(
            "**Depyler Version**: {}\n\n",
            self.metadata.depyler_version
        ));

        md.push_str("## Executive Summary\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!(
            "| Single-Shot Rate | {:.1}% |\n",
            self.summary.single_shot_rate
        ));
        md.push_str(&format!(
            "| 95% CI | [{:.1}%, {:.1}%] |\n",
            self.summary.confidence_interval_95.0, self.summary.confidence_interval_95.1
        ));
        md.push_str(&format!(
            "| Total Files | {} |\n",
            self.summary.total_python_files
        ));
        md.push_str(&format!(
            "| Compiled Successfully | {} |\n",
            self.summary.compilation.success
        ));
        md.push_str(&format!(
            "| Compilation Failures | {} |\n\n",
            self.summary.compilation.failure
        ));

        md.push_str("## Error Distribution\n\n");
        md.push_str("| Error Code | Count | Description |\n");
        md.push_str("|------------|-------|-------------|\n");
        for err in &self.error_distribution.by_error_code {
            md.push_str(&format!("| {} | {} | {} |\n", err.code, err.count, err.description));
        }
        md.push('\n');

        md.push_str("## Top Blockers\n\n");
        if !self.blocker_analysis.p0_critical.is_empty() {
            md.push_str("### P0 Critical\n\n");
            for b in &self.blocker_analysis.p0_critical {
                md.push_str(&format!("- **{}** ({} occurrences): {}\n", b.error_code, b.count, b.root_cause));
            }
            md.push('\n');
        }
        if !self.blocker_analysis.p1_high.is_empty() {
            md.push_str("### P1 High\n\n");
            for b in &self.blocker_analysis.p1_high {
                md.push_str(&format!("- **{}** ({} occurrences): {}\n", b.error_code, b.count, b.root_cause));
            }
            md.push('\n');
        }

        md.push_str("## Toyota Way Metrics\n\n");
        md.push_str(&format!(
            "- Jidoka Alerts: {}\n",
            self.toyota_way_metrics.jidoka_alerts
        ));
        md.push_str(&format!(
            "- Andon Triggers: {}\n",
            self.toyota_way_metrics.andon_triggers
        ));
        md.push_str(&format!(
            "- Kaizen Opportunities: {}\n\n",
            self.toyota_way_metrics.kaizen_opportunities
        ));

        if !self.toyota_way_metrics.hansei_items.is_empty() {
            md.push_str("### Hansei (反省) - Lessons Learned\n\n");
            for item in &self.toyota_way_metrics.hansei_items {
                md.push_str(&format!("- {item}\n"));
            }
        }

        md
    }

    /// Export to terminal format.
    pub fn to_terminal(&self) -> String {
        let mut out = String::new();

        out.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        out.push_str(&format!(
            "║  CORPUS ANALYSIS: {:<42} ║\n",
            self.metadata.corpus_name
        ));
        out.push_str("╠══════════════════════════════════════════════════════════════╣\n");
        out.push_str(&format!(
            "║  Single-Shot Rate:        {:>5.1}%                             ║\n",
            self.summary.single_shot_rate
        ));
        out.push_str(&format!(
            "║  95% CI:                  [{:>4.1}%, {:>4.1}%]                       ║\n",
            self.summary.confidence_interval_95.0, self.summary.confidence_interval_95.1
        ));
        out.push_str(&format!(
            "║  Total Files:             {:>5}                              ║\n",
            self.summary.total_python_files
        ));
        out.push_str(&format!(
            "║  ✓ Compiled OK:           {:>5}                              ║\n",
            self.summary.compilation.success
        ));
        out.push_str(&format!(
            "║  ✗ Compilation Failed:    {:>5}                              ║\n",
            self.summary.compilation.failure
        ));
        out.push_str("╚══════════════════════════════════════════════════════════════╝\n");

        out
    }

    /// Write report to file.
    pub fn write_to_file(&self, path: &Path, format: ReportFormat) -> anyhow::Result<()> {
        let content = match format {
            ReportFormat::Json => self.to_json()?,
            ReportFormat::Markdown => self.to_markdown(),
            ReportFormat::Terminal => self.to_terminal(),
        };

        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_report() -> CorpusReport {
        let config = CorpusConfig::default();
        let transpile_results = vec![
            TranspilationResult {
                python_file: std::path::PathBuf::from("a.py"),
                rust_file: Some(std::path::PathBuf::from("a.rs")),
                cargo_dir: Some(std::path::PathBuf::from(".")),
                success: true,
                error: None,
                duration: std::time::Duration::from_millis(100),
            },
        ];
        let compile_results = vec![
            CompilationResult {
                rust_file: std::path::PathBuf::from("a.rs"),
                python_file: std::path::PathBuf::from("a.py"),
                success: true,
                exit_code: Some(0),
                stderr: None,
                stdout: None,
                duration: std::time::Duration::from_millis(50),
                cargo_first: true,
            },
        ];
        let taxonomy = ErrorTaxonomy {
            errors: vec![],
            by_category: HashMap::new(),
            by_code: HashMap::new(),
            blockers: vec![],
        };
        let statistics = StatisticalAnalysis {
            total_files: 1,
            passed_files: 1,
            failed_files: 0,
            single_shot_rate: 100.0,
            ci_95_lower: 5.0,
            ci_95_upper: 100.0,
            mean_errors_per_file: 0.0,
            std_deviation: 0.0,
            median_errors: 0.0,
            total_errors: 0,
        };

        CorpusReport::new(&config, transpile_results, compile_results, taxonomy, statistics)
    }

    #[test]
    fn test_report_creation() {
        let report = create_test_report();
        assert_eq!(report.summary.single_shot_rate, 100.0);
        assert_eq!(report.summary.total_python_files, 1);
    }

    #[test]
    fn test_to_json() {
        let report = create_test_report();
        let json = report.to_json().unwrap();

        assert!(json.contains("single_shot_rate"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_to_markdown() {
        let report = create_test_report();
        let md = report.to_markdown();

        assert!(md.contains("# Corpus Analysis Report"));
        assert!(md.contains("Single-Shot Rate"));
        assert!(md.contains("100.0%"));
    }

    #[test]
    fn test_to_terminal() {
        let report = create_test_report();
        let term = report.to_terminal();

        assert!(term.contains("CORPUS ANALYSIS"));
        assert!(term.contains("Single-Shot Rate"));
    }

    #[test]
    fn test_single_shot_rate() {
        let report = create_test_report();
        assert_eq!(report.single_shot_rate(), 100.0);
    }
}

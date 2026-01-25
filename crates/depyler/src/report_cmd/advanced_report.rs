//! Advanced Report Output (GH-209 Phase 5)
//!
//! Rich ML-powered report with clustering, graph analysis, and domain breakdown.
//! Enabled via `depyler report --advanced`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::analysis::{
    analyze_extended_results, DomainBreakdown, ErrorEntry, ExtendedAnalysisResult, SemanticDomain,
};
use super::clustering::{ClusterAnalysis, ClusterConfig, ErrorClusterAnalyzer};
use super::graph_analysis::GraphAnalysis;

/// Advanced report configuration (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReportConfig {
    /// Include clustering analysis
    pub include_clustering: bool,
    /// Include graph analysis
    pub include_graph: bool,
    /// Number of top errors to highlight
    pub top_errors: usize,
    /// Number of clusters to show
    pub top_clusters: usize,
    /// Number of communities to show
    pub top_communities: usize,
}

impl Default for AdvancedReportConfig {
    fn default() -> Self {
        Self {
            include_clustering: true,
            include_graph: true,
            top_errors: 10,
            top_clusters: 5,
            top_communities: 5,
        }
    }
}

/// Summary section of advanced report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    /// Total files analyzed
    pub total_files: usize,
    /// Files that compiled successfully
    pub passed: usize,
    /// Files that failed to compile
    pub failed: usize,
    /// Overall pass rate (0-100)
    pub pass_rate: f64,
    /// Andon status: GREEN, YELLOW, or RED
    pub status: String,
}

impl ReportSummary {
    pub fn from_counts(passed: usize, failed: usize) -> Self {
        let total_files = passed + failed;
        let pass_rate = if total_files > 0 {
            (passed as f64 / total_files as f64) * 100.0
        } else {
            0.0
        };

        let status = if pass_rate >= 80.0 {
            "GREEN"
        } else if pass_rate >= 50.0 {
            "YELLOW"
        } else {
            "RED"
        }
        .to_string();

        Self {
            total_files,
            passed,
            failed,
            pass_rate,
            status,
        }
    }
}

/// Error breakdown entry for report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBreakdownEntry {
    /// Error code (e.g., "E0308")
    pub code: String,
    /// Human-readable description
    pub description: String,
    /// Number of occurrences
    pub count: usize,
    /// Percentage of total failures
    pub percentage: f64,
    /// Sample file names
    pub samples: Vec<String>,
    /// Priority level: P0-CRITICAL, P1-HIGH, P2-MEDIUM, P3-LOW
    pub priority: String,
}

/// Domain statistics entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStats {
    /// Domain name
    pub name: String,
    /// Number passed
    pub passed: usize,
    /// Number failed
    pub failed: usize,
    /// Pass rate (0-100)
    pub pass_rate: f64,
}

/// Full advanced report structure (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReport {
    /// Report metadata
    pub version: String,
    /// Summary statistics
    pub summary: ReportSummary,
    /// Domain breakdown
    pub domain_breakdown: Vec<DomainStats>,
    /// Top errors by count
    pub error_breakdown: Vec<ErrorBreakdownEntry>,
    /// ML clustering results (optional)
    pub clusters: Option<Vec<ClusterSummary>>,
    /// Graph analysis results (optional)
    pub graph: Option<GraphSummary>,
}

/// Simplified cluster summary for report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSummary {
    /// Cluster ID
    pub id: usize,
    /// Auto-generated label
    pub label: String,
    /// Number of files in cluster
    pub file_count: usize,
    /// Dominant error code
    pub dominant_error: String,
    /// Cohesion score (lower = tighter)
    pub cohesion: f64,
}

/// Simplified graph summary for report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSummary {
    /// Number of unique error types
    pub node_count: usize,
    /// Number of co-occurrence relationships
    pub edge_count: usize,
    /// Graph density (0-1)
    pub density: f64,
    /// Top communities
    pub communities: Vec<CommunitySummary>,
    /// Most central error codes
    pub top_central: Vec<String>,
}

/// Simplified community summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunitySummary {
    /// Community name
    pub name: String,
    /// Error codes in community
    pub error_codes: Vec<String>,
    /// Total files affected
    pub total_files: usize,
    /// Centrality sum
    pub centrality: f64,
}

/// Build advanced report from analysis results (GH-209)
pub fn build_advanced_report(
    results: &[ExtendedAnalysisResult],
    config: &AdvancedReportConfig,
) -> AdvancedReport {
    // Get basic analysis
    let (passed, failed, taxonomy, domain_breakdown) = analyze_extended_results(results);

    // Build summary
    let summary = ReportSummary::from_counts(passed, failed);

    // Build domain stats
    let domain_stats = build_domain_stats(&domain_breakdown);

    // Build error breakdown
    let error_breakdown = build_error_breakdown(&taxonomy, failed, config.top_errors);

    // Build clustering if enabled
    let clusters = if config.include_clustering && failed > 0 {
        let analyzer = ErrorClusterAnalyzer::with_config(ClusterConfig::default());
        let analysis = analyzer.cluster_errors(results);
        Some(summarize_clusters(&analysis, config.top_clusters))
    } else {
        None
    };

    // Build graph analysis if enabled
    let graph = if config.include_graph && failed > 0 {
        let analysis = GraphAnalysis::from_results(results);
        Some(summarize_graph(&analysis, config.top_communities))
    } else {
        None
    };

    AdvancedReport {
        version: "2.0".to_string(),
        summary,
        domain_breakdown: domain_stats,
        error_breakdown,
        clusters,
        graph,
    }
}

/// Build domain statistics from breakdown
fn build_domain_stats(breakdown: &DomainBreakdown) -> Vec<DomainStats> {
    vec![
        DomainStats {
            name: "Core Language".to_string(),
            passed: breakdown.core_lang_pass,
            failed: breakdown.core_lang_fail,
            pass_rate: breakdown.pass_rate(SemanticDomain::CoreLanguage),
        },
        DomainStats {
            name: "Stdlib (Common)".to_string(),
            passed: breakdown.stdlib_common_pass,
            failed: breakdown.stdlib_common_fail,
            pass_rate: breakdown.pass_rate(SemanticDomain::StdlibCommon),
        },
        DomainStats {
            name: "Stdlib (Advanced)".to_string(),
            passed: breakdown.stdlib_advanced_pass,
            failed: breakdown.stdlib_advanced_fail,
            pass_rate: breakdown.pass_rate(SemanticDomain::StdlibAdvanced),
        },
        DomainStats {
            name: "External Packages".to_string(),
            passed: breakdown.external_pass,
            failed: breakdown.external_fail,
            pass_rate: breakdown.pass_rate(SemanticDomain::External),
        },
        DomainStats {
            name: "Unknown".to_string(),
            passed: breakdown.unknown_pass,
            failed: breakdown.unknown_fail,
            pass_rate: breakdown.pass_rate(SemanticDomain::Unknown),
        },
    ]
}

/// Build error breakdown from taxonomy
fn build_error_breakdown(
    taxonomy: &HashMap<String, ErrorEntry>,
    total_failures: usize,
    limit: usize,
) -> Vec<ErrorBreakdownEntry> {
    let mut entries: Vec<_> = taxonomy
        .iter()
        .map(|(code, entry)| {
            let percentage = if total_failures > 0 {
                (entry.count as f64 / total_failures as f64) * 100.0
            } else {
                0.0
            };

            let priority = if entry.count >= 20 {
                "P0-CRITICAL"
            } else if entry.count >= 10 {
                "P1-HIGH"
            } else if entry.count >= 5 {
                "P2-MEDIUM"
            } else {
                "P3-LOW"
            }
            .to_string();

            ErrorBreakdownEntry {
                code: code.clone(),
                description: error_description(code),
                count: entry.count,
                percentage,
                samples: entry.samples.clone(),
                priority,
            }
        })
        .collect();

    entries.sort_by(|a, b| b.count.cmp(&a.count));
    entries.truncate(limit);
    entries
}

/// Get error description
fn error_description(code: &str) -> String {
    match code {
        "E0308" => "Mismatched types (type inference failure)".to_string(),
        "E0425" => "Cannot find value in scope".to_string(),
        "E0433" => "Failed to resolve module path".to_string(),
        "E0277" => "Trait not implemented".to_string(),
        "E0599" => "Method not found on type".to_string(),
        "E0382" => "Use of moved value".to_string(),
        "E0502" => "Cannot borrow as mutable".to_string(),
        "E0106" => "Missing lifetime specifier".to_string(),
        "TRANSPILE" => "Transpiler limitation".to_string(),
        "UNKNOWN" => "Unknown error".to_string(),
        _ => format!("Rust error {}", code),
    }
}

/// Summarize clusters for report
fn summarize_clusters(analysis: &ClusterAnalysis, limit: usize) -> Vec<ClusterSummary> {
    analysis
        .clusters
        .iter()
        .take(limit)
        .map(|c| ClusterSummary {
            id: c.id,
            label: c.label.clone(),
            file_count: c.member_count(),
            dominant_error: c.dominant_error_code.clone(),
            cohesion: c.cohesion,
        })
        .collect()
}

/// Summarize graph analysis for report
fn summarize_graph(analysis: &GraphAnalysis, limit: usize) -> GraphSummary {
    let communities: Vec<CommunitySummary> = analysis
        .communities
        .iter()
        .take(limit)
        .map(|c| CommunitySummary {
            name: c.name.clone(),
            error_codes: c.error_codes.clone(),
            total_files: c.total_files,
            centrality: c.centrality_sum,
        })
        .collect();

    GraphSummary {
        node_count: analysis.graph.node_count(),
        edge_count: analysis.graph.edge_count(),
        density: analysis.density,
        communities,
        top_central: analysis.top_central.clone(),
    }
}

/// Format report as JSON string
pub fn format_json(report: &AdvancedReport) -> String {
    serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string())
}

/// Format report as human-readable text
pub fn format_text(report: &AdvancedReport) -> String {
    let mut lines = Vec::new();

    lines.push("═══════════════════════════════════════════════════════════════".to_string());
    lines.push("                    ADVANCED CORPUS ANALYSIS                    ".to_string());
    lines.push("═══════════════════════════════════════════════════════════════".to_string());
    lines.push(String::new());

    // Summary
    lines.push(format!("Status: {} [{:.1}%]", report.summary.status, report.summary.pass_rate));
    lines.push(format!(
        "Files: {} total, {} passed, {} failed",
        report.summary.total_files, report.summary.passed, report.summary.failed
    ));
    lines.push(String::new());

    // Domain breakdown
    lines.push("DOMAIN BREAKDOWN".to_string());
    lines.push("────────────────".to_string());
    for domain in &report.domain_breakdown {
        let total = domain.passed + domain.failed;
        if total > 0 {
            lines.push(format!(
                "  {:<20} {:>5} / {:>5} ({:>5.1}%)",
                domain.name, domain.passed, total, domain.pass_rate
            ));
        }
    }
    lines.push(String::new());

    // Top errors
    lines.push("TOP ERRORS".to_string());
    lines.push("──────────".to_string());
    for (i, error) in report.error_breakdown.iter().take(5).enumerate() {
        lines.push(format!(
            "  {}. {} ({}) - {} occurrences ({:.1}%)",
            i + 1,
            error.code,
            error.priority,
            error.count,
            error.percentage
        ));
        lines.push(format!("     {}", error.description));
    }
    lines.push(String::new());

    // Clusters
    if let Some(clusters) = &report.clusters {
        lines.push("ERROR CLUSTERS".to_string());
        lines.push("──────────────".to_string());
        for cluster in clusters.iter().take(3) {
            lines.push(format!("  • {} ({} files)", cluster.label, cluster.file_count));
        }
        lines.push(String::new());
    }

    // Graph analysis
    if let Some(graph) = &report.graph {
        lines.push("GRAPH ANALYSIS".to_string());
        lines.push("──────────────".to_string());
        lines.push(format!("  Nodes: {} error types", graph.node_count));
        lines.push(format!("  Edges: {} co-occurrences", graph.edge_count));
        lines.push(format!("  Density: {:.3}", graph.density));
        if !graph.communities.is_empty() {
            lines.push("  Communities:".to_string());
            for comm in graph.communities.iter().take(3) {
                lines.push(format!("    • {} ({} files)", comm.name, comm.total_files));
            }
        }
        lines.push(String::new());
    }

    lines.push("═══════════════════════════════════════════════════════════════".to_string());

    lines.join("\n")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report_cmd::analysis::{AnalysisResult, AstFeatures};

    fn make_result(name: &str, success: bool, error_code: Option<&str>) -> ExtendedAnalysisResult {
        ExtendedAnalysisResult {
            base: AnalysisResult {
                name: name.to_string(),
                success,
                error_code: error_code.map(String::from),
                error_message: Some("test error".to_string()),
            },
            semantic_domain: SemanticDomain::CoreLanguage,
            ast_features: AstFeatures::default(),
            imports: vec![],
        }
    }

    #[test]
    fn test_advanced_report_config_default() {
        let config = AdvancedReportConfig::default();
        assert!(config.include_clustering);
        assert!(config.include_graph);
        assert_eq!(config.top_errors, 10);
    }

    #[test]
    fn test_report_summary_from_counts() {
        let summary = ReportSummary::from_counts(80, 20);
        assert_eq!(summary.total_files, 100);
        assert_eq!(summary.passed, 80);
        assert_eq!(summary.failed, 20);
        assert!((summary.pass_rate - 80.0).abs() < 0.01);
        assert_eq!(summary.status, "GREEN");
    }

    #[test]
    fn test_report_summary_yellow() {
        let summary = ReportSummary::from_counts(60, 40);
        assert_eq!(summary.status, "YELLOW");
    }

    #[test]
    fn test_report_summary_red() {
        let summary = ReportSummary::from_counts(30, 70);
        assert_eq!(summary.status, "RED");
    }

    #[test]
    fn test_report_summary_empty() {
        let summary = ReportSummary::from_counts(0, 0);
        assert_eq!(summary.pass_rate, 0.0);
        assert_eq!(summary.status, "RED");
    }

    #[test]
    fn test_build_advanced_report_empty() {
        let results: Vec<ExtendedAnalysisResult> = vec![];
        let config = AdvancedReportConfig::default();
        let report = build_advanced_report(&results, &config);

        assert_eq!(report.summary.total_files, 0);
        assert!(report.clusters.is_none());
        assert!(report.graph.is_none());
    }

    #[test]
    fn test_build_advanced_report_all_pass() {
        let results = vec![
            make_result("a.py", true, None),
            make_result("b.py", true, None),
        ];
        let config = AdvancedReportConfig::default();
        let report = build_advanced_report(&results, &config);

        assert_eq!(summary_pass_rate(&report), 100.0);
        assert!(report.clusters.is_none()); // No failures = no clusters
    }

    #[test]
    fn test_build_advanced_report_with_failures() {
        let results = vec![
            make_result("a.py", true, None),
            make_result("b.py", false, Some("E0308")),
            make_result("c.py", false, Some("E0308")),
            make_result("d.py", false, Some("E0425")),
        ];
        let config = AdvancedReportConfig::default();
        let report = build_advanced_report(&results, &config);

        assert_eq!(report.summary.passed, 1);
        assert_eq!(report.summary.failed, 3);
        assert!(report.clusters.is_some());
        assert!(report.graph.is_some());
    }

    #[test]
    fn test_build_domain_stats() {
        let mut breakdown = DomainBreakdown::default();
        breakdown.core_lang_pass = 8;
        breakdown.core_lang_fail = 2;
        breakdown.external_pass = 4;
        breakdown.external_fail = 6;

        let stats = build_domain_stats(&breakdown);

        let core = stats.iter().find(|s| s.name == "Core Language").unwrap();
        assert_eq!(core.passed, 8);
        assert_eq!(core.failed, 2);
        assert!((core.pass_rate - 80.0).abs() < 0.01);

        let external = stats.iter().find(|s| s.name == "External Packages").unwrap();
        assert_eq!(external.passed, 4);
        assert_eq!(external.failed, 6);
    }

    #[test]
    fn test_error_description_known() {
        assert!(error_description("E0308").contains("type"));
        assert!(error_description("E0425").contains("scope"));
        assert!(error_description("E0599").contains("Method"));
    }

    #[test]
    fn test_error_description_unknown() {
        let desc = error_description("E9999");
        assert!(desc.contains("E9999"));
    }

    #[test]
    fn test_format_json() {
        let results = vec![make_result("a.py", true, None)];
        let config = AdvancedReportConfig::default();
        let report = build_advanced_report(&results, &config);
        let json = format_json(&report);

        assert!(json.contains("\"version\""));
        assert!(json.contains("\"summary\""));
    }

    #[test]
    fn test_format_text() {
        let results = vec![
            make_result("a.py", true, None),
            make_result("b.py", false, Some("E0308")),
        ];
        let config = AdvancedReportConfig::default();
        let report = build_advanced_report(&results, &config);
        let text = format_text(&report);

        assert!(text.contains("ADVANCED CORPUS ANALYSIS"));
        assert!(text.contains("DOMAIN BREAKDOWN"));
        assert!(text.contains("TOP ERRORS"));
    }

    #[test]
    fn test_cluster_summary_fields() {
        let summary = ClusterSummary {
            id: 0,
            label: "Test Cluster".to_string(),
            file_count: 10,
            dominant_error: "E0308".to_string(),
            cohesion: 0.5,
        };
        assert_eq!(summary.id, 0);
        assert_eq!(summary.file_count, 10);
    }

    #[test]
    fn test_graph_summary_fields() {
        let summary = GraphSummary {
            node_count: 5,
            edge_count: 3,
            density: 0.3,
            communities: vec![],
            top_central: vec!["E0308".to_string()],
        };
        assert_eq!(summary.node_count, 5);
        assert_eq!(summary.density, 0.3);
    }

    // Helper to extract pass rate
    fn summary_pass_rate(report: &AdvancedReport) -> f64 {
        report.summary.pass_rate
    }
}

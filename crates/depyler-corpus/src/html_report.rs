//! Rich text report generator module (DEPYLER-REPORT-V2).
//!
//! Generates enhanced TEXT reports with:
//! - ASCII art bar charts
//! - Box-drawing tables
//! - Colored terminal output
//! - Optional SVG via trueno-viz (future)
//!
//! NO JAVASCRIPT - Pure Rust text generation.

use crate::clustering::ClusteringResult;
use crate::graph::ErrorGraph;
use crate::report::CorpusReport;
use crate::semantic::SemanticClassification;
use std::io::Write;
use std::path::Path;
use std::fmt::Write as _;

/// Rich text report generator - pure Rust, no JavaScript.
pub struct HtmlReportGenerator {
    /// Use colored output.
    colored: bool,
    /// Width for ASCII bar charts.
    bar_width: usize,
}

impl Default for HtmlReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlReportGenerator {
    /// Create a new text report generator.
    pub fn new() -> Self {
        Self { colored: true, bar_width: 40 }
    }

    /// Disable colored output.
    #[must_use]
    pub fn without_color(mut self) -> Self {
        self.colored = false;
        self
    }

    /// Generate rich TEXT report from corpus analysis results.
    pub fn generate(
        &self,
        report: &CorpusReport,
        semantic: Option<&SemanticClassification>,
        clusters: Option<&ClusteringResult>,
        graph: Option<&ErrorGraph>,
    ) -> String {
        let mut out = String::new();

        // Header
        out.push_str(&Self::generate_header(report));

        // Executive Summary
        out.push_str(&self.generate_summary(report));

        // Domain breakdown (if available)
        if let Some(sem) = semantic {
            out.push_str(&Self::generate_domain_section(sem));
        }

        // Error distribution with ASCII bar chart
        out.push_str(&Self::generate_error_distribution(report));

        // Cluster analysis (if available)
        if let Some(clust) = clusters {
            out.push_str(&Self::generate_cluster_section(clust));
        }

        // Graph analysis (if available)
        if let Some(g) = graph {
            out.push_str(&Self::generate_graph_section(g));
        }

        // Blockers section
        out.push_str(&Self::generate_blockers_section(report));

        // Toyota Way metrics
        out.push_str(&Self::generate_toyota_section(report));

        // Footer
        out.push_str(&Self::generate_footer());

        out
    }

    /// Write text report to file.
    pub fn write_to_file(
        &self,
        path: &Path,
        report: &CorpusReport,
        semantic: Option<&SemanticClassification>,
        clusters: Option<&ClusteringResult>,
        graph: Option<&ErrorGraph>,
    ) -> anyhow::Result<()> {
        let text = self.generate(report, semantic, clusters, graph);
        let mut file = std::fs::File::create(path)?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }

    fn generate_header(report: &CorpusReport) -> String {
        let andon = if report.summary.single_shot_rate >= 80.0 {
            "GREEN"
        } else if report.summary.single_shot_rate >= 50.0 {
            "YELLOW"
        } else {
            "RED"
        };

        format!(
            r"
╔══════════════════════════════════════════════════════════════════════════════╗
║                     DEPYLER CORPUS ANALYSIS REPORT                           ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  Corpus:    {:<64} ║
║  Generated: {:<64} ║
║  Version:   {:<64} ║
║  Andon:     {:<64} ║
╚══════════════════════════════════════════════════════════════════════════════╝

",
            report.metadata.corpus_name,
            report.metadata.generated_at,
            report.metadata.depyler_version,
            andon
        )
    }

    fn generate_summary(&self, report: &CorpusReport) -> String {
        let rate = report.summary.single_shot_rate;
        let bar = Self::ascii_bar(rate / 100.0, self.bar_width);

        format!(
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                            EXECUTIVE SUMMARY                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│  Single-Shot Rate:  {:<6.1}%                                                  │
│  Progress:          [{}] {:.1}%                     │
│                                                                              │
│  Total Files:       {:<6}                                                    │
│  Compiled OK:       {:<6}                                                    │
│  Failed:            {:<6}                                                    │
│                                                                              │
│  95% CI:            [{:.1}%, {:.1}%]                                            │
└─────────────────────────────────────────────────────────────────────────────┘

",
            rate,
            bar,
            rate,
            report.summary.total_python_files,
            report.summary.compilation.success,
            report.summary.compilation.failure,
            report.summary.confidence_interval_95.0,
            report.summary.confidence_interval_95.1
        )
    }

    #[allow(clippy::cast_precision_loss)]
    fn generate_domain_section(semantic: &SemanticClassification) -> String {
        let mut out = String::new();

        out.push_str(
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                         DOMAIN CLASSIFICATION                                │
├──────────────┬────────┬────────┬────────────┬───────────────────────────────┤
│ Domain       │ Total  │ Passed │ Pass Rate  │ Distribution                  │
├──────────────┼────────┼────────┼────────────┼───────────────────────────────┤
",
        );

        let max_total = semantic.by_domain.values().map(|s| s.total).max().unwrap_or(1);

        for (domain, stats) in &semantic.by_domain {
            let bar = Self::ascii_bar(stats.total as f64 / max_total as f64, 25);
            writeln!(out, "│ {:<12} │ {:>6} │ {:>6} │ {:>8.1}%  │ {} │",
                format!("{:?}", domain),
                stats.total,
                stats.passed,
                stats.pass_rate,
                bar
                ).expect("write to String");
        }

        out.push_str(
            "└──────────────┴────────┴────────┴────────────┴───────────────────────────────┘\n\n",
        );

        out
    }

    #[allow(clippy::cast_precision_loss)]
    fn generate_error_distribution(report: &CorpusReport) -> String {
        let mut out = String::new();

        out.push_str(
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                         ERROR DISTRIBUTION                                   │
├──────────┬────────┬────────────────────────────────────────────────────────┤
│ Code     │ Count  │ Bar                                                    │
├──────────┼────────┼────────────────────────────────────────────────────────┤
",
        );

        let max_count =
            report.error_distribution.by_error_code.iter().map(|e| e.count).max().unwrap_or(1);

        for err in report.error_distribution.by_error_code.iter().take(10) {
            let bar = Self::ascii_bar(err.count as f64 / max_count as f64, 50);
            writeln!(out, "│ {:<8} │ {:>6} │ {} │", err.code, err.count, bar).expect("write to String");
        }

        out.push_str(
            "└──────────┴────────┴────────────────────────────────────────────────────────┘\n\n",
        );

        out
    }

    fn generate_cluster_section(clusters: &ClusteringResult) -> String {
        let mut out = String::new();

        write!(out,
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                         ERROR CLUSTERING (K={})                               │
│  Silhouette Score: {:.3}                                                      │
├──────────────────────────────┬────────┬──────────┬─────────────────────────┤
│ Cluster                      │ Size   │ Dominant │ Variance                │
├──────────────────────────────┼────────┼──────────┼─────────────────────────┤
",
            clusters.k, clusters.silhouette_score
            ).expect("write to String");

        for cluster in &clusters.clusters {
            writeln!(out, "│ {:<28} │ {:>6} │ {:<8} │ {:>21.3} │",
                truncate(&cluster.label, 28),
                cluster.size,
                cluster.dominant_code,
                cluster.variance
                ).expect("write to String");
        }

        out.push_str(
            "└──────────────────────────────┴────────┴──────────┴─────────────────────────┘\n\n",
        );

        out
    }

    fn generate_graph_section(graph: &ErrorGraph) -> String {
        let mut out = String::new();

        write!(out,
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                         GRAPH ANALYSIS                                       │
│  Modularity: {:.3}  |  Communities: {}                                         │
├───────────────────────────────────────────────────────────────────────────────┤
│                         Top Errors by PageRank                               │
├─────┬──────────┬────────┬────────────────────────────────────────────────────┤
│ #   │ Error    │ Count  │ PageRank                                           │
├─────┼──────────┼────────┼────────────────────────────────────────────────────┤
",
            graph.modularity,
            graph.communities.len()
            ).expect("write to String");

        for (i, node) in graph.nodes.iter().take(5).enumerate() {
            let bar = Self::ascii_bar(node.pagerank * 10.0, 45);
            writeln!(out, "│ {:>3} │ {:<8} │ {:>6} │ {} │",
                i + 1,
                node.code,
                node.count,
                bar
            ).expect("write to String");
        }

        out.push_str(
            "├─────┴──────────┴────────┴────────────────────────────────────────────────────┤\n",
        );
        out.push_str(
            "│                         Communities                                          │\n",
        );
        out.push_str(
            "├────────────────────────────────┬─────────┬────────────┬──────────────────────┤\n",
        );
        out.push_str(
            "│ Community                      │ Members │ Count      │ Dominant             │\n",
        );
        out.push_str(
            "├────────────────────────────────┼─────────┼────────────┼──────────────────────┤\n",
        );

        for comm in &graph.communities {
            writeln!(out, "│ {:<30} │ {:>7} │ {:>10} │ {:<20} │",
                truncate(&comm.label, 30),
                comm.members.len(),
                comm.total_count,
                comm.dominant
                ).expect("write to String");
        }

        out.push_str(
            "└────────────────────────────────┴─────────┴────────────┴──────────────────────┘\n\n",
        );

        out
    }

    fn generate_blockers_section(report: &CorpusReport) -> String {
        let mut out = String::new();

        out.push_str(
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                         BLOCKERS ANALYSIS                                    │
├──────────┬──────────┬────────┬───────────────────────────────────────────────┤
│ Priority │ Error    │ Count  │ Root Cause                                    │
├──────────┼──────────┼────────┼───────────────────────────────────────────────┤
",
        );

        for b in &report.blocker_analysis.p0_critical {
            writeln!(out, "│ P0-CRIT  │ {:<8} │ {:>6} │ {:<45} │",
                b.error_code,
                b.count,
                truncate(&b.root_cause, 45)
                ).expect("write to String");
        }

        for b in &report.blocker_analysis.p1_high {
            writeln!(out, "│ P1-HIGH  │ {:<8} │ {:>6} │ {:<45} │",
                b.error_code,
                b.count,
                truncate(&b.root_cause, 45)
                ).expect("write to String");
        }

        for b in &report.blocker_analysis.p2_medium {
            writeln!(out, "│ P2-MED   │ {:<8} │ {:>6} │ {:<45} │",
                b.error_code,
                b.count,
                truncate(&b.root_cause, 45)
                ).expect("write to String");
        }

        if report.blocker_analysis.p0_critical.is_empty()
            && report.blocker_analysis.p1_high.is_empty()
            && report.blocker_analysis.p2_medium.is_empty()
        {
            out.push_str(
                "│          │          │        │ No significant blockers found                 │\n",
            );
        }

        out.push_str(
            "└──────────┴──────────┴────────┴───────────────────────────────────────────────┘\n\n",
        );

        out
    }

    fn generate_toyota_section(report: &CorpusReport) -> String {
        let mut out = String::new();

        write!(out,
            r"┌─────────────────────────────────────────────────────────────────────────────┐
│                         TOYOTA WAY METRICS                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│  Jidoka Alerts:        {:>4}                                                  │
│  Andon Triggers:       {:>4}                                                  │
│  Kaizen Opportunities: {:>4}                                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│  Hansei (Lessons Learned):                                                   │
",
            report.toyota_way_metrics.jidoka_alerts,
            report.toyota_way_metrics.andon_triggers,
            report.toyota_way_metrics.kaizen_opportunities
            ).expect("write to String");

        if report.toyota_way_metrics.hansei_items.is_empty() {
            out.push_str(
                "│    - No lessons recorded                                                    │\n",
            );
        } else {
            for item in &report.toyota_way_metrics.hansei_items {
                writeln!(out, "│    - {:<70} │", truncate(item, 70)).expect("write to String");
            }
        }

        out.push_str(
            "└─────────────────────────────────────────────────────────────────────────────┘\n\n",
        );

        out
    }

    #[allow(clippy::cast_precision_loss)]
    fn generate_footer() -> String {
        r"═══════════════════════════════════════════════════════════════════════════════
  Generated by Depyler Corpus Analyzer | https://github.com/paiml/depyler
═══════════════════════════════════════════════════════════════════════════════
"
        .to_string()
    }

    /// Generate ASCII progress bar.
    #[allow(clippy::cast_precision_loss)]
    fn ascii_bar(ratio: f64, width: usize) -> String {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let filled = (ratio.clamp(0.0, 1.0) * width as f64).round() as usize;
        let empty = width.saturating_sub(filled);
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }
}

/// Truncate string to max length with ellipsis.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else if max > 3 {
        format!("{}...", &s[..max - 3])
    } else {
        s[..max].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CorpusConfig;
    use crate::statistics::StatisticalAnalysis;
    use crate::taxonomy::ErrorTaxonomy;
    use std::collections::HashMap;

    fn create_test_report() -> CorpusReport {
        let config = CorpusConfig::default();
        let transpile_results = vec![];
        let compile_results = vec![];
        let taxonomy = ErrorTaxonomy {
            errors: vec![],
            by_category: HashMap::new(),
            by_code: HashMap::new(),
            blockers: vec![],
        };
        let statistics = StatisticalAnalysis {
            total_files: 10,
            passed_files: 3,
            failed_files: 7,
            single_shot_rate: 30.0,
            ci_95_lower: 10.0,
            ci_95_upper: 60.0,
            mean_errors_per_file: 2.5,
            std_deviation: 1.0,
            median_errors: 2.0,
            total_errors: 25,
        };

        CorpusReport::new(&config, &transpile_results, &compile_results, &taxonomy, &statistics)
    }

    #[test]
    fn test_text_generator_creation() {
        let gen = HtmlReportGenerator::new();
        assert!(gen.colored);
    }

    #[test]
    fn test_without_color() {
        let gen = HtmlReportGenerator::new().without_color();
        assert!(!gen.colored);
    }

    #[test]
    fn test_generate_basic_text() {
        let gen = HtmlReportGenerator::new();
        let report = create_test_report();
        let text = gen.generate(&report, None, None, None);

        assert!(text.contains("DEPYLER CORPUS ANALYSIS REPORT"));
        assert!(text.contains("EXECUTIVE SUMMARY"));
        assert!(text.contains("ERROR DISTRIBUTION"));
    }

    #[test]
    fn test_ascii_bar() {
        let gen = HtmlReportGenerator::new();

        let bar_0 = gen.ascii_bar(0.0, 10);
        assert_eq!(bar_0, "░░░░░░░░░░");

        let bar_50 = gen.ascii_bar(0.5, 10);
        assert_eq!(bar_50, "█████░░░░░");

        let bar_100 = gen.ascii_bar(1.0, 10);
        assert_eq!(bar_100, "██████████");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 8), "hello...");
        assert_eq!(truncate("hi", 2), "hi");
    }

    #[test]
    fn test_generate_with_semantic() {
        let gen = HtmlReportGenerator::new();
        let report = create_test_report();

        let mut by_domain = HashMap::new();
        by_domain
            .insert(crate::semantic::PythonDomain::Core, crate::semantic::DomainStats::new(5, 3));

        let semantic =
            SemanticClassification { by_domain, file_domains: HashMap::new(), confidence: 0.9 };

        let text = gen.generate(&report, Some(&semantic), None, None);
        assert!(text.contains("DOMAIN CLASSIFICATION"));
    }

    #[test]
    fn test_generate_with_clusters() {
        let gen = HtmlReportGenerator::new();
        let report = create_test_report();

        let clusters = ClusteringResult {
            clusters: vec![crate::clustering::ErrorCluster {
                id: 0,
                centroid: vec![1.0, 2.0],
                members: vec!["E0308".to_string()],
                dominant_code: "E0308".to_string(),
                size: 1,
                variance: 0.1,
                label: "Type Errors".to_string(),
            }],
            total_error_types: 1,
            silhouette_score: 0.5,
            k: 1,
        };

        let text = gen.generate(&report, None, Some(&clusters), None);
        assert!(text.contains("ERROR CLUSTERING"));
        assert!(text.contains("Silhouette Score"));
    }

    #[test]
    fn test_generate_with_graph() {
        let gen = HtmlReportGenerator::new();
        let report = create_test_report();

        let graph = ErrorGraph {
            nodes: vec![crate::graph::ErrorNode {
                code: "E0308".to_string(),
                count: 10,
                pagerank: 0.5,
                community: 0,
            }],
            edges: vec![],
            communities: vec![],
            top_by_pagerank: vec!["E0308".to_string()],
            modularity: 0.3,
        };

        let text = gen.generate(&report, None, None, Some(&graph));
        assert!(text.contains("GRAPH ANALYSIS"));
        assert!(text.contains("PageRank"));
    }

    #[test]
    fn test_andon_status() {
        let gen = HtmlReportGenerator::new();
        let report = create_test_report();

        let text = gen.generate(&report, None, None, None);
        // 30% rate = RED
        assert!(text.contains("RED"));
    }
}

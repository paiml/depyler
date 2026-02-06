//! Impact Scoring Module
//!
//! Implements a PageRank-style algorithm to identify "Patient Zero" -
//! the nodes that cause the most downstream errors.

use crate::builder::DependencyGraph;
use crate::error_overlay::OverlaidError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Impact score for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactScore {
    /// Node ID
    pub node_id: String,
    /// Direct error count at this node
    pub direct_errors: usize,
    /// Downstream errors caused by this node
    pub downstream_errors: usize,
    /// PageRank-style score (higher = more impact)
    pub pagerank_score: f64,
    /// Combined impact score
    pub total_impact: f64,
}

/// Patient Zero - a node identified as root cause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientZero {
    /// Node ID
    pub node_id: String,
    /// Impact score
    pub impact_score: f64,
    /// Number of direct errors
    pub direct_errors: usize,
    /// Number of downstream nodes affected
    pub downstream_affected: usize,
    /// Recommended fix priority (1 = highest)
    pub fix_priority: usize,
    /// Estimated errors fixed if this node is fixed
    pub estimated_fix_impact: usize,
}

/// Calculates impact scores for nodes in the graph
pub struct ImpactScorer<'a> {
    graph: &'a DependencyGraph,
    errors: &'a [OverlaidError],
    /// Damping factor for PageRank (typically 0.85)
    damping: f64,
    /// Number of iterations for PageRank convergence
    iterations: usize,
}

impl<'a> ImpactScorer<'a> {
    /// Create a new impact scorer
    pub fn new(graph: &'a DependencyGraph, errors: &'a [OverlaidError]) -> Self {
        Self {
            graph,
            errors,
            damping: 0.85,
            iterations: 20,
        }
    }

    /// Set damping factor
    pub fn with_damping(mut self, damping: f64) -> Self {
        self.damping = damping.clamp(0.0, 1.0);
        self
    }

    /// Set number of iterations
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations.max(1);
        self
    }

    /// Calculate impact scores for all nodes
    pub fn calculate_impact(&self) -> Vec<ImpactScore> {
        let node_ids = self.graph.node_ids();
        if node_ids.is_empty() {
            return vec![];
        }

        // Step 1: Count direct errors per node
        let direct_errors = self.count_direct_errors();

        // Step 2: Calculate PageRank scores
        let pagerank_scores = self.calculate_pagerank(&node_ids);

        // Step 3: Calculate downstream errors
        let downstream_errors = self.calculate_downstream_errors(&node_ids, &direct_errors);

        // Step 4: Combine into impact scores
        node_ids
            .iter()
            .map(|id| {
                let direct = *direct_errors.get(id).unwrap_or(&0);
                let downstream = *downstream_errors.get(id).unwrap_or(&0);
                let pagerank = *pagerank_scores.get(id).unwrap_or(&0.0);

                // Total impact = direct errors + weighted downstream + pagerank bonus
                let total = direct as f64
                    + 0.5 * downstream as f64
                    + 10.0 * pagerank * direct.max(1) as f64;

                ImpactScore {
                    node_id: id.clone(),
                    direct_errors: direct,
                    downstream_errors: downstream,
                    pagerank_score: pagerank,
                    total_impact: total,
                }
            })
            .collect()
    }

    /// Count direct errors at each node
    fn count_direct_errors(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for error in self.errors {
            if let Some(ref node_id) = error.node_id {
                *counts.entry(node_id.clone()).or_insert(0) += 1;
            }
        }
        counts
    }

    /// Calculate PageRank scores
    fn calculate_pagerank(&self, node_ids: &[String]) -> HashMap<String, f64> {
        let n = node_ids.len();
        if n == 0 {
            return HashMap::new();
        }

        // Initialize scores uniformly
        let mut scores: HashMap<String, f64> = node_ids
            .iter()
            .map(|id| (id.clone(), 1.0 / n as f64))
            .collect();

        // Calculate out-degrees
        let mut out_degrees: HashMap<String, usize> = HashMap::new();
        for id in node_ids {
            let outgoing = self.graph.outgoing_edges(id);
            out_degrees.insert(id.clone(), outgoing.len());
        }

        // Iterate to convergence
        for _ in 0..self.iterations {
            let mut new_scores: HashMap<String, f64> = HashMap::new();

            for id in node_ids {
                let mut rank = (1.0 - self.damping) / n as f64;

                // Sum contributions from incoming edges
                let incoming = self.graph.incoming_edges(id);
                for (source, _) in incoming {
                    let source_out = *out_degrees.get(&source.id).unwrap_or(&1);
                    let source_score = *scores.get(&source.id).unwrap_or(&0.0);
                    rank += self.damping * source_score / source_out.max(1) as f64;
                }

                new_scores.insert(id.clone(), rank);
            }

            scores = new_scores;
        }

        scores
    }

    /// Calculate downstream errors (errors in nodes that depend on this one)
    fn calculate_downstream_errors(
        &self,
        node_ids: &[String],
        direct_errors: &HashMap<String, usize>,
    ) -> HashMap<String, usize> {
        let mut downstream: HashMap<String, usize> = HashMap::new();

        for id in node_ids {
            // Find all nodes that call this node
            let incoming = self.graph.incoming_edges(id);
            let mut total_downstream = 0;

            for (caller, _) in incoming {
                // Count errors in the caller
                total_downstream += direct_errors.get(&caller.id).unwrap_or(&0);
            }

            downstream.insert(id.clone(), total_downstream);
        }

        downstream
    }

    /// Identify Patient Zero nodes (highest impact)
    pub fn identify_patient_zeros(&self, scores: &[ImpactScore], top_n: usize) -> Vec<PatientZero> {
        let mut sorted_scores: Vec<_> = scores.iter().collect();
        sorted_scores.sort_by(|a, b| {
            b.total_impact
                .partial_cmp(&a.total_impact)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        sorted_scores
            .into_iter()
            .take(top_n)
            .enumerate()
            .filter(|(_, score)| score.total_impact > 0.0)
            .map(|(idx, score)| {
                let downstream_affected = self.graph.incoming_edges(&score.node_id).len();

                PatientZero {
                    node_id: score.node_id.clone(),
                    impact_score: score.total_impact,
                    direct_errors: score.direct_errors,
                    downstream_affected,
                    fix_priority: idx + 1,
                    estimated_fix_impact: score.direct_errors + score.downstream_errors,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::GraphBuilder;
    use crate::error_overlay::ErrorOverlay;

    #[test]
    fn test_impact_scorer_empty() {
        let python = r#"
def foo():
    return 42
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors);
        let scores = scorer.calculate_impact();

        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].direct_errors, 0);
    }

    #[test]
    fn test_impact_scorer_with_errors() {
        let python = r#"
def problematic():
    return "bug"

def caller():
    return problematic()
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let raw_errors = vec![
            ("E0308".to_string(), "type mismatch".to_string(), 20),
            ("E0308".to_string(), "type mismatch".to_string(), 50),
        ];
        let overlaid = overlay.overlay_errors(&raw_errors);

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();

        assert!(!scores.is_empty());
    }

    #[test]
    fn test_patient_zero_ranking() {
        let python = r#"
def root_cause():
    return "bug"

def a():
    return root_cause()

def b():
    return root_cause()

def c():
    return root_cause()
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        // Errors in all callers
        let overlay = ErrorOverlay::new(&graph);
        let raw_errors = vec![
            ("E0308".to_string(), "error in a".to_string(), 50),
            ("E0308".to_string(), "error in b".to_string(), 80),
            ("E0308".to_string(), "error in c".to_string(), 110),
        ];
        let overlaid = overlay.overlay_errors(&raw_errors);

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let patient_zeros = scorer.identify_patient_zeros(&scores, 3);

        // root_cause should have highest downstream impact
        // Note: exact ranking depends on error-to-node mapping
        assert!(!patient_zeros.is_empty());
    }

    #[test]
    fn test_pagerank_convergence() {
        let python = r#"
def a():
    return b()

def b():
    return c()

def c():
    return 1
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(50);
        let scores = scorer.calculate_impact();

        // c should have highest PageRank (most called)
        let c_score = scores.iter().find(|s| s.node_id == "c");
        assert!(c_score.is_some());
    }

    #[test]
    fn test_impact_scorer_empty_graph() {
        let graph = DependencyGraph::new();
        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors);
        let scores = scorer.calculate_impact();
        assert!(scores.is_empty());
    }

    #[test]
    fn test_with_damping_clamped() {
        let graph = DependencyGraph::new();
        let errors: Vec<OverlaidError> = vec![];

        // Damping > 1.0 should be clamped to 1.0
        let scorer = ImpactScorer::new(&graph, &errors).with_damping(2.0);
        assert_eq!(scorer.damping, 1.0);

        // Damping < 0.0 should be clamped to 0.0
        let scorer = ImpactScorer::new(&graph, &errors).with_damping(-0.5);
        assert_eq!(scorer.damping, 0.0);

        // Normal damping preserved
        let scorer = ImpactScorer::new(&graph, &errors).with_damping(0.5);
        assert!((scorer.damping - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_with_iterations_minimum_1() {
        let graph = DependencyGraph::new();
        let errors: Vec<OverlaidError> = vec![];

        // 0 iterations should become 1
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(0);
        assert_eq!(scorer.iterations, 1);

        // Normal iterations preserved
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(100);
        assert_eq!(scorer.iterations, 100);
    }

    #[test]
    fn test_identify_patient_zeros_empty_scores() {
        let graph = DependencyGraph::new();
        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors);

        let patient_zeros = scorer.identify_patient_zeros(&[], 5);
        assert!(patient_zeros.is_empty());
    }

    #[test]
    fn test_patient_zero_fix_priority_ordering() {
        let python = r#"
def bug_a():
    return "x"

def bug_b():
    return bug_a()

def caller1():
    return bug_a()

def caller2():
    return bug_a()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let raw_errors = vec![
            ("E0308".to_string(), "error".to_string(), 10),
            ("E0308".to_string(), "error".to_string(), 50),
            ("E0308".to_string(), "error".to_string(), 80),
        ];
        let overlaid = overlay.overlay_errors(&raw_errors);

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let pzs = scorer.identify_patient_zeros(&scores, 5);

        // Fix priorities should be sequential starting from 1
        for (i, pz) in pzs.iter().enumerate() {
            assert_eq!(pz.fix_priority, i + 1);
        }
    }

    #[test]
    fn test_patient_zero_impact_score_positive() {
        let python = r#"
def root():
    return "bug"

def user():
    return root()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let raw_errors = vec![("E0308".to_string(), "error".to_string(), 10)];
        let overlaid = overlay.overlay_errors(&raw_errors);

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let pzs = scorer.identify_patient_zeros(&scores, 5);

        // All patient zeros should have positive impact
        for pz in &pzs {
            assert!(pz.impact_score > 0.0);
        }
    }

    #[test]
    fn test_patient_zero_top_n_limit() {
        let python = r#"
def a():
    return 1
def b():
    return 2
def c():
    return 3
def d():
    return 4
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        // Errors at different lines to associate with different nodes
        let raw_errors = vec![
            ("E0308".to_string(), "e1".to_string(), 10),
            ("E0308".to_string(), "e2".to_string(), 30),
            ("E0308".to_string(), "e3".to_string(), 50),
            ("E0308".to_string(), "e4".to_string(), 70),
        ];
        let overlaid = overlay.overlay_errors(&raw_errors);

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();

        // Request only top 2
        let pzs = scorer.identify_patient_zeros(&scores, 2);
        assert!(pzs.len() <= 2);
    }

    #[test]
    fn test_impact_score_serde_roundtrip() {
        let score = ImpactScore {
            node_id: "foo".to_string(),
            direct_errors: 3,
            downstream_errors: 5,
            pagerank_score: 0.42,
            total_impact: 7.2,
        };

        let json = serde_json::to_string(&score).unwrap();
        let deserialized: ImpactScore = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.node_id, "foo");
        assert_eq!(deserialized.direct_errors, 3);
        assert_eq!(deserialized.downstream_errors, 5);
        assert!((deserialized.pagerank_score - 0.42).abs() < f64::EPSILON);
    }

    #[test]
    fn test_patient_zero_serde_roundtrip() {
        let pz = PatientZero {
            node_id: "root_cause".to_string(),
            impact_score: 15.5,
            direct_errors: 2,
            downstream_affected: 4,
            fix_priority: 1,
            estimated_fix_impact: 6,
        };

        let json = serde_json::to_string(&pz).unwrap();
        let deserialized: PatientZero = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.node_id, "root_cause");
        assert_eq!(deserialized.fix_priority, 1);
        assert_eq!(deserialized.estimated_fix_impact, 6);
    }

    #[test]
    fn test_direct_errors_counted_correctly() {
        let python = "def target():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlaid = vec![
            OverlaidError {
                code: "E0308".to_string(),
                message: "a".to_string(),
                rust_line: 1,
                python_line_estimate: 2,
                node_id: Some("target".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
            OverlaidError {
                code: "E0308".to_string(),
                message: "b".to_string(),
                rust_line: 2,
                python_line_estimate: 2,
                node_id: Some("target".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
        ];

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();

        let target_score = scores.iter().find(|s| s.node_id == "target").unwrap();
        assert_eq!(target_score.direct_errors, 2);
    }

    // ========================================================================
    // S9B7: Coverage tests for impact scoring
    // ========================================================================

    #[test]
    fn test_s9b7_impact_score_debug_clone() {
        let score = ImpactScore {
            node_id: "n".to_string(),
            direct_errors: 1,
            downstream_errors: 2,
            pagerank_score: 0.5,
            total_impact: 3.0,
        };
        let debug = format!("{:?}", score);
        assert!(debug.contains("ImpactScore"));
        let cloned = score.clone();
        assert_eq!(cloned.node_id, "n");
    }

    #[test]
    fn test_s9b7_patient_zero_debug_clone() {
        let pz = PatientZero {
            node_id: "root".to_string(),
            impact_score: 10.0,
            direct_errors: 3,
            downstream_affected: 2,
            fix_priority: 1,
            estimated_fix_impact: 5,
        };
        let debug = format!("{:?}", pz);
        assert!(debug.contains("PatientZero"));
        let cloned = pz.clone();
        assert_eq!(cloned.fix_priority, 1);
    }

    #[test]
    fn test_s9b7_count_direct_errors_no_node_id() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        // Error without node_id should not count
        let overlaid = vec![OverlaidError {
            code: "E0308".to_string(),
            message: "a".to_string(),
            rust_line: 1,
            python_line_estimate: 1,
            node_id: None,
            association_confidence: 0.0,
            upstream_suspects: vec![],
        }];
        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let target_score = scores.iter().find(|s| s.node_id == "foo").unwrap();
        assert_eq!(target_score.direct_errors, 0);
    }

    #[test]
    fn test_s9b7_identify_patient_zeros_filters_zero_impact() {
        let python = "def foo():\n    pass\n\ndef bar():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors);
        let scores = scorer.calculate_impact();
        // With no errors, total_impact should be based on pagerank only
        // With damping=0.85 and uniform init, total_impact = 10*pr*max(direct,1)
        // The filter is > 0.0, so nodes with positive pagerank will be included
        let pzs = scorer.identify_patient_zeros(&scores, 10);
        for pz in &pzs {
            assert!(pz.impact_score > 0.0);
        }
    }

    #[test]
    fn test_s9b7_with_damping_normal_value() {
        let graph = DependencyGraph::new();
        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors).with_damping(0.9);
        assert!((scorer.damping - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn test_s9b7_downstream_errors_with_caller_errors() {
        let python = r#"
def callee():
    return 1

def caller():
    return callee()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlaid = vec![OverlaidError {
            code: "E0308".to_string(),
            message: "err".to_string(),
            rust_line: 1,
            python_line_estimate: 1,
            node_id: Some("caller".to_string()),
            association_confidence: 0.9,
            upstream_suspects: vec![],
        }];
        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        // callee has caller as incoming, and caller has 1 direct error
        let callee_score = scores.iter().find(|s| s.node_id == "callee").unwrap();
        assert_eq!(callee_score.downstream_errors, 1);
    }

    #[test]
    fn test_pagerank_scores_all_nonnegative() {
        let python = r#"
def a():
    return b()
def b():
    return c()
def c():
    return 1
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(100);
        let scores = scorer.calculate_impact();

        // All PageRank scores should be non-negative
        for score in &scores {
            assert!(
                score.pagerank_score >= 0.0,
                "Negative pagerank for {}",
                score.node_id
            );
        }
        // Chain: a -> b -> c. c is most depended-upon, should have highest pagerank
        let a_pr = scores.iter().find(|s| s.node_id == "a").unwrap().pagerank_score;
        let c_pr = scores.iter().find(|s| s.node_id == "c").unwrap().pagerank_score;
        assert!(c_pr >= a_pr, "c should have higher pagerank than a");
    }

    // ========================================================================
    // S12: Deep coverage tests for impact scoring
    // ========================================================================

    #[test]
    fn test_s12_pagerank_with_cycle() {
        // a -> b -> a (cyclic dependency)
        let python = r#"
def a():
    return b()

def b():
    return a()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(50);
        let scores = scorer.calculate_impact();

        // Both nodes should have valid (non-NaN, non-negative) scores
        for score in &scores {
            assert!(!score.pagerank_score.is_nan(), "NaN pagerank for {}", score.node_id);
            assert!(score.pagerank_score >= 0.0, "Negative pagerank for {}", score.node_id);
        }
    }

    #[test]
    fn test_s12_pagerank_zero_damping() {
        // With damping=0, all nodes should get equal scores = 1/n
        let python = r#"
def a():
    return b()
def b():
    return 1
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors)
            .with_damping(0.0)
            .with_iterations(20);
        let scores = scorer.calculate_impact();

        // With damping=0, pagerank = (1-0)/n = 1/n for all nodes
        let n = scores.len() as f64;
        for score in &scores {
            let expected = 1.0 / n;
            assert!(
                (score.pagerank_score - expected).abs() < 0.01,
                "Node {} expected pagerank ~{}, got {}",
                score.node_id, expected, score.pagerank_score
            );
        }
    }

    #[test]
    fn test_s12_pagerank_max_damping() {
        let python = r#"
def source():
    return sink()
def sink():
    return 1
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors)
            .with_damping(1.0)
            .with_iterations(50);
        let scores = scorer.calculate_impact();

        for score in &scores {
            assert!(!score.pagerank_score.is_nan());
            assert!(score.pagerank_score >= 0.0);
        }
    }

    #[test]
    fn test_s12_sink_node_pagerank() {
        // Sink node has no outgoing edges; should converge correctly
        let python = r#"
def caller():
    return sink()
def sink():
    return 42
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(100);
        let scores = scorer.calculate_impact();

        let sink_score = scores.iter().find(|s| s.node_id == "sink").unwrap();
        assert!(sink_score.pagerank_score > 0.0, "Sink node should have positive pagerank");
    }

    #[test]
    fn test_s12_self_referencing_node() {
        // Recursive function: a calls itself
        let python = r#"
def recursive():
    return recursive()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let errors: Vec<OverlaidError> = vec![];
        let scorer = ImpactScorer::new(&graph, &errors).with_iterations(20);
        let scores = scorer.calculate_impact();

        assert_eq!(scores.len(), 1);
        assert!(!scores[0].pagerank_score.is_nan());
        assert!(scores[0].pagerank_score > 0.0);
    }

    #[test]
    fn test_s12_downstream_errors_multiple_callers() {
        let python = r#"
def target():
    return 1

def caller_a():
    return target()

def caller_b():
    return target()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        // Both callers have errors
        let overlaid = vec![
            OverlaidError {
                code: "E0308".to_string(),
                message: "a".to_string(),
                rust_line: 1,
                python_line_estimate: 4,
                node_id: Some("caller_a".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
            OverlaidError {
                code: "E0308".to_string(),
                message: "b".to_string(),
                rust_line: 2,
                python_line_estimate: 7,
                node_id: Some("caller_b".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
        ];

        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();

        // target has both callers as incoming, each with 1 error -> downstream = 2
        let target_score = scores.iter().find(|s| s.node_id == "target").unwrap();
        assert_eq!(target_score.downstream_errors, 2);
    }

    #[test]
    fn test_s12_patient_zero_top_n_zero() {
        let python = "def a():\n    return 1\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlaid = vec![OverlaidError {
            code: "E0308".to_string(),
            message: "e".to_string(),
            rust_line: 1,
            python_line_estimate: 1,
            node_id: Some("a".to_string()),
            association_confidence: 0.9,
            upstream_suspects: vec![],
        }];
        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let pzs = scorer.identify_patient_zeros(&scores, 0);
        assert!(pzs.is_empty());
    }

    #[test]
    fn test_s12_patient_zero_top_n_exceeds_nodes() {
        let python = "def single():\n    return 1\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlaid = vec![OverlaidError {
            code: "E0308".to_string(),
            message: "e".to_string(),
            rust_line: 1,
            python_line_estimate: 1,
            node_id: Some("single".to_string()),
            association_confidence: 0.9,
            upstream_suspects: vec![],
        }];
        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        // Request 100 patient zeros but only 1 node with positive impact
        let pzs = scorer.identify_patient_zeros(&scores, 100);
        assert!(pzs.len() <= scores.len());
    }

    #[test]
    fn test_s12_impact_total_formula() {
        // total = direct + 0.5 * downstream + 10 * pagerank * max(direct, 1)
        let python = "def only():\n    return 1\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlaid = vec![
            OverlaidError {
                code: "E0308".to_string(),
                message: "e1".to_string(),
                rust_line: 1,
                python_line_estimate: 1,
                node_id: Some("only".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
            OverlaidError {
                code: "E0308".to_string(),
                message: "e2".to_string(),
                rust_line: 2,
                python_line_estimate: 1,
                node_id: Some("only".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
        ];
        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let only = scores.iter().find(|s| s.node_id == "only").unwrap();
        assert_eq!(only.direct_errors, 2);
        // total = 2 + 0.5*0 + 10 * pagerank * max(2, 1) = 2 + 20*pagerank
        let expected = 2.0 + 10.0 * only.pagerank_score * 2.0;
        assert!((only.total_impact - expected).abs() < 0.001);
    }

    #[test]
    fn test_s12_patient_zero_estimated_fix_impact() {
        let python = r#"
def root():
    return 1
def user():
    return root()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlaid = vec![
            OverlaidError {
                code: "E0308".to_string(),
                message: "e1".to_string(),
                rust_line: 1,
                python_line_estimate: 2,
                node_id: Some("root".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
            OverlaidError {
                code: "E0308".to_string(),
                message: "e2".to_string(),
                rust_line: 2,
                python_line_estimate: 4,
                node_id: Some("user".to_string()),
                association_confidence: 0.9,
                upstream_suspects: vec![],
            },
        ];
        let scorer = ImpactScorer::new(&graph, &overlaid);
        let scores = scorer.calculate_impact();
        let pzs = scorer.identify_patient_zeros(&scores, 5);
        for pz in &pzs {
            // estimated_fix_impact = direct_errors + downstream_errors
            let score = scores.iter().find(|s| s.node_id == pz.node_id).unwrap();
            assert_eq!(pz.estimated_fix_impact, score.direct_errors + score.downstream_errors);
        }
    }
}

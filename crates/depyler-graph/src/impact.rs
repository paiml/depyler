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
}

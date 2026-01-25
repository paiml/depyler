//! Graph-based error analysis module (DEPYLER-REPORT-V2).
//!
//! Uses graph algorithms (PageRank, Louvain community detection) to identify
//! central error patterns and error communities for targeted fixing.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A node in the error graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorNode {
    /// Error code (e.g., "E0308").
    pub code: String,
    /// Frequency count.
    pub count: usize,
    /// PageRank score (0.0-1.0, higher = more central).
    pub pagerank: f64,
    /// Community ID from Louvain algorithm.
    pub community: usize,
}

/// An edge representing error co-occurrence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEdge {
    /// Source error code.
    pub from: String,
    /// Target error code.
    pub to: String,
    /// Co-occurrence weight.
    pub weight: f64,
}

/// Community of related errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCommunity {
    /// Community ID.
    pub id: usize,
    /// Member error codes.
    pub members: Vec<String>,
    /// Dominant error in community.
    pub dominant: String,
    /// Total error count in community.
    pub total_count: usize,
    /// Community label.
    pub label: String,
}

/// Complete error graph with analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorGraph {
    /// Nodes in the graph.
    pub nodes: Vec<ErrorNode>,
    /// Edges in the graph.
    pub edges: Vec<ErrorEdge>,
    /// Detected communities.
    pub communities: Vec<ErrorCommunity>,
    /// Top errors by PageRank.
    pub top_by_pagerank: Vec<String>,
    /// Modularity score of community detection.
    pub modularity: f64,
}

/// Graph analyzer for error patterns.
pub struct GraphAnalyzer {
    /// Damping factor for PageRank (typically 0.85).
    damping: f64,
    /// Maximum iterations for PageRank.
    max_iterations: usize,
    /// Convergence threshold.
    convergence: f64,
}

impl Default for GraphAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphAnalyzer {
    /// Create a new graph analyzer with default parameters.
    pub fn new() -> Self {
        Self {
            damping: 0.85,
            max_iterations: 100,
            convergence: 1e-6,
        }
    }

    /// Build and analyze the error graph.
    ///
    /// # Arguments
    /// * `error_counts` - Map of error codes to their frequencies
    /// * `co_occurrences` - Map of (error1, error2) pairs to co-occurrence count
    pub fn analyze(
        &self,
        error_counts: &HashMap<String, usize>,
        co_occurrences: &HashMap<(String, String), usize>,
    ) -> ErrorGraph {
        if error_counts.is_empty() {
            return ErrorGraph {
                nodes: vec![],
                edges: vec![],
                communities: vec![],
                top_by_pagerank: vec![],
                modularity: 0.0,
            };
        }

        // Build edges from co-occurrences
        let edges = self.build_edges(co_occurrences, error_counts);

        // Build adjacency map for PageRank
        let adjacency = self.build_adjacency(&edges, error_counts);

        // Calculate PageRank scores
        let pagerank = self.pagerank(&adjacency, error_counts.keys().cloned().collect());

        // Detect communities using Louvain
        let (community_map, modularity) = self.louvain_communities(&adjacency, error_counts);

        // Build nodes with PageRank and community info
        let mut nodes: Vec<ErrorNode> = error_counts
            .iter()
            .map(|(code, &count)| ErrorNode {
                code: code.clone(),
                count,
                pagerank: *pagerank.get(code).unwrap_or(&0.0),
                community: *community_map.get(code).unwrap_or(&0),
            })
            .collect();

        // Sort by PageRank descending
        nodes.sort_by(|a, b| b.pagerank.partial_cmp(&a.pagerank).unwrap());

        // Top errors by PageRank
        let top_by_pagerank: Vec<String> = nodes.iter().take(5).map(|n| n.code.clone()).collect();

        // Build communities
        let communities = self.build_communities(&nodes, &community_map);

        ErrorGraph {
            nodes,
            edges,
            communities,
            top_by_pagerank,
            modularity,
        }
    }

    fn build_edges(
        &self,
        co_occurrences: &HashMap<(String, String), usize>,
        error_counts: &HashMap<String, usize>,
    ) -> Vec<ErrorEdge> {
        let max_count = *error_counts.values().max().unwrap_or(&1) as f64;

        co_occurrences
            .iter()
            .map(|((from, to), &count)| {
                let weight = count as f64 / max_count;
                ErrorEdge {
                    from: from.clone(),
                    to: to.clone(),
                    weight,
                }
            })
            .collect()
    }

    fn build_adjacency(
        &self,
        edges: &[ErrorEdge],
        error_counts: &HashMap<String, usize>,
    ) -> HashMap<String, Vec<(String, f64)>> {
        let mut adjacency: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        // Initialize all nodes
        for code in error_counts.keys() {
            adjacency.entry(code.clone()).or_default();
        }

        // Add edges (bidirectional for undirected graph)
        for edge in edges {
            adjacency
                .entry(edge.from.clone())
                .or_default()
                .push((edge.to.clone(), edge.weight));
            adjacency
                .entry(edge.to.clone())
                .or_default()
                .push((edge.from.clone(), edge.weight));
        }

        adjacency
    }

    /// Calculate PageRank scores for error nodes.
    fn pagerank(
        &self,
        adjacency: &HashMap<String, Vec<(String, f64)>>,
        nodes: Vec<String>,
    ) -> HashMap<String, f64> {
        let n = nodes.len();
        if n == 0 {
            return HashMap::new();
        }

        let initial_score = 1.0 / n as f64;
        let mut scores: HashMap<String, f64> =
            nodes.iter().map(|n| (n.clone(), initial_score)).collect();

        for _ in 0..self.max_iterations {
            let mut new_scores: HashMap<String, f64> = HashMap::new();
            let mut max_diff = 0.0f64;

            for node in &nodes {
                // Sum contributions from incoming edges
                let incoming_sum: f64 = adjacency
                    .iter()
                    .filter(|(_, neighbors)| neighbors.iter().any(|(n, _)| n == node))
                    .map(|(from, neighbors)| {
                        let out_degree = neighbors.len() as f64;
                        if out_degree > 0.0 {
                            scores.get(from).unwrap_or(&0.0) / out_degree
                        } else {
                            0.0
                        }
                    })
                    .sum();

                let new_score = (1.0 - self.damping) / n as f64 + self.damping * incoming_sum;

                let old_score = *scores.get(node).unwrap_or(&0.0);
                max_diff = max_diff.max((new_score - old_score).abs());

                new_scores.insert(node.clone(), new_score);
            }

            scores = new_scores;

            if max_diff < self.convergence {
                break;
            }
        }

        // Normalize scores
        let sum: f64 = scores.values().sum();
        if sum > 0.0 {
            for score in scores.values_mut() {
                *score /= sum;
            }
        }

        scores
    }

    /// Detect communities using simplified Louvain algorithm.
    fn louvain_communities(
        &self,
        adjacency: &HashMap<String, Vec<(String, f64)>>,
        error_counts: &HashMap<String, usize>,
    ) -> (HashMap<String, usize>, f64) {
        let nodes: Vec<String> = error_counts.keys().cloned().collect();

        if nodes.is_empty() {
            return (HashMap::new(), 0.0);
        }

        // Initialize: each node in its own community
        let mut community: HashMap<String, usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();

        // Total edge weight
        let total_weight: f64 = adjacency
            .values()
            .flat_map(|neighbors| neighbors.iter())
            .map(|(_, w)| w)
            .sum::<f64>()
            / 2.0; // Divide by 2 for undirected graph

        if total_weight == 0.0 {
            return (community, 0.0);
        }

        // Iterate to improve modularity
        let mut improved = true;
        while improved {
            improved = false;

            for node in &nodes {
                let current_comm = *community.get(node).unwrap_or(&0);

                // Find neighboring communities
                let neighbor_comms: HashSet<usize> = adjacency
                    .get(node)
                    .map(|neighbors| {
                        neighbors
                            .iter()
                            .map(|(n, _)| *community.get(n).unwrap_or(&0))
                            .collect()
                    })
                    .unwrap_or_default();

                // Try moving to each neighboring community
                let mut best_comm = current_comm;
                let mut best_delta = 0.0;

                for &new_comm in &neighbor_comms {
                    if new_comm == current_comm {
                        continue;
                    }

                    let delta = self.modularity_delta(
                        node,
                        current_comm,
                        new_comm,
                        &community,
                        adjacency,
                        total_weight,
                    );

                    if delta > best_delta {
                        best_delta = delta;
                        best_comm = new_comm;
                    }
                }

                if best_comm != current_comm {
                    community.insert(node.clone(), best_comm);
                    improved = true;
                }
            }
        }

        // Renumber communities to be contiguous
        let mut comm_map: HashMap<usize, usize> = HashMap::new();
        let mut next_id = 0;
        for comm in community.values_mut() {
            let new_id = *comm_map.entry(*comm).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            *comm = new_id;
        }

        // Calculate final modularity
        let modularity = self.calculate_modularity(&community, adjacency, total_weight);

        (community, modularity)
    }

    fn modularity_delta(
        &self,
        node: &str,
        from_comm: usize,
        to_comm: usize,
        community: &HashMap<String, usize>,
        adjacency: &HashMap<String, Vec<(String, f64)>>,
        total_weight: f64,
    ) -> f64 {
        if total_weight == 0.0 {
            return 0.0;
        }

        // Sum of weights to nodes in target community
        let k_in: f64 = adjacency
            .get(node)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .filter(|(n, _)| *community.get(n).unwrap_or(&0) == to_comm)
                    .map(|(_, w)| w)
                    .sum()
            })
            .unwrap_or(0.0);

        // Sum of weights to nodes in source community
        let k_out: f64 = adjacency
            .get(node)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .filter(|(n, _)| *community.get(n).unwrap_or(&0) == from_comm)
                    .map(|(_, w)| w)
                    .sum()
            })
            .unwrap_or(0.0);

        // Node degree
        let k_i: f64 = adjacency
            .get(node)
            .map(|neighbors| neighbors.iter().map(|(_, w)| w).sum())
            .unwrap_or(0.0);

        // Sum of degrees in target community
        let sigma_tot: f64 = community
            .iter()
            .filter(|(_, &c)| c == to_comm)
            .map(|(n, _)| {
                adjacency
                    .get(n)
                    .map(|neighbors| neighbors.iter().map(|(_, w)| w).sum())
                    .unwrap_or(0.0)
            })
            .sum();

        // Modularity gain formula
        let m = total_weight;
        (k_in - k_out) / m - k_i * sigma_tot / (2.0 * m * m)
    }

    fn calculate_modularity(
        &self,
        community: &HashMap<String, usize>,
        adjacency: &HashMap<String, Vec<(String, f64)>>,
        total_weight: f64,
    ) -> f64 {
        if total_weight == 0.0 {
            return 0.0;
        }

        let m = total_weight;
        let mut q = 0.0;

        for (node_i, &comm_i) in community {
            let k_i: f64 = adjacency
                .get(node_i)
                .map(|n| n.iter().map(|(_, w)| w).sum())
                .unwrap_or(0.0);

            for (node_j, &comm_j) in community {
                if comm_i != comm_j {
                    continue;
                }

                let k_j: f64 = adjacency
                    .get(node_j)
                    .map(|n| n.iter().map(|(_, w)| w).sum())
                    .unwrap_or(0.0);

                // Edge weight between i and j
                let a_ij: f64 = adjacency
                    .get(node_i)
                    .and_then(|neighbors| neighbors.iter().find(|(n, _)| n == node_j))
                    .map(|(_, w)| *w)
                    .unwrap_or(0.0);

                q += a_ij - k_i * k_j / (2.0 * m);
            }
        }

        q / (2.0 * m)
    }

    fn build_communities(
        &self,
        nodes: &[ErrorNode],
        community_map: &HashMap<String, usize>,
    ) -> Vec<ErrorCommunity> {
        let mut communities: HashMap<usize, Vec<&ErrorNode>> = HashMap::new();

        for node in nodes {
            let comm = *community_map.get(&node.code).unwrap_or(&0);
            communities.entry(comm).or_default().push(node);
        }

        communities
            .into_iter()
            .map(|(id, members)| {
                let member_codes: Vec<String> = members.iter().map(|n| n.code.clone()).collect();

                let dominant = members
                    .iter()
                    .max_by_key(|n| n.count)
                    .map(|n| n.code.clone())
                    .unwrap_or_default();

                let total_count: usize = members.iter().map(|n| n.count).sum();

                let label = self.generate_community_label(&members);

                ErrorCommunity {
                    id,
                    members: member_codes,
                    dominant,
                    total_count,
                    label,
                }
            })
            .collect()
    }

    fn generate_community_label(&self, members: &[&ErrorNode]) -> String {
        if members.is_empty() {
            return "Empty Community".to_string();
        }

        // Analyze error code patterns
        let type_errors = members.iter().filter(|n| n.code.starts_with("E03")).count();
        let resolution_errors = members.iter().filter(|n| n.code.starts_with("E04")).count();
        let borrow_errors = members.iter().filter(|n| n.code.starts_with("E05")).count();

        let total = members.len();

        if type_errors > total / 2 {
            "Type System Community".to_string()
        } else if resolution_errors > total / 2 {
            "Resolution Community".to_string()
        } else if borrow_errors > total / 2 {
            "Ownership Community".to_string()
        } else {
            format!(
                "Mixed Community ({} errors)",
                members.iter().map(|n| n.count).sum::<usize>()
            )
        }
    }
}

/// Utility to extract co-occurrences from file error lists.
pub fn extract_co_occurrences(
    file_errors: &[(String, Vec<String>)], // (filename, error_codes)
) -> HashMap<(String, String), usize> {
    let mut co_occurrences: HashMap<(String, String), usize> = HashMap::new();

    for (_filename, errors) in file_errors {
        // For each pair of distinct errors in the same file
        for (i, e1) in errors.iter().enumerate() {
            for e2 in errors.iter().skip(i + 1) {
                // Canonical ordering to avoid duplicates
                let (a, b) = if e1 < e2 { (e1, e2) } else { (e2, e1) };
                *co_occurrences.entry((a.clone(), b.clone())).or_insert(0) += 1;
            }
        }
    }

    co_occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_analyzer_empty() {
        let analyzer = GraphAnalyzer::new();
        let result = analyzer.analyze(&HashMap::new(), &HashMap::new());

        assert!(result.nodes.is_empty());
        assert!(result.edges.is_empty());
        assert!(result.communities.is_empty());
    }

    #[test]
    fn test_graph_analyzer_single_error() {
        let analyzer = GraphAnalyzer::new();
        let mut errors = HashMap::new();
        errors.insert("E0308".to_string(), 10);

        let result = analyzer.analyze(&errors, &HashMap::new());

        assert_eq!(result.nodes.len(), 1);
        assert_eq!(result.nodes[0].code, "E0308");
        assert!(result.nodes[0].pagerank > 0.0);
    }

    #[test]
    fn test_graph_analyzer_with_co_occurrences() {
        let analyzer = GraphAnalyzer::new();
        let mut errors = HashMap::new();
        errors.insert("E0308".to_string(), 50);
        errors.insert("E0412".to_string(), 30);
        errors.insert("E0425".to_string(), 20);

        let mut co_occur = HashMap::new();
        co_occur.insert(("E0308".to_string(), "E0412".to_string()), 10);
        co_occur.insert(("E0308".to_string(), "E0425".to_string()), 5);

        let result = analyzer.analyze(&errors, &co_occur);

        assert_eq!(result.nodes.len(), 3);
        assert_eq!(result.edges.len(), 2);
        assert!(!result.top_by_pagerank.is_empty());
    }

    #[test]
    fn test_pagerank_convergence() {
        let analyzer = GraphAnalyzer::new();
        let mut adjacency: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        adjacency.insert("A".to_string(), vec![("B".to_string(), 1.0)]);
        adjacency.insert(
            "B".to_string(),
            vec![("A".to_string(), 1.0), ("C".to_string(), 1.0)],
        );
        adjacency.insert("C".to_string(), vec![("B".to_string(), 1.0)]);

        let nodes = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let scores = analyzer.pagerank(&adjacency, nodes);

        // All scores should sum to ~1.0
        let sum: f64 = scores.values().sum();
        assert!((sum - 1.0).abs() < 0.01);

        // B should have highest PageRank (most connections)
        assert!(scores.get("B").unwrap_or(&0.0) >= scores.get("A").unwrap_or(&0.0));
    }

    #[test]
    fn test_extract_co_occurrences() {
        let file_errors = vec![
            (
                "file1.rs".to_string(),
                vec!["E0308".to_string(), "E0412".to_string()],
            ),
            (
                "file2.rs".to_string(),
                vec![
                    "E0308".to_string(),
                    "E0412".to_string(),
                    "E0425".to_string(),
                ],
            ),
        ];

        let co_occur = extract_co_occurrences(&file_errors);

        // E0308-E0412 appears in both files
        assert_eq!(
            co_occur.get(&("E0308".to_string(), "E0412".to_string())),
            Some(&2)
        );
    }

    #[test]
    fn test_community_detection() {
        let analyzer = GraphAnalyzer::new();
        let mut errors = HashMap::new();
        errors.insert("E0308".to_string(), 50);
        errors.insert("E0309".to_string(), 40);
        errors.insert("E0425".to_string(), 20);
        errors.insert("E0426".to_string(), 15);

        let mut co_occur = HashMap::new();
        // Type errors co-occur together
        co_occur.insert(("E0308".to_string(), "E0309".to_string()), 20);
        // Resolution errors co-occur together
        co_occur.insert(("E0425".to_string(), "E0426".to_string()), 10);

        let result = analyzer.analyze(&errors, &co_occur);

        // Should have detected communities
        assert!(!result.communities.is_empty());
        assert!(result.modularity >= -0.5 && result.modularity <= 1.0);
    }

    #[test]
    fn test_error_node_creation() {
        let node = ErrorNode {
            code: "E0308".to_string(),
            count: 10,
            pagerank: 0.5,
            community: 0,
        };

        assert_eq!(node.code, "E0308");
        assert_eq!(node.count, 10);
    }

    #[test]
    fn test_community_label_generation() {
        let analyzer = GraphAnalyzer::new();

        let node1 = ErrorNode {
            code: "E0308".to_string(),
            count: 10,
            pagerank: 0.5,
            community: 0,
        };
        let node2 = ErrorNode {
            code: "E0309".to_string(),
            count: 5,
            pagerank: 0.3,
            community: 0,
        };
        let type_nodes = vec![&node1, &node2];

        let label = analyzer.generate_community_label(&type_nodes);
        assert!(label.contains("Type"));
    }
}

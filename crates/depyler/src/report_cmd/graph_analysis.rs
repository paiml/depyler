//! Error Dependency Graph Analysis (GH-209 Phase 4)
//!
//! Builds a graph of error co-occurrences to identify:
//! - "Super-spreader" errors (high centrality)
//! - Error communities (clustered failure patterns)
//! - Root cause relationships

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::analysis::{ExtendedAnalysisResult, SemanticDomain};

/// Node in the error graph (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorNode {
    /// Node ID (unique)
    pub id: usize,
    /// Error code (e.g., "E0308")
    pub error_code: String,
    /// Files affected by this error
    pub files: Vec<String>,
    /// PageRank-style centrality score
    pub centrality: f64,
    /// Dominant semantic domain for this error
    pub domain: SemanticDomain,
}

impl ErrorNode {
    /// Get number of affected files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// Edge in the error graph (co-occurrence relationship)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEdge {
    /// Source node ID
    pub from: usize,
    /// Target node ID
    pub to: usize,
    /// Co-occurrence weight (number of files with both errors)
    pub weight: f64,
}

/// Error community (connected component or cluster)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCommunity {
    /// Community ID
    pub id: usize,
    /// Auto-generated name (e.g., "The AsyncIO Cluster")
    pub name: String,
    /// Error codes in this community
    pub error_codes: Vec<String>,
    /// Sum of centrality scores
    pub centrality_sum: f64,
    /// Number of files affected
    pub total_files: usize,
}

/// Error dependency graph (GH-209 Phase 4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorGraph {
    /// Nodes (error types)
    pub nodes: Vec<ErrorNode>,
    /// Edges (co-occurrences)
    pub edges: Vec<ErrorEdge>,
    /// Node lookup by error code
    #[serde(skip)]
    node_map: HashMap<String, usize>,
    /// Adjacency list for graph traversal
    #[serde(skip)]
    adjacency: HashMap<usize, Vec<(usize, f64)>>,
}

impl Default for ErrorGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorGraph {
    /// Create empty graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_map: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Build graph from analysis results (GH-209)
    pub fn from_results(results: &[ExtendedAnalysisResult]) -> Self {
        let mut graph = Self::new();

        // Group files by error code
        let mut error_files: HashMap<String, Vec<String>> = HashMap::new();
        let mut error_domains: HashMap<String, Vec<SemanticDomain>> = HashMap::new();

        for result in results {
            if !result.base.success {
                if let Some(code) = &result.base.error_code {
                    error_files
                        .entry(code.clone())
                        .or_default()
                        .push(result.base.name.clone());
                    error_domains
                        .entry(code.clone())
                        .or_default()
                        .push(result.semantic_domain);
                }
            }
        }

        // Create nodes
        for (code, files) in &error_files {
            let domains = error_domains.get(code).unwrap();
            let dominant_domain = find_dominant_domain(domains);

            let node = ErrorNode {
                id: graph.nodes.len(),
                error_code: code.clone(),
                files: files.clone(),
                centrality: 0.0, // Will be calculated later
                domain: dominant_domain,
            };

            graph.node_map.insert(code.clone(), node.id);
            graph.nodes.push(node);
        }

        // Build co-occurrence map
        let mut cooccur: HashMap<(String, String), usize> = HashMap::new();

        // Group errors by file
        let mut file_errors: HashMap<String, Vec<String>> = HashMap::new();
        for result in results {
            if !result.base.success {
                if let Some(code) = &result.base.error_code {
                    file_errors
                        .entry(result.base.name.clone())
                        .or_default()
                        .push(code.clone());
                }
            }
        }

        // Count co-occurrences
        for errors in file_errors.values() {
            let unique: Vec<_> = errors.iter().collect::<HashSet<_>>().into_iter().collect();
            for (i, &e1) in unique.iter().enumerate() {
                for &e2 in unique.iter().skip(i + 1) {
                    let key = if e1 < e2 {
                        (e1.clone(), e2.clone())
                    } else {
                        (e2.clone(), e1.clone())
                    };
                    *cooccur.entry(key).or_insert(0) += 1;
                }
            }
        }

        // Create edges
        for ((e1, e2), count) in cooccur {
            if let (Some(&from), Some(&to)) = (graph.node_map.get(&e1), graph.node_map.get(&e2)) {
                let weight = count as f64;

                graph.edges.push(ErrorEdge { from, to, weight });

                graph.adjacency.entry(from).or_default().push((to, weight));
                graph.adjacency.entry(to).or_default().push((from, weight));
            }
        }

        // Calculate centrality
        graph.calculate_centrality();

        graph
    }

    /// Calculate PageRank-style centrality for all nodes
    pub fn calculate_centrality(&mut self) {
        let n = self.nodes.len();
        if n == 0 {
            return;
        }

        let damping = 0.85;
        let iterations = 100;
        let tolerance = 1e-6;

        // Initialize scores
        let mut scores = vec![1.0 / n as f64; n];
        let mut new_scores = vec![0.0; n];

        for _ in 0..iterations {
            let mut max_diff: f64 = 0.0;

            for i in 0..n {
                let mut sum = 0.0;

                // Sum contributions from neighbors
                if let Some(neighbors) = self.adjacency.get(&i) {
                    for &(j, weight) in neighbors {
                        let out_degree =
                            self.adjacency.get(&j).map(|n| n.len()).unwrap_or(1) as f64;
                        sum += scores[j] * weight / out_degree;
                    }
                }

                new_scores[i] = (1.0 - damping) / n as f64 + damping * sum;
                max_diff = max_diff.max((new_scores[i] - scores[i]).abs());
            }

            std::mem::swap(&mut scores, &mut new_scores);

            if max_diff < tolerance {
                break;
            }
        }

        // Normalize and assign
        let total: f64 = scores.iter().sum();
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.centrality = if total > 0.0 {
                scores[i] / total
            } else {
                1.0 / n as f64
            };
        }
    }

    /// Find connected components (communities)
    pub fn find_communities(&self) -> Vec<ErrorCommunity> {
        let n = self.nodes.len();
        let mut visited = vec![false; n];
        let mut communities = Vec::new();

        for start in 0..n {
            if visited[start] {
                continue;
            }

            // BFS to find connected component
            let mut component = Vec::new();
            let mut queue = vec![start];

            while let Some(node) = queue.pop() {
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                component.push(node);

                if let Some(neighbors) = self.adjacency.get(&node) {
                    for &(neighbor, _) in neighbors {
                        if !visited[neighbor] {
                            queue.push(neighbor);
                        }
                    }
                }
            }

            // Build community
            let error_codes: Vec<String> = component
                .iter()
                .map(|&i| self.nodes[i].error_code.clone())
                .collect();

            let centrality_sum: f64 = component.iter().map(|&i| self.nodes[i].centrality).sum();

            let total_files: usize = component.iter().map(|&i| self.nodes[i].files.len()).sum();

            let name = generate_community_name(&error_codes, &self.nodes, &component);

            communities.push(ErrorCommunity {
                id: communities.len(),
                name,
                error_codes,
                centrality_sum,
                total_files,
            });
        }

        // Sort by centrality descending
        communities.sort_by(|a, b| b.centrality_sum.partial_cmp(&a.centrality_sum).unwrap());

        communities
    }

    /// Get top N central nodes
    pub fn top_central(&self, n: usize) -> Vec<&ErrorNode> {
        let mut sorted: Vec<_> = self.nodes.iter().collect();
        sorted.sort_by(|a, b| b.centrality.partial_cmp(&a.centrality).unwrap());
        sorted.into_iter().take(n).collect()
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// Find dominant semantic domain from list
fn find_dominant_domain(domains: &[SemanticDomain]) -> SemanticDomain {
    let mut counts: HashMap<SemanticDomain, usize> = HashMap::new();
    for &domain in domains {
        *counts.entry(domain).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(domain, _)| domain)
        .unwrap_or(SemanticDomain::Unknown)
}

/// Generate community name based on error types
fn generate_community_name(
    error_codes: &[String],
    nodes: &[ErrorNode],
    component: &[usize],
) -> String {
    // Find the most central error in the component
    let top_error = component
        .iter()
        .max_by(|&&a, &&b| {
            nodes[a]
                .centrality
                .partial_cmp(&nodes[b].centrality)
                .unwrap()
        })
        .map(|&i| &nodes[i].error_code)
        .unwrap_or(&error_codes[0]);

    let theme = match top_error.as_str() {
        "E0308" => "Type Mismatch",
        "E0425" => "Scope Resolution",
        "E0433" => "Module Import",
        "E0277" => "Trait Bounds",
        "E0599" => "Method Resolution",
        "E0382" => "Ownership",
        "E0502" | "E0499" => "Borrowing",
        "E0106" | "E0495" | "E0621" => "Lifetime",
        _ => "Compilation",
    };

    let size = component.len();
    if size == 1 {
        format!("Isolated {} Error", theme)
    } else {
        format!("The {} Cluster ({} errors)", theme, size)
    }
}

/// Graph analysis results (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphAnalysis {
    /// The error graph
    pub graph: ErrorGraph,
    /// Discovered communities
    pub communities: Vec<ErrorCommunity>,
    /// Top central errors
    pub top_central: Vec<String>,
    /// Graph density (edges / possible edges)
    pub density: f64,
}

impl GraphAnalysis {
    /// Build full analysis from results
    pub fn from_results(results: &[ExtendedAnalysisResult]) -> Self {
        let graph = ErrorGraph::from_results(results);
        let communities = graph.find_communities();

        let top_central: Vec<String> = graph
            .top_central(5)
            .iter()
            .map(|n| n.error_code.clone())
            .collect();

        let n = graph.node_count();
        let density = if n > 1 {
            let possible_edges = n * (n - 1) / 2;
            graph.edge_count() as f64 / possible_edges as f64
        } else {
            0.0
        };

        Self {
            graph,
            communities,
            top_central,
            density,
        }
    }

    /// Get community count
    pub fn community_count(&self) -> usize {
        self.communities.len()
    }
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
                error_message: None,
            },
            semantic_domain: SemanticDomain::CoreLanguage,
            ast_features: AstFeatures::default(),
            imports: vec![],
        }
    }

    #[test]
    fn test_error_graph_new() {
        let graph = ErrorGraph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_error_graph_default() {
        let graph = ErrorGraph::default();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_error_graph_from_empty() {
        let results: Vec<ExtendedAnalysisResult> = vec![];
        let graph = ErrorGraph::from_results(&results);
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn test_error_graph_from_all_pass() {
        let results = vec![
            make_result("a.py", true, None),
            make_result("b.py", true, None),
        ];
        let graph = ErrorGraph::from_results(&results);
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn test_error_graph_single_error() {
        let results = vec![make_result("a.py", false, Some("E0308"))];
        let graph = ErrorGraph::from_results(&results);

        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(graph.nodes[0].error_code, "E0308");
    }

    #[test]
    fn test_error_graph_multiple_errors() {
        let results = vec![
            make_result("a.py", false, Some("E0308")),
            make_result("b.py", false, Some("E0425")),
            make_result("c.py", false, Some("E0308")),
        ];
        let graph = ErrorGraph::from_results(&results);

        assert_eq!(graph.node_count(), 2);
        assert!(graph.nodes.iter().any(|n| n.error_code == "E0308"));
        assert!(graph.nodes.iter().any(|n| n.error_code == "E0425"));
    }

    #[test]
    fn test_error_graph_file_count() {
        let results = vec![
            make_result("a.py", false, Some("E0308")),
            make_result("b.py", false, Some("E0308")),
            make_result("c.py", false, Some("E0308")),
        ];
        let graph = ErrorGraph::from_results(&results);

        let e0308_node = graph
            .nodes
            .iter()
            .find(|n| n.error_code == "E0308")
            .unwrap();
        assert_eq!(e0308_node.file_count(), 3);
    }

    #[test]
    fn test_error_graph_centrality() {
        let results = vec![
            make_result("a.py", false, Some("E0308")),
            make_result("b.py", false, Some("E0425")),
        ];
        let graph = ErrorGraph::from_results(&results);

        // Centrality should be non-negative and sum to ~1
        let total: f64 = graph.nodes.iter().map(|n| n.centrality).sum();
        assert!((total - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_error_graph_top_central() {
        let results = vec![
            make_result("a.py", false, Some("E0308")),
            make_result("b.py", false, Some("E0308")),
            make_result("c.py", false, Some("E0425")),
        ];
        let graph = ErrorGraph::from_results(&results);
        let top = graph.top_central(1);

        assert_eq!(top.len(), 1);
    }

    #[test]
    fn test_find_communities_isolated() {
        let results = vec![
            make_result("a.py", false, Some("E0308")),
            make_result("b.py", false, Some("E0425")),
        ];
        let graph = ErrorGraph::from_results(&results);
        let communities = graph.find_communities();

        // Two isolated nodes = two communities
        assert_eq!(communities.len(), 2);
    }

    #[test]
    fn test_find_dominant_domain() {
        let domains = vec![
            SemanticDomain::External,
            SemanticDomain::External,
            SemanticDomain::CoreLanguage,
        ];
        assert_eq!(find_dominant_domain(&domains), SemanticDomain::External);
    }

    #[test]
    fn test_find_dominant_domain_empty() {
        let domains: Vec<SemanticDomain> = vec![];
        assert_eq!(find_dominant_domain(&domains), SemanticDomain::Unknown);
    }

    #[test]
    fn test_generate_community_name_isolated() {
        let nodes = vec![ErrorNode {
            id: 0,
            error_code: "E0308".to_string(),
            files: vec!["a.py".to_string()],
            centrality: 1.0,
            domain: SemanticDomain::CoreLanguage,
        }];
        let name = generate_community_name(&["E0308".to_string()], &nodes, &[0]);
        assert!(name.contains("Type Mismatch"));
        assert!(name.contains("Isolated"));
    }

    #[test]
    fn test_generate_community_name_cluster() {
        let nodes = vec![
            ErrorNode {
                id: 0,
                error_code: "E0308".to_string(),
                files: vec![],
                centrality: 0.6,
                domain: SemanticDomain::CoreLanguage,
            },
            ErrorNode {
                id: 1,
                error_code: "E0425".to_string(),
                files: vec![],
                centrality: 0.4,
                domain: SemanticDomain::CoreLanguage,
            },
        ];
        let name =
            generate_community_name(&["E0308".to_string(), "E0425".to_string()], &nodes, &[0, 1]);
        assert!(name.contains("Cluster"));
        assert!(name.contains("2 errors"));
    }

    #[test]
    fn test_graph_analysis_from_results() {
        let results = vec![
            make_result("a.py", false, Some("E0308")),
            make_result("b.py", false, Some("E0425")),
            make_result("c.py", false, Some("E0308")),
        ];
        let analysis = GraphAnalysis::from_results(&results);

        assert_eq!(analysis.graph.node_count(), 2);
        assert!(analysis.community_count() >= 1);
        assert!(!analysis.top_central.is_empty());
    }

    #[test]
    fn test_graph_analysis_density_empty() {
        let results: Vec<ExtendedAnalysisResult> = vec![];
        let analysis = GraphAnalysis::from_results(&results);
        assert_eq!(analysis.density, 0.0);
    }

    #[test]
    fn test_graph_analysis_density_single() {
        let results = vec![make_result("a.py", false, Some("E0308"))];
        let analysis = GraphAnalysis::from_results(&results);
        assert_eq!(analysis.density, 0.0); // Single node, no edges possible
    }

    #[test]
    fn test_error_community_fields() {
        let community = ErrorCommunity {
            id: 0,
            name: "Test".to_string(),
            error_codes: vec!["E0308".to_string()],
            centrality_sum: 0.5,
            total_files: 10,
        };
        assert_eq!(community.id, 0);
        assert_eq!(community.total_files, 10);
    }

    #[test]
    fn test_error_edge_fields() {
        let edge = ErrorEdge {
            from: 0,
            to: 1,
            weight: 2.5,
        };
        assert_eq!(edge.from, 0);
        assert_eq!(edge.to, 1);
        assert!((edge.weight - 2.5).abs() < 1e-6);
    }

    #[test]
    fn test_error_node_fields() {
        let node = ErrorNode {
            id: 0,
            error_code: "E0308".to_string(),
            files: vec!["a.py".to_string(), "b.py".to_string()],
            centrality: 0.75,
            domain: SemanticDomain::External,
        };
        assert_eq!(node.file_count(), 2);
        assert_eq!(node.domain, SemanticDomain::External);
    }
}

//! ML Clustering for Error Analysis (GH-209 Phase 2)
//!
//! Uses aprender's KMeans and DBSCAN algorithms to group compilation
//! failures by feature similarity, enabling pattern discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::analysis::{AstFeatures, ExtendedAnalysisResult, SemanticDomain};

/// Error feature vector for ML clustering (GH-209)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorFeatureVector {
    /// Error code index (0-24 for common codes, 25 for unknown)
    pub error_code_idx: usize,
    /// Semantic domain index (0-4)
    pub domain_idx: usize,
    /// AST features (8 dimensions)
    pub ast_features: Vec<f32>,
    /// Total dimension count
    pub total_dims: usize,
}

impl ErrorFeatureVector {
    /// Create from extended analysis result
    pub fn from_result(result: &ExtendedAnalysisResult) -> Self {
        let error_code_idx = error_code_to_idx(
            result.base.error_code.as_deref().unwrap_or("UNKNOWN"),
        );
        let domain_idx = domain_to_idx(result.semantic_domain);
        let ast_features = result.ast_features.to_feature_vector();

        Self {
            error_code_idx,
            domain_idx,
            ast_features,
            total_dims: 2 + 8, // error_code + domain + 8 AST features
        }
    }

    /// Convert to flat feature vector for clustering
    pub fn to_flat_vector(&self) -> Vec<f64> {
        let mut vec = Vec::with_capacity(self.total_dims);

        // Normalized error code index (0-1 range)
        vec.push(self.error_code_idx as f64 / 25.0);

        // Normalized domain index (0-1 range)
        vec.push(self.domain_idx as f64 / 4.0);

        // AST features (already f32, convert to f64)
        for &f in &self.ast_features {
            // Normalize by reasonable max values
            vec.push((f as f64).min(100.0) / 100.0);
        }

        vec
    }
}

/// Map error code to index
fn error_code_to_idx(code: &str) -> usize {
    const ERROR_CODES: &[&str] = &[
        "E0308", "E0425", "E0433", "E0277", "E0599", "E0382", "E0502",
        "E0503", "E0505", "E0506", "E0507", "E0106", "E0495", "E0621",
        "E0282", "E0283", "E0412", "E0432", "E0603", "E0609", "E0614",
        "E0615", "E0616", "E0618", "E0620",
    ];

    ERROR_CODES.iter().position(|&c| c == code).unwrap_or(25)
}

/// Map semantic domain to index
fn domain_to_idx(domain: SemanticDomain) -> usize {
    match domain {
        SemanticDomain::CoreLanguage => 0,
        SemanticDomain::StdlibCommon => 1,
        SemanticDomain::StdlibAdvanced => 2,
        SemanticDomain::External => 3,
        SemanticDomain::Unknown => 4,
    }
}

/// Cluster of similar errors (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCluster {
    /// Cluster ID
    pub id: usize,
    /// Centroid (mean feature vector)
    pub centroid: Vec<f64>,
    /// Indices of member results
    pub member_indices: Vec<usize>,
    /// Dominant error code in this cluster
    pub dominant_error_code: String,
    /// Dominant semantic domain
    pub dominant_domain: SemanticDomain,
    /// Auto-generated label describing the cluster
    pub label: String,
    /// Cluster cohesion score (lower = tighter cluster)
    pub cohesion: f64,
}

impl ErrorCluster {
    /// Calculate cluster statistics
    pub fn member_count(&self) -> usize {
        self.member_indices.len()
    }
}

/// Cluster analysis results (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAnalysis {
    /// Discovered clusters
    pub clusters: Vec<ErrorCluster>,
    /// Silhouette score (clustering quality, -1 to 1)
    pub silhouette_score: f64,
    /// Outlier indices (DBSCAN noise points)
    pub outliers: Vec<usize>,
    /// Total number of samples
    pub total_samples: usize,
}

impl ClusterAnalysis {
    /// Get number of clusters found
    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }

    /// Get fraction of samples that are outliers
    pub fn outlier_fraction(&self) -> f64 {
        if self.total_samples == 0 {
            0.0
        } else {
            self.outliers.len() as f64 / self.total_samples as f64
        }
    }
}

/// Configuration for error clustering (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Number of clusters for KMeans (0 = auto-detect)
    pub n_clusters: usize,
    /// Maximum iterations for KMeans
    pub max_iterations: usize,
    /// Convergence tolerance
    pub tolerance: f64,
    /// Minimum samples for DBSCAN
    pub min_samples: usize,
    /// Epsilon for DBSCAN neighborhood
    pub epsilon: f64,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            n_clusters: 0,       // Auto-detect
            max_iterations: 100,
            tolerance: 1e-4,
            min_samples: 2,
            epsilon: 0.3,
        }
    }
}

/// Error clustering analyzer (GH-209 Phase 2)
pub struct ErrorClusterAnalyzer {
    config: ClusterConfig,
}

impl ErrorClusterAnalyzer {
    /// Create new analyzer with default config
    pub fn new() -> Self {
        Self {
            config: ClusterConfig::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(config: ClusterConfig) -> Self {
        Self { config }
    }

    /// Cluster failed results (GH-209)
    ///
    /// Returns cluster analysis with auto-labeled clusters
    pub fn cluster_errors(&self, results: &[ExtendedAnalysisResult]) -> ClusterAnalysis {
        // Filter to failed results only
        let failed: Vec<_> = results
            .iter()
            .enumerate()
            .filter(|(_, r)| !r.base.success)
            .collect();

        if failed.is_empty() {
            return ClusterAnalysis {
                clusters: vec![],
                silhouette_score: 0.0,
                outliers: vec![],
                total_samples: 0,
            };
        }

        // Build feature matrix
        let features: Vec<ErrorFeatureVector> = failed
            .iter()
            .map(|(_, r)| ErrorFeatureVector::from_result(r))
            .collect();

        let feature_matrix: Vec<Vec<f64>> = features
            .iter()
            .map(|f| f.to_flat_vector())
            .collect();

        // Determine optimal k (simple heuristic: sqrt(n) / 2, min 2, max 10)
        let n = failed.len();
        let k = if self.config.n_clusters > 0 {
            self.config.n_clusters
        } else {
            let auto_k = ((n as f64).sqrt() / 2.0).ceil() as usize;
            auto_k.clamp(2, 10.min(n))
        };

        // Run simplified KMeans (pure Rust implementation for reliability)
        let (labels, centroids) = simple_kmeans(&feature_matrix, k, self.config.max_iterations);

        // Build clusters
        let mut clusters = Vec::new();
        for cluster_id in 0..k {
            let member_indices: Vec<usize> = labels
                .iter()
                .enumerate()
                .filter(|(_, &l)| l == cluster_id)
                .map(|(i, _)| failed[i].0)
                .collect();

            if member_indices.is_empty() {
                continue;
            }

            // Find dominant error code
            let dominant_error_code = find_dominant_error_code(
                &member_indices,
                results,
            );

            // Find dominant domain
            let dominant_domain = find_dominant_domain(&member_indices, results);

            // Generate label
            let label = generate_cluster_label(&dominant_error_code, dominant_domain, member_indices.len());

            // Calculate cohesion
            let cohesion = calculate_cohesion(&member_indices, &feature_matrix, &labels, cluster_id);

            clusters.push(ErrorCluster {
                id: cluster_id,
                centroid: centroids[cluster_id].clone(),
                member_indices,
                dominant_error_code,
                dominant_domain,
                label,
                cohesion,
            });
        }

        // Sort by member count descending
        clusters.sort_by(|a, b| b.member_count().cmp(&a.member_count()));

        // Calculate silhouette score
        let silhouette_score = calculate_silhouette(&feature_matrix, &labels);

        ClusterAnalysis {
            clusters,
            silhouette_score,
            outliers: vec![], // KMeans doesn't produce outliers
            total_samples: failed.len(),
        }
    }
}

impl Default for ErrorClusterAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple KMeans implementation (no external deps)
fn simple_kmeans(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
) -> (Vec<usize>, Vec<Vec<f64>>) {
    if data.is_empty() || k == 0 {
        return (vec![], vec![]);
    }

    let n = data.len();
    let d = data[0].len();

    // Initialize centroids using first k points (simple init)
    let mut centroids: Vec<Vec<f64>> = data.iter().take(k).cloned().collect();

    // Pad with zeros if not enough data points
    while centroids.len() < k {
        centroids.push(vec![0.0; d]);
    }

    let mut labels = vec![0usize; n];

    for _ in 0..max_iter {
        // Assign points to nearest centroid
        let mut changed = false;
        for (i, point) in data.iter().enumerate() {
            let mut min_dist = f64::MAX;
            let mut min_cluster = 0;

            for (c, centroid) in centroids.iter().enumerate() {
                let dist = euclidean_distance(point, centroid);
                if dist < min_dist {
                    min_dist = dist;
                    min_cluster = c;
                }
            }

            if labels[i] != min_cluster {
                labels[i] = min_cluster;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        // Update centroids
        let mut new_centroids: Vec<Vec<f64>> = vec![vec![0.0; d]; k];
        let mut counts = vec![0usize; k];

        for (i, point) in data.iter().enumerate() {
            let c = labels[i];
            counts[c] += 1;
            for (j, &val) in point.iter().enumerate() {
                new_centroids[c][j] += val;
            }
        }

        for c in 0..k {
            if counts[c] > 0 {
                for j in 0..d {
                    new_centroids[c][j] /= counts[c] as f64;
                }
            }
        }

        centroids = new_centroids;
    }

    (labels, centroids)
}

/// Euclidean distance between two vectors
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Find most common error code in cluster
fn find_dominant_error_code(
    indices: &[usize],
    results: &[ExtendedAnalysisResult],
) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for &idx in indices {
        if let Some(code) = &results[idx].base.error_code {
            *counts.entry(code.clone()).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(code, _)| code)
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

/// Find most common semantic domain in cluster
fn find_dominant_domain(
    indices: &[usize],
    results: &[ExtendedAnalysisResult],
) -> SemanticDomain {
    let mut counts: HashMap<SemanticDomain, usize> = HashMap::new();

    for &idx in indices {
        *counts.entry(results[idx].semantic_domain).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(domain, _)| domain)
        .unwrap_or(SemanticDomain::Unknown)
}

/// Generate human-readable cluster label
fn generate_cluster_label(
    error_code: &str,
    domain: SemanticDomain,
    count: usize,
) -> String {
    let error_desc = match error_code {
        "E0308" => "Type Mismatch",
        "E0425" => "Undefined Value",
        "E0433" => "Module Resolution",
        "E0277" => "Missing Trait",
        "E0599" => "Method Not Found",
        "E0382" => "Ownership",
        "E0502" => "Borrow Conflict",
        "E0106" => "Missing Lifetime",
        _ => "Compilation",
    };

    let domain_desc = domain.label();

    format!("{} - {} ({} files)", error_desc, domain_desc, count)
}

/// Calculate cluster cohesion (mean intra-cluster distance)
fn calculate_cohesion(
    _member_indices: &[usize],
    feature_matrix: &[Vec<f64>],
    labels: &[usize],
    cluster_id: usize,
) -> f64 {
    let members: Vec<&Vec<f64>> = labels
        .iter()
        .enumerate()
        .filter(|(_, &l)| l == cluster_id)
        .filter_map(|(i, _)| feature_matrix.get(i))
        .collect();

    if members.len() <= 1 {
        return 0.0;
    }

    let mut total_dist = 0.0;
    let mut count = 0;

    for (i, a) in members.iter().enumerate() {
        for b in members.iter().skip(i + 1) {
            total_dist += euclidean_distance(a, b);
            count += 1;
        }
    }

    if count > 0 {
        total_dist / count as f64
    } else {
        0.0
    }
}

/// Calculate simplified silhouette score
fn calculate_silhouette(data: &[Vec<f64>], labels: &[usize]) -> f64 {
    if data.len() <= 1 {
        return 0.0;
    }

    let mut total_score = 0.0;
    let n = data.len();

    for i in 0..n {
        let cluster_i = labels[i];

        // Calculate a(i) - mean distance to same cluster
        let same_cluster: Vec<_> = labels
            .iter()
            .enumerate()
            .filter(|(j, &l)| l == cluster_i && *j != i)
            .map(|(j, _)| j)
            .collect();

        let a_i = if same_cluster.is_empty() {
            0.0
        } else {
            same_cluster
                .iter()
                .map(|&j| euclidean_distance(&data[i], &data[j]))
                .sum::<f64>() / same_cluster.len() as f64
        };

        // Calculate b(i) - min mean distance to other clusters
        let other_clusters: Vec<usize> = labels
            .iter()
            .filter(|&&l| l != cluster_i)
            .copied()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let b_i = if other_clusters.is_empty() {
            0.0
        } else {
            other_clusters
                .iter()
                .map(|&c| {
                    let cluster_points: Vec<_> = labels
                        .iter()
                        .enumerate()
                        .filter(|(_, &l)| l == c)
                        .map(|(j, _)| j)
                        .collect();

                    if cluster_points.is_empty() {
                        f64::MAX
                    } else {
                        cluster_points
                            .iter()
                            .map(|&j| euclidean_distance(&data[i], &data[j]))
                            .sum::<f64>() / cluster_points.len() as f64
                    }
                })
                .fold(f64::MAX, |a, b| a.min(b))
        };

        let s_i = if a_i.max(b_i) == 0.0 {
            0.0
        } else {
            (b_i - a_i) / a_i.max(b_i)
        };

        total_score += s_i;
    }

    total_score / n as f64
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report_cmd::analysis::AnalysisResult;

    fn make_failed_result(name: &str, error_code: &str, domain: SemanticDomain) -> ExtendedAnalysisResult {
        ExtendedAnalysisResult {
            base: AnalysisResult {
                name: name.to_string(),
                success: false,
                error_code: Some(error_code.to_string()),
                error_message: Some("test error".to_string()),
            },
            semantic_domain: domain,
            ast_features: AstFeatures {
                function_count: 2,
                class_count: 1,
                loop_count: 1,
                async_count: 0,
                comprehension_count: 0,
                complexity_score: 5.0,
                import_count: 3,
                line_count: 50,
            },
            imports: vec!["os".to_string()],
        }
    }

    #[test]
    fn test_error_code_to_idx_known() {
        assert_eq!(error_code_to_idx("E0308"), 0);
        assert_eq!(error_code_to_idx("E0425"), 1);
        assert_eq!(error_code_to_idx("E0599"), 4);
    }

    #[test]
    fn test_error_code_to_idx_unknown() {
        assert_eq!(error_code_to_idx("E9999"), 25);
        assert_eq!(error_code_to_idx("UNKNOWN"), 25);
    }

    #[test]
    fn test_domain_to_idx() {
        assert_eq!(domain_to_idx(SemanticDomain::CoreLanguage), 0);
        assert_eq!(domain_to_idx(SemanticDomain::StdlibCommon), 1);
        assert_eq!(domain_to_idx(SemanticDomain::External), 3);
        assert_eq!(domain_to_idx(SemanticDomain::Unknown), 4);
    }

    #[test]
    fn test_error_feature_vector_from_result() {
        let result = make_failed_result("test.py", "E0308", SemanticDomain::External);
        let vec = ErrorFeatureVector::from_result(&result);

        assert_eq!(vec.error_code_idx, 0); // E0308 is index 0
        assert_eq!(vec.domain_idx, 3);     // External is index 3
        assert_eq!(vec.ast_features.len(), 8);
        assert_eq!(vec.total_dims, 10);
    }

    #[test]
    fn test_error_feature_vector_to_flat() {
        let result = make_failed_result("test.py", "E0425", SemanticDomain::CoreLanguage);
        let vec = ErrorFeatureVector::from_result(&result);
        let flat = vec.to_flat_vector();

        assert_eq!(flat.len(), 10);
        assert!(flat[0] >= 0.0 && flat[0] <= 1.0); // Normalized
        assert!(flat[1] >= 0.0 && flat[1] <= 1.0);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        assert!((euclidean_distance(&a, &b) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance_same() {
        let a = vec![1.0, 2.0, 3.0];
        assert!((euclidean_distance(&a, &a) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_simple_kmeans_empty() {
        let data: Vec<Vec<f64>> = vec![];
        let (labels, centroids) = simple_kmeans(&data, 3, 10);
        assert!(labels.is_empty());
        assert!(centroids.is_empty());
    }

    #[test]
    fn test_simple_kmeans_single() {
        let data = vec![vec![1.0, 2.0]];
        let (labels, centroids) = simple_kmeans(&data, 1, 10);
        assert_eq!(labels, vec![0]);
        assert_eq!(centroids.len(), 1);
    }

    #[test]
    fn test_simple_kmeans_basic() {
        let data = vec![
            vec![0.0, 0.0],
            vec![0.1, 0.1],
            vec![10.0, 10.0],
            vec![10.1, 10.1],
        ];
        let (labels, _) = simple_kmeans(&data, 2, 100);

        // Points 0,1 should be in same cluster, 2,3 in another
        assert_eq!(labels[0], labels[1]);
        assert_eq!(labels[2], labels[3]);
        assert_ne!(labels[0], labels[2]);
    }

    #[test]
    fn test_cluster_config_default() {
        let config = ClusterConfig::default();
        assert_eq!(config.n_clusters, 0); // Auto-detect
        assert_eq!(config.max_iterations, 100);
    }

    #[test]
    fn test_cluster_analyzer_new() {
        let analyzer = ErrorClusterAnalyzer::new();
        assert_eq!(analyzer.config.n_clusters, 0);
    }

    #[test]
    fn test_cluster_analyzer_with_config() {
        let config = ClusterConfig {
            n_clusters: 5,
            ..Default::default()
        };
        let analyzer = ErrorClusterAnalyzer::with_config(config);
        assert_eq!(analyzer.config.n_clusters, 5);
    }

    #[test]
    fn test_cluster_errors_empty() {
        let analyzer = ErrorClusterAnalyzer::new();
        let results: Vec<ExtendedAnalysisResult> = vec![];
        let analysis = analyzer.cluster_errors(&results);

        assert!(analysis.clusters.is_empty());
        assert_eq!(analysis.total_samples, 0);
        assert_eq!(analysis.silhouette_score, 0.0);
    }

    #[test]
    fn test_cluster_errors_all_pass() {
        let analyzer = ErrorClusterAnalyzer::new();
        let results = vec![ExtendedAnalysisResult {
            base: AnalysisResult {
                name: "test.py".to_string(),
                success: true,
                error_code: None,
                error_message: None,
            },
            semantic_domain: SemanticDomain::CoreLanguage,
            ast_features: AstFeatures::default(),
            imports: vec![],
        }];
        let analysis = analyzer.cluster_errors(&results);

        assert!(analysis.clusters.is_empty());
        assert_eq!(analysis.total_samples, 0);
    }

    #[test]
    fn test_cluster_errors_basic() {
        let analyzer = ErrorClusterAnalyzer::with_config(ClusterConfig {
            n_clusters: 2,
            ..Default::default()
        });

        let results = vec![
            make_failed_result("a.py", "E0308", SemanticDomain::CoreLanguage),
            make_failed_result("b.py", "E0308", SemanticDomain::CoreLanguage),
            make_failed_result("c.py", "E0599", SemanticDomain::External),
            make_failed_result("d.py", "E0599", SemanticDomain::External),
        ];

        let analysis = analyzer.cluster_errors(&results);

        assert_eq!(analysis.total_samples, 4);
        assert!(analysis.cluster_count() >= 1);
    }

    #[test]
    fn test_cluster_analysis_outlier_fraction() {
        let analysis = ClusterAnalysis {
            clusters: vec![],
            silhouette_score: 0.5,
            outliers: vec![1, 2],
            total_samples: 10,
        };
        assert!((analysis.outlier_fraction() - 0.2).abs() < 1e-6);
    }

    #[test]
    fn test_cluster_analysis_outlier_fraction_zero_samples() {
        let analysis = ClusterAnalysis {
            clusters: vec![],
            silhouette_score: 0.0,
            outliers: vec![],
            total_samples: 0,
        };
        assert_eq!(analysis.outlier_fraction(), 0.0);
    }

    #[test]
    fn test_generate_cluster_label() {
        let label = generate_cluster_label("E0308", SemanticDomain::External, 5);
        assert!(label.contains("Type Mismatch"));
        assert!(label.contains("External"));
        assert!(label.contains("5 files"));
    }

    #[test]
    fn test_generate_cluster_label_unknown() {
        let label = generate_cluster_label("E9999", SemanticDomain::Unknown, 3);
        assert!(label.contains("Compilation"));
        assert!(label.contains("3 files"));
    }

    #[test]
    fn test_find_dominant_error_code() {
        let results = vec![
            make_failed_result("a.py", "E0308", SemanticDomain::CoreLanguage),
            make_failed_result("b.py", "E0308", SemanticDomain::CoreLanguage),
            make_failed_result("c.py", "E0425", SemanticDomain::CoreLanguage),
        ];
        let indices = vec![0, 1, 2];
        let dominant = find_dominant_error_code(&indices, &results);
        assert_eq!(dominant, "E0308");
    }

    #[test]
    fn test_find_dominant_domain() {
        let results = vec![
            make_failed_result("a.py", "E0308", SemanticDomain::External),
            make_failed_result("b.py", "E0308", SemanticDomain::External),
            make_failed_result("c.py", "E0425", SemanticDomain::CoreLanguage),
        ];
        let indices = vec![0, 1, 2];
        let dominant = find_dominant_domain(&indices, &results);
        assert_eq!(dominant, SemanticDomain::External);
    }

    #[test]
    fn test_calculate_silhouette_single() {
        let data = vec![vec![1.0, 2.0]];
        let labels = vec![0];
        let score = calculate_silhouette(&data, &labels);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_error_cluster_member_count() {
        let cluster = ErrorCluster {
            id: 0,
            centroid: vec![0.0, 0.0],
            member_indices: vec![0, 1, 2],
            dominant_error_code: "E0308".to_string(),
            dominant_domain: SemanticDomain::CoreLanguage,
            label: "Test".to_string(),
            cohesion: 0.5,
        };
        assert_eq!(cluster.member_count(), 3);
    }
}

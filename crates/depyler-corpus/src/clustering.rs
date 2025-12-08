//! ML-based error clustering module (DEPYLER-REPORT-V2).
//!
//! Uses feature vectors to cluster similar errors for pattern discovery.
//! Designed for optional integration with aprender for GPU-accelerated K-means.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A cluster of similar errors identified by the analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCluster {
    /// Cluster ID (0-indexed).
    pub id: usize,
    /// Centroid feature vector.
    pub centroid: Vec<f64>,
    /// Error codes in this cluster.
    pub members: Vec<String>,
    /// Dominant error code in the cluster.
    pub dominant_code: String,
    /// Cluster size.
    pub size: usize,
    /// Intra-cluster variance (lower = tighter cluster).
    pub variance: f64,
    /// Human-readable label for the cluster.
    pub label: String,
}

/// Feature vector for an error, used in clustering.
#[derive(Debug, Clone)]
pub struct ErrorFeatures {
    /// Error code.
    pub code: String,
    /// Frequency count.
    pub count: usize,
    /// Category index (0-8 for ErrorCategory variants).
    pub category_idx: usize,
    /// Is type-related error (E03xx).
    pub is_type_error: bool,
    /// Is resolution error (E04xx).
    pub is_resolution_error: bool,
    /// Is borrow checker error (E05xx).
    pub is_borrow_error: bool,
    /// Severity based on frequency (0.0-1.0).
    pub severity: f64,
}

impl ErrorFeatures {
    /// Convert to feature vector for clustering.
    pub fn to_vector(&self) -> Vec<f64> {
        vec![
            self.count as f64,
            self.category_idx as f64,
            if self.is_type_error { 1.0 } else { 0.0 },
            if self.is_resolution_error { 1.0 } else { 0.0 },
            if self.is_borrow_error { 1.0 } else { 0.0 },
            self.severity,
        ]
    }

    /// Create from error code and count.
    pub fn from_error(code: &str, count: usize, total_errors: usize) -> Self {
        let category_idx = Self::category_index(code);
        let severity = if total_errors > 0 {
            (count as f64 / total_errors as f64).min(1.0)
        } else {
            0.0
        };

        Self {
            code: code.to_string(),
            count,
            category_idx,
            is_type_error: code.starts_with("E03"),
            is_resolution_error: code.starts_with("E04"),
            is_borrow_error: code.starts_with("E05"),
            severity,
        }
    }

    fn category_index(code: &str) -> usize {
        match code {
            "E0308" => 0, // TypeMismatch
            "E0412" => 1, // UndefinedType
            "E0425" => 2, // UndefinedValue
            "E0282" => 3, // TypeAnnotation
            "E0277" => 4, // TraitBound
            c if c.starts_with("E050") => 5, // BorrowCheck
            c if c.starts_with("E010") || c.starts_with("E062") => 6, // Lifetime
            c if c.starts_with("E006") || c.starts_with("E043") => 7, // Syntax
            _ => 8, // Other
        }
    }
}

/// Result of cluster analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringResult {
    /// Identified clusters.
    pub clusters: Vec<ErrorCluster>,
    /// Total number of error types analyzed.
    pub total_error_types: usize,
    /// Silhouette score (-1 to 1, higher = better clustering).
    pub silhouette_score: f64,
    /// Optimal K used.
    pub k: usize,
}

/// Error cluster analyzer using K-means algorithm.
pub struct ClusterAnalyzer {
    /// Maximum number of clusters to try.
    max_k: usize,
    /// Maximum iterations for K-means.
    max_iterations: usize,
    /// Convergence threshold.
    convergence_threshold: f64,
}

impl Default for ClusterAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterAnalyzer {
    /// Create a new cluster analyzer with default parameters.
    pub fn new() -> Self {
        Self {
            max_k: 5,
            max_iterations: 100,
            convergence_threshold: 0.001,
        }
    }

    /// Set maximum K for elbow method.
    pub fn with_max_k(mut self, max_k: usize) -> Self {
        self.max_k = max_k;
        self
    }

    /// Analyze error distribution and identify clusters.
    pub fn analyze(&self, error_counts: &HashMap<String, usize>) -> ClusteringResult {
        if error_counts.is_empty() {
            return ClusteringResult {
                clusters: vec![],
                total_error_types: 0,
                silhouette_score: 0.0,
                k: 0,
            };
        }

        let total_errors: usize = error_counts.values().sum();

        // Convert errors to feature vectors
        let features: Vec<ErrorFeatures> = error_counts
            .iter()
            .map(|(code, &count)| ErrorFeatures::from_error(code, count, total_errors))
            .collect();

        let data: Vec<Vec<f64>> = features.iter().map(|f| f.to_vector()).collect();

        // Determine optimal K using elbow method
        let optimal_k = self.find_optimal_k(&data);

        // Run K-means clustering
        let (assignments, centroids) = self.kmeans(&data, optimal_k);

        // Build clusters
        let clusters = self.build_clusters(&features, &assignments, &centroids);

        // Calculate silhouette score
        let silhouette = self.calculate_silhouette(&data, &assignments);

        ClusteringResult {
            clusters,
            total_error_types: error_counts.len(),
            silhouette_score: silhouette,
            k: optimal_k,
        }
    }

    /// Find optimal K using simplified elbow method.
    fn find_optimal_k(&self, data: &[Vec<f64>]) -> usize {
        if data.len() <= 2 {
            return data.len();
        }

        let max_k = self.max_k.min(data.len());
        let mut best_k = 2;
        let mut best_improvement = 0.0;
        let mut prev_inertia = f64::MAX;

        for k in 2..=max_k {
            let (assignments, centroids) = self.kmeans(data, k);
            let inertia = self.calculate_inertia(data, &assignments, &centroids);

            if prev_inertia < f64::MAX {
                let improvement = (prev_inertia - inertia) / prev_inertia;
                if improvement > best_improvement && improvement > 0.1 {
                    best_improvement = improvement;
                    best_k = k;
                }
            }
            prev_inertia = inertia;
        }

        best_k
    }

    /// Simple K-means implementation.
    fn kmeans(&self, data: &[Vec<f64>], k: usize) -> (Vec<usize>, Vec<Vec<f64>>) {
        if data.is_empty() || k == 0 {
            return (vec![], vec![]);
        }

        let k = k.min(data.len());
        let dim = data[0].len();

        // Initialize centroids (first k points)
        let mut centroids: Vec<Vec<f64>> = data.iter().take(k).cloned().collect();

        let mut assignments = vec![0usize; data.len()];

        for _ in 0..self.max_iterations {
            // Assignment step
            let mut changed = false;
            for (i, point) in data.iter().enumerate() {
                let nearest = self.nearest_centroid(point, &centroids);
                if assignments[i] != nearest {
                    assignments[i] = nearest;
                    changed = true;
                }
            }

            if !changed {
                break;
            }

            // Update step
            let mut new_centroids = vec![vec![0.0; dim]; k];
            let mut counts = vec![0usize; k];

            for (i, point) in data.iter().enumerate() {
                let cluster = assignments[i];
                counts[cluster] += 1;
                for (j, &val) in point.iter().enumerate() {
                    new_centroids[cluster][j] += val;
                }
            }

            for (i, centroid) in new_centroids.iter_mut().enumerate() {
                if counts[i] > 0 {
                    for val in centroid.iter_mut() {
                        *val /= counts[i] as f64;
                    }
                }
            }

            // Check convergence
            let mut max_diff = 0.0f64;
            for (old, new) in centroids.iter().zip(new_centroids.iter()) {
                let diff: f64 = old
                    .iter()
                    .zip(new.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum();
                max_diff = max_diff.max(diff);
            }

            centroids = new_centroids;

            if max_diff < self.convergence_threshold {
                break;
            }
        }

        (assignments, centroids)
    }

    fn nearest_centroid(&self, point: &[f64], centroids: &[Vec<f64>]) -> usize {
        centroids
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let dist: f64 = point
                    .iter()
                    .zip(c.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum();
                (i, dist)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    fn calculate_inertia(
        &self,
        data: &[Vec<f64>],
        assignments: &[usize],
        centroids: &[Vec<f64>],
    ) -> f64 {
        data.iter()
            .zip(assignments.iter())
            .map(|(point, &cluster)| {
                if cluster < centroids.len() {
                    point
                        .iter()
                        .zip(centroids[cluster].iter())
                        .map(|(a, b)| (a - b).powi(2))
                        .sum::<f64>()
                } else {
                    0.0
                }
            })
            .sum()
    }

    fn build_clusters(
        &self,
        features: &[ErrorFeatures],
        assignments: &[usize],
        centroids: &[Vec<f64>],
    ) -> Vec<ErrorCluster> {
        let mut clusters: HashMap<usize, Vec<&ErrorFeatures>> = HashMap::new();

        for (i, feat) in features.iter().enumerate() {
            if i < assignments.len() {
                clusters.entry(assignments[i]).or_default().push(feat);
            }
        }

        clusters
            .into_iter()
            .map(|(id, members)| {
                let member_codes: Vec<String> =
                    members.iter().map(|f| f.code.clone()).collect();

                // Find dominant error code (most frequent)
                let dominant = members
                    .iter()
                    .max_by_key(|f| f.count)
                    .map(|f| f.code.clone())
                    .unwrap_or_default();

                // Calculate variance
                let centroid = if id < centroids.len() {
                    centroids[id].clone()
                } else {
                    vec![]
                };

                let variance = if !centroid.is_empty() {
                    members
                        .iter()
                        .map(|f| {
                            let vec = f.to_vector();
                            vec.iter()
                                .zip(centroid.iter())
                                .map(|(a, b)| (a - b).powi(2))
                                .sum::<f64>()
                        })
                        .sum::<f64>()
                        / members.len() as f64
                } else {
                    0.0
                };

                // Generate label
                let label = self.generate_cluster_label(&dominant, &members);

                ErrorCluster {
                    id,
                    centroid,
                    members: member_codes,
                    dominant_code: dominant,
                    size: members.len(),
                    variance,
                    label,
                }
            })
            .collect()
    }

    fn generate_cluster_label(&self, dominant: &str, members: &[&ErrorFeatures]) -> String {
        let type_count = members.iter().filter(|f| f.is_type_error).count();
        let resolution_count = members.iter().filter(|f| f.is_resolution_error).count();
        let borrow_count = members.iter().filter(|f| f.is_borrow_error).count();

        if type_count > members.len() / 2 {
            "Type System Errors".to_string()
        } else if resolution_count > members.len() / 2 {
            "Name Resolution Errors".to_string()
        } else if borrow_count > members.len() / 2 {
            "Ownership Errors".to_string()
        } else {
            format!("Mixed Errors (dominant: {})", dominant)
        }
    }

    fn calculate_silhouette(&self, data: &[Vec<f64>], assignments: &[usize]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mut total_silhouette = 0.0;

        for (i, point) in data.iter().enumerate() {
            let cluster = assignments[i];

            // a(i) = average distance to points in same cluster
            let same_cluster: Vec<_> = data
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i && assignments[*j] == cluster)
                .map(|(_, p)| p)
                .collect();

            let a = if same_cluster.is_empty() {
                0.0
            } else {
                same_cluster
                    .iter()
                    .map(|p| self.euclidean_distance(point, p))
                    .sum::<f64>()
                    / same_cluster.len() as f64
            };

            // b(i) = minimum average distance to points in other clusters
            let mut min_b = f64::MAX;
            let max_cluster = *assignments.iter().max().unwrap_or(&0);

            for other_cluster in 0..=max_cluster {
                if other_cluster == cluster {
                    continue;
                }

                let other_points: Vec<_> = data
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| assignments[*j] == other_cluster)
                    .map(|(_, p)| p)
                    .collect();

                if !other_points.is_empty() {
                    let avg_dist = other_points
                        .iter()
                        .map(|p| self.euclidean_distance(point, p))
                        .sum::<f64>()
                        / other_points.len() as f64;
                    min_b = min_b.min(avg_dist);
                }
            }

            let s = if min_b == f64::MAX || a.max(min_b) == 0.0 {
                0.0
            } else {
                (min_b - a) / a.max(min_b)
            };

            total_silhouette += s;
        }

        total_silhouette / data.len() as f64
    }

    fn euclidean_distance(&self, a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_features_to_vector() {
        let features = ErrorFeatures::from_error("E0308", 10, 100);
        let vec = features.to_vector();

        assert_eq!(vec.len(), 6);
        assert_eq!(vec[0], 10.0); // count
        assert_eq!(vec[1], 0.0); // category_idx for E0308
        assert_eq!(vec[2], 1.0); // is_type_error
    }

    #[test]
    fn test_cluster_analyzer_empty() {
        let analyzer = ClusterAnalyzer::new();
        let result = analyzer.analyze(&HashMap::new());

        assert!(result.clusters.is_empty());
        assert_eq!(result.total_error_types, 0);
    }

    #[test]
    fn test_cluster_analyzer_single_error() {
        let analyzer = ClusterAnalyzer::new();
        let mut errors = HashMap::new();
        errors.insert("E0308".to_string(), 5);

        let result = analyzer.analyze(&errors);

        assert_eq!(result.total_error_types, 1);
        assert_eq!(result.k, 1);
    }

    #[test]
    fn test_cluster_analyzer_multiple_errors() {
        let analyzer = ClusterAnalyzer::new();
        let mut errors = HashMap::new();
        errors.insert("E0308".to_string(), 50);
        errors.insert("E0412".to_string(), 30);
        errors.insert("E0425".to_string(), 40);
        errors.insert("E0277".to_string(), 10);
        errors.insert("E0502".to_string(), 5);

        let result = analyzer.analyze(&errors);

        assert_eq!(result.total_error_types, 5);
        assert!(!result.clusters.is_empty());
        assert!(result.silhouette_score >= -1.0 && result.silhouette_score <= 1.0);
    }

    #[test]
    fn test_error_cluster_label() {
        let analyzer = ClusterAnalyzer::new();
        let features = vec![
            ErrorFeatures::from_error("E0308", 10, 100),
            ErrorFeatures::from_error("E0307", 5, 100),
        ];
        let refs: Vec<&ErrorFeatures> = features.iter().collect();

        let label = analyzer.generate_cluster_label("E0308", &refs);
        assert!(label.contains("Type"));
    }

    #[test]
    fn test_kmeans_basic() {
        let analyzer = ClusterAnalyzer::new();
        let data = vec![
            vec![1.0, 1.0],
            vec![1.1, 1.1],
            vec![5.0, 5.0],
            vec![5.1, 5.1],
        ];

        let (assignments, _centroids) = analyzer.kmeans(&data, 2);

        // Points should be grouped: [0,1] in one cluster, [2,3] in another
        assert_eq!(assignments[0], assignments[1]);
        assert_eq!(assignments[2], assignments[3]);
        assert_ne!(assignments[0], assignments[2]);
    }

    #[test]
    fn test_silhouette_score_bounds() {
        let analyzer = ClusterAnalyzer::new();
        let data = vec![vec![1.0, 1.0], vec![5.0, 5.0], vec![9.0, 9.0]];
        let assignments = vec![0, 1, 2];

        let score = analyzer.calculate_silhouette(&data, &assignments);

        // Silhouette should be between -1 and 1
        assert!(score >= -1.0 && score <= 1.0);
    }

    #[test]
    fn test_euclidean_distance() {
        let analyzer = ClusterAnalyzer::new();
        let dist = analyzer.euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_with_max_k() {
        let analyzer = ClusterAnalyzer::new().with_max_k(10);
        assert_eq!(analyzer.max_k, 10);
    }
}

//! Error clustering by root cause
//!
//! Groups related errors together to identify the most impactful fix targets.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::classifier::ErrorClassification;
use super::compiler::CompilationError;

/// Root cause of a cluster of errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RootCause {
    /// Gap in transpiler implementation
    TranspilerGap {
        /// Type of gap (missing_method, type_inference, etc.)
        gap_type: String,
        /// Location in transpiler code
        location: String,
    },
    /// Unknown root cause
    Unknown,
}

impl RootCause {
    /// Create root cause from error code and message
    pub fn from_error_code(code: &str, message: &str) -> Self {
        match code {
            "E0599" => {
                // Extract method name from message
                let gap_type = if message.contains("method") {
                    "missing_method"
                } else {
                    "missing_field"
                };
                RootCause::TranspilerGap {
                    gap_type: gap_type.to_string(),
                    location: "expr_gen.rs".to_string(),
                }
            }
            "E0308" => RootCause::TranspilerGap {
                gap_type: "type_inference".to_string(),
                location: "type_mapper.rs".to_string(),
            },
            "E0277" => RootCause::TranspilerGap {
                gap_type: "missing_trait".to_string(),
                location: "type_mapper.rs".to_string(),
            },
            "E0425" => RootCause::TranspilerGap {
                gap_type: "undefined_variable".to_string(),
                location: "func_gen.rs".to_string(),
            },
            "E0433" => RootCause::TranspilerGap {
                gap_type: "missing_import".to_string(),
                location: "stmt_gen.rs".to_string(),
            },
            "E0382" | "E0502" | "E0507" => RootCause::TranspilerGap {
                gap_type: "borrow_checker".to_string(),
                location: "expr_gen.rs".to_string(),
            },
            _ => RootCause::Unknown,
        }
    }
}

/// Suggested fix for a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFix {
    /// Description of the fix
    pub description: String,
    /// File to modify
    pub file: PathBuf,
    /// Confidence in the fix
    pub confidence: f64,
}

impl SuggestedFix {
    /// Apply the suggested fix
    pub fn apply(&self) -> anyhow::Result<super::state::AppliedFix> {
        // Stub implementation - will be implemented in GREEN phase
        Ok(super::state::AppliedFix {
            iteration: 0,
            error_code: String::new(),
            description: self.description.clone(),
            file_modified: self.file.clone(),
            commit_hash: None,
            verified: false,
        })
    }
}

/// Cluster of related errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCluster {
    /// Root cause of the cluster
    pub root_cause: RootCause,
    /// Error code (E0599, E0308, etc.)
    pub error_code: String,
    /// Examples blocked by this error
    pub examples_blocked: Vec<PathBuf>,
    /// Sample errors from this cluster
    pub sample_errors: Vec<CompilationError>,
    /// Confidence in fix (0.0-1.0)
    pub fix_confidence: f64,
    /// Suggested fix (if available)
    pub suggested_fix: Option<SuggestedFix>,
}

impl ErrorCluster {
    /// Calculate impact score: number of examples * confidence
    pub fn impact_score(&self) -> f64 {
        self.examples_blocked.len() as f64 * self.fix_confidence
    }
}

/// Clusters errors by root cause
pub struct ErrorClusterer {
    // Configuration for clustering
}

impl ErrorClusterer {
    /// Create a new error clusterer
    pub fn new() -> Self {
        Self {}
    }

    /// Cluster errors by error code and subcategory
    pub fn cluster(&self, classifications: &[ErrorClassification]) -> Vec<ErrorCluster> {
        // Group by error code
        let mut groups: HashMap<String, Vec<&ErrorClassification>> = HashMap::new();
        for c in classifications {
            groups
                .entry(c.error.code.clone())
                .or_default()
                .push(c);
        }

        // Convert to clusters
        groups
            .into_iter()
            .map(|(error_code, errors)| {
                let sample_errors: Vec<CompilationError> =
                    errors.iter().map(|e| e.error.clone()).collect();

                let examples_blocked: Vec<PathBuf> =
                    errors.iter().map(|e| e.error.file.clone()).collect();

                let avg_confidence =
                    errors.iter().map(|e| e.confidence).sum::<f64>() / errors.len() as f64;

                let root_cause = if let Some(first) = errors.first() {
                    RootCause::from_error_code(&error_code, &first.error.message)
                } else {
                    RootCause::Unknown
                };

                ErrorCluster {
                    root_cause,
                    error_code,
                    examples_blocked,
                    sample_errors,
                    fix_confidence: avg_confidence,
                    suggested_fix: None,
                }
            })
            .collect()
    }
}

impl Default for ErrorClusterer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converge::classifier::ErrorCategory;

    #[test]
    fn test_root_cause_from_e0599() {
        let root = RootCause::from_error_code("E0599", "no method named `foo`");
        assert!(matches!(root, RootCause::TranspilerGap { .. }));
    }

    #[test]
    fn test_cluster_groups_by_error_code() {
        let clusterer = ErrorClusterer::new();
        let classifications = vec![
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "method not found".to_string(),
                    file: PathBuf::from("a.rs"),
                    line: 1,
                    column: 1,
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.9,
            },
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "method not found".to_string(),
                    file: PathBuf::from("b.rs"),
                    line: 2,
                    column: 2,
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.85,
            },
        ];

        let clusters = clusterer.cluster(&classifications);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].sample_errors.len(), 2);
    }

    #[test]
    fn test_impact_score() {
        let cluster = ErrorCluster {
            root_cause: RootCause::Unknown,
            error_code: "E0599".to_string(),
            examples_blocked: vec![
                PathBuf::from("a.py"),
                PathBuf::from("b.py"),
                PathBuf::from("c.py"),
            ],
            sample_errors: vec![],
            fix_confidence: 0.9,
            suggested_fix: None,
        };

        let expected = 3.0 * 0.9;
        assert!((cluster.impact_score() - expected).abs() < 0.01);
    }
}

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
            groups.entry(c.error.code.clone()).or_default().push(c);
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
    fn test_root_cause_from_e0599_field() {
        let root = RootCause::from_error_code("E0599", "no field named `bar`");
        if let RootCause::TranspilerGap { gap_type, .. } = root {
            assert_eq!(gap_type, "missing_field");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0308() {
        let root = RootCause::from_error_code("E0308", "mismatched types");
        if let RootCause::TranspilerGap { gap_type, location } = root {
            assert_eq!(gap_type, "type_inference");
            assert_eq!(location, "type_mapper.rs");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0277() {
        let root = RootCause::from_error_code("E0277", "trait bound not satisfied");
        if let RootCause::TranspilerGap { gap_type, .. } = root {
            assert_eq!(gap_type, "missing_trait");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0425() {
        let root = RootCause::from_error_code("E0425", "cannot find value");
        if let RootCause::TranspilerGap { gap_type, location } = root {
            assert_eq!(gap_type, "undefined_variable");
            assert_eq!(location, "func_gen.rs");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0433() {
        let root = RootCause::from_error_code("E0433", "failed to resolve");
        if let RootCause::TranspilerGap { gap_type, location } = root {
            assert_eq!(gap_type, "missing_import");
            assert_eq!(location, "stmt_gen.rs");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0382() {
        let root = RootCause::from_error_code("E0382", "use of moved value");
        if let RootCause::TranspilerGap { gap_type, .. } = root {
            assert_eq!(gap_type, "borrow_checker");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0502() {
        let root = RootCause::from_error_code("E0502", "cannot borrow");
        if let RootCause::TranspilerGap { gap_type, .. } = root {
            assert_eq!(gap_type, "borrow_checker");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_from_e0507() {
        let root = RootCause::from_error_code("E0507", "cannot move out of");
        if let RootCause::TranspilerGap { gap_type, .. } = root {
            assert_eq!(gap_type, "borrow_checker");
        } else {
            panic!("Expected TranspilerGap");
        }
    }

    #[test]
    fn test_root_cause_unknown() {
        let root = RootCause::from_error_code("E9999", "unknown error");
        assert!(matches!(root, RootCause::Unknown));
    }

    #[test]
    fn test_root_cause_debug() {
        let root = RootCause::TranspilerGap {
            gap_type: "test".to_string(),
            location: "test.rs".to_string(),
        };
        let debug = format!("{:?}", root);
        assert!(debug.contains("TranspilerGap"));
        assert!(debug.contains("test"));
    }

    #[test]
    fn test_root_cause_clone() {
        let root = RootCause::TranspilerGap {
            gap_type: "type_inference".to_string(),
            location: "type_mapper.rs".to_string(),
        };
        let cloned = root.clone();
        assert!(matches!(cloned, RootCause::TranspilerGap { .. }));
    }

    #[test]
    fn test_suggested_fix_struct() {
        let fix = SuggestedFix {
            description: "Add missing method".to_string(),
            file: PathBuf::from("expr_gen.rs"),
            confidence: 0.95,
        };
        assert_eq!(fix.description, "Add missing method");
        assert_eq!(fix.file, PathBuf::from("expr_gen.rs"));
        assert!((fix.confidence - 0.95).abs() < 0.001);
    }

    #[test]
    fn test_suggested_fix_clone() {
        let fix = SuggestedFix {
            description: "Fix type".to_string(),
            file: PathBuf::from("type_mapper.rs"),
            confidence: 0.8,
        };
        let cloned = fix.clone();
        assert_eq!(fix.description, cloned.description);
        assert_eq!(fix.file, cloned.file);
    }

    #[test]
    fn test_suggested_fix_apply() {
        let fix = SuggestedFix {
            description: "Test fix".to_string(),
            file: PathBuf::from("test.rs"),
            confidence: 0.9,
        };
        let result = fix.apply();
        assert!(result.is_ok());
        let applied = result.unwrap();
        assert_eq!(applied.description, "Test fix");
        assert!(!applied.verified);
    }

    #[test]
    fn test_error_cluster_struct() {
        let cluster = ErrorCluster {
            root_cause: RootCause::Unknown,
            error_code: "E0308".to_string(),
            examples_blocked: vec![PathBuf::from("a.py")],
            sample_errors: vec![],
            fix_confidence: 0.75,
            suggested_fix: None,
        };
        assert_eq!(cluster.error_code, "E0308");
        assert_eq!(cluster.examples_blocked.len(), 1);
    }

    #[test]
    fn test_error_cluster_clone() {
        let cluster = ErrorCluster {
            root_cause: RootCause::Unknown,
            error_code: "E0599".to_string(),
            examples_blocked: vec![PathBuf::from("a.py"), PathBuf::from("b.py")],
            sample_errors: vec![],
            fix_confidence: 0.85,
            suggested_fix: None,
        };
        let cloned = cluster.clone();
        assert_eq!(cluster.error_code, cloned.error_code);
        assert_eq!(
            cluster.examples_blocked.len(),
            cloned.examples_blocked.len()
        );
    }

    #[test]
    fn test_error_cluster_with_fix() {
        let cluster = ErrorCluster {
            root_cause: RootCause::TranspilerGap {
                gap_type: "missing_method".to_string(),
                location: "expr_gen.rs".to_string(),
            },
            error_code: "E0599".to_string(),
            examples_blocked: vec![PathBuf::from("test.py")],
            sample_errors: vec![],
            fix_confidence: 0.95,
            suggested_fix: Some(SuggestedFix {
                description: "Add method".to_string(),
                file: PathBuf::from("expr_gen.rs"),
                confidence: 0.95,
            }),
        };
        assert!(cluster.suggested_fix.is_some());
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
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.9,
                suggested_fix: None,
            },
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "method not found".to_string(),
                    file: PathBuf::from("b.rs"),
                    line: 2,
                    column: 2,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.85,
                suggested_fix: None,
            },
        ];

        let clusters = clusterer.cluster(&classifications);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].sample_errors.len(), 2);
    }

    #[test]
    fn test_cluster_multiple_error_codes() {
        let clusterer = ErrorClusterer::new();
        let classifications = vec![
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "method not found".to_string(),
                    file: PathBuf::from("a.rs"),
                    line: 1,
                    column: 1,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "missing_method".to_string(),
                confidence: 0.9,
                suggested_fix: None,
            },
            ErrorClassification {
                error: CompilationError {
                    code: "E0308".to_string(),
                    message: "mismatched types".to_string(),
                    file: PathBuf::from("b.rs"),
                    line: 2,
                    column: 2,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "type_inference".to_string(),
                confidence: 0.85,
                suggested_fix: None,
            },
        ];

        let clusters = clusterer.cluster(&classifications);
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test_cluster_empty() {
        let clusterer = ErrorClusterer::new();
        let classifications: Vec<ErrorClassification> = vec![];
        let clusters = clusterer.cluster(&classifications);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_clusterer_default() {
        let clusterer = ErrorClusterer::default();
        let clusters = clusterer.cluster(&[]);
        assert!(clusters.is_empty());
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

    #[test]
    fn test_impact_score_zero_confidence() {
        let cluster = ErrorCluster {
            root_cause: RootCause::Unknown,
            error_code: "E0599".to_string(),
            examples_blocked: vec![PathBuf::from("a.py")],
            sample_errors: vec![],
            fix_confidence: 0.0,
            suggested_fix: None,
        };
        assert!((cluster.impact_score() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_impact_score_empty_examples() {
        let cluster = ErrorCluster {
            root_cause: RootCause::Unknown,
            error_code: "E0599".to_string(),
            examples_blocked: vec![],
            sample_errors: vec![],
            fix_confidence: 1.0,
            suggested_fix: None,
        };
        assert!((cluster.impact_score() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_cluster_average_confidence() {
        let clusterer = ErrorClusterer::new();
        let classifications = vec![
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "test".to_string(),
                    file: PathBuf::from("a.rs"),
                    line: 1,
                    column: 1,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "test".to_string(),
                confidence: 0.8,
                suggested_fix: None,
            },
            ErrorClassification {
                error: CompilationError {
                    code: "E0599".to_string(),
                    message: "test".to_string(),
                    file: PathBuf::from("b.rs"),
                    line: 1,
                    column: 1,
                    ..Default::default()
                },
                category: ErrorCategory::TranspilerGap,
                subcategory: "test".to_string(),
                confidence: 1.0,
                suggested_fix: None,
            },
        ];

        let clusters = clusterer.cluster(&classifications);
        assert_eq!(clusters.len(), 1);
        // Average of 0.8 and 1.0 is 0.9
        assert!((clusters[0].fix_confidence - 0.9).abs() < 0.01);
    }
}

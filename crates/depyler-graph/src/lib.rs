//! DEPYLER-1300: Dependency Graph Analysis for Error Reasoning
//!
//! This crate transforms the transpiler from a "compiler" into a "reasoning engine"
//! by building dependency graphs that reveal error topology and enable ML-based fixes.
//!
//! # Architecture
//!
//! ```text
//! Python Source
//!     │
//!     ▼
//! ┌─────────────────┐
//! │  AST Parser     │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐     ┌─────────────────┐
//! │ Dependency      │────►│ Error Overlay   │
//! │ Graph Builder   │     │ (Rust Errors)   │
//! └────────┬────────┘     └────────┬────────┘
//!          │                       │
//!          ▼                       ▼
//! ┌─────────────────┐     ┌─────────────────┐
//! │ Impact Scorer   │◄────│ Vectorized      │
//! │ (PageRank)      │     │ Failures (JSON) │
//! └────────┬────────┘     └─────────────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Patient Zero    │
//! │ Identification  │
//! └─────────────────┘
//! ```

mod builder;
mod error_overlay;
mod impact;
mod vectorize;

pub use builder::{DependencyGraph, EdgeKind, GraphBuilder, NodeKind};
pub use error_overlay::{ErrorOverlay, OverlaidError};
pub use impact::{ImpactScore, ImpactScorer, PatientZero};
pub use vectorize::{
    serialize_to_json, serialize_to_ndjson, vectorize_failures, AstContext, FailureContext,
    FailureLabels, GraphContext, VectorizedFailure,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// A node in the dependency graph representing a Python entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique identifier
    pub id: String,
    /// Node kind (function, class, module)
    pub kind: NodeKind,
    /// Source file
    pub file: PathBuf,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Number of errors rooted at this node
    pub error_count: usize,
    /// Impact score (PageRank-style)
    pub impact_score: f64,
}

/// An edge in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Edge kind (calls, imports, inherits)
    pub kind: EdgeKind,
    /// Weight for impact calculation
    pub weight: f64,
}

/// Complete analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphAnalysis {
    /// The dependency graph
    pub node_count: usize,
    /// Edge count
    pub edge_count: usize,
    /// Patient Zero nodes (highest impact)
    pub patient_zeros: Vec<PatientZero>,
    /// Vectorized failures for ML training
    pub vectorized_failures: Vec<VectorizedFailure>,
    /// Error distribution by node
    pub error_distribution: HashMap<String, usize>,
    /// Total errors analyzed
    pub total_errors: usize,
}

/// Main entry point for graph-based error analysis
pub fn analyze_with_graph(
    python_source: &str,
    rust_errors: &[(String, String, usize)], // (code, message, line)
) -> Result<GraphAnalysis, GraphError> {
    // Build dependency graph from Python source
    let mut builder = GraphBuilder::new();
    let graph = builder.build_from_source(python_source)?;

    // Overlay errors onto graph
    let overlay = ErrorOverlay::new(&graph);
    let overlaid_errors = overlay.overlay_errors(rust_errors);

    // Calculate impact scores
    let scorer = ImpactScorer::new(&graph, &overlaid_errors);
    let scores = scorer.calculate_impact();

    // Identify patient zeros
    let patient_zeros = scorer.identify_patient_zeros(&scores, 5);

    // Vectorize failures for ML
    let vectorized = vectorize::vectorize_failures(&graph, &overlaid_errors, python_source);

    // Build error distribution
    let mut error_distribution = HashMap::new();
    for error in &overlaid_errors {
        if let Some(node_id) = &error.node_id {
            *error_distribution.entry(node_id.clone()).or_insert(0) += 1;
        }
    }

    Ok(GraphAnalysis {
        node_count: graph.node_count(),
        edge_count: graph.edge_count(),
        patient_zeros,
        vectorized_failures: vectorized,
        error_distribution,
        total_errors: rust_errors.len(),
    })
}

/// Errors that can occur during graph analysis
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Failed to parse Python source: {0}")]
    ParseError(String),

    #[error("Graph construction failed: {0}")]
    BuildError(String),

    #[error("Error overlay failed: {0}")]
    OverlayError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_simple_function() {
        let python = r#"
def foo():
    return 42

def bar():
    return foo() + 1
"#;

        let errors = vec![("E0308".to_string(), "mismatched types".to_string(), 5)];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(analysis.node_count >= 2); // foo and bar
        assert!(analysis.edge_count >= 1); // bar calls foo
    }

    #[test]
    fn test_analyze_class_hierarchy() {
        let python = r#"
class Base:
    def method(self):
        pass

class Derived(Base):
    def method(self):
        super().method()
"#;

        let errors = vec![("E0599".to_string(), "no method found".to_string(), 7)];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(analysis.node_count >= 2); // Base and Derived
    }

    #[test]
    fn test_patient_zero_identification() {
        let python = r#"
def problematic():
    return "bug"

def caller1():
    return problematic()

def caller2():
    return problematic()

def caller3():
    return problematic()
"#;

        // Errors in all callers point back to problematic()
        let errors = vec![
            ("E0308".to_string(), "type mismatch".to_string(), 5),
            ("E0308".to_string(), "type mismatch".to_string(), 8),
            ("E0308".to_string(), "type mismatch".to_string(), 11),
        ];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        // problematic should have highest impact
        if !analysis.patient_zeros.is_empty() {
            assert!(analysis.patient_zeros[0].impact_score > 0.0);
        }
    }

    #[test]
    fn test_vectorized_failure_output() {
        let python = r#"
def foo(x: int) -> str:
    return x  # E0308: expected str, found int
"#;

        let errors = vec![(
            "E0308".to_string(),
            "expected str, found int".to_string(),
            3,
        )];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(!analysis.vectorized_failures.is_empty());

        // Verify vectorized failure has AST context
        let failure = &analysis.vectorized_failures[0];
        assert_eq!(failure.error_code, "E0308");
    }

    #[test]
    fn test_analyze_with_no_errors() {
        let python = "def foo():\n    return 42\n";
        let errors: Vec<(String, String, usize)> = vec![];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert_eq!(analysis.total_errors, 0);
        assert!(analysis.vectorized_failures.is_empty());
        assert!(analysis.error_distribution.is_empty());
        // Patient zeros may still be identified via pagerank even without errors
        // but none should have direct errors
        for pz in &analysis.patient_zeros {
            assert_eq!(pz.direct_errors, 0);
        }
    }

    #[test]
    fn test_analyze_invalid_python() {
        let python = "def broken(:\n";
        let errors: Vec<(String, String, usize)> = vec![];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_empty_source() {
        let python = "";
        let errors: Vec<(String, String, usize)> = vec![];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert_eq!(analysis.node_count, 0);
        assert_eq!(analysis.edge_count, 0);
    }

    #[test]
    fn test_analyze_error_distribution() {
        let python = r#"
def foo():
    return 42

def bar():
    return foo()
"#;
        let errors = vec![
            ("E0308".to_string(), "err1".to_string(), 10),
            ("E0599".to_string(), "err2".to_string(), 10),
        ];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert_eq!(analysis.total_errors, 2);
        // Errors should be distributed across nodes
        let total_dist: usize = analysis.error_distribution.values().sum();
        assert!(total_dist <= 2);
    }

    #[test]
    fn test_graph_error_display_parse() {
        let err = GraphError::ParseError("unexpected token".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("parse"));
        assert!(msg.contains("unexpected token"));
    }

    #[test]
    fn test_graph_error_display_build() {
        let err = GraphError::BuildError("node conflict".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("construction"));
        assert!(msg.contains("node conflict"));
    }

    #[test]
    fn test_graph_error_display_overlay() {
        let err = GraphError::OverlayError("mapping failed".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("overlay"));
        assert!(msg.contains("mapping failed"));
    }

    #[test]
    fn test_graph_node_serde_roundtrip() {
        let node = GraphNode {
            id: "my_func".to_string(),
            kind: NodeKind::Function,
            file: std::path::PathBuf::from("test.py"),
            line: 10,
            column: 4,
            error_count: 2,
            impact_score: 3.14,
        };

        let json = serde_json::to_string(&node).unwrap();
        let deserialized: GraphNode = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "my_func");
        assert_eq!(deserialized.kind, NodeKind::Function);
        assert_eq!(deserialized.line, 10);
        assert_eq!(deserialized.column, 4);
        assert_eq!(deserialized.error_count, 2);
    }

    #[test]
    fn test_graph_edge_serde_roundtrip() {
        let edge = GraphEdge {
            kind: EdgeKind::Calls,
            weight: 2.5,
        };

        let json = serde_json::to_string(&edge).unwrap();
        let deserialized: GraphEdge = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.kind, EdgeKind::Calls);
        assert!((deserialized.weight - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_graph_analysis_serde_roundtrip() {
        let analysis = GraphAnalysis {
            node_count: 5,
            edge_count: 3,
            patient_zeros: vec![],
            vectorized_failures: vec![],
            error_distribution: HashMap::new(),
            total_errors: 0,
        };

        let json = serde_json::to_string(&analysis).unwrap();
        let deserialized: GraphAnalysis = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.node_count, 5);
        assert_eq!(deserialized.edge_count, 3);
        assert_eq!(deserialized.total_errors, 0);
    }

    #[test]
    fn test_analyze_complex_program() {
        let python = r#"
import math

class Shape:
    def area(self):
        pass

class Circle(Shape):
    def area(self):
        return 3.14

def compute(shape):
    return shape.area()

def main():
    c = Circle()
    return compute(c)
"#;
        let errors = vec![
            ("E0599".to_string(), "no method area".to_string(), 30),
            ("E0308".to_string(), "type mismatch".to_string(), 50),
        ];

        let result = analyze_with_graph(python, &errors);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        // Should have import, classes, methods, and functions
        assert!(analysis.node_count >= 5);
        assert_eq!(analysis.total_errors, 2);
    }

    // ========================================================================
    // S9B7: Coverage tests for graph lib
    // ========================================================================

    #[test]
    fn test_s9b7_graph_node_debug_clone() {
        let node = GraphNode {
            id: "n".to_string(),
            kind: NodeKind::Function,
            file: std::path::PathBuf::from("test.py"),
            line: 1,
            column: 1,
            error_count: 0,
            impact_score: 0.0,
        };
        let debug = format!("{:?}", node);
        assert!(debug.contains("GraphNode"));
        let cloned = node.clone();
        assert_eq!(cloned.id, "n");
    }

    #[test]
    fn test_s9b7_graph_edge_debug_clone() {
        let edge = GraphEdge {
            kind: EdgeKind::Imports,
            weight: 1.0,
        };
        let debug = format!("{:?}", edge);
        assert!(debug.contains("GraphEdge"));
        let cloned = edge.clone();
        assert_eq!(cloned.kind, EdgeKind::Imports);
    }

    #[test]
    fn test_s9b7_graph_analysis_debug_clone() {
        let analysis = GraphAnalysis {
            node_count: 0,
            edge_count: 0,
            patient_zeros: vec![],
            vectorized_failures: vec![],
            error_distribution: HashMap::new(),
            total_errors: 0,
        };
        let debug = format!("{:?}", analysis);
        assert!(debug.contains("GraphAnalysis"));
        let cloned = analysis.clone();
        assert_eq!(cloned.node_count, 0);
    }

    #[test]
    fn test_s9b7_analyze_with_graph_multiple_errors_same_line() {
        let python = "def foo():\n    return 42\n";
        let errors = vec![
            ("E0308".to_string(), "e1".to_string(), 5),
            ("E0308".to_string(), "e2".to_string(), 5),
        ];
        let result = analyze_with_graph(python, &errors).unwrap();
        assert_eq!(result.total_errors, 2);
    }

    #[test]
    fn test_s9b7_graph_error_debug() {
        let e1 = GraphError::ParseError("bad".to_string());
        let debug1 = format!("{:?}", e1);
        assert!(debug1.contains("ParseError"));

        let e2 = GraphError::BuildError("err".to_string());
        let debug2 = format!("{:?}", e2);
        assert!(debug2.contains("BuildError"));

        let e3 = GraphError::OverlayError("fail".to_string());
        let debug3 = format!("{:?}", e3);
        assert!(debug3.contains("OverlayError"));
    }

    #[test]
    fn test_s9b7_analyze_with_graph_only_class() {
        let python = r#"
class Standalone:
    def method(self):
        pass
"#;
        let errors: Vec<(String, String, usize)> = vec![];
        let result = analyze_with_graph(python, &errors).unwrap();
        assert!(result.node_count >= 2);
        assert_eq!(result.total_errors, 0);
    }

    #[test]
    fn test_s9b7_error_distribution_aggregation() {
        let python = "def foo():\n    return 42\n";
        let errors = vec![
            ("E0308".to_string(), "e1".to_string(), 10),
            ("E0599".to_string(), "e2".to_string(), 10),
            ("E0425".to_string(), "e3".to_string(), 10),
        ];
        let result = analyze_with_graph(python, &errors).unwrap();
        assert_eq!(result.total_errors, 3);
        // All errors may map to the same node or not
        let total_dist: usize = result.error_distribution.values().sum();
        assert!(total_dist <= 3);
    }

    #[test]
    fn test_graph_error_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<GraphError>();
        assert_sync::<GraphError>();
    }

    // ========================================================================
    // S12: Deep coverage tests for graph lib
    // ========================================================================

    #[test]
    fn test_s12_analyze_only_imports() {
        let python = "import os\nimport sys\n";
        let errors: Vec<(String, String, usize)> = vec![];
        let result = analyze_with_graph(python, &errors).unwrap();
        assert_eq!(result.node_count, 2);
        assert_eq!(result.total_errors, 0);
    }

    #[test]
    fn test_s12_analyze_large_error_count() {
        let python = "def foo():\n    return 42\n";
        let errors: Vec<(String, String, usize)> = (0..50)
            .map(|i| ("E0308".to_string(), format!("error {i}"), i * 10))
            .collect();
        let result = analyze_with_graph(python, &errors).unwrap();
        assert_eq!(result.total_errors, 50);
    }

    #[test]
    fn test_s12_analyze_with_inheritance_chain() {
        let python = r#"
class A:
    def m(self):
        pass
class B(A):
    def m(self):
        pass
class C(B):
    def m(self):
        pass
"#;
        let errors = vec![("E0308".to_string(), "err".to_string(), 20)];
        let result = analyze_with_graph(python, &errors).unwrap();
        // 3 classes + 3 methods = 6 nodes minimum
        assert!(result.node_count >= 6);
    }

    #[test]
    fn test_s12_graph_analysis_full_serde() {
        let python = r#"
def a():
    return b()
def b():
    return 1
"#;
        let errors = vec![("E0308".to_string(), "mismatch".to_string(), 10)];
        let analysis = analyze_with_graph(python, &errors).unwrap();
        let json = serde_json::to_string(&analysis).unwrap();
        let back: GraphAnalysis = serde_json::from_str(&json).unwrap();
        assert_eq!(back.node_count, analysis.node_count);
        assert_eq!(back.total_errors, 1);
    }

    #[test]
    fn test_s12_error_distribution_multiple_nodes() {
        let python = r#"
def foo():
    return 1
def bar():
    return 2
"#;
        // Two errors: one near foo, one near bar
        let errors = vec![
            ("E0308".to_string(), "e1".to_string(), 10),  // py_line=1, near foo
            ("E0308".to_string(), "e2".to_string(), 50),  // py_line=5, near bar
        ];
        let result = analyze_with_graph(python, &errors).unwrap();
        assert_eq!(result.total_errors, 2);
        // At least some errors should be distributed
        let total_dist: usize = result.error_distribution.values().sum();
        assert!(total_dist > 0);
    }

    #[test]
    fn test_s12_graph_error_source_trait() {
        // GraphError implements std::error::Error (via thiserror)
        let err = GraphError::ParseError("test".to_string());
        let e: &dyn std::error::Error = &err;
        assert!(e.source().is_none()); // Simple string errors have no source
    }
}

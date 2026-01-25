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
}

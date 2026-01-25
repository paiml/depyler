//! Error Overlay Module
//!
//! Maps Rust compiler errors onto the Python dependency graph,
//! enabling identification of "Patient Zero" - the root cause functions.

use crate::builder::DependencyGraph;
use serde::{Deserialize, Serialize};

/// An error overlaid onto the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlaidError {
    /// Error code (e.g., E0308)
    pub code: String,
    /// Error message
    pub message: String,
    /// Line number in generated Rust
    pub rust_line: usize,
    /// Estimated Python line (may differ due to codegen)
    pub python_line_estimate: usize,
    /// Node ID this error is associated with (if found)
    pub node_id: Option<String>,
    /// Confidence in node association (0.0 - 1.0)
    pub association_confidence: f64,
    /// Upstream nodes that may be root cause
    pub upstream_suspects: Vec<String>,
}

/// Overlays errors onto a dependency graph
pub struct ErrorOverlay<'a> {
    graph: &'a DependencyGraph,
}

impl<'a> ErrorOverlay<'a> {
    /// Create a new error overlay
    pub fn new(graph: &'a DependencyGraph) -> Self {
        Self { graph }
    }

    /// Overlay errors onto the graph
    pub fn overlay_errors(
        &self,
        errors: &[(String, String, usize)], // (code, message, line)
    ) -> Vec<OverlaidError> {
        errors
            .iter()
            .map(|(code, message, line)| self.overlay_single_error(code, message, *line))
            .collect()
    }

    /// Overlay a single error onto the graph
    fn overlay_single_error(&self, code: &str, message: &str, rust_line: usize) -> OverlaidError {
        // Estimate Python line (rough heuristic: Rust line / 10 for generated code)
        // This is a simplification - real implementation would use source maps
        let python_line_estimate = self.estimate_python_line(rust_line);

        // Find the node this error is most likely associated with
        let (node_id, confidence) = self.find_associated_node(python_line_estimate);

        // Find upstream nodes that might be the root cause
        let upstream_suspects = if let Some(ref nid) = node_id {
            self.find_upstream_suspects(nid)
        } else {
            vec![]
        };

        OverlaidError {
            code: code.to_string(),
            message: message.to_string(),
            rust_line,
            python_line_estimate,
            node_id,
            association_confidence: confidence,
            upstream_suspects,
        }
    }

    /// Estimate Python line from Rust line
    fn estimate_python_line(&self, rust_line: usize) -> usize {
        // Generated Rust is typically more verbose
        // Rough heuristic: Python line ≈ Rust line / 10
        // Real implementation would use source maps
        (rust_line / 10).max(1)
    }

    /// Find the node most likely associated with an error
    fn find_associated_node(&self, python_line: usize) -> (Option<String>, f64) {
        let mut best_match: Option<(String, usize, f64)> = None;

        for node_id in self.graph.node_ids() {
            if let Some(node) = self.graph.get_node(&node_id) {
                let distance = python_line.abs_diff(node.line);

                // Calculate confidence based on distance
                let confidence = 1.0 / (1.0 + distance as f64 / 10.0);

                if best_match.is_none() || distance < best_match.as_ref().unwrap().1 {
                    best_match = Some((node_id.clone(), distance, confidence));
                }
            }
        }

        match best_match {
            Some((id, _, conf)) if conf > 0.3 => (Some(id), conf),
            _ => (None, 0.0),
        }
    }

    /// Find upstream nodes that might be root cause
    fn find_upstream_suspects(&self, node_id: &str) -> Vec<String> {
        let mut suspects = Vec::new();

        // Get all nodes that call this node (incoming edges)
        let incoming = self.graph.incoming_edges(node_id);
        for (caller, _) in incoming {
            suspects.push(caller.id.clone());
        }

        // Also check for inheritance
        if let Some(node) = self.graph.get_node(node_id) {
            if node.kind == crate::builder::NodeKind::Method {
                // Check if this method overrides a base class method
                if let Some(class_name) = node_id.split('.').next() {
                    let outgoing = self.graph.outgoing_edges(class_name);
                    for (base, edge) in outgoing {
                        if edge.kind == crate::builder::EdgeKind::Inherits {
                            suspects.push(base.id.clone());
                        }
                    }
                }
            }
        }

        suspects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::GraphBuilder;

    #[test]
    fn test_overlay_simple() {
        let python = r#"
def foo():
    return 42
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let errors = vec![("E0308".to_string(), "mismatched types".to_string(), 20)];

        let overlaid = overlay.overlay_errors(&errors);
        assert_eq!(overlaid.len(), 1);
        assert_eq!(overlaid[0].code, "E0308");
    }

    #[test]
    fn test_find_associated_node() {
        let python = r#"
def foo():
    return 42

def bar():
    return 100
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);

        // Error on line 2 should associate with foo (line 2)
        let (node, conf) = overlay.find_associated_node(2);
        assert!(node.is_some());
        assert!(conf > 0.5);
    }

    #[test]
    fn test_upstream_suspects() {
        let python = r#"
def problematic():
    return "bug"

def caller():
    return problematic()
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);

        // Error in problematic should have caller as upstream suspect
        let suspects = overlay.find_upstream_suspects("problematic");
        assert!(suspects.contains(&"caller".to_string()));
    }
}

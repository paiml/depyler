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

    #[test]
    fn test_overlay_errors_empty() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let overlaid = overlay.overlay_errors(&[]);
        assert!(overlaid.is_empty());
    }

    #[test]
    fn test_overlay_multiple_errors() {
        let python = "def foo():\n    pass\n\ndef bar():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let errors = vec![
            ("E0308".to_string(), "type mismatch".to_string(), 10),
            ("E0599".to_string(), "no method found".to_string(), 20),
            ("E0425".to_string(), "undefined value".to_string(), 30),
        ];
        let overlaid = overlay.overlay_errors(&errors);

        assert_eq!(overlaid.len(), 3);
        assert_eq!(overlaid[0].code, "E0308");
        assert_eq!(overlaid[1].code, "E0599");
        assert_eq!(overlaid[2].code, "E0425");
    }

    #[test]
    fn test_overlay_preserves_error_message() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let errors = vec![(
            "E0308".to_string(),
            "expected i32, found String".to_string(),
            5,
        )];
        let overlaid = overlay.overlay_errors(&errors);

        assert_eq!(overlaid[0].message, "expected i32, found String");
        assert_eq!(overlaid[0].rust_line, 5);
    }

    #[test]
    fn test_estimate_python_line_minimum_is_1() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        // Rust line 1 / 10 = 0, but max(1) = 1
        let estimated = overlay.estimate_python_line(1);
        assert_eq!(estimated, 1);
    }

    #[test]
    fn test_estimate_python_line_ratio() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        // Rust line 100 / 10 = 10
        let estimated = overlay.estimate_python_line(100);
        assert_eq!(estimated, 10);
    }

    #[test]
    fn test_find_associated_node_empty_graph() {
        let graph = DependencyGraph::new();
        let overlay = ErrorOverlay::new(&graph);

        let (node, conf) = overlay.find_associated_node(5);
        assert!(node.is_none());
        assert_eq!(conf, 0.0);
    }

    #[test]
    fn test_find_associated_node_closest_match() {
        let python = r#"
def first():
    pass

def second():
    pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        // Line 2 should match closest to first (line 2)
        let (node, conf) = overlay.find_associated_node(2);
        assert!(node.is_some());
        assert!(conf > 0.3);
    }

    #[test]
    fn test_upstream_suspects_no_callers() {
        let python = "def lonely():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let suspects = overlay.find_upstream_suspects("lonely");
        assert!(suspects.is_empty());
    }

    #[test]
    fn test_upstream_suspects_nonexistent_node() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let suspects = overlay.find_upstream_suspects("nonexistent");
        assert!(suspects.is_empty());
    }

    #[test]
    fn test_upstream_suspects_method_inheritance() {
        let python = r#"
class Base:
    def method(self):
        pass

class Derived(Base):
    def method(self):
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        // Derived.method is a Method node, so inheritance suspects should include Base
        let suspects = overlay.find_upstream_suspects("Derived.method");
        assert!(suspects.contains(&"Base".to_string()));
    }

    #[test]
    fn test_overlaid_error_serde_roundtrip() {
        let error = OverlaidError {
            code: "E0308".to_string(),
            message: "type mismatch".to_string(),
            rust_line: 42,
            python_line_estimate: 5,
            node_id: Some("foo".to_string()),
            association_confidence: 0.85,
            upstream_suspects: vec!["bar".to_string()],
        };

        let json = serde_json::to_string(&error).unwrap();
        let deserialized: OverlaidError = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.code, "E0308");
        assert_eq!(deserialized.rust_line, 42);
        assert_eq!(deserialized.node_id, Some("foo".to_string()));
        assert_eq!(deserialized.upstream_suspects.len(), 1);
    }

    // ========================================================================
    // S9B7: Coverage tests for error_overlay
    // ========================================================================

    #[test]
    fn test_s9b7_overlay_single_error_with_no_node_match() {
        let graph = DependencyGraph::new();
        let overlay = ErrorOverlay::new(&graph);
        let errors = vec![("E0308".to_string(), "msg".to_string(), 100)];
        let overlaid = overlay.overlay_errors(&errors);
        assert_eq!(overlaid.len(), 1);
        assert!(overlaid[0].node_id.is_none());
        assert_eq!(overlaid[0].association_confidence, 0.0);
        assert!(overlaid[0].upstream_suspects.is_empty());
    }

    #[test]
    fn test_s9b7_estimate_python_line_at_zero() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);
        // Rust line 0 / 10 = 0, max(1) = 1
        assert_eq!(overlay.estimate_python_line(0), 1);
    }

    #[test]
    fn test_s9b7_estimate_python_line_large_value() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);
        assert_eq!(overlay.estimate_python_line(1000), 100);
    }

    #[test]
    fn test_s9b7_upstream_suspects_for_function_node() {
        let python = r#"
def dep():
    return 1

def caller():
    dep()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);
        // dep is called by caller, so upstream suspects for dep include caller
        let suspects = overlay.find_upstream_suspects("dep");
        assert!(suspects.contains(&"caller".to_string()));
    }

    #[test]
    fn test_s9b7_overlaid_error_fields() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);
        let errors = vec![("E0599".to_string(), "no method found".to_string(), 5)];
        let overlaid = overlay.overlay_errors(&errors);
        assert_eq!(overlaid[0].code, "E0599");
        assert_eq!(overlaid[0].message, "no method found");
        assert_eq!(overlaid[0].rust_line, 5);
        assert!(overlaid[0].python_line_estimate >= 1);
    }

    #[test]
    fn test_s9b7_overlaid_error_debug_clone() {
        let error = OverlaidError {
            code: "E0308".to_string(),
            message: "test".to_string(),
            rust_line: 10,
            python_line_estimate: 1,
            node_id: None,
            association_confidence: 0.0,
            upstream_suspects: vec![],
        };
        let debug = format!("{:?}", error);
        assert!(debug.contains("OverlaidError"));
        let cloned = error.clone();
        assert_eq!(cloned.code, "E0308");
    }

    #[test]
    fn test_association_confidence_decreases_with_distance() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);

        // Closer lines should have higher confidence
        let (_, conf_close) = overlay.find_associated_node(2);
        let (_, conf_far) = overlay.find_associated_node(50);

        // Near the node should be higher confidence
        assert!(conf_close >= conf_far);
    }

    // ========================================================================
    // S12: Deep coverage tests for error_overlay
    // ========================================================================

    #[test]
    fn test_s12_find_associated_node_confidence_boundary() {
        // Confidence = 1/(1 + dist/10). At dist=23, conf ~= 0.303 (just barely above 0.3)
        // At dist=24, conf ~= 0.294 (below 0.3)
        let python = "def foo():\n    pass\n"; // foo at line 1
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);

        // Very far away: confidence drops below 0.3
        let (node, conf) = overlay.find_associated_node(100);
        // At distance 99, conf = 1/(1 + 99/10) = 1/10.9 ≈ 0.092 < 0.3 -> None
        assert!(node.is_none(), "Expected None for far distance, got {:?}, conf={}", node, conf);
        assert_eq!(conf, 0.0);
    }

    #[test]
    fn test_s12_find_associated_node_equidistant() {
        // Two nodes at equal distance - should pick one consistently
        let python = r#"
def first():
    pass



def second():
    pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);

        // Line 4 is equidistant between first(line 2) and second(line 7)
        let (node, conf) = overlay.find_associated_node(4);
        assert!(node.is_some());
        assert!(conf > 0.3);
    }

    #[test]
    fn test_s12_upstream_suspects_with_inheritance() {
        let python = r#"
class Animal:
    def speak(self):
        pass

class Dog(Animal):
    def speak(self):
        pass

class Cat(Animal):
    def speak(self):
        pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);

        // Dog.speak is a Method node; Dog inherits from Animal
        let suspects = overlay.find_upstream_suspects("Dog.speak");
        assert!(suspects.contains(&"Animal".to_string()));
    }

    #[test]
    fn test_s12_upstream_suspects_function_not_method() {
        // Functions (not methods) should not check inheritance
        let python = r#"
def standalone():
    return 1
def caller():
    return standalone()
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);

        let suspects = overlay.find_upstream_suspects("standalone");
        // Should have caller as suspect, but no inheritance check
        assert!(suspects.contains(&"caller".to_string()));
    }

    #[test]
    fn test_s12_overlay_preserves_line_estimate() {
        let python = "def foo():\n    pass\n";
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);

        // Various Rust lines -> Python line estimates
        let errors = vec![
            ("E0308".to_string(), "e".to_string(), 10),
            ("E0308".to_string(), "e".to_string(), 100),
            ("E0308".to_string(), "e".to_string(), 0),
        ];
        let overlaid = overlay.overlay_errors(&errors);
        assert_eq!(overlaid[0].python_line_estimate, 1); // 10/10 = 1
        assert_eq!(overlaid[1].python_line_estimate, 10); // 100/10 = 10
        assert_eq!(overlaid[2].python_line_estimate, 1); // 0/10 = 0 -> max(1) = 1
    }

    #[test]
    fn test_s12_overlay_multiple_nodes_closest() {
        let python = r#"
def alpha():
    pass

def beta():
    pass

def gamma():
    pass
"#;
        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();
        let overlay = ErrorOverlay::new(&graph);

        // Error near beta (line ~5)
        let errors = vec![("E0308".to_string(), "e".to_string(), 50)]; // py_line = 5
        let overlaid = overlay.overlay_errors(&errors);
        // Should associate with the closest node
        assert!(overlaid[0].node_id.is_some());
    }
}

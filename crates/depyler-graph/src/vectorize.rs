//! Vectorization Module
//!
//! Serializes AST context of errors into structured format for ML training.
//! Creates the dataset required for "Given this AST context, predict the fix".

use crate::builder::DependencyGraph;
use crate::error_overlay::OverlaidError;
use serde::{Deserialize, Serialize};

/// Vectorized failure ready for ML training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizedFailure {
    /// Unique identifier for this failure
    pub id: String,
    /// Error code (e.g., E0308)
    pub error_code: String,
    /// Error message
    pub error_message: String,
    /// AST context around the error
    pub ast_context: AstContext,
    /// Graph context (node relationships)
    pub graph_context: GraphContext,
    /// Python source snippet
    pub source_snippet: String,
    /// Labels for supervised learning
    pub labels: FailureLabels,
}

/// AST context around an error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstContext {
    /// Function/method name containing the error
    pub containing_function: Option<String>,
    /// Class name if in a method
    pub containing_class: Option<String>,
    /// Return type annotation (if present)
    pub return_type: Option<String>,
    /// Parameter types (if annotated)
    pub parameter_types: Vec<String>,
    /// Local variable types inferred
    pub local_types: Vec<(String, String)>,
    /// Statement kind (return, assign, call, etc.)
    pub statement_kind: String,
    /// Expression kind (call, binop, name, etc.)
    pub expression_kind: String,
    /// Depth in AST (0 = top level)
    pub ast_depth: usize,
}

/// Graph context for an error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphContext {
    /// Node ID in the dependency graph
    pub node_id: Option<String>,
    /// Number of callers (in-degree)
    pub in_degree: usize,
    /// Number of callees (out-degree)
    pub out_degree: usize,
    /// Names of functions called
    pub callees: Vec<String>,
    /// Names of callers
    pub callers: Vec<String>,
    /// Inheritance chain (for methods)
    pub inheritance_chain: Vec<String>,
}

/// Labels for supervised ML training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureLabels {
    /// Category of the error
    pub category: String,
    /// Sub-category (e.g., "double_result_wrap")
    pub subcategory: String,
    /// Suggested fix type
    pub fix_type: String,
    /// Confidence in the categorization
    pub confidence: f64,
}

/// Context for failure extraction
pub struct FailureContext<'a> {
    #[allow(dead_code)]
    graph: &'a DependencyGraph,
    source: &'a str,
}

impl<'a> FailureContext<'a> {
    /// Create a new failure context
    pub fn new(graph: &'a DependencyGraph, source: &'a str) -> Self {
        Self { graph, source }
    }

    /// Extract source snippet around a line
    fn extract_snippet(&self, line: usize, context_lines: usize) -> String {
        let lines: Vec<&str> = self.source.lines().collect();
        if lines.is_empty() {
            return String::new();
        }
        // Bound line to valid range (line numbers are 1-indexed from rustc)
        let bounded_line = line.min(lines.len()).max(1);
        let start = bounded_line.saturating_sub(context_lines + 1);
        let end = (bounded_line + context_lines).min(lines.len());

        lines[start..end].join("\n")
    }

    /// Classify error into category and subcategory
    fn classify_error(&self, code: &str, message: &str) -> (String, String, String, f64) {
        // E0308 sub-patterns
        if code == "E0308" {
            if message.contains("expected") && message.contains("Result") {
                return (
                    "type_mismatch".to_string(),
                    "double_result_wrap".to_string(),
                    "unwrap_result".to_string(),
                    0.9,
                );
            }
            if message.contains("DepylerValue") {
                return (
                    "type_mismatch".to_string(),
                    "depyler_value_leak".to_string(),
                    "type_annotation".to_string(),
                    0.85,
                );
            }
            if message.contains("&str") && message.contains("String") {
                return (
                    "type_mismatch".to_string(),
                    "string_ref_mismatch".to_string(),
                    "to_string".to_string(),
                    0.9,
                );
            }
            if message.contains("i32") || message.contains("i64") || message.contains("f64") {
                return (
                    "type_mismatch".to_string(),
                    "numeric_type_mismatch".to_string(),
                    "cast".to_string(),
                    0.8,
                );
            }
            return (
                "type_mismatch".to_string(),
                "general".to_string(),
                "type_inference".to_string(),
                0.6,
            );
        }

        // E0599: Missing method
        if code == "E0599" {
            return (
                "missing_method".to_string(),
                "stdlib_mapping".to_string(),
                "add_trait_impl".to_string(),
                0.8,
            );
        }

        // E0425: Undefined value
        if code == "E0425" {
            return (
                "undefined".to_string(),
                "missing_import".to_string(),
                "add_import".to_string(),
                0.7,
            );
        }

        // E0277: Trait bound
        if code == "E0277" {
            return (
                "trait_bound".to_string(),
                "missing_trait".to_string(),
                "derive_trait".to_string(),
                0.75,
            );
        }

        // Default
        (
            "unknown".to_string(),
            "unknown".to_string(),
            "manual_fix".to_string(),
            0.3,
        )
    }
}

/// Vectorize failures from overlaid errors
pub fn vectorize_failures(
    graph: &DependencyGraph,
    errors: &[OverlaidError],
    source: &str,
) -> Vec<VectorizedFailure> {
    let context = FailureContext::new(graph, source);

    errors
        .iter()
        .enumerate()
        .map(|(idx, error)| {
            let (category, subcategory, fix_type, confidence) =
                context.classify_error(&error.code, &error.message);

            // Build graph context
            let (in_degree, out_degree, callers, callees, inheritance) =
                if let Some(ref node_id) = error.node_id {
                    let incoming = graph.incoming_edges(node_id);
                    let outgoing = graph.outgoing_edges(node_id);
                    (
                        incoming.len(),
                        outgoing.len(),
                        incoming.iter().map(|(n, _)| n.id.clone()).collect(),
                        outgoing.iter().map(|(n, _)| n.id.clone()).collect(),
                        vec![], // Would need more analysis for inheritance
                    )
                } else {
                    (0, 0, vec![], vec![], vec![])
                };

            VectorizedFailure {
                id: format!("failure_{}", idx),
                error_code: error.code.clone(),
                error_message: error.message.clone(),
                ast_context: AstContext {
                    containing_function: error.node_id.clone(),
                    containing_class: None, // Would extract from node_id if method
                    return_type: None,
                    parameter_types: vec![],
                    local_types: vec![],
                    statement_kind: "unknown".to_string(),
                    expression_kind: "unknown".to_string(),
                    ast_depth: 0,
                },
                graph_context: GraphContext {
                    node_id: error.node_id.clone(),
                    in_degree,
                    out_degree,
                    callees,
                    callers,
                    inheritance_chain: inheritance,
                },
                source_snippet: context.extract_snippet(error.python_line_estimate, 3),
                labels: FailureLabels {
                    category,
                    subcategory,
                    fix_type,
                    confidence,
                },
            }
        })
        .collect()
}

/// Serialize failures to JSON for ML training
pub fn serialize_to_json(failures: &[VectorizedFailure]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(failures)
}

/// Serialize failures to NDJSON (newline-delimited JSON) for streaming
pub fn serialize_to_ndjson(failures: &[VectorizedFailure]) -> Result<String, serde_json::Error> {
    let lines: Result<Vec<String>, _> = failures
        .iter()
        .map(serde_json::to_string)
        .collect();
    Ok(lines?.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::GraphBuilder;
    use crate::error_overlay::ErrorOverlay;

    #[test]
    fn test_vectorize_simple() {
        let python = r#"
def foo():
    return 42
"#;

        let mut builder = GraphBuilder::new();
        let graph = builder.build_from_source(python).unwrap();

        let overlay = ErrorOverlay::new(&graph);
        let raw_errors = vec![
            ("E0308".to_string(), "mismatched types".to_string(), 20),
        ];
        let overlaid = overlay.overlay_errors(&raw_errors);

        let vectorized = vectorize_failures(&graph, &overlaid, python);

        assert_eq!(vectorized.len(), 1);
        assert_eq!(vectorized[0].error_code, "E0308");
        assert_eq!(vectorized[0].labels.category, "type_mismatch");
    }

    #[test]
    fn test_classify_double_result() {
        let context = FailureContext {
            graph: &DependencyGraph::new(),
            source: "",
        };

        let (cat, sub, fix, conf) =
            context.classify_error("E0308", "expected Vec, found Result<Vec>");

        assert_eq!(cat, "type_mismatch");
        assert_eq!(sub, "double_result_wrap");
        assert_eq!(fix, "unwrap_result");
        assert!(conf > 0.8);
    }

    #[test]
    fn test_classify_depyler_value() {
        let context = FailureContext {
            graph: &DependencyGraph::new(),
            source: "",
        };

        let (cat, sub, _, _) =
            context.classify_error("E0308", "expected f64, found DepylerValue");

        assert_eq!(cat, "type_mismatch");
        assert_eq!(sub, "depyler_value_leak");
    }

    #[test]
    fn test_serialize_to_json() {
        let failures = vec![VectorizedFailure {
            id: "test".to_string(),
            error_code: "E0308".to_string(),
            error_message: "test".to_string(),
            ast_context: AstContext {
                containing_function: Some("foo".to_string()),
                containing_class: None,
                return_type: None,
                parameter_types: vec![],
                local_types: vec![],
                statement_kind: "return".to_string(),
                expression_kind: "call".to_string(),
                ast_depth: 1,
            },
            graph_context: GraphContext {
                node_id: Some("foo".to_string()),
                in_degree: 1,
                out_degree: 0,
                callees: vec![],
                callers: vec!["bar".to_string()],
                inheritance_chain: vec![],
            },
            source_snippet: "return 42".to_string(),
            labels: FailureLabels {
                category: "type_mismatch".to_string(),
                subcategory: "general".to_string(),
                fix_type: "type_inference".to_string(),
                confidence: 0.8,
            },
        }];

        let json = serialize_to_json(&failures).unwrap();
        assert!(json.contains("E0308"));
        assert!(json.contains("type_mismatch"));
    }

    #[test]
    fn test_serialize_to_ndjson() {
        let failures = vec![
            VectorizedFailure {
                id: "f1".to_string(),
                error_code: "E0308".to_string(),
                error_message: "test1".to_string(),
                ast_context: AstContext {
                    containing_function: None,
                    containing_class: None,
                    return_type: None,
                    parameter_types: vec![],
                    local_types: vec![],
                    statement_kind: "".to_string(),
                    expression_kind: "".to_string(),
                    ast_depth: 0,
                },
                graph_context: GraphContext {
                    node_id: None,
                    in_degree: 0,
                    out_degree: 0,
                    callees: vec![],
                    callers: vec![],
                    inheritance_chain: vec![],
                },
                source_snippet: "".to_string(),
                labels: FailureLabels {
                    category: "".to_string(),
                    subcategory: "".to_string(),
                    fix_type: "".to_string(),
                    confidence: 0.0,
                },
            },
        ];

        let ndjson = serialize_to_ndjson(&failures).unwrap();
        // NDJSON should have one line per record
        assert_eq!(ndjson.lines().count(), 1);
    }
}

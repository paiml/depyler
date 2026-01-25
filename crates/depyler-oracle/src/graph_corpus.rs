//! DEPYLER-1303: Graph-aware corpus integration for Oracle training.
//!
//! Converts VectorizedFailure from depyler-graph into Oracle training samples,
//! enabling graph-guided fix suggestions.

use crate::classifier::ErrorCategory;
use crate::training::TrainingSample;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Graph context for an error (from depyler-graph)
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

/// Labels for supervised ML training (from depyler-graph)
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

/// Vectorized failure from depyler-graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizedFailure {
    /// Unique identifier for this failure
    pub id: String,
    /// Error code (e.g., E0308)
    pub error_code: String,
    /// Error message
    pub error_message: String,
    /// Graph context (node relationships)
    pub graph_context: GraphContext,
    /// Python source snippet
    pub source_snippet: String,
    /// Labels for supervised learning
    pub labels: FailureLabels,
}

/// Convert depyler-graph category string to Oracle ErrorCategory
fn map_category(category: &str, subcategory: &str, error_code: &str) -> ErrorCategory {
    match (category, subcategory, error_code) {
        ("type_mismatch", _, _) | (_, _, "E0308") => ErrorCategory::TypeMismatch,
        ("trait_bound", _, _) | (_, _, "E0277") => ErrorCategory::TraitBound,
        ("undefined", "missing_import", _) | (_, _, "E0433") | (_, _, "E0425") => {
            ErrorCategory::MissingImport
        }
        ("missing_method", _, _) | (_, _, "E0599") => ErrorCategory::TraitBound,
        (_, _, code) if code.starts_with("E0") => {
            // Map common error codes
            match code {
                "E0382" | "E0502" | "E0503" | "E0505" | "E0507" => ErrorCategory::BorrowChecker,
                "E0106" | "E0495" | "E0621" | "E0700" | "E0759" => ErrorCategory::LifetimeError,
                "E0061" | "E0070" | "E0220" | "E0282" => ErrorCategory::TypeMismatch,
                "E0531" | "E0658" | "E0679" => ErrorCategory::SyntaxError,
                _ => ErrorCategory::Other,
            }
        }
        _ => ErrorCategory::Other,
    }
}

/// Generate fix suggestion based on graph context and error type
fn generate_fix_suggestion(failure: &VectorizedFailure) -> Option<String> {
    let fix_type = &failure.labels.fix_type;
    let node_id = failure
        .graph_context
        .node_id
        .as_deref()
        .unwrap_or("unknown");

    match fix_type.as_str() {
        "unwrap_result" => Some(format!(
            "In {}: Remove double Result wrapping - use ? operator directly",
            node_id
        )),
        "type_annotation" => Some(format!(
            "In {}: Add explicit type annotation to resolve DepylerValue",
            node_id
        )),
        "to_string" => Some(format!(
            "In {}: Convert &str to String using .to_string()",
            node_id
        )),
        "cast" => Some(format!(
            "In {}: Add numeric type cast (as i64, as f64)",
            node_id
        )),
        "add_trait_impl" => Some(format!(
            "In {}: Implement required trait or use trait object",
            node_id
        )),
        "add_import" => Some(format!(
            "In {}: Add missing use statement or import",
            node_id
        )),
        "derive_trait" => Some(format!(
            "In {}: Add #[derive(Clone, Debug)] or implement trait",
            node_id
        )),
        "type_inference" => {
            // More specific fix based on error code
            match failure.error_code.as_str() {
                "E0308" => Some(format!(
                    "In {}: Type mismatch - check return type or add explicit conversion",
                    node_id
                )),
                "E0282" => Some(format!(
                    "In {}: Cannot infer type - add explicit type annotation",
                    node_id
                )),
                _ => Some(format!("In {}: Review type inference logic", node_id)),
            }
        }
        _ => None,
    }
}

/// Build enhanced error message with graph context
fn build_graph_enhanced_message(failure: &VectorizedFailure) -> String {
    let base_msg = &failure.error_message;
    let ctx = &failure.graph_context;

    // Add graph context to message for better pattern matching
    let mut enhanced = format!("[{}] {}", failure.error_code, base_msg);

    if let Some(ref node_id) = ctx.node_id {
        enhanced.push_str(&format!(" [in: {}]", node_id));
    }

    if ctx.in_degree > 0 {
        enhanced.push_str(&format!(" [callers: {}]", ctx.in_degree));
    }

    if ctx.out_degree > 0 {
        enhanced.push_str(&format!(" [callees: {}]", ctx.out_degree));
    }

    enhanced
}

/// Load vectorized failures from NDJSON file
pub fn load_vectorized_failures(path: &Path) -> anyhow::Result<Vec<VectorizedFailure>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut failures = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<VectorizedFailure>(&line) {
            Ok(failure) => failures.push(failure),
            Err(e) => {
                eprintln!("Warning: Failed to parse vectorized failure: {}", e);
            }
        }
    }

    Ok(failures)
}

/// Convert vectorized failures to Oracle training samples
pub fn convert_to_training_samples(failures: &[VectorizedFailure]) -> Vec<TrainingSample> {
    failures
        .iter()
        .map(|failure| {
            let category = map_category(
                &failure.labels.category,
                &failure.labels.subcategory,
                &failure.error_code,
            );

            let message = build_graph_enhanced_message(failure);
            let fix = generate_fix_suggestion(failure);

            match fix {
                Some(f) => TrainingSample::with_fix(&message, category, &f),
                None => TrainingSample::new(&message, category),
            }
        })
        .collect()
}

/// Build graph corpus from NDJSON file
pub fn build_graph_corpus(path: &Path) -> anyhow::Result<Vec<TrainingSample>> {
    let failures = load_vectorized_failures(path)?;
    Ok(convert_to_training_samples(&failures))
}

/// Statistics about the graph corpus
#[derive(Debug, Default)]
pub struct GraphCorpusStats {
    pub total_samples: usize,
    pub by_category: std::collections::HashMap<String, usize>,
    pub by_error_code: std::collections::HashMap<String, usize>,
    pub with_graph_context: usize,
    pub with_fix_suggestions: usize,
}

/// Analyze graph corpus and return statistics
pub fn analyze_graph_corpus(failures: &[VectorizedFailure]) -> GraphCorpusStats {
    let mut stats = GraphCorpusStats {
        total_samples: failures.len(),
        ..Default::default()
    };

    for failure in failures {
        // Count by category
        *stats
            .by_category
            .entry(failure.labels.category.clone())
            .or_insert(0) += 1;

        // Count by error code
        *stats
            .by_error_code
            .entry(failure.error_code.clone())
            .or_insert(0) += 1;

        // Count with graph context
        if failure.graph_context.node_id.is_some() {
            stats.with_graph_context += 1;
        }

        // Count with fix suggestions
        if generate_fix_suggestion(failure).is_some() {
            stats.with_fix_suggestions += 1;
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_failure(code: &str, category: &str, subcategory: &str) -> VectorizedFailure {
        VectorizedFailure {
            id: "test".to_string(),
            error_code: code.to_string(),
            error_message: "test error".to_string(),
            graph_context: GraphContext {
                node_id: Some("test_func".to_string()),
                in_degree: 2,
                out_degree: 1,
                callees: vec!["helper".to_string()],
                callers: vec!["main".to_string(), "other".to_string()],
                inheritance_chain: vec![],
            },
            source_snippet: "x = foo()".to_string(),
            labels: FailureLabels {
                category: category.to_string(),
                subcategory: subcategory.to_string(),
                fix_type: "type_inference".to_string(),
                confidence: 0.8,
            },
        }
    }

    #[test]
    fn test_map_category_type_mismatch() {
        let cat = map_category("type_mismatch", "general", "E0308");
        assert_eq!(cat, ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_map_category_trait_bound() {
        let cat = map_category("trait_bound", "missing_trait", "E0277");
        assert_eq!(cat, ErrorCategory::TraitBound);
    }

    #[test]
    fn test_map_category_borrow_checker() {
        let cat = map_category("unknown", "unknown", "E0382");
        assert_eq!(cat, ErrorCategory::BorrowChecker);
    }

    #[test]
    fn test_convert_to_training_samples() {
        let failures = vec![make_test_failure("E0308", "type_mismatch", "general")];

        let samples = convert_to_training_samples(&failures);

        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].category, ErrorCategory::TypeMismatch);
        assert!(samples[0].message.contains("[E0308]"));
        assert!(samples[0].message.contains("[in: test_func]"));
    }

    #[test]
    fn test_generate_fix_suggestion() {
        let mut failure = make_test_failure("E0308", "type_mismatch", "double_result_wrap");
        failure.labels.fix_type = "unwrap_result".to_string();

        let fix = generate_fix_suggestion(&failure);

        assert!(fix.is_some());
        assert!(fix.unwrap().contains("Remove double Result wrapping"));
    }

    #[test]
    fn test_build_graph_enhanced_message() {
        let failure = make_test_failure("E0308", "type_mismatch", "general");

        let msg = build_graph_enhanced_message(&failure);

        assert!(msg.contains("[E0308]"));
        assert!(msg.contains("[in: test_func]"));
        assert!(msg.contains("[callers: 2]"));
        assert!(msg.contains("[callees: 1]"));
    }

    #[test]
    fn test_analyze_graph_corpus() {
        let failures = vec![
            make_test_failure("E0308", "type_mismatch", "general"),
            make_test_failure("E0277", "trait_bound", "missing_trait"),
        ];

        let stats = analyze_graph_corpus(&failures);

        assert_eq!(stats.total_samples, 2);
        assert_eq!(stats.by_category.len(), 2);
        assert_eq!(stats.with_graph_context, 2);
    }
}

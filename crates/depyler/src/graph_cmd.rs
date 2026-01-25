//! DEPYLER-1303: Graph Analysis Commands
//!
//! CLI interface for the Graph Engine to identify Patient Zeros
//! and generate vectorized failures for ML training.

use anyhow::Result;
use depyler_core::DepylerPipeline;
use depyler_graph::{
    analyze_with_graph, serialize_to_json, serialize_to_ndjson, GraphBuilder, ImpactScorer,
    PatientZero,
};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Result of corpus analysis
#[derive(Debug, serde::Serialize)]
pub struct CorpusAnalysis {
    /// Number of files analyzed
    pub files_analyzed: usize,
    /// Number of files with errors
    pub files_with_errors: usize,
    /// Total errors found
    pub total_errors: usize,
    /// Top Patient Zeros
    pub patient_zeros: Vec<PatientZeroSummary>,
    /// Error distribution by code
    pub error_distribution: std::collections::HashMap<String, usize>,
}

/// Summary of a Patient Zero for JSON output
#[derive(Debug, Clone, serde::Serialize)]
pub struct PatientZeroSummary {
    /// Node identifier
    pub node_id: String,
    /// Impact score (0.0-1.0)
    pub impact_score: f64,
    /// Number of direct errors
    pub direct_errors: usize,
    /// Number of downstream nodes affected
    pub downstream_affected: usize,
    /// Fix priority (1 = highest)
    pub fix_priority: usize,
    /// Estimated impact if fixed
    pub estimated_fix_impact: usize,
}

impl From<&PatientZero> for PatientZeroSummary {
    fn from(pz: &PatientZero) -> Self {
        Self {
            node_id: pz.node_id.clone(),
            impact_score: pz.impact_score,
            direct_errors: pz.direct_errors,
            downstream_affected: pz.downstream_affected,
            fix_priority: pz.fix_priority,
            estimated_fix_impact: pz.estimated_fix_impact,
        }
    }
}

/// Transpile a single file with panic isolation
fn transpile_isolated(python_source: &str) -> Option<String> {
    // Set a silent panic hook temporarily
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // Silent panic handler

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(python_source).ok()
    }));

    // Restore previous panic hook
    std::panic::set_hook(prev_hook);

    match result {
        Ok(Some(code)) => Some(code),
        _ => None,
    }
}

/// Analyze a corpus and identify Patient Zeros
pub fn analyze_corpus(corpus_dir: &Path, top_n: usize, output: Option<&Path>) -> Result<()> {
    let mut all_errors: Vec<(String, String, usize)> = Vec::new();
    let mut all_python_sources: Vec<(PathBuf, String)> = Vec::new();
    let mut error_distribution: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut files_analyzed = 0;
    let mut files_with_errors = 0;
    let mut files_panicked = 0;

    println!("Analyzing corpus: {}", corpus_dir.display());

    // Find all Python files
    for entry in WalkDir::new(corpus_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "py")
                && !e.path().to_string_lossy().contains("__pycache__")
        })
    {
        let path = entry.path();
        files_analyzed += 1;

        // Read Python source
        let python_source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Transpile to Rust (in isolated thread to catch panics)
        let rust_code = match transpile_isolated(&python_source) {
            Some(code) => code,
            None => {
                files_panicked += 1;
                continue;
            }
        };

        // Try to compile the Rust code
        let errors = check_rust_compilation(&rust_code);
        if !errors.is_empty() {
            files_with_errors += 1;
            for (code, msg, line) in &errors {
                *error_distribution.entry(code.clone()).or_insert(0) += 1;
                all_errors.push((code.clone(), msg.clone(), *line));
            }
            all_python_sources.push((path.to_path_buf(), python_source));
        }
    }

    if files_panicked > 0 {
        println!("Warning: {} files caused transpiler panics", files_panicked);
    }

    println!(
        "Analyzed {} files, {} with errors ({} total errors)",
        files_analyzed,
        files_with_errors,
        all_errors.len()
    );

    // Build combined graph from all sources
    let mut combined_graph = depyler_graph::DependencyGraph::new();
    for (path, source) in &all_python_sources {
        let mut builder = GraphBuilder::new();
        if let Ok(graph) = builder.build_from_source(source) {
            // Merge nodes and edges (simplified - just add the graphs)
            for node_id in graph.node_ids() {
                if let Some(node) = graph.get_node(&node_id) {
                    // Prefix node ID with file path for uniqueness
                    let prefixed_id = format!(
                        "{}::{}",
                        path.file_stem()
                            .unwrap_or_default()
                            .to_string_lossy(),
                        node_id
                    );
                    let mut prefixed_node = node.clone();
                    prefixed_node.id = prefixed_id;
                    combined_graph.add_node(prefixed_node);
                }
            }
        }
    }

    // Calculate impact scores
    let error_overlay = depyler_graph::ErrorOverlay::new(&combined_graph);
    let overlaid_errors = error_overlay.overlay_errors(&all_errors);
    let scorer = ImpactScorer::new(&combined_graph, &overlaid_errors);
    let scores = scorer.calculate_impact();
    let patient_zeros = scorer.identify_patient_zeros(&scores, top_n);

    // Build summary
    let analysis = CorpusAnalysis {
        files_analyzed,
        files_with_errors,
        total_errors: all_errors.len(),
        patient_zeros: patient_zeros.iter().map(PatientZeroSummary::from).collect(),
        error_distribution,
    };

    // Output
    let json = serde_json::to_string_pretty(&analysis)?;
    if let Some(output_path) = output {
        fs::write(output_path, &json)?;
        println!("Analysis written to: {}", output_path.display());
    } else {
        println!("{}", json);
    }

    // Print Patient Zeros summary
    if !analysis.patient_zeros.is_empty() {
        println!("\nTop {} Patient Zeros:", top_n.min(patient_zeros.len()));
        println!("{:-<60}", "");
        for (i, pz) in analysis.patient_zeros.iter().enumerate() {
            println!(
                "{}. {} (impact: {:.3}, direct: {}, downstream: {}, priority: {})",
                i + 1,
                pz.node_id,
                pz.impact_score,
                pz.direct_errors,
                pz.downstream_affected,
                pz.fix_priority
            );
        }
    }

    Ok(())
}

/// Vectorize failures from a corpus for ML training
pub fn vectorize_corpus(
    corpus_dir: &Path,
    output: &Path,
    format: &str, // "json" or "ndjson"
) -> Result<()> {
    let mut all_vectorized = Vec::new();
    let mut files_panicked = 0;
    let mut files_processed = 0;

    eprintln!("Vectorizing failures from: {}", corpus_dir.display());

    // Find all Python files
    for entry in WalkDir::new(corpus_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "py")
                && !e.path().to_string_lossy().contains("__pycache__")
        })
    {
        let path = entry.path();
        files_processed += 1;

        // Read Python source
        let python_source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Transpile to Rust (in isolated thread)
        let rust_code = match transpile_isolated(&python_source) {
            Some(code) => code,
            None => {
                files_panicked += 1;
                continue;
            }
        };

        // Get compilation errors (with panic isolation)
        let rust_code_clone = rust_code.clone();
        let errors = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            check_rust_compilation(&rust_code_clone)
        }))
        .unwrap_or_else(|_| vec![]);

        if errors.is_empty() {
            continue;
        }

        // Build graph and vectorize (with panic isolation)
        let python_source_clone = python_source.clone();
        let errors_clone = errors.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            analyze_with_graph(&python_source_clone, &errors_clone)
        }));

        match result {
            Ok(Ok(analysis)) => {
                all_vectorized.extend(analysis.vectorized_failures);
            }
            Ok(Err(_)) | Err(_) => {
                files_panicked += 1;
            }
        }
    }

    eprintln!(
        "Processed {} files ({} panicked)",
        files_processed, files_panicked
    );

    // Serialize output
    let output_str = match format {
        "ndjson" => serialize_to_ndjson(&all_vectorized)?,
        _ => serialize_to_json(&all_vectorized)?,
    };

    fs::write(output, &output_str)?;
    eprintln!(
        "Vectorized {} failures to: {}",
        all_vectorized.len(),
        output.display()
    );

    Ok(())
}

/// Check Rust code compilation and extract errors
fn check_rust_compilation(rust_code: &str) -> Vec<(String, String, usize)> {
    use std::process::Command;

    // Write to temp file in temp directory
    let temp_dir = match tempfile::tempdir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let temp_file = temp_dir.path().join("check.rs");
    let temp_output = temp_dir.path().join("check");

    if fs::write(&temp_file, rust_code).is_err() {
        return vec![];
    }

    // Run rustc --error-format=json (output to temp dir, not /dev/null)
    let output = Command::new("rustc")
        .args(["--error-format=json", "--crate-type=lib", "--emit=metadata"])
        .arg("-o")
        .arg(&temp_output)
        .arg(&temp_file)
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => return vec![],
    };

    // Parse JSON errors
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut errors = Vec::new();

    for line in stderr.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if json.get("level").and_then(|l| l.as_str()) == Some("error") {
                let code = json
                    .get("code")
                    .and_then(|c| c.get("code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("E0000")
                    .to_string();

                let message = json
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();

                let line_num = json
                    .get("spans")
                    .and_then(|s| s.as_array())
                    .and_then(|a| a.first())
                    .and_then(|s| s.get("line_start"))
                    .and_then(|l| l.as_u64())
                    .unwrap_or(1) as usize;

                if !code.is_empty() {
                    errors.push((code, message, line_num));
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_rust_compilation_valid() {
        let code = "fn main() {}";
        let errors = check_rust_compilation(code);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_rust_compilation_invalid() {
        let code = "fn main() { let x: i32 = \"not a number\"; }";
        let errors = check_rust_compilation(code);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|(code, _, _)| code == "E0308"));
    }

    #[test]
    fn test_patient_zero_summary_from() {
        let pz = PatientZero {
            node_id: "test_func".to_string(),
            impact_score: 0.85,
            direct_errors: 3,
            downstream_affected: 10,
            fix_priority: 1,
            estimated_fix_impact: 5,
        };
        let summary = PatientZeroSummary::from(&pz);
        assert_eq!(summary.node_id, "test_func");
        assert_eq!(summary.impact_score, 0.85);
    }
}

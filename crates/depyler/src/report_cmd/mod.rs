//! Corpus Analysis Report Command
//!
//! Minimal report implementation focused on testable pure functions.
//! GH-209: Extended with ML clustering and semantic domain analysis.

pub mod analysis;
pub mod clustering;      // GH-209 Phase 2: ML Clustering
pub mod graph_analysis;  // GH-209 Phase 4: Graph Analysis
pub mod filter;

use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Report command arguments
pub struct ReportArgs {
    pub corpus: Option<PathBuf>,
    pub format: String,
    pub output: Option<PathBuf>,
    pub skip_clean: bool,
    pub target_rate: f64,
    pub filter: Option<String>,
    pub tag: Option<String>,
    pub limit: Option<usize>,
    pub sample: Option<usize>,
    pub bisect: bool,
    pub fail_fast: bool,
}

/// Compilation result for a single file
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub name: String,
    pub success: bool,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Error taxonomy entry
#[derive(Debug, Default, Clone)]
pub struct ErrorTaxonomy {
    pub count: usize,
    pub samples: Vec<String>,
}

/// Handle the report command - simplified version
pub fn handle_report_command(args: ReportArgs) -> Result<()> {
    let corpus_path = args.corpus.unwrap_or_else(default_corpus_path);

    println!("Corpus Analysis Report");
    println!("======================");
    println!("Corpus: {}", corpus_path.display());
    println!("Target: {:.0}%", args.target_rate * 100.0);
    println!("Format: {}", args.format);

    // For now, just print status - actual compilation delegated to converge command
    println!("\nUse 'depyler converge' for full compilation analysis.");

    Ok(())
}

/// Find default corpus path
pub fn default_corpus_path() -> PathBuf {
    let candidates = [
        PathBuf::from("/home/noah/src/reprorusted-python-cli/examples"),
        PathBuf::from("../reprorusted-python-cli/examples"),
        PathBuf::from("./examples"),
    ];

    for path in &candidates {
        if path.exists() {
            return path.clone();
        }
    }

    PathBuf::from(".")
}

/// Extract error code and message from cargo output
pub fn extract_error(stderr: &str) -> (String, String) {
    let strip_ansi = |s: &str| -> String {
        let mut result = String::new();
        let mut in_escape = false;
        for c in s.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape {
                if c == 'm' {
                    in_escape = false;
                }
            } else {
                result.push(c);
            }
        }
        result
    };

    // Look for error[EXXXX] pattern
    for line in stderr.lines() {
        if let Some(start) = line.find("error[E") {
            if let Some(end) = line[start..].find(']') {
                let code = &line[start + 6..start + end];
                let msg = strip_ansi(line[start + end + 1..].trim());
                return (code.to_string(), msg);
            }
        }
    }

    // Check for transpiler errors
    for line in stderr.lines() {
        if line.starts_with("Error: Failed to transpile") {
            for cause_line in stderr.lines() {
                if cause_line.trim().starts_with("Expression type not yet supported") {
                    return ("TRANSPILE".to_string(), strip_ansi(cause_line.trim()));
                }
                if cause_line.trim().starts_with("Unsupported") {
                    return ("TRANSPILE".to_string(), strip_ansi(cause_line.trim()));
                }
            }
            return ("TRANSPILE".to_string(), strip_ansi(line));
        }
        if line.starts_with("Error:") {
            return ("DEPYLER".to_string(), strip_ansi(line));
        }
    }

    // Fallback
    for line in stderr.lines() {
        let lower = line.to_lowercase();
        if lower.starts_with("error") {
            return ("UNKNOWN".to_string(), strip_ansi(line));
        }
    }

    ("UNKNOWN".to_string(), "Unknown error".to_string())
}

/// Analyze compilation results
pub fn analyze_results(results: &[CompileResult]) -> (usize, usize, HashMap<String, ErrorTaxonomy>) {
    let mut pass = 0;
    let mut fail = 0;
    let mut taxonomy: HashMap<String, ErrorTaxonomy> = HashMap::new();

    for result in results {
        if result.success {
            pass += 1;
        } else {
            fail += 1;

            if let Some(code) = &result.error_code {
                let entry = taxonomy.entry(code.clone()).or_default();
                entry.count += 1;
                if entry.samples.len() < 3 {
                    let sample = format!(
                        "{}: {}",
                        result.name,
                        result.error_message.as_deref().unwrap_or("?")
                    );
                    entry.samples.push(sample);
                }
            }
        }
    }

    (pass, fail, taxonomy)
}

/// Get error description
pub fn error_description(code: &str) -> &'static str {
    match code {
        "E0425" => "Cannot find value in scope",
        "E0412" => "Cannot find type in scope",
        "E0308" => "Mismatched types",
        "E0277" => "Trait not implemented",
        "E0432" => "Unresolved import",
        "E0599" => "Method not found",
        "E0433" => "Failed to resolve path",
        "E0423" => "Expected value, found type",
        "E0369" => "Binary operation not supported",
        "E0255" => "Name already defined",
        "E0618" => "Expected function",
        "E0609" => "No field on type",
        "E0601" => "main function not found",
        "E0573" => "Expected type",
        "TRANSPILE" => "Transpiler limitation",
        "DEPYLER" => "General transpiler error",
        _ => "See rustc --explain",
    }
}

/// Get fix recommendation
pub fn fix_recommendation(code: &str) -> &'static str {
    match code {
        "E0425" => "Declare variables before use",
        "E0412" => "Add generic parameter detection",
        "E0308" => "Standardize numeric types",
        "E0277" => "Add missing trait implementations",
        "E0432" => "Fix import resolution",
        "E0599" => "Check method resolution",
        "E0433" => "Update module path resolution",
        "E0423" => "Fix value/type confusion",
        "E0369" => "Add operator overloading",
        "TRANSPILE" => "Add support in expr_gen.rs",
        "DEPYLER" => "Check error message",
        _ => "Investigate error pattern",
    }
}

/// Generate ASCII bar chart
pub fn ascii_bar(ratio: f64, width: usize) -> String {
    let filled = (ratio.clamp(0.0, 1.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled).green(), "░".repeat(empty).dimmed())
}

/// Print terminal report
pub fn print_terminal_report(
    total: usize,
    pass: usize,
    fail: usize,
    rate: f64,
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    _target: f64,
) {
    println!();
    println!("{}", "Summary".bold().underline());
    println!("  Total: {}", total);
    println!("  Pass:  {}", pass.to_string().green());
    println!("  Fail:  {}", fail.to_string().red());
    println!("  Rate:  {}", format!("{:.1}%", rate).cyan().bold());

    let andon = if rate >= 80.0 {
        "GREEN".green().bold()
    } else if rate >= 50.0 {
        "YELLOW".yellow().bold()
    } else {
        "RED".red().bold()
    };
    println!("  Status: {}", andon);

    if !taxonomy.is_empty() {
        println!();
        println!("{}", "Error Taxonomy".bold().underline());

        let mut sorted: Vec<_> = taxonomy.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        for (code, entry) in sorted.iter().take(10) {
            let desc = error_description(code);
            println!("  {} ({}) - {}", code.yellow(), entry.count, desc);
        }
    }
}

/// Print JSON report
pub fn print_json_report(
    total: usize,
    pass: usize,
    fail: usize,
    rate: f64,
    taxonomy: &HashMap<String, ErrorTaxonomy>,
    target: f64,
) -> Result<()> {
    let errors: Vec<_> = taxonomy
        .iter()
        .map(|(code, entry)| {
            serde_json::json!({
                "code": code,
                "count": entry.count,
                "description": error_description(code),
                "samples": entry.samples,
            })
        })
        .collect();

    let report = serde_json::json!({
        "summary": {
            "total": total,
            "pass": pass,
            "fail": fail,
            "rate": rate,
            "target": target * 100.0,
            "status": if rate >= 80.0 { "GREEN" } else if rate >= 50.0 { "YELLOW" } else { "RED" }
        },
        "errors": errors
    });

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_error_rust_error_code() {
        let stderr = "error[E0425]: cannot find value `x` in this scope";
        let (code, msg) = extract_error(stderr);
        assert_eq!(code, "E0425");
        assert!(msg.contains("cannot find value"));
    }

    #[test]
    fn test_extract_error_transpile() {
        let stderr = "Error: Failed to transpile\nCaused by:\n  Expression type not yet supported: Lambda";
        let (code, msg) = extract_error(stderr);
        assert_eq!(code, "TRANSPILE");
        assert!(msg.contains("Expression type not yet supported"));
    }

    #[test]
    fn test_extract_error_unsupported() {
        let stderr = "Error: Failed to transpile\nCaused by:\n  Unsupported syntax";
        let (code, msg) = extract_error(stderr);
        assert_eq!(code, "TRANSPILE");
        assert!(msg.contains("Unsupported"));
    }

    #[test]
    fn test_extract_error_depyler() {
        let stderr = "Error: Something went wrong";
        let (code, _msg) = extract_error(stderr);
        assert_eq!(code, "DEPYLER");
    }

    #[test]
    fn test_extract_error_unknown() {
        let stderr = "some random output";
        let (code, _) = extract_error(stderr);
        assert_eq!(code, "UNKNOWN");
    }

    #[test]
    fn test_extract_error_strips_ansi() {
        let stderr = "error[E0308]: \x1b[31mmismatched types\x1b[0m";
        let (code, msg) = extract_error(stderr);
        assert_eq!(code, "E0308");
        assert!(!msg.contains("\x1b"));
    }

    #[test]
    fn test_extract_error_empty() {
        let (code, _) = extract_error("");
        assert_eq!(code, "UNKNOWN");
    }

    #[test]
    fn test_extract_error_whitespace() {
        let (code, _) = extract_error("   \n\t  ");
        assert_eq!(code, "UNKNOWN");
    }

    #[test]
    fn test_analyze_results_all_pass() {
        let results = vec![
            CompileResult { name: "a".into(), success: true, error_code: None, error_message: None },
            CompileResult { name: "b".into(), success: true, error_code: None, error_message: None },
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 2);
        assert_eq!(fail, 0);
        assert!(taxonomy.is_empty());
    }

    #[test]
    fn test_analyze_results_with_failures() {
        let results = vec![
            CompileResult { name: "a".into(), success: true, error_code: None, error_message: None },
            CompileResult { name: "b".into(), success: false, error_code: Some("E0425".into()), error_message: Some("not found".into()) },
            CompileResult { name: "c".into(), success: false, error_code: Some("E0425".into()), error_message: Some("not found".into()) },
            CompileResult { name: "d".into(), success: false, error_code: Some("E0308".into()), error_message: Some("type mismatch".into()) },
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 1);
        assert_eq!(fail, 3);
        assert_eq!(taxonomy.get("E0425").unwrap().count, 2);
        assert_eq!(taxonomy.get("E0308").unwrap().count, 1);
    }

    #[test]
    fn test_analyze_results_samples_limited() {
        let results: Vec<_> = (0..10)
            .map(|i| CompileResult {
                name: format!("file{}", i),
                success: false,
                error_code: Some("E0425".into()),
                error_message: Some(format!("error {}", i)),
            })
            .collect();
        let (_, _, taxonomy) = analyze_results(&results);
        assert_eq!(taxonomy.get("E0425").unwrap().count, 10);
        assert_eq!(taxonomy.get("E0425").unwrap().samples.len(), 3);
    }

    #[test]
    fn test_analyze_results_empty() {
        let results: Vec<CompileResult> = vec![];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 0);
        assert_eq!(fail, 0);
        assert!(taxonomy.is_empty());
    }

    #[test]
    fn test_error_description_known_codes() {
        assert!(error_description("E0425").contains("find"));
        assert!(error_description("E0308").contains("type"));
        assert!(error_description("E0277").contains("Trait"));
        assert!(error_description("TRANSPILE").contains("Transpiler"));
    }

    #[test]
    fn test_error_description_unknown() {
        assert!(error_description("E9999").contains("rustc"));
    }

    #[test]
    fn test_fix_recommendation_known_codes() {
        assert!(!fix_recommendation("E0425").is_empty());
        assert!(!fix_recommendation("E0308").is_empty());
        assert!(!fix_recommendation("TRANSPILE").is_empty());
    }

    #[test]
    fn test_fix_recommendation_unknown() {
        let rec = fix_recommendation("UNKNOWN_CODE");
        assert!(!rec.is_empty());
    }

    #[test]
    fn test_ascii_bar_full() {
        let bar = ascii_bar(1.0, 10);
        assert!(bar.contains("█"));
    }

    #[test]
    fn test_ascii_bar_empty() {
        let bar = ascii_bar(0.0, 10);
        assert!(bar.contains("░"));
    }

    #[test]
    fn test_ascii_bar_half() {
        let bar = ascii_bar(0.5, 10);
        assert!(!bar.is_empty());
    }

    #[test]
    fn test_ascii_bar_clamps() {
        let bar1 = ascii_bar(-0.5, 10);
        let bar2 = ascii_bar(1.5, 10);
        assert!(!bar1.is_empty());
        assert!(!bar2.is_empty());
    }

    #[test]
    fn test_default_corpus_path() {
        let path = default_corpus_path();
        assert!(!path.as_os_str().is_empty());
    }

    #[test]
    fn test_report_args_defaults() {
        let args = ReportArgs {
            corpus: None,
            format: "terminal".into(),
            output: None,
            skip_clean: false,
            target_rate: 0.8,
            filter: None,
            tag: None,
            limit: None,
            sample: None,
            bisect: false,
            fail_fast: false,
        };
        assert_eq!(args.target_rate, 0.8);
        assert!(!args.bisect);
    }

    #[test]
    fn test_print_terminal_report() {
        let taxonomy = HashMap::new();
        print_terminal_report(10, 8, 2, 80.0, &taxonomy, 0.8);
    }

    #[test]
    fn test_print_terminal_report_with_errors() {
        let mut taxonomy = HashMap::new();
        taxonomy.insert("E0425".to_string(), ErrorTaxonomy {
            count: 5,
            samples: vec!["sample1: error".to_string()],
        });
        print_terminal_report(10, 2, 8, 20.0, &taxonomy, 0.8);
    }

    #[test]
    fn test_print_terminal_report_yellow_status() {
        let taxonomy = HashMap::new();
        print_terminal_report(100, 60, 40, 60.0, &taxonomy, 0.8);
    }

    #[test]
    fn test_print_terminal_report_green_status() {
        let taxonomy = HashMap::new();
        print_terminal_report(100, 85, 15, 85.0, &taxonomy, 0.8);
    }

    #[test]
    fn test_print_json_report() {
        let taxonomy = HashMap::new();
        let result = print_json_report(0, 0, 0, 0.0, &taxonomy, 0.8);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_report_with_data() {
        let mut taxonomy = HashMap::new();
        taxonomy.insert("E0425".to_string(), ErrorTaxonomy {
            count: 2,
            samples: vec!["sample: error".to_string()],
        });
        let result = print_json_report(2, 1, 1, 50.0, &taxonomy, 0.8);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_result_debug() {
        let result = CompileResult {
            name: "test".to_string(),
            success: true,
            error_code: None,
            error_message: None,
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_error_taxonomy_default() {
        let taxonomy = ErrorTaxonomy::default();
        assert_eq!(taxonomy.count, 0);
        assert!(taxonomy.samples.is_empty());
    }

    #[test]
    fn test_handle_report_command() {
        let args = ReportArgs {
            corpus: Some(PathBuf::from("/tmp")),
            format: "text".into(),
            output: None,
            skip_clean: false,
            target_rate: 0.8,
            filter: None,
            tag: None,
            limit: None,
            sample: None,
            bisect: false,
            fail_fast: false,
        };
        let result = handle_report_command(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_report_command_default_corpus() {
        let args = ReportArgs {
            corpus: None,
            format: "json".into(),
            output: None,
            skip_clean: true,
            target_rate: 0.9,
            filter: Some("test".into()),
            tag: Some("Dict".into()),
            limit: Some(10),
            sample: Some(5),
            bisect: false,
            fail_fast: true,
        };
        let result = handle_report_command(args);
        assert!(result.is_ok());
    }
}

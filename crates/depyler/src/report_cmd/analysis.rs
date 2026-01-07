//! Pure analysis functions for report command - extracted for EXTREME TDD
//!
//! This module contains pure, side-effect-free functions that can be
//! thoroughly tested with unit tests. The main report_cmd/mod.rs becomes
//! a thin shim that calls these functions.
//!
//! DEPYLER-COVERAGE-95: Extracted for testability

use std::collections::HashMap;

/// Compilation result for analysis (borrowed/owned view)
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub name: String,
    pub success: bool,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Error taxonomy entry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ErrorEntry {
    pub count: usize,
    pub samples: Vec<String>,
}

/// Extract error code and message from rustc/depyler stderr output
///
/// Returns (code, message) tuple
pub fn extract_error(stderr: &str) -> (String, String) {
    // Strip ANSI codes
    let clean = strip_ansi_codes(stderr);

    // Try to find rustc error code pattern: error[EXXXX]:
    if let Some(code) = extract_rust_error_code(&clean) {
        let message = extract_error_message(&clean);
        return (code, message);
    }

    // Check for transpiler errors
    if clean.contains("Failed to transpile") || clean.contains("Unsupported")
        || clean.contains("not yet supported")
    {
        let message = extract_transpile_message(&clean);
        return ("TRANSPILE".to_string(), message);
    }

    // Check for depyler errors
    if clean.contains("Error:") {
        return ("DEPYLER".to_string(), clean.lines().next().unwrap_or("").to_string());
    }

    // Unknown error
    ("UNKNOWN".to_string(), clean.lines().next().unwrap_or("").to_string())
}

/// Strip ANSI escape codes from string
pub fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip until 'm' (end of ANSI sequence)
            while let Some(&next) = chars.peek() {
                chars.next();
                if next == 'm' {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Extract rustc error code (e.g., E0308) from stderr
pub fn extract_rust_error_code(stderr: &str) -> Option<String> {
    // Pattern: error[E0XXX]:
    if let Some(start) = stderr.find("error[E") {
        let rest = &stderr[start + 6..]; // After "error["
        if let Some(end) = rest.find(']') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

/// Extract error message from rustc stderr
pub fn extract_error_message(stderr: &str) -> String {
    // Find the message after the error code
    if let Some(colon_pos) = stderr.find("]:") {
        let message = stderr[colon_pos + 2..].trim();
        // Get first line only
        message.lines().next().unwrap_or("").to_string()
    } else {
        stderr.lines().next().unwrap_or("").to_string()
    }
}

/// Extract message from transpiler error
pub fn extract_transpile_message(stderr: &str) -> String {
    // Look for "Caused by:" section
    if let Some(caused_by) = stderr.find("Caused by:") {
        let rest = &stderr[caused_by + 10..];
        rest.lines()
            .find(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string())
            .unwrap_or_else(|| "Unknown transpiler error".to_string())
    } else if let Some(unsupported) = stderr.find("Unsupported") {
        stderr[unsupported..]
            .lines()
            .next()
            .unwrap_or("Unsupported syntax")
            .to_string()
    } else if let Some(not_supported) = stderr.find("not yet supported") {
        stderr[not_supported.saturating_sub(30)..]
            .lines()
            .next()
            .unwrap_or("Feature not supported")
            .to_string()
    } else {
        "Transpilation failed".to_string()
    }
}

/// Analyze compilation results and build error taxonomy
///
/// Returns (pass_count, fail_count, error_taxonomy)
pub fn analyze_results(results: &[AnalysisResult]) -> (usize, usize, HashMap<String, ErrorEntry>) {
    let mut taxonomy: HashMap<String, ErrorEntry> = HashMap::new();
    let mut pass = 0;
    let mut fail = 0;

    for result in results {
        if result.success {
            pass += 1;
        } else {
            fail += 1;
            let code = result.error_code.clone().unwrap_or_else(|| "UNKNOWN".to_string());
            let entry = taxonomy.entry(code).or_default();
            entry.count += 1;

            // Keep up to 3 samples per error type
            if entry.samples.len() < 3 {
                if let Some(msg) = &result.error_message {
                    entry.samples.push(format!("{}: {}", result.name, msg));
                }
            }
        }
    }

    (pass, fail, taxonomy)
}

/// Get human-readable description for error code
pub fn error_description(code: &str) -> &'static str {
    match code {
        "E0425" => "Cannot find value in scope (undefined variable/function)",
        "E0412" => "Cannot find type in scope (missing generic/type)",
        "E0308" => "Mismatched types (type inference failure)",
        "E0277" => "Trait not implemented (missing impl)",
        "E0432" => "Unresolved import (missing crate/module)",
        "E0599" => "Method not found (wrong type/missing impl)",
        "E0433" => "Failed to resolve (unresolved module path)",
        "E0423" => "Expected value, found type (usage error)",
        "E0369" => "Binary operation not supported between types",
        "E0255" => "Name already defined in this scope",
        "E0618" => "Expected function, found different type",
        "E0609" => "No field on type (struct field access)",
        "E0601" => "main function not found",
        "E0573" => "Expected type, found something else",
        "E0382" => "Use of moved value (ownership error)",
        "E0507" => "Cannot move out of borrowed content",
        "E0502" => "Cannot borrow as mutable (already borrowed)",
        "E0499" => "Cannot borrow as mutable more than once",
        "E0515" => "Cannot return value referencing local variable",
        "E0106" => "Missing lifetime specifier",
        "TRANSPILE" => "Unsupported Python expression/statement (transpiler limitation)",
        "DEPYLER" => "General transpiler error (input/output issue)",
        "EXEC" => "Execution error (runtime failure)",
        "UNKNOWN" => "Unknown error type",
        _ => "See `rustc --explain` for details",
    }
}

/// Get fix recommendation for error code
pub fn fix_recommendation(code: &str) -> &'static str {
    match code {
        "E0425" => "Update codegen.rs to properly declare variables before use",
        "E0412" => "Add generic parameter detection in type_inference.rs",
        "E0308" => "Standardize numeric types in rust_type_mapper.rs",
        "E0277" => "Add missing trait implementations or bounds",
        "E0432" => "Fix import resolution in module_mapper.rs",
        "E0599" => "Check method resolution and trait bounds in type_mapper.rs",
        "E0433" => "Update module path resolution in module_mapper.rs",
        "E0423" => "Fix value/type confusion in codegen",
        "E0369" => "Add operator overloading or type coercion in expr_gen.rs",
        "E0382" => "Review ownership tracking in var_analysis.rs",
        "E0507" => "Add proper cloning or borrowing patterns",
        "E0502" => "Fix mutable borrow patterns in stmt_gen.rs",
        "E0499" => "Review variable reuse patterns",
        "E0515" => "Fix lifetime issues in return statements",
        "E0106" => "Add lifetime annotations in type signatures",
        "TRANSPILE" => "Add support for unsupported expression type in rust_gen/expr_gen.rs",
        "DEPYLER" => "Fix general transpiler error (check error message for details)",
        "EXEC" => "Review generated code for runtime issues",
        "UNKNOWN" => "Investigate error pattern and update transpiler",
        _ => "Investigate error pattern and update transpiler",
    }
}

/// Generate ASCII progress bar
pub fn ascii_bar(ratio: f64, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let clamped = ratio.clamp(0.0, 1.0);
    let filled = (clamped * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty))
}

/// Determine Andon status from pass rate
pub fn andon_status(rate: f64) -> &'static str {
    if rate >= 80.0 {
        "GREEN"
    } else if rate >= 50.0 {
        "YELLOW"
    } else {
        "RED"
    }
}

/// Determine priority level from error count
pub fn priority_level(count: usize) -> &'static str {
    if count >= 20 {
        "P0-CRITICAL"
    } else if count >= 10 {
        "P1-HIGH"
    } else if count >= 5 {
        "P2-MEDIUM"
    } else {
        "P3-LOW"
    }
}

/// Calculate pass rate percentage
pub fn calculate_rate(pass: usize, total: usize) -> f64 {
    if total > 0 {
        (pass as f64 / total as f64) * 100.0
    } else {
        0.0
    }
}

/// Calculate impact percentage of an error type
pub fn calculate_impact(error_count: usize, total_failures: usize) -> f64 {
    if total_failures > 0 {
        (error_count as f64 / total_failures as f64) * 100.0
    } else {
        0.0
    }
}

/// Sort errors by count (descending)
pub fn sort_by_count(taxonomy: &HashMap<String, ErrorEntry>) -> Vec<(String, ErrorEntry)> {
    let mut sorted: Vec<_> = taxonomy
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));
    sorted
}

/// Build co-occurrence map from results
/// Maps (error_code_1, error_code_2) -> count
pub fn build_co_occurrence_map(results: &[AnalysisResult]) -> HashMap<(String, String), usize> {
    let mut map: HashMap<(String, String), usize> = HashMap::new();

    // Group errors by file name
    let mut file_errors: HashMap<String, Vec<String>> = HashMap::new();
    for result in results {
        if !result.success {
            if let Some(code) = &result.error_code {
                file_errors
                    .entry(result.name.clone())
                    .or_default()
                    .push(code.clone());
            }
        }
    }

    // Count co-occurrences
    for errors in file_errors.values() {
        for (i, e1) in errors.iter().enumerate() {
            for e2 in errors.iter().skip(i + 1) {
                // Ensure consistent ordering
                let key = if e1 < e2 {
                    (e1.clone(), e2.clone())
                } else {
                    (e2.clone(), e1.clone())
                };
                *map.entry(key).or_insert(0) += 1;
            }
        }
    }

    map
}

// ============================================================================
// EXTREME TDD: Comprehensive Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== extract_error tests ==========

    #[test]
    fn test_extract_error_e0308() {
        let (code, msg) = extract_error("error[E0308]: mismatched types");
        assert_eq!(code, "E0308");
        assert!(msg.contains("mismatched"));
    }

    #[test]
    fn test_extract_error_e0425() {
        let (code, msg) = extract_error("error[E0425]: cannot find value `x` in this scope");
        assert_eq!(code, "E0425");
        assert!(msg.contains("cannot find value"));
    }

    #[test]
    fn test_extract_error_e0277() {
        let (code, msg) = extract_error("error[E0277]: the trait bound `T: Clone` is not satisfied");
        assert_eq!(code, "E0277");
        assert!(msg.contains("trait bound"));
    }

    #[test]
    fn test_extract_error_e0382() {
        let (code, _) = extract_error("error[E0382]: borrow of moved value");
        assert_eq!(code, "E0382");
    }

    #[test]
    fn test_extract_error_e0599() {
        let (code, _) = extract_error("error[E0599]: no method named `foo` found");
        assert_eq!(code, "E0599");
    }

    #[test]
    fn test_extract_error_e0433() {
        let (code, _) = extract_error("error[E0433]: failed to resolve: use of undeclared crate");
        assert_eq!(code, "E0433");
    }

    #[test]
    fn test_extract_error_transpile_failed() {
        let (code, msg) = extract_error("Error: Failed to transpile\nCaused by:\n  Lambda not supported");
        assert_eq!(code, "TRANSPILE");
        assert!(msg.contains("Lambda") || msg.contains("not supported"));
    }

    #[test]
    fn test_extract_error_unsupported() {
        let (code, _) = extract_error("Unsupported syntax in expression");
        assert_eq!(code, "TRANSPILE");
    }

    #[test]
    fn test_extract_error_not_yet_supported() {
        let (code, _) = extract_error("Feature not yet supported: async generators");
        assert_eq!(code, "TRANSPILE");
    }

    #[test]
    fn test_extract_error_general_error() {
        let (code, _) = extract_error("Error: Something went wrong");
        assert_eq!(code, "DEPYLER");
    }

    #[test]
    fn test_extract_error_unknown() {
        let (code, _) = extract_error("random output with no pattern");
        assert_eq!(code, "UNKNOWN");
    }

    #[test]
    fn test_extract_error_empty() {
        let (code, msg) = extract_error("");
        assert_eq!(code, "UNKNOWN");
        assert!(msg.is_empty());
    }

    #[test]
    fn test_extract_error_multiline() {
        let stderr = "error[E0308]: mismatched types\n  --> src/main.rs:5:5\n   |\n5  |     return x;";
        let (code, msg) = extract_error(stderr);
        assert_eq!(code, "E0308");
        assert!(msg.contains("mismatched"));
    }

    // ========== strip_ansi_codes tests ==========

    #[test]
    fn test_strip_ansi_codes_none() {
        assert_eq!(strip_ansi_codes("hello world"), "hello world");
    }

    #[test]
    fn test_strip_ansi_codes_red() {
        assert_eq!(strip_ansi_codes("\x1b[31mred\x1b[0m"), "red");
    }

    #[test]
    fn test_strip_ansi_codes_multiple() {
        assert_eq!(
            strip_ansi_codes("\x1b[1m\x1b[31mBold Red\x1b[0m"),
            "Bold Red"
        );
    }

    #[test]
    fn test_strip_ansi_codes_mixed() {
        assert_eq!(
            strip_ansi_codes("prefix \x1b[32mgreen\x1b[0m suffix"),
            "prefix green suffix"
        );
    }

    #[test]
    fn test_strip_ansi_codes_empty() {
        assert_eq!(strip_ansi_codes(""), "");
    }

    // ========== extract_rust_error_code tests ==========

    #[test]
    fn test_extract_rust_error_code_found() {
        assert_eq!(
            extract_rust_error_code("error[E0308]: mismatched"),
            Some("E0308".to_string())
        );
    }

    #[test]
    fn test_extract_rust_error_code_not_found() {
        assert_eq!(extract_rust_error_code("warning: unused variable"), None);
    }

    #[test]
    fn test_extract_rust_error_code_malformed() {
        assert_eq!(extract_rust_error_code("error[E0308 missing bracket"), None);
    }

    // ========== analyze_results tests ==========

    #[test]
    fn test_analyze_results_empty() {
        let (pass, fail, taxonomy) = analyze_results(&[]);
        assert_eq!(pass, 0);
        assert_eq!(fail, 0);
        assert!(taxonomy.is_empty());
    }

    #[test]
    fn test_analyze_results_all_pass() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: true, error_code: None, error_message: None },
            AnalysisResult { name: "b".into(), success: true, error_code: None, error_message: None },
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 2);
        assert_eq!(fail, 0);
        assert!(taxonomy.is_empty());
    }

    #[test]
    fn test_analyze_results_all_fail() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: false, error_code: Some("E0308".into()), error_message: Some("type".into()) },
            AnalysisResult { name: "b".into(), success: false, error_code: Some("E0308".into()), error_message: Some("type".into()) },
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 0);
        assert_eq!(fail, 2);
        assert_eq!(taxonomy.get("E0308").unwrap().count, 2);
    }

    #[test]
    fn test_analyze_results_mixed() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: true, error_code: None, error_message: None },
            AnalysisResult { name: "b".into(), success: false, error_code: Some("E0425".into()), error_message: None },
            AnalysisResult { name: "c".into(), success: false, error_code: Some("E0308".into()), error_message: None },
            AnalysisResult { name: "d".into(), success: false, error_code: Some("E0425".into()), error_message: None },
        ];
        let (pass, fail, taxonomy) = analyze_results(&results);
        assert_eq!(pass, 1);
        assert_eq!(fail, 3);
        assert_eq!(taxonomy.get("E0425").unwrap().count, 2);
        assert_eq!(taxonomy.get("E0308").unwrap().count, 1);
    }

    #[test]
    fn test_analyze_results_samples_limited() {
        let results: Vec<AnalysisResult> = (0..10)
            .map(|i| AnalysisResult {
                name: format!("file{}", i),
                success: false,
                error_code: Some("E0425".into()),
                error_message: Some(format!("error {}", i)),
            })
            .collect();
        let (_, _, taxonomy) = analyze_results(&results);
        assert_eq!(taxonomy.get("E0425").unwrap().count, 10);
        assert_eq!(taxonomy.get("E0425").unwrap().samples.len(), 3); // Limited to 3
    }

    #[test]
    fn test_analyze_results_no_error_code() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: false, error_code: None, error_message: None },
        ];
        let (_, fail, taxonomy) = analyze_results(&results);
        assert_eq!(fail, 1);
        assert!(taxonomy.contains_key("UNKNOWN"));
    }

    // ========== error_description tests ==========

    #[test]
    fn test_error_description_e0425() {
        assert!(error_description("E0425").contains("undefined") || error_description("E0425").contains("find value"));
    }

    #[test]
    fn test_error_description_e0308() {
        assert!(error_description("E0308").contains("type"));
    }

    #[test]
    fn test_error_description_e0277() {
        assert!(error_description("E0277").contains("Trait") || error_description("E0277").contains("impl"));
    }

    #[test]
    fn test_error_description_e0382() {
        assert!(error_description("E0382").contains("moved") || error_description("E0382").contains("ownership"));
    }

    #[test]
    fn test_error_description_transpile() {
        assert!(error_description("TRANSPILE").contains("transpiler") || error_description("TRANSPILE").contains("Unsupported"));
    }

    #[test]
    fn test_error_description_depyler() {
        assert!(!error_description("DEPYLER").is_empty());
    }

    #[test]
    fn test_error_description_unknown_code() {
        assert!(error_description("XYZABC").contains("explain"));
    }

    #[test]
    fn test_error_description_all_known_codes() {
        let codes = ["E0425", "E0412", "E0308", "E0277", "E0432", "E0599", "E0433", "E0423", "E0369", "E0382", "E0507"];
        for code in codes {
            let desc = error_description(code);
            assert!(!desc.is_empty(), "Description for {} should not be empty", code);
            assert!(!desc.contains("explain"), "Known code {} should have specific description", code);
        }
    }

    // ========== fix_recommendation tests ==========

    #[test]
    fn test_fix_recommendation_e0425() {
        assert!(fix_recommendation("E0425").contains("codegen"));
    }

    #[test]
    fn test_fix_recommendation_e0308() {
        assert!(fix_recommendation("E0308").contains("type"));
    }

    #[test]
    fn test_fix_recommendation_transpile() {
        assert!(fix_recommendation("TRANSPILE").contains("expr_gen"));
    }

    #[test]
    fn test_fix_recommendation_unknown() {
        assert!(fix_recommendation("XYZABC").contains("Investigate"));
    }

    #[test]
    fn test_fix_recommendation_all_known_codes() {
        let codes = ["E0425", "E0308", "E0277", "E0599", "E0433", "TRANSPILE", "DEPYLER"];
        for code in codes {
            let rec = fix_recommendation(code);
            assert!(!rec.is_empty(), "Recommendation for {} should not be empty", code);
        }
    }

    // ========== ascii_bar tests ==========

    #[test]
    fn test_ascii_bar_zero() {
        let bar = ascii_bar(0.0, 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2591}').count(), 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2588}').count(), 0);
    }

    #[test]
    fn test_ascii_bar_full() {
        let bar = ascii_bar(1.0, 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2588}').count(), 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2591}').count(), 0);
    }

    #[test]
    fn test_ascii_bar_half() {
        let bar = ascii_bar(0.5, 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2588}').count(), 5);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2591}').count(), 5);
    }

    #[test]
    fn test_ascii_bar_clamps_negative() {
        let bar = ascii_bar(-0.5, 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2591}').count(), 10);
    }

    #[test]
    fn test_ascii_bar_clamps_over_one() {
        let bar = ascii_bar(1.5, 10);
        assert_eq!(bar.chars().filter(|c| *c == '\u{2588}').count(), 10);
    }

    #[test]
    fn test_ascii_bar_zero_width() {
        let bar = ascii_bar(0.5, 0);
        assert!(bar.is_empty());
    }

    #[test]
    fn test_ascii_bar_various_widths() {
        for width in [5, 10, 20, 50] {
            let bar = ascii_bar(0.5, width);
            let total_chars: usize = bar.chars().count();
            assert_eq!(total_chars, width);
        }
    }

    // ========== andon_status tests ==========

    #[test]
    fn test_andon_status_green() {
        assert_eq!(andon_status(80.0), "GREEN");
        assert_eq!(andon_status(85.0), "GREEN");
        assert_eq!(andon_status(100.0), "GREEN");
    }

    #[test]
    fn test_andon_status_yellow() {
        assert_eq!(andon_status(50.0), "YELLOW");
        assert_eq!(andon_status(65.0), "YELLOW");
        assert_eq!(andon_status(79.9), "YELLOW");
    }

    #[test]
    fn test_andon_status_red() {
        assert_eq!(andon_status(0.0), "RED");
        assert_eq!(andon_status(25.0), "RED");
        assert_eq!(andon_status(49.9), "RED");
    }

    // ========== priority_level tests ==========

    #[test]
    fn test_priority_level_critical() {
        assert_eq!(priority_level(20), "P0-CRITICAL");
        assert_eq!(priority_level(50), "P0-CRITICAL");
        assert_eq!(priority_level(100), "P0-CRITICAL");
    }

    #[test]
    fn test_priority_level_high() {
        assert_eq!(priority_level(10), "P1-HIGH");
        assert_eq!(priority_level(15), "P1-HIGH");
        assert_eq!(priority_level(19), "P1-HIGH");
    }

    #[test]
    fn test_priority_level_medium() {
        assert_eq!(priority_level(5), "P2-MEDIUM");
        assert_eq!(priority_level(7), "P2-MEDIUM");
        assert_eq!(priority_level(9), "P2-MEDIUM");
    }

    #[test]
    fn test_priority_level_low() {
        assert_eq!(priority_level(0), "P3-LOW");
        assert_eq!(priority_level(1), "P3-LOW");
        assert_eq!(priority_level(4), "P3-LOW");
    }

    // ========== calculate_rate tests ==========

    #[test]
    fn test_calculate_rate_zero() {
        assert_eq!(calculate_rate(0, 0), 0.0);
        assert_eq!(calculate_rate(0, 10), 0.0);
    }

    #[test]
    fn test_calculate_rate_full() {
        assert_eq!(calculate_rate(10, 10), 100.0);
    }

    #[test]
    fn test_calculate_rate_half() {
        assert_eq!(calculate_rate(5, 10), 50.0);
    }

    #[test]
    fn test_calculate_rate_various() {
        assert!((calculate_rate(3, 4) - 75.0).abs() < 0.01);
        assert!((calculate_rate(1, 3) - 33.333).abs() < 0.01);
    }

    // ========== calculate_impact tests ==========

    #[test]
    fn test_calculate_impact_zero() {
        assert_eq!(calculate_impact(0, 0), 0.0);
        assert_eq!(calculate_impact(0, 10), 0.0);
    }

    #[test]
    fn test_calculate_impact_full() {
        assert_eq!(calculate_impact(10, 10), 100.0);
    }

    #[test]
    fn test_calculate_impact_half() {
        assert_eq!(calculate_impact(5, 10), 50.0);
    }

    // ========== sort_by_count tests ==========

    #[test]
    fn test_sort_by_count_empty() {
        let taxonomy: HashMap<String, ErrorEntry> = HashMap::new();
        let sorted = sort_by_count(&taxonomy);
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_sort_by_count_single() {
        let mut taxonomy = HashMap::new();
        taxonomy.insert("E0308".to_string(), ErrorEntry { count: 5, samples: vec![] });
        let sorted = sort_by_count(&taxonomy);
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].0, "E0308");
    }

    #[test]
    fn test_sort_by_count_descending() {
        let mut taxonomy = HashMap::new();
        taxonomy.insert("E0308".to_string(), ErrorEntry { count: 5, samples: vec![] });
        taxonomy.insert("E0425".to_string(), ErrorEntry { count: 10, samples: vec![] });
        taxonomy.insert("E0277".to_string(), ErrorEntry { count: 3, samples: vec![] });

        let sorted = sort_by_count(&taxonomy);
        assert_eq!(sorted[0].0, "E0425"); // Highest count first
        assert_eq!(sorted[1].0, "E0308");
        assert_eq!(sorted[2].0, "E0277"); // Lowest count last
    }

    // ========== build_co_occurrence_map tests ==========

    #[test]
    fn test_build_co_occurrence_map_empty() {
        let results: Vec<AnalysisResult> = vec![];
        let map = build_co_occurrence_map(&results);
        assert!(map.is_empty());
    }

    #[test]
    fn test_build_co_occurrence_map_all_pass() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: true, error_code: None, error_message: None },
        ];
        let map = build_co_occurrence_map(&results);
        assert!(map.is_empty());
    }

    #[test]
    fn test_build_co_occurrence_map_single_error() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: false, error_code: Some("E0308".into()), error_message: None },
        ];
        let map = build_co_occurrence_map(&results);
        // Single error in a file, no co-occurrence
        assert!(map.is_empty());
    }

    #[test]
    fn test_build_co_occurrence_map_no_overlap() {
        let results = vec![
            AnalysisResult { name: "a".into(), success: false, error_code: Some("E0308".into()), error_message: None },
            AnalysisResult { name: "b".into(), success: false, error_code: Some("E0425".into()), error_message: None },
        ];
        let map = build_co_occurrence_map(&results);
        // Different files, no co-occurrence
        assert!(map.is_empty());
    }

    // ========== AnalysisResult tests ==========

    #[test]
    fn test_analysis_result_debug() {
        let result = AnalysisResult {
            name: "test".to_string(),
            success: true,
            error_code: None,
            error_message: None,
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("test"));
    }

    #[test]
    fn test_analysis_result_clone() {
        let result = AnalysisResult {
            name: "test".to_string(),
            success: false,
            error_code: Some("E0308".to_string()),
            error_message: Some("type error".to_string()),
        };
        let cloned = result.clone();
        assert_eq!(result.name, cloned.name);
        assert_eq!(result.success, cloned.success);
        assert_eq!(result.error_code, cloned.error_code);
    }

    // ========== ErrorEntry tests ==========

    #[test]
    fn test_error_entry_default() {
        let entry = ErrorEntry::default();
        assert_eq!(entry.count, 0);
        assert!(entry.samples.is_empty());
    }

    #[test]
    fn test_error_entry_clone() {
        let entry = ErrorEntry {
            count: 5,
            samples: vec!["a".to_string(), "b".to_string()],
        };
        let cloned = entry.clone();
        assert_eq!(entry.count, cloned.count);
        assert_eq!(entry.samples, cloned.samples);
    }

    #[test]
    fn test_error_entry_equality() {
        let e1 = ErrorEntry { count: 5, samples: vec![] };
        let e2 = ErrorEntry { count: 5, samples: vec![] };
        let e3 = ErrorEntry { count: 3, samples: vec![] };
        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }
}

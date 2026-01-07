//! Pure functions for explain command - EXTREME TDD
//!
//! DEPYLER-COVERAGE-95: Extracted from lib.rs for testability
//! These functions have no side effects and can be comprehensively tested.

use serde::{Deserialize, Serialize};

/// Compilation error parsed from rustc output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompilationError {
    /// Error code (e.g., "E0308")
    pub code: String,
    /// Error message
    pub message: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
}

/// A correlated decision from the transpiler trace
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CorrelatedDecision {
    /// Error code this correlates to
    pub error_code: String,
    /// Decision category
    pub category: String,
    /// Decision name
    pub name: String,
    /// Path chosen by transpiler
    pub chosen_path: String,
    /// Alternative paths considered
    pub alternatives: Vec<String>,
    /// Confidence score
    pub confidence: f64,
    /// Human-readable explanation
    pub explanation: String,
}

/// Parse Rust compiler errors from stderr output
///
/// # Arguments
/// * `stderr` - The stderr output from rustc
///
/// # Returns
/// A vector of parsed compilation errors
pub fn parse_rust_errors(stderr: &str) -> Vec<CompilationError> {
    let error_re = regex::Regex::new(r"error\[([E\d]+)\]: (.+?)(?:\n|$)").unwrap();
    let location_re = regex::Regex::new(r"--> .+?:(\d+):(\d+)").unwrap();

    let mut errors = Vec::new();
    let mut current_code = String::new();
    let mut current_message = String::new();

    for line in stderr.lines() {
        if let Some(caps) = error_re.captures(line) {
            current_code = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            current_message = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
        } else if let Some(caps) = location_re.captures(line) {
            if !current_code.is_empty() {
                let line_num: usize = caps.get(1).map(|m| m.as_str().parse().unwrap_or(0)).unwrap_or(0);
                let col_num: usize = caps.get(2).map(|m| m.as_str().parse().unwrap_or(0)).unwrap_or(0);
                errors.push(CompilationError {
                    code: current_code.clone(),
                    message: current_message.clone(),
                    line: line_num,
                    column: col_num,
                });
                current_code.clear();
                current_message.clear();
            }
        }
    }

    errors
}

/// Get relevant decision categories for an error code
///
/// Maps Rust error codes to likely transpiler decision categories
pub fn get_relevant_categories(error_code: &str) -> Vec<&'static str> {
    match error_code {
        "E0277" => vec!["TypeMapping", "BorrowStrategy", "Ownership"],
        "E0308" => vec!["TypeMapping"],
        "E0382" => vec!["Ownership", "BorrowStrategy"],
        "E0502" | "E0503" => vec!["BorrowStrategy", "LifetimeInfer"],
        "E0106" => vec!["LifetimeInfer"],
        "E0412" | "E0433" => vec!["ImportResolve", "TypeMapping"],
        "E0599" => vec!["MethodDispatch", "TypeMapping"],
        "E0425" => vec!["ImportResolve", "NameResolution"],
        "E0061" => vec!["TypeMapping", "FunctionCall"],
        "E0658" => vec!["FeatureGate"],
        "E0583" => vec!["ImportResolve", "ModuleResolution"],
        _ => vec![],
    }
}

/// Generate human-readable explanation for error-decision correlation
///
/// # Arguments
/// * `error_code` - The Rust error code
/// * `category` - The decision category
/// * `chosen_path` - The path chosen by the transpiler
pub fn generate_error_explanation(error_code: &str, category: &str, chosen_path: &str) -> String {
    match (error_code, category) {
        ("E0277", "TypeMapping") => format!(
            "Type '{}' may not implement required trait. Consider explicit trait bounds.",
            chosen_path
        ),
        ("E0277", "BorrowStrategy") => format!(
            "Borrow strategy '{}' may conflict with trait requirements.",
            chosen_path
        ),
        ("E0277", "Ownership") => format!(
            "Ownership model '{}' may require trait implementation.",
            chosen_path
        ),
        ("E0308", "TypeMapping") => format!(
            "Type mapping chose '{}' but context expects different type.",
            chosen_path
        ),
        ("E0382", "Ownership") => format!(
            "Ownership decision '{}' caused value to be moved. Consider cloning.",
            chosen_path
        ),
        ("E0382", "BorrowStrategy") => format!(
            "Borrow strategy '{}' may have transferred ownership unexpectedly.",
            chosen_path
        ),
        ("E0502", "BorrowStrategy") | ("E0503", "BorrowStrategy") => format!(
            "Borrow strategy '{}' conflicts with existing borrow.",
            chosen_path
        ),
        ("E0502", "LifetimeInfer") | ("E0503", "LifetimeInfer") => format!(
            "Lifetime inference '{}' may need explicit annotation.",
            chosen_path
        ),
        ("E0106", "LifetimeInfer") => format!(
            "Lifetime inference chose '{}' but explicit annotation needed.",
            chosen_path
        ),
        ("E0412", "TypeMapping") | ("E0433", "TypeMapping") => format!(
            "Type '{}' not found. Check import statements.",
            chosen_path
        ),
        ("E0412", "ImportResolve") | ("E0433", "ImportResolve") => format!(
            "Import resolution for '{}' failed. Module may not exist.",
            chosen_path
        ),
        ("E0599", "MethodDispatch") => format!(
            "Method dispatch for '{}' found no matching method.",
            chosen_path
        ),
        ("E0599", "TypeMapping") => format!(
            "Type '{}' has no method with that name. Check type inference.",
            chosen_path
        ),
        ("E0425", "ImportResolve") => format!(
            "Name '{}' not found in scope. Check imports and local bindings.",
            chosen_path
        ),
        ("E0425", "NameResolution") => format!(
            "Name resolution for '{}' failed. Variable may not be defined.",
            chosen_path
        ),
        ("E0061", "TypeMapping") => format!(
            "Type mapping for arguments to '{}' may be incorrect.",
            chosen_path
        ),
        ("E0061", "FunctionCall") => format!(
            "Function call '{}' has wrong number of arguments.",
            chosen_path
        ),
        _ => format!(
            "Decision '{}' in category '{}' may have caused this error.",
            chosen_path, category
        ),
    }
}

/// Filter errors by error code
///
/// # Arguments
/// * `errors` - The list of errors to filter
/// * `filter_code` - Optional error code to filter by
///
/// # Returns
/// Filtered list of errors
pub fn filter_errors_by_code<'a>(
    errors: &'a [CompilationError],
    filter_code: Option<&str>,
) -> Vec<&'a CompilationError> {
    match filter_code {
        Some(code) => errors.iter().filter(|e| e.code == code).collect(),
        None => errors.iter().collect(),
    }
}

/// Check if a category matches any of the relevant categories
pub fn category_matches(category_str: &str, relevant_categories: &[&str]) -> bool {
    relevant_categories.iter().any(|c| category_str.contains(c))
}

/// Extract error code from a Rust compiler error message
///
/// # Examples
/// ```
/// use depyler::explain_shim::extract_error_code_from_message;
/// assert_eq!(extract_error_code_from_message("error[E0308]: mismatched types"), Some("E0308".to_string()));
/// ```
pub fn extract_error_code_from_message(error: &str) -> Option<String> {
    error
        .char_indices()
        .find_map(|(i, c)| {
            if c == 'E' && error.len() >= i + 5 {
                let candidate = &error[i..i + 5];
                if candidate.chars().skip(1).all(|d| d.is_ascii_digit()) {
                    return Some(candidate.to_string());
                }
            }
            None
        })
}

/// Count errors by code
pub fn count_errors_by_code(errors: &[CompilationError]) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for error in errors {
        *counts.entry(error.code.clone()).or_insert(0) += 1;
    }
    counts
}

/// Get unique error codes from a list of errors
pub fn get_unique_error_codes(errors: &[CompilationError]) -> Vec<String> {
    let mut codes: Vec<String> = errors.iter().map(|e| e.code.clone()).collect();
    codes.sort();
    codes.dedup();
    codes
}

/// Categorize error severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical errors that prevent compilation
    Critical,
    /// Type errors that may be fixable
    TypeRelated,
    /// Borrow checker errors
    BorrowChecker,
    /// Import/module resolution errors
    ImportRelated,
    /// Other errors
    Other,
}

impl ErrorSeverity {
    /// Get severity from error code
    pub fn from_error_code(code: &str) -> Self {
        match code {
            "E0308" | "E0277" | "E0061" => ErrorSeverity::TypeRelated,
            "E0382" | "E0502" | "E0503" | "E0106" => ErrorSeverity::BorrowChecker,
            "E0412" | "E0433" | "E0425" | "E0583" => ErrorSeverity::ImportRelated,
            "E0601" | "E0658" => ErrorSeverity::Critical,
            _ => ErrorSeverity::Other,
        }
    }

    /// Get human-readable name
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Critical => "Critical",
            ErrorSeverity::TypeRelated => "Type-related",
            ErrorSeverity::BorrowChecker => "Borrow checker",
            ErrorSeverity::ImportRelated => "Import/module",
            ErrorSeverity::Other => "Other",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== parse_rust_errors tests ====================

    #[test]
    fn test_parse_rust_errors_single_error() {
        let stderr = r#"error[E0308]: mismatched types
 --> src/main.rs:10:5
  |
10|     let x: i32 = "hello";
  |                  ^^^^^^^ expected `i32`, found `&str`"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E0308");
        assert_eq!(errors[0].line, 10);
        assert_eq!(errors[0].column, 5);
    }

    #[test]
    fn test_parse_rust_errors_multiple_errors() {
        let stderr = r#"error[E0308]: mismatched types
 --> src/main.rs:10:5
error[E0382]: use of moved value
 --> src/main.rs:15:10"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].code, "E0308");
        assert_eq!(errors[1].code, "E0382");
    }

    #[test]
    fn test_parse_rust_errors_empty_input() {
        let errors = parse_rust_errors("");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_rust_errors_no_errors() {
        let stderr = "warning: unused variable `x`";
        let errors = parse_rust_errors(stderr);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_parse_rust_errors_e0277() {
        let stderr = r#"error[E0277]: the trait bound `Foo: Display` is not satisfied
 --> src/lib.rs:5:15"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E0277");
        assert!(errors[0].message.contains("trait bound"));
    }

    #[test]
    fn test_parse_rust_errors_e0502() {
        let stderr = r#"error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
 --> src/main.rs:8:9"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E0502");
    }

    #[test]
    fn test_parse_rust_errors_preserves_message() {
        let stderr = r#"error[E0599]: no method named `foo` found for type `Bar`
 --> src/lib.rs:12:8"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("no method named"));
    }

    // ==================== get_relevant_categories tests ====================

    #[test]
    fn test_get_relevant_categories_e0277() {
        let cats = get_relevant_categories("E0277");
        assert!(cats.contains(&"TypeMapping"));
        assert!(cats.contains(&"BorrowStrategy"));
        assert!(cats.contains(&"Ownership"));
    }

    #[test]
    fn test_get_relevant_categories_e0308() {
        let cats = get_relevant_categories("E0308");
        assert!(cats.contains(&"TypeMapping"));
        assert_eq!(cats.len(), 1);
    }

    #[test]
    fn test_get_relevant_categories_e0382() {
        let cats = get_relevant_categories("E0382");
        assert!(cats.contains(&"Ownership"));
        assert!(cats.contains(&"BorrowStrategy"));
    }

    #[test]
    fn test_get_relevant_categories_e0502() {
        let cats = get_relevant_categories("E0502");
        assert!(cats.contains(&"BorrowStrategy"));
        assert!(cats.contains(&"LifetimeInfer"));
    }

    #[test]
    fn test_get_relevant_categories_e0503() {
        let cats = get_relevant_categories("E0503");
        assert!(cats.contains(&"BorrowStrategy"));
        assert!(cats.contains(&"LifetimeInfer"));
    }

    #[test]
    fn test_get_relevant_categories_e0106() {
        let cats = get_relevant_categories("E0106");
        assert!(cats.contains(&"LifetimeInfer"));
    }

    #[test]
    fn test_get_relevant_categories_e0412() {
        let cats = get_relevant_categories("E0412");
        assert!(cats.contains(&"ImportResolve"));
        assert!(cats.contains(&"TypeMapping"));
    }

    #[test]
    fn test_get_relevant_categories_e0433() {
        let cats = get_relevant_categories("E0433");
        assert!(cats.contains(&"ImportResolve"));
        assert!(cats.contains(&"TypeMapping"));
    }

    #[test]
    fn test_get_relevant_categories_e0599() {
        let cats = get_relevant_categories("E0599");
        assert!(cats.contains(&"MethodDispatch"));
        assert!(cats.contains(&"TypeMapping"));
    }

    #[test]
    fn test_get_relevant_categories_e0425() {
        let cats = get_relevant_categories("E0425");
        assert!(cats.contains(&"ImportResolve"));
        assert!(cats.contains(&"NameResolution"));
    }

    #[test]
    fn test_get_relevant_categories_unknown() {
        let cats = get_relevant_categories("E9999");
        assert!(cats.is_empty());
    }

    // ==================== generate_error_explanation tests ====================

    #[test]
    fn test_generate_explanation_e0277_type_mapping() {
        let exp = generate_error_explanation("E0277", "TypeMapping", "String");
        assert!(exp.contains("String"));
        assert!(exp.contains("trait"));
    }

    #[test]
    fn test_generate_explanation_e0277_borrow_strategy() {
        let exp = generate_error_explanation("E0277", "BorrowStrategy", "&mut");
        assert!(exp.contains("&mut"));
        assert!(exp.contains("trait"));
    }

    #[test]
    fn test_generate_explanation_e0308_type_mapping() {
        let exp = generate_error_explanation("E0308", "TypeMapping", "i32");
        assert!(exp.contains("i32"));
        assert!(exp.contains("different type"));
    }

    #[test]
    fn test_generate_explanation_e0382_ownership() {
        let exp = generate_error_explanation("E0382", "Ownership", "move");
        assert!(exp.contains("moved"));
        assert!(exp.contains("cloning"));
    }

    #[test]
    fn test_generate_explanation_e0502_borrow() {
        let exp = generate_error_explanation("E0502", "BorrowStrategy", "&");
        assert!(exp.contains("conflicts"));
    }

    #[test]
    fn test_generate_explanation_e0106_lifetime() {
        let exp = generate_error_explanation("E0106", "LifetimeInfer", "'a");
        assert!(exp.contains("'a"));
        assert!(exp.contains("annotation"));
    }

    #[test]
    fn test_generate_explanation_e0412_import() {
        let exp = generate_error_explanation("E0412", "ImportResolve", "HashMap");
        assert!(exp.contains("HashMap"));
    }

    #[test]
    fn test_generate_explanation_e0599_method() {
        let exp = generate_error_explanation("E0599", "MethodDispatch", "to_string");
        assert!(exp.contains("to_string"));
        assert!(exp.contains("method"));
    }

    #[test]
    fn test_generate_explanation_unknown() {
        let exp = generate_error_explanation("E9999", "Unknown", "path");
        assert!(exp.contains("path"));
        assert!(exp.contains("Unknown"));
    }

    // ==================== filter_errors_by_code tests ====================

    #[test]
    fn test_filter_errors_none_filter() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
            CompilationError { code: "E0382".into(), message: "".into(), line: 2, column: 1 },
        ];
        let filtered = filter_errors_by_code(&errors, None);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_errors_with_filter() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
            CompilationError { code: "E0382".into(), message: "".into(), line: 2, column: 1 },
            CompilationError { code: "E0308".into(), message: "".into(), line: 3, column: 1 },
        ];
        let filtered = filter_errors_by_code(&errors, Some("E0308"));
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_errors_no_matches() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
        ];
        let filtered = filter_errors_by_code(&errors, Some("E9999"));
        assert!(filtered.is_empty());
    }

    // ==================== category_matches tests ====================

    #[test]
    fn test_category_matches_exact() {
        assert!(category_matches("TypeMapping", &["TypeMapping"]));
    }

    #[test]
    fn test_category_matches_contains() {
        assert!(category_matches("TypeMapping::Int", &["TypeMapping"]));
    }

    #[test]
    fn test_category_matches_multiple() {
        assert!(category_matches("BorrowStrategy", &["Ownership", "BorrowStrategy"]));
    }

    #[test]
    fn test_category_matches_none() {
        assert!(!category_matches("Something", &["TypeMapping", "Ownership"]));
    }

    // ==================== extract_error_code_from_message tests ====================

    #[test]
    fn test_extract_error_code_standard() {
        assert_eq!(extract_error_code_from_message("error[E0308]: msg"), Some("E0308".into()));
    }

    #[test]
    fn test_extract_error_code_in_text() {
        assert_eq!(extract_error_code_from_message("see E0277 for details"), Some("E0277".into()));
    }

    #[test]
    fn test_extract_error_code_no_code() {
        assert_eq!(extract_error_code_from_message("just a message"), None);
    }

    #[test]
    fn test_extract_error_code_partial() {
        assert_eq!(extract_error_code_from_message("E12"), None); // too short
    }

    // ==================== count_errors_by_code tests ====================

    #[test]
    fn test_count_errors_empty() {
        let counts = count_errors_by_code(&[]);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_count_errors_single() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
        ];
        let counts = count_errors_by_code(&errors);
        assert_eq!(counts.get("E0308"), Some(&1));
    }

    #[test]
    fn test_count_errors_multiple_same() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
            CompilationError { code: "E0308".into(), message: "".into(), line: 2, column: 1 },
            CompilationError { code: "E0308".into(), message: "".into(), line: 3, column: 1 },
        ];
        let counts = count_errors_by_code(&errors);
        assert_eq!(counts.get("E0308"), Some(&3));
    }

    #[test]
    fn test_count_errors_multiple_different() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
            CompilationError { code: "E0382".into(), message: "".into(), line: 2, column: 1 },
            CompilationError { code: "E0308".into(), message: "".into(), line: 3, column: 1 },
        ];
        let counts = count_errors_by_code(&errors);
        assert_eq!(counts.get("E0308"), Some(&2));
        assert_eq!(counts.get("E0382"), Some(&1));
    }

    // ==================== get_unique_error_codes tests ====================

    #[test]
    fn test_unique_codes_empty() {
        let codes = get_unique_error_codes(&[]);
        assert!(codes.is_empty());
    }

    #[test]
    fn test_unique_codes_single() {
        let errors = vec![
            CompilationError { code: "E0308".into(), message: "".into(), line: 1, column: 1 },
        ];
        let codes = get_unique_error_codes(&errors);
        assert_eq!(codes, vec!["E0308"]);
    }

    #[test]
    fn test_unique_codes_dedupe() {
        let errors = vec![
            CompilationError { code: "E0382".into(), message: "".into(), line: 1, column: 1 },
            CompilationError { code: "E0308".into(), message: "".into(), line: 2, column: 1 },
            CompilationError { code: "E0382".into(), message: "".into(), line: 3, column: 1 },
        ];
        let codes = get_unique_error_codes(&errors);
        assert_eq!(codes.len(), 2);
        assert!(codes.contains(&"E0308".to_string()));
        assert!(codes.contains(&"E0382".to_string()));
    }

    // ==================== ErrorSeverity tests ====================

    #[test]
    fn test_severity_critical() {
        assert_eq!(ErrorSeverity::from_error_code("E0601"), ErrorSeverity::Critical);
        assert_eq!(ErrorSeverity::from_error_code("E0658"), ErrorSeverity::Critical);
    }

    #[test]
    fn test_severity_type_related() {
        assert_eq!(ErrorSeverity::from_error_code("E0308"), ErrorSeverity::TypeRelated);
        assert_eq!(ErrorSeverity::from_error_code("E0277"), ErrorSeverity::TypeRelated);
        assert_eq!(ErrorSeverity::from_error_code("E0061"), ErrorSeverity::TypeRelated);
    }

    #[test]
    fn test_severity_borrow_checker() {
        assert_eq!(ErrorSeverity::from_error_code("E0382"), ErrorSeverity::BorrowChecker);
        assert_eq!(ErrorSeverity::from_error_code("E0502"), ErrorSeverity::BorrowChecker);
        assert_eq!(ErrorSeverity::from_error_code("E0503"), ErrorSeverity::BorrowChecker);
        assert_eq!(ErrorSeverity::from_error_code("E0106"), ErrorSeverity::BorrowChecker);
    }

    #[test]
    fn test_severity_import_related() {
        assert_eq!(ErrorSeverity::from_error_code("E0412"), ErrorSeverity::ImportRelated);
        assert_eq!(ErrorSeverity::from_error_code("E0433"), ErrorSeverity::ImportRelated);
        assert_eq!(ErrorSeverity::from_error_code("E0425"), ErrorSeverity::ImportRelated);
        assert_eq!(ErrorSeverity::from_error_code("E0583"), ErrorSeverity::ImportRelated);
    }

    #[test]
    fn test_severity_other() {
        assert_eq!(ErrorSeverity::from_error_code("E9999"), ErrorSeverity::Other);
    }

    #[test]
    fn test_severity_as_str() {
        assert_eq!(ErrorSeverity::Critical.as_str(), "Critical");
        assert_eq!(ErrorSeverity::TypeRelated.as_str(), "Type-related");
        assert_eq!(ErrorSeverity::BorrowChecker.as_str(), "Borrow checker");
        assert_eq!(ErrorSeverity::ImportRelated.as_str(), "Import/module");
        assert_eq!(ErrorSeverity::Other.as_str(), "Other");
    }

    // ==================== CompilationError tests ====================

    #[test]
    fn test_compilation_error_equality() {
        let e1 = CompilationError {
            code: "E0308".into(),
            message: "test".into(),
            line: 10,
            column: 5,
        };
        let e2 = CompilationError {
            code: "E0308".into(),
            message: "test".into(),
            line: 10,
            column: 5,
        };
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_compilation_error_inequality() {
        let e1 = CompilationError {
            code: "E0308".into(),
            message: "test".into(),
            line: 10,
            column: 5,
        };
        let e2 = CompilationError {
            code: "E0382".into(),
            message: "test".into(),
            line: 10,
            column: 5,
        };
        assert_ne!(e1, e2);
    }

    #[test]
    fn test_compilation_error_clone() {
        let e1 = CompilationError {
            code: "E0308".into(),
            message: "test".into(),
            line: 10,
            column: 5,
        };
        let e2 = e1.clone();
        assert_eq!(e1, e2);
    }

    // ==================== CorrelatedDecision tests ====================

    #[test]
    fn test_correlated_decision_clone() {
        let d1 = CorrelatedDecision {
            error_code: "E0308".into(),
            category: "TypeMapping".into(),
            name: "test".into(),
            chosen_path: "i32".into(),
            alternatives: vec!["i64".into()],
            confidence: 0.9,
            explanation: "Test explanation".into(),
        };
        let d2 = d1.clone();
        assert_eq!(d1.error_code, d2.error_code);
        assert_eq!(d1.confidence, d2.confidence);
    }

    // ==================== Edge case tests ====================

    #[test]
    fn test_parse_errors_with_notes() {
        let stderr = r#"error[E0308]: mismatched types
 --> src/main.rs:10:5
  |
note: this is a note
  |
10|     code here"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_parse_errors_with_help() {
        let stderr = r#"error[E0382]: use of moved value: `x`
 --> src/main.rs:5:5
  |
help: consider cloning the value"#;

        let errors = parse_rust_errors(stderr);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E0382");
    }

    #[test]
    fn test_generate_explanation_e0277_ownership() {
        let exp = generate_error_explanation("E0277", "Ownership", "owned");
        assert!(exp.contains("Ownership"));
    }

    #[test]
    fn test_generate_explanation_e0382_borrow_strategy() {
        let exp = generate_error_explanation("E0382", "BorrowStrategy", "&mut");
        assert!(exp.contains("ownership"));
    }

    #[test]
    fn test_generate_explanation_e0061_type() {
        let exp = generate_error_explanation("E0061", "TypeMapping", "fn call");
        assert!(exp.contains("arguments"));
    }

    #[test]
    fn test_generate_explanation_e0061_function() {
        let exp = generate_error_explanation("E0061", "FunctionCall", "my_func");
        assert!(exp.contains("arguments"));
    }

    #[test]
    fn test_get_relevant_categories_e0061() {
        let cats = get_relevant_categories("E0061");
        assert!(cats.contains(&"TypeMapping"));
        assert!(cats.contains(&"FunctionCall"));
    }

    #[test]
    fn test_get_relevant_categories_e0658() {
        let cats = get_relevant_categories("E0658");
        assert!(cats.contains(&"FeatureGate"));
    }

    #[test]
    fn test_get_relevant_categories_e0583() {
        let cats = get_relevant_categories("E0583");
        assert!(cats.contains(&"ImportResolve"));
        assert!(cats.contains(&"ModuleResolution"));
    }
}

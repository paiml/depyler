//! Error taxonomy module (Phase 4a).
//!
//! Classifies Rust compiler errors according to the taxonomy defined in
//! the specification. Based on defect classification research [Shull et al., 2002].

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error category based on root cause analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCategory {
    /// E0308: Mismatched types - type inference failure.
    TypeMismatch,
    /// E0412: Cannot find type - generic parameter unresolved.
    UndefinedType,
    /// E0425: Cannot find value - missing import/binding.
    UndefinedValue,
    /// E0282: Type annotations needed - insufficient type info.
    TypeAnnotation,
    /// E0277: Trait not implemented - missing trait impl.
    TraitBound,
    /// E0502, E0503, E0505: Borrow checker errors.
    BorrowCheck,
    /// E0106, E0621: Lifetime errors.
    Lifetime,
    /// E0061, E0433: Syntax/parsing errors.
    Syntax,
    /// Other/uncategorized errors.
    Other,
}

impl ErrorCategory {
    /// Get the category for a given error code.
    pub fn from_error_code(code: &str) -> Self {
        match code {
            "E0308" => Self::TypeMismatch,
            "E0412" => Self::UndefinedType,
            "E0425" => Self::UndefinedValue,
            "E0282" => Self::TypeAnnotation,
            "E0277" => Self::TraitBound,
            "E0502" | "E0503" | "E0505" => Self::BorrowCheck,
            "E0106" | "E0621" => Self::Lifetime,
            "E0061" | "E0433" => Self::Syntax,
            _ => Self::Other,
        }
    }

    /// Get a human-readable description of this category.
    pub fn description(&self) -> &'static str {
        match self {
            Self::TypeMismatch => "Type inference failure",
            Self::UndefinedType => "Generic parameter unresolved",
            Self::UndefinedValue => "Missing import/binding",
            Self::TypeAnnotation => "Insufficient type info",
            Self::TraitBound => "Missing trait implementation",
            Self::BorrowCheck => "Ownership violation",
            Self::Lifetime => "Missing lifetime annotation",
            Self::Syntax => "Malformed code generation",
            Self::Other => "Uncategorized error",
        }
    }
}

/// Blocker priority level based on frequency and impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BlockerPriority {
    /// P0: Critical - blocks >20% of corpus.
    P0Critical,
    /// P1: High - blocks 10-20% of corpus.
    P1High,
    /// P2: Medium - blocks 5-10% of corpus.
    P2Medium,
    /// P3: Low - blocks <5% of corpus.
    P3Low,
}

impl BlockerPriority {
    /// Determine priority based on frequency and total count.
    pub fn from_frequency(count: usize, total: usize) -> Self {
        if total == 0 {
            return Self::P3Low;
        }
        let percentage = (count as f64 / total as f64) * 100.0;
        if percentage > 20.0 || count >= 50 {
            Self::P0Critical
        } else if percentage > 10.0 || count >= 20 {
            Self::P1High
        } else if percentage > 5.0 || count >= 10 {
            Self::P2Medium
        } else {
            Self::P3Low
        }
    }
}

/// A parsed Rust compiler error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustError {
    /// The error code (e.g., "E0308").
    pub code: String,
    /// The error message.
    pub message: String,
    /// The file where the error occurred.
    pub file: String,
    /// The line number (if available).
    pub line: Option<usize>,
    /// The error category.
    pub category: ErrorCategory,
}

impl RustError {
    /// Parse a Rust compiler error from stderr output.
    pub fn parse(line: &str) -> Option<Self> {
        // Pattern: error[E0308]: mismatched types
        if !line.starts_with("error[E") {
            return None;
        }

        let code_start = line.find('[')? + 1;
        let code_end = line.find(']')?;
        let code = line[code_start..code_end].to_string();

        let message_start = line.find(':')?;
        let message = line[message_start + 1..].trim().to_string();

        let category = ErrorCategory::from_error_code(&code);

        Some(Self {
            code,
            message,
            file: String::new(),
            line: None,
            category,
        })
    }

    /// Parse multiple errors from compiler output.
    pub fn parse_all(output: &str) -> Vec<Self> {
        output.lines().filter_map(Self::parse).collect()
    }
}

/// Full error taxonomy analysis for a corpus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTaxonomy {
    /// All errors collected.
    pub errors: Vec<RustError>,
    /// Error counts by category.
    pub by_category: HashMap<ErrorCategory, usize>,
    /// Error counts by code.
    pub by_code: HashMap<String, usize>,
    /// Blocker analysis.
    pub blockers: Vec<BlockerInfo>,
}

/// Information about a blocker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockerInfo {
    /// The error code.
    pub error_code: String,
    /// Number of occurrences.
    pub count: usize,
    /// Priority level.
    pub priority: BlockerPriority,
    /// Category.
    pub category: ErrorCategory,
    /// Root cause description.
    pub root_cause: String,
    /// Recommended fix.
    pub recommended_fix: String,
}

impl ErrorTaxonomy {
    /// Classify errors from compilation results.
    pub fn classify(results: &[super::compiler::CompilationResult]) -> Self {
        let mut errors = Vec::new();
        let mut by_category: HashMap<ErrorCategory, usize> = HashMap::new();
        let mut by_code: HashMap<String, usize> = HashMap::new();

        for result in results {
            if let Some(ref stderr) = result.stderr {
                let parsed = RustError::parse_all(stderr);
                for error in parsed {
                    *by_category.entry(error.category).or_insert(0) += 1;
                    *by_code.entry(error.code.clone()).or_insert(0) += 1;
                    errors.push(error);
                }
            }
        }

        let total_files = results.len();
        let blockers = Self::analyze_blockers(&by_code, total_files);

        Self {
            errors,
            by_category,
            by_code,
            blockers,
        }
    }

    fn analyze_blockers(by_code: &HashMap<String, usize>, total: usize) -> Vec<BlockerInfo> {
        let mut blockers: Vec<_> = by_code
            .iter()
            .map(|(code, &count)| {
                let category = ErrorCategory::from_error_code(code);
                let priority = BlockerPriority::from_frequency(count, total);
                BlockerInfo {
                    error_code: code.clone(),
                    count,
                    priority,
                    category,
                    root_cause: category.description().to_string(),
                    recommended_fix: Self::suggest_fix(code),
                }
            })
            .collect();

        blockers.sort_by(|a, b| b.count.cmp(&a.count));
        blockers
    }

    fn suggest_fix(code: &str) -> String {
        match code {
            "E0308" => "Improve bidirectional type inference".to_string(),
            "E0412" => "Resolve generic type parameters from context".to_string(),
            "E0425" => "Add missing imports or variable bindings".to_string(),
            "E0282" => "Add explicit type annotations".to_string(),
            "E0277" => "Implement required traits".to_string(),
            _ => "Investigate specific pattern".to_string(),
        }
    }

    /// Get blockers by priority level.
    pub fn blockers_by_priority(&self, priority: BlockerPriority) -> Vec<&BlockerInfo> {
        self.blockers
            .iter()
            .filter(|b| b.priority == priority)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::CompilationResult;
    use std::path::PathBuf;
    use std::time::Duration;

    #[test]
    fn test_error_category_from_code() {
        assert_eq!(
            ErrorCategory::from_error_code("E0308"),
            ErrorCategory::TypeMismatch
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0412"),
            ErrorCategory::UndefinedType
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0425"),
            ErrorCategory::UndefinedValue
        );
        assert_eq!(
            ErrorCategory::from_error_code("E9999"),
            ErrorCategory::Other
        );
    }

    #[test]
    fn test_error_category_from_code_all_variants() {
        // TypeAnnotation
        assert_eq!(
            ErrorCategory::from_error_code("E0282"),
            ErrorCategory::TypeAnnotation
        );
        // TraitBound
        assert_eq!(
            ErrorCategory::from_error_code("E0277"),
            ErrorCategory::TraitBound
        );
        // BorrowCheck variants
        assert_eq!(
            ErrorCategory::from_error_code("E0502"),
            ErrorCategory::BorrowCheck
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0503"),
            ErrorCategory::BorrowCheck
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0505"),
            ErrorCategory::BorrowCheck
        );
        // Lifetime variants
        assert_eq!(
            ErrorCategory::from_error_code("E0106"),
            ErrorCategory::Lifetime
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0621"),
            ErrorCategory::Lifetime
        );
        // Syntax variants
        assert_eq!(
            ErrorCategory::from_error_code("E0061"),
            ErrorCategory::Syntax
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0433"),
            ErrorCategory::Syntax
        );
    }

    #[test]
    fn test_error_category_description_all() {
        assert_eq!(
            ErrorCategory::TypeMismatch.description(),
            "Type inference failure"
        );
        assert_eq!(
            ErrorCategory::UndefinedType.description(),
            "Generic parameter unresolved"
        );
        assert_eq!(
            ErrorCategory::UndefinedValue.description(),
            "Missing import/binding"
        );
        assert_eq!(
            ErrorCategory::TypeAnnotation.description(),
            "Insufficient type info"
        );
        assert_eq!(
            ErrorCategory::TraitBound.description(),
            "Missing trait implementation"
        );
        assert_eq!(
            ErrorCategory::BorrowCheck.description(),
            "Ownership violation"
        );
        assert_eq!(
            ErrorCategory::Lifetime.description(),
            "Missing lifetime annotation"
        );
        assert_eq!(
            ErrorCategory::Syntax.description(),
            "Malformed code generation"
        );
        assert_eq!(ErrorCategory::Other.description(), "Uncategorized error");
    }

    #[test]
    fn test_rust_error_parse() {
        let line = "error[E0308]: mismatched types";
        let error = RustError::parse(line).unwrap();

        assert_eq!(error.code, "E0308");
        assert_eq!(error.message, "mismatched types");
        assert_eq!(error.category, ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_rust_error_parse_invalid() {
        assert!(RustError::parse("warning: unused variable").is_none());
        assert!(RustError::parse("").is_none());
    }

    #[test]
    fn test_rust_error_parse_malformed() {
        // Missing closing bracket
        assert!(RustError::parse("error[E0308 mismatched types").is_none());
        // Missing colon
        assert!(RustError::parse("error[E0308] mismatched types").is_none());
    }

    #[test]
    fn test_parse_multiple_errors() {
        let output = r#"error[E0308]: mismatched types
note: some note
error[E0412]: cannot find type `T` in this scope
warning: unused variable"#;

        let errors = RustError::parse_all(output);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].code, "E0308");
        assert_eq!(errors[1].code, "E0412");
    }

    #[test]
    fn test_blocker_priority_from_frequency() {
        // P0: >20% or >=50 occurrences
        assert_eq!(
            BlockerPriority::from_frequency(50, 200),
            BlockerPriority::P0Critical
        );
        assert_eq!(
            BlockerPriority::from_frequency(50, 100),
            BlockerPriority::P0Critical
        );

        // P1: >10% or >=20 occurrences
        assert_eq!(
            BlockerPriority::from_frequency(20, 200),
            BlockerPriority::P1High
        );

        // P2: >5% or >=10 occurrences
        assert_eq!(
            BlockerPriority::from_frequency(10, 200),
            BlockerPriority::P2Medium
        );

        // P3: <5% and <10 occurrences
        assert_eq!(
            BlockerPriority::from_frequency(5, 200),
            BlockerPriority::P3Low
        );
    }

    #[test]
    fn test_blocker_priority_from_frequency_zero_total() {
        // Edge case: total == 0 should return P3Low
        assert_eq!(
            BlockerPriority::from_frequency(10, 0),
            BlockerPriority::P3Low
        );
    }

    #[test]
    fn test_blocker_priority_from_frequency_percentage_thresholds() {
        // Test boundary cases for percentage-based thresholds
        // P0: >20%
        assert_eq!(
            BlockerPriority::from_frequency(21, 100),
            BlockerPriority::P0Critical
        );
        // P1: 10-20% (percentage > 10)
        assert_eq!(
            BlockerPriority::from_frequency(15, 100),
            BlockerPriority::P1High
        );
        // P2: 5-10% (percentage > 5)
        assert_eq!(
            BlockerPriority::from_frequency(6, 100),
            BlockerPriority::P2Medium
        );
    }

    #[test]
    fn test_error_category_description() {
        assert!(!ErrorCategory::TypeMismatch.description().is_empty());
        assert!(!ErrorCategory::Other.description().is_empty());
    }

    fn make_result(stderr: Option<&str>) -> CompilationResult {
        CompilationResult {
            rust_file: PathBuf::from("test.rs"),
            python_file: PathBuf::from("test.py"),
            success: stderr.is_none(),
            exit_code: if stderr.is_none() { Some(0) } else { Some(1) },
            stderr: stderr.map(String::from),
            stdout: None,
            duration: Duration::from_millis(100),
            cargo_first: false,
        }
    }

    #[test]
    fn test_error_taxonomy_classify_empty() {
        let results: Vec<CompilationResult> = vec![];
        let taxonomy = ErrorTaxonomy::classify(&results);
        assert!(taxonomy.errors.is_empty());
        assert!(taxonomy.by_category.is_empty());
        assert!(taxonomy.by_code.is_empty());
        assert!(taxonomy.blockers.is_empty());
    }

    #[test]
    fn test_error_taxonomy_classify_with_errors() {
        let results = vec![
            make_result(Some("error[E0308]: mismatched types")),
            make_result(Some("error[E0308]: mismatched types\nerror[E0412]: cannot find type")),
            make_result(None), // Success case
        ];

        let taxonomy = ErrorTaxonomy::classify(&results);

        assert_eq!(taxonomy.errors.len(), 3);
        assert_eq!(
            taxonomy.by_category.get(&ErrorCategory::TypeMismatch),
            Some(&2)
        );
        assert_eq!(
            taxonomy.by_category.get(&ErrorCategory::UndefinedType),
            Some(&1)
        );
        assert_eq!(taxonomy.by_code.get("E0308"), Some(&2));
        assert_eq!(taxonomy.by_code.get("E0412"), Some(&1));
    }

    #[test]
    fn test_error_taxonomy_blockers_by_priority() {
        let results = vec![
            make_result(Some("error[E0308]: mismatched types")),
            make_result(Some("error[E0308]: mismatched types")),
        ];

        let taxonomy = ErrorTaxonomy::classify(&results);

        // With 2 errors out of 2 results, this is >20% so P0Critical
        let critical = taxonomy.blockers_by_priority(BlockerPriority::P0Critical);
        assert!(!critical.is_empty());

        // P1, P2, P3 should be empty
        let high = taxonomy.blockers_by_priority(BlockerPriority::P1High);
        let medium = taxonomy.blockers_by_priority(BlockerPriority::P2Medium);
        let low = taxonomy.blockers_by_priority(BlockerPriority::P3Low);
        assert!(high.is_empty());
        assert!(medium.is_empty());
        assert!(low.is_empty());
    }

    #[test]
    fn test_error_taxonomy_suggest_fix_all_codes() {
        // Test that suggest_fix returns meaningful suggestions for known codes
        assert!(ErrorTaxonomy::suggest_fix("E0308").contains("inference"));
        assert!(ErrorTaxonomy::suggest_fix("E0412").contains("generic"));
        assert!(ErrorTaxonomy::suggest_fix("E0425").contains("import"));
        assert!(ErrorTaxonomy::suggest_fix("E0282").contains("annotation"));
        assert!(ErrorTaxonomy::suggest_fix("E0277").contains("trait"));
        // Unknown code
        assert!(ErrorTaxonomy::suggest_fix("E9999").contains("Investigate"));
    }

    #[test]
    fn test_blocker_info_fields() {
        let results = vec![make_result(Some("error[E0308]: mismatched types"))];
        let taxonomy = ErrorTaxonomy::classify(&results);

        assert!(!taxonomy.blockers.is_empty());
        let blocker = &taxonomy.blockers[0];
        assert_eq!(blocker.error_code, "E0308");
        assert_eq!(blocker.count, 1);
        assert_eq!(blocker.category, ErrorCategory::TypeMismatch);
        assert!(!blocker.root_cause.is_empty());
        assert!(!blocker.recommended_fix.is_empty());
    }

    #[test]
    fn test_blocker_priority_ordering() {
        // Test that PartialOrd works correctly (P0 is first/lowest in enum order)
        assert!(BlockerPriority::P0Critical < BlockerPriority::P1High);
        assert!(BlockerPriority::P1High < BlockerPriority::P2Medium);
        assert!(BlockerPriority::P2Medium < BlockerPriority::P3Low);
    }

    #[test]
    fn test_rust_error_fields() {
        let error = RustError {
            code: "E0308".to_string(),
            message: "mismatched types".to_string(),
            file: "test.rs".to_string(),
            line: Some(42),
            category: ErrorCategory::TypeMismatch,
        };

        assert_eq!(error.code, "E0308");
        assert_eq!(error.file, "test.rs");
        assert_eq!(error.line, Some(42));
    }

    #[test]
    fn test_error_category_serde() {
        let cat = ErrorCategory::TypeMismatch;
        let json = serde_json::to_string(&cat).unwrap();
        assert!(json.contains("TYPE_MISMATCH"));
        let deserialized: ErrorCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, cat);
    }

    #[test]
    fn test_blocker_priority_serde() {
        let priority = BlockerPriority::P0Critical;
        let json = serde_json::to_string(&priority).unwrap();
        let deserialized: BlockerPriority = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, priority);
    }

    #[test]
    fn test_blockers_sorted_by_count() {
        let results = vec![
            make_result(Some("error[E0308]: a\nerror[E0308]: b\nerror[E0308]: c")),
            make_result(Some("error[E0412]: x")),
        ];

        let taxonomy = ErrorTaxonomy::classify(&results);

        // Blockers should be sorted by count descending
        assert_eq!(taxonomy.blockers[0].error_code, "E0308");
        assert_eq!(taxonomy.blockers[0].count, 3);
        assert_eq!(taxonomy.blockers[1].error_code, "E0412");
        assert_eq!(taxonomy.blockers[1].count, 1);
    }
}

//! Pure oracle helper functions - EXTREME TDD
//!
//! DEPYLER-COVERAGE-95: Extracted from lib.rs for testability
//! Contains pure functions for oracle classification and error handling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error classification categories for oracle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OracleCategory {
    TypeMismatch,
    BorrowChecker,
    LifetimeError,
    TraitBound,
    ImportError,
    MethodNotFound,
    SyntaxError,
    UnknownError,
}

impl OracleCategory {
    /// Get category from error code
    pub fn from_error_code(code: &str) -> Self {
        match code {
            "E0308" | "E0277" | "E0061" => OracleCategory::TypeMismatch,
            "E0382" | "E0502" | "E0503" | "E0597" => OracleCategory::BorrowChecker,
            "E0106" | "E0495" => OracleCategory::LifetimeError,
            "E0412" | "E0433" | "E0425" | "E0583" => OracleCategory::ImportError,
            "E0599" | "E0609" => OracleCategory::MethodNotFound,
            _ if code.starts_with('E') && code.len() == 5 => OracleCategory::UnknownError,
            _ => OracleCategory::SyntaxError,
        }
    }

    /// Get human-readable name
    pub fn as_str(&self) -> &'static str {
        match self {
            OracleCategory::TypeMismatch => "Type Mismatch",
            OracleCategory::BorrowChecker => "Borrow Checker",
            OracleCategory::LifetimeError => "Lifetime Error",
            OracleCategory::TraitBound => "Trait Bound",
            OracleCategory::ImportError => "Import Error",
            OracleCategory::MethodNotFound => "Method Not Found",
            OracleCategory::SyntaxError => "Syntax Error",
            OracleCategory::UnknownError => "Unknown Error",
        }
    }

    /// Get suggested fix approach
    pub fn fix_hint(&self) -> &'static str {
        match self {
            OracleCategory::TypeMismatch => "Check type annotations and conversions",
            OracleCategory::BorrowChecker => "Review ownership and borrowing patterns",
            OracleCategory::LifetimeError => "Add explicit lifetime annotations",
            OracleCategory::TraitBound => "Implement required traits or add bounds",
            OracleCategory::ImportError => "Check imports and module paths",
            OracleCategory::MethodNotFound => "Verify method exists for type",
            OracleCategory::SyntaxError => "Fix syntax errors",
            OracleCategory::UnknownError => "Review error message for details",
        }
    }
}

/// Confidence level for oracle predictions
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Confidence(f64);

impl Confidence {
    /// Create a new confidence value (clamped to 0.0-1.0)
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Get the raw value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Check if confidence is high (>= 0.8)
    pub fn is_high(&self) -> bool {
        self.0 >= 0.8
    }

    /// Check if confidence is medium (>= 0.5)
    pub fn is_medium(&self) -> bool {
        self.0 >= 0.5
    }

    /// Check if confidence is low (< 0.5)
    pub fn is_low(&self) -> bool {
        self.0 < 0.5
    }

    /// Get confidence as percentage
    pub fn as_percent(&self) -> f64 {
        self.0 * 100.0
    }

    /// Get confidence level description
    pub fn level(&self) -> &'static str {
        if self.is_high() {
            "High"
        } else if self.is_medium() {
            "Medium"
        } else {
            "Low"
        }
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Self(0.5)
    }
}

/// Oracle classification result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub category: OracleCategory,
    pub confidence: f64,
    pub error_code: Option<String>,
    pub suggested_fix: Option<String>,
}

impl ClassificationResult {
    /// Create a new classification result
    pub fn new(category: OracleCategory, confidence: f64) -> Self {
        Self {
            category,
            confidence: confidence.clamp(0.0, 1.0),
            error_code: None,
            suggested_fix: Some(category.fix_hint().to_string()),
        }
    }

    /// Add error code
    pub fn with_error_code(mut self, code: impl Into<String>) -> Self {
        self.error_code = Some(code.into());
        self
    }

    /// Add suggested fix
    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.suggested_fix = Some(fix.into());
        self
    }
}

/// Classify an error by keywords in the message
pub fn classify_by_keywords(error_message: &str) -> OracleCategory {
    let message_lower = error_message.to_lowercase();

    if message_lower.contains("mismatched types") || message_lower.contains("expected") && message_lower.contains("found") {
        OracleCategory::TypeMismatch
    } else if message_lower.contains("borrow") || message_lower.contains("moved") || message_lower.contains("cannot move") {
        OracleCategory::BorrowChecker
    } else if message_lower.contains("lifetime") || message_lower.contains("'a") || message_lower.contains("'static") {
        OracleCategory::LifetimeError
    } else if message_lower.contains("trait") && message_lower.contains("not satisfied") {
        OracleCategory::TraitBound
    } else if message_lower.contains("cannot find") || message_lower.contains("not found") || message_lower.contains("unresolved") {
        OracleCategory::ImportError
    } else if message_lower.contains("no method") || message_lower.contains("no field") {
        OracleCategory::MethodNotFound
    } else {
        OracleCategory::UnknownError
    }
}

/// Extract error code from error message
pub fn extract_error_code(message: &str) -> Option<String> {
    // Pattern: E followed by 4 digits
    for (i, c) in message.char_indices() {
        if c == 'E' && message.len() >= i + 5 {
            let candidate = &message[i..i + 5];
            if candidate.chars().skip(1).all(|d| d.is_ascii_digit()) {
                return Some(candidate.to_string());
            }
        }
    }
    None
}

/// Parse multiple error codes from a message
pub fn extract_all_error_codes(message: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = message.chars().collect();

    while i < chars.len() {
        if chars[i] == 'E' && i + 4 < chars.len() {
            if chars[i + 1..i + 5].iter().all(|c| c.is_ascii_digit()) {
                codes.push(chars[i..i + 5].iter().collect());
                i += 5;
                continue;
            }
        }
        i += 1;
    }

    codes
}

/// Count error occurrences by category
pub fn count_by_category(errors: &[ClassificationResult]) -> HashMap<OracleCategory, usize> {
    let mut counts = HashMap::new();
    for error in errors {
        *counts.entry(error.category).or_insert(0) += 1;
    }
    counts
}

/// Get the most common error category
pub fn most_common_category(errors: &[ClassificationResult]) -> Option<OracleCategory> {
    let counts = count_by_category(errors);
    counts.into_iter().max_by_key(|(_, count)| *count).map(|(cat, _)| cat)
}

/// Calculate average confidence
pub fn average_confidence(results: &[ClassificationResult]) -> f64 {
    if results.is_empty() {
        return 0.0;
    }
    let sum: f64 = results.iter().map(|r| r.confidence).sum();
    sum / results.len() as f64
}

/// Filter results by minimum confidence
pub fn filter_by_confidence(results: &[ClassificationResult], min_confidence: f64) -> Vec<&ClassificationResult> {
    results.iter().filter(|r| r.confidence >= min_confidence).collect()
}

/// Oracle training statistics
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TrainingStats {
    pub total_samples: usize,
    pub correct_predictions: usize,
    pub accuracy: f64,
    pub category_accuracies: HashMap<String, f64>,
}

impl TrainingStats {
    /// Calculate accuracy from predictions
    pub fn from_predictions(correct: usize, total: usize) -> Self {
        let accuracy = if total > 0 {
            correct as f64 / total as f64
        } else {
            0.0
        };

        Self {
            total_samples: total,
            correct_predictions: correct,
            accuracy,
            category_accuracies: HashMap::new(),
        }
    }

    /// Check if accuracy meets threshold
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.accuracy >= threshold
    }

    /// Get accuracy as percentage
    pub fn accuracy_percent(&self) -> f64 {
        self.accuracy * 100.0
    }
}

/// Oracle configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OracleConfig {
    pub min_confidence: f64,
    pub use_moe: bool,
    pub use_keyword_fallback: bool,
    pub max_suggestions: usize,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.5,
            use_moe: true,
            use_keyword_fallback: true,
            max_suggestions: 3,
        }
    }
}

impl OracleConfig {
    /// Create a high-confidence config
    pub fn high_confidence() -> Self {
        Self {
            min_confidence: 0.8,
            ..Default::default()
        }
    }

    /// Create a fallback-only config
    pub fn fallback_only() -> Self {
        Self {
            use_moe: false,
            use_keyword_fallback: true,
            min_confidence: 0.0,
            max_suggestions: 5,
        }
    }
}

/// Format classification result for display
pub fn format_classification(result: &ClassificationResult) -> String {
    let mut output = String::new();

    output.push_str(&format!("Category: {}\n", result.category.as_str()));
    output.push_str(&format!("Confidence: {:.1}%\n", result.confidence * 100.0));

    if let Some(code) = &result.error_code {
        output.push_str(&format!("Error Code: {}\n", code));
    }

    if let Some(fix) = &result.suggested_fix {
        output.push_str(&format!("Suggested Fix: {}\n", fix));
    }

    output
}

/// Batch classify errors
pub fn batch_classify(error_messages: &[&str]) -> Vec<ClassificationResult> {
    error_messages
        .iter()
        .map(|msg| {
            let code = extract_error_code(msg);
            let category = code
                .as_ref()
                .map(|c| OracleCategory::from_error_code(c))
                .unwrap_or_else(|| classify_by_keywords(msg));

            ClassificationResult::new(category, 0.7)
                .with_error_code(code.unwrap_or_default())
        })
        .collect()
}

/// Priority for fixing errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FixPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl FixPriority {
    /// Get priority from category
    pub fn from_category(category: OracleCategory) -> Self {
        match category {
            OracleCategory::SyntaxError => FixPriority::Critical,
            OracleCategory::ImportError => FixPriority::High,
            OracleCategory::TypeMismatch => FixPriority::High,
            OracleCategory::BorrowChecker => FixPriority::Medium,
            OracleCategory::LifetimeError => FixPriority::Medium,
            OracleCategory::TraitBound => FixPriority::Medium,
            OracleCategory::MethodNotFound => FixPriority::Low,
            OracleCategory::UnknownError => FixPriority::Low,
        }
    }

    /// Get priority name
    pub fn as_str(&self) -> &'static str {
        match self {
            FixPriority::Critical => "Critical",
            FixPriority::High => "High",
            FixPriority::Medium => "Medium",
            FixPriority::Low => "Low",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== OracleCategory tests ====================

    #[test]
    fn test_category_from_type_mismatch_codes() {
        assert_eq!(OracleCategory::from_error_code("E0308"), OracleCategory::TypeMismatch);
        assert_eq!(OracleCategory::from_error_code("E0061"), OracleCategory::TypeMismatch);
    }

    #[test]
    fn test_category_from_borrow_checker_codes() {
        assert_eq!(OracleCategory::from_error_code("E0382"), OracleCategory::BorrowChecker);
        assert_eq!(OracleCategory::from_error_code("E0502"), OracleCategory::BorrowChecker);
        assert_eq!(OracleCategory::from_error_code("E0503"), OracleCategory::BorrowChecker);
        assert_eq!(OracleCategory::from_error_code("E0597"), OracleCategory::BorrowChecker);
    }

    #[test]
    fn test_category_from_lifetime_codes() {
        assert_eq!(OracleCategory::from_error_code("E0106"), OracleCategory::LifetimeError);
        assert_eq!(OracleCategory::from_error_code("E0495"), OracleCategory::LifetimeError);
    }

    #[test]
    fn test_category_from_import_codes() {
        assert_eq!(OracleCategory::from_error_code("E0412"), OracleCategory::ImportError);
        assert_eq!(OracleCategory::from_error_code("E0433"), OracleCategory::ImportError);
        assert_eq!(OracleCategory::from_error_code("E0425"), OracleCategory::ImportError);
    }

    #[test]
    fn test_category_from_method_not_found() {
        assert_eq!(OracleCategory::from_error_code("E0599"), OracleCategory::MethodNotFound);
        assert_eq!(OracleCategory::from_error_code("E0609"), OracleCategory::MethodNotFound);
    }

    #[test]
    fn test_category_from_unknown() {
        assert_eq!(OracleCategory::from_error_code("E9999"), OracleCategory::UnknownError);
    }

    #[test]
    fn test_category_from_non_error_code() {
        assert_eq!(OracleCategory::from_error_code("syntax"), OracleCategory::SyntaxError);
    }

    #[test]
    fn test_category_as_str() {
        assert_eq!(OracleCategory::TypeMismatch.as_str(), "Type Mismatch");
        assert_eq!(OracleCategory::BorrowChecker.as_str(), "Borrow Checker");
    }

    #[test]
    fn test_category_fix_hint() {
        assert!(OracleCategory::TypeMismatch.fix_hint().contains("type"));
        assert!(OracleCategory::BorrowChecker.fix_hint().contains("ownership"));
    }

    // ==================== Confidence tests ====================

    #[test]
    fn test_confidence_new_clamped() {
        assert_eq!(Confidence::new(1.5).value(), 1.0);
        assert_eq!(Confidence::new(-0.5).value(), 0.0);
        assert_eq!(Confidence::new(0.75).value(), 0.75);
    }

    #[test]
    fn test_confidence_is_high() {
        assert!(Confidence::new(0.9).is_high());
        assert!(Confidence::new(0.8).is_high());
        assert!(!Confidence::new(0.79).is_high());
    }

    #[test]
    fn test_confidence_is_medium() {
        assert!(Confidence::new(0.5).is_medium());
        assert!(Confidence::new(0.7).is_medium());
        assert!(!Confidence::new(0.49).is_medium());
    }

    #[test]
    fn test_confidence_is_low() {
        assert!(Confidence::new(0.3).is_low());
        assert!(!Confidence::new(0.5).is_low());
    }

    #[test]
    fn test_confidence_as_percent() {
        assert_eq!(Confidence::new(0.5).as_percent(), 50.0);
        assert_eq!(Confidence::new(1.0).as_percent(), 100.0);
    }

    #[test]
    fn test_confidence_level() {
        assert_eq!(Confidence::new(0.9).level(), "High");
        assert_eq!(Confidence::new(0.6).level(), "Medium");
        assert_eq!(Confidence::new(0.3).level(), "Low");
    }

    // ==================== ClassificationResult tests ====================

    #[test]
    fn test_classification_result_new() {
        let result = ClassificationResult::new(OracleCategory::TypeMismatch, 0.8);
        assert_eq!(result.category, OracleCategory::TypeMismatch);
        assert_eq!(result.confidence, 0.8);
        assert!(result.suggested_fix.is_some());
    }

    #[test]
    fn test_classification_result_with_error_code() {
        let result = ClassificationResult::new(OracleCategory::TypeMismatch, 0.8)
            .with_error_code("E0308");
        assert_eq!(result.error_code, Some("E0308".to_string()));
    }

    #[test]
    fn test_classification_result_with_fix() {
        let result = ClassificationResult::new(OracleCategory::TypeMismatch, 0.8)
            .with_fix("Custom fix");
        assert_eq!(result.suggested_fix, Some("Custom fix".to_string()));
    }

    #[test]
    fn test_classification_result_confidence_clamped() {
        let result = ClassificationResult::new(OracleCategory::TypeMismatch, 1.5);
        assert_eq!(result.confidence, 1.0);
    }

    // ==================== classify_by_keywords tests ====================

    #[test]
    fn test_classify_type_mismatch_keywords() {
        assert_eq!(
            classify_by_keywords("mismatched types: expected i32, found String"),
            OracleCategory::TypeMismatch
        );
    }

    #[test]
    fn test_classify_borrow_checker_keywords() {
        assert_eq!(
            classify_by_keywords("cannot borrow `x` as mutable"),
            OracleCategory::BorrowChecker
        );
        assert_eq!(
            classify_by_keywords("value moved here"),
            OracleCategory::BorrowChecker
        );
    }

    #[test]
    fn test_classify_lifetime_keywords() {
        assert_eq!(
            classify_by_keywords("missing lifetime specifier"),
            OracleCategory::LifetimeError
        );
    }

    #[test]
    fn test_classify_trait_bound_keywords() {
        assert_eq!(
            classify_by_keywords("the trait bound `T: Clone` is not satisfied"),
            OracleCategory::TraitBound
        );
    }

    #[test]
    fn test_classify_import_keywords() {
        assert_eq!(
            classify_by_keywords("cannot find type `HashMap` in this scope"),
            OracleCategory::ImportError
        );
    }

    #[test]
    fn test_classify_method_not_found_keywords() {
        assert_eq!(
            classify_by_keywords("no method named `foo` found"),
            OracleCategory::MethodNotFound
        );
    }

    // ==================== extract_error_code tests ====================

    #[test]
    fn test_extract_error_code_standard() {
        assert_eq!(extract_error_code("error[E0308]: mismatched"), Some("E0308".to_string()));
    }

    #[test]
    fn test_extract_error_code_in_text() {
        assert_eq!(extract_error_code("see E0277 for help"), Some("E0277".to_string()));
    }

    #[test]
    fn test_extract_error_code_none() {
        assert_eq!(extract_error_code("no error code here"), None);
    }

    #[test]
    fn test_extract_all_error_codes() {
        let codes = extract_all_error_codes("E0308 and E0382 found");
        assert_eq!(codes.len(), 2);
        assert!(codes.contains(&"E0308".to_string()));
        assert!(codes.contains(&"E0382".to_string()));
    }

    #[test]
    fn test_extract_all_error_codes_empty() {
        let codes = extract_all_error_codes("no codes");
        assert!(codes.is_empty());
    }

    // ==================== count_by_category tests ====================

    #[test]
    fn test_count_by_category() {
        let results = vec![
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.8),
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.7),
            ClassificationResult::new(OracleCategory::BorrowChecker, 0.9),
        ];
        let counts = count_by_category(&results);
        assert_eq!(counts.get(&OracleCategory::TypeMismatch), Some(&2));
        assert_eq!(counts.get(&OracleCategory::BorrowChecker), Some(&1));
    }

    #[test]
    fn test_most_common_category() {
        let results = vec![
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.8),
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.7),
            ClassificationResult::new(OracleCategory::BorrowChecker, 0.9),
        ];
        assert_eq!(most_common_category(&results), Some(OracleCategory::TypeMismatch));
    }

    #[test]
    fn test_most_common_category_empty() {
        let results: Vec<ClassificationResult> = vec![];
        assert_eq!(most_common_category(&results), None);
    }

    // ==================== average_confidence tests ====================

    #[test]
    fn test_average_confidence() {
        let results = vec![
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.8),
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.6),
        ];
        assert!((average_confidence(&results) - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_average_confidence_empty() {
        let results: Vec<ClassificationResult> = vec![];
        assert_eq!(average_confidence(&results), 0.0);
    }

    // ==================== filter_by_confidence tests ====================

    #[test]
    fn test_filter_by_confidence() {
        let results = vec![
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.8),
            ClassificationResult::new(OracleCategory::TypeMismatch, 0.4),
            ClassificationResult::new(OracleCategory::BorrowChecker, 0.9),
        ];
        let filtered = filter_by_confidence(&results, 0.5);
        assert_eq!(filtered.len(), 2);
    }

    // ==================== TrainingStats tests ====================

    #[test]
    fn test_training_stats_from_predictions() {
        let stats = TrainingStats::from_predictions(80, 100);
        assert_eq!(stats.total_samples, 100);
        assert_eq!(stats.correct_predictions, 80);
        assert_eq!(stats.accuracy, 0.8);
    }

    #[test]
    fn test_training_stats_zero_samples() {
        let stats = TrainingStats::from_predictions(0, 0);
        assert_eq!(stats.accuracy, 0.0);
    }

    #[test]
    fn test_training_stats_meets_threshold() {
        let stats = TrainingStats::from_predictions(80, 100);
        assert!(stats.meets_threshold(0.8));
        assert!(stats.meets_threshold(0.7));
        assert!(!stats.meets_threshold(0.9));
    }

    #[test]
    fn test_training_stats_accuracy_percent() {
        let stats = TrainingStats::from_predictions(75, 100);
        assert_eq!(stats.accuracy_percent(), 75.0);
    }

    // ==================== OracleConfig tests ====================

    #[test]
    fn test_oracle_config_default() {
        let config = OracleConfig::default();
        assert_eq!(config.min_confidence, 0.5);
        assert!(config.use_moe);
        assert!(config.use_keyword_fallback);
    }

    #[test]
    fn test_oracle_config_high_confidence() {
        let config = OracleConfig::high_confidence();
        assert_eq!(config.min_confidence, 0.8);
    }

    #[test]
    fn test_oracle_config_fallback_only() {
        let config = OracleConfig::fallback_only();
        assert!(!config.use_moe);
        assert!(config.use_keyword_fallback);
    }

    // ==================== format_classification tests ====================

    #[test]
    fn test_format_classification() {
        let result = ClassificationResult::new(OracleCategory::TypeMismatch, 0.85)
            .with_error_code("E0308");
        let output = format_classification(&result);
        assert!(output.contains("Type Mismatch"));
        assert!(output.contains("85.0%"));
        assert!(output.contains("E0308"));
    }

    // ==================== batch_classify tests ====================

    #[test]
    fn test_batch_classify() {
        let messages = vec![
            "error[E0308]: mismatched types",
            "error[E0382]: use of moved value",
        ];
        let results = batch_classify(&messages);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].category, OracleCategory::TypeMismatch);
        assert_eq!(results[1].category, OracleCategory::BorrowChecker);
    }

    #[test]
    fn test_batch_classify_empty() {
        let messages: Vec<&str> = vec![];
        let results = batch_classify(&messages);
        assert!(results.is_empty());
    }

    // ==================== FixPriority tests ====================

    #[test]
    fn test_fix_priority_from_category() {
        assert_eq!(FixPriority::from_category(OracleCategory::SyntaxError), FixPriority::Critical);
        assert_eq!(FixPriority::from_category(OracleCategory::ImportError), FixPriority::High);
        assert_eq!(FixPriority::from_category(OracleCategory::BorrowChecker), FixPriority::Medium);
        assert_eq!(FixPriority::from_category(OracleCategory::UnknownError), FixPriority::Low);
    }

    #[test]
    fn test_fix_priority_ordering() {
        assert!(FixPriority::Critical < FixPriority::High);
        assert!(FixPriority::High < FixPriority::Medium);
        assert!(FixPriority::Medium < FixPriority::Low);
    }

    #[test]
    fn test_fix_priority_as_str() {
        assert_eq!(FixPriority::Critical.as_str(), "Critical");
        assert_eq!(FixPriority::Low.as_str(), "Low");
    }

    // ==================== Edge cases ====================

    #[test]
    fn test_classify_empty_message() {
        assert_eq!(classify_by_keywords(""), OracleCategory::UnknownError);
    }

    #[test]
    fn test_extract_code_short_message() {
        assert_eq!(extract_error_code("E03"), None); // Too short
    }

    #[test]
    fn test_confidence_default() {
        assert_eq!(Confidence::default().value(), 0.5);
    }
}

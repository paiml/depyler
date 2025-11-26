//! Error classification types and logic.

use serde::{Deserialize, Serialize};

/// Categories of transpilation errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Type mismatch errors (e.g., expected i32, found &str)
    TypeMismatch,
    /// Borrow checker violations
    BorrowChecker,
    /// Missing import or use statement
    MissingImport,
    /// Syntax errors
    SyntaxError,
    /// Lifetime annotation errors
    LifetimeError,
    /// Trait bound not satisfied
    TraitBound,
    /// Uncategorized errors
    Other,
}

impl ErrorCategory {
    /// Get human-readable name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::TypeMismatch => "Type Mismatch",
            Self::BorrowChecker => "Borrow Checker",
            Self::MissingImport => "Missing Import",
            Self::SyntaxError => "Syntax Error",
            Self::LifetimeError => "Lifetime Error",
            Self::TraitBound => "Trait Bound",
            Self::Other => "Other",
        }
    }

    /// Get category index for ML model.
    #[must_use]
    pub fn index(&self) -> usize {
        match self {
            Self::TypeMismatch => 0,
            Self::BorrowChecker => 1,
            Self::MissingImport => 2,
            Self::SyntaxError => 3,
            Self::LifetimeError => 4,
            Self::TraitBound => 5,
            Self::Other => 6,
        }
    }

    /// Create from index.
    #[must_use]
    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => Self::TypeMismatch,
            1 => Self::BorrowChecker,
            2 => Self::MissingImport,
            3 => Self::SyntaxError,
            4 => Self::LifetimeError,
            5 => Self::TraitBound,
            _ => Self::Other,
        }
    }

    /// All categories.
    #[must_use]
    pub fn all() -> &'static [ErrorCategory] {
        &[
            Self::TypeMismatch,
            Self::BorrowChecker,
            Self::MissingImport,
            Self::SyntaxError,
            Self::LifetimeError,
            Self::TraitBound,
            Self::Other,
        ]
    }
}

/// Error classifier using rule-based and ML approaches.
pub struct ErrorClassifier {
    /// Keywords indicating type mismatch
    type_keywords: Vec<&'static str>,
    /// Keywords indicating borrow issues
    borrow_keywords: Vec<&'static str>,
    /// Keywords indicating missing imports
    import_keywords: Vec<&'static str>,
    /// Keywords indicating lifetime issues
    lifetime_keywords: Vec<&'static str>,
    /// Keywords indicating trait bound issues
    trait_keywords: Vec<&'static str>,
}

impl ErrorClassifier {
    /// Create a new classifier.
    #[must_use]
    pub fn new() -> Self {
        Self {
            type_keywords: vec![
                "expected", "found", "mismatched types", "type mismatch",
                "cannot coerce", "incompatible types",
            ],
            borrow_keywords: vec![
                "borrow", "borrowed", "move", "moved", "cannot move",
                "value used after move", "ownership",
            ],
            import_keywords: vec![
                "not found", "unresolved", "cannot find",
                "no such", "undefined", "use of undeclared",
            ],
            lifetime_keywords: vec![
                "lifetime", "'a", "'static", "does not live long enough",
                "borrowed value", "dangling",
            ],
            trait_keywords: vec![
                "trait", "impl", "not implemented", "bound",
                "doesn't implement", "the trait bound",
            ],
        }
    }

    /// Classify an error message using keyword matching.
    #[must_use]
    pub fn classify_by_keywords(&self, message: &str) -> ErrorCategory {
        let lower = message.to_lowercase();

        // Check lifetime first (more specific)
        if self.lifetime_keywords.iter().any(|k| lower.contains(k)) {
            return ErrorCategory::LifetimeError;
        }

        // Check borrow checker
        if self.borrow_keywords.iter().any(|k| lower.contains(k)) {
            return ErrorCategory::BorrowChecker;
        }

        // Check trait bounds
        if self.trait_keywords.iter().any(|k| lower.contains(k)) {
            return ErrorCategory::TraitBound;
        }

        // Check type mismatch
        if self.type_keywords.iter().any(|k| lower.contains(k)) {
            return ErrorCategory::TypeMismatch;
        }

        // Check missing import
        if self.import_keywords.iter().any(|k| lower.contains(k)) {
            return ErrorCategory::MissingImport;
        }

        // Check for obvious syntax errors
        if lower.contains("syntax") || lower.contains("parse") || lower.contains("unexpected") {
            return ErrorCategory::SyntaxError;
        }

        ErrorCategory::Other
    }

    /// Get confidence score for classification.
    #[must_use]
    pub fn confidence(&self, message: &str, category: ErrorCategory) -> f32 {
        let lower = message.to_lowercase();

        let keywords = match category {
            ErrorCategory::TypeMismatch => &self.type_keywords,
            ErrorCategory::BorrowChecker => &self.borrow_keywords,
            ErrorCategory::MissingImport => &self.import_keywords,
            ErrorCategory::LifetimeError => &self.lifetime_keywords,
            ErrorCategory::TraitBound => &self.trait_keywords,
            ErrorCategory::SyntaxError => return if lower.contains("syntax") { 0.9 } else { 0.5 },
            ErrorCategory::Other => return 0.3,
        };

        let matches = keywords.iter().filter(|k| lower.contains(*k)).count();
        let confidence = (matches as f32 / keywords.len() as f32).min(1.0);

        // Boost if multiple keywords match
        if matches > 1 {
            (confidence * 1.2).min(0.95)
        } else {
            confidence.max(0.5)
        }
    }
}

impl Default for ErrorClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_index_roundtrip() {
        for cat in ErrorCategory::all() {
            assert_eq!(ErrorCategory::from_index(cat.index()), *cat);
        }
    }

    #[test]
    fn test_classify_type_mismatch() {
        let classifier = ErrorClassifier::new();
        let msg = "error: expected `i32`, found `&str`";
        assert_eq!(classifier.classify_by_keywords(msg), ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_classify_borrow_checker() {
        let classifier = ErrorClassifier::new();
        let msg = "error: cannot move out of borrowed content";
        assert_eq!(classifier.classify_by_keywords(msg), ErrorCategory::BorrowChecker);
    }

    #[test]
    fn test_classify_missing_import() {
        let classifier = ErrorClassifier::new();
        let msg = "error: cannot find type `HashMap` in this scope";
        assert_eq!(classifier.classify_by_keywords(msg), ErrorCategory::MissingImport);
    }

    #[test]
    fn test_classify_lifetime() {
        let classifier = ErrorClassifier::new();
        let msg = "error: `x` does not live long enough";
        assert_eq!(classifier.classify_by_keywords(msg), ErrorCategory::LifetimeError);
    }

    #[test]
    fn test_classify_trait_bound() {
        let classifier = ErrorClassifier::new();
        let msg = "error: the trait bound `Foo: Clone` is not satisfied";
        assert_eq!(classifier.classify_by_keywords(msg), ErrorCategory::TraitBound);
    }

    #[test]
    fn test_confidence_high() {
        let classifier = ErrorClassifier::new();
        let msg = "mismatched types: expected i32, found &str";
        let conf = classifier.confidence(msg, ErrorCategory::TypeMismatch);
        assert!(conf > 0.5);
    }

    #[test]
    fn test_confidence_low_for_wrong_category() {
        let classifier = ErrorClassifier::new();
        let msg = "mismatched types";
        let conf = classifier.confidence(msg, ErrorCategory::BorrowChecker);
        // Lower confidence for wrong category
        assert!(conf <= 0.7);
    }

    #[test]
    fn test_category_names() {
        assert_eq!(ErrorCategory::TypeMismatch.name(), "Type Mismatch");
        assert_eq!(ErrorCategory::BorrowChecker.name(), "Borrow Checker");
    }
}

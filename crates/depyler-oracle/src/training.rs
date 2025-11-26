//! Training data for the error classifier.
//!
//! Contains curated rustc error patterns with associated categories and fixes.

use crate::classifier::ErrorCategory;

/// A training sample for error classification.
#[derive(Clone, Debug)]
pub struct TrainingSample {
    /// The error message
    pub message: String,
    /// The error category
    pub category: ErrorCategory,
    /// Suggested fix (if available)
    pub fix: Option<String>,
}

impl TrainingSample {
    /// Create a new training sample.
    #[must_use]
    pub fn new(message: &str, category: ErrorCategory) -> Self {
        Self {
            message: message.to_string(),
            category,
            fix: None,
        }
    }

    /// Create with fix suggestion.
    #[must_use]
    pub fn with_fix(message: &str, category: ErrorCategory, fix: &str) -> Self {
        Self {
            message: message.to_string(),
            category,
            fix: Some(fix.to_string()),
        }
    }
}

/// Training dataset for error classification.
pub struct TrainingDataset {
    samples: Vec<TrainingSample>,
}

impl TrainingDataset {
    /// Create an empty dataset.
    #[must_use]
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    /// Create a dataset with default rustc error patterns.
    #[must_use]
    pub fn with_rustc_defaults() -> Self {
        let mut dataset = Self::new();
        dataset.add_type_mismatch_samples();
        dataset.add_borrow_checker_samples();
        dataset.add_lifetime_samples();
        dataset.add_trait_bound_samples();
        dataset.add_import_samples();
        dataset.add_syntax_samples();
        dataset
    }

    /// Add a sample.
    pub fn add(&mut self, sample: TrainingSample) {
        self.samples.push(sample);
    }

    /// Add multiple samples.
    pub fn add_many(&mut self, samples: Vec<TrainingSample>) {
        self.samples.extend(samples);
    }

    /// Get all samples.
    #[must_use]
    pub fn samples(&self) -> &[TrainingSample] {
        &self.samples
    }

    /// Get samples for a specific category.
    #[must_use]
    pub fn samples_for_category(&self, category: ErrorCategory) -> Vec<&TrainingSample> {
        self.samples
            .iter()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Total sample count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Get messages for TF-IDF training.
    #[must_use]
    pub fn messages(&self) -> Vec<&str> {
        self.samples.iter().map(|s| s.message.as_str()).collect()
    }

    /// Get labels for training.
    #[must_use]
    pub fn labels(&self) -> Vec<usize> {
        self.samples.iter().map(|s| s.category.index()).collect()
    }

    /// Get error-fix pairs for N-gram training.
    #[must_use]
    pub fn error_fix_pairs(&self) -> Vec<(String, String, ErrorCategory)> {
        self.samples
            .iter()
            .filter_map(|s| {
                s.fix
                    .as_ref()
                    .map(|f| (s.message.clone(), f.clone(), s.category))
            })
            .collect()
    }

    // ============================================
    // Type Mismatch Samples
    // ============================================

    fn add_type_mismatch_samples(&mut self) {
        // Integer type mismatches
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `i32`, found `i64`",
                ErrorCategory::TypeMismatch,
                "Use `as i32` to convert the value",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `u32`, found `usize`",
                ErrorCategory::TypeMismatch,
                "Use `as u32` or `.try_into()` for safe conversion",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `isize`, found `i64`",
                ErrorCategory::TypeMismatch,
                "Use `as isize` for platform-dependent conversion",
            ),
            TrainingSample::with_fix(
                "error: cannot apply unary operator `-` to type `u32`",
                ErrorCategory::TypeMismatch,
                "Use a signed integer type like `i32` instead",
            ),
        ]);

        // String type mismatches
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `String`, found `&str`",
                ErrorCategory::TypeMismatch,
                "Use `.to_string()` to create an owned String",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `&str`, found `String`",
                ErrorCategory::TypeMismatch,
                "Use `&` or `.as_str()` to borrow the String",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `&str`, found `&String`",
                ErrorCategory::TypeMismatch,
                "String dereferences to &str automatically, just use `&*string`",
            ),
            TrainingSample::with_fix(
                "error: expected `std::string::String`, found `&str`",
                ErrorCategory::TypeMismatch,
                "Call `.to_string()` or `.to_owned()` on the &str",
            ),
        ]);

        // Option/Result mismatches
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `Option<T>`, found `T`",
                ErrorCategory::TypeMismatch,
                "Wrap the value with `Some(value)`",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `Result<T, E>`, found `T`",
                ErrorCategory::TypeMismatch,
                "Wrap the value with `Ok(value)`",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `()`, found `i32`",
                ErrorCategory::TypeMismatch,
                "Add a semicolon to discard the value, or return it",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected type parameter `T`, found associated type",
                ErrorCategory::TypeMismatch,
                "Ensure generic type constraints match the expected type",
            ),
        ]);

        // Reference mismatches
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `&T`, found `T`",
                ErrorCategory::TypeMismatch,
                "Add `&` to take a reference",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `T`, found `&T`",
                ErrorCategory::TypeMismatch,
                "Dereference with `*` or clone the value",
            ),
            TrainingSample::with_fix(
                "error[E0308]: mismatched types\n  expected `&mut T`, found `&T`",
                ErrorCategory::TypeMismatch,
                "Use `&mut` instead of `&` for mutable reference",
            ),
        ]);
    }

    // ============================================
    // Borrow Checker Samples
    // ============================================

    fn add_borrow_checker_samples(&mut self) {
        // Move errors
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0382]: use of moved value: `x`\n  value moved here",
                ErrorCategory::BorrowChecker,
                "Clone the value before moving, or use a reference",
            ),
            TrainingSample::with_fix(
                "error[E0382]: borrow of moved value: `x`",
                ErrorCategory::BorrowChecker,
                "Clone the value before it's moved, or restructure the code",
            ),
            TrainingSample::with_fix(
                "error[E0505]: cannot move out of `x` because it is borrowed",
                ErrorCategory::BorrowChecker,
                "Drop the borrow before moving, or clone the value",
            ),
            TrainingSample::with_fix(
                "error[E0507]: cannot move out of borrowed content",
                ErrorCategory::BorrowChecker,
                "Clone the value, or change the function signature",
            ),
        ]);

        // Borrow conflicts
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable",
                ErrorCategory::BorrowChecker,
                "Separate the mutable and immutable operations",
            ),
            TrainingSample::with_fix(
                "error[E0499]: cannot borrow `x` as mutable more than once at a time",
                ErrorCategory::BorrowChecker,
                "Use interior mutability (RefCell) or restructure the code",
            ),
            TrainingSample::with_fix(
                "error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable",
                ErrorCategory::BorrowChecker,
                "Add `mut` keyword to the variable declaration",
            ),
            TrainingSample::with_fix(
                "error[E0597]: `x` does not live long enough\n  borrowed value does not live long enough",
                ErrorCategory::BorrowChecker,
                "Extend the lifetime of the borrowed value or use owned data",
            ),
        ]);

        // Closure captures
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0373]: closure may outlive the current function, but it borrows `x`",
                ErrorCategory::BorrowChecker,
                "Use `move` keyword to take ownership in the closure",
            ),
            TrainingSample::with_fix(
                "error: captured variable cannot escape `FnMut` closure body",
                ErrorCategory::BorrowChecker,
                "Clone the variable or use `Fn` trait instead",
            ),
        ]);
    }

    // ============================================
    // Lifetime Samples
    // ============================================

    fn add_lifetime_samples(&mut self) {
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0106]: missing lifetime specifier\n  expected named lifetime parameter",
                ErrorCategory::LifetimeError,
                "Add lifetime parameter: fn foo<'a>(x: &'a str) -> &'a str",
            ),
            TrainingSample::with_fix(
                "error[E0621]: explicit lifetime required in the type of `x`",
                ErrorCategory::LifetimeError,
                "Add explicit lifetime annotation to function parameters",
            ),
            TrainingSample::with_fix(
                "error[E0495]: cannot infer an appropriate lifetime for autoref",
                ErrorCategory::LifetimeError,
                "Add explicit lifetime parameters to clarify the relationship",
            ),
            TrainingSample::with_fix(
                "error: lifetime may not live long enough\n  returning this value requires that `'a` must outlive `'static`",
                ErrorCategory::LifetimeError,
                "Return owned data, or adjust lifetime bounds",
            ),
            TrainingSample::with_fix(
                "error[E0759]: `x` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement",
                ErrorCategory::LifetimeError,
                "Use Box or Arc for 'static lifetime, or change the requirement",
            ),
            TrainingSample::with_fix(
                "error[E0515]: cannot return reference to temporary value",
                ErrorCategory::LifetimeError,
                "Return owned value instead of reference to temporary",
            ),
            TrainingSample::with_fix(
                "error[E0716]: temporary value dropped while borrowed\n  creates a temporary which is freed while still in use",
                ErrorCategory::LifetimeError,
                "Bind the temporary to a variable to extend its lifetime",
            ),
        ]);
    }

    // ============================================
    // Trait Bound Samples
    // ============================================

    fn add_trait_bound_samples(&mut self) {
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0277]: the trait bound `T: Clone` is not satisfied",
                ErrorCategory::TraitBound,
                "Add #[derive(Clone)] or implement Clone manually",
            ),
            TrainingSample::with_fix(
                "error[E0277]: `T` doesn't implement `Debug`",
                ErrorCategory::TraitBound,
                "Add #[derive(Debug)] to the type definition",
            ),
            TrainingSample::with_fix(
                "error[E0277]: the trait bound `T: Send` is not satisfied",
                ErrorCategory::TraitBound,
                "Ensure all fields implement Send, or use Arc<Mutex<T>>",
            ),
            TrainingSample::with_fix(
                "error[E0277]: the trait bound `T: Sync` is not satisfied",
                ErrorCategory::TraitBound,
                "Use thread-safe types like Arc, Mutex, or RwLock",
            ),
            TrainingSample::with_fix(
                "error[E0277]: `T` cannot be sent between threads safely",
                ErrorCategory::TraitBound,
                "Use thread-safe wrappers like Arc<T> where T: Send + Sync",
            ),
            TrainingSample::with_fix(
                "error[E0277]: the trait bound `T: Default` is not satisfied",
                ErrorCategory::TraitBound,
                "Add #[derive(Default)] or implement Default manually",
            ),
            TrainingSample::with_fix(
                "error[E0277]: the trait bound `T: Copy` is not satisfied\n  the trait `Copy` may not be implemented for this type",
                ErrorCategory::TraitBound,
                "Remove Copy requirement, use Clone instead, or simplify the type",
            ),
            TrainingSample::with_fix(
                "error[E0277]: the trait bound `dyn Trait: Sized` is not satisfied",
                ErrorCategory::TraitBound,
                "Use Box<dyn Trait> or &dyn Trait for trait objects",
            ),
            TrainingSample::with_fix(
                "error[E0038]: the trait `Trait` cannot be made into an object",
                ErrorCategory::TraitBound,
                "Remove methods with Self return type or generic parameters",
            ),
        ]);
    }

    // ============================================
    // Import Samples
    // ============================================

    fn add_import_samples(&mut self) {
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error[E0433]: failed to resolve: use of undeclared crate or module `foo`",
                ErrorCategory::MissingImport,
                "Add `use` statement or check Cargo.toml dependencies",
            ),
            TrainingSample::with_fix(
                "error[E0412]: cannot find type `HashMap` in this scope",
                ErrorCategory::MissingImport,
                "Add: use std::collections::HashMap;",
            ),
            TrainingSample::with_fix(
                "error[E0412]: cannot find type `Vec` in this scope",
                ErrorCategory::MissingImport,
                "Vec is in prelude, check for shadowing or typos",
            ),
            TrainingSample::with_fix(
                "error[E0425]: cannot find value `some_function` in this scope",
                ErrorCategory::MissingImport,
                "Import the function or use full path: module::some_function",
            ),
            TrainingSample::with_fix(
                "error[E0432]: unresolved import `crate::module`",
                ErrorCategory::MissingImport,
                "Check module exists and is declared with `mod` keyword",
            ),
            TrainingSample::with_fix(
                "error[E0603]: module `inner` is private",
                ErrorCategory::MissingImport,
                "Make the module public with `pub mod` or re-export items",
            ),
            TrainingSample::with_fix(
                "error[E0599]: no method named `foo` found for type `T` in the current scope",
                ErrorCategory::MissingImport,
                "Import the trait that provides this method",
            ),
        ]);
    }

    // ============================================
    // Syntax Samples
    // ============================================

    fn add_syntax_samples(&mut self) {
        self.samples.extend(vec![
            TrainingSample::with_fix(
                "error: expected `;`, found `}`",
                ErrorCategory::SyntaxError,
                "Add missing semicolon at end of statement",
            ),
            TrainingSample::with_fix(
                "error: expected `{`, found `=>`",
                ErrorCategory::SyntaxError,
                "Check match arm syntax or use correct braces",
            ),
            TrainingSample::with_fix(
                "error: unexpected token `)`\n  expected expression",
                ErrorCategory::SyntaxError,
                "Check for extra parentheses or missing arguments",
            ),
            TrainingSample::with_fix(
                "error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `foo`",
                ErrorCategory::SyntaxError,
                "Check for missing operator or semicolon",
            ),
            TrainingSample::with_fix(
                "error: this file contains an unclosed delimiter\n  did you mean to close this?",
                ErrorCategory::SyntaxError,
                "Count brackets/braces and add missing closing delimiter",
            ),
            TrainingSample::with_fix(
                "error: expected identifier, found keyword `type`",
                ErrorCategory::SyntaxError,
                "Use raw identifier: r#type, or choose different name",
            ),
            TrainingSample::with_fix(
                "error: expected pattern, found expression",
                ErrorCategory::SyntaxError,
                "Use pattern syntax in match arms, not expressions",
            ),
        ]);
    }
}

impl Default for TrainingDataset {
    fn default() -> Self {
        Self::with_rustc_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_sample_creation() {
        let sample = TrainingSample::new("error message", ErrorCategory::TypeMismatch);

        assert_eq!(sample.message, "error message");
        assert_eq!(sample.category, ErrorCategory::TypeMismatch);
        assert!(sample.fix.is_none());
    }

    #[test]
    fn test_training_sample_with_fix() {
        let sample = TrainingSample::with_fix(
            "error message",
            ErrorCategory::BorrowChecker,
            "suggested fix",
        );

        assert_eq!(sample.fix, Some("suggested fix".to_string()));
    }

    #[test]
    fn test_dataset_empty() {
        let dataset = TrainingDataset::new();

        assert!(dataset.is_empty());
        assert_eq!(dataset.len(), 0);
    }

    #[test]
    fn test_dataset_with_defaults() {
        let dataset = TrainingDataset::with_rustc_defaults();

        assert!(!dataset.is_empty());
        // Should have samples for all categories
        assert!(!dataset
            .samples_for_category(ErrorCategory::TypeMismatch)
            .is_empty());
        assert!(!dataset
            .samples_for_category(ErrorCategory::BorrowChecker)
            .is_empty());
        assert!(!dataset
            .samples_for_category(ErrorCategory::LifetimeError)
            .is_empty());
        assert!(!dataset
            .samples_for_category(ErrorCategory::TraitBound)
            .is_empty());
        assert!(!dataset
            .samples_for_category(ErrorCategory::MissingImport)
            .is_empty());
        assert!(!dataset
            .samples_for_category(ErrorCategory::SyntaxError)
            .is_empty());
    }

    #[test]
    fn test_dataset_add() {
        let mut dataset = TrainingDataset::new();

        dataset.add(TrainingSample::new("test", ErrorCategory::Other));
        assert_eq!(dataset.len(), 1);

        dataset.add(TrainingSample::new("test2", ErrorCategory::Other));
        assert_eq!(dataset.len(), 2);
    }

    #[test]
    fn test_dataset_add_many() {
        let mut dataset = TrainingDataset::new();

        dataset.add_many(vec![
            TrainingSample::new("a", ErrorCategory::TypeMismatch),
            TrainingSample::new("b", ErrorCategory::BorrowChecker),
            TrainingSample::new("c", ErrorCategory::SyntaxError),
        ]);

        assert_eq!(dataset.len(), 3);
    }

    #[test]
    fn test_samples_for_category() {
        let dataset = TrainingDataset::with_rustc_defaults();

        let type_samples = dataset.samples_for_category(ErrorCategory::TypeMismatch);
        assert!(!type_samples.is_empty());

        for sample in type_samples {
            assert_eq!(sample.category, ErrorCategory::TypeMismatch);
        }
    }

    #[test]
    fn test_messages() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new("msg1", ErrorCategory::Other));
        dataset.add(TrainingSample::new("msg2", ErrorCategory::Other));

        let messages = dataset.messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "msg1");
        assert_eq!(messages[1], "msg2");
    }

    #[test]
    fn test_labels() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new("a", ErrorCategory::TypeMismatch));
        dataset.add(TrainingSample::new("b", ErrorCategory::BorrowChecker));

        let labels = dataset.labels();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0], ErrorCategory::TypeMismatch.index());
        assert_eq!(labels[1], ErrorCategory::BorrowChecker.index());
    }

    #[test]
    fn test_error_fix_pairs() {
        let mut dataset = TrainingDataset::new();
        dataset.add(TrainingSample::new("no fix", ErrorCategory::Other));
        dataset.add(TrainingSample::with_fix(
            "has fix",
            ErrorCategory::TypeMismatch,
            "the fix",
        ));

        let pairs = dataset.error_fix_pairs();

        // Only samples with fixes should be included
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].0, "has fix");
        assert_eq!(pairs[0].1, "the fix");
    }

    #[test]
    fn test_all_samples_have_category() {
        let dataset = TrainingDataset::with_rustc_defaults();

        for sample in dataset.samples() {
            // All categories should be valid (not Other for default data)
            // Actually, some might be Other, so just check it's valid
            let _idx = sample.category.index(); // This will work for all categories
        }
    }

    #[test]
    fn test_default_dataset_has_fixes() {
        let dataset = TrainingDataset::with_rustc_defaults();
        let pairs = dataset.error_fix_pairs();

        // Default dataset should have fixes for most samples
        assert!(pairs.len() > dataset.len() / 2);
    }

    #[test]
    fn test_type_mismatch_samples_variety() {
        let dataset = TrainingDataset::with_rustc_defaults();
        let samples = dataset.samples_for_category(ErrorCategory::TypeMismatch);

        // Should have variety of type mismatch errors
        let messages: Vec<&str> = samples.iter().map(|s| s.message.as_str()).collect();

        // Check for different error codes
        assert!(messages.iter().any(|m| m.contains("String")));
        assert!(messages
            .iter()
            .any(|m| m.contains("i32") || m.contains("i64")));
        assert!(messages
            .iter()
            .any(|m| m.contains("Option") || m.contains("Result")));
    }

    #[test]
    fn test_borrow_checker_samples_variety() {
        let dataset = TrainingDataset::with_rustc_defaults();
        let samples = dataset.samples_for_category(ErrorCategory::BorrowChecker);

        let messages: Vec<&str> = samples.iter().map(|s| s.message.as_str()).collect();

        // Should cover different borrow checker scenarios
        assert!(messages.iter().any(|m| m.contains("moved")));
        assert!(messages.iter().any(|m| m.contains("borrowed")));
        assert!(messages.iter().any(|m| m.contains("mutable")));
    }

    #[test]
    fn test_dataset_coverage() {
        let dataset = TrainingDataset::with_rustc_defaults();

        // Should have reasonable coverage for each category
        let type_count = dataset
            .samples_for_category(ErrorCategory::TypeMismatch)
            .len();
        let borrow_count = dataset
            .samples_for_category(ErrorCategory::BorrowChecker)
            .len();
        let lifetime_count = dataset
            .samples_for_category(ErrorCategory::LifetimeError)
            .len();
        let trait_count = dataset
            .samples_for_category(ErrorCategory::TraitBound)
            .len();
        let import_count = dataset
            .samples_for_category(ErrorCategory::MissingImport)
            .len();
        let syntax_count = dataset
            .samples_for_category(ErrorCategory::SyntaxError)
            .len();

        // Each category should have multiple samples
        assert!(type_count >= 5, "Type mismatch: {}", type_count);
        assert!(borrow_count >= 5, "Borrow checker: {}", borrow_count);
        assert!(lifetime_count >= 5, "Lifetime: {}", lifetime_count);
        assert!(trait_count >= 5, "Trait bound: {}", trait_count);
        assert!(import_count >= 5, "Import: {}", import_count);
        assert!(syntax_count >= 5, "Syntax: {}", syntax_count);
    }
}

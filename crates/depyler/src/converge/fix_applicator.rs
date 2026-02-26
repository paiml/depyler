//! Fix Applicator - DEPYLER-1305
//!
//! Bridges Oracle classifications to executable fixes. This is the "hands"
//! that allow the Oracle to apply its suggestions.
//!
//! # Architecture
//!
//! ```text
//! Oracle Classification → FixApplicator → Applied Fix
//!                               ↓
//!                     ┌─────────┴─────────┐
//!                     │                   │
//!           GeneratedRustFixer   TranspilerPatcher
//!           (immediate wins)     (permanent fixes)
//! ```
//!
//! The GeneratedRustFixer modifies the transpiled .rs files directly.
//! The TranspilerPatcher (future) modifies depyler-core source.

use super::classifier::ErrorClassification;
use super::clusterer::SuggestedFix;
use super::compiler::CompilationError;
use super::state::AppliedFix;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

/// Strategy for applying fixes
pub trait FixApplicator: Send + Sync {
    /// Apply a fix based on error classification
    fn apply_fix(
        &self,
        classification: &ErrorClassification,
        rust_source: &str,
    ) -> Result<FixApplicationResult>;

    /// Check if this applicator can handle the error
    fn can_handle(&self, classification: &ErrorClassification) -> bool;
}

/// Result of applying a fix
#[derive(Debug, Clone)]
pub struct FixApplicationResult {
    /// Whether the fix was applied
    pub applied: bool,
    /// The modified source code (if applied)
    pub modified_source: Option<String>,
    /// Description of what was done
    pub description: String,
    /// Confidence in the fix
    pub confidence: f64,
    /// Type of fix applied
    pub fix_type: FixType,
}

/// Type of fix that was applied
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixType {
    /// Fix applied to generated Rust code
    GeneratedRust,
    /// Fix applied to transpiler source (permanent)
    TranspilerPatch,
    /// No fix available
    None,
}

/// Fixes generated Rust code directly
///
/// This provides immediate wins by fixing the .rs output files.
/// These fixes are temporary and will be overwritten on next transpile.
pub struct GeneratedRustFixer {
    /// Pattern-based transformations indexed by error code
    transforms: HashMap<String, Vec<RustTransform>>,
}

/// A transformation that can be applied to Rust source
struct RustTransform {
    name: String,
    pattern: regex::Regex,
    replacement: ReplaceStrategy,
    confidence: f64,
}

/// How to replace the matched pattern
#[allow(dead_code)] // Literal reserved for future simple replacements
enum ReplaceStrategy {
    /// Simple text replacement
    Literal(String),
    /// Regex capture group replacement
    Regex(String),
    /// Custom function
    Function(fn(&str, &regex::Captures) -> String),
}

impl GeneratedRustFixer {
    /// Create a new fixer with default transforms
    pub fn new() -> Self {
        let mut transforms = HashMap::new();

        // E0308: Type mismatch fixes
        transforms.insert(
            "E0308".to_string(),
            vec![
                // Add .into() for type conversion
                RustTransform {
                    name: "add_into".to_string(),
                    pattern: regex::Regex::new(r"expected `([^`]+)`, found `([^`]+)`")
                        .expect("static regex"),
                    replacement: ReplaceStrategy::Function(add_into_conversion),
                    confidence: 0.7,
                },
                // Add .to_string() for &str → String
                RustTransform {
                    name: "add_to_string".to_string(),
                    pattern: regex::Regex::new(r"expected `String`, found `&str`")
                        .expect("static regex"),
                    replacement: ReplaceStrategy::Regex(".to_string()".to_string()),
                    confidence: 0.85,
                },
            ],
        );

        // E0599: Missing method fixes
        transforms.insert(
            "E0599".to_string(),
            vec![
                // .keys() → .as_object().unwrap().keys() for serde_json::Value
                RustTransform {
                    name: "value_keys".to_string(),
                    pattern: regex::Regex::new(r"no method named `keys` found for enum `Value`")
                        .expect("static regex"),
                    replacement: ReplaceStrategy::Function(fix_value_keys),
                    confidence: 0.9,
                },
                // .items() → .as_object().unwrap().iter() for serde_json::Value
                RustTransform {
                    name: "value_items".to_string(),
                    pattern: regex::Regex::new(r"no method named `items` found for enum `Value`")
                        .expect("static regex"),
                    replacement: ReplaceStrategy::Function(fix_value_items),
                    confidence: 0.9,
                },
            ],
        );

        // E0277: Trait bound fixes
        transforms.insert(
            "E0277".to_string(),
            vec![
                // Add .clone() for Clone bound
                RustTransform {
                    name: "add_clone".to_string(),
                    pattern: regex::Regex::new(r"the trait `Clone` is not implemented")
                        .expect("static regex"),
                    replacement: ReplaceStrategy::Regex(".clone()".to_string()),
                    confidence: 0.6,
                },
            ],
        );

        // E0382: Borrow checker fixes
        transforms.insert(
            "E0382".to_string(),
            vec![
                // Pre-compute .is_some() before move
                RustTransform {
                    name: "precompute_is_some".to_string(),
                    pattern: regex::Regex::new(r"borrow of moved value.*\.is_some\(\)")
                        .expect("static regex"),
                    replacement: ReplaceStrategy::Function(fix_precompute_is_some),
                    confidence: 0.8,
                },
            ],
        );

        Self { transforms }
    }

    /// Try to apply transforms for a specific error
    fn try_transforms(
        &self,
        error: &CompilationError,
        source: &str,
    ) -> Option<(String, String, f64)> {
        let transforms = self.transforms.get(&error.code)?;

        for transform in transforms {
            if transform.pattern.is_match(&error.message) {
                let modified = match &transform.replacement {
                    ReplaceStrategy::Literal(s) => {
                        // Simple replacement - need more context
                        Some(s.clone())
                    }
                    ReplaceStrategy::Regex(repl) => {
                        // Apply regex replacement to error line
                        let lines: Vec<&str> = source.lines().collect();
                        if error.line > 0 && error.line <= lines.len() {
                            let line_idx = error.line - 1;
                            let old_line = lines[line_idx];
                            // This is a placeholder - real impl needs context
                            let new_line = format!("{}{}", old_line, repl);
                            let mut result = lines.to_vec();
                            result[line_idx] = &new_line;
                            Some(result.join("\n"))
                        } else {
                            None
                        }
                    }
                    ReplaceStrategy::Function(f) => {
                        // Apply custom function
                        transform.pattern.captures(&error.message).map(|caps| f(source, &caps))
                    }
                };

                if let Some(new_source) = modified {
                    if new_source != source {
                        return Some((new_source, transform.name.clone(), transform.confidence));
                    }
                }
            }
        }

        None
    }
}

impl Default for GeneratedRustFixer {
    fn default() -> Self {
        Self::new()
    }
}

impl FixApplicator for GeneratedRustFixer {
    fn apply_fix(
        &self,
        classification: &ErrorClassification,
        rust_source: &str,
    ) -> Result<FixApplicationResult> {
        // Try to apply a transform
        if let Some((modified, name, confidence)) =
            self.try_transforms(&classification.error, rust_source)
        {
            return Ok(FixApplicationResult {
                applied: true,
                modified_source: Some(modified),
                description: format!(
                    "Applied '{}' for {} at line {}",
                    name, classification.error.code, classification.error.line
                ),
                confidence,
                fix_type: FixType::GeneratedRust,
            });
        }

        // No applicable fix
        Ok(FixApplicationResult {
            applied: false,
            modified_source: None,
            description: format!(
                "No fix available for {} at line {}",
                classification.error.code, classification.error.line
            ),
            confidence: 0.0,
            fix_type: FixType::None,
        })
    }

    fn can_handle(&self, classification: &ErrorClassification) -> bool {
        self.transforms.contains_key(&classification.error.code)
    }
}

// ============================================================================
// Transform Functions
// ============================================================================

/// Add .into() for type conversion
fn add_into_conversion(source: &str, _caps: &regex::Captures) -> String {
    // This is a placeholder - real implementation needs more context
    // about where to insert .into()
    source.to_string()
}

/// Fix .keys() on serde_json::Value
fn fix_value_keys(source: &str, _caps: &regex::Captures) -> String {
    // Replace .keys() with .as_object().map(|o| o.keys()).unwrap_or_default()
    source.replace(
        ".keys()",
        ".as_object().map(|o| o.keys().collect::<Vec<_>>()).unwrap_or_default()",
    )
}

/// Fix .items() on serde_json::Value
fn fix_value_items(source: &str, _caps: &regex::Captures) -> String {
    // Replace .items() with .as_object().map(|o| o.iter()).unwrap_or_default()
    source.replace(
        ".items()",
        ".as_object().map(|o| o.iter().collect::<Vec<_>>()).unwrap_or_default()",
    )
}

/// Pre-compute is_some() before value is moved
fn fix_precompute_is_some(source: &str, _caps: &regex::Captures) -> String {
    // This needs more sophisticated analysis to:
    // 1. Find the variable being moved
    // 2. Insert the pre-computation before the move
    // For now, return unchanged
    source.to_string()
}

// ============================================================================
// Apply Fix via SuggestedFix
// ============================================================================

impl SuggestedFix {
    /// Apply the fix using the appropriate applicator
    pub fn apply_with_source(
        &self,
        source: &str,
        classification: &ErrorClassification,
    ) -> Result<AppliedFix> {
        let fixer = GeneratedRustFixer::new();
        let result = fixer.apply_fix(classification, source)?;

        if result.applied {
            // Write the modified source back to the file
            if let Some(modified) = &result.modified_source {
                std::fs::write(&self.file, modified)?;
            }
        }

        Ok(AppliedFix {
            iteration: 0,
            error_code: classification.error.code.clone(),
            description: result.description,
            file_modified: self.file.clone(),
            commit_hash: None,
            verified: result.applied,
        })
    }
}

// ============================================================================
// Composite Applicator
// ============================================================================

/// Applies fixes using multiple strategies
pub struct CompositeFixApplicator {
    applicators: Vec<Box<dyn FixApplicator>>,
}

impl CompositeFixApplicator {
    /// Create with default applicators
    pub fn new() -> Self {
        Self { applicators: vec![Box::new(GeneratedRustFixer::new())] }
    }

    /// Add a custom applicator
    pub fn with_applicator(mut self, applicator: Box<dyn FixApplicator>) -> Self {
        self.applicators.push(applicator);
        self
    }

    /// Apply fixes to a batch of classifications
    pub fn apply_batch(
        &self,
        classifications: &[ErrorClassification],
        source_files: &HashMap<PathBuf, String>,
    ) -> Vec<FixApplicationResult> {
        let mut results = Vec::new();

        for classification in classifications {
            // Get the source for this error's file
            if let Some(source) = source_files.get(&classification.error.file) {
                // Try each applicator until one succeeds
                for applicator in &self.applicators {
                    if applicator.can_handle(classification) {
                        match applicator.apply_fix(classification, source) {
                            Ok(result) if result.applied => {
                                results.push(result);
                                break;
                            }
                            Ok(result) => {
                                results.push(result);
                            }
                            Err(e) => {
                                results.push(FixApplicationResult {
                                    applied: false,
                                    modified_source: None,
                                    description: format!("Error applying fix: {}", e),
                                    confidence: 0.0,
                                    fix_type: FixType::None,
                                });
                            }
                        }
                    }
                }
            }
        }

        results
    }
}

impl Default for CompositeFixApplicator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_rust_fixer_new() {
        let fixer = GeneratedRustFixer::new();
        assert!(fixer.transforms.contains_key("E0308"));
        assert!(fixer.transforms.contains_key("E0599"));
        assert!(fixer.transforms.contains_key("E0277"));
        assert!(fixer.transforms.contains_key("E0382"));
    }

    #[test]
    fn test_fix_value_keys() {
        let source = "let keys = data.keys();";
        let caps = regex::Regex::new(r"keys").unwrap().captures("keys").unwrap();
        let result = fix_value_keys(source, &caps);
        assert!(result.contains("as_object()"));
    }

    #[test]
    fn test_fix_value_items() {
        let source = "for (k, v) in data.items() {";
        let caps = regex::Regex::new(r"items").unwrap().captures("items").unwrap();
        let result = fix_value_items(source, &caps);
        assert!(result.contains("as_object()"));
    }

    #[test]
    fn test_can_handle_known_error() {
        let fixer = GeneratedRustFixer::new();
        let classification = ErrorClassification {
            error: CompilationError {
                code: "E0308".to_string(),
                message: "mismatched types".to_string(),
                file: PathBuf::from("test.rs"),
                line: 1,
                column: 1,
                ..Default::default()
            },
            category: super::super::classifier::ErrorCategory::TranspilerGap,
            subcategory: "type_inference".to_string(),
            confidence: 0.9,
            suggested_fix: None,
        };
        assert!(fixer.can_handle(&classification));
    }

    #[test]
    fn test_cannot_handle_unknown_error() {
        let fixer = GeneratedRustFixer::new();
        let classification = ErrorClassification {
            error: CompilationError {
                code: "E9999".to_string(),
                message: "unknown error".to_string(),
                file: PathBuf::from("test.rs"),
                line: 1,
                column: 1,
                ..Default::default()
            },
            category: super::super::classifier::ErrorCategory::Unknown,
            subcategory: "unknown".to_string(),
            confidence: 0.5,
            suggested_fix: None,
        };
        assert!(!fixer.can_handle(&classification));
    }

    #[test]
    fn test_composite_applicator_new() {
        let applicator = CompositeFixApplicator::new();
        assert!(!applicator.applicators.is_empty());
    }

    #[test]
    fn test_fix_application_result_default() {
        let result = FixApplicationResult {
            applied: false,
            modified_source: None,
            description: "No fix".to_string(),
            confidence: 0.0,
            fix_type: FixType::None,
        };
        assert!(!result.applied);
        assert_eq!(result.fix_type, FixType::None);
    }

    #[test]
    fn test_fix_type_variants() {
        assert_ne!(FixType::GeneratedRust, FixType::TranspilerPatch);
        assert_ne!(FixType::TranspilerPatch, FixType::None);
        assert_ne!(FixType::GeneratedRust, FixType::None);
    }

    // === apply_fix tests ===

    fn make_classification(code: &str, message: &str, line: usize) -> ErrorClassification {
        ErrorClassification {
            error: CompilationError {
                code: code.to_string(),
                message: message.to_string(),
                file: PathBuf::from("test.rs"),
                line,
                column: 1,
                ..Default::default()
            },
            category: super::super::classifier::ErrorCategory::TranspilerGap,
            subcategory: "test".to_string(),
            confidence: 0.9,
            suggested_fix: None,
        }
    }

    #[test]
    fn test_apply_fix_no_matching_transform() {
        let fixer = GeneratedRustFixer::new();
        let classification = make_classification("E9999", "unknown error", 1);
        let result = fixer.apply_fix(&classification, "fn test() {}").unwrap();
        assert!(!result.applied);
        assert!(result.modified_source.is_none());
        assert_eq!(result.fix_type, FixType::None);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_apply_fix_e0599_value_keys() {
        let fixer = GeneratedRustFixer::new();
        let classification =
            make_classification("E0599", "no method named `keys` found for enum `Value`", 1);
        let source = "let keys = data.keys();";
        let result = fixer.apply_fix(&classification, source).unwrap();
        assert!(result.applied);
        assert!(result.modified_source.is_some());
        assert_eq!(result.fix_type, FixType::GeneratedRust);
        let modified = result.modified_source.unwrap();
        assert!(modified.contains("as_object()"));
    }

    #[test]
    fn test_apply_fix_e0599_value_items() {
        let fixer = GeneratedRustFixer::new();
        let classification =
            make_classification("E0599", "no method named `items` found for enum `Value`", 1);
        let source = "for (k, v) in data.items() {";
        let result = fixer.apply_fix(&classification, source).unwrap();
        assert!(result.applied);
        let modified = result.modified_source.unwrap();
        assert!(modified.contains("as_object()"));
        assert!(modified.contains("iter()"));
    }

    #[test]
    fn test_apply_fix_description_format() {
        let fixer = GeneratedRustFixer::new();
        let classification =
            make_classification("E0599", "no method named `keys` found for enum `Value`", 5);
        let source = "let keys = data.keys();";
        let result = fixer.apply_fix(&classification, source).unwrap();
        assert!(result.description.contains("E0599"));
        assert!(result.description.contains("line 5"));
    }

    #[test]
    fn test_apply_fix_no_fix_description() {
        let fixer = GeneratedRustFixer::new();
        let classification = make_classification("E0308", "some unrecognized message", 10);
        let result = fixer.apply_fix(&classification, "fn test() {}").unwrap();
        assert!(!result.applied);
        assert!(result.description.contains("No fix available"));
        assert!(result.description.contains("E0308"));
    }

    // === add_into_conversion tests ===

    #[test]
    fn test_add_into_conversion_returns_source_unchanged() {
        let source = "let x: i32 = 42;";
        let re = regex::Regex::new(r"test").unwrap();
        let caps = re.captures("test").unwrap();
        let result = add_into_conversion(source, &caps);
        assert_eq!(result, source);
    }

    // === fix_precompute_is_some tests ===

    #[test]
    fn test_fix_precompute_is_some_returns_source_unchanged() {
        let source = "if val.is_some() { consume(val); }";
        let re = regex::Regex::new(r"test").unwrap();
        let caps = re.captures("test").unwrap();
        let result = fix_precompute_is_some(source, &caps);
        assert_eq!(result, source);
    }

    // === try_transforms tests ===

    #[test]
    fn test_try_transforms_unknown_code_returns_none() {
        let fixer = GeneratedRustFixer::new();
        let error = CompilationError {
            code: "E9999".to_string(),
            message: "unknown error".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
            ..Default::default()
        };
        let result = fixer.try_transforms(&error, "fn test() {}");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_transforms_no_pattern_match() {
        let fixer = GeneratedRustFixer::new();
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "some unrelated message".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
            ..Default::default()
        };
        let result = fixer.try_transforms(&error, "fn test() {}");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_transforms_regex_replacement() {
        let fixer = GeneratedRustFixer::new();
        let source = "let x = value";
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "expected `String`, found `&str`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
            ..Default::default()
        };
        let result = fixer.try_transforms(&error, source);
        assert!(result.is_some());
        let (modified, name, confidence) = result.unwrap();
        assert!(modified.contains(".to_string()"));
        assert_eq!(name, "add_to_string");
        assert!(confidence > 0.0);
    }

    #[test]
    fn test_try_transforms_invalid_line_number() {
        let fixer = GeneratedRustFixer::new();
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "expected `String`, found `&str`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 0, // Invalid line number
            column: 1,
            ..Default::default()
        };
        let result = fixer.try_transforms(&error, "let x = value");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_transforms_line_beyond_source() {
        let fixer = GeneratedRustFixer::new();
        let error = CompilationError {
            code: "E0308".to_string(),
            message: "expected `String`, found `&str`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 100, // Beyond source lines
            column: 1,
            ..Default::default()
        };
        let result = fixer.try_transforms(&error, "let x = value");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_transforms_function_replacement_value_keys() {
        let fixer = GeneratedRustFixer::new();
        let error = CompilationError {
            code: "E0599".to_string(),
            message: "no method named `keys` found for enum `Value`".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
            ..Default::default()
        };
        let source = "let k = data.keys();";
        let result = fixer.try_transforms(&error, source);
        assert!(result.is_some());
        let (modified, name, _) = result.unwrap();
        assert!(modified.contains("as_object()"));
        assert_eq!(name, "value_keys");
    }

    // === CompositeFixApplicator tests ===

    #[test]
    fn test_composite_with_applicator() {
        let applicator =
            CompositeFixApplicator::new().with_applicator(Box::new(GeneratedRustFixer::new()));
        assert_eq!(applicator.applicators.len(), 2); // default + added
    }

    #[test]
    fn test_composite_default() {
        let applicator = CompositeFixApplicator::default();
        assert_eq!(applicator.applicators.len(), 1);
    }

    #[test]
    fn test_apply_batch_empty() {
        let applicator = CompositeFixApplicator::new();
        let source_files: HashMap<PathBuf, String> = HashMap::new();
        let results = applicator.apply_batch(&[], &source_files);
        assert!(results.is_empty());
    }

    #[test]
    fn test_apply_batch_file_not_found() {
        let applicator = CompositeFixApplicator::new();
        let classification = make_classification("E0308", "type mismatch", 1);
        let source_files: HashMap<PathBuf, String> = HashMap::new();
        let results = applicator.apply_batch(&[classification], &source_files);
        assert!(results.is_empty()); // No source found for file
    }

    #[test]
    fn test_apply_batch_with_matching_source() {
        let applicator = CompositeFixApplicator::new();
        let classification =
            make_classification("E0599", "no method named `keys` found for enum `Value`", 1);
        let mut source_files = HashMap::new();
        source_files.insert(PathBuf::from("test.rs"), "let k = data.keys();".to_string());
        let results = applicator.apply_batch(&[classification], &source_files);
        assert_eq!(results.len(), 1);
        assert!(results[0].applied);
    }

    #[test]
    fn test_apply_batch_unhandled_error() {
        let applicator = CompositeFixApplicator::new();
        let classification = make_classification("E9999", "unknown", 1);
        let mut source_files = HashMap::new();
        source_files.insert(PathBuf::from("test.rs"), "fn test() {}".to_string());
        let results = applicator.apply_batch(&[classification], &source_files);
        assert!(results.is_empty()); // No applicator can_handle E9999
    }

    #[test]
    fn test_apply_batch_multiple_classifications() {
        let applicator = CompositeFixApplicator::new();
        let c1 = make_classification("E0599", "no method named `keys` found for enum `Value`", 1);
        let c2 = make_classification("E0599", "no method named `items` found for enum `Value`", 2);
        let mut source_files = HashMap::new();
        source_files.insert(
            PathBuf::from("test.rs"),
            "let k = data.keys();\nfor (k, v) in data.items() {".to_string(),
        );
        let results = applicator.apply_batch(&[c1, c2], &source_files);
        assert_eq!(results.len(), 2);
    }

    // === GeneratedRustFixer Default trait ===

    #[test]
    fn test_generated_rust_fixer_default() {
        let fixer = GeneratedRustFixer::default();
        assert!(fixer.transforms.contains_key("E0308"));
    }

    // === FixApplicationResult struct tests ===

    #[test]
    fn test_fix_application_result_applied() {
        let result = FixApplicationResult {
            applied: true,
            modified_source: Some("modified".to_string()),
            description: "Applied fix".to_string(),
            confidence: 0.95,
            fix_type: FixType::GeneratedRust,
        };
        assert!(result.applied);
        assert_eq!(result.confidence, 0.95);
        assert_eq!(result.fix_type, FixType::GeneratedRust);
    }

    #[test]
    fn test_fix_application_result_clone() {
        let result = FixApplicationResult {
            applied: true,
            modified_source: Some("code".to_string()),
            description: "test".to_string(),
            confidence: 0.8,
            fix_type: FixType::TranspilerPatch,
        };
        let cloned = result.clone();
        assert_eq!(cloned.applied, result.applied);
        assert_eq!(cloned.fix_type, result.fix_type);
    }

    #[test]
    fn test_fix_application_result_debug() {
        let result = FixApplicationResult {
            applied: false,
            modified_source: None,
            description: "debug test".to_string(),
            confidence: 0.0,
            fix_type: FixType::None,
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("debug test"));
    }

    #[test]
    fn test_fix_type_copy() {
        let ft = FixType::GeneratedRust;
        let copied = ft;
        assert_eq!(ft, copied);
    }

    #[test]
    fn test_fix_type_debug() {
        let ft = FixType::TranspilerPatch;
        let debug = format!("{:?}", ft);
        assert!(debug.contains("TranspilerPatch"));
    }

    #[test]
    fn test_e0277_clone_transform_exists() {
        let fixer = GeneratedRustFixer::new();
        assert!(fixer.transforms.contains_key("E0277"));
        let transforms = fixer.transforms.get("E0277").unwrap();
        assert!(!transforms.is_empty());
    }

    #[test]
    fn test_e0382_transform_exists() {
        let fixer = GeneratedRustFixer::new();
        assert!(fixer.transforms.contains_key("E0382"));
    }

    #[test]
    fn test_try_transforms_e0277_clone() {
        let fixer = GeneratedRustFixer::new();
        let error = CompilationError {
            code: "E0277".to_string(),
            message: "the trait `Clone` is not implemented".to_string(),
            file: PathBuf::from("test.rs"),
            line: 1,
            column: 1,
            ..Default::default()
        };
        let source = "let x = data";
        let result = fixer.try_transforms(&error, source);
        assert!(result.is_some());
        let (modified, name, _) = result.unwrap();
        assert!(modified.contains(".clone()"));
        assert_eq!(name, "add_clone");
    }

    #[test]
    fn test_fix_value_keys_multiple_occurrences() {
        let source = "let a = d1.keys(); let b = d2.keys();";
        let re = regex::Regex::new(r"keys").unwrap();
        let caps = re.captures("keys").unwrap();
        let result = fix_value_keys(source, &caps);
        // Both .keys() occurrences should be replaced with the as_object pattern
        assert!(result.contains("as_object()"));
        // The direct .keys() call at top level should be gone
        assert!(!result.contains("d1.keys()"));
        assert!(!result.contains("d2.keys()"));
    }

    #[test]
    fn test_fix_value_items_no_match() {
        let source = "let x = data.values();";
        let re = regex::Regex::new(r"items").unwrap();
        let caps = re.captures("items").unwrap();
        let result = fix_value_items(source, &caps);
        // No .items() to replace, should be unchanged
        assert_eq!(result, source);
    }
}

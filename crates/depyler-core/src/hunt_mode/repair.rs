//! Jidoka Repair Engine - Automated Fix Application with Quality Gates
//!
//! Implements Jidoka (自働化) - Automation with Human Touch
//! System automatically stops when quality cannot be assured.
//!
//! Searches a library of "Mutators" (code transformations) and applies
//! fixes with confidence thresholds.

use super::isolator::ReproCase;

/// Result of a repair attempt
#[derive(Debug, Clone)]
pub enum RepairResult {
    /// Fix was successfully applied and verified
    Success(Fix),
    /// Fix found but confidence too low - needs human review
    NeedsHumanReview {
        fix: Fix,
        confidence: f64,
        reason: String,
    },
    /// No applicable fix found
    NoFixFound,
}

/// A fix that can be applied to the transpiler
#[derive(Debug, Clone)]
pub struct Fix {
    /// Unique identifier for this fix
    pub id: String,
    /// Ticket reference (e.g., "DEPYLER-0705")
    pub ticket_id: String,
    /// Description of what this fix does
    pub description: String,
    /// The mutator that generated this fix
    pub mutator_name: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// The generated Rust code after fix
    pub rust_output: String,
    /// Location in depyler-core to patch (if applicable)
    pub patch_location: Option<PatchLocation>,
}

/// Location to apply a patch in the transpiler
#[derive(Debug, Clone)]
pub struct PatchLocation {
    /// File path relative to depyler-core
    pub file: String,
    /// Line number (approximate)
    pub line: u32,
    /// Description of the change needed
    pub change_description: String,
}

/// A code transformation strategy
pub trait Mutator: std::fmt::Debug + Send + Sync {
    /// Name of this mutator
    fn name(&self) -> &str;

    /// Check if this mutator can handle the given repro case
    fn can_handle(&self, repro: &ReproCase) -> bool;

    /// Attempt to generate a fix
    fn apply(&self, repro: &ReproCase) -> Option<Fix>;

    /// Estimated confidence for this type of fix
    fn base_confidence(&self) -> f64;
}

/// Jidoka Repair Engine: Applies fixes with quality gates
///
/// Jidoka: Only proceed if quality is assured.
#[derive(Debug)]
pub struct JidokaRepairEngine {
    /// Available mutators
    mutators: Vec<Box<dyn Mutator>>,
    /// Minimum confidence threshold for auto-apply
    quality_threshold: f64,
    /// Statistics
    total_attempts: u32,
    successful_fixes: u32,
}

impl JidokaRepairEngine {
    /// Create a new repair engine with given quality threshold
    pub fn new(quality_threshold: f64) -> Self {
        let mut engine = Self {
            mutators: Vec::new(),
            quality_threshold,
            total_attempts: 0,
            successful_fixes: 0,
        };
        engine.register_builtin_mutators();
        engine
    }

    /// Register built-in mutators
    fn register_builtin_mutators(&mut self) {
        self.mutators.push(Box::new(ImportMutator));
        self.mutators.push(Box::new(TypeCoercionMutator));
        self.mutators.push(Box::new(SerdeValueFallbackMutator));
        self.mutators.push(Box::new(ToStringMutator));
        self.mutators.push(Box::new(CloneMutator));
    }

    /// Register a custom mutator
    pub fn register_mutator(&mut self, mutator: Box<dyn Mutator>) {
        self.mutators.push(mutator);
    }

    /// Attempt to repair a reproduction case
    ///
    /// Jidoka: Stop the line if fix quality is uncertain.
    pub fn attempt_repair(&mut self, repro: &ReproCase) -> anyhow::Result<RepairResult> {
        self.total_attempts += 1;

        for mutator in &self.mutators {
            if !mutator.can_handle(repro) {
                continue;
            }

            if let Some(mut fix) = mutator.apply(repro) {
                let confidence = self.evaluate_fix_confidence(&fix, repro);
                fix.confidence = confidence;

                // Jidoka: Only proceed if quality is assured
                if confidence < self.quality_threshold {
                    return Ok(RepairResult::NeedsHumanReview {
                        fix,
                        confidence,
                        reason: format!(
                            "Confidence {:.1}% below threshold {:.1}%",
                            confidence * 100.0,
                            self.quality_threshold * 100.0
                        ),
                    });
                }

                self.successful_fixes += 1;
                return Ok(RepairResult::Success(fix));
            }
        }

        Ok(RepairResult::NoFixFound)
    }

    /// Evaluate confidence in a fix
    fn evaluate_fix_confidence(&self, fix: &Fix, _repro: &ReproCase) -> f64 {
        // Base confidence from mutator
        let base = fix.confidence;

        // Adjust based on:
        // 1. Historical success rate of this mutator
        // 2. Complexity of the error pattern
        // 3. Amount of code changed

        // For now, use base confidence with slight adjustment
        (base * 0.9).min(1.0)
    }

    /// Get repair statistics
    pub fn stats(&self) -> (u32, u32) {
        (self.total_attempts, self.successful_fixes)
    }

    /// Success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        self.successful_fixes as f64 / self.total_attempts as f64
    }
}

// Built-in Mutators

/// Mutator that adds missing imports
#[derive(Debug)]
struct ImportMutator;

impl Mutator for ImportMutator {
    fn name(&self) -> &str {
        "ImportMutator"
    }

    fn can_handle(&self, repro: &ReproCase) -> bool {
        repro.expected_error == "E0432" || repro.expected_error == "E0433"
    }

    fn apply(&self, repro: &ReproCase) -> Option<Fix> {
        Some(Fix {
            id: format!("fix_import_{}", repro.pattern_id),
            ticket_id: "DEPYLER-AUTO".to_string(),
            description: "Add missing crate import".to_string(),
            mutator_name: self.name().to_string(),
            confidence: self.base_confidence(),
            rust_output: String::new(), // Would be filled by actual transpilation
            patch_location: Some(PatchLocation {
                file: "rust_gen.rs".to_string(),
                line: 0,
                change_description: "Add use statement for external crate".to_string(),
            }),
        })
    }

    fn base_confidence(&self) -> f64 {
        0.95 // Import fixes are usually reliable
    }
}

/// Mutator that handles type coercion
#[derive(Debug)]
struct TypeCoercionMutator;

impl Mutator for TypeCoercionMutator {
    fn name(&self) -> &str {
        "TypeCoercionMutator"
    }

    fn can_handle(&self, repro: &ReproCase) -> bool {
        repro.expected_error == "E0308"
    }

    fn apply(&self, repro: &ReproCase) -> Option<Fix> {
        Some(Fix {
            id: format!("fix_type_{}", repro.pattern_id),
            ticket_id: "DEPYLER-AUTO".to_string(),
            description: "Add type coercion".to_string(),
            mutator_name: self.name().to_string(),
            confidence: self.base_confidence(),
            rust_output: String::new(),
            patch_location: Some(PatchLocation {
                file: "rust_gen/expr_gen.rs".to_string(),
                line: 0,
                change_description: "Add .into() or explicit type conversion".to_string(),
            }),
        })
    }

    fn base_confidence(&self) -> f64 {
        0.80 // Type coercion needs more care
    }
}

/// Mutator that falls back to serde_json::Value for untyped data
#[derive(Debug)]
struct SerdeValueFallbackMutator;

impl Mutator for SerdeValueFallbackMutator {
    fn name(&self) -> &str {
        "SerdeValueFallbackMutator"
    }

    fn can_handle(&self, repro: &ReproCase) -> bool {
        repro.expected_error == "E0277" || repro.expected_error == "E0308"
    }

    fn apply(&self, repro: &ReproCase) -> Option<Fix> {
        // Only apply if source mentions dict or json
        if !repro.source.to_lowercase().contains("dict")
            && !repro.source.to_lowercase().contains("json")
        {
            return None;
        }

        Some(Fix {
            id: format!("fix_serde_{}", repro.pattern_id),
            ticket_id: "DEPYLER-AUTO".to_string(),
            description: "Fallback to serde_json::Value for dynamic typing".to_string(),
            mutator_name: self.name().to_string(),
            confidence: self.base_confidence(),
            rust_output: String::new(),
            patch_location: Some(PatchLocation {
                file: "rust_gen/expr_gen.rs".to_string(),
                line: 0,
                change_description: "Use serde_json::Value instead of concrete type".to_string(),
            }),
        })
    }

    fn base_confidence(&self) -> f64 {
        0.75 // Fallback, so slightly lower confidence
    }
}

/// Mutator that adds .to_string() for string conversion
#[derive(Debug)]
struct ToStringMutator;

impl Mutator for ToStringMutator {
    fn name(&self) -> &str {
        "ToStringMutator"
    }

    fn can_handle(&self, repro: &ReproCase) -> bool {
        repro.expected_error == "E0308"
    }

    fn apply(&self, _repro: &ReproCase) -> Option<Fix> {
        // This mutator is conservative - only applies in specific cases
        None // TODO: Implement pattern detection
    }

    fn base_confidence(&self) -> f64 {
        0.90
    }
}

/// Mutator that adds .clone() for ownership issues
#[derive(Debug)]
struct CloneMutator;

impl Mutator for CloneMutator {
    fn name(&self) -> &str {
        "CloneMutator"
    }

    fn can_handle(&self, repro: &ReproCase) -> bool {
        repro.expected_error == "E0382" || repro.expected_error == "E0507"
    }

    fn apply(&self, repro: &ReproCase) -> Option<Fix> {
        Some(Fix {
            id: format!("fix_clone_{}", repro.pattern_id),
            ticket_id: "DEPYLER-AUTO".to_string(),
            description: "Add .clone() to avoid move/borrow conflict".to_string(),
            mutator_name: self.name().to_string(),
            confidence: self.base_confidence(),
            rust_output: String::new(),
            patch_location: Some(PatchLocation {
                file: "rust_gen/expr_gen.rs".to_string(),
                line: 0,
                change_description: "Add .clone() to expression".to_string(),
            }),
        })
    }

    fn base_confidence(&self) -> f64 {
        0.70 // Clone can have performance implications
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_repro(error_code: &str, source: &str) -> ReproCase {
        ReproCase::new(
            source.to_string(),
            error_code.to_string(),
            "test_pattern".to_string(),
        )
    }

    #[test]
    fn test_repair_engine_new() {
        let engine = JidokaRepairEngine::new(0.85);
        assert!(!engine.mutators.is_empty());
        assert_eq!(engine.total_attempts, 0);
    }

    #[test]
    fn test_attempt_repair_import_error() {
        let mut engine = JidokaRepairEngine::new(0.85);
        let repro = create_test_repro("E0432", "import json");

        let result = engine.attempt_repair(&repro).unwrap();
        assert!(matches!(result, RepairResult::Success(_)));
    }

    #[test]
    fn test_attempt_repair_low_confidence() {
        let mut engine = JidokaRepairEngine::new(0.99); // Very high threshold
        let repro = create_test_repro("E0308", "def f() -> str: return 42");

        let result = engine.attempt_repair(&repro).unwrap();
        // Should need human review due to high threshold
        assert!(matches!(result, RepairResult::NeedsHumanReview { .. }));
    }

    #[test]
    fn test_attempt_repair_no_fix() {
        let mut engine = JidokaRepairEngine::new(0.85);
        let repro = create_test_repro("E9999", "unknown error");

        let result = engine.attempt_repair(&repro).unwrap();
        assert!(matches!(result, RepairResult::NoFixFound));
    }

    #[test]
    fn test_success_rate() {
        let mut engine = JidokaRepairEngine::new(0.5); // Low threshold for testing

        // Attempt some repairs
        let repro1 = create_test_repro("E0432", "import json");
        let repro2 = create_test_repro("E9999", "unknown");

        let _ = engine.attempt_repair(&repro1);
        let _ = engine.attempt_repair(&repro2);

        assert_eq!(engine.total_attempts, 2);
        assert!(engine.success_rate() >= 0.0 && engine.success_rate() <= 1.0);
    }

    #[test]
    fn test_serde_fallback_only_for_dict() {
        let mut engine = JidokaRepairEngine::new(0.5);

        // Should apply for dict-related code
        let repro_dict = create_test_repro("E0277", "def f(d: dict): pass");
        let result = engine.attempt_repair(&repro_dict).unwrap();
        assert!(!matches!(result, RepairResult::NoFixFound));

        // For non-dict code, serde fallback shouldn't apply
        // (but other mutators might)
    }

    #[test]
    fn test_fix_structure() {
        let fix = Fix {
            id: "fix_1".to_string(),
            ticket_id: "DEPYLER-0705".to_string(),
            description: "Test fix".to_string(),
            mutator_name: "TestMutator".to_string(),
            confidence: 0.9,
            rust_output: "fn test() {}".to_string(),
            patch_location: Some(PatchLocation {
                file: "test.rs".to_string(),
                line: 42,
                change_description: "Test change".to_string(),
            }),
        };

        assert_eq!(fix.id, "fix_1");
        assert!(fix.confidence > 0.85);
        assert!(fix.patch_location.is_some());
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_repair_result_debug() {
        let fix = Fix {
            id: "test".to_string(),
            ticket_id: "DEPYLER-001".to_string(),
            description: "desc".to_string(),
            mutator_name: "mut".to_string(),
            confidence: 0.9,
            rust_output: String::new(),
            patch_location: None,
        };
        let success = RepairResult::Success(fix.clone());
        let debug_str = format!("{:?}", success);
        assert!(debug_str.contains("Success"));

        let no_fix = RepairResult::NoFixFound;
        let debug_str = format!("{:?}", no_fix);
        assert!(debug_str.contains("NoFixFound"));

        let needs_review = RepairResult::NeedsHumanReview {
            fix,
            confidence: 0.5,
            reason: "low conf".to_string(),
        };
        let debug_str = format!("{:?}", needs_review);
        assert!(debug_str.contains("NeedsHumanReview"));
    }

    #[test]
    fn test_repair_result_clone() {
        let fix = Fix {
            id: "clone_test".to_string(),
            ticket_id: "DEPYLER-002".to_string(),
            description: "clone".to_string(),
            mutator_name: "CloneMut".to_string(),
            confidence: 0.8,
            rust_output: "fn f() {}".to_string(),
            patch_location: None,
        };
        let result = RepairResult::Success(fix);
        let cloned = result.clone();
        assert!(matches!(cloned, RepairResult::Success(_)));
    }

    #[test]
    fn test_fix_debug() {
        let fix = Fix {
            id: "debug_fix".to_string(),
            ticket_id: "DEPYLER-003".to_string(),
            description: "debug test".to_string(),
            mutator_name: "DebugMutator".to_string(),
            confidence: 0.75,
            rust_output: "let x = 1;".to_string(),
            patch_location: Some(PatchLocation {
                file: "src/lib.rs".to_string(),
                line: 100,
                change_description: "Add variable".to_string(),
            }),
        };
        let debug_str = format!("{:?}", fix);
        assert!(debug_str.contains("debug_fix"));
        assert!(debug_str.contains("DEPYLER-003"));
        assert!(debug_str.contains("DebugMutator"));
    }

    #[test]
    fn test_fix_clone() {
        let fix = Fix {
            id: "orig".to_string(),
            ticket_id: "DEPYLER-004".to_string(),
            description: "original".to_string(),
            mutator_name: "OrigMut".to_string(),
            confidence: 0.95,
            rust_output: "fn main() {}".to_string(),
            patch_location: None,
        };
        let cloned = fix.clone();
        assert_eq!(cloned.id, "orig");
        assert_eq!(cloned.ticket_id, "DEPYLER-004");
        assert_eq!(cloned.confidence, 0.95);
    }

    #[test]
    fn test_patch_location_debug() {
        let loc = PatchLocation {
            file: "expr_gen.rs".to_string(),
            line: 250,
            change_description: "Fix type coercion".to_string(),
        };
        let debug_str = format!("{:?}", loc);
        assert!(debug_str.contains("expr_gen.rs"));
        assert!(debug_str.contains("250"));
        assert!(debug_str.contains("Fix type coercion"));
    }

    #[test]
    fn test_patch_location_clone() {
        let loc = PatchLocation {
            file: "stmt_gen.rs".to_string(),
            line: 500,
            change_description: "Modify statement".to_string(),
        };
        let cloned = loc.clone();
        assert_eq!(cloned.file, "stmt_gen.rs");
        assert_eq!(cloned.line, 500);
        assert_eq!(cloned.change_description, "Modify statement");
    }

    #[test]
    fn test_engine_stats() {
        let mut engine = JidokaRepairEngine::new(0.5);
        assert_eq!(engine.stats(), (0, 0));

        let repro = create_test_repro("E0432", "import json");
        let _ = engine.attempt_repair(&repro);

        let (attempts, _successes) = engine.stats();
        assert_eq!(attempts, 1);
    }

    #[test]
    fn test_success_rate_zero_attempts() {
        let engine = JidokaRepairEngine::new(0.85);
        assert_eq!(engine.success_rate(), 0.0);
    }

    #[test]
    fn test_register_custom_mutator() {
        #[derive(Debug)]
        struct CustomMutator;
        impl Mutator for CustomMutator {
            fn name(&self) -> &str {
                "CustomMutator"
            }
            fn can_handle(&self, _repro: &ReproCase) -> bool {
                false
            }
            fn apply(&self, _repro: &ReproCase) -> Option<Fix> {
                None
            }
            fn base_confidence(&self) -> f64 {
                0.5
            }
        }

        let mut engine = JidokaRepairEngine::new(0.85);
        let initial_count = engine.mutators.len();
        engine.register_mutator(Box::new(CustomMutator));
        assert_eq!(engine.mutators.len(), initial_count + 1);
    }

    #[test]
    fn test_import_mutator_name() {
        let mutator = ImportMutator;
        assert_eq!(mutator.name(), "ImportMutator");
    }

    #[test]
    fn test_import_mutator_can_handle() {
        let mutator = ImportMutator;
        assert!(mutator.can_handle(&create_test_repro("E0432", "import x")));
        assert!(mutator.can_handle(&create_test_repro("E0433", "from x import y")));
        assert!(!mutator.can_handle(&create_test_repro("E0308", "type error")));
    }

    #[test]
    fn test_import_mutator_base_confidence() {
        let mutator = ImportMutator;
        assert_eq!(mutator.base_confidence(), 0.95);
    }

    #[test]
    fn test_type_coercion_mutator_name() {
        let mutator = TypeCoercionMutator;
        assert_eq!(mutator.name(), "TypeCoercionMutator");
    }

    #[test]
    fn test_type_coercion_mutator_can_handle() {
        let mutator = TypeCoercionMutator;
        assert!(mutator.can_handle(&create_test_repro("E0308", "mismatched types")));
        assert!(!mutator.can_handle(&create_test_repro("E0432", "import")));
    }

    #[test]
    fn test_type_coercion_mutator_apply() {
        let mutator = TypeCoercionMutator;
        let repro = create_test_repro("E0308", "def f() -> str: return 42");
        let fix = mutator.apply(&repro);
        assert!(fix.is_some());
        let fix = fix.unwrap();
        assert_eq!(fix.mutator_name, "TypeCoercionMutator");
        assert!(fix.patch_location.is_some());
    }

    #[test]
    fn test_type_coercion_mutator_base_confidence() {
        let mutator = TypeCoercionMutator;
        assert_eq!(mutator.base_confidence(), 0.80);
    }

    #[test]
    fn test_serde_value_fallback_mutator_name() {
        let mutator = SerdeValueFallbackMutator;
        assert_eq!(mutator.name(), "SerdeValueFallbackMutator");
    }

    #[test]
    fn test_serde_value_fallback_mutator_can_handle() {
        let mutator = SerdeValueFallbackMutator;
        assert!(mutator.can_handle(&create_test_repro("E0277", "trait not satisfied")));
        assert!(mutator.can_handle(&create_test_repro("E0308", "type mismatch")));
        assert!(!mutator.can_handle(&create_test_repro("E0382", "moved value")));
    }

    #[test]
    fn test_serde_value_fallback_mutator_apply_no_dict() {
        let mutator = SerdeValueFallbackMutator;
        let repro = create_test_repro("E0277", "def f(x: int): pass");
        let fix = mutator.apply(&repro);
        assert!(fix.is_none(), "Should not apply to non-dict code");
    }

    #[test]
    fn test_serde_value_fallback_mutator_apply_with_json() {
        let mutator = SerdeValueFallbackMutator;
        let repro = create_test_repro("E0277", "def f(data: json): pass");
        let fix = mutator.apply(&repro);
        assert!(fix.is_some(), "Should apply to json-related code");
    }

    #[test]
    fn test_serde_value_fallback_mutator_base_confidence() {
        let mutator = SerdeValueFallbackMutator;
        assert_eq!(mutator.base_confidence(), 0.75);
    }

    #[test]
    fn test_to_string_mutator_name() {
        let mutator = ToStringMutator;
        assert_eq!(mutator.name(), "ToStringMutator");
    }

    #[test]
    fn test_to_string_mutator_can_handle() {
        let mutator = ToStringMutator;
        assert!(mutator.can_handle(&create_test_repro("E0308", "type mismatch")));
        assert!(!mutator.can_handle(&create_test_repro("E0432", "import")));
    }

    #[test]
    fn test_to_string_mutator_apply() {
        let mutator = ToStringMutator;
        let repro = create_test_repro("E0308", "def f() -> str: return x");
        let fix = mutator.apply(&repro);
        assert!(fix.is_none(), "ToStringMutator returns None (conservative)");
    }

    #[test]
    fn test_to_string_mutator_base_confidence() {
        let mutator = ToStringMutator;
        assert_eq!(mutator.base_confidence(), 0.90);
    }

    #[test]
    fn test_clone_mutator_name() {
        let mutator = CloneMutator;
        assert_eq!(mutator.name(), "CloneMutator");
    }

    #[test]
    fn test_clone_mutator_can_handle() {
        let mutator = CloneMutator;
        assert!(mutator.can_handle(&create_test_repro("E0382", "value used after move")));
        assert!(mutator.can_handle(&create_test_repro("E0507", "cannot move out of")));
        assert!(!mutator.can_handle(&create_test_repro("E0308", "type mismatch")));
    }

    #[test]
    fn test_clone_mutator_apply() {
        let mutator = CloneMutator;
        let repro = create_test_repro("E0382", "def f(x): y = x; z = x");
        let fix = mutator.apply(&repro);
        assert!(fix.is_some());
        let fix = fix.unwrap();
        assert_eq!(fix.mutator_name, "CloneMutator");
        assert!(fix.description.contains("clone"));
    }

    #[test]
    fn test_clone_mutator_base_confidence() {
        let mutator = CloneMutator;
        assert_eq!(mutator.base_confidence(), 0.70);
    }

    #[test]
    fn test_engine_debug() {
        let engine = JidokaRepairEngine::new(0.85);
        let debug_str = format!("{:?}", engine);
        assert!(debug_str.contains("JidokaRepairEngine"));
        assert!(debug_str.contains("mutators"));
    }

    #[test]
    fn test_fix_without_patch_location() {
        let fix = Fix {
            id: "no_patch".to_string(),
            ticket_id: "DEPYLER-005".to_string(),
            description: "No patch needed".to_string(),
            mutator_name: "NoPatchMut".to_string(),
            confidence: 0.99,
            rust_output: "let x = 42;".to_string(),
            patch_location: None,
        };
        assert!(fix.patch_location.is_none());
        assert_eq!(fix.confidence, 0.99);
    }

    #[test]
    fn test_attempt_repair_type_coercion() {
        let mut engine = JidokaRepairEngine::new(0.5);
        let repro = create_test_repro("E0308", "def f() -> str: return 42");
        let result = engine.attempt_repair(&repro).unwrap();
        // TypeCoercionMutator should handle E0308
        assert!(!matches!(result, RepairResult::NoFixFound));
    }

    #[test]
    fn test_attempt_repair_clone_error() {
        let mut engine = JidokaRepairEngine::new(0.5);
        let repro = create_test_repro("E0382", "def f(x): y = x; z = x");
        let result = engine.attempt_repair(&repro).unwrap();
        // CloneMutator should handle E0382
        assert!(!matches!(result, RepairResult::NoFixFound));
    }

    #[test]
    fn test_multiple_repairs_stats() {
        let mut engine = JidokaRepairEngine::new(0.5);

        // Several repairs
        let _ = engine.attempt_repair(&create_test_repro("E0432", "import json"));
        let _ = engine.attempt_repair(&create_test_repro("E0308", "type error"));
        let _ = engine.attempt_repair(&create_test_repro("E9999", "unknown"));
        let _ = engine.attempt_repair(&create_test_repro("E0382", "move error"));

        let (attempts, _) = engine.stats();
        assert_eq!(attempts, 4);
        assert!(engine.success_rate() > 0.0);
    }

    #[test]
    fn test_needs_human_review_reason() {
        let mut engine = JidokaRepairEngine::new(0.95); // Very high threshold
        let repro = create_test_repro("E0382", "def f(x): y = x");
        let result = engine.attempt_repair(&repro).unwrap();

        if let RepairResult::NeedsHumanReview {
            reason, confidence, ..
        } = result
        {
            assert!(reason.contains("below threshold"));
            assert!(confidence < 0.95);
        } else {
            panic!("Expected NeedsHumanReview result");
        }
    }
}

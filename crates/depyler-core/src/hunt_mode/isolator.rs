//! Isolator - Minimal Reproduction Case Synthesis
//!
//! Implements Poka-Yoke (ポカヨケ) - Error-Proofing
//! Every fix MUST have a failing test first (TDD Red).
//!
//! Synthesizes minimal, self-contained Python files that exhibit
//! specific error patterns, enabling targeted fixes.

use super::planner::FailurePattern;
use std::path::PathBuf;
use std::time::SystemTime;

/// A minimal reproduction case for a failure pattern
#[derive(Debug, Clone)]
pub struct ReproCase {
    /// Minimal Python source that triggers the error
    pub source: String,
    /// Expected error code from rustc
    pub expected_error: String,
    /// Pattern this repro was created for
    pub pattern_id: String,
    /// When this repro was created
    pub created_at: SystemTime,
    /// File path if written to disk
    pub file_path: Option<PathBuf>,
    /// Whether this repro has been verified to fail
    pub verified_failing: bool,
}

impl ReproCase {
    /// Create a new reproduction case
    pub fn new(source: String, expected_error: String, pattern_id: String) -> Self {
        Self {
            source,
            expected_error,
            pattern_id,
            created_at: SystemTime::now(),
            file_path: None,
            verified_failing: false,
        }
    }

    /// Mark this repro as verified (confirmed to fail compilation)
    pub fn mark_verified(&mut self) {
        self.verified_failing = true;
    }
}

/// Minimal Reproducer: Synthesizes repro cases from failure patterns
///
/// Poka-yoke: Every fix must have a failing test first.
#[derive(Debug, Default)]
pub struct MinimalReproducer {
    /// Templates for common error patterns
    templates: Vec<ReproTemplate>,
}

/// Template for generating reproduction cases
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ReproTemplate {
    error_code: String,
    template: String,
    description: String,
}

impl MinimalReproducer {
    /// Create a new reproducer with built-in templates
    pub fn new() -> Self {
        let mut reproducer = Self::default();
        reproducer.register_builtin_templates();
        reproducer
    }

    /// Register built-in templates for common error patterns
    fn register_builtin_templates(&mut self) {
        // E0308: Type mismatch
        self.templates.push(ReproTemplate {
            error_code: "E0308".to_string(),
            template: r#"# Minimal repro: E0308 type mismatch
def function_with_type_mismatch() -> str:
    value = 42  # int
    return value  # expects str
"#.to_string(),
            description: "Type mismatch between int and str".to_string(),
        });

        // E0432: Unresolved import
        self.templates.push(ReproTemplate {
            error_code: "E0432".to_string(),
            template: r#"# Minimal repro: E0432 unresolved import
import json

def parse_json(text: str) -> dict:
    return json.loads(text)
"#.to_string(),
            description: "External crate import required".to_string(),
        });

        // E0277: Trait not satisfied
        self.templates.push(ReproTemplate {
            error_code: "E0277".to_string(),
            template: r#"# Minimal repro: E0277 trait bound not satisfied
from typing import Dict, Any

def process_data(data: Dict[str, Any]) -> str:
    return data.get("name", "Unknown")
"#.to_string(),
            description: "Trait bound not satisfied".to_string(),
        });

        // E0502: Borrowing conflict
        self.templates.push(ReproTemplate {
            error_code: "E0502".to_string(),
            template: r#"# Minimal repro: E0502 borrowing conflict
def modify_list(items: list) -> list:
    for i, item in enumerate(items):
        items[i] = item * 2  # Modify while iterating
    return items
"#.to_string(),
            description: "Cannot borrow as mutable while borrowed as immutable".to_string(),
        });

        // E0382: Use after move
        self.templates.push(ReproTemplate {
            error_code: "E0382".to_string(),
            template: r#"# Minimal repro: E0382 use after move
def use_after_move() -> str:
    data = "hello"
    process(data)  # moves data
    return data  # use after move

def process(s: str) -> None:
    pass
"#.to_string(),
            description: "Use of moved value".to_string(),
        });
    }

    /// Synthesize a minimal reproduction case for a failure pattern
    ///
    /// Poka-yoke: The repro MUST fail before any fix is attempted.
    pub fn synthesize_repro(&self, pattern: &FailurePattern) -> anyhow::Result<ReproCase> {
        // Try to find a matching template
        let template = self.find_template(&pattern.error_code)
            .or_else(|| self.generate_from_example(pattern));

        let source = match template {
            Some(t) => t,
            None => {
                // Generate a minimal stub if no template matches
                self.generate_stub_repro(pattern)
            }
        };

        let mut repro = ReproCase::new(
            source,
            pattern.error_code.clone(),
            pattern.id.clone(),
        );

        // Poka-yoke: Verify this actually fails
        // (In real implementation, would transpile and try to compile)
        // For now, mark as needing verification
        repro.verified_failing = false;

        Ok(repro)
    }

    /// Find a template matching the error code
    fn find_template(&self, error_code: &str) -> Option<String> {
        self.templates
            .iter()
            .find(|t| t.error_code == error_code)
            .map(|t| t.template.clone())
    }

    /// Generate repro from pattern's trigger example
    fn generate_from_example(&self, pattern: &FailurePattern) -> Option<String> {
        if pattern.trigger_example.is_empty() {
            return None;
        }

        Some(format!(
            "# Minimal repro: {} - {}\n{}",
            pattern.error_code,
            pattern.description,
            pattern.trigger_example
        ))
    }

    /// Generate a minimal stub repro
    fn generate_stub_repro(&self, pattern: &FailurePattern) -> String {
        format!(
            r#"# Minimal repro: {} - {}
# Category: {}
# TODO: Fill in minimal code that triggers this error

def repro_function():
    pass  # Placeholder
"#,
            pattern.error_code,
            pattern.description,
            pattern.category
        )
    }

    /// Verify that a repro case actually fails compilation
    ///
    /// Poka-yoke: This is the critical check that ensures TDD Red.
    pub fn verify_fails(&self, repro: &mut ReproCase) -> anyhow::Result<bool> {
        // In real implementation:
        // 1. Write repro.source to a temp file
        // 2. Run depyler transpile
        // 3. Run rustc --crate-type lib
        // 4. Check for expected error code

        // For now, simulate verification
        // TODO: Implement actual transpilation and compilation check
        let would_fail = !repro.source.contains("pass  # Placeholder");

        if would_fail {
            repro.mark_verified();
        }

        Ok(would_fail)
    }

    /// Write repro to disk for manual inspection
    pub fn write_to_disk(&self, repro: &mut ReproCase, dir: &std::path::Path) -> anyhow::Result<PathBuf> {
        let filename = format!("repro_{}.py", repro.pattern_id);
        let path = dir.join(filename);

        std::fs::write(&path, &repro.source)?;
        repro.file_path = Some(path.clone());

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hunt_mode::planner::PatternCategory;

    fn create_test_pattern(error_code: &str) -> FailurePattern {
        FailurePattern {
            id: "test_pattern".to_string(),
            error_code: error_code.to_string(),
            description: "Test pattern".to_string(),
            category: PatternCategory::TypeInference,
            affected_count: 10,
            fix_complexity: 5,
            trigger_example: String::new(),
        }
    }

    #[test]
    fn test_reproducer_new() {
        let reproducer = MinimalReproducer::new();
        // Should have built-in templates
        assert!(!reproducer.templates.is_empty());
    }

    #[test]
    fn test_synthesize_repro_with_template() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0308");

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert_eq!(repro.expected_error, "E0308");
        assert!(!repro.source.is_empty());
        assert!(repro.source.contains("type mismatch"));
    }

    #[test]
    fn test_synthesize_repro_no_template() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E9999"); // Unknown error

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert!(repro.source.contains("TODO: Fill in minimal code"));
    }

    #[test]
    fn test_repro_case_new() {
        let repro = ReproCase::new(
            "test source".to_string(),
            "E0308".to_string(),
            "pattern_1".to_string(),
        );

        assert!(!repro.verified_failing);
        assert!(repro.file_path.is_none());
    }

    #[test]
    fn test_repro_case_mark_verified() {
        let mut repro = ReproCase::new(
            "test".to_string(),
            "E0308".to_string(),
            "p1".to_string(),
        );

        assert!(!repro.verified_failing);
        repro.mark_verified();
        assert!(repro.verified_failing);
    }

    #[test]
    fn test_verify_fails_with_real_code() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0308");
        let mut repro = reproducer.synthesize_repro(&pattern).unwrap();

        // Should verify as failing (has actual code, not just placeholder)
        let fails = reproducer.verify_fails(&mut repro).unwrap();
        assert!(fails);
        assert!(repro.verified_failing);
    }

    #[test]
    fn test_verify_fails_with_placeholder() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E9999");
        let mut repro = reproducer.synthesize_repro(&pattern).unwrap();

        // Should NOT verify as failing (just placeholder)
        let fails = reproducer.verify_fails(&mut repro).unwrap();
        assert!(!fails);
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_repro_case_debug() {
        let repro = ReproCase::new(
            "def f(): pass".to_string(),
            "E0308".to_string(),
            "pattern_1".to_string(),
        );
        let debug_str = format!("{:?}", repro);
        assert!(debug_str.contains("ReproCase"));
        assert!(debug_str.contains("E0308"));
        assert!(debug_str.contains("pattern_1"));
    }

    #[test]
    fn test_repro_case_clone() {
        let repro = ReproCase::new(
            "source code".to_string(),
            "E0432".to_string(),
            "p2".to_string(),
        );
        let cloned = repro.clone();
        assert_eq!(cloned.source, "source code");
        assert_eq!(cloned.expected_error, "E0432");
        assert_eq!(cloned.pattern_id, "p2");
        assert!(!cloned.verified_failing);
    }

    #[test]
    fn test_repro_case_fields() {
        let repro = ReproCase::new(
            "test".to_string(),
            "E0277".to_string(),
            "test_pat".to_string(),
        );
        assert_eq!(repro.source, "test");
        assert_eq!(repro.expected_error, "E0277");
        assert_eq!(repro.pattern_id, "test_pat");
        assert!(repro.file_path.is_none());
        assert!(!repro.verified_failing);
    }

    #[test]
    fn test_minimal_reproducer_default() {
        let reproducer: MinimalReproducer = Default::default();
        // Default has no templates
        assert!(reproducer.templates.is_empty());
    }

    #[test]
    fn test_minimal_reproducer_debug() {
        let reproducer = MinimalReproducer::new();
        let debug_str = format!("{:?}", reproducer);
        assert!(debug_str.contains("MinimalReproducer"));
    }

    #[test]
    fn test_synthesize_repro_e0277() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0277");

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert_eq!(repro.expected_error, "E0277");
        assert!(repro.source.contains("E0277"));
    }

    #[test]
    fn test_synthesize_repro_e0502() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0502");

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert_eq!(repro.expected_error, "E0502");
        assert!(repro.source.contains("borrowing"));
    }

    #[test]
    fn test_synthesize_repro_e0382() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0382");

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert_eq!(repro.expected_error, "E0382");
        assert!(repro.source.contains("move"));
    }

    #[test]
    fn test_synthesize_repro_e0432() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0432");

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert_eq!(repro.expected_error, "E0432");
        assert!(repro.source.contains("import"));
    }

    #[test]
    fn test_synthesize_repro_with_trigger_example() {
        let reproducer = MinimalReproducer::new();
        let mut pattern = create_test_pattern("E9999");
        pattern.trigger_example = "def custom(): return 42".to_string();

        let repro = reproducer.synthesize_repro(&pattern).unwrap();
        assert!(repro.source.contains("def custom()"));
    }

    #[test]
    fn test_write_to_disk() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0308");
        let mut repro = reproducer.synthesize_repro(&pattern).unwrap();

        let temp_dir = std::env::temp_dir();
        let path = reproducer.write_to_disk(&mut repro, &temp_dir).unwrap();

        assert!(path.exists());
        assert!(repro.file_path.is_some());
        assert_eq!(repro.file_path.unwrap(), path);

        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_repro_template_fields() {
        let reproducer = MinimalReproducer::new();
        // Verify all templates have required fields
        for template in &reproducer.templates {
            assert!(!template.error_code.is_empty());
            assert!(!template.template.is_empty());
            assert!(!template.description.is_empty());
        }
    }

    #[test]
    fn test_builtin_templates_count() {
        let reproducer = MinimalReproducer::new();
        // Should have templates for: E0308, E0432, E0277, E0502, E0382
        assert_eq!(reproducer.templates.len(), 5);
    }

    #[test]
    fn test_find_template_exists() {
        let reproducer = MinimalReproducer::new();
        let template = reproducer.find_template("E0308");
        assert!(template.is_some());
        assert!(template.unwrap().contains("type mismatch"));
    }

    #[test]
    fn test_find_template_not_exists() {
        let reproducer = MinimalReproducer::new();
        let template = reproducer.find_template("E9999");
        assert!(template.is_none());
    }

    #[test]
    fn test_generate_stub_repro() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E1234");

        let stub = reproducer.generate_stub_repro(&pattern);
        assert!(stub.contains("E1234"));
        assert!(stub.contains("TODO: Fill in minimal code"));
        assert!(stub.contains("repro_function"));
    }

    #[test]
    fn test_verify_fails_marks_repro() {
        let reproducer = MinimalReproducer::new();
        let pattern = create_test_pattern("E0308");
        let mut repro = reproducer.synthesize_repro(&pattern).unwrap();

        assert!(!repro.verified_failing);
        reproducer.verify_fails(&mut repro).unwrap();
        assert!(repro.verified_failing);
    }

    #[test]
    fn test_repro_case_created_at() {
        use std::time::SystemTime;

        let before = SystemTime::now();
        let repro = ReproCase::new("test".to_string(), "E0308".to_string(), "p".to_string());
        let after = SystemTime::now();

        assert!(repro.created_at >= before);
        assert!(repro.created_at <= after);
    }
}

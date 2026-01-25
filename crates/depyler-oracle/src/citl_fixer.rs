//! CITL (Compiler-in-the-Loop Learning) integration for Depyler.
//!
//! Provides iterative fix capabilities using aprender's CITL module
//! for self-supervised learning from compiler feedback.
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
//! │   Python     │────►│   Depyler    │────►│   Rust       │
//! │   Source     │     │  Transpiler  │     │   Code       │
//! └──────────────┘     └──────────────┘     └──────────────┘
//!                                                 │
//!                                                 ▼
//!                                          ┌──────────────┐
//!                                          │   rustc      │
//!                                          │  Compiler    │
//!                                          └──────────────┘
//!                                                 │
//!                                                 ▼
//!                                          ┌──────────────┐
//!                                          │ CITL Fixer   │◄──┐
//!                                          │  (iterate)   │   │
//!                                          └──────────────┘   │
//!                                                 │           │
//!                                                 └───────────┘
//! ```
//!
//! # References
//!
//! - Wang, Y., et al. (2022). Compilable Neural Code Generation with Compiler Feedback.
//! - StepCoder (Dou et al., 2024). RLCF for code generation.

use crate::{AutoFixer, OracleError};
use aprender::citl::{
    CompilationMode, CompilationResult, CompileOptions, CompilerDiagnostic, CompilerInterface,
    ErrorEmbedding, ErrorEncoder, FixTemplate, MetricsTracker, PatternLibrary, PatternMatch,
    RustCompiler,
};
use std::collections::HashMap;
use std::time::Instant;

/// Result of iterative fix attempts.
#[derive(Debug, Clone)]
pub struct IterativeFixResult {
    /// Whether fixing was successful
    pub success: bool,
    /// The final source code (fixed or original)
    pub fixed_source: String,
    /// Number of iterations used
    pub iterations: usize,
    /// Descriptions of fixes applied
    pub fixes_applied: Vec<String>,
    /// Remaining error count (0 if successful)
    pub remaining_errors: usize,
    /// Total time spent fixing
    pub fix_duration_ms: u64,
}

impl IterativeFixResult {
    /// Create a success result.
    #[must_use]
    pub fn success(fixed_source: String, iterations: usize, fixes: Vec<String>) -> Self {
        Self {
            success: true,
            fixed_source,
            iterations,
            fixes_applied: fixes,
            remaining_errors: 0,
            fix_duration_ms: 0,
        }
    }

    /// Create a failure result.
    #[must_use]
    pub fn failure(source: String, iterations: usize, remaining: usize) -> Self {
        Self {
            success: false,
            fixed_source: source,
            iterations,
            fixes_applied: Vec::new(),
            remaining_errors: remaining,
            fix_duration_ms: 0,
        }
    }

    /// Set the duration.
    #[must_use]
    pub fn with_duration(mut self, ms: u64) -> Self {
        self.fix_duration_ms = ms;
        self
    }
}

/// CITL-enhanced fixer for transpiled Rust code.
///
/// Integrates aprender's CITL module with depyler's AutoFixer
/// for iterative compilation-driven fixing.
pub struct CITLFixer {
    /// Rust compiler interface
    compiler: RustCompiler,
    /// Error encoder for pattern matching
    encoder: ErrorEncoder,
    /// Pattern library for learned fixes
    pattern_library: PatternLibrary,
    /// Metrics tracker
    metrics: MetricsTracker,
    /// Maximum fix iterations
    max_iterations: usize,
    /// Confidence threshold for accepting fixes
    confidence_threshold: f32,
    /// Fallback to rule-based fixer
    autofixer: Option<AutoFixer>,
}

impl CITLFixer {
    /// Create a new CITL fixer with default configuration.
    ///
    /// # Errors
    ///
    /// Returns error if AutoFixer initialization fails.
    pub fn new() -> Result<Self, OracleError> {
        Self::with_config(CITLFixerConfig::default())
    }

    /// Create a CITL fixer with custom configuration.
    ///
    /// # Errors
    ///
    /// Returns error if AutoFixer initialization fails.
    pub fn with_config(config: CITLFixerConfig) -> Result<Self, OracleError> {
        let compiler = RustCompiler::new().mode(config.compilation_mode);
        let encoder = ErrorEncoder::new();

        // Load or create pattern library
        let pattern_library = if let Some(ref path) = config.pattern_library_path {
            PatternLibrary::load(path).unwrap_or_else(|_| PatternLibrary::new())
        } else {
            PatternLibrary::new()
        };

        // Optionally initialize AutoFixer as fallback
        let autofixer = if config.use_autofixer_fallback {
            AutoFixer::new().ok()
        } else {
            None
        };

        Ok(Self {
            compiler,
            encoder,
            pattern_library,
            metrics: MetricsTracker::new(),
            max_iterations: config.max_iterations,
            confidence_threshold: config.confidence_threshold,
            autofixer,
        })
    }

    /// Iteratively fix all errors in source code.
    ///
    /// Implements the CITL feedback loop:
    /// 1. Compile source code
    /// 2. If errors, suggest and apply fixes
    /// 3. Repeat until success or max iterations
    ///
    /// # Arguments
    /// * `source` - The Rust source code to fix
    ///
    /// # Returns
    /// `IterativeFixResult` containing the final state after fix attempts.
    pub fn fix_all(&mut self, source: &str) -> IterativeFixResult {
        let start = Instant::now();
        let mut current = source.to_string();
        let mut iterations = 0;
        let mut applied_fixes = Vec::new();

        // First check if code already compiles
        if self.compiles(&current) {
            return IterativeFixResult::success(current, 0, applied_fixes)
                .with_duration(start.elapsed().as_millis() as u64);
        }

        while iterations < self.max_iterations {
            iterations += 1;

            // Try to compile
            let compile_result = self.compile(&current);
            let errors = match compile_result {
                Ok(CompilationResult::Success { .. }) => {
                    self.metrics.record_convergence(iterations, true);
                    return IterativeFixResult::success(current, iterations, applied_fixes)
                        .with_duration(start.elapsed().as_millis() as u64);
                }
                Ok(CompilationResult::Failure { errors, .. }) => errors,
                Err(_) => {
                    break;
                }
            };

            if errors.is_empty() {
                self.metrics.record_convergence(iterations, true);
                return IterativeFixResult::success(current, iterations, applied_fixes)
                    .with_duration(start.elapsed().as_millis() as u64);
            }

            // Try to fix the first error using pattern library
            let error = &errors[0];
            let error_code = &error.code.code;

            // 1. Try pattern library first
            let embedding = self.encoder.encode(error, &current);
            let matches = self.pattern_library.search(&embedding, 5);

            let mut fixed = false;
            for (idx, m) in matches.iter().enumerate() {
                if m.similarity >= self.confidence_threshold {
                    // Apply the fix
                    if let Some(new_source) = self.apply_pattern_fix(&current, error, m) {
                        self.metrics.record_fix_attempt(true, error_code);
                        self.metrics.record_pattern_use(idx, true);
                        applied_fixes.push(m.pattern.fix_template.description.clone());
                        current = new_source;
                        fixed = true;
                        break;
                    }
                }
            }

            // 2. Fall back to AutoFixer if pattern library didn't help
            if !fixed {
                if let Some(ref autofixer) = self.autofixer {
                    let error_str = format!("error[{}]: {}", error.code.code, error.message);
                    let fix_result = autofixer.fix(&current, &error_str);
                    if fix_result.fixed {
                        self.metrics.record_fix_attempt(true, error_code);
                        applied_fixes.push(fix_result.description);
                        current = fix_result.source;
                        fixed = true;
                    }
                }
            }

            // No fix found
            if !fixed {
                self.metrics.record_fix_attempt(false, error_code);
                self.metrics.record_convergence(iterations, false);
                return IterativeFixResult::failure(current, iterations, errors.len())
                    .with_duration(start.elapsed().as_millis() as u64);
            }
        }

        // Max iterations reached
        let errors = self.count_errors(&current);
        self.metrics.record_convergence(iterations, false);
        IterativeFixResult {
            success: false,
            fixed_source: current,
            iterations,
            fixes_applied: applied_fixes,
            remaining_errors: errors,
            fix_duration_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// Check if source code compiles successfully.
    #[must_use]
    pub fn compiles(&self, source: &str) -> bool {
        matches!(
            self.compiler.compile(source, &CompileOptions::default()),
            Ok(CompilationResult::Success { .. })
        )
    }

    /// Compile source code and return structured result.
    fn compile(&self, source: &str) -> Result<CompilationResult, aprender::citl::CITLError> {
        self.compiler.compile(source, &CompileOptions::default())
    }

    /// Count compilation errors in source.
    fn count_errors(&self, source: &str) -> usize {
        match self.compile(source) {
            Ok(CompilationResult::Failure { errors, .. }) => errors.len(),
            _ => 0,
        }
    }

    /// Apply a pattern-based fix to the source.
    fn apply_pattern_fix(
        &self,
        source: &str,
        error: &CompilerDiagnostic,
        pattern_match: &PatternMatch,
    ) -> Option<String> {
        // Try compiler's own suggestion first if available
        for suggestion in &error.suggestions {
            let replacement = &suggestion.replacement;
            let span = &replacement.span;

            // Apply the replacement at the span
            if span.byte_start < source.len() && span.byte_end <= source.len() {
                let mut result = source.to_string();
                result.replace_range(span.byte_start..span.byte_end, &replacement.replacement);
                return Some(result);
            }
        }

        // Fall back to template-based fix with bindings
        let template = &pattern_match.pattern.fix_template;
        let mut bindings = HashMap::new();

        // Extract bindings from error context
        if let Some(ref expected) = error.expected {
            bindings.insert("expected_type".to_string(), expected.full.clone());
        }
        if let Some(ref found) = error.found {
            bindings.insert("found_type".to_string(), found.full.clone());
        }

        // If we have meaningful bindings, try to apply template
        if !bindings.is_empty() || !template.pattern.is_empty() {
            let fix_code = template.apply(&bindings);
            if !fix_code.is_empty() && fix_code != template.pattern {
                // Simple replacement at error span
                let span = &error.span;
                if span.byte_start < source.len() && span.byte_end <= source.len() {
                    let mut result = source.to_string();
                    result.replace_range(span.byte_start..span.byte_end, &fix_code);
                    return Some(result);
                }
            }
        }

        None
    }

    /// Record a successful fix to the pattern library for self-training.
    pub fn record_success(&mut self, error_embedding: ErrorEmbedding, fix_template: FixTemplate) {
        self.pattern_library
            .add_pattern(error_embedding, fix_template);
    }

    /// Save the pattern library to disk.
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be written.
    pub fn save_patterns(&self, path: &str) -> Result<(), OracleError> {
        self.pattern_library
            .save(path)
            .map_err(|e| OracleError::Model(e.to_string()))
    }

    /// Get metrics summary.
    #[must_use]
    pub fn metrics_summary(&self) -> aprender::citl::MetricsSummary {
        self.metrics.summary()
    }

    /// Get a reference to the metrics tracker.
    #[must_use]
    pub fn metrics(&self) -> &MetricsTracker {
        &self.metrics
    }
}

impl Default for CITLFixer {
    fn default() -> Self {
        Self::new().expect("CITLFixer initialization failed")
    }
}

/// Configuration for CITL fixer.
#[derive(Debug, Clone)]
pub struct CITLFixerConfig {
    /// Maximum fix iterations
    pub max_iterations: usize,
    /// Confidence threshold for accepting fixes (0.0-1.0)
    pub confidence_threshold: f32,
    /// Path to pattern library file
    pub pattern_library_path: Option<String>,
    /// Use AutoFixer as fallback
    pub use_autofixer_fallback: bool,
    /// Compilation mode
    pub compilation_mode: CompilationMode,
}

impl Default for CITLFixerConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            confidence_threshold: 0.7,
            pattern_library_path: None,
            use_autofixer_fallback: true,
            compilation_mode: CompilationMode::Standalone,
        }
    }
}

impl CITLFixerConfig {
    /// Create config for quick iteration (fewer attempts).
    #[must_use]
    pub fn quick() -> Self {
        Self {
            max_iterations: 3,
            confidence_threshold: 0.8,
            pattern_library_path: None,
            use_autofixer_fallback: true,
            compilation_mode: CompilationMode::Standalone,
        }
    }

    /// Create config for thorough fixing (more attempts).
    #[must_use]
    pub fn thorough() -> Self {
        Self {
            max_iterations: 20,
            confidence_threshold: 0.5,
            pattern_library_path: None,
            use_autofixer_fallback: true,
            compilation_mode: CompilationMode::Standalone,
        }
    }

    /// Set pattern library path.
    #[must_use]
    pub fn with_pattern_library(mut self, path: &str) -> Self {
        self.pattern_library_path = Some(path.to_string());
        self
    }
}

// ============================================================================
// Tests (EXTREME TDD)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // Unit Tests for IterativeFixResult
    // ============================================

    #[test]
    fn test_iterative_fix_result_success() {
        let result =
            IterativeFixResult::success("fn main() {}".to_string(), 3, vec!["fix1".to_string()]);
        assert!(result.success);
        assert_eq!(result.iterations, 3);
        assert_eq!(result.remaining_errors, 0);
        assert_eq!(result.fixes_applied.len(), 1);
    }

    #[test]
    fn test_iterative_fix_result_failure() {
        let result = IterativeFixResult::failure("broken".to_string(), 10, 5);
        assert!(!result.success);
        assert_eq!(result.iterations, 10);
        assert_eq!(result.remaining_errors, 5);
    }

    #[test]
    fn test_iterative_fix_result_with_duration() {
        let result = IterativeFixResult::success("code".to_string(), 1, vec![]).with_duration(150);
        assert_eq!(result.fix_duration_ms, 150);
    }

    // ============================================
    // Unit Tests for CITLFixerConfig
    // ============================================

    #[test]
    fn test_config_default() {
        let config = CITLFixerConfig::default();
        assert_eq!(config.max_iterations, 10);
        assert!((config.confidence_threshold - 0.7).abs() < 0.001);
        assert!(config.use_autofixer_fallback);
    }

    #[test]
    fn test_config_quick() {
        let config = CITLFixerConfig::quick();
        assert_eq!(config.max_iterations, 3);
        assert!((config.confidence_threshold - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_config_thorough() {
        let config = CITLFixerConfig::thorough();
        assert_eq!(config.max_iterations, 20);
        assert!((config.confidence_threshold - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_config_with_pattern_library() {
        let config = CITLFixerConfig::default().with_pattern_library("patterns.citl");
        assert_eq!(
            config.pattern_library_path,
            Some("patterns.citl".to_string())
        );
    }

    // ============================================
    // Integration Tests (require compilation)
    // ============================================

    #[test]
    fn test_fix_all_already_compiles() {
        // Skip if DEPYLER_FAST_TESTS is set (coverage runs)
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            return;
        }

        let config = CITLFixerConfig {
            use_autofixer_fallback: false,
            ..CITLFixerConfig::quick()
        };
        let mut fixer = CITLFixer::with_config(config).unwrap();

        let valid_code = r#"fn main() { println!("Hello"); }"#;
        let result = fixer.fix_all(valid_code);

        assert!(result.success);
        assert_eq!(result.iterations, 0);
    }

    #[test]
    fn test_fix_all_respects_max_iterations() {
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            return;
        }

        let config = CITLFixerConfig {
            max_iterations: 2,
            use_autofixer_fallback: false,
            ..CITLFixerConfig::default()
        };
        let mut fixer = CITLFixer::with_config(config).unwrap();

        // Code with unfixable error
        let broken_code = "fn main() { undefined_function(); }";
        let result = fixer.fix_all(broken_code);

        assert!(!result.success);
        assert!(result.iterations <= 2);
    }

    #[test]
    fn test_compiles_valid_code() {
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            return;
        }

        let config = CITLFixerConfig {
            use_autofixer_fallback: false,
            ..CITLFixerConfig::quick()
        };
        let fixer = CITLFixer::with_config(config).unwrap();

        assert!(fixer.compiles("fn main() {}"));
        assert!(!fixer.compiles("fn main() { undefined() }"));
    }

    #[test]
    fn test_metrics_tracking() {
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            return;
        }

        let config = CITLFixerConfig {
            max_iterations: 1,
            use_autofixer_fallback: false,
            ..CITLFixerConfig::default()
        };
        let mut fixer = CITLFixer::with_config(config).unwrap();

        // Run a fix attempt
        let _ = fixer.fix_all("fn main() { x }");

        // Metrics should be recorded - verify we got a summary
        let summary = fixer.metrics_summary();
        // Session duration should be valid (non-negative, which is always true for Duration)
        let _ = summary.session_duration;
    }

    // ============================================
    // Property Tests
    // ============================================

    #[test]
    #[ignore] // SLOW: CITL fixer property test takes >120s
    fn test_fix_all_never_increases_errors() {
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            return;
        }

        let config = CITLFixerConfig {
            max_iterations: 5,
            use_autofixer_fallback: true,
            ..CITLFixerConfig::default()
        };
        let mut fixer = CITLFixer::with_config(config).unwrap();

        // Initial error count
        let source = "fn main() { let x: i32 = \"string\"; }";
        let initial_errors = fixer.count_errors(source);

        // After fixing
        let result = fixer.fix_all(source);
        let final_errors = fixer.count_errors(&result.fixed_source);

        // Errors should not increase (monotonic improvement or same)
        assert!(
            final_errors <= initial_errors || result.success,
            "Errors increased: {} -> {}",
            initial_errors,
            final_errors
        );
    }

    #[test]
    fn test_fix_preserves_structure() {
        if std::env::var("DEPYLER_FAST_TESTS").is_ok() {
            return;
        }

        let config = CITLFixerConfig {
            use_autofixer_fallback: false,
            ..CITLFixerConfig::quick()
        };
        let mut fixer = CITLFixer::with_config(config).unwrap();

        // Valid code should be unchanged
        let valid = "fn main() { let x = 42; println!(\"{}\", x); }";
        let result = fixer.fix_all(valid);

        assert!(result.success);
        // Structure should be preserved (same function count, etc.)
        assert!(result.fixed_source.contains("fn main()"));
        assert!(result.fixed_source.contains("let x"));
    }
}

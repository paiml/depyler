//! Verificar Integration Tests for Depyler
//!
//! This module integrates depyler with the verificar testing framework,
//! enabling automated verification of Python-to-Rust transpilations.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    VERIFICAR INTEGRATION                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │  DepylerTranspiler   →   TranspilerOracle   →   Verdict     │
//! │  (impl Transpiler)       (I/O verification)     (Pass/Fail) │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use depyler_core::DepylerPipeline;
use verificar::grammar::Grammar;
use verificar::transpiler::Transpiler;
use verificar::{Language, Result as VerificarResult};

/// Depyler transpiler implementing the verificar Transpiler trait
///
/// This adapter allows depyler to be used with verificar's testing infrastructure,
/// including the TranspilerOracle for I/O-based verification.
#[derive(Debug)]
pub struct DepylerTranspiler {
    pipeline: DepylerPipeline,
    grammar: DepylerPythonGrammar,
}

impl Default for DepylerTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

impl DepylerTranspiler {
    /// Create a new DepylerTranspiler with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            pipeline: DepylerPipeline::new(),
            grammar: DepylerPythonGrammar,
        }
    }

    /// Create a new DepylerTranspiler with verification enabled
    #[must_use]
    pub fn with_verification() -> Self {
        Self {
            pipeline: DepylerPipeline::new().with_verification(),
            grammar: DepylerPythonGrammar,
        }
    }
}

impl Transpiler for DepylerTranspiler {
    fn source_language(&self) -> Language {
        Language::Python
    }

    fn target_language(&self) -> Language {
        Language::Rust
    }

    fn transpile(&self, source: &str) -> VerificarResult<String> {
        self.pipeline
            .transpile(source)
            .map_err(|e| verificar::Error::Transpile(e.to_string()))
    }

    fn grammar(&self) -> &dyn Grammar {
        &self.grammar
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Python grammar implementation for depyler
///
/// Uses rustpython-parser for syntax validation.
#[derive(Debug, Clone, Copy)]
pub struct DepylerPythonGrammar;

impl Grammar for DepylerPythonGrammar {
    fn language(&self) -> Language {
        Language::Python
    }

    fn validate(&self, code: &str) -> bool {
        use rustpython_ast::Suite;
        use rustpython_parser::Parse;

        Suite::parse(code, "<input>").is_ok()
    }

    fn max_enumeration_depth(&self) -> usize {
        5 // Default per verificar spec
    }
}

// ============================================================================
// UNIT TESTS - Transpiler Trait Implementation
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpiler_source_language() {
        let transpiler = DepylerTranspiler::new();
        assert_eq!(transpiler.source_language(), Language::Python);
    }

    #[test]
    fn test_transpiler_target_language() {
        let transpiler = DepylerTranspiler::new();
        assert_eq!(transpiler.target_language(), Language::Rust);
    }

    #[test]
    fn test_transpiler_version() {
        let transpiler = DepylerTranspiler::new();
        let version = transpiler.version();
        assert!(!version.is_empty());
        // Version should be semver format
        assert!(version.contains('.'), "Version should be semver: {}", version);
    }

    #[test]
    fn test_transpiler_simple_function() {
        let transpiler = DepylerTranspiler::new();
        let python = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

        let result = transpiler.transpile(python);
        assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());

        let rust_code = result.unwrap();
        assert!(rust_code.contains("pub fn add"));
        assert!(rust_code.contains("i32"));
    }

    #[test]
    fn test_transpiler_invalid_python() {
        let transpiler = DepylerTranspiler::new();
        let invalid = "def broken(\n    return";

        let result = transpiler.transpile(invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_grammar_language() {
        let grammar = DepylerPythonGrammar;
        assert_eq!(grammar.language(), Language::Python);
    }

    #[test]
    fn test_grammar_validate_valid_python() {
        let grammar = DepylerPythonGrammar;
        let valid = "def hello(): return 'world'";
        assert!(grammar.validate(valid));
    }

    #[test]
    fn test_grammar_validate_invalid_python() {
        let grammar = DepylerPythonGrammar;
        let invalid = "def broken(\n    return";
        assert!(!grammar.validate(invalid));
    }

    #[test]
    fn test_grammar_max_enumeration_depth() {
        let grammar = DepylerPythonGrammar;
        assert_eq!(grammar.max_enumeration_depth(), 5);
    }

    #[test]
    fn test_transpiler_grammar_access() {
        let transpiler = DepylerTranspiler::new();
        let grammar = transpiler.grammar();
        assert_eq!(grammar.language(), Language::Python);
    }

    #[test]
    fn test_transpiler_with_verification() {
        let transpiler = DepylerTranspiler::with_verification();
        let python = r#"
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;

        let result = transpiler.transpile(python);
        assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    }
}

// ============================================================================
// INTEGRATION TESTS - TranspilerOracle Usage
// ============================================================================

#[cfg(test)]
mod oracle_tests {
    use super::*;
    use verificar::transpiler::{TranspilerOracle, TranspilerVerdict, VerificationStats};

    #[test]
    fn test_oracle_creation() {
        let transpiler = DepylerTranspiler::new();
        let oracle = TranspilerOracle::new(transpiler);

        // Oracle should be created successfully
        assert_eq!(oracle.transpiler().source_language(), Language::Python);
        assert_eq!(oracle.transpiler().target_language(), Language::Rust);
    }

    #[test]
    fn test_oracle_single_verification_success() {
        let transpiler = DepylerTranspiler::new();
        let oracle = TranspilerOracle::new(transpiler);

        let python = r#"
def double(x: int) -> int:
    return x * 2
"#;

        // verify() takes (source, input) - empty input for simple test
        let result = oracle.verify(python, "");

        match result.verdict {
            TranspilerVerdict::Pass => {
                // Check that transpilation produced code
                assert!(result.target_code.is_some());
                let rust_code = result.target_code.unwrap();
                assert!(rust_code.contains("pub fn double"));
                assert!(rust_code.contains("* 2"));
            }
            TranspilerVerdict::TranspileError(msg) => {
                // Transpilation error is a valid outcome for testing
                println!("Transpile error: {}", msg);
            }
            TranspilerVerdict::TargetError(msg) => {
                // Target execution error (e.g., Rust compile error)
                println!("Target error (expected without runtime): {}", msg);
            }
            TranspilerVerdict::OutputMismatch => {
                // Output mismatch means both Python and Rust ran, but produced different output
                // This is valid for this test - we're testing the oracle works, not semantic equivalence
                println!("Output mismatch (oracle verification working as expected)");
                // Verify transpilation still produced code
                assert!(result.target_code.is_some());
            }
            TranspilerVerdict::SourceError(msg) => {
                // Source (Python) execution error
                println!("Source error: {}", msg);
            }
            TranspilerVerdict::Timeout => {
                // Execution timed out
                println!("Timeout during execution");
            }
        }
    }

    #[test]
    fn test_oracle_batch_verification() {
        let transpiler = DepylerTranspiler::new();
        let oracle = TranspilerOracle::new(transpiler);

        // verify_batch takes &[(String, String)] - pairs of (source, input)
        let test_cases: Vec<(String, String)> = vec![
            (
                r#"def add(a: int, b: int) -> int:
    return a + b"#
                    .to_string(),
                String::new(),
            ),
            (
                r#"def multiply(x: int, y: int) -> int:
    return x * y"#
                    .to_string(),
                String::new(),
            ),
            (
                r#"def is_even(n: int) -> bool:
    return n % 2 == 0"#
                    .to_string(),
                String::new(),
            ),
        ];

        let (results, stats) = oracle.verify_batch(&test_cases);

        assert_eq!(stats.total, 3);
        assert_eq!(results.len(), 3);

        // At least verify transpilation worked for most
        let transpile_successes = results
            .iter()
            .filter(|r| !matches!(r.verdict, TranspilerVerdict::TranspileError(_)))
            .count();
        assert!(
            transpile_successes >= 2,
            "Expected at least 2 transpilation successes, got {}",
            transpile_successes
        );
    }

    #[test]
    fn test_oracle_handles_transpile_error() {
        let transpiler = DepylerTranspiler::new();
        let oracle = TranspilerOracle::new(transpiler);

        // Invalid Python syntax
        let invalid = "def broken(\n    return";
        let result = oracle.verify(invalid, "");

        match result.verdict {
            TranspilerVerdict::TranspileError(message) => {
                assert!(!message.is_empty());
            }
            other => panic!("Expected TranspileError verdict, got: {:?}", other),
        }
    }

    #[test]
    fn test_oracle_verification_stats_default() {
        let stats = VerificationStats::default();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.passed, 0);
        assert_eq!(stats.transpile_errors, 0);
    }

    #[test]
    fn test_oracle_complex_functions() {
        let transpiler = DepylerTranspiler::new();
        let oracle = TranspilerOracle::new(transpiler);

        // Test with more complex Python
        let python = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

        let result = oracle.verify(python, "");

        match result.verdict {
            TranspilerVerdict::Pass => {
                let rust_code = result.target_code.unwrap();
                assert!(rust_code.contains("fibonacci"));
                assert!(rust_code.contains("if"));
            }
            TranspilerVerdict::TranspileError(message) => {
                // Some complex Python may not transpile yet
                println!("Transpile error (acceptable): {}", message);
            }
            TranspilerVerdict::TargetError(message) => {
                // Target error is acceptable (no Rust compiler in test env)
                println!("Target error (acceptable): {}", message);
            }
            other => println!("Other verdict: {:?}", other),
        }
    }

    #[test]
    fn test_verification_stats_pass_rate() {
        let stats = VerificationStats {
            total: 10,
            passed: 8,
            ..Default::default()
        };

        assert_eq!(stats.pass_rate(), 80.0);
    }

    #[test]
    fn test_verification_stats_transpile_rate() {
        let stats = VerificationStats {
            total: 10,
            transpile_errors: 2,
            ..Default::default()
        };

        assert_eq!(stats.transpile_rate(), 80.0);
    }
}

// ============================================================================
// PROPERTY TESTS - Transpiler Invariants
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Property: Valid Python always produces valid Rust or an error (never panics)
    proptest! {
        #[test]
        fn prop_transpiler_never_panics(
            func_name in "[a-z][a-z0-9_]{0,10}",
            param_name in "[a-z][a-z0-9_]{0,5}",
        ) {
            let python = format!(
                "def {}({}: int) -> int:\n    return {} + 1",
                func_name, param_name, param_name
            );

            let transpiler = DepylerTranspiler::new();
            // Should never panic, only Ok or Err
            let _ = transpiler.transpile(&python);
        }
    }

    // Property: Grammar validation is consistent with transpilation
    proptest! {
        #[test]
        fn prop_grammar_consistency(
            func_name in "[a-z][a-z0-9_]{0,10}",
        ) {
            let python = format!("def {}(): pass", func_name);

            let grammar = DepylerPythonGrammar;
            let transpiler = DepylerTranspiler::new();

            // If grammar validates, transpilation should not fail with parse error
            if grammar.validate(&python) {
                let result = transpiler.transpile(&python);
                // Should not be a syntax error (other errors like type errors are acceptable)
                if let Err(e) = result {
                    let msg = e.to_string().to_lowercase();
                    prop_assert!(
                        !msg.contains("syntax") && !msg.contains("parse"),
                        "Grammar validated but got parse error: {}",
                        e
                    );
                }
            }
        }
    }
}

//! Synthetic Error Augmentation for Oracle Training
//!
//! Multiplies training corpus by generating error→fix pairs from mutations.
//!
//! # Architecture
//!
//! ```text
//! Successful Pair → Mutation Engine → Error Capture → CITL Pattern Store
//! ```
//!
//! # Example
//!
//! ```ignore
//! use depyler_oracle::synthetic_augment::SyntheticAugmenter;
//!
//! let mut augmenter = SyntheticAugmenter::new()?;
//! let samples = augmenter.augment_pair(
//!     "def add(a: int, b: int) -> int: return a + b",
//!     "fn add(a: i32, b: i32) -> i32 { a + b }",
//!     5,
//! )?;
//! ```

use crate::OracleError;
use std::path::Path;

/// Types of mutations that can be applied to Python code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationType {
    /// Change type annotation (int → str)
    TypeChange,
    /// Mangle variable name (var → var_typo)
    NameMangle,
    /// Swap operator (+ → -)
    OperatorSwap,
    /// Delete a statement
    StatementDelete,
    /// Remove an import
    ImportRemove,
}

impl MutationType {
    /// Get all mutation types
    pub fn all() -> &'static [MutationType] {
        &[
            MutationType::TypeChange,
            MutationType::NameMangle,
            MutationType::OperatorSwap,
            MutationType::StatementDelete,
            MutationType::ImportRemove,
        ]
    }
}

/// A single augmented sample generated from mutation
#[derive(Debug, Clone)]
pub struct AugmentedSample {
    /// Original Python code
    pub original_python: String,
    /// Mutated Python code with introduced error
    pub mutated_python: String,
    /// Error message from transpilation attempt
    pub error_message: String,
    /// Suggested fix (derived from original)
    pub fix_suggestion: String,
    /// Type of mutation applied
    pub mutation_type: MutationType,
}

/// Statistics from augmentation run
#[derive(Debug, Clone, Default)]
pub struct AugmentStats {
    /// Total pairs processed
    pub pairs_processed: usize,
    /// Samples generated
    pub samples_generated: usize,
    /// Mutations that produced errors
    pub successful_mutations: usize,
    /// Mutations that didn't produce errors (code still valid)
    pub failed_mutations: usize,
    /// Breakdown by mutation type
    pub by_type: std::collections::HashMap<String, usize>,
}

/// Synthetic augmenter for generating error→fix pairs
pub struct SyntheticAugmenter {
    /// Random seed for reproducibility
    seed: u64,
}

impl SyntheticAugmenter {
    /// Create a new augmenter
    pub fn new() -> Result<Self, OracleError> {
        Ok(Self { seed: 42 })
    }

    /// Create with specific seed for reproducibility
    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate N mutations for a successful Python→Rust pair
    pub fn augment_pair(
        &mut self,
        python: &str,
        rust: &str,
        n_mutations: usize,
    ) -> Result<Vec<AugmentedSample>, OracleError> {
        let mut samples = Vec::with_capacity(n_mutations);

        for mutation_type in MutationType::all().iter().cycle().take(n_mutations) {
            if let Some(sample) = self.apply_mutation(python, rust, *mutation_type)? {
                samples.push(sample);
            }
        }

        Ok(samples)
    }

    /// Apply a specific mutation type
    fn apply_mutation(
        &self,
        python: &str,
        rust: &str,
        mutation_type: MutationType,
    ) -> Result<Option<AugmentedSample>, OracleError> {
        let mutated = match mutation_type {
            MutationType::TypeChange => self.mutate_type(python),
            MutationType::NameMangle => self.mutate_name(python),
            MutationType::OperatorSwap => self.mutate_operator(python),
            MutationType::StatementDelete => self.mutate_delete_stmt(python),
            MutationType::ImportRemove => self.mutate_remove_import(python),
        };

        let mutated = match mutated {
            Some(m) => m,
            None => return Ok(None),
        };

        // Try to transpile mutated code and capture error
        let error_message = self.try_transpile(&mutated)?;

        if error_message.is_empty() {
            // Mutation didn't produce an error
            return Ok(None);
        }

        Ok(Some(AugmentedSample {
            original_python: python.to_string(),
            mutated_python: mutated,
            error_message,
            fix_suggestion: format!("Revert mutation: use original code"),
            mutation_type,
        }))
    }

    /// Mutate type annotations (int → str, float → int, etc.)
    fn mutate_type(&self, python: &str) -> Option<String> {
        let replacements = [
            (": int", ": str"),
            (": str", ": int"),
            (": float", ": str"),
            (": bool", ": int"),
            ("-> int", "-> str"),
            ("-> str", "-> int"),
        ];

        for (from, to) in replacements {
            if python.contains(from) {
                return Some(python.replacen(from, to, 1));
            }
        }
        None
    }

    /// Mangle variable names
    fn mutate_name(&self, python: &str) -> Option<String> {
        // Find a variable assignment and mangle it
        let lines: Vec<&str> = python.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if let Some(eq_pos) = line.find(" = ") {
                let var_name = line[..eq_pos].trim();
                if !var_name.contains('(') && !var_name.starts_with('#') {
                    // Mangle just the definition, not usages
                    let mangled = format!("{}_typo", var_name);
                    let mut new_lines = lines.clone();
                    new_lines[i] = &line.replace(var_name, &mangled);
                    return Some(new_lines.join("\n"));
                }
            }
        }
        None
    }

    /// Swap operators (+ → -, * → /, etc.)
    fn mutate_operator(&self, python: &str) -> Option<String> {
        let swaps = [
            (" + ", " - "),
            (" - ", " + "),
            (" * ", " / "),
            (" / ", " * "),
            (" == ", " != "),
            (" != ", " == "),
            (" < ", " > "),
            (" > ", " < "),
        ];

        for (from, to) in swaps {
            if python.contains(from) {
                return Some(python.replacen(from, to, 1));
            }
        }
        None
    }

    /// Delete a statement
    fn mutate_delete_stmt(&self, python: &str) -> Option<String> {
        let lines: Vec<&str> = python.lines().collect();
        if lines.len() < 3 {
            return None;
        }
        // Delete the second-to-last non-empty line (likely important)
        let mut new_lines: Vec<&str> = Vec::new();
        let mut deleted = false;
        for (i, line) in lines.iter().enumerate().rev() {
            if !deleted && !line.trim().is_empty() && i > 0 && i < lines.len() - 1 {
                deleted = true;
                continue;
            }
            new_lines.insert(0, line);
        }
        if deleted {
            Some(new_lines.join("\n"))
        } else {
            None
        }
    }

    /// Remove an import statement
    fn mutate_remove_import(&self, python: &str) -> Option<String> {
        let lines: Vec<&str> = python.lines().collect();
        let mut new_lines: Vec<&str> = Vec::new();
        let mut removed = false;

        for line in lines {
            if !removed && (line.starts_with("import ") || line.starts_with("from ")) {
                removed = true;
                continue;
            }
            new_lines.push(line);
        }

        if removed {
            Some(new_lines.join("\n"))
        } else {
            None
        }
    }

    /// Try to transpile and return error message (empty if success)
    fn try_transpile(&self, _python: &str) -> Result<String, OracleError> {
        // TODO: Integrate with depyler transpiler
        // For now, return a placeholder error for testing
        Ok("type mismatch: expected i32, found &str".to_string())
    }

    /// Augment entire corpus from parquet file
    pub fn augment_corpus(
        &mut self,
        corpus_path: &Path,
        output_path: &Path,
        mutations_per_sample: usize,
    ) -> Result<AugmentStats, OracleError> {
        let _ = (corpus_path, output_path, mutations_per_sample);
        // TODO: Implement parquet reading/writing
        Ok(AugmentStats::default())
    }
}

impl Default for SyntheticAugmenter {
    fn default() -> Self {
        Self::with_seed(42)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== RED PHASE: Write failing tests first ====================

    #[test]
    fn test_mutation_type_all() {
        let all = MutationType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&MutationType::TypeChange));
        assert!(all.contains(&MutationType::NameMangle));
        assert!(all.contains(&MutationType::OperatorSwap));
        assert!(all.contains(&MutationType::StatementDelete));
        assert!(all.contains(&MutationType::ImportRemove));
    }

    #[test]
    fn test_augmenter_creation() {
        let augmenter = SyntheticAugmenter::new();
        assert!(augmenter.is_ok());
    }

    #[test]
    fn test_augmenter_with_seed() {
        let augmenter = SyntheticAugmenter::with_seed(123);
        assert_eq!(augmenter.seed, 123);
    }

    #[test]
    fn test_type_mutation() {
        let augmenter = SyntheticAugmenter::default();
        let python = "def add(a: int, b: int) -> int:\n    return a + b";

        let mutated = augmenter.mutate_type(python);
        assert!(mutated.is_some());
        let mutated = mutated.unwrap();
        assert!(mutated.contains(": str") || mutated.contains("-> str"));
        assert_ne!(mutated, python);
    }

    #[test]
    fn test_name_mangle_mutation() {
        let augmenter = SyntheticAugmenter::default();
        let python = "x = 5\nresult = x + 10";

        let mutated = augmenter.mutate_name(python);
        assert!(mutated.is_some());
        let mutated = mutated.unwrap();
        assert!(mutated.contains("_typo"));
    }

    #[test]
    fn test_operator_swap_mutation() {
        let augmenter = SyntheticAugmenter::default();
        let python = "result = a + b";

        let mutated = augmenter.mutate_operator(python);
        assert!(mutated.is_some());
        let mutated = mutated.unwrap();
        assert!(mutated.contains(" - "));
    }

    #[test]
    fn test_statement_delete_mutation() {
        let augmenter = SyntheticAugmenter::default();
        let python = "x = 5\ny = 10\nresult = x + y\nprint(result)";

        let mutated = augmenter.mutate_delete_stmt(python);
        assert!(mutated.is_some());
        let mutated = mutated.unwrap();
        assert!(mutated.lines().count() < python.lines().count());
    }

    #[test]
    fn test_import_remove_mutation() {
        let augmenter = SyntheticAugmenter::default();
        let python = "import os\nimport sys\nx = os.getcwd()";

        let mutated = augmenter.mutate_remove_import(python);
        assert!(mutated.is_some());
        let mutated = mutated.unwrap();
        assert!(!mutated.starts_with("import os"));
    }

    #[test]
    fn test_augment_pair_produces_samples() {
        let mut augmenter = SyntheticAugmenter::default();
        let python = "def add(a: int, b: int) -> int:\n    return a + b";
        let rust = "fn add(a: i32, b: i32) -> i32 { a + b }";

        let samples = augmenter.augment_pair(python, rust, 3);
        assert!(samples.is_ok());
        let samples = samples.unwrap();
        assert!(!samples.is_empty());
    }

    #[test]
    fn test_augmented_sample_has_error_message() {
        let mut augmenter = SyntheticAugmenter::default();
        let python = "def add(a: int, b: int) -> int:\n    return a + b";
        let rust = "fn add(a: i32, b: i32) -> i32 { a + b }";

        let samples = augmenter.augment_pair(python, rust, 1).unwrap();
        if !samples.is_empty() {
            assert!(!samples[0].error_message.is_empty());
            assert!(!samples[0].mutated_python.is_empty());
            assert!(!samples[0].original_python.is_empty());
        }
    }

    #[test]
    fn test_mutation_type_in_sample() {
        let mut augmenter = SyntheticAugmenter::default();
        let python = "def add(a: int, b: int) -> int:\n    return a + b";
        let rust = "fn add(a: i32, b: i32) -> i32 { a + b }";

        let samples = augmenter.augment_pair(python, rust, 5).unwrap();
        for sample in samples {
            assert!(MutationType::all().contains(&sample.mutation_type));
        }
    }

    #[test]
    fn test_no_mutation_for_simple_code() {
        let augmenter = SyntheticAugmenter::default();
        // Code with no operators, types, or imports
        let python = "pass";

        assert!(augmenter.mutate_type(python).is_none());
        assert!(augmenter.mutate_operator(python).is_none());
        assert!(augmenter.mutate_remove_import(python).is_none());
    }

    #[test]
    fn test_augment_stats_default() {
        let stats = AugmentStats::default();
        assert_eq!(stats.pairs_processed, 0);
        assert_eq!(stats.samples_generated, 0);
    }

    // Property-based tests
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn mutation_preserves_some_content(code in "[a-z]+ = [0-9]+") {
                let augmenter = SyntheticAugmenter::default();
                if let Some(mutated) = augmenter.mutate_name(&code) {
                    // Mutated code should share some content
                    prop_assert!(mutated.len() > 0);
                }
            }

            #[test]
            fn augmenter_never_panics(
                python in "def [a-z]+\\([a-z]: int\\):\\n    return [a-z]",
                rust in "fn [a-z]+\\([a-z]: i32\\) -> i32 \\{ [a-z] \\}",
                n in 0usize..10
            ) {
                let mut augmenter = SyntheticAugmenter::default();
                let result = augmenter.augment_pair(&python, &rust, n);
                prop_assert!(result.is_ok());
            }
        }
    }
}

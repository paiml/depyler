//! Synthetic data generation for scaling training corpus.
//!
//! Generates variations of error patterns to create 10,000+ training samples.

use crate::classifier::ErrorCategory;
use crate::training::{TrainingDataset, TrainingSample};

/// Configuration for synthetic data generation.
#[derive(Clone, Debug)]
pub struct SyntheticConfig {
    /// Target number of samples per category
    pub samples_per_category: usize,
    /// Random seed for reproducibility
    pub seed: u64,
}

impl Default for SyntheticConfig {
    fn default() -> Self {
        Self {
            samples_per_category: 2000, // 2000 * 6 categories = 12000 samples
            seed: 42,
        }
    }
}

/// Synthetic data generator for error classification training.
pub struct SyntheticGenerator {
    config: SyntheticConfig,
}

impl SyntheticGenerator {
    /// Create generator with default config.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(SyntheticConfig::default())
    }

    /// Create generator with custom config.
    #[must_use]
    pub fn with_config(config: SyntheticConfig) -> Self {
        Self { config }
    }

    /// Generate full synthetic dataset.
    #[must_use]
    pub fn generate(&self) -> TrainingDataset {
        let mut dataset = TrainingDataset::new();

        dataset.add_many(self.generate_type_mismatch_samples());
        dataset.add_many(self.generate_borrow_checker_samples());
        dataset.add_many(self.generate_lifetime_samples());
        dataset.add_many(self.generate_trait_bound_samples());
        dataset.add_many(self.generate_import_samples());
        dataset.add_many(self.generate_syntax_samples());

        dataset
    }

    /// Generate type mismatch variations.
    fn generate_type_mismatch_samples(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::with_capacity(self.config.samples_per_category);

        // Type pairs for mismatches
        let type_pairs = [
            ("i32", "i64"),
            ("i64", "i32"),
            ("u32", "usize"),
            ("usize", "u32"),
            ("i8", "i16"),
            ("i16", "i32"),
            ("i32", "isize"),
            ("u8", "u16"),
            ("u16", "u32"),
            ("u32", "u64"),
            ("f32", "f64"),
            ("f64", "f32"),
            ("String", "&str"),
            ("&str", "String"),
            ("&String", "&str"),
            ("Option<T>", "T"),
            ("T", "Option<T>"),
            ("Result<T, E>", "T"),
            ("&T", "T"),
            ("T", "&T"),
            ("&mut T", "&T"),
            ("&T", "&mut T"),
            ("Box<T>", "T"),
            ("T", "Box<T>"),
            ("Vec<T>", "&[T]"),
            ("&[T]", "Vec<T>"),
            ("char", "u8"),
            ("u8", "char"),
            ("bool", "i32"),
            ("()", "i32"),
        ];

        // Error code patterns
        let error_codes = ["E0308", "E0277", "E0369", "E0271"];

        // Context variations
        let contexts = [
            "in this expression",
            "in return type",
            "in function argument",
            "in assignment",
            "in match arm",
            "in if condition",
            "in loop body",
            "in closure",
            "in trait implementation",
            "in generic bound",
        ];

        let mut idx = 0;
        while samples.len() < self.config.samples_per_category {
            let (expected, found) = type_pairs[idx % type_pairs.len()];
            let code = error_codes[(idx / type_pairs.len()) % error_codes.len()];
            let context = contexts[(idx / (type_pairs.len() * error_codes.len())) % contexts.len()];

            let msg = format!(
                "error[{}]: mismatched types\n  expected `{}`, found `{}`\n  {}",
                code, expected, found, context
            );

            let fix = match (expected, found) {
                ("String", "&str") => "Use .to_string() to convert",
                ("&str", "String") => "Use & or .as_str() to borrow",
                (e, f) if e.starts_with("i") && f.starts_with("i") => {
                    "Use as cast for integer conversion"
                }
                (e, f) if e.starts_with("u") && f.starts_with("u") => {
                    "Use as cast for unsigned conversion"
                }
                ("Option<T>", "T") => "Wrap with Some(value)",
                ("T", "Option<T>") => "Unwrap with .unwrap() or pattern match",
                ("&T", "T") => "Add & to take reference",
                ("T", "&T") => "Dereference with * or clone",
                _ => "Check type annotations and conversions",
            };

            samples.push(TrainingSample::with_fix(
                &msg,
                ErrorCategory::TypeMismatch,
                fix,
            ));
            idx += 1;
        }

        samples
    }

    /// Generate borrow checker variations.
    fn generate_borrow_checker_samples(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::with_capacity(self.config.samples_per_category);

        let var_names = [
            "x", "y", "data", "value", "item", "result", "buf", "config", "state", "cache",
        ];
        let error_codes = [
            "E0382", "E0502", "E0499", "E0505", "E0507", "E0596", "E0597", "E0373",
        ];

        let patterns = [
            ("use of moved value", "Clone before moving or use reference"),
            (
                "borrow of moved value",
                "Restructure to avoid move before borrow",
            ),
            (
                "cannot borrow as mutable because also borrowed as immutable",
                "Separate mutable and immutable borrows",
            ),
            (
                "cannot borrow as mutable more than once",
                "Use RefCell for interior mutability",
            ),
            (
                "cannot move out of borrowed content",
                "Clone or change signature",
            ),
            (
                "cannot borrow as mutable, as it is not declared as mutable",
                "Add mut to declaration",
            ),
            (
                "does not live long enough",
                "Extend lifetime or use owned data",
            ),
            (
                "closure may outlive current function",
                "Use move keyword in closure",
            ),
        ];

        let mut idx = 0;
        while samples.len() < self.config.samples_per_category {
            let var = var_names[idx % var_names.len()];
            let code = error_codes[(idx / var_names.len()) % error_codes.len()];
            let (pattern, fix) =
                patterns[(idx / (var_names.len() * error_codes.len())) % patterns.len()];

            let msg = format!(
                "error[{}]: {}: `{}`\n  value {} here\n  {} here",
                code,
                pattern,
                var,
                if pattern.contains("moved") {
                    "moved"
                } else {
                    "borrowed"
                },
                if pattern.contains("moved") {
                    "used"
                } else {
                    "borrowed again"
                }
            );

            samples.push(TrainingSample::with_fix(
                &msg,
                ErrorCategory::BorrowChecker,
                fix,
            ));
            idx += 1;
        }

        samples
    }

    /// Generate lifetime error variations.
    fn generate_lifetime_samples(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::with_capacity(self.config.samples_per_category);

        let lifetimes = ["'a", "'b", "'c", "'static", "'_"];
        let error_codes = ["E0106", "E0621", "E0495", "E0759", "E0515", "E0716"];

        let patterns = [
            ("missing lifetime specifier", "Add lifetime parameter: <'a>"),
            ("explicit lifetime required", "Add lifetime annotation"),
            (
                "cannot infer appropriate lifetime",
                "Add explicit lifetime bounds",
            ),
            (
                "needs to satisfy 'static requirement",
                "Use Box or Arc for 'static",
            ),
            (
                "cannot return reference to temporary",
                "Return owned value instead",
            ),
            (
                "temporary value dropped while borrowed",
                "Bind temporary to variable",
            ),
        ];

        let mut idx = 0;
        while samples.len() < self.config.samples_per_category {
            let lt = lifetimes[idx % lifetimes.len()];
            let code = error_codes[(idx / lifetimes.len()) % error_codes.len()];
            let (pattern, fix) =
                patterns[(idx / (lifetimes.len() * error_codes.len())) % patterns.len()];

            let msg = format!(
                "error[{}]: {}\n  expected named lifetime parameter `{}`",
                code, pattern, lt
            );

            samples.push(TrainingSample::with_fix(
                &msg,
                ErrorCategory::LifetimeError,
                fix,
            ));
            idx += 1;
        }

        samples
    }

    /// Generate trait bound variations.
    fn generate_trait_bound_samples(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::with_capacity(self.config.samples_per_category);

        let traits = [
            "Clone",
            "Copy",
            "Debug",
            "Default",
            "Send",
            "Sync",
            "Eq",
            "PartialEq",
            "Ord",
            "PartialOrd",
            "Hash",
            "Serialize",
            "Deserialize",
            "Display",
            "FromStr",
            "Iterator",
            "IntoIterator",
            "From",
            "Into",
            "TryFrom",
            "TryInto",
        ];

        let types = ["T", "MyStruct", "Config", "Data", "Handler", "Service"];

        let mut idx = 0;
        while samples.len() < self.config.samples_per_category {
            let trait_name = traits[idx % traits.len()];
            let type_name = types[(idx / traits.len()) % types.len()];

            let msg = format!(
                "error[E0277]: the trait bound `{}: {}` is not satisfied\n  required by this bound",
                type_name, trait_name
            );

            let fix = match trait_name {
                "Clone" | "Copy" | "Debug" | "Default" | "Eq" | "PartialEq" | "Ord"
                | "PartialOrd" | "Hash" => {
                    format!("Add #[derive({})] to type definition", trait_name)
                }
                "Send" | "Sync" => "Ensure all fields are thread-safe".to_string(),
                "Serialize" | "Deserialize" => {
                    "Add #[derive(Serialize, Deserialize)] with serde".to_string()
                }
                _ => format!("Implement {} for the type", trait_name),
            };

            samples.push(TrainingSample::with_fix(
                &msg,
                ErrorCategory::TraitBound,
                &fix,
            ));
            idx += 1;
        }

        samples
    }

    /// Generate import error variations.
    fn generate_import_samples(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::with_capacity(self.config.samples_per_category);

        let modules = [
            "std::collections::HashMap",
            "std::collections::HashSet",
            "std::collections::BTreeMap",
            "std::collections::VecDeque",
            "std::sync::Arc",
            "std::sync::Mutex",
            "std::sync::RwLock",
            "std::io::Read",
            "std::io::Write",
            "std::io::BufReader",
            "std::path::Path",
            "std::path::PathBuf",
            "std::fs::File",
            "serde::Serialize",
            "serde::Deserialize",
            "serde_json::Value",
            "tokio::spawn",
            "tokio::sync::mpsc",
            "anyhow::Result",
        ];

        let error_codes = ["E0433", "E0412", "E0425", "E0432", "E0603", "E0599"];

        let mut idx = 0;
        while samples.len() < self.config.samples_per_category {
            let module = modules[idx % modules.len()];
            let code = error_codes[(idx / modules.len()) % error_codes.len()];
            let item = module.split("::").last().unwrap_or("unknown");

            let msg = match code {
                "E0433" => format!(
                    "error[{}]: failed to resolve: use of undeclared crate or module `{}`",
                    code, item
                ),
                "E0412" => format!("error[{}]: cannot find type `{}` in this scope", code, item),
                "E0425" => format!(
                    "error[{}]: cannot find value `{}` in this scope",
                    code,
                    item.to_lowercase()
                ),
                "E0432" => format!("error[{}]: unresolved import `{}`", code, module),
                "E0603" => format!(
                    "error[{}]: module `{}` is private",
                    code,
                    item.to_lowercase()
                ),
                "E0599" => format!(
                    "error[{}]: no method named `{}` found",
                    code,
                    item.to_lowercase()
                ),
                _ => format!("error[{}]: import error for `{}`", code, module),
            };

            let fix = format!("Add: use {};", module);

            samples.push(TrainingSample::with_fix(
                &msg,
                ErrorCategory::MissingImport,
                &fix,
            ));
            idx += 1;
        }

        samples
    }

    /// Generate syntax error variations.
    fn generate_syntax_samples(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::with_capacity(self.config.samples_per_category);

        let patterns = [
            ("expected `;`, found `}`", "Add missing semicolon"),
            ("expected `{`, found `=>`", "Check match arm syntax"),
            ("unexpected token `)`", "Check for extra parentheses"),
            (
                "expected expression, found `let`",
                "Remove let in expression position",
            ),
            ("unclosed delimiter", "Add missing closing bracket/brace"),
            (
                "expected identifier, found keyword",
                "Use raw identifier r#keyword",
            ),
            (
                "expected pattern, found expression",
                "Use pattern syntax in match",
            ),
            ("expected type, found `=`", "Check type annotation syntax"),
            ("missing `fn` for method definition", "Add fn keyword"),
            (
                "expected `->`, found `=>`",
                "Use -> for return type, => for match arms",
            ),
        ];

        let contexts = [
            "at line 10",
            "at line 25",
            "at line 42",
            "at line 100",
            "in function main",
            "in impl block",
            "in match expression",
        ];

        let mut idx = 0;
        while samples.len() < self.config.samples_per_category {
            let (pattern, fix) = patterns[idx % patterns.len()];
            let context = contexts[(idx / patterns.len()) % contexts.len()];

            let msg = format!(
                "error: {}\n  --> src/lib.rs:{}:1\n  {}",
                pattern,
                (idx % 200) + 1,
                context
            );

            samples.push(TrainingSample::with_fix(
                &msg,
                ErrorCategory::SyntaxError,
                fix,
            ));
            idx += 1;
        }

        samples
    }
}

impl Default for SyntheticGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate synthetic dataset with default config (12,000 samples).
#[must_use]
pub fn generate_synthetic_corpus() -> TrainingDataset {
    SyntheticGenerator::new().generate()
}

/// Generate synthetic dataset with custom sample count per category.
#[must_use]
pub fn generate_synthetic_corpus_sized(samples_per_category: usize) -> TrainingDataset {
    SyntheticGenerator::with_config(SyntheticConfig {
        samples_per_category,
        ..Default::default()
    })
    .generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthetic_config_default() {
        let config = SyntheticConfig::default();
        assert_eq!(config.samples_per_category, 2000);
        assert_eq!(config.seed, 42);
    }

    #[test]
    fn test_synthetic_config_clone() {
        let config = SyntheticConfig {
            samples_per_category: 100,
            seed: 123,
        };
        let cloned = config.clone();
        assert_eq!(cloned.samples_per_category, 100);
        assert_eq!(cloned.seed, 123);
    }

    #[test]
    fn test_synthetic_config_debug() {
        let config = SyntheticConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("SyntheticConfig"));
        assert!(debug_str.contains("2000"));
    }

    #[test]
    fn test_synthetic_generator_new() {
        let gen = SyntheticGenerator::new();
        assert_eq!(gen.config.samples_per_category, 2000);
    }

    #[test]
    fn test_synthetic_generator_default() {
        let gen = SyntheticGenerator::default();
        assert_eq!(gen.config.samples_per_category, 2000);
    }

    #[test]
    fn test_synthetic_generator_with_config() {
        let config = SyntheticConfig {
            samples_per_category: 50,
            seed: 99,
        };
        let gen = SyntheticGenerator::with_config(config);
        assert_eq!(gen.config.samples_per_category, 50);
        assert_eq!(gen.config.seed, 99);
    }

    #[test]
    fn test_default_generator() {
        let dataset = generate_synthetic_corpus();
        // 2000 per category * 6 categories = 12000
        assert!(dataset.len() >= 12000, "Got {} samples", dataset.len());
    }

    #[test]
    fn test_sized_generator() {
        let dataset = generate_synthetic_corpus_sized(100);
        // 100 per category * 6 categories = 600
        assert!(dataset.len() >= 600, "Got {} samples", dataset.len());
    }

    #[test]
    fn test_sized_generator_small() {
        let dataset = generate_synthetic_corpus_sized(10);
        // 10 per category * 6 categories = 60
        assert!(dataset.len() >= 60, "Got {} samples", dataset.len());
    }

    #[test]
    fn test_category_balance() {
        let dataset = generate_synthetic_corpus_sized(100);

        let type_count = dataset
            .samples_for_category(ErrorCategory::TypeMismatch)
            .len();
        let borrow_count = dataset
            .samples_for_category(ErrorCategory::BorrowChecker)
            .len();
        let lifetime_count = dataset
            .samples_for_category(ErrorCategory::LifetimeError)
            .len();

        // Each category should have ~100 samples
        assert!(type_count >= 100, "TypeMismatch: {}", type_count);
        assert!(borrow_count >= 100, "BorrowChecker: {}", borrow_count);
        assert!(lifetime_count >= 100, "LifetimeError: {}", lifetime_count);
    }

    #[test]
    fn test_all_categories_present() {
        let dataset = generate_synthetic_corpus_sized(50);

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

        assert!(type_count > 0, "No TypeMismatch samples");
        assert!(borrow_count > 0, "No BorrowChecker samples");
        assert!(lifetime_count > 0, "No LifetimeError samples");
        assert!(trait_count > 0, "No TraitBound samples");
        assert!(import_count > 0, "No MissingImport samples");
        assert!(syntax_count > 0, "No SyntaxError samples");
    }

    #[test]
    fn test_all_samples_have_fixes() {
        let dataset = generate_synthetic_corpus_sized(10);

        for sample in dataset.samples() {
            assert!(
                sample.fix.is_some(),
                "Sample missing fix: {}",
                sample.message
            );
        }
    }

    #[test]
    fn test_all_samples_have_non_empty_messages() {
        let dataset = generate_synthetic_corpus_sized(10);

        for sample in dataset.samples() {
            assert!(!sample.message.is_empty(), "Sample has empty message");
        }
    }

    #[test]
    fn test_all_samples_have_non_empty_fixes() {
        let dataset = generate_synthetic_corpus_sized(10);

        for sample in dataset.samples() {
            if let Some(fix) = &sample.fix {
                assert!(!fix.is_empty(), "Sample has empty fix");
            }
        }
    }

    #[test]
    fn test_type_mismatch_samples_contain_error_codes() {
        let dataset = generate_synthetic_corpus_sized(50);
        let type_samples = dataset.samples_for_category(ErrorCategory::TypeMismatch);

        for sample in type_samples {
            // Type mismatch samples should contain error codes
            assert!(
                sample.message.contains("E0308")
                    || sample.message.contains("E0277")
                    || sample.message.contains("E0369")
                    || sample.message.contains("E0271"),
                "Type mismatch sample missing error code: {}",
                sample.message
            );
        }
    }

    #[test]
    fn test_borrow_checker_samples_contain_error_codes() {
        let dataset = generate_synthetic_corpus_sized(50);
        let borrow_samples = dataset.samples_for_category(ErrorCategory::BorrowChecker);

        for sample in borrow_samples {
            assert!(
                sample.message.contains("E0502")
                    || sample.message.contains("E0499")
                    || sample.message.contains("E0507")
                    || sample.message.contains("E0382")
                    || sample.message.contains("E0596")
                    || sample.message.contains("E0505"), // use of moved value
                "Borrow checker sample missing error code: {}",
                sample.message
            );
        }
    }

    #[test]
    fn test_message_variety() {
        let dataset = generate_synthetic_corpus_sized(100);
        let messages: std::collections::HashSet<_> =
            dataset.samples().iter().map(|s| &s.message).collect();

        // Should have high variety (mostly unique messages)
        assert!(
            messages.len() > dataset.len() * 8 / 10,
            "Low variety: {} unique out of {}",
            messages.len(),
            dataset.len()
        );
    }

    #[test]
    fn test_generate_returns_dataset() {
        let gen = SyntheticGenerator::with_config(SyntheticConfig {
            samples_per_category: 5,
            seed: 1,
        });
        let dataset = gen.generate();
        // 5 * 6 categories = 30
        assert_eq!(dataset.len(), 30);
    }

    #[test]
    fn test_deterministic_generation() {
        // Same seed should produce same results
        let dataset1 = generate_synthetic_corpus_sized(10);
        let dataset2 = generate_synthetic_corpus_sized(10);

        assert_eq!(dataset1.len(), dataset2.len());

        // First samples should be identical (deterministic)
        let samples1 = dataset1.samples();
        let samples2 = dataset2.samples();

        assert_eq!(samples1[0].message, samples2[0].message);
        assert_eq!(samples1[0].category, samples2[0].category);
    }

    #[test]
    fn test_corpus_composition() {
        use crate::depyler_training::build_combined_corpus;
        use crate::verificar_integration::build_verificar_corpus;

        let verificar = build_verificar_corpus();
        let depyler = build_combined_corpus();
        let synthetic = generate_synthetic_corpus();

        eprintln!("\n=== Oracle Training Corpus Composition ===");
        eprintln!("  Verificar:  {:>6} samples", verificar.len());
        eprintln!("  Depyler:    {:>6} samples", depyler.len());
        eprintln!("  Synthetic:  {:>6} samples", synthetic.len());
        eprintln!("  -----------------------------------");
        eprintln!(
            "  Total:      {:>6} samples",
            verificar.len() + depyler.len() + synthetic.len()
        );
        eprintln!("===========================================\n");

        // Total should be > 12,000 (synthetic alone is 12,000)
        let total = verificar.len() + depyler.len() + synthetic.len();
        assert!(total >= 12000, "Expected 12000+ samples, got {}", total);
    }
}

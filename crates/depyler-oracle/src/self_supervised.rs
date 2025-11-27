//! Self-Supervised Corpus Generation for Oracle Training.
//!
//! This module implements the metaheuristic oracle specification:
//! - Parse Python stdlib to extract function signatures
//! - Generate Python examples programmatically
//! - Transpile through Depyler
//! - Compile with rustc
//! - Auto-label errors by error code
//!
//! # References
//!
//! - Storn & Price (1997): Differential Evolution
//! - Ratner et al. (2017): Snorkel weak supervision

use crate::{ErrorCategory, TrainingDataset};
use anyhow::Result;
use aprender::synthetic::{
    DiversityMonitor, DiversityScore, QualityDegradationDetector, SyntheticConfig,
    SyntheticGenerator,
};
use std::collections::HashMap;

// ============================================================================
// Domain Types (Phase 1: Stdlib Parser)
// ============================================================================

/// Represents a Python stdlib function signature.
#[derive(Debug, Clone, PartialEq)]
pub struct StdlibFunction {
    /// Module path (e.g., "os.path")
    pub module: String,
    /// Function name (e.g., "join")
    pub name: String,
    /// Full signature string (e.g., "(path, *paths) -> str")
    pub signature: String,
    /// Argument types parsed from signature
    pub arg_types: Vec<PyType>,
    /// Return type if annotated
    pub return_type: Option<PyType>,
    /// Examples extracted from docstrings
    pub docstring_examples: Vec<String>,
}

/// Python type representation for code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyType {
    Int,
    Float,
    Str,
    Bool,
    Bytes,
    List(Box<PyType>),
    Dict(Box<PyType>, Box<PyType>),
    Tuple(Vec<PyType>),
    Optional(Box<PyType>),
    Any,
    Path,
    FileHandle,
    Callable,
    Iterator(Box<PyType>),
}

impl PyType {
    /// Generate a sample value for this type.
    #[must_use]
    pub fn sample_value(&self) -> String {
        match self {
            PyType::Int => "42".to_string(),
            PyType::Float => "3.14".to_string(),
            PyType::Str => "\"hello\"".to_string(),
            PyType::Bool => "True".to_string(),
            PyType::Bytes => "b\"data\"".to_string(),
            PyType::List(inner) => format!("[{}]", inner.sample_value()),
            PyType::Dict(k, v) => format!("{{{}: {}}}", k.sample_value(), v.sample_value()),
            PyType::Tuple(types) => {
                let vals: Vec<_> = types.iter().map(PyType::sample_value).collect();
                format!("({})", vals.join(", "))
            }
            PyType::Optional(inner) => inner.sample_value(),
            PyType::Any => "None".to_string(),
            PyType::Path => "Path(\"/tmp/test\")".to_string(),
            PyType::FileHandle => "open(\"/tmp/test.txt\")".to_string(),
            PyType::Callable => "lambda x: x".to_string(),
            PyType::Iterator(inner) => format!("iter([{}])", inner.sample_value()),
        }
    }
}

// ============================================================================
// Phase 2: Example Generator
// ============================================================================

/// Generated Python example for corpus building.
#[derive(Debug, Clone)]
pub struct PythonExample {
    /// The Python source code
    pub source: String,
    /// The stdlib function being exercised
    pub target_function: String,
    /// Generation strategy used
    pub strategy: GenerationStrategy,
    /// Content hash for deduplication
    pub content_hash: u64,
}

/// Strategy used for generating examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationStrategy {
    /// Extract example from docstring
    DocstringMining,
    /// Enumerate valid type combinations
    TypeEnumeration,
    /// Generate boundary/edge cases
    EdgeCases,
    /// Intentionally invalid inputs to induce errors
    ErrorInduction,
    /// Chain multiple stdlib calls
    Composition,
}

/// Generator for Python examples from stdlib signatures.
#[derive(Debug)]
pub struct PythonExampleGenerator {
    stdlib_funcs: Vec<StdlibFunction>,
    diversity_monitor: DiversityMonitor,
}

impl PythonExampleGenerator {
    /// Create a new generator from stdlib function signatures.
    #[must_use]
    pub fn new(stdlib_funcs: Vec<StdlibFunction>) -> Self {
        Self {
            stdlib_funcs,
            diversity_monitor: DiversityMonitor::new(100), // Window size of 100
        }
    }

    /// Get the number of stdlib functions available.
    #[must_use]
    pub fn function_count(&self) -> usize {
        self.stdlib_funcs.len()
    }

    /// Check current diversity score.
    #[must_use]
    pub fn diversity_score(&self) -> DiversityScore {
        self.diversity_monitor.latest().unwrap_or_default()
    }
}

impl SyntheticGenerator for PythonExampleGenerator {
    type Input = StdlibFunction;
    type Output = PythonExample;

    fn generate(
        &self,
        seeds: &[Self::Input],
        config: &SyntheticConfig,
    ) -> aprender::error::Result<Vec<Self::Output>> {
        let mut examples = Vec::new();
        let target_count = (seeds.len() as f32 * config.augmentation_ratio) as usize;

        for func in seeds.iter().take(target_count.max(seeds.len())) {
            // Generate from docstring examples
            for doc_example in &func.docstring_examples {
                let example = PythonExample {
                    source: doc_example.clone(),
                    target_function: format!("{}.{}", func.module, func.name),
                    strategy: GenerationStrategy::DocstringMining,
                    content_hash: hash_content(doc_example),
                };
                if self.quality_score(&example, func) >= config.quality_threshold {
                    examples.push(example);
                }
            }

            // Generate type enumeration examples
            let type_example = generate_type_example(func);
            let example = PythonExample {
                source: type_example.clone(),
                target_function: format!("{}.{}", func.module, func.name),
                strategy: GenerationStrategy::TypeEnumeration,
                content_hash: hash_content(&type_example),
            };
            if self.quality_score(&example, func) >= config.quality_threshold {
                examples.push(example);
            }

            // Generate error induction examples
            let error_example = generate_error_example(func);
            let example = PythonExample {
                source: error_example.clone(),
                target_function: format!("{}.{}", func.module, func.name),
                strategy: GenerationStrategy::ErrorInduction,
                content_hash: hash_content(&error_example),
            };
            // Error examples always included (we want errors!)
            examples.push(example);
        }

        Ok(examples)
    }

    fn quality_score(&self, generated: &Self::Output, _seed: &Self::Input) -> f32 {
        // Basic quality heuristics
        let mut score: f32 = 0.5;

        // Has actual code
        if !generated.source.trim().is_empty() {
            score += 0.2;
        }

        // Contains the target function
        if generated.source.contains(generated.target_function.split('.').next_back().unwrap_or("")) {
            score += 0.2;
        }

        // Not too short (trivial)
        if generated.source.len() > 20 {
            score += 0.1;
        }

        score.min(1.0)
    }

    fn diversity_score(&self, batch: &[Self::Output]) -> f32 {
        if batch.is_empty() {
            return 0.0;
        }

        use std::collections::HashSet;
        let unique_hashes: HashSet<_> = batch.iter().map(|e| e.content_hash).collect();
        let unique_strategies: HashSet<_> = batch.iter().map(|e| e.strategy).collect();
        let unique_functions: HashSet<_> = batch.iter().map(|e| &e.target_function).collect();

        let hash_diversity = unique_hashes.len() as f32 / batch.len() as f32;
        let strategy_diversity = unique_strategies.len() as f32 / 5.0; // 5 strategies
        let function_diversity = unique_functions.len() as f32 / batch.len().min(100) as f32;

        (hash_diversity + strategy_diversity + function_diversity) / 3.0
    }
}

// ============================================================================
// Phase 3: Pipeline Integration
// ============================================================================

/// Result of transpiling and compiling a Python example.
#[derive(Debug, Clone)]
pub struct TranspileResult {
    /// Original Python source
    pub python_source: String,
    /// Generated Rust code (if transpilation succeeded)
    pub rust_output: Option<String>,
    /// Transpilation error (if any)
    pub transpile_error: Option<String>,
    /// Compilation errors from rustc
    pub compile_errors: Vec<RustcError>,
    /// Content hash for deduplication
    pub content_hash: u64,
}

/// Parsed rustc error.
#[derive(Debug, Clone)]
pub struct RustcError {
    /// Error code (e.g., "E0308")
    pub code: String,
    /// Full error message
    pub message: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Compiler suggestion if available
    pub suggestion: Option<String>,
}

// ============================================================================
// Phase 3: Auto-Labeler
// ============================================================================

/// Maps rustc error codes to Oracle categories.
///
/// # Error Code Mapping Strategy
/// - E03xx: Type system errors → TypeMismatch
/// - E04xx: Name resolution → MissingImport/SyntaxError
/// - E05xx: Borrow checker → BorrowChecker
/// - E06xx: Lifetime errors → LifetimeError
#[must_use]
pub fn auto_label(error: &RustcError) -> ErrorCategory {
    match error.code.as_str() {
        // Type mismatches
        "E0308" | "E0277" | "E0282" | "E0283" => ErrorCategory::TypeMismatch,

        // Borrow checker
        "E0382" | "E0499" | "E0502" | "E0503" | "E0505" | "E0507" | "E0596" | "E0597" => {
            ErrorCategory::BorrowChecker
        }

        // Missing imports
        "E0432" | "E0433" | "E0412" => ErrorCategory::MissingImport,

        // Syntax-like errors
        "E0425" | "E0423" | "E0424" | "E0609" => ErrorCategory::SyntaxError,

        // Lifetime errors
        "E0106" | "E0495" | "E0621" => ErrorCategory::LifetimeError,

        // Trait bounds
        "E0599" | "E0600" | "E0369" | "E0631" => ErrorCategory::TraitBound,

        _ => ErrorCategory::Other,
    }
}

// ============================================================================
// Phase 4: Corpus Generation Pipeline
// ============================================================================

/// Configuration for corpus generation.
#[derive(Debug, Clone)]
pub struct CorpusConfig {
    /// Target number of samples to generate
    pub target_samples: usize,
    /// Batch size for processing
    pub batch_size: usize,
    /// Quality threshold for accepting samples
    pub quality_threshold: f32,
    /// Maximum duplicate rate before Andon
    pub max_duplicate_rate: f32,
}

impl Default for CorpusConfig {
    fn default() -> Self {
        Self {
            target_samples: 50_000,
            batch_size: 100,
            quality_threshold: 0.7,
            max_duplicate_rate: 0.05,
        }
    }
}

/// Metrics collected during corpus generation.
#[derive(Debug, Clone, Default)]
pub struct CorpusMetrics {
    /// Total samples generated
    pub total_generated: usize,
    /// Samples that passed quality filter
    pub accepted: usize,
    /// Samples rejected for quality
    pub rejected_quality: usize,
    /// Samples rejected as duplicates
    pub rejected_duplicate: usize,
    /// Distribution of error categories
    pub category_distribution: HashMap<ErrorCategory, usize>,
    /// Unique error codes seen
    pub unique_error_codes: usize,
    /// Current diversity score
    pub diversity_score: f32,
}

impl CorpusMetrics {
    /// Calculate acceptance rate.
    #[must_use]
    pub fn acceptance_rate(&self) -> f32 {
        if self.total_generated == 0 {
            0.0
        } else {
            self.accepted as f32 / self.total_generated as f32
        }
    }

    /// Calculate duplicate rate.
    #[must_use]
    pub fn duplicate_rate(&self) -> f32 {
        if self.total_generated == 0 {
            0.0
        } else {
            self.rejected_duplicate as f32 / self.total_generated as f32
        }
    }

    /// Calculate class imbalance ratio (max / min).
    #[must_use]
    pub fn imbalance_ratio(&self) -> f32 {
        if self.category_distribution.is_empty() {
            return 0.0;
        }
        let max = *self.category_distribution.values().max().unwrap_or(&0) as f32;
        let min = *self.category_distribution.values().min().unwrap_or(&1).max(&1) as f32;
        max / min
    }
}

/// Self-supervised corpus generator orchestrating the full pipeline.
#[allow(dead_code)] // Fields used in future phases
pub struct SelfSupervisedCorpusGenerator {
    generator: PythonExampleGenerator,
    config: CorpusConfig,
    quality_detector: QualityDegradationDetector,
    seen_hashes: std::collections::HashSet<u64>,
    metrics: CorpusMetrics,
}

impl SelfSupervisedCorpusGenerator {
    /// Create a new corpus generator.
    #[must_use]
    pub fn new(stdlib_funcs: Vec<StdlibFunction>, config: CorpusConfig) -> Self {
        Self {
            generator: PythonExampleGenerator::new(stdlib_funcs),
            config: config.clone(),
            quality_detector: QualityDegradationDetector::new(config.quality_threshold, 100),
            seen_hashes: std::collections::HashSet::new(),
            metrics: CorpusMetrics::default(),
        }
    }

    /// Get current metrics.
    #[must_use]
    pub fn metrics(&self) -> &CorpusMetrics {
        &self.metrics
    }

    /// Generate corpus from stdlib functions.
    pub fn generate(&mut self) -> Result<TrainingDataset> {
        let dataset = TrainingDataset::new();

        // TODO: Implement full pipeline
        // 1. Generate Python examples from stdlib
        // 2. Transpile each through Depyler
        // 3. Compile with rustc
        // 4. Extract errors and auto-label
        // 5. Add to dataset with deduplication

        Ok(dataset)
    }

    /// Add a transpile result to the corpus.
    pub fn add_result(&mut self, result: &TranspileResult) -> bool {
        self.metrics.total_generated += 1;

        // Check for duplicate
        if self.seen_hashes.contains(&result.content_hash) {
            self.metrics.rejected_duplicate += 1;
            return false;
        }
        self.seen_hashes.insert(result.content_hash);

        // Process compile errors
        for error in &result.compile_errors {
            let category = auto_label(error);
            *self.metrics.category_distribution.entry(category).or_insert(0) += 1;
        }

        self.metrics.accepted += 1;
        true
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn hash_content(content: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

fn generate_type_example(func: &StdlibFunction) -> String {
    let args: Vec<_> = func.arg_types.iter().map(PyType::sample_value).collect();
    format!(
        "from {} import {}\nresult = {}({})",
        func.module,
        func.name,
        func.name,
        args.join(", ")
    )
}

fn generate_error_example(func: &StdlibFunction) -> String {
    // Generate intentionally wrong type to induce error
    format!(
        "from {} import {}\nresult = {}(None)  # Wrong type",
        func.module, func.name, func.name
    )
}

// ============================================================================
// EXTREME TDD: Tests (RED PHASE - These should FAIL initially)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Phase 1: Stdlib Parser Tests
    // ========================================================================

    #[test]
    fn test_pytype_sample_value_int() {
        assert_eq!(PyType::Int.sample_value(), "42");
    }

    #[test]
    fn test_pytype_sample_value_str() {
        assert_eq!(PyType::Str.sample_value(), "\"hello\"");
    }

    #[test]
    fn test_pytype_sample_value_list() {
        let list_type = PyType::List(Box::new(PyType::Int));
        assert_eq!(list_type.sample_value(), "[42]");
    }

    #[test]
    fn test_pytype_sample_value_dict() {
        let dict_type = PyType::Dict(Box::new(PyType::Str), Box::new(PyType::Int));
        assert_eq!(dict_type.sample_value(), "{\"hello\": 42}");
    }

    #[test]
    fn test_stdlib_function_creation() {
        let func = StdlibFunction {
            module: "os.path".to_string(),
            name: "join".to_string(),
            signature: "(path, *paths) -> str".to_string(),
            arg_types: vec![PyType::Str, PyType::Str],
            return_type: Some(PyType::Str),
            docstring_examples: vec!["os.path.join('/home', 'user')".to_string()],
        };

        assert_eq!(func.module, "os.path");
        assert_eq!(func.name, "join");
        assert_eq!(func.arg_types.len(), 2);
    }

    // ========================================================================
    // Phase 2: Example Generator Tests
    // ========================================================================

    fn sample_stdlib_function() -> StdlibFunction {
        StdlibFunction {
            module: "os.path".to_string(),
            name: "join".to_string(),
            signature: "(path, *paths) -> str".to_string(),
            arg_types: vec![PyType::Str, PyType::Str],
            return_type: Some(PyType::Str),
            docstring_examples: vec!["os.path.join('/home', 'user')".to_string()],
        }
    }

    #[test]
    fn test_python_example_generator_creation() {
        let funcs = vec![sample_stdlib_function()];
        let gen = PythonExampleGenerator::new(funcs);
        assert_eq!(gen.function_count(), 1);
    }

    #[test]
    fn test_python_example_generator_generates_examples() {
        let funcs = vec![sample_stdlib_function()];
        let gen = PythonExampleGenerator::new(funcs.clone());
        let config = SyntheticConfig::default();

        let examples = gen.generate(&funcs, &config).expect("generation should succeed");

        // Should generate at least docstring + type + error examples
        assert!(examples.len() >= 2, "Expected at least 2 examples, got {}", examples.len());
    }

    #[test]
    fn test_python_example_generator_quality_score() {
        let func = sample_stdlib_function();
        let gen = PythonExampleGenerator::new(vec![func.clone()]);

        let good_example = PythonExample {
            source: "os.path.join('/home', 'user')".to_string(),
            target_function: "os.path.join".to_string(),
            strategy: GenerationStrategy::DocstringMining,
            content_hash: 12345,
        };

        let score = gen.quality_score(&good_example, &func);
        assert!(score >= 0.7, "Good example should have high quality score: {}", score);
    }

    #[test]
    fn test_python_example_generator_diversity_score() {
        let func = sample_stdlib_function();
        let gen = PythonExampleGenerator::new(vec![func]);

        let examples = vec![
            PythonExample {
                source: "example1".to_string(),
                target_function: "os.path.join".to_string(),
                strategy: GenerationStrategy::DocstringMining,
                content_hash: 1,
            },
            PythonExample {
                source: "example2".to_string(),
                target_function: "os.path.join".to_string(),
                strategy: GenerationStrategy::TypeEnumeration,
                content_hash: 2,
            },
            PythonExample {
                source: "example3".to_string(),
                target_function: "os.path.exists".to_string(),
                strategy: GenerationStrategy::ErrorInduction,
                content_hash: 3,
            },
        ];

        let score = SyntheticGenerator::diversity_score(&gen, &examples);
        assert!(score > 0.5, "Diverse examples should have high diversity: {:.2}", score);
    }

    // ========================================================================
    // Phase 3: Auto-Labeler Tests
    // ========================================================================

    #[test]
    fn test_auto_label_type_mismatch() {
        let error = RustcError {
            code: "E0308".to_string(),
            message: "mismatched types".to_string(),
            line: 10,
            column: 5,
            suggestion: None,
        };
        assert_eq!(auto_label(&error), ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_auto_label_borrow_checker() {
        let error = RustcError {
            code: "E0382".to_string(),
            message: "use of moved value".to_string(),
            line: 10,
            column: 5,
            suggestion: None,
        };
        assert_eq!(auto_label(&error), ErrorCategory::BorrowChecker);
    }

    #[test]
    fn test_auto_label_missing_import() {
        let error = RustcError {
            code: "E0433".to_string(),
            message: "failed to resolve".to_string(),
            line: 10,
            column: 5,
            suggestion: None,
        };
        assert_eq!(auto_label(&error), ErrorCategory::MissingImport);
    }

    #[test]
    fn test_auto_label_lifetime() {
        let error = RustcError {
            code: "E0106".to_string(),
            message: "missing lifetime specifier".to_string(),
            line: 10,
            column: 5,
            suggestion: None,
        };
        assert_eq!(auto_label(&error), ErrorCategory::LifetimeError);
    }

    #[test]
    fn test_auto_label_trait_bound() {
        let error = RustcError {
            code: "E0599".to_string(),
            message: "no method named".to_string(),
            line: 10,
            column: 5,
            suggestion: None,
        };
        assert_eq!(auto_label(&error), ErrorCategory::TraitBound);
    }

    #[test]
    fn test_auto_label_unknown() {
        let error = RustcError {
            code: "E9999".to_string(),
            message: "unknown error".to_string(),
            line: 10,
            column: 5,
            suggestion: None,
        };
        assert_eq!(auto_label(&error), ErrorCategory::Other);
    }

    // ========================================================================
    // Phase 4: Corpus Generator Tests
    // ========================================================================

    #[test]
    fn test_corpus_config_defaults() {
        let config = CorpusConfig::default();
        assert_eq!(config.target_samples, 50_000);
        assert_eq!(config.batch_size, 100);
        assert!((config.quality_threshold - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn test_corpus_metrics_acceptance_rate() {
        let mut metrics = CorpusMetrics::default();
        metrics.total_generated = 100;
        metrics.accepted = 80;
        assert!((metrics.acceptance_rate() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_corpus_metrics_duplicate_rate() {
        let mut metrics = CorpusMetrics::default();
        metrics.total_generated = 100;
        metrics.rejected_duplicate = 5;
        assert!((metrics.duplicate_rate() - 0.05).abs() < f32::EPSILON);
    }

    #[test]
    fn test_corpus_metrics_imbalance_ratio() {
        let mut metrics = CorpusMetrics::default();
        metrics.category_distribution.insert(ErrorCategory::TypeMismatch, 100);
        metrics.category_distribution.insert(ErrorCategory::BorrowChecker, 50);
        metrics.category_distribution.insert(ErrorCategory::Other, 10);

        assert!((metrics.imbalance_ratio() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_self_supervised_generator_creation() {
        let funcs = vec![sample_stdlib_function()];
        let config = CorpusConfig::default();
        let gen = SelfSupervisedCorpusGenerator::new(funcs, config);

        assert_eq!(gen.metrics().total_generated, 0);
        assert_eq!(gen.metrics().accepted, 0);
    }

    #[test]
    fn test_self_supervised_generator_add_result() {
        let funcs = vec![sample_stdlib_function()];
        let config = CorpusConfig::default();
        let mut gen = SelfSupervisedCorpusGenerator::new(funcs, config);

        let result = TranspileResult {
            python_source: "test code".to_string(),
            rust_output: Some("fn main() {}".to_string()),
            transpile_error: None,
            compile_errors: vec![RustcError {
                code: "E0308".to_string(),
                message: "mismatched types".to_string(),
                line: 1,
                column: 1,
                suggestion: None,
            }],
            content_hash: 12345,
        };

        assert!(gen.add_result(&result));
        assert_eq!(gen.metrics().accepted, 1);
        assert_eq!(gen.metrics().category_distribution.get(&ErrorCategory::TypeMismatch), Some(&1));
    }

    #[test]
    fn test_self_supervised_generator_deduplication() {
        let funcs = vec![sample_stdlib_function()];
        let config = CorpusConfig::default();
        let mut gen = SelfSupervisedCorpusGenerator::new(funcs, config);

        let result = TranspileResult {
            python_source: "test code".to_string(),
            rust_output: None,
            transpile_error: None,
            compile_errors: vec![],
            content_hash: 12345, // Same hash
        };

        assert!(gen.add_result(&result));
        assert!(!gen.add_result(&result)); // Duplicate rejected
        assert_eq!(gen.metrics().rejected_duplicate, 1);
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_generate_type_example() {
        let func = sample_stdlib_function();
        let example = generate_type_example(&func);

        assert!(example.contains("from os.path import join"));
        assert!(example.contains("join("));
    }

    #[test]
    fn test_generate_error_example() {
        let func = sample_stdlib_function();
        let example = generate_error_example(&func);

        assert!(example.contains("None"));
        assert!(example.contains("join"));
    }

    #[test]
    fn test_hash_content_deterministic() {
        let content = "test content";
        let hash1 = hash_content(content);
        let hash2 = hash_content(content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_content_different_for_different_content() {
        let hash1 = hash_content("content A");
        let hash2 = hash_content("content B");
        assert_ne!(hash1, hash2);
    }
}

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
use aprender::metaheuristics::{
    Budget, DifferentialEvolution, OptimizationResult, PerturbativeMetaheuristic, SearchSpace,
};
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
// Phase 4: Metaheuristic Optimizer
// ============================================================================

/// Parameters to optimize using Differential Evolution.
///
/// The DE optimizer finds the best combination of these parameters
/// to maximize Oracle classification accuracy on the generated corpus.
#[derive(Debug, Clone)]
pub struct GenerationParams {
    /// Weight for DocstringMining strategy (0.0-1.0)
    pub weight_docstring: f64,
    /// Weight for TypeEnumeration strategy (0.0-1.0)
    pub weight_type_enum: f64,
    /// Weight for EdgeCases strategy (0.0-1.0)
    pub weight_edge_cases: f64,
    /// Weight for ErrorInduction strategy (0.0-1.0)
    pub weight_error_induction: f64,
    /// Weight for Composition strategy (0.0-1.0)
    pub weight_composition: f64,
    /// Quality threshold for accepting samples (0.0-1.0)
    pub quality_threshold: f64,
    /// Minimum diversity score to prevent mode collapse (0.0-1.0)
    pub min_diversity: f64,
    /// Augmentation ratio (1.0-10.0)
    pub augmentation_ratio: f64,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            weight_docstring: 0.3,
            weight_type_enum: 0.3,
            weight_edge_cases: 0.15,
            weight_error_induction: 0.15,
            weight_composition: 0.1,
            quality_threshold: 0.7,
            min_diversity: 0.5,
            augmentation_ratio: 2.0,
        }
    }
}

impl GenerationParams {
    /// Number of parameters (dimensions in search space).
    pub const DIM: usize = 8;

    /// Create from a parameter vector (DE solution).
    #[must_use]
    pub fn from_vec(params: &[f64]) -> Self {
        assert!(params.len() >= Self::DIM, "Need {} params, got {}", Self::DIM, params.len());

        // Normalize strategy weights to sum to 1.0
        let weight_sum = params[0] + params[1] + params[2] + params[3] + params[4];
        let norm = if weight_sum > 0.0 { weight_sum } else { 1.0 };

        Self {
            weight_docstring: params[0] / norm,
            weight_type_enum: params[1] / norm,
            weight_edge_cases: params[2] / norm,
            weight_error_induction: params[3] / norm,
            weight_composition: params[4] / norm,
            quality_threshold: params[5].clamp(0.1, 0.99),
            min_diversity: params[6].clamp(0.1, 0.99),
            augmentation_ratio: params[7].clamp(1.0, 10.0),
        }
    }

    /// Convert to a parameter vector for DE.
    #[must_use]
    pub fn to_vec(&self) -> Vec<f64> {
        vec![
            self.weight_docstring,
            self.weight_type_enum,
            self.weight_edge_cases,
            self.weight_error_induction,
            self.weight_composition,
            self.quality_threshold,
            self.min_diversity,
            self.augmentation_ratio,
        ]
    }

    /// Get the search space bounds for DE optimization.
    #[must_use]
    pub fn search_space() -> SearchSpace {
        SearchSpace::Continuous {
            dim: Self::DIM,
            lower: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.1, 1.0],
            upper: vec![1.0, 1.0, 1.0, 1.0, 1.0, 0.99, 0.99, 10.0],
        }
    }

    /// Get strategy weights as a HashMap for easy lookup.
    #[must_use]
    pub fn strategy_weights(&self) -> HashMap<GenerationStrategy, f64> {
        let mut weights = HashMap::new();
        weights.insert(GenerationStrategy::DocstringMining, self.weight_docstring);
        weights.insert(GenerationStrategy::TypeEnumeration, self.weight_type_enum);
        weights.insert(GenerationStrategy::EdgeCases, self.weight_edge_cases);
        weights.insert(GenerationStrategy::ErrorInduction, self.weight_error_induction);
        weights.insert(GenerationStrategy::Composition, self.weight_composition);
        weights
    }
}

/// Configuration for the metaheuristic optimizer.
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Maximum function evaluations for DE.
    pub max_evaluations: usize,
    /// Population size for DE.
    pub population_size: usize,
    /// Mutation factor F (0.0-2.0).
    pub mutation_factor: f64,
    /// Crossover rate CR (0.0-1.0).
    pub crossover_rate: f64,
    /// Random seed for reproducibility.
    pub seed: Option<u64>,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            max_evaluations: 1000,
            population_size: 20,
            mutation_factor: 0.8,
            crossover_rate: 0.9,
            seed: Some(42),
        }
    }
}

/// Result of metaheuristic optimization.
#[derive(Debug, Clone)]
pub struct OptimizedResult {
    /// Best generation parameters found.
    pub params: GenerationParams,
    /// Best fitness value achieved (higher = better).
    pub fitness: f64,
    /// Number of evaluations performed.
    pub evaluations: usize,
    /// Convergence history.
    pub history: Vec<f64>,
    /// Whether optimization converged.
    pub converged: bool,
}

/// Metaheuristic optimizer for corpus generation parameters.
///
/// Uses Differential Evolution to find optimal generation parameters
/// that maximize Oracle classification accuracy.
///
/// # Example
///
/// ```ignore
/// use depyler_oracle::self_supervised::{MetaheuristicOptimizer, OptimizerConfig};
///
/// let config = OptimizerConfig::default();
/// let optimizer = MetaheuristicOptimizer::new(config);
///
/// let result = optimizer.optimize(|params| {
///     // Evaluate fitness: generate corpus, train Oracle, measure accuracy
///     evaluate_accuracy(params)
/// });
///
/// println!("Best params: {:?}", result.params);
/// println!("Best fitness: {:.3}", result.fitness);
/// ```
pub struct MetaheuristicOptimizer {
    config: OptimizerConfig,
    de: DifferentialEvolution,
}

impl MetaheuristicOptimizer {
    /// Create a new optimizer with the given configuration.
    #[must_use]
    pub fn new(config: OptimizerConfig) -> Self {
        let mut de = DifferentialEvolution::default();
        de.population_size = config.population_size;
        de.mutation_factor = config.mutation_factor;
        de.crossover_rate = config.crossover_rate;

        Self { config, de }
    }

    /// Run optimization to find best generation parameters.
    ///
    /// The fitness function should:
    /// 1. Generate corpus using the given parameters
    /// 2. Train an Oracle on the corpus
    /// 3. Evaluate k-fold cross-validation accuracy
    /// 4. Return accuracy (higher = better)
    ///
    /// Note: DE minimizes, so we return negative fitness internally.
    pub fn optimize<F>(&mut self, fitness_fn: F) -> OptimizedResult
    where
        F: Fn(&GenerationParams) -> f64,
    {
        let space = GenerationParams::search_space();
        let budget = Budget::Evaluations(self.config.max_evaluations);

        // Wrap fitness to:
        // 1. Convert raw params to GenerationParams
        // 2. Negate (DE minimizes, we want to maximize accuracy)
        let wrapped_fitness = |raw_params: &[f64]| {
            let params = GenerationParams::from_vec(raw_params);
            let fitness = fitness_fn(&params);
            -fitness // Negate for minimization
        };

        let result: OptimizationResult<Vec<f64>> =
            self.de.optimize(&wrapped_fitness, &space, budget);

        OptimizedResult {
            params: GenerationParams::from_vec(&result.solution),
            fitness: -result.objective_value, // Un-negate
            evaluations: result.evaluations,
            history: result.history.iter().map(|v| -v).collect(),
            converged: result.converged(),
        }
    }

    /// Get the current best solution if optimization was run.
    #[must_use]
    pub fn best(&self) -> Option<GenerationParams> {
        self.de.best().map(|v| GenerationParams::from_vec(v))
    }

    /// Reset the optimizer state for a new run.
    pub fn reset(&mut self) {
        self.de.reset();
    }
}

/// Evaluate fitness for a given set of generation parameters.
///
/// This is the objective function for the metaheuristic optimizer.
/// Higher values indicate better parameter configurations.
///
/// # Arguments
/// * `params` - Generation parameters to evaluate
/// * `stdlib_funcs` - Stdlib functions for corpus generation
/// * `eval_samples` - Number of samples to generate for evaluation
///
/// # Returns
/// Fitness score in [0.0, 1.0], representing Oracle accuracy.
#[allow(dead_code)] // Used in optimization loop
pub fn evaluate_fitness(
    params: &GenerationParams,
    stdlib_funcs: &[StdlibFunction],
    eval_samples: usize,
) -> f64 {
    // Create generator with optimized params
    let config = CorpusConfig {
        target_samples: eval_samples,
        batch_size: 50,
        quality_threshold: params.quality_threshold as f32,
        max_duplicate_rate: 0.05,
    };

    let mut generator = SelfSupervisedCorpusGenerator::new(stdlib_funcs.to_vec(), config);

    // Generate corpus
    let _dataset = match generator.generate() {
        Ok(ds) => ds,
        Err(_) => return 0.0, // Penalize failed generation
    };

    let metrics = generator.metrics();

    // Fitness components:
    // 1. Acceptance rate (want high)
    let acceptance_score = metrics.acceptance_rate() as f64;

    // 2. Category balance (want low imbalance)
    let balance_score = 1.0 / (1.0 + metrics.imbalance_ratio() as f64 / 10.0);

    // 3. Diversity (want high)
    let diversity_score = metrics.diversity_score as f64;

    // 4. Error code coverage (want many unique codes)
    let coverage_score = (metrics.unique_error_codes as f64 / 50.0).min(1.0);

    // Weighted combination
    0.3 * acceptance_score + 0.3 * balance_score + 0.2 * diversity_score + 0.2 * coverage_score
}

// ============================================================================
// Phase 5: Evaluation and Benchmarking
// ============================================================================

/// Metrics for evaluating corpus quality.
#[derive(Debug, Clone, Default)]
pub struct EvaluationMetrics {
    /// Number of samples in corpus
    pub corpus_size: usize,
    /// Percentage of unique samples (non-duplicate)
    pub uniqueness_rate: f64,
    /// Class balance score (1.0 = perfectly balanced)
    pub class_balance: f64,
    /// Coverage of error categories (0.0-1.0)
    pub category_coverage: f64,
    /// Diversity score from DiversityMonitor (0.0-1.0)
    pub diversity_score: f64,
    /// Estimated Oracle accuracy (from k-fold CV)
    pub estimated_accuracy: f64,
    /// F1 score (macro-averaged)
    pub macro_f1: f64,
}

impl EvaluationMetrics {
    /// Create metrics from corpus generation results.
    #[must_use]
    pub fn from_corpus(metrics: &CorpusMetrics, k_fold_accuracy: f64, macro_f1: f64) -> Self {
        let total_categories = 7; // ErrorCategory variants
        let covered_categories = metrics.category_distribution.len();

        Self {
            corpus_size: metrics.accepted,
            uniqueness_rate: 1.0 - metrics.duplicate_rate() as f64,
            class_balance: 1.0 / (1.0 + metrics.imbalance_ratio() as f64 / 10.0),
            category_coverage: covered_categories as f64 / total_categories as f64,
            diversity_score: metrics.diversity_score as f64,
            estimated_accuracy: k_fold_accuracy,
            macro_f1,
        }
    }

    /// Check if metrics meet minimum quality thresholds.
    #[must_use]
    pub fn meets_thresholds(&self, min_accuracy: f64, min_diversity: f64) -> bool {
        self.estimated_accuracy >= min_accuracy && self.diversity_score >= min_diversity
    }

    /// Calculate overall quality score (0.0-1.0).
    #[must_use]
    pub fn overall_score(&self) -> f64 {
        // Weighted combination of all metrics
        let weights = [
            (self.estimated_accuracy, 0.35),
            (self.macro_f1, 0.25),
            (self.diversity_score, 0.15),
            (self.class_balance, 0.15),
            (self.category_coverage, 0.10),
        ];

        weights.iter().map(|(v, w)| v * w).sum()
    }
}

/// Configuration for evaluation runs.
#[derive(Debug, Clone)]
pub struct EvaluationConfig {
    /// Number of folds for k-fold cross-validation.
    pub k_folds: usize,
    /// Minimum accuracy threshold for success.
    pub min_accuracy: f64,
    /// Minimum diversity threshold.
    pub min_diversity: f64,
    /// Random seed for reproducibility.
    pub seed: u64,
}

impl Default for EvaluationConfig {
    fn default() -> Self {
        Self {
            k_folds: 5,
            min_accuracy: 0.85,
            min_diversity: 0.5,
            seed: 42,
        }
    }
}

/// Benchmark results comparing different configurations.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Configuration name/description.
    pub name: String,
    /// Generation parameters used.
    pub params: GenerationParams,
    /// Evaluation metrics achieved.
    pub metrics: EvaluationMetrics,
    /// Time taken for corpus generation (seconds).
    pub generation_time_secs: f64,
    /// Time taken for Oracle training (seconds).
    pub training_time_secs: f64,
}

impl BenchmarkResult {
    /// Create a new benchmark result.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        params: GenerationParams,
        metrics: EvaluationMetrics,
        generation_time_secs: f64,
        training_time_secs: f64,
    ) -> Self {
        Self {
            name: name.into(),
            params,
            metrics,
            generation_time_secs,
            training_time_secs,
        }
    }

    /// Check if this result is better than another.
    #[must_use]
    pub fn is_better_than(&self, other: &Self) -> bool {
        self.metrics.overall_score() > other.metrics.overall_score()
    }
}

/// Evaluator for running benchmarks and comparisons.
pub struct Evaluator {
    config: EvaluationConfig,
    results: Vec<BenchmarkResult>,
}

impl Evaluator {
    /// Create a new evaluator.
    #[must_use]
    pub fn new(config: EvaluationConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// Get the evaluation configuration.
    #[must_use]
    pub fn config(&self) -> &EvaluationConfig {
        &self.config
    }

    /// Get all benchmark results.
    #[must_use]
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Add a benchmark result.
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }

    /// Find the best result by overall score.
    #[must_use]
    pub fn best_result(&self) -> Option<&BenchmarkResult> {
        self.results
            .iter()
            .max_by(|a, b| {
                a.metrics
                    .overall_score()
                    .partial_cmp(&b.metrics.overall_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Calculate baseline metrics for comparison.
    #[must_use]
    pub fn baseline_metrics(&self) -> EvaluationMetrics {
        // Default baseline represents current Oracle without self-supervised corpus
        EvaluationMetrics {
            corpus_size: 99, // Current verificar corpus size
            uniqueness_rate: 0.95,
            class_balance: 0.6,
            category_coverage: 0.71, // 5/7 categories
            diversity_score: 0.7,
            estimated_accuracy: 0.84, // Current Oracle accuracy
            macro_f1: 0.80,
        }
    }

    /// Check if a result improves over baseline.
    #[must_use]
    pub fn improves_over_baseline(&self, metrics: &EvaluationMetrics) -> bool {
        let baseline = self.baseline_metrics();
        metrics.overall_score() > baseline.overall_score()
    }

    /// Generate a summary report of all results.
    #[must_use]
    pub fn summary_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Self-Supervised Corpus Evaluation Report ===\n\n");

        let baseline = self.baseline_metrics();
        report.push_str(&format!(
            "Baseline: accuracy={:.2}%, F1={:.2}, score={:.3}\n\n",
            baseline.estimated_accuracy * 100.0,
            baseline.macro_f1,
            baseline.overall_score()
        ));

        for (i, result) in self.results.iter().enumerate() {
            let improvement = result.metrics.overall_score() - baseline.overall_score();
            let status = if improvement > 0.0 { "✓" } else { "✗" };

            report.push_str(&format!(
                "{}. {} {}\n   Accuracy: {:.2}% | F1: {:.2} | Diversity: {:.2}\n   Score: {:.3} ({:+.3})\n   Time: {:.1}s gen + {:.1}s train\n\n",
                i + 1,
                result.name,
                status,
                result.metrics.estimated_accuracy * 100.0,
                result.metrics.macro_f1,
                result.metrics.diversity_score,
                result.metrics.overall_score(),
                improvement,
                result.generation_time_secs,
                result.training_time_secs,
            ));
        }

        if let Some(best) = self.best_result() {
            report.push_str(&format!("Best configuration: {}\n", best.name));
        }

        report
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
// Phase 2: Curriculum Learning
// ============================================================================

/// Difficulty levels for curriculum learning.
///
/// Examples are generated in progressive difficulty order,
/// starting with simple patterns and advancing to complex ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DifficultyLevel {
    /// Simple single-function calls with default args
    Basic,
    /// Multiple arguments, type variations
    Intermediate,
    /// Error-inducing patterns, edge cases
    Advanced,
    /// Composition, multi-step patterns
    Expert,
}

impl DifficultyLevel {
    /// Get strategies appropriate for this difficulty level.
    #[must_use]
    pub fn strategies(&self) -> Vec<GenerationStrategy> {
        match self {
            DifficultyLevel::Basic => vec![GenerationStrategy::DocstringMining],
            DifficultyLevel::Intermediate => vec![
                GenerationStrategy::DocstringMining,
                GenerationStrategy::TypeEnumeration,
            ],
            DifficultyLevel::Advanced => vec![
                GenerationStrategy::TypeEnumeration,
                GenerationStrategy::EdgeCases,
                GenerationStrategy::ErrorInduction,
            ],
            DifficultyLevel::Expert => vec![
                GenerationStrategy::EdgeCases,
                GenerationStrategy::ErrorInduction,
                GenerationStrategy::Composition,
            ],
        }
    }

    /// Get weight multiplier for this level (higher = more samples).
    #[must_use]
    pub fn weight(&self) -> f64 {
        match self {
            DifficultyLevel::Basic => 0.3,
            DifficultyLevel::Intermediate => 0.3,
            DifficultyLevel::Advanced => 0.25,
            DifficultyLevel::Expert => 0.15,
        }
    }
}

/// Curriculum scheduler for progressive learning.
///
/// Manages the progression through difficulty levels during
/// corpus generation to improve model learning.
#[derive(Debug, Clone)]
pub struct CurriculumScheduler {
    /// Current difficulty level
    current_level: DifficultyLevel,
    /// Samples to generate per level
    samples_per_level: usize,
    /// Samples generated at current level
    samples_generated: usize,
    /// Total samples generated across all levels
    total_generated: usize,
}

impl CurriculumScheduler {
    /// Create a new curriculum scheduler.
    #[must_use]
    pub fn new(samples_per_level: usize) -> Self {
        Self {
            current_level: DifficultyLevel::Basic,
            samples_per_level,
            samples_generated: 0,
            total_generated: 0,
        }
    }

    /// Get current difficulty level.
    #[must_use]
    pub fn current_level(&self) -> DifficultyLevel {
        self.current_level
    }

    /// Get total samples generated.
    #[must_use]
    pub fn total_generated(&self) -> usize {
        self.total_generated
    }

    /// Record that a sample was generated.
    pub fn record_sample(&mut self) {
        self.samples_generated += 1;
        self.total_generated += 1;
    }

    /// Try to advance to the next difficulty level.
    ///
    /// Returns `true` if advanced, `false` if already at Expert.
    pub fn try_advance(&mut self) -> bool {
        if self.samples_generated >= self.samples_per_level {
            match self.current_level {
                DifficultyLevel::Basic => {
                    self.current_level = DifficultyLevel::Intermediate;
                    self.samples_generated = 0;
                    true
                }
                DifficultyLevel::Intermediate => {
                    self.current_level = DifficultyLevel::Advanced;
                    self.samples_generated = 0;
                    true
                }
                DifficultyLevel::Advanced => {
                    self.current_level = DifficultyLevel::Expert;
                    self.samples_generated = 0;
                    true
                }
                DifficultyLevel::Expert => false,
            }
        } else {
            false
        }
    }

    /// Check if curriculum is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current_level == DifficultyLevel::Expert
            && self.samples_generated >= self.samples_per_level
    }

    /// Reset scheduler to beginning.
    pub fn reset(&mut self) {
        self.current_level = DifficultyLevel::Basic;
        self.samples_generated = 0;
        self.total_generated = 0;
    }
}

// ============================================================================
// Phase 2: Optimizer Runner
// ============================================================================

/// Configuration for running the optimizer.
#[derive(Debug, Clone)]
pub struct OptimizationRunConfig {
    /// Number of stdlib functions to use for evaluation
    pub eval_stdlib_count: usize,
    /// Number of samples to generate per evaluation
    pub eval_samples: usize,
    /// Maximum DE evaluations
    pub max_evaluations: usize,
    /// Whether to use curriculum learning
    pub use_curriculum: bool,
}

impl Default for OptimizationRunConfig {
    fn default() -> Self {
        Self {
            eval_stdlib_count: 20,
            eval_samples: 100,
            max_evaluations: 500,
            use_curriculum: true,
        }
    }
}

/// Run optimization to find best generation parameters.
///
/// This is the main entry point for Phase 2 optimizer execution.
pub fn run_optimization(
    stdlib_funcs: &[StdlibFunction],
    config: &OptimizationRunConfig,
) -> OptimizedResult {
    let optimizer_config = OptimizerConfig {
        max_evaluations: config.max_evaluations,
        population_size: 15,
        mutation_factor: 0.7,
        crossover_rate: 0.9,
        seed: Some(42),
    };

    let mut optimizer = MetaheuristicOptimizer::new(optimizer_config);

    // Use subset of stdlib for faster evaluation
    let eval_funcs: Vec<_> = stdlib_funcs
        .iter()
        .take(config.eval_stdlib_count)
        .cloned()
        .collect();

    optimizer.optimize(|params| {
        evaluate_fitness_with_curriculum(params, &eval_funcs, config.eval_samples, config.use_curriculum)
    })
}

/// Evaluate fitness with optional curriculum learning.
fn evaluate_fitness_with_curriculum(
    params: &GenerationParams,
    stdlib_funcs: &[StdlibFunction],
    eval_samples: usize,
    use_curriculum: bool,
) -> f64 {
    if !use_curriculum {
        return evaluate_fitness(params, stdlib_funcs, eval_samples);
    }

    // Use curriculum scheduler for progressive difficulty
    let samples_per_level = eval_samples / 4;
    let mut scheduler = CurriculumScheduler::new(samples_per_level);
    let mut total_fitness = 0.0;
    let mut level_count = 0;

    while !scheduler.is_complete() {
        let level = scheduler.current_level();
        let strategies = level.strategies();

        // Adjust params for current level's strategies
        let level_fitness = evaluate_level_fitness(params, stdlib_funcs, &strategies, samples_per_level);
        total_fitness += level_fitness * level.weight();
        level_count += 1;

        // Simulate generating samples
        for _ in 0..samples_per_level {
            scheduler.record_sample();
        }
        scheduler.try_advance();
    }

    if level_count > 0 {
        total_fitness / level_count as f64 * 4.0 // Normalize
    } else {
        0.0
    }
}

/// Evaluate fitness for a specific difficulty level's strategies.
fn evaluate_level_fitness(
    _params: &GenerationParams,
    stdlib_funcs: &[StdlibFunction],
    strategies: &[GenerationStrategy],
    _samples: usize,
) -> f64 {
    // Simplified fitness based on strategy coverage and stdlib count
    let strategy_diversity = strategies.len() as f64 / 5.0;
    let stdlib_coverage = (stdlib_funcs.len() as f64 / 50.0).min(1.0);

    (strategy_diversity + stdlib_coverage) / 2.0
}

// ============================================================================
// Phase 2: Autofixer Integration
// ============================================================================

/// Fix pattern extracted from training sample.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FixPattern {
    /// Add type conversion (.into(), as, etc.)
    TypeConversion,
    /// Add .clone() to fix borrow issues
    AddClone,
    /// Add missing import/use statement
    AddImport,
    /// Add lifetime annotation
    AddLifetime,
    /// Implement required trait
    ImplementTrait,
    /// Generic fix pattern
    Other(String),
}

impl FixPattern {
    /// Create fix pattern from error category.
    #[must_use]
    pub fn from_category(category: ErrorCategory) -> Self {
        match category {
            ErrorCategory::TypeMismatch => FixPattern::TypeConversion,
            ErrorCategory::BorrowChecker => FixPattern::AddClone,
            ErrorCategory::MissingImport => FixPattern::AddImport,
            ErrorCategory::LifetimeError => FixPattern::AddLifetime,
            ErrorCategory::TraitBound => FixPattern::ImplementTrait,
            ErrorCategory::SyntaxError | ErrorCategory::Other => {
                FixPattern::Other("manual_review".to_string())
            }
        }
    }
}

/// Extracted fix template from corpus sample.
#[derive(Debug, Clone)]
pub struct ExtractedFix {
    /// Error category this fix applies to
    pub category: ErrorCategory,
    /// Fix pattern type
    pub pattern: FixPattern,
    /// Original error message pattern
    pub error_pattern: String,
    /// Suggested fix code template
    pub fix_template: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Extract fix patterns from a training sample.
#[must_use]
pub fn extract_fix_pattern(sample: &crate::TrainingSample) -> Option<ExtractedFix> {
    let category = sample.category;
    let pattern = FixPattern::from_category(category);

    // Generate fix template based on category
    let fix_template = match category {
        ErrorCategory::TypeMismatch => "value.into() or value as Type".to_string(),
        ErrorCategory::BorrowChecker => "value.clone()".to_string(),
        ErrorCategory::MissingImport => "use crate::module::Type;".to_string(),
        ErrorCategory::LifetimeError => "'a annotation".to_string(),
        ErrorCategory::TraitBound => "impl Trait for Type".to_string(),
        _ => return None,
    };

    Some(ExtractedFix {
        category,
        pattern,
        error_pattern: sample.message.clone(),
        fix_template,
        confidence: 0.7, // Default confidence
    })
}

/// Corpus-based fix predictor trained from extracted patterns.
#[derive(Debug, Default)]
pub struct CorpusFixPredictor {
    /// Extracted fix patterns by category
    patterns: HashMap<ErrorCategory, Vec<ExtractedFix>>,
    /// Total patterns extracted
    pattern_count: usize,
}

impl CorpusFixPredictor {
    /// Create a new predictor.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an extracted fix to the predictor.
    pub fn add_fix(&mut self, fix: ExtractedFix) {
        self.patterns
            .entry(fix.category)
            .or_default()
            .push(fix);
        self.pattern_count += 1;
    }

    /// Get pattern count.
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.pattern_count
    }

    /// Get categories with patterns.
    #[must_use]
    pub fn categories(&self) -> Vec<ErrorCategory> {
        self.patterns.keys().copied().collect()
    }

    /// Predict fix for an error, returning highest confidence fix.
    #[must_use]
    pub fn predict(&self, category: ErrorCategory) -> Option<&ExtractedFix> {
        self.patterns.get(&category).and_then(|fixes| {
            fixes
                .iter()
                .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
        })
    }

    /// Train predictor from training dataset.
    pub fn train_from_corpus(&mut self, corpus: &crate::TrainingDataset) {
        for sample in corpus.samples() {
            if let Some(fix) = extract_fix_pattern(sample) {
                self.add_fix(fix);
            }
        }
    }
}

// ============================================================================
// EXTREME TDD: Tests (RED PHASE - These should FAIL initially)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::training::TrainingSample;

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

    // ========================================================================
    // Phase 4: Metaheuristic Optimizer Tests
    // ========================================================================

    #[test]
    fn test_generation_params_default() {
        let params = GenerationParams::default();

        // Weights should sum to ~1.0
        let weight_sum = params.weight_docstring
            + params.weight_type_enum
            + params.weight_edge_cases
            + params.weight_error_induction
            + params.weight_composition;
        assert!((weight_sum - 1.0).abs() < 0.01, "Weights should sum to 1.0");

        // Quality threshold in valid range
        assert!(params.quality_threshold >= 0.0 && params.quality_threshold <= 1.0);
    }

    #[test]
    fn test_generation_params_from_vec() {
        let raw = vec![0.2, 0.3, 0.1, 0.2, 0.2, 0.75, 0.6, 3.0];
        let params = GenerationParams::from_vec(&raw);

        // Weights should be normalized
        let weight_sum = params.weight_docstring
            + params.weight_type_enum
            + params.weight_edge_cases
            + params.weight_error_induction
            + params.weight_composition;
        assert!((weight_sum - 1.0).abs() < 0.001, "Weights should be normalized");

        // Quality threshold preserved
        assert!((params.quality_threshold - 0.75).abs() < 0.001);

        // Augmentation ratio preserved
        assert!((params.augmentation_ratio - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_generation_params_to_vec() {
        let params = GenerationParams::default();
        let vec = params.to_vec();

        assert_eq!(vec.len(), GenerationParams::DIM);
        assert!((vec[5] - 0.7).abs() < 0.001); // quality_threshold
        assert!((vec[7] - 2.0).abs() < 0.001); // augmentation_ratio
    }

    #[test]
    fn test_generation_params_roundtrip() {
        let original = GenerationParams::default();
        let vec = original.to_vec();
        let restored = GenerationParams::from_vec(&vec);

        assert!((original.quality_threshold - restored.quality_threshold).abs() < 0.001);
        assert!((original.min_diversity - restored.min_diversity).abs() < 0.001);
    }

    #[test]
    fn test_generation_params_search_space() {
        let space = GenerationParams::search_space();

        match space {
            SearchSpace::Continuous { dim, lower, upper } => {
                assert_eq!(dim, GenerationParams::DIM);
                assert_eq!(lower.len(), dim);
                assert_eq!(upper.len(), dim);

                // Verify bounds are valid
                for i in 0..dim {
                    assert!(lower[i] <= upper[i], "Invalid bounds at dim {}", i);
                }
            }
            _ => panic!("Expected Continuous search space"),
        }
    }

    #[test]
    fn test_generation_params_strategy_weights() {
        let params = GenerationParams::default();
        let weights = params.strategy_weights();

        assert_eq!(weights.len(), 5);
        assert!(weights.contains_key(&GenerationStrategy::DocstringMining));
        assert!(weights.contains_key(&GenerationStrategy::TypeEnumeration));
        assert!(weights.contains_key(&GenerationStrategy::EdgeCases));
        assert!(weights.contains_key(&GenerationStrategy::ErrorInduction));
        assert!(weights.contains_key(&GenerationStrategy::Composition));
    }

    #[test]
    fn test_generation_params_clamp_bounds() {
        // Test that values outside bounds are clamped
        let raw = vec![0.5, 0.5, 0.0, 0.0, 0.0, -0.5, 2.0, 0.1];
        let params = GenerationParams::from_vec(&raw);

        // quality_threshold should be clamped to [0.1, 0.99]
        assert!(params.quality_threshold >= 0.1 && params.quality_threshold <= 0.99);

        // min_diversity should be clamped to [0.1, 0.99]
        assert!(params.min_diversity >= 0.1 && params.min_diversity <= 0.99);

        // augmentation_ratio should be clamped to [1.0, 10.0]
        assert!(params.augmentation_ratio >= 1.0 && params.augmentation_ratio <= 10.0);
    }

    #[test]
    fn test_optimizer_config_default() {
        let config = OptimizerConfig::default();

        assert!(config.max_evaluations > 0);
        assert!(config.population_size > 0);
        assert!(config.mutation_factor >= 0.0 && config.mutation_factor <= 2.0);
        assert!(config.crossover_rate >= 0.0 && config.crossover_rate <= 1.0);
    }

    #[test]
    fn test_metaheuristic_optimizer_creation() {
        let config = OptimizerConfig {
            max_evaluations: 100,
            population_size: 10,
            mutation_factor: 0.5,
            crossover_rate: 0.7,
            seed: Some(42),
        };

        let optimizer = MetaheuristicOptimizer::new(config);
        assert!(optimizer.best().is_none()); // No solution yet
    }

    #[test]
    fn test_metaheuristic_optimizer_simple_fitness() {
        let config = OptimizerConfig {
            max_evaluations: 50, // Small for test speed
            population_size: 10,
            mutation_factor: 0.8,
            crossover_rate: 0.9,
            seed: Some(42),
        };

        let mut optimizer = MetaheuristicOptimizer::new(config);

        // Simple fitness: prefer high quality_threshold
        let result = optimizer.optimize(|params| params.quality_threshold);

        // Should find params with quality_threshold near upper bound (0.99)
        assert!(result.fitness > 0.5, "Should improve from initial");
        assert!(result.evaluations > 0);
        assert!(!result.history.is_empty());
    }

    #[test]
    fn test_metaheuristic_optimizer_reset() {
        let config = OptimizerConfig {
            max_evaluations: 20,
            population_size: 5,
            ..Default::default()
        };

        let mut optimizer = MetaheuristicOptimizer::new(config.clone());

        // Run once
        let _ = optimizer.optimize(|_| 0.5);
        assert!(optimizer.best().is_some());

        // Reset
        optimizer.reset();
        assert!(optimizer.best().is_none());
    }

    #[test]
    fn test_optimized_result_fields() {
        let config = OptimizerConfig {
            max_evaluations: 30,
            population_size: 5,
            ..Default::default()
        };

        let mut optimizer = MetaheuristicOptimizer::new(config);
        let result = optimizer.optimize(|_| 0.75);

        // Check result structure
        assert!(result.fitness >= 0.0);
        assert!(result.evaluations > 0);
        assert!(!result.history.is_empty());
        // params should be valid
        assert!(result.params.quality_threshold >= 0.1);
    }

    #[test]
    fn test_evaluate_fitness_empty_stdlib() {
        let params = GenerationParams::default();
        let fitness = evaluate_fitness(&params, &[], 10);

        // Empty stdlib should produce some fitness (generator returns empty but valid)
        assert!(fitness >= 0.0 && fitness <= 1.0);
    }

    #[test]
    fn test_evaluate_fitness_with_sample_stdlib() {
        let stdlib_funcs = vec![sample_stdlib_function()];
        let params = GenerationParams::default();
        let fitness = evaluate_fitness(&params, &stdlib_funcs, 10);

        // Should produce valid fitness
        assert!(fitness >= 0.0 && fitness <= 1.0);
    }

    // ========================================================================
    // Phase 5: Evaluation and Benchmarking Tests
    // ========================================================================

    #[test]
    fn test_evaluation_metrics_default() {
        let metrics = EvaluationMetrics::default();

        assert_eq!(metrics.corpus_size, 0);
        assert!((metrics.estimated_accuracy - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_evaluation_metrics_from_corpus() {
        let mut corpus_metrics = CorpusMetrics::default();
        corpus_metrics.accepted = 1000;
        corpus_metrics.total_generated = 1100;
        corpus_metrics.rejected_duplicate = 50;
        corpus_metrics.diversity_score = 0.8;
        corpus_metrics.category_distribution.insert(ErrorCategory::TypeMismatch, 300);
        corpus_metrics.category_distribution.insert(ErrorCategory::BorrowChecker, 200);
        corpus_metrics.category_distribution.insert(ErrorCategory::Other, 500);

        let eval_metrics = EvaluationMetrics::from_corpus(&corpus_metrics, 0.92, 0.88);

        assert_eq!(eval_metrics.corpus_size, 1000);
        assert!(eval_metrics.uniqueness_rate > 0.9);
        assert!((eval_metrics.estimated_accuracy - 0.92).abs() < 0.001);
        assert!((eval_metrics.macro_f1 - 0.88).abs() < 0.001);
    }

    #[test]
    fn test_evaluation_metrics_meets_thresholds() {
        let metrics = EvaluationMetrics {
            estimated_accuracy: 0.90,
            diversity_score: 0.7,
            ..Default::default()
        };

        assert!(metrics.meets_thresholds(0.85, 0.5));
        assert!(!metrics.meets_thresholds(0.95, 0.5));
        assert!(!metrics.meets_thresholds(0.85, 0.8));
    }

    #[test]
    fn test_evaluation_metrics_overall_score() {
        let metrics = EvaluationMetrics {
            estimated_accuracy: 0.95,
            macro_f1: 0.93,
            diversity_score: 0.8,
            class_balance: 0.9,
            category_coverage: 1.0,
            ..Default::default()
        };

        let score = metrics.overall_score();

        // Score should be weighted combination
        let expected = 0.95 * 0.35 + 0.93 * 0.25 + 0.8 * 0.15 + 0.9 * 0.15 + 1.0 * 0.10;
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn test_evaluation_config_default() {
        let config = EvaluationConfig::default();

        assert_eq!(config.k_folds, 5);
        assert!((config.min_accuracy - 0.85).abs() < 0.001);
        assert!((config.min_diversity - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_benchmark_result_creation() {
        let params = GenerationParams::default();
        let metrics = EvaluationMetrics {
            estimated_accuracy: 0.92,
            ..Default::default()
        };

        let result = BenchmarkResult::new("Test Config", params, metrics, 10.5, 5.2);

        assert_eq!(result.name, "Test Config");
        assert!((result.generation_time_secs - 10.5).abs() < 0.001);
        assert!((result.training_time_secs - 5.2).abs() < 0.001);
    }

    #[test]
    fn test_benchmark_result_comparison() {
        let params = GenerationParams::default();

        let better = BenchmarkResult::new(
            "Better",
            params.clone(),
            EvaluationMetrics {
                estimated_accuracy: 0.95,
                macro_f1: 0.93,
                ..Default::default()
            },
            1.0,
            1.0,
        );

        let worse = BenchmarkResult::new(
            "Worse",
            params,
            EvaluationMetrics {
                estimated_accuracy: 0.80,
                macro_f1: 0.75,
                ..Default::default()
            },
            1.0,
            1.0,
        );

        assert!(better.is_better_than(&worse));
        assert!(!worse.is_better_than(&better));
    }

    #[test]
    fn test_evaluator_creation() {
        let config = EvaluationConfig::default();
        let evaluator = Evaluator::new(config);

        assert!(evaluator.results().is_empty());
        assert_eq!(evaluator.config().k_folds, 5);
    }

    #[test]
    fn test_evaluator_add_results() {
        let config = EvaluationConfig::default();
        let mut evaluator = Evaluator::new(config);

        let result1 = BenchmarkResult::new(
            "Config A",
            GenerationParams::default(),
            EvaluationMetrics {
                estimated_accuracy: 0.90,
                ..Default::default()
            },
            1.0,
            1.0,
        );

        let result2 = BenchmarkResult::new(
            "Config B",
            GenerationParams::default(),
            EvaluationMetrics {
                estimated_accuracy: 0.95,
                ..Default::default()
            },
            1.0,
            1.0,
        );

        evaluator.add_result(result1);
        evaluator.add_result(result2);

        assert_eq!(evaluator.results().len(), 2);
    }

    #[test]
    fn test_evaluator_best_result() {
        let config = EvaluationConfig::default();
        let mut evaluator = Evaluator::new(config);

        evaluator.add_result(BenchmarkResult::new(
            "Low",
            GenerationParams::default(),
            EvaluationMetrics {
                estimated_accuracy: 0.80,
                macro_f1: 0.75,
                ..Default::default()
            },
            1.0,
            1.0,
        ));

        evaluator.add_result(BenchmarkResult::new(
            "High",
            GenerationParams::default(),
            EvaluationMetrics {
                estimated_accuracy: 0.95,
                macro_f1: 0.93,
                ..Default::default()
            },
            1.0,
            1.0,
        ));

        evaluator.add_result(BenchmarkResult::new(
            "Medium",
            GenerationParams::default(),
            EvaluationMetrics {
                estimated_accuracy: 0.88,
                macro_f1: 0.85,
                ..Default::default()
            },
            1.0,
            1.0,
        ));

        let best = evaluator.best_result().expect("Should have best result");
        assert_eq!(best.name, "High");
    }

    #[test]
    fn test_evaluator_baseline_metrics() {
        let evaluator = Evaluator::new(EvaluationConfig::default());
        let baseline = evaluator.baseline_metrics();

        // Should match current Oracle metrics
        assert_eq!(baseline.corpus_size, 99);
        assert!((baseline.estimated_accuracy - 0.84).abs() < 0.01);
    }

    #[test]
    fn test_evaluator_improves_over_baseline() {
        let evaluator = Evaluator::new(EvaluationConfig::default());

        let improved = EvaluationMetrics {
            corpus_size: 5000,
            uniqueness_rate: 0.98,
            class_balance: 0.9,
            category_coverage: 1.0,
            diversity_score: 0.85,
            estimated_accuracy: 0.95,
            macro_f1: 0.93,
        };

        let worse = EvaluationMetrics {
            estimated_accuracy: 0.70,
            macro_f1: 0.65,
            ..Default::default()
        };

        assert!(evaluator.improves_over_baseline(&improved));
        assert!(!evaluator.improves_over_baseline(&worse));
    }

    #[test]
    fn test_evaluator_summary_report() {
        let config = EvaluationConfig::default();
        let mut evaluator = Evaluator::new(config);

        evaluator.add_result(BenchmarkResult::new(
            "Test Config",
            GenerationParams::default(),
            EvaluationMetrics {
                estimated_accuracy: 0.92,
                macro_f1: 0.90,
                diversity_score: 0.8,
                ..Default::default()
            },
            15.5,
            3.2,
        ));

        let report = evaluator.summary_report();

        assert!(report.contains("Self-Supervised Corpus Evaluation Report"));
        assert!(report.contains("Baseline"));
        assert!(report.contains("Test Config"));
        assert!(report.contains("92.00%")); // Accuracy
    }

    #[test]
    fn test_evaluator_empty_best_result() {
        let evaluator = Evaluator::new(EvaluationConfig::default());
        assert!(evaluator.best_result().is_none());
    }

    // ========================================================================
    // Phase 6: Curriculum Learning Tests
    // ========================================================================

    #[test]
    fn test_difficulty_level_ordering() {
        assert!(DifficultyLevel::Basic < DifficultyLevel::Intermediate);
        assert!(DifficultyLevel::Intermediate < DifficultyLevel::Advanced);
        assert!(DifficultyLevel::Advanced < DifficultyLevel::Expert);
    }

    #[test]
    fn test_difficulty_level_strategies() {
        // Basic should include DocstringMining
        let basic_strategies = DifficultyLevel::Basic.strategies();
        assert!(basic_strategies.contains(&GenerationStrategy::DocstringMining));

        // Advanced should include ErrorInduction
        let advanced_strategies = DifficultyLevel::Advanced.strategies();
        assert!(advanced_strategies.contains(&GenerationStrategy::ErrorInduction));

        // Expert should include Composition
        let expert_strategies = DifficultyLevel::Expert.strategies();
        assert!(expert_strategies.contains(&GenerationStrategy::Composition));
    }

    #[test]
    fn test_difficulty_level_weight() {
        let total_weight: f64 = DifficultyLevel::Basic.weight()
            + DifficultyLevel::Intermediate.weight()
            + DifficultyLevel::Advanced.weight()
            + DifficultyLevel::Expert.weight();

        assert!((total_weight - 1.0).abs() < 0.001, "Weights should sum to 1.0");
    }

    #[test]
    fn test_curriculum_scheduler_creation() {
        let scheduler = CurriculumScheduler::new(100);

        assert_eq!(scheduler.current_level(), DifficultyLevel::Basic);
        assert_eq!(scheduler.total_generated(), 0);
        assert!(!scheduler.is_complete());
    }

    #[test]
    fn test_curriculum_scheduler_record_sample() {
        let mut scheduler = CurriculumScheduler::new(10);

        for _ in 0..5 {
            scheduler.record_sample();
        }

        assert_eq!(scheduler.total_generated(), 5);
        assert_eq!(scheduler.current_level(), DifficultyLevel::Basic);
    }

    #[test]
    fn test_curriculum_scheduler_advance() {
        let mut scheduler = CurriculumScheduler::new(2);

        // Record samples to trigger advancement
        scheduler.record_sample();
        scheduler.record_sample();
        let advanced = scheduler.try_advance();

        assert!(advanced);
        assert_eq!(scheduler.current_level(), DifficultyLevel::Intermediate);
    }

    #[test]
    fn test_curriculum_scheduler_full_progression() {
        let mut scheduler = CurriculumScheduler::new(1);

        // Progress through all levels
        scheduler.record_sample();
        scheduler.try_advance(); // -> Intermediate

        scheduler.record_sample();
        scheduler.try_advance(); // -> Advanced

        scheduler.record_sample();
        scheduler.try_advance(); // -> Expert

        scheduler.record_sample();
        let at_end = !scheduler.try_advance(); // No more levels

        assert!(at_end);
        assert!(scheduler.is_complete());
        assert_eq!(scheduler.current_level(), DifficultyLevel::Expert);
    }

    #[test]
    fn test_curriculum_scheduler_reset() {
        let mut scheduler = CurriculumScheduler::new(1);

        scheduler.record_sample();
        scheduler.try_advance();
        scheduler.reset();

        assert_eq!(scheduler.current_level(), DifficultyLevel::Basic);
        assert_eq!(scheduler.total_generated(), 0);
    }

    // ========================================================================
    // Phase 7: Optimization Runner Tests
    // ========================================================================

    #[test]
    fn test_optimization_run_config_default() {
        let config = OptimizationRunConfig::default();

        assert!(config.eval_stdlib_count > 0);
        assert!(config.eval_samples > 0);
        assert!(config.max_evaluations > 0);
        assert!(config.use_curriculum); // Default is true (curriculum learning enabled)
    }

    #[test]
    fn test_run_optimization_basic() {
        let stdlib_funcs = vec![sample_stdlib_function()];
        let config = OptimizationRunConfig {
            eval_stdlib_count: 1,
            eval_samples: 5,
            max_evaluations: 10,
            use_curriculum: false,
        };

        let result = run_optimization(&stdlib_funcs, &config);

        assert!(result.fitness >= 0.0);
        assert!(result.evaluations > 0);
    }

    #[test]
    fn test_run_optimization_with_curriculum() {
        let stdlib_funcs = vec![sample_stdlib_function()];
        let config = OptimizationRunConfig {
            eval_stdlib_count: 1,
            eval_samples: 5,
            max_evaluations: 10,
            use_curriculum: true,
        };

        let result = run_optimization(&stdlib_funcs, &config);

        // Should still produce valid result with curriculum
        assert!(result.fitness >= 0.0);
        assert!(result.evaluations > 0);
    }

    // ========================================================================
    // Phase 8: Autofixer Integration Tests
    // ========================================================================

    #[test]
    fn test_fix_pattern_from_category() {
        assert!(matches!(
            FixPattern::from_category(ErrorCategory::TypeMismatch),
            FixPattern::TypeConversion
        ));
        assert!(matches!(
            FixPattern::from_category(ErrorCategory::BorrowChecker),
            FixPattern::AddClone
        ));
        assert!(matches!(
            FixPattern::from_category(ErrorCategory::MissingImport),
            FixPattern::AddImport
        ));
        assert!(matches!(
            FixPattern::from_category(ErrorCategory::LifetimeError),
            FixPattern::AddLifetime
        ));
        assert!(matches!(
            FixPattern::from_category(ErrorCategory::TraitBound),
            FixPattern::ImplementTrait
        ));
    }

    #[test]
    fn test_fix_pattern_other() {
        let pattern = FixPattern::from_category(ErrorCategory::Other);
        assert!(matches!(pattern, FixPattern::Other(_)));
    }

    #[test]
    fn test_corpus_fix_predictor_creation() {
        let predictor = CorpusFixPredictor::new();

        assert_eq!(predictor.pattern_count(), 0);
        assert!(predictor.predict(ErrorCategory::TypeMismatch).is_none());
    }

    #[test]
    fn test_corpus_fix_predictor_add_fix() {
        let mut predictor = CorpusFixPredictor::new();

        let fix = ExtractedFix {
            category: ErrorCategory::TypeMismatch,
            pattern: FixPattern::TypeConversion,
            error_pattern: "expected i32, found String".to_string(),
            fix_template: "use .parse::<i32>()".to_string(),
            confidence: 0.9,
        };

        predictor.add_fix(fix);

        assert_eq!(predictor.pattern_count(), 1);
    }

    #[test]
    fn test_corpus_fix_predictor_predict() {
        let mut predictor = CorpusFixPredictor::new();

        predictor.add_fix(ExtractedFix {
            category: ErrorCategory::TypeMismatch,
            pattern: FixPattern::TypeConversion,
            error_pattern: "type mismatch".to_string(),
            fix_template: "use .into()".to_string(),
            confidence: 0.8,
        });

        predictor.add_fix(ExtractedFix {
            category: ErrorCategory::TypeMismatch,
            pattern: FixPattern::TypeConversion,
            error_pattern: "expected struct".to_string(),
            fix_template: "use From::from()".to_string(),
            confidence: 0.9,
        });

        let prediction = predictor.predict(ErrorCategory::TypeMismatch);

        // Should return the highest confidence fix
        assert!(prediction.is_some());
        let fix = prediction.unwrap();
        assert!((fix.confidence - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_corpus_fix_predictor_train_from_corpus() {
        use crate::TrainingDataset;

        let mut predictor = CorpusFixPredictor::new();
        let mut dataset = TrainingDataset::new();

        dataset.add(TrainingSample {
            message: "mismatched types expected i32 found String".to_string(),
            category: ErrorCategory::TypeMismatch,
            fix: Some("use .parse::<i32>()".to_string()),
        });

        dataset.add(TrainingSample {
            message: "cannot borrow as mutable".to_string(),
            category: ErrorCategory::BorrowChecker,
            fix: Some("use .clone()".to_string()),
        });

        predictor.train_from_corpus(&dataset);

        // Should extract patterns from both samples
        assert!(predictor.pattern_count() >= 2);
    }

    #[test]
    fn test_extract_fix_pattern_type_mismatch() {
        let sample = TrainingSample {
            message: "type mismatch error".to_string(),
            category: ErrorCategory::TypeMismatch,
            fix: Some("convert type".to_string()),
        };

        let extracted = extract_fix_pattern(&sample);

        assert!(extracted.is_some());
        let fix = extracted.unwrap();
        assert_eq!(fix.category, ErrorCategory::TypeMismatch);
        assert!(matches!(fix.pattern, FixPattern::TypeConversion));
    }

    #[test]
    fn test_extract_fix_pattern_syntax_error() {
        let sample = TrainingSample {
            message: "syntax error".to_string(),
            category: ErrorCategory::SyntaxError,
            fix: None,
        };

        let extracted = extract_fix_pattern(&sample);

        // SyntaxError should not produce a fix pattern
        assert!(extracted.is_none());
    }
}

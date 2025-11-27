# Metaheuristic Oracle: Self-Supervised Corpus Generation

**Version**: 1.0.0
**Status**: Draft
**Authors**: Depyler Team
**Date**: 2025-11-27

## Abstract

This specification defines a metaheuristic-optimized Oracle system for compile error classification. Unlike template-based synthetic data generation, this approach generates Python examples programmatically from stdlib documentation, transpiles them through Depyler, and captures real rustc errors to build a self-supervised training corpus.

## 1. Motivation

### 1.1 Current Limitations

The existing Oracle achieves 91% k-fold CV accuracy using:
- **Template-based synthetics** (verificar): ~30k combinations of error templates × types × contexts
- **Manual samples**: ~200 hand-labeled error-fix pairs from bug fixes

**Problem**: Template saturation. After exhausting combinatorial space (~50 error codes × 30 types × 20 contexts = 30,000 patterns), additional synthetic samples provide diminishing returns because:

1. TF-IDF features saturate on repeated keywords
2. Synthetic errors lack multi-line context and real variable names
3. Template distribution doesn't match actual transpilation error distribution

### 1.2 Proposed Solution

Generate **real** compile errors by:
1. Parsing Python stdlib documentation and source code
2. Programmatically generating 100k+ Python examples that exercise stdlib patterns
3. Transpiling each through Depyler
4. Compiling outputs with rustc
5. Auto-labeling errors by error code
6. Training Oracle on this self-supervised corpus

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Self-Supervised Corpus Pipeline                       │
└─────────────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Python     │    │   Example    │    │   Depyler    │    │    rustc     │
│   Stdlib     │ →  │  Generator   │ →  │  Transpile   │ →  │   Compile    │
│   Parser     │    │  (Aprender)  │    │              │    │              │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
  Signatures         Python Code          Rust Code           Errors
  + Docstrings       Examples             (may fail)          (labeled)
                          │                                        │
                          │     ┌──────────────────────────┐       │
                          └────→│  Metaheuristic Optimizer │←──────┘
                                │  (Differential Evolution) │
                                └──────────────────────────┘
                                           │
                                           ▼
                                ┌──────────────────────────┐
                                │   Oracle Classifier      │
                                │   (Random Forest + TF-IDF)│
                                └──────────────────────────┘
```

## 3. Components

### 3.1 Python Stdlib Parser

Extract function signatures, type hints, and docstring examples from Python stdlib.

```rust
use rustpython_parser::ast;

pub struct StdlibFunction {
    pub module: String,           // "os.path"
    pub name: String,             // "join"
    pub signature: String,        // "(path, *paths) -> str"
    pub arg_types: Vec<PyType>,   // [Path, Path...]
    pub return_type: Option<PyType>,
    pub docstring_examples: Vec<String>,
}

pub fn parse_stdlib() -> Vec<StdlibFunction> {
    // Parse typeshed stubs + CPython docstrings
    // Returns ~2000 function signatures
}
```

### 3.2 Example Generator (aprender::synthetic)

Leverage aprender's `SyntheticGenerator` trait for combinatorial example generation:

```rust
use aprender::synthetic::{SyntheticGenerator, SyntheticConfig, DiversityMonitor};

pub struct PythonExampleGenerator {
    stdlib_funcs: Vec<StdlibFunction>,
    diversity_monitor: DiversityMonitor,
}

impl SyntheticGenerator for PythonExampleGenerator {
    type Input = StdlibFunction;
    type Output = PythonExample;

    fn generate(&self, seed: &Self::Input) -> Result<Vec<Self::Output>> {
        // Generate examples exercising this function
        // with various argument combinations
    }

    fn quality_score(&self, sample: &Self::Output) -> f64 {
        // Score based on syntax validity, type coverage
    }

    fn diversity_score(&self, sample: &Self::Output) -> f64 {
        // Score based on distinctness from existing corpus
    }
}
```

**Generation Strategies**:

| Strategy | Description | Examples/Function |
|----------|-------------|-------------------|
| Docstring Mining | Extract examples from docstrings | 1-3 |
| Type Enumeration | Enumerate valid type combinations | 10-50 |
| Edge Cases | Boundary values, None, empty | 5-10 |
| Error Induction | Invalid types, missing args | 10-20 |
| Composition | Chain multiple stdlib calls | 20-100 |

### 3.3 Transpile/Compile Pipeline

Batch processing with error capture:

```rust
pub struct TranspileResult {
    pub python_source: String,
    pub rust_output: Option<String>,
    pub transpile_error: Option<String>,
    pub compile_errors: Vec<RustcError>,
}

pub struct RustcError {
    pub code: String,        // "E0308"
    pub message: String,     // Full error message with context
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

pub async fn batch_transpile_compile(
    examples: Vec<PythonExample>,
    parallelism: usize,
) -> Vec<TranspileResult> {
    // Parallel transpilation and compilation
    // Captures all errors for corpus
}
```

### 3.4 Auto-Labeler

Map rustc error codes to Oracle categories:

```rust
pub fn auto_label(error: &RustcError) -> ErrorCategory {
    match error.code.as_str() {
        // Type mismatches
        "E0308" | "E0277" | "E0282" | "E0283" => ErrorCategory::TypeMismatch,

        // Borrow checker
        "E0382" | "E0499" | "E0502" | "E0503" | "E0505" |
        "E0507" | "E0596" | "E0597" => ErrorCategory::BorrowChecker,

        // Missing imports
        "E0432" | "E0433" | "E0412" => ErrorCategory::MissingImport,

        // Syntax
        "E0425" | "E0423" | "E0424" | "E0609" => ErrorCategory::SyntaxError,

        // Lifetime
        "E0106" | "E0495" | "E0621" => ErrorCategory::LifetimeError,

        // Trait bounds
        "E0599" | "E0600" | "E0369" | "E0631" => ErrorCategory::TraitBound,

        _ => ErrorCategory::Other,
    }
}
```

### 3.5 Metaheuristic Optimizer (aprender::metaheuristics)

Use Differential Evolution to optimize generation parameters:

```rust
use aprender::metaheuristics::{DifferentialEvolution, SearchSpace, Budget};

pub struct GenerationParams {
    pub augmentation_ratio: f64,      // Synthetic vs real examples
    pub type_variation_depth: usize,  // How many type combinations
    pub composition_depth: usize,     // How many chained calls
    pub error_induction_rate: f64,    // Rate of intentionally invalid examples
    pub quality_threshold: f64,       // Minimum quality score
}

impl GenerationParams {
    pub fn to_vector(&self) -> Vec<f64> {
        vec![
            self.augmentation_ratio,
            self.type_variation_depth as f64,
            self.composition_depth as f64,
            self.error_induction_rate,
            self.quality_threshold,
        ]
    }

    pub fn from_vector(v: &[f64]) -> Self {
        Self {
            augmentation_ratio: v[0],
            type_variation_depth: v[1] as usize,
            composition_depth: v[2] as usize,
            error_induction_rate: v[3],
            quality_threshold: v[4],
        }
    }
}

pub fn optimize_generation_params() -> GenerationParams {
    let space = SearchSpace::Continuous {
        dim: 5,
        lower: vec![0.1, 1.0, 1.0, 0.0, 0.5],
        upper: vec![2.0, 10.0, 5.0, 0.3, 0.95],
    };

    let objective = |params: &[f64]| {
        let gen_params = GenerationParams::from_vector(params);
        let corpus = generate_corpus(&gen_params);
        let accuracy = cross_validate_oracle(&corpus);
        1.0 - accuracy  // Minimize (maximize accuracy)
    };

    let mut de = DifferentialEvolution::default();
    let result = de.optimize(&objective, &space, Budget::Evaluations(1000));

    GenerationParams::from_vector(&result.best_solution)
}
```

### 3.6 Oracle Classifier Enhancement

Integrate with aprender's model selection:

```rust
use aprender::tree::RandomForest;
use aprender::model_selection::{KFold, cross_val_score};
use aprender::text::TfIdfVectorizer;

pub struct EnhancedOracle {
    vectorizer: TfIdfVectorizer,
    classifier: RandomForest,
    diversity_monitor: DiversityMonitor,
}

impl EnhancedOracle {
    pub fn train_with_monitoring(&mut self, corpus: &TrainingDataset) {
        // Monitor for distribution drift during training
        self.diversity_monitor.observe(&corpus.samples());

        if self.diversity_monitor.detect_drift() {
            warn!("Corpus drift detected - consider rebalancing");
        }

        let (features, labels) = self.extract_features(corpus);
        self.classifier.fit(&features, &labels);
    }
}
```

## 4. aprender Integration Points

### 4.1 AutoML (TPE)

Tree-structured Parzen Estimator for hyperparameter optimization:

```rust
use aprender::automl::{TPE, HyperparameterSpace};

let space = HyperparameterSpace::new()
    .add_int("n_estimators", 50, 500)
    .add_int("max_depth", 3, 20)
    .add_float("min_samples_split", 0.01, 0.3)
    .add_categorical("criterion", &["gini", "entropy"]);

let tpe = TPE::new(space);
let best = tpe.optimize(|params| train_and_evaluate(&params), 100);
```

### 4.2 Synthetic Data Quality

Quality-aware generation with automatic filtering:

```rust
use aprender::synthetic::{QualityDegradationDetector, ValidationResult};

let detector = QualityDegradationDetector::new(0.7);
for sample in generated_samples {
    match detector.validate(&sample) {
        ValidationResult::Accept => corpus.add(sample),
        ValidationResult::Reject(reason) => log::debug!("Rejected: {}", reason),
        ValidationResult::NeedsReview => review_queue.push(sample),
    }
}
```

### 4.3 Diversity Monitoring

Prevent mode collapse in synthetic generation:

```rust
use aprender::synthetic::{DiversityMonitor, DiversityScore};

let mut monitor = DiversityMonitor::new();
monitor.configure()
    .with_min_entropy(0.7)
    .with_max_duplicates(0.05)
    .with_coverage_threshold(0.8);

for batch in generation_batches {
    let score: DiversityScore = monitor.score(&batch);
    if score.is_degraded() {
        // Adjust generation parameters
        generator.increase_exploration();
    }
}
```

### 4.4 Weak Supervision

Label noisy samples using labeling functions:

```rust
use aprender::synthetic::weak_supervision::{LabelModel, LabelingFunction};

// Define labeling functions
let lf_error_code = LabelingFunction::new(|error: &str| {
    if error.contains("E0308") { Some(Category::TypeMismatch) }
    else { None }
});

let lf_keyword = LabelingFunction::new(|error: &str| {
    if error.contains("borrow") || error.contains("move") {
        Some(Category::BorrowChecker)
    } else { None }
});

let label_model = LabelModel::new(vec![lf_error_code, lf_keyword]);
let probabilistic_labels = label_model.fit_predict(&unlabeled_errors);
```

## 5. Training Pipeline

### 5.1 Corpus Generation

```rust
pub async fn generate_self_supervised_corpus() -> TrainingDataset {
    // 1. Parse stdlib
    let stdlib = parse_stdlib();
    info!("Parsed {} stdlib functions", stdlib.len());

    // 2. Optimize generation params
    let params = optimize_generation_params();

    // 3. Generate examples
    let generator = PythonExampleGenerator::new(stdlib, params);
    let examples = generator.generate_batch(100_000);

    // 4. Transpile and compile
    let results = batch_transpile_compile(examples, num_cpus::get()).await;

    // 5. Extract errors and auto-label
    let mut dataset = TrainingDataset::new();
    for result in results {
        for error in result.compile_errors {
            let category = auto_label(&error);
            dataset.add(TrainingSample::new(&error.message, category));
        }
    }

    // 6. Quality filter
    let quality_detector = QualityDegradationDetector::new(0.7);
    dataset.filter(|s| quality_detector.validate(s).is_accept());

    info!("Generated {} training samples", dataset.len());
    dataset
}
```

### 5.2 Incremental Training

Support continuous learning as Depyler evolves:

```rust
pub fn incremental_train(
    existing_corpus: &TrainingDataset,
    new_errors: Vec<RustcError>,
) -> TrainingDataset {
    let mut corpus = existing_corpus.clone();

    // Add new errors with auto-labeling
    for error in new_errors {
        let category = auto_label(&error);
        corpus.add(TrainingSample::new(&error.message, category));
    }

    // Rebalance if needed
    let imbalance = corpus.class_imbalance_ratio();
    if imbalance > 3.0 {
        corpus.apply_smote(target_ratio: 1.5);
    }

    corpus
}
```

## 6. Evaluation Metrics

### 6.1 Classification Metrics

| Metric | Target | Current |
|--------|--------|---------|
| K-fold CV Accuracy | ≥95% | 91.4% |
| Macro F1 | ≥0.93 | 0.91 |
| Weighted F1 | ≥0.95 | 0.98 |
| Leave-one-out Accuracy | ≥85% | 79% |

### 6.2 Corpus Quality Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| Diversity Score | Entropy of category distribution | ≥0.8 |
| Duplicate Rate | Exact/near duplicates | ≤5% |
| Coverage | Stdlib functions exercised | ≥70% |
| Error Induction Rate | Samples that produce errors | ≥80% |

### 6.3 Transpiler Progress Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| Compile Success Rate | % of transpiled code that compiles | ≥90% |
| Error Category Distribution | Match expected distribution | KL < 0.1 |
| Novel Error Discovery | New error patterns found | Track |

## 7. Scientific References

### 7.1 Metaheuristic Optimization

1. **Storn, R., & Price, K. (1997).** Differential Evolution - A Simple and Efficient Heuristic for Global Optimization over Continuous Spaces. *Journal of Global Optimization*, 11(4), 341-359.
   - Foundation for DE algorithm used in hyperparameter optimization

2. **Hansen, N. (2016).** The CMA Evolution Strategy: A Tutorial. *arXiv:1604.00772*.
   - Advanced evolution strategy for continuous optimization

3. **Kennedy, J., & Eberhart, R. (1995).** Particle Swarm Optimization. *Proceedings of ICNN'95*, 4, 1942-1948.
   - Alternative swarm-based metaheuristic

### 7.2 AutoML and Hyperparameter Optimization

4. **Bergstra, J., Bardenet, R., Bengio, Y., & Kégl, B. (2011).** Algorithms for Hyper-Parameter Optimization. *Advances in Neural Information Processing Systems*, 24.
   - Tree-structured Parzen Estimator (TPE) algorithm

5. **Feurer, M., Klein, A., Eggensperger, K., Springenberg, J., Blum, M., & Hutter, F. (2015).** Efficient and Robust Automated Machine Learning. *Advances in Neural Information Processing Systems*, 28.
   - Auto-sklearn: combined model/hyperparameter selection

### 7.3 Synthetic Data and Data Augmentation

6. **Cubuk, E. D., Zoph, B., Mane, D., Vasudevan, V., & Le, Q. V. (2019).** AutoAugment: Learning Augmentation Strategies from Data. *CVPR*, 113-123.
   - Automated augmentation policy learning

7. **Wei, J., & Zou, K. (2019).** EDA: Easy Data Augmentation Techniques for Boosting Performance on Text Classification Tasks. *EMNLP-IJCNLP*, 6382-6388.
   - Text-specific augmentation (synonym replacement, random insertion/swap/deletion)

8. **Ratner, A., Bach, S. H., Ehrenberg, H., Fries, J., Wu, S., & Ré, C. (2017).** Snorkel: Rapid Training Data Creation with Weak Supervision. *VLDB*, 11(3), 269-282.
   - Programmatic labeling and weak supervision

### 7.4 Compiler Error Classification

9. **Bhatia, S., Kohli, P., & Singh, R. (2018).** Neuro-Symbolic Program Corrector for Introductory Programming Assignments. *ICSE*, 60-70.
   - Neural network approach to program repair

10. **Ahmed, U. Z., Kumar, P., Karkare, A., Kar, P., & Gulwani, S. (2018).** Compilation Error Repair: For the Student Programs, from the Student Programs. *ICSE-SEET*, 78-87.
    - Error message clustering and repair suggestion

### 7.5 Random Forest and Ensemble Methods

11. **Breiman, L. (2001).** Random Forests. *Machine Learning*, 45(1), 5-32.
    - Foundational random forest paper

12. **Chen, T., & Guestrin, C. (2016).** XGBoost: A Scalable Tree Boosting System. *KDD*, 785-794.
    - Gradient boosting alternative for classification

## 8. Implementation Roadmap

### Phase 1: Stdlib Parser (Week 1)
- [ ] Parse typeshed stubs for type annotations
- [ ] Extract docstring examples from CPython
- [ ] Build function signature database

### Phase 2: Example Generator (Week 2)
- [ ] Implement `SyntheticGenerator` for Python
- [ ] Add type enumeration strategy
- [ ] Add composition strategy
- [ ] Integrate diversity monitoring

### Phase 3: Pipeline Integration (Week 3)
- [ ] Batch transpile/compile harness
- [ ] Auto-labeler with error code mapping
- [ ] Quality filtering

### Phase 4: Metaheuristic Optimization (Week 4)
- [ ] DE-based parameter optimization
- [ ] TPE hyperparameter tuning
- [ ] Incremental training support

### Phase 5: Evaluation (Week 5)
- [ ] Benchmark against current Oracle
- [ ] A/B test on new errors
- [ ] Document accuracy improvements

## 9. Success Criteria

1. **Corpus Size**: Generate ≥50,000 unique error samples
2. **Classification Accuracy**: Achieve ≥95% k-fold CV accuracy
3. **Coverage**: Exercise ≥70% of Python stdlib functions
4. **Diversity**: Maintain diversity score ≥0.8
5. **Transpiler Progress**: Enable fixing 5+ more examples to compile

## 10. Risks and Mitigations

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Most examples produce same errors | High | High | Error deduplication, diversity monitoring |
| Stdlib parsing incomplete | Medium | Medium | Fallback to manual signatures |
| Transpile/compile too slow | Medium | Medium | Parallelization, caching |
| Oracle overfits to synthetic | Medium | High | Hold-out real errors for validation |
| aprender API changes | Low | Medium | Pin version, abstract interfaces |

## Appendix A: Error Code Reference

| Code | Category | Description |
|------|----------|-------------|
| E0308 | TypeMismatch | Mismatched types |
| E0277 | TypeMismatch/TraitBound | Trait not satisfied |
| E0282 | TypeMismatch | Type annotations needed |
| E0382 | BorrowChecker | Use of moved value |
| E0499 | BorrowChecker | Cannot borrow mutably twice |
| E0502 | BorrowChecker | Cannot borrow as mutable and immutable |
| E0432 | MissingImport | Unresolved import |
| E0433 | MissingImport | Failed to resolve |
| E0425 | SyntaxError | Cannot find value |
| E0599 | TraitBound | No method found |
| E0106 | LifetimeError | Missing lifetime specifier |

## Appendix B: aprender Feature Matrix

| Feature | Module | Use Case |
|---------|--------|----------|
| Differential Evolution | `metaheuristics::de` | Generation param optimization |
| TPE | `automl::tpe` | Hyperparameter tuning |
| Random Forest | `tree::RandomForest` | Classification |
| TF-IDF | `text::TfIdfVectorizer` | Feature extraction |
| Diversity Monitor | `synthetic::diversity` | Mode collapse detection |
| Quality Validation | `synthetic::validator` | Sample filtering |
| Weak Supervision | `synthetic::weak_supervision` | Noisy labeling |
| Cross-validation | `model_selection::KFold` | Evaluation |

# Metaheuristic Oracle Phase 2: Optimizer Execution and Autofixer Integration

**Version**: 1.0.0
**Status**: DRAFT
**Author**: Claude Code
**Date**: 2025-11-27

---

## 1. Executive Summary

Phase 2 extends the self-supervised corpus generation pipeline with:
- **Optimizer Execution**: Run DE to find optimal generation parameters
- **Integration Tests**: End-to-end corpus generation validation
- **Curriculum Learning**: Progressive difficulty in example generation
- **Autofixer Integration**: Wire corpus to autofixer training

## 2. Phase 2 Components

### 2.1 Optimizer Execution

**Goal**: Run Differential Evolution to find optimal `GenerationParams`.

```rust
/// Run optimization and persist best parameters.
pub fn run_optimization(
    stdlib_funcs: &[StdlibFunction],
    eval_samples: usize,
    max_evaluations: usize,
) -> Result<OptimizedResult> {
    let config = OptimizerConfig {
        max_evaluations,
        population_size: 20,
        mutation_factor: 0.8,
        crossover_rate: 0.9,
        seed: Some(42),
    };

    let mut optimizer = MetaheuristicOptimizer::new(config);

    let result = optimizer.optimize(|params| {
        evaluate_fitness(params, stdlib_funcs, eval_samples)
    });

    // Persist best params to disk
    persist_params(&result.params)?;

    Ok(result)
}
```

**Success Criteria**:
- Find params with fitness > 0.6
- Converge within max_evaluations
- Params improve Oracle accuracy vs baseline (84%)

### 2.2 Integration Tests

**Goal**: Validate end-to-end corpus generation pipeline.

```rust
#[test]
fn test_end_to_end_corpus_generation() {
    // 1. Parse stdlib functions
    let stdlib_funcs = parse_stdlib_stubs();
    assert!(stdlib_funcs.len() >= 10);

    // 2. Generate examples
    let generator = PythonExampleGenerator::new(stdlib_funcs.clone());
    let examples = generator.generate(&stdlib_funcs, &SyntheticConfig::default())?;
    assert!(examples.len() >= 50);

    // 3. Transpile and compile (mock)
    let mut corpus_gen = SelfSupervisedCorpusGenerator::new(
        stdlib_funcs,
        CorpusConfig::default(),
    );

    // 4. Verify metrics
    let metrics = corpus_gen.metrics();
    assert!(metrics.acceptance_rate() > 0.5);
}
```

### 2.3 Curriculum Learning Extension

**Goal**: Generate examples in progressive difficulty order.

```rust
/// Difficulty levels for curriculum learning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DifficultyLevel {
    /// Simple single-function calls
    Basic,
    /// Multiple arguments, type variations
    Intermediate,
    /// Error-inducing, edge cases
    Advanced,
    /// Composition, complex patterns
    Expert,
}

/// Curriculum scheduler for progressive learning.
pub struct CurriculumScheduler {
    current_level: DifficultyLevel,
    samples_per_level: usize,
    samples_generated: usize,
}

impl CurriculumScheduler {
    /// Advance to next difficulty level.
    pub fn advance(&mut self) -> bool {
        if self.samples_generated >= self.samples_per_level {
            self.current_level = match self.current_level {
                DifficultyLevel::Basic => DifficultyLevel::Intermediate,
                DifficultyLevel::Intermediate => DifficultyLevel::Advanced,
                DifficultyLevel::Advanced => DifficultyLevel::Expert,
                DifficultyLevel::Expert => return false, // No more levels
            };
            self.samples_generated = 0;
            true
        } else {
            false
        }
    }
}
```

**Mapping to Generation Strategies**:
| Level | Strategies |
|-------|------------|
| Basic | DocstringMining, TypeEnumeration |
| Intermediate | TypeEnumeration with edge values |
| Advanced | ErrorInduction, EdgeCases |
| Expert | Composition, multi-step patterns |

### 2.4 Autofixer Integration

**Goal**: Wire generated corpus to AutoFixer training.

```rust
use crate::autofixer::{AutoFixer, FixContext, TransformRule};

/// Train autofixer on generated corpus.
pub fn train_autofixer_from_corpus(
    corpus: &TrainingDataset,
) -> Result<AutoFixer> {
    let mut fixer = AutoFixer::new();

    // Extract fix patterns from corpus
    for sample in corpus.samples() {
        if let Some(fix) = extract_fix_pattern(sample) {
            fixer.add_rule(fix);
        }
    }

    // Train n-gram predictor
    fixer.train_ngram_predictor(corpus.samples())?;

    Ok(fixer)
}

/// Extract fix pattern from training sample.
fn extract_fix_pattern(sample: &TrainingSample) -> Option<TransformRule> {
    // Match error category to fix template
    match sample.category {
        ErrorCategory::TypeMismatch => Some(TransformRule::TypeConversion),
        ErrorCategory::BorrowChecker => Some(TransformRule::AddClone),
        ErrorCategory::MissingImport => Some(TransformRule::AddImport),
        _ => None,
    }
}
```

## 3. Implementation Plan

### Phase 2.1: Optimizer CLI (RED)
- [ ] Add `depyler oracle optimize` subcommand
- [ ] Persist optimized params to `~/.depyler/oracle_params.json`
- [ ] Add progress reporting during optimization

### Phase 2.2: Integration Tests (RED)
- [ ] `tests/oracle_integration.rs` with end-to-end tests
- [ ] Mock transpile/compile for fast testing
- [ ] Verify metrics against baseline

### Phase 2.3: Curriculum Learning (RED)
- [ ] `DifficultyLevel` enum
- [ ] `CurriculumScheduler` struct
- [ ] Strategy weighting by difficulty

### Phase 2.4: Autofixer Integration (RED)
- [ ] `train_autofixer_from_corpus()` function
- [ ] N-gram training from corpus
- [ ] Fix pattern extraction

## 4. Success Criteria

| Metric | Target | Baseline |
|--------|--------|----------|
| Oracle Accuracy | >= 90% | 84% |
| Corpus Size | >= 1000 | 99 |
| Category Coverage | >= 6/7 | 5/7 |
| Autofixer Fix Rate | >= 30% | 0% |
| Optimization Time | < 60s | N/A |

## 5. Test Matrix

| Component | Unit Tests | Integration Tests | Property Tests |
|-----------|------------|-------------------|----------------|
| Optimizer | 5 | 2 | 1 |
| Curriculum | 4 | 1 | 1 |
| Autofixer | 5 | 2 | 1 |
| E2E Pipeline | - | 3 | - |

## 6. Oracle Improve Command (DEPYLER-0585)

Enterprise-grade continuous improvement loop for production codebases.

### 6.1 Usage

```bash
depyler oracle improve \
  --input-dir ./python-codebase \
  --target-rate 1.0 \
  --max-iterations 50 \
  --export-corpus ./corpus \
  --verbose
```

### 6.2 Training-Style Output

```
🧠 Training Loop Started
──────────────────────────────────────────────────────────────────────
 Epoch    Transpile      Compile         Rate            Δ     Status
──────────────────────────────────────────────────────────────────────
     1      155/212        64/212        30.2%          +64  ↑ improving
     2      155/212        89/212        42.0%          +25  ↑ improving
     3      155/212       112/212        52.8%          +23  ↑ improving
     ...
    47      155/212       212/212       100.0%           +1  ✓ DONE
──────────────────────────────────────────────────────────────────────
🎉 Target achieved: 100.0% compilation rate
```

### 6.3 Corpus Export Format

```jsonl
{"file":"example.py","error":"error[E0308]: mismatched types"}
{"file":"example.py","error":"error[E0599]: no method named `foo`"}
```

### 6.4 Error Category Distribution (Real Data)

| Error Code | Count | Description |
|------------|-------|-------------|
| E0308 | 434 | Mismatched types |
| E0599 | 373 | Method not found |
| E0433 | 327 | Unresolved module |
| E0432 | 291 | Unresolved import |
| E0277 | 232 | Trait not satisfied |
| E0282 | 147 | Type annotations needed |
| E0425 | 133 | Cannot find value |

## 7. References

- Phase 1 Spec: `docs/specifications/metaheuristic-oracle-spec.md`
- Phase 1 Review: `docs/reviews/metaheuristic-oracle-spec-review.md`
- Storn & Price (1997): Differential Evolution
- Bengio et al. (2009): Curriculum Learning

---

*Specification created: 2025-11-27*
*Updated: 2025-11-27 - Added Oracle Improve Command (DEPYLER-0585)*

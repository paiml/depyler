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

## 7. CITL MLOps Pipeline (GH-156)

### 7.1 Problem Statement

Depyler generates Rust code that sometimes fails to compile. We need a closed-loop system to:
1. **Collect** real compilation errors from transpilation attempts
2. **Train** ML models on actual error patterns (not synthetic)
3. **Fix** the transpiler to prevent future errors
4. **Share** training data with OIP for cross-project learning

### 7.2 Complete MLOps Pipeline

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    CITL (Compiler-in-the-Loop) Pipeline                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐          │
│  │  Python  │───▶│ Depyler  │───▶│   rustc  │───▶│  Errors  │          │
│  │  Source  │    │Transpile │    │ Compile  │    │  Corpus  │          │
│  └──────────┘    └──────────┘    └──────────┘    └────┬─────┘          │
│                                                        │                │
│                    ┌───────────────────────────────────┘                │
│                    ▼                                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    depyler oracle improve                        │   │
│  │  • Iterates until target compile rate (default: 100%)           │   │
│  │  • Exports corpus to .depyler-improve/<timestamp>/              │   │
│  │  • Tracks error category distribution                            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                    │                                                    │
│                    ▼                                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                  depyler oracle export-oip                       │   │
│  │  • Converts corpus to OIP format (Parquet/JSONL)                │   │
│  │  • Maps E0xxx codes to OIP DefectCategory taxonomy              │   │
│  │  • Applies Feldman (2020) long-tail reweighting                 │   │
│  │  • Exports to alimentar-compatible Arrow batches                │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                    │                                                    │
│                    ▼                                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                   OIP (Organizational Intelligence)              │   │
│  │  • Imports via import_depyler_corpus()                          │   │
│  │  • Combines with other project training data                    │   │
│  │  • Trains unified defect prediction model                       │   │
│  │  • Exports back to depyler for autofixer training               │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.3 Pipeline Steps (Exact Commands)

**Step 1: Generate Training Corpus**
```bash
# Run improve loop on Python codebase
depyler oracle improve \
  --input-dir ./python-project \
  --target-rate 1.0 \
  --max-iterations 50 \
  --export-corpus ./.depyler-improve

# Output: .depyler-improve/<timestamp>/corpus.jsonl
```

**Step 2: Export to OIP Format**
```bash
# Convert to Parquet with long-tail reweighting
depyler oracle export-oip \
  --input-dir ./.depyler-improve/latest \
  --output ./training_data.parquet \
  --format parquet \
  --min-confidence 0.80 \
  --include-clippy \
  --reweight 1.5

# Output: training_data.parquet (alimentar-compatible)
```

**Step 3: Import into OIP**
```rust
// In organization-intelligence-plugin
use oip::citl::import_depyler_corpus;

let examples = import_depyler_corpus("training_data.parquet")?;
// Returns Vec<DepylerExport> with mapped categories
```

**Step 4: Train and Deploy**
```bash
# Train MoE oracle on combined corpus
depyler oracle train --corpus ./data/unified_corpus.parquet

# Test autofixer on new errors
depyler oracle classify "error[E0308]: mismatched types"
```

### 7.4 Error Category Mapping

| Rust Error | OIP DefectCategory | Confidence | Weight |
|------------|-------------------|------------|--------|
| E0308 | TypeErrors | 0.95 | 1.0 |
| E0277 | TraitBounds | 0.95 | 1.5 |
| E0502/E0503/E0505 | OwnershipBorrow | 0.95 | 1.5 |
| E0106/E0621 | LifetimeErrors | 0.90 | 2.0 |
| E0433/E0432 | ImportErrors | 0.90 | 1.0 |
| E0599 | MethodNotFound | 0.85 | 1.2 |
| E0425 | UndefinedVariable | 0.85 | 1.0 |
| clippy::* | StyleViolations | 0.75 | 0.5 |

**Feldman Reweighting**: Rare error categories (LifetimeErrors, TraitBounds) receive higher weights to prevent model bias toward common errors.

### 7.5 Data Format Specifications

**CITL Corpus (JSONL)**:
```json
{"file":"example.py","rust_file":"example.rs","error":"error[E0308]: mismatched types","error_code":"E0308","line":42,"suggestion":"expected `i32`, found `&str`"}
```

**OIP Export (Parquet Schema)**:
```
source_file: Utf8
rust_file: Utf8
error_code: Utf8 (nullable)
clippy_lint: Utf8 (nullable)
level: Utf8
message: Utf8
oip_category: Utf8
confidence: Float64
line_start: Int64
line_end: Int64
suggestion: Utf8 (nullable)
python_construct: Utf8 (nullable)
timestamp: Int64
depyler_version: Utf8
weight: Float32
```

## 8. Model Training with alimentar/entrenar

### 8.1 Training Data Pipeline

The training pipeline uses alimentar for data loading and entrenar for model training:

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         Model Training Pipeline                           │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌──────────┐  │
│  │   Parquet   │───▶│  alimentar  │───▶│  entrenar   │───▶│  Model   │  │
│  │   Corpus    │    │  DataLoader │    │   Trainer   │    │  .gguf   │  │
│  └─────────────┘    └─────────────┘    └─────────────┘    └──────────┘  │
│                                                                          │
│  Features:                                                               │
│  • WeightedDataLoader for Feldman reweighting (--reweight 1.5)          │
│  • AsyncPrefetchDataset for parallel I/O                                 │
│  • TieredCurriculum for progressive difficulty                          │
│  • ExplainabilityCallback for feature attribution                       │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

### 8.2 Training Commands

**Step 1: Load Corpus with alimentar**
```rust
use alimentar::{ArrowDataset, WeightedDataLoader, AsyncPrefetchDataset};
use alimentar::streaming::ParquetSource;

// Load training corpus with weighted sampling
let dataset = ArrowDataset::from_parquet("training_data.parquet")?;
let weights = dataset.column_as_vec::<f32>("weight")?;

let loader = WeightedDataLoader::new(dataset, weights)?
    .batch_size(32)
    .num_samples(10_000)
    .seed(42);

// Or use async prefetch for large datasets
let source = ParquetSource::new("training_data.parquet", 1024)?;
let streaming = AsyncPrefetchDataset::from_parquet("training_data.parquet", 512, 4)?;
```

**Step 2: Train with entrenar**
```rust
use entrenar::train::{Trainer, TrainConfig, EarlyStopping, CheckpointCallback};
use entrenar::train::{TieredCurriculum, ExplainabilityCallback, ExplainMethod};
use entrenar::optim::AdamW;

// Create trainer with callbacks
let mut trainer = Trainer::new(params, Box::new(AdamW::new(0.0001, 0.9, 0.999, 1e-8, 0.01)),
    TrainConfig::default());

// Add CITL-specific callbacks
trainer.add_callback(EarlyStopping::new(5, 0.001));
trainer.add_callback(CheckpointCallback::new("./checkpoints"));
trainer.add_callback(TieredCurriculum::new(vec![0.6, 0.7, 0.8]));  // 60%→70%→80%
trainer.add_callback(ExplainabilityCallback::new(ExplainMethod::PermutationImportance)
    .with_top_k(10));

// Train
let result = trainer.train(100, || loader.iter(), |batch| model.forward(batch));
println!("Final loss: {:.4}, Efficiency: {:.4}",
    result.final_loss,
    result.accuracy / (result.corpus_size as f64).ln());
```

**Step 3: CLI Training Command**
```bash
# Train oracle model
depyler oracle train \
  --corpus ./training_data.parquet \
  --epochs 100 \
  --batch-size 32 \
  --lr 0.0001 \
  --curriculum 60,70,80 \
  --reweight 1.5 \
  --output ./oracle_model.gguf

# Expected output:
# Epoch  1/100: loss=2.3456, acc=45.2%, tier=1
# Epoch 10/100: loss=1.2345, acc=62.1%, tier=1 → tier=2 ↑
# Epoch 25/100: loss=0.5678, acc=71.5%, tier=2 → tier=3 ↑
# ...
# Training complete: acc=89.3%, efficiency=0.847
```

### 8.3 Training Configuration (YAML)

```yaml
# oracle-train.yaml
model:
  type: moe_oracle
  experts: 8
  hidden_dim: 256

data:
  train: training_data.parquet
  batch_size: 32
  weighted: true
  prefetch: 4

optimizer:
  name: adamw
  lr: 0.0001
  weight_decay: 0.01

training:
  epochs: 100
  grad_clip: 1.0
  early_stopping:
    patience: 5
    min_delta: 0.001

curriculum:
  thresholds: [0.6, 0.7, 0.8]

callbacks:
  - checkpoint:
      dir: ./checkpoints
      save_best: true
  - explainability:
      method: permutation_importance
      top_k: 10
```

```bash
entrenar train oracle-train.yaml
```

### 8.4 Monitoring Training Progress

**Real-time Metrics** (via MonitorCallback):
```
┌─────────────────────────────────────────────────────────────────┐
│ Oracle Training Monitor                                         │
├─────────────────────────────────────────────────────────────────┤
│ Epoch: 25/100          Tier: 2 (Intermediate)                   │
│ Loss:  0.5678          Accuracy: 71.5%                          │
│ LR:    0.00008         Grad Norm: 0.234                         │
│ ETA:   12m 34s         Efficiency: 0.723                        │
├─────────────────────────────────────────────────────────────────┤
│ Top Features (Permutation Importance):                          │
│   1. error_code      0.342                                      │
│   2. message_length  0.187                                      │
│   3. has_suggestion  0.156                                      │
│   4. line_number     0.089                                      │
│   5. file_type       0.067                                      │
└─────────────────────────────────────────────────────────────────┘
```

## 9. Work Tracking & Monitoring

### 9.1 GitHub Issue Tracking

| Issue | Description | Status | Assignee |
|-------|-------------|--------|----------|
| [GH-156](https://github.com/paiml/depyler/issues/156) | OIP Export Command | ✅ Done | - |
| [GH-157](https://github.com/paiml/depyler/issues/157) | CITL Integration QA | 🔄 Open | - |
| alimentar#3 | WeightedDataLoader | ✅ Closed | - |
| alimentar#4 | AsyncPrefetchDataset | ✅ Closed | - |

### 9.2 Monitor When Training Starts

**Option 1: Watch GH-157 for activity**
```bash
# Check issue status
gh issue view 157 -R paiml/depyler --json state,assignees,comments

# Subscribe to notifications
gh issue edit 157 -R paiml/depyler --add-assignee @me
```

**Option 2: Check CI/CD for training jobs**
```bash
# List recent workflow runs
gh run list -R paiml/depyler --workflow=train

# Watch for new runs
gh run watch -R paiml/depyler
```

**Option 3: Monitor corpus directory**
```bash
# Check if training corpus exists
ls -la ~/.depyler/training_corpus/

# Watch for model checkpoints
watch -n 60 'ls -la ~/.depyler/checkpoints/ 2>/dev/null'
```

### 9.3 Training Readiness Checklist

Before starting model training, verify:

- [ ] `training_data.parquet` exists and has >1000 samples
- [ ] alimentar WeightedDataLoader tested (`cargo test -p alimentar weighted`)
- [ ] entrenar TieredCurriculum tested (`cargo test -p entrenar curriculum`)
- [ ] GH-157 QA validation complete
- [ ] Oracle improve loop run on real codebase

**Verification Script**:
```bash
#!/bin/bash
# check_training_ready.sh

echo "=== Training Readiness Check ==="

# Check corpus
if [ -f "training_data.parquet" ]; then
  rows=$(python3 -c "import pyarrow.parquet as pq; print(pq.read_table('training_data.parquet').num_rows)")
  echo "✅ Corpus: $rows samples"
else
  echo "❌ Corpus: training_data.parquet not found"
fi

# Check alimentar
if cargo test -p alimentar weighted --quiet 2>/dev/null; then
  echo "✅ alimentar: WeightedDataLoader tests pass"
else
  echo "❌ alimentar: WeightedDataLoader tests fail"
fi

# Check entrenar
if cargo test -p entrenar curriculum --quiet 2>/dev/null; then
  echo "✅ entrenar: Curriculum tests pass"
else
  echo "❌ entrenar: Curriculum tests fail"
fi

# Check GH-157 status
status=$(gh issue view 157 -R paiml/depyler --json state -q '.state')
echo "📋 GH-157 Status: $status"
```

## 10. Implementation Verification (GH-156, GH-157)

### 10.1 Depyler Components ✅

| Component | File | Status | Tests |
|-----------|------|--------|-------|
| OipTrainingExample | `compilation_trainer.rs:1400` | ✅ Done | 2 |
| map_error_to_oip_category | `compilation_trainer.rs:1420` | ✅ Done | - |
| OipExportFormat | `compilation_trainer.rs:1470` | ✅ Done | - |
| export_oip_corpus | `compilation_trainer.rs:1480` | ✅ Done | 2 |
| load_corpus_cache | `compilation_trainer.rs:1550` | ✅ Done | - |
| ExportOip CLI | `lib.rs:OracleCommands` | ✅ Done | - |
| data_store module | `data_store.rs` | ✅ Done | 2 |

### 10.2 OIP Components ✅

| Component | File | Status | Tests |
|-----------|------|--------|-------|
| DepylerExport struct | `citl/mod.rs` | ✅ Done | 1 |
| import_depyler_corpus | `citl/mod.rs` | ✅ Done | 3 |
| Category mapping | `citl/mod.rs` | ✅ Done | 1 |

### 10.3 Sister Project Integration ✅

| Project | Purpose | Status |
|---------|---------|--------|
| alimentar | Arrow/Parquet I/O | ✅ Synced |
| aprender | ML models, book chapter | ✅ Synced |
| depyler | CITL training, OIP export | ✅ Synced |
| OIP | Cross-project corpus import | ✅ Synced |

### 10.4 Test Results

```
# CITL Spec Tests
cargo test --package depyler citl_spec
running 20 tests ... ok

# Data Store Tests
cargo test --package depyler-oracle data_store
running 2 tests ... ok

# OIP Import Tests
cargo test --lib depyler (in OIP)
running 3 tests ... ok
```

## 11. How This Solves Our Problem

### 11.1 Before CITL
- Transpiler bugs discovered ad-hoc
- No systematic error collection
- Manual fix pattern identification
- No cross-project learning

### 11.2 After CITL
1. **Automated Error Collection**: Every compilation failure captured
2. **Categorized Corpus**: Errors mapped to actionable categories
3. **ML-Powered Fixes**: Oracle suggests fixes based on patterns
4. **Cross-Project Learning**: OIP combines depyler data with other tools
5. **Continuous Improvement**: Feedback loop improves transpiler

### 11.3 Expected Outcomes

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Error Classification | Manual | Automated | ∞ |
| Fix Suggestion Rate | 0% | 30%+ | +30% |
| Corpus Size | 99 synthetic | 1000+ real | 10x |
| Category Coverage | 5/7 | 7/7 | 100% |
| Cross-Project Data | None | Full OIP | New |

## 12. Code Review & External QA Validation

### 12.1 Review Checklist

**Architecture Review** (Lead Engineer)
- [ ] Pipeline stages are decoupled and testable independently
- [ ] Data formats documented with schemas (§7.5)
- [ ] Error category mapping is exhaustive for Rust errors
- [ ] Feldman reweighting rationale documented
- [ ] Cross-project integration points clearly defined

**Code Quality Review** (Senior Developer)
- [ ] All public APIs have doc comments
- [ ] Error handling uses `Result<T>` with descriptive errors
- [ ] No unwrap/expect in production paths
- [ ] Complexity ≤10 per function (PMAT enforced)
- [ ] Test coverage ≥80% (cargo-llvm-cov)

**Security Review** (Security Engineer)
- [ ] No arbitrary code execution from corpus data
- [ ] Parquet files validated before deserialization
- [ ] File paths sanitized in export commands
- [ ] No secrets in training data schemas

### 12.2 QA Validation Commands

**Step 1: Verify Test Suite**
```bash
# All CITL-related tests
cargo test --package depyler citl_spec -- --nocapture
cargo test --package depyler-oracle data_store -- --nocapture

# Expected: 22 tests passing
```

**Step 2: End-to-End Pipeline Test**
```bash
# Create test corpus
mkdir -p /tmp/qa_test
echo 'def add(x: int, y: int) -> int: return x + y' > /tmp/qa_test/test.py

# Run improve loop (1 iteration)
cargo run -- oracle improve \
  --input-dir /tmp/qa_test \
  --max-iterations 1 \
  --export-corpus /tmp/qa_corpus

# Verify corpus created
ls -la /tmp/qa_corpus/
# Expected: corpus.jsonl file

# Export to OIP format
cargo run -- oracle export-oip \
  --input-dir /tmp/qa_corpus \
  --output /tmp/qa_test.parquet \
  --format parquet

# Verify Parquet created
file /tmp/qa_test.parquet
# Expected: Apache Parquet file
```

**Step 3: Verify OIP Import**
```bash
cd ../organization-intelligence-plugin
cargo test --lib depyler -- --nocapture

# Expected: 3 tests passing
```

**Step 4: Verify Sister Projects**
```bash
# All repos should be clean
for repo in alimentar aprender depyler organization-intelligence-plugin; do
  echo "=== $repo ==="
  (cd ../$repo && git status --short)
done
# Expected: No uncommitted changes
```

### 12.3 Acceptance Criteria

| Criterion | Validation Method | Pass/Fail |
|-----------|------------------|-----------|
| Corpus generation works | `oracle improve` creates JSONL | |
| OIP export works | `oracle export-oip` creates Parquet | |
| Category mapping complete | 8+ error codes mapped | |
| Tests pass | 25+ tests green | |
| No regressions | `cargo test --workspace` | |
| Documentation complete | §7-9 filled in | |

### 12.4 Sign-Off Requirements

**Required Approvals**:
1. **Tech Lead**: Architecture and design approval
2. **QA Engineer**: All validation commands pass
3. **Security**: No vulnerabilities in data handling
4. **Product Owner**: Feature meets requirements

**Sign-Off Template**:
```
CITL MLOps Pipeline Review Sign-Off

Date: ____________
Reviewer: ____________
Role: ____________

[ ] I have reviewed the specification (§7-9)
[ ] I have executed the QA validation commands (§10.2)
[ ] All acceptance criteria pass (§10.3)
[ ] I approve this implementation for production

Signature: ____________
Notes: ____________
```

### 12.5 Known Limitations

| Limitation | Impact | Mitigation |
|------------|--------|------------|
| Clippy lints not fully mapped | Some style issues uncategorized | Default to StyleViolations |
| Parquet schema fixed | Adding fields requires migration | Version schema in header |
| OIP import is one-way | No bidirectional sync | Future: Add export-from-oip |
| Reweighting is static | Doesn't adapt to corpus changes | Future: Dynamic reweighting |

### 12.6 Rollback Procedure

If issues discovered post-deployment:

```bash
# 1. Disable export command
git revert <commit-hash>

# 2. Clear generated corpora
rm -rf .depyler-improve/

# 3. Restore previous Oracle model
cp ~/.depyler/oracle_params.json.bak ~/.depyler/oracle_params.json

# 4. Notify OIP team to discard imported data
```

## 13. References

- Phase 1 Spec: `docs/specifications/metaheuristic-oracle-spec.md`
- Phase 1 Review: `docs/reviews/metaheuristic-oracle-spec-review.md`
- Storn & Price (1997): Differential Evolution
- Bengio et al. (2009): Curriculum Learning
- Feldman (2020): Does Learning Require Memorization? (Long-tail reweighting)
- alimentar: Arrow-based dataset management
- OIP: Organizational Intelligence Plugin architecture

---

*Specification created: 2025-11-27*
*Updated: 2025-11-27 - Added Oracle Improve Command (DEPYLER-0585)*
*Updated: 2025-11-28 - Added CITL MLOps Pipeline, Implementation Verification (GH-156, GH-157)*
*Updated: 2025-11-28 - Added Code Review & External QA Validation (§12)*
*Updated: 2025-11-28 - Added Model Training with alimentar/entrenar (§8), Work Tracking & Monitoring (§9)*

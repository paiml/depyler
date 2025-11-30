# ORACLE-WEAKSUP: Weak Supervision for Error Classification

## Status: TODO
## Priority: P0
## Created: 2025-11-30

## Problem Statement

170 failed transpilations (28%) are currently wasted. Weak supervision can auto-label
these failures using Snorkel-style labeling functions, turning failures into training data.

## Proposed Solution

Use aprender `weak_supervision` to:
1. Define labeling functions (LFs) that classify errors by regex/pattern
2. Train label model to combine noisy LF outputs
3. Generate probabilistic labels for oracle training

## Implementation

### Codebase Dependencies

| Crate | Module | Purpose |
|-------|--------|---------|
| aprender | `weak_supervision::LabelModel` | Snorkel-style label aggregation |
| aprender | `weak_supervision::LFOutput` | Labeling function outputs |
| depyler-oracle | `classifier::ErrorCategory` | Target label space |

### Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│ Failed Sample   │────▶│ Labeling Funcs   │────▶│ Label Model     │
│ (error message) │     │ (LF1, LF2, LF3)  │     │ (Snorkel)       │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                                          │
                              ┌────────────────────────────┘
                              ▼
                        ┌─────────────────┐
                        │ Probabilistic   │
                        │ Labels (0.0-1.0)│
                        └─────────────────┘
```

### Book Chapter Reference

- aprender: `docs/specifications/weak-supervision-spec.md`
- Snorkel paper: Ratner et al. (2017) VLDB

### Cargo Run Examples

```bash
# Train label model on failed transpilations
cargo run -p depyler-oracle --example train_label_model -- \
  --corpus data/depyler_citl_corpus_v2.parquet \
  --output models/label_model.apr

# Apply labels to unlabeled failures
cargo run -p depyler-oracle --example apply_weak_labels -- \
  --model models/label_model.apr \
  --input data/failures.parquet \
  --output data/labeled_failures.parquet

# Evaluate LF coverage and accuracy
cargo run -p depyler-oracle --example lf_analysis -- \
  --corpus data/depyler_citl_corpus_v2.parquet
```

### API Design

```rust
// crates/depyler-oracle/src/weak_label.rs
use aprender::weak_supervision::{LabelModel, LFOutput, ABSTAIN};

pub struct WeakLabeler {
    label_model: LabelModel,
    labeling_functions: Vec<Box<dyn LabelingFunction>>,
}

pub trait LabelingFunction: Send + Sync {
    fn name(&self) -> &str;
    fn apply(&self, error_msg: &str, python_code: &str) -> LFOutput;
}

impl WeakLabeler {
    pub fn new() -> Self;

    /// Register a labeling function
    pub fn add_lf(&mut self, lf: Box<dyn LabelingFunction>);

    /// Train label model on labeled subset
    pub fn fit(&mut self, samples: &[LabeledSample]) -> Result<(), OracleError>;

    /// Predict probabilistic labels
    pub fn predict(&self, error_msg: &str, python_code: &str) -> Vec<(ErrorCategory, f32)>;
}

// Built-in labeling functions
pub struct TypeMismatchLF;      // "expected X, found Y"
pub struct MissingItemLF;       // "cannot find"
pub struct BorrowErrorLF;       // "borrow", "move"
pub struct LifetimeLF;          // "'a", "lifetime"
pub struct AsyncErrorLF;        // "async", "await", "Future"
pub struct UnsupportedFeatureLF; // "not supported", "unsupported"
```

### Labeling Functions

| LF | Pattern | Target Category | Expected Accuracy |
|----|---------|-----------------|-------------------|
| TypeMismatchLF | `expected .*, found` | TypeMismatch | 90% |
| MissingItemLF | `cannot find\|not found` | MissingItem | 85% |
| BorrowErrorLF | `borrow\|move\|&mut` | BorrowError | 80% |
| LifetimeLF | `lifetime\|'[a-z]` | LifetimeError | 75% |
| AsyncErrorLF | `async\|await\|Future` | AsyncError | 95% |
| UnsupportedLF | `not supported\|unsupported` | UnsupportedFeature | 90% |

## Acceptance Criteria

- [ ] 6+ labeling functions implemented
- [ ] Label model trained on 170 failures
- [ ] Coverage ≥80% (LFs label ≥80% of samples)
- [ ] Estimated accuracy ≥75%
- [ ] TDD with ≥95% coverage
- [ ] Integration with oracle training pipeline

## Test Plan

```rust
#[test]
fn test_type_mismatch_lf() {
    let lf = TypeMismatchLF;
    let output = lf.apply("expected i32, found String", "x: int = 'hello'");
    assert_eq!(output, LFOutput::Label(ErrorCategory::TypeMismatch.index()));
}

#[test]
fn test_label_model_aggregation() {
    let mut labeler = WeakLabeler::new();
    labeler.add_lf(Box::new(TypeMismatchLF));
    labeler.add_lf(Box::new(MissingItemLF));

    let probs = labeler.predict("expected i32, found &str", "x = 'hello'");
    assert!(probs[0].1 > 0.7); // TypeMismatch should be high confidence
}
```

## References

- Issue: https://github.com/paiml/depyler/issues/188
- Snorkel: https://arxiv.org/abs/1711.10160
- Corpus: https://huggingface.co/datasets/paiml/depyler-citl

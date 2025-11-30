# ORACLE-SYNTHETIC: Synthetic Error Augmentation for Oracle Training

## Status: TODO
## Priority: P0
## Created: 2025-11-30

## Problem Statement

The depyler oracle has only 606 training pairs (439 successful). Synthetic augmentation
can multiply this 5x by generating error→fix variations from successful transpilations.

## Proposed Solution

Use entrenar `citl` + aprender `synthetic` to:
1. Mutate successful Python→Rust pairs to introduce errors
2. Capture depyler error messages
3. Generate error→fix training pairs

## Implementation

### Codebase Dependencies

| Crate | Module | Purpose |
|-------|--------|---------|
| entrenar | `citl::DecisionCITL` | Pattern mining from mutations |
| aprender | `synthetic::SyntheticFactory` | Code mutation generation |
| depyler-oracle | `corpus_citl` | Integration point |

### Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│ Successful Pair │────▶│ Mutation Engine  │────▶│ Error Capture   │
│ (Python→Rust)   │     │ (aprender)       │     │ (depyler)       │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                                          │
                              ┌────────────────────────────┘
                              ▼
                        ┌─────────────────┐
                        │ CITL Pattern    │
                        │ Store (entrenar)│
                        └─────────────────┘
```

### Book Chapter Reference

- entrenar: `book/src/advanced/citl.md`
- aprender: `docs/specifications/synthetic-data-spec.md`

### Cargo Run Examples

```bash
# Generate synthetic errors from successful pairs
cargo run -p depyler-oracle --example synthetic_augment -- \
  --input data/depyler_citl_corpus_v2.parquet \
  --output data/augmented_corpus.parquet \
  --mutations 5

# Verify augmentation quality
cargo run -p depyler-oracle --example validate_augmented -- \
  --corpus data/augmented_corpus.parquet
```

### API Design

```rust
// crates/depyler-oracle/src/synthetic_augment.rs
pub struct SyntheticAugmenter {
    mutation_engine: aprender::synthetic::SyntheticFactory,
    citl: entrenar::citl::DecisionCITL,
}

impl SyntheticAugmenter {
    pub fn new() -> Result<Self, OracleError>;

    /// Generate N mutations per successful pair
    pub fn augment_pair(
        &mut self,
        python: &str,
        rust: &str,
        n_mutations: usize,
    ) -> Result<Vec<AugmentedSample>, OracleError>;

    /// Augment entire corpus
    pub fn augment_corpus(
        &mut self,
        corpus_path: &Path,
        output_path: &Path,
        mutations_per_sample: usize,
    ) -> Result<AugmentStats, OracleError>;
}

pub struct AugmentedSample {
    pub original_python: String,
    pub mutated_python: String,
    pub error_message: String,
    pub fix_suggestion: String,
    pub mutation_type: MutationType,
}

pub enum MutationType {
    TypeChange,      // int → str
    NameMangle,      // var → var_typo
    OperatorSwap,    // + → -
    StatementDelete, // remove line
    ImportRemove,    // remove import
}
```

## Acceptance Criteria

- [ ] 5+ mutation types implemented
- [ ] Augmented corpus ≥3000 pairs (5x original)
- [ ] All mutations produce valid error messages
- [ ] TDD with ≥95% coverage
- [ ] Integration test with real corpus
- [ ] Book chapter updated

## Test Plan

```rust
#[test]
fn test_type_change_mutation() {
    let augmenter = SyntheticAugmenter::new().unwrap();
    let samples = augmenter.augment_pair(
        "def add(a: int, b: int) -> int: return a + b",
        "fn add(a: i32, b: i32) -> i32 { a + b }",
        1,
    ).unwrap();
    assert!(!samples.is_empty());
    assert!(samples[0].error_message.contains("type"));
}
```

## References

- Issue: https://github.com/paiml/depyler/issues/188
- Corpus: https://huggingface.co/datasets/paiml/depyler-citl

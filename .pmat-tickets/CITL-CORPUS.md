# CITL-CORPUS: Integrate entrenar DecisionCITL for Error Pattern Mining

## Status: IN_PROGRESS
## Priority: P0
## Created: 2025-11-30

## Problem Statement

The depyler-oracle crate uses `aprender::citl` for iterative fixing but doesn't
leverage `entrenar::citl::DecisionCITL` for Tarantula fault localization and 
pattern mining from the reprorusted corpus.

## Current State

- 606 Python-Rust pairs in corpus
- 170 pairs (28%) fail transpilation
- CITLFixer exists but no systematic pattern mining from failures

## Proposed Solution

1. **Mine Error Patterns**: Extract error→fix patterns from 436 successful pairs
2. **Fault Localization**: Use Tarantula algorithm to identify suspicious AST decisions
3. **Pattern Store**: Build BM25+dense hybrid retrieval for fix suggestions
4. **Integration**: Connect to CITLFixer for runtime fix suggestions

## Acceptance Criteria

- [ ] Tests pass for pattern extraction from corpus
- [ ] DecisionCITL ingests successful transpilation sessions
- [ ] Pattern store persists and loads error patterns
- [ ] Fix suggestions improve transpilation success rate
- [ ] All code follows Extreme TDD methodology

## Technical Design

```rust
// New module: crates/depyler-oracle/src/corpus_citl.rs
pub struct CorpusCITL {
    trainer: entrenar::citl::DecisionCITL,
    pattern_store: entrenar::citl::DecisionPatternStore,
}

impl CorpusCITL {
    pub fn ingest_from_parquet(&mut self, path: &Path) -> Result<()>;
    pub fn suggest_fix(&self, error: &str, context: &[String]) -> Vec<FixSuggestion>;
}
```

## References

- entrenar CITL module: `/home/noah/src/entrenar/src/citl/`
- Corpus: `/home/noah/src/reprorusted-python-cli/data/depyler_citl_corpus.parquet`
- Issue: https://github.com/paiml/depyler/issues/186

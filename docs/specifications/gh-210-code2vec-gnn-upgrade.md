# GH-210: Upgrade Depyler Oracle with Aprender Code2Vec & GNN

## Objective

Upgrade `depyler-oracle` to use proper AST-based embeddings and enhanced GNN capabilities from `aprender`.

## Current State Analysis

### Existing Implementation

1. **Feature Extraction** (`features.rs`): 12 hand-crafted features
   - Message length, keyword counts, structural features
   - Batch extraction to `Matrix<f32>` for Random Forest

2. **AST Embeddings** (`ast_embeddings.rs`): Heuristic line-by-line parsing
   - Basic Code2Vec-style path extraction
   - 128-dimensional embeddings
   - Limited to pattern matching on raw text

3. **GNN Encoder** (`gnn_encoder.rs`): Working but limited
   - Uses `aprender::citl::GNNErrorEncoder`
   - 256-dimensional embeddings
   - Linear similarity search (no HNSW index)

4. **Combined Embedding**: 512 dimensions (12 + 128 + 128 + 256)

### Identified Improvements

| Component | Current | Target |
|-----------|---------|--------|
| AST Extraction | Heuristic regex | HIR/syn proper parsing |
| Feature Dims | 12 | 73 (with one-hot + keywords) |
| Similarity Search | Linear O(n) | HNSW O(log n) |
| GNN Training | Static patterns | Online fine-tuning |

## Implementation Plan

### Phase 1: Enhanced Feature Engineering (12 → 73 dims)

**File**: `crates/depyler-oracle/src/features.rs`

Add:
- Error code one-hot encoding (25 dims): E0308, E0425, E0433, etc.
- Keyword occurrence counts (36 dims): type, borrow, lifetime, trait keywords
- Total: 12 + 25 + 36 = 73 dimensions

```rust
pub struct EnhancedErrorFeatures {
    // Existing 12 features
    pub base_features: ErrorFeatures,
    // New error code one-hot (25 common error codes)
    pub error_code_onehot: [f32; 25],
    // Keyword occurrence counts (36 categories)
    pub keyword_counts: [f32; 36],
}
```

### Phase 2: Proper AST Path Extraction

**File**: `crates/depyler-oracle/src/ast_embeddings.rs`

Upgrade from heuristic to proper AST parsing:

1. **Python Source**: Use `rustpython-parser` (already in workspace)
   - Extract function definitions, variable assignments, class structures
   - Build proper Code2Vec path contexts

2. **Rust Source**: Use `syn` crate (already in workspace)
   - Extract fn signatures, let bindings, impl blocks
   - Match paths between Python source → Rust generated

```rust
pub fn extract_python_paths_via_hir(source: &str) -> Vec<PathContext> {
    // Use depyler-core's HirExpr for proper Python AST
    // ...
}

pub fn extract_rust_paths_via_syn(source: &str) -> Vec<PathContext> {
    // Use syn::parse_file for proper Rust AST
    // ...
}
```

### Phase 3: HNSW Index for Similarity Search

**File**: `crates/depyler-oracle/src/gnn_encoder.rs`

Replace linear search with HNSW (Hierarchical Navigable Small World):

```rust
use aprender::index::hnsw::{HnswIndex, HnswConfig};

pub struct DepylerGnnEncoder {
    config: GnnEncoderConfig,
    encoder: GNNErrorEncoder,
    hnsw_index: Option<HnswIndex<f32>>,  // NEW: HNSW index
    ast_embedder: Option<AstEmbedder>,
    patterns: HashMap<String, StructuralPattern>,
    stats: GnnEncoderStats,
}

impl DepylerGnnEncoder {
    pub fn find_similar_hnsw(&self, embedding: &[f32], k: usize) -> Vec<(usize, f32)> {
        // O(log n) similarity search
        self.hnsw_index.as_ref()?.search(embedding, k)
    }
}
```

### Phase 4: Integration with Oracle Query Loop

**File**: `crates/depyler-oracle/src/lib.rs`

Wire enhanced features into main Oracle:

```rust
impl Oracle {
    pub fn classify_with_enhanced_features(&self, error: &str) -> (ErrorCategory, f64) {
        // 1. Extract enhanced 73-dim features
        let features = self.enhanced_extractor.extract(error);

        // 2. Get AST embeddings if source available
        let ast_embed = self.ast_embedder.embed_combined(python_src, rust_src);

        // 3. Get GNN embedding with HNSW lookup
        let gnn_embed = self.gnn_encoder.encode_with_hnsw(error);

        // 4. Combine and classify
        let combined = concat_features(&features, &ast_embed, &gnn_embed);
        self.classifier.predict(&combined)
    }
}
```

## Testing Strategy

### Unit Tests

1. **Feature extraction**: Verify 73-dim output
2. **AST path extraction**: Test Python/Rust parsing
3. **HNSW index**: Verify O(log n) search complexity
4. **End-to-end**: Classify known error patterns

### Property Tests

1. Feature vectors always non-negative
2. AST paths deterministic for same input
3. HNSW returns consistent nearest neighbors

### Integration Tests

1. Retrain oracle with enhanced features
2. Measure accuracy improvement on validation set
3. Benchmark query latency (should improve with HNSW)

## Success Metrics

1. **Accuracy**: +10% improvement in error classification (target: 85%+)
2. **Latency**: Query time < 10ms with HNSW vs 50ms linear
3. **Coverage**: Handle "UNKNOWN" errors better (reduce from 15% to 5%)

## Dependencies

- `aprender` v0.20.2 (existing)
- `rustpython-parser` (existing in workspace)
- `syn` (existing in workspace)
- No new crates required

## Timeline

- Phase 1: Enhanced Features - ✅ COMPLETE
- Phase 2: AST Extraction - ✅ COMPLETE
- Phase 3: HNSW Index - ✅ COMPLETE
- Phase 4: Integration - ✅ COMPLETE

## Implementation Summary

### Phase 1: Enhanced Feature Engineering (12 → 73 dims)
- `ERROR_CODES`: 25 common Rust error codes for one-hot encoding
- `KEYWORD_CATEGORIES`: 9 categories with 36 keyword features
- `EnhancedErrorFeatures`: Combined 73-dimensional features
- 8 tests passing

### Phase 2: Proper AST Path Extraction
- `PythonPathVisitor`: rustpython-parser based Python AST traversal
- `RustPathVisitor`: syn-based Rust AST traversal
- Function, class, method, parameter, control flow extraction
- Fallback to heuristic on parse errors
- 6 tests passing

### Phase 3: HNSW Index for Similarity Search
- `HNSWIndex` from aprender::index::hnsw
- O(log n) similarity search (vs O(n) linear)
- Configurable M and ef_construction parameters
- Stats tracking for HNSW vs linear queries
- 9 tests passing

### Phase 4: Integration with Oracle Query Loop
- `Oracle::classify_enhanced()`: Combined classification method
- Uses Random Forest + GNN + HNSW + AST embeddings
- `EnhancedClassificationResult`: Rich result with all signals
- Pattern fix extraction from structural matches
- 7 tests passing

**Total: 30 new tests across all phases**

## References

- [Code2Vec Paper](https://arxiv.org/abs/1803.09473)
- [HNSW Paper](https://arxiv.org/abs/1603.09320)
- aprender documentation
- entrenar CITL spec

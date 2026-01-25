# GH-209: Rich Signal Reporting with ML Clustering & Graph Analysis

## Objective

Transform `depyler report` from a simple error counter into an ML-powered diagnostic engine that classifies failures by Python semantic domain and uses clustering to identify root cause patterns.

## Current State Analysis

### Existing Implementation

1. **AnalysisResult** (`report_cmd/analysis.rs`): Basic struct
   - name, success, error_code, error_message
   - No AST features or import tracking

2. **ErrorEntry**: Simple count + samples (max 3)
   - No clustering or similarity detection
   - Linear O(n) sample collection

3. **Error Taxonomy**: HashMap<String, ErrorEntry>
   - Groups by error code only (E0425, E0308, etc.)
   - No semantic domain classification

### Identified Improvements

| Component | Current | Target |
|-----------|---------|--------|
| Classification | Error code only | Semantic domain (Core/Stdlib/External) |
| Clustering | None | KMeans/DBSCAN on feature vectors |
| Analysis | Simple counts | Graph centrality metrics |
| Output | Basic JSON | Rich JSON with --advanced flag |

## Implementation Plan

### Phase 1: Extended AnalysisResult with AST Features

**File**: `crates/depyler/src/report_cmd/analysis.rs`

Add semantic domain tracking and AST features:

```rust
/// Extended analysis result with ML features (GH-209)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedAnalysisResult {
    pub base: AnalysisResult,
    pub semantic_domain: SemanticDomain,
    pub ast_features: AstFeatures,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticDomain {
    CoreLanguage,   // for, if, class, def, yield
    StdlibCommon,   // sys, os, re, collections, json
    StdlibAdvanced, // asyncio, multiprocessing, typing
    External,       // numpy, pandas, requests
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstFeatures {
    pub function_count: usize,
    pub class_count: usize,
    pub loop_count: usize,
    pub async_count: usize,
    pub comprehension_count: usize,
    pub complexity_score: f32,
}
```

### Phase 2: ML Clustering Integration (Aprender)

**File**: `crates/depyler/src/report_cmd/clustering.rs` (new)

```rust
use aprender::cluster::{KMeans, KMeansConfig, DBSCAN, DBSCANConfig};
use aprender::primitives::{Matrix, Vector};

/// Error feature vector for clustering
pub struct ErrorFeatureVector {
    pub error_code_idx: usize,     // One-hot position
    pub ast_node_type: usize,      // Enum index
    pub library_imported: usize,   // Stdlib/external index
    pub function_complexity: f32,  // Complexity score
    pub file_size: f32,            // Normalized file size
    pub import_count: usize,       // Number of imports
}

/// Cluster analysis results
pub struct ClusterAnalysis {
    pub clusters: Vec<ErrorCluster>,
    pub silhouette_score: f32,
    pub outliers: Vec<usize>,
}

pub struct ErrorCluster {
    pub id: usize,
    pub centroid: Vec<f32>,
    pub member_indices: Vec<usize>,
    pub dominant_error_code: String,
    pub dominant_domain: SemanticDomain,
    pub label: String,  // Auto-generated: "DataFrame iteration failures"
}

impl ErrorClusterAnalyzer {
    pub fn cluster_errors(&self, results: &[ExtendedAnalysisResult]) -> ClusterAnalysis {
        // 1. Build feature matrix
        let features = self.build_feature_matrix(results);

        // 2. Run KMeans with optimal k (elbow method)
        let k = self.find_optimal_k(&features);
        let kmeans = KMeans::new(KMeansConfig { n_clusters: k, ..Default::default() });

        // 3. Identify outliers with DBSCAN
        let dbscan = DBSCAN::new(DBSCANConfig::default());

        // 4. Build cluster descriptors
        self.build_cluster_descriptors(results, labels)
    }
}
```

### Phase 3: Semantic Domain Classifier

**File**: `crates/depyler/src/report_cmd/semantic_classifier.rs` (new)

```rust
/// Classify Python imports into semantic domains
pub fn classify_imports(imports: &[String]) -> SemanticDomain {
    let core_keywords = ["for", "if", "class", "def", "yield", "lambda", "with"];
    let stdlib_common = ["sys", "os", "re", "collections", "json", "io", "pathlib"];
    let stdlib_advanced = ["asyncio", "multiprocessing", "typing", "dataclasses"];
    let external = ["numpy", "pandas", "requests", "django", "flask"];

    // Scoring: External > StdlibAdvanced > StdlibCommon > CoreLanguage
    // ...
}

/// Extract imports from Python source
pub fn extract_imports(python_source: &str) -> Vec<String> {
    // Use rustpython-parser for accurate extraction
}
```

### Phase 4: Graph Analysis (Error Dependency Graph)

**File**: `crates/depyler/src/report_cmd/graph_analysis.rs` (new)

```rust
use std::collections::{HashMap, HashSet};

/// Error dependency graph for centrality analysis
pub struct ErrorGraph {
    nodes: Vec<ErrorNode>,
    edges: Vec<ErrorEdge>,
    adjacency: HashMap<usize, Vec<usize>>,
}

pub struct ErrorNode {
    pub id: usize,
    pub error_code: String,
    pub files: Vec<String>,
    pub centrality: f32,  // PageRank-style score
}

pub struct ErrorEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f32,  // Co-occurrence frequency
}

impl ErrorGraph {
    /// Build graph from error co-occurrences in files
    pub fn from_results(results: &[ExtendedAnalysisResult]) -> Self { ... }

    /// Calculate PageRank-style centrality
    pub fn calculate_centrality(&mut self) {
        // Simplified PageRank (power iteration)
        let damping = 0.85;
        let iterations = 100;
        // ...
    }

    /// Find error communities (connected components)
    pub fn find_communities(&self) -> Vec<ErrorCommunity> {
        // Simple Louvain-style community detection
    }
}

pub struct ErrorCommunity {
    pub id: usize,
    pub name: String,  // "The AsyncIO Cluster"
    pub error_codes: Vec<String>,
    pub centrality_sum: f32,
}
```

### Phase 5: Report Output Enhancement

**File**: `crates/depyler/src/report_cmd/mod.rs`

Add `--advanced` flag for rich output:

```rust
#[derive(Args)]
pub struct ReportArgs {
    #[arg(long)]
    pub advanced: bool,  // NEW: Enable ML-powered analysis

    #[arg(long)]
    pub output: Option<PathBuf>,  // Output file (JSON/HTML)
}

/// Advanced report output
#[derive(Serialize)]
pub struct AdvancedReport {
    pub summary: ReportSummary,
    pub domain_breakdown: DomainBreakdown,
    pub clusters: Vec<ErrorCluster>,
    pub graph_analysis: GraphAnalysis,
}

pub struct DomainBreakdown {
    pub core_lang_pass_rate: f64,
    pub stdlib_common_pass_rate: f64,
    pub stdlib_advanced_pass_rate: f64,
    pub external_pass_rate: f64,
}

pub struct GraphAnalysis {
    pub top_central_errors: Vec<ErrorNode>,
    pub communities: Vec<ErrorCommunity>,
}
```

## Testing Strategy

### Unit Tests

1. **Semantic classification**: Verify domain detection for imports
2. **Feature extraction**: Test AST feature extraction accuracy
3. **Clustering**: Verify cluster formation with synthetic data
4. **Graph analysis**: Test centrality calculation correctness

### Property Tests

1. Clustering always produces valid assignments
2. Centrality scores sum to 1.0
3. Domain classification is deterministic

### Integration Tests

1. Full pipeline: Load corpus -> Cluster -> Report
2. Benchmark: Clustering time < 500ms for 1000 files

## Success Metrics

1. **Semantic Insight**: Break down pass/fail by domain (Core: 82%, External: 45%)
2. **Cluster Quality**: Silhouette score > 0.5
3. **Actionable Output**: Top 3 error communities with fix recommendations
4. **Performance**: `--advanced` adds < 1s overhead

## Dependencies

- `aprender` v0.20.2 (existing) - KMeans, DBSCAN
- `rustpython-parser` (existing) - Import extraction
- No new crates required

## Timeline

- Phase 1: Extended AnalysisResult - ✅ COMPLETE (35 tests)
- Phase 2: ML Clustering - ✅ COMPLETE (24 tests)
- Phase 3: Semantic Domain Classifier - ✅ COMPLETE (integrated in Phase 1)
- Phase 4: Graph Analysis - ✅ COMPLETE (20 tests)
- Phase 5: Report Enhancement - PENDING

## Implementation Summary

### Phase 1: Extended AnalysisResult (103 total tests)
- `SemanticDomain` enum: CoreLanguage, StdlibCommon, StdlibAdvanced, External, Unknown
- `AstFeatures` struct: function_count, class_count, loop_count, async_count, etc.
- `ExtendedAnalysisResult`: combines base result with ML features
- `DomainBreakdown`: tracks pass/fail rates by semantic domain
- `extract_imports()`: parses Python imports from source
- `classify_domain()`: categorizes code by import patterns
- `extract_ast_features()`: heuristic AST feature extraction

### Phase 2: ML Clustering (24 tests)
- `ErrorFeatureVector`: 10-dimensional feature vector for clustering
- `simple_kmeans()`: Pure Rust KMeans implementation
- `ErrorCluster`: labeled cluster with centroid, members, dominant error/domain
- `ClusterAnalysis`: results with silhouette score and outliers
- `ErrorClusterAnalyzer`: configurable analyzer with auto-k detection
- Auto-generated cluster labels: "Type Mismatch - External Packages (5 files)"

### Phase 3: Semantic Domain Classifier (integrated)
- Implemented as part of Phase 1
- STDLIB_COMMON, STDLIB_ADVANCED, EXTERNAL_PACKAGES constants
- Priority-based classification: External > Advanced > Common > Core

### Phase 4: Graph Analysis (20 tests)
- `ErrorGraph`: nodes (error types) and edges (co-occurrences)
- `ErrorNode`: error_code, files, centrality, domain
- `ErrorEdge`: from, to, weight (co-occurrence count)
- `calculate_centrality()`: PageRank-style centrality scores
- `find_communities()`: BFS-based connected component detection
- `ErrorCommunity`: auto-named clusters like "The Type Mismatch Cluster"
- `GraphAnalysis`: full analysis with density, top_central, communities

# Depyler Graph Architecture Specification

**DEPYLER-1300: The Neural Leap**

**Version**: 1.0.0
**Date**: 2025-01-23
**Status**: ACTIVE

---

## Executive Summary

This document specifies the architectural shift from **AST-walking** to **Graph-walking** for error resolution. The current 28.2% compilation rate cannot be improved by linear bug-fixing. The next 20% requires **Structural Knowledge** - understanding not just *what* errors occur, but *where they came from*.

### The Problem

```
E0308: mismatched types
  expected `i32`, found `DepylerValue`
```

AST-walking sees: "Type mismatch at line 42"
Graph-walking sees: "Data flows from function A → B → C; type degraded at B"

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    DEPYLER GRAPH ENGINE                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   Python    │    │ Dependency  │    │   Error     │     │
│  │   Source    │───▶│   Graph     │───▶│   Overlay   │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                            │                   │            │
│                            ▼                   ▼            │
│                    ┌─────────────┐    ┌─────────────┐      │
│                    │   Impact    │◀───│ Vectorized  │      │
│                    │   Scorer    │    │  Failures   │      │
│                    └─────────────┘    └─────────────┘      │
│                            │                   │            │
│                            ▼                   ▼            │
│                    ┌─────────────┐    ┌─────────────┐      │
│                    │  Patient    │    │   Graph     │      │
│                    │    Zero     │    │   Oracle    │      │
│                    └─────────────┘    └─────────────┘      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 2. Core Components

### 2.1 Dependency Graph (`depyler-graph::builder`)

**Purpose**: Build a directed graph of Python entities and their relationships.

**Nodes** (GraphNode):
- Functions
- Classes
- Modules
- Methods

**Edges** (GraphEdge):
- `Calls`: Function A calls function B
- `Imports`: Module A imports module B
- `Inherits`: Class A inherits from class B

**Key Properties**:
- `line`: Source line number
- `column`: Source column
- `error_count`: Number of errors rooted at this node
- `impact_score`: PageRank-style influence metric

### 2.2 Error Overlay (`depyler-graph::error_overlay`)

**Purpose**: Map Rust compiler errors back to Python graph nodes.

**Process**:
1. Parse rustc error output (code, message, line)
2. Estimate Python line from Rust line (source map heuristic)
3. Find nearest graph node by proximity
4. Associate error with node + confidence score
5. Identify upstream suspects (callers, base classes)

**Output**: `OverlaidError` with:
- `node_id`: Associated Python function/class
- `association_confidence`: 0.0-1.0
- `upstream_suspects`: Potential root cause nodes

### 2.3 Impact Scorer (`depyler-graph::impact`)

**Purpose**: Identify "Patient Zero" - the root cause functions.

**Algorithm**: Modified PageRank with Error Propagation

```rust
// Iterate until convergence
for _ in 0..20 {
    for node in nodes {
        // Base score from errors
        let base = direct_errors[node] + downstream_errors[node];

        // PageRank contribution from callers
        let pagerank = 0.85 * sum(caller.score / caller.out_degree);

        // Damping factor
        scores[node] = 0.15 + pagerank + base * 0.1;
    }
}
```

**Output**: `PatientZero` list ordered by impact score

### 2.4 Vectorized Failures (`depyler-graph::vectorize`)

**Purpose**: Create ML-ready datasets from errors.

**Schema**:
```json
{
  "id": "failure_42",
  "error_code": "E0308",
  "error_message": "expected i32, found DepylerValue",
  "ast_context": {
    "containing_function": "process_data",
    "containing_class": "DataProcessor",
    "return_type": "Dict[str, int]",
    "parameter_types": ["List[int]", "str"],
    "statement_kind": "return",
    "expression_kind": "call",
    "ast_depth": 3
  },
  "graph_context": {
    "node_id": "DataProcessor.process_data",
    "in_degree": 5,
    "out_degree": 2,
    "callees": ["helper_a", "helper_b"],
    "callers": ["main", "test_process", ...],
    "inheritance_chain": ["BaseProcessor"]
  },
  "labels": {
    "category": "type_mismatch",
    "subcategory": "depyler_value_leak",
    "fix_type": "type_annotation",
    "confidence": 0.85
  }
}
```

## 3. The Graph Oracle

### 3.1 Training Data Generation

**From AST-only** (previous approach):
```python
# Training sample
input: "E0308: expected i32, found DepylerValue"
output: "add type annotation"
```

**To Graph-aware** (new approach):
```python
# Training sample with graph context
input: {
    "error": "E0308: expected i32, found DepylerValue",
    "in_degree": 5,         # Called by 5 functions
    "out_degree": 2,        # Calls 2 functions
    "centrality": 0.73,     # High-traffic node
    "depth": 3,             # 3 levels deep in call chain
    "callers": ["main", "handler"],  # Who calls this
    "return_annotation": "Dict"      # Type hint present
}
output: {
    "fix": "propagate_type_from_callers",
    "confidence": 0.92
}
```

### 3.2 Feature Engineering

**Graph Centrality Features**:
| Feature | Description | Importance |
|---------|-------------|------------|
| `in_degree` | Number of callers | High - indicates impact radius |
| `out_degree` | Number of callees | Medium - indicates complexity |
| `pagerank` | PageRank score | High - indicates influence |
| `betweenness` | Path centrality | High - identifies bottlenecks |
| `depth` | Distance from entry point | Medium - context isolation |

**Error Context Features**:
| Feature | Description | Importance |
|---------|-------------|------------|
| `error_code` | E0308, E0599, etc. | High - error type |
| `expected_type` | What Rust expected | High - target type |
| `found_type` | What was found | High - source of bug |
| `upstream_errors` | Errors in callers | Medium - cascade indicator |

### 3.3 Oracle Training Pipeline

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│ reprorusted     │     │ vectorize_      │     │  Graph Oracle   │
│ corpus          │────▶│ failures()      │────▶│  Training       │
│ (280 files)     │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                      │                       │
         │                      │                       │
         ▼                      ▼                       ▼
    Python AST           NDJSON Dataset         Trained Model
    + Rust Errors       (graph context)         (fix predictor)
```

**Training Command**:
```bash
# Step 1: Generate vectorized failures
depyler graph vectorize \
    --corpus ../reprorusted-python-cli \
    --output failures.ndjson

# Step 2: Train Graph Oracle
depyler oracle train \
    --input failures.ndjson \
    --model graph-oracle-v1 \
    --features graph-aware

# Step 3: Validate
depyler oracle validate \
    --model graph-oracle-v1 \
    --corpus ../reprorusted-python-cli \
    --roi-output docs/oracle_roi_metrics.json
```

## 4. Impact Strike Protocol

### 4.1 Identify Top 5 Patient Zeros

```bash
# Analyze corpus and identify highest-impact failures
depyler graph analyze \
    --corpus ../reprorusted-python-cli \
    --top 5 \
    --output patient-zeros.json
```

**Expected Output**:
```json
{
  "patient_zeros": [
    {
      "node_id": "config.load_settings",
      "impact_score": 0.92,
      "direct_errors": 3,
      "downstream_errors": 47,
      "error_codes": ["E0308", "E0599"],
      "callers_blocked": 12
    },
    // ... top 5
  ]
}
```

### 4.2 Do Not Fix Manually

**Critical**: The goal is NOT to manually fix these functions.

**Instead**:
1. Vectorize the failures to create training data
2. Feed to Graph Oracle for pattern learning
3. Let Oracle suggest fixes
4. Measure acceptance rate

This creates a **positive feedback loop** where:
- High-impact fixes → Higher acceptance rate
- Graph centrality → Better fix predictions

## 5. ROI Loop Integration

### 5.1 Metrics Tracking

Every convergence run writes to `docs/oracle_roi_metrics.json`:

```json
{
  "timestamp": "2025-01-23T10:30:00Z",
  "session": "converge-20250123-103000",
  "baseline": {
    "files_processed": 280,
    "compile_errors": 1002
  },
  "oracle_performance": {
    "suggestions_made": 150,
    "suggestions_accepted": 87,
    "acceptance_rate": 0.58,
    "by_centrality": {
      "high_centrality_acceptance": 0.82,
      "low_centrality_acceptance": 0.41
    }
  },
  "roi_metrics": {
    "high_confidence_fixes": 87,
    "estimated_savings_cents": 348
  }
}
```

### 5.2 Hypothesis

> **H1**: Graph centrality positively correlates with fix acceptance rate.

**Validation**:
- Track `acceptance_rate` by node centrality bucket
- Expected: High-centrality nodes have higher acceptance
- Rationale: Central nodes have more context, better inference

## 6. Implementation Roadmap

### Phase 1: Infrastructure (COMPLETE)
- [x] DEPYLER-1300: Create `depyler-graph` crate
- [x] DEPYLER-1301: Reconnect ROI metrics
- [x] DEPYLER-1302: This design document

### Phase 2: Integration (NEXT)
- [ ] DEPYLER-1303: CLI commands for graph analysis
- [ ] DEPYLER-1304: Vectorization pipeline for corpus
- [ ] DEPYLER-1305: Patient Zero identification script

### Phase 3: Oracle Retraining
- [ ] DEPYLER-1306: Graph-aware feature extraction
- [ ] DEPYLER-1307: Oracle training with graph features
- [ ] DEPYLER-1308: Validation against baseline

### Phase 4: ROI Validation
- [ ] DEPYLER-1309: Centrality vs acceptance correlation analysis
- [ ] DEPYLER-1310: Production deployment of Graph Oracle

## 7. Success Criteria

| Metric | Current | Target | Method |
|--------|---------|--------|--------|
| Compilation Rate | 28.2% | 50% | Graph Oracle fixes |
| E0308 Resolution | ~30% | 60% | Type propagation from graph |
| Fix Acceptance Rate | Unknown | 70% | High-centrality targeting |
| ROI (savings) | $29.24 | $100+ | Higher confidence fixes |

## 8. References

- `crates/depyler-graph/src/lib.rs` - Core API
- `crates/depyler-graph/src/builder.rs` - Graph construction
- `crates/depyler-graph/src/impact.rs` - PageRank scorer
- `crates/depyler-graph/src/vectorize.rs` - ML dataset generation
- `docs/specifications/1.0-single-shot-compile.md` - Single-shot spec
- `docs/oracle_roi_metrics.json` - ROI tracking

---

**End of Specification**

*"We are no longer guessing at types. We are solving the graph."*

# Decision Traces Signal Specification

**Version**: 1.0.0
**Status**: Draft
**Authors**: depyler team
**Date**: 2025-11-29

## Abstract

This specification defines a system for capturing transpiler decision traces during Python→Rust code generation, enabling Compiler-in-the-Loop (CITL) training signal collection. The system integrates with renacer for trace capture and entrenar for model training, supporting autonomous overnight sessions with remote monitoring.

## 1. Introduction

### 1.1 Problem Statement

Current transpiler error analysis relies on post-hoc rustc error messages. This reactive approach lacks:
1. **Causality**: Which codegen decision led to the error?
2. **Context**: What alternatives were considered?
3. **Correlation**: Which decision patterns predict success/failure?

### 1.2 Solution Overview

Instrument depyler's codegen with decision traces that capture:
- Decision points (branch choices in codegen)
- Decision context (types, patterns, HIR nodes)
- Decision outcomes (success/error correlation)

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     AUTONOMOUS SESSION                          │
├─────────────────────────────────────────────────────────────────┤
│  depyler transpile                                              │
│       │                                                         │
│       ▼                                                         │
│  ┌─────────────┐    ┌──────────────┐    ┌───────────────────┐  │
│  │ HIR Builder │───▶│ Rust CodeGen │───▶│ Decision Traces   │  │
│  └─────────────┘    └──────────────┘    │ (msgpack mmap)    │  │
│                                          └─────────┬─────────┘  │
│                                                    │            │
│                                                    ▼            │
│                                          ┌─────────────────┐    │
│                                          │ renacer daemon  │    │
│                                          │ (trace ingest)  │    │
│                                          └────────┬────────┘    │
└───────────────────────────────────────────────────┼─────────────┘
                                                    │
                                                    ▼ OTLP/gRPC
┌─────────────────────────────────────────────────────────────────┐
│                     REMOTE MONITOR                              │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐                     │
│  │ entrenar        │    │ Dashboard       │                     │
│  │ (CITL trainer)  │    │ (Grafana/CLI)   │                     │
│  └────────┬────────┘    └─────────────────┘                     │
│           │                                                      │
│           ▼                                                      │
│  ┌─────────────────────────────────────────┐                    │
│  │ Pattern Library (HNSW index)            │                    │
│  │ - (decision_seq, error_code) → fix_diff │                    │
│  └─────────────────────────────────────────┘                    │
└─────────────────────────────────────────────────────────────────┘
```

## 3. Decision Trace Schema

### 3.1 Core Trace Structure

```rust
use renacer::decision_trace::{DecisionTrace, generate_decision_id};

/// Decision point categories in depyler codegen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionCategory {
    TypeMapping,      // Python type → Rust type
    BorrowStrategy,   // &T vs T vs &mut T
    LifetimeInfer,    // 'a annotations
    MethodDispatch,   // trait method resolution
    ImportResolve,    // use statements
    ErrorHandling,    // Result/Option wrapping
    Ownership,        // move vs clone vs borrow
}

/// Extended decision trace for CITL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepylerDecision {
    /// Base trace from renacer
    pub trace: DecisionTrace,

    /// Decision category
    pub category: DecisionCategory,

    /// Python AST node hash (for pattern matching)
    pub py_ast_hash: u64,

    /// Chosen codegen path
    pub chosen_path: String,

    /// Alternatives considered
    pub alternatives: Vec<String>,

    /// Confidence score (0.0-1.0)
    pub confidence: f32,

    /// Source span in Python
    pub py_span: (usize, usize),

    /// Target span in generated Rust
    pub rs_span: Option<(usize, usize)>,
}
```

### 3.2 Decision Point Macro

```rust
/// Macro for instrumenting decision points in codegen
#[macro_export]
macro_rules! trace_decision {
    ($category:expr, $chosen:expr, $alternatives:expr, $confidence:expr) => {{
        #[cfg(feature = "decision-tracing")]
        {
            use $crate::decision_trace::{DECISION_WRITER, DepylerDecision};
            use renacer::decision_trace::generate_decision_id;

            let decision = DepylerDecision {
                trace: renacer::decision_trace::DecisionTrace {
                    id: generate_decision_id(
                        stringify!($category),
                        $chosen,
                        file!(),
                        line!(),
                    ),
                    timestamp_ns: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64,
                    thread_id: {
                        use std::hash::{Hash, Hasher};
                        let mut h = std::collections::hash_map::DefaultHasher::new();
                        std::thread::current().id().hash(&mut h);
                        h.finish()
                    },
                    source_file: file!().to_string(),
                    source_line: line!(),
                },
                category: $category,
                py_ast_hash: 0, // Set by caller
                chosen_path: $chosen.to_string(),
                alternatives: $alternatives.iter().map(|s| s.to_string()).collect(),
                confidence: $confidence,
                py_span: (0, 0),
                rs_span: None,
            };

            if let Some(writer) = DECISION_WRITER.get() {
                let _ = writer.lock().unwrap().append(&decision);
            }
        }
    }};
}
```

## 4. Integration Points

### 4.1 expr_gen.rs Instrumentation

```rust
// In rust_gen/expr_gen.rs

fn generate_binop(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<TokenStream> {
    let lhs_ty = self.infer_type(left)?;
    let rhs_ty = self.infer_type(right)?;

    // Decision: numeric promotion strategy
    let (strategy, alternatives, confidence) = match (&lhs_ty, &rhs_ty) {
        (RustType::I32, RustType::I64) => ("promote_lhs", vec!["cast_rhs", "error"], 0.9),
        (RustType::F32, RustType::F64) => ("promote_lhs", vec!["cast_rhs", "error"], 0.85),
        (RustType::String, _) => ("stringify_rhs", vec!["error"], 0.7),
        _ => ("direct", vec![], 1.0),
    };

    trace_decision!(
        DecisionCategory::TypeMapping,
        strategy,
        alternatives,
        confidence
    );

    // ... rest of codegen
}
```

### 4.2 borrowing_context.rs Instrumentation

```rust
// In borrowing_context.rs

fn determine_borrow_strategy(&self, var: &str, usage: &Usage) -> BorrowStrategy {
    let (strategy, alternatives, confidence) = match usage {
        Usage::Read => ("immutable_ref", vec!["clone", "move"], 0.95),
        Usage::Write => ("mutable_ref", vec!["owned"], 0.8),
        Usage::Consume => ("move", vec!["clone"], 0.75),
        Usage::Multiple => ("clone", vec!["rc", "arc"], 0.6),
    };

    trace_decision!(
        DecisionCategory::BorrowStrategy,
        strategy,
        alternatives,
        confidence
    );

    // ... resolve strategy
}
```

## 5. Remote Monitoring Integration

### 5.1 renacer Daemon Configuration

```toml
# /etc/renacer/depyler-monitor.toml

[ingest]
watch_paths = ["/tmp/depyler_decisions.msgpack"]
poll_interval_ms = 100

[export]
otlp_endpoint = "http://localhost:4317"
batch_size = 100
flush_interval_ms = 1000

[sampling]
# Sample 10% of decisions for remote export (full local capture)
remote_sample_rate = 0.1
# Circuit breaker: max 1000 decisions/sec remote
max_remote_rate = 1000
```

### 5.2 trueno-rag Pattern Retrieval

Use trueno-rag for hybrid retrieval of error-fix patterns. This combines:
- **Sparse retrieval** (BM25): Lexical matching on error codes and messages
- **Dense retrieval**: Semantic similarity of decision sequences
- **Fusion**: Reciprocal Rank Fusion (RRF) for optimal combination

```rust
// In entrenar training loop

use trueno_rag::{
    pipeline::RagPipelineBuilder,
    chunk::FixedSizeChunker,
    embed::TruenoEmbedder,
    rerank::CrossEncoderReranker,
    fusion::FusionStrategy,
    Document,
};
use itertools::Itertools;

/// Decision pattern store using trueno-rag hybrid retrieval
pub struct DecisionPatternStore {
    pipeline: RagPipeline,
    fix_registry: HashMap<ChunkId, FixPattern>,
}

/// A stored fix pattern
#[derive(Debug, Clone)]
pub struct FixPattern {
    pub error_code: String,
    pub decision_sequence: Vec<String>,
    pub fix_diff: String,
    pub success_count: u32,
    pub failure_count: u32,
}

impl DecisionPatternStore {
    pub fn new() -> Result<Self> {
        // Build RAG pipeline with hybrid retrieval
        let pipeline = RagPipelineBuilder::new()
            .chunker(FixedSizeChunker::new(256)) // Decision sequences are short
            .embedder(TruenoEmbedder::new(384)?) // Trueno-native embeddings
            .reranker(CrossEncoderReranker::new()?) // Rerank for precision
            .fusion(FusionStrategy::RRF { k: 60.0 }) // Reciprocal Rank Fusion
            .build()?;

        Ok(Self {
            pipeline,
            fix_registry: HashMap::new(),
        })
    }

    /// Index a successful fix pattern
    pub fn index_fix(&mut self, pattern: FixPattern) -> Result<()> {
        // Create document from decision sequence + error context
        let doc_text = format!(
            "ERROR: {} DECISIONS: {} FIX: {}",
            pattern.error_code,
            pattern.decision_sequence.join(" → "),
            pattern.fix_diff.lines().take(5).collect::<Vec<_>>().join(" ")
        );

        let doc = Document::new(&doc_text)
            .with_metadata("error_code", &pattern.error_code)
            .with_metadata("success_rate",
                &format!("{:.2}", pattern.success_count as f32 /
                    (pattern.success_count + pattern.failure_count) as f32));

        let chunk_id = self.pipeline.index_document(&doc)?;
        self.fix_registry.insert(chunk_id, pattern);
        Ok(())
    }

    /// Query for fix suggestions given error context
    pub fn suggest_fix(
        &self,
        error_code: &str,
        decision_context: &[String],
        k: usize,
    ) -> Result<Vec<&FixPattern>> {
        // Build query combining error and decision context
        let query = format!(
            "ERROR: {} CONTEXT: {}",
            error_code,
            decision_context.join(" → ")
        );

        // Hybrid retrieval with reranking
        let results = self.pipeline.query(&query, k)?;

        // Return fix patterns sorted by fused score
        Ok(results
            .iter()
            .filter_map(|r| self.fix_registry.get(&r.chunk.id))
            .collect())
    }
}

/// CITL trainer using trueno-rag pattern store
pub struct DecisionCITL {
    pattern_store: DecisionPatternStore,
}

impl DecisionCITL {
    /// Ingest decision traces and correlate with compilation outcomes
    pub fn ingest_session(
        &mut self,
        traces: &[DepylerDecision],
        outcome: CompileOutcome,
        fix_diff: Option<&str>,
    ) -> Result<()> {
        // Group decisions by file
        let by_file = traces.iter().group_by(|d| &d.trace.source_file);

        for (_file, decisions) in &by_file {
            let decision_seq: Vec<_> = decisions
                .map(|d| d.chosen_path.clone())
                .collect();

            match &outcome {
                CompileOutcome::Success => {
                    // Update success counts for matching patterns
                    // (pattern matching logic)
                }
                CompileOutcome::Error { code, .. } => {
                    if let Some(diff) = fix_diff {
                        // Index new fix pattern
                        let pattern = FixPattern {
                            error_code: code.clone(),
                            decision_sequence: decision_seq,
                            fix_diff: diff.to_string(),
                            success_count: 1,
                            failure_count: 0,
                        };
                        self.pattern_store.index_fix(pattern)?;
                    }
                }
            }
        }
        Ok(())
    }
}
```

### 5.3 Autonomous Session Hooks

```json
// .claude/settings.json additions

{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "/home/noah/src/depyler/scripts/decision_trace_export.sh",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

```bash
#!/bin/bash
# scripts/decision_trace_export.sh
# Export decision traces to remote entrenar instance

set -euo pipefail

TRACE_FILE="/tmp/depyler_decisions.msgpack"
REMOTE_ENDPOINT="${ENTRENAR_ENDPOINT:-http://localhost:8080/ingest}"

if [[ -f "$TRACE_FILE" ]]; then
    # Check if file has new data (mtime changed)
    CURRENT_MTIME=$(stat -c %Y "$TRACE_FILE")
    LAST_MTIME=$(cat /tmp/depyler_trace_mtime 2>/dev/null || echo 0)

    if [[ "$CURRENT_MTIME" -gt "$LAST_MTIME" ]]; then
        # Export via renacer CLI
        renacer export --format otlp --endpoint "$REMOTE_ENDPOINT" "$TRACE_FILE" &
        echo "$CURRENT_MTIME" > /tmp/depyler_trace_mtime
    fi
fi
```

## 6. Dashboard Integration

### 6.1 Metrics Exposed

| Metric | Type | Description |
|--------|------|-------------|
| `depyler_decisions_total` | Counter | Total decisions made |
| `depyler_decisions_by_category` | Counter | Decisions per category |
| `depyler_decision_confidence` | Histogram | Confidence distribution |
| `depyler_decision_alternatives` | Histogram | Alternatives considered |
| `depyler_decision_success_rate` | Gauge | Success rate per path |

### 6.2 CLI Dashboard Extension

```bash
# Add to overnight_dashboard.sh

echo "┌─ DECISION TRACES ─────────────────────────────────────────┐"
if [[ -f "/tmp/depyler_decisions.msgpack" ]]; then
    TRACE_COUNT=$(renacer stats /tmp/depyler_decisions.msgpack 2>/dev/null | grep count | awk '{print $2}')
    echo "│ Decisions captured: $TRACE_COUNT"
    renacer stats /tmp/depyler_decisions.msgpack 2>/dev/null | grep -E "category|confidence" | sed 's/^/│ /'
else
    echo "│ No decision traces"
fi
echo "└──────────────────────────────────────────────────────────┘"
```

## 7. Implementation Plan

### Phase 1: Core Infrastructure (Week 1)
1. Add `decision-tracing` feature flag to depyler-core
2. Implement `DepylerDecision` struct
3. Create `trace_decision!` macro
4. Add thread-local `MmapDecisionWriter`

### Phase 2: Codegen Instrumentation (Week 2)
1. Instrument `expr_gen.rs` (20 decision points)
2. Instrument `stmt_gen.rs` (15 decision points)
3. Instrument `borrowing_context.rs` (10 decision points)
4. Instrument `type_mapper.rs` (8 decision points)

### Phase 3: Remote Integration (Week 3)
1. Configure renacer daemon for trace ingestion
2. Implement entrenar CITL integration
3. Add overnight session hooks
4. Create dashboard extensions

### Phase 4: Pattern Learning (Week 4)
1. Collect baseline traces from 311 examples
2. Train initial pattern embeddings
3. Implement suggestion API
4. Validate on held-out examples

## 8. Performance Considerations

### 8.1 Overhead Budget

| Component | Target | Measurement |
|-----------|--------|-------------|
| Decision capture | <1μs/decision | Memory-mapped write |
| Sampling overhead | <0.1% | Branch prediction friendly |
| Remote export | <5% | Async, batched |
| Total transpile overhead | <10% | End-to-end |

### 8.2 Memory Budget

- Decision trace: ~128 bytes/decision
- Typical file: ~50 decisions
- Session buffer: 10MB mmap (78,000 decisions)
- Circular buffer eviction at 80% capacity

## 9. Error-Decision Correlation

### 9.1 Span Mapping Algorithm

When rustc reports an error at a Rust span, correlate with decisions:

```rust
/// Find decisions that contributed to error at given Rust span
fn correlate_error(
    decisions: &[DepylerDecision],
    error_span: (usize, usize),
) -> Vec<&DepylerDecision> {
    decisions
        .iter()
        .filter(|d| {
            d.rs_span.map_or(false, |(start, end)| {
                // Decision's output overlaps with error location
                start <= error_span.1 && end >= error_span.0
            })
        })
        .collect()
}
```

### 9.2 Causal Chain Reconstruction

Build decision dependency graph to find root cause:

```
Error E0308 at line 47
    └── Decision: TypeMapping → "promote_lhs" (line 45)
        └── Decision: BorrowStrategy → "immutable_ref" (line 42)
            └── Decision: ImportResolve → "std::convert" (line 3)
```

## 10. Graceful Degradation

### 10.1 Renacer Unavailable

If renacer is not installed, fall back to JSON logging:

```rust
#[cfg(feature = "decision-tracing")]
fn get_writer() -> Box<dyn DecisionWriter> {
    if renacer_available() {
        Box::new(MmapDecisionWriter::new("/tmp/depyler_decisions.msgpack"))
    } else {
        // Fallback: append-only JSON
        Box::new(JsonFileWriter::new("/tmp/depyler_decisions.jsonl"))
    }
}
```

### 10.2 Remote Endpoint Unavailable

Queue traces locally, retry with exponential backoff:

```toml
[export.retry]
max_attempts = 5
initial_backoff_ms = 100
max_backoff_ms = 30000
queue_size = 10000
```

## 11. Security Considerations

1. **No secrets in traces**: Decision traces contain only structural info
2. **Rate limiting**: Circuit breaker prevents DoS via trace flooding
3. **Local-first**: Full traces local only; sampled subset to remote
4. **Authenticated export**: OTLP export requires auth token

---

## References

1. **Ball, T. & Larus, J.R.** (1996). Efficient Path Profiling. *MICRO-29*, 46-57. doi:10.1109/MICRO.1996.566449

2. **Liblit, B., Aiken, A., Zheng, A.X., & Jordan, M.I.** (2005). Bug Isolation via Remote Program Sampling. *PLDI*, 141-154. doi:10.1145/1065010.1065014

3. **Chilimbi, T.M., Liblit, B., Mehra, K., Nori, A.V., & Vaswani, K.** (2009). HOLMES: Effective Statistical Debugging via Efficient Path Profiling. *ICSE*, 34-44. doi:10.1109/ICSE.2009.5070506

4. **Zeller, A.** (2002). Isolating Cause-Effect Chains from Computer Programs. *FSE*, 1-10. doi:10.1145/587051.587053

5. **Jones, J.A. & Harrold, M.J.** (2005). Empirical Evaluation of the Tarantula Automatic Fault-Localization Technique. *ASE*, 273-282. doi:10.1145/1101908.1101949

6. **Xu, G., Arnold, M., Mitchell, N., Rountev, A., & Sevitsky, G.** (2009). Go with the Flow: Profiling Copies to Find Runtime Bloat. *PLDI*, 419-430. doi:10.1145/1542476.1542523

7. **Brun, Y. & Ernst, M.D.** (2004). Finding Latent Code Errors via Machine Learning over Program Executions. *ICSE*, 480-490. doi:10.1109/ICSE.2004.1317470

8. **Mytkowicz, T., Diwan, A., Hauswirth, M., & Sweeney, P.F.** (2010). Evaluating the Accuracy of Java Profilers. *PLDI*, 187-197. doi:10.1145/1806596.1806618

9. **Curtsinger, C. & Berger, E.D.** (2015). Coz: Finding Code that Counts with Causal Profiling. *SOSP*, 184-197. doi:10.1145/2815400.2815409

10. **Pradel, M. & Gross, T.R.** (2011). Detecting Anomalies in the Order of Equally-Typed Method Arguments. *ISSTA*, 232-242. doi:10.1145/2001420.2001447

### Retrieval-Augmented Generation & Hybrid Search

11. **Lewis, P., Perez, E., Piktus, A., et al.** (2020). Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks. *NeurIPS*, 33, 9459-9474. arXiv:2005.11401

12. **Karpukhin, V., Oguz, B., Min, S., et al.** (2020). Dense Passage Retrieval for Open-Domain Question Answering. *EMNLP*, 6769-6781. doi:10.18653/v1/2020.emnlp-main.550

13. **Robertson, S. & Zaragoza, H.** (2009). The Probabilistic Relevance Framework: BM25 and Beyond. *Foundations and Trends in IR*, 3(4), 333-389. doi:10.1561/1500000019

14. **Cormack, G.V., Clarke, C.L.A., & Buettcher, S.** (2009). Reciprocal Rank Fusion Outperforms Condorcet and Individual Rank Learning Methods. *SIGIR*, 758-759. doi:10.1145/1571941.1572114

15. **Nogueira, R. & Cho, K.** (2019). Passage Re-ranking with BERT. *arXiv:1901.04085*.

16. **Izacard, G. & Grave, E.** (2021). Leveraging Passage Retrieval with Generative Models for Open Domain Question Answering. *EACL*, 874-880. doi:10.18653/v1/2021.eacl-main.74

17. **Guu, K., Lee, K., Tung, Z., Pasupat, P., & Chang, M.** (2020). REALM: Retrieval-Augmented Language Model Pre-Training. *ICML*, 3929-3938.

18. **Khattab, O. & Zaharia, M.** (2020). ColBERT: Efficient and Effective Passage Search via Contextualized Late Interaction over BERT. *SIGIR*, 39-48. doi:10.1145/3397271.3401075

19. **Xiong, L., Xiong, C., Li, Y., et al.** (2021). Approximate Nearest Neighbor Negative Contrastive Learning for Dense Text Retrieval. *ICLR*. arXiv:2007.00808

20. **Borgeaud, S., Mensch, A., Hoffmann, J., et al.** (2022). Improving Language Models by Retrieving from Trillions of Tokens. *ICML*, 2206-2240. arXiv:2112.04426

---

## Appendix A: Decision Category Taxonomy

| Category | Decision Points | Example |
|----------|-----------------|---------|
| TypeMapping | 15 | `i32` vs `i64` for int literal |
| BorrowStrategy | 12 | `&str` vs `String` for string param |
| LifetimeInfer | 8 | Elision vs explicit `'a` |
| MethodDispatch | 10 | `iter()` vs `into_iter()` |
| ImportResolve | 6 | `std::` vs external crate |
| ErrorHandling | 9 | `?` vs `unwrap()` vs `expect()` |
| Ownership | 11 | `clone()` vs move vs `Rc` |

## Appendix B: Example Trace Output

```json
{
  "id": 14823947283947,
  "timestamp_ns": 1732857600000000000,
  "thread_id": 1,
  "source_file": "rust_gen/expr_gen.rs",
  "source_line": 847,
  "category": "TypeMapping",
  "py_ast_hash": 9823749823,
  "chosen_path": "promote_lhs",
  "alternatives": ["cast_rhs", "error"],
  "confidence": 0.9,
  "py_span": [45, 52],
  "rs_span": [123, 138]
}
```

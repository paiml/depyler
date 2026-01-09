# Deterministic Scientific Corpus Report Specification

## Document Metadata

| Field | Value |
|-------|-------|
| **Specification ID** | DEPYLER-CORPUS-001 |
| **Version** | 1.0.0-draft |
| **Status** | AWAITING CODE REVIEW |
| **Author** | Depyler Team |
| **Created** | 2025-12-06 |
| **Toyota Way Principle** | 現地現物 (Genchi Genbutsu) - Go and See |

---

## 1. Executive Summary

This specification defines a **deterministic, reproducible scientific reporting system** for analyzing Python-to-Rust transpilation corpus quality. The system produces publication-grade reports that scientifically identify and classify compilation blockers using the PAIML Rust data science stack.

### Key Objectives

1. **Determinism**: Identical inputs produce byte-identical outputs
2. **Reproducibility**: Any researcher can replicate results given same corpus
3. **Scientific Rigor**: Peer-reviewed methodology with statistical validation
4. **Corpus Agnostic**: Works with any Python codebase (Netflix, AWS, Google, Weta Digital, etc.)
5. **Toyota Way Compliance**: Built-in quality at every stage (自働化 - Jidoka)

---

## 2. Scientific Foundation

### 2.1 Peer-Reviewed Citations

This specification is grounded in established research methodologies:

| # | Citation | Relevance | DOI/URL |
|---|----------|-----------|---------|
| 1 | Nagappan, N., Ball, T., & Zeller, A. (2006). "Mining metrics to predict component failures." *ICSE '06* | Defect prediction from code metrics | 10.1145/1134285.1134349 |
| 2 | Hassan, A. E. (2009). "Predicting faults using the complexity of code changes." *ICSE '09* | Change entropy as quality predictor | 10.1109/ICSE.2009.5070510 |
| 3 | Bird, C., et al. (2011). "Don't touch my code! Examining the effects of ownership on software quality." *FSE '11* | Code ownership and defect density | 10.1145/2025113.2025119 |
| 4 | Rahman, F., & Devanbu, P. (2013). "How, and why, process metrics are better." *ICSE '13* | Process vs. product metrics | 10.1109/ICSE.2013.6606589 |
| 5 | Ray, B., et al. (2014). "A large scale study of programming languages and code quality in GitHub." *FSE '14* | Language effects on defects | 10.1145/2635868.2635922 |
| 6 | Gousios, G., & Spinellis, D. (2012). "GHTorrent: GitHub's data from a firehose." *MSR '12* | Large-scale repository mining | 10.1109/MSR.2012.6224294 |
| 7 | Mockus, A., & Votta, L. G. (2000). "Identifying reasons for software changes using historic databases." *ICSM '00* | Root cause analysis methodology | 10.1109/ICSM.2000.883028 |
| 8 | Zimmermann, T., et al. (2007). "Predicting defects for Eclipse." *ICSE '07* | Cross-project defect prediction | 10.1109/ICSE.2007.56 |
| 9 | Shull, F., et al. (2002). "What we have learned about fighting defects." *IEEE Software* | Defect classification taxonomy | 10.1109/MS.2002.1049393 |
| 10 | Basili, V. R., et al. (1996). "A validation of object-oriented design metrics as quality indicators." *IEEE TSE* | Metric validation methodology | 10.1109/32.544352 |

### 2.2 Toyota Production System Alignment

| TPS Principle | Application in This System |
|---------------|---------------------------|
| **自働化 (Jidoka)** | Automatic detection of compilation failures with root cause classification |
| **現地現物 (Genchi Genbutsu)** | Direct analysis of actual compiler errors, not abstractions |
| **改善 (Kaizen)** | Iterative PDCA cycles to improve single-shot rate |
| **反省 (Hansei)** | Post-mortem analysis of blocker patterns |
| **Andon (アンドン)** | Real-time quality alerts when rate drops below threshold |
| **Poka-Yoke (ポカヨケ)** | Deterministic checksums prevent silent failures |

---

## 3. System Architecture

### 3.1 Component Stack

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    CORPUS SCIENTIFIC REPORT SYSTEM                       │
├─────────────────────────────────────────────────────────────────────────┤
│  Layer 5: Report Generation (Presentar + Trueno-Viz)                    │
│  ├── HTML5/PDF Publication Reports                                       │
│  ├── Interactive Dashboards (WASM)                                       │
│  ├── Confusion Matrices, ROC Curves, Heatmaps                           │
│  └── Terminal ASCII Reports (CI/CD friendly)                            │
├─────────────────────────────────────────────────────────────────────────┤
│  Layer 4: Statistical Analysis (Aprender + Entrenar)                    │
│  ├── Classification Metrics (precision, recall, F1)                     │
│  ├── Drift Detection (quality regression monitoring)                    │
│  ├── Feature Importance (SHAP, Integrated Gradients)                    │
│  └── Defect Prediction Models                                           │
├─────────────────────────────────────────────────────────────────────────┤
│  Layer 3: Metric Extraction (PMAT + Depyler)                            │
│  ├── TDG (Technical Debt Grading)                                       │
│  ├── Cyclomatic/Cognitive Complexity                                    │
│  ├── Error Classification (E0308, E0412, E0425, etc.)                   │
│  └── AST Analysis (Python + Rust)                                       │
├─────────────────────────────────────────────────────────────────────────┤
│  Layer 2: Orchestration (Batuta Oracle)                                 │
│  ├── Tool Selection & Configuration                                     │
│  ├── Dependency Resolution                                              │
│  ├── Execution DAG Management                                           │
│  └── Artifact Lineage Tracking                                          │
├─────────────────────────────────────────────────────────────────────────┤
│  Layer 1: Numerical Foundation (Trueno)                                 │
│  ├── SIMD-Accelerated Computations                                      │
│  ├── Statistical Functions (mean, variance, correlation)                │
│  └── Tensor Operations for Metric Aggregation                           │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Data Flow

```
                    ┌─────────────────┐
                    │  CORPUS INPUT   │
                    │  (Python Files) │
                    └────────┬────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────────────┐
│  PHASE 1: ARTIFACT CLEARING (Determinism Guarantee)                    │
│  ────────────────────────────────────────────────────────────────────  │
│  1. Remove all *.rs files in corpus                                    │
│  2. Remove all Cargo.toml files in corpus                              │
│  3. Remove all target/ directories                                     │
│  4. Verify clean state (file count = 0)                                │
│  5. Record corpus state hash (BLAKE3)                                  │
└────────────────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────────────┐
│  PHASE 2: TRANSPILATION (Cargo-First Strategy)                         │
│  ────────────────────────────────────────────────────────────────────  │
│  FOR each *.py file:                                                   │
│    1. depyler transpile <file.py> -o <file.rs>                        │
│    2. Generate Cargo.toml with dependencies                            │
│    3. Record transpilation metrics (time, warnings)                    │
│    4. Capture any transpilation errors                                 │
└────────────────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────────────┐
│  PHASE 3: COMPILATION VERIFICATION                                     │
│  ────────────────────────────────────────────────────────────────────  │
│  FOR each generated Cargo.toml directory:                              │
│    1. cargo build --release 2>&1                                       │
│    2. Capture exit code (0 = PASS, non-0 = FAIL)                      │
│    3. Parse compiler errors (rustc error codes)                        │
│    4. Extract error line numbers and context                           │
│    5. Classify error by taxonomy (see §4.2)                            │
└────────────────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────────────┐
│  PHASE 4: STATISTICAL ANALYSIS                                         │
│  ────────────────────────────────────────────────────────────────────  │
│  1. Compute single-shot compilation rate                               │
│  2. Classify errors by category (type, scope, inference)               │
│  3. Calculate error frequency distribution                             │
│  4. Compute correlation matrix (error type × code features)            │
│  5. Detect statistical outliers (Grubbs test)                          │
│  6. Calculate confidence intervals (95% CI)                            │
└────────────────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────────────┐
│  PHASE 5: REPORT GENERATION                                            │
│  ────────────────────────────────────────────────────────────────────  │
│  1. Generate JSON artifact (machine-readable)                          │
│  2. Generate Markdown report (human-readable)                          │
│  3. Generate visualizations (PNG/SVG)                                  │
│  4. Compute report checksum (BLAKE3)                                   │
│  5. Sign report with Ed25519 (optional)                                │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Error Taxonomy

### 4.1 Rust Compiler Error Classification

Based on analysis of reprorusted-python-cli and established defect taxonomies [9]:

| Category | Error Codes | Description | Root Cause |
|----------|-------------|-------------|------------|
| **TYPE_MISMATCH** | E0308 | Mismatched types | Type inference failure |
| **UNDEFINED_TYPE** | E0412 | Cannot find type | Generic parameter unresolved |
| **UNDEFINED_VALUE** | E0425 | Cannot find value | Missing import/binding |
| **TYPE_ANNOTATION** | E0282 | Type annotations needed | Insufficient type info |
| **TRAIT_BOUND** | E0277 | Trait not implemented | Missing trait impl |
| **BORROW_CHECK** | E0502, E0503, E0505 | Borrow checker errors | Ownership violation |
| **LIFETIME** | E0106, E0621 | Lifetime errors | Missing lifetime annotation |
| **SYNTAX** | E0061, E0433 | Syntax/parsing errors | Malformed code generation |

### 4.2 Blocker Priority Matrix

Using defect prediction methodology from [1] and [8]:

| Priority | Impact | Frequency Threshold | Action |
|----------|--------|---------------------|--------|
| **P0-CRITICAL** | Blocks >20% of corpus | ≥50 occurrences | Immediate PDCA cycle |
| **P1-HIGH** | Blocks 10-20% of corpus | 20-49 occurrences | Next sprint priority |
| **P2-MEDIUM** | Blocks 5-10% of corpus | 10-19 occurrences | Backlog item |
| **P3-LOW** | Blocks <5% of corpus | <10 occurrences | Monitor only |

---

## 5. Determinism Guarantees

### 5.1 Reproducibility Requirements

Per [6] and [7], all corpus analysis must satisfy:

```rust
/// Determinism invariant: f(corpus, config) → identical_output
///
/// Given:
///   - corpus_hash: BLAKE3 hash of all input Python files
///   - config_hash: BLAKE3 hash of tool configuration
///   - depyler_version: Semantic version string
///
/// Then:
///   report_hash = H(corpus_hash || config_hash || depyler_version)
///
/// For any two executions E1, E2:
///   if (E1.corpus_hash == E2.corpus_hash) &&
///      (E1.config_hash == E2.config_hash) &&
///      (E1.depyler_version == E2.depyler_version)
///   then:
///      E1.report_hash == E2.report_hash
```

### 5.2 Artifact Manifest

Every report execution produces:

```yaml
# corpus_report_manifest.yaml
version: "1.0.0"
execution:
  timestamp: "2025-12-06T12:00:00Z"  # ISO 8601
  machine_id: "sha256:abc123..."      # Anonymous machine fingerprint
  depyler_version: "3.22.0"
  rust_version: "1.83.0"

corpus:
  name: "reprorusted-python-cli"
  path: "../reprorusted-python-cli"
  python_files: 244
  source_hash: "blake3:def456..."     # Hash of all .py files

artifacts:
  - path: "report.json"
    hash: "blake3:ghi789..."
    format: "application/json"
  - path: "report.md"
    hash: "blake3:jkl012..."
    format: "text/markdown"
  - path: "error_heatmap.png"
    hash: "blake3:mno345..."
    format: "image/png"

checksums:
  algorithm: "BLAKE3"
  manifest_hash: "blake3:pqr678..."
```

### 5.3 Clean State Protocol

Before any corpus analysis (Toyota Way: 5S methodology):

```bash
#!/bin/bash
# clean_corpus.sh - Deterministic artifact clearing

CORPUS_PATH="${1:-../reprorusted-python-cli}"

echo "=== SEIRI (整理) - Sort: Identifying artifacts ==="
RS_COUNT=$(find "$CORPUS_PATH" -name "*.rs" -type f | wc -l)
CARGO_COUNT=$(find "$CORPUS_PATH" -name "Cargo.toml" -type f | wc -l)
TARGET_COUNT=$(find "$CORPUS_PATH" -name "target" -type d | wc -l)

echo "=== SEITON (整頓) - Set in Order: Recording state ==="
echo "RS files: $RS_COUNT"
echo "Cargo.toml files: $CARGO_COUNT"
echo "target/ directories: $TARGET_COUNT"

echo "=== SEISO (清掃) - Shine: Cleaning artifacts ==="
find "$CORPUS_PATH" -name "*.rs" -type f -delete
find "$CORPUS_PATH" -name "Cargo.toml" -type f -delete
find "$CORPUS_PATH" -name "Cargo.lock" -type f -delete
find "$CORPUS_PATH" -type d -name "target" -exec rm -rf {} + 2>/dev/null

echo "=== SEIKETSU (清潔) - Standardize: Verifying clean state ==="
VERIFY_RS=$(find "$CORPUS_PATH" -name "*.rs" -type f | wc -l)
VERIFY_CARGO=$(find "$CORPUS_PATH" -name "Cargo.toml" -type f | wc -l)

if [ "$VERIFY_RS" -eq 0 ] && [ "$VERIFY_CARGO" -eq 0 ]; then
    echo "✓ Clean state verified"
    exit 0
else
    echo "✗ Clean state verification FAILED"
    exit 1
fi
```

---

## 6. Report Schema

### 6.1 JSON Report Structure

```json
{
  "$schema": "https://depyler.dev/schemas/corpus-report-v1.json",
  "version": "1.0.0",
  "metadata": {
    "generated_at": "2025-12-06T12:00:00Z",
    "corpus_name": "reprorusted-python-cli",
    "corpus_hash": "blake3:...",
    "depyler_version": "3.22.0",
    "report_hash": "blake3:..."
  },
  "summary": {
    "total_python_files": 244,
    "transpilation": {
      "success": 244,
      "failure": 0,
      "rate": 100.0
    },
    "compilation": {
      "success": 84,
      "failure": 160,
      "rate": 34.4
    },
    "single_shot_rate": 34.4,
    "confidence_interval_95": [28.5, 40.3]
  },
  "error_distribution": {
    "by_category": {
      "TYPE_MISMATCH": { "count": 59, "percentage": 34.3 },
      "UNDEFINED_TYPE": { "count": 45, "percentage": 26.2 },
      "UNDEFINED_VALUE": { "count": 47, "percentage": 27.3 },
      "TYPE_ANNOTATION": { "count": 8, "percentage": 4.7 },
      "OTHER": { "count": 13, "percentage": 7.6 }
    },
    "by_error_code": [
      { "code": "E0308", "count": 59, "description": "mismatched types" },
      { "code": "E0412", "count": 45, "description": "cannot find type `T`" },
      { "code": "E0425", "count": 47, "description": "cannot find value" },
      { "code": "E0282", "count": 8, "description": "type annotations needed" }
    ]
  },
  "blocker_analysis": {
    "p0_critical": [],
    "p1_high": [
      {
        "error_code": "E0308",
        "count": 59,
        "root_cause": "Type inference failure in generic contexts",
        "recommended_fix": "DEPYLER-0745: Improve bidirectional type inference"
      },
      {
        "error_code": "E0412",
        "count": 45,
        "root_cause": "Generic type parameter T not resolved",
        "recommended_fix": "DEPYLER-0744: Infer concrete types from return statements"
      }
    ],
    "p2_medium": [],
    "p3_low": []
  },
  "statistical_analysis": {
    "mean_errors_per_file": 1.07,
    "std_deviation": 0.82,
    "median_errors": 1,
    "correlation_matrix": {
      "file_size_vs_errors": 0.23,
      "complexity_vs_errors": 0.41,
      "import_count_vs_errors": 0.67
    }
  },
  "toyota_way_metrics": {
    "jidoka_alerts": 2,
    "andon_triggers": 0,
    "kaizen_opportunities": 3,
    "hansei_items": ["Generic inference needs improvement"]
  }
}
```

### 6.2 Markdown Report Template

```markdown
# Corpus Analysis Report: {corpus_name}

**Generated**: {timestamp}
**Depyler Version**: {version}
**Report Hash**: `{report_hash}`

## Executive Summary

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Single-Shot Rate | {rate}% | 80% | {status_emoji} |
| 95% CI | [{ci_low}%, {ci_high}%] | - | - |
| P0 Blockers | {p0_count} | 0 | {p0_status} |
| P1 Blockers | {p1_count} | <3 | {p1_status} |

## Error Distribution

{error_heatmap_image}

### Top Blockers by Frequency

{blocker_table}

## Root Cause Analysis (Five Whys)

{five_whys_analysis}

## Recommended PDCA Cycles

{pdca_recommendations}

## Statistical Appendix

{statistical_tables}

---
*Report generated deterministically. Verify with: `blake3sum report.json`*
```

---

## 7. Corpus Configuration

### 7.1 Supported Corpora

| Corpus | Path | Description | Characteristics |
|--------|------|-------------|-----------------|
| **reprorusted-python-cli** | `../reprorusted-python-cli` | Default test corpus | CLI tools, diverse patterns |
| **Netflix** | Custom | Media streaming utilities | Async, networking |
| **AWS** | Custom | Cloud SDK utilities | boto3 patterns |
| **Google** | Custom | GCP SDK utilities | API clients |
| **Weta Digital** | Custom | VFX pipeline tools | NumPy-heavy |

### 7.2 Corpus Configuration File

```yaml
# corpus_config.yaml
corpus:
  name: "reprorusted-python-cli"
  path: "../reprorusted-python-cli"

  # File selection
  include_patterns:
    - "examples/**/*.py"
  exclude_patterns:
    - "**/__pycache__/**"
    - "**/test_*.py"
    - "**/__init__.py"

  # Analysis settings
  analysis:
    max_file_size_kb: 100
    timeout_per_file_sec: 30
    parallel_workers: 4

  # Quality thresholds
  thresholds:
    single_shot_rate_target: 80.0
    p0_blocker_max: 0
    p1_blocker_max: 3

  # Output settings
  output:
    directory: "./reports"
    formats: ["json", "markdown", "png"]
    include_raw_errors: true
```

---

## 8. Implementation Phases

### Phase 1: Core Infrastructure (Week 1)

```rust
// depyler-corpus/src/lib.rs
pub mod cleaner;      // Artifact clearing (5S methodology)
pub mod transpiler;   // Batch transpilation runner
pub mod compiler;     // Cargo build verification
pub mod parser;       // Error parsing and classification
```

### Phase 2: Analysis Engine (Week 2)

```rust
// depyler-corpus/src/analysis/mod.rs
pub mod taxonomy;     // Error classification
pub mod statistics;   // Statistical analysis (via aprender)
pub mod correlation;  // Feature correlation (via trueno)
pub mod prediction;   // Defect prediction models
```

### Phase 3: Report Generation (Week 3)

```rust
// depyler-corpus/src/report/mod.rs
pub mod json;         // JSON report generation
pub mod markdown;     // Markdown report generation
pub mod visualization;// Charts (via trueno-viz)
pub mod manifest;     // Determinism manifest
```

### Phase 4: Integration & Testing (Week 4)

```rust
// depyler-corpus/src/cli.rs
/// CLI interface for corpus analysis
#[derive(Parser)]
pub struct CorpusAnalyze {
    /// Path to corpus (default: ../reprorusted-python-cli)
    #[arg(short, long, default_value = "../reprorusted-python-cli")]
    corpus: PathBuf,

    /// Output directory for reports
    #[arg(short, long, default_value = "./reports")]
    output: PathBuf,

    /// Output formats
    #[arg(short, long, default_values = ["json", "markdown"])]
    formats: Vec<String>,

    /// Skip artifact clearing (use existing transpilation)
    #[arg(long)]
    no_clean: bool,
}
```

---

## 9. Verification Protocol

### 9.1 Reproducibility Test

```bash
#!/bin/bash
# verify_determinism.sh

# Run analysis twice
./depyler corpus-analyze --corpus ../reprorusted-python-cli --output /tmp/run1
./depyler corpus-analyze --corpus ../reprorusted-python-cli --output /tmp/run2

# Compare report hashes
HASH1=$(blake3sum /tmp/run1/report.json | cut -d' ' -f1)
HASH2=$(blake3sum /tmp/run2/report.json | cut -d' ' -f1)

if [ "$HASH1" == "$HASH2" ]; then
    echo "✓ DETERMINISM VERIFIED: $HASH1"
    exit 0
else
    echo "✗ DETERMINISM FAILURE"
    echo "  Run 1: $HASH1"
    echo "  Run 2: $HASH2"
    diff /tmp/run1/report.json /tmp/run2/report.json
    exit 1
fi
```

### 9.2 Statistical Validation

Per [10], validate metrics using:

1. **Internal Consistency**: Cronbach's alpha > 0.7
2. **Construct Validity**: Factor analysis confirms expected structure
3. **Predictive Validity**: Error categories predict compilation outcome

---

## 10. Quality Gates

### 10.1 Pre-Implementation Checklist

- [ ] Peer review of this specification
- [ ] Citations verified and accessible
- [ ] Tool dependencies confirmed (batuta, aprender, etc.)
- [ ] Test corpus available and clean
- [ ] CI/CD pipeline configured

### 10.2 Post-Implementation Validation

- [ ] Determinism test passes (§9.1)
- [ ] Statistical validation passes (§9.2)
- [ ] Report schema validates against JSON schema
- [ ] Markdown renders correctly in GitHub
- [ ] Visualizations are publication quality

---

## 11. References

[1] N. Nagappan, T. Ball, and A. Zeller, "Mining metrics to predict component failures," in *Proc. ICSE*, 2006.

[2] A. E. Hassan, "Predicting faults using the complexity of code changes," in *Proc. ICSE*, 2009.

[3] C. Bird et al., "Don't touch my code! Examining the effects of ownership on software quality," in *Proc. FSE*, 2011.

[4] F. Rahman and P. Devanbu, "How, and why, process metrics are better," in *Proc. ICSE*, 2013.

[5] B. Ray et al., "A large scale study of programming languages and code quality in GitHub," in *Proc. FSE*, 2014.

[6] G. Gousios and D. Spinellis, "GHTorrent: GitHub's data from a firehose," in *Proc. MSR*, 2012.

[7] A. Mockus and L. G. Votta, "Identifying reasons for software changes using historic databases," in *Proc. ICSM*, 2000.

[8] T. Zimmermann et al., "Predicting defects for Eclipse," in *Proc. ICSE*, 2007.

[9] F. Shull et al., "What we have learned about fighting defects," *IEEE Software*, 2002.

[10] V. R. Basili et al., "A validation of object-oriented design metrics as quality indicators," *IEEE TSE*, 1996.

---

## Appendix A: Tool Integration Matrix

| Tool | Purpose | Input | Output | Integration Point |
|------|---------|-------|--------|-------------------|
| **Depyler** | Transpilation | .py files | .rs + Cargo.toml | Phase 2 |
| **PMAT** | Code metrics | .rs files | JSON metrics | Phase 3 |
| **Aprender** | Statistics | Metric arrays | Analysis results | Phase 4 |
| **Trueno** | Numerics | Tensors | Computed values | Phase 4 |
| **Trueno-Viz** | Visualization | Data points | PNG/SVG | Phase 5 |
| **Presentar** | Dashboards | Report data | HTML/WASM | Phase 5 |
| **Batuta** | Orchestration | Config | Execution DAG | All phases |

---

## Appendix B: Error Code Reference

| Code | Category | Description | Common Fix |
|------|----------|-------------|------------|
| E0282 | TYPE_ANNOTATION | Type annotations needed | Add explicit type |
| E0308 | TYPE_MISMATCH | Mismatched types | Fix type inference |
| E0412 | UNDEFINED_TYPE | Cannot find type | Resolve generic |
| E0425 | UNDEFINED_VALUE | Cannot find value | Add import/binding |
| E0277 | TRAIT_BOUND | Trait not implemented | Add trait impl |
| E0502 | BORROW_CHECK | Cannot borrow | Fix ownership |
| E0106 | LIFETIME | Missing lifetime | Add lifetime param |

---

**END OF SPECIFICATION**

*Awaiting code review before implementation.*

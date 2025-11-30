# Depyler Oracle Training Reports

**Version:** 1.1.0
**Status:** Draft
**Author:** Depyler Team
**Date:** 2025-11-30
**PMAT Work Item:** DEPYLER-0601

> **NOTE:** This document describes depyler-specific usage of the general-purpose
> experiment tracking system defined in [entrenar's experiment-tracking-spec](https://github.com/paiml/entrenar/blob/main/docs/specifications/experiment-tracking-spec.md).

## Abstract

This specification defines how the Depyler oracle training system uses entrenar's experiment tracking framework. It covers depyler-specific pre-flight checks, metrics, and artifacts while inheriting the general Toyota Way principles from the parent specification.

## 1. Introduction

### 1.1 Problem Statement

Ad-hoc training pipelines without structured reporting lead to:

1.  **Wasted compute** - Bugs in data pipelines (e.g., deduplication failures) go undetected for hours [1].
2.  **Irreproducibility** - Cannot recreate successful training runs [2].
3.  **No systematic improvement** - No baseline comparison for hyperparameter tuning [3].
4.  **Audit failures** - Cannot demonstrate model provenance for compliance [4].
5.  **Hidden Environmental Cost** - Untracked carbon footprint of training cycles [11].

### 1.2 Solution

A YAML-based training report format that captures:

-   **Pre-flight validation** (corpus integrity, environment checks) - *Jidoka*
-   **Training configuration** (hyperparameters, seeds, data sources) - *Standardization*
-   **Runtime metrics** (duration, resource usage, convergence, energy) - *Visual Control*
-   **Post-training evaluation** (accuracy, confusion matrix, drift detection) - *Kaizen*
-   **Artifacts** (model files, checksums, lineage) - *Genchi Genbutsu*

### 1.3 Design Principles: The Toyota Way

This specification embodies the following lean manufacturing principles applied to ML Engineering:

| Principle | Application in Spec | Rationale | Citation |
| :--- | :--- | :--- | :--- |
| **Jidoka** (Built-in Quality) | `preflight` checks must pass or training aborts. | Stop the line immediately when a defect (e.g., duplicates) is detected. | [1] |
| **Standardization** | Strict JSON Schema validation for all reports. | Stable processes are the foundation for continuous improvement. | [6] |
| **Visual Control** | Metrics exposed to Grafana/Prometheus. | Make no problems hidden; deviations should be immediately visible. | [3] |
| **Genchi Genbutsu** | Full `lineage` and `artifacts` linking. | Go and see the actual data and code that produced the result. | [4] |
| **Kaizen** (Continuous Improvement) | `drift` detection and `cycle` tracking. | Measure against baselines to ensure each cycle is better than the last. | [10] |
| **Reproducibility** | Cryptographic hashes and seeds. | Same config → same model (within floating-point tolerance). | [2] |
| **Green AI** | Energy consumption tracking. | Account for and minimize environmental impact of compute. | [11] |

## 2. Report Schema

### 2.1 File Location

```
docs/training-reports/
├── 2025/
│   └── 11/
│       ├── 2025-11-30T070312Z_cycle-187.yaml
│       ├── 2025-11-30T071523Z_cycle-188.yaml
│       └── ...
└── schema/
    └── training-report-v1.schema.json
```

**Naming convention:** `{ISO8601-timestamp}_cycle-{N}.yaml`

### 2.2 Full Schema

```yaml
# Training Run Report v1.1
# JSON Schema: docs/training-reports/schema/training-report-v1.schema.json

report_version: "1.1"
generated_at: "2025-11-30T07:03:12Z"
generated_by: "extract-training-data v3.21.0"

# ============================================================ 
# SECTION 1: PRE-FLIGHT VALIDATION (oracle-validate)
# Principle: Jidoka (Stop the line on error)
# ============================================================ 
preflight:
  status: "PASS"  # PASS | FAIL | SKIP
  started_at: "2025-11-30T07:00:00Z"
  completed_at: "2025-11-30T07:00:05Z"
  duration_seconds: 5

  checks:
    corpus_integrity: # [1] Data Quality
      status: "PASS"
      total_lines: 422
      unique_hashes: 422
      duplicate_ratio: 0.0
      message: "All entries have unique hashes"

    environment: # [2] Reproducibility
      status: "PASS"
      rust_version: "1.83.0"
      cargo_version: "1.83.0"
      depyler_version: "3.21.0"
      aprender_version: "0.5.0"

    dependencies: # [5] Technical Debt Prevention
      status: "PASS"
      depyler_binary: "/home/noah/src/depyler/target/release/depyler"
      depyler_sha256: "a1b2c3d4..."
      extract_binary: "/home/noah/src/depyler/target/release/extract-training-data"
      extract_sha256: "e5f6g7h8..."

    disk_space:
      status: "PASS"
      available_gb: 45.2
      required_gb: 1.0

    gpu:
      status: "SKIP"
      message: "GPU training not enabled"

  # If any check fails, training MUST NOT proceed
  blocking_failures: []

# ============================================================ 
# SECTION 2: DATA SOURCES AND LINEAGE
# Principle: Genchi Genbutsu (Traceability)
# ============================================================ 
data:
  corpus_file: "training_corpus/errors.jsonl"
  corpus_sha256: "abc123def456..." # [4] Provenance
  corpus_stats:
    total_samples: 422
    unique_samples: 422

  sources:
    - name: "reprorusted-python-cli"
      type: "transpilation_errors"
      path: "/home/noah/src/reprorusted-python-cli/examples"
      commit: "a1b2c3d4"
      files_processed: 500
      errors_extracted: 312

    - name: "verificar_synthetic"
      type: "synthetic"
      generator: "verificar v0.3"
      seed: 187042
      samples: 110

  class_distribution:
    TypeMismatch: 156
    BorrowChecker: 89
    MissingImport: 78
    SyntaxError: 45
    LifetimeError: 32
    TraitBound: 15
    Other: 7

  # Data quality metrics per Amershi et al. [6]
  quality_metrics:
    label_noise_estimate: 0.02  # Estimated mislabeling rate
    class_imbalance_ratio: 22.3  # max_class / min_class
    feature_coverage: 0.94  # % of vocabulary seen in training

# ============================================================ 
# SECTION 3: TRAINING CONFIGURATION
# Principle: Standardization
# ============================================================ 
training:
  cycle: 187
  mode: "accumulate"  # fresh | accumulate

  hyperparameters: # [3] Experiment Tracking
    algorithm: "RandomForest"
    n_estimators: 100
    max_depth: 10
    min_samples_split: 2
    min_samples_leaf: 1
    max_features: "sqrt"
    bootstrap: true

  reproducibility: # [2] Pineau et al.
    random_seed: 187042
    deterministic: true
    seed_derivation: "42 + cycle * 1000 + timestamp % 1000"

  preprocessing:
    vectorizer: "TF-IDF"
    max_features: 5000
    ngram_range: [1, 2]
    stop_words: "english"

  balancing:
    enabled: true
    strategy: "cap_per_class"
    max_per_class: 2000

# ============================================================ 
# SECTION 4: RUNTIME & ENERGY METRICS
# Principle: Visual Control / Green AI
# ============================================================ 
runtime:
  started_at: "2025-11-30T07:00:05Z"
  completed_at: "2025-11-30T07:03:12Z"
  duration_seconds: 187

  phases:
    extraction:
      duration_seconds: 45
      files_processed: 500
      errors_extracted: 312
    preprocessing:
      duration_seconds: 12
      vocabulary_size: 4823
    training:
      duration_seconds: 98
      trees_built: 100
    evaluation:
      duration_seconds: 15
      test_samples: 845
    serialization:
      duration_seconds: 17
      model_size_bytes: 514501

  resources:
    peak_memory_mb: 1245
    cpu_percent_avg: 78.5
    threads_used: 8

  energy: # [11] Strubell et al.
    estimated_joules: 4500
    co2_eq_grams: 0.5
    provider: "local_workstation"
    efficiency_index: 24.1 # Joules per sample

# ============================================================ 
# SECTION 5: EVALUATION METRICS
# Principle: Kaizen (Measure to Improve)
# ============================================================ 
evaluation:
  # Primary metrics per Japkowicz & Shah [7]
  accuracy: 0.755
  precision_macro: 0.72
  recall_macro: 0.68
  f1_macro: 0.70

  # Per-class metrics
  per_class:
    TypeMismatch:
      precision: 0.82
      recall: 0.79
      f1: 0.80
      support: 156
    # ... etc

  # Confusion matrix (row=actual, col=predicted)
  confusion_matrix:
    labels: ["TypeMismatch", "BorrowChecker", "MissingImport", "SyntaxError", "LifetimeError", "TraitBound", "Other"]
    matrix:
      - [123, 12, 8, 5, 3, 2, 3]   # TypeMismatch
      # ... etc

  # Cross-validation results per Kohavi [8]
  cross_validation:
    method: "stratified_k_fold"
    k: 5
    scores: [0.74, 0.76, 0.75, 0.77, 0.75]
    mean: 0.754
    std: 0.011

  # Model calibration per Niculescu-Mizil & Caruana [9]
  calibration:
    brier_score: 0.18
    expected_calibration_error: 0.05

# ============================================================ 
# SECTION 6: DRIFT DETECTION
# Principle: Kaizen (Detect Deviation)
# ============================================================ 
drift:
  # Concept drift detection per Gama et al. [10]
  baseline_accuracy: 0.769  # From previous best model
  current_accuracy: 0.755
  delta: -0.014

  status: "STABLE"  # STABLE | WARNING | DRIFT
  threshold_warning: 0.03
  threshold_drift: 0.05

  # Statistical test
  statistical_test:
    method: "McNemar"
    statistic: 2.34
    p_value: 0.126
    significant: false

  # Feature drift (distribution shift in input data)
  feature_drift:
    method: "PSI"  # Population Stability Index
    psi_score: 0.08
    status: "STABLE"  # PSI < 0.1 = stable, 0.1-0.25 = moderate, > 0.25 = significant

# ============================================================ 
# SECTION 7: MODEL CARD INFO
# Standard: Mitchell et al. [12]
# ============================================================ 
model_card:
  intended_use: "Classifying Python-to-Rust transpilation errors for auto-correction hints."
  limitations: "Trained primarily on synthetic data; may underperform on complex generic/trait errors."
  ethical_considerations: "None identified; model processes code snippets only."
  training_data_license: "MIT"

# ============================================================ 
# SECTION 8: ARTIFACTS
# ============================================================ 
artifacts:
  model:
    path: "depyler_oracle.apr"
    format: "aprender_v1"
    size_bytes: 514501
    sha256: "789abc012def..."
    compression: "zstd"

  corpus:
    path: "training_corpus/errors.jsonl"
    size_bytes: 51721
    sha256: "abc123def456..."

  report:
    path: "docs/training-reports/2025/11/2025-11-30T070312Z_cycle-187.yaml"

  logs:
    training_log: "training_corpus/logs/overnight_20251130.log"
    extraction_log: "training_corpus/logs/extraction_20251130.log"

# ============================================================ 
# SECTION 9: LINEAGE AND PROVENANCE
# ============================================================ 
lineage:
  parent_model:
    cycle: 186
    report: "docs/training-reports/2025/11/2025-11-30T065512Z_cycle-186.yaml"
    accuracy: 0.755

  code_version:
    repository: "https://github.com/paiml/depyler"
    commit: "46bfb1c"
    branch: "main"
    dirty: false

  toolchain:
    rust: "1.83.0"
    cargo: "1.83.0"
    depyler: "3.21.0"
    aprender: "0.5.0"
    extract_training_data: "3.21.0"
```

## 3. Validation Rules (oracle-validate)

### 3.1 Pre-flight Checks

The `make oracle-validate` target MUST pass before training proceeds. This implements **Jidoka**: automated quality checks that stop the process.

```rust
pub struct PreflightValidator {
    checks: Vec<Box<dyn PreflightCheck>>,
}

pub trait PreflightCheck {
    fn name(&self) -> &str;
    fn run(&self) -> PreflightResult;
    fn is_blocking(&self) -> bool;
}
```

### 3.2 Required Checks

| Check | Blocking | Validation |
|-------|----------|------------|
| `corpus_integrity` | Yes | `unique_hashes == total_lines` |
| `corpus_minimum` | Yes | `total_lines >= 50` |
| `class_coverage` | Yes | All 7 classes present |
| `environment` | Yes | Rust/Cargo versions match |
| `disk_space` | Yes | `available_gb >= 1.0` |
| `binary_checksums` | No | Binaries match expected SHA256 |
| `gpu_available` | No | GPU present if gpu feature enabled |

## 4. Integration with PMAT

### 4.1 Work Item Linking

Each training run links to PMAT work items:

```yaml
pmat:
  work_item: "DEPYLER-0601"
  sprint: "2025-W48"
  epic: "Oracle ML Training"

  metrics_contribution:
    - metric: "model_accuracy"
      value: 0.755
      unit: "ratio"
    - metric: "training_duration"
      value: 187
      unit: "seconds"
    - metric: "corpus_size"
      value: 422
      unit: "samples"
```

## 5. Makefile Integration

```makefile
.PHONY: oracle-validate
oracle-validate: ## Validate corpus integrity before training
	@echo "🔍 Running pre-flight validation..."
	@cargo run --release -p depyler-oracle --bin oracle-validate -- \
		--corpus training_corpus/errors.jsonl \
		--output docs/training-reports/preflight-$(shell date +%Y%m%dT%H%M%S).yaml
	@echo "✅ Pre-flight validation passed"

.PHONY: oracle-cycle-scientific
oracle-cycle-scientific: oracle-validate ## Full scientific training cycle with reporting
	@CYCLE=$$(cat training_corpus/.cycle_count 2>/dev/null || echo 0); \
	CYCLE=$$((CYCLE + 1)); \
	echo $$CYCLE > training_corpus/.cycle_count; \
	TIMESTAMP=$$(date -u +%Y-%m-%dT%H%M%SZ); \
	REPORT="docs/training-reports/$$(date +%Y)/$$(date +%m)/$${TIMESTAMP}_cycle-$${CYCLE}.yaml"; \
	mkdir -p $$(dirname $$REPORT); \
	cargo run --release -p depyler-oracle --bin oracle-train -- \
		--corpus training_corpus/errors.jsonl \
		--output depyler_oracle.apr \
		--report $$REPORT \
		--cycle $$CYCLE; \
	echo "📊 Report: $$REPORT"
```

## 6. Analysis Tooling

### 6.1 Report Queries

```bash
# Find all runs with accuracy > 0.80
yq '.evaluation.accuracy > 0.80' docs/training-reports/**/*.yaml

# Plot accuracy over time
oracle-analyze trend --metric accuracy --output accuracy_trend.png
```

### 6.2 Dashboard Integration

Reports feed into Grafana/Prometheus:

```yaml
# prometheus.yml scrape config
- job_name: 'depyler-oracle'
  static_configs:
    - targets: ['localhost:9090']
  metrics_path: '/metrics'
```

## 7. References

[1] Polyzotis, N., Roy, S., Whang, S. E., & Zinkevich, M. (2019). **Data lifecycle challenges in production machine learning: A survey.** ACM SIGMOD Record, 47(2), 17-28. https://doi.org/10.1145/3299887.3299891

[2] Pineau, J., Vincent-Lamarre, P., Sinha, K., et al. (2021). **Improving reproducibility in machine learning research: A report from the NeurIPS 2019 reproducibility program.** Journal of Machine Learning Research, 22(164), 1-20. https://jmlr.org/papers/v22/20-303.html

[3] Zaharia, M., Chen, A., Davidson, A., et al. (2018). **Accelerating the machine learning lifecycle with MLflow.** IEEE Data Engineering Bulletin, 41(4), 39-45.

[4] Vartak, M., Subramanyam, H., Lee, W. E., et al. (2016). **ModelDB: A system for machine learning model management.** Proceedings of the Workshop on Human-In-the-Loop Data Analytics, 1-3. https://doi.org/10.1145/2939502.2939516

[5] Sculley, D., Holt, G., Golovin, D., et al. (2015). **Hidden technical debt in machine learning systems.** Advances in Neural Information Processing Systems, 28, 2503-2511.

[6] Amershi, S., Begel, A., Bird, C., et al. (2019). **Software engineering for machine learning: A case study.** IEEE/ACM 41st International Conference on Software Engineering: Software Engineering in Practice (ICSE-SEIP), 291-300. https://doi.org/10.1109/ICSE-SEIP.2019.00042

[7] Japkowicz, N., & Shah, M. (2011). **Evaluating learning algorithms: A classification perspective.** Cambridge University Press. ISBN: 978-0521196000

[8] Kohavi, R. (1995). **A study of cross-validation and bootstrap for accuracy estimation and model selection.** Proceedings of the 14th International Joint Conference on Artificial Intelligence (IJCAI), 2, 1137-1143.

[9] Niculescu-Mizil, A., & Caruana, R. (2005). **Predicting good probabilities with supervised learning.** Proceedings of the 22nd International Conference on Machine Learning (ICML), 625-632. https://doi.org/10.1145/1102351.1102430

[10] Gama, J., Žliobaitė, I., Bifet, A., Pechenizkiy, M., & Bouchachia, A. (2014). **A survey on concept drift adaptation.** ACM Computing Surveys, 46(4), 1-37. https://doi.org/10.1145/2523813

[11] Strubell, E., Ganesh, A., & McCallum, A. (2019). **Energy and Policy Considerations for Deep Learning in NLP.** Proceedings of the 57th Annual Meeting of the Association for Computational Linguistics, 3645-3650.

[12] Mitchell, M., Wu, S., Zaldivar, A., et al. (2019). **Model Cards for Model Reporting.** Proceedings of the Conference on Fairness, Accountability, and Transparency, 220-229.

## 8. Appendix A: JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/paiml/depyler/training-report-v1.schema.json",
  "title": "Depyler Training Report",
  "type": "object",
  "required": ["report_version", "generated_at", "preflight", "data", "training", "evaluation", "artifacts", "energy", "model_card"],
  "properties": {
    "report_version": { "type": "string", "pattern": "^\\d+\\.\\d+$" },
    "generated_at": { "type": "string", "format": "date-time" },
    "preflight": {
      "type": "object",
      "required": ["status", "checks"],
      "properties": {
        "status": { "enum": ["PASS", "FAIL", "SKIP"] },
        "checks": { "type": "object" }
      }
    },
    "evaluation": {
      "type": "object",
      "required": ["accuracy"],
      "properties": {
        "accuracy": { "type": "number", "minimum": 0, "maximum": 1 }
      }
    }
  }
}
```

## 9. Appendix B: Migration from Ad-hoc Scripts

| Before (Ad-hoc) | After (Scientific) |
|-----------------|-------------------|
| `sort -u` dedup | `HashSet<String>` with validation |
| No pre-flight | `oracle-validate` required (Jidoka) |
| Console output only | YAML report + metrics (Visual Control) |
| Manual monitoring | Automated drift detection (Kaizen) |
| No lineage | Full provenance chain (Genchi Genbutsu) |
| Unknown Impact | Energy and CO2 tracking (Green AI) |
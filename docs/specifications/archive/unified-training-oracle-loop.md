# Unified Training Oracle Loop (UTOL) Specification

**Version**: 1.0.0
**Status**: Draft
**Authors**: Depyler Team
**Toyota Way Principles**: Jidoka, Kaizen, Genchi Genbutsu, Heijunka

---

## Executive Summary

The Unified Training Oracle Loop (UTOL) replaces manual "Apex Hunt" prompt-driven cycles with an automated, self-correcting compilation feedback system. UTOL embodies **Jidoka** (autonomation with human touch) - the system detects problems, stops the line, and self-corrects while providing rich visual feedback to operators.

**Core Value Proposition**: Transform ad-hoc debugging into a **deterministic convergence machine** that achieves 80%+ single-shot compilation rate through continuous learning.

---

## 1. Toyota Way Foundation

### 1.1 Guiding Principles

| Principle | Japanese | Application in UTOL |
|-----------|----------|---------------------|
| **Jidoka** | 自働化 | Auto-stop on compilation failure, self-diagnose, self-repair |
| **Kaizen** | 改善 | Each iteration improves the model incrementally |
| **Genchi Genbutsu** | 現地現物 | Observe actual compilation errors, not abstractions |
| **Heijunka** | 平準化 | Level the training load across error categories |
| **Andon** | 行灯 | Visual feedback system for loop status |
| **Poka-Yoke** | ポカヨケ | Error-proofing through deterministic seeds |
| **Hansei** | 反省 | Post-loop reflection and regression detection |

### 1.2 Quality Built-In (品質は工程で作り込む)

UTOL does not bolt quality onto output - it builds quality into the process:

```
┌─────────────────────────────────────────────────────────────────┐
│                    UTOL Quality Pipeline                        │
├─────────────────────────────────────────────────────────────────┤
│  Corpus → Features → Train → Evaluate → Deploy → Observe → ↺   │
│     ↑        ↑         ↑        ↑          ↑         ↑         │
│   Poka-    Jidoka   Kaizen   Hansei    Genchi    Andon        │
│   Yoke                                 Genbutsu               │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. System Architecture

### 2.1 Component Overview

```
┌──────────────────────────────────────────────────────────────────────┐
│                     UNIFIED TRAINING ORACLE LOOP                      │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │
│  │   Corpus    │───▶│  Entrenar   │───▶│  Aprender   │              │
│  │   Manager   │    │  Trainer    │    │   Oracle    │              │
│  └─────────────┘    └─────────────┘    └─────────────┘              │
│         │                  │                  │                      │
│         ▼                  ▼                  ▼                      │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │
│  │   Depyler   │◀───│   ADWIN     │◀───│  Lineage    │              │
│  │  Compiler   │    │   Drift     │    │  Tracker    │              │
│  └─────────────┘    └─────────────┘    └─────────────┘              │
│         │                  │                  │                      │
│         └──────────────────┴──────────────────┘                      │
│                            │                                         │
│                            ▼                                         │
│                   ┌─────────────────┐                               │
│                   │   Andon TUI     │                               │
│                   │  Visual Status  │                               │
│                   └─────────────────┘                               │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

### 2.2 Data Flow

```
                    PDCA Cycle (Deming Wheel)

       PLAN                              DO
    ┌─────────┐                    ┌─────────┐
    │ Assess  │                    │ Train   │
    │ Corpus  │─────────────────▶ │ Model   │
    │ State   │                    │         │
    └─────────┘                    └─────────┘
         ▲                              │
         │                              ▼
    ┌─────────┐                    ┌─────────┐
    │ Update  │                    │ Compile │
    │ Corpus  │◀────────────────── │ Corpus  │
    │         │                    │         │
    └─────────┘                    └─────────┘
       ACT                             CHECK
```

---

## 3. Visual Feedback System (Andon)

### 3.1 Loop Status Display

```
╔══════════════════════════════════════════════════════════════════════╗
║                    UTOL - Unified Training Oracle Loop               ║
╠══════════════════════════════════════════════════════════════════════╣
║                                                                      ║
║  Iteration: [████████████░░░░░░░░] 12/20 (60%)                      ║
║  Estimated Convergence: 83.2% → Target: 80.0%  ✓ ON TRACK           ║
║                                                                      ║
║  ┌─────────────────────────────────────────────────────────────┐    ║
║  │ Model Status                                                 │    ║
║  ├─────────────────────────────────────────────────────────────┤    ║
║  │ Last Trained:    2025-12-08 20:22:15 UTC (3 min ago)        │    ║
║  │ Model Size:      503 KB (zstd compressed)                    │    ║
║  │ Model Version:   oracle-3.21.0-1733688135                   │    ║
║  │ Training Samples: 12,847                                     │    ║
║  └─────────────────────────────────────────────────────────────┘    ║
║                                                                      ║
║  ┌─────────────────────────────────────────────────────────────┐    ║
║  │ Evaluation Metrics                                           │    ║
║  ├─────────────────────────────────────────────────────────────┤    ║
║  │ Accuracy:     ▁▂▃▄▅▆▇█ 85.3% (+2.1%)                        │    ║
║  │ F1-Score:     ▁▂▃▄▅▆▇░ 78.9% (+1.4%)                        │    ║
║  │ Compile Rate: ▁▂▃▄▅▆▇█ 83.2% (+3.7%)                        │    ║
║  │ Drift Status: ● STABLE                                       │    ║
║  └─────────────────────────────────────────────────────────────┘    ║
║                                                                      ║
║  ┌─────────────────────────────────────────────────────────────┐    ║
║  │ Category Breakdown                                           │    ║
║  ├─────────────────────────────────────────────────────────────┤    ║
║  │ TypeMismatch:  ████████████████░░░░ 82%  (target: 80%)  ✓   │    ║
║  │ TraitBound:    █████████████░░░░░░░ 71%  (target: 80%)  ⚠   │    ║
║  │ Import:        ██████████████████░░ 91%  (target: 80%)  ✓   │    ║
║  │ Scope:         ███████████████░░░░░ 78%  (target: 80%)  ⚠   │    ║
║  │ Borrowing:     ████████████████░░░░ 84%  (target: 80%)  ✓   │    ║
║  │ Lifetime:      ████████████░░░░░░░░ 65%  (target: 80%)  ✗   │    ║
║  │ Syntax:        █████████████████░░░ 88%  (target: 80%)  ✓   │    ║
║  └─────────────────────────────────────────────────────────────┘    ║
║                                                                      ║
║  Current Action: Compiling example_subprocess/task_runner.py        ║
║  Last Error:     E0308 type mismatch (fixed in iteration 11)        ║
║                                                                      ║
║  [Space] Pause  [Q] Quit  [R] Force Retrain  [D] Details            ║
╚══════════════════════════════════════════════════════════════════════╝
```

### 3.2 Sparkline Metrics (Entrenar TUI)

```rust
// From entrenar::train::tui
pub const SPARK_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

// Usage in UTOL
let accuracy_history = [0.72, 0.75, 0.78, 0.81, 0.83, 0.85];
let sparkline = sparkline(&accuracy_history, 8);  // "▁▂▃▅▆█"
```

### 3.3 Progress Bar with Kalman-Filtered ETA

```rust
// From entrenar::train::tui::KalmanEta
pub struct LoopProgress {
    current_iteration: usize,
    max_iterations: usize,
    eta: KalmanEta,           // Smooth ETA estimation
    convergence_estimate: f64, // Predicted final accuracy
}

impl LoopProgress {
    pub fn display(&self) -> String {
        let pct = self.current_iteration as f64 / self.max_iterations as f64;
        let filled = (pct * 20.0) as usize;
        let bar = "█".repeat(filled) + &"░".repeat(20 - filled);

        format!(
            "Iteration: [{}] {}/{} ({:.0}%)  ETA: {}",
            bar,
            self.current_iteration,
            self.max_iterations,
            pct * 100.0,
            self.eta.format_remaining()
        )
    }
}
```

### 3.4 Andon Alert Levels

| Level | Symbol | Color | Meaning | Action |
|-------|--------|-------|---------|--------|
| **Stable** | `●` | Green | On track | Continue |
| **Warning** | `◐` | Yellow | Approaching threshold | Monitor |
| **Critical** | `○` | Red | Below threshold | Stop & Diagnose |
| **Drift** | `⚡` | Orange | Model degradation | Retrain |

---

## 4. Configuration System

### 4.1 Default Configuration (Zero-Config)

```yaml
# .depyler/utol.yaml (auto-generated if missing)
utol:
  version: "1.0"

  # Corpus settings (defaults to reprorusted-python-cli)
  corpus:
    path: "../reprorusted-python-cli"
    include_patterns:
      - "**/*.py"
    exclude_patterns:
      - "**/test_*.py"
      - "**/__pycache__/**"

  # Training settings
  training:
    synthetic_samples: 12000
    seed: 42
    balance_classes: true

  # Convergence settings
  convergence:
    target_rate: 0.80          # 80% single-shot compilation
    max_iterations: 50
    patience: 5                 # Stop if no improvement for N iterations
    min_delta: 0.005           # Minimum improvement threshold

  # Model settings
  model:
    path: "depyler_oracle.apr"
    n_estimators: 100
    max_depth: 10

  # Visual feedback
  display:
    mode: "rich"               # rich | minimal | json | silent
    refresh_ms: 500
    show_sparklines: true
    show_category_breakdown: true
```

### 4.2 Custom Configuration (Enterprise)

```yaml
# Example: Custom corpus for enterprise deployment
utol:
  version: "1.0"

  corpus:
    path: "/opt/company/python-monorepo"
    include_patterns:
      - "services/**/*.py"
      - "libs/**/*.py"
    exclude_patterns:
      - "**/vendor/**"
      - "**/migrations/**"

  # Custom training data sources
  training:
    synthetic_samples: 20000
    seed: 12345
    additional_sources:
      - type: "jsonl"
        path: "/opt/company/error-corpus.jsonl"
        priority: 2
      - type: "oip"
        path: "/opt/company/oip-data.json"
        priority: 1

  # Stricter convergence for production
  convergence:
    target_rate: 0.90          # 90% for production systems
    max_iterations: 100
    patience: 10

  # Distributed training
  distributed:
    enabled: true
    workers: 4

  # Integration
  integration:
    prometheus_endpoint: "http://metrics:9090"
    slack_webhook: "${SLACK_WEBHOOK_URL}"
    alert_on_regression: true
```

### 4.3 Environment Variable Overrides

```bash
# Override any config via environment
export UTOL_CORPUS_PATH="../my-corpus"
export UTOL_TARGET_RATE=0.85
export UTOL_MAX_ITERATIONS=30
export UTOL_DISPLAY_MODE=minimal
```

---

## 5. Loop Algorithm

### 5.1 Main Loop (Pseudocode)

```rust
pub async fn run_utol(config: UtolConfig) -> UtolResult {
    // Initialize components
    let mut corpus = CorpusManager::new(&config.corpus)?;
    let mut oracle = Oracle::load_or_train(&config.model)?;
    let mut lineage = OracleLineage::load_or_create()?;
    let mut display = AndonDisplay::new(&config.display)?;
    let mut drift_detector = ADWIN::with_delta(0.002);

    // PLAN: Assess initial state
    let mut state = LoopState {
        iteration: 0,
        compile_rate: 0.0,
        last_trained: oracle.last_trained(),
        model_size: oracle.size_bytes(),
        patience_counter: 0,
    };

    display.show_header(&state)?;

    // Main PDCA loop
    while state.iteration < config.convergence.max_iterations {
        state.iteration += 1;
        display.update_iteration(&state)?;

        // DO: Compile corpus
        let results = compile_corpus(&corpus, &oracle).await?;

        // CHECK: Evaluate results
        let metrics = evaluate_results(&results)?;
        state.compile_rate = metrics.compile_rate;

        // Update drift detector
        let drift_status = drift_detector.observe(metrics.error_rate);
        display.update_metrics(&metrics, drift_status)?;

        // ACT: Decide next action
        match decide_action(&state, &metrics, &config.convergence) {
            Action::Converged => {
                display.show_success(&state)?;
                break;
            }
            Action::Retrain { failing_examples } => {
                // Extract new training samples from failures
                let new_samples = extract_samples(&failing_examples)?;
                corpus.add_samples(new_samples)?;

                // Retrain oracle
                oracle = retrain_oracle(&corpus, &config.training)?;
                state.last_trained = Utc::now();
                state.model_size = oracle.size_bytes();
                state.patience_counter = 0;

                // Record in lineage
                lineage.record_training(&oracle, &metrics)?;
                display.update_model_status(&state)?;
            }
            Action::NoImprovement => {
                state.patience_counter += 1;
                if state.patience_counter >= config.convergence.patience {
                    display.show_plateau(&state)?;
                    break;
                }
            }
            Action::Continue => {
                // Nothing to do, continue to next iteration
            }
        }

        // Check for regressions
        if let Some((reason, delta)) = lineage.find_regression() {
            display.show_regression_alert(&reason, delta)?;
        }
    }

    // Final report
    let report = generate_report(&state, &lineage)?;
    display.show_final_report(&report)?;

    Ok(UtolResult {
        final_compile_rate: state.compile_rate,
        iterations: state.iteration,
        model_version: oracle.version(),
        converged: state.compile_rate >= config.convergence.target_rate,
    })
}
```

### 5.2 Convergence Estimation (Kalman Filter)

```rust
// Predict final convergence rate using Kalman-filtered trend
pub struct ConvergenceEstimator {
    kalman: KalmanFilter,
    history: Vec<f64>,
    target: f64,
}

impl ConvergenceEstimator {
    pub fn update(&mut self, compile_rate: f64) -> ConvergenceEstimate {
        self.history.push(compile_rate);
        let smoothed = self.kalman.update(compile_rate);

        // Linear extrapolation with uncertainty
        let trend = self.calculate_trend();
        let estimated_final = smoothed + trend * self.remaining_iterations() as f64;
        let confidence = self.calculate_confidence();

        ConvergenceEstimate {
            current: compile_rate,
            estimated_final,
            confidence,
            will_converge: estimated_final >= self.target,
            iterations_to_target: self.estimate_iterations_to_target(trend),
        }
    }
}
```

### 5.3 Action Decision Matrix

| Compile Rate | Trend | Drift | Patience | Action |
|--------------|-------|-------|----------|--------|
| ≥ target | any | Stable | any | **Converged** |
| < target | Improving | Stable | any | **Continue** |
| < target | Flat | Stable | < max | **Continue** |
| < target | Flat | Stable | ≥ max | **Plateau** |
| < target | Degrading | Warning | any | **Retrain** |
| any | any | Drift | any | **Retrain** |
| < target | any | Stable | any | **Retrain** (if new failures) |

---

## 6. Integration with Existing Tooling

### 6.1 Entrenar Integration

```rust
// Use entrenar's Trainer for model training
use entrenar::train::{Trainer, TrainConfig, Batch};
use entrenar::train::callback::{
    EarlyStopping,
    TerminalMonitorCallback,
    CheckpointCallback,
};
use entrenar::monitor::AndonSystem;

pub fn train_oracle_with_entrenar(
    corpus: &UnifiedCorpus,
    config: &TrainingConfig,
) -> Result<Oracle> {
    let (features, labels) = corpus.to_features()?;

    let trainer_config = TrainConfig::new()
        .with_grad_clip(1.0)
        .with_log_interval(10);

    let mut trainer = Trainer::new(
        oracle.parameters(),
        Adam::new(config.learning_rate),
        trainer_config,
    );

    // Add callbacks for visual feedback
    trainer.add_callback(TerminalMonitorCallback::new(
        RefreshPolicy::Adaptive { min_ms: 100, max_ms: 500 }
    ));
    trainer.add_callback(EarlyStopping::new(
        config.patience,
        config.min_delta,
    ));
    trainer.add_callback(CheckpointCallback::new(
        save_best_only: true,
    ));

    // Train with visual feedback
    let result = trainer.train(
        config.max_epochs,
        || corpus.iter_batches(config.batch_size),
        |batch| oracle.forward(batch),
    )?;

    Ok(oracle)
}
```

### 6.2 Aprender Integration

```rust
// Use aprender for model persistence and metrics
use aprender::format::{save, load, SaveOptions, ModelType, Compression};
use aprender::metrics::{accuracy, f1_score, confusion_matrix};
use aprender::scoring::QualityScore;

pub fn save_oracle(oracle: &Oracle, path: &Path) -> Result<()> {
    let options = SaveOptions::new()
        .with_name("depyler-oracle")
        .with_description("UTOL-trained error classification model")
        .with_compression(Compression::ZstdDefault);

    save(&oracle.classifier, ModelType::RandomForest, path, options)?;
    Ok(())
}

pub fn evaluate_oracle(oracle: &Oracle, test_set: &TestSet) -> Metrics {
    let predictions = oracle.predict_batch(&test_set.features);

    Metrics {
        accuracy: accuracy(&predictions, &test_set.labels),
        f1_macro: f1_score(&predictions, &test_set.labels, Average::Macro),
        f1_weighted: f1_score(&predictions, &test_set.labels, Average::Weighted),
        confusion: confusion_matrix(&predictions, &test_set.labels),
        quality_score: QualityScore::compute(&oracle),
    }
}
```

### 6.3 Drift Detection (ADWIN)

```rust
// Use aprender's ADWIN for drift detection
use aprender::online::ADWIN;

pub struct DriftMonitor {
    detector: ADWIN,
    window_size: usize,
    alert_threshold: f64,
}

impl DriftMonitor {
    pub fn observe(&mut self, error_occurred: bool) -> DriftStatus {
        let value = if error_occurred { 1.0 } else { 0.0 };
        self.detector.add(value);

        match self.detector.status() {
            adwin::Status::Stable => DriftStatus::Stable,
            adwin::Status::Warning => DriftStatus::Warning,
            adwin::Status::Drift => DriftStatus::Drift,
        }
    }

    pub fn stats(&self) -> DriftStats {
        DriftStats {
            n_samples: self.detector.n_samples(),
            error_rate: self.detector.mean(),
            std_dev: self.detector.std_dev(),
        }
    }
}
```

---

## 7. CLI Interface

### 7.1 Commands

```bash
# Run UTOL with defaults (uses corpus at ../reprorusted-python-cli)
depyler utol

# Run with custom corpus
depyler utol --corpus /path/to/corpus

# Run with specific target rate
depyler utol --target-rate 0.85

# Run in minimal display mode (CI-friendly)
depyler utol --display minimal

# Run with JSON output (for automation)
depyler utol --display json --output results.json

# Show current model status
depyler utol status

# Force retrain
depyler utol retrain --force

# Generate report
depyler utol report --format markdown --output report.md

# Watch mode - continuously monitor and re-run on file changes
depyler utol --watch

# Watch mode with custom debounce interval
depyler utol --watch --watch-debounce 2000
```

### 7.2 Output Formats

**Rich Mode** (default, interactive):
```
See Section 3.1 Visual Feedback Display
```

**Minimal Mode** (CI/CD):
```
UTOL [12/20] 60% | Rate: 83.2% | Target: 80.0% | Status: ON_TRACK
UTOL [13/20] 65% | Rate: 84.1% | Target: 80.0% | Status: ON_TRACK
UTOL [14/20] 70% | Rate: 85.0% | Target: 80.0% | Status: CONVERGED
```

**JSON Mode** (automation):
```json
{
  "iteration": 14,
  "max_iterations": 20,
  "compile_rate": 0.850,
  "target_rate": 0.800,
  "status": "converged",
  "model": {
    "version": "oracle-3.21.0-1733688135",
    "size_bytes": 515072,
    "last_trained": "2025-12-08T20:22:15Z"
  },
  "metrics": {
    "accuracy": 0.853,
    "f1_score": 0.789,
    "drift_status": "stable"
  },
  "category_rates": {
    "TypeMismatch": 0.82,
    "TraitBound": 0.71,
    "Import": 0.91,
    "Scope": 0.78,
    "Borrowing": 0.84,
    "Lifetime": 0.65,
    "Syntax": 0.88
  }
}
```

---

## 8. Peer-Reviewed Specifications

The following 25 specifications support UTOL implementation:

### 8.1 Core Loop Specifications

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| **UTOL-001** | Loop State Machine | Draft | Formal state machine definition |
| **UTOL-002** | Convergence Criteria | Draft | Mathematical convergence definition |
| **UTOL-003** | Action Decision Logic | Draft | Decision matrix formalization |
| **UTOL-004** | Iteration Bounds | Draft | Max iterations, patience logic |
| **UTOL-005** | Error Recovery | Draft | Graceful handling of failures |

### 8.2 Training Specifications

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| **UTOL-010** | Unified Corpus Format | Draft | Standard corpus structure |
| **UTOL-011** | Sample Extraction | Draft | Extracting samples from failures |
| **UTOL-012** | Feature Engineering | Draft | 73-dimension feature space |
| **UTOL-013** | Class Balancing | Draft | Heijunka for training data |
| **UTOL-014** | Incremental Training | Draft | Online learning updates |

### 8.3 Evaluation Specifications

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| **UTOL-020** | Metric Definitions | Draft | Accuracy, F1, compile rate |
| **UTOL-021** | K-Fold Cross Validation | Draft | Validation methodology |
| **UTOL-022** | Drift Detection | Draft | ADWIN integration |
| **UTOL-023** | Regression Detection | Draft | Quality degradation alerts |
| **UTOL-024** | Category-Level Metrics | Draft | Per-category tracking |

### 8.4 Visual Feedback Specifications

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| **UTOL-030** | Andon Display Format | Draft | TUI layout specification |
| **UTOL-031** | Sparkline Rendering | Draft | Unicode sparkline format |
| **UTOL-032** | Progress Bar | Draft | Kalman-filtered ETA |
| **UTOL-033** | Alert Levels | Draft | Stable/Warning/Critical/Drift |
| **UTOL-034** | Refresh Policy | Draft | Adaptive refresh rates |

### 8.5 Configuration Specifications

| ID | Title | Status | Description |
|----|-------|--------|-------------|
| **UTOL-040** | YAML Schema | Draft | Configuration file format |
| **UTOL-041** | Environment Overrides | Draft | Env var precedence |
| **UTOL-042** | Default Values | Draft | Zero-config defaults |
| **UTOL-043** | Enterprise Customization | Draft | Multi-corpus support |
| **UTOL-044** | Distributed Training | Draft | Multi-worker support |

---

## 9. Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

- [ ] **UTOL-001**: Implement loop state machine
- [ ] **UTOL-010**: Define unified corpus format
- [ ] **UTOL-040**: Create YAML configuration schema
- [ ] **UTOL-030**: Basic TUI display

### Phase 2: Training Integration (Week 3-4)

- [ ] **UTOL-011**: Sample extraction from failures
- [ ] **UTOL-012**: Feature engineering pipeline
- [ ] **UTOL-014**: Incremental training support
- [ ] **UTOL-020**: Metric computation

### Phase 3: Visual Feedback (Week 5-6)

- [ ] **UTOL-031**: Sparkline rendering
- [ ] **UTOL-032**: Progress bar with ETA
- [ ] **UTOL-033**: Alert system
- [ ] **UTOL-034**: Adaptive refresh

### Phase 4: Production Hardening (Week 7-8)

- [ ] **UTOL-022**: ADWIN drift detection
- [ ] **UTOL-023**: Regression detection
- [ ] **UTOL-005**: Error recovery
- [ ] **UTOL-044**: Distributed training

---

## 10. Quality Gates

### 10.1 Pre-Commit Gates

```yaml
# Enforced via .git/hooks/pre-commit
quality_gates:
  - name: "UTOL Tests Pass"
    command: "cargo test -p depyler-oracle utol_"
    required: true

  - name: "TDG Score"
    command: "pmat tdg check-quality --min-grade A-"
    required: true

  - name: "No Regressions"
    command: "pmat tdg check-regression --baseline baseline.json"
    required: true
```

### 10.2 CI/CD Gates

```yaml
# .github/workflows/utol.yml
jobs:
  utol-validation:
    steps:
      - name: Run UTOL
        run: |
          depyler utol \
            --corpus ./test-corpus \
            --target-rate 0.80 \
            --max-iterations 20 \
            --display json \
            --output utol-results.json

      - name: Validate Convergence
        run: |
          jq -e '.status == "converged"' utol-results.json

      - name: Check No Regression
        run: |
          jq -e '.compile_rate >= 0.80' utol-results.json
```

### 10.3 Quality Scoring (Aprender)

```rust
// 100-point Toyota Way quality score
pub struct UtolQualityScore {
    // Kaizen: Continuous improvement
    accuracy_score: f32,        // 25 pts

    // Jidoka: Built-in quality
    convergence_score: f32,     // 20 pts

    // Muda elimination: Efficiency
    iteration_efficiency: f32,   // 15 pts

    // Genchi Genbutsu: Direct observation
    coverage_score: f32,         // 15 pts

    // Standardization
    reproducibility: f32,        // 15 pts

    // Poka-yoke: Error proofing
    robustness: f32,             // 10 pts
}
```

---

## 11. Monitoring & Observability

### 11.1 Prometheus Metrics

```rust
// Exported metrics for monitoring
pub struct UtolMetrics {
    // Counters
    utol_iterations_total: Counter,
    utol_retrains_total: Counter,
    utol_failures_total: Counter,

    // Gauges
    utol_compile_rate: Gauge,
    utol_accuracy: Gauge,
    utol_model_size_bytes: Gauge,
    utol_drift_status: Gauge,  // 0=stable, 1=warning, 2=drift

    // Histograms
    utol_iteration_duration_seconds: Histogram,
    utol_training_duration_seconds: Histogram,
}
```

### 11.2 Alerting Rules

```yaml
# prometheus/alerts.yml
groups:
  - name: utol
    rules:
      - alert: UtolDriftDetected
        expr: utol_drift_status == 2
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "UTOL drift detected - model degradation"

      - alert: UtolCompileRateLow
        expr: utol_compile_rate < 0.70
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "UTOL compile rate below 70%"
```

---

## 12. Example Usage

### 12.1 Default Usage (Zero Config)

```bash
# Just run it - uses defaults for reprorusted-python-cli corpus
cd /path/to/depyler
depyler utol
```

### 12.2 Enterprise Usage

```bash
# Create custom config
cat > .depyler/utol.yaml << 'EOF'
utol:
  corpus:
    path: "/opt/company/python-services"
  convergence:
    target_rate: 0.90
    max_iterations: 100
  integration:
    prometheus_endpoint: "http://monitoring:9090"
EOF

# Run with custom config
depyler utol --config .depyler/utol.yaml
```

### 12.3 CI/CD Integration

```yaml
# .github/workflows/nightly.yml
name: Nightly UTOL Training
on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM daily

jobs:
  train:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run UTOL
        run: |
          depyler utol \
            --target-rate 0.80 \
            --display minimal \
            --output utol-results.json

      - name: Upload Model
        uses: actions/upload-artifact@v4
        with:
          name: oracle-model
          path: depyler_oracle.apr

      - name: Post Results to Slack
        if: always()
        run: |
          curl -X POST $SLACK_WEBHOOK \
            -H 'Content-Type: application/json' \
            -d "$(jq '{text: \"UTOL: \(.status) at \(.compile_rate | . * 100 | floor)%\"}' utol-results.json)"
```

---

## 13. Glossary

| Term | Definition |
|------|------------|
| **UTOL** | Unified Training Oracle Loop |
| **Andon** | Visual management system (Toyota term) |
| **Jidoka** | Autonomation - automation with human touch |
| **Kaizen** | Continuous improvement |
| **Heijunka** | Production leveling/smoothing |
| **PDCA** | Plan-Do-Check-Act (Deming cycle) |
| **ADWIN** | Adaptive Windowing drift detection algorithm |
| **Compile Rate** | Percentage of corpus that compiles successfully |
| **Convergence** | Reaching target compile rate |
| **Drift** | Model performance degradation over time |

---

## 14. References

1. Ohno, T. (1988). *Toyota Production System: Beyond Large-Scale Production*. Productivity Press.
2. Liker, J. (2004). *The Toyota Way: 14 Management Principles*. McGraw-Hill.
3. Bifet, A. & Gavalda, R. (2007). Learning from Time-Changing Data with Adaptive Windowing. *SIAM International Conference on Data Mining*.
4. Kalman, R.E. (1960). A New Approach to Linear Filtering and Prediction Problems. *Journal of Basic Engineering*.
5. Deming, W.E. (1986). *Out of the Crisis*. MIT Press.
6. Shingo, S. (1986). *Zero Quality Control: Source Inspection and the Poka-Yoke System*. Productivity Press.
7. Poppendieck, M., & Poppendieck, T. (2003). *Lean Software Development: An Agile Toolkit*. Addison-Wesley.
8. Rother, M. (2009). *Toyota Kata: Managing People for Improvement, Adaptiveness and Superior Results*. McGraw-Hill.
9. Sculley, D., et al. (2015). Hidden Technical Debt in Machine Learning Systems. *Advances in Neural Information Processing Systems 28*.
10. Barr, E. T., et al. (2015). The Oracle Problem in Software Testing: A Survey. *IEEE Transactions on Software Engineering*, 41(5).
11. Settles, B. (2009). *Active Learning Literature Survey*. University of Wisconsin-Madison.
12. Breiman, L. (2001). Random Forests. *Machine Learning*, 45(1).
13. Humble, J., & Farley, D. (2010). *Continuous Delivery: Reliable Software Releases through Build, Test, and Deployment Automation*. Addison-Wesley.
14. Womack, J. P., Jones, D. T., & Roos, D. (1990). *The Machine That Changed the World*. Free Press.
15. Amershi, S., et al. (2019). Software Engineering for Machine Learning: A Case Study. *ICSE-SEIP*.

---

## 15. Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-12-08 | Initial specification |

---

*"Stop and fix problems to get quality right the first time."* - Toyota Way Principle #5 (Jidoka)

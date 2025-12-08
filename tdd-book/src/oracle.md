# Oracle: ML-Powered Error Classification

The Depyler Oracle is an internal ML system that classifies transpilation compile errors and suggests fixes. It uses a Random Forest classifier trained on error patterns to accelerate the debug-fix cycle.

## What the Oracle Does

When transpiled Rust code fails to compile, the Oracle:

1. **Extracts features** from the error message (TF-IDF, error codes, keywords)
2. **Classifies** the error into categories (TypeMismatch, BorrowChecker, MissingImport, etc.)
3. **Suggests fixes** based on historical patterns
4. **Detects drift** to signal when retraining is needed

## Workflow: The Compile Heuristic

```
Python Source
     │
     ▼
┌─────────────┐
│  Transpile  │
└─────────────┘
     │
     ▼
┌─────────────┐
│   rustc     │───── Success ─────► Done
└─────────────┘
     │
   Error
     │
     ▼
┌─────────────┐
│  Featurize  │  Extract TF-IDF features from error text
└─────────────┘
     │
     ▼
┌─────────────┐
│  Classify   │  Random Forest predicts error category
└─────────────┘
     │
     ▼
┌─────────────┐
│   Suggest   │  Return fix templates for category
└─────────────┘
     │
     ▼
Developer applies fix to TRANSPILER (never to generated code)
```

## Error Categories

| Category | Description | Example Fix |
|----------|-------------|-------------|
| `TypeMismatch` | Type conversion needed | Use `.into()` or `as` |
| `BorrowChecker` | Ownership/borrowing issue | Clone or use reference |
| `MissingImport` | Missing `use` statement | Add import |
| `SyntaxError` | Malformed syntax | Check braces/semicolons |
| `LifetimeError` | Lifetime annotation needed | Add `'a` annotation |
| `TraitBound` | Missing trait impl | Implement trait or add bound |
| `Other` | Uncategorized | Review full error |

## Model Architecture

### Primary: Random Forest Classifier
- **Algorithm**: Random Forest Classifier (100 trees, max depth 10)
- **Features**: TF-IDF vectors from error messages
- **Training data**: Synthetic corpus + verificar integration + depyler-specific patterns
- **Model file**: `depyler_oracle.apr` (generated on first use)

### NEW: MoE Oracle (DEPYLER-0580)

The **Mixture of Experts (MoE) Oracle** provides specialized error classification with 4 domain experts:

```
                    ┌─────────────────┐
   Error Code ─────►│  Gating Network │
   + Context        └────────┬────────┘
                             │
         ┌───────────────────┼───────────────────┐
         ▼                   ▼                   ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│   TypeSystem    │ │ ScopeResolution │ │   MethodField   │
│   Expert (0)    │ │   Expert (1)    │ │   Expert (2)    │
│ E0308,E0277,    │ │ E0425,E0412,    │ │ E0599,E0609,    │
│ E0606,E0061     │ │ E0433,E0423     │ │ E0615           │
└────────┬────────┘ └────────┬────────┘ └────────┬────────┘
         │                   │                   │
         └───────────────────┼───────────────────┘
                             ▼
                  ┌─────────────────────┐
                  │  SyntaxBorrowing    │
                  │     Expert (3)      │
                  │ E0369,E0282,E0027   │
                  └─────────────────────┘
```

**Expert Domains**:

| Expert | Error Codes | Specialization |
|--------|-------------|----------------|
| TypeSystem | E0308, E0277, E0606 | Type mismatches, trait bounds |
| ScopeResolution | E0425, E0412, E0433 | Missing imports, undefined names |
| MethodField | E0599, E0609 | Method/field not found |
| SyntaxBorrowing | E0369, E0282, E0027 | Operators, type annotations |

**Usage**:

```rust
use depyler_oracle::{classify_with_moe, ExpertDomain};

let result = classify_with_moe("E0308", "mismatched types expected i32, found String");
println!("Expert: {:?}", result.primary_expert);  // TypeSystem
println!("Fix: {:?}", result.suggested_fix);      // Some("Add type coercion...")
println!("Confidence: {:.2}", result.confidence); // 0.85
```

**Why MoE?**
- **Specialization**: Each expert learns domain-specific patterns
- **Interpretability**: Clear routing based on error codes
- **Extensibility**: Easy to add new experts for new error domains
- **Robustness**: Works without training (uses default fix patterns)

## Using the Oracle

The Oracle is **not** a user-facing feature. It's internal developer infrastructure.

### Automatic Loading

```rust
use depyler_oracle::Oracle;

// Loads from disk or trains and saves
let oracle = Oracle::load_or_train()?;
```

On first call, `load_or_train()`:
1. Checks for `depyler_oracle.apr` in project root
2. If found, loads the trained model
3. If not found, trains on the combined corpus and saves

Subsequent calls load the cached model (~100ms vs ~60s training).

### Classifying Errors

```rust
use depyler_oracle::{Oracle, ErrorFeatures};

let oracle = Oracle::load_or_train()?;
let features = ErrorFeatures::from_error_message(
    "error[E0308]: mismatched types - expected `i32`, found `&str`"
);
let result = oracle.classify(&features)?;

println!("Category: {:?}", result.category);      // TypeMismatch
println!("Confidence: {}", result.confidence);    // 0.85
println!("Suggested: {:?}", result.suggested_fix); // Some("Convert type using `.into()` or `as`")
```

## Training on Custom Corpus

Organizations can train the Oracle on their own error patterns:

```rust
use depyler_oracle::{Oracle, TrainingDataset, TrainingSample, ErrorCategory};
use aprender::primitives::Matrix;

// Build custom corpus
let mut dataset = TrainingDataset::new();
dataset.add(TrainingSample::new(
    "error[E0277]: the trait bound `MyType: Display` is not satisfied",
    ErrorCategory::TraitBound,
));
// ... add more samples

// Convert to features
let (features, labels) = depyler_oracle::samples_to_features(dataset.samples());
let labels: Vec<usize> = labels.as_slice().iter().map(|&x| x as usize).collect();

// Train
let mut oracle = Oracle::new();
oracle.train(&features, &labels)?;

// Save for reuse
oracle.save(Path::new("my_company_oracle.apr"))?;
```

### Training Data Requirements

- **Minimum**: 50+ samples for basic accuracy
- **Recommended**: 500+ samples for robust classification
- **Optimal**: 1,000+ samples for production use

Each sample needs:
- Error message text (the `rustc` output)
- Correct category label

## Synthetic Data Generation

Instead of manually labeling errors, use combinatorial generation:

```rust
use depyler_oracle::{generate_synthetic_corpus, generate_synthetic_corpus_sized};

// Default: 12,000+ samples via template × type × context combinations
let corpus = generate_synthetic_corpus();

// Custom size
let corpus = generate_synthetic_corpus_sized(50_000);
```

The generator combines:
- Error message templates (E0308, E0382, E0433, etc.)
- Type variations (i32, String, Vec<T>, etc.)
- Context patterns (function calls, assignments, returns)

This produces diverse training data without real compilation.

## AutoML Hyperparameter Tuning

Automatically find optimal model configuration:

```rust
use depyler_oracle::{automl_optimize, automl_quick, AutoMLConfig};

// Quick tuning (fewer iterations)
let result = automl_quick(&corpus)?;

// Full optimization
let config = AutoMLConfig::default();
let result = automl_optimize(&corpus, config)?;

println!("Best n_trees: {}", result.best_n_trees);
println!("Best max_depth: {}", result.best_max_depth);
println!("Accuracy: {:.2}%", result.accuracy * 100.0);
```

AutoML searches the hyperparameter space (tree count, depth, features) to maximize cross-validation accuracy.

**Workflow**: Synthetic generation + AutoML = scalable corpus without manual labeling.

## Learning from GitHub History (OIP Integration)

The Oracle can learn from Git commit history using the **organizational-intelligence-plugin (OIP)**. This provides real-world training data extracted from how developers actually fix errors.

### The Recipe

```bash
# Step 1: Use OIP to extract training data from a Rust repository
oip extract-training-data --repo ../depyler --output training-data.json

# Step 2: Load and convert in depyler-oracle
```

```rust
use depyler_oracle::{
    load_oip_training_data, convert_oip_to_depyler,
    analyze_corpus, get_moe_samples_from_oip
};
use std::path::Path;

// Load OIP training data
let oip_data = load_oip_training_data(Path::new("training-data.json"))?;

// Analyze what we loaded
let stats = analyze_corpus(&oip_data);
println!("Loaded {} examples", stats.total_examples);
println!("By expert domain:");
for (domain, count) in &stats.by_expert {
    println!("  {:?}: {}", domain, count);
}

// Convert to depyler format
let depyler_dataset = convert_oip_to_depyler(&oip_data);

// Or get MoE training samples
let moe_samples = get_moe_samples_from_oip(&oip_data);
```

### Category Mapping

OIP uses 18 defect categories (10 general + 8 transpiler-specific). These map to depyler's `ErrorCategory` and `ExpertDomain`:

| OIP Category | depyler ErrorCategory | MoE ExpertDomain |
|-------------|----------------------|------------------|
| `OwnershipBorrow` | `BorrowChecker` | `SyntaxBorrowing` |
| `MemorySafety` | `BorrowChecker` | `SyntaxBorrowing` |
| `TypeErrors` | `TypeMismatch` | `TypeSystem` |
| `TypeAnnotationGaps` | `TypeMismatch` | `TypeSystem` |
| `TraitBounds` | `TraitBound` | `TypeSystem` |
| `StdlibMapping` | `MissingImport` | `ScopeResolution` |
| `ASTTransform` | `MissingImport` | `ScopeResolution` |
| `ConfigurationErrors` | `MissingImport` | `ScopeResolution` |
| `ApiMisuse` | `Other` | `MethodField` |
| `IteratorChain` | `Other` | `MethodField` |
| `ComprehensionBugs` | `Other` | `MethodField` |

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                organizational-intelligence-plugin                │
│                                                                 │
│  oip extract-training-data --repo ../depyler                    │
│      │                                                          │
│      ▼                                                          │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Git History Mining                                     │    │
│  │  - Filter fix commits (fix:, bug:, patch:)              │    │
│  │  - Extract commit messages                              │    │
│  │  - Auto-label with rule-based classifier               │    │
│  └─────────────────────────────────────────────────────────┘    │
│      │                                                          │
│      ▼                                                          │
│  training-data.json                                             │
│  {train: [...], validation: [...], test: [...]}                 │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      depyler-oracle                             │
│                                                                 │
│  load_oip_training_data("training-data.json")                   │
│      │                                                          │
│      ▼                                                          │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Category Mapping                                       │    │
│  │  OipDefectCategory → ErrorCategory / ExpertDomain       │    │
│  └─────────────────────────────────────────────────────────┘    │
│      │                                                          │
│      ▼                                                          │
│  TrainingDataset / MoE training samples                         │
└──────────────────────────────────────────────────────────────────┘
```

### Real Example

From OIP's training on the organizational-intelligence-plugin repo:

```
OIP Corpus Statistics:
  Total examples: 31
  Avg confidence: 0.83
  By category:
    ASTTransform: 9 (29%)
    OwnershipBorrow: 5 (16%)
    ConfigurationErrors: 5 (16%)
    TypeErrors: 4 (13%)
    TraitBounds: 3 (10%)
    MemorySafety: 3 (10%)
    SecurityVulnerabilities: 1 (3%)
    ComprehensionBugs: 1 (3%)
  By expert domain:
    ScopeResolution: 14 (45%)
    SyntaxBorrowing: 9 (29%)
    TypeSystem: 7 (23%)
    MethodField: 1 (3%)
```

### Benefits

1. **Real patterns**: Learns from how developers actually fix errors, not synthetic examples
2. **Domain-specific**: Your codebase's unique error patterns are captured
3. **Continuous learning**: Re-extract as commit history grows
4. **No manual labeling**: OIP auto-labels based on commit message analysis

### Combining with Synthetic Data

Best results come from mixing real and synthetic data:

```rust
use depyler_oracle::{
    generate_synthetic_corpus,
    load_oip_training_data,
    convert_oip_to_depyler
};

// Start with synthetic corpus (10,000+ samples)
let mut corpus = generate_synthetic_corpus();

// Add real GitHub data
let oip_data = load_oip_training_data(Path::new("training-data.json"))?;
let real_data = convert_oip_to_depyler(&oip_data);

for sample in real_data.samples() {
    corpus.add(sample.clone());
}

// Train on combined corpus
let (features, labels) = samples_to_features(corpus.samples());
oracle.train(&features, &labels)?;
```

**Why combine?**
- Synthetic provides breadth (all error codes)
- Real provides depth (domain-specific patterns)
- Together: robust classification across error types

## Unified Training Pipeline

For reproducible, deterministic training, use the **Unified Training Pipeline**. This merges all data sources with guaranteed reproducibility.

### Architecture

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│  Synthetic  │  │   Depyler   │  │  Verificar  │  │ OIP GitHub  │  │ Real Errors │
│   Corpus    │  │   Corpus    │  │   Corpus    │  │   Corpus    │  │    File     │
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘
       │                │                │                │                │
       └────────────────┴────────────────┴────────────────┴────────────────┘
                                         │
                                         ▼
                              ┌─────────────────────┐
                              │   Merge & Dedupe    │
                              │   (by error hash)   │
                              └──────────┬──────────┘
                                         │
                                         ▼
                              ┌─────────────────────┐
                              │  Deterministic      │
                              │  Shuffle (seed=42)  │
                              └──────────┬──────────┘
                                         │
                                         ▼
                              ┌─────────────────────┐
                              │   Train Oracle      │
                              │   (Random Forest)   │
                              └──────────┬──────────┘
                                         │
                                         ▼
                              ┌─────────────────────┐
                              │  Save Model (.apr)  │
                              └─────────────────────┘
```

### Basic Usage

```rust
use depyler_oracle::{
    build_unified_corpus, build_default_unified_corpus,
    UnifiedTrainingConfig, print_merge_stats
};

// Default configuration (12,000 synthetic samples, seed=42)
let result = build_default_unified_corpus();
print_merge_stats(&result.stats);

// Use the dataset for training
let dataset = result.dataset;
```

### Custom Configuration

```rust
let config = UnifiedTrainingConfig {
    seed: 42,                          // Reproducibility seed
    synthetic_samples: 20_000,         // More synthetic data
    oip_data_path: Some("training-data.json".to_string()),
    real_errors_path: Some("production_errors.txt".to_string()),
    balance_classes: true,             // Balance class distribution
    max_per_class: Some(5000),         // Cap samples per category
};

let result = build_unified_corpus(&config);
```

### With OIP Data

```rust
use depyler_oracle::build_unified_corpus_with_oip;

// Convenience function for OIP integration
let result = build_unified_corpus_with_oip("training-data.json");

println!("Merge Statistics:");
println!("  Synthetic:     {} samples", result.stats.synthetic_count);
println!("  Depyler:       {} samples", result.stats.depyler_count);
println!("  Verificar:     {} samples", result.stats.verificar_count);
println!("  OIP GitHub:    {} samples", result.stats.oip_count);
println!("  Duplicates:    {} removed", result.stats.duplicates_removed);
println!("  Final count:   {} samples", result.stats.final_count);
```

### Real Errors File Format

Load production errors from a simple text file:

```
# Format: ERROR_CODE|context|category|fix
E0308|expected i32, found String|TypeMismatch|Use .parse() or change type
E0382|value moved here|BorrowChecker|Clone the value or use reference
E0433|unresolved import|MissingImport|Add use statement
```

### Deduplication

The pipeline deduplicates samples by normalizing error messages:
- Convert to lowercase
- Collapse whitespace
- Hash for O(1) lookup

This prevents duplicate samples from different sources from skewing training.

### Deterministic Shuffling

Uses a Linear Congruential Generator (LCG) for reproducible shuffling:

```rust
// Same seed = same order, every time
let config = UnifiedTrainingConfig {
    seed: 42,  // Change seed for different shuffle
    ..Default::default()
};
```

This ensures:
- Reproducible experiments
- Consistent cross-validation splits
- Debuggable training issues

### Class Balancing

Prevent majority class dominance:

```rust
let config = UnifiedTrainingConfig {
    balance_classes: true,
    max_per_class: Some(2000),  // Cap at 2000 samples per category
    ..Default::default()
};
```

### Statistics

The pipeline returns detailed statistics:

```rust
let result = build_unified_corpus(&config);

// Category distribution
for (category, count) in &result.stats.by_category {
    let pct = (*count as f64 / result.stats.final_count as f64) * 100.0;
    println!("  {:?}: {} ({:.1}%)", category, count, pct);
}
```

Example output:
```
Unified Corpus Statistics:
  Data Sources:
    Synthetic:     12000 samples
    Depyler:          45 samples
    Verificar:       120 samples
    OIP GitHub:       31 samples
    Real Errors:       0 samples
  Merge Results:
    Before dedupe: 12196 samples
    Duplicates:       23 removed
    Final count:   12173 samples
  By Category:
    TypeMismatch: 4521 (37.1%)
    BorrowChecker: 2890 (23.7%)
    MissingImport: 1834 (15.1%)
    ...
```

## Rust-Based Corpus Extraction (TDD)

The corpus extraction has been rewritten in Rust for type safety and proper deduplication. This replaces the legacy bash scripts.

### Binary Usage

```bash
# Build the extraction binary
cargo build --release -p depyler-oracle --bin extract-training-data

# Run with defaults (verificar corpus → training_corpus/errors.jsonl)
./target/release/extract-training-data

# With options
./target/release/extract-training-data \
    --input-dir target/verificar/corpus \
    --output-dir target/verificar/output \
    --corpus training_corpus/errors.jsonl \
    --cycle 5 \
    --max-files 1000 \
    --verbose
```

### Makefile Integration

```bash
# Harvest real transpilation errors using Rust binary
make oracle-harvest

# Full training cycle
make oracle-cycle
```

### Library API

```rust
use depyler_oracle::corpus_extract::{TrainingCorpus, TrainingError};
use std::path::Path;

// Load existing corpus (deduplicates on load)
let mut corpus = TrainingCorpus::load(Path::new("errors.jsonl"))?;
println!("Loaded {} unique errors", corpus.len());

// Add new error (auto-generates hash for deduplication)
let error = TrainingError::new(
    "E0308",                           // error_code
    "mismatched types",                // message
    "expected i32, found String",      // context
    "examples/foo.py",                 // file
    3,                                 // cycle
);

if corpus.insert(error) {
    println!("New error added");
} else {
    println!("Duplicate, skipped");
}

// Merge another corpus (returns count of new unique errors)
let other = TrainingCorpus::load(Path::new("other.jsonl"))?;
let new_count = corpus.merge(other);
println!("Added {} new unique errors", new_count);

// Save
corpus.save(Path::new("errors.jsonl"))?;
```

### Why Rust Instead of Bash?

| Issue with Bash | Rust Solution |
|-----------------|---------------|
| `sort -u` on JSON doesn't dedupe by hash field | `HashSet<String>` with proper hash key |
| Subshell variable loss (SC2031) | No subshells, explicit state |
| md5sum differs across platforms | `DefaultHasher` (deterministic) |
| Silent failures | `Result<T, E>` with proper errors |
| 57K entries → 63 unique (bug) | Impossible with HashSet |

### Example: Corpus Extraction Demo

```bash
# Run the extraction example
cargo run --release -p depyler-oracle --example corpus_extract_demo
```

## Advanced: Full Custom Pipeline

For organizations with large codebases and compute budget:

### 1. Mass Transpile Your Codebase

```bash
find . -name "*.py" | xargs -I {} depyler transpile {} -o {}.rs
```

### 2. Capture Real Compile Errors

```rust
use std::process::Command;

fn capture_errors(rs_file: &str) -> Option<String> {
    let output = Command::new("rustc")
        .args(["--crate-type", "lib", rs_file])
        .output()
        .ok()?;

    if !output.status.success() {
        Some(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        None
    }
}
```

### 3. Auto-Label by Error Code

```rust
fn auto_label(error: &str) -> ErrorCategory {
    if error.contains("E0308") { ErrorCategory::TypeMismatch }
    else if error.contains("E0382") || error.contains("E0505") { ErrorCategory::BorrowChecker }
    else if error.contains("E0433") || error.contains("E0412") { ErrorCategory::MissingImport }
    else if error.contains("E0106") || error.contains("E0495") { ErrorCategory::LifetimeError }
    else if error.contains("E0277") { ErrorCategory::TraitBound }
    else { ErrorCategory::Other }
}
```

### 4. Merge Real + Synthetic Corpora

```rust
let mut corpus = generate_synthetic_corpus();

// Add real errors from your codebase
for (error_msg, category) in real_errors {
    corpus.add(TrainingSample::new(&error_msg, category));
}
```

### 5. Extended AutoML Search

```rust
let config = AutoMLConfig {
    max_iterations: 1000,      // 10× default
    n_trees_range: (50, 500),  // wider search
    max_depth_range: (5, 20),  // deeper trees
    cv_folds: 10,              // more rigorous
};

let result = automl_full(&corpus, config)?;
```

### 6. Continuous Learning

Re-train periodically as transpiler evolves:

```rust
// Weekly cron job
let new_errors = collect_recent_failures();
corpus.extend(new_errors);
let result = automl_optimize(&corpus, config)?;
oracle.save(Path::new("updated_oracle.apr"))?;
```

**Key insight**: Real errors from *your* codebase capture domain-specific patterns that synthetic data misses.

## Drift Detection

The Oracle monitors classification accuracy over time:

```rust
let mut oracle = Oracle::load_or_train()?;

// After each classification batch, report accuracy
let status = oracle.check_drift(recent_accuracy);

match status {
    DriftStatus::NoDrift => { /* Model performing well */ }
    DriftStatus::Warning => { /* Consider retraining soon */ }
    DriftStatus::Drift => { /* Retrain immediately */ }
}
```

Drift detection uses a sliding window comparison of historical vs. recent accuracy.

## Continuous Oracle Retraining (Issue #211)

The Oracle now implements **automatic change detection** to trigger retraining when the codebase or training corpus changes. This eliminates the "stale model" defect where transpiler improvements don't benefit from updated training data.

### How It Works

When `Oracle::load_or_train()` is called:

1. **Load Training State**: Reads `.depyler/oracle_state.json`
2. **Get Current State**: Fetches git HEAD SHA and computes corpus hash
3. **Compare States**: Checks if retraining is needed
4. **Auto-Retrain**: If changes detected, retrains and saves new state

```
┌─────────────────────────────────────────────────────────────────┐
│                    Oracle::load_or_train()                       │
│                                                                 │
│  ┌─────────────────┐     ┌─────────────────┐                    │
│  │ Load Training   │     │ Get Current     │                    │
│  │ State (.json)   │     │ SHA + Hash      │                    │
│  └────────┬────────┘     └────────┬────────┘                    │
│           │                       │                             │
│           └───────────┬───────────┘                             │
│                       ▼                                         │
│              ┌─────────────────┐                                │
│              │ needs_retraining│                                │
│              │ (sha, hash)?    │                                │
│              └────────┬────────┘                                │
│                       │                                         │
│           ┌───────────┴───────────┐                             │
│           ▼                       ▼                             │
│    ┌──────────────┐       ┌──────────────┐                      │
│    │  No Changes  │       │   Changed!   │                      │
│    │  Load Model  │       │   Retrain    │                      │
│    └──────────────┘       └──────┬───────┘                      │
│                                  │                              │
│                                  ▼                              │
│                          ┌──────────────┐                       │
│                          │ Save State   │                       │
│                          │ (.json)      │                       │
│                          └──────────────┘                       │
└─────────────────────────────────────────────────────────────────┘
```

### Training State Structure

The training state is stored in `.depyler/oracle_state.json`:

```json
{
  "last_trained_commit_sha": "abc123def456...",
  "corpus_hash": "a1b2c3d4e5f6g7h8",
  "last_trained_at": "2024-01-15T10:30:00Z",
  "sample_count": 12500,
  "model_version": "3.21.0"
}
```

| Field | Purpose |
|-------|---------|
| `last_trained_commit_sha` | Git HEAD when last trained |
| `corpus_hash` | Hash of training corpus file mtimes |
| `last_trained_at` | ISO 8601 timestamp |
| `sample_count` | Number of training samples used |
| `model_version` | Depyler version (for architecture changes) |

### Retraining Triggers

Retraining is automatically triggered when:

| Condition | Why |
|-----------|-----|
| **Commit SHA changed** | Transpiler code modified |
| **Corpus hash changed** | Training data files modified |
| **Model version changed** | Oracle architecture updated |
| **No state file exists** | First-time training |

### Usage

The feature is **automatic** - no code changes needed:

```rust
use depyler_oracle::Oracle;

// Automatically checks for changes and retrains if needed
let oracle = Oracle::load_or_train()?;

// Output examples:
// "📊 Oracle: Loaded cached model (no changes detected)"
// "📊 Oracle: Codebase changes detected, triggering retraining..."
// "📊 Oracle: Training complete (12500 samples), state saved"
```

### Manual State Management

For advanced use cases:

```rust
use depyler_oracle::TrainingState;
use std::path::Path;

// Load existing state
let state = TrainingState::load(&Path::new(".depyler/oracle_state.json"))?;

// Check if retraining needed
let current_sha = TrainingState::get_current_commit_sha();
let corpus_paths = get_training_corpus_paths();
let corpus_hash = TrainingState::compute_corpus_hash(&corpus_paths);

if state.map_or(true, |s| s.needs_retraining(&current_sha, &corpus_hash)) {
    println!("Retraining required!");
}

// Create and save new state after training
let new_state = TrainingState::new(current_sha, corpus_hash, sample_count);
new_state.save(&Path::new(".depyler/oracle_state.json"))?;
```

### Benefits

1. **No stale models**: Oracle always reflects latest transpiler patterns
2. **Zero manual intervention**: Automatic change detection
3. **Fast loads**: If no changes, loads cached model (~100ms)
4. **Audit trail**: State file tracks training history
5. **Version awareness**: Retrains when model architecture changes

## Configuration

```rust
use depyler_oracle::{Oracle, OracleConfig};

let config = OracleConfig {
    n_estimators: 10_000,  // Number of trees
    max_depth: 10,         // Maximum tree depth
    random_state: Some(42), // Reproducibility seed
};

let oracle = Oracle::with_config(config);
```

## Performance Characteristics

| Operation | Time |
|-----------|------|
| Load cached model | ~100ms |
| Train full corpus | ~60s |
| Single classification | ~1ms |
| Feature extraction | ~0.1ms |

## Integration with Depyler

The Oracle integrates at two points:

1. **Autofixer**: Automatically applies suggested fixes during iterative compilation
2. **Error reporting**: Enriches error messages with category and suggestions

```rust
use depyler_oracle::{AutoFixer, FixContext};

let autofixer = AutoFixer::new();
let context = FixContext {
    error_message: "...",
    source_code: "...",
    // ...
};

if let Some(fix) = autofixer.suggest_fix(&context) {
    // Apply fix to transpiler logic
}
```

## Why Not Ship the Model?

The model file (`depyler_oracle.apr`) is **not** distributed because:

1. **Size**: ~50MB serialized Random Forest
2. **Freshness**: Should be trained on current error patterns
3. **Customization**: Different codebases have different patterns
4. **Training is fast**: ~60s on first use, then cached

## Doctest Transpilation: Semantic Equivalence Training

The **highest-fidelity training signal** comes from transpiling Python doctests to Rust doc tests. Unlike compile-only validation, a passing doc test proves **semantic equivalence**.

### Training Signal Hierarchy

| Signal | What It Proves | Strength |
|--------|----------------|----------|
| `rustc` exit code | Compiles | Low |
| `rustc` error message | Type/syntax correct | Medium |
| `cargo test --doc` compile | Doc test syntax valid | High |
| **`cargo test --doc` pass** | **Semantic equivalence** | **Highest** |

### How It Works

**Python Source with Doctest:**
```python
def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number.

    >>> fibonacci(0)
    0
    >>> fibonacci(10)
    55
    """
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

**Transpiled Rust with Doc Test:**
```rust
/// Calculate the nth Fibonacci number.
///
/// ```
/// use mylib::fibonacci;
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(10), 55);
/// ```
fn fibonacci(n: i32) -> i32 {
    if n <= 1 { n } else { fibonacci(n - 1) + fibonacci(n - 2) }
}
```

### Why Doctests Matter for CITL

1. **Zero-cost corpus**: Python stdlib has ~10,000 doctests—no generation needed
2. **Human-verified**: Documentation is reviewed; synthetic data is not
3. **Micro-granular I/O pairs**: One function, one input, one expected output
4. **Type oracle**: `fibonacci(10) → 55` implies return type is `i32`
5. **Ground truth**: Pass = semantics preserved, not just "compiles"

### Doctest Result Types

```rust
pub enum DoctestResult {
    /// Doc test passed - semantic equivalence proven
    Pass { python_input: String, rust_input: String, output: String },

    /// Compiled but failed at runtime (semantic bug)
    RuntimeFail { expected: String, actual: String },

    /// Doc test failed to compile (type/syntax error)
    CompileFail { error_code: String, error_message: String },

    /// Could not transpile the doctest expression
    TranspileFail { python_input: String, error: String },
}
```

Each result type generates a training signal:
- **Pass** → Positive example for Oracle success
- **RuntimeFail** → Semantic bug in transpiler logic
- **CompileFail** → Type/syntax error for Oracle classification
- **TranspileFail** → Unsupported construct (gap identification)

### Corpus Size Estimates

| Source | Estimated Doctests |
|--------|-------------------|
| Python stdlib | 5,000+ |
| NumPy | 3,000+ |
| Pandas | 5,000+ |
| **Total** | **13,500+** |

### Integration with Oracle Training

```rust
use depyler_oracle::{DoctestResult, TrainingSignal};

fn doctest_to_training_signal(result: &DoctestResult) -> TrainingSignal {
    match result {
        DoctestResult::Pass { .. } => TrainingSignal::positive(),
        DoctestResult::RuntimeFail { .. } => TrainingSignal::semantic_bug(),
        DoctestResult::CompileFail { error_code, .. } => {
            TrainingSignal::from_rustc_error(error_code)
        }
        DoctestResult::TranspileFail { .. } => TrainingSignal::unsupported(),
    }
}
```

> **See also**: `docs/specifications/doctest-transpilation-citl-spec.md` for full implementation details.

---

## 80% Single-Shot Compile Rate: Acceleration Strategies

To achieve 80% single-shot compilation success (up from ~24%), the Oracle implements 5 acceleration strategies using ML and fault localization techniques.

### Strategy #1: Tarantula-Guided Codegen Hotfixes (DEPYLER-0631)

**Fault localization** using spectrum-based suspiciousness scoring from the `tarantula` crate.

```rust
use depyler_oracle::{TarantulaIntegration, TarantulaConfig, TarantulaCorpus};

// Configure fault localization
let config = TarantulaConfig {
    suspiciousness_threshold: 0.5,
    min_coverage: 0.1,
    max_suspects: 10,
    ..Default::default()
};

let mut integration = TarantulaIntegration::new(config);

// Register test results
integration.register_pass("test_type_inference", &covered_lines);
integration.register_fail("test_borrow_check", &covered_lines, "E0382");

// Get suspicious locations
let suspects = integration.get_suspects();
for suspect in suspects {
    println!("Line {}: suspiciousness={:.2}", suspect.line, suspect.suspiciousness);
}
```

**Components**:
- `TarantulaIntegration`: Core fault localization engine
- `TarantulaBridge`: Connects to external tarantula crate
- `TarantulaCorpus`: Manages test coverage data with deduplication

### Strategy #2: CITL Error-Pattern Library (DEPYLER-0632)

**Compiler-In-The-Loop** pattern matching for error classification.

```rust
use depyler_oracle::{ErrorPatternLibrary, ErrorPattern, PatternMatch};

// Create library
let mut library = ErrorPatternLibrary::new()?;

// Add patterns
let pattern = ErrorPattern::new(
    "E0308",
    "type_mismatch_i32_string",
    r"expected `i32`, found `(String|&str)`",
    "Use .parse::<i32>() for String→i32 conversion",
    0.85,
);
library.add_pattern(pattern)?;

// Match errors
let matches = library.match_error("E0308", "expected `i32`, found `String`");
for m in matches {
    println!("Pattern: {} (similarity={:.2})", m.pattern_id, m.similarity);
    println!("Fix: {}", m.suggested_fix);
}

// Persistence
library.save_json(Path::new("patterns.json"))?;
library.save_binary(Path::new("patterns.bin"))?;
```

**Features**:
- Levenshtein similarity scoring
- Category-based filtering
- JSON/binary serialization
- Pattern normalization

### Strategy #3: Curriculum Learning (DEPYLER-0633)

**Progressive difficulty ordering** for transpilation training.

```rust
use depyler_oracle::{CurriculumScheduler, CurriculumConfig, LearningItem};

let config = CurriculumConfig {
    initial_difficulty: 0.2,
    difficulty_increment: 0.1,
    success_threshold: 0.8,
    window_size: 10,
    ..Default::default()
};

let mut scheduler = CurriculumScheduler::new(config);

// Add items with complexity scores
scheduler.add_item(LearningItem {
    id: "simple_add".to_string(),
    complexity: 0.1,   // Very simple
    features: vec!["arithmetic".to_string()],
    ..Default::default()
});

scheduler.add_item(LearningItem {
    id: "generic_trait".to_string(),
    complexity: 0.8,   // Complex
    features: vec!["generics".to_string(), "traits".to_string()],
    ..Default::default()
});

// Get next batch (ordered by difficulty)
let batch = scheduler.next_batch(5);

// Record results
scheduler.record_success("simple_add");
scheduler.record_failure("generic_trait", "E0277");

// Scheduler adapts difficulty based on success rate
```

**Complexity Scoring**:
| Feature | Weight |
|---------|--------|
| Nested loops | +0.15 |
| Generic types | +0.20 |
| Async/await | +0.25 |
| Trait bounds | +0.20 |
| Lifetime annotations | +0.30 |

### Strategy #4: Knowledge Distillation (DEPYLER-0634)

**LLM-to-Oracle knowledge transfer** using temperature-scaled soft targets.

```rust
use depyler_oracle::{KnowledgeDistiller, DistillationConfig, LlmFixExample};

let config = DistillationConfig {
    temperature: 3.0,          // Soft target temperature
    alpha: 0.7,                // Balance hard/soft loss
    min_confidence: 0.8,       // Minimum LLM confidence
    promotion_threshold: 10,   // Applications before promotion
    max_patterns: 1000,
};

let mut distiller = KnowledgeDistiller::new(config);

// Collect LLM fix examples
distiller.collect_example(LlmFixExample {
    error_code: "E0308".to_string(),
    error_message: "expected i32, found String".to_string(),
    original_code: "let x: i32 = s;".to_string(),
    fixed_code: "let x: i32 = s.parse().unwrap();".to_string(),
    diff: "+.parse().unwrap()".to_string(),
    explanation: Some("String needs parsing".to_string()),
    llm_confidence: 0.92,
    validated: true,
});

// Extract patterns from examples
let patterns = distiller.extract_patterns();

// Get promotion candidates (high success rate)
let candidates = distiller.get_promotion_candidates();

// Export to pattern library
let mut library = ErrorPatternLibrary::new()?;
let promoted = distiller.export_to_library(&mut library);
println!("Promoted {} patterns to library", promoted);

// Soft classification (temperature-scaled)
let soft_labels = distiller.classify_soft("E0308", "expected i32");
for (category, prob) in soft_labels {
    println!("{:?}: {:.2}%", category, prob * 100.0);
}
```

**Integration with `entrenar`**:
```rust
use entrenar::distill::{soft_target_loss, DistillationConfig as EntrenarConfig};

// Temperature-scaled KL divergence loss
let loss = soft_target_loss(&teacher_logits, &student_logits, temperature);
```

### Strategy #5: GNN Error Encoder (DEPYLER-0635)

**Graph Neural Network** for structural error pattern matching based on [Yasunaga & Liang 2020](https://arxiv.org/abs/2005.10636).

```rust
use depyler_oracle::{DepylerGnnEncoder, GnnEncoderConfig, ErrorPattern};

let config = GnnEncoderConfig {
    hidden_dim: 64,
    output_dim: 256,
    similarity_threshold: 0.7,
    max_similar: 5,
    use_hnsw: true,  // Hierarchical Navigable Small World index
};

let mut encoder = DepylerGnnEncoder::new(config)?;

// Index patterns from library
let pattern = ErrorPattern::new("E0308", "type_mismatch", "...", "...", 0.9);
encoder.index_pattern(&pattern, "fn foo(x: i32) { let s: String = x; }");

// Find similar patterns for new error
let similar = encoder.find_similar(
    "E0308",
    "expected String, found i32",
    "fn bar(y: i32) { let t: String = y; }"
);

for s in similar {
    println!("Pattern: {} (similarity={:.2})", s.pattern_id, s.similarity);
    println!("Success rate: {:.1}%", s.success_rate * 100.0);
}

// Raw embedding for custom similarity search
let embedding = encoder.encode_error("E0308", "type mismatch", "source code");
```

**Integration with `aprender`**:
```rust
use aprender::citl::{GNNErrorEncoder, ProgramFeedbackGraph, NodeType};

// Build program-error graph
let graph = ProgramFeedbackGraph::new();
graph.add_node(NodeType::Error, "E0308");
graph.add_node(NodeType::Location, "line 42");
graph.add_edge(0, 1, "at");

// Encode with GNN
let encoder = GNNErrorEncoder::new(hidden_dim, output_dim);
let embedding = encoder.encode(&graph);
```

### Combined Pipeline

All 5 strategies work together:

```
Python Source
     │
     ▼
┌─────────────┐
│  Transpile  │
└─────────────┘
     │
     ▼
┌─────────────┐
│   rustc     │───── Success ─────► Done (80% target)
└─────────────┘
     │
   Error
     │
     ├──► Strategy #2: Pattern Library Match
     │         │
     │         ├──► Found → Apply fix template
     │         │
     │         └──► Not found ──┐
     │                          │
     ├──► Strategy #5: GNN Similar Search ◄─┘
     │         │
     │         └──► Find structurally similar fixes
     │
     ├──► Strategy #3: Curriculum Adjustment
     │         │
     │         └──► Record failure, adjust difficulty
     │
     ├──► Strategy #1: Tarantula Localization
     │         │
     │         └──► Identify suspicious transpiler lines
     │
     └──► Strategy #4: LLM Distillation (fallback)
               │
               └──► Query LLM, extract pattern, promote if validated
```

### Configuration

```rust
use depyler_oracle::{
    OracleConfig, TarantulaConfig, CurriculumConfig,
    DistillationConfig, GnnEncoderConfig
};

let oracle_config = OracleConfig {
    tarantula: TarantulaConfig {
        suspiciousness_threshold: 0.5,
        ..Default::default()
    },
    curriculum: CurriculumConfig {
        initial_difficulty: 0.2,
        success_threshold: 0.8,
        ..Default::default()
    },
    distillation: DistillationConfig {
        temperature: 3.0,
        promotion_threshold: 10,
        ..Default::default()
    },
    gnn: GnnEncoderConfig {
        similarity_threshold: 0.7,
        ..Default::default()
    },
    ..Default::default()
};
```

### Expected Impact

| Metric | Before | Target | Mechanism |
|--------|--------|--------|-----------|
| Single-shot compile | 24% | 80% | All strategies |
| Error classification | 70% | 95% | GNN + Pattern Library |
| Fix suggestion accuracy | 50% | 85% | Distillation + CITL |
| Debug cycle time | 5 min | 30 sec | Tarantula localization |

---

## Summary

The Oracle is a compile-error classification system that:
- Classifies errors into actionable categories
- Suggests fixes based on learned patterns
- Supports custom training for bespoke codebases
- Detects model drift for retraining triggers
- **Uses doctest transpilation for highest-fidelity training signals**
- **Implements 5 acceleration strategies for 80% single-shot compile rate**

It's internal infrastructure that helps maintainers fix transpiler bugs faster by providing structured feedback on error patterns.

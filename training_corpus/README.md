# Depyler Oracle Training Corpus

This directory contains training data for the Depyler Oracle ML model.

## Files

- `errors.jsonl` - Real compilation errors from reprorusted-python-cli (generated)
- `oip_data.json` - OIP extracted training data (generated)
- `.gitignore` - Excludes generated files from git

## Usage

### Extract Training Data
```bash
# Extract real errors from reprorusted-python-cli examples
./scripts/extract_training_data.sh

# Extract OIP data (optional)
./scripts/extract_oip_training_data.sh
```

### Train Oracle Model
```bash
# Full training with all data sources
make train-oracle

# Quick training (synthetic only, for testing)
make train-oracle-fast

# Extract data only (no training)
make oracle-extract
```

## Data Format

### errors.jsonl
```json
{"error_code": "E0308", "message": "mismatched types", "context": "let x: i32 = \"hello\"", "file": "example.py", "hash": "abc123"}
```

### oip_data.json
```json
[
  {"error_type": "TypeMismatch", "message": "...", "fix": "..."}
]
```

## Pipeline

```
reprorusted-python-cli/examples/*.py
           ↓
   depyler transpile
           ↓
   cargo check (errors)
           ↓
   errors.jsonl
           ↓
   train_unified_corpus
           ↓
   depyler_oracle.apr
```

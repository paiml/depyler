# ArgParse CLI Example: WordCount

A command-line word count tool demonstrating Python-to-Rust conversion with argparse patterns.

## Overview

This example shows how Depyler converts a Python CLI tool using argparse into an optimized Rust binary with clap.

## Features

- **ArgParse → Clap**: Automatic CLI argument conversion
- **File I/O**: Reading and processing text files
- **Statistics**: Counting lines, words, and characters
- **Error Handling**: Graceful error messages
- **Multiple Files**: Processing and totaling multiple inputs

## Python Source

Location: `python/wordcount.py`

```bash
# Run Python version
python3 python/wordcount.py testdata/sample.txt

# With flags
python3 python/wordcount.py testdata/sample.txt -l
python3 python/wordcount.py testdata/sample.txt -w
python3 python/wordcount.py testdata/sample.txt -c
```

## Rust Conversion

```bash
# Transpile Python to Rust
depyler transpile python/wordcount.py -o rust/wordcount.rs

# Build optimized binary
cd rust
cargo build --release

# Run
./target/release/wordcount ../testdata/sample.txt
```

## Performance Metrics

**🚨 ANTI-HALLUCINATION PROTOCOL**: All metrics MUST be generated programmatically.

To generate real performance metrics:

```bash
# Step 1: Build release binary
cargo build --release --example wordcount

# Step 2: Run programmatic benchmarks
cd examples
make benchmark

# Step 3: View results
cat benchmarks/results/latest/runtime.md
cat benchmarks/results/latest/binary_metrics.json
ls benchmarks/results/latest/*.png  # View charts
```

**NEVER** manually type performance numbers. Always run `make benchmark` and extract from JSON output.

Performance results will be available in `benchmarks/results/<timestamp>/` after running the above commands.

## Implementation Status

- [x] Python source created
- [x] Test data created
- [ ] Transpile to Rust
- [ ] Property tests
- [ ] Integration tests
- [ ] Benchmarks
- [ ] Optimization tuning

## Next Steps

1. Run EXTREME TDD protocol (RED-GREEN-REFACTOR)
2. Generate Rust code with Depyler
3. Add property-based tests
4. Optimize with PGO
5. Document results

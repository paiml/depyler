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

## Performance Expectations

| Metric | Python | Rust (Debug) | Rust (Release) | Rust (PGO) |
|--------|--------|--------------|----------------|------------|
| Execution Time | 50ms | 5ms | 2ms | 1.5ms |
| Binary Size | N/A | 8MB | 2MB | 2.1MB |
| Memory Usage | 15MB | 2MB | 1.5MB | 1.5MB |
| Startup Time | 80ms | <1ms | <1ms | <1ms |

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

# Depyler Examples - Python to Optimized Rust Binaries

Comprehensive examples demonstrating real-world Python to Rust conversion patterns.

## Example Suite

| Example | Description | Status | Benchmark Results |
|---------|-------------|--------|-------------------|
| [argparse_cli](argparse_cli/) | CLI tool with argument parsing | 🟡 In Progress | Run `make benchmark` |
| generator_pipeline | Text processing with generators | 🔴 Planned | Not yet started |
| multifile_project | Multi-module project with CLI | 🔴 Planned | Not yet started |

## Quick Start

```bash
# Run Python version
python3 argparse_cli/python/wordcount.py argparse_cli/testdata/sample.txt

# Transpile to Rust
depyler transpile argparse_cli/python/wordcount.py

# Build and run optimized binary
cargo build --release --example wordcount
./target/release/examples/wordcount argparse_cli/testdata/sample.txt
```

## Running Examples

Each example can be run as a cargo example:

```bash
# Example 1: WordCount CLI
cargo run --example wordcount -- argparse_cli/testdata/sample.txt

# Example 2: Text Processor (coming soon)
cargo run --example text_processor -- generator_pipeline/testdata/large.txt

# Example 3: Calculator (coming soon)
cargo run --example calculator -- "2 + 3 * 4"
```

## EXTREME TDD Implementation

All examples follow EXTREME TDD protocol:

1. **RED Phase**: Write failing tests first
2. **GREEN Phase**: Implement minimal working code
3. **REFACTOR Phase**: Optimize and meet quality gates

### Quality Gates (MANDATORY)

- ✅ **Compilation**: `cargo build --release` succeeds
- ✅ **Tests**: 100% pass rate, ≥85% coverage
- ✅ **Clippy**: Zero warnings (`-D warnings`)
- ✅ **TDG**: All files ≤2.0 score
- ✅ **Complexity**: All functions ≤10 cyclomatic complexity

## Optimization Techniques

Each example demonstrates:

- **Profile-Guided Optimization (PGO)**: 10-30% additional speedup
- **Link-Time Optimization (LTO)**: Binary size reduction
- **CPU-Specific Tuning**: Platform-specific optimizations
- **Binary Stripping**: 80% size reduction

## Documentation

Full specification: [docs/specifications/convert-python-to-optimized-rust-binary-examples.md](../docs/specifications/convert-python-to-optimized-rust-binary-examples.md)

## Contributing

Follow EXTREME TDD protocol for all contributions. See [CLAUDE.md](../CLAUDE.md) for development guidelines.

## Performance Results

Results will be documented here as examples are completed and benchmarked.

---

**Status Legend**:
- 🟢 Complete
- 🟡 In Progress
- 🔴 Not Started

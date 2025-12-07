# Depyler Examples - Python to Optimized Rust Binaries

Comprehensive examples demonstrating real-world Python to Rust conversion patterns.

## Quick Start - Run All Examples

```bash
# List all available examples
cargo run --example 2>&1 | head -50

# Run a specific example (from project root)
cargo run --example fibonacci
cargo run --example quicksort
cargo run --example basic_lambda
```

## Example Categories

### Algorithm Examples
```bash
cargo run --example fibonacci       # Fibonacci sequence
cargo run --example quicksort       # QuickSort implementation
```

### Data Structure Examples
```bash
cargo run --example basic_class_test     # Basic OOP patterns
cargo run --example dict_assign          # Dictionary operations
cargo run --example set_operations       # Set operations
```

### Functional Programming
```bash
cargo run --example basic_lambda         # Lambda expressions
cargo run --example lambda_demo          # Advanced lambdas
cargo run --example lambda_test          # Lambda with collections
```

### Standard Library Usage
```bash
cargo run --example stdlib_comprehensive_test   # Comprehensive stdlib
cargo run --example stdlib_string_methods_test  # String methods
cargo run --example test_math_module            # Math operations
cargo run --example test_datetime_module        # Date/time handling
cargo run --example test_json_parsing           # JSON processing
```

### CLI Tools
```bash
# WordCount CLI with argparse
cargo run --example wordcount -- examples/argparse_cli/testdata/sample.txt
```

## Example Suite Status

| Example | Description | Status | Benchmark Results |
|---------|-------------|--------|-------------------|
| [argparse_cli](argparse_cli/) | CLI tool with argument parsing | 🟡 In Progress | Run `make benchmark` |
| algorithms/ | Sorting, searching algorithms | 🟢 Complete | See examples |
| data_processing/ | List/dict operations | 🟢 Complete | See examples |

## Transpilation Workflow

```bash
# 1. Run Python version
python3 argparse_cli/python/wordcount.py argparse_cli/testdata/sample.txt

# 2. Transpile to Rust
depyler transpile argparse_cli/python/wordcount.py

# 3. Build optimized binary
cargo build --release --example wordcount

# 4. Run the binary
./target/release/examples/wordcount argparse_cli/testdata/sample.txt
```

## Building All Examples

```bash
# Build all examples in release mode
cargo build --release --examples

# List built binaries
ls -la target/release/examples/
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

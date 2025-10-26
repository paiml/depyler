# Depyler

[![Crates.io](https://img.shields.io/crates/v/depyler.svg)](https://crates.io/crates/depyler)
[![Documentation](https://docs.rs/depyler/badge.svg)](https://docs.rs/depyler)
[![CI](https://github.com/paiml/depyler/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/depyler/actions/workflows/ci.yml)
[![TDD Book](https://github.com/paiml/depyler/actions/workflows/book.yml/badge.svg)](https://github.com/paiml/depyler/actions/workflows/book.yml)
[![Stdlib Validation](https://img.shields.io/badge/stdlib%20validation-27%20modules%20%7C%20151%20tests-brightgreen.svg)](tdd-book/VALIDATION-FINAL-2025-10-26.md)
[![Coverage](https://img.shields.io/badge/coverage-70.16%25-yellow.svg)](https://codecov.io/gh/paiml/depyler)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Apache License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust 1.83+](https://img.shields.io/badge/rust-1.83+-orange.svg)](https://www.rust-lang.org)

A Python-to-Rust transpiler with semantic verification and memory safety analysis. Depyler translates annotated Python code into idiomatic Rust, preserving program semantics while providing compile-time safety guarantees.

## 🎉 Current Release: v3.19.14 - 100% Stdlib Collection Coverage!

**Major Milestone Achieved** - Complete coverage of Python stdlib collection methods:

### What's New in v3.19.14

**Stdlib Coverage: 100% (40/40 methods)**
- ✅ **List methods** (11/11): append, extend, insert, remove, pop, clear, index, count, sort, reverse, copy
- ✅ **Dict methods** (10/10): get, keys, values, items, pop, clear, update, setdefault, popitem, copy
- ✅ **Set methods** (8/8): add, remove, discard, pop, clear, union, intersection, difference
- ✅ **String methods** (11/11): upper, lower, strip, startswith, endswith, split, join, find, replace, count, isdigit, isalpha

**Bugs Fixed (4)**
- DEPYLER-0222: dict.get() without default value
- DEPYLER-0223: dict.update() and set.update() routing
- DEPYLER-0225: str.split(sep) Pattern trait error
- DEPYLER-0226: str.count() routing disambiguation

**Quality Metrics**
- Tests: 443/443 passing (100%)
- Clippy: Zero warnings
- Coverage: 80%+
- Zero regressions

**Installation**
```bash
cargo install depyler
```

See [CHANGELOG.md](CHANGELOG.md) for complete details and [GitHub Release](https://github.com/paiml/depyler/releases/tag/v3.19.14).

## Installation

```bash
cargo install depyler
```

### Requirements

- Rust 1.83.0 or later
- Python 3.8+ (for test validation)

## Usage

### Basic Transpilation

```bash
# Transpile a Python file to Rust
depyler transpile example.py

# Transpile with semantic verification
depyler transpile example.py --verify

# Analyze migration complexity
depyler analyze example.py
```

### Example

**Input** (`example.py`):
```python
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

**Output** (`example.rs`):
```rust
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

### Library Usage

```rust
use depyler::{transpile_file, TranspileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = TranspileOptions::default()
        .with_verification(true);

    let rust_code = transpile_file("example.py", options)?;
    println!("{}", rust_code);

    Ok(())
}
```

## Features

### Core Capabilities

- **Type-directed transpilation**: Uses Python type annotations to generate appropriate Rust types
- **Memory safety analysis**: Infers ownership and borrowing patterns
- **Semantic verification**: Property-based testing to verify behavioral equivalence
- **Multiple backends**: Generate Rust or Ruchy script code

### Supported Python Features

**Currently Supported:**
- Functions with type annotations
- Basic types (int, float, str, bool)
- Collections (List, Dict, Tuple, Set)
- Control flow (if, while, for, match)
- List/dict/set comprehensions
- **Generator expressions** (NEW in v3.13.0) ✨
- **Generator functions** (yield statements)
- Exception handling (mapped to Result<T, E>)
- Classes and methods
- **Assert statements** (NEW in v3.18.2) ✨
- Async/await (functions and methods - FIXED in v3.18.2)
- Context managers (with statements)
- Iterators
- **Print statements** (correctly generates println! macro)

**Not Supported:**
- Dynamic features (eval, exec)
- Runtime reflection
- Multiple inheritance
- Monkey patching

See [documentation](https://docs.rs/depyler) for complete feature list.

## Python Stdlib Module Support

**Production-Ready Status**: 100% TDD Book validation complete (27/27 modules, 151/151 tests passing)

Depyler provides comprehensive support for Python standard library modules, validated through systematic testing. All listed modules have been verified to transpile correctly and generate compilable, semantically equivalent Rust code.

### Validation Results

**Modules Validated**: 27/27 (100%)
**Total Tests**: 151/151 (100% pass rate)
**Status**: Production-ready for validated modules
**Validation Date**: 2025-10-26

### Supported Modules by Category

#### Data Serialization & Encoding
- **json** (6/6 tests) - Serialization/deserialization, loads, dumps, roundtrip
- **struct** (6/6 tests) - Binary data packing/unpacking (format codes: 'i', 'ii')
- **base64** (6/6 tests) - Base64 encoding/decoding, urlsafe variants
- **csv** (6/6 tests) - CSV file handling, reader, writer, DictReader/Writer

#### Date, Time & Calendar
- **datetime** (6/6 tests) - Date/time operations, parsing, formatting
- **calendar** (5/5 tests) - Calendar functions (weekday, isleap, monthrange)
- **time** (5/5 tests) - Time operations (sleep, perf_counter, monotonic)

#### Cryptography & Security
- **hashlib** (6/6 tests) - Cryptographic hash functions (MD5, SHA1, SHA256, SHA512)
- **secrets** (6/6 tests) - Cryptographically secure random number generation

#### Text Processing
- **textwrap** (6/6 tests) - Text wrapping and formatting operations
- **re** (6/6 tests) - Regular expression operations, pattern matching
- **string** (6/6 tests) - String manipulation (case, trim, split, search, replace)

#### Mathematics & Statistics
- **math** (6/6 tests) - Mathematical functions (arithmetic, trigonometric, hyperbolic)
- **decimal** (5/5 tests) - Decimal floating-point arithmetic with precision control
- **fractions** (5/5 tests) - Rational number arithmetic
- **statistics** (6/6 tests) - Statistical functions (mean, median, mode, stdev, variance)

#### File System & I/O
- **os** (5/5 tests) - OS interface (getcwd, listdir, path operations, getenv)
- **pathlib** (6/6 tests) - Object-oriented filesystem paths
- **io** (5/5 tests) - Core I/O operations (StringIO, BytesIO)

#### Data Structures & Algorithms
- **collections** (4/4 tests) - Specialized container datatypes
- **copy** (6/6 tests) - Shallow and deep copy operations
- **memoryview** (6/6 tests) - Memory view objects for efficient array operations
- **array** (6/6 tests) - Efficient arrays of numeric values

#### Functional Programming
- **itertools** (6/6 tests) - Functions for efficient looping (chain, islice, repeat, count)
- **functools** (4/4 tests) - Higher-order functions (reduce, partial, lru_cache)

#### Random Number Generation
- **random** (5/5 tests) - Pseudo-random number generators (uniform, shuffle, sample, seed)

#### System & Runtime
- **sys** (6/6 tests) - System-specific parameters and functions

### Quality Assurance

All validated modules passed comprehensive testing including:
- **Transpilation**: Python code successfully converted to Rust
- **Compilation**: Generated Rust code compiles with rustc
- **Semantic Equivalence**: Behavior matches original Python code
- **Edge Cases**: Boundary conditions and error handling verified

### Validation Methodology

The validation campaign followed strict TDD protocols:
1. Each module tested with 4-6 comprehensive test cases
2. All tests use formal verification (`--verify` flag)
3. Generated code must compile with zero warnings
4. Zero regressions in core transpiler tests (87/87 passing)
5. Quality gates: A- TDG grade, complexity ≤10, zero SATD

### Bug Discovery & Resolution

**Session 1** (8 modules): 4 critical bugs discovered and fixed
- DEPYLER-0021: struct module implementation (P0)
- DEPYLER-0022: memoryview/bytes literal support (P0)
- DEPYLER-0023: Rust keyword collision fix (P1)
- DEPYLER-0024: copy.copy validation (P1 - already fixed)

**Session 2** (19 modules): 0 bugs discovered (exceptional quality indicator)

The dramatic difference in bug discovery rate (50% → 0%) demonstrates transpiler maturity and excellent pattern coverage.

### Usage Notes

For applications using these validated stdlib modules, Depyler is considered **production-ready**. The transpiler generates idiomatic, safe Rust code with verified semantic equivalence to the original Python.

For the complete validation report, see [tdd-book/VALIDATION-FINAL-2025-10-26.md](tdd-book/VALIDATION-FINAL-2025-10-26.md).

## MCP Integration

Depyler provides an MCP (Model Context Protocol) server for integration with AI assistants like Claude Code.

### Setup

Add to Claude Desktop config (`~/.config/Claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "depyler": {
      "command": "depyler",
      "args": ["agent", "start", "--foreground", "--port", "3000"]
    }
  }
}
```

### Available Tools

- `transpile_python` - Convert Python code to Rust
- `analyze_migration_complexity` - Analyze migration effort
- `verify_transpilation` - Verify semantic equivalence
- `pmat_quality_check` - Code quality analysis

See [docs/MCP_QUICKSTART.md](docs/MCP_QUICKSTART.md) for detailed usage.

## Architecture

Depyler uses a multi-stage compilation pipeline:

```
Python AST → HIR → Type Inference → Rust AST → Code Generation
```

Key components:
- **Parser**: RustPython AST parser
- **HIR**: High-level intermediate representation
- **Type System**: Conservative type inference with annotation support
- **Verification**: Property-based testing for semantic equivalence
- **Codegen**: Rust code generation via syn/quote

## Project Status & Roadmap

**Current Version**: v3.19.14
**Status**: Production Ready - 100% stdlib collection coverage achieved

### Roadmap Highlights

**✅ Completed (v3.19.14)**
- 100% stdlib collection methods (list, dict, set, string)
- Zero P0 blocking bugs
- Complete release cycle (GitHub + crates.io)
- Idiomatic Rust code generation

**🎯 Next Priorities**
- Advanced stdlib methods (dict.copy, set.issubset, etc.)
- Type tracking for set.remove() with variables
- Performance optimizations
- Error message improvements

See [docs/execution/roadmap.yaml](docs/execution/roadmap.yaml) for detailed tracking.

## Quality Standards

This project follows strict quality standards enforced by CI:
- Test coverage: 80%+ (443 passing tests in core, 600+ workspace-wide)
- Max cyclomatic complexity: ≤10 (enforced via PMAT)
- Max cognitive complexity: ≤10 (enforced via PMAT)
- Zero clippy warnings (`-D warnings` - BLOCKING)
- Zero self-admitted technical debt (SATD - BLOCKING)
- TDG grade: A- minimum (≥85 points)
- CI validates all transpiled code compiles

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run with coverage
cargo llvm-cov --html --open

# Run benchmarks
cargo bench
```

### Quality Checks

```bash
# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Quality gates
pmat quality-gate
```

## Documentation

- [API Documentation](https://docs.rs/depyler)
- [MCP Quickstart](docs/MCP_QUICKSTART.md)
- [Agent Mode Guide](AGENT.md)
- [Architecture](ARCHITECTURE.md)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome. Please follow the quality standards:

1. Write tests first (TDD)
2. Maintain 80%+ coverage for new code
3. Pass all clippy checks
4. Update documentation

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

# Depyler

[![Crates.io](https://img.shields.io/crates/v/depyler.svg)](https://crates.io/crates/depyler)
[![Documentation](https://docs.rs/depyler/badge.svg)](https://docs.rs/depyler)
[![CI](https://github.com/paiml/depyler/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/depyler/actions/workflows/ci.yml)
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

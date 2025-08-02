# Depyler

[![Crates.io](https://img.shields.io/crates/v/depyler.svg)](https://crates.io/crates/depyler)
[![Documentation](https://docs.rs/depyler/badge.svg)](https://docs.rs/depyler)
[![CI](https://github.com/paiml/depyler/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/depyler/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-85%25+-brightgreen.svg)](https://codecov.io/gh/paiml/depyler)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Apache License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust 1.83+](https://img.shields.io/badge/rust-1.83+-orange.svg)](https://www.rust-lang.org)
[![MCP Compatible](https://img.shields.io/badge/MCP-Compatible-green)](https://modelcontextprotocol.io)
[![Downloads](https://img.shields.io/crates/d/depyler)](https://crates.io/crates/depyler)

**Energy-efficient Python-to-Rust transpiler** with progressive verification capabilities. Transform Python code into safe, performant Rust while reducing energy consumption by 75-85%. Built with zero tolerance for technical debt and extreme quality standards following the Toyota Way.

> **Toyota Way Success**: Achieved 100% SATD elimination, 0 incomplete implementations, and comprehensive test coverage. Project maintains zero defects policy with property-based testing, formal verification readiness, and ownership inference. Latest v1.0.6 release adds basic class support and floor division operator handling.

## 🚀 Installation

Install `depyler` using one of the following methods:

- **From Crates.io (Recommended):**
  ```bash
  cargo install depyler
  ```

- **From Source:**
  ```bash
  git clone https://github.com/paiml/depyler
  cd depyler
  cargo build --release
  cargo install --path crates/depyler
  ```

- **From GitHub Releases:**
  Pre-built binaries are available on the [releases page](https://github.com/paiml/depyler/releases).

### Requirements
- **Rust:** 1.83.0 or later
- **Python:** 3.8+ (for test validation)

## 🚀 Getting Started

### Quick Start

```bash
# Transpile a Python file
depyler transpile example.py

# Transpile with verification
depyler transpile example.py --verify

# Analyze code complexity before transpilation
depyler analyze example.py

# Interactive mode with AI suggestions
depyler interactive example.py --suggest

# Check transpilation compatibility
depyler check example.py

# Inspect AST/HIR representations
depyler inspect example.py --repr ast
```

### Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
depyler = "1.0.2"
```

Basic usage:

```rust
use depyler::{transpile_file, TranspileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = TranspileOptions::default()
        .with_verification(true)
        .with_optimization_level(2);
    
    let rust_code = transpile_file("example.py", options)?;
    println!("{}", rust_code);
    
    Ok(())
}
```

## Key Features

### 🔄 Core Transpilation
- **Python AST to HIR** - High-level intermediate representation for safe transformations
- **Type Inference** - Smart type analysis with annotation support
- **Memory Safety** - Automatic ownership and borrowing inference
- **Direct Rules Engine** - Pattern-based Python-to-Rust transformations
- **String Optimization** - Interning for frequently used literals, Cow<str> for flexible ownership

### ⚡ Performance & Efficiency
- **Energy Reduction** - 75-85% lower energy consumption vs Python
- **Binary Optimization** - Compile with LTO, strip, and `panic=abort`
- **Zero-Copy Strings** - Smart string allocation strategies with Cow<str>
- **LLVM Backend** - Leverages Rust's optimizing compiler
- **String Interning** - Automatic interning for strings used >3 times

### 🛡️ Safety & Verification
- **Property-Based Testing** - QuickCheck for semantic equivalence
- **Memory Safety Analysis** - Prevents use-after-free and data races
- **Bounds Checking** - Automatic insertion where needed
- **Contract Verification** - Pre/post condition checking
- **Formal Verification Ready** - Structured for future SMT integration

### 🤖 AI Integration
- **Model Context Protocol** - Full MCP v1.0 support
- **Interactive Mode** - AI-powered transpilation assistance
- **Annotation Suggestions** - Smart optimization hints
- **Complexity Analysis** - Migration difficulty assessment

### 🎯 Supported Python Features

#### ✅ Production Ready
- Functions with type annotations
- Basic types (`int`, `float`, `str`, `bool`)
- Collections (`List`, `Dict`, `Tuple`, `Set`)
- Control flow (`if`, `while`, `for`, `match`)
- List and dict comprehensions
- Exception handling → `Result<T, E>`
- Basic classes (struct conversion, __init__ → new())
- Pattern matching (Python 3.10+)
- Floor division operator (//) with proper Python semantics
- String optimization (interning, Cow<str> support)
- Protocol to Trait mapping
- Const generic array inference

#### 🚧 In Development
- Async/await support
- Generator expressions
- Lambda functions
- Class inheritance
- Decorators

#### ❌ Not Supported
- Dynamic features (`eval`, `exec`)
- Runtime reflection
- Monkey patching
- Multiple inheritance

## 📋 Tool Usage

### CLI Interface

```bash
# Basic transpilation
depyler transpile input.py              # Creates input.rs
depyler transpile input.py -o output.rs # Custom output
depyler transpile src/ -o rust/         # Directory mode

# Analysis and verification
depyler check input.py                  # Compatibility check
depyler analyze input.py                # Complexity metrics
depyler verify output.rs                # Verify generated code
depyler inspect input.py --repr ast     # View AST/HIR

# Interactive features
depyler interactive input.py            # Interactive session
depyler interactive input.py --suggest  # With AI suggestions

# Quality enforcement
depyler transpile input.py --verify --strict  # Full verification
depyler quality-check input.py               # Toyota Way scoring
```

### MCP Integration

#### Using with Claude Code

```bash
# Add to Claude Code
claude mcp add depyler ~/.local/bin/depyler
```

#### MCP Server Mode

```bash
# Start MCP server
depyler serve --mcp

# Available MCP tools:
# - transpile_python: Convert Python to Rust
# - analyze_complexity: Code complexity analysis
# - verify_transpilation: Verify semantic equivalence
# - suggest_annotations: Optimization hints
```

### HTTP API

```bash
# Start HTTP server
depyler serve --port 8080 --cors

# API endpoints
curl "http://localhost:8080/health"
curl "http://localhost:8080/api/v1/transpile" \
  -H "Content-Type: application/json" \
  -d '{"code":"def add(a: int, b: int) -> int: return a + b"}'
```

## 🏗️ Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Python AST    │────▶│      HIR        │────▶│   Rust AST      │
│  (rustpython)   │     │  (Intermediate) │     │     (syn)       │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                       │                        │
         ▼                       ▼                        ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│ Type Inference  │     │  Optimizations  │     │ Code Generation │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## 🚦 CI/CD Integration

### GitHub Actions Example
```yaml
name: Transpile and Verify
on: [push, pull_request]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Install depyler
        run: cargo install depyler
        
      - name: Check Python compatibility
        run: depyler check src/**/*.py
        
      - name: Transpile to Rust
        run: depyler transpile src/ -o rust/ --verify
        
      - name: Run quality checks
        run: depyler quality-check src/**/*.py --strict
```

## Toyota Way Quality Standards

This project exemplifies the Toyota Way philosophy through disciplined quality practices:

### 自働化 (Jidoka) - Build Quality In
- **ZERO SATD**: No TODO, FIXME, HACK, or placeholder implementations
- **ZERO Incomplete**: All features fully implemented with unreachable!() removed
- **ZERO High Complexity**: No function exceeds cyclomatic complexity of 20
- **100% Verification**: All generated code must compile and pass tests

### 現地現物 (Genchi Genbutsu) - Go and See
- **Real-World Testing**: Validated against actual Python codebases
- **Performance Profiling**: Energy consumption measured on real hardware
- **Direct Debugging**: Debug at the generated Rust level, not just HIR

### 改善 (Kaizen) - Continuous Improvement
- **v1.0.1**: Fixed all SATD markers and incomplete implementations
- **v1.0.2**: Enhanced string optimization with interning and Cow<str>
- **Next**: Lifetime analysis enhancements for better borrowing inference

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run with coverage
cargo tarpaulin --out Html

# Run property tests
cargo test --features quickcheck

# Run benchmarks
cargo bench

# Run specific test suites
cargo test -p depyler-core        # Core transpilation
cargo test -p depyler-verify      # Verification
cargo test -p depyler-mcp         # MCP integration
```

## Recent Updates

### 🚀 v1.0.2 - String Optimization Excellence
- **String Interning**: Automatically intern strings used >3 times
- **Cow<str> Support**: Flexible ownership for read-only vs mutable strings
- **Usage Analysis**: Context-aware string optimization
- **Zero Allocations**: Eliminated unnecessary .to_string() calls

### 🏆 v1.0.1 - Toyota Way Implementation
- **SATD Elimination**: Removed all 12 TODO/FIXME markers
- **Complete Implementation**: Fixed 6 unreachable!() calls
- **Quality Infrastructure**: Added pre-release audit scripts
- **Zero Defects**: Achieved Toyota Way compliance

## 🤝 Contributing

We welcome contributions! Please follow our quality standards:

1. **Write tests first** - TDD is mandatory
2. **Maintain coverage** - 85%+ for all new code
3. **Zero warnings** - `cargo clippy -- -D warnings`
4. **Format code** - `cargo fmt`
5. **Document changes** - Update relevant docs

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📚 Documentation

- **[API Documentation](https://docs.rs/depyler)** - Complete Rust API reference
- **[User Guide](docs/user-guide.md)** - Getting started tutorial
- **[Migration Guide](docs/migration-guide.md)** - Python to Rust transition
- **[Python-Rust Spec](docs/python-to-rust-spec.md)** - Language mapping
- **[Safety Guarantees](docs/safety-guarantees.md)** - Memory safety analysis
- **[MCP Integration](docs/mcp-integration.md)** - AI assistant integration

## 🚦 Roadmap

### ✅ v1.0 - Core Transpilation (Released)
- Safe subset transpilation
- Toyota Way quality metrics
- Property-based testing
- Basic MCP integration

### ✅ v1.0.2 - String Optimization (Released)
- String interning for efficiency
- Cow<str> for flexible ownership
- Usage-based optimization
- Zero unnecessary allocations

### 🚧 v1.1 - Enhanced Type System (Next)
- Advanced lifetime inference
- Better dataclass support
- Improved async patterns
- Contract verification

### 🔮 v1.2 - Advanced Patterns
- Full async/await support
- Iterator protocol
- Context managers
- Exception chaining

See [ROADMAP.md](ROADMAP.md) for detailed plans.

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

**Built with extreme quality standards by the Depyler team**
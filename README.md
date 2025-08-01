# Depyler

[![Crates.io](https://img.shields.io/crates/v/depyler.svg)](https://crates.io/crates/depyler)
[![Documentation](https://docs.rs/depyler/badge.svg)](https://docs.rs/depyler)
[![CI](https://github.com/paiml/depyler/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/depyler/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-85%25+-brightgreen.svg)](https://codecov.io/gh/paiml/depyler)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 1.83+](https://img.shields.io/badge/rust-1.83+-orange.svg)](https://www.rust-lang.org)
[![MCP Compatible](https://img.shields.io/badge/MCP-Compatible-green)](https://modelcontextprotocol.io)

**Energy-efficient Python-to-Rust transpiler** with progressive verification capabilities. Transform Python code into safe, performant Rust while reducing energy consumption by 75-85%. Built with zero tolerance for technical debt and extreme quality standards.

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

# Analyze code complexity before transpilation
depyler analyze example.py

# Run with verification
depyler transpile example.py --verify

# Interactive mode with AI suggestions
depyler interactive example.py --suggest
```

### Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
depyler = "0.3.2"
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

### ⚡ Performance & Efficiency
- **Energy Reduction** - 75-85% lower energy consumption vs Python
- **Binary Optimization** - Compile with LTO, strip, and `panic=abort`
- **Zero-Copy Strings** - Smart string allocation strategies
- **LLVM Backend** - Leverages Rust's optimizing compiler

### 🛡️ Safety & Verification
- **Property-Based Testing** - QuickCheck for semantic equivalence
- **Memory Safety Analysis** - Prevents use-after-free and data races
- **Bounds Checking** - Automatic insertion where needed
- **Contract Verification** - Pre/post condition checking

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
- Basic classes and dataclasses
- Pattern matching (Python 3.10+)

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
depyler quality-check input.py               # PMAT scoring
```

### MCP Integration

```bash
# Use with Claude or other MCP-compatible AI assistants
depyler serve --mcp

# Available MCP tools:
# - transpile_python: Convert Python to Rust
# - analyze_complexity: Code complexity analysis
# - verify_transpilation: Verify semantic equivalence
# - suggest_annotations: Optimization hints
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

## 📊 Quality Standards

Following the **Toyota Way** principles:

### 自働化 (Jidoka) - Build Quality In
- Never ship incomplete transpilation
- All generated code must compile
- Verification-first development

### 現地現物 (Genchi Genbutsu) - Direct Observation  
- Test against real Python codebases
- Profile actual compilation times
- Debug at the Rust level

### 改善 (Kaizen) - Continuous Improvement
- Incremental verification levels
- Performance baselines
- Code quality targets

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

## 🤝 Contributing

We welcome contributions! Please follow our quality standards:

1. **Write tests first** - TDD is mandatory
2. **Maintain coverage** - 85%+ for all new code
3. **Zero warnings** - `cargo clippy -- -D warnings`
4. **Format code** - `cargo fmt`
5. **Document changes** - Update relevant docs

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📚 Documentation

- **[User Guide](docs/user-guide.md)** - Getting started tutorial
- **[Migration Guide](docs/migration-guide.md)** - Python to Rust transition
- **[API Documentation](https://docs.rs/depyler)** - Rust API reference
- **[Python-Rust Spec](docs/python-to-rust-spec.md)** - Language mapping
- **[Safety Guarantees](docs/safety-guarantees.md)** - Memory safety analysis

## 🚦 Roadmap

### Current: v1.0 - Core Transpilation ✅
- Safe subset transpilation
- PMAT quality metrics
- Property-based testing
- Basic MCP integration

### Next: v1.1 - Enhanced Type System
- Lifetime inference
- Dataclass support
- Improved string handling
- Contract verification

### Future: v1.2 - Advanced Patterns
- Async/await support
- Iterator protocol
- Context managers
- Exception patterns

See [ROADMAP.md](ROADMAP.md) for detailed plans.

## 📄 License

Licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

**Built with extreme quality standards by the Depyler team**
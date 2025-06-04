# Depyler v0.1.0 Release Notes

**Release Date:** January 6, 2025

We are excited to announce the first release of Depyler, a Python-to-Rust transpiler focused on energy efficiency and memory safety. This initial release provides a solid foundation for transpiling a practical subset of Python to idiomatic, safe Rust code.

## 🎯 Highlights

- **Core Transpilation Engine**: Fully functional Python-to-Rust transpiler supporting essential Python features
- **Type-Safe Code Generation**: Automatic type inference and mapping to Rust's type system
- **Memory Optimized**: Uses SmallVec and other optimizations to minimize allocations
- **Comprehensive Testing**: 62.88% function coverage with 70 tests
- **Energy Efficient**: Generated Rust code uses significantly less energy than interpreted Python

## ✨ Features

### Supported Python Features
- **Basic Types**: `int`, `float`, `str`, `bool`, `None`
- **Collections**: `list`, `dict`, `tuple` with type annotations
- **Control Flow**: `if/else`, `while`, `for` loops
- **Functions**: With type annotations and return types
- **Operations**: All arithmetic, comparison, logical, and bitwise operators
- **Type Annotations**: Full support for `typing` module annotations

### Architecture Highlights
- **HIR (High-level Intermediate Representation)**: Clean abstraction layer between Python AST and Rust code
- **Unified Code Generation**: Single source of truth for all code generation paths
- **Context-Aware Errors**: Detailed error messages with source locations
- **Property Verification**: Framework for ensuring semantic correctness

## 📦 Installation

### Quick Install (Linux/macOS)
```bash
curl -sSfL https://github.com/paiml/depyler/releases/latest/download/install.sh | sh
```

### Manual Download
Download pre-built binaries for your platform:
- [Linux x64](https://github.com/paiml/depyler/releases/download/v0.1.0/depyler-linux-amd64.tar.gz)
- [Linux ARM64](https://github.com/paiml/depyler/releases/download/v0.1.0/depyler-linux-arm64.tar.gz)
- [macOS Intel](https://github.com/paiml/depyler/releases/download/v0.1.0/depyler-darwin-amd64.tar.gz)
- [macOS Apple Silicon](https://github.com/paiml/depyler/releases/download/v0.1.0/depyler-darwin-arm64.tar.gz)
- [Windows x64](https://github.com/paiml/depyler/releases/download/v0.1.0/depyler-windows-amd64.zip)

### Build from Source
```bash
git clone https://github.com/paiml/depyler.git
cd depyler
cargo build --release
cargo install --path crates/depyler
```

## 🚀 Usage

### Basic Example
```python
# fibonacci.py
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

Transpile to Rust:
```bash
depyler transpile fibonacci.py -o fibonacci.rs
```

### Example with Type Annotations
```python
# process.py
from typing import List, Dict

def process_data(items: List[int]) -> Dict[str, int]:
    result = {"sum": 0, "count": len(items)}
    for item in items:
        result["sum"] = result["sum"] + item
    return result
```

## 🔧 Technical Details

### Code Quality Metrics
- **Function Coverage**: 62.88% (exceeding 60% threshold)
- **Total Tests**: 70 comprehensive tests
- **Zero Warnings**: Clean clippy and fmt checks
- **Complexity**: Reduced through strategy pattern refactoring

### Performance
- **Transpilation Speed**: ~1.8 MB/s on typical Python code
- **Memory Usage**: Optimized with SmallVec for common patterns
- **Binary Size**: ~15-20MB release build (platform dependent)

## ⚠️ Known Limitations

This is a v0.1.0 release with the following limitations:

1. **Limited Python Feature Set**: Only supports V1 subset (no classes, decorators, async)
2. **Code Formatting**: Generated Rust code needs manual formatting (rustfmt integration coming)
3. **Docstrings**: Currently converted to string literals instead of doc comments
4. **Error Recovery**: Stops on first error (no error recovery yet)
5. **Standard Library**: No Python stdlib support yet

## 🔮 Future Roadmap

- **v0.2.0**: Rustfmt integration, improved error messages, basic class support
- **v0.3.0**: Extended Python features, list/dict comprehensions
- **v0.4.0**: Advanced type inference, async support
- **v1.0.0**: Production-ready with comprehensive Python subset

## 🙏 Acknowledgments

- Built following Toyota Way principles (Jidoka, Genchi Genbutsu, Hansei, Kaizen)
- Inspired by the Rash project's quality standards
- Uses rustpython-parser for Python AST parsing
- Follows NASA/SQLite testing standards

## 📊 Project Stats

- **Lines of Code**: ~3,500
- **Crates**: 5 modular crates
- **Dependencies**: Minimal, security audited
- **License**: MIT OR Apache-2.0

## 🐛 Bug Reports

Please report issues at: https://github.com/paiml/depyler/issues

## 📚 Documentation

- [User Guide](https://github.com/paiml/depyler/blob/main/docs/user-guide.md)
- [CLI Reference](https://github.com/paiml/depyler/blob/main/docs/cli-reference.md)
- [Project Overview](https://github.com/paiml/depyler/blob/main/docs/project-overview.md)

---

Thank you for trying Depyler! We're excited to help you transition your Python code to energy-efficient Rust.
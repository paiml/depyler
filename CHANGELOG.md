# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-06

### Initial Release

#### Core Features
- **Python-to-Rust Transpiler**: Full support for Python V1 subset
  - Basic types: int, float, str, bool, None
  - Collections: list, dict, tuple
  - Control flow: if/else, while, for loops
  - Functions with type annotations
  - Binary and unary operations
  - List/dict comprehensions (planned)

#### Architecture
- **Unified Code Generation**: Single source of truth for HIR-to-Rust conversion
- **Type System**: Sophisticated type mapping with configurable strategies
- **Error Handling**: Context-aware errors with source location tracking
- **Memory Optimized**: SmallVec usage for common patterns

#### Code Quality
- **Test Coverage**: 62.88% function coverage with 70 tests
- **Zero Warnings**: All clippy and formatting checks pass
- **Documentation**: Comprehensive API documentation
- **Performance**: Optimized memory allocations and compile times

#### Verification
- **Property-based Testing**: Framework for correctness verification
- **Semantic Preservation**: Ensures Python semantics are preserved
- **Panic-free Guarantees**: Optional verification for generated code

#### Developer Experience
- **CLI Interface**: Simple `depyler transpile` command
- **Error Messages**: Clear, actionable error reporting
- **Extensible Design**: Easy to add new Python features

[Unreleased]: https://github.com/depyler/depyler/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/depyler/depyler/releases/tag/v0.1.0
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-01-06

**Interactive Playground & Enterprise-Ready Quality Improvements**

[Full Release Notes](./RELEASE_NOTES_v0.3.0.md)

### Added
- **Interactive Playground**: Zero-configuration WebAssembly-powered environment for instant Python-to-Rust transpilation
  - Real-time side-by-side Python and Rust execution with performance metrics
  - Intelli-Sensei code intelligence with smart suggestions and anti-pattern detection
  - Three-column view (Python → HIR → Rust) with synchronized scrolling
  - Visual energy gauge showing up to 97% energy reduction
  - Offline capable with intelligent LRU caching for sub-50ms transpilation
- **Enhanced Type Inference**: Better generic handling, collection type propagation, and function signature analysis
- **PMAT Quality Framework**: Comprehensive metrics for Productivity, Maintainability, Accessibility, and Testability
- **Multi-Platform CI/CD**: Automated releases for Linux, macOS, and Windows with binary size tracking
- **Improved Error Messages**: Context-aware errors with source location tracking and helpful suggestions

### Changed
- **Performance**: 15% faster transpilation with 30% lower memory footprint
- **CLI Interface**: `--verify` flag now requires a value (`basic`, `standard`, or `strict`)
- **API Changes**: `TranspileOptions::verify` now uses `VerificationLevel` enum
- **Default Output**: Changed from `./output` to `./rust_output`
- **Test Coverage**: Increased from 85% to 89%
- **PMAT TDG Score**: Improved from 2.1 to 1.8 (14% better)
- **Energy Efficiency**: Increased from 93% to 97%

### Fixed
- Lambda inference improvements for nested patterns and async handlers
- String interpolation edge cases with escaped characters
- Ownership inference for nested function calls
- Platform-specific issues including OpenSSL dependencies and linker errors
- Interactive mode timeouts in CI environments

### Security
- Network APIs disabled in playground sandbox for security
- Execution time limited to 5 seconds to prevent infinite loops

## [0.2.0] - 2025-01-06

### Added
- **AWS Lambda Transpilation Pipeline**: Complete end-to-end Lambda function transpilation with automatic event type inference
- **Lambda CLI Commands**: New `lambda analyze`, `lambda convert`, `lambda test`, `lambda build`, and `lambda deploy` commands
- **Event Type Inference Engine**: ML-based pattern matching for S3, API Gateway, SQS, SNS, DynamoDB, and EventBridge events
- **Cold Start Optimization**: 85-95% reduction through pre-warming, binary optimization, and memory pre-allocation
- **cargo-lambda Integration**: Seamless deployment to AWS Lambda with optimized builds for ARM64 and x86_64
- **Lambda Code Generation**: Event-specific type mappings, error handling, and performance monitoring
- **Test Harness**: Automatic test suite generation with local Lambda event simulation
- **Deployment Templates**: SAM and CDK template generation for infrastructure as code
- **Performance Monitoring**: Built-in cold start tracking and memory profiling

### Changed
- **Version**: Major version bump to 0.2.0 for Lambda features
- **Test Coverage**: Increased to 85%+ across all modules
- **CI/CD Pipeline**: Fixed all test failures and coverage issues
- **Documentation**: Added comprehensive Lambda transpilation guide

### Fixed
- Coverage build failures with proper conditional compilation
- All clippy warnings and formatting issues across the workspace
- Interactive mode test timeout in CI environments
- Field reassignment patterns for better code quality
- Broken URLs in README documentation

## [0.1.2] - 2025-01-06

### Added
- **Enhanced Test Coverage**: Achieved 76.95% test coverage across workspace
- **Comprehensive Testing**: Added extensive unit tests for analyzer metrics, type flow, and contract verification modules
- **Quality Standards**: Maintained PMAT TDG score of 1.03 and complexity of 4

### Changed
- **Code Quality**: Fixed all clippy warnings and formatting issues
- **InteractiveSession**: Added proper Default trait implementation
- **Public API**: Made complexity_rating function public for external use

### Fixed
- **Lint Issues**: Resolved InteractiveSession Default implementation clippy warning
- **Unused Variables**: Fixed unused variable warnings in quickcheck.rs
- **Dead Code**: Resolved dead code warnings for complexity_rating function
- **Auto-fixes**: Applied cargo fix suggestions across multiple modules

### Quality Metrics
- **Test Coverage**: 76.95% (up from previous releases)
- **PMAT TDG Score**: 1.03 ✅ (target: 1.0-2.0)
- **Cyclomatic Complexity**: 4 ✅ (target: ≤20)
- **Code Quality**: All clippy lints resolved

## [0.1.1] - 2025-01-06

### Added
- **Augmented Assignment Operators**: Full support for `+=`, `-=`, `*=`, `/=`, `%=`, etc.
- **Membership Operators**: Implemented `in` and `not in` operators for dictionary membership checks
- **QuickCheck Integration**: Property-based testing framework for transpilation correctness
- **Operator Test Suite**: Comprehensive tests covering all supported operators
- **Property Tests**: Verification of type preservation, purity, and panic-freedom properties

### Changed
- **Reduced Complexity**: Refactored HirExpr::to_rust_expr from cyclomatic complexity 42 to <20
- **Cleaner AST Bridge**: Modularized expression and statement conversion with dedicated converters
- **Better Error Messages**: More informative error reporting for unsupported constructs

### Fixed
- Fixed transpilation of augmented assignment operators
- Fixed dictionary membership test operators
- Improved handling of string literals in generated code

### Metrics
- **V1.0 Transpilation Success Rate**: 100% (4/4 examples)
- **Code Quality Score**: 75.0/100
- **Major complexity hotspots refactored**

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

[Unreleased]: https://github.com/paiml/depyler/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/paiml/depyler/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/paiml/depyler/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/paiml/depyler/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/paiml/depyler/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/paiml/depyler/releases/tag/v0.1.0
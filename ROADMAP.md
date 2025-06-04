# Depyler Development Roadmap

## Overview

Depyler is a pragmatic Python-to-Rust transpiler with progressive verification capabilities. This roadmap outlines our development milestones, focusing on delivering practical value while building toward ambitious verification goals.

## V1.0 - Core Transpilation (3 months)
**Status: In Progress**

### Goals
- Safe subset transpilation with rustpython-parser
- PMAT integration for quality metrics
- Property-based test generation
- Basic MCP fallback for unsupported constructs

### Features
- ✅ Python AST to HIR conversion
- ✅ Direct transpilation rules for basic constructs
- ✅ Type mapping (int, float, str, bool, List, Dict, Optional)
- ✅ Complexity analysis (cyclomatic, cognitive)
- ✅ Property-based test generation framework
- ✅ CLI with transpile, analyze, and check commands
- ⏳ MCP protocol implementation
- ⏳ Comprehensive test suite

### Supported Python Constructs
- Functions with type annotations
- Basic control flow (if/elif/else, while, for)
- Primitive types and operations
- Lists, dictionaries, tuples
- Simple pattern matching

### Verification Properties
- Type preservation (static analysis)
- Panic freedom (bounds checking)
- Termination (simple cases)
- Purity detection

### Metrics
- Parsing: 20MB/s target
- Direct transpilation: 90% coverage on safe subset
- Binary size: <5MB

## V1.1 - Enhanced Type System (6 months)

### Goals
- Lifetime inference for simple borrowing patterns
- `@dataclass` support with ownership inference
- Improved string handling (String vs &str)
- Contract-based verification

### New Features
- Borrowing inference for function parameters
- Dataclass to struct conversion
- Method transpilation
- Basic error handling patterns
- Const correctness inference

### Verification Enhancements
- Pre/post condition checking
- Invariant preservation
- Simple alias analysis
- Ownership transfer validation

## V1.2 - Async & Advanced Patterns (9 months)

### Goals
- `async`/`await` support
- Iterator protocol mapping
- Context managers → RAII
- Basic formal verification for critical properties

### New Features
- Async function transpilation
- Generator to iterator conversion
- `with` statement to RAII pattern
- Exception to Result<T, E> mapping
- List comprehensions
- Lambda expressions (limited)

### Verification Enhancements
- Async safety properties
- Resource leak detection
- Deadlock freedom (simple cases)
- SMT integration for core properties

## V2.0 - Full Python Subset (12 months)

### Goals
- Class inheritance (single)
- Generator expressions
- Limited dynamic dispatch
- SMT-based verification for core properties

### New Features
- Class to struct+impl conversion
- Trait inference from protocols
- Dynamic attribute access (limited)
- Decorators (subset)
- Multiple return values
- Keyword arguments

### Verification Enhancements
- Full SMT solver integration
- Refinement type inference
- Effect system for side effects
- Concurrency verification

## Future Directions

### V2.1 - Performance Optimization
- Profile-guided transpilation
- SIMD pattern recognition
- Parallelization opportunities
- Zero-copy optimizations

### V2.2 - Ecosystem Integration
- PyO3 interop layer
- Cargo integration
- IDE support (LSP)
- Debugging support

### V3.0 - Advanced Verification
- Dependent type support
- Separation logic integration
- Concurrent program verification
- Machine-checked proofs

## Success Metrics

### Adoption Metrics
- GitHub stars: 1k+ (V1.0), 5k+ (V2.0)
- Production users: 10+ companies
- Community contributors: 50+

### Technical Metrics
- Python coverage: 60% (V1.0), 80% (V2.0)
- Verification confidence: 90%+ on supported subset
- Performance: Within 2x of hand-written Rust

### Quality Metrics
- Zero soundness bugs in transpilation
- 95%+ test coverage
- All code PMAT-validated

## Contributing

We welcome contributions! Priority areas:
1. Python construct coverage
2. Verification properties
3. Performance optimization
4. Documentation and examples
5. IDE integration

See CONTRIBUTING.md for details.

## Verification Milestones

### Month 1-3 (V1.0)
- [x] Static type checking
- [x] Bounds checking insertion
- [x] Simple termination analysis
- [ ] QuickCheck integration

### Month 4-6 (V1.1)
- [ ] Lifetime inference
- [ ] Ownership analysis
- [ ] Contract verification
- [ ] Alias analysis

### Month 7-9 (V1.2)
- [ ] Async verification
- [ ] Resource tracking
- [ ] Effect system
- [ ] SMT experiments

### Month 10-12 (V2.0)
- [ ] Full SMT integration
- [ ] Refinement types
- [ ] Concurrency verification
- [ ] Proof generation

This roadmap is ambitious but achievable, balancing immediate practical value with long-term verification goals.
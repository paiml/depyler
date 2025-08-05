# Depyler Architecture

## Overview

Depyler is a Python-to-Rust transpiler designed for energy-efficient, safe code generation with progressive verification. The architecture emphasizes correctness, performance, and maintainability through a multi-stage pipeline.

## Core Architecture

### Pipeline Stages

```
Python Source → AST → HIR → Optimization → Rust AST → Rust Code
                 ↓      ↓        ↓           ↓
              Analysis Verification  Type    Code Gen
```

### 1. AST Bridge (`ast_bridge/`)
- **Purpose**: Convert Python AST to High-level Intermediate Representation (HIR)
- **Components**:
  - `converters.rs`: Expression and statement converters
  - `type_extraction.rs`: Type annotation extraction
  - `mod.rs`: Public API and coordination

### 2. HIR (High-level Intermediate Representation)
- **Purpose**: Language-agnostic representation of Python semantics
- **Key Types**:
  - `HirModule`: Top-level module structure
  - `HirFunction`: Function definitions with properties
  - `HirClass`: Class definitions with methods and fields
  - `HirExpr`: Expression variants (literals, operations, comprehensions)
  - `HirStmt`: Statement variants (assignments, control flow)

### 3. Code Generation (`rust_gen.rs`)
- **Purpose**: Generate idiomatic Rust code from HIR
- **Features**:
  - Type-aware code generation
  - Ownership inference
  - Lifetime annotations
  - Import management

### 4. Type System (`type_mapper.rs`)
- **Purpose**: Map Python types to Rust equivalents
- **Capabilities**:
  - Primitive type mapping
  - Collection type handling
  - Generic type resolution
  - Custom type support

### 5. Optimization Pipeline
- **Constant Propagation**: Evaluate compile-time constants
- **Dead Code Elimination**: Remove unused code
- **Common Subexpression Elimination**: Reduce redundant computations
- **Function Inlining**: Inline small functions
- **String Optimization**: Use `&str` where possible

### 6. Verification (`depyler-verify/`)
- **Contract Verification**: Design by Contract support
- **Lifetime Analysis**: Ensure memory safety
- **Memory Safety**: Prevent undefined behavior
- **Property Testing**: Validate transpilation correctness

## Module Organization

### Core Modules (`depyler-core/`)
- `ast_bridge/`: Python AST to HIR conversion
- `direct_rules.rs`: HIR to Rust AST rules
- `hir.rs`: HIR type definitions
- `lsp.rs`: Language Server Protocol implementation
- `migration_suggestions.rs`: Python-to-Rust idiom advisor
- `module_mapper.rs`: Standard library mappings
- `optimization.rs`: Optimization passes
- `rust_gen.rs`: Rust code generation
- `type_mapper.rs`: Type system mapping

### Supporting Crates
- `depyler-analyzer/`: Static analysis and metrics
- `depyler-mcp/`: Model Context Protocol integration
- `depyler-verify/`: Verification and safety checks
- `depyler-wasm/`: WebAssembly bindings

## Testing Architecture

### Test Coverage Strategy (v2.2.2)
- **Line Coverage**: 69.55% (target: 80%)
- **Testing Philosophy**: Every module includes:
  1. Unit tests for core functionality
  2. Property tests for invariants
  3. Doctests for documentation
  4. Example files for usage

### Test Organization
```
module.rs                    # Main implementation
module_tests.rs             # Unit tests (embedded or separate)
tests/module_property_tests.rs  # Property-based tests
examples/module_demo.rs     # Working examples
```

### Property Testing Framework
- Uses `proptest` for randomized testing
- Custom generators for Python patterns
- Invariant checking (type preservation, panic-freedom)
- Edge case discovery through shrinking

### Integration Testing
- End-to-end transpilation tests
- Cross-module interaction tests
- Platform-specific tests (Linux, macOS, Windows)
- Performance regression tests

## Quality Assurance

### Continuous Integration
- **GitHub Actions**: Multi-stage pipeline
- **Test Suite**: Unit, property, integration, doctests
- **Code Quality**: Clippy, rustfmt, coverage
- **Performance**: Benchmark tracking
- **Security**: Dependency audits

### Quality Metrics
- **PMAT Score**: Productivity, Maintainability, Accessibility, Testability
- **Cyclomatic Complexity**: Target <20 per function
- **Test Coverage**: Minimum 70% line coverage
- **Zero Defects**: Toyota Way principle

## Performance Considerations

### Memory Efficiency
- `SmallVec` for common small collections
- String interning for repeated literals
- `Cow<str>` for flexible string ownership
- Arena allocation for AST nodes

### Compilation Speed
- Incremental compilation support
- Parallel processing where possible
- Caching of analysis results
- Optimized HIR traversal

## Security Model

### Input Validation
- Sanitize Python source code
- Validate HIR transformations
- Prevent code injection
- Resource limits (recursion, memory)

### Generated Code Safety
- Memory safety guarantees
- No undefined behavior
- Safe error handling
- Secure defaults

## Future Architecture Plans

### v3.0 (Beta Release)
- Advanced async/await support
- Enhanced type inference
- Improved error recovery
- Plugin system for extensions
- Incremental transpilation
- LSP feature completeness

### Long-term Vision
- Multi-language frontend support
- LLVM backend option
- Distributed transpilation
- AI-assisted optimization
- Formal verification integration
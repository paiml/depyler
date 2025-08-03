# Next-Gen Depyler Development Roadmap v2

This document provides a **strictly sequential** development plan for Depyler. Each priority level must be **100% complete** before moving to the next. Each completed priority results in a new cargo release.

## Development Principles

1. **Sequential Execution**: Complete ALL tasks in a priority before moving to the next
2. **Quality Gates**: Each priority must pass ALL quality checks before release
3. **Incremental Releases**: Each priority completion triggers a new minor version release
4. **No Skipping**: Never jump ahead to future priorities

## Current Status (v1.0.8)

### Completed Features
- ✅ Type inference and ownership fixes
- ✅ String handling optimizations  
- ✅ Property verification improvements
- ✅ Protocol to trait mapping
- ✅ Floor division operator
- ✅ Basic class support
- ✅ Const generic arrays
- ✅ Lambda function support
- ✅ Power operator (`**`)
- ✅ Dictionary nested assignment (including subscript)
- ✅ Set operations (type, operators, methods, comprehensions)

### Known Gaps
- Match/case statements
- Break/continue with labels
- Frozen sets

## Priority 1: Core Language Completeness (v1.1.0)

**Goal**: Complete all basic Python language features for v1.0 compatibility

### 1.1 Operators
- [x] Power operator (`**`) transpilation
  - [x] Integer power with `.pow()`
  - [x] Float power with `.powf()`
  - [x] Handle negative exponents
  - [x] Add overflow checking
  - [x] Import `num_traits::Pow` when needed

### 1.2 Collections
- [x] Complete dictionary assignment
  - [x] Nested assignments `d[k1][k2] = v`
  - [x] Tuple key assignments `d[(x, y)] = v`
  - [x] Entry API optimization
  - [x] `get_mut` for updates
- [x] Set operations
  - [x] Set type with HashSet/BTreeSet
  - [x] Set operators (&, |, -, ^)
  - [x] Set methods (add, remove, discard, clear, pop)
  - [x] Set comprehensions
  - [x] Frozen sets

### 1.3 Control Flow
- [x] Break/continue statements
  - [x] Basic break in loops
  - [x] Basic continue in loops
  - [x] Proper control flow handling
  - Note: Python doesn't support labeled break/continue
- [x] Match/case statements deferred to v2.0
  - Note: Structural pattern matching is complex and not critical for v1.x

**Status**: ✅ **COMPLETED** - All core language features implemented
**Release**: v1.1.0 ✅ Released to crates.io on 2025-01-03

## Priority 2: Method Resolution & Attributes (v1.2.0)

**Goal**: Complete object-oriented programming support

### 2.1 Method Calls
- [ ] Instance method resolution
  - [ ] `self` parameter handling
  - [ ] Method lookup order
  - [ ] Bound methods
  - [ ] Static methods (@staticmethod)
  - [ ] Class methods (@classmethod)

### 2.2 Attribute Access
- [ ] Attribute resolution
  - [ ] Instance attributes
  - [ ] Class attributes  
  - [ ] Property decorators
  - [ ] Getters/setters
  - [ ] `__getattr__` support

### 2.3 Inheritance
- [ ] Single inheritance
  - [ ] Base class methods
  - [ ] Super() calls
  - [ ] Method overriding
  - [ ] Constructor chaining

**Release**: v1.2.0 after ALL above tasks complete

## Priority 3: Advanced Type Features (v1.3.0)

**Goal**: Complete type system for complex Python patterns

### 3.1 Decorators
- [ ] Function decorators
  - [ ] Simple decorators
  - [ ] Parameterized decorators
  - [ ] Decorator stacking
  - [ ] Built-in decorators (property, staticmethod, etc.)

### 3.2 Context Managers
- [ ] With statement support
  - [ ] `__enter__`/`__exit__` protocol
  - [ ] Exception handling in context
  - [ ] Multiple context managers
  - [ ] Async context managers

### 3.3 Iterators & Generators
- [ ] Iterator protocol
  - [ ] `__iter__`/`__next__`
  - [ ] Custom iterators
  - [ ] Iterator helpers
- [ ] Generator functions
  - [ ] Yield expressions
  - [ ] Generator state
  - [ ] Send/throw support

**Release**: v1.3.0 after ALL above tasks complete

## Priority 4: Async/Await Support (v1.4.0)

**Goal**: Full async Python support

### 4.1 Basic Async
- [ ] Async function definitions
- [ ] Await expressions
- [ ] Async type inference
- [ ] Runtime selection (tokio/async-std)

### 4.2 Async Iteration
- [ ] Async iterators
- [ ] Async generators
- [ ] Async comprehensions
- [ ] AsyncIterator trait mapping

### 4.3 Async Context
- [ ] Async with statements
- [ ] Async context managers
- [ ] Concurrent execution helpers

**Release**: v1.4.0 after ALL above tasks complete

## Priority 5: Module System (v1.5.0)

**Goal**: Full Python module and package support

### 5.1 Imports
- [ ] Module imports
- [ ] Package imports
- [ ] Relative imports
- [ ] Star imports
- [ ] Import aliasing

### 5.2 Module Structure
- [ ] `__init__.py` handling
- [ ] Package hierarchies
- [ ] Module attributes
- [ ] Circular import resolution

**Release**: v1.5.0 after ALL above tasks complete

## Priority 6: Standard Library Mapping (v1.6.0)

**Goal**: Map common Python stdlib to Rust equivalents

### 6.1 Core Modules
- [ ] `os` → std::fs, std::env
- [ ] `sys` → std::env, std::process
- [ ] `json` → serde_json
- [ ] `re` → regex
- [ ] `datetime` → chrono

### 6.2 Collections
- [ ] `collections` → std::collections
- [ ] `itertools` → itertools
- [ ] `functools` → closures/traits

**Release**: v1.6.0 after ALL above tasks complete

## Priority 7: Optimization & Polish (v2.0.0)

**Goal**: Production-ready performance and quality

### 7.1 Performance
- [ ] Optimization passes
- [ ] Inlining heuristics
- [ ] Dead code elimination
- [ ] Const propagation

### 7.2 Diagnostics
- [ ] Enhanced error messages
- [ ] Type inference hints
- [ ] Migration suggestions
- [ ] Performance warnings

### 7.3 Tooling
- [ ] IDE integration
- [ ] Debugging support
- [ ] Profiling integration
- [ ] Documentation generation

**Release**: v2.0.0 - Major version for production readiness

## Quality Gates for Each Release

Before ANY release:
1. [ ] All tests passing (100%)
2. [ ] Zero clippy warnings with pedantic lints
3. [ ] Documentation complete for new features
4. [ ] Changelog updated
5. [ ] Examples added and tested
6. [ ] Performance benchmarks show no regression
7. [ ] Manual testing on real Python projects
8. [ ] Version bumped in all Cargo.toml files
9. [ ] Git tag created and pushed
10. [ ] Published to crates.io

## Version Numbering

- **v1.0.x**: Bug fixes only
- **v1.x.0**: Each completed priority
- **v2.0.0**: Production ready with optimizations

## Current Next Steps

1. Start Priority 1 (Core Language Completeness)
2. Begin with power operator implementation
3. Complete ALL Priority 1 tasks
4. Run quality gates
5. Release v1.1.0
6. Only then move to Priority 2

This roadmap ensures systematic progress with clear milestones and prevents the priority jumping that occurred previously.
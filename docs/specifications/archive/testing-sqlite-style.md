# Depyler SQLite-Style Testing Specification

**Purpose**: Achieve comprehensive, systematic validation inspired by SQLite's legendary test coverage
**Created**: 2025-10-16 (v3.19.14)
**Philosophy**: Toyota Way (Jidoka) + SQLite's "100% MC/DC coverage" approach
**Target**: Production-grade reliability for Python→Rust transpilation

---

## Background: SQLite Testing Philosophy

SQLite achieves extraordinary reliability through:
- **100% branch coverage** - every code path tested
- **100% MC/DC coverage** - every condition tested independently
- **1,000x more test code than production code**
- **Systematic validation** - not random testing
- **Diverse test methods** - unit, integration, fuzz, boundary, mutation
- **Long-running stress tests** - days of continuous operation

**Depyler Adaptation**: Apply these principles to transpiler testing

---

## Current Status (v3.19.14)

### Test Coverage Metrics

**Tests**: 443 passing (core), 600+ workspace-wide
**Coverage**: 80%+ via cargo-llvm-cov
**Quality**: Zero regressions, A- TDG grade
**Complexity**: All functions ≤10

### Existing Test Categories

1. **Unit Tests**: Individual function testing
2. **Integration Tests**: End-to-end transpilation pipeline
3. **Property Tests**: Random input generation (quickcheck, proptest)
4. **Mutation Tests**: Code mutation validation
5. **Example Validation**: All examples must transpile and compile
6. **Regression Tests**: Bug-specific test suites

---

## SQLite-Style Testing Framework for Depyler

### Layer 1: Systematic Transpilation Validation

**Objective**: Test EVERY Python language construct systematically

**File**: `tests/sqlite_style_systematic_validation.rs`

**Test Matrix** (20 categories × 5 tests each = 100 tests):

1. **Literals** (5 tests)
   - Integer literals (various bases: decimal, hex, octal, binary)
   - Float literals (scientific notation, special values: inf, nan)
   - String literals (raw, unicode, f-strings, multiline)
   - Boolean literals (True, False, truthiness)
   - None literal (null handling)

2. **Binary Operators** (5 tests)
   - Arithmetic (+, -, *, /, //, %, **)
   - Comparison (==, !=, <, <=, >, >=)
   - Logical (and, or, not)
   - Bitwise (&, |, ^, <<, >>)
   - Membership (in, not in)

3. **Unary Operators** (5 tests)
   - Negation (-x)
   - Logical not (not x)
   - Bitwise not (~x)
   - Positive (+x)
   - Identity tests (is, is not)

4. **Variables** (5 tests)
   - Assignment (simple, multiple, unpacking)
   - Augmented assignment (+=, -=, *=, etc.)
   - Type annotations (explicit, inferred)
   - Scope resolution (local, global, nonlocal)
   - Shadowing and rebinding

5. **Control Flow** (5 tests)
   - If/elif/else (nested, single, ternary)
   - While loops (break, continue, else)
   - For loops (range, iterables, enumerate)
   - Match statements (Python 3.10+)
   - Return statements (early, late, implicit)

6. **Functions** (5 tests)
   - Definition (parameters, defaults, type hints)
   - Calls (positional, keyword, *args, **kwargs)
   - Closures (capture, mutation)
   - Recursion (tail, non-tail, mutual)
   - Lambda expressions

7. **Collections - Lists** (5 tests)
   - Creation (literals, constructor, comprehension)
   - Access (indexing, slicing, negative indices)
   - Methods (append, extend, insert, remove, pop, clear, index, count, sort, reverse, copy)
   - Iteration (for loop, enumerate, zip)
   - Mutation (in-place modification)

8. **Collections - Dicts** (5 tests)
   - Creation (literals, constructor, comprehension)
   - Access (keys, values, items, get)
   - Methods (update, setdefault, pop, popitem, clear, copy)
   - Iteration (keys, values, items)
   - Nesting (dict of dicts, mixed types)

9. **Collections - Sets** (5 tests)
   - Creation (literals, constructor, comprehension)
   - Operations (union, intersection, difference)
   - Methods (add, remove, discard, pop, clear)
   - Set relations (issubset, issuperset, isdisjoint)
   - Frozen sets (immutable variant)

10. **Collections - Strings** (5 tests)
    - Creation (literals, constructor, formatting)
    - Methods (upper, lower, strip, startswith, endswith, split, join, find, replace, count, isdigit, isalpha)
    - Slicing (substring extraction)
    - Concatenation (+ operator, join)
    - Encoding/decoding (utf-8, ascii)

11. **Classes** (5 tests)
    - Definition (fields, methods, constructors)
    - Instantiation (new objects)
    - Methods (instance, class, static)
    - Inheritance (single, method override)
    - Properties (getters, setters)

12. **Async/Await** (5 tests)
    - Async functions (definition, calling)
    - Await expressions (single, multiple)
    - Async methods (in classes)
    - Async iterators (async for)
    - Async context managers (async with)

13. **Exception Handling** (5 tests)
    - Try/except (single, multiple handlers)
    - Try/finally (cleanup)
    - Try/except/else/finally (complete)
    - Raise statements (new, re-raise)
    - Custom exceptions (user-defined)

14. **Comprehensions** (5 tests)
    - List comprehensions (simple, nested, conditional)
    - Dict comprehensions (key-value pairs)
    - Set comprehensions (unique values)
    - Generator expressions (lazy evaluation)
    - Complex nesting (multiple levels)

15. **Iterators** (5 tests)
    - Built-in iterators (range, enumerate, zip)
    - Custom iterators (__iter__, __next__)
    - Generator functions (yield)
    - Generator expressions (lazy)
    - Iteration protocol compliance

16. **Context Managers** (5 tests)
    - With statements (single, multiple)
    - Custom context managers (__enter__, __exit__)
    - Exception handling in context
    - Resource cleanup verification
    - Nested contexts

17. **Decorators** (5 tests)
    - Function decorators (single, stacked)
    - Class decorators (modification)
    - Decorator factories (parameterized)
    - Built-in decorators (@staticmethod, @classmethod, @property)
    - Preservation of metadata

18. **Type System** (5 tests)
    - Type annotations (variables, functions, classes)
    - Generic types (List[T], Dict[K,V])
    - Optional types (Optional[T], Union)
    - Type inference (implicit typing)
    - Type checking (compatibility)

19. **Edge Cases** (5 tests)
    - Empty collections ([], {}, set())
    - Boundary values (max/min int, float limits)
    - Unicode handling (special characters)
    - Deep nesting (recursive structures)
    - Large data (performance stress)

20. **Error Conditions** (5 tests)
    - Syntax errors (malformed code)
    - Type errors (incompatible operations)
    - Runtime errors (division by zero)
    - Undefined variables (name errors)
    - Unsupported features (graceful degradation)

---

### Layer 2: MC/DC Coverage Testing

**Modified Condition/Decision Coverage** - SQLite's gold standard

**Objective**: Test every condition independently affects outcome

**Example**:
```python
# Condition: (a > 0) and (b < 10)
# MC/DC requires testing:
# 1. a > 0 is true, changes outcome
# 2. a > 0 is false, changes outcome
# 3. b < 10 is true, changes outcome
# 4. b < 10 is false, changes outcome
```

**Implementation**: Property-based testing with constraint solving

**File**: `tests/mcdc_coverage_validation.rs`

**Test Cases** (per boolean expression in transpiler):
1. All conditions true → result true
2. All conditions false → result false
3. Each condition flipped independently → result changes
4. Minimal test set for 100% MC/DC

---

### Layer 3: Boundary Value Analysis

**Objective**: Test limits of every numeric operation

**File**: `tests/boundary_value_tests.rs` (already exists - enhance)

**Test Matrix**:

| Type | Min Value | Max Value | Special |
|------|-----------|-----------|---------|
| i8   | -128      | 127       | 0, -1, 1 |
| i16  | -32768    | 32767     | 0, -1, 1 |
| i32  | -2^31     | 2^31-1    | 0, -1, 1 |
| i64  | -2^63     | 2^63-1    | 0, -1, 1 |
| u8   | 0         | 255       | 0, 1, 255 |
| u16  | 0         | 65535     | 0, 1, 65535 |
| u32  | 0         | 2^32-1    | 0, 1, max |
| u64  | 0         | 2^64-1    | 0, 1, max |
| f32  | -3.4e38   | 3.4e38    | 0.0, inf, nan |
| f64  | -1.8e308  | 1.8e308   | 0.0, inf, nan |

**Collection Sizes**:
- Empty (0 elements)
- Single (1 element)
- Small (2-10 elements)
- Medium (100-1000 elements)
- Large (10000+ elements)

---

### Layer 4: Fuzzing & Random Testing

**Objective**: Find bugs humans don't think of

**Tools**: cargo-fuzz, proptest, quickcheck

**File**: `fuzz/fuzz_targets/transpile_random.rs`

**Fuzzing Strategies**:

1. **Syntax Fuzzing**: Random valid Python AST generation
2. **Semantic Fuzzing**: Type-correct but unusual programs
3. **Boundary Fuzzing**: Edge cases in numeric operations
4. **Mutation Fuzzing**: Valid programs with small changes
5. **Corpus-guided**: Learn from past bugs

**Duration**: 24 hour continuous fuzzing runs in CI

---

### Layer 5: Regression Test Database

**Objective**: Never fix the same bug twice

**File**: `tests/regression_database.rs`

**Format** (inspired by SQLite's test database):
```rust
#[test]
fn regression_depyler_0222_dict_get_without_default() {
    // BUG: dict.get() returned Option<T> instead of T
    // FIXED: v3.19.14 - Added .unwrap_or_default()
    let python = r#"
def test(d: dict) -> int:
    return d.get("key")
"#;
    let rust = transpile(python).unwrap();
    assert!(rust.contains("unwrap_or_default"));
    assert_compiles(&rust);
}

#[test]
fn regression_depyler_0223_dict_update_routing() {
    // BUG: dict.update() routed to set handler
    // FIXED: v3.19.14 - Added disambiguation logic
    let python = r#"
d = {"a": 1}
d.update({"b": 2})
"#;
    let rust = transpile(python).unwrap();
    assert!(rust.contains("extend"));
    assert_compiles(&rust);
}
```

**Database Structure**:
- One test per historical bug (DEPYLER-XXXX)
- Comment explains original bug
- Test ensures bug stays fixed
- Links to issue/commit

---

### Layer 6: Stress & Endurance Testing

**Objective**: Find memory leaks, performance degradation

**File**: `tests/stress_endurance_tests.rs`

**Test Scenarios**:

1. **Long-running transpilation**: 1 million LOC Python file
2. **Repeated transpilation**: Same file 10,000 times (memory leak detection)
3. **Deep nesting**: 100-level nested structures
4. **Large collections**: 1 million element lists/dicts
5. **Concurrent transpilation**: 100 threads transpiling simultaneously

**Duration**: 1 hour continuous operation

**Metrics**:
- Memory usage (must remain constant)
- Throughput (must not degrade)
- Error rate (must be 0%)

---

### Layer 7: Example Validation (Existing - Enhanced)

**Objective**: All examples are integration tests

**File**: `tests/example_validation.rs` (already exists)

**Enhancement**: Add SQLite-style validation

**Current**:
```bash
depyler transpile examples/showcase/binary_search.py
cargo check examples/showcase/binary_search.rs
```

**Enhanced**:
```rust
#[test]
fn example_binary_search_sqlites_style() {
    let py_file = "examples/showcase/binary_search.py";
    let rs_file = "examples/showcase/binary_search.rs";

    // 1. Transpile
    let result = transpile_file(py_file);
    assert!(result.is_ok());

    // 2. Write Rust output
    std::fs::write(rs_file, result.unwrap()).unwrap();

    // 3. Compile with rustc
    assert_compiles(rs_file);

    // 4. Execute and verify output
    let output = execute_binary(rs_file);
    let expected = execute_python(py_file);
    assert_eq!(output, expected);

    // 5. Benchmark performance
    let rust_time = benchmark_rust(rs_file);
    let python_time = benchmark_python(py_file);
    assert!(rust_time < python_time * 2.0);  // Within 2x

    // 6. Memory usage
    let rust_mem = measure_memory(rs_file);
    let python_mem = measure_memory(py_file);
    assert!(rust_mem < python_mem * 1.5);  // Within 1.5x
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (v3.20.0)
**Duration**: 2 weeks
**Tests**: 100 systematic validation tests

- Create `sqlite_style_systematic_validation.rs`
- Implement 20 categories × 5 tests
- Target: 100% coverage of existing language features
- Integrate with CI

### Phase 2: MC/DC Coverage (v3.21.0)
**Duration**: 2 weeks
**Tests**: 50 MC/DC tests

- Implement `mcdc_coverage_validation.rs`
- Use proptest for constraint solving
- Target: 100% MC/DC on all boolean conditions
- Document coverage metrics

### Phase 3: Enhanced Fuzzing (v3.22.0)
**Duration**: 1 week
**Tests**: Continuous fuzzing

- Set up cargo-fuzz infrastructure
- Create 5 fuzz targets
- 24-hour CI fuzzing runs
- Corpus management

### Phase 4: Stress Testing (v3.23.0)
**Duration**: 1 week
**Tests**: 5 endurance tests

- Implement long-running tests
- Memory leak detection
- Performance regression detection
- CI integration

### Phase 5: Regression Database (v3.24.0)
**Duration**: Ongoing
**Tests**: ~50 tests (one per historical bug)

- Migrate all DEPYLER-XXXX bugs to regression tests
- Automated test generation from issues
- Maintain database as bugs are fixed

---

## Success Metrics

### Quantitative Targets

| Metric | Current | Target (v3.25.0) | SQLite Equivalent |
|--------|---------|------------------|-------------------|
| Test Lines / Code Lines | ~2:1 | 10:1 | 1000:1 |
| Branch Coverage | 80% | 100% | 100% |
| MC/DC Coverage | Unknown | 100% | 100% |
| Mutation Score | Unknown | 80% | 90%+ |
| Fuzz Hours/Week | 0 | 168 (24×7) | Continuous |
| Regression Tests | ~30 | 100+ | 1000+ |
| Stress Test Duration | 0 | 1 hour | Days |

### Qualitative Targets

- **Zero P0 bugs** in production releases
- **Regression-free** releases (no bugs reappear)
- **Performance predictability** (consistent benchmarks)
- **Memory safety** (no leaks, no undefined behavior)
- **User confidence** (trust transpiled code)

---

## Test Organization

### Directory Structure

```
tests/
├── sqlite_style_systematic_validation.rs  # Layer 1 (100 tests)
├── mcdc_coverage_validation.rs            # Layer 2 (50 tests)
├── boundary_value_tests.rs                # Layer 3 (existing, enhance)
├── stress_endurance_tests.rs              # Layer 6 (5 long-running)
├── regression_database.rs                 # Layer 5 (50+ bugs)
├── example_validation.rs                  # Layer 7 (existing, enhance)
├── property_tests.rs                      # Existing
├── mutation_testing.rs                    # Existing
└── integration_tests/
    ├── transpilation_tests.rs
    └── semantic_equivalence.rs

fuzz/
└── fuzz_targets/
    ├── transpile_random.rs                # Layer 4
    ├── parse_random.rs
    ├── codegen_random.rs
    ├── type_inference_random.rs
    └── stdlib_methods_random.rs
```

---

## CI/CD Integration

### Pre-commit Hooks

```bash
# Quick validation (< 1 minute)
cargo test --lib
cargo clippy -- -D warnings
pmat quality-gate
```

### Pull Request Checks

```bash
# Comprehensive validation (< 10 minutes)
cargo test --workspace
cargo llvm-cov --fail-under-lines 80
pmat tdg . --min-grade A-
cargo bench --no-run  # Ensure benchmarks compile
```

### Nightly CI

```bash
# Full SQLite-style validation (1 hour)
cargo test --workspace -- --include-ignored
cargo fuzz run transpile_random -- -max_total_time=3600
cargo test --test stress_endurance_tests
cargo mutants --all
```

---

## Toyota Way Integration

### Jidoka (Autonomation)

**Stop the Line**: Any test failure blocks merge
**Build Quality In**: Tests written before code
**Automated Detection**: CI catches issues immediately

### Genchi Genbutsu (Go and See)

**Real Examples**: Test actual user code patterns
**Execution Validation**: Generated Rust must compile and run
**Performance Measurement**: Actual benchmarks, not estimates

### Kaizen (Continuous Improvement)

**Incremental Coverage**: Add tests with each feature
**Learn from Bugs**: Every bug becomes a regression test
**Systematic Growth**: Expand test matrix methodically

### Hansei (Reflection)

**Post-release Review**: Analyze what tests missed
**Update Framework**: Improve test design based on findings
**Document Lessons**: Update this spec with learnings

---

## Comparison to SQLite

| Aspect | SQLite | Depyler (Target) | Depyler (Current) |
|--------|--------|------------------|-------------------|
| Test:Code Ratio | 1000:1 | 10:1 | 2:1 |
| Branch Coverage | 100% | 100% | 80% |
| MC/DC Coverage | 100% | 100% | Unknown |
| Fuzzing | Continuous | 24×7 | None |
| Stress Tests | Days | Hours | None |
| Regression DB | Massive | Comprehensive | Partial |

**Realistic Goal**: While SQLite's 1000:1 ratio is extreme, Depyler can achieve 10:1 and still have world-class reliability for a transpiler.

---

## References

- **SQLite Testing**: https://www.sqlite.org/testing.html
- **MC/DC Coverage**: NASA standard for safety-critical software
- **Toyota Way**: docs/execution/roadmap.md
- **Ruchy Systematic Validation**: ../ruchy/docs/testing/SYSTEMATIC-VALIDATION-FRAMEWORK.md
- **CLAUDE.md**: Project development protocol

---

## Conclusion

By adopting SQLite's systematic testing philosophy, Depyler will achieve:

1. **Production-grade reliability** - Users can trust transpiled code
2. **Regression-free releases** - Bugs never reappear
3. **Predictable performance** - Benchmarks stay consistent
4. **Community confidence** - Open source quality standard

The investment in comprehensive testing pays dividends in:
- Faster development (catch bugs early)
- Easier maintenance (tests document behavior)
- User adoption (reliability breeds trust)
- Long-term sustainability (technical debt prevention)

---

**Version**: 1.0
**Created**: 2025-10-16
**Last Updated**: 2025-10-16
**Status**: SPECIFICATION - Implementation begins v3.20.0
**Owner**: Depyler Core Team
**Reviewers**: Required before implementation

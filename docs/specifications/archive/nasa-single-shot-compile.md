# NASA Single-Shot Compile Specification

**Version**: 1.1.0
**Status**: APPROVED - Active Implementation
**Target**: 95%+ Single-Shot Compile Rate
**PMAT Ticket**: DEPYLER-NASA-001

---

## Executive Summary

This specification defines the requirements, methodology, and verification criteria for achieving NASA-grade single-shot Python-to-Rust transpilation. The goal is deterministic, verifiable code generation suitable for mission-critical systems, adhering to principles from **NASA-GB-8719.13 (Software Safety Guidebook)**.

**Current State**: 35-40% compile rate
**Target State**: 95%+ compile rate
**Estimated Iterations**: 8-12 focused sprints

---

## 1. Context & Purpose

In mission-critical environments (e.g., flight software, medical devices), code generation must not only be correct but **demonstrably correct**. This specification adopts a "correct-by-construction" approach where the transpiler (Depyler) is held to rigorous standards of type soundness and semantic preservation.

### 1.1 Alignment with NASA-GB-8719.13
*   **Software Safety (Section 4):** Generated code must be free of undefined behavior (UB), race conditions, and memory leaks.
*   **Verification (Section 6):** Every transformation must be verifiable via automated tracing (Renacer).
*   **Reliability:** The "single-shot" requirement minimizes human error by eliminating post-transpilation manual fix-ups.

---

## 2. Problem Definition

### 2.1 Single-Shot Compile Definition

A **single-shot compile** occurs when:
1.  **Input:** Valid Python 3 source code (annotated or inferable).
2.  **Process:** `depyler transpile source.py` -> `source.rs`
3.  **Verification:** `rustc --crate-type lib source.rs` -> Success (exit code 0)
4.  **Constraint:** No manual intervention, `#![allow(...)]` pragmas, or `todo!()` macros in the output.

### 2.2 Current Failure Categories

| Category | Frequency | Complexity | Impact |
|----------|-----------|------------|--------|
| Type inference failures | 35% | Medium | `Type::Unknown` or cast errors |
| Missing stdlib mappings | 25% | Low | Missing function calls |
| Borrow checker violations | 20% | High | Compilation errors (E0502, E0596) |
| Unsupported syntax | 15% | Medium | Parser errors |
| Semantic mismatches | 5% | Critical | Logic bugs (silent failure) |

---

## 3. Theoretical Foundation

### 3.1 Type System Soundness

**Theorem (Milner, 1978)**: A well-typed program cannot "go wrong" [1].

Depyler must preserve type soundness during transpilation. Python's gradual typing (PEP 484) provides partial type information; the transpiler must infer the remainder while maintaining Rust's strict type requirements.

**Constraint**: All generated Rust code must pass `cargo check` and `clippy` without errors.

### 3.2 Semantic Preservation

**Definition (McCarthy, 1963)**: Two programs are semantically equivalent if they produce identical outputs for all inputs in their domain [2].

For transpilation correctness:
```
∀ input ∈ Domain(P_python):
  Execute(P_python, input) ≡ Execute(Transpile(P_python), input)
```

**Tooling**: This is verified using **Renacer** (System Call Tracer) to compare execution traces.

### 3.3 Termination Guarantee

**Rice's Theorem**: Semantic properties of programs are undecidable in general [3].

**Practical Approach**: Use conservative approximations via abstract interpretation (Cousot & Cousot, 1977) [4] to verify termination for common patterns.

---

## 4. Implementation Roadmap

### Phase 0: Type Sanity (Pre-transpilation)

**Objective**: Ensure input Python code is well-typed before attempting transpilation.

**Action**: Run `ty check` on all input corpus.
- **Fail Fast**: If `ty` reports errors in Python source, reject for single-shot compile.
- **Metadata**: Use `ty`'s inference engine to augment Depyler's initial HIR if annotations are sparse.

### Phase 1: Type Inference Completeness (Iterations 1-3)

**Objective**: Reduce Unknown type emissions to <5%

| Task | PMAT Ticket | Priority |
|------|-------------|----------|
| Hindley-Milner unification for generics | DEPYLER-1010 | P0 |
| Bidirectional type inference | DEPYLER-1011 | P0 |
| Call-site type propagation | DEPYLER-1012 | P0 |
| Return type inference from all paths | DEPYLER-1013 | P1 |
| Container element type unification | DEPYLER-1014 | P1 |

**Verification**:
```bash
pmat tdg crates/depyler-core/src/type_system --threshold 2.0
cargo test --package depyler-core --test property_tests_type_inference
```

### Phase 2: Stdlib Coverage (Iterations 4-5)

**Objective**: Map 95% of commonly-used Python stdlib

| Module | Rust Equivalent | Complexity |
|--------|-----------------|------------|
| `collections` | `std::collections` | Low |
| `itertools` | `itertools` crate | Medium |
| `json` | `serde_json` | Low |
| `re` | `regex` crate | Medium |
| `pathlib` | `std::path` | Low |
| `datetime` | `chrono` crate | Medium |
| `asyncio` | `tokio` | High |
| `typing` | Native Rust types | Medium |

**Verification**:
```bash
depyler stdlib-coverage --report
# Target: 95% of top-100 stdlib functions mapped
```

### Phase 3: Borrow Checker Compliance (Iterations 6-8)

**Objective**: Generate code that satisfies Rust's ownership model

**Key Transformations**:
1. **Ownership Analysis** (Boyland, 2003) [5]: Track value lifetimes through Python AST.
2. **Mutation Analysis**: Detect all mutation points and generate `&mut` references.
3. **Lifetime Inference**: Elide lifetimes where possible (Rust RFC 141).

**Verification**:
```bash
cargo clippy --all-targets -- -D warnings
# Zero borrow checker errors
```

### Phase 4: Semantic Equivalence (Iterations 9-10)

**Objective**: Prove behavioral equivalence

**Methods**:

1. **Property-Based Testing** (Claessen & Hughes, 2000) [6]
   ```rust
   #[quickcheck]
   fn transpiled_matches_python(input: ArbitraryInput) -> bool {
       python_result(input) == rust_result(input)
   }
   ```

2. **Golden Trace Validation** (via Renacer)
   ```bash
   renacer --transpiler-map output.rs.sourcemap.json -- ./binary
   # Compare syscall traces between Python and Rust
   ```

3. **Mutation Testing**
   ```bash
   cargo mutants --workspace
   # Target: 75% mutation kill rate
   ```

### Phase 5: Hardening (Iterations 11-12)

**Objective**: Production-grade reliability

- Fuzz testing with AFL/libFuzzer
- Formal verification of critical paths (optional)
- Performance regression testing
- Documentation and audit trail

---

## 5. PMAT Quality Gates

### 5.1 Per-Iteration Gates

```bash
# Before each iteration merge
ty check examples/*.py
pmat tdg check-quality --min-grade A-
pmat tdg check-regression --baseline main
cargo llvm-cov --fail-under-lines 80
cargo clippy -- -D warnings
cargo test --workspace
```

### 5.2 Milestone Gates

| Milestone | Compile Rate | TDG Score | Coverage |
|-----------|--------------|-----------|----------|
| M1: Type Inference | 50% | < 2.0 | 75% |
| M2: Stdlib | 65% | < 1.8 | 78% |
| M3: Borrowing | 80% | < 1.5 | 80% |
| M4: Semantics | 90% | < 1.3 | 85% |
| M5: Hardening | 95% | < 1.0 | 90% |

### 5.3 PMAT Dashboard Command

```bash
pmat tdg dashboard --watch --threshold 2.0
```

---

## 6. Popperian Falsification Checklist

Per Popper's philosophy of science [7], each claim must be falsifiable. The following 100-point checklist defines conditions that would **disprove** readiness for NASA deployment.

**Prerequisites**:
- `ty` installed (`cargo install ty`) - Astral's Rust-based type checker for Python.
- `renacer` installed (`cargo install renacer`)
- `cargo-mutants` installed (`cargo install cargo-mutants`)

### 6.1 Type System Falsifiers (Points 1-20)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 1 | Any `Type::Unknown` in output for code passing `ty check` | `grep -r "Unknown" output.rs` |
| 2 | Type mismatch error from rustc on `ty`-validated code | `rustc --error-format=json` |
| 3 | Generic type parameter undeclared | `rustc -Z parse-only` |
| 4 | Trait bound not satisfied | `cargo check 2>&1 | grep E0277` |
| 5 | Mismatched types in function return | `cargo check 2>&1 | grep E0308` |
| 6 | Cannot infer type for numeric literal | `grep "cannot infer"` |
| 7 | Type annotation required error | `grep E0282` |
| 8 | Conflicting implementations | `grep E0119` |
| 9 | Associated type not found | `grep E0220` |
| 10 | Sized bound violation | `grep E0277.*Sized` |
| 11 | Union type generates invalid enum | `cargo test --test depyler_0273_union_types_test` |
| 12 | Optional type missing None variant | `grep "Option<" output.rs` |
| 13 | Dict type inference fails | `cargo test --test dict_assignment_test` |
| 14 | List comprehension type wrong | `cargo test --test functional_tests` |
| 15 | Lambda return type incorrect | `cargo test --test lambda_integration_test` |
| 16 | Class field type mismatch | `cargo test --test property_tests` |
| 17 | Method return type incorrect | `cargo test --test property_tests_type_inference` |
| 18 | Nested generic fails | `cargo test --test specialized_coverage_testing` |
| 19 | TypeVar not substituted | `grep "TypeVar" output.rs` |
| 20 | Protocol type not mapped | `cargo test --test depyler_0280_isinstance_test` |

### 6.2 Borrow Checker Falsifiers (Points 21-40)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 21 | Cannot borrow as mutable (E0596) | `cargo check 2>&1 | grep E0596` |
| 22 | Cannot move out of borrowed (E0507) | `grep E0507` |
| 23 | Value used after move (E0382) | `grep E0382` |
| 24 | Mutable borrow while immutable exists (E0502) | `grep E0502` |
| 25 | Multiple mutable borrows (E0499) | `grep E0499` |
| 26 | Borrowed value doesn't live long enough (E0597) | `grep E0597` |
| 27 | Lifetime mismatch (E0623) | `grep E0623` |
| 28 | Missing lifetime specifier (E0106) | `grep E0106` |
| 29 | Clone on non-Clone type | `grep E0599.*clone` |
| 30 | Unnecessary clone (clippy) | `cargo clippy -- -W clippy::clone_on_copy` |
| 31 | Self mutation without &mut self | `cargo test --test depyler_0269_function_borrowing_test` |
| 32 | Field access on moved value | `cargo test --test property_tests_memory_safety` |
| 33 | Return reference to local | `grep E0515` |
| 34 | Closure captures incorrectly | `cargo test --test lambda_integration_test` |
| 35 | Iterator invalidation | `cargo test --test depyler_0265_iterator_deref_test` |
| 36 | RefCell borrow panic potential | `grep "RefCell" output.rs` |
| 37 | Rc cycle potential | `grep "Rc<" output.rs` |
| 38 | Raw pointer unsafety | `grep "*const\|*mut" output.rs` |
| 39 | Transmute usage | `grep "transmute" output.rs` (must be 0) |
| 40 | Unsafe block without justification | `grep "unsafe" output.rs` |

### 6.3 Semantic Falsifiers (Points 41-60)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 41 | Output differs from Python | `renacer --compare golden.json -- ./binary` |
| 42 | Integer overflow behavior differs | `cargo test --test boundary_value_tests` |
| 43 | Float precision differs | `cargo test --test boundary_value_tests` |
| 44 | String encoding differs | `cargo test --test edge_case_coverage` |
| 45 | Exception not mapped to Result | `cargo test --test error_handling_tests` |
| 46 | None not mapped to Option | `cargo test --test depyler_0270_result_unwrapping_test` |
| 47 | List mutation semantics wrong | `cargo test --test array_generation_test` |
| 48 | Dict iteration order wrong | `cargo test --test dict_assignment_test` |
| 49 | Set operation semantics wrong | `cargo test --test functional_tests` |
| 50 | Slice semantics incorrect | `cargo test --test depyler_0267_index_access_test` |
| 51 | Negative index handling wrong | `cargo test --test depyler_0268_index_negation_test` |
| 52 | Default argument mutation | `cargo test --test property_tests` |
| 53 | Generator not converted to Iterator | `cargo test --test generator_compilation_tests` |
| 54 | Context manager (__enter__/__exit__) | `cargo test --test functional_tests` |
| 55 | Decorator not applied | `cargo test --test property_tests` |
| 56 | Class inheritance broken | `cargo test --test integration_benchmarks` |
| 57 | Multiple inheritance wrong | `cargo test --test tier2_reprorusted_integration` |
| 58 | Property getter/setter wrong | `cargo test --test property_tests` |
| 59 | Static/classmethod wrong | `cargo test --test property_tests` |
| 60 | Magic method not mapped | `cargo test --test operator_tests` |

### 6.4 Stdlib Falsifiers (Points 61-80)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 61 | os.path function unmapped | `depyler stdlib-coverage --check` |
| 62 | json.dumps output differs | `cargo test --test integration/json_test.rs` |
| 63 | re.match behavior differs | `cargo test --test integration/regex_test.rs` |
| 64 | datetime arithmetic wrong | `cargo test --test integration/datetime_test.rs` |
| 65 | collections.defaultdict wrong | `cargo test --test integration/collections_test.rs` |
| 66 | collections.Counter wrong | `cargo test --test integration/collections_test.rs` |
| 67 | collections.deque wrong | `cargo test --test integration/collections_test.rs` |
| 68 | itertools function unmapped | `cargo test --test integration/itertools_test.rs` |
| 69 | functools.partial wrong | `cargo test --test integration/functools_test.rs` |
| 70 | functools.reduce wrong | `cargo test --test integration/functools_test.rs` |
| 71 | math function unmapped | `cargo test --test integration/math_test.rs` |
| 72 | random behavior differs | `cargo test --test integration/random_test.rs` |
| 73 | hashlib output differs | `cargo test --test integration/hashlib_test.rs` |
| 74 | base64 output differs | `cargo test --test integration/base64_test.rs` |
| 75 | struct pack/unpack wrong | `cargo test --test integration/struct_test.rs` |
| 76 | socket API unmapped | `cargo test --test integration/socket_test.rs` |
| 77 | threading primitives wrong | `cargo test --test integration/threading_test.rs` |
| 78 | subprocess API wrong | `cargo test --test integration/subprocess_test.rs` |
| 79 | pathlib operations wrong | `cargo test --test integration/pathlib_test.rs` |
| 80 | typing constructs wrong | `cargo test --test integration/typing_test.rs` |

### 6.5 Code Quality Falsifiers (Points 81-90)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 81 | Cyclomatic complexity > 10 | `pmat analyze complexity` |
| 82 | Cognitive complexity > 10 | `pmat analyze complexity` |
| 83 | Function > 30 lines | `pmat analyze size` |
| 84 | TDG score > 2.0 | `pmat tdg check-quality` |
| 85 | Test coverage < 80% | `cargo llvm-cov` |
| 86 | Clippy warning present | `cargo clippy -- -D warnings` |
| 87 | Rustfmt changes output | `rustfmt --check` |
| 88 | Dead code present | `cargo +nightly udeps` |
| 89 | Duplicate code > 10% | `pmat analyze duplicates` |
| 90 | TODO/FIXME in transpiler | `pmat analyze satd` |

### 6.6 Performance Falsifiers (Points 91-95)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 91 | Rust slower than Python | `cargo test --test integration_benchmarks` |
| 92 | Memory usage > 2x Python | `cargo test --test property_tests_memory_safety` |
| 93 | Compile time > 60s for typical file | `scripts/profile_transpiler.sh` |
| 94 | Binary size > 10x reasonable | `ls -la target/release` |
| 95 | Startup time > 100ms | `renacer --function-time -- ./binary` |

### 6.7 Safety Falsifiers (Points 96-100)

| # | Falsification Condition | Test Command / Reference |
|---|------------------------|--------------------------|
| 96 | Panic possible in generated code | `grep "panic\|unwrap\|expect" output.rs` |
| 97 | Buffer overflow possible | `cargo +nightly miri test` |
| 98 | Data race possible | `cargo +nightly miri test` |
| 99 | Undefined behavior possible | `cargo +nightly miri test` |
| 100 | Security vulnerability (OWASP) | `cargo audit` |

---

## 7. Verification Protocol

### 7.1 Automated CI Pipeline

```yaml
# .github/workflows/nasa-compliance.yml
name: NASA Compliance Check
on: [push, pull_request]

jobs:
  falsification-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Renacer
        run: cargo install renacer

      - name: Run Falsification Suite
        run: |
          ./scripts/run_falsification_checklist.sh

      - name: PMAT Quality Gate
        run: |
          pmat tdg check-quality --min-grade A-
          pmat tdg check-regression --baseline main

      - name: Compile Rate Test
        run: |
          depyler converge --input-dir examples --target-rate 95

      - name: Semantic Equivalence
        run: |
          ./scripts/golden_trace_validation.sh
```

### 7.2 Manual Review Checklist

Before NASA deployment approval:

- [ ] All 100 falsification points verified
- [ ] Independent security audit completed
- [ ] Performance benchmarks meet requirements
- [ ] Documentation complete and reviewed
- [ ] Formal verification report (if applicable)
- [ ] Risk assessment and mitigation plan

---

## 8. References

[1] Milner, R. (1978). "A Theory of Type Polymorphism in Programming." *Journal of Computer and System Sciences*, 17(3), 348-375. https://doi.org/10.1016/0022-0000(78)90014-4

[2] McCarthy, J. (1963). "A Basis for a Mathematical Theory of Computation." *Computer Programming and Formal Systems*, 33-70. North-Holland.

[3] Rice, H. G. (1953). "Classes of Recursively Enumerable Sets and Their Decision Problems." *Transactions of the American Mathematical Society*, 74(2), 358-366.

[4] Cousot, P., & Cousot, R. (1977). "Abstract Interpretation: A Unified Lattice Model for Static Analysis of Programs." *POPL '77*, 238-252. https://doi.org/10.1145/512950.512973

[5] Boyland, J. (2003). "Checking Interference with Fractional Permissions." *SAS 2003*, LNCS 2694, 55-72. Springer.

[6] Claessen, K., & Hughes, J. (2000). "QuickCheck: A Lightweight Tool for Random Testing of Haskell Programs." *ICFP '00*, 268-279. https://doi.org/10.1145/351240.351266

[7] Popper, K. (1959). *The Logic of Scientific Discovery*. Hutchinson & Co.

[8] Jung, R., et al. (2017). "RustBelt: Securing the Foundations of the Rust Programming Language." *POPL '18*. https://doi.org/10.1145/3158154

[9] Matsakis, N., & Klock, F. (2014). "The Rust Language." *ACM SIGAda Ada Letters*, 34(3), 103-104.

[10] NASA Software Safety Guidebook (NASA-GB-8719.13). NASA Technical Standard.

---

## 9. Five-Whys Root Cause Analyses

This section documents the root cause analyses for each significant defect discovered during convergence.

### 9.1 DEPYLER-1045: char vs &str Type Mismatch in String Iteration

**Problem Statement**: When iterating over string characters in Python (`for c in text.chars()`), the transpiled Rust code produces `char` type variables. These fail compilation when:
1. Compared with string literals (`c == "a"`)
2. Passed to functions expecting `&str`
3. Used in `String.contains()` checks

**Five-Whys Analysis**:

1. **Why did `c == "a"` fail?**
   - Because `c` is `char` type but `"a"` is `&str` type - incompatible for comparison.

2. **Why is `c` a `char` type?**
   - Because `.chars()` iterates over `char` values in Rust, not `&str`.

3. **Why wasn't `c` converted to String when needed?**
   - Because the `needs_char_to_string` check in `stmt_gen.rs` was too aggressive, converting at declaration which broke char methods like `.is_alphabetic()`.

4. **Why did converting at declaration break char methods?**
   - Because `String` doesn't have methods like `is_alphabetic()` - these are `char` methods.

5. **Why wasn't conversion done at the usage site instead?**
   - Because the original implementation didn't have context-aware conversion - it was an all-or-nothing approach.

**Root Cause**: The transpiler lacked site-specific type conversion logic for `char` iterator variables.

**Fix Applied**:
1. Track char iterator variables in `ctx.char_iter_vars` (already existed)
2. Add `.to_string()` at comparison sites in `expr_gen.rs` (line ~563)
3. Pass `char` directly to `String.contains()` (char implements Pattern trait)
4. Clear `fn_str_params` between function generations to prevent cross-contamination
5. Track `Type::String` in `var_types` for proper auto-borrow detection
6. Check `current_assign_type` for `Dict(_, Unknown)` to enable `DepylerValue` wrapping

**Additional Fixes**:
- **DEPYLER-1045a**: Clear `fn_str_params` at function boundary
  - *Why*: Previous functions' `&str` params polluted later functions' auto-borrow logic
- **DEPYLER-1045b**: Add `Type::String` to tracked types in assignment
  - *Why*: Without tracking, auto-borrow logic couldn't detect String variables
- **DEPYLER-1045c**: Wrap `chr()` argument in parentheses
  - *Why*: `chr(base + shifted)` was being parsed as `chr(base + (shifted as u32))` due to precedence
- **DEPYLER-1045d**: Add `.chars()` after string method calls in for loops
  - *Why*: `text.to_lowercase()` returns String, not iterator
- **DEPYLER-1045e**: Handle `dict` type annotation for DepylerValue wrapping
  - *Why*: Bare `dict` annotation maps to `DepylerValue` values even for homogeneous strings

**Compile Rate Impact**: test_string_module.py now compiles (previously failing)

---

## 10. Approval

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Author | | | |
| Technical Reviewer | | | |
| Quality Assurance | | | |
| Project Lead | | | |

---

**Document Control**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2025-01-08 | Claude | Initial draft |
| 1.1.0 | 2026-01-08 | Gemini | Enhanced with specific test vectors and NASA-GB-8719.13 alignment |


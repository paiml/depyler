# Multi-Corpus Convergence with Concurrent Oracle Training

**Version**: 1.2.0
**Date**: 2026-02-01
**Status**: PROPOSED
**Authors**: Depyler Team
**Ticket**: DEPYLER-CONVERGE-MULTI

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Corpora Under Test](#2-corpora-under-test)
3. [Goal 1: Multi-Corpus Single-Shot Compile](#3-goal-1-multi-corpus-single-shot-compile)
4. [Goal 2: Concurrent Oracle Training](#4-goal-2-concurrent-oracle-training)
5. [Goal 3: PMAT Comply A+ Grade](#5-goal-3-pmat-comply-a-grade)
6. [Goal 4: FAST Coverage at 95%](#6-goal-4-fast-coverage-at-95)
7. [Falsification Framework](#7-falsification-framework)
8. [Architecture](#8-architecture)
9. [Implementation Phases](#9-implementation-phases)
10. [Risk Register](#10-risk-register)
11. [References](#11-references)

---

## 1. Executive Summary

This specification defines a unified convergence campaign that targets five
independent Python corpora simultaneously, trains the oracle on the combined
error signal, raises PMAT compliance to A+, and holds FAST-tier test coverage
at 95%. Each goal carries explicit Popperian falsification criteria so that
progress is measured by attempted refutations rather than confirmations.

### Current State (2026-02-02, iter 17) -- MEASURED

| Metric | Current | Target | Gap |
|--------|---------|--------|-----|
| Single-shot compile (internal) | 80% (256/320) | 80% | Met |
| Single-shot compile (reprorusted-std-only) | **92.7% (38/41)** | 80% | **Met (+12.7 pp)** |
| Single-shot compile (fully-typed-reprorusted) | **62.5% (10/16)** | 60% | **Met (+2.5 pp)** |
| Single-shot compile (hugging-face-gtc) | **4.7% (6/128)** | 40% | 35.3 pp |
| Single-shot compile (jax-gtc) | **0% (0/7)** | 40% | 40 pp |
| Single-shot compile (algorithm-comp) | **47.5% (48/101)** | 40% | **Met (+7.5 pp)** |
| Oracle accuracy | 85% | 92% | 7 pp |
| PMAT TDG grade | B+ | A+ | 2 notches |
| FAST coverage | ~60% | 95% | ~35 pp |

### Progress Log

| Iteration | Date | Tier 1 | Tier 2 | Tier 3 | Commit | Notes |
|-----------|------|--------|--------|--------|--------|-------|
| 0 (baseline) | 2026-01-31 | 0/20 (0%) | 0/15 (0%) | 0/128 (0%) | 09af6141 | Initial measurement (incorrect file counts) |
| 1 | 2026-01-31 | 18/68 (26%) | 2/23 (8%) | 14/277 (5%) | b9c25bc9 | UnionType/enum/macro fixes; corrected file counts |
| 2 | 2026-01-31 | 30/68 (44%) | 5/23 (21%) | 16/277 (5%) | 635db9f7 | Expand UnionType/enum/macro fixes |
| 3 | 2026-01-31 | 30/68 (44%) | 5/23 (21%) | 16/277 (5%) | 19412a1e | TYPE_CHECKING/__name__/Sequence fallbacks; no rate change |
| 4 | 2026-01-31 | **7/20 (35%)** | **2/20 (10%)** | **0/261 (0%)** | 0eecb875 | Corrected file counts, TMPDIR fix, line-based TYPE_CHECKING filter |
| 5 | 2026-01-31 | 10/20 (50%) | 2/15 (13.3%) | 0/128 (0%) | 74717e35 | io.StringIO, docstring, operator fixes; correct Tier 2=15, Tier 3=128 |
| 5b | 2026-01-31 | 10/20 (50%) | 2/15 (13.3%) | 0/128 (0%) | 90ac176a | Cursor Write import, HashMap injection |
| 6 | 2026-01-31 | 12/20 (60%) | 2/15 (13.3%) | 0/128 (0%) | f14bd685 | Truthiness, chain-iter, pathlib, type(), enum fixes |
| 7 | 2026-01-31 | **19/20 (95%)** | **2/15 (13.3%)** | **0/128 (0%)** | b91fef3e | 7 convergence fixes + bool truthiness type-awareness; **Tier 1 TARGET MET** |
| 8 | 2026-01-31 | **19/20 (95%)** | **9/15 (60%)** | **0/128 (0%)** | ae8d0cbf | HashMap, PathOrStringUnion, struct stub, dict insert fixes; **Tier 2 TARGET MET** |
| 9 | 2026-01-31 | 19/20 (95%) | 9/15 (60%) | 0/128 (0%) | (merged into iter10) | bool truthiness, sorted_vec, field access fixes |
| 10 | 2026-01-31 | **17/20 (85%)** | **6/15 (40%)** | **1/128 (0.8%)** | 33c56447 | Vec contains deref, membership check, float/int comparison; **first Tier 3 file compiles (training/trl.py)**; NOTE: Tier 1/2 regression from measurement methodology correction |
| 11 | 2026-02-01 | 17/20 (85%) | **10/16 (62%)** | **3/128 (2.3%)** | acb71446 | Enum Display (..) pattern, borrowed alias .clone(), deref string comparison, Vec<DepylerValue>.join(); **Tier 2 target met at 62%**; 2 new Tier 3 files (hub/collections, inference/streaming) |
| 12 | 2026-02-01 | 17/20 (85%) | **9/15 (60%)** | **4/128 (3.1%)** | 7c349b37 | !String truthiness, r#false/r#true, deref unwrap, &str→.to_string() in ::new(), DepylerValue::Str clone, [String].contains→[&str], &Option deref in ::new(), (*ref_option).unwrap; **+1 new Tier 3: inference/optimization** |
| 13 | 2026-02-01 | 17/20 (85%) | 9/15 (60%) | 4/128 (3.1%) | 97f582d2 | DepylerValue::from(Enum) → DepylerValue::Str(format!), From<Enum> impls, validate_not_none 2-arg fix + turbofish, CSE py_mul .into() removal; error count reduced (rag/indexing 7→2, audio/music 14→5) but no new files flipped |
| 14 | 2026-02-01 | 17/20 (85%) | 9/15 (60%) | **5/128 (3.9%)** | 97f582d2 | Multi-line .into() removal in py_mul/py_div chains, tuple(T,DV)→Vec<T> when .len() called; **+1 new Tier 3: deployment/optimization** |
| 15 | 2026-02-01 | 17/20 (85%) | 9/15 (60%) | **6/128 (4.7%)** | d7a6c265 | Type-aware CSE int/float literal fix, Vec.to_string()→.clone(), mixed numeric min/max; **+1 new Tier 3: agents/planning** (inference/embeddings has pre-existing `(f64>` codegen bug); Tier 5 baseline: 19/80 (23.7%); Wolfram reducibility framework added to spec |
| 16 | 2026-02-02 | **38/41 (92.6%)** | 9/15 (60%) | 6/128 (4.7%) | 7c6117bc | Bitwise AND truthiness fix (`if x & N {` → `if (x & N) != 0 {`), Result double-wrapping fix (`Ok(fn()?)`); reverted broken .to_i64() blanket replace; **Tier 1 re-counted to 41 files** (3 transpile failures); **Tier 5: 21/80 (26.2%)** (+2 files: bounds, power now compile) |
| 17 | 2026-02-02 | **38/41 (92.7%)** | **10/16 (62.5%)** | 6/128 (4.7%) | (pending) | Multi-line Result signature detection (`has_result_return_multiline`), generic function name extraction fix (strip `<'a, T>`), `find_call_close_paren` for nested calls, `Ok(!fname(` pattern, trailing comma in arith parens, `&ref → &mut ref` call-site fix, regex match `.to_string()` fix; **Tier 2 +1 file (62.5%)**; **Tier 5: 48/101 (47.5%)** (+27 files, corpus re-counted to 101) |

**Measurement methodology notes**:
- (iter 3) `depyler transpile` writes .rs files alongside .py files.
  Measurements must read from the on-disk .rs file, NOT capture stdout.
- (iter 4) **File count correction**: Previous iterations used broader `find`
  that included test files and deeper nesting. Correct counts using
  `find $CORPUS -name "*.py" -not -name "__init__.py"`:
  Tier 1 = 20, Tier 2 = 15, Tier 3 = 128 (corrected from 20/261 at iter 5).
- (iter 4) **TMPDIR fix**: `rustc` requires a writable temp directory.
  Sandbox environments block `/tmp` access. Set `TMPDIR` to scratchpad.
- (iter 4) **Stale binary**: `CARGO_TARGET_DIR=/Volumes/LambdaCache/cargo-target`
  means `cargo build --release` writes to that location, NOT `./target/release/`.
  Always copy: `cp /Volumes/LambdaCache/cargo-target/release/depyler ./target/release/`

### Root Cause Analysis (Revised 2026-01-31, iter 7)

**Tier 1 error distribution** (20 files, 19 compiling = 95%):

| Status | Files | Description |
|--------|-------|-------------|
| Compiling | 19 | All stdlib modules except `re` |
| No .rs generated | 1 | `re` -- regex module unsupported |

**Tier 2 error distribution** (15 files, 9 compiling = 60%):

| Status | Files | Description |
|--------|-------|-------------|
| Compiling | 9 | check_test_lib_crates, export_hf_corpus, generate_insights, golden_traces_analyzer, hitl_sampler, verify_qa_checklist, category_diff, corpus_quality_report, zero_success_analyzer |
| `E0308: mismatched types` | 2 | clippy_gate, measure_compile_rate (heterogeneous dict values) |
| `E0308+E0061` | 2 | augment_corpus (class type), label_corpus (constructor args) |
| No .rs generated | 2 | synthetic_augmenter (3-arg replace), weak_supervision (panic) |

**Tier 3 error distribution** (128 files, 1 compiling = 0.8%):

| Error Class | Count | Description |
|-------------|-------|-------------|
| `E0308: mismatched types` | ~1160 | Dominant: &String vs &str, DepylerValue vs String, f64 vs int |
| `E0599: method not found` | ~551 | Vec\<DepylerValue\>.join(), Enum::new(), .is_none() on non-Option |
| `E0277: trait not satisfied` | ~424 | Vec indexing with &String, Display trait |
| `E0282: type annotation needed` | ~85 | Inference failures |
| `E0425: cannot find value` | 449 | Missing scope references |
| Other | ~1046 | Various (E0433, E0061, etc.) |

Closest to compiling (Tier 3): training/trl.py, deployment/optimization.py, agents/planning.py, inference/embeddings.py, hub/collections.py, inference/streaming.py, inference/optimization.py (all COMPILE); inference/batch.py (5 errors), rag/indexing.py (3 errors), audio/music.py (5 errors)

**Tier 4 error distribution** (7 files, 0 compiling = 0%):

| Error Class | Count | Description |
|-------------|-------|-------------|
| `E0433: unresolved module` | ~20 | `jax`, `jnp` modules have no Rust mapping |
| `E0599: method not found` | ~25 | `Array.shape`, `.reshape()`, `.at` etc. |
| `E0308: mismatched types` | ~15 | `Array` vs concrete numeric types |
| `E0562: impl Trait` | ~5 | `impl Fn()` in local variable positions |
| `E0425: cannot find value` | ~10 | Unmapped kwargs (`static_argnums`, `axis_name`) |
| Transpile panic | 1 | `neural/layers.py` -- `where` keyword in identifier position |

Root cause: JAX APIs (`jax.grad`, `jax.jit`, `jax.vmap`, `jnp.array`) are
entirely unmapped. Requires a dedicated JAX-to-Rust stdlib mapping layer.

**Tier 5 error distribution** (80 files, 19 compiling = 23.7%):

| Error Class | Count | Description |
|-------------|-------|-------------|
| `E0308: mismatched types` | ~180 | Generic type propagation, `Vec<T>` vs concrete, return type mismatch |
| `E0609: private field` | ~126 | Struct fields not marked `pub` in generated code |
| `E0277: trait not satisfied` | ~118 | Missing Clone, Copy, PartialEq derives for data structures |
| `E0599: method not found` | ~80 | Missing methods on generated struct types |
| `E0282: type annotation needed` | ~26 | Type inference failures in complex expressions |
| Other | ~79 | E0425, E0061, E0433, etc. |
| Transpile failure | 1 | `heap/kth_element` -- depyler crash |

Root cause: Categories with 0% pass rate (graph, tree, linked_list, matrix,
backtracking, divide_and_conquer, interval) all involve recursive data
structures (`Optional[ListNode]`, `Optional[TreeNode]`), adjacency lists,
or multi-dimensional array indexing. The zero-dependency nature isolates
these as core transpiler limitations rather than library mapping gaps.
| `E0433: unresolved module` | 39 | Undeclared crates/modules |
| `E0599: method not found` | 36 | Methods on wrong types |
| `E0573: expected type found variant` | 34 | Struct/enum confusion |
| `E0423: expected value found struct` | 25 | Enum path separator (dot vs ::) |
| `E0277: trait bound not satisfied` | 22 | Missing trait implementations |
| Transpile failure | 18 | Crashes (2 abort, 16 error) |
| Syntax: unbalanced delimiters | 18 | 9 `}`, 8 `)`, 1 `}` mismatch |
| `E0432: unresolved import` | 5 | md5, hmac, etc. |
| `E0061: wrong arg count` | 2 | Function arity mismatch |
| `E0562: impl Trait` | 1 | impl Trait in wrong position |
| `E0416: duplicate binding` | 1 | Pattern binding error |

### Governing Epistemology

> "The criterion of the scientific status of a theory is its
> falsifiability, or refutability, or testability."
> -- Karl R. Popper, *Conjectures and Refutations* (1963), p. 37

Every goal in this specification is stated as a **bold conjecture** paired
with a **concrete falsifier**: an observable outcome that, if produced,
forces us to revise the conjecture. Goals that cannot be falsified are
excluded on principle (Popper, 1959, Section 6).

---

## 2. Corpora Under Test

### 2.1 reprorusted-std-only (Tier 1 -- Stdlib)

| Property | Value |
|----------|-------|
| Location | `~/src/reprorusted-std-only` |
| Python files | 68 (43 non-test) |
| LOC | ~1,310 |
| Complexity | Low -- stdlib only, zero third-party imports |
| Quality gates | 6-gate Jidoka (includes AST gate-0 for import purity) |
| Test coverage | 100% (182 tests) |
| Categories | 20 stdlib domains (builtins, collections, itertools, ...) |

**Transpilation prognosis**: Highest success probability. The zero-dependency
constraint aligns with depyler's existing `std`-mapping pipeline. Most
patterns (builtins, pathlib, json, csv) already have codegen rules.

### 2.2 fully-typed-reprorusted-python-cli (Tier 2 -- Typed Pipeline)

| Property | Value |
|----------|-------|
| Location | `~/src/fully-typed-reprorusted-python-cli` |
| Python files | 23 (16 non-test) |
| LOC | ~1,170 |
| Complexity | Moderate -- pipeline infrastructure, weak supervision |
| Quality gates | 5-gate Jidoka |
| Test coverage | 100% (152 tests) |
| Key modules | weak_supervision, synthetic_augmenter, corpus_quality_report |

**Transpilation prognosis**: Moderate difficulty. Full type annotations
reduce inference ambiguity, but modules like `weak_supervision.py` (8,200
LOC) stress function-size limits and complex dict/dataclass patterns.

### 2.3 hugging-face-ground-truth-corpus (Tier 3 -- Enterprise ML)

| Property | Value |
|----------|-------|
| Location | `~/src/hugging-face-ground-truth-corpus` |
| Python files | 277 (143 non-test) |
| LOC | ~148,000 |
| Complexity | High -- ML framework with 14 categories |
| Quality gates | 5-gate Jidoka |
| Test coverage | 98%+ (16,000 tests) |
| Categories | hub, inference, preprocessing, training, evaluation, ... |

**Transpilation prognosis**: Lowest initial success rate. Heavy use of
third-party ML libraries (torch, transformers, datasets) that have no Rust
equivalents. The primary value is in the **error signal**: compile failures
from this corpus train the oracle on unsupported-library patterns that would
otherwise be unobserved.

### 2.4 jax-ground-truth-corpus (Tier 4 -- JAX/ML)

| Property | Value |
|----------|-------|
| Location | `~/src/jax-ground-truth-corpus` |
| Python files | 7 (in `src/jax_gtc/`) |
| LOC | ~2,042 |
| Complexity | High -- JAX ML framework with autodiff, distributed computing |
| Categories | 7: arrays, autodiff, distributed, neural, random, training, transforms |
| Key modules | operations, derivatives, parallel, layers, prng, optimizers, core |

**Category breakdown**:

| Category | Module | Description |
|----------|--------|-------------|
| `arrays` | `operations.py` | JAX array operations, broadcasting, reshaping |
| `autodiff` | `derivatives.py` | Automatic differentiation, gradients, Jacobians |
| `distributed` | `parallel.py` | Multi-device parallelism, sharding, pmap |
| `neural` | `layers.py` | Neural network layers, activation functions |
| `random` | `prng.py` | PRNG key management, random sampling |
| `training` | `optimizers.py` | Gradient descent, Adam, learning rate schedules |
| `transforms` | `core.py` | JIT compilation, vmap, pmap transforms |

**Transpilation prognosis**: JAX relies heavily on NumPy-like array semantics,
functional transformations (jit, vmap, grad), and hardware-agnostic dispatch.
These patterns stress the transpiler's ability to handle higher-order functions,
array broadcasting, and numerical computing idioms. Smaller corpus size (7 files)
makes it tractable for rapid iteration while providing structurally distinct
error signal from the HuggingFace corpus.

### 2.5 algorithm-competition-corpus (Tier 5 -- Algorithmic)

| Property | Value |
|----------|-------|
| Location | `~/src/algorithm-competition-corpus` |
| Python files | 80 (in `src/algorithm_corpus/`) |
| LOC | ~10,457 |
| Complexity | Moderate -- pure algorithmic Python with data structures, recursion |
| Categories | 20: backtracking, binary_search, bit_manipulation, divide_and_conquer, dynamic_programming, graph, greedy, heap, heap_priority_queue, interval, linked_list, math, matrix, number, queue, sliding_window, sorting, stack, string, tree, two_pointers |

**Category breakdown**:

| Category | Files | Description |
|----------|-------|-------------|
| `backtracking` | 4 | N-queens, permutations, subsets, sudoku |
| `binary_search` | 5 | Binary search, bounds, peak finding, rotated array |
| `bit_manipulation` | 3 | Basic/advanced bit ops, bitwise tricks |
| `divide_and_conquer` | 3 | Merge sort, max subarray, binary search DC |
| `dynamic_programming` | 7 | Fibonacci, coin change, edit distance, knapsack, LCS, LIS, word break |
| `graph` | 9 | BFS, DFS, Dijkstra, Bellman-Ford, topological sort, union-find, cycle detection |
| `greedy` | 4 | Activity selection, interval scheduling, coin, jump game |
| `heap` | 3 | Heap ops, kth element, kth problems |
| `interval` | 2 | Interval operations, scheduling |
| `linked_list` | 4 | Node, operations, advanced, problems |
| `math` | 5 | GCD, primes, power, combinatorics, geometry |
| `matrix` | 3 | Matrix ops, path finding, search |
| `number` | 2 | Digit ops, conversion |
| `queue` | 2 | Sliding max, queue reconstruction |
| `sliding_window` | 2 | Kadane, window patterns |
| `sorting` | 5 | Merge, quick, heap, counting, radix sort |
| `stack` | 3 | Monotonic stack, expressions, parentheses |
| `string` | 4 | KMP, Rabin-Karp, longest substring, string ops |
| `tree` | 5 | BST, traversal, properties, LCA, tree node |
| `two_pointers` | 5 | Two sum, container, palindrome, partition, subarray |

**Transpilation prognosis**: This corpus exercises pure algorithmic patterns --
recursion, dynamic programming, graph traversal, tree manipulation -- without
third-party library dependencies. The challenge is structural: recursive data
types (`Optional[ListNode]`, `Optional[TreeNode]`), nested list/matrix indexing,
and complex control flow (backtracking with pruning). The zero-dependency nature
means compilation failures indicate core transpiler limitations rather than
missing library mappings. At 80 files across 20 categories, this provides the
broadest single-tier category distribution.

**Baseline measurement** (2026-02-01): **19/80 (23.7%)**

Per-category pass rates:
| Category | Pass/Total | Rate |
|----------|-----------|------|
| binary_search | 4/5 | 80% |
| bit_manipulation | 2/3 | 67% |
| greedy | 2/4 | 50% |
| number | 1/2 | 50% |
| queue | 1/2 | 50% |
| sliding_window | 1/2 | 50% |
| two_pointers | 2/5 | 40% |
| heap | 1/3 | 33% |
| stack | 1/3 | 33% |
| string | 1/4 | 25% |
| math | 1/5 | 20% |
| sorting | 1/5 | 20% |
| dynamic_programming | 1/7 | 14% |
| backtracking | 0/4 | 0% |
| divide_and_conquer | 0/3 | 0% |
| graph | 0/9 | 0% |
| interval | 0/2 | 0% |
| linked_list | 0/4 | 0% |
| matrix | 0/3 | 0% |
| tree | 0/5 | 0% |

---

## 3. Goal 1: Multi-Corpus Single-Shot Compile

### 3.1 Conjecture

> **C1**: Depyler can achieve single-shot compilation rates of 80% (Tier 1),
> 60% (Tier 2), 40% (Tier 3), 40% (Tier 4), and 40% (Tier 5) within a bounded
> number of UTOL iterations against five structurally distinct Python corpora.

### 3.2 Rationale

Single-corpus optimization risks overfitting the transpiler to one
distribution of Python idioms. Ohno (1988) warns against *muda* (waste) from
localized optimization that degrades global throughput: "The worst waste is
the waste of overproduction -- producing things that are not needed" (p. 19).
By converging against five corpora simultaneously, we follow the Toyota
Production System principle of *heijunka* (production leveling) applied to
error-class distributions (Liker, 2004, Principle 4).

### 3.3 Falsification Criteria (Popper)

Each falsifier is a **modus tollens** test: if the predicted outcome does not
obtain, the conjecture is refuted and must be revised.

| ID | Falsifier | Observable | Threshold | Popper Category |
|----|-----------|------------|-----------|-----------------|
| F1.1 | Tier 1 plateau | Compile rate stalls below 80% for 10 consecutive UTOL iterations | < 80% after iter 50 | Basic statement (Popper, 1959, Sec. 28) |
| F1.2 | Tier 2 regression | Tier 2 rate drops > 5 pp while optimizing for Tier 1 | delta < -5 pp | Corroboration degree (Popper, 1963, Ch. 10) |
| F1.3 | Tier 3 zero signal | No new error categories discovered from Tier 3 after 20 iterations | new_categories == 0 | Informative content (Popper, 1959, Sec. 35) |
| F1.4 | Cross-corpus interference | Fixing Tier 3/4 errors introduces regressions in Tier 1 | Tier 1 delta < -2 pp per fix batch | Severity of test (Popper, 1963, Ch. 10) |
| F1.5 | Tier 4 structural divergence | JAX corpus errors share < 30% overlap with Tier 3 error classes | overlap < 0.30 | Degree of falsifiability (Popper, 1959, Sec. 36) |
| F1.6 | Tier 5 recursive-type ceiling | Recursive data structure categories (tree, linked_list, graph) plateau below 20% compile rate despite 10 iterations targeting them | rate < 20% after 10 targeted iters | Empirical content (Popper, 1959, Sec. 35) |

### 3.4 Success Metrics

```
Tier 1 (std-only):     ████████████████████████████████████████ 80%
Tier 2 (typed-cli):    ████████████████████████████████         60%
Tier 3 (hf-corpus):    ████████████████████                     40%
Tier 4 (jax-gtc):      ████████████████████                     40%
Tier 5 (algo-comp):    ████████████████████                     40%
Combined (weighted):   ████████████████████████████████████     ~52%
```

### 3.5 Measurement Protocol

```bash
# Per-corpus measurement (deterministic, reproducible)
depyler converge --corpus ~/src/reprorusted-std-only \
  --target-rate 0.80 --max-iterations 50 --seed 42

depyler converge --corpus ~/src/fully-typed-reprorusted-python-cli \
  --target-rate 0.60 --max-iterations 50 --seed 42

depyler converge --corpus ~/src/hugging-face-ground-truth-corpus \
  --target-rate 0.40 --max-iterations 50 --seed 42

depyler converge --corpus ~/src/jax-ground-truth-corpus \
  --target-rate 0.40 --max-iterations 50 --seed 42

depyler converge --corpus ~/src/algorithm-competition-corpus \
  --target-rate 0.40 --max-iterations 50 --seed 42

# Cross-corpus regression gate
depyler converge --corpus ~/src/reprorusted-std-only \
  --regression-baseline baseline_tier1.json
```

### 3.6 Scholarly Grounding

The multi-corpus approach follows established compiler-testing methodology.
Yang et al. (2011) demonstrated that Csmith's randomized testing found 325
bugs in production C compilers by varying input distributions. Our tiered
corpus strategy is a structured analogue: each tier provides a distinct input
distribution that exercises different transpiler subsystems.

> "Testing a compiler with diverse programs is essential because bugs tend to
> cluster around unusual combinations of features."
> -- Yang, Chen, Eide, & Regehr (2011), *Finding and Understanding Bugs in
> C Compilers*, PLDI '11, p. 283

Le Goues et al. (2012) showed that automated program repair (GenProg) achieves
higher patch quality when the test suite covers multiple failure modes. Our
multi-corpus convergence extends this to transpiler repair: the oracle learns
richer error-to-fix mappings when training data spans stdlib, typed, and ML
domains.

> "Expressive test suites improve the quality of automatically generated
> patches by reducing the likelihood of overfitting to a single test."
> -- Le Goues, Nguyen, Forrest, & Weimer (2012), *GenProg: A Generic Method
> for Automatic Software Repair*, IEEE TSE 38(1), p. 62

### 3.7 Computational Reducibility and Transpilation Limits

Wolfram's (2025) systematic study of computational reducibility across all
Turing machines of bounded size provides theoretical grounding for a central
question in transpilation: *when does translating from one computational model
to another yield meaningful speedup, and when is improvement bounded?*

Wolfram demonstrates that transitioning from a smaller machine class (s=3) to
a larger one (s=4) sometimes produces exponential-to-constant speedup (e.g.,
machine 1342057), but sometimes yields zero improvement (e.g., machine
600720). This maps directly to transpilation: some Python patterns
(interpreter overhead, GIL contention, dynamic dispatch) yield dramatic Rust
speedups, while others (inherently sequential algorithms, I/O-bound code)
yield at most constant-factor improvement. The irreducibility result provides
theoretical grounding for why transpilation speedup is not guaranteed -- it is
a fundamental computational limitation, not an engineering one.

> "There are computations where no amount of analysis can predict the outcome
> faster than simply running the computation step by step."
> -- Wolfram, S. (2025), *On the Determination of Computational Complexity
> from the Minimal Sizes of Turing Machines*, Section 3

**Implications for depyler's convergence strategy**:

1. **Reducibility classification**. Before optimizing, classify transpilation
   targets by whether their core computation is *reducible* (interpreter
   overhead, dynamic dispatch -- transpilation helps) or *irreducible* (the
   algorithm itself is the bottleneck -- transpilation gives constant-factor
   improvement at best). This informs which optimization passes to run.

2. **Nondeterministic search over equivalent programs**. Wolfram shows that
   exploring multiple computation branches simultaneously can turn exponential
   runtimes into constant ones. For depyler, this argues for generating
   multiple candidate transpilations in parallel and selecting the best --
   rather than committing to a single strategy. The UTOL loop's multi-corpus
   approach is a structural analogue: each corpus tier explores a different
   region of the program space.

3. **Lower bound estimation via ground-truth corpora**. Wolfram proves lower
   bounds by exhaustive enumeration within a machine class. Depyler's
   five-tier corpus system serves the same function at practical scale: for
   known Python patterns, the ground-truth corpora establish "best known
   transpilation" baselines. The Tier 5 algorithm-competition corpus is
   particularly suited to this, as its pure-algorithmic nature isolates
   computation from I/O and library overhead.

4. **Runtime outlier awareness**. Wolfram shows runtimes in the wild are
   erratic, with dramatic outliers at specific inputs. This is relevant to
   transpilation validation: testing on typical inputs may miss pathological
   cases. Property-based testing (already used in the falsification
   framework) is the correct response.

**Limitation**: Wolfram enumerates all programs of size N. Depyler operates on
human-written Python, which is a tiny, structured subset of all possible
programs. The wildness Wolfram observes in arbitrary Turing machines is less
prevalent in real codebases. The theoretical pessimism about irreducibility
therefore overestimates the difficulty of practical transpilation. The
five-tier corpus system provides the empirical counterweight: measured compile
rates on real code test whether the theoretical ceiling is binding in practice.

---

## 4. Goal 2: Concurrent Oracle Training

### 4.1 Conjecture

> **C2**: Training the oracle on the combined error stream from all five
> corpora produces a model with higher accuracy and lower category confusion
> than sequential per-corpus training.

### 4.2 Rationale

Deming (1986) argued that process improvement requires learning from the
full range of variation, not a filtered subset: "It is not enough to do your
best; you must know what to do, and then do your best" (p. 19). The oracle
currently trains on a single corpus (reprorusted-python-cli, 368 files).
Adding two structurally distinct corpora introduces error categories that the
current model has never observed (e.g., unsupported ML library calls from
Tier 3, stdlib-only patterns from Tier 1).

The Toyota Way principle of *genchi genbutsu* (go and see) demands that
models be trained on real production data, not synthetic approximations
(Liker, 2004, Principle 12). Concurrent training satisfies this by exposing
the oracle to the actual distribution of errors that users encounter.

### 4.3 Falsification Criteria

| ID | Falsifier | Observable | Threshold | Popper Category |
|----|-----------|------------|-----------|-----------------|
| F2.1 | No accuracy gain | Combined-corpus oracle accuracy <= single-corpus accuracy | delta <= 0 pp | Crucial experiment (Popper, 1963, Ch. 10) |
| F2.2 | Category collapse | Any error category falls below 70% F1-score after combined training | F1 < 0.70 | Degree of falsifiability (Popper, 1959, Sec. 36) |
| F2.3 | Training divergence | Loss fails to converge within 100 epochs on combined data | loss_delta > 0 for 20 consecutive epochs | Basic statement |
| F2.4 | Latency regression | Inference latency exceeds 10ms per classification after combined training | p99 > 10ms | Severity of test |

### 4.4 Training Architecture

```
                    +-------------------------------+
                    |      Unified Error Stream     |
                    +-------------------------------+
                  |       |       |       |       |
           +------+ +-----+ +----+ +------+ +------+
           |Tier 1| |Tier2| |Tier| |Tier 4| |Tier 5|
           |stdlib| |typed| | 3  | | JAX  | |algo  |
           |errors| |errs | |ML  | |array | |comp  |
           +------+ +-----+ +----+ +------+ +------+
                  |       |       |       |       |
                    +-------------------------------+
                    | Heijunka Balancer             |
                    | (class-weighted sample)       |
                    +-------------------------------+
                              |
                    +-------------------------------+
                    | Oracle MOE Training           |
                    | (mixture of experts)          |
                    +-------------------------------+
                              |
     +----------+--------+--------+----------+-----------+
     |          |                 |           |           |
+--------+ +--------+     +---------+ +--------+ +----------+
|Stdlib  | |Type    |     |Library  | |Numeric | |Algorithm |
|Expert  | |Expert  |     |Expert   | |Expert  | |Expert    |
+--------+ +--------+     +---------+ +--------+ +----------+
```

### 4.5 Scholarly Grounding

The mixture-of-experts approach to error classification is supported by
Jacobs et al. (1991), who demonstrated that modular expert networks achieve
lower error rates than monolithic architectures when input distributions
are heterogeneous:

> "A system of expert networks, each specializing in a different region
> of input space, combined with a gating network that learns to assign
> inputs to experts, can outperform a single large network."
> -- Jacobs, Jordan, Nowlan, & Hinton (1991), *Adaptive Mixtures of Local
> Experts*, Neural Computation 3(1), p. 79

Curriculum learning (Bengio et al., 2009) provides theoretical support
for the tiered training order (stdlib -> typed -> ML -> JAX -> algorithmic). Starting with
simpler examples and progressively introducing complexity has been shown
to improve final model accuracy:

> "Training with a curriculum that introduces gradually more complex
> examples leads to faster convergence and better generalization."
> -- Bengio, Louradour, Collobert, & Weston (2009), *Curriculum Learning*,
> ICML '09, p. 41

---

## 5. Goal 3: PMAT Comply A+ Grade

### 5.1 Conjecture

> **C3**: The depyler codebase can achieve PMAT TDG grade A+ (score <= 1.0)
> across all crates while maintaining or increasing the single-shot compile
> rate, demonstrating that code quality and transpiler capability are not
> in tension.

### 5.2 Current State

| Metric | Current | A+ Target | Gap |
|--------|---------|-----------|-----|
| Average TDG | 0.70 | <= 1.0 | Met (avg) |
| Warning files (TDG > 2.0) | 7 | 0 | 7 files |
| Critical files (TDG > 2.5) | 0 | 0 | Met |
| Cyclomatic complexity violations | 57 functions | 0 | 57 functions |
| `unwrap()` calls | 712 | <= 100 | 612 calls |
| SATD markers | 0 | 0 | Met |
| 95th percentile TDG | 1.45 | <= 1.5 | Met |
| 99th percentile TDG | 1.79 | <= 2.0 | Met |

The primary blockers are the 57 complexity violations concentrated in
three files: `expr_gen.rs` (44), `stmt_gen.rs` (11), `func_gen.rs` (2).

### 5.3 Falsification Criteria

| ID | Falsifier | Observable | Threshold | Popper Category |
|----|-----------|------------|-----------|-----------------|
| F3.1 | Quality-capability tradeoff | Compile rate drops > 2 pp after any refactoring commit | delta < -2 pp | Crucial experiment |
| F3.2 | Complexity irreducibility | Any function cannot be decomposed below CC=10 without changing semantics | CC > 10, semantics-preserving decomposition impossible | Degree of falsifiability |
| F3.3 | Unwrap cascade | Replacing `unwrap()` with proper error handling introduces > 5 new test failures | new_failures > 5 per batch of 50 replacements | Basic statement |
| F3.4 | TDG measurement instability | TDG scores vary > 0.3 between consecutive measurements on identical code | variance > 0.3 | Reproducibility (Popper, 1959, Sec. 22) |

### 5.4 Refactoring Strategy

The refactoring follows Fowler's (2018) principle of behavior-preserving
transformations guided by test coverage:

> "Refactoring is a disciplined technique for restructuring an existing
> body of code, altering its internal structure without changing its
> external behavior."
> -- Fowler, M. (2018), *Refactoring: Improving the Design of Existing
> Code*, 2nd ed., p. xvi

Applied to the Toyota Way, this maps to *kaizen* (continuous improvement)
at the function level (Imai, 1986, Ch. 1). Each complexity-reducing
decomposition is a small improvement that compounds:

> "Kaizen means ongoing improvement involving everybody, without
> spending much money."
> -- Imai, M. (1986), *Kaizen: The Key to Japan's Competitive Success*,
> p. xxix

**Priority order** (highest debt first):

1. `expr_gen.rs` -- Extract method objects for each Python expression type.
   Target: 44 violations -> 0. Apply the Strategy pattern (Gamma et al.,
   1994) to replace match-arm bloat with dispatch tables.

2. `stmt_gen.rs` -- Split compound statement handlers into single-
   responsibility functions. Target: 11 violations -> 0.

3. `func_gen.rs` -- Extract parameter-handling logic. Target: 2 -> 0.

4. `unwrap()` elimination -- Replace with `?` operator and typed errors
   in batches of 50, with regression tests after each batch.

### 5.5 Scholarly Grounding

The relationship between code complexity and defect density is well-
established. McCabe (1976) demonstrated that cyclomatic complexity
correlates with testing difficulty and defect probability:

> "The cyclomatic complexity of a structured program is the number of
> linearly independent paths and therefore the number of tests required
> to exercise every path."
> -- McCabe, T. J. (1976), *A Complexity Measure*, IEEE TSE SE-2(4), p. 308

Nagappan et al. (2006) confirmed at Microsoft that code complexity metrics
predict post-release defects with statistical significance:

> "Complexity metrics computed from the code are statistically significant
> predictors of pre-release and post-release defect density."
> -- Nagappan, Ball, & Zeller (2006), *Mining Metrics to Predict Component
> Failures*, ICSE '06, p. 452

---

## 6. Goal 4: FAST Coverage at 95%

### 6.1 Conjecture

> **C4**: The depyler workspace can achieve 95% line coverage under the FAST
> test tier (cargo-nextest with PROPTEST_CASES=5, timeout <= 5 minutes) by
> adding targeted unit tests to uncovered branches without increasing total
> test execution time beyond 5 minutes.

### 6.2 Definition of FAST

FAST coverage is the line coverage achieved by the quick-feedback test tier
that runs on every commit. It excludes slow integration tests, fuzz tests,
and property tests with high iteration counts. The constraint is
**time-bounded**: the entire suite must complete in <= 5 minutes to maintain
developer flow state (Csikszentmihalyi, 1990).

Current configuration:
```toml
# .pmat-metrics.toml
test_fast_max_ms = 300_000  # 5 minutes hard limit
```

### 6.3 Current State

| Metric | Current | Target | Gap |
|--------|---------|--------|-----|
| FAST coverage | ~60% | 95% | ~35 pp |
| FAST suite time | ~2 min | <= 5 min | 3 min headroom |
| Covered lines | ~4,800 | ~7,600 | ~2,800 lines |
| Total testable lines | ~8,000 | ~8,000 | -- |
| Excluded modules | 18 patterns | Reduce to 8 | 10 modules to cover |

### 6.4 Falsification Criteria

| ID | Falsifier | Observable | Threshold | Popper Category |
|----|-----------|------------|-----------|-----------------|
| F4.1 | Time budget exceeded | Adding tests to reach 95% pushes FAST suite beyond 5 min | wall_time > 300s | Basic statement |
| F4.2 | Diminishing returns | Covering the last 5% (90% -> 95%) requires more tests than the first 30% (60% -> 90%) | tests_for_last_5pp > tests_for_first_30pp | Degree of corroboration |
| F4.3 | Coverage inflation | Coverage increases without corresponding defect-detection capability (mutation score flat) | mutation_score_delta == 0 while coverage_delta > 10 pp | Ad hoc hypothesis (Popper, 1963, Ch. 1) |
| F4.4 | Flaky test introduction | New tests produce non-deterministic results under identical inputs | flaky_count > 0 with seed=42 | Reproducibility |

### 6.5 Coverage Strategy

**Phase 1: Reduce exclusion list** (60% -> 75%)

The current exclusion regex in the Makefile hides 18 module patterns from
coverage measurement. Several are now testable:

| Module | Current Status | Action |
|--------|---------------|--------|
| `converge/compiler.rs` | Excluded (I/O) | Add mock-based unit tests |
| `utol_cmd.rs` | Excluded (CLI) | Add argument-parsing tests |
| `compile_cmd.rs` | Excluded (CLI) | Add subcommand tests |
| `report_cmd/mod.rs` | Excluded | Add formatting tests |
| `depyler/src/lib.rs` | Excluded (thin wrapper) | Add integration smoke tests |

**Phase 2: Branch coverage for codegen** (75% -> 90%)

The code generation modules (`expr_gen.rs`, `stmt_gen.rs`, `func_gen.rs`)
have many match arms that are exercised by examples but not by unit tests.
Adding per-arm unit tests closes this gap.

**Phase 3: Edge-case saturation** (90% -> 95%)

Target remaining uncovered branches with boundary-value tests derived from
the falsification suite. Prioritize branches that handle error paths, as
these are disproportionately likely to contain latent defects (Beizer, 1990).

### 6.6 Scholarly Grounding

The 95% threshold is informed by empirical studies of coverage effectiveness.
Inozemtseva and Holmes (2014) found that coverage above 90% correlates with
improved fault detection, but only when combined with mutation testing:

> "There is a low to moderate correlation between coverage and the
> effectiveness of a test suite at detecting faults... Test suites with
> both high coverage and high mutation scores are more effective."
> -- Inozemtseva & Holmes (2014), *Coverage Is Not Strongly Correlated
> with Test Suite Effectiveness*, ICSE '14, p. 435

The FAST constraint (5-minute ceiling) follows the Toyota Way principle of
*takt time* -- the pace at which units must be completed to meet demand
(Ohno, 1988, p. 32). In software, the "demand" is developer feedback
frequency: tests that exceed 5 minutes break flow and are skipped.

> "The key to the Toyota Production System is takt time. It determines
> the pace, and every process must match it."
> -- Ohno, T. (1988), *Toyota Production System: Beyond Large-Scale
> Production*, p. 32

Mockus et al. (2009) demonstrated at Avaya that fast test feedback loops
(< 5 minutes) reduce defect escape rates by 40% compared to nightly-only
testing:

> "Developers who receive test feedback within minutes are significantly
> more likely to fix defects before committing than those who wait for
> overnight builds."
> -- Mockus, Nagappan, & Dinh-Trong (2009), *Test Coverage and Post-
> Verification Defects*, ESEM '09, p. 296

---

## 7. Falsification Framework

### 7.1 Epistemological Foundation

This specification adopts Popper's critical rationalism as its
epistemological framework. The key commitments are:

1. **Asymmetry of verification and falsification** (Popper, 1959, Sec. 6):
   No finite number of successful compilations can verify the conjecture
   "depyler correctly transpiles all Python." But a single compilation
   failure *can* falsify a specific conjecture about a specific pattern.

2. **Degrees of falsifiability** (Popper, 1959, Sec. 36): Conjectures
   that prohibit more observations are more falsifiable and therefore
   more scientifically valuable. "Compile rate >= 80% on stdlib corpus"
   is more falsifiable than "compile rate improves over time" because it
   specifies a threshold and a corpus.

3. **Corroboration, not confirmation** (Popper, 1963, Ch. 10): When a
   conjecture survives a severe test, it is *corroborated* but not
   confirmed. The compile-rate targets remain conjectures even when met;
   they become more trustworthy only as they survive increasingly severe
   tests (new corpora, adversarial inputs).

4. **Elimination of ad hoc hypotheses** (Popper, 1963, Ch. 1): If a
   conjecture is falsified, the revision must be independently testable,
   not merely a patch that accommodates the falsifying instance. Example:
   if Tier 1 compile rate stalls at 75%, the fix cannot be "exclude the
   failing files"; it must be a transpiler improvement testable against
   held-out examples.

### 7.2 Falsification Protocol

Every UTOL iteration produces a **falsification report**:

```json
{
  "iteration": 12,
  "conjectures": [
    {
      "id": "C1",
      "status": "corroborated",
      "tier1_rate": 0.82,
      "tier2_rate": 0.55,
      "tier3_rate": 0.31,
      "falsifiers_tested": ["F1.1", "F1.2", "F1.3", "F1.4"],
      "falsifiers_triggered": []
    },
    {
      "id": "C2",
      "status": "partially_falsified",
      "oracle_accuracy": 0.88,
      "falsifiers_triggered": ["F2.2"],
      "revision": "Retrain with class-balanced sampling for E0282"
    }
  ],
  "escape_rate": 0.03,
  "popper_health": "GREEN"
}
```

### 7.3 Escape Rate as Falsification Metric

The escape rate (DEPYLER-1321) measures the fraction of compile failures
where the oracle suggested `DepylerValue` as a fallback type instead of
a concrete type. Following Popper's demarcation criterion, if the escape
rate exceeds 20%, the type-inference system is "immunizing" itself against
falsification by retreating to an unfalsifiable catch-all type:

> A theory that is not refutable by any conceivable event is non-
> scientific. Irrefutability is not a virtue but a vice.
> -- Popper, K. R. (1963), *Conjectures and Refutations*, p. 36

**Threshold**: escape_rate > 0.20 triggers a STOP-THE-LINE event.

### 7.4 Consolidated Falsification Matrix

| Goal | Conjecture | Falsifier Count | Severity if Falsified |
|------|-----------|-----------------|----------------------|
| G1 (Compile) | C1 | 6 | P0 -- blocks release |
| G2 (Oracle) | C2 | 4 | P1 -- blocks training |
| G3 (PMAT A+) | C3 | 4 | P1 -- blocks refactoring |
| G4 (Coverage) | C4 | 4 | P2 -- tracks regression |

Total falsifiers: **18 independent tests** of the specification's claims.

### 7.5 Computational Reducibility as Falsification Constraint

Wolfram (2025) provides a theoretical lens for interpreting falsification
outcomes. The distinction between *computationally reducible* and
*computationally irreducible* processes (Wolfram, 2002, Ch. 12) partitions
transpilation targets into two classes:

1. **Reducible targets**: Python code where the runtime cost is dominated by
   interpreter overhead, dynamic dispatch, or GIL contention. Transpilation
   to Rust eliminates these costs, yielding super-linear speedup. These
   patterns are the "easy wins" -- binary search, bit manipulation, simple
   numeric algorithms. The Tier 5 per-category data confirms this: categories
   dominated by simple control flow (binary_search: 80%, bit_manipulation:
   67%) compile at high rates.

2. **Irreducible targets**: Python code where the computation itself is the
   bottleneck -- recursive tree traversal, graph search with backtracking,
   complex state machines. Transpilation yields at most constant-factor
   improvement, and the transpiler must correctly encode complex type
   relationships (`Option<Box<TreeNode>>`, adjacency lists). The Tier 5 data
   confirms this: categories with recursive data structures (tree: 0%,
   graph: 0%, linked_list: 0%) have zero compile rate.

This partition informs the falsification framework: falsifier F1.6
(recursive-type ceiling) tests whether irreducible patterns form a hard
boundary. If the 0% categories remain at 0% after targeted iteration, the
conjecture that "40% is achievable for Tier 5" is falsified for the
irreducible sub-corpus, requiring either a revised target or a fundamental
change in how the transpiler handles recursive types.

Wolfram's runtime outlier finding -- that runtimes across the space of Turing
machines are erratic with dramatic outliers at specific inputs -- reinforces
the property-based testing approach in falsifier F4.4. Typical-input testing
may miss pathological cases where transpiled code diverges semantically.

---

## 8. Architecture

### 8.1 Multi-Corpus Convergence Pipeline

```
 +-----------+  +-----------+  +----------+  +---------+  +----------+
 | Tier 1    |  | Tier 2    |  | Tier 3   |  | Tier 4  |  | Tier 5   |
 | std-only  |  | typed-cli |  | hf-corp  |  | jax-gtc |  | algo-comp|
 | 20 files  |  | 15 files  |  | 128 files|  | 7 files |  | 80 files |
 +-----------+  +-----------+  +----------+  +---------+  +----------+
       |              |              |             |             |
       v              v              v             v             v
 +------------------------------------------------------------------+
 |               Unified Transpilation Phase                        |
 |    depyler transpile --corpus $TIER --seed 42                    |
 +------------------------------------------------------------------+
       |              |              |             |             |
       v              v              v             v             v
 +------------------------------------------------------------------+
 |               Batch Compilation Phase                            |
 |    BatchCompiler (tokio, semaphore=16 jobs)                      |
 +------------------------------------------------------------------+
       |              |              |             |             |
       v              v              v             v             v
 +------------------------------------------------------------------+
 |               Error Stream Merge                                 |
 |    Heijunka-balanced sampling across tiers                       |
 +------------------------------------------------------------------+
                           |
                           v
 +--------------------------------------------------+
 |          Oracle Training (Concurrent)             |
 |  MOE: stdlib + type + library + numeric + algo    |
 |  Curriculum: Tier 1 -> 2 -> 3 -> 4 -> 5          |
 +--------------------------------------------------+
                      |
           +----------+----------+
           |                     |
           v                     v
 +-------------------+  +-------------------+
 | Error Clusterer   |  | Fix Applicator    |
 | (root cause       |  | (transpiler       |
 |  grouping)        |  |  patching)        |
 +-------------------+  +-------------------+
           |                     |
           v                     v
 +---------------------------------------------+
 |          Falsification Checkpoint            |
 |  - Per-tier compile rates                    |
 |  - Cross-tier regression gate                |
 |  - Escape rate check                         |
 |  - Oracle accuracy by category               |
 +---------------------------------------------+
                      |
              +-------+-------+
              |               |
         [PASS]          [FAIL]
              |               |
              v               v
     Next iteration    Stop-the-line
                       Root cause analysis
```

### 8.2 Concurrent Training Data Flow

The oracle receives error records tagged with their corpus tier. The
heijunka balancer ensures no single tier dominates the training batch,
following the Toyota principle of leveled production:

```
Training batch composition (per epoch):
  - Tier 1 errors: 20% (class-balanced within tier)
  - Tier 2 errors: 20%
  - Tier 3 errors: 20%
  - Tier 4 errors: 20%
  - Tier 5 errors: 20%

Within each tier, class balance (heijunka):
  - E0308 (type mismatch): capped at 25% of tier allocation
  - E0277 (trait bound):   capped at 25%
  - E0599 (method):        capped at 25%
  - Other:                 remaining 25%
```

### 8.3 PMAT Quality Pipeline Integration

```
  Code change
       |
       v
  [Pre-commit hook]
       |
       +-- pmat tdg check-quality --min-grade A+
       +-- pmat analyze complexity --max-cyclomatic 10
       +-- pmat analyze satd --fail-on-violation
       +-- cargo clippy -- -D warnings
       |
       v
  [CI Pipeline]
       |
       +-- cargo llvm-cov --fail-under-lines 95
       +-- cargo mutants --workspace (target: 80% kill)
       +-- depyler converge --regression-baseline
       |
       v
  [Falsification Gate]
       |
       +-- All 18 falsifiers evaluated
       +-- Escape rate < 0.20
       +-- No cross-tier regression
       |
       v
  [Merge allowed]
```

---

## 9. Implementation Phases

### Phase 1: Baseline Measurement

Establish falsifiable baselines for all five corpora before any changes.

**Deliverables**:
- `baseline_tier1.json` -- reprorusted-std-only compile rate
- `baseline_tier2.json` -- fully-typed-reprorusted compile rate
- `baseline_tier3.json` -- hugging-face-gtc compile rate
- `baseline_tier4.json` -- jax-gtc compile rate
- `baseline_tier5.json` -- algorithm-competition-corpus compile rate
- `baseline_oracle.json` -- Oracle accuracy by category per tier
- `baseline_coverage.json` -- FAST coverage report

**Falsification test**: If any baseline measurement is non-deterministic
(varies between runs with seed=42), halt and fix the transpiler's
determinism invariant before proceeding.

### Phase 2: Tier 1 Convergence + Coverage Foundation

Focus on the easiest corpus first (curriculum learning). Simultaneously
begin reducing the coverage exclusion list.

**Deliverables**:
- Tier 1 compile rate >= 80%
- FAST coverage >= 75%
- 7 warning-level TDG files reduced to 4

**Falsification test**: F1.1 (plateau), F4.1 (time budget).

### Phase 3: Tier 2 Convergence + Oracle Training

Add Tier 2 errors to the oracle training stream. Begin complexity
reduction in `expr_gen.rs`.

**Deliverables**:
- Tier 2 compile rate >= 60%
- Oracle accuracy >= 88%
- `expr_gen.rs` violations reduced from 44 to 22

**Falsification test**: F2.1 (no accuracy gain), F1.2 (Tier 1 regression).

### Phase 4: Tier 3 Error Harvesting + PMAT A+

Harvest error signal from Tier 3 without expecting high compile rates.
Complete all complexity refactoring.

**Deliverables**:
- Tier 3 compile rate >= 40%
- All TDG warning files at grade A or better
- All complexity violations resolved
- `unwrap()` count <= 100

**Falsification test**: F1.3 (zero signal), F3.1 (quality-capability
tradeoff), F3.2 (complexity irreducibility).

### Phase 5: Coverage Saturation + Final Falsification

Push FAST coverage to 95% and run the complete falsification matrix.

**Deliverables**:
- FAST coverage >= 95%
- All 18 falsifiers evaluated
- Falsification report published
- Oracle accuracy >= 92%

**Falsification test**: All 18 falsifiers (F1.1-F4.4).

---

## 10. Risk Register

| Risk | Probability | Impact | Mitigation | Falsifier |
|------|-------------|--------|------------|-----------|
| Tier 3 corpus too complex for current transpiler | High | Low (expected) | Use for error signal only, not compile-rate target | F1.3 |
| Tier 4 JAX patterns orthogonal to existing fixes | Medium | Low | JAX array/autodiff errors train numeric expert | F1.5 |
| Tier 5 recursive data structures (tree/graph/linked_list) block compilation | High | Medium | Recursive types need dedicated `Option<Box<T>>` codegen; zero-dep corpus isolates core transpiler limitations | F1.6 |
| Coverage tests slow down FAST suite | Medium | High | Profile each new test, enforce 100ms ceiling per test | F4.1 |
| Complexity refactoring breaks codegen | Medium | High | Full regression suite after each decomposition | F3.1 |
| Oracle overfits to dominant error class (E0308) | Medium | Medium | Heijunka balancing, per-class F1 monitoring | F2.2 |
| Cross-corpus interference during convergence | Low | High | Independent baseline regression gates per tier | F1.4 |
| TDG measurement tool variability | Low | Medium | Pin pmat version, run 3x and take median | F3.4 |

---

## 11. References

### Compiler Testing and Automated Repair

- Yang, X., Chen, Y., Eide, E., & Regehr, J. (2011). Finding and
  Understanding Bugs in C Compilers. *Proceedings of the 32nd ACM
  SIGPLAN Conference on Programming Language Design and Implementation
  (PLDI '11)*, pp. 283-294. ACM. https://doi.org/10.1145/1993498.1993532

- Le Goues, C., Nguyen, T., Forrest, S., & Weimer, W. (2012). GenProg:
  A Generic Method for Automatic Software Repair. *IEEE Transactions on
  Software Engineering*, 38(1), pp. 54-72.
  https://doi.org/10.1109/TSE.2011.104

- Gulwani, S., Polozov, O., & Singh, R. (2017). Program Synthesis.
  *Foundations and Trends in Programming Languages*, 4(1-2), pp. 1-119.
  https://doi.org/10.1561/2500000010

### Machine Learning and Expert Systems

- Jacobs, R. A., Jordan, M. I., Nowlan, S. J., & Hinton, G. E. (1991).
  Adaptive Mixtures of Local Experts. *Neural Computation*, 3(1),
  pp. 79-87. https://doi.org/10.1162/neco.1991.3.1.79

- Bengio, Y., Louradour, J., Collobert, R., & Weston, J. (2009).
  Curriculum Learning. *Proceedings of the 26th International Conference
  on Machine Learning (ICML '09)*, pp. 41-48. ACM.
  https://doi.org/10.1145/1553374.1553380

### Software Quality and Complexity

- McCabe, T. J. (1976). A Complexity Measure. *IEEE Transactions on
  Software Engineering*, SE-2(4), pp. 308-320.
  https://doi.org/10.1109/TSE.1976.233837

- Nagappan, N., Ball, T., & Zeller, A. (2006). Mining Metrics to Predict
  Component Failures. *Proceedings of the 28th International Conference
  on Software Engineering (ICSE '06)*, pp. 452-461. ACM.
  https://doi.org/10.1145/1134285.1134349

- Fowler, M. (2018). *Refactoring: Improving the Design of Existing
  Code* (2nd ed.). Addison-Wesley.

- Gamma, E., Helm, R., Johnson, R., & Vlissides, J. (1994). *Design
  Patterns: Elements of Reusable Object-Oriented Software*.
  Addison-Wesley.

### Testing Effectiveness

- Inozemtseva, L. & Holmes, R. (2014). Coverage Is Not Strongly
  Correlated with Test Suite Effectiveness. *Proceedings of the 36th
  International Conference on Software Engineering (ICSE '14)*,
  pp. 435-445. ACM. https://doi.org/10.1145/2568225.2568271

- Mockus, A., Nagappan, N., & Dinh-Trong, T. T. (2009). Test Coverage
  and Post-Verification Defects: A Multiple Case Study. *3rd
  International Symposium on Empirical Software Engineering and
  Measurement (ESEM '09)*, pp. 291-301. IEEE.
  https://doi.org/10.1109/ESEM.2009.5315981

- Beizer, B. (1990). *Software Testing Techniques* (2nd ed.).
  Van Nostrand Reinhold.

### Toyota Production System and Quality Management

- Ohno, T. (1988). *Toyota Production System: Beyond Large-Scale
  Production*. Productivity Press.

- Liker, J. K. (2004). *The Toyota Way: 14 Management Principles from
  the World's Greatest Manufacturer*. McGraw-Hill.

- Imai, M. (1986). *Kaizen: The Key to Japan's Competitive Success*.
  McGraw-Hill.

- Deming, W. E. (1986). *Out of the Crisis*. MIT Press.

### Philosophy of Science

- Popper, K. R. (1959). *The Logic of Scientific Discovery*.
  Hutchinson. [Originally published as *Logik der Forschung*, 1934.]

- Popper, K. R. (1963). *Conjectures and Refutations: The Growth of
  Scientific Knowledge*. Routledge & Kegan Paul.

### Computational Complexity and Reducibility

- Wolfram, S. (2025). On the Determination of Computational Complexity
  from the Minimal Sizes of Turing Machines. *Wolfram Physics Project
  Technical Report*. https://writings.stephenwolfram.com/2025/01/on-the-determination-of-computational-complexity-from-the-minimal-sizes-of-turing-machines/

- Wolfram, S. (2002). *A New Kind of Science*. Wolfram Media. Chapter 12:
  The Principle of Computational Equivalence.

### Flow and Productivity

- Csikszentmihalyi, M. (1990). *Flow: The Psychology of Optimal
  Experience*. Harper & Row.

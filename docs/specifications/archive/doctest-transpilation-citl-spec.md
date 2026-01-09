# Doctest Transpilation for CITL Training (Toyota Way)

**Specification Version:** 1.1.0 (Toyota Way Revision)
**Status:** Draft
**Authors:** Depyler Team
**Created:** 2025-11-29
**References:** GH-172, metaheuristic-oracle-spec.md

## 1. Executive Summary

### 1.1 Problem Statement: The Waste of Synthetic Validation
Currently, the CITL (Compiler-in-the-Loop) training pipeline relies heavily on synthetic data generation or successful compilation as a proxy for correctness. This creates **Muda (Waste)**:
1.  **False Positives**: Code compiles but does the wrong thing (Semantic Drift).
2.  **Over-processing**: Generating thousands of synthetic examples that don't reflect real-world usage.
3.  **Defect Escapes**: Logic bugs aren't caught until runtime or manual review.

### 1.2 Proposed Solution: Doctest Transpilation
Leverage the "Plastic Surgery Hypothesis" [1] by treating existing Python doctests as a high-fidelity, zero-cost validation corpus. By transpiling `>>>` examples into `/// ```rust` doc tests, we achieve **Jidoka (Automation with human intelligence)**—using human-verified examples to drive automated training.

### 1.3 Success Metrics
-   **Zero Cost Creation**: Utilize 10,000+ existing doctests in stdlib/pandas/numpy.
-   **Semantic Proof**: 100% of passing doc tests guarantee semantic equivalence.
-   **Type Oracle**: Infer accurate types from I/O pairs (e.g., `len("s") -> 1` implies `String -> usize`).

## 2. Toyota Way Analysis (Genchi Genbutsu)

### 2.1 Current State (The Waste)
| Metric | Synthetic Data | Compile-Only Validation | Doctest Transpilation |
| :--- | :--- | :--- | :--- |
| **Signal Quality** | Low (Artificial) | Medium (Syntax/Types) | **High (Semantic + Behavioral)** |
| **Creation Cost** | High (Compute/Logic) | Low (Free) | **Zero (Mined)** |
| **False Positives** | High | High (Logic bugs pass) | **Near Zero** |
| **Toyota Principle** | **Muda** (Over-production) | **Muri** (Strain on downstream QA) | **Just-in-Time** (Right signal at right time) |

> **Annotation [1]:** Barr et al. (2014) proposed the "Plastic Surgery Hypothesis," stating that fixes already exist in the codebase. We extend this: *validation* already exists in the documentation. This minimizes the search space for the oracle by anchoring it to known-good states.

### 2.2 Five Whys: Root Cause of Semantic Drift
1.  **Why do transpiled functions fail at runtime?**
    *Because the logic doesn't match the Python behavior.*
2.  **Why doesn't the compiler catch this?**
    *Because `rustc` only checks type safety, not semantic correctness.*
3.  **Why don't we have semantic tests?**
    *Because writing unit tests for every transpiled function is too expensive (Muri).*
4.  **Why is it expensive?**
    *Because we are manually duplicating effort.*
5.  **Root Cause**: We are ignoring the **Single Source of Truth (Ichigen Kanri)** present in Python docstrings.

## 3. Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                   Doctest CITL Pipeline                          │
└─────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Python     │    │   Doctest    │    │   Depyler    │
│   Source     │ →  │  Extractor   │ →  │  Transpile   │
│  + Docstring │    │  (>>> lines) │    │  (fn + docs) │
└──────────────┘    └──────────────┘    └──────────────┘
                           │                   │
                           ▼                   ▼
                    ┌──────────────┐    ┌──────────────┐
                    │   I/O Pairs  │    │  Rust Code   │
                    │  (input,out) │    │  + Doc Tests │
                    └──────────────┘    └──────────────┘
                           │                   │
                           └─────────┬─────────┘
                                     ▼
                           ┌──────────────────┐
                           │ cargo test --doc │
                           └────────┬─────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
              ┌─────────┐    ┌─────────┐    ┌─────────┐
              │  PASS   │    │ COMPILE │    │ RUNTIME │
              │(Correct)│    │  FAIL   │    │  FAIL   │
              └─────────┘    └─────────┘    └─────────┘
                    │               │               │
                    ▼               ▼               ▼
              ┌─────────┐    ┌─────────┐    ┌─────────┐
              │ +1 for  │    │ Type or │    │ Semantic│
              │ Oracle  │    │ syntax  │    │ bug in  │
              │ success │    │ error   │    │transpile│
              └─────────┘    └─────────┘    └─────────┘
```

> **Annotation [2]:** This pipeline implements a "Test-Driven Repair" loop, similar to techniques used in GenProg (Le Goues et al., 2012). Instead of evolving patches blindly, we use the test cases (doctests) as the fitness function, ensuring monotonic improvement.

## 4. Implementation Details

### 4.1 Doctest Extraction Strategy
We must parse Python docstrings to extract `>>>` blocks.

```rust
// Example of target transformation
// Python:
// >>> add(1, 2)
// 3

// Rust:
/// ```rust
/// assert_eq!(add(1, 2), 3);
/// ```
```

> **Annotation [3]:** By transforming the specification language (Python doctest) into the verification language (Rust doc test), we perform a "homomorphic translation" of the test vector. This preserves the semantic intent while adapting to the target syntax.

### 4.2 Handling Non-Determinism (Heijunka)
Some doctests output dictionaries (unordered) or memory addresses.
**Solution:**
-   **Normalization:** Sort dictionary keys before comparison.
-   **Masking:** Replace memory addresses (0x...) with placeholders.

> **Annotation [4]:** This aligns with "Heijunka" (Leveling). By normalizing outputs, we smooth out the "unevenness" (Mura) of non-deterministic execution, preventing flaky tests from poisoning the CITL signal.

### 4.3 Type Inference via IO Examples
Doctests provide concrete examples of input types.
`fib(10) -> 55`
This tells the TypeEnvironment:
-   Input is integer-like.
-   Output is integer-like.
-   Constraint: `typeof(fib(10)) == typeof(55)`.

> **Annotation [5]:** This is a form of "Programming by Example" (PBE) or inductive synthesis (Gulwani, 2011). We use the I/O pairs to refine the search space for type inference, effectively pruning invalid type assignments early.

## 5. Corpus Acquisition (Genchi Genbutsu)

### 5.1 Critical Distinction: Mine, Don't Write

| Repo | Role | Doctests? |
|------|------|-----------|
| `reprorusted-python-cli` | Transpilation error corpus | **No** (we wrote it) |
| `python/cpython` | Doctest mining source | **Yes** (they wrote it) |
| `numpy/numpy` | Doctest mining source | **Yes** (they wrote it) |
| `pandas-dev/pandas` | Doctest mining source | **Yes** (they wrote it) |

> **Anti-pattern (Muda):** Adding doctests to our own examples = writing tests = waste.
> **Pattern (Just-in-Time):** Mining doctests from stdlib = free validation corpus.

### 5.2 Acquisition Script

```bash
#!/bin/bash
# scripts/acquire-doctest-corpus.sh

CORPUS_DIR="${1:-/tmp/doctest-corpus}"
mkdir -p "$CORPUS_DIR"

# Clone sources (shallow for size, ~500MB total)
echo "Cloning CPython stdlib..."
git clone --depth 1 https://github.com/python/cpython.git "$CORPUS_DIR/cpython"

echo "Cloning NumPy..."
git clone --depth 1 https://github.com/numpy/numpy.git "$CORPUS_DIR/numpy"

echo "Cloning Pandas..."
git clone --depth 1 https://github.com/pandas-dev/pandas.git "$CORPUS_DIR/pandas"

# Extract doctests to JSON
echo "Extracting doctests..."
depyler extract-doctests "$CORPUS_DIR/cpython/Lib" --output "$CORPUS_DIR/stdlib-doctests.json"
depyler extract-doctests "$CORPUS_DIR/numpy/numpy" --output "$CORPUS_DIR/numpy-doctests.json"
depyler extract-doctests "$CORPUS_DIR/pandas/pandas" --output "$CORPUS_DIR/pandas-doctests.json"

# Merge into single corpus
depyler merge-doctests \
    "$CORPUS_DIR/stdlib-doctests.json" \
    "$CORPUS_DIR/numpy-doctests.json" \
    "$CORPUS_DIR/pandas-doctests.json" \
    --output "$CORPUS_DIR/unified-doctests.json"

echo "Done. Corpus at: $CORPUS_DIR/unified-doctests.json"
```

### 5.3 Estimated Corpus Size

| Source | Files | Est. Functions | Est. Doctests |
|--------|-------|----------------|---------------|
| CPython `Lib/` | ~300 | ~2,000 | 5,000+ |
| NumPy `numpy/` | ~200 | ~1,500 | 3,000+ |
| Pandas `pandas/` | ~400 | ~2,500 | 5,000+ |
| **Total** | **~900** | **~6,000** | **~13,000+** |

### 5.4 Licensing

All sources are permissively licensed:
- CPython: PSF License (BSD-like)
- NumPy: BSD 3-Clause
- Pandas: BSD 3-Clause

Doctests are extracted as (input, output) pairs for training, not redistributed.

### 5.5 Storage Format

```json
{
  "source": "cpython",
  "module": "os.path",
  "function": "join",
  "doctests": [
    {
      "input": "os.path.join('/home', 'user', 'file.txt')",
      "expected": "'/home/user/file.txt'",
      "line": 142
    }
  ],
  "signature": "(path, *paths) -> str",
  "docstring": "Join one or more path components..."
}
```

> **Annotation [11]:** By storing doctests in a structured format, we enable **incremental training**. As CPython evolves, we can diff the corpus and retrain only on delta, reducing compute waste (Muda).

## 6. Roadmap (Kaizen)

### Phase 0: Corpus Acquisition (Week 0)
-   **Goal:** Clone CPython, NumPy, Pandas; extract doctests.
-   **Deliverable:** `unified-doctests.json` with ~13,000 examples.
-   **Annotation [12]:** Start with "raw materials" (Genchi Genbutsu). Before building the pipeline, verify the corpus exists and has expected density.

### Phase 1: Extraction & Formatting (Week 1)
-   **Goal:** Parse `>>>` lines from AST.
-   **Deliverable:** `DoctestExtractor` struct.
-   **Annotation [6]:** Focus on "small batch sizes" first. Only simple arithmetic functions. Avoid the complexity of full objects initially to establish the pipeline (MVP).

### Phase 2: Transpilation & Execution (Week 2)
-   **Goal:** Generate `/// ```rust` blocks and run `cargo test`.
-   **Deliverable:** `cargo test --doc` passing for simple stdlib functions.
-   **Annotation [7]:** "Fail Fast". We want compilation errors immediately if the generated test is invalid, providing immediate feedback to the Oracle.

### Phase 3: Feedback Loop Integration (Week 3)
-   **Goal:** Feed pass/fail signals back to `entrenar`.
-   **Deliverable:** CITL pipeline accepts doctest results as reward signal.
-   **Annotation [8]:** Reinforcement Learning from Code Execution (RLCE). The compiler feedback acts as the environment response, guiding the agent (Oracle) toward valid transpilation policies.

## 6. Scientific & Academic Justifications

> **Annotation [9]:** **Oracles in Software Testing:** The doctest acts as a "Partial Oracle" (Staats et al., 2011). It doesn't verify *every* behavior, but it verifies *canonical* behaviors specified by the author, which often cover the "happy path" and critical edge cases.

> **Annotation [10]:** **N-Version Programming:** While not strictly N-version (since we are translating), the comparison between Python runtime output and Rust runtime output acts as a differential testing harness (McKeeman, 1998), effectively finding discrepancies in the language semantics themselves (e.g., integer overflow behavior).

## 7. References
1.  Barr, E. T., et al. (2014). The Plastic Surgery Hypothesis. *FSE '14*.
2.  Le Goues, C., et al. (2012). GenProg: A Generic Method for Automatic Software Repair. *IEEE TSE*.
3.  Gulwani, S. (2011). Automating string processing in spreadsheets using input-output examples. *POPL*.
4.  Staats, M., et al. (2011). Programs, tests, and oracles: the foundations of testing revealed. *ICSE*.
5.  McKeeman, W. M. (1998). Differential testing for software. *Digital Technical Journal*.
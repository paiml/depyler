# Post-Mortem: Single-Shot Compile Failure & Architecture Indictment

**Document Version**: 2.4.0 (MAJOR PROGRESS)
**Date**: 2026-01-09
**Status**: **REFACTORING IN PROGRESS**
**Author**: Claude Code Analysis
**Severity**: **CRITICAL**

---

## 1. Executive Summary: The Illusion of Competence

The current compile rate has improved to **39.4% (69/175 files)**. We have resolved two of the three major architectural flaws.

**PROGRESS UPDATE (2026-01-09)**:
- **RC-1 (String Matching Loop Destruction)**: **RESOLVED**. Replaced regex-based logic with `syn` AST parsing.
- **RC-2 (Type Naivety)**: **RESOLVED**. Implemented `DepylerValue` sum type injection for heterogeneous dictionaries.
- **RC-3 (Semantic Laziness)**: **ACTIVE**. Compiler fails on string indexing and return type wrapping.

---

## 2. Methodology: Scientific Invalidation

This analysis rejects the "incremental fix" mindset. We follow a strict falsification protocol:
1.  **Empirical Baseline**: Analyze the current 28% compile rate using `rustc` diagnostics.
2.  **Hypothesis Generation**: Identify the "Big Three" architectural failure points.
3.  **Stress Testing**: Create `examples/falsification_suite.py` to target these weaknesses.
4.  **Verification**: Execute `./scripts/prove_failure.sh` to confirm the code is uncompilable.

---

## 3. Error Distribution: The Data of Decay

| Error Code | Count | Percentage | Category | Status |
|------------|-------|------------|----------|--------|
| E0308 | 228 | 38.4% | Type mismatch (RC-2/RC-4) | **PARTIALLY FIXED** |
| E0425 | 125 | 21.1% | Undefined variable (RC-1) | **FIXED** |
| E0277 | 81 | 13.7% | Trait bounds (RC-3) | **ACTIVE** |
| E0599 | 70 | 11.8% | Method not found (RC-4) | **ACTIVE** |
| E0282 | 52 | 8.8% | Type inference (RC-5) | **ACTIVE** |

---

## 4. RC-1: The String-Matching Anti-Pattern (The "Dragon Book" Violation)

**Severity**: FATAL
**Location**: `crates/depyler-core/src/rust_gen/stmt_gen_complex.rs:636`
**Status**: **RESOLVED**

#### The Evidence
The code explicitly attempts to parse Rust logic by converting tokens back to strings and searching for substrings:
```rust
// OLD CODE (DELETED)
let first_stmt = try_stmts[0].to_string();
if first_stmt.contains("parse") && first_stmt.contains("unwrap_or_default") { ... }
```

#### The Fix
Replaced with `syn::parse2` to inspect the AST. Logic now strictly guards against control flow statements, only matching `Let` bindings and `Expr` assignments.
**Verified**: `prove_failure.sh` confirms E0425 is gone.

---

## 5. RC-2: Naive Type Mapping (The "Static/Dynamic" Fallacy)

**Severity**: CRITICAL
**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Status**: **RESOLVED**

#### The Evidence
The transpiler assumes a 1:1 mapping between Python syntax and Rust types without structural unification.
*   Python: `d = {"a": 1, "b": "str"}` (Valid)
*   Rust Gen: `HashMap::from([("a", 1), ("b", "str")])` (Invalid)

#### The Fix
Implemented **Type Unification**.
1.  Modified `TypeMapper` to default `Any/Unknown` types to `DepylerValue` in NASA mode.
2.  Modified `expr_gen` to detect mixed types and generate `HashMap<String, DepylerValue>`.
3.  Injects a `DepylerValue` enum definition into the generated code automatically.
**Verified**: `prove_failure.sh` confirms dict type mismatch is gone.

---

## 6. RC-3: Semantic Laziness (The "Index" Fallacy)

**Severity**: HIGH
**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs`
**Status**: **ACTIVE**

#### The Evidence
Input: `s[i]` -> Output: `s[i as usize]`
Error: `String` cannot be indexed by `usize`.

#### The Indictment
This is **Translation by Google Translate**. It translates words (syntax) but ignores grammar (semantics).
1.  **Memory Model Ignorance**: It ignores that Rust Strings are UTF-8.
2.  **O(1) vs O(N)**: It attempts to force O(1) syntax onto an O(N) operation (`chars().nth()`).

**Corrective Action**: Semantic Lowering to distinct IR nodes.

---

## 7. Falsification Checklist: The Proof of Failure

We do not test if it "works". We test if it is **robust**. 

### RC-1 Attack Vectors (String Matching Hall of Shame) - [PASSED]
*   **F1**: Variable name contains keyword (`parse_parser`).
*   **F2**: Comment contains keyword (`# parse this`).
*   **F3**: Whitespace variation (`x . parse ()`).
*   **F4**: Nested statement (`if True: x.parse()`).
*   **F5**: For-loop with inner parse (The primary loop-breaker).

### RC-2 Attack Vectors (Type Naivety) - [PASSED]
*   **T1**: Mixed-type dicts (`{"a": 1, "b": "2"}`).
*   **T2**: Return type checking for `Dict[str, Any]`.

### RC-3 Attack Vectors (Semantic Laziness) - [FAILING]
*   **S1**: Direct string indexing (`s[0]`).
*   **S2**: Dict membership (`x in d`) without type info.

---

## 8. Architectural Ultimatum: The 3 Missing Pillars

The following architectural components are **missing** and must be implemented:

1.  **The Symbol Table**: Proper scope tracking and type propagation.
2.  **The CFG (Control Flow Graph)**: To understand statement dominance.
3.  **The Semantic Barrier**: Separation between Parsing (Python -> IR) and Generation (IR -> Rust).

---

## 9. Verdict & Recommendation

**Verdict**: The project is recovering. RC-1 and RC-2 are fixed.
**Recommendation**: Proceed to fix RC-3 (String Indexing).

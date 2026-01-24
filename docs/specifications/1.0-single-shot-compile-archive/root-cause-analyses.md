# Root Cause Analyses Archive (Five-Whys)

**Archived from**: `1.0-single-shot-compile.md` Section 3
**Archive Date**: 2026-01-25
**Coverage**: DEPYLER-1073 through DEPYLER-1148

This document preserves the detailed Five-Whys root cause analyses for completed investigations. These serve as a reference for understanding past decisions and avoiding repeated mistakes.

---

## 3.18 Five-Whys: Float Key Collection Failure (DEPYLER-1073)
*Status: FIXED* (Jan 10, 2026)

*   **Why 1**: Why did `d = {0.1: "val"}` fail in Rust?
    *   *Answer*: `HashMap<f64, String>` was generated, but `f64` does not implement `Eq` or `Hash` due to NaN values.
*   **Why 2**: Why not use a wrapper like `OrderedFloat`?
    *   *Answer*: To maintain NASA-grade "std-only" requirements without external dependencies.
*   **Why 3**: What already implements `Eq` and `Hash` safely for floats?
    *   *Answer*: Our `DepylerValue` enum, which uses `total_cmp` for its `Float` variant.
*   **Why 4**: How do we integrate this?
    *   *Answer*: By updating the `TypeMapper` and `codegen` to detect `Dict[float, V]` and map it to `HashMap<DepylerValue, V>`.
*   **Why 5**: What is the result?
    *   *Answer*: Seamless support for float keys in dictionaries and sets.

**Corrective Action**: Modified `TypeMapper` and `stmt_gen` to use `DepylerValue` as the key type whenever a `float` key is detected.

---

## 3.19 Root Cause: Generator/Iterator Lifecycle (DEPYLER-1078)
*Status: FIXED* (Jan 11, 2026)

*   **Problem 1 (Ownership)**: Generator state structs captured references to local variables that went out of scope.
    *   **Resolution**: Enforce `.clone()` on non-Copy types during generator struct initialization.
*   **Problem 2 (Semantic Mismatch)**: `next(it, None)` in Python returns the next item or `None`. Our wrapper was generating `Some(it.next())` which resulted in `Option<Option<T>>`.
    *   **Resolution**: Detected the `None` default and mapped directly to `.next()` for standard `Option<T>` behavior.
*   **Problem 3 (Mutability)**: Variables holding iterators were declared with `let`, preventing the `.next()` call which requires `&mut self`.
    *   **Resolution**: Updated `stmt_gen` to detect iterator assignments and enforce `let mut`.

---

## 3.20 Root Cause: Tuple & Result Type Mismatches (DEPYLER-1079)
*Status: FIXED* (Jan 11, 2026)

*   **Problem 1 (Result Mapping)**: Functions returning `Result<Option<T>>` failed when using `return x if cond else None`.
    *   **Resolution**: Wrapped the if-expression in `Ok(...)` to match the return signature.
*   **Problem 2 (Vec Truthiness)**: Explicit `if vec:` checks failed for `Vec` because it's not a boolean.
    *   **Resolution**: Added support to convert `vec` to `!vec.is_empty()` in if-expression conditions.
*   **Problem 3 (Zip Ownership)**: `zip()` on reference collections yielded `(&T, &U)`, but consumers expected `(T, U)`.
    *   **Resolution**: Added `.map(|(a,b)| (a.clone(), b.clone()))` to the zip iterator chain.

---

## 3.21 Root Cause: Lifetime Mismatch in Iterator Returns (DEPYLER-1080)
*Status: FIXED* (Jan 11, 2026)

*   **Problem**: Functions with multiple reference parameters returning `impl Iterator + 'a` failed with E0623.
    *   **Why 1**: The iterator captures references to both parameters but only bounds on `'a`.
    *   **Why 2**: Rust requires all captured references to have compatible lifetimes.
    *   **Why 3**: Different lifetime params ('a, 'b, 'c) create incompatible bounds.
*   **Resolution**: Modified `func_gen_inference.rs` to unify all reference parameter lifetimes to single `'a`.

---

## 3.22 Root Cause: Generator Iterator State (DEPYLER-1082)
*Status: FIXED* (Jan 11, 2026)

*   **Problem 1 (E0308 Boxing)**: Generator struct fields were typed as `Box<dyn Iterator>` but initialization used bare impl Iterator params.
    *   **Resolution**: Added `Box::new(...) as _` wrapper for Iterator/Generator params.
*   **Problem 2 (E0277 Debug)**: `#[derive(Debug)]` fails for structs containing `Box<dyn Iterator>`.
    *   **Resolution**: Generate manual `Debug` impl using `finish_non_exhaustive()`.
*   **Problem 3 (E0271 FlatMap)**: Identity patterns returned `&i32` instead of owned values.
    *   **Resolution**: Detect identity patterns and use `.copied()` instead of `.map(|x| x)`.
*   **Problem 4 (E0599 IntoIterator)**: `Box<dyn Iterator>` doesn't implement `IntoIterator`.
    *   **Resolution**: Generate `while let Some(x) = self.g.next()` instead of for-loop.

---

## 3.23 Root Cause: Tuple Filter Pattern Move (DEPYLER-1081)
*Status: FIXED* (Jan 11, 2026)

*   **Problem**: Filter closures with tuple destructuring like `|&(i, v)|` caused E0507.
    *   **Why 1**: `|&(i, v)|` attempts to destructure a reference and move ownership of elements.
    *   **Why 2**: `String` is not `Copy`, so moving it out of `&(i32, String)` is invalid.
*   **Resolution**: Use `|(a, b)|` instead of `|&(a, b)|` — Rust's match ergonomics handle it.

---

## 3.24 Root Cause: Integer Cast Precedence (DEPYLER-1083)
*Status: FIXED* (Jan 12, 2026)

*   **Problem**: Slice expressions like `data[i:i+size]` failed due to i32/isize type mixing.
    *   **Why 1**: Generated `let stop_idx = #stop as isize;` where `#stop` is the converted expression.
    *   **Why 2**: When `#stop` is `i + size`, the generated code is `i + size as isize`.
    *   **Why 3**: Rust operator precedence parses this as `i + (size as isize)` not `(i + size) as isize`.
*   **Resolution**: Parenthesize slice index expressions: `(#stop) as isize`.

---

## 3.25 Root Cause: Return Type Inference from Expression Statements (DEPYLER-1084)
*Status: FIXED* (Jan 12, 2026)

*   **Problem**: Functions without explicit returns were incorrectly inferred as returning a type.
    *   **Why 1**: `infer_return_type_from_body` checked for trailing `HirStmt::Expr` and treated it as implicit return.
    *   **Why 2**: Python expression statements just evaluate and discard — they're NOT returns.
    *   **Why 3**: Rust has implicit returns (last expression is return value), but Python doesn't.
*   **Resolution**: Only explicit `return x` statements now contribute to return type inference.

---

## 3.26 Root Cause: Value Lifting for Branch Type Unification (DEPYLER-1085)
*Status: FIXED* (Jan 12, 2026)

*   **Problem**: If/else branches with mismatched types (one DepylerValue, one concrete) caused E0308.
    *   **Why 1**: `data[i] if cond else 0` generates different types when `data` has Unknown element type.
    *   **Why 2**: `data[i]` with Unknown elements becomes `DepylerValue`, but `0` is `i32`.
    *   **Why 3**: Rust requires both branches to have the same type.
*   **Resolution**: Implemented Value Lifting in `convert_ifexpr()` — detect DepylerValue branches and lift concrete branches.

---

## 3.27 Root Cause: Time Module Semantic Mismatch (DEPYLER-1086)
*Status: FIXED* (Jan 12, 2026)

*   **Problem**: Three time-related tests failed despite correct code generation logic.
    *   **Why 1**: Tests expected `chrono` types but `CodeGenContext` defaults to `nasa_mode = true`.
    *   **Why 2**: In NASA mode, `time` module maps to `std::time`, not `chrono`.
*   **Resolution**: Updated test suite assertions to match NASA mode expectations.

---

## 3.28 Root Cause: Inline Clap Attributes Line Removal (DEPYLER-1088)
*Status: FIXED* (Jan 12, 2026)

*   **Problem**: Parse errors (~70 files) with "unexpected close delimiter".
    *   **Why 1**: Line-based filter removed `#[command(...)]` and `#[arg(...)]` attributes in NASA mode.
    *   **Why 2**: `rustfmt` occasionally put attributes inline with enum variants.
    *   **Why 3**: Line filter deleted the *entire line*, including the variant definition.
*   **Resolution**: Move attribute removal logic *before* line filtering.

---

## 3.29 Strategic Analysis: Breaking the E0308 Stalemate (DEPYLER-1100)
*Status: COMPLETE* (Jan 13, 2026)

**Five Architectural Strategies Evaluated**:

1. **Inverse Lowering** (DepylerValue-first): ❌ Defeats transpilation purpose
2. **Trait-Based Dispatch** (PyOps): ✅ Architecturally sound, long-term scalable
3. **Global Unification Engine** (Hindley-Milner Lite): ❌ Very high effort
4. **Profile-Guided Optimization** (MonkeyType): ❌ Requires test execution
5. **Automated Oracle Loop** (Compiler-Driven Repair): ✅ Lowest risk, fastest to implement

**Recommendation**: Phased Hybrid Approach — Enhanced Oracle Loop + Targeted Trait Dispatch.

---

## 3.30 Root Cause: Inference Black Box (DEPYLER-1148)
*Status: FIXED* (Jan 16, 2026)

*   **Problem**: The transpiler reached a plateau at 43%. Internal reasoning for incorrect type mappings was invisible.
*   **Why 1**: Why did `slice_example` become a `String` when base was `Vec<i32>`?
    *   *Answer*: The inference engine lacked base-aware context.
*   **Why 2**: Why was this hard to debug?
    *   *Answer*: The transpilation pipeline was a "Black Box".
*   **Resolution**: Activated the **CITL Flight Recorder** with `init_decision_tracing()`.

---

## Lessons Learned

1. **Python and Rust have fundamentally different semantics** — implicit returns, truthiness, ownership all differ
2. **Operator precedence matters** — always parenthesize complex expressions before casting
3. **Trait system can handle complexity** — PyOps traits proved more maintainable than manual coercion
4. **Visibility is critical** — the Flight Recorder enabled rapid debugging of inference issues
5. **NASA mode creates parallel paths** — must test both modes to avoid regressions

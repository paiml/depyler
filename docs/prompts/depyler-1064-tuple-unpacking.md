**To:** Depyler Core Engineering Team
**From:** Architecture Lead
**Subject:** DEPYLER-1064: The Tuple Unpacking Offensive

Team,

We have momentum. The fix for `argparse` option matching (DEPYLER-1063) unlocked a massive 18.7% of our corpus. We are now at 23.0% single-shot compilation on `reprorusted`.

Our next target is **Tuple Unpacking (E0308)**. This is the single largest remaining cluster of type mismatch errors (~2,800 occurrences).

### The Problem: Inert Tuples
When `DepylerValue` holds a tuple, Rust doesn't know how to unpack it.
```python
# Python
x, y = get_coordinates()  # Returns (int, int) or DepylerValue
```
```rust
// Current Rust Output (Fails)
let (x, y) = get_coordinates(); // Error: expected tuple, found DepylerValue
```

### The Solution: Active Extraction
We must teach `DepylerValue` to *be* a tuple source.

**Implementation Plan (DEPYLER-1064):**

1.  **Enhance `DepylerValue`**:
    *   Implement `get_tuple_elem(&self, index: usize) -> DepylerValue`.
    *   This method should handle `DepylerValue::Tuple`, `DepylerValue::List`, and panic safely (or return `None`-equivalent) for other variants.

2.  **Update Codegen (`stmt_gen.rs`)**:
    *   Detect `HirStmt::Assign` where `target` is `Tuple(...)`.
    *   **Crucial Change**: Instead of generating a `let` binding with a pattern, generate a temporary variable and explicit extraction calls.
    *   *Example Target Rust*:
        ```rust
        let _tmp = get_coordinates();
        let x = _tmp.get_tuple_elem(0).extract_as_hint(Type::Int);
        let y = _tmp.get_tuple_elem(1).extract_as_hint(Type::Int);
        ```

3.  **Falsification Criteria**:
    *   **Heterogeneous**: `x, y = (1, "a")`. Must extract `i32` and `String` correctly.
    *   **Nested**: `(x, (y, z)) = ...`. Recursive extraction must work.
    *   **Length Mismatch**: `x, y = (1,)`. Must panic with a clear message at runtime, NOT segfault or compile error.

**Why this works**: It moves the "shape" check from compile time (where we lack info) to runtime (where we have the data). This aligns perfectly with our Hybrid Fallback strategy.

**Execute DEPYLER-1064.** This is the key to breaking the 30% barrier.

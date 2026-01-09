# 4. Five Whys Root Cause Analysis

[← Back to TOC](../pareto-complete-single-shot.md)

---

## Summary

**Root Cause**: The type inference architecture is **flow-insensitive**, causing cascading failures when Python features combine.

---

## Why #1: Why is compilation rate 22% when feature coverage is 95%?

**Answer**: QA tests features in **isolation**. Real code combines features that interact.

**Evidence**: 20% of defects are "Integration Failures" per OIP analysis.

---

## Why #2: Why do feature interactions cause type errors?

**Answer**: Type inference **loses context** when features combine. Falls back to `serde_json::Value`.

```rust
// Generated (wrong):
let items: serde_json::Value = ...;
items.iter().map(|(k, v)| ...)  // E0599: no method `iter`

// Should be:
let items: HashMap<String, i32> = ...;
items.iter().map(|(k, v)| ...)  // ✓
```

---

## Why #3: Why does type inference fail in complex contexts?

**Answer**: System is **flow-insensitive**—types each expression independently.

```python
x = get_data()      # Unknown
y = x.process()     # Unknown (can't infer from Unknown)
z = y.result        # Unknown (cascade)
```

---

## Why #4: Why is the type system flow-insensitive?

**Answer**: **Technical debt** taken for velocity. Original design prioritized shipping over robustness.

```
Sprint 1: Get basic transpilation working → ✅
Sprint 2: Add more Python features → ✅
Sprint 3: Fix type errors → Patch E0308
Sprint 4: More fixes → Patch E0599
... (90+ sprints of patching)
```

---

## Why #5: Why hasn't the type system been fixed?

**Answer**: **Whack-a-Mole antipattern**. Each fix adds complexity without improving architecture.

```
[E0308] → Patch → [E0425] → Patch → [E0599] → Patch → ...
                     ↓
              New errors emerge
                     ↓
              More patches needed
```

---

## Conclusion

We've been treating **symptoms** (individual error codes) instead of the **disease** (flow-insensitive type inference).

**Solution**: Two-phase bidirectional type inference. See [08-architecture.md](08-architecture.md).

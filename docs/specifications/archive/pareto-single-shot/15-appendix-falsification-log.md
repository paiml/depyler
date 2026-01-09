# 15. Appendix A: Falsification Log

[← Back to TOC](../pareto-complete-single-shot.md)

---

## Purpose

Track all falsification attempts per Popperian methodology. Every architectural change is a hypothesis that can be disproven.

---

## Log Format

```
Cycle N: [Title] (Date)
- Hypothesis: What we conjectured
- Test: How we tested it
- Result: CONFIRMED / FALSIFIED
- Evidence: Metrics
- Action: What we did next
```

---

## Cycle 14: i32→i64 Consistency (December 15, 2025)

**Hypothesis**: Unifying Type::Int to i64 across all type mappers will fix E0308 mismatches between clap args and function parameters.

**Test**: `depyler converge --seed 42`

**Result**: 🔴 **FALSIFIED**

**Evidence**:
- Before: 22.0% (139/632)
- After: 18.7% (113/604)
- Regression: -3.3 points

**Action**: Jidoka triggered. Investigate root cause before proceeding.

---

## Cycle 13: Sprint 2 Forward Propagation (December 14, 2025)

**Hypothesis**: Forward type propagation will improve convergence on subset.

**Test**: Subset corpus test

**Result**: ✅ **CONFIRMED** (on subset)

**Evidence**:
- Subset: 73% convergence achieved
- Full corpus: 22% (unchanged)

**Action**: Subset success does not generalize. Need full corpus measurement.

---

## Cycle 12: Deque Type Tracking (December 13, 2025)

**Hypothesis**: Tracking `deque` as collection type will fix truthiness checks.

**Test**: `depyler converge`

**Result**: ⚠️ **INCONCLUSIVE**

**Evidence**: Minor improvement in specific cases, no significant corpus-level change.

**Action**: Continue with type inference improvements.

---

## Cycle 1-11: Various Patches

Multiple error-specific patches following Whack-a-Mole pattern. See git history for details.

**Cumulative Result**: 95% QA checklist, 22% convergence. Pattern recognized as antipattern.

---

## Template for New Entries

```markdown
## Cycle N: [Title] (Date)

**Hypothesis**: [What we conjectured]

**Test**: [How we tested it]

**Result**: [CONFIRMED / FALSIFIED / INCONCLUSIVE]

**Evidence**:
- Before: X%
- After: Y%
- Delta: ±Z points

**Action**: [What we did next]
```

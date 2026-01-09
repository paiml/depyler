# 10. Implementation Roadmap

[← Back to TOC](../pareto-complete-single-shot.md)

---

## Sprint Plan

| Sprint | Focus | Target | Status |
|--------|-------|--------|--------|
| **S1** | Type fallback + imports | 30% | ✅ Completed |
| **S2** | Forward propagation | 45% | ✅ 73% achieved |
| **S3** | Backward + call-site | 70% | 🔴 Regression (18.7%) |
| **S4** | Unification | 80% | Pending |

---

## Checkpoints

### Checkpoint 1 (S1 Complete)
- [x] Type fallback generates valid Rust
- [x] Import tracking implemented
- [x] Baseline: 30% convergence

### Checkpoint 2 (S2 Complete)
- [x] Forward type propagation
- [x] Module stub inference
- [x] Target: 45% → **Achieved 73%**

### Checkpoint 3 (S3 - Current)
- [ ] Backward type propagation
- [ ] Call-site specialization
- [ ] Target: 70%
- **Status**: 🔴 Regression detected (18.7%)
- **Action**: Jidoka - investigate root cause

### Checkpoint 4 (S4 - Pending)
- [ ] Full type unification
- [ ] Constraint solving
- [ ] Target: 80%+

---

## Statistical Validation

Each checkpoint must report:
1. **Sample Size (n)**: Number of files (e.g., n=604)
2. **Confidence Interval**: 95% CI (e.g., 30% ± 4%)
3. **Effect Size**: Cohen's d vs baseline
4. **Raw Data**: `data/convergence-sprint-X.json`

---

## Command

```bash
depyler converge --corpus $CORPUS --output-stats stats.json --seed 42
```

---

## Current Status (December 15, 2025)

```
CONVERGE | Dir: examples | Target: 80.0%
[604/604] Compiling... 100% complete
📊 Oracle: Loaded cached model
DONE | NOT_CONVERGED | 18.7% (113/604) | 1 iterations
Error: Target rate not reached: 18.7% < 80.0%
```

**Root Cause Investigation Required**: i32→i64 changes (DEPYLER-1017) likely introduced regressions.

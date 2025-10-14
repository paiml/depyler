# Depyler Roadmap: A → C → B Strategy

**Decision**: User selected sequence A, C, then B for next milestones
**Rationale**: Achieve 80% coverage, improve quality, then add features

## Sprint Sequence

### ✅ v3.19.0: Quality-Focused Coverage (COMPLETE)
- **Duration**: 1 day (9 hours)
- **Achievement**: 77.52% → 77.66% (+0.14%)
- **Tests Added**: 46 comprehensive tests (unit + property + mutation)
- **Status**: COMPLETE

### 🎯 v3.19.1: Precision Coverage → 80% (NEXT)
- **Goal**: Close 2.34% gap (77.66% → 80.00%)
- **Duration**: 3-4 hours
- **Tests**: ~10 targeted tests
- **Strategy**: Precision targeting of highest-impact modules
- **Status**: PLANNED

**Target Modules**:
1. import_gen.rs (60%, 28 lines) → +0.12%
2. context.rs (66%, 12 lines) → +0.05%
3. func_gen.rs (69%, 170 lines) → +0.70%
4. stmt_gen.rs (82%, 100 lines) → +0.41%
5. type_mapper.rs (75%, 165 lines) → +0.68%

**Expected Total**: +1.96% → 79.62%, buffer work for final 0.38%

### 🔧 v3.19.2: Quality Improvements (AFTER v3.19.1)
- **Goal**: Reduce complexity violations from 57 to ≤40
- **Duration**: 6-8 hours
- **Reduction**: 16-17 violations (28-30% improvement)
- **Strategy**: "Low-hanging fruit" - functions with complexity 11-15
- **Status**: PLANNED

**Target Functions**:
- stmt_gen.rs: 11 → 5-6 violations (-45% to -55%)
- func_gen.rs: 2 → 0 violations (-100%) ✅
- expr_gen.rs: 44 → 42 violations (-4.5%)

**Efficiency**: ~2.5 violations reduced per hour

### 🚀 v3.20.0: Feature Work (AFTER v3.19.2)
- **Goal**: Add 9 high-value Python features
- **Duration**: 2-3 weeks (84-104 hours)
- **Impact**: Doubles useful Python subset
- **Status**: PLANNED

**Priority 1 Features** (Week 1):
- F-Strings (12-16h, HIGH impact)
- Match Statements (16-20h, HIGH impact)
- Walrus Operator (4-6h, MEDIUM impact)

**Priority 2 Features** (Week 2):
- TypedDict (8-10h, MEDIUM impact)
- Protocol Support (10-12h, MEDIUM impact)

**Priority 3 Features** (Week 2-3):
- collections module (12-16h, HIGH impact)
- itertools module (8-12h, MEDIUM impact)

**Priority 4 Features** (Week 3):
- Custom Exceptions (6-8h, MEDIUM impact)
- Context Managers (8-10h, MEDIUM impact)

## Timeline

```
Week 1:
- Day 1-2: v3.19.1 (precision coverage → 80%)
- Day 2-3: v3.19.2 (quality improvements)
- Day 3-5: v3.20.0 Week 1 (f-strings, match, walrus)

Week 2:
- Day 1-3: v3.20.0 Week 2 (TypedDict, Protocol, collections)
- Day 4-5: v3.20.0 Week 3 start (itertools)

Week 3:
- Day 1-3: v3.20.0 Week 3 (exceptions, context managers)
- Day 4-5: Polish, testing, documentation
- Release: v3.20.0
```

## Success Criteria

### v3.19.1
✅ Coverage ≥80.00%
✅ All tests passing
✅ Zero clippy warnings

### v3.19.2
✅ Complexity violations ≤40 (from 57)
✅ All refactored functions ≤10
✅ Zero regressions

### v3.20.0
✅ All 9 features implemented
✅ 122+ new tests
✅ All quality gates passing
✅ Documentation updated

## Total Investment

- **Time**: ~3.5 weeks
- **v3.19.1**: 0.5 days
- **v3.19.2**: 1 day
- **v3.20.0**: 2-3 weeks

**Expected Outcomes**:
- 80% test coverage ✅
- <40 complexity violations ✅
- 9 new major features ✅
- Significantly expanded Python subset ✅

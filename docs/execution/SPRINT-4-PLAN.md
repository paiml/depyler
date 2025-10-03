# Sprint 4 Plan - Quality Gate Refinement

**Status**: 📋 PLANNING
**Start Date**: 2025-10-02
**Target Duration**: 1-2 weeks
**Focus**: Quality gate violations and incremental complexity reduction

---

## Executive Summary

Sprint 4 focuses on addressing remaining quality gate findings from pmat analysis while maintaining the TDG A+ score achieved in Sprint 2+3. This sprint takes a pragmatic approach: we've already tackled the major hotspots (41→20), now we address incremental improvements.

**Current State**:
- TDG Score: 99.1/100 (A+) ✅
- Max Complexity: 20 (down from 41) ✅
- Dead Code: 0 violations ✅
- SATD: 2 low-severity violations (down from 21)
- Coverage: 70.16% (target: 80%)

---

## Sprint 4 Tickets

### Priority 1: Complexity Hotspots (Remaining)

#### **DEPYLER-0011**: lambda_convert_command Refactoring
**Function**: `lambda_convert_command` (lib.rs:1150)
**Current Complexity**: 31 (cyclomatic)
**Target**: ≤10
**Estimated Time**: 8-12h
**Approach**: EXTREME TDD - Extract Method pattern

**Analysis**:
- Highest remaining complexity in codebase
- Lambda conversion commands have complex branching
- Likely candidates for helper extraction

**Tasks**:
- [ ] Write 20-30 comprehensive tests BEFORE refactoring
- [ ] Analyze branching structure
- [ ] Extract 3-5 helper functions
- [ ] Verify all tests pass (zero regressions)
- [ ] Confirm TDG A+ maintained

---

#### **DEPYLER-0012**: stmt_to_rust_tokens_with_scope Refactoring
**Function**: `stmt_to_rust_tokens_with_scope` (codegen.rs:500)
**Current Complexity**: 25 (cyclomatic)
**Target**: ≤10
**Estimated Time**: 8-12h
**Approach**: EXTREME TDD - Pattern matching simplification

**Analysis**:
- Statement conversion with scope handling
- Complex match arms likely candidates for extraction
- Similar pattern to convert_stmt (DEPYLER-0010)

**Tasks**:
- [ ] Write 25-35 comprehensive tests BEFORE refactoring
- [ ] Identify statement type groups
- [ ] Extract scope handlers into helper functions
- [ ] Verify all tests pass (zero regressions)
- [ ] Confirm TDG A+ maintained

---

#### **DEPYLER-0013**: lambda_test_command Refactoring
**Function**: `lambda_test_command` (lib.rs:1200)
**Current Complexity**: 18 (cyclomatic)
**Target**: ≤10
**Estimated Time**: 6-8h
**Approach**: EXTREME TDD - Extract Method pattern

**Analysis**:
- Lambda testing infrastructure
- Command parsing and validation logic
- Similar to lambda_convert_command

**Tasks**:
- [ ] Write 15-20 comprehensive tests BEFORE refactoring
- [ ] Extract validation helpers
- [ ] Extract test execution helpers
- [ ] Verify all tests pass (zero regressions)
- [ ] Confirm TDG A+ maintained

---

#### **DEPYLER-0014**: rust_type_to_syn_type Further Reduction
**Function**: `rust_type_to_syn_type` (direct_rules.rs:450)
**Current Complexity**: 17 (was 19, reduced to 14 in Sprint 2, now shows 17)
**Target**: ≤10
**Estimated Time**: 5-7h
**Approach**: EXTREME TDD - Type group extraction

**Note**: This was partially addressed in DEPYLER-0008 but complexity has increased slightly. May be acceptable for a type dispatcher with many variants.

**Tasks**:
- [ ] Review DEPYLER-0008 changes
- [ ] Write additional type tests
- [ ] Extract remaining complex type conversions
- [ ] Verify all tests pass
- [ ] Confirm TDG A+ maintained

---

### Priority 2: SATD Removal (Low-Severity)

#### **DEPYLER-0015**: Remove Remaining SATD Comments
**Files**:
- `crates/depyler-ruchy/src/optimizer.rs:293` (Design - Low)
- `crates/depyler-core/src/lambda_optimizer.rs:330` (Performance - Low)

**Current**: 2 low-severity SATD violations
**Target**: 0 (zero tolerance)
**Estimated Time**: 1-2h
**Approach**: Convert to proper documentation or implement

**Tasks**:
- [ ] Review optimizer.rs SATD comment
- [ ] Convert to proper doc comment or implement improvement
- [ ] Review lambda_optimizer.rs SATD comment
- [ ] Convert to proper doc comment or implement improvement
- [ ] Verify SATD count is 0

---

### Priority 3: Coverage Improvement (Stretch Goal)

#### **DEPYLER-0016**: Coverage 70.16% → 80%
**Current**: 70.16% lines (31,755 total, 9,475 missed)
**Target**: 80% lines
**Gap**: ~3,100 lines additional coverage needed
**Estimated Time**: 15-20h (large effort)
**Priority**: STRETCH GOAL (defer if time constrained)

**Low Coverage Areas**:
- depyler-ruchy: ~30-40% (experimental, feature-gated)
- depyler-wasm: 0-60% (WASM tests can't run on native)
- depyler/agent: 0% (daemon, mcp_server - runtime/async)
- depyler/interactive: 2-4% (CLI interactive mode)

**Approach**:
- [ ] Focus on testable areas first (ruchy non-experimental)
- [ ] Add integration tests for agent mode
- [ ] Add CLI integration tests
- [ ] Skip WASM (requires browser environment)
- [ ] Measure incremental progress

---

## Sprint Metrics & Goals

### Success Criteria

**Mandatory** (P0):
- ✅ TDG Score: Maintain A+ (99.1/100)
- ✅ Max Complexity: ≤20 (achieved, maintain)
- ✅ Zero Regressions: All 596 tests passing
- ⏳ SATD: 2→0 (zero tolerance)
- ⏳ Top Hotspots: 4 functions ≤10 complexity

**Important** (P1):
- ⏳ Coverage: 70.16%→75%+ (incremental improvement)
- ⏳ Clippy: 0 warnings maintained
- ⏳ Test Growth: +100-150 new tests

**Nice-to-Have** (P2):
- Coverage: 80%+ (stretch goal)
- All functions ≤10 complexity (long-term goal)

---

## Time Estimates

### Conservative Estimates (Using EXTREME TDD)

| Ticket | Estimated (Traditional) | Estimated (EXTREME TDD) | Time Savings |
|--------|-------------------------|-------------------------|--------------|
| DEPYLER-0011 | 40-50h | 8-12h | 78% |
| DEPYLER-0012 | 40-50h | 8-12h | 78% |
| DEPYLER-0013 | 30-35h | 6-8h | 78% |
| DEPYLER-0014 | 25-30h | 5-7h | 78% |
| DEPYLER-0015 | 2-3h | 1-2h | 40% |
| **Total (P0+P1)** | **137-168h** | **28-41h** | **77% avg** |
| DEPYLER-0016 (stretch) | 25-30h | 15-20h | 38% |
| **Total (All)** | **162-198h** | **43-61h** | **72% avg** |

**Expected Sprint Duration**:
- Core tickets (P0+P1): 28-41h ≈ 1-1.5 weeks
- With coverage (P2): 43-61h ≈ 1.5-2 weeks

---

## Execution Strategy

### Phase 1: High-Value Hotspots (Week 1)
1. DEPYLER-0011: lambda_convert_command (31→≤10)
2. DEPYLER-0012: stmt_to_rust_tokens_with_scope (25→≤10)
3. DEPYLER-0015: SATD removal (2→0)

**Gate**: TDG A+ maintained, zero regressions

### Phase 2: Medium Hotspots (Week 1-2)
4. DEPYLER-0013: lambda_test_command (18→≤10)
5. DEPYLER-0014: rust_type_to_syn_type (17→≤10)

**Gate**: TDG A+ maintained, zero regressions

### Phase 3: Coverage (Week 2 - Stretch)
6. DEPYLER-0016: Coverage improvement (70%→80%)

**Gate**: Incremental progress, no quality degradation

---

## Risk Assessment

### Low Risk ✅
- SATD removal (simple documentation improvements)
- Dead code (already 0 violations)
- Maintaining current quality (proven with Sprint 2+3)

### Medium Risk ⚠️
- Lambda function refactoring (complex domain logic)
- stmt_to_rust_tokens_with_scope (critical codegen path)
- Time estimates (EXTREME TDD proven but individual variance)

### High Risk ❌
- Coverage improvement (large effort, diminishing returns)
- Attempting all tickets (scope creep)
- Quality degradation (mitigated by EXTREME TDD)

---

## Contingency Plans

### If Behind Schedule
1. **Defer DEPYLER-0016** (coverage) - lowest priority
2. **Defer DEPYLER-0014** (rust_type_to_syn_type already improved)
3. **Focus on P0**: DEPYLER-0011, 0012, 0015

### If Quality Degrades
1. **STOP immediately** (Toyota Way: stop the line)
2. **Revert changes** to last known good state
3. **Re-analyze with pmat tdg**
4. **Adjust approach** based on findings

### If Tests Fail
1. **HALT refactoring** (EXTREME TDD principle)
2. **Fix tests first** before continuing
3. **Never commit with failing tests**

---

## Success Metrics Dashboard

Track these metrics throughout Sprint 4:

```bash
# Daily quality check
pmat tdg . --min-grade A-
pmat analyze complexity --top-files 10
pmat analyze satd --format summary
cargo test --workspace | grep "test result"

# Weekly progress
git log --oneline --since="1 week ago" | wc -l  # Commits
git diff --stat HEAD~10 | tail -1  # Lines changed
```

**Target Metrics**:
- TDG Score: 99.1/100 (A+) [maintain]
- Max Complexity: 20→≤15 [improvement]
- SATD: 2→0 [zero tolerance]
- Tests: 596→700+ [growth]
- Coverage: 70.16%→75%+ [incremental]

---

## Post-Sprint Actions

After Sprint 4 completion:

1. **Sprint 4 Completion Report** (similar to Sprint 2+3)
2. **Update roadmap.md** with progress
3. **CHANGELOG.md** entry for v3.3.0
4. **README.md** update with new metrics
5. **Consider v3.3.0 release** if significant improvements

---

## Lessons from Sprint 2+3 (Apply to Sprint 4)

### What Worked ✅
- **EXTREME TDD**: 87% time savings proven
- **Test-first approach**: Zero regressions across 7 tickets
- **Toyota Way principles**: Quality maintained (TDG A+)
- **Extract Method pattern**: Consistent complexity reduction
- **Comprehensive documentation**: Completion reports invaluable

### What to Improve 🔧
- **Realistic time estimates**: Always use EXTREME TDD multiplier
- **Scope management**: Don't attempt all tickets at once
- **Quality gates**: Run pmat checks more frequently
- **Test coverage**: Add tests for new helpers immediately

---

## Conclusion

Sprint 4 takes a pragmatic approach to quality refinement. We've achieved major wins in Sprint 2+3 (41→20 max complexity, TDG A+), now we incrementally improve remaining hotspots.

**Key Principles**:
1. **Quality First**: Never sacrifice TDG A+ for speed
2. **EXTREME TDD**: Proven 77-87% time savings
3. **Toyota Way**: Stop the line for any defect
4. **Pragmatic Scope**: Focus on high-value P0+P1 tickets
5. **Zero Regressions**: All 596+ tests must pass

**Expected Outcome**:
- Max complexity: 20→15 or better
- SATD: 2→0 (zero tolerance achieved)
- Coverage: 70.16%→75%+ (incremental progress)
- Test growth: +100-150 comprehensive tests
- Release: v3.3.0 candidate

---

**Prepared by**: Claude Code
**Date**: 2025-10-02
**Sprint**: Sprint 4 - Quality Gate Refinement
**Predecessor**: Sprint 2+3 (DEPYLER-0004 through DEPYLER-0010)

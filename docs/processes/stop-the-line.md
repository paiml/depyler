# Stop the Line (Jidoka) Protocol

## 🛑 Philosophy: Never Pass Defects Downstream

**Toyota Way Principle**: When a defect is discovered, **STOP IMMEDIATELY**. Do not continue until the root cause is fixed and verified.

This document defines Depyler's mandatory defect response protocol.

---

## When to Stop the Line

### 🚨 IMMEDIATE STOP - P0 Defects

Stop ALL work immediately for:

1. **Compilation Failures**: Generated Rust code doesn't compile
2. **Type Safety Violations**: Generated code has type errors
3. **Memory Safety Issues**: Generated code has use-after-free, double-free, etc.
4. **Incorrect Semantics**: Transpiled code behavior differs from Python

**Action**: Create GitHub issue with `stop-the-line` label, halt all feature work.

### ⚠️ BLOCK RELEASE - P1 Defects

Block releases (but can continue other work):

1. **Clippy Warnings**: Generated code triggers clippy warnings with `-D warnings`
2. **Non-Idiomatic Code**: Generates valid but unidiomatic Rust
3. **Performance Regressions**: Transpilation speed drops >20%
4. **Coverage Gaps**: Matrix-testing coverage falls below 80%

**Action**: Create GitHub issue, fix before next release.

### 📋 TRACK - P2/P3 Defects

Track for future improvement:

1. **Optimization Opportunities**: Better code generation possible
2. **Documentation Gaps**: Missing explanations
3. **Error Message Quality**: Confusing diagnostics

**Action**: Create GitHub issue with appropriate priority label.

---

## The Stop the Line Process

### Step 1: 🛑 STOP

**Immediately when defect discovered:**

```bash
# 1. Stop current work
git stash  # Save current work

# 2. Document the defect
echo "DEFECT DISCOVERED: $(date)" >> .stop-the-line.log
echo "Description: [brief summary]" >> .stop-the-line.log
echo "Evidence: [quality gate that failed]" >> .stop-the-line.log
```

**Communication:**
- Post in team chat: "🛑 STOP THE LINE - [brief description]"
- Update sprint board: Mark current task as BLOCKED
- No new feature work until defect resolved

### Step 2: 📋 DOCUMENT

**Create GitHub Issue** using `transpiler_bug.yml` template:

```bash
# Navigate to GitHub Issues
# Click "New Issue" → "Transpiler Bug (Stop the Line)"
# Fill out ALL required fields:
#   - Ticket ID: DEPYLER-XXXX (next sequential number)
#   - Severity: P0 (if blocking)
#   - Discovery Phase: Matrix Testing - Column A → B
#   - Bug Description: [detailed explanation]
#   - Expected Output: [idiomatic Rust]
#   - Actual Output: [current generation]
#   - Python Source: [input code]
#   - Quality Gate Failures: [which gates failed]
```

**Ticket Naming Convention:**
- Format: `[DEPYLER-XXXX] Brief description`
- Example: `[DEPYLER-0269] isinstance() generates invalid Rust code`

**Sequential Numbering:**
```bash
# Find next ticket number
gh issue list --label transpiler --json number --jq '.[0].number' | awk '{print $1 + 1}'

# Or check docs/execution/roadmap.yaml for latest DEPYLER-XXXX
```

### Step 3: 🔍 ANALYZE

**Root Cause Analysis** (Genchi Genbutsu - Go See):

```bash
# 1. Reproduce the bug
cd examples/01_basic_types/column_a
depyler transpile column_a.py --output /tmp/test.rs

# 2. Examine generated code
cat /tmp/test.rs | grep -A 5 "BUG_PATTERN"

# 3. Find transpiler source
rg "BUG_PATTERN" crates/depyler-core/src/rust_gen/

# 4. Understand why it generates incorrect code
# - Read the function that generates this pattern
# - Check if it's a template issue or logic bug
# - Identify ALL similar patterns
```

**Document Root Cause in GitHub Issue:**
```markdown
## Root Cause Analysis

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs:2143`

**Problem**: `isinstance()` calls are not stripped during transpilation

**Why**: The transpiler treats `isinstance()` as a regular function call
instead of recognizing it as a Python builtin that should be removed
in statically-typed Rust.

**Scope**: Affects ALL examples using `isinstance()` (estimated 15%)
```

### Step 4: 🔧 FIX

**Test-Driven Fix** (Extreme TDD):

```bash
# 1. Write FAILING test FIRST
cat > tests/depyler_0269_isinstance_test.rs << 'EOF'
#[test]
fn test_isinstance_removed_in_rust() {
    let python = r#"
def check_type(x: int) -> bool:
    return isinstance(x, int)
"#;
    let rust = depyler_transpile(python);

    // Should NOT contain isinstance
    assert!(!rust.contains("isinstance"));

    // Should return true (type system guarantees)
    assert!(rust.contains("true"));
}
EOF

# 2. Verify test FAILS (RED)
cargo test depyler_0269_isinstance_test
# Expected: FAIL (isinstance still generated)

# 3. Fix transpiler code
# Edit crates/depyler-core/src/rust_gen/expr_gen.rs
# Add logic to detect isinstance() and replace with type-safe equivalent

# 4. Verify test PASSES (GREEN)
cargo test depyler_0269_isinstance_test
# Expected: PASS

# 5. Run ALL tests (no regressions)
cargo test --workspace
# Expected: ALL PASS
```

**Quality Gates for Fix:**

```bash
# MANDATORY: All must pass before proceeding

# 1. All tests pass
cargo test --workspace

# 2. Zero clippy warnings
cargo clippy --workspace -- -D warnings

# 3. TDG grade maintained
pmat tdg . --min-grade A- --fail-on-violation

# 4. No regressions in existing examples
cd python-to-rust-conversion-examples
for example in examples/*/column_a/column_a.py; do
    depyler transpile "$example" --output /tmp/test.rs
    rustc --crate-type lib /tmp/test.rs || echo "REGRESSION: $example"
done
```

### Step 5: 🔄 RE-TRANSPILE

**Re-generate ALL affected examples:**

```bash
# 1. Find all affected files
cd python-to-rust-conversion-examples
rg "isinstance" examples/*/column_a/*.py -l > affected_files.txt

# 2. Re-transpile each one
while read -r python_file; do
    example_dir=$(dirname "$python_file")
    rust_file="${example_dir/column_a/column_b}/src/lib.rs"

    echo "Re-transpiling: $python_file → $rust_file"
    depyler transpile "$python_file" --output "$rust_file"

    # Verify compilation
    cd "$(dirname "$rust_file")/.."
    cargo check || echo "❌ FAILED: $rust_file"
    cd -
done < affected_files.txt

# 3. Verify ALL quality gates
./scripts/validate_all_examples.sh
```

### Step 6: ✅ VERIFY

**Comprehensive Verification:**

```bash
# 1. Run full CI/CD pipeline locally
./.github/workflows/validate.yml  # (if using act)
# OR
gh workflow run validate.yml --ref $(git branch --show-current)

# 2. Check mutation testing still passes
cargo mutants --check

# 3. Verify documentation updated
git diff docs/execution/roadmap.yaml CHANGELOG.md

# 4. Ensure ticket checklist complete
gh issue view DEPYLER-XXXX
# All checkboxes must be checked ✅
```

### Step 7: 🚀 RESUME

**Only resume when ALL conditions met:**

- ✅ Fix implemented with regression test
- ✅ All quality gates passing (tests, clippy, TDG, coverage)
- ✅ All affected examples re-transpiled and verified
- ✅ CHANGELOG.md updated with fix description
- ✅ GitHub issue closed with verification evidence
- ✅ Team notified: "✅ Line resumed - [ticket] fixed and verified"

**Resume Command:**

```bash
# 1. Commit the fix
git add -A
git commit -m "[DEPYLER-XXXX] Fix isinstance() transpilation bug

- Remove isinstance() calls in generated Rust
- Type system guarantees make runtime checks unnecessary
- Added regression test: tests/depyler_0269_isinstance_test.rs
- Re-transpiled 12 affected examples

Quality Gates:
- ✅ All tests passing (445/445)
- ✅ Zero clippy warnings
- ✅ TDG grade: A- (maintained)
- ✅ Coverage: 82.3% (no regression)

Closes: DEPYLER-XXXX"

# 2. Push and create PR
git push origin fix/depyler-0269-isinstance
gh pr create --title "[DEPYLER-0269] Fix isinstance() transpilation" \
             --body "Fixes #XXXX - See commit message for details"

# 3. Wait for CI to pass, then merge
gh pr merge --squash --auto

# 4. Announce resumption
echo "✅ STOP THE LINE RESOLVED - Development resumed"

# 5. Continue with original task
git stash pop  # Restore original work
```

---

## Defect Categories and Response Times

| Category | Severity | Response Time | Action |
|----------|----------|---------------|--------|
| **Compilation Failure** | P0 | Immediate (same day) | Stop all work, fix immediately |
| **Type Safety Violation** | P0 | Immediate (same day) | Stop all work, fix immediately |
| **Clippy Warnings** | P1 | 1-2 days | Block release, fix before ship |
| **Non-Idiomatic Code** | P1 | 1 week | Block release, improve quality |
| **Optimization Opportunity** | P2 | 2-4 weeks | Track, fix in future sprint |
| **Documentation Gap** | P3 | Backlog | Track, prioritize with other work |

---

## Metrics and Tracking

### Defect Velocity

Track transpiler improvement over time:

```bash
# Count defects by week
gh issue list --label transpiler --state closed \
  --json closedAt,number \
  --jq 'group_by(.closedAt | strptime("%Y-%m-%d") | strftime("%Y-W%U")) | map({week: .[0].closedAt | strptime("%Y-%m-%d") | strftime("%Y-W%U"), count: length})'
```

**Target Metrics:**
- **Defect Discovery Rate**: Should INCREASE initially (finding more bugs)
- **Defect Resolution Time**: Should DECREASE over time (faster fixes)
- **Defects per Example**: Should DECREASE over time (better transpiler)
- **Reopened Defects**: Should be ZERO (fix once, fix right)

### Quality Trend

```bash
# Track clippy warnings over time
git log --oneline --all --grep="clippy warnings" \
  | grep -oP '\d+ warnings' \
  | awk '{print $1}' \
  | spark  # Sparkline visualization
```

**Success Criteria:**
- Week 1: 50 warnings → Fix transpiler → 30 warnings
- Week 2: 30 warnings → Fix transpiler → 10 warnings
- Week 3: 10 warnings → Fix transpiler → 0 warnings
- **Goal**: Zero warnings generated for ANY Python input

---

## GitHub Automation

### Auto-Label Issues

```yaml
# .github/workflows/label-issues.yml
name: Auto-label Stop the Line Issues
on:
  issues:
    types: [opened]

jobs:
  label:
    runs-on: ubuntu-latest
    steps:
      - name: Add stop-the-line label for P0 bugs
        if: contains(github.event.issue.title, '[DEPYLER-') && contains(github.event.issue.labels.*.name, 'P0')
        uses: actions-ecosystem/action-add-labels@v1
        with:
          labels: stop-the-line
```

### Auto-notify Team

```yaml
# .github/workflows/notify-stop-the-line.yml
name: Notify Team on Stop the Line
on:
  issues:
    types: [labeled]

jobs:
  notify:
    if: github.event.label.name == 'stop-the-line'
    runs-on: ubuntu-latest
    steps:
      - name: Send Slack notification
        run: |
          curl -X POST ${{ secrets.SLACK_WEBHOOK_URL }} \
            -H 'Content-Type: application/json' \
            -d '{
              "text": "🛑 STOP THE LINE: ${{ github.event.issue.title }}",
              "blocks": [{
                "type": "section",
                "text": {
                  "type": "mrkdwn",
                  "text": "*Stop the Line Alert*\n\n${{ github.event.issue.html_url }}"
                }
              }]
            }'
```

---

## Examples of Past Defects

### DEPYLER-0267: Index Access Bug (.copied() vs .cloned())

**Discovery**: Matrix-testing Column A → B
**Root Cause**: `expr_gen.rs:2130` uses `.copied()` for all index access
**Fix**: Change to `.cloned()` for non-Copy types
**Impact**: 3 examples affected, 15 functions fixed
**Time to Fix**: 2 hours
**Verification**: All examples re-transpiled, zero clippy warnings

### DEPYLER-0268: Index Negation Bug

**Discovery**: Matrix-testing Column A → B
**Root Cause**: `expr_gen.rs:2143` generates `(-idx) as usize` (usize doesn't have Neg trait)
**Fix**: Use `.unsigned_abs()` for signed-to-unsigned conversion
**Impact**: All examples with negative indexing
**Time to Fix**: 1 hour
**Verification**: 5 regression tests added, all pass

### DEPYLER-0269: isinstance() Transpilation (Current)

**Discovery**: Matrix-testing Column A → B (01_basic_types)
**Root Cause**: isinstance() not recognized as Python builtin
**Fix**: [IN PROGRESS]
**Impact**: Estimated 15% of examples
**Time to Fix**: [PENDING]

---

## References

### Toyota Way Principles

- **Jidoka (自働化)**: Automation with human intelligence - stop on defects
- **Genchi Genbutsu (現地現物)**: Go see for yourself - inspect actual code
- **Kaizen (改善)**: Continuous improvement - each fix makes transpiler better
- **Hansei (反省)**: Reflection - learn from defects, prevent recurrence

### Scientific Foundation

- **Mutation Testing** (DeMillo et al., 1978): Transpiler bugs are "surviving mutants"
- **Property-Based Testing** (QuickCheck, 2000): Automated defect discovery
- **Formal Verification**: Each fix must be proven correct via tests

### Related Documentation

- [CLAUDE.md](../../CLAUDE.md) - Development protocols
- [roadmap.yaml](../execution/roadmap.yaml) - Ticket tracking
- [CHANGELOG.md](../../CHANGELOG.md) - Release history
- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Contribution guidelines

---

## Appendix: Defect Response Checklist

Print this and post near your workspace:

```
🛑 STOP THE LINE CHECKLIST

When defect discovered:

☐ 1. STOP all feature work immediately
☐ 2. Document defect in .stop-the-line.log
☐ 3. Create GitHub issue with transpiler_bug.yml template
☐ 4. Add stop-the-line label (if P0)
☐ 5. Root cause analysis - find exact location in code
☐ 6. Write FAILING regression test (TDD RED phase)
☐ 7. Fix transpiler code
☐ 8. Verify test PASSES (TDD GREEN phase)
☐ 9. Run ALL quality gates (tests, clippy, TDG, coverage)
☐ 10. Re-transpile ALL affected examples
☐ 11. Verify ALL examples pass quality gates
☐ 12. Update CHANGELOG.md
☐ 13. Commit with [DEPYLER-XXXX] prefix
☐ 14. Create PR with quality gate evidence
☐ 15. Wait for CI to pass
☐ 16. Merge and close issue
☐ 17. Announce: "✅ Line resumed"
☐ 18. Resume original work

NEVER SKIP STEPS. QUALITY IS NON-NEGOTIABLE.
```

---

**Last Updated**: 2025-10-27
**Status**: ACTIVE
**Owner**: Depyler Core Team
**Review Cycle**: Monthly

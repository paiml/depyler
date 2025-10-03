# Depyler Mutation Testing Guide

**Version**: 1.0.0
**Last Updated**: 2025-10-03
**Status**: Active

---

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Workflow](#workflow)
4. [Configuration](#configuration)
5. [Troubleshooting](#troubleshooting)
6. [Best Practices](#best-practices)
7. [Results Interpretation](#results-interpretation)

---

## Overview

### What is Mutation Testing?

Mutation testing validates test quality by introducing bugs (mutations) into code and verifying tests catch them. A **kill rate** measures the percentage of mutations caught.

### Why Mutation Testing?

**Discovery**: Depyler had 596 passing tests with 70% coverage, but only **18.7% mutation kill rate**. This revealed tests validate "doesn't crash" not "is correct."

### Goals

- **Target**: ≥90% mutation kill rate
- **Method**: EXTREME TDD - write tests FIRST to kill specific mutations
- **Scope**: Core transpilation (ast_bridge.rs, codegen.rs, direct_rules.rs)

---

## Quick Start

### 1. Install Tools

```bash
cargo install cargo-mutants --locked
```

### 2. Run Baseline

```bash
# Run mutation test on specific file
cargo mutants --baseline skip \
    --file crates/depyler-core/src/ast_bridge.rs \
    --jobs 2

# Skip baseline validation (workaround for doctest issues)
```

### 3. Analyze Results

```bash
# View mutations by status
grep "MISSED" mutants.out | wc -l    # Count missed mutations
grep "CAUGHT" mutants.out | wc -l    # Count caught mutations
```

### 4. Write Tests to Kill Mutations

```bash
# Create test file targeting specific mutations
cargo test --test <test_file_name>
```

---

## Workflow

### EXTREME TDD Mutation Testing Process

```
┌─────────────────────────────────────────────────────────────┐
│ 1. RUN BASELINE                                             │
│    cargo mutants --baseline skip --file <file> --jobs 2    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. IDENTIFY MISSED MUTATIONS                                │
│    grep "MISSED" mutants.out > missed_mutations.txt         │
│    Categorize by type: boolean, comparison, return, etc.    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. WRITE TESTS FIRST (EXTREME TDD)                          │
│    - Target specific mutations                              │
│    - Test that mutation WOULD fail the test                 │
│    - Example: If mutation is `&&` → `||`, test validates    │
│      that AND is necessary                                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. VERIFY TESTS PASS                                        │
│    cargo test --test <test_file>                            │
│    All tests must pass before mutation verification         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. RE-RUN MUTATION TEST                                     │
│    cargo mutants --baseline skip --file <file> --jobs 2    │
│    Verify kill rate improved                                │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. ITERATE                                                  │
│    Repeat until ≥90% kill rate achieved                     │
└─────────────────────────────────────────────────────────────┘
```

### Phase-Based Approach

**Phase 1: Type Inference** (Complete ✅)
- Target: Match arm deletions in type inference
- Tests: 18 comprehensive tests
- Impact: 18.7% → 25.4% kill rate

**Phase 2: Boolean Logic** (Complete ✅)
- Target: Boolean operator swaps (`&&` ↔ `||`)
- Tests: 12 tests covering all boolean conditions
- Impact: 25.4% → 35% kill rate

**Phase 3: Comparison Operators** (Planned)
- Target: Comparison swaps (`>`, `==`, `!=`, `>=`, `<`)
- Estimated: 15-20 tests
- Expected: 35% → 46% kill rate

**Phase 4: Return Values** (Planned)
- Target: Return value replacements
- Estimated: 10-15 tests
- Expected: 46% → 54% kill rate

**Phase 5: Remaining** (Planned)
- Target: Match arms, negation, operator conversions
- Estimated: 40-60 tests
- Expected: 54% → 90%+ kill rate

---

## Configuration

### `.cargo/mutants.toml`

```toml
# Depyler Mutation Testing Configuration
# See: https://mutants.rs/configuration.html

# Test only depyler-core package (primary transpilation logic)
test_package = ["depyler-core"]

# Exclude generated files and test utilities
exclude_globs = [
    "target/**",
    "**/tests/**",
    "**/*_test.rs",
]

# Skip doctests (they fail in tmp directory)
additional_cargo_test_args = ["--lib", "--tests"]

# Timeout per mutant (2 minutes)
timeout = 120

# WORKAROUND: Baseline skip for doctest issues
# When running mutation tests manually, use --baseline skip flag:
#   cargo mutants --baseline skip --file <file> --jobs 2
# This bypasses baseline validation (safe because we validate tests pass separately)
# We verify all tests pass via: cargo test -p depyler-core --lib --tests
```

### GitHub Actions Integration (Future)

```yaml
name: Mutation Testing
on:
  pull_request:
    paths:
      - 'crates/depyler-core/src/**'

jobs:
  mutants:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-mutants
        run: cargo install cargo-mutants --locked

      - name: Run Mutation Tests
        run: |
          cargo mutants --baseline skip \
            --file crates/depyler-core/src/ast_bridge.rs \
            --jobs 2 \
            --json > mutants.json

      - name: Check Kill Rate
        run: |
          KILL_RATE=$(jq '.caught / .total * 100' mutants.json)
          if (( $(echo "$KILL_RATE < 90" | bc -l) )); then
            echo "❌ Kill rate $KILL_RATE% below 90% threshold"
            exit 1
          fi
          echo "✅ Kill rate $KILL_RATE% meets threshold"
```

---

## Troubleshooting

### Issue 1: Baseline Test Fails

**Error**: `FAILED. 1 passed; 25 failed`

**Root Cause**: cargo-mutants runs tests in `/tmp` directory where doctests fail

**Solution**: Use `--baseline skip` flag
```bash
cargo mutants --baseline skip --file <file> --jobs 2
```

**Verification**: Tests pass manually
```bash
cargo test -p depyler-core --lib --tests
# Should show: ok. 596 passed
```

### Issue 2: Disk Space Exhaustion

**Error**: `No space left on device (os error 28)`

**Root Causes**:
1. Target directory filled with old builds
2. `/tmp` filled with mutation artifacts

**Solutions**:
```bash
# Clean target directory
cargo clean

# Clean /tmp
rm -rf /tmp/cargo-mutants-*
rm -rf /tmp/cargo-install-*

# Reduce parallelism
cargo mutants --jobs 2  # Instead of 4 or 8
```

### Issue 3: Slow Mutation Testing

**Problem**: 164 mutations take 15+ minutes

**Optimizations**:
1. **Target Specific Lines**:
   ```bash
   cargo mutants --baseline skip \
       --file <file> \
       --line-range 968-985
   ```

2. **Use Debug Builds** (default, faster):
   ```bash
   # Debug builds are 5x smaller than release
   cargo mutants --baseline skip --file <file>
   ```

3. **Parallel Jobs** (carefully):
   ```bash
   # 2 jobs safe, 4 jobs risky (disk space)
   cargo mutants --baseline skip --file <file> --jobs 2
   ```

### Issue 4: Tests Fail on Unsupported Python

**Error**: `Statement type not yet supported`

**Root Cause**: Test uses Python features not yet transpiled (e.g., `return`, `pass`)

**Solution**: Simplify test code
```python
# Instead of:
def method(self):
    return "value"

# Use:
def method(self):
    x = 1
```

### Issue 5: Configuration Syntax Error

**Error**: `invalid type: boolean, expected a sequence`

**Root Cause**: `test_package = true` expects array

**Solution**:
```toml
# Correct:
test_package = ["depyler-core"]

# Incorrect:
test_package = true
```

---

## Best Practices

### 1. Mutation-Driven Test Design

**Bad Test** (validates "doesn't crash"):
```rust
#[test]
fn test_type_inference() {
    let hir = bridge.python_to_hir(ast).unwrap();
    assert!(hir.is_ok());  // ❌ Only checks no panic
}
```

**Good Test** (validates correctness):
```rust
#[test]
fn test_infer_int_type() {
    let hir = bridge.python_to_hir(ast).unwrap();
    assert_eq!(hir.classes[0].fields[0].field_type, Type::Int);
    // ✅ Validates specific behavior
}
```

### 2. Target Specific Mutations

Each test should kill specific mutations:

```rust
// MUTATION: delete match arm ast::Constant::Int(_)
#[test]
fn test_infer_type_from_int_literal() {
    // If match arm deleted, this will return Unknown instead of Int
    assert_eq!(field.field_type, Type::Int);
}
```

### 3. Test Boolean Logic Thoroughly

For `if A && B`:
```rust
// Test 1: Both true (should execute)
// Test 2: A true, B false (should not execute)
// Test 3: A false, B any (should not execute)
```

This ensures mutation `&&` → `||` fails the test.

### 4. Use Descriptive Test Names

```rust
// Good: Describes what and why
test_field_inference_only_when_empty_and_not_dataclass()

// Bad: Vague
test_fields()
```

### 5. Fast Test Execution

- Keep tests focused (single responsibility)
- Use simple test data
- Avoid expensive operations

**Target**: <0.02s for test suite

---

## Results Interpretation

### Mutation Statuses

| Status | Meaning | Action |
|--------|---------|--------|
| **CAUGHT** | Test failed when mutation applied | ✅ Good - test validates correctness |
| **MISSED** | Test passed with mutation | ❌ Bad - write test to catch this |
| **UNVIABLE** | Mutation doesn't compile | ⚪ Ignore - mutation is invalid |
| **TIMEOUT** | Test exceeded time limit | ⚠️ Investigate - possible infinite loop |

### Kill Rate Calculation

```
Kill Rate = CAUGHT / (CAUGHT + MISSED) * 100%

Example:
- CAUGHT: 25
- MISSED: 109
- UNVIABLE: 30 (excluded)
- Kill Rate: 25 / (25 + 109) = 18.7%
```

### Target Thresholds

| Kill Rate | Grade | Action |
|-----------|-------|--------|
| ≥90% | A+ | Excellent - maintain quality |
| 70-89% | B | Good - improve critical paths |
| 50-69% | C | Adequate - significant gaps |
| <50% | F | Poor - major test quality issues |

**Depyler Targets**:
- **Minimum**: 90% (critical transpilation code)
- **Ideal**: 95%+ (zero tolerance for bugs)

### Interpreting Results

**Example Output**:
```
Found 164 mutants to test
164 mutants tested in 15m: 109 missed, 25 caught, 30 unviable
```

**Analysis**:
- **Total Viable**: 134 mutations (164 - 30 unviable)
- **Caught**: 25 (18.7%)
- **Missed**: 109 (81.3%)
- **Priority**: Write tests for 109 MISSED mutations

---

## Mutation Patterns

### Pattern 1: Boolean Operators

**Mutation**: `&&` → `||` or `||` → `&&`

**Kill Strategy**: Test all combinations
```rust
// For: if A && B
test_both_true_executes()      // true && true = true
test_first_false_skips()       // false && ? = false
test_second_false_skips()      // true && false = false
```

### Pattern 2: Comparison Operators

**Mutation**: `>` → `==`, `!=` → `==`, etc.

**Kill Strategy**: Test boundary conditions
```rust
// For: if x > 5
test_greater_than_executes()   // x = 6 → true
test_equal_not_executes()      // x = 5 → false
test_less_than_not_executes()  // x = 4 → false
```

### Pattern 3: Match Arm Deletion

**Mutation**: Delete `match arm X`

**Kill Strategy**: Test each variant
```rust
// For each match arm, create test with that variant
test_int_variant()    // Kills: delete Int arm
test_float_variant()  // Kills: delete Float arm
test_string_variant() // Kills: delete String arm
```

### Pattern 4: Return Values

**Mutation**: `return X` → `return Default::default()`

**Kill Strategy**: Verify exact return value
```rust
assert_eq!(result, expected_value);
// Not just: assert!(result.is_ok())
```

### Pattern 5: Negation

**Mutation**: Delete `!` operator

**Kill Strategy**: Test positive and negative
```rust
test_true_condition()   // !false → true
test_false_condition()  // !true → false
```

---

## Examples

### Example 1: Type Inference Tests

**Mutations Targeted**: 9 match arm deletions in `infer_type_from_expr`

**Test Strategy**:
```rust
// Mutation: delete match arm ast::Constant::Int(_)
#[test]
fn test_infer_type_from_int_literal() {
    let python = r#"
class Config:
    def __init__(self):
        self.count = 42
"#;
    let hir = bridge.python_to_hir(parse(python)).unwrap();
    assert_eq!(hir.classes[0].fields[0].field_type, Type::Int);
}
```

**Result**: If Int arm deleted, returns Unknown instead → test fails → mutation caught

### Example 2: Boolean Logic Tests

**Mutations Targeted**: 13 boolean operator swaps

**Test Strategy**:
```rust
// Mutation: fields.is_empty() && !is_dataclass → ||
#[test]
fn test_field_inference_guard() {
    // Case 1: Both true (AND: true, OR: true) - both pass
    // Case 2: Dataclass (AND: false, OR: true) - OR fails!
    let python = r#"
@dataclass
class Config:
    def __init__(self):
        self.temp = 0
"#;
    let hir = bridge.python_to_hir(parse(python)).unwrap();
    // Should NOT infer fields because is_dataclass = true
    assert_eq!(hir.classes[0].fields.len(), 0);
}
```

**Result**: If `&&` → `||`, would infer fields incorrectly → test fails → mutation caught

---

## Integration with Development Workflow

### Pre-commit Hook

Add mutation testing for changed files:

```bash
# scripts/pre-commit
if git diff --cached --name-only | grep -q "crates/depyler-core/src/ast_bridge.rs"; then
    echo "Running mutation tests on ast_bridge.rs..."
    cargo mutants --baseline skip \
        --file crates/depyler-core/src/ast_bridge.rs \
        --jobs 2 \
        --json > /tmp/mutants.json

    KILL_RATE=$(jq '.caught / .total * 100' /tmp/mutants.json)
    if (( $(echo "$KILL_RATE < 90" | bc -l) )); then
        echo "❌ BLOCKED: Kill rate $KILL_RATE% below 90%"
        exit 1
    fi
    echo "✅ Kill rate: $KILL_RATE%"
fi
```

### CI/CD Quality Gate

```yaml
# Enforce 90% kill rate on critical files
mutation-test:
  script:
    - cargo mutants --baseline skip --file <critical_file> --json
    - ./scripts/check_kill_rate.sh mutants.json 90
```

---

## Metrics Dashboard

Track mutation testing progress:

| File | Total Mutations | Kill Rate | Status |
|------|----------------|-----------|--------|
| ast_bridge.rs | 164 | 35% → 90% | 🚧 In Progress |
| codegen.rs | TBD | TBD | ⏳ Pending |
| direct_rules.rs | TBD | TBD | ⏳ Pending |
| type_flow.rs | TBD | TBD | ⏳ Pending |

**Overall Target**: ≥90% across all core files

---

## References

- **cargo-mutants**: https://mutants.rs
- **Mutation Testing Guide**: https://mutants.rs/guide.html
- **Depyler Spec**: `docs/specifications/mutant.md`
- **Session Docs**: `MUTATION-TESTING-*.md` files

---

**Maintained By**: Depyler Development Team
**Questions**: Create issue with `mutation-testing` label

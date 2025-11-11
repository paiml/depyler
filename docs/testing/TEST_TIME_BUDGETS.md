# Test Time Budgets

This document defines the strict time budgets for Depyler test targets and the property test optimizations that enforce them.

## Time Requirements (MANDATORY)

| Target | Time Limit | Property Test Iterations | Use Case |
|--------|-----------|-------------------------|----------|
| `make test-pre-commit-fast` | **< 30 seconds** | PROPTEST_CASES=1, QUICKCHECK_TESTS=1 | Ultra-fast pre-commit validation (library tests only) |
| `make test-fast` | **< 5 minutes** | PROPTEST_CASES=5, QUICKCHECK_TESTS=5 | Fast development feedback loop |
| `make coverage` | **< 10 minutes** | PROPTEST_CASES=10, QUICKCHECK_TESTS=10 | Coverage measurement with reduced iterations |
| `make test` | **No limit** | Default (PROPTEST_CASES=256, QUICKCHECK_TESTS=100) | Comprehensive testing with full iterations |

## Current Performance

### test-pre-commit-fast
- **Actual Runtime**: 0.581 seconds ✅
- **Budget**: < 30 seconds
- **Headroom**: 29.419 seconds (98.1% under budget)
- **Property Test Config**: PROPTEST_CASES=1, QUICKCHECK_TESTS=1
- **Scope**: Library tests only (--lib)

### test-fast
- **Actual Runtime**: 8.23 seconds ✅
- **Budget**: < 5 minutes (300 seconds)
- **Headroom**: 291.77 seconds (97.3% under budget)
- **Property Test Config**: PROPTEST_CASES=5, QUICKCHECK_TESTS=5

### coverage
- **Actual Runtime**: 9 minutes 3 seconds (543s) ✅
- **Budget**: < 10 minutes (600 seconds)
- **Headroom**: 57 seconds (9.5% under budget)
- **Property Test Config**: PROPTEST_CASES=10, QUICKCHECK_TESTS=10
- **Exclusions**: Benchmark tests (property_test_benchmarks, integration_benchmarks)
- **Note**: Optimized to skip 300+ second benchmark tests

### test
- **Budget**: No time limit
- **Property Test Config**: DEFAULT (PROPTEST_CASES=256, QUICKCHECK_TESTS=100)
- **Purpose**: Comprehensive testing with full property test coverage

## Property Test Iteration Rationale

### Why reduce iterations?

Property-based testing is excellent for finding edge cases, but running hundreds of iterations can slow down development feedback loops. The reduction strategy:

1. **Pre-commit (1 iteration)**: Smoke test only - catches compilation and obvious errors
2. **Development (5 iterations)**: Catches most common issues quickly
3. **Coverage (10 iterations)**: Balances thoroughness with speed for CI/CD
4. **Comprehensive (256+ iterations)**: Full exploration for pre-release validation

### Trade-offs

- **Single iteration (pre-commit)**: Catches only obvious errors; designed for instant feedback
- **Fast iterations (5x)**: Catches most common issues; acceptable risk during development
- **Moderate iterations (10x)**: Good coverage without excessive runtime
- **Full iterations (256+)**: Comprehensive property validation (pre-release gate)

## Makefile Targets Summary

```makefile
# Ultra-fast pre-commit validation (< 30s)
test-pre-commit-fast:
    PROPTEST_CASES=1 QUICKCHECK_TESTS=1 cargo test --lib --quiet

# Fast development feedback (< 5 min)
test-fast:
    PROPTEST_CASES=5 QUICKCHECK_TESTS=5 cargo test --lib --quiet
    PROPTEST_CASES=5 QUICKCHECK_TESTS=5 cargo test --test property_tests --quiet

# Coverage measurement (< 10 min)
coverage:
    PROPTEST_CASES=10 QUICKCHECK_TESTS=10 cargo llvm-cov nextest --no-fail-fast \
        --skip 'property_test_benchmarks::' --skip 'integration_benchmarks::'

# Comprehensive testing (no limit)
test:
    cargo llvm-cov test --workspace --all-features  # Uses defaults
```

## Enforcement

Time budgets are NOT suggestions - they are **hard requirements**:

- If `test-pre-commit-fast` exceeds 30 seconds → reduce to library tests only or skip property tests
- If `test-fast` exceeds 5 minutes → reduce property test iterations further
- If `coverage` exceeds 10 minutes → reduce iterations or skip expensive test suites
- If `test` takes too long → acceptable, but document expected runtime

## Monitoring

Track test runtimes over time:

```bash
# Time pre-commit tests
time make test-pre-commit-fast

# Time test-fast
time make test-fast

# Time coverage
time make coverage

# Time comprehensive tests
time make test
```

If any target exceeds its budget, immediately:
1. Identify slow test suites (use `cargo test -- --nocapture`)
2. Reduce property test iterations OR
3. Skip expensive integration tests in fast/coverage targets

## History

- **2025-11-09**: All time budgets established and enforced
  - `test-pre-commit-fast`: Created with 0.581s runtime (98.1% under 30s budget)
  - `test-fast`: 49.7s → 8.23s (optimized from PROPTEST_CASES=10 → 5)
  - `coverage`: Already optimized with PROPTEST_CASES=10
  - `test`: Updated to run comprehensive suite with defaults

## Usage in Git Hooks

The `test-pre-commit-fast` target is designed for pre-commit hooks:

```bash
# .git/hooks/pre-commit
#!/bin/bash
make test-pre-commit-fast || {
    echo "❌ Pre-commit tests failed"
    exit 1
}
```

This provides instant feedback (<1s) while catching compilation errors and basic functionality issues.

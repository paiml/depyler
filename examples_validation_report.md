# Depyler Example Validation Report

**Generated**: 2025-10-07 15:31:21 UTC
**Ticket**: DEPYLER-0027
**Sprint**: Sprint 6 - Example Validation & Quality Gates

## Summary

- **Total Examples**: 66
- **Passed**: 0 (0%)
- **Failed**: 66 (100%)
- **Skipped**: 0

## Quality Gates

Each example must pass ALL of the following:

1. ✅ **Clippy**: Zero warnings (`cargo clippy --all-targets -- -D warnings`)
2. ✅ **Tests**: 100% pass rate (`cargo test --all-features`)
3. ✅ **Coverage**: ≥80% (`cargo llvm-cov --summary-only --fail-under-lines 80`)
4. ✅ **TDG Grade**: A- or higher (`pmat tdg <file> --min-grade A-`)
5. ✅ **Complexity**: ≤10 cyclomatic (`pmat analyze complexity <file> --max-cyclomatic 10`)
6. ✅ **SATD**: Zero technical debt (`pmat analyze satd <file> --fail-on-violation`)

## Passed Examples (0)


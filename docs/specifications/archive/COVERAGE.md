# Rust Code Coverage: Engineering Guide

## Executive Summary

LLVM source-based coverage (`-C instrument-coverage`) is the canonical approach for Rust projects. Inline test modules cause coverage attribution failures due to CFG compilation boundaries. Solution: Use integration tests in `tests/` directory for instrumentation coverage, supplemented by unit tests only when testing private APIs.

## Technical Foundation

### Coverage Instrumentation Architecture

```
Source Code → rustc + -C instrument-coverage 
           → LLVM IR with llvm.instrprof.increment intrinsics
           → Binary with embedded coverage map (LLVM COV_MAP format v6)
           → Execution produces .profraw files
           → llvm-profdata merge → .profdata
           → llvm-cov report/show → Human-readable output
```

**Key invariant**: Coverage maps correlate LLVM IR basic blocks to source locations. Conditional compilation (`#[cfg(test)]`) disrupts this mapping.

### The Inline Test Module Pathology

```rust
// ❌ Breaks coverage attribution
pub fn critical_path() -> Result<()> { 
    // Executed during tests, reported as 0% coverage
    Ok(())
}

#[cfg(test)]
mod tests {  // CFG boundary creates separate compilation unit
    #[test]
    fn test_critical_path() {
        super::critical_path().unwrap();  // Coverage counter not attributed
    }
}
```

**Root cause**: 
- `#[cfg(test)]` changes symbol visibility and mangling
- LLVM's profiling runtime allocates counters per translation unit
- Coverage map references disappear at the test/non-test boundary

## Canonical Solution: Integration Test Structure

```
project/
├── src/
│   ├── lib.rs              # Public API only, no #[cfg(test)]
│   ├── module.rs           # Business logic
│   └── internal/           # Private modules
│       └── helper.rs
└── tests/                  # Integration tests (separate crate)
    ├── api_tests.rs        # Test public interface
    ├── edge_cases.rs       # Boundary conditions
    └── integration/        # Multi-module scenarios
        └── mod.rs
```

**Advantages**:
- Clean coverage attribution (separate compilation units)
- Public API validation (can't access internals)
- Faster incremental builds (test changes don't invalidate src/)
- Smaller release binaries (test code excluded)

### When to Keep Inline Tests

Only for testing private APIs where integration tests cannot reach:

```rust
// src/internal/helper.rs
fn private_fn() -> usize { 42 }  // Private, but needs testing

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_private_fn() {
        assert_eq!(private_fn(), 42);
    }
}
```

**Mitigation**: Exclude from coverage reports (see Configuration below).

## Tooling: cargo-llvm-cov

```bash
# Installation
cargo install cargo-llvm-cov --locked

# Add to rustup (for llvm-tools)
rustup component add llvm-tools-preview
```

### Essential Commands

```bash
# Local development with HTML report
cargo llvm-cov --html --open

# CI/CD LCOV generation
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Branch coverage (requires nightly)
cargo +nightly llvm-cov --branch --html

# With nextest (faster test runner)
cargo llvm-cov nextest --lcov --output-path coverage.lcov

# Exclude patterns
cargo llvm-cov --ignore-filename-regex='tests?\.rs' --lcov
```

### Performance Characteristics

| Project Size | Coverage Runtime | Overhead vs `cargo test` |
|--------------|------------------|--------------------------|
| <10K LOC     | 1-2 min          | +30-40%                  |
| 10-50K LOC   | 3-5 min          | +40-50%                  |
| >50K LOC     | 5-10 min         | +50-60%                  |

**Optimization**: Use `--workspace` flag with workspace-level exclusions rather than per-package runs.

## CI/CD Integration

### GitHub Actions (Production Template)

```yaml
name: Coverage

on:
  push:
    branches: [main]
  pull_request:

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview
      
      - name: Install cargo-llvm-cov
        uses: taiki-e/install-action@cargo-llvm-cov
      
      - name: Restore cache
        uses: Swatinem/rust-cache@v2
      
      - name: Generate coverage
        run: |
          cargo llvm-cov --all-features --workspace --lcov \
            --output-path lcov.info \
            --ignore-filename-regex='tests?\.rs'
      
      - name: Coverage threshold check
        run: |
          COVERAGE=$(cargo llvm-cov --all-features --workspace --summary-only \
            | grep -oP '\d+\.\d+(?=%)' | head -1)
          echo "Coverage: $COVERAGE%"
          if (( $(echo "$COVERAGE < 80.0" | bc -l) )); then
            echo "❌ Coverage $COVERAGE% below 80% threshold"
            exit 1
          fi
          echo "✓ Coverage $COVERAGE% meets threshold"
      
      - name: Upload to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          fail_ci_if_error: true
          token: ${{ secrets.CODECOV_TOKEN }}
```

### GitLab CI

```yaml
coverage:
  image: rust:latest
  variables:
    LLVM_PROFILE_FILE: "target/coverage/%p-%m.profraw"
  before_script:
    - rustup component add llvm-tools-preview
    - cargo install cargo-llvm-cov --locked
  script:
    - cargo llvm-cov --workspace --lcov --output-path lcov.info
  coverage: '/\d+\.\d+%\s+\d+\.\d+%\s+(\d+\.\d+)%/'
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: coverage.xml
```

## Configuration

### Project-Level Settings

```toml
# .cargo/config.toml
[alias]
cov = "llvm-cov --html --open"
cov-ci = "llvm-cov --all-features --workspace --lcov --output-path lcov.info"

# Cargo.toml (optional dev-dependency for version consistency)
[dev-dependencies]
cargo-llvm-cov = "0.6"
```

### Exclusion Patterns

```rust
// Top of file: Exclude entire file (use sparingly)
#![cfg_attr(coverage_nightly, coverage(off))]

// Inline exclusion markers (for tools like grcov)
pub fn infallible_operation() {
    self.conn.prepare("SELECT 1").unwrap() // grcov-excl-line
}

// Region exclusions
// grcov-excl-start
#[derive(Debug, Clone, Copy)]  // Compiler-generated code
pub struct Config {
    pub field: String,
}
// grcov-excl-stop
```

### IDE Integration (VS Code)

```json
// .vscode/settings.json
{
  "coverage-gutters.coverageFileNames": [
    "lcov.info",
    "target/llvm-cov/lcov.info"
  ],
  "coverage-gutters.showLineCoverage": true,
  "coverage-gutters.showRulerCoverage": true,
  "coverage-gutters.showGutterCoverage": true
}
```

**Required extension**: Coverage Gutters (ryanluker.vscode-coverage-gutters)

**Workflow**:
```bash
cargo llvm-cov --lcov --output-path lcov.info
# In VS Code: Cmd/Ctrl+Shift+P → "Coverage Gutters: Display Coverage"
```

## Quality Thresholds

### Industry Benchmarks

| Domain                  | Minimum Coverage | Target Coverage |
|-------------------------|------------------|-----------------|
| System utilities        | 70%              | 85%             |
| Web services/APIs       | 75%              | 90%             |
| Safety-critical systems | 85%              | 95%+            |
| Libraries (public API)  | 80%              | 90%             |

### Practical Thresholds

```rust
// Example: Quality gate implementation
pub struct CoverageGate {
    line_threshold: f64,      // 80.0%
    branch_threshold: f64,    // 70.0% (harder to achieve)
    function_threshold: f64,  // 85.0%
}

impl QualityCheck for CoverageGate {
    fn check(&self) -> Result<()> {
        let report = run_coverage_analysis()?;
        
        ensure!(
            report.line_coverage >= self.line_threshold,
            "Line coverage {:.1}% < {:.1}%",
            report.line_coverage, self.line_threshold
        );
        
        ensure!(
            report.branch_coverage >= self.branch_threshold,
            "Branch coverage {:.1}% < {:.1}%", 
            report.branch_coverage, self.branch_threshold
        );
        
        Ok(())
    }
}
```

**Recommendation**: Start at 70%, ratchet up by 5% per quarter until reaching 85%.

## Troubleshooting

### Issue: 0% Coverage Despite Passing Tests

**Diagnosis**: Inline test modules with `#[cfg(test)]`

**Solution**: Migrate to integration tests
```bash
# Automated migration
find src -name "*.rs" -exec grep -l '#\[cfg(test)\]' {} \; | while read file; do
    testfile="tests/$(basename "$file" .rs)_tests.rs"
    awk '/^#\[cfg\(test\)\]/,/^}$/' "$file" > "$testfile"
    sed -i '/^#\[cfg(test)\]/,/^}$/d' "$file"
done
```

### Issue: Coverage Decreases on Refactoring

**Cause**: Dead code elimination or inlining optimizations

**Fix**: Build with debug assertions
```bash
cargo llvm-cov --profile test-coverage

# Cargo.toml
[profile.test-coverage]
inherits = "test"
opt-level = 0
codegen-units = 1
```

### Issue: Inconsistent Coverage Between Runs

**Cause**: Non-deterministic test execution (threading, timing)

**Fix**: Use deterministic test framework
```bash
cargo llvm-cov nextest --test-threads=1
```

### Issue: Coverage Tools Not Found

**Diagnosis**: llvm-tools not installed or version mismatch

**Solution**:
```bash
# Check LLVM version
rustc --version --verbose | grep LLVM

# Install matching llvm-tools
rustup component add llvm-tools-preview

# Verify installation
llvm-profdata --version
llvm-cov --version

# Manual path configuration if needed
export LLVM_COV=$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov
export LLVM_PROFDATA=$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata
```

## Advanced Patterns

### Differential Coverage

Track coverage changes in PRs:

```bash
# Generate baseline
git checkout main
cargo llvm-cov --lcov --output-path baseline.lcov

# Generate PR coverage
git checkout feature-branch
cargo llvm-cov --lcov --output-path feature.lcov

# Compare (requires lcov tools)
genhtml --diff-file baseline.lcov feature.lcov --output-directory diff-cov/
```

### Multi-Target Coverage

For cross-compilation scenarios:

```bash
# Coverage for multiple targets
for target in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu; do
    cargo llvm-cov --target $target --lcov --output-path "coverage-${target}.lcov"
done

# Merge reports
lcov --add-tracefile coverage-x86_64-unknown-linux-gnu.lcov \
     --add-tracefile coverage-aarch64-unknown-linux-gnu.lcov \
     --output-file merged.lcov
```

### Workspace-Level Coverage

```bash
# Root Cargo.toml
[workspace]
members = ["crate-a", "crate-b", "crate-c"]

# Generate workspace coverage
cargo llvm-cov --workspace --lcov --output-path workspace.lcov

# Per-crate exclusions
cargo llvm-cov --workspace \
    --exclude crate-a \
    --exclude-from-report crate-b \
    --lcov
```

## Documentation Coverage

Track doc-test coverage separately:

```bash
# Include doctests (requires nightly)
cargo +nightly llvm-cov --doctests --html

# Example doctest
/// ```
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

**Note**: Doc-tests are integration tests and contribute to coverage naturally.

## Performance Profiling with Coverage

Combine coverage with profiling:

```bash
# Generate coverage
cargo llvm-cov --lcov --output-path lcov.info

# Identify untested hot paths
# 1. Profile with perf/flamegraph
# 2. Overlay coverage data
# 3. Prioritize tests for high-frequency, low-coverage code
```

## Migration Checklist

- [ ] Install `cargo-llvm-cov` and `llvm-tools-preview`
- [ ] Create `tests/` directory for integration tests
- [ ] Migrate high-value tests from inline modules
- [ ] Add coverage job to CI/CD pipeline
- [ ] Set initial threshold at 70%
- [ ] Configure IDE coverage visualization
- [ ] Document coverage exclusion policy
- [ ] Add pre-commit hook to prevent new inline test modules
- [ ] Schedule quarterly threshold increases
- [ ] Integrate with code review process (block PRs decreasing coverage)

## References

- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
- [rustc instrumentation docs](https://doc.rust-lang.org/rustc/instrument-coverage.html)
- [cargo-llvm-cov repository](https://github.com/taiki-e/cargo-llvm-cov)
- [Codecov Rust guide](https://about.codecov.io/language/rust/)

---

**Version**: 1.0  
**Last Updated**: 2025-10-02  
**Applicability**: Rust 1.70+ (stable), LLVM 13+

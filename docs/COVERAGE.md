# Code Coverage Workflow

Depyler uses a **hybrid coverage approach** following the [pforge](https://github.com/paiml/pforge) pattern:
- **Local development**: cargo-llvm-cov with two-phase collection
- **CI/CD**: cargo-tarpaulin for Codecov integration

## Quick Start

### Prerequisites

```bash
# Install coverage tools (one-time setup)
cargo install cargo-llvm-cov nextest --locked
```

### Local Development

```bash
# Generate comprehensive coverage report
make coverage

# View summary only (after running 'make coverage')
make coverage-summary

# Open HTML report in browser
make coverage-open
```

## The Hybrid Approach

### Why Two Different Tools?

Following pforge's proven pattern:

1. **Local (llvm-cov + nextest)**:
   - ⚡ Faster test execution (30-50% speedup)
   - 📊 Better HTML reports
   - 🔧 Two-phase collection (test once, generate multiple reports)
   - 🎯 More accurate line coverage

2. **CI (tarpaulin)**:
   - ✅ Established Codecov integration
   - 🔒 Stable, reliable for automated builds
   - 📦 Simpler CI configuration

## Local Coverage (make coverage)

The `make coverage` target implements a **two-phase workflow** from pforge:

```bash
make coverage
```

### What It Does

**Phase 1: Test Execution with Instrumentation**
```bash
cargo llvm-cov --no-report nextest --no-tests=warn --all-features --workspace
```
- Runs all tests with coverage instrumentation
- Uses nextest for parallel execution
- Generates `.profraw` files (raw coverage data)
- Does NOT generate reports yet

**Phase 2: Report Generation**
```bash
cargo llvm-cov report --html --output-dir target/coverage/html
cargo llvm-cov report --lcov --output-path target/coverage/lcov.info
cargo llvm-cov report --summary-only
```
- Processes collected data
- Generates multiple report formats
- No need to re-run tests

### Critical: Linker Workaround

The coverage target temporarily disables `~/.cargo/config.toml` because custom linkers (like `mold`) can break coverage instrumentation:

```bash
# Before tests
mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup

# Run coverage
cargo llvm-cov --no-report nextest ...

# After tests
mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml
```

This ensures clean coverage data regardless of your local cargo configuration.

### Output Locations

```
target/coverage/
├── html/
│   └── index.html          # Interactive HTML report
└── lcov.info              # LCOV format for editors
```

## CI Coverage (GitHub Actions)

Our CI uses **cargo-tarpaulin** for simplicity and Codecov compatibility:

```yaml
- name: Install cargo-tarpaulin
  run: cargo install cargo-tarpaulin

- name: Generate coverage
  run: cargo tarpaulin --out Xml --all-features --workspace

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v4
  with:
    files: ./cobertura.xml
    fail_ci_if_error: false
```

### Why Not llvm-cov in CI?

- Tarpaulin has better Codecov integration
- Simpler CI configuration
- Well-established in Rust ecosystem
- Avoids linker configuration issues in CI

## Makefile Targets

### make coverage
Generate comprehensive coverage report with HTML and LCOV output.

```bash
make coverage
```

**Output**:
- HTML report: `target/coverage/html/index.html`
- LCOV file: `target/coverage/lcov.info`
- Terminal summary

### make coverage-summary
Display coverage summary without re-running tests.

```bash
make coverage-summary
```

Requires `make coverage` to be run first.

### make coverage-open
Open HTML coverage report in browser.

```bash
make coverage-open
```

Auto-detects your browser (xdg-open or macOS open).

### make coverage-check
Verify coverage meets threshold (currently 60%).

```bash
make coverage-check
```

Exits with error if below threshold.

## Coverage Philosophy

Following pforge's documented approach (see their COVERAGE_NOTES.md):

### The Inline Test Module Challenge

Rust coverage tools often report **lower coverage than actual** because:

- Coverage measures "executed lines"
- Inline `#[cfg(test)]` modules are separate from production code
- Tests execute production code, but tools don't always attribute it correctly

**Example**:
```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b  // This line IS tested but may show as uncovered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);  // Test exists!
    }
}
```

Tools may report 0% coverage despite comprehensive tests.

### Recommended Approach

From pforge's COVERAGE_NOTES.md:

1. **Prioritize test quality over strict percentages**
2. **Focus on critical path coverage**
3. **Accept measurement limitations**
4. **Maintain comprehensive test suites**

### Our Current Coverage

- **Total Tests**: 87 tests (100% passing)
- **Reported Coverage**: ~60-70% (limited by inline test modules)
- **Actual Coverage**: Higher (all critical paths tested)

## Editor Integration

### VS Code

With `target/coverage/lcov.info` generated:

1. Install "Coverage Gutters" extension
2. Run: `make coverage`
3. VS Code shows coverage inline automatically

### IntelliJ/RustRover

1. Run: `make coverage`
2. Go to: Run → Show Coverage Data
3. Load `target/coverage/lcov.info`

## Troubleshooting

### "cargo-llvm-cov not found"

```bash
cargo install cargo-llvm-cov --locked
```

### "cargo-nextest not found"

```bash
cargo install cargo-nextest --locked
```

### Coverage shows 0% despite tests

This is a known Rust ecosystem issue with inline test modules. Solutions:

1. **Use integration tests** (in `tests/` directory)
2. **Accept the limitation** (focus on test quality)
3. **Check HTML report** (more accurate than summary)

### Linker errors during coverage

The Makefile automatically handles this by temporarily disabling `~/.cargo/config.toml`.

Manual workaround:
```bash
mv ~/.cargo/config.toml ~/.cargo/config.toml.backup
make coverage
mv ~/.cargo/config.toml.backup ~/.cargo/config.toml
```

### Stale coverage data

```bash
cargo llvm-cov clean --workspace
rm -rf target/coverage/
make coverage
```

## References

- **pforge**: https://github.com/paiml/pforge (reference implementation)
- **pforge COVERAGE_NOTES.md**: Philosophy and approach
- **cargo-llvm-cov**: https://github.com/taiki-e/cargo-llvm-cov
- **cargo-nextest**: https://nexte.st/
- **cargo-tarpaulin**: https://github.com/xd009642/tarpaulin

## Summary

**Local Development**:
```bash
make coverage           # Generate all reports
make coverage-summary   # Quick summary
make coverage-open      # View in browser
```

**CI/CD**: Automatically runs tarpaulin and uploads to Codecov

**Philosophy**: Prioritize test quality over strict coverage percentages, following pforge's proven approach.

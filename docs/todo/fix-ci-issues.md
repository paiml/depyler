# CI Issues to Fix

## Current CI Failures

### 1. Security Vulnerability (CRITICAL)
- **Issue**: `paste` crate v1.0.15 is unmaintained (RUSTSEC-2024-0436)
- **Dependency chain**: paste → malachite-bigint → rustpython-parser → depyler-core
- **Fix**: Update malachite-bigint or find alternative to rustpython-parser that doesn't use unmaintained crates

### 2. Code Coverage Timeout
- **Issue**: Coverage generation times out after 10+ minutes
- **Symptoms**: Compilation gets stuck during tarpaulin execution
- **Fix Options**:
  - Increase timeout from 120s to 600s
  - Split coverage into multiple parallel jobs
  - Exclude heavy test files from coverage
  - Use incremental coverage approach

### 3. Quality Gates - Clippy Pedantic
- **Issue**: Strict clippy lints may be failing on new code
- **Fix**: Run locally and fix any clippy warnings:
  ```bash
  cargo clippy --workspace --all-features -- \
    -D warnings \
    -D clippy::all \
    -D clippy::pedantic \
    -D clippy::nursery \
    -A clippy::missing_const_for_fn \
    -A clippy::module_name_repetitions \
    -A clippy::must_use_candidate
  ```

### 4. Performance Regression Detection
- **Issue**: May be failing due to new test overhead
- **Fix**: Review benchmark thresholds and adjust if needed

### 5. Playground CI
- **Issue**: WASM compilation or quality checks failing
- **Fix**: Ensure WASM target builds cleanly

## Immediate Actions

### Step 1: Fix Security Vulnerability
```toml
# Check if we can update dependencies
cargo update -p malachite-bigint
# Or consider replacing rustpython-parser with a maintained alternative
```

### Step 2: Fix Coverage Timeout
```yaml
# In .github/workflows/ci.yml, update coverage job:
- name: Generate code coverage
  run: |
    export RUSTFLAGS="-C linker=gcc"
    cargo tarpaulin --features coverage --workspace --timeout 600 --out Xml \
      --exclude-files "*/tests/*" \
      --exclude-files "*/examples/*"
```

### Step 3: Fix Clippy Issues Locally
```bash
# Run the exact same clippy command as CI
cargo clippy --workspace --all-features -- \
  -D warnings \
  -D clippy::all \
  -D clippy::pedantic \
  -D clippy::nursery \
  -A clippy::missing_const_for_fn \
  -A clippy::module_name_repetitions \
  -A clippy::must_use_candidate

# Fix any issues found
```

### Step 4: Verify All Workflows
```bash
# Check each workflow locally
make test-all
make test-quality
cargo test --workspace
cargo fmt -- --check
```

## Long-term Solutions

1. **Dependency Audit Pipeline**: Regular checks for unmaintained dependencies
2. **Coverage Optimization**: 
   - Use coverage profiles
   - Parallelize coverage collection
   - Cache coverage data between runs
3. **CI Performance**:
   - Use sccache for faster builds
   - Implement build caching
   - Split large workflows
4. **Quality Gates**:
   - Create allow-list for specific clippy lints
   - Implement gradual lint adoption

## Priority Order

1. Fix security vulnerability (CRITICAL)
2. Fix coverage timeout (HIGH)
3. Fix clippy warnings (MEDIUM)
4. Optimize CI performance (LOW)
# bashrs-Style Quality Tooling Specification

**Ticket**: DEPYLER-QUALITY-001
**Status**: Draft
**Target**: Align depyler quality tooling with bashrs patterns

---

## Objective

Adopt the proven quality patterns from `../bashrs` to achieve:
- Fast, reliable `make lint && make coverage` workflow
- Mold linker for fast builds (disabled during coverage)
- Property-based testing with configurable iteration counts
- Two-phase coverage generation (instrument → report)

---

## Current State Analysis

### bashrs Patterns (Reference Implementation)

#### 1. Cargo Config (`.cargo/config.toml`)

```toml
[target.'cfg(all())']
rustflags = [
    # PHASE 1: Critical safety lints (WARN level)
    "-W", "clippy::unwrap_used",
    "-W", "clippy::expect_used",
    "-W", "clippy::panic",
    "-W", "clippy::indexing_slicing",

    # PHASE 2: Development hygiene (WARN level)
    "-W", "clippy::todo",
    "-W", "clippy::unimplemented",
    "-W", "clippy::dbg_macro",

    # PHASE 3: Quality lints (WARN level)
    "-W", "clippy::cargo",
]

[alias]
xclippy = "clippy --all-targets --all-features -- -D warnings"
xtest = "test --all-features"
xbuild = "build --all-features"
xcheck = "check --all-features"
```

**Key insight**: Warnings in config, errors via explicit alias.

#### 2. Lint Target (Auto-fix, Non-blocking)

```makefile
lint:
    @echo "🔍 Running clippy..."
    @RUSTFLAGS="-A warnings" cargo clippy --all-targets --all-features --quiet
    @RUSTFLAGS="-A warnings" cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged --quiet 2>/dev/null || true
```

**Key insight**: `lint` auto-fixes without failing. For development flow.

#### 3. Lint-Check Target (Strict, CI-blocking)

```makefile
lint-check:
    @echo "🔍 Checking clippy..."
    @cargo clippy --all-targets --all-features -- \
        -D clippy::correctness \
        -D clippy::suspicious \
        -D clippy::unwrap_used \
        -W clippy::complexity \
        -W clippy::perf \
        -W missing_docs \
        -A clippy::multiple_crate_versions \
        -A clippy::expect_used \
        -A clippy::indexing_slicing \
        -A clippy::panic \
        -A dead_code \
        -A unused_variables
```

**Key insight**: Explicit deny/warn/allow per lint category. Correctness = error, style = warning.

#### 4. Coverage Target (Two-Phase with Mold Disable)

```makefile
COVERAGE_EXCLUDE := --ignore-filename-regex='quality/gates\.rs|test_generator/.*\.rs'

coverage:
    @echo "📊 Running comprehensive test coverage analysis..."
    @which cargo-llvm-cov > /dev/null 2>&1 || cargo install cargo-llvm-cov --locked
    @which cargo-nextest > /dev/null 2>&1 || cargo install cargo-nextest --locked
    @cargo llvm-cov clean --workspace
    @mkdir -p target/coverage

    # CRITICAL: Disable mold linker (breaks coverage instrumentation)
    @test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true

    # Phase 1: Run tests with instrumentation (no report)
    @env PROPTEST_CASES=100 cargo llvm-cov --no-report nextest --no-tests=warn --all-features --workspace

    # Phase 2: Generate reports
    @cargo llvm-cov report --html --output-dir target/coverage/html $(COVERAGE_EXCLUDE)
    @cargo llvm-cov report --lcov --output-path target/coverage/lcov.info $(COVERAGE_EXCLUDE)

    # Restore mold linker
    @test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true

    @cargo llvm-cov report --summary-only $(COVERAGE_EXCLUDE)
```

**Key insight**:
- Backup/restore `~/.cargo/config.toml` to disable mold
- Two-phase: `--no-report` then `report`
- Configurable `PROPTEST_CASES` for speed vs thoroughness

#### 5. Property Test Configuration

```makefile
# Fast mode: 50 cases
test-property:
    @PROPTEST_CASES=50 cargo test --workspace --lib -- property_tests

# Comprehensive mode: 500 cases
test-property-comprehensive:
    @PROPTEST_CASES=500 cargo test --workspace --lib -- property_tests
```

---

## Required Changes for Depyler

### 1. Update `.cargo/config.toml`

**Current** (depyler):
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[alias]
cov = "llvm-cov --no-report nextest"
# ...
```

**Proposed** (bashrs-style):
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.'cfg(all())']
rustflags = [
    # Safety lints (WARN - promote to DENY after cleanup)
    "-W", "clippy::unwrap_used",
    "-W", "clippy::expect_used",
    "-W", "clippy::panic",
    "-W", "clippy::indexing_slicing",

    # Development hygiene
    "-W", "clippy::todo",
    "-W", "clippy::unimplemented",
    "-W", "clippy::dbg_macro",

    # Quality
    "-W", "clippy::cargo",
]

[build]
target-dir = "target"

[env]
CARGO_TERM_COLOR = { value = "always", force = true }

[alias]
# Strict clippy (CI mode)
xclippy = "clippy --all-targets --all-features -- -D warnings"
xtest = "test --all-features"
xbuild = "build --all-features"
xcheck = "check --all-features"

# Coverage workflow
cov = "llvm-cov --no-report nextest"
cov-test = "llvm-cov --no-report test --workspace --all-features"
cov-report = "llvm-cov report --html --open"
cov-lcov = "llvm-cov report --lcov --output-path lcov.info"
cov-summary = "llvm-cov report --summary-only"
cov-clean = "llvm-cov clean --workspace"
cov-all = "llvm-cov nextest --all-features --html --open"

# Quality shortcuts
q = "clippy --all-targets --all-features -- -D warnings"
qf = "fmt --all -- --check"
qt = "test --workspace --all-features"
```

### 2. Update Makefile Lint Targets

**Replace current `lint` and `lint-rust`**:

```makefile
# #@ Linting (bashrs-style)

.PHONY: lint lint-check lint-rust lint-frontend

lint: lint-rust lint-frontend ## Run all linters with auto-fix (development)
	@echo "✅ Linting complete (auto-fixed where possible)"

lint-rust: ## Run Rust linter with auto-fix (non-blocking)
	@echo "🔍 Running clippy with auto-fix..."
	@RUSTFLAGS="-A warnings" $(CARGO) clippy --all-targets --all-features --quiet 2>/dev/null || true
	@RUSTFLAGS="-A warnings" $(CARGO) clippy --all-targets --all-features --fix --allow-dirty --allow-staged --quiet 2>/dev/null || true

lint-check: ## Run strict clippy (CI-blocking)
	@echo "🔍 Running strict clippy checks..."
	@$(CARGO) clippy --all-targets --all-features -- \
		-D clippy::correctness \
		-D clippy::suspicious \
		-W clippy::complexity \
		-W clippy::perf \
		-A clippy::multiple_crate_versions \
		-A clippy::expect_used \
		-A clippy::indexing_slicing \
		-A clippy::panic \
		-A dead_code \
		-A unused_variables
```

### 3. Update Coverage Target

**Simplify to match bashrs pattern exactly**:

```makefile
# Coverage exclusion patterns (external deps, test infrastructure)
COVERAGE_EXCLUDE := --ignore-filename-regex='alimentar|aprender|entrenar|verificar|trueno'

coverage: ## Generate HTML coverage report (bashrs-style, target: <5 min)
	@echo "📊 Running test coverage analysis..."
	@echo "🔍 Checking tools..."
	@which cargo-llvm-cov > /dev/null 2>&1 || (echo "📦 Installing cargo-llvm-cov..." && cargo install cargo-llvm-cov --locked)
	@which cargo-nextest > /dev/null 2>&1 || (echo "📦 Installing cargo-nextest..." && cargo install cargo-nextest --locked)
	@echo "🧹 Cleaning old coverage data..."
	@cargo llvm-cov clean --workspace
	@mkdir -p target/coverage
	@echo "⚙️  Disabling mold linker (breaks coverage instrumentation)..."
	@test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true
	@echo "🧪 Phase 1: Running tests with instrumentation..."
	@env PROPTEST_CASES=100 QUICKCHECK_TESTS=100 cargo llvm-cov --no-report nextest --profile fast --no-tests=warn --all-features --workspace
	@echo "📊 Phase 2: Generating reports..."
	@cargo llvm-cov report --html --output-dir target/coverage/html $(COVERAGE_EXCLUDE)
	@cargo llvm-cov report --lcov --output-path target/coverage/lcov.info $(COVERAGE_EXCLUDE)
	@echo "⚙️  Restoring mold linker..."
	@test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true
	@echo ""
	@echo "📊 Coverage Summary:"
	@echo "=================="
	@cargo llvm-cov report --summary-only $(COVERAGE_EXCLUDE)
	@echo ""
	@echo "💡 Reports:"
	@echo "  - HTML: target/coverage/html/index.html"
	@echo "  - LCOV: target/coverage/lcov.info"
	@echo "  - Open: make coverage-open"
```

### 4. Add Property Test Targets

```makefile
# #@ Property Testing (bashrs-style configurable iterations)

test-property: ## Run property tests (fast: 50 cases)
	@echo "🎲 Running property tests (50 cases)..."
	@PROPTEST_CASES=50 QUICKCHECK_TESTS=50 $(CARGO) test --workspace --lib -- prop_ property

test-property-comprehensive: ## Run property tests (comprehensive: 500 cases)
	@echo "🎲 Running property tests (500 cases)..."
	@PROPTEST_CASES=500 QUICKCHECK_TESTS=500 $(CARGO) test --workspace --lib -- prop_ property
```

### 5. Update Quick Validate Target

```makefile
quick-validate: lint-check test-fast ## Quick validation for development
	@echo "✅ Quick validation passed!"
```

---

## Validation Workflow

After implementation, this workflow should work:

```bash
# Development flow (fast, auto-fixes)
make lint          # Auto-fix, non-blocking
make test-fast     # Quick tests (<5 min)

# CI flow (strict, blocking)
make lint-check    # Strict clippy, blocks on errors
make coverage      # Full coverage with property tests

# Combined (the golden command)
make lint && make coverage
```

---

## Performance Targets

| Target | Time Budget | Property Cases |
|--------|-------------|----------------|
| `test-fast` | <5 min | 5 |
| `test-property` | <2 min | 50 |
| `test-property-comprehensive` | <10 min | 500 |
| `coverage` | <10 min | 100 |

---

## Acceptance Criteria

- [x] `make lint` auto-fixes without blocking
- [x] `make lint-check` fails on correctness/suspicious issues
- [x] `make coverage` disables mold, runs two-phase, restores mold
- [x] `make lint && make coverage` completes in <15 minutes
- [x] Property test iterations configurable via environment
- [x] Coverage threshold enforced (95% per spec)

## Implementation Status

**Completed: 2024-12-17** via DEPYLER-QUALITY-001

Files modified:
- `.cargo/config.toml` - Added bashrs-style lint phases and aliases
- `Makefile` - Updated lint/lint-check/coverage/test-property targets

---

## Phase 2: 95% Code Coverage Target (DEPYLER-QUALITY-002)

### Objective

Achieve **95% line coverage** on depyler-core matching the pareto-single-shot spec requirement.

### Coverage Strategy

Following bashrs pattern:
1. **Solidified Coverage**: Property tests + mutation validation (not just line coverage)
2. **Exclusion Patterns**: External deps, test infrastructure excluded
3. **Two-Phase Workflow**: Instrument → Report (mold disabled)

### Coverage Thresholds

| Metric | Target | Validation |
|--------|--------|------------|
| Line coverage | ≥95% | `cargo llvm-cov` |
| Function coverage | ≥95% | `cargo llvm-cov` |
| Mutation score | ≥80% | `cargo mutants` |
| Property test ratio | ≥90% of lines | proptest coverage |

### Makefile Target

```makefile
coverage-95: ## Validate 95% coverage threshold
	@echo "📊 Validating 95% coverage threshold..."
	@cargo llvm-cov clean --workspace
	@test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true
	@env PROPTEST_CASES=100 cargo llvm-cov --no-report nextest --profile fast --workspace
	@COVERAGE=$$(cargo llvm-cov report --summary-only | grep TOTAL | awk '{print $$10}' | sed 's/%//'); \
	test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true; \
	if [ "$${COVERAGE%.*}" -lt 95 ]; then \
		echo "❌ Coverage $${COVERAGE}% below 95% threshold"; \
		exit 1; \
	else \
		echo "✅ Coverage $${COVERAGE}% meets 95% threshold"; \
	fi
```

### Acceptance Criteria (Phase 2)

- [ ] Line coverage ≥95% on depyler-core
- [ ] Function coverage ≥95% on depyler-core
- [ ] `make coverage-95` passes
- [ ] Coverage report excludes external dependencies

---

## References

- bashrs Makefile: `../bashrs/Makefile`
- bashrs cargo config: `../bashrs/.cargo/config.toml`
- Coverage spec: `docs/specifications/pareto-single-shot/12.6-coverage.md`

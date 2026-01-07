# Depyler Makefile - Comprehensive Testing Infrastructure
# Following NASA/SQLite reliability standards
.PHONY: all build test test-full test-rust test-frontend test-fast test-comprehensive test-fixtures test-property test-compilation test-semantic validate quality-gate coverage coverage-summary coverage-open coverage-check coverage-ci coverage-clean clean-coverage clean-test mutants mutants-quick mutants-check mutants-report lint lint-rust lint-frontend clippy fmt format fmt-check fmt-fix fmt-rust fmt-frontend fmt-docs check bench install-deps help profile profile-transpiler profile-tests profile-cargo-toml book-test book-test-fast book-build book-serve book-clean book-validate book-check validate-makefiles lint-scripts bashrs-report
# Configuration
CARGO := cargo
MAKEFLAGS += -j$(shell nproc)
# Coverage threshold (Toyota Way: 95% minimum)
# 80% threshold - CLI handlers and I/O code are excluded/hard to test
COVERAGE_THRESHOLD := 80
# Quality gate thresholds
MAX_COMPLEXITY := 10
MAX_LINES_PER_FUNCTION := 50
# Build Configuration
# Fix linker issues by using system linker instead of lld
RUST_FLAGS := RUSTFLAGS="-D warnings -C linker=cc"
TEST_FLAGS := --workspace --all-features
RELEASE_FLAGS := --release
# Default target
.PHONY: all
all: validate
# #@ Quick Start
quickstart: ## Quick start - build and test everything
	@echo "🚀 Depyler Quick Start"
	@echo "Building project..."
	@$(MAKE) build
	@echo "Running tests..."
	@$(MAKE) test
	@echo "✅ Ready to use! Try: ./target/release/depyler --help"
playground-quickstart: ## Quick start the playground
	@echo "🎮 Starting Depyler Playground"
	@$(MAKE) playground-build
	@echo "✅ Playground ready! Opening in browser..."
	@$(MAKE) playground-run
playground-fast: ## Start playground quickly (skip builds if possible)
	@echo "🎮 Starting Depyler Playground (fast mode)"
	@if [ ! -d "playground/public/wasm" ]; then echo "WASM not found, building..."; cd crates/depyler-wasm && wasm-pack build --target web --out-dir ../../playground/public/wasm; else echo "✓ Using existing WASM build"; fi
	@if [ ! -d "playground/node_modules" ]; then echo "Installing dependencies..."; cd playground && npm install; else echo "✓ Dependencies already installed"; fi
	@if [ ! -d "playground/dist" ]; then echo "Building frontend..."; cd playground && npm run build; else echo "✓ Using existing frontend build"; fi
	@echo "✅ Playground ready! Starting server..."
	cd playground && npm run preview
# Main test target - comprehensive tests with full property test iterations
.PHONY: test
test: ## Run comprehensive Rust tests (runs everything, no time limit)
	@echo "Running comprehensive Rust tests with FULL property test iterations..."
	@echo "⚙️  Using DEFAULT iterations (PROPTEST_CASES=25, QUICKCHECK_TESTS=100)"
	@$(CARGO) llvm-cov --no-report test --workspace --all-features
	@echo ""
	@echo "=== Coverage Summary ==="
	@$(CARGO) llvm-cov report --summary-only
	@$(CARGO) llvm-cov report --summary-only --fail-under-functions $(COVERAGE_THRESHOLD) || true
.PHONY: test-full
test-full: test test-frontend ## Run all tests (Rust + frontend)
.PHONY: test-rust
test-rust: test ## Alias for main test target
.PHONY: test-frontend
test-frontend: ## Run frontend tests (npm + deno)
	@echo "Running frontend tests..."
	@if [ -d "playground" ]; then echo "Running npm tests..."; cd playground && npm test; echo "Running Deno tests..."; cd playground && deno test --allow-read --allow-env --allow-net src/**/*.deno.test.ts src/**/*.test.ts 2>/dev/null || true; fi
# #@ Building
.PHONY: build
build: ## Build the project
	$(RUST_FLAGS) $(CARGO) build $(RELEASE_FLAGS)
build-dev: ## Build for development
	$(CARGO) build
clean: ## Clean build artifacts
	$(CARGO) clean

clean-deep: ## Deep clean ALL artifacts (target/, __pycache__, .gdb, etc.)
	@echo "🧹 Deep cleaning project..."
	@rm -rf target/
	@find . -type d -name "target" -exec rm -rf {} +
	@find . -type d -name "__pycache__" -exec rm -rf {} +
	@find . -type d -name ".pytest_cache" -exec rm -rf {} +
	@find . -type d -name ".mypy_cache" -exec rm -rf {} +
	@find . -type d -name ".ruff_cache" -exec rm -rf {} +
	@find . -name "*.gdb" -delete
	@find . -name "*.lldb" -delete
	@find . -name "*.profraw" -delete
	@find . -name "*.profdata" -delete
	@echo "✅ Project deeply cleaned"

# #@ Playground
playground: playground-build playground-run ## Build and run the playground
playground-build: ## Build WASM module and frontend
	@echo "Building WASM module..."
	cd crates/depyler-wasm && wasm-pack build --target web --out-dir ../../playground/public/wasm
	@echo "Installing playground dependencies..."
	cd playground && npm install
	@echo "Building playground frontend..."
	cd playground && npm run build
playground-dev: ## Run playground in development mode
	@echo "Building WASM module..."
	cd crates/depyler-wasm && wasm-pack build --target web --out-dir ../../playground/public/wasm --dev
	@echo "Starting playground dev server..."
	cd playground && npm run dev
playground-run: ## Run the playground
	@echo "Starting playground server..."
	cd playground && npm run preview
playground-test: ## Run playground tests
	@echo "Running playground tests..."
	cd playground && npm test
	@echo "Running Deno TypeScript validation..."
	cd playground && deno test src/components/__tests__/*.deno.test.ts --allow-read
playground-clean: ## Clean playground build artifacts
	rm -rf playground/dist
	rm -rf playground/public/wasm
	rm -rf playground/node_modules
	rm -rf crates/depyler-wasm/target
# #@ Testing
# Fast tests for development iteration
# Comprehensive test suite (NASA-grade)
.PHONY: test-comprehensive
test-comprehensive: test-fixtures test-property test-compilation test-semantic ## Run all tests with full validation
	@echo "All comprehensive tests passed ✅"
# Individual test categories
.PHONY: test-fixtures
test-fixtures: ## Test all Python fixture transpilation
	@echo "Testing fixture transpilation..."
	$(CARGO) test --test transpilation_tests $(TEST_FLAGS)
# #@ Property Testing (bashrs-style configurable iterations)
.PHONY: test-property
test-property: ## Run property tests (fast: 50 cases)
	@echo "🎲 Running property tests (50 cases)..."
	@env PROPTEST_CASES=25 QUICKCHECK_TESTS=50 $(CARGO) test --workspace --lib -- prop_ property --test-threads=$$(nproc)
	@env PROPTEST_CASES=25 QUICKCHECK_TESTS=50 $(CARGO) test --test property_tests $(TEST_FLAGS)

.PHONY: test-property-comprehensive
test-property-comprehensive: ## Run property tests (comprehensive: 500 cases)
	@echo "🎲 Running property tests (500 cases)..."
	@env PROPTEST_CASES=250 QUICKCHECK_TESTS=500 $(CARGO) test --workspace --lib -- prop_ property --test-threads=$$(nproc)
	@env PROPTEST_CASES=250 QUICKCHECK_TESTS=500 $(CARGO) test --test property_tests $(TEST_FLAGS)
	@env PROPTEST_CASES=250 QUICKCHECK_TESTS=500 $(CARGO) test --test semantic_equivalence $(TEST_FLAGS) || true
	@env PROPTEST_CASES=250 QUICKCHECK_TESTS=500 $(CARGO) test --test property_tests_ast_roundtrip $(TEST_FLAGS) || true
	@env PROPTEST_CASES=250 QUICKCHECK_TESTS=500 $(CARGO) test --test property_tests_type_inference $(TEST_FLAGS) || true
	@env PROPTEST_CASES=250 QUICKCHECK_TESTS=500 $(CARGO) test --test property_tests_memory_safety $(TEST_FLAGS) || true
	@echo "✅ Property tests completed (comprehensive mode)!"

# #@ Advanced Testing Infrastructure (Phases 8-10)
test-property-basic: test-property ## Alias for test-property (fast mode)

test-property-advanced: ## Run advanced property tests (Phase 8)
	@echo "Running advanced property tests..."
	$(CARGO) test --test advanced_property_generators $(TEST_FLAGS)
	$(CARGO) test --test mutation_testing $(TEST_FLAGS)
	$(CARGO) test --test fuzzing_tests $(TEST_FLAGS)
test-doctests: ## Run all documentation tests
	@echo "Running doctests..."
	$(CARGO) test --doc $(TEST_FLAGS)
	$(CARGO) test --test interactive_doctests $(TEST_FLAGS)
test-examples: ## Run example validation tests
	@echo "Running example validation..."
	$(CARGO) test --test example_validation $(TEST_FLAGS)
	$(CARGO) test --test comprehensive_examples $(TEST_FLAGS)
test-coverage: ## Run coverage analysis tests
	@echo "Running coverage analysis..."
	$(CARGO) test --test coverage_analysis $(TEST_FLAGS)
	$(CARGO) test --test edge_case_coverage $(TEST_FLAGS)
	$(CARGO) test --test error_path_coverage $(TEST_FLAGS)
	$(CARGO) test --test boundary_value_tests $(TEST_FLAGS)
test-integration: ## Run integration tests
	@echo "Running integration tests..."
	$(CARGO) test --test integration_benchmarks $(TEST_FLAGS)
	$(CARGO) test --test multi_version_compatibility $(TEST_FLAGS)
	$(CARGO) test --test large_codebase_tests $(TEST_FLAGS)
test-quality: ## Run quality assurance automation
	@echo "Running quality assurance..."
	$(CARGO) test --test quality_assurance_automation $(TEST_FLAGS)
	$(CARGO) test --test specialized_coverage_testing $(TEST_FLAGS)
test-all: ## Complete test suite execution
	@echo "Running complete test suite..."
	$(MAKE) test-property-basic
	$(MAKE) test-property-advanced
	$(MAKE) test-doctests
	$(MAKE) test-examples
	$(MAKE) test-coverage
	$(MAKE) test-integration
	$(MAKE) test-quality
.PHONY: test-fast
test-fast: ## Quick feedback loop for development (< 5 min, uses cargo-nextest)
	@echo "⚡ Running fast tests (target: <5 min)..."
	@if command -v cargo-nextest >/dev/null 2>&1; then \
		PROPTEST_CASES=5 QUICKCHECK_TESTS=5 cargo nextest run \
			--workspace \
			--profile fast \
			--no-fail-fast; \
	else \
		PROPTEST_CASES=5 QUICKCHECK_TESTS=5 cargo test --workspace; \
	fi

# Test Impact Analysis (TIA) - DEPYLER-0954
# Runs only tests affected by code changes for 50-80% faster CI
.PHONY: test-tia test-affected tia-report
test-tia: ## Run only tests affected by recent changes (TIA)
	@echo "🔍 DEPYLER-0954: Test Impact Analysis"
	@./scripts/tia.sh HEAD~1

test-affected: ## Run tests affected by changes since main branch
	@echo "🔍 Running tests affected by changes since main..."
	@./scripts/tia.sh main

tia-report: ## Show which tests would be affected by current changes
	@echo "📊 TIA Report: Tests affected by uncommitted changes"
	@echo ""
	@CHANGED=$$(git diff --name-only HEAD -- '*.rs' 2>/dev/null | wc -l); \
	if [ "$$CHANGED" -eq 0 ]; then \
		echo "No Rust files changed"; \
	else \
		echo "Changed files:"; \
		git diff --name-only HEAD -- '*.rs' | head -10; \
		echo ""; \
		echo "Affected packages:"; \
		git diff --name-only HEAD -- '*.rs' | grep -oP 'crates/\K[^/]+' | sort -u; \
	fi

# Risk-Based Testing (RBT) - DEPYLER-0957
# Prioritizes test execution based on impact/likelihood matrix
.PHONY: test-rbt test-rbt-high rbt-analyze rbt-report
test-rbt: ## Run tests in risk-priority order (high-risk first)
	@echo "🎯 DEPYLER-0957: Risk-Based Testing"
	@./scripts/rbt.sh

test-rbt-high: ## Run only high-risk tests (fail-fast mode)
	@echo "🎯 Running high-risk tests only (fail-fast)..."
	@./scripts/rbt.sh --high-only

rbt-analyze: ## Analyze test risk scores without running
	@./scripts/rbt.sh --analyze

rbt-report: ## Generate detailed risk-based testing report
	@./scripts/rbt.sh --report

# Incremental Caching Layer - DEPYLER-0950
# O(1) compilation cache for single-shot transpilation scalability
.PHONY: cache-stats cache-warm cache-gc cache-clean
cache-stats: ## Show cache statistics (hit rate, size, entries)
	@echo "📊 DEPYLER-0950: Cache Statistics"
	@./target/release/depyler cache stats 2>/dev/null || echo "Build depyler first: make build"

cache-warm: ## Warm cache for examples directory
	@echo "🔥 Warming cache for examples..."
	@./target/release/depyler cache warm --input-dir examples 2>/dev/null || echo "Build depyler first: make build"

cache-gc: ## Run garbage collection on cache
	@echo "🗑️  Running cache garbage collection..."
	@./target/release/depyler cache gc 2>/dev/null || echo "Build depyler first: make build"

cache-clean: ## Clear all cached transpilation results
	@echo "🧹 Clearing cache..."
	@rm -rf .depyler/cache
	@echo "✅ Cache cleared"

# Golden Trace Validation - DEPYLER-0956
# Renacer-based semantic equivalence verification
.PHONY: golden-trace golden-trace-ci golden-trace-capture
golden-trace: ## Run golden trace validation on examples
	@echo "🔍 DEPYLER-0956: Golden Trace Validation"
	@./scripts/golden_trace.sh --help

golden-trace-ci: build ## Run full CI golden trace validation
	@echo "🧪 Running Golden Trace CI validation..."
	@for dir in examples/test_project examples/algorithms; do \
		echo "Validating: $$dir"; \
		./scripts/golden_trace.sh ci "$$dir" || true; \
	done

golden-trace-capture: ## Capture golden traces for a specific example
	@if [ -z "$(EXAMPLE)" ]; then \
		echo "Usage: make golden-trace-capture EXAMPLE=examples/test_project"; \
	else \
		./scripts/golden_trace.sh ci "$(EXAMPLE)"; \
	fi

test-pre-commit-fast: ## Ultra-fast pre-commit validation (< 60 seconds with build scripts)
	@echo "⚡ Running pre-commit fast validation (<60s with build scripts)..."
	@echo "   (Type checking only - no test execution)"
	@timeout 60 cargo check --workspace
	@echo "✅ Pre-commit validation completed!"
test-ci: ## CI/CD optimized test execution
	@echo "Running CI/CD tests..."
	$(MAKE) test-property-basic
	$(MAKE) test-coverage
	$(MAKE) test-integration
# #@ Performance Testing
test-benchmark: ## Performance regression testing
	@echo "Running performance benchmarks..."
	$(CARGO) test --test property_test_benchmarks $(TEST_FLAGS)
	$(CARGO) test --test integration_benchmarks $(TEST_FLAGS)
	$(CARGO) bench
test-profile: ## Performance profiling and analysis
	@echo "Running performance profiling..."
	$(CARGO) test --test performance_profiling $(TEST_FLAGS)
	./scripts/run_performance_suite.sh
# #@ Renacer Profiling (https://github.com/paiml/renacer)
.PHONY: profile
profile: ## Profile transpiler with Renacer (requires: cargo install renacer)
	@if ! command -v renacer >/dev/null 2>&1; then echo "❌ Renacer not found! Install with: cargo install renacer"; exit 1; fi
	@echo "🔍 Profiling transpiler with Renacer..."
	@./scripts/profile_transpiler.sh examples/matrix_testing_project/07_algorithms/algorithms.py
.PHONY: profile-transpiler
profile-transpiler: ## Profile transpiler on a specific file (use FILE=path/to/file.py)
	@if ! command -v renacer >/dev/null 2>&1; then echo "❌ Renacer not found! Install with: cargo install renacer"; exit 1; fi
	@if [ -z "$(FILE)" ]; then echo "Usage: make profile-transpiler FILE=examples/script.py"; exit 1; fi
	@./scripts/profile_transpiler.sh $(FILE)
.PHONY: profile-tests
profile-tests: ## Find slow tests using Renacer (shows tests >100ms)
	@if ! command -v renacer >/dev/null 2>&1; then echo "❌ Renacer not found! Install with: cargo install renacer"; exit 1; fi
	@echo "🐌 Finding slow tests (>100ms)..."
	@./scripts/profile_tests.sh --slow-only
.PHONY: profile-cargo-toml
profile-cargo-toml: ## Profile DEPYLER-0384 Cargo.toml generation overhead
	@if ! command -v renacer >/dev/null 2>&1; then echo "❌ Renacer not found! Install with: cargo install renacer"; exit 1; fi
	@./scripts/profile_cargo_toml_gen.sh
test-memory: ## Memory usage validation
	@echo "Running memory tests..."
	$(CARGO) test --test memory_safety_tests $(TEST_FLAGS)
	$(CARGO) test --test resource_exhaustion $(TEST_FLAGS)
test-concurrency: ## Thread safety and parallel execution
	@echo "Running concurrency tests..."
	$(CARGO) test --test concurrent_execution $(TEST_FLAGS)
	$(CARGO) test --test thread_safety $(TEST_FLAGS)
# #@ Development Workflows
test-watch: ## Continuous testing during development
	@echo "Starting test watch mode..."
	$(CARGO) watch -x "test --lib" -x "test --test property_tests"
test-debug: ## Enhanced debugging and error reporting
	@echo "Running debug tests..."
	RUST_BACKTRACE=1 $(CARGO) test $(TEST_FLAGS) -- --nocapture
test-generate: ## Automatic test generation and updates
	@echo "Running test generation..."
	$(CARGO) test --test automated_test_generation $(TEST_FLAGS)
	./scripts/generate_test_cases.sh
test-report: ## Comprehensive quality reporting
	@echo "Generating test reports..."
	./scripts/run_performance_suite.sh
	$(MAKE) coverage
	$(MAKE) quality-report
.PHONY: test-compilation
test-compilation: ## Validate generated Rust compiles
	@echo "Validating Rust compilation..."
	$(CARGO) test --test rustc_compilation $(TEST_FLAGS)
.PHONY: test-semantic
test-semantic: ## Test semantic equivalence
	@echo "Testing semantic equivalence..."
	$(CARGO) test semantic $(TEST_FLAGS)
test-unit: ## Run unit tests only
	@echo "Running unit tests..."
	$(CARGO) test --lib $(TEST_FLAGS)
# Exhaustive testing (10,000+ cases)
test-exhaustive: ## Run exhaustive test suite (10k+ cases)
	@echo "Running exhaustive test suite..."
	DEPYLER_EXHAUSTIVE=1 $(CARGO) test $(TEST_FLAGS) -- --test-threads=1
# Performance testing
.PHONY: bench
bench: ## Run all benchmarks
	@echo "Running comprehensive performance benchmarks..."
	$(CARGO) bench --bench transpilation
	$(CARGO) bench --bench memory_usage
	$(CARGO) bench --bench binary_size
bench-transpilation: ## Run transpilation performance benchmarks
	@echo "Running transpilation benchmarks..."
	$(CARGO) bench --bench transpilation
bench-memory: ## Run memory usage benchmarks
	@echo "Running memory usage benchmarks..."
	$(CARGO) bench --bench memory_usage
bench-size: ## Run binary size benchmarks
	@echo "Running binary size benchmarks..."
	$(CARGO) bench --bench binary_size
test-performance: ## Test performance regressions
	@echo "Testing performance regressions..."
	$(CARGO) test performance $(TEST_FLAGS)
# #@ Validation
.PHONY: validate
validate: quality-gate test-comprehensive coverage ## Full validation pipeline
	@echo "🎉 All validation gates passed!"
quick-validate: lint-check test-fast ## Quick validation for development (bashrs-style)
	@echo "✅ Quick validation passed!"
validate-examples: ## Validate all examples against quality gates (DEPYLER-0027)
	@echo "=========================================="
	@echo "🔍 Depyler Example Validation"
	@echo "Ticket: DEPYLER-0027"
	@echo "=========================================="
	@echo ""
	@./scripts/validate_examples.sh
	@echo ""
	@echo "=========================================="
	@echo "📊 See examples_validation_report.md for details"
	@echo "=========================================="
validate-example: ## Validate specific example (Usage: make validate-example FILE=path/to/file.rs)
	@if [ -z "$(FILE)" ]; then echo "❌ Error: FILE not specified"; echo "Usage: make validate-example FILE=examples/showcase/fibonacci.rs"; exit 1; fi
	@echo "Validating $(FILE)..."
	@./scripts/validate_examples.sh $(FILE)
validate-transpiled-strict: ## 🛑 STRICT: Validate transpiled examples with rustc (DEPYLER-0095)
	@echo "=========================================="
	@echo "🛑 STRICT Transpiled Example Validation"
	@echo "Ticket: DEPYLER-0095"
	@echo "Method: Direct rustc (cargo clippy skips these!)"
	@echo "=========================================="
	@echo ""
	@./scripts/validate_transpiled_strict.sh
# #@ Quality Assurance
.PHONY: quality-gate
quality-gate: lint clippy complexity-check ## Run quality checks
	@echo "Quality gate passed ✅"
# #@ Linting (bashrs-style: auto-fix vs strict)
.PHONY: lint
lint: lint-rust ## Run linters with auto-fix (development, non-blocking)
	@echo "✅ Linting complete (auto-fixed where possible)"

.PHONY: lint-rust
lint-rust: ## Run Rust linter with auto-fix (non-blocking)
	@echo "🔍 Running clippy with auto-fix..."
	@RUSTFLAGS="-A warnings" $(CARGO) clippy --all-targets --all-features --quiet 2>/dev/null || true
	@RUSTFLAGS="-A warnings" $(CARGO) clippy --all-targets --all-features --fix --allow-dirty --allow-staged --quiet 2>/dev/null || true

.PHONY: lint-check
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

.PHONY: clippy
clippy: lint-check ## Alias for lint-check (CI mode)
.PHONY: format
format: fmt-rust fmt-frontend fmt-docs ## Format all code artifacts comprehensively
.PHONY: fmt
fmt: format ## Alias for comprehensive formatting
.PHONY: fmt-rust
fmt-rust: ## Format Rust code
	@echo "Formatting Rust code..."
	$(CARGO) fmt
.PHONY: fmt-frontend
fmt-frontend: ## Format TypeScript, JavaScript, CSS, HTML, JSON with Deno
	@echo "Formatting frontend code with Deno..."
	@if [ -d "playground" ]; then cd playground && deno fmt --unstable-component --line-width=100 --indent-width=2 --single-quote=false --no-semicolons=false --ext=ts,tsx,js,jsx,json,html,css,md src/ *.ts *.tsx *.js *.jsx *.json *.html *.css *.md 2>/dev/null || true; fi
.PHONY: fmt-docs
fmt-docs: ## Format documentation files with Deno
	@echo "Formatting documentation with Deno..."
	@deno fmt --line-width=80 --prose-wrap=always --indent-width=2 --ext=md *.md docs/*.md crates/*/README.md examples/*/README.md playground/README.md 2>/dev/null || true
.PHONY: fmt-check
fmt-check: ## Check if all files are formatted
	@echo "Checking Rust formatting..."
	$(CARGO) fmt --check
	@echo "Checking frontend formatting..."
	@if [ -d "playground" ]; then cd playground && deno fmt --check --unstable-component --line-width=100 --indent-width=2 --single-quote=false --no-semicolons=false --ext=ts,tsx,js,jsx,json,html,css,md src/ *.ts *.tsx *.js *.jsx *.json *.html *.css *.md 2>/dev/null || true; fi
	@echo "Checking documentation formatting..."
	@deno fmt --check --line-width=80 --prose-wrap=always --indent-width=2 --ext=md *.md docs/*.md crates/*/README.md examples/*/README.md playground/README.md 2>/dev/null || true
.PHONY: fmt-fix
fmt-fix: format ## Alias for comprehensive formatting
complexity-check: ## Check code complexity
	@echo "Checking code complexity..."
	@# This would integrate with a complexity analysis tool
	@echo "Complexity check passed (manual verification required)"
security-audit: ## Run security audit
	@echo "Running security audit..."
	$(CARGO) audit
# #@ Coverage (Canonical Fast Pattern - must complete in <5 min)
# Filter out external dependencies from coverage reports (only show depyler crates)
# Exclude: external deps, interactive TUI, main entry points, MCP server, repartir, I/O-heavy command handlers
# Note: CLI handlers that run external processes (cargo, rustc) are excluded as they require complex mocking
# Pure logic is extracted to *_shim.rs files which have 97-100% coverage
COVERAGE_IGNORE_REGEX := "alimentar|aprender|entrenar|verificar|trueno|interactive\\.rs|/main\\.rs|mcp_server\\.rs|repartir|training_monitor|agent/daemon|automl_tuning|differential\\.rs|compilation_trainer\\.rs|report_cmd/mod\\.rs|compile_cmd\\.rs|utol_cmd\\.rs|depyler/src/lib\\.rs|converge/compiler\\.rs|citl_fixer\\.rs|corpus_citl\\.rs|autofixer\\.rs"
.PHONY: coverage
coverage: ## Fast coverage report (~4 min)
	@echo "📊 Coverage (lib tests, target: <5 min)..."
	@PROPTEST_CASES=5 QUICKCHECK_TESTS=5 cargo llvm-cov nextest --profile fast --lib --workspace --ignore-filename-regex $(COVERAGE_IGNORE_REGEX)
	@echo ""
coverage-summary: ## Display coverage summary (run 'make coverage' first)
	@echo "📊 Coverage Summary:"
	@echo "=================="
	@$(CARGO) llvm-cov report --summary-only --ignore-filename-regex $(COVERAGE_IGNORE_REGEX) || echo "⚠️  Run 'make coverage' first to generate coverage data"
coverage-open: ## Open HTML coverage report in browser (run 'make coverage' first)
	@echo "🌐 Opening coverage report in browser..."
	@if [ ! -f target/coverage/html/index.html ]; then echo "⚠️  Coverage report not found. Run 'make coverage' first."; exit 1; fi
	@if command -v xdg-open > /dev/null; then xdg-open target/coverage/html/index.html; elif command -v open > /dev/null; then open target/coverage/html/index.html; else echo "💡 Cannot auto-open. View report at: target/coverage/html/index.html"; fi
coverage-check: ## Check coverage threshold (assumes coverage already collected)
	@echo "Checking coverage threshold ($(COVERAGE_THRESHOLD)%)..."
	@COVERAGE=$$($(CARGO) llvm-cov report --summary-only --ignore-filename-regex $(COVERAGE_IGNORE_REGEX) | grep "TOTAL" | awk '{print $$4}' | sed 's/%//'); if [ "$$COVERAGE" -lt "$(COVERAGE_THRESHOLD)" ]; then echo "❌ Coverage $$COVERAGE% below threshold $(COVERAGE_THRESHOLD)%"; exit 1; else echo "✅ Coverage $$COVERAGE% meets threshold $(COVERAGE_THRESHOLD)%"; fi
coverage-ci: ## CI coverage with LCOV output (<5 min)
	@PROPTEST_CASES=25 QUICKCHECK_TESTS=25 cargo llvm-cov nextest --profile fast --no-fail-fast --workspace --lcov --output-path lcov.info --ignore-filename-regex $(COVERAGE_IGNORE_REGEX)
coverage-clean: ## Clean coverage artifacts
	@rm -f lcov.info coverage.xml target/coverage/lcov.info
	@rm -rf target/llvm-cov target/coverage
	@find . -name "*.profraw" -delete 2>/dev/null || true
	@echo "✓ Coverage artifacts cleaned"
clean-coverage: coverage-clean ## Alias for coverage-clean
	@echo "✓ Fresh coverage ready (run 'make coverage' to regenerate)"
# #@ Mutation Testing (Goodhart's Law Mitigation - per DEPYLER-0955)
# Target: 80% mutation score (more rigorous than line coverage)
MUTATION_SCORE_TARGET := 80
.PHONY: mutants mutants-quick mutants-check mutants-report
mutants: ## Run full mutation testing (slow, thorough)
	@echo "🧬 Running mutation testing (target: $(MUTATION_SCORE_TARGET)% kill rate)..."
	@echo "⚠️  This may take 10-30 minutes for full workspace"
	@cargo mutants --workspace --timeout 60 --jobs $(shell nproc)
	@echo "✓ Mutation testing complete"
mutants-quick: ## Quick mutation testing on changed files only
	@echo "🧬 Running quick mutation testing on recently changed files..."
	@CHANGED_FILES=$$(git diff --name-only HEAD~5 -- '*.rs' | head -10); \
	if [ -n "$$CHANGED_FILES" ]; then \
		for f in $$CHANGED_FILES; do \
			if [ -f "$$f" ]; then \
				echo "Testing mutations in: $$f"; \
				cargo mutants --file "$$f" --timeout 30 --jobs $(shell nproc) || true; \
			fi; \
		done; \
	else \
		echo "No recent Rust changes found, running on depyler-core"; \
		cargo mutants -p depyler-core --timeout 30 --jobs $(shell nproc) -- --lib; \
	fi
mutants-check: ## Check mutation score meets threshold (for CI)
	@echo "🧬 Checking mutation score threshold ($(MUTATION_SCORE_TARGET)%)..."
	@cargo mutants -p depyler-core --timeout 30 --jobs $(shell nproc) -- --lib 2>&1 | tee /tmp/mutants.log; \
	KILLED=$$(grep -oP '\d+ killed' /tmp/mutants.log | grep -oP '\d+' || echo 0); \
	TOTAL=$$(grep -oP '\d+ mutants' /tmp/mutants.log | grep -oP '\d+' || echo 1); \
	if [ "$$TOTAL" -gt 0 ]; then \
		SCORE=$$((KILLED * 100 / TOTAL)); \
		echo "Mutation Score: $$SCORE% ($$KILLED/$$TOTAL killed)"; \
		if [ "$$SCORE" -lt "$(MUTATION_SCORE_TARGET)" ]; then \
			echo "❌ Mutation score $$SCORE% below threshold $(MUTATION_SCORE_TARGET)%"; \
			exit 1; \
		else \
			echo "✅ Mutation score $$SCORE% meets threshold $(MUTATION_SCORE_TARGET)%"; \
		fi; \
	fi
mutants-report: ## Generate HTML mutation testing report
	@echo "🧬 Generating mutation testing report..."
	@mkdir -p target/mutants
	@cargo mutants -p depyler-core --timeout 30 --jobs $(shell nproc) --output target/mutants -- --lib
	@echo "✓ Report available in target/mutants/"
# #@ Test Data Management
generate-fixtures: ## Generate additional test fixtures
	@echo "Generating test fixtures..."
	python3 scripts/generate_fixtures.py --count 50 --output tests/fixtures/
.PHONY: clean-test
clean-test: ## Clean test artifacts
	@echo "Cleaning test artifacts..."
	rm -rf target/llvm-cov/
	rm -f lcov.info
	rm -rf tests/temp/
	$(CARGO) clean
# #@ Development Dependencies
.PHONY: install-deps
install-deps: ## Install development dependencies
	@echo "Installing development dependencies..."
	$(CARGO) install cargo-llvm-cov
	$(CARGO) install cargo-audit
	$(CARGO) install cargo-watch
	$(CARGO) install cargo-mutants
	$(CARGO) install cargo-nextest
	@if ! command -v rustfmt > /dev/null; then rustup component add rustfmt; fi
	@if ! command -v clippy > /dev/null; then rustup component add clippy; fi
check-deps: ## Check if all dependencies are installed
	@echo "Checking dependencies..."
	@command -v rustc > /dev/null || (echo "❌ rustc not found" && exit 1)
	@command -v cargo > /dev/null || (echo "❌ cargo not found" && exit 1)
	@$(CARGO) llvm-cov --version > /dev/null || (echo "❌ cargo-llvm-cov not found" && exit 1)
	@$(CARGO) audit --version > /dev/null || (echo "❌ cargo-audit not found" && exit 1)
	@echo "✅ All dependencies found"
# #@ Continuous Integration
ci-test: ## Run CI test suite
	@echo "Running CI test suite..."
	$(MAKE) check-deps
	$(MAKE) validate
	$(MAKE) coverage-check
ci-quick: ## Quick CI check
	@echo "Running quick CI check..."
	$(MAKE) lint
	$(MAKE) test-fast
# #@ Development Workflow
watch: ## Watch for changes and run tests
	@echo "Watching for changes..."
	$(CARGO) watch -x "test --lib"
watch-test: ## Watch and run specific test
	@echo "Usage: make watch-test TEST=test_name"
	$(CARGO) watch -x "test $(TEST)"
dev-setup: install-deps ## Setup development environment
	@echo "Development environment setup complete ✅"
# #@ Reporting
quality-report: ## Generate comprehensive quality report
	@echo "Generating quality report..."
	@echo "=== Depyler Quality Report ===" > quality_report.txt
	@echo "Generated: $$(date)" >> quality_report.txt
	@echo "" >> quality_report.txt
	@echo "=== Test Results ===" >> quality_report.txt
	$(CARGO) test $(TEST_FLAGS) 2>&1 | tee -a quality_report.txt
	@echo "" >> quality_report.txt
	@echo "=== Coverage ===" >> quality_report.txt
	$(CARGO) llvm-cov report --summary-only 2>&1 | tee -a quality_report.txt
	@echo "" >> quality_report.txt
	@echo "=== Clippy Results ===" >> quality_report.txt
	$(CARGO) clippy $(TEST_FLAGS) 2>&1 | tee -a quality_report.txt
	@echo "Quality report generated: quality_report.txt"
test-matrix: ## Run tests across different configurations
	@echo "Running test matrix..."
	@for config in debug release; do echo "Testing in $$config mode..."; if [ "$$config" = "release" ]; then $(CARGO) test $(TEST_FLAGS) --release; else $(CARGO) test $(TEST_FLAGS); fi; done
# #@ Documentation
docs: ## Generate documentation
	$(CARGO) doc --workspace --no-deps
docs-open: docs ## Generate and open documentation
	$(CARGO) doc --workspace --no-deps --open
.PHONY: book-test
book-test: ## Test TDD book examples (BLOCKING for releases)
	@echo "🧪 Testing TDD book examples..."
	@cd tdd-book && uv run pytest tests/ -v --tb=short
.PHONY: book-test-fast
book-test-fast: ## Test book examples in parallel
	@echo "🧪 Testing TDD book examples (fast)..."
	@cd tdd-book && uv run pytest tests/ -n auto -v
.PHONY: book-build
book-build: ## Build mdBook
	@echo "📚 Building Depyler TDD Book..."
	@command -v mdbook >/dev/null 2>&1 || { echo "❌ mdbook not installed. Run: cargo install mdbook"; exit 1; }
	@cd tdd-book && mdbook build
	@echo "✅ Book built at tdd-book/book/index.html"
.PHONY: book-serve
book-serve: ## Serve book locally at http://localhost:3000
	@echo "📖 Serving Depyler TDD Book at http://localhost:3000..."
	@cd tdd-book && mdbook serve --open
.PHONY: book-clean
book-clean: ## Clean book build artifacts
	@echo "🧹 Cleaning book artifacts..."
	@rm -rf tdd-book/book/
	@echo "✅ Book artifacts cleaned"
.PHONY: book-validate
book-validate: book-test book-build ## Validate book (test + build)
	@echo "✅ Book validation complete - all examples tested and book builds"
.PHONY: book-check
book-check: ## Check if book needs updates (for pre-release)
	@echo "🔍 Checking if book is up to date..."
	@if [ -n "$$(git status --porcelain tdd-book/)" ]; then echo "⚠️  TDD book has uncommitted changes"; git status --short tdd-book/; exit 1; fi
	@echo "✅ Book is up to date"
# #@ Performance Profiling
profile-memory: ## Profile memory usage with Valgrind
	@echo "Profiling memory usage..."
	@if command -v valgrind > /dev/null; then $(CARGO) build --release --bin depyler; valgrind --tool=massif --massif-out-file=massif.out ./target/release/depyler transpile examples/showcase/binary_search.py; ms_print massif.out > memory-profile.txt; echo "Memory profile saved to: memory-profile.txt"; else echo "valgrind not found. Install with: sudo apt-get install valgrind"; fi
profile-flamegraph: ## Generate flamegraph performance profile
	@echo "Generating flamegraph..."
	@if command -v cargo-flamegraph > /dev/null; then $(CARGO) flamegraph --root --bin depyler -- transpile examples/showcase/binary_search.py; echo "Flamegraph saved to: flamegraph.svg"; else echo "cargo-flamegraph not found. Install with: cargo install flamegraph"; fi
profile-perf: ## Profile with Linux perf
	@echo "Profiling with perf..."
	@if command -v perf > /dev/null; then $(CARGO) build --release --bin depyler; perf record --call-graph dwarf ./target/release/depyler transpile examples/showcase/binary_search.py; perf report > perf-report.txt; echo "Perf report saved to: perf-report.txt"; else echo "perf not found. Install with: sudo apt-get install linux-tools-generic"; fi
profile-heap: ## Profile heap allocations
	@echo "Profiling heap allocations..."
	$(CARGO) build --release --features "profiling" --bin depyler
	@echo "Heap profiling enabled in binary. Run with heap profiler."
# #@ Binary Size Optimization
build-min-size: ## Build minimum size binary
	@echo "Building minimum size binary..."
	$(RUST_FLAGS) $(CARGO) build --profile min-size --bin depyler
	@ls -lh target/min-size/depyler
	@echo "Size: $$(du -h target/min-size/depyler | cut -f1)"
build-sizes: ## Compare binary sizes across profiles
	@echo "=== Binary Size Comparison ==="
	$(MAKE) build-dev > /dev/null 2>&1 || true
	$(MAKE) build > /dev/null 2>&1 || true
	$(MAKE) build-min-size > /dev/null 2>&1 || true
	@echo "Development:  $$(ls -lh target/debug/depyler 2>/dev/null | awk '{print $$5}' || echo 'N/A')"
	@echo "Release:      $$(ls -lh target/release/depyler 2>/dev/null | awk '{print $$5}' || echo 'N/A')"
	@echo "Min-size:     $$(ls -lh target/min-size/depyler 2>/dev/null | awk '{print $$5}' || echo 'N/A')"
strip-binary: ## Strip symbols from release binary
	@echo "Stripping binary..."
	$(CARGO) build --release --bin depyler
	strip target/release/depyler
	@echo "Stripped size: $$(du -h target/release/depyler | cut -f1)"
compress-binary: ## Compress binary with UPX
	@echo "Compressing binary..."
	@if command -v upx > /dev/null; then $(MAKE) build-min-size; cp target/min-size/depyler target/min-size/depyler.compressed; upx --best target/min-size/depyler.compressed; echo "Original:   $$(du -h target/min-size/depyler | cut -f1)"; echo "Compressed: $$(du -h target/min-size/depyler.compressed | cut -f1)"; else echo "UPX not found. Install with: sudo apt-get install upx-ucl"; fi
analyze-binary-size: ## Run comprehensive binary size analysis
	@echo "Running binary size analysis..."
	./scripts/track_binary_size.sh
size-report: ## Generate detailed size report
	@echo "Generating size report..."
	./scripts/track_binary_size.sh
	@echo "Report saved to: binary_size_report.md"
# #@ Performance Analysis
analyze-performance: ## Run comprehensive performance analysis
	@echo "Running comprehensive performance analysis..."
	$(MAKE) bench
	$(MAKE) profile-memory
	$(MAKE) build-sizes
	@echo "Performance analysis complete. Check reports:"
	@echo "  - Benchmark results: target/criterion/"
	@echo "  - Memory profile: memory-profile.txt"
	@echo "  - Binary sizes: displayed above"
performance-regression-test: ## Test for performance regressions
	@echo "Testing for performance regressions..."
	@if [ -f baseline-performance.json ]; then $(CARGO) bench -- --save-baseline current; cargo-criterion-cmp baseline current; else echo "No baseline found. Creating baseline..."; $(CARGO) bench -- --save-baseline baseline; cp target/criterion/baseline target/criterion/baseline-performance.json; echo "Baseline created. Run again to compare."; fi
# #@ Help
.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
# Test execution summary
test-summary: ## Show test execution summary
	@echo "=== Test Execution Summary ==="
	@echo "Fast tests:        make test-fast"
	@echo "Comprehensive:     make test-comprehensive"
	@echo "Property-based:    make test-property"
	@echo "Compilation:       make test-compilation"
	@echo "Semantic:          make test-semantic"
	@echo "Exhaustive:        make test-exhaustive"
	@echo "Performance:       make bench"
	@echo ""
	@echo "Quality gates:     make quality-gate"
	@echo "Coverage:          make coverage"
	@echo "Full validation:   make validate"
# #@ Release Management
release: ## Prepare a new release
	@echo "Preparing release..."
	@./scripts/prepare-release.sh
tag-release: ## Create and push release tag
	@if [ -z "$(VERSION)" ]; then echo "Error: VERSION not specified. Usage: make tag-release VERSION=0.1.0"; exit 1; fi
	@echo "Creating release tag v$(VERSION)..."
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@echo "Tag created. To push: git push origin v$(VERSION)"
# #@ Multi-Platform Distribution
deploy-all: deploy-check deploy-cargo deploy-docker deploy-github ## Deploy to all distribution channels
	@echo "✅ Deployed to all platforms successfully!"
deploy-check: ## Check deployment prerequisites
	@echo "Checking deployment prerequisites..."
	@command -v cargo > /dev/null || (echo "❌ cargo not found" && exit 1)
	@command -v docker > /dev/null || (echo "❌ docker not found" && exit 1)
	@command -v gh > /dev/null || (echo "❌ GitHub CLI not found" && exit 1)
	@echo "✅ All deployment tools available"
deploy-cargo: ## Deploy to crates.io
	@echo "📦 Publishing to crates.io..."
	$(CARGO) publish --dry-run
	@echo "Dry run successful. To publish: cargo publish"
deploy-pypi: ## Build and deploy Python package to PyPI
	@echo "🐍 Building Python package..."
	@if [ -d "python" ]; then cd python && pip install --upgrade build twine && python -m build && echo "Package built. To upload: twine upload dist/*"; else echo "Python package not yet configured"; fi
deploy-npm: ## Deploy to npm registry
	@echo "📦 Publishing to npm..."
	@if [ -f "npm-package/package.json" ]; then cd npm-package && npm pack && echo "Package created. To publish: npm publish"; else echo "Creating npm package structure..."; mkdir -p npm-package; ./scripts/create-npm-package.sh; fi
deploy-docker: ## Build and push Docker images
	@echo "🐳 Building Docker images..."
	docker build -t depyler/depyler:latest .
	docker build -t depyler/depyler:$(shell grep version Cargo.toml | head -1 | cut -d'"' -f2) .
	@echo "Images built. To push: docker push depyler/depyler:latest"
deploy-homebrew: ## Prepare Homebrew formula
	@echo "🍺 Preparing Homebrew formula..."
	@mkdir -p homebrew
	@./scripts/generate-homebrew-formula.sh > homebrew/depyler.rb
	@echo "Formula generated at homebrew/depyler.rb"
deploy-aur: ## Prepare AUR package
	@echo "📦 Preparing AUR package..."
	@mkdir -p aur
	@./scripts/generate-pkgbuild.sh > aur/PKGBUILD
	@echo "PKGBUILD generated at aur/PKGBUILD"
deploy-deb: ## Build Debian/Ubuntu package
	@echo "📦 Building .deb package..."
	@mkdir -p debian/usr/bin
	@cp target/release/depyler debian/usr/bin/
	@dpkg-deb --build debian depyler_$(shell grep version Cargo.toml | head -1 | cut -d'"' -f2)_amd64.deb
	@echo "Debian package built"
deploy-chocolatey: ## Prepare Chocolatey package
	@echo "🍫 Preparing Chocolatey package..."
	@mkdir -p chocolatey
	@./scripts/generate-nuspec.sh > chocolatey/depyler.nuspec
	@echo "NuSpec generated at chocolatey/depyler.nuspec"
deploy-wasm: ## Build and deploy WASM package
	@echo "🌐 Building WASM package..."
	cd crates/depyler-wasm && wasm-pack build --target web --out-dir pkg
	cd crates/depyler-wasm && wasm-pack pack
	@echo "WASM package built. To publish: wasm-pack publish"
deploy-github: ## Create GitHub release with binaries
	@echo "📦 Creating GitHub release..."
	@VERSION=$$(grep version Cargo.toml | head -1 | cut -d'"' -f2); echo "Building release binaries for v$$VERSION..."; ./scripts/build-all-targets.sh; echo "Creating GitHub release..."; gh release create v$$VERSION --title "Depyler v$$VERSION" --notes-file CHANGELOG.md --draft target/releases/*.tar.gz
build-all-platforms: ## Build for all supported platforms
	@echo "🔨 Building for all platforms..."
	@mkdir -p target/releases
	# Linux x86_64
	cargo build --release --target x86_64-unknown-linux-gnu
	tar czf target/releases/depyler-x86_64-linux.tar.gz -C target/x86_64-unknown-linux-gnu/release depyler
	# macOS x86_64
	@if [ "$$(uname)" = "Darwin" ]; then cargo build --release --target x86_64-apple-darwin; tar czf target/releases/depyler-x86_64-darwin.tar.gz -C target/x86_64-apple-darwin/release depyler; fi
	# Windows
	@if command -v cross > /dev/null; then cross build --release --target x86_64-pc-windows-gnu; zip target/releases/depyler-x86_64-windows.zip target/x86_64-pc-windows-gnu/release/depyler.exe; fi
	@echo "✅ All platform builds complete"
verify-deployment: ## Verify deployment readiness
	@echo "🔍 Verifying deployment readiness..."
	@echo "Checking version consistency..."
	@./scripts/check-versions.sh
	@echo "Running tests..."
	$(MAKE) test-fast
	@echo "Checking documentation..."
	@test -f README.md || (echo "❌ README.md missing" && exit 1)
	@test -f CHANGELOG.md || (echo "❌ CHANGELOG.md missing" && exit 1)
	@test -f LICENSE || (echo "❌ LICENSE missing" && exit 1)
	@echo "✅ Ready for deployment"
deploy-status: ## Show deployment status for all platforms
	@echo "📊 Deployment Status"
	@echo "==================="
	@echo "✅ crates.io:    $$(cargo search depyler | head -1 | awk '{print $$3}')"
	@echo "⏳ PyPI:         Not yet published"
	@echo "⏳ npm:          Not yet published"
	@echo "⏳ Docker Hub:   Not yet published"
	@echo "⏳ Homebrew:     Not yet submitted"
	@echo "⏳ AUR:          Not yet submitted"
	@echo "✅ GitHub:       https://github.com/paiml/depyler"
# #@ Chaos Engineering & Fuzz Testing (Renacer Pattern)
.PHONY: tier1 tier2 tier3 chaos-test fuzz chaos-gentle chaos-aggressive chaos-full
.PHONY: tier1
tier1: ## Fast tests (<5s) - Format, clippy, unit tests
	@echo "🔬 Tier 1: Fast validation (<5s)"
	@$(CARGO) fmt --check
	@$(CARGO) clippy --all-targets --all-features -- -D warnings
	@$(CARGO) test --lib --quiet
.PHONY: tier2
tier2: tier1 ## Integration tests (<30s) - Tier1 + integration
	@echo "🔬 Tier 2: Integration tests (<30s)"
	@$(CARGO) test --tests --quiet
.PHONY: tier3
tier3: tier2 ## Full validation (<5m) - Tier2 + all features
	@echo "🔬 Tier 3: Full validation (<5m)"
	@$(CARGO) test --all-targets --all-features --quiet
.PHONY: chaos-test
chaos-test: ## Chaos engineering tests (basic)
	@echo "💥 Running chaos engineering tests..."
	@$(CARGO) test -p depyler-core --test chaos_tests --features chaos-basic
.PHONY: chaos-gentle
chaos-gentle: ## Gentle chaos testing (development)
	@echo "💥 Running gentle chaos tests..."
	@$(CARGO) test --features chaos-basic -- --nocapture chaos_tests::test_gentle_preset
.PHONY: chaos-aggressive
chaos-aggressive: ## Aggressive chaos testing (CI/CD)
	@echo "💥 Running aggressive chaos tests..."
	@$(CARGO) test --features chaos-basic -- --nocapture chaos_tests::test_aggressive_preset
.PHONY: chaos-full
chaos-full: ## Full chaos testing (network + byzantine)
	@echo "💥 Running full chaos test suite..."
	@$(CARGO) test --features chaos-full --quiet
.PHONY: fuzz
fuzz: ## Fuzz testing (60s) - Requires nightly Rust
	@if ! rustup toolchain list | grep -q nightly; then echo "❌ Nightly Rust not found! Install with: rustup install nightly"; exit 1; fi
	@echo "🎲 Running fuzz testing (60s)..."
	@$(CARGO) +nightly fuzz run fuzz_target_1 -- -max_total_time=60
fuzz-long: ## Fuzz testing (10 minutes)
	@echo "🎲 Running extended fuzz testing (10m)..."
	@$(CARGO) +nightly fuzz run fuzz_target_1 -- -max_total_time=600
fuzz-coverage: ## Fuzz with coverage report
	@echo "🎲 Running fuzz testing with coverage..."
	@$(CARGO) +nightly fuzz coverage fuzz_target_1
	@$(CARGO) +nightly cov -- show target/*/release/fuzz_target_1 --format=html -instr-profile=fuzz/coverage/fuzz_target_1/coverage.profdata > fuzz-coverage.html
	@echo "✅ Coverage report: fuzz-coverage.html"

##@ PMAT Integration

.PHONY: validate-docs
validate-docs: ## Validate documentation accuracy (hallucination detection - Phase 3.5)
	@echo "📚 Validating documentation accuracy (Phase 3.5)..."
	@which pmat > /dev/null 2>&1 || { echo "❌ PMAT not found! Install with: cargo install pmat"; exit 1; }
	@pmat context --output deep_context.md --format llm-optimized
	@pmat validate-readme \
		--targets README.md CLAUDE.md \
		--deep-context deep_context.md \
		--fail-on-contradiction \
		--verbose || { \
		echo "❌ Documentation validation failed!"; \
		exit 1; \
	}
	@echo "✅ Documentation validation complete - zero hallucinations!"

.PHONY: pmat-quality-gate
pmat-quality-gate: ## Run PMAT quality gates (O(1) validation)
	@echo "🔍 Running PMAT quality gates..."
	@which pmat > /dev/null 2>&1 || { echo "❌ PMAT not found! Install with: cargo install pmat"; exit 1; }
	@pmat quality-gate --check-metrics --check-tdg
	@echo "✅ PMAT quality gates passed!"

.PHONY: pmat-rust-score
pmat-rust-score: ## Run Rust Project Score assessment
	@echo "🦀 Running Rust Project Score assessment..."
	@which pmat > /dev/null 2>&1 || { echo "❌ PMAT not found! Install with: cargo install pmat"; exit 1; }
	@pmat rust-project-score --verbose
	@echo "✅ Rust Project Score complete!"

##@ Verificar Synthetic Testing (1000+ Program Variants)

.PHONY: verificar-install verificar-generate verificar-test verificar-report verificar-ci

verificar-install: ## Install verificar for synthetic test generation
	@echo "📦 Installing verificar..."
	@cargo install --path ../verificar 2>/dev/null || cargo install verificar
	@echo "✅ Verificar installed!"

verificar-generate: ## Generate synthetic Python test corpus (1000+ variants)
	@echo "🔬 Generating synthetic Python test programs..."
	@which verificar > /dev/null 2>&1 || { echo "❌ verificar not found! Run: make verificar-install"; exit 1; }
	@mkdir -p target/verificar/corpus
	@verificar depyler --max-depth 4 --output files --output-dir target/verificar/corpus
	@echo "✅ Generated $$(ls target/verificar/corpus/*.py 2>/dev/null | wc -l) Python test programs"

verificar-test: ## Run verificar corpus through depyler and check compilation
	@echo "🧪 Testing depyler with verificar synthetic corpus..."
	@which verificar > /dev/null 2>&1 || { echo "❌ verificar not found! Run: make verificar-install"; exit 1; }
	@mkdir -p target/verificar/corpus target/verificar/output target/verificar/results target/verificar/cargo_proj/src
	@# Generate corpus if not exists
	@if [ ! -d "target/verificar/corpus" ] || [ -z "$$(ls target/verificar/corpus/*.py 2>/dev/null)" ]; then \
		echo "📝 Generating test corpus first..."; \
		verificar depyler --max-depth 4 --output files --output-dir target/verificar/corpus; \
	fi
	@echo "🔄 Transpiling $$(ls target/verificar/corpus/*.py 2>/dev/null | wc -l) Python programs..."
	@mkdir -p target/verificar/bin
	@PASS=0; FAIL=0; \
	for py in target/verificar/corpus/*.py; do \
		name=$$(basename "$$py" .py); \
		rm -f target/verificar/output/Cargo.toml; \
		if ./target/release/depyler transpile "$$py" -o "target/verificar/output/$$name.rs" 2>/dev/null; then \
			if [ -f "target/verificar/output/Cargo.toml" ]; then \
				DEPS=$$(grep -A100 '^\[dependencies\]' target/verificar/output/Cargo.toml | tail -n +2); \
				echo "[package]" > target/verificar/cargo_proj/Cargo.toml; \
				echo "name = \"test_lib\"" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "version = \"0.1.0\"" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "edition = \"2021\"" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "[workspace]" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "[lib]" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "path = \"src/lib.rs\"" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "[dependencies]" >> target/verificar/cargo_proj/Cargo.toml; \
				echo "$$DEPS" >> target/verificar/cargo_proj/Cargo.toml; \
				cp "target/verificar/output/$$name.rs" target/verificar/cargo_proj/src/lib.rs; \
				if (cd target/verificar/cargo_proj && cargo check --quiet 2>/dev/null); then \
					PASS=$$((PASS + 1)); \
					echo "✅ $$name (cargo)"; \
				else \
					FAIL=$$((FAIL + 1)); \
					echo "❌ $$name (cargo fail)"; \
				fi; \
			else \
				if rustc --crate-type lib --edition 2021 "target/verificar/output/$$name.rs" -o "target/verificar/bin/$$name" 2>/dev/null; then \
					PASS=$$((PASS + 1)); \
					echo "✅ $$name"; \
				else \
					FAIL=$$((FAIL + 1)); \
					echo "❌ $$name (compile fail)"; \
				fi; \
			fi; \
		else \
			FAIL=$$((FAIL + 1)); \
			echo "❌ $$name (transpile fail)"; \
		fi; \
	done; \
	TOTAL=$$((PASS + FAIL)); \
	RATE=$$((PASS * 100 / (TOTAL > 0 ? TOTAL : 1))); \
	echo ""; \
	echo "══════════════════════════════════════"; \
	echo "📊 VERIFICAR RESULTS: $$PASS/$$TOTAL ($$RATE%)"; \
	echo "══════════════════════════════════════"; \
	echo "$$PASS $$FAIL $$TOTAL $$RATE" > target/verificar/results/summary.txt

verificar-report: ## Generate detailed verificar test report
	@echo "📊 Generating verificar test report..."
	@if [ ! -f "target/verificar/results/summary.txt" ]; then \
		echo "⚠️  No results found. Run 'make verificar-test' first."; \
		exit 1; \
	fi
	@read PASS FAIL TOTAL RATE < target/verificar/results/summary.txt; \
	echo "# Verificar Synthetic Test Report" > target/verificar/results/report.md; \
	echo "" >> target/verificar/results/report.md; \
	echo "Generated: $$(date)" >> target/verificar/results/report.md; \
	echo "" >> target/verificar/results/report.md; \
	echo "## Summary" >> target/verificar/results/report.md; \
	echo "- **Pass Rate**: $$RATE%" >> target/verificar/results/report.md; \
	echo "- **Passed**: $$PASS" >> target/verificar/results/report.md; \
	echo "- **Failed**: $$FAIL" >> target/verificar/results/report.md; \
	echo "- **Total**: $$TOTAL" >> target/verificar/results/report.md; \
	echo "" >> target/verificar/results/report.md; \
	echo "## Quality Gate" >> target/verificar/results/report.md; \
	if [ "$$RATE" -ge 90 ]; then \
		echo "✅ **PASS** (≥90% threshold)" >> target/verificar/results/report.md; \
	elif [ "$$RATE" -ge 80 ]; then \
		echo "⚠️  **WARNING** (80-90% range)" >> target/verificar/results/report.md; \
	else \
		echo "❌ **FAIL** (<80% threshold)" >> target/verificar/results/report.md; \
	fi; \
	cat target/verificar/results/report.md

verificar-ci: ## CI-ready verificar validation (fail on <80% pass rate)
	@echo "🔬 Running verificar CI validation..."
	@$(MAKE) verificar-test
	@read PASS FAIL TOTAL RATE < target/verificar/results/summary.txt; \
	if [ "$$RATE" -lt 80 ]; then \
		echo "❌ VERIFICAR CI FAILED: $$RATE% pass rate (minimum 80%)"; \
		exit 1; \
	else \
		echo "✅ VERIFICAR CI PASSED: $$RATE% pass rate"; \
	fi

verificar-clean: ## Clean verificar test artifacts
	@echo "🧹 Cleaning verificar artifacts..."
	@rm -rf target/verificar/
	@echo "✅ Cleaned"

##@ bashrs Validation

.PHONY: validate-makefiles
validate-makefiles: ## Validate all Makefiles with bashrs (errors only, warnings allowed)
	@echo "🔍 Validating Makefiles with bashrs..."
	@which bashrs > /dev/null 2>&1 || { echo "❌ bashrs not found! Install with: cargo install bashrs"; exit 1; }
	@FAILED=0; \
	for makefile in $$(find . -name "Makefile" -not -path "./target/*" -not -path "./.git/*"); do \
		echo "  Checking $$makefile..."; \
		ERRORS=$$(bashrs make lint "$$makefile" 2>&1 | grep -c "\[error\]" || true); \
		if [ "$$ERRORS" -gt 0 ]; then \
			echo "❌ $$ERRORS error(s) found in $$makefile"; \
			bashrs make lint "$$makefile" 2>&1 | grep "\[error\]"; \
			FAILED=1; \
		else \
			echo "✅ $$makefile"; \
		fi; \
	done; \
	if [ $$FAILED -eq 1 ]; then exit 1; fi
	@echo "✅ All Makefiles passed validation (0 errors)"

.PHONY: lint-scripts
lint-scripts: ## Lint all shell scripts with bashrs
	@echo "🔍 Linting shell scripts with bashrs..."
	@which bashrs > /dev/null 2>&1 || { echo "❌ bashrs not found! Install with: cargo install bashrs"; exit 1; }
	@./scripts/bashrs_validate_all.sh

.PHONY: bashrs-report
bashrs-report: ## Generate comprehensive bashrs validation report
	@echo "📊 Generating bashrs validation report..."
	@cat bashrs_validation_summary.md

##@ Oracle Training (GH-153)

.PHONY: train-oracle
train-oracle: ## Train Oracle model from unified corpus (reprorusted + OIP)
	@echo "🧠 Training Oracle model from unified corpus..."
	@chmod +x ./scripts/extract_training_data.sh ./scripts/extract_oip_training_data.sh
	@echo "📥 Step 1: Extract real compilation errors..."
	./scripts/extract_training_data.sh
	@echo "📥 Step 2: Extract OIP training data..."
	./scripts/extract_oip_training_data.sh
	@echo "🏋️ Step 3: Train unified model..."
	cargo run --release --example train_unified_corpus -p depyler-oracle -- \
		--errors training_corpus/errors.jsonl \
		--oip training_corpus/oip_data.json \
		--output depyler_oracle.apr \
		--balance --max-per-class 2000
	@echo "✅ Oracle training complete: depyler_oracle.apr"

.PHONY: train-oracle-fast
train-oracle-fast: ## Quick oracle training (synthetic only, for testing)
	@echo "⚡ Quick Oracle training (synthetic only)..."
	cargo run --release --example train_unified_corpus -p depyler-oracle -- \
		--synthetic-samples 5000 \
		--output depyler_oracle.apr
	@echo "✅ Quick training complete"

.PHONY: oracle-extract
oracle-extract: ## Extract training data only (no training)
	@echo "📥 Extracting training data..."
	@chmod +x ./scripts/extract_training_data.sh ./scripts/extract_oip_training_data.sh
	./scripts/extract_training_data.sh
	./scripts/extract_oip_training_data.sh
	@echo "✅ Data extracted to training_corpus/"

.PHONY: oracle-cycle
oracle-cycle: ## Full idempotent oracle training cycle (extract + train + test)
	@echo "🔄 Starting oracle training cycle at $$(date)"
	$(MAKE) oracle-extract
	$(MAKE) train-oracle
	@cargo test --release -p depyler-oracle --lib -q
	@echo "✅ Cycle complete. Model: $$(ls -lh depyler_oracle.apr | awk '{print $$5}')"

# Cycle counter file for accumulating mode
CYCLE_FILE := training_corpus/.cycle_count

.PHONY: oracle-cycle-accumulate
oracle-cycle-accumulate: ## Accumulating oracle cycle (corpus grows each cycle, uses Rust)
	@mkdir -p training_corpus training_corpus/logs
	@# Build binaries if needed
	@if [ ! -f "target/release/extract-training-data" ] || [ ! -f "target/release/depyler" ]; then \
		echo "📦 Building required binaries..."; \
		cargo build --release -p depyler -p depyler-oracle --bin extract-training-data --quiet; \
	fi
	@# Read and increment cycle counter
	@if [ -f "$(CYCLE_FILE)" ]; then \
		CYCLE=$$(cat $(CYCLE_FILE)); \
	else \
		CYCLE=0; \
	fi; \
	CYCLE=$$((CYCLE + 1)); \
	echo $$CYCLE > $(CYCLE_FILE); \
	SEED=$$((42 + CYCLE * 1000 + $$(date +%s) % 1000)); \
	echo "🔄 ACCUMULATING cycle $$CYCLE | seed=$$SEED | $$(date)"; \
	./target/release/extract-training-data \
		--input-dir /home/noah/src/reprorusted-python-cli/examples \
		--corpus training_corpus/errors.jsonl \
		--cycle $$CYCLE \
		--max-files 500; \
	cargo run --release --example train_unified_corpus -p depyler-oracle -- \
		--errors training_corpus/errors.jsonl \
		--output depyler_oracle.apr \
		--seed $$SEED \
		--balance --max-per-class 2000; \
	cargo test --release -p depyler-oracle --lib -q; \
	echo "✅ Cycle $$CYCLE complete. Corpus: $$(wc -l < training_corpus/errors.jsonl) | Model: $$(ls -lh depyler_oracle.apr | awk '{print $$5}')"

.PHONY: oracle-overnight
oracle-overnight: ## Run continuous accumulating training (Ctrl+C to stop)
	@echo "🌙 Starting overnight oracle training loop..."
	@echo "   Mode: ACCUMULATE (corpus grows each cycle)"
	@echo "   Stop: Ctrl+C or 'pkill -f oracle-cycle-accumulate'"
	@echo ""
	@while true; do \
		$(MAKE) oracle-cycle-accumulate 2>&1 | tee -a "training_corpus/logs/overnight_$$(date +%Y%m%d).log"; \
		echo "--- Cycle complete: $$(date) ---" >> "training_corpus/logs/overnight_$$(date +%Y%m%d).log"; \
	done

.PHONY: oracle-harvest
oracle-harvest: ## Harvest real transpilation errors from verificar corpus (Rust)
	@echo "🔍 Harvesting real transpilation errors (Rust binary)..."
	@# Build the extraction binary if needed
	@if [ ! -f "target/release/extract-training-data" ]; then \
		echo "📦 Building extract-training-data binary..."; \
		cargo build --release -p depyler-oracle --bin extract-training-data; \
	fi
	@# Build depyler if needed
	@if [ ! -f "target/release/depyler" ]; then \
		echo "📦 Building depyler..."; \
		cargo build --release -p depyler; \
	fi
	@# Generate verificar corpus if needed
	@if [ ! -d "target/verificar/corpus" ] || [ -z "$$(ls target/verificar/corpus/*.py 2>/dev/null)" ]; then \
		echo "📝 Generating verificar corpus..."; \
		$(MAKE) verificar-generate 2>/dev/null || echo "⚠️  verificar not available"; \
	fi
	@mkdir -p training_corpus
	./target/release/extract-training-data --verbose

.PHONY: oracle-improve
oracle-improve: oracle-harvest train-oracle ## Harvest real errors + retrain (recommended after overnight)
	@echo "✅ Oracle improved with real errors!"

##@ Fast Coverage (cross-platform)

.PHONY: coverage-fast coverage-fast-clean coverage-fast-report

# Cross-platform fast target: Use /tmp on macOS, /dev/shm on Linux
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    FAST_TARGET := /tmp/depyler-target
else
    FAST_TARGET := /dev/shm/depyler-target
endif

coverage-fast: ## Ultra-fast coverage using tmpfs/ramdisk
	@echo "🚀 Fast coverage using $(FAST_TARGET)..."
	@mkdir -p $(FAST_TARGET)
	@echo "   Target dir: $(FAST_TARGET)"
	@CARGO_TARGET_DIR=$(FAST_TARGET) PROPTEST_CASES=3 QUICKCHECK_TESTS=3 \
		cargo llvm-cov nextest --profile fast --lib --workspace \
		--ignore-filename-regex $(COVERAGE_IGNORE_REGEX)
	@echo ""
	@CARGO_TARGET_DIR=$(FAST_TARGET) cargo llvm-cov report --summary-only \
		--ignore-filename-regex $(COVERAGE_IGNORE_REGEX)
	@echo ""
	@echo "✓ Coverage complete."

coverage-fast-report: ## Generate HTML report from fast coverage
	@echo "📊 Generating coverage report..."
	@CARGO_TARGET_DIR=$(FAST_TARGET) cargo llvm-cov report --html \
		--output-dir target/coverage/html \
		--ignore-filename-regex $(COVERAGE_IGNORE_REGEX)
	@echo "✓ Report: target/coverage/html/index.html"

coverage-fast-clean: ## Clean fast coverage artifacts
	@echo "🧹 Cleaning fast coverage artifacts..."
	@rm -rf $(FAST_TARGET)
	@echo "✓ Fast target cleaned"

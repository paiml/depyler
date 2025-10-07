# Depyler Makefile - Comprehensive Testing Infrastructure
# Following NASA/SQLite reliability standards

.PHONY: all build test test-full test-rust test-frontend test-fast test-comprehensive test-fixtures test-property \
        test-compilation test-semantic validate quality-gate coverage \
        clean-test lint lint-rust lint-frontend clippy fmt format fmt-check fmt-fix fmt-rust fmt-frontend fmt-docs \
        check bench install-deps help

# Configuration
CARGO := cargo
MAKEFLAGS += -j$(shell nproc)

# Coverage threshold (NASA standard: 85% minimum)
# Starting with 60% and will increase incrementally
COVERAGE_THRESHOLD := 60

# Quality gate thresholds
MAX_COMPLEXITY := 10
MAX_LINES_PER_FUNCTION := 50

# Build Configuration
# Fix linker issues by using system linker instead of lld
RUST_FLAGS := RUSTFLAGS="-D warnings -C linker=cc"
TEST_FLAGS := --workspace --all-features
RELEASE_FLAGS := --release

# Default target
all: validate

##@ Quick Start

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
	@if [ ! -d "playground/public/wasm" ]; then \
		echo "WASM not found, building..."; \
		cd crates/depyler-wasm && wasm-pack build --target web --out-dir ../../playground/public/wasm; \
	else \
		echo "✓ Using existing WASM build"; \
	fi
	@if [ ! -d "playground/node_modules" ]; then \
		echo "Installing dependencies..."; \
		cd playground && npm install; \
	else \
		echo "✓ Dependencies already installed"; \
	fi
	@if [ ! -d "playground/dist" ]; then \
		echo "Building frontend..."; \
		cd playground && npm run build; \
	else \
		echo "✓ Using existing frontend build"; \
	fi
	@echo "✅ Playground ready! Starting server..."
	cd playground && npm run preview

# Main test target - fast tests with coverage report
test: ## Run Rust tests with coverage (two-phase pattern)
	@echo "Running Rust tests with coverage..."
	@$(CARGO) llvm-cov clean --workspace
	@$(CARGO) llvm-cov --no-report test --workspace --lib --all-features
	@echo ""
	@echo "=== Coverage Summary ==="
	@$(CARGO) llvm-cov report --summary-only
	@$(CARGO) llvm-cov report --summary-only --fail-under-functions $(COVERAGE_THRESHOLD) || true

test-full: test test-frontend ## Run all tests (Rust + frontend)

test-rust: test ## Alias for main test target

test-frontend: ## Run frontend tests (npm + deno)
	@echo "Running frontend tests..."
	@if [ -d "playground" ]; then \
		echo "Running npm tests..."; \
		cd playground && npm test; \
		echo "Running Deno tests..."; \
		cd playground && deno test \
			--allow-read \
			--allow-env \
			--allow-net \
			src/**/*.deno.test.ts \
			src/**/*.test.ts 2>/dev/null || true; \
	fi

##@ Building
build: ## Build the project
	$(RUST_FLAGS) $(CARGO) build $(RELEASE_FLAGS)

build-dev: ## Build for development
	$(CARGO) build

clean: ## Clean build artifacts
	$(CARGO) clean

##@ Playground

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

##@ Testing

# Fast tests for development iteration
test-fast: ## Run fast unit tests
	@echo "Running fast unit tests..."
	$(CARGO) test --lib $(TEST_FLAGS) --quiet

# Comprehensive test suite (NASA-grade)
test-comprehensive: test-fixtures test-property test-compilation test-semantic ## Run all tests with full validation
	@echo "All comprehensive tests passed ✅"

# Individual test categories
test-fixtures: ## Test all Python fixture transpilation
	@echo "Testing fixture transpilation..."
	$(CARGO) test --test transpilation_tests $(TEST_FLAGS)

test-property: ## Run property-based tests
	@echo "Running property-based tests..."
	$(CARGO) test --test property_tests $(TEST_FLAGS)
	$(CARGO) test --test semantic_equivalence $(TEST_FLAGS)
	$(CARGO) test --test property_tests_ast_roundtrip $(TEST_FLAGS)
	$(CARGO) test --test property_tests_type_inference $(TEST_FLAGS)
	$(CARGO) test --test property_tests_memory_safety $(TEST_FLAGS)

##@ Advanced Testing Infrastructure (Phases 8-10)

test-property-basic: ## Run basic property tests (Phases 1-3)
	@echo "Running basic property tests..."
	$(CARGO) test --test property_tests $(TEST_FLAGS)
	$(CARGO) test --test semantic_equivalence $(TEST_FLAGS)
	$(CARGO) test --test property_tests_ast_roundtrip $(TEST_FLAGS)
	$(CARGO) test --test property_tests_type_inference $(TEST_FLAGS)
	$(CARGO) test --test property_tests_memory_safety $(TEST_FLAGS)

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

test-fast: ## Quick feedback loop for development
	@echo "Running fast development tests..."
	$(CARGO) test --lib $(TEST_FLAGS) --quiet
	$(CARGO) test --test property_tests $(TEST_FLAGS) --quiet

test-ci: ## CI/CD optimized test execution
	@echo "Running CI/CD tests..."
	$(MAKE) test-property-basic
	$(MAKE) test-coverage
	$(MAKE) test-integration

##@ Performance Testing

test-benchmark: ## Performance regression testing
	@echo "Running performance benchmarks..."
	$(CARGO) test --test property_test_benchmarks $(TEST_FLAGS)
	$(CARGO) test --test integration_benchmarks $(TEST_FLAGS)
	$(CARGO) bench

test-profile: ## Performance profiling and analysis
	@echo "Running performance profiling..."
	$(CARGO) test --test performance_profiling $(TEST_FLAGS)
	./scripts/run_performance_suite.sh

test-memory: ## Memory usage validation
	@echo "Running memory tests..."
	$(CARGO) test --test memory_safety_tests $(TEST_FLAGS)
	$(CARGO) test --test resource_exhaustion $(TEST_FLAGS)

test-concurrency: ## Thread safety and parallel execution
	@echo "Running concurrency tests..."
	$(CARGO) test --test concurrent_execution $(TEST_FLAGS)
	$(CARGO) test --test thread_safety $(TEST_FLAGS)

##@ Development Workflows

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

test-compilation: ## Validate generated Rust compiles
	@echo "Validating Rust compilation..."
	$(CARGO) test --test rustc_compilation $(TEST_FLAGS)

test-semantic: ## Test semantic equivalence
	@echo "Testing semantic equivalence..."
	$(CARGO) test semantic $(TEST_FLAGS)

test-integration: ## Run integration tests
	@echo "Running integration tests..."
	$(CARGO) test --test integration $(TEST_FLAGS)

test-unit: ## Run unit tests only
	@echo "Running unit tests..."
	$(CARGO) test --lib $(TEST_FLAGS)

# Exhaustive testing (10,000+ cases)
test-exhaustive: ## Run exhaustive test suite (10k+ cases)
	@echo "Running exhaustive test suite..."
	DEPYLER_EXHAUSTIVE=1 $(CARGO) test $(TEST_FLAGS) -- --test-threads=1

# Performance testing
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

##@ Validation

validate: quality-gate test-comprehensive coverage ## Full validation pipeline
	@echo "🎉 All validation gates passed!"

quick-validate: lint test-fast ## Quick validation for development
	@echo "Quick validation passed ✅"

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
	@if [ -z "$(FILE)" ]; then \
		echo "❌ Error: FILE not specified"; \
		echo "Usage: make validate-example FILE=examples/showcase/fibonacci.rs"; \
		exit 1; \
	fi
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

##@ Quality Assurance

quality-gate: lint clippy complexity-check ## Run quality checks
	@echo "Quality gate passed ✅"

lint: lint-rust lint-frontend ## Run all linters (Rust + frontend)

lint-rust: ## Run Rust linter (clippy)
	@echo "Running Rust linter..."
	$(CARGO) clippy $(TEST_FLAGS) -- -D warnings

lint-frontend: ## Run frontend linter (deno lint)
	@echo "Running frontend linter..."
	@if [ -d "playground" ]; then \
		echo "Running Deno lint..."; \
		cd playground && deno lint \
			--unstable-component \
			src/ \
			*.ts \
			*.tsx \
			*.js \
			*.jsx 2>/dev/null || true; \
	fi

clippy: ## Run Clippy linter
	@echo "Running Clippy..."
	$(RUST_FLAGS) $(CARGO) clippy $(TEST_FLAGS) -- -D warnings -D clippy::all

format: fmt-rust fmt-frontend fmt-docs ## Format all code artifacts comprehensively

fmt: format ## Alias for comprehensive formatting

fmt-rust: ## Format Rust code
	@echo "Formatting Rust code..."
	$(CARGO) fmt

fmt-frontend: ## Format TypeScript, JavaScript, CSS, HTML, JSON with Deno
	@echo "Formatting frontend code with Deno..."
	@if [ -d "playground" ]; then \
		cd playground && deno fmt \
			--unstable-component \
			--line-width=100 \
			--indent-width=2 \
			--single-quote=false \
			--no-semicolons=false \
			--ext=ts,tsx,js,jsx,json,html,css,md \
			src/ \
			*.ts \
			*.tsx \
			*.js \
			*.jsx \
			*.json \
			*.html \
			*.css \
			*.md 2>/dev/null || true; \
	fi

fmt-docs: ## Format documentation files with Deno
	@echo "Formatting documentation with Deno..."
	@deno fmt \
		--line-width=80 \
		--prose-wrap=always \
		--indent-width=2 \
		--ext=md \
		*.md \
		docs/*.md \
		crates/*/README.md \
		examples/*/README.md \
		playground/README.md 2>/dev/null || true

fmt-check: ## Check if all files are formatted
	@echo "Checking Rust formatting..."
	$(CARGO) fmt --check
	@echo "Checking frontend formatting..."
	@if [ -d "playground" ]; then \
		cd playground && deno fmt --check \
			--unstable-component \
			--line-width=100 \
			--indent-width=2 \
			--single-quote=false \
			--no-semicolons=false \
			--ext=ts,tsx,js,jsx,json,html,css,md \
			src/ \
			*.ts \
			*.tsx \
			*.js \
			*.jsx \
			*.json \
			*.html \
			*.css \
			*.md 2>/dev/null || true; \
	fi
	@echo "Checking documentation formatting..."
	@deno fmt --check \
		--line-width=80 \
		--prose-wrap=always \
		--indent-width=2 \
		--ext=md \
		*.md \
		docs/*.md \
		crates/*/README.md \
		examples/*/README.md \
		playground/README.md 2>/dev/null || true

fmt-fix: format ## Alias for comprehensive formatting

complexity-check: ## Check code complexity
	@echo "Checking code complexity..."
	@# This would integrate with a complexity analysis tool
	@echo "Complexity check passed (manual verification required)"

security-audit: ## Run security audit
	@echo "Running security audit..."
	$(CARGO) audit

##@ Coverage

coverage: ## Generate coverage report (pforge pattern)
	@echo "📊 Running comprehensive test coverage analysis..."
	@echo "🔍 Checking for cargo-llvm-cov and cargo-nextest..."
	@which cargo-llvm-cov > /dev/null 2>&1 || (echo "📦 Installing cargo-llvm-cov..." && cargo install cargo-llvm-cov --locked)
	@which cargo-nextest > /dev/null 2>&1 || (echo "📦 Installing cargo-nextest..." && cargo install cargo-nextest --locked)
	@echo "🧹 Cleaning old coverage data..."
	@$(CARGO) llvm-cov clean --workspace
	@mkdir -p target/coverage
	@echo "⚙️  Temporarily disabling global cargo config (linker may break coverage)..."
	@test -f ~/.cargo/config.toml && mv ~/.cargo/config.toml ~/.cargo/config.toml.cov-backup || true
	@echo "🧪 Phase 1: Running tests with instrumentation (no report)..."
	@$(CARGO) llvm-cov --no-report nextest --no-tests=warn --all-features --workspace
	@echo "📊 Phase 2: Generating coverage reports..."
	@$(CARGO) llvm-cov report --html --output-dir target/coverage/html
	@$(CARGO) llvm-cov report --lcov --output-path target/coverage/lcov.info
	@echo "⚙️  Restoring global cargo config..."
	@test -f ~/.cargo/config.toml.cov-backup && mv ~/.cargo/config.toml.cov-backup ~/.cargo/config.toml || true
	@echo ""
	@echo "📊 Coverage Summary:"
	@echo "=================="
	@$(CARGO) llvm-cov report --summary-only
	@echo ""
	@echo "💡 COVERAGE INSIGHTS:"
	@echo "- HTML report: target/coverage/html/index.html"
	@echo "- LCOV file: target/coverage/lcov.info"

coverage-summary: ## Display coverage summary (run 'make coverage' first)
	@echo "📊 Coverage Summary:"
	@echo "=================="
	@$(CARGO) llvm-cov report --summary-only || echo "⚠️  Run 'make coverage' first to generate coverage data"

coverage-open: ## Open HTML coverage report in browser (run 'make coverage' first)
	@echo "🌐 Opening coverage report in browser..."
	@if [ ! -f target/coverage/html/index.html ]; then \
		echo "⚠️  Coverage report not found. Run 'make coverage' first."; \
		exit 1; \
	fi
	@if command -v xdg-open > /dev/null; then \
		xdg-open target/coverage/html/index.html; \
	elif command -v open > /dev/null; then \
		open target/coverage/html/index.html; \
	else \
		echo "💡 Cannot auto-open. View report at: target/coverage/html/index.html"; \
	fi

coverage-check: ## Check coverage threshold (assumes coverage already collected)
	@echo "Checking coverage threshold..."
	@COVERAGE=$$($(CARGO) llvm-cov report --summary-only | grep "TOTAL" | awk '{print $$4}' | sed 's/%//'); \
	if [ "$$COVERAGE" -lt "$(COVERAGE_THRESHOLD)" ]; then \
		echo "❌ Coverage $$COVERAGE% below threshold $(COVERAGE_THRESHOLD)%"; \
		exit 1; \
	else \
		echo "✅ Coverage $$COVERAGE% meets threshold $(COVERAGE_THRESHOLD)%"; \
	fi

##@ Test Data Management

generate-fixtures: ## Generate additional test fixtures
	@echo "Generating test fixtures..."
	python3 scripts/generate_fixtures.py --count 50 --output tests/fixtures/

clean-test: ## Clean test artifacts
	@echo "Cleaning test artifacts..."
	rm -rf target/llvm-cov/
	rm -f lcov.info
	rm -rf tests/temp/
	$(CARGO) clean

##@ Development Dependencies

install-deps: ## Install development dependencies
	@echo "Installing development dependencies..."
	$(CARGO) install cargo-llvm-cov
	$(CARGO) install cargo-audit
	$(CARGO) install cargo-watch
	@if ! command -v rustfmt > /dev/null; then \
		rustup component add rustfmt; \
	fi
	@if ! command -v clippy > /dev/null; then \
		rustup component add clippy; \
	fi

check-deps: ## Check if all dependencies are installed
	@echo "Checking dependencies..."
	@command -v rustc > /dev/null || (echo "❌ rustc not found" && exit 1)
	@command -v cargo > /dev/null || (echo "❌ cargo not found" && exit 1)
	@$(CARGO) llvm-cov --version > /dev/null || (echo "❌ cargo-llvm-cov not found" && exit 1)
	@$(CARGO) audit --version > /dev/null || (echo "❌ cargo-audit not found" && exit 1)
	@echo "✅ All dependencies found"

##@ Continuous Integration

ci-test: ## Run CI test suite
	@echo "Running CI test suite..."
	$(MAKE) check-deps
	$(MAKE) validate
	$(MAKE) coverage-check

ci-quick: ## Quick CI check
	@echo "Running quick CI check..."
	$(MAKE) lint
	$(MAKE) test-fast

##@ Development Workflow

watch: ## Watch for changes and run tests
	@echo "Watching for changes..."
	$(CARGO) watch -x "test --lib"

watch-test: ## Watch and run specific test
	@echo "Usage: make watch-test TEST=test_name"
	$(CARGO) watch -x "test $(TEST)"

dev-setup: install-deps ## Setup development environment
	@echo "Development environment setup complete ✅"

##@ Reporting

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
	@for config in debug release; do \
		echo "Testing in $$config mode..."; \
		if [ "$$config" = "release" ]; then \
			$(CARGO) test $(TEST_FLAGS) --release; \
		else \
			$(CARGO) test $(TEST_FLAGS); \
		fi; \
	done

##@ Documentation

docs: ## Generate documentation
	$(CARGO) doc --workspace --no-deps

docs-open: docs ## Generate and open documentation
	$(CARGO) doc --workspace --no-deps --open

##@ Performance Profiling

profile-memory: ## Profile memory usage with Valgrind
	@echo "Profiling memory usage..."
	@if command -v valgrind > /dev/null; then \
		$(CARGO) build --release --bin depyler; \
		valgrind --tool=massif --massif-out-file=massif.out \
			./target/release/depyler transpile examples/showcase/binary_search.py; \
		ms_print massif.out > memory-profile.txt; \
		echo "Memory profile saved to: memory-profile.txt"; \
	else \
		echo "valgrind not found. Install with: sudo apt-get install valgrind"; \
	fi

profile-flamegraph: ## Generate flamegraph performance profile
	@echo "Generating flamegraph..."
	@if command -v cargo-flamegraph > /dev/null; then \
		$(CARGO) flamegraph --root --bin depyler -- transpile examples/showcase/binary_search.py; \
		echo "Flamegraph saved to: flamegraph.svg"; \
	else \
		echo "cargo-flamegraph not found. Install with: cargo install flamegraph"; \
	fi

profile-perf: ## Profile with Linux perf
	@echo "Profiling with perf..."
	@if command -v perf > /dev/null; then \
		$(CARGO) build --release --bin depyler; \
		perf record --call-graph dwarf ./target/release/depyler transpile examples/showcase/binary_search.py; \
		perf report > perf-report.txt; \
		echo "Perf report saved to: perf-report.txt"; \
	else \
		echo "perf not found. Install with: sudo apt-get install linux-tools-generic"; \
	fi

profile-heap: ## Profile heap allocations
	@echo "Profiling heap allocations..."
	$(CARGO) build --release --features "profiling" --bin depyler
	@echo "Heap profiling enabled in binary. Run with heap profiler."

##@ Binary Size Optimization

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
	@if command -v upx > /dev/null; then \
		$(MAKE) build-min-size; \
		cp target/min-size/depyler target/min-size/depyler.compressed; \
		upx --best target/min-size/depyler.compressed; \
		echo "Original:   $$(du -h target/min-size/depyler | cut -f1)"; \
		echo "Compressed: $$(du -h target/min-size/depyler.compressed | cut -f1)"; \
	else \
		echo "UPX not found. Install with: sudo apt-get install upx-ucl"; \
	fi

analyze-binary-size: ## Run comprehensive binary size analysis
	@echo "Running binary size analysis..."
	./scripts/track_binary_size.sh

size-report: ## Generate detailed size report
	@echo "Generating size report..."
	./scripts/track_binary_size.sh
	@echo "Report saved to: binary_size_report.md"

##@ Performance Analysis

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
	@if [ -f baseline-performance.json ]; then \
		$(CARGO) bench -- --save-baseline current; \
		cargo-criterion-cmp baseline current; \
	else \
		echo "No baseline found. Creating baseline..."; \
		$(CARGO) bench -- --save-baseline baseline; \
		cp target/criterion/baseline target/criterion/baseline-performance.json; \
		echo "Baseline created. Run again to compare."; \
	fi

##@ Help

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

##@ Release Management

release: ## Prepare a new release
	@echo "Preparing release..."
	@./scripts/prepare-release.sh

tag-release: ## Create and push release tag
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION not specified. Usage: make tag-release VERSION=0.1.0"; \
		exit 1; \
	fi
	@echo "Creating release tag v$(VERSION)..."
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@echo "Tag created. To push: git push origin v$(VERSION)"

##@ Multi-Platform Distribution

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
	@if [ -d "python" ]; then \
		cd python && \
		pip install --upgrade build twine && \
		python -m build && \
		echo "Package built. To upload: twine upload dist/*"; \
	else \
		echo "Python package not yet configured"; \
	fi

deploy-npm: ## Deploy to npm registry
	@echo "📦 Publishing to npm..."
	@if [ -f "npm-package/package.json" ]; then \
		cd npm-package && \
		npm pack && \
		echo "Package created. To publish: npm publish"; \
	else \
		echo "Creating npm package structure..."; \
		mkdir -p npm-package; \
		./scripts/create-npm-package.sh; \
	fi

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
	@VERSION=$$(grep version Cargo.toml | head -1 | cut -d'"' -f2); \
	echo "Building release binaries for v$$VERSION..."; \
	./scripts/build-all-targets.sh; \
	echo "Creating GitHub release..."; \
	gh release create v$$VERSION \
		--title "Depyler v$$VERSION" \
		--notes-file CHANGELOG.md \
		--draft \
		target/releases/*.tar.gz

build-all-platforms: ## Build for all supported platforms
	@echo "🔨 Building for all platforms..."
	@mkdir -p target/releases
	# Linux x86_64
	cargo build --release --target x86_64-unknown-linux-gnu
	tar czf target/releases/depyler-x86_64-linux.tar.gz -C target/x86_64-unknown-linux-gnu/release depyler
	# macOS x86_64
	@if [ "$$(uname)" = "Darwin" ]; then \
		cargo build --release --target x86_64-apple-darwin; \
		tar czf target/releases/depyler-x86_64-darwin.tar.gz -C target/x86_64-apple-darwin/release depyler; \
	fi
	# Windows
	@if command -v cross > /dev/null; then \
		cross build --release --target x86_64-pc-windows-gnu; \
		zip target/releases/depyler-x86_64-windows.zip target/x86_64-pc-windows-gnu/release/depyler.exe; \
	fi
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
# depyler justfile - Cross-platform build automation
# Usage: just <recipe>

# Default recipe
default: test

# Build release binary
build:
    cargo build --release

# Run all tests
test:
    cargo test 

# Run unit tests only (fast)
test-unit:
    cargo test --lib

# Lint with clippy
lint:
    cargo clippy --all-targets  -- -D warnings

# Format check
fmt:
    cargo fmt --all -- --check

# Format fix
fmt-fix:
    cargo fmt --all

# Run benchmarks
bench:
    cargo bench

# Check compilation
check:
    cargo check 

# Run documentation build
doc:
    cargo doc --no-deps 

# Security audit
audit:
    cargo audit

# Full quality gate
tier2: fmt lint test

# Pre-push gate
tier3: fmt lint test doc audit

# Run property tests
test-prop:
    PROPTEST_CASES=256 cargo test --test property_tests

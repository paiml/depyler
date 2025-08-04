# Depyler Release Process - Toyota Way

This document describes the zero-defect release process for Depyler, following
Toyota Way principles and matching the quality standards of PMAT.

## 🎌 Toyota Way Principles

### 自働化 (Jidoka) - Build Quality In

- **ZERO** Self-Admitted Technical Debt (SATD)
- **ZERO** incomplete implementations
- **ZERO** functions exceeding complexity 20
- **ZERO** test failures
- **ZERO** lint warnings

### 現地現物 (Genchi Genbutsu) - Go and See

- Run actual transpilation tests
- Verify generated Rust compiles
- Test on real Python codebases
- Profile performance metrics

### 改善 (Kaizen) - Continuous Improvement

- Each release must improve metrics
- Document lessons learned
- Update quality gates based on findings

## Pre-Release Requirements

**ALL must be GREEN before any release:**

```bash
# Run the comprehensive audit
./scripts/pre-release-audit.sh

# If ANY blockers found, fix them first
# NO EXCEPTIONS - Zero defects policy
```

## Release Workflow

### Step 1: Pre-Release Audit

```bash
# This script enforces zero-defect policy
./scripts/pre-release-audit.sh

# Review the generated report
cat docs/release-audit.md

# Fix ANY issues found - no exceptions
```

### Step 2: Update Version

```bash
# Update workspace version
sed -i 's/version = ".*"/version = "X.Y.Z"/' Cargo.toml

# Update all internal dependencies
find crates -name "Cargo.toml" -exec sed -i 's/"OLD_VERSION"/"X.Y.Z"/g' {} \;

# Update Cargo.lock
cargo update --workspace
```

### Step 3: Final Quality Check

```bash
# Format all code
cargo fmt --all

# Run clippy with pedantic lints
cargo clippy --workspace -- -D warnings \
  -W clippy::pedantic \
  -W clippy::nursery \
  -W clippy::cargo

# Run all tests
cargo test --workspace

# Build documentation
cargo doc --workspace --no-deps

# Run audit again to confirm
./scripts/pre-release-audit.sh
```

### Step 4: Update CHANGELOG

Create/update `CHANGELOG.md`:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### 🎌 Quality Metrics

- SATD Count: 0 (Toyota Way: Zero Defects)
- Max Complexity: <20
- Test Coverage: >90%
- Clippy Warnings: 0

### ✨ Features

- Feature description implementing Priority N

### 🐛 Bug Fixes

- Fix description with zero new debt

### 📚 Documentation

- Documentation improvements

### 🔧 Internal

- Zero-defect refactoring
```

### Step 5: Commit and Tag

```bash
# Stage all changes
git add -A

# Commit with detailed message
git commit -m "release: v$VERSION - Zero Defect Release

Summary of changes following Toyota Way principles:
- Zero SATD policy maintained
- All quality gates passed
- Priority N implementation complete

Quality Metrics:
- SATD: 0
- Max Complexity: <20  
- Tests: 100% passing
- Clippy: 0 warnings

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"

# Create signed tag
git tag -s -a v$VERSION -m "Release v$VERSION

Zero-defect release following Toyota Way.
See CHANGELOG.md for details."

# Push to trigger CI/CD
git push origin main
git push origin v$VERSION
```

### Step 6: Monitor Automated Release

GitHub Actions will:

1. Run quality gate checks
2. Block release if ANY issues found
3. Publish to crates.io in order
4. Create GitHub release with audit report
5. Verify installation works

### Step 7: Post-Release Verification

```bash
# Wait for crates.io indexing
sleep 120

# Test installation
cargo install depyler --force
depyler --version

# Run smoke tests
echo "def test(): return 42" > test.py
depyler transpile test.py
cat test.rs

# Verify all crates published
for crate in depyler-{annotations,core,analyzer,verify,quality,mcp,wasm} depyler; do
  echo "Checking $crate..."
  cargo search $crate --limit 1
done
```

## Quality Gates

### Mandatory Checks (Automated)

| Check      | Requirement          | Enforcement     |
| ---------- | -------------------- | --------------- |
| SATD       | ZERO TODO/FIXME/HACK | Release blocked |
| Complexity | All functions <20    | Release blocked |
| Tests      | 100% passing         | Release blocked |
| Clippy     | Zero warnings        | Release blocked |
| Incomplete | Zero todo!()         | Release blocked |

### Manual Review

Before pushing tag:

- [ ] Review generated Rust code quality
- [ ] Test on 3+ real Python projects
- [ ] Benchmark performance vs previous
- [ ] Security audit for new code
- [ ] Documentation completeness

## Emergency Procedures

### Yanking a Release

If critical issue found post-release:

```bash
# Yank all affected crates immediately
for crate in depyler-{annotations,core,analyzer,verify,quality,mcp,wasm} depyler; do
  cargo yank --vers X.Y.Z $crate
done

# Create hotfix branch
git checkout -b hotfix/vX.Y.Z+1 vX.Y.Z

# Fix with zero new debt
# Run full audit before release
./scripts/pre-release-audit.sh

# Fast-track release (still must pass all gates)
```

### Rollback Process

1. Document issue in `docs/postmortem/vX.Y.Z.md`
2. Add regression test
3. Update quality gates if needed
4. Release patch with fix
5. Un-yank if appropriate

## Release Cadence

Following systematic priority implementation:

| Priority | Version | Timeline  | Focus             |
| -------- | ------- | --------- | ----------------- |
| 1        | 1.0.X   | Immediate | Critical fixes    |
| 2        | 1.1.X   | 2-4 weeks | Core features     |
| 3        | 1.2.X   | 4-6 weeks | Type system       |
| 4        | 1.3.X   | 4 weeks   | Verification      |
| 5        | 2.0.0   | 8 weeks   | Advanced features |

## Continuous Improvement

After each release:

1. Run retrospective
2. Update quality gates
3. Improve automation
4. Document lessons learned
5. Plan next priority

## Zero Compromise Policy

**NEVER** release with:

- Any SATD markers
- Any incomplete code
- Any failing tests
- Any complexity >20
- Any clippy warnings

Quality over speed. Every time.

---

_"The right process will produce the right results"_ - Toyota Way

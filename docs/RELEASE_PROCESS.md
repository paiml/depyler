# Depyler Release Process

This document describes the systematic release process for Depyler, following the implementation workflow defined in our [development roadmap](todo/next-gen-depyler.md).

## Overview

Each release follows a strict quality-first approach:
1. Implement features for a single priority level
2. Run comprehensive quality checks
3. Push to GitHub with proper tags
4. Build and publish releases
5. Move to the next priority

## Pre-Release Checklist

- [ ] All tests passing: `cargo test --workspace`
- [ ] Clippy clean: `cargo clippy --workspace -- -D warnings`
- [ ] Documentation updated: `cargo doc --no-deps`
- [ ] CHANGELOG.md updated with all changes
- [ ] Version numbers updated in all Cargo.toml files
- [ ] README.md reflects new features

## Release Steps

### 1. Update Versions

Update the workspace version in the root `Cargo.toml`:
```toml
[workspace.package]
version = "X.Y.Z"
```

Update all internal dependencies to match:
```bash
# Update all depyler-* dependencies
find crates -name "Cargo.toml" -exec sed -i 's/"OLD_VERSION"/"NEW_VERSION"/g' {} \;
```

### 2. Run Quality Checks

```bash
# Full test suite
cargo test --workspace

# Clippy with pedantic lints
cargo clippy --workspace -- -D warnings

# Check documentation
cargo doc --no-deps --open

# Run benchmarks to ensure no regression
cargo bench
```

### 3. Commit and Tag

```bash
# Commit version updates
git add -A
git commit -m "chore: bump version to X.Y.Z

Update workspace and crate versions for the new release."

# Create annotated tag
git tag -a vX.Y.Z -m "Release vX.Y.Z - <Brief Description>

<Detailed release notes here>"

# Push to GitHub
git push origin main
git push origin vX.Y.Z
```

### 4. Create GitHub Release

Using the GitHub CLI:
```bash
gh release create vX.Y.Z \
  --title "vX.Y.Z - <Title>" \
  --notes "<Full release notes>"
```

Or manually through the GitHub web interface.

### 5. Publish to crates.io

Run the publish script:
```bash
./scripts/publish-crates.sh
```

Or manually publish in order:
```bash
cargo publish -p depyler-annotations
cargo publish -p depyler-core  
cargo publish -p depyler-analyzer
cargo publish -p depyler-verify
cargo publish -p depyler-quality
cargo publish -p depyler-mcp
cargo publish -p depyler-wasm
cargo publish -p depyler
```

Wait 30 seconds between each publish for crates.io indexing.

## Version Numbering

We follow semantic versioning:
- **MAJOR** (X.0.0): Breaking changes or major feature sets
- **MINOR** (0.X.0): New features, backward compatible
- **PATCH** (0.0.X): Bug fixes and minor improvements

Priority mapping to versions:
- Priority 1 (Critical Fixes): Patch releases (1.0.X)
- Priority 2 (Core Features): Minor releases (1.1.X)
- Priority 3 (Type System): Minor releases (1.2.X)
- Priority 4 (Verification): Minor releases (1.3.X)
- Priority 5 (Advanced): Major release (2.0.0)

## Quality Gates

Each release must pass ALL quality gates:
- ✅ 100% of tests passing
- ✅ Zero clippy warnings with pedantic lints
- ✅ Code coverage >85% overall, >90% for new code
- ✅ All examples compile and run correctly
- ✅ Documentation builds without warnings
- ✅ Benchmarks show <5% performance regression
- ✅ Binary size within 10% of target
- ✅ Successful test on 3 real Python projects
- ✅ Cross-platform CI fully green

## Post-Release

1. Monitor crates.io for successful indexing
2. Check GitHub issues for immediate problems
3. Update project boards with completed items
4. Plan next priority implementation
5. Announce release on relevant channels

## Rollback Plan

If critical issues are discovered:
1. `cargo yank --vers X.Y.Z depyler` (and all affected crates)
2. Fix issues on hotfix branch
3. Release patch version X.Y.(Z+1)
4. Update yanked version notice on crates.io

## Release Artifacts

Each release should include:
- Source code (automatic via Git)
- Pre-built binaries for major platforms (via GitHub Actions)
- Comprehensive release notes
- Migration guide (if applicable)
- Performance comparison data

## Communication

Release announcements should be made on:
- GitHub Releases page
- crates.io (via README)
- Project documentation site
- Relevant Rust community channels

---

Following this process ensures each release maintains our extreme quality standards while delivering incremental value to users.
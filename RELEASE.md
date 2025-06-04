# Depyler v0.1.0 Release Checklist

This document outlines the steps for releasing Depyler v0.1.0.

## Pre-Release Verification

### ✅ Code Quality
- [x] All tests passing (70 tests)
- [x] Function coverage > 60% (achieved 62.88%)
- [x] Zero clippy warnings
- [x] Code formatted with `cargo fmt`
- [x] No security vulnerabilities (`cargo audit`)

### ✅ Documentation
- [x] README.md with accurate installation instructions
- [x] CHANGELOG.md following Keep a Changelog format
- [x] API documentation generated with `cargo doc`
- [x] Example code working correctly

### ✅ Release Infrastructure
- [x] GitHub Actions workflow for automated releases
- [x] Multi-platform build matrix (Linux, macOS, Windows)
- [x] Installer script with proper error handling
- [x] SHA256 checksums generation

## Release Process

### 1. Prepare Release
```bash
# Run the release preparation script
./scripts/prepare-release.sh

# This will:
# - Update version in Cargo.toml
# - Update version in release workflow
# - Update CHANGELOG.md
# - Run all tests
# - Check formatting and linting
# - Build release binary
```

### 2. Review Changes
```bash
# Review all changes
git diff

# Ensure version is consistent across:
# - Cargo.toml (workspace version)
# - .github/workflows/release.yml (line 109)
# - CHANGELOG.md
```

### 3. Commit Changes
```bash
git add -A
git commit -m "chore: prepare release v0.1.0"
```

### 4. Create and Push Tag
```bash
# Create annotated tag
git tag -a v0.1.0 -m "Release v0.1.0

Initial release of Depyler - Python to Rust transpiler
- Core transpilation for Python V1 subset
- Type inference and mapping
- Memory-optimized architecture
- 62.88% test coverage
"

# Push changes and tag
git push origin main
git push origin v0.1.0
```

### 5. GitHub Actions Will Automatically:
1. Create GitHub release with generated notes
2. Build binaries for all platforms:
   - `depyler-linux-amd64.tar.gz`
   - `depyler-linux-arm64.tar.gz`
   - `depyler-darwin-amd64.tar.gz`
   - `depyler-darwin-arm64.tar.gz`
   - `depyler-windows-amd64.zip`
3. Generate and upload `install.sh`
4. Create SHA256SUMS file

### 6. Post-Release
1. Verify release assets on GitHub
2. Test installation script:
   ```bash
   curl -sSfL https://github.com/paiml/depyler/releases/download/v0.1.0/install.sh | sh
   ```
3. Update documentation if needed
4. Announce release

## Installation Verification

### Quick Install (Verified)
```bash
# Download and run installer
curl -sSfL https://github.com/paiml/depyler/releases/latest/download/install.sh | sh

# The installer will:
# - Detect platform (Linux/macOS, x64/ARM64)
# - Download appropriate binary
# - Install to ~/.local/bin
# - Verify installation
# - Provide PATH setup instructions
```

### Manual Install (Verified)
```bash
# Download for your platform
wget https://github.com/paiml/depyler/releases/latest/download/depyler-linux-amd64.tar.gz

# Extract
tar xzf depyler-linux-amd64.tar.gz

# Install
sudo mv depyler /usr/local/bin/

# Verify
depyler --version
```

### Build from Source (Verified)
```bash
# Clone repository
git clone https://github.com/paiml/depyler.git
cd depyler

# Build and install
cargo build --release
cargo install --path crates/depyler

# Verify
depyler --version
```

## Known Issues for v0.1.0

1. Generated Rust code needs formatting improvements
2. Limited Python feature support (V1 subset only)
3. Docstring comments are converted to string literals (will be fixed)

## Future Releases

- v0.2.0: Improved code generation, rustfmt integration
- v0.3.0: Extended Python feature support
- v0.4.0: Advanced type inference and optimization
- v1.0.0: Production-ready with full Python subset support
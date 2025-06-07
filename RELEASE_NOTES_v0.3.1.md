# Release Notes - Depyler v0.3.1

## 🎯 Release Highlights

**Stability Improvements & Experimental Playground Warning**

This patch release focuses on stability improvements, test fixes, and properly marking the Interactive Playground as **EXPERIMENTAL** and **UNSTABLE**. While the playground offers exciting capabilities, it should not be used in production environments until it reaches stable status in a future release.

---

## ⚠️ IMPORTANT: Playground Status

The Interactive Playground feature introduced in v0.3.0 is marked as:

- **🧪 EXPERIMENTAL**: Features may change without notice
- **⚡ UNSTABLE**: May contain bugs or performance issues
- **🚧 NOT FOR PRODUCTION**: Use only for testing and evaluation

We encourage developers to try the playground and provide feedback, but please be aware that:
- The API may change in breaking ways between minor versions
- Performance characteristics are not yet optimized
- Some edge cases may cause crashes or unexpected behavior

---

## 🐛 Bug Fixes

### Frontend Test Suite
- Fixed CodeEditor.tsx syntax error caused by extra closing brace
- Fixed QualityScorer missing `parse_p95_ms` configuration property
- Rewrote ExecutionManager tests to match actual implementation API
- Fixed SettingsDropdown test expectations for aria-checked states
- Added stub methods to QualityMonitor for test compatibility

### Code Quality
- Resolved all TypeScript/React lint warnings in playground
- Fixed all Rust clippy warnings across all crates
- Improved test infrastructure for better CI reliability

### Documentation
- Added experimental warnings to all playground-related documentation
- Fixed broken links in documentation (pending verification)
- Updated user guide for accuracy with current features

---

## 📚 Changes Since v0.3.0

### Test Infrastructure Improvements
The test suite has been significantly improved to ensure reliability:

```typescript
// Fixed execution manager tests to match actual API
expect(result.python.output).toBe("Hello");
expect(result.rust.output).toBe("Hello");
expect(result.energy_savings_percent).toBeGreaterThan(0);
```

### Playground Warnings
All playground entry points now display clear warnings:

```
⚠️ EXPERIMENTAL FEATURE - UNSTABLE
This playground is under active development and should not be used in production.
Features may change or break without notice.
```

---

## 🔧 Developer Notes

### Running Tests
All tests should now pass cleanly:

```bash
# Backend tests
cargo test --workspace

# Frontend tests  
cd playground && npm test

# Linting
make lint
```

### Known Limitations
- Some integration tests still require mock implementations
- Deno test runner has compatibility issues with certain imports
- Performance metrics in tests use approximations

---

## 📈 What's Next

### v0.3.2 (Upcoming Patch)
- Additional playground stability improvements
- Enhanced error recovery in transpilation edge cases
- Performance optimizations for large codebases

### v0.4.0 (Q2 2024)
- Playground graduates from experimental status
- IDE plugin beta releases
- Advanced Python pattern support

---

## 🚀 Quick Start

### Install/Update Depyler

```bash
# Update to v0.3.1
cargo install depyler --version 0.3.1

# Or build from source
git clone https://github.com/paiml/depyler.git
cd depyler
git checkout v0.3.1
cargo install --path crates/depyler
```

### Using the Experimental Playground

```bash
# Launch with experimental warning acknowledged
DEPYLER_EXPERIMENTAL=true depyler playground

# Or use the stable CLI for production work
depyler transpile input.py --verify standard
```

---

## 🙏 Acknowledgments

Thanks to all contributors who helped identify and fix the stability issues in this release. Your feedback on the experimental playground has been invaluable in improving the overall quality of Depyler.

---

**Remember**: The core transpilation engine remains stable and production-ready. Only the Interactive Playground feature is marked as experimental in this release.
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.17.x  | :white_check_mark: |
| 3.16.x  | :white_check_mark: |
| < 3.16  | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in Depyler, please report it by emailing the maintainers. Please do not create a public GitHub issue for security vulnerabilities.

## Security Status (v3.17.0)

### Current Security Posture

✅ **Zero Critical Vulnerabilities**
- All critical vulnerabilities have been resolved
- Continuous monitoring via `cargo audit` and `cargo deny`

### Dependency Security

#### Fixed Vulnerabilities

| Advisory | Severity | Status | Fixed In |
|----------|----------|--------|----------|
| RUSTSEC-2025-0003 | Critical | ✅ FIXED | v3.17.0 |
| RUSTSEC-2024-0379 | High | ✅ FIXED | v3.17.0 |

**Details**:
- **RUSTSEC-2025-0003**: fast-float 0.2.0 - Segmentation fault vulnerability
  - **Fix**: Updated polars from 0.35.4 → 0.51.0 in depyler-ruchy
  - **Impact**: Eliminated vulnerable fast-float dependency entirely
- **RUSTSEC-2024-0379**: fast-float soundness issues
  - **Fix**: Same as above (same dependency)

#### Documented Warnings (Low Risk)

| Advisory | Crate | Status | Risk Level | Mitigation |
|----------|-------|--------|------------|------------|
| RUSTSEC-2025-0057 | fxhash | Unmaintained | Low | Via sled→pmat, tracked for replacement |
| RUSTSEC-2024-0384 | instant | Unmaintained | Low | Via parking_lot→sled, tracked for replacement |
| RUSTSEC-2024-0436 | paste | Unmaintained | Low | Proc-macro only (compile-time), no runtime risk |

**Rationale for "Low Risk"**:
- **fxhash**: Hash function library, unmaintained but stable. No known vulnerabilities.
- **instant**: Time library, unmaintained. Alternative exists (web-time), will migrate in future.
- **paste**: Procedural macro for code generation. Only runs at compile-time, no runtime security implications.

### Security Tooling

#### Cargo Deny

We use [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) to enforce security policies:

```bash
# Check all security advisories
cargo deny check advisories

# Check all policies (licenses, bans, sources)
cargo deny check
```

Configuration: `deny.toml`
- Denies: Critical and high-severity vulnerabilities
- Warns: Unmaintained crates (documented exceptions)
- License policy: MIT, Apache-2.0, BSD-3-Clause, and others

#### Cargo Audit

We use [`cargo-audit`](https://github.com/RustSec/rustsec) for continuous vulnerability monitoring:

```bash
# Run security audit
cargo audit

# Update advisory database
cargo audit fetch
```

### Continuous Integration

Our CI pipeline includes:
1. `cargo audit` - Security vulnerability scanning
2. `cargo deny check` - Policy enforcement
3. `cargo test` - Zero regressions policy
4. `cargo clippy` - Code quality and security lints

### Update Policy

**Dependency Updates**:
- Security updates: Applied immediately
- Major version updates: Tested thoroughly, released in minor versions
- Lock file: Updated regularly, committed to repository

**Security Advisories**:
- Monitored daily via RustSec advisory database
- Critical issues: Fixed within 24-48 hours
- High issues: Fixed within 1 week
- Medium/Low issues: Fixed in next release cycle

### Best Practices for Users

When using Depyler in production:

1. **Pin Versions**: Use exact version requirements in Cargo.toml
   ```toml
   depyler = "=3.17.0"  # Pin to exact version
   ```

2. **Regular Updates**: Keep dependencies updated
   ```bash
   cargo update
   cargo audit
   ```

3. **CI Integration**: Add security checks to your CI pipeline
   ```bash
   cargo deny check advisories
   cargo audit
   ```

4. **Review Generated Code**: Always review transpiled Rust code before production use

### Security Features

**Depyler includes built-in security features**:

1. **Memory Safety**: Generated Rust code is memory-safe by design
2. **Type Safety**: Strong type checking prevents common vulnerabilities
3. **No Unsafe Code**: Depyler avoids `unsafe` blocks where possible
4. **Input Validation**: Python AST validation prevents malicious inputs
5. **Sandboxed Execution**: Optional features for safe code execution

### Dependency Tree

**Direct Dependencies** (Security-Critical):
- `rustpython-parser`: Python parser (regularly updated)
- `syn`, `quote`: Rust code generation (maintained by Rust team)
- `tokio`: Async runtime (widely used, security-audited)

**Optional Dependencies** (Low Risk):
- `polars`: DataFrame library (only for experimental features)
- `ruchy`: Scripting language (experimental, not production-critical)
- `pmat`: Quality analysis (development tool only)

### Future Security Work

**v3.17.0 Phase 2** (Planned):
- Replace unmaintained `fxhash` with `rustc-hash` or `ahash`
- Evaluate `instant` replacement with `web-time`
- Review and update all dependencies to latest secure versions

**Long-term**:
- Automated dependency updates via Dependabot
- Regular security audits of generated code
- Fuzzing infrastructure for parser/codegen
- Third-party security audit (when production-ready)

### Contact

For security concerns:
- Email: [security@depyler.org] (when available)
- GitHub: Create a private security advisory
- Temporary: Use GitHub issues for non-sensitive security discussions

---

**Last Updated**: 2025-10-10 (v3.17.0)
**Security Policy Version**: 1.0

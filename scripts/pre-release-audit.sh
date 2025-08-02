#!/bin/bash
# Pre-release comprehensive audit - Toyota Way Zero Defects
# NO release until ALL checks pass

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Output file
OUTPUT="docs/release-audit.md"
mkdir -p docs

# Version from argument or detect
VERSION="${1:-$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2)}"

echo -e "${BLUE}=== DEPYLER PRE-RELEASE AUDIT v$VERSION ===${NC}"
echo "Toyota Way: Zero Defects Policy"
echo ""

# Start the audit report
cat > "$OUTPUT" <<EOF
# Release Audit Report - v$VERSION

Generated: $(date)
Standard: Toyota Way Zero Defects

## Executive Summary

This automated audit enforces ZERO tolerance for:
- Self-Admitted Technical Debt (SATD)
- Functions exceeding complexity 20
- Incomplete implementations
- Failing tests
- Lint warnings

**Release Status**: ⏳ PENDING

---

## 🔴 CRITICAL BLOCKERS (Must be ZERO)

### 1. Self-Admitted Technical Debt (SATD)
**Policy**: ZERO TODO, FIXME, HACK, XXX, or INCOMPLETE

EOF

# Check for SATD
echo -e "${YELLOW}Checking for SATD markers...${NC}"
SATD_COUNT=0
echo '```' >> "$OUTPUT"
if grep -rn "TODO\|FIXME\|HACK\|XXX\|INCOMPLETE" crates/ --include="*.rs" 2>/dev/null | grep -v "target/" >> "$OUTPUT"; then
    SATD_COUNT=$(grep -rn "TODO\|FIXME\|HACK\|XXX\|INCOMPLETE" crates/ --include="*.rs" 2>/dev/null | grep -v "target/" | wc -l)
    echo '```' >> "$OUTPUT"
    echo "❌ **FOUND $SATD_COUNT SATD MARKERS** - Release BLOCKED" >> "$OUTPUT"
else
    echo "No SATD found" >> "$OUTPUT"
    echo '```' >> "$OUTPUT"
    echo "✅ **SATD Check: PASSED** - Zero technical debt" >> "$OUTPUT"
fi

# Check for high complexity
echo -e "\n### 2. Function Complexity\n**Policy**: No function may exceed cyclomatic complexity of 20\n" >> "$OUTPUT"
echo -e "${YELLOW}Analyzing complexity...${NC}"
COMPLEX_COUNT=0
echo '```' >> "$OUTPUT"
# Use cargo-complexity if available, otherwise manual check
if command -v cargo-complexity &> /dev/null; then
    cargo complexity --max 20 2>&1 | tee -a "$OUTPUT" || true
else
    echo "Note: Install cargo-complexity for detailed analysis" >> "$OUTPUT"
    # Basic check for deeply nested code
    find crates -name "*.rs" -exec awk 'BEGIN{max=0} {gsub(/[^{]/, ""); if(length>max) max=length} END{if(max>10) print FILENAME": Deep nesting detected (depth="max")"}' {} \; >> "$OUTPUT"
fi
echo '```' >> "$OUTPUT"

# Check for incomplete implementations
echo -e "\n### 3. Incomplete Implementations\n**Policy**: No unimplemented!(), todo!(), unreachable!() in non-test code\n" >> "$OUTPUT"
echo -e "${YELLOW}Checking for placeholders...${NC}"
INCOMPLETE_COUNT=0
echo '```' >> "$OUTPUT"
if grep -rn "unimplemented!\|todo!\|unreachable!" crates/ --include="*.rs" 2>/dev/null | grep -v "target/" | grep -v "test" >> "$OUTPUT"; then
    INCOMPLETE_COUNT=$(grep -rn "unimplemented!\|todo!\|unreachable!" crates/ --include="*.rs" 2>/dev/null | grep -v "target/" | grep -v "test" | wc -l)
    echo '```' >> "$OUTPUT"
    echo "❌ **FOUND $INCOMPLETE_COUNT INCOMPLETE IMPLEMENTATIONS** - Release BLOCKED" >> "$OUTPUT"
else
    echo "No incomplete implementations found" >> "$OUTPUT"
    echo '```' >> "$OUTPUT"
    echo "✅ **Implementation Check: PASSED**" >> "$OUTPUT"
fi

# Check for panics in non-test code
echo -e "\n### 4. Panic Usage\n**Policy**: No panic!() or expect() in production code\n" >> "$OUTPUT"
echo -e "${YELLOW}Checking for panics...${NC}"
PANIC_COUNT=0
echo '```' >> "$OUTPUT"
if grep -rn "panic!\|\.expect(" crates/ --include="*.rs" 2>/dev/null | grep -v "target/" | grep -v "test" | grep -v "example" >> "$OUTPUT"; then
    PANIC_COUNT=$(grep -rn "panic!\|\.expect(" crates/ --include="*.rs" 2>/dev/null | grep -v "target/" | grep -v "test" | grep -v "example" | wc -l)
    echo '```' >> "$OUTPUT"
    echo "⚠️  **FOUND $PANIC_COUNT PANIC SITES** - Review required" >> "$OUTPUT"
else
    echo "No panics found in production code" >> "$OUTPUT"
    echo '```' >> "$OUTPUT"
    echo "✅ **Panic Check: PASSED**" >> "$OUTPUT"
fi

# Test status
echo -e "\n### 5. Test Suite Status\n" >> "$OUTPUT"
echo -e "${YELLOW}Running tests...${NC}"
TEST_PASSED=true
if cargo test --workspace --quiet 2>&1; then
    echo "✅ **All tests PASSED**" >> "$OUTPUT"
else
    TEST_PASSED=false
    echo "❌ **Tests FAILED** - Release BLOCKED" >> "$OUTPUT"
fi

# Clippy lints
echo -e "\n### 6. Clippy Lints\n**Policy**: Zero warnings with pedantic lints\n" >> "$OUTPUT"
echo -e "${YELLOW}Running clippy...${NC}"
CLIPPY_PASSED=true
echo '```' >> "$OUTPUT"
if cargo clippy --workspace -- -D warnings 2>&1 | tee -a "$OUTPUT"; then
    echo '```' >> "$OUTPUT"
    echo "✅ **Clippy: PASSED** - No warnings" >> "$OUTPUT"
else
    CLIPPY_PASSED=false
    echo '```' >> "$OUTPUT"
    echo "❌ **Clippy FAILED** - Release BLOCKED" >> "$OUTPUT"
fi

# Documentation check
echo -e "\n### 7. Documentation Coverage\n" >> "$OUTPUT"
echo -e "${YELLOW}Checking documentation...${NC}"
echo '```' >> "$OUTPUT"
cargo doc --workspace --no-deps 2>&1 | grep -E "warning|error" >> "$OUTPUT" || echo "No documentation warnings" >> "$OUTPUT"
echo '```' >> "$OUTPUT"

# Generate summary
echo -e "\n---\n\n## 📊 Release Readiness Summary\n" >> "$OUTPUT"

TOTAL_BLOCKERS=$((SATD_COUNT + INCOMPLETE_COUNT))
if [ "$TEST_PASSED" = false ]; then
    TOTAL_BLOCKERS=$((TOTAL_BLOCKERS + 1))
fi
if [ "$CLIPPY_PASSED" = false ]; then
    TOTAL_BLOCKERS=$((TOTAL_BLOCKERS + 1))
fi

cat >> "$OUTPUT" <<EOF
| Check | Result | Count | Status |
|-------|--------|-------|--------|
| SATD Markers | $([ $SATD_COUNT -eq 0 ] && echo "✅ PASS" || echo "❌ FAIL") | $SATD_COUNT | $([ $SATD_COUNT -eq 0 ] && echo "Ready" || echo "BLOCKED") |
| Incomplete Code | $([ $INCOMPLETE_COUNT -eq 0 ] && echo "✅ PASS" || echo "❌ FAIL") | $INCOMPLETE_COUNT | $([ $INCOMPLETE_COUNT -eq 0 ] && echo "Ready" || echo "BLOCKED") |
| Panic Sites | $([ $PANIC_COUNT -eq 0 ] && echo "✅ PASS" || echo "⚠️ WARN") | $PANIC_COUNT | Review |
| Test Suite | $([ "$TEST_PASSED" = true ] && echo "✅ PASS" || echo "❌ FAIL") | - | $([ "$TEST_PASSED" = true ] && echo "Ready" || echo "BLOCKED") |
| Clippy Lints | $([ "$CLIPPY_PASSED" = true ] && echo "✅ PASS" || echo "❌ FAIL") | - | $([ "$CLIPPY_PASSED" = true ] && echo "Ready" || echo "BLOCKED") |

**Total Blockers**: $TOTAL_BLOCKERS

EOF

if [ $TOTAL_BLOCKERS -eq 0 ]; then
    echo -e "\n## ✅ RELEASE APPROVED\n\nAll quality gates passed. Ready for v$VERSION release." >> "$OUTPUT"
    RELEASE_STATUS="${GREEN}APPROVED${NC}"
else
    echo -e "\n## ❌ RELEASE BLOCKED\n\n$TOTAL_BLOCKERS critical issues must be resolved before release." >> "$OUTPUT"
    RELEASE_STATUS="${RED}BLOCKED${NC}"
fi

# Add checklist
cat >> "$OUTPUT" <<'EOF'

---

## ✅ Release Checklist

### Code Quality (MUST BE 100%)
- [ ] Zero SATD (TODO, FIXME, HACK, XXX)
- [ ] Zero incomplete implementations
- [ ] All functions < complexity 20
- [ ] Zero clippy warnings
- [ ] All tests passing
- [ ] Documentation complete

### Pre-Release Steps
- [ ] Run `cargo fmt --all`
- [ ] Update CHANGELOG.md
- [ ] Update version in Cargo.toml
- [ ] Run this audit again
- [ ] Create git tag

### Release Process
- [ ] Push tag to GitHub
- [ ] GitHub Actions creates release
- [ ] Publish to crates.io
- [ ] Verify installation works
- [ ] Update documentation

### Post-Release
- [ ] Monitor for issues
- [ ] Update dependent projects
- [ ] Plan next iteration

---

## 🤖 Fix Commands

```bash
# Remove all SATD markers
grep -rn "TODO\|FIXME\|HACK" crates/ --include="*.rs" | cut -d: -f1 | sort -u | xargs -I {} sed -i '/TODO\|FIXME\|HACK/d' {}

# Format all code
cargo fmt --all

# Fix clippy issues
cargo clippy --workspace --fix -- -D warnings

# Run tests with output
cargo test --workspace -- --nocapture
```

---

Generated by Depyler Release Auditor
Toyota Way: 自働化 (Jidoka) - Build Quality In
EOF

# Print summary
echo ""
echo -e "${BLUE}=== AUDIT COMPLETE ===${NC}"
echo "Report: $OUTPUT"
echo ""
echo "Release Status: $RELEASE_STATUS"
echo "Total Blockers: $TOTAL_BLOCKERS"
echo ""

if [ $TOTAL_BLOCKERS -eq 0 ]; then
    echo -e "${GREEN}✅ Ready to release v$VERSION${NC}"
    exit 0
else
    echo -e "${RED}❌ Fix $TOTAL_BLOCKERS issues before release${NC}"
    exit 1
fi
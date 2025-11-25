# Verificar Corpus Testing Framework

**Purpose**: Systematic transpiler validation using verificar-generated test programs to stop thrashing and identify root causes of failures.

## Quick Start

```bash
# Generate test corpus
verificar generate -l python -c 50 -d 3 -o json > corpus.json

# Run systematic tests
python3 test_corpus_v2.py corpus.json

# View results
cat test_results_v2_*/SUMMARY.txt
```

## What This Solves

### Before (Thrashing)
- ❌ Manual testing of random examples
- ❌ No systematic coverage
- ❌ Same bugs hit repeatedly
- ❌ No metrics or tracking
- ❌ Can't measure progress

### After (Systematic)
- ✅ Automated test generation (verificar)
- ✅ Comprehensive coverage (depth 1-3+)
- ✅ Error categorization (E0308, E0425, etc.)
- ✅ Quantitative metrics (36% pass rate)
- ✅ Track improvements over time

## Current Results (2025-11-25)

### Corpus: 50 programs (depth 3)

| Metric | Count | Rate |
|--------|-------|------|
| **Transpilation Success** | 50/50 | **100%** |
| **Compilation Success** | 18/50 | **36%** |
| **Overall Pass** | 18/50 | **36%** |

### Error Breakdown

| Error | Count | % | Issue |
|-------|-------|---|-------|
| **E0308** (type mismatch) | 15 | 47% | Negative literals, None |
| **E0425** (undefined var) | 10 | 31% | Forward refs, self-refs |
| Other | 7 | 22% | Various |

### Pattern Analysis

**By Depth**:
- Depth 1: 100% pass (1/1) ✅
- Depth 2: 67% pass (20/30) ⚠️
- Depth 3: 0% pass (0/18) ❌

**Key Finding**: All depth-3 failures are parenthesized negative literals!

## Priority Fixes

### 1. Negative Literal Type Inference (E0308)

**Test Case**: `x = -1` or `x = (-1)`

**Current**:
```rust
pub const x: serde_json::Value = -1;
```

**Error**:
```
error[E0308]: mismatched types
  expected `serde_json::Value`
  found integer `-1`
```

**Fix**: Infer `i32` directly for integer literals:
```rust
pub const x: i32 = -1;
```

**Impact**: Fixes 15 failures (~47%)

### 2. Undefined Variable Detection (E0425)

**Test Case**: `x = y` (where y is undefined)

**Current**: Transpiles without error
**Error**: `E0425: cannot find value 'y'`

**Fix Options**:
1. Pre-pass to detect undefined variables
2. Emit placeholder for testing: `let y: serde_json::Value = Default::default();`
3. Fail transpilation with clear error

**Impact**: Fixes 10 failures (~31%)

### 3. Self-Reference Handling

**Test Case**: `x = x` (before x is defined)

**Current**: Transpiles, then fails in Rust
**Fix**: Detect circular dependency, emit error or warning

**Impact**: Edge case, low priority

## Files

### Core Scripts

- **test_corpus_v2.py**: Main testing script with cargo build
- **test_corpus.py**: V1 (rustc only) - deprecated
- **FINDINGS.md**: Detailed analysis and methodology

### Generated Files

- **corpus_d3_c50.json**: 50 programs, depth 3
- **corpus_d2_c10.json**: 10 programs, depth 2 (quick test)
- **test_results_v2_*/**: Test results with logs and reports

## Usage

### Generate Custom Corpus

```bash
# Small test (fast)
verificar generate -c 10 -d 2 -o json > test.json

# Medium test (comprehensive)
verificar generate -c 50 -d 3 -o json > medium.json

# Large test (exhaustive)
verificar generate -c 200 -d 4 -o json > large.json
```

### Run Tests

```bash
# Run on any corpus
python3 test_corpus_v2.py <corpus.json>

# Results saved to test_results_v2_<timestamp>/
```

### Analyze Results

```bash
# Read summary
cat test_results_v2_*/SUMMARY.txt

# Check specific failures
ls test_results_v2_*/*.log

# JSON report for programmatic analysis
jq '.error_categories' test_results_v2_*/report.json
```

## Integration with CI/CD

```yaml
name: Verificar Corpus Testing

on: [push, pull_request]

jobs:
  corpus-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install verificar
        run: cargo install verificar

      - name: Generate corpus
        run: verificar generate -c 100 -d 3 -o json > corpus.json

      - name: Run tests
        run: python3 tests/verificar_corpus/test_corpus_v2.py corpus.json

      - name: Check pass rate
        run: |
          PASS_RATE=$(jq '.stats.compile_success / .stats.total * 100' test_results_v2_*/report.json)
          if (( $(echo "$PASS_RATE < 80" | bc -l) )); then
            echo "Pass rate $PASS_RATE% below threshold 80%"
            exit 1
          fi

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: corpus-test-results
          path: test_results_v2_*
```

## Roadmap

### Phase 1: Foundation ✅ (DONE)
- [x] Verificar integration
- [x] Test script with cargo build
- [x] Error categorization
- [x] Metrics tracking

### Phase 2: Fix Core Issues (CURRENT)
- [ ] Fix E0308 negative literals (15 failures)
- [ ] Fix E0425 undefined variables (10 failures)
- [ ] Re-run corpus to verify fixes
- [ ] Target: 80% pass rate

### Phase 3: Expand Coverage
- [ ] Depth 4-5 programs
- [ ] Control flow (if/else, loops)
- [ ] Functions and classes
- [ ] Target: 90% pass rate on depth 4

### Phase 4: Production
- [ ] CI/CD integration
- [ ] Automated regression testing
- [ ] Performance benchmarking
- [ ] Target: 95%+ pass rate

## Impact

**Before Verificar**:
- reprorusted: 9/9 examples failing (0% success)
- No systematic understanding of why
- Thrashing on same issues

**After Verificar**:
- Identified 2 root causes (E0308, E0425)
- Quantified impact (47% + 31% = 78% of failures)
- Clear path forward with measurable progress
- Can track improvements: 36% → 80% → 95%

## See Also

- [FINDINGS.md](./FINDINGS.md) - Detailed analysis
- [Verificar](https://github.com/paiml/verificar) - Test generator
- [Depyler](https://github.com/paiml/depyler) - Python→Rust transpiler

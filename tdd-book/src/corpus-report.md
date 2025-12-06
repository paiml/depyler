# Corpus Report: Scientific Analysis

The `depyler report` command provides deterministic, scientific analysis of Python corpus transpilation quality using Toyota Way methodology.

## Overview

The report command runs a 4-phase pipeline on a Python corpus:

1. **Clean** - Remove previous transpilation artifacts
2. **Transpile + Compile** - Convert Python files to Rust and attempt compilation (integrated phase for accurate error capture)
3. **Analyze** - Classify errors by taxonomy
4. **Report** - Generate actionable output

## Quick Start

```bash
# Analyze default corpus (reprorusted-python-cli)
depyler report

# Analyze custom corpus
depyler report --corpus /path/to/python/project

# Generate JSON output
depyler report --format json --output report.json

# Generate Markdown report
depyler report --format markdown --output report.md
```

## Command Options

| Option | Default | Description |
|--------|---------|-------------|
| `-c, --corpus` | reprorusted-python-cli | Path to Python corpus |
| `-f, --format` | terminal | Output format: terminal, json, markdown |
| `-o, --output` | stdout | Output file path |
| `--skip-clean` | false | Skip clean phase (reuse previous transpilation) |
| `--target-rate` | 0.8 | Target compilation success rate (0.0-1.0) |

## Output Formats

### Terminal Output

The default terminal output provides:

```
================================================================================
                         CORPUS COMPILATION REPORT
================================================================================

📊 Executive Summary
────────────────────────────────────────────────────────────────────────────────
  Corpus:           /path/to/corpus
  Total Files:      244
  Compiled:         84
  Failed:           160
  Success Rate:     34.4%
  Target Rate:      80.0%
  Andon Status:     🔴 RED (< 50%)

📋 Error Taxonomy (by Impact)
────────────────────────────────────────────────────────────────────────────────
  #1  E0425 (cannot find value)      - 45 files (28.1%)
      Description: Unresolved identifier or variable
      Sample: error[E0425]: cannot find value `foo` in this scope

  #2  E0412 (cannot find type)       - 32 files (20.0%)
      Description: Unknown type reference
      Sample: error[E0412]: cannot find type `CustomType` in this scope

  #3  E0308 (mismatched types)       - 28 files (17.5%)
      Description: Type mismatch in expression
      Sample: error[E0308]: mismatched types expected `i32`, found `String`

🎯 Priority Fix Items
────────────────────────────────────────────────────────────────────────────────
  P0-CRITICAL:
    - E0425: Improve variable/import resolution (45 files)

  P1-HIGH:
    - E0412: Enhance type discovery (32 files)
    - E0308: Fix type coercion (28 files)

  P2-MEDIUM:
    - E0277: Add trait implementations (15 files)
```

### JSON Output

Machine-readable output for CI/CD integration:

```json
{
  "corpus_path": "/path/to/corpus",
  "total_files": 244,
  "compiled": 84,
  "failed": 160,
  "success_rate": 0.344,
  "target_rate": 0.8,
  "target_reached": false,
  "andon_status": "red",
  "error_taxonomy": [
    {
      "error_code": "E0425",
      "count": 45,
      "percentage": 28.1,
      "description": "Unresolved identifier or variable",
      "sample": "error[E0425]: cannot find value `foo` in this scope",
      "priority": "P0-CRITICAL",
      "fix_recommendation": "Improve variable/import resolution"
    }
  ]
}
```

### Markdown Output

Documentation-ready report:

```markdown
# Corpus Compilation Report

## Summary
- **Corpus**: /path/to/corpus
- **Success Rate**: 34.4% (84/244)
- **Status**: 🔴 RED

## Error Taxonomy
| Rank | Code | Count | % | Description |
|------|------|-------|---|-------------|
| 1 | E0425 | 45 | 28.1% | Unresolved identifier |
| 2 | E0412 | 32 | 20.0% | Unknown type |
...
```

## Toyota Way Methodology

The report command follows lean manufacturing principles:

### Jidoka (自働化) - Autonomation
Automatic detection of compilation failures with immediate feedback.

### Genchi Genbutsu (現地現物) - Go and See
Direct observation of actual Rust compiler errors, not assumptions.

### Kaizen (改善) - Continuous Improvement
Prioritized fix recommendations for incremental progress.

### 5S Organization
- **Seiri**: Sort errors by impact
- **Seiton**: Organize by error code
- **Seiso**: Clean error samples (strip ANSI codes)
- **Seiketsu**: Standardized taxonomy
- **Shitsuke**: Discipline in tracking

## Andon Status Alerts

| Status | Rate | Action |
|--------|------|--------|
| 🟢 GREEN | ≥80% | Proceed to release |
| 🟡 YELLOW | 50-80% | Focus on P0/P1 items |
| 🔴 RED | <50% | Stop the line, address critical issues |

## Error Code Reference

| Code | Description | Common Cause |
|------|-------------|--------------|
| E0425 | Cannot find value | Missing import, undefined variable |
| E0412 | Cannot find type | Undefined struct/enum, missing use |
| E0308 | Mismatched types | Type coercion error |
| E0277 | Trait not satisfied | Missing trait impl |
| E0432 | Unresolved import | Invalid module path |
| E0599 | Method not found | Missing method impl |
| E0433 | Failed module resolution | Invalid crate/module reference |

## CI/CD Integration

### GitHub Actions

```yaml
- name: Corpus Analysis
  run: |
    depyler report --format json --output report.json

    # Check if target rate met
    RATE=$(jq '.success_rate' report.json)
    if (( $(echo "$RATE < 0.8" | bc -l) )); then
      echo "Compilation rate $RATE below 80% target"
      exit 1
    fi
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

REPORT=$(depyler report --format json 2>/dev/null)
STATUS=$(echo "$REPORT" | jq -r '.andon_status')

if [ "$STATUS" = "red" ]; then
    echo "❌ Corpus compilation rate is RED - commit blocked"
    exit 1
fi
```

## Workflow Examples

### Daily Corpus Check

```bash
# Morning check
depyler report --format markdown --output daily-$(date +%Y%m%d).md

# Track progress over time
ls -la daily-*.md
```

### Focus Development Session

```bash
# Identify top priority
depyler report 2>&1 | grep "P0-CRITICAL"

# Fix transpiler issues
# ... development work ...

# Verify improvement
depyler report
```

### Release Gate

```bash
# Check if ready for release
if depyler report --target-rate 0.9 2>&1 | grep -q "🟢 GREEN"; then
    echo "Ready for release!"
    cargo publish
else
    echo "Not ready - continue fixing"
fi
```

## Troubleshooting

### Report takes too long

Use `--skip-clean` to reuse previous transpilation:

```bash
depyler report --skip-clean
```

### Custom corpus not found

Ensure the path contains `.py` files:

```bash
find /path/to/corpus -name "*.py" | wc -l
```

### Permission errors

Check write permissions for output directory:

```bash
depyler report --output /tmp/report.json
```

## See Also

- [CLI Usage Guide](./cli-usage.md) - Full command reference
- [Hunt Mode](./hunt-mode.md) - Automated calibration
- [Oracle](./oracle.md) - ML error classification

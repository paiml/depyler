# CITL Corpus Extraction

This chapter describes how to extract training corpora for Compiler-in-the-Loop (CITL) training of the depyler oracle.

## Overview

CITL training requires high-quality Python code with executable examples. The best sources are:

| Source | Doctests | Quality | Notes |
|--------|----------|---------|-------|
| CPython stdlib | ~1,700 | Excellent | Written by core developers |
| reprorusted-python-cli | ~6,800 tests | Excellent | Curated for transpilation |
| PyPI packages | Varies | Variable | Requires filtering |

## Prerequisites

Install the `alimentar` corpus extraction tool:

```bash
cargo install alimentar

# Verify installation
alimentar --version
```

## Extracting CPython Stdlib Doctests

### Automated Extraction

The easiest way is using the Makefile in [reprorusted-python-cli](https://github.com/paiml/reprorusted-python-cli):

```bash
git clone https://github.com/paiml/reprorusted-python-cli
cd reprorusted-python-cli

# Extract ~1,700 doctests to data/corpora/
make extract-cpython-doctests
```

### Manual Extraction

For custom extraction or debugging:

```bash
# 1. Clone CPython
git clone --depth 1 https://github.com/python/cpython /tmp/cpython

# 2. Filter out problematic directories (non-UTF-8 files)
rsync -a --exclude='test' --exclude='idlelib' --exclude='turtledemo' \
    /tmp/cpython/Lib/ /tmp/cpython-lib-clean/

# 3. Extract with alimentar
CPYTHON_SHA=$(cd /tmp/cpython && git rev-parse --short HEAD)
alimentar doctest extract /tmp/cpython-lib-clean \
    -o cpython-doctests.parquet \
    --source cpython \
    --version "$CPYTHON_SHA"
```

### Why Filter Directories?

Some CPython directories contain intentional encoding edge cases:

| Directory | Issue | Solution |
|-----------|-------|----------|
| `test/` | Non-UTF-8 test files | Exclude |
| `idlelib/` | Legacy GUI encodings | Exclude |
| `turtledemo/` | Demo files | Exclude |

## Output Format

The corpus is stored in Apache Parquet format with this schema:

| Column | Type | Description |
|--------|------|-------------|
| `source_file` | string | Path within CPython Lib/ |
| `module` | string | Python module name |
| `function` | string | Function containing doctest |
| `input` | string | Python code to execute |
| `expected` | string | Expected output |
| `line_number` | int | Source line number |
| `source` | string | Corpus identifier ("cpython") |
| `version` | string | Git commit SHA |

### Example Record

```json
{
  "source_file": "collections/__init__.py",
  "module": "collections",
  "function": "namedtuple",
  "input": "Point = namedtuple('Point', ['x', 'y'])\nPoint(11, y=22)",
  "expected": "Point(x=11, y=22)",
  "line_number": 342,
  "source": "cpython",
  "version": "cfcd524"
}
```

## Inspecting Corpora

```bash
# Using alimentar
alimentar inspect cpython-doctests.parquet

# Using DuckDB
duckdb -c "SELECT module, COUNT(*) as count
           FROM 'cpython-doctests.parquet'
           GROUP BY module
           ORDER BY count DESC
           LIMIT 10"

# Using Python
python -c "
import pyarrow.parquet as pq
table = pq.read_table('cpython-doctests.parquet')
print(f'Records: {table.num_rows}')
print(table.schema)
"
```

## Corpus Statistics

CPython stdlib extraction yields approximately:

| Metric | Value |
|--------|-------|
| Total doctests | ~1,700 |
| Unique modules | ~90 |
| File size | ~300 KB (zstd) |

Top modules by doctest count:

| Module | Doctests |
|--------|----------|
| `collections` | 156 |
| `pathlib` | 89 |
| `datetime` | 78 |
| `re` | 67 |
| `json` | 45 |

## Integration with CITL Training

Once extracted, feed the corpus into the training loop:

```bash
# Train oracle from corpus
depyler oracle train \
    --corpus cpython-doctests.parquet \
    --min-samples 50

# Run CITL improvement loop
depyler oracle improve \
    --corpus cpython-doctests.parquet \
    --iterations 3

# Export patterns for downstream ML
depyler oracle export-oip \
    --output citl_patterns.jsonl
```

## Reproducibility

All extractions are fully reproducible:

| Aspect | Guarantee |
|--------|-----------|
| Source | Pinned by git commit SHA |
| Filter | Deterministic rsync excludes |
| Format | Parquet with zstd compression |
| Metadata | Source + version embedded |

The commit SHA is embedded in every record, enabling exact reproduction of any corpus.

## Related Resources

- [reprorusted-python-cli corpus docs](https://github.com/paiml/reprorusted-python-cli/blob/main/docs/corpus-extraction.md)
- [alimentar repository](https://github.com/paiml/alimentar)
- [Doctest Transpilation Spec](../../docs/specifications/doctest-transpilation-citl-spec.md)
- [Oracle ML Classification](./oracle.md)

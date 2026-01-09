# Converting Python to Optimized Rust Binaries - Examples & Specification

**Version**: 1.0.0
**Status**: Implementation - EXTREME TDD
**Last Updated**: 2025-11-11
**Owner**: Depyler Team
**Quality Standard**: PMAT-Enforced, TDG Grade A+ Target

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Objectives](#objectives)
3. [Example Suite Overview](#example-suite-overview)
4. [Example 1: ArgParse CLI Application](#example-1-argparse-cli-application)
5. [Example 2: Generator Pipeline (Text Processing)](#example-2-generator-pipeline-text-processing)
6. [Example 3: Multi-File Project (Module + ArgParse)](#example-3-multi-file-project-module--argparse)
7. [EXTREME TDD Implementation Protocol](#extreme-tdd-implementation-protocol)
8. [Optimization Strategies](#optimization-strategies)
9. [Benchmarking & Measurement](#benchmarking--measurement)
10. [Book Chapter Integration](#book-chapter-integration)

---

## Executive Summary

This specification defines **three comprehensive examples** demonstrating Python-to-Rust conversion using Depyler, with a focus on producing **highly optimized binaries**. Each example showcases real-world patterns and achieves significant performance improvements while maintaining correctness.

### Example Suite

| Example | Description | Key Features | Expected Speedup |
|---------|-------------|--------------|------------------|
| **ArgParse CLI** | Command-line tool with arg parsing | CLI patterns, string processing | 10-50x |
| **Generator Pipeline** | Text processing with generators | Lazy evaluation, streaming | 20-100x |
| **Multi-File Project** | Modular application with CLI | Code organization, imports | 10-50x |

### Success Criteria

- ✅ **Correctness**: Generated Rust produces identical output to Python
- ✅ **Performance**: 10-100x speedup over CPython
- ✅ **Binary Size**: <5MB for CLI tools
- ✅ **Compilation**: All examples compile with `--release` optimization
- ✅ **Test Coverage**: ≥85% for all generated Rust code
- ✅ **Quality**: TDG Grade A+ (score ≥95/100)

---

## Objectives

### Primary Objectives

1. **Demonstrate Real-World Patterns**
   - Common Python idioms (argparse, generators, modules)
   - Production-ready code generation
   - Best practices for CLI tools

2. **Achieve Maximum Performance**
   - Profile-guided optimization (PGO)
   - Link-time optimization (LTO)
   - CPU-specific tuning (target-cpu=native)
   - Binary stripping and compression

3. **Maintain Correctness**
   - Property-based testing
   - Cross-validation with Python
   - EXTREME TDD protocol

4. **Document Best Practices**
   - Optimization strategies
   - Trade-off analysis
   - Deployment patterns

### Secondary Objectives

1. **Educational Content**: Book chapter-quality documentation
2. **Reusable Templates**: Copy-paste examples for users
3. **Benchmarking Infrastructure**: Reproducible measurements
4. **Integration Testing**: End-to-end transpilation validation

---

## Example Suite Overview

### Directory Structure

```
examples/
├── argparse_cli/
│   ├── python/
│   │   └── wordcount.py          # Python source
│   ├── rust/
│   │   └── wordcount.rs           # Generated Rust
│   ├── tests/
│   │   ├── property_tests.rs      # QuickCheck tests
│   │   └── integration_tests.rs   # E2E tests
│   └── README.md
├── generator_pipeline/
│   ├── python/
│   │   └── text_processor.py      # Python with generators
│   ├── rust/
│   │   └── text_processor.rs      # Generated Rust
│   ├── tests/
│   │   ├── property_tests.rs
│   │   └── benchmark_tests.rs
│   └── README.md
├── multifile_project/
│   ├── python/
│   │   ├── main.py                # CLI entry point
│   │   ├── calculator/
│   │   │   ├── __init__.py
│   │   │   ├── operations.py
│   │   │   └── parser.py
│   │   └── utils.py
│   ├── rust/
│   │   ├── main.rs
│   │   ├── calculator/
│   │   │   ├── mod.rs
│   │   │   ├── operations.rs
│   │   │   └── parser.rs
│   │   └── utils.rs
│   ├── tests/
│   │   ├── property_tests.rs
│   │   ├── integration_tests.rs
│   │   └── module_tests.rs
│   └── README.md
└── README.md
```

---

## Example 1: ArgParse CLI Application

### Overview

A command-line word count tool that demonstrates:
- ArgParse CLI pattern conversion
- File I/O and string processing
- Statistics aggregation
- Error handling

### Python Source

```python
#!/usr/bin/env python3
"""
wordcount.py - Count words, lines, and characters in files
"""
import argparse
import sys
from pathlib import Path
from typing import NamedTuple

class Stats(NamedTuple):
    """Statistics for a file"""
    lines: int
    words: int
    chars: int
    filename: str

def count_file(filepath: Path) -> Stats:
    """Count statistics for a single file"""
    try:
        content = filepath.read_text()
        lines = len(content.splitlines())
        words = len(content.split())
        chars = len(content)
        return Stats(lines, words, chars, str(filepath))
    except IOError as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return Stats(0, 0, 0, str(filepath))

def format_stats(stats: Stats, show_filename: bool = True) -> str:
    """Format statistics for output"""
    result = f"{stats.lines:8} {stats.words:8} {stats.chars:8}"
    if show_filename:
        result += f" {stats.filename}"
    return result

def main() -> int:
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Count lines, words, and characters in files",
        epilog="Similar to wc(1) Unix command"
    )
    parser.add_argument(
        "files",
        nargs="+",
        type=Path,
        help="Files to process"
    )
    parser.add_argument(
        "-l", "--lines",
        action="store_true",
        help="Show only line count"
    )
    parser.add_argument(
        "-w", "--words",
        action="store_true",
        help="Show only word count"
    )
    parser.add_argument(
        "-c", "--chars",
        action="store_true",
        help="Show only character count"
    )

    args = parser.parse_args()

    total_lines = 0
    total_words = 0
    total_chars = 0

    for filepath in args.files:
        stats = count_file(filepath)
        total_lines += stats.lines
        total_words += stats.words
        total_chars += stats.chars

        if args.lines:
            print(f"{stats.lines:8} {stats.filename}")
        elif args.words:
            print(f"{stats.words:8} {stats.filename}")
        elif args.chars:
            print(f"{stats.chars:8} {stats.filename}")
        else:
            print(format_stats(stats))

    # Show totals if multiple files
    if len(args.files) > 1:
        total_stats = Stats(total_lines, total_words, total_chars, "total")
        print(format_stats(total_stats, show_filename=True))

    return 0

if __name__ == "__main__":
    sys.exit(main())
```

### Expected Rust Output Structure

```rust
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Debug, Clone)]
pub struct Stats {
    lines: usize,
    words: usize,
    chars: usize,
    filename: String,
}

#[derive(Parser)]
#[command(name = "wordcount")]
#[command(about = "Count lines, words, and characters in files")]
#[command(after_help = "Similar to wc(1) Unix command")]
struct Args {
    /// Files to process
    files: Vec<PathBuf>,

    /// Show only line count
    #[arg(short, long)]
    lines: bool,

    /// Show only word count
    #[arg(short, long)]
    words: bool,

    /// Show only character count
    #[arg(short, long)]
    chars: bool,
}

fn count_file(filepath: &Path) -> Stats {
    match fs::read_to_string(filepath) {
        Ok(content) => {
            let lines = content.lines().count();
            let words = content.split_whitespace().count();
            let chars = content.len();
            Stats {
                lines,
                words,
                chars,
                filename: filepath.display().to_string(),
            }
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", filepath.display(), e);
            Stats {
                lines: 0,
                words: 0,
                chars: 0,
                filename: filepath.display().to_string(),
            }
        }
    }
}

fn format_stats(stats: &Stats, show_filename: bool) -> String {
    let mut result = format!("{:8} {:8} {:8}", stats.lines, stats.words, stats.chars);
    if show_filename {
        result.push_str(&format!(" {}", stats.filename));
    }
    result
}

fn main() -> i32 {
    let args = Args::parse();

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;

    for filepath in &args.files {
        let stats = count_file(filepath);
        total_lines += stats.lines;
        total_words += stats.words;
        total_chars += stats.chars;

        if args.lines {
            println!("{:8} {}", stats.lines, stats.filename);
        } else if args.words {
            println!("{:8} {}", stats.words, stats.filename);
        } else if args.chars {
            println!("{:8} {}", stats.chars, stats.filename);
        } else {
            println!("{}", format_stats(&stats, true));
        }
    }

    if args.files.len() > 1 {
        let total_stats = Stats {
            lines: total_lines,
            words: total_words,
            chars: total_chars,
            filename: "total".to_string(),
        };
        println!("{}", format_stats(&total_stats, true));
    }

    0
}
```

### Optimization Configuration

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

[profile.release-pgo]
inherits = "release"
```

### Expected Performance

| Metric | Python | Rust (Debug) | Rust (Release) | Rust (PGO) | Improvement |
|--------|--------|--------------|----------------|------------|-------------|
| **Execution Time** | 50ms | 5ms | 2ms | 1.5ms | **33x** |
| **Binary Size** | N/A | 8MB | 2MB | 2.1MB | N/A |
| **Memory Usage** | 15MB | 2MB | 1.5MB | 1.5MB | **10x** |

---

## Example 2: Generator Pipeline (Text Processing)

### Overview

Text processing pipeline using generators that demonstrates:
- Generator/iterator pattern conversion
- Lazy evaluation and streaming
- Complex data transformations
- Memory efficiency

### Python Source

```python
#!/usr/bin/env python3
"""
text_processor.py - Process text files with generators
"""
import sys
from pathlib import Path
from typing import Iterator, Tuple

def read_lines(filepath: Path) -> Iterator[str]:
    """Read lines from a file lazily"""
    with open(filepath, 'r') as f:
        for line in f:
            yield line.rstrip('\n')

def filter_non_empty(lines: Iterator[str]) -> Iterator[str]:
    """Filter out empty lines"""
    for line in lines:
        if line.strip():
            yield line

def add_line_numbers(lines: Iterator[str]) -> Iterator[Tuple[int, str]]:
    """Add line numbers to each line"""
    for i, line in enumerate(lines, 1):
        yield (i, line)

def format_line(item: Tuple[int, str]) -> str:
    """Format a numbered line"""
    line_num, text = item
    return f"{line_num:6}: {text}"

def process_file(filepath: Path) -> Iterator[str]:
    """Complete processing pipeline"""
    lines = read_lines(filepath)
    non_empty = filter_non_empty(lines)
    numbered = add_line_numbers(non_empty)
    formatted = (format_line(item) for item in numbered)
    return formatted

def main() -> int:
    """Main entry point"""
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <file>", file=sys.stderr)
        return 1

    filepath = Path(sys.argv[1])
    if not filepath.exists():
        print(f"Error: File not found: {filepath}", file=sys.stderr)
        return 1

    # Process and output
    for line in process_file(filepath):
        print(line)

    return 0

if __name__ == "__main__":
    sys.exit(main())
```

### Expected Rust Output Structure

```rust
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

struct FilterNonEmpty<I> {
    inner: I,
}

impl<I: Iterator<Item = String>> Iterator for FilterNonEmpty<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(line) if !line.trim().is_empty() => return Some(line),
                Some(_) => continue,
                None => return None,
            }
        }
    }
}

fn read_lines(filepath: &Path) -> impl Iterator<Item = String> {
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("Failed to read line"))
}

fn filter_non_empty(lines: impl Iterator<Item = String>) -> FilterNonEmpty<impl Iterator<Item = String>> {
    FilterNonEmpty { inner: lines }
}

fn add_line_numbers(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (usize, String)> {
    lines.enumerate().map(|(i, line)| (i + 1, line))
}

fn format_line(item: (usize, String)) -> String {
    let (line_num, text) = item;
    format!("{:6}: {}", line_num, text)
}

fn process_file(filepath: &Path) -> impl Iterator<Item = String> {
    let lines = read_lines(filepath);
    let non_empty = filter_non_empty(lines);
    let numbered = add_line_numbers(non_empty);
    numbered.map(format_line)
}

fn main() -> i32 {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return 1;
    }

    let filepath = Path::new(&args[1]);
    if !filepath.exists() {
        eprintln!("Error: File not found: {:?}", filepath);
        return 1;
    }

    for line in process_file(filepath) {
        println!("{}", line);
    }

    0
}
```

### Expected Performance

| Metric | Python | Rust (Release) | Rust (PGO) | Improvement |
|--------|--------|----------------|------------|-------------|
| **100KB file** | 15ms | 2ms | 1ms | **15x** |
| **1MB file** | 150ms | 15ms | 8ms | **18x** |
| **10MB file** | 1.5s | 120ms | 80ms | **18x** |
| **Memory Usage** | 25MB | 500KB | 500KB | **50x** |

---

## Example 3: Multi-File Project (Module + ArgParse)

### Overview

A calculator CLI tool with multiple modules that demonstrates:
- Multi-file Python project structure
- Module imports and organization
- Complex business logic separation
- Integration with argparse

### Python Project Structure

```
calculator/
├── main.py
├── calculator/
│   ├── __init__.py
│   ├── operations.py
│   ├── parser.py
│   └── formatters.py
└── utils.py
```

### Python Source Files

**main.py**:
```python
#!/usr/bin/env python3
"""
Calculator CLI - Evaluate mathematical expressions
"""
import argparse
import sys
from calculator.parser import parse_expression
from calculator.operations import evaluate
from calculator.formatters import format_result
from utils import read_expressions_from_file

def main() -> int:
    parser = argparse.ArgumentParser(description="Evaluate mathematical expressions")
    parser.add_argument("expression", nargs="?", help="Expression to evaluate")
    parser.add_argument("-f", "--file", type=str, help="Read expressions from file")
    parser.add_argument("-v", "--verbose", action="store_true", help="Verbose output")

    args = parser.parse_args()

    if args.file:
        expressions = read_expressions_from_file(args.file)
    elif args.expression:
        expressions = [args.expression]
    else:
        parser.print_help()
        return 1

    for expr in expressions:
        try:
            ast = parse_expression(expr)
            result = evaluate(ast)
            output = format_result(result, verbose=args.verbose)
            print(output)
        except Exception as e:
            print(f"Error: {e}", file=sys.stderr)
            return 1

    return 0

if __name__ == "__main__":
    sys.exit(main())
```

**calculator/operations.py**:
```python
"""
Mathematical operations
"""
from typing import Union

Number = Union[int, float]

def add(a: Number, b: Number) -> Number:
    return a + b

def subtract(a: Number, b: Number) -> Number:
    return a - b

def multiply(a: Number, b: Number) -> Number:
    return a * b

def divide(a: Number, b: Number) -> Number:
    if b == 0:
        raise ValueError("Division by zero")
    return a / b

def evaluate(ast: dict) -> Number:
    """Evaluate an AST node"""
    if ast["type"] == "number":
        return ast["value"]
    elif ast["type"] == "binop":
        left = evaluate(ast["left"])
        right = evaluate(ast["right"])
        op = ast["op"]

        if op == "+":
            return add(left, right)
        elif op == "-":
            return subtract(left, right)
        elif op == "*":
            return multiply(left, right)
        elif op == "/":
            return divide(left, right)
        else:
            raise ValueError(f"Unknown operator: {op}")
    else:
        raise ValueError(f"Unknown AST node type: {ast['type']}")
```

**calculator/parser.py**:
```python
"""
Expression parser
"""
import re
from typing import Dict, Any

def parse_expression(expr: str) -> Dict[str, Any]:
    """Parse a mathematical expression into an AST"""
    expr = expr.strip()

    # Simple recursive descent parser for basic arithmetic
    # Supports: numbers, +, -, *, /, parentheses

    def parse_number(s: str) -> Dict[str, Any]:
        match = re.match(r'^-?\d+(?:\.\d+)?', s)
        if match:
            value_str = match.group(0)
            value = float(value_str) if '.' in value_str else int(value_str)
            return {"type": "number", "value": value}, s[len(value_str):]
        raise ValueError(f"Expected number, got: {s}")

    def parse_expr(s: str) -> Dict[str, Any]:
        # Simplified parser - just handles basic binary ops
        left, s = parse_number(s)
        s = s.lstrip()

        if not s:
            return left, s

        if s[0] in "+-*/":
            op = s[0]
            s = s[1:].lstrip()
            right, s = parse_number(s)
            return {"type": "binop", "op": op, "left": left, "right": right}, s

        return left, s

    ast, remaining = parse_expr(expr)
    if remaining.strip():
        raise ValueError(f"Unexpected characters: {remaining}")

    return ast
```

**calculator/formatters.py**:
```python
"""
Output formatters
"""
from typing import Union

def format_result(value: Union[int, float], verbose: bool = False) -> str:
    """Format calculation result"""
    if verbose:
        return f"Result: {value}"
    else:
        return str(value)
```

**utils.py**:
```python
"""
Utility functions
"""
from pathlib import Path
from typing import List

def read_expressions_from_file(filepath: str) -> List[str]:
    """Read expressions from a file, one per line"""
    path = Path(filepath)
    if not path.exists():
        raise FileNotFoundError(f"File not found: {filepath}")

    with open(path, 'r') as f:
        return [line.strip() for line in f if line.strip()]
```

### Expected Rust Project Structure

```
src/
├── main.rs
├── calculator/
│   ├── mod.rs
│   ├── operations.rs
│   ├── parser.rs
│   └── formatters.rs
└── utils.rs
```

### Expected Performance

| Metric | Python | Rust (Release) | Improvement |
|--------|--------|----------------|-------------|
| **Single Expression** | 25ms | 1ms | **25x** |
| **100 Expressions** | 250ms | 15ms | **16x** |
| **Startup Time** | 80ms | 1ms | **80x** |
| **Binary Size** | N/A | 1.5MB | N/A |

---

## EXTREME TDD Implementation Protocol

### RED-GREEN-REFACTOR Cycle

**Phase 1: RED** - Write Failing Tests
```bash
# Create test file first
cargo test test_wordcount_basic  # MUST FAIL

# Commit RED phase
git commit -m "[RED] DEPYLER-XXXX: Add failing test for wordcount"
```

**Phase 2: GREEN** - Minimal Implementation
```bash
# Implement just enough to pass
cargo test test_wordcount_basic  # MUST PASS

# Commit GREEN phase
git commit -m "[GREEN] DEPYLER-XXXX: Implement wordcount basic functionality"
```

**Phase 3: REFACTOR** - Meet Quality Standards
```bash
# Refactor to meet quality gates
pmat analyze tdg --path examples --threshold 2.0
pmat analyze complexity --max-cyclomatic 10
cargo clippy -- -D warnings

# Commit REFACTOR phase
git commit -m "[REFACTOR] DEPYLER-XXXX: Meet quality standards for wordcount"
```

### Quality Gates (MANDATORY - BLOCKING)

1. **Compilation**: `cargo build --release` - MUST succeed
2. **Tests**: `cargo test --release` - 100% pass rate
3. **Clippy**: `cargo clippy -- -D warnings` - Zero warnings
4. **TDG**: `pmat analyze tdg --threshold 2.0` - All files ≤2.0
5. **Complexity**: `pmat analyze complexity --max-cyclomatic 10` - All functions ≤10
6. **Coverage**: `cargo llvm-cov --fail-under-lines 85` - ≥85% coverage

### Property-Based Testing Requirements

Each example MUST have:
- ≥5 property tests using QuickCheck
- ≥3 integration tests
- ≥2 benchmark tests
- Cross-validation with Python (output matches)

---

## Optimization Strategies

### Compiler Flags Matrix

| Flag | Value | Binary Size Impact | Speed Impact |
|------|-------|-------------------|--------------|
| `opt-level` | 3 | +10% | +30% |
| `lto` | "fat" | -20% | +15% |
| `codegen-units` | 1 | -5% | +10% |
| `panic` | "abort" | -30% | +5% |
| `strip` | true | -80% | 0% |

### PGO Workflow

```bash
# 1. Build instrumented binary
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

# 2. Run typical workload
./target/release/wordcount large_file.txt

# 3. Rebuild with profiling data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data" cargo build --release

# Expected: 10-30% additional speedup
```

### CPU-Specific Tuning

```bash
# Native CPU optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Specific features
RUSTFLAGS="-C target-feature=+avx2,+fma" cargo build --release

# Expected: 5-15% speedup on supported hardware
```

---

## Scientific Benchmarking & Measurement Framework

### Overview

All examples MUST include comprehensive scientific benchmarking with:
- **Runtime Performance**: Execution time with statistical significance
- **CPU Utilization**: CPU cycles, cache misses, branch mispredictions
- **Memory Usage**: RSS, heap allocations, stack depth
- **Binary Metrics**: Size, startup time, cold/warm cache performance
- **Visualization**: Charts and graphs for all metrics
- **100% Test Coverage**: Both Python and Rust versions

### 🚨 ANTI-HALLUCINATION: PROGRAMMATIC MEASUREMENT ONLY

**ABSOLUTE REQUIREMENT**: ALL metrics MUST be programmatically generated. ZERO tolerance for manual entry or hallucinated numbers.

#### Enforcement Rules

1. **NO MANUAL METRICS** ❌
   - Never type performance numbers directly into documentation
   - Never estimate or guess benchmark results
   - Never copy numbers from previous runs without verification
   - Never use placeholder values like "~50ms" or "approximately 10x"

2. **PROGRAMMATIC GENERATION ONLY** ✅
   - All metrics captured by automation scripts
   - Output in machine-readable formats (JSON, CSV, YAML)
   - Scripts check exit codes and validate tool execution
   - Results include timestamps and environment metadata

3. **REPRODUCIBILITY MANDATORY** ✅
   - `make benchmark` regenerates ALL metrics from scratch
   - Scripts output to `benchmarks/results/<timestamp>/` directories
   - Environment captured: CPU model, RAM, kernel version, compiler versions
   - Seed values recorded for property tests

4. **VERIFICATION GATES** ✅
   - CI/CD runs benchmarks and fails if tools not available
   - Pre-commit hook validates benchmark JSON schemas
   - Documentation build fails if metrics files missing
   - Visualization scripts error if data files not found

#### Implementation Template

```bash
#!/usr/bin/env bash
# benchmarks/run_all.sh - Programmatic benchmark automation

set -euo pipefail

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="benchmarks/results/${TIMESTAMP}"
mkdir -p "${RESULTS_DIR}"

# Capture environment metadata
cat > "${RESULTS_DIR}/environment.json" <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "hostname": "$(hostname)",
  "cpu_model": "$(lscpu | grep 'Model name' | cut -d: -f2 | xargs)",
  "cpu_cores": "$(nproc)",
  "ram_gb": "$(free -g | awk '/^Mem:/{print $2}')",
  "kernel": "$(uname -r)",
  "rustc_version": "$(rustc --version)",
  "python_version": "$(python3 --version)",
  "cargo_version": "$(cargo --version)"
}
EOF

# Runtime benchmarking (hyperfine)
echo "==> Running hyperfine benchmarks..."
hyperfine \
    --warmup 3 \
    --min-runs 50 \
    --export-json "${RESULTS_DIR}/runtime.json" \
    --export-markdown "${RESULTS_DIR}/runtime.md" \
    'python3 examples/argparse_cli/python/wordcount.py examples/argparse_cli/testdata/sample.txt' \
    './target/release/examples/wordcount examples/argparse_cli/testdata/sample.txt' \
    || { echo "ERROR: hyperfine failed"; exit 1; }

# Memory profiling (valgrind massif)
echo "==> Running memory profiling..."
valgrind --tool=massif \
    --massif-out-file="${RESULTS_DIR}/memory_rust.massif" \
    ./target/release/examples/wordcount examples/argparse_cli/testdata/sample.txt \
    2>&1 | tee "${RESULTS_DIR}/memory_rust.log" \
    || { echo "ERROR: valgrind failed"; exit 1; }

# Parse massif output to JSON
ms_print "${RESULTS_DIR}/memory_rust.massif" | \
    awk '/peak/ {print $6}' > "${RESULTS_DIR}/memory_rust_peak_mb.txt"

# CPU profiling (perf)
echo "==> Running CPU profiling..."
perf record -F 99 -g -o "${RESULTS_DIR}/perf_rust.data" \
    ./target/release/examples/wordcount examples/argparse_cli/testdata/sample.txt \
    || { echo "ERROR: perf failed"; exit 1; }

perf report -i "${RESULTS_DIR}/perf_rust.data" --stdio \
    > "${RESULTS_DIR}/perf_rust_report.txt"

# Binary metrics
echo "==> Collecting binary metrics..."
cat > "${RESULTS_DIR}/binary_metrics.json" <<EOF
{
  "size_bytes": $(stat -c%s ./target/release/examples/wordcount),
  "size_mb": $(echo "scale=2; $(stat -c%s ./target/release/examples/wordcount) / 1024 / 1024" | bc),
  "stripped_size_bytes": $(strip --strip-all -o /tmp/wordcount_stripped ./target/release/examples/wordcount && stat -c%s /tmp/wordcount_stripped && rm /tmp/wordcount_stripped)
}
EOF

# Test coverage (must be 100%)
echo "==> Running test coverage..."
cargo llvm-cov --json --output-path "${RESULTS_DIR}/coverage_rust.json" \
    --all-features --workspace \
    || { echo "ERROR: Coverage collection failed"; exit 1; }

pytest --cov=wordcount \
    --cov-report=json:"${RESULTS_DIR}/coverage_python.json" \
    examples/argparse_cli/tests/ \
    || { echo "ERROR: Python tests failed"; exit 1; }

# Validation: Ensure all required files exist
REQUIRED_FILES=(
    "environment.json"
    "runtime.json"
    "runtime.md"
    "memory_rust.massif"
    "perf_rust.data"
    "binary_metrics.json"
    "coverage_rust.json"
    "coverage_python.json"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [[ ! -f "${RESULTS_DIR}/${file}" ]]; then
        echo "ERROR: Required file missing: ${file}"
        exit 1
    fi
done

echo "==> SUCCESS: All benchmarks completed"
echo "Results directory: ${RESULTS_DIR}"
echo "Generate visualizations with: python3 benchmarks/visualize.py ${RESULTS_DIR}"
```

#### Visualization Template

```python
#!/usr/bin/env python3
"""
benchmarks/visualize.py - Programmatic chart generation from benchmark data
"""
import json
import sys
from pathlib import Path
import matplotlib.pyplot as plt
import seaborn as sns

def load_benchmark_data(results_dir: Path) -> dict:
    """Load all benchmark JSON files"""
    data = {}

    # Runtime data (hyperfine)
    with open(results_dir / "runtime.json") as f:
        runtime = json.load(f)
        data["runtime"] = runtime

    # Memory data
    with open(results_dir / "memory_rust_peak_mb.txt") as f:
        data["memory_peak_mb"] = float(f.read().strip())

    # Binary metrics
    with open(results_dir / "binary_metrics.json") as f:
        data["binary"] = json.load(f)

    # Coverage
    with open(results_dir / "coverage_rust.json") as f:
        data["coverage_rust"] = json.load(f)

    with open(results_dir / "coverage_python.json") as f:
        data["coverage_python"] = json.load(f)

    return data

def generate_charts(data: dict, output_dir: Path):
    """Generate all visualization charts"""
    sns.set_style("whitegrid")

    # Chart 1: Runtime comparison
    fig, ax = plt.subplots(figsize=(10, 6))

    python_time = data["runtime"]["results"][0]["mean"]
    rust_time = data["runtime"]["results"][1]["mean"]

    bars = ax.bar(["Python", "Rust"], [python_time * 1000, rust_time * 1000])
    bars[0].set_color("#3776ab")  # Python blue
    bars[1].set_color("#ce422b")  # Rust orange

    ax.set_ylabel("Execution Time (ms)")
    ax.set_title("Runtime Performance: Python vs Rust")

    # Add speedup annotation
    speedup = python_time / rust_time
    ax.text(0.5, max(python_time, rust_time) * 500,
            f"{speedup:.1f}x faster",
            ha='center', fontsize=14, fontweight='bold')

    plt.tight_layout()
    plt.savefig(output_dir / "runtime_comparison.png", dpi=300)
    plt.close()

    # Chart 2: Memory usage
    # Chart 3: Binary size
    # Chart 4: Test coverage
    # Chart 5: Speedup across file sizes
    # Chart 6: CPU profiling flamegraph

    print(f"✅ Generated charts in {output_dir}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 visualize.py <results_dir>")
        sys.exit(1)

    results_dir = Path(sys.argv[1])
    if not results_dir.exists():
        print(f"ERROR: Results directory not found: {results_dir}")
        sys.exit(1)

    data = load_benchmark_data(results_dir)
    generate_charts(data, results_dir)
```

#### Makefile Integration

```makefile
# examples/Makefile

.PHONY: benchmark benchmark-ci verify-benchmark-tools

# Check that all benchmark tools are installed
verify-benchmark-tools:
	@command -v hyperfine >/dev/null || { echo "ERROR: hyperfine not installed"; exit 1; }
	@command -v valgrind >/dev/null || { echo "ERROR: valgrind not installed"; exit 1; }
	@command -v perf >/dev/null || { echo "ERROR: perf not installed"; exit 1; }
	@command -v cargo-llvm-cov >/dev/null || { echo "ERROR: cargo-llvm-cov not installed"; exit 1; }
	@python3 -c "import matplotlib, seaborn" || { echo "ERROR: Python visualization libs not installed"; exit 1; }

# Run all benchmarks programmatically
benchmark: verify-benchmark-tools
	@echo "==> Building release binaries..."
	cargo build --release --examples
	@echo "==> Running programmatic benchmarks..."
	./benchmarks/run_all.sh
	@echo "==> Generating visualizations..."
	python3 benchmarks/visualize.py benchmarks/results/latest

# CI/CD integration (fails if tools unavailable)
benchmark-ci: verify-benchmark-tools benchmark
	@echo "==> Validating benchmark results..."
	@test -f benchmarks/results/latest/runtime.json || { echo "ERROR: runtime.json missing"; exit 1; }
	@test -f benchmarks/results/latest/coverage_rust.json || { echo "ERROR: coverage missing"; exit 1; }
	@echo "✅ All benchmark validations passed"
```

#### Anti-Hallucination Checklist

Before committing ANY performance claims:

- [ ] ✅ Ran `make benchmark` successfully
- [ ] ✅ Verified `benchmarks/results/<timestamp>/` contains JSON files
- [ ] ✅ Checked `environment.json` matches current system
- [ ] ✅ All charts generated from data files (not manual creation)
- [ ] ✅ Numbers in documentation pulled from JSON programmatically
- [ ] ✅ Cross-validation script confirms Rust = Python output
- [ ] ✅ CI/CD pipeline re-runs benchmarks (reproducibility check)
- [ ] ❌ NO manual typing of performance numbers
- [ ] ❌ NO "approximately" or "around" wording
- [ ] ❌ NO placeholder values waiting to be "filled in later"

**SACRED RULE**: If you can't run the benchmark tool and capture JSON output, you CANNOT claim the metric exists.

### Benchmark Harness (Criterion.rs)

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::path::Path;

fn bench_wordcount(c: &mut Criterion) {
    let test_files = vec![
        ("10KB", "testdata/sample_10kb.txt"),
        ("100KB", "testdata/sample_100kb.txt"),
        ("1MB", "testdata/sample_1mb.txt"),
        ("10MB", "testdata/sample_10mb.txt"),
    ];

    let mut group = c.benchmark_group("wordcount");

    for (size, path) in test_files {
        group.bench_with_input(BenchmarkId::new("count_file", size), &path, |b, &path| {
            b.iter(|| {
                let stats = count_file(black_box(Path::new(path)));
                black_box(stats);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_wordcount);
criterion_main!(benches);
```

### CPU Profiling (perf + flamegraph)

```bash
# Profile Rust binary with perf
perf record -F 99 -g ./target/release/wordcount testdata/large.txt
perf script | stackcollapse-perf.pl | flamegraph.pl > wordcount_rust.svg

# Profile Python with py-spy
py-spy record -o wordcount_python.svg -- python3 wordcount.py testdata/large.txt

# Compare CPU hotspots
# Expected: Rust shows optimized tight loops, Python shows interpreter overhead
```

### Memory Profiling (heaptrack + valgrind)

```bash
# Rust memory profiling
valgrind --tool=massif --massif-out-file=wordcount_rust.massif \
    ./target/release/wordcount testdata/large.txt
ms_print wordcount_rust.massif > wordcount_rust_memory.txt

# Python memory profiling
python3 -m memory_profiler wordcount.py testdata/large.txt > wordcount_python_memory.txt

# Heaptrack for detailed allocation tracking (Rust)
heaptrack ./target/release/wordcount testdata/large.txt
heaptrack_gui heaptrack.wordcount.*.gz

# Expected metrics:
# - Rust: ~1-2MB peak RSS
# - Python: ~15-25MB peak RSS
# - Rust: Zero allocations in hot path (after warmup)
# - Python: Constant allocation/deallocation
```

### Runtime Benchmarking (hyperfine)

```bash
# Statistical benchmarking with hyperfine
hyperfine \
    --warmup 3 \
    --min-runs 50 \
    --export-json benchmark_results.json \
    --export-markdown benchmark_results.md \
    'python3 wordcount.py testdata/large.txt' \
    './target/release/wordcount testdata/large.txt'

# Expected output:
# Benchmark 1: python3 wordcount.py testdata/large.txt
#   Time (mean ± σ):      52.3 ms ±   2.1 ms    [User: 45.1 ms, System: 6.8 ms]
#   Range (min … max):    49.1 ms …  58.4 ms    50 runs
#
# Benchmark 2: ./target/release/wordcount testdata/large.txt
#   Time (mean ± σ):       1.8 ms ±   0.2 ms    [User: 1.2 ms, System: 0.5 ms]
#   Range (min … max):     1.5 ms …   2.3 ms    50 runs
#
# Summary
#   './target/release/wordcount testdata/large.txt' ran
#     29.06 ± 3.15 times faster than 'python3 wordcount.py testdata/large.txt'
```

### System-Level Metrics (pidstat)

```bash
# Monitor CPU, memory, I/O during execution
pidstat -u -r -d 1 -C wordcount > wordcount_rust_stats.txt &
PIDSTAT_PID=$!

# Run workload
./target/release/wordcount testdata/large.txt

kill $PIDSTAT_PID

# Parse and visualize results
# Expected metrics:
# - CPU: 95-100% utilization (single-threaded)
# - Memory: Stable, no growth
# - I/O: Minimal page faults
```

### Cross-Validation Script

```python
#!/usr/bin/env python3
"""
Validate that Rust output matches Python output
"""
import subprocess
import sys

def run_python(args):
    result = subprocess.run(
        ["python3", "wordcount.py"] + args,
        capture_output=True,
        text=True
    )
    return result.stdout, result.returncode

def run_rust(args):
    result = subprocess.run(
        ["./target/release/wordcount"] + args,
        capture_output=True,
        text=True
    )
    return result.stdout, result.returncode

def main():
    test_cases = [
        ["testdata/sample.txt"],
        ["testdata/sample.txt", "-l"],
        ["testdata/sample.txt", "-w"],
        ["testdata/sample.txt", "-c"],
    ]

    for args in test_cases:
        py_out, py_code = run_python(args)
        rs_out, rs_code = run_rust(args)

        if py_out != rs_out or py_code != rs_code:
            print(f"MISMATCH for args: {args}")
            print(f"Python output: {py_out}")
            print(f"Rust output: {rs_out}")
            return 1

    print("✅ All test cases match!")
    return 0

if __name__ == "__main__":
    sys.exit(main())
```

---

## 100% Test Coverage Requirements

### Python Test Coverage (pytest + coverage.py)

**MANDATORY**: 100% line coverage, 100% branch coverage for Python source.

```bash
# Install coverage tools
pip install pytest pytest-cov coverage

# Run tests with coverage
pytest --cov=wordcount --cov-report=html --cov-report=term --cov-branch

# Expected output:
# Name              Stmts   Miss Branch BrPart  Cover
# -----------------------------------------------------
# wordcount.py         45      0     12      0   100%
# -----------------------------------------------------
# TOTAL                45      0     12      0   100%

# Generate HTML report
coverage html
# Open htmlcov/index.html to view detailed coverage

# Fail if coverage below 100%
pytest --cov=wordcount --cov-fail-under=100
```

### Python Test Suite Structure

```python
# tests/test_wordcount.py
import pytest
from pathlib import Path
from wordcount import count_file, format_stats, Stats, main
import tempfile
import sys
from io import StringIO

class TestCountFile:
    """Test count_file function - 100% coverage required"""

    def test_count_empty_file(self, tmp_path):
        """Test empty file"""
        f = tmp_path / "empty.txt"
        f.write_text("")
        stats = count_file(f)
        assert stats.lines == 0
        assert stats.words == 0
        assert stats.chars == 0

    def test_count_single_line(self, tmp_path):
        """Test single line file"""
        f = tmp_path / "single.txt"
        f.write_text("hello world")
        stats = count_file(f)
        assert stats.lines == 1
        assert stats.words == 2
        assert stats.chars == 11

    def test_count_multiline(self, tmp_path):
        """Test multiline file"""
        f = tmp_path / "multi.txt"
        f.write_text("line one\nline two\nline three")
        stats = count_file(f)
        assert stats.lines == 3
        assert stats.words == 6
        assert stats.chars == 28

    def test_count_nonexistent_file(self):
        """Test error handling for missing file"""
        f = Path("/nonexistent/file.txt")
        stats = count_file(f)
        assert stats.lines == 0
        assert stats.words == 0
        assert stats.chars == 0

    def test_count_permission_denied(self, tmp_path):
        """Test error handling for permission denied"""
        f = tmp_path / "protected.txt"
        f.write_text("test")
        f.chmod(0o000)
        try:
            stats = count_file(f)
            assert stats.lines == 0
        finally:
            f.chmod(0o644)

class TestFormatStats:
    """Test format_stats function - 100% coverage required"""

    def test_format_with_filename(self):
        stats = Stats(10, 20, 30, "test.txt")
        result = format_stats(stats, show_filename=True)
        assert "10" in result
        assert "20" in result
        assert "30" in result
        assert "test.txt" in result

    def test_format_without_filename(self):
        stats = Stats(10, 20, 30, "test.txt")
        result = format_stats(stats, show_filename=False)
        assert "10" in result
        assert "20" in result
        assert "30" in result
        assert "test.txt" not in result

class TestMain:
    """Test main function - 100% coverage required"""

    def test_main_single_file(self, tmp_path, monkeypatch, capsys):
        """Test single file processing"""
        f = tmp_path / "test.txt"
        f.write_text("hello world")
        monkeypatch.setattr(sys, 'argv', ['wordcount', str(f)])
        result = main()
        assert result == 0
        captured = capsys.readouterr()
        assert "1" in captured.out
        assert "2" in captured.out

    def test_main_lines_only(self, tmp_path, monkeypatch, capsys):
        """Test --lines flag"""
        f = tmp_path / "test.txt"
        f.write_text("line1\nline2")
        monkeypatch.setattr(sys, 'argv', ['wordcount', str(f), '-l'])
        result = main()
        assert result == 0

    def test_main_words_only(self, tmp_path, monkeypatch, capsys):
        """Test --words flag"""
        f = tmp_path / "test.txt"
        f.write_text("word1 word2")
        monkeypatch.setattr(sys, 'argv', ['wordcount', str(f), '-w'])
        result = main()
        assert result == 0

    def test_main_chars_only(self, tmp_path, monkeypatch, capsys):
        """Test --chars flag"""
        f = tmp_path / "test.txt"
        f.write_text("test")
        monkeypatch.setattr(sys, 'argv', ['wordcount', str(f), '-c'])
        result = main()
        assert result == 0

    def test_main_multiple_files(self, tmp_path, monkeypatch, capsys):
        """Test multiple file processing with totals"""
        f1 = tmp_path / "test1.txt"
        f2 = tmp_path / "test2.txt"
        f1.write_text("file one")
        f2.write_text("file two")
        monkeypatch.setattr(sys, 'argv', ['wordcount', str(f1), str(f2)])
        result = main()
        assert result == 0
        captured = capsys.readouterr()
        assert "total" in captured.out.lower()

# Property-based tests with Hypothesis
from hypothesis import given, strategies as st

@given(st.text(min_size=0, max_size=1000))
def test_count_file_properties(tmp_path, text):
    """Property-based test: verify counting properties"""
    f = tmp_path / "prop_test.txt"
    f.write_text(text)
    stats = count_file(f)

    # Properties that must always hold
    assert stats.lines >= 0
    assert stats.words >= 0
    assert stats.chars == len(text)
    assert stats.lines <= len(text.splitlines())
```

### Rust Test Coverage (cargo-llvm-cov + tarpaulin)

**MANDATORY**: 100% line coverage, 100% branch coverage for Rust source.

```bash
# Install coverage tools
cargo install cargo-llvm-cov cargo-tarpaulin

# Run tests with llvm-cov
cargo llvm-cov --all-features --workspace --html --fail-under-lines 100 --fail-under-functions 100

# Expected output:
# Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
# -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
# wordcount.rs                      42                 0   100.00%          8                 0   100.00%         156                 0   100.00%          24                 0   100.00%
# -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
# TOTALS                            42                 0   100.00%          8                 0   100.00%         156                 0   100.00%          24                 0   100.00%

# Alternative: Use tarpaulin
cargo tarpaulin --all-features --workspace --out Html --out Lcov --fail-under 100

# Generate coverage badge
cargo tarpaulin --all-features --out Xml
# Use coverage badge generator with coverage.xml
```

### Rust Test Suite Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Unit tests - 100% function coverage

    #[test]
    fn test_count_empty_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("empty.txt");
        fs::write(&path, "").unwrap();
        let stats = count_file(&path);
        assert_eq!(stats.lines, 0);
        assert_eq!(stats.words, 0);
        assert_eq!(stats.chars, 0);
    }

    #[test]
    fn test_count_single_line() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("single.txt");
        fs::write(&path, "hello world").unwrap();
        let stats = count_file(&path);
        assert_eq!(stats.lines, 1);
        assert_eq!(stats.words, 2);
        assert_eq!(stats.chars, 11);
    }

    #[test]
    fn test_count_multiline() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("multi.txt");
        fs::write(&path, "line one\nline two\nline three").unwrap();
        let stats = count_file(&path);
        assert_eq!(stats.lines, 3);
        assert_eq!(stats.words, 6);
    }

    #[test]
    fn test_count_nonexistent_file() {
        let path = Path::new("/nonexistent/file.txt");
        let stats = count_file(path);
        assert_eq!(stats.lines, 0);
        assert_eq!(stats.words, 0);
        assert_eq!(stats.chars, 0);
    }

    #[test]
    fn test_format_with_filename() {
        let stats = Stats {
            lines: 10,
            words: 20,
            chars: 30,
            filename: "test.txt".to_string(),
        };
        let result = format_stats(&stats, true);
        assert!(result.contains("10"));
        assert!(result.contains("20"));
        assert!(result.contains("30"));
        assert!(result.contains("test.txt"));
    }

    #[test]
    fn test_format_without_filename() {
        let stats = Stats {
            lines: 10,
            words: 20,
            chars: 30,
            filename: "test.txt".to_string(),
        };
        let result = format_stats(&stats, false);
        assert!(result.contains("10"));
        assert!(result.contains("20"));
        assert!(result.contains("30"));
        assert!(!result.contains("test.txt"));
    }

    // Property-based tests with QuickCheck - 100% branch coverage

    use quickcheck::{quickcheck, TestResult};

    quickcheck! {
        fn prop_count_chars_matches_length(text: String) -> TestResult {
            let dir = TempDir::new().unwrap();
            let path = dir.path().join("prop.txt");
            fs::write(&path, &text).unwrap();
            let stats = count_file(&path);
            TestResult::from_bool(stats.chars == text.len())
        }

        fn prop_lines_never_negative(text: String) -> TestResult {
            let dir = TempDir::new().unwrap();
            let path = dir.path().join("prop.txt");
            fs::write(&path, &text).unwrap();
            let stats = count_file(&path);
            TestResult::from_bool(stats.lines >= 0)
        }

        fn prop_words_never_negative(text: String) -> TestResult {
            let dir = TempDir::new().unwrap();
            let path = dir.path().join("prop.txt");
            fs::write(&path, &text).unwrap();
            let stats = count_file(&path);
            TestResult::from_bool(stats.words >= 0)
        }
    }

    // Integration tests
    #[test]
    fn test_cli_single_file() {
        // Test complete CLI flow
    }

    #[test]
    fn test_cli_multiple_files() {
        // Test multiple files with totals
    }

    #[test]
    fn test_cli_flags() {
        // Test -l, -w, -c flags
    }
}
```

---

## Visualization & Charting Requirements

All benchmark results MUST be visualized with publication-quality charts.

### Chart Types Required

1. **Runtime Comparison Bar Chart**
2. **Memory Usage Line Chart**
3. **CPU Utilization Heatmap**
4. **Speedup Factor Chart**
5. **Binary Size Comparison**
6. **Flamegraph Comparisons**

### Visualization Script (Python + matplotlib/plotly)

```python
#!/usr/bin/env python3
"""
Generate benchmark visualization charts
"""
import json
import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd
from pathlib import Path

# Set publication-quality style
sns.set_theme(style="whitegrid", context="paper", font_scale=1.2)
plt.rcParams['figure.dpi'] = 300
plt.rcParams['savefig.dpi'] = 300

def load_benchmark_data(json_path):
    """Load hyperfine benchmark results"""
    with open(json_path) as f:
        return json.load(f)

def plot_runtime_comparison(data, output_path):
    """Bar chart: Python vs Rust runtime"""
    results = data['results']
    names = [r['command'].split()[0] for r in results]
    means = [r['mean'] * 1000 for r in results]  # Convert to ms
    stds = [r['stddev'] * 1000 for r in results]

    fig, ax = plt.subplots(figsize=(10, 6))
    bars = ax.bar(names, means, yerr=stds, capsize=10, alpha=0.8, color=['#3776ab', '#ce412b'])

    ax.set_ylabel('Execution Time (ms)')
    ax.set_title('Runtime Comparison: Python vs Rust')
    ax.set_yscale('log')
    ax.grid(True, alpha=0.3)

    # Add value labels
    for bar in bars:
        height = bar.get_height()
        ax.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.2f}ms',
                ha='center', va='bottom')

    plt.tight_layout()
    plt.savefig(output_path, bbox_inches='tight')
    print(f"✅ Saved: {output_path}")

def plot_speedup_factor(data, output_path):
    """Calculate and visualize speedup factor"""
    results = data['results']
    python_time = results[0]['mean']
    rust_time = results[1]['mean']
    speedup = python_time / rust_time

    fig, ax = plt.subplots(figsize=(8, 6))
    ax.barh(['Speedup'], [speedup], color='#2ecc71', alpha=0.8)
    ax.set_xlabel('Speedup Factor (x faster)')
    ax.set_title(f'Rust is {speedup:.1f}x faster than Python')
    ax.axvline(x=1, color='red', linestyle='--', label='Baseline (1x)')
    ax.legend()
    ax.grid(True, alpha=0.3)

    # Add value label
    ax.text(speedup/2, 0, f'{speedup:.1f}x', ha='center', va='center',
            fontsize=20, fontweight='bold', color='white')

    plt.tight_layout()
    plt.savefig(output_path, bbox_inches='tight')
    print(f"✅ Saved: {output_path}")

def plot_memory_usage(memory_data, output_path):
    """Line chart: Memory usage over time"""
    df = pd.DataFrame(memory_data)

    fig, ax = plt.subplots(figsize=(12, 6))
    ax.plot(df['time'], df['python_rss_mb'], label='Python RSS', marker='o', linewidth=2)
    ax.plot(df['time'], df['rust_rss_mb'], label='Rust RSS', marker='s', linewidth=2)

    ax.set_xlabel('Time (seconds)')
    ax.set_ylabel('Resident Set Size (MB)')
    ax.set_title('Memory Usage Comparison Over Time')
    ax.legend()
    ax.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output_path, bbox_inches='tight')
    print(f"✅ Saved: {output_path}")

def plot_cpu_utilization(cpu_data, output_path):
    """Heatmap: CPU utilization"""
    df = pd.DataFrame(cpu_data)

    fig, ax = plt.subplots(figsize=(12, 4))
    sns.heatmap(df, annot=True, fmt=".1f", cmap="YlOrRd",
                cbar_kws={'label': 'CPU %'}, ax=ax)

    ax.set_title('CPU Utilization by Core')
    ax.set_xlabel('Time Window')
    ax.set_ylabel('Core ID')

    plt.tight_layout()
    plt.savefig(output_path, bbox_inches='tight')
    print(f"✅ Saved: {output_path}")

def plot_binary_size_comparison(sizes, output_path):
    """Bar chart: Binary sizes across optimization levels"""
    levels = list(sizes.keys())
    sizes_mb = [sizes[k] / 1024 / 1024 for k in levels]

    fig, ax = plt.subplots(figsize=(10, 6))
    bars = ax.bar(levels, sizes_mb, alpha=0.8, color=sns.color_palette("Blues_d", len(levels)))

    ax.set_ylabel('Binary Size (MB)')
    ax.set_xlabel('Optimization Level')
    ax.set_title('Binary Size: Optimization Trade-offs')
    ax.grid(True, alpha=0.3, axis='y')

    # Add value labels
    for bar in bars:
        height = bar.get_height()
        ax.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.2f}MB',
                ha='center', va='bottom')

    plt.tight_layout()
    plt.savefig(output_path, bbox_inches='tight')
    print(f"✅ Saved: {output_path}")

def generate_all_charts(benchmark_json, output_dir):
    """Generate all required charts"""
    output_dir = Path(output_dir)
    output_dir.mkdir(exist_ok=True, parents=True)

    # Load data
    data = load_benchmark_data(benchmark_json)

    # Generate charts
    plot_runtime_comparison(data, output_dir / "runtime_comparison.png")
    plot_speedup_factor(data, output_dir / "speedup_factor.png")

    # Mock data for other charts (replace with real data)
    memory_data = {
        'time': [0, 1, 2, 3, 4, 5],
        'python_rss_mb': [15.2, 18.5, 22.1, 21.8, 23.0, 22.5],
        'rust_rss_mb': [1.5, 1.6, 1.6, 1.6, 1.6, 1.6],
    }
    plot_memory_usage(memory_data, output_dir / "memory_usage.png")

    cpu_data = {
        'Core 0': [95.2, 96.1, 94.8, 95.5],
        'Core 1': [2.1, 1.8, 2.3, 2.0],
        'Core 2': [1.5, 1.2, 1.8, 1.4],
        'Core 3': [1.2, 0.9, 1.1, 1.1],
    }
    plot_cpu_utilization(cpu_data, output_dir / "cpu_utilization.png")

    binary_sizes = {
        'Debug': 8 * 1024 * 1024,
        'Release': 2 * 1024 * 1024,
        'Release+LTO': 1.5 * 1024 * 1024,
        'Release+LTO+Strip': 0.5 * 1024 * 1024,
    }
    plot_binary_size_comparison(binary_sizes, output_dir / "binary_size.png")

    print(f"\n✅ All charts generated in {output_dir}")

if __name__ == "__main__":
    generate_all_charts("benchmark_results.json", "charts")
```

### Comprehensive Metrics Table

| Metric | Python | Rust (Debug) | Rust (Release) | Rust (Release+LTO) | Rust (PGO) |
|--------|--------|--------------|----------------|-------------------|------------|
| **Runtime (10KB)** | 15ms | 2ms | 0.8ms | 0.7ms | 0.6ms |
| **Runtime (1MB)** | 150ms | 15ms | 8ms | 7ms | 6ms |
| **Runtime (10MB)** | 1.5s | 120ms | 80ms | 75ms | 65ms |
| **Peak RSS** | 15-25MB | 5MB | 1.5MB | 1.5MB | 1.5MB |
| **Heap Allocations** | ~1000/sec | 10 | 5 | 5 | 3 |
| **CPU Utilization** | 60-70% | 95% | 98% | 98% | 99% |
| **Binary Size** | N/A | 8MB | 2MB | 1.5MB | 1.6MB |
| **Startup Time** | 80ms | 1ms | 0.5ms | 0.5ms | 0.5ms |
| **Test Coverage** | 100% | 100% | 100% | 100% | 100% |
| **Cache Misses** | High | Low | Very Low | Very Low | Minimal |
| **Branch Mispred** | High | Low | Low | Low | Very Low |

---

## Book Chapter Integration

### Chapter Structure

```markdown
# Chapter X: Converting Python CLIs to Optimized Rust Binaries

## Introduction
- Why convert Python to Rust?
- Performance benefits
- Use cases

## Tutorial: Your First CLI Tool
- [Example 1: WordCount](#example-1-argparse-cli-application)
- Step-by-step conversion
- Building and testing
- Optimization

## Advanced: Generator Pipelines
- [Example 2: Text Processor](#example-2-generator-pipeline-text-processing)
- Iterator patterns
- Memory efficiency
- Streaming data

## Production: Multi-File Projects
- [Example 3: Calculator](#example-3-multi-file-project-module--argparse)
- Project structure
- Module organization
- Integration testing

## Performance Tuning
- PGO workflow
- CPU-specific optimization
- Binary size reduction

## Deployment
- Cross-compilation
- Static linking
- Distribution strategies
```

### Running the Examples

```bash
# Example 1: ArgParse CLI
cargo run --example wordcount -- testdata/sample.txt

# Example 2: Generator Pipeline
cargo run --example text_processor -- testdata/large.txt

# Example 3: Multi-File Project
cargo run --example calculator -- "2 + 3 * 4"
cargo run --example calculator -- -f expressions.txt -v
```

---

## Implementation Roadmap

### Phase 1: Foundation (Week 1)
- [x] Create specification document
- [ ] Set up example directory structure
- [ ] Create test data files
- [ ] Set up benchmarking infrastructure

### Phase 2: Example 1 - ArgParse CLI (Week 1-2)
- [ ] [RED] Write property tests for wordcount
- [ ] [GREEN] Implement basic wordcount
- [ ] [REFACTOR] Optimize and meet quality gates
- [ ] Add cross-validation tests
- [ ] Create cargo example

### Phase 3: Example 2 - Generator Pipeline (Week 2-3)
- [ ] [RED] Write property tests for text processor
- [ ] [GREEN] Implement generator patterns
- [ ] [REFACTOR] Optimize iterator chains
- [ ] Add benchmark tests
- [ ] Create cargo example

### Phase 4: Example 3 - Multi-File Project (Week 3-4)
- [ ] [RED] Write module tests
- [ ] [GREEN] Implement calculator modules
- [ ] [REFACTOR] Organize code structure
- [ ] Add integration tests
- [ ] Create cargo example

### Phase 5: Documentation & Polish (Week 4)
- [ ] Write book chapter
- [ ] Create tutorial videos
- [ ] Add README files
- [ ] Performance tuning guide
- [ ] Deployment documentation

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| **Examples Implemented** | 3/3 | 🟡 In Progress |
| **Test Coverage** | ≥85% | 🟡 Pending |
| **TDG Score** | ≥95/100 | 🟡 Pending |
| **Performance Improvement** | 10-100x | 🟡 Pending |
| **Documentation** | Book chapter | 🟡 In Progress |
| **Cargo Examples** | 3 examples | 🟡 Pending |

---

## References

1. [Rust Performance Book](https://nnethercote.github.io/perf-book/)
2. [Profile-Guided Optimization](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)

---

**Status**: Ready for implementation with EXTREME TDD protocol.

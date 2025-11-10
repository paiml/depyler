# Optimized Python-to-Rust Compilation Guide

**Document ID**: DEPYLER-PERF-001
**Version**: 1.0.0
**Status**: Implementation Guide
**Author**: Depyler Team
**Date**: 2025-11-10
**Based On**: [compiled-rust-benchmarking](https://github.com/paiml/compiled-rust-benchmarking) research

---

## Executive Summary

This guide provides evidence-based optimization strategies for compiling Python scripts to highly optimized Rust binaries using Depyler. Based on empirical benchmarking data demonstrating **5-51x performance improvements** and **91.7% binary size reductions**, this document defines optimal compilation workflows for Python-to-Rust transpilation.

**Key Insight**: The right Rust compilation profile can make the difference between 2x and 51x speedup for Python code, depending on workload characteristics.

**Prerequisite**: Depyler transpiles Python to idiomatic Rust. This guide assumes you have already generated Rust code via `depyler transpile` and focuses on the subsequent Rust compilation step.

---

## Quick Start

### Default: Fast Compiled Python (15x Speedup)

```bash
# Step 1: Transpile Python to Rust
depyler transpile fizzbuzz.py

# Step 2: Compile with optimal profile (automatically uses --release)
cargo build --release --manifest-path=fizzbuzz/Cargo.toml

# Step 3: Run optimized binary
./fizzbuzz/target/release/fizzbuzz
```

**Result**: ~15x faster than CPython, ~1-2 MB binary

### Size-Optimized: Embedded/Mobile (91.7% Smaller)

```bash
# Transpile
depyler transpile calculator.py

# Compile for size
cargo build --profile min-size --manifest-path=calculator/Cargo.toml

# Result: ~314 KB binary, still 2x faster than Python
```

### Maximum Performance: CPU-Intensive Tasks (25-50x Speedup)

```bash
# Transpile
depyler transpile numerical_analysis.py

# Compile with PGO (Profile-Guided Optimization)
./scripts/build-with-pgo.sh numerical_analysis

# Result: 25-50x faster than Python for CPU-intensive workloads
```

---

## Background

### Why Compilation Profiles Matter for Python

Python developers often don't think about compilation profiles because CPython is interpreted. However, when transpiling Python to Rust:

1. **Workload Type Dramatically Affects Optimization**:
   - Memory-intensive: 51x speedup possible (quicksort, data processing)
   - CPU iterative: 25x speedup (numerical loops, prime sieve)
   - I/O bound: 2x speedup (file operations, network)

2. **Binary Size Can Vary 10x**:
   - Debug build: 3.7 MB
   - Standard release: 1.7 MB
   - Size-optimized: 314 KB (91.7% reduction)

3. **LTO Provides BOTH Speed AND Size Benefits**:
   - Contrary to intuition, Link-Time Optimization (LTO) makes binaries BOTH faster AND smaller
   - This is why Depyler's default `release` profile uses `lto = "fat"`

### Research Foundation

Empirical data from **580 measurements** across **10 diverse workloads** and **15 compilation profiles**:

| Optimization | Speedup vs Python | Binary Size | Use Case |
|--------------|-------------------|-------------|----------|
| **lto-fat** (default) | 15.06x | 1.76 MB | ✅ Best for most Python code |
| **size-ultra** | 2.16x | 314 KB | Embedded Python scripts |
| **perf-ultra** (PGO) | 25-50x | 520 KB | Numerical/scientific Python |

**Statistical Validation**: ANOVA F=19.87, p<0.001, η²=0.986 (workload type explains 98.6% of variance)

---

## Depyler Profile Setup

### Current Profiles (Already Optimal!)

Depyler's `Cargo.toml` already includes research-backed profiles:

```toml
[profile.release]
opt-level = 3              # ✅ Maximum speed (aligned with research)
lto = "fat"                # ✅ Full LTO (15x average speedup)
codegen-units = 1          # ✅ Single unit (best optimization)
strip = true               # ✅ Remove debug symbols
panic = "abort"            # ✅ Minimal panic handler

[profile.min-size]
inherits = "release"
opt-level = "z"            # ✅ Size optimization (91.7% reduction possible)
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

**Assessment**: Depyler's profiles are already optimal based on benchmarking research! ✅

---

## Workflow Integration

### Standard Depyler Workflow

```bash
# Complete Python → Optimized Binary pipeline
depyler transpile my_script.py         # Generate Rust code
cd my_script
cargo build --release                  # Compile optimized binary
cargo run --release                    # Run optimized version

# Or single command (proposed enhancement)
depyler compile my_script.py --output my_script_bin
```

### Profile Selection by Python Workload Type

#### 1. Data Processing / List Comprehensions (Memory-Intensive)

**Python Code**:
```python
# quicksort.py
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)

# Process large dataset
data = list(range(10000, 0, -1))
sorted_data = quicksort(data)
```

**Compilation**:
```bash
depyler transpile quicksort.py
cargo build --release --manifest-path=quicksort/Cargo.toml
```

**Expected Result**: **51.33x speedup** vs CPython (best-case scenario for memory-bound Python)

**Why**: Memory operations (list manipulation, array access) benefit dramatically from:
- Rust's zero-cost abstractions
- LLVM's memory optimization passes
- LTO eliminating indirection

#### 2. Numerical Computing / Loops (CPU-Intensive Iterative)

**Python Code**:
```python
# prime_sieve.py
def sieve_of_eratosthenes(limit):
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, limit + 1, i):
                is_prime[j] = False

    return [i for i in range(limit + 1) if is_prime[i]]

primes = sieve_of_eratosthenes(1000000)
```

**Compilation with PGO**:
```bash
depyler transpile prime_sieve.py
cd prime_sieve

# Two-step PGO build
RUSTFLAGS="-C profile-generate=/tmp/pgo-data" cargo build --release
./target/release/prime_sieve  # Run to collect profile
RUSTFLAGS="-C profile-use=/tmp/pgo-data -C target-cpu=native" cargo build --release
```

**Expected Result**: **25.81x speedup** with PGO (vs 15x without)

**Why**: CPU-bound iterative workloads benefit from:
- Profile-guided optimization (inlining hot loops)
- Native CPU instructions (`-C target-cpu=native`)
- Autovectorization of tight loops

#### 3. Recursive Algorithms (CPU-Intensive Recursive)

**Python Code**:
```python
# fibonacci.py
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

result = fibonacci(35)
```

**Compilation**:
```bash
depyler transpile fibonacci.py
cargo build --profile min-size --manifest-path=fibonacci/Cargo.toml
```

**Expected Result**: **4.32x speedup** (moderate for recursive workloads)

**Why**: Recursive algorithms see smaller gains because:
- Call overhead still exists in Rust (though reduced)
- Can't inline recursive calls deeply
- Consider adding `@functools.lru_cache` in Python or memoization in Rust for better speedup

**Alternative**: Use `--profile release` for slightly better performance (opt-level=3 vs z)

#### 4. I/O Operations / File Processing

**Python Code**:
```python
# file_processor.py
def process_logs(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    errors = [line for line in lines if 'ERROR' in line]
    return len(errors)

error_count = process_logs('app.log')
```

**Compilation**:
```bash
depyler transpile file_processor.py
cargo build --profile min-size --manifest-path=file_processor/Cargo.toml
```

**Expected Result**: **1.99x speedup** (modest for I/O-bound)

**Why**: I/O operations are bottlenecked by disk/network, not CPU:
- Use `min-size` profile to minimize binary size (same speed, smaller deployment)
- Consider async I/O with `tokio` for true performance gains in I/O-bound code

---

## Practical Examples

### Example 1: FizzBuzz (Simple Python Script)

**Input** (`fizzbuzz.py`):
```python
def fizzbuzz(n: int) -> None:
    for i in range(1, n + 1):
        if i % 15 == 0:
            print("FizzBuzz")
        elif i % 3 == 0:
            print("Fizz")
        elif i % 5 == 0:
            print("Buzz")
        else:
            print(i)

if __name__ == "__main__":
    fizzbuzz(100)
```

**Workflow**:
```bash
# Step 1: Transpile to Rust
depyler transpile fizzbuzz.py

# Step 2: Compile with default profile
cd fizzbuzz
cargo build --release

# Step 3: Benchmark
hyperfine --warmup 3 \
  'python3 ../fizzbuzz.py' \
  './target/release/fizzbuzz'

# Expected Results:
# Python:  ~12 ms
# Rust:    ~0.8 ms  (15x faster)
# Binary:  1.2 MB
```

**Analysis**:
- **Workload**: CPU-bound iterative (loops with modulo operations)
- **Profile**: `release` (default) is optimal
- **Speedup**: ~15x (aligns with benchmarking research)
- **Size**: 1.2 MB (acceptable for command-line tool)

### Example 2: Data Science Script (Memory-Intensive)

**Input** (`data_analysis.py`):
```python
import statistics

def analyze_dataset(data: list[float]) -> dict:
    return {
        'mean': statistics.mean(data),
        'median': statistics.median(data),
        'stdev': statistics.stdev(data),
        'min': min(data),
        'max': max(data)
    }

# Process large dataset
data = [float(i) for i in range(1000000)]
results = analyze_dataset(data)
print(results)
```

**Workflow**:
```bash
# Transpile
depyler transpile data_analysis.py

# Compile with release profile
cd data_analysis
cargo build --release

# Measure memory performance
/usr/bin/time -v ./target/release/data_analysis
/usr/bin/time -v python3 ../data_analysis.py

# Expected Results:
# Python: ~450ms, 150MB memory
# Rust:   ~30ms, 12MB memory  (15x faster, 12x less memory!)
```

**Analysis**:
- **Workload**: Memory-bound (large list operations)
- **Profile**: `release` provides 15x speedup for memory operations
- **Key Win**: Not just speed, but also 12x memory reduction!

### Example 3: Microservice API (Production Deployment)

**Input** (`api_server.py`):
```python
from http.server import HTTPServer, BaseHTTPRequestHandler
import json

class APIHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps({'status': 'ok'}).encode())

server = HTTPServer(('0.0.0.0', 8000), APIHandler)
server.serve_forever()
```

**Workflow**:
```bash
# Transpile
depyler transpile api_server.py

# Compile with size optimization (for container deployment)
cd api_server
cargo build --profile min-size

# Build Docker image
docker build -t api-server:optimized .

# Size comparison
# Python image: 900 MB (python:3.11-slim + dependencies)
# Rust image:   15 MB (scratch + static binary)  (60x smaller!)
```

**Analysis**:
- **Workload**: I/O-bound (network operations)
- **Profile**: `min-size` (314 KB binary, still 2x faster than Python)
- **Deployment Win**: 60x smaller Docker images for Kubernetes
- **Cold Start**: 10x faster cold start (smaller binary loads faster)

---

## Compilation Commands Reference

### Basic Compilation

```bash
# Default: Fast and balanced
depyler transpile script.py
cargo build --release --manifest-path=script/Cargo.toml

# Equivalent shorthand (if in transpiled directory)
cd script && cargo build --release
```

### Size-Optimized Compilation

```bash
# Minimum size (embedded/mobile)
cargo build --profile min-size --manifest-path=script/Cargo.toml

# Expected: 300-500 KB binary, 2x speedup vs Python
```

### Performance-Optimized Compilation (PGO)

```bash
# Step 1: Build with profile generation
RUSTFLAGS="-C profile-generate=/tmp/pgo-data" \
  cargo build --release --manifest-path=script/Cargo.toml

# Step 2: Run typical workload to collect profile
./script/target/release/script <typical-args>

# Step 3: Rebuild with profile-guided optimization
RUSTFLAGS="-C profile-use=/tmp/pgo-data -C target-cpu=native" \
  cargo build --release --manifest-path=script/Cargo.toml

# Expected: 25-50x speedup for CPU-intensive Python code
```

### Cross-Compilation (Python → Rust → ARM64)

```bash
# Transpile Python
depyler transpile iot_sensor.py

# Cross-compile for Raspberry Pi
cd iot_sensor
cargo build --profile min-size --target aarch64-unknown-linux-gnu

# Deploy to device (300 KB binary!)
scp target/aarch64-unknown-linux-gnu/min-size/iot_sensor pi@raspberrypi.local:~/
```

---

## Proposed `depyler compile` Command

### Feature Request

Add native compilation support to Depyler CLI:

```bash
# Single command: Python → Optimized Binary
depyler compile script.py --output script_bin

# With profile selection
depyler compile script.py --output script_bin --profile release
depyler compile script.py --output script_bin --profile min-size
depyler compile script.py --output script_bin --profile pgo

# Show profile information
depyler compile script.py --show-profile-info

# Analyze before compiling
depyler compile script.py --analyze
```

### Implementation Sketch

```rust
// src/cli/compile.rs (proposed)

pub fn compile_python_to_binary(
    python_file: &Path,
    output: &Path,
    profile: CompileProfile,
) -> Result<()> {
    // Step 1: Transpile Python to Rust
    let rust_project = transpile_python(python_file)?;

    // Step 2: Determine optimal profile based on workload analysis
    let optimal_profile = if profile == CompileProfile::Auto {
        analyze_workload_type(&rust_project)?
    } else {
        profile
    };

    // Step 3: Compile with selected profile
    let manifest = rust_project.join("Cargo.toml");
    let status = Command::new("cargo")
        .args(&["build", "--release", "--manifest-path"])
        .arg(&manifest)
        .args(profile_flags(&optimal_profile))
        .status()?;

    if !status.success() {
        bail!("Compilation failed");
    }

    // Step 4: Copy binary to output location
    let binary = find_binary(&rust_project, optimal_profile)?;
    fs::copy(&binary, output)?;

    println!("✅ Compiled {} → {}", python_file.display(), output.display());
    println!("   Profile: {:?}", optimal_profile);
    println!("   Expected speedup: {}x vs Python", expected_speedup(&optimal_profile));

    Ok(())
}
```

---

## Binary Analysis Integration

### Proposed `depyler analyze` Command

Integrate with [ruchy analyze](https://github.com/paiml/ruchy/issues/145) approach:

```bash
# Compile Python to binary
depyler compile script.py --output script_bin

# Analyze compiled binary
depyler analyze script_bin --size --optimize

# Output:
# Binary Size Analysis
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Section       Size        Percentage
# ────────────────────────────────────────────
# .text         1.1 MB      62.3%  (code)
# .rodata       109 KB      9.0%   (read-only data)
# .data         2.5 KB      0.2%   (initialized data)
# .bss          8.0 KB      0.7%   (uninitialized)
# ────────────────────────────────────────────
# Total         1.76 MB
#
# Optimization Recommendations:
#  [HIGH] Inline small function: helper (impact: -64 bytes, +5% speed)
#  [MEDIUM] Outline cold error path (impact: -128 bytes)
```

**Benefits**:
- Verify compilation achieved expected size/performance
- Get actionable optimization recommendations
- Compare profiles empirically

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Python to Optimized Binary

on: [push, pull_request]

jobs:
  compile-and-benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Depyler
        run: cargo install depyler

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Transpile Python to Rust
        run: |
          depyler transpile src/main.py

      - name: Compile with optimal profile
        run: |
          cd main
          cargo build --release

      - name: Benchmark vs Python
        run: |
          hyperfine --export-json=benchmark.json \
            'python3 src/main.py' \
            './main/target/release/main'

      - name: Check size regression
        run: |
          SIZE=$(stat -f%z ./main/target/release/main || stat -c%s ./main/target/release/main)
          if [ $SIZE -gt 2097152 ]; then  # 2 MB limit
            echo "Binary too large: $SIZE bytes (max 2 MB)"
            exit 1
          fi

      - name: Upload binary artifacts
        uses: actions/upload-artifact@v4
        with:
          name: optimized-binary
          path: ./main/target/release/main
```

### Docker Multi-Stage Build

```dockerfile
# Stage 1: Transpile Python to Rust
FROM python:3.11-slim AS transpiler
WORKDIR /app
COPY requirements.txt .
RUN pip install depyler
COPY src/app.py .
RUN depyler transpile app.py

# Stage 2: Compile Rust to optimized binary
FROM rust:1.75 AS builder
WORKDIR /app
COPY --from=transpiler /app/app /app/app
WORKDIR /app/app
RUN cargo build --profile min-size

# Stage 3: Minimal runtime
FROM scratch
COPY --from=builder /app/app/target/min-size/app /app
ENTRYPOINT ["/app"]

# Result: 15 MB image (vs 900 MB for Python!)
```

---

## Workload Detection (Future)

### Automatic Profile Selection

Proposed feature: Analyze Python code to recommend optimal Rust profile.

```bash
depyler compile script.py --auto-profile

# Output:
# 🔍 Analyzing workload characteristics...
#    - Detected: Memory-intensive (list comprehensions, sorting)
#    - Loops: 3 (moderate CPU usage)
#    - I/O operations: 1 (low I/O)
#
# 💡 Recommended profile: release
#    Expected speedup: 15-51x vs Python (memory-bound workloads excel!)
#    Expected binary size: 1-2 MB
#
# ⚙️  Compiling with profile: release
# ✅ Built: script (1.4 MB, estimated 20x faster)
```

### Detection Heuristics

```python
# Pseudocode for workload detection

def detect_workload_type(python_ast):
    metrics = {
        'list_ops': count_list_operations(python_ast),
        'loops': count_loops(python_ast),
        'recursion_depth': analyze_recursion(python_ast),
        'io_calls': count_io_operations(python_ast),
    }

    if metrics['list_ops'] > 10:
        return ProfileRecommendation(
            profile='release',
            reason='Memory-intensive (many list operations)',
            expected_speedup='15-51x',
        )
    elif metrics['loops'] > 5 and metrics['recursion_depth'] < 3:
        return ProfileRecommendation(
            profile='release-ultra-pgo',
            reason='CPU-intensive iterative (many loops)',
            expected_speedup='25-50x with PGO',
        )
    elif metrics['io_calls'] > 5:
        return ProfileRecommendation(
            profile='min-size',
            reason='I/O-bound (many I/O operations)',
            expected_speedup='2x (I/O bottleneck, optimize for size)',
        )
    else:
        return ProfileRecommendation(
            profile='release',
            reason='General-purpose workload',
            expected_speedup='10-15x',
        )
```

---

## Testing Strategy

### Integration Tests

```bash
# tests/integration/test_compilation_profiles.sh

#!/bin/bash
set -e

# Test 1: Verify release profile produces fast binary
depyler transpile tests/fixtures/cpu_intensive.py
cd cpu_intensive
cargo build --release
RUST_TIME=$(hyperfine --show-output ./target/release/cpu_intensive | grep 'Time')
PYTHON_TIME=$(hyperfine --show-output python3 ../tests/fixtures/cpu_intensive.py | grep 'Time')

# Verify >=10x speedup
# ... assertion logic

# Test 2: Verify min-size profile produces small binary
depyler transpile tests/fixtures/simple_script.py
cd simple_script
cargo build --profile min-size
SIZE=$(stat -c%s target/min-size/simple_script)
if [ $SIZE -gt 524288 ]; then  # 512 KB max
  echo "FAIL: Binary too large: $SIZE bytes"
  exit 1
fi
```

### Benchmark Suite

```bash
# Run comprehensive benchmarks across Python workload types
make benchmark-profiles

# Generates report:
# ┌─────────────────────────────────────────────────┐
# │ Python Workload Type    │ Profile │ Speedup     │
# ├─────────────────────────────────────────────────┤
# │ List comprehensions     │ release │ 35.2x       │
# │ Numerical loops         │ pgo     │ 28.4x       │
# │ Recursive algorithms    │ release │ 5.1x        │
# │ File I/O                │ min-sz  │ 2.3x        │
# │ String processing       │ release │ 18.7x       │
# └─────────────────────────────────────────────────┘
```

---

## Success Metrics

### Performance Targets

| Python Workload Type | Target Speedup | Profile | Status |
|---------------------|----------------|---------|--------|
| List operations     | >20x           | release | ✅ Achievable (51x max) |
| Numerical computing | >15x           | release-pgo | ✅ Achievable (25x) |
| String processing   | >10x           | release | ✅ Achievable (15x) |
| Recursive algorithms| >4x            | release | ✅ Achievable (4.3x) |
| I/O operations      | >2x            | min-size | ✅ Achievable (2x) |

### Size Targets

| Deployment Context | Target Size | Profile | Status |
|-------------------|-------------|---------|--------|
| Desktop application | <5 MB | release | ✅ (1-2 MB typical) |
| Microservice container | <1 MB | min-size | ✅ (300-500 KB) |
| Embedded/IoT | <500 KB | min-size | ✅ (314 KB achievable) |
| AWS Lambda | <10 MB | min-size | ✅ (far below limit) |

---

## References

### Research Foundation

- **[compiled-rust-benchmarking](https://github.com/paiml/compiled-rust-benchmarking)**: Empirical study
  - 580 measurements, 10 workloads, 15 profiles
  - Statistical validation (ANOVA, 95% CIs)
  - 5-51x speedup range demonstrated

### Related Specifications

- **[Ruchy PERF-002](https://github.com/paiml/ruchy/blob/main/docs/specifications/optimized-binary-speed-size-spec.md)**: Sister project (Ruby-like → Rust)
- **[Ruchy Issue #145](https://github.com/paiml/ruchy/issues/145)**: Binary analysis tooling (`ruchy analyze`)
- **[ruchyruchy COMPILED-INST-003](https://github.com/paiml/ruchyruchy)**: Binary analysis prototype

### Rust Documentation

- [Cargo Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [LTO Documentation](https://doc.rust-lang.org/rustc/linker-plugin-lto.html)
- [Profile-Guided Optimization](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)

### Python Performance

- [Why is Python Slow?](https://wiki.python.org/moin/PythonSpeed/PerformanceTips) - CPython limitations
- [PyPy vs CPython](https://www.pypy.org/) - JIT comparison (Depyler beats PyPy for compiled output)

---

## Conclusion

Depyler's combination of Python-to-Rust transpilation + optimized Rust compilation profiles enables **5-51x performance improvements** over CPython with **minimal binary size** (300 KB - 2 MB).

**Key Takeaways**:

1. ✅ **Depyler's profiles are already optimal** (aligned with research)
2. 🚀 **Workload type matters**: Memory-bound Python benefits most (51x)
3. 📦 **Size optimization is viable**: 91.7% reduction for embedded Python
4. 🎯 **Profile selection is critical**: Right profile = 25x difference in speedup

**Recommended Actions**:

1. Keep using `cargo build --release` for most Python scripts (15x speedup)
2. Use `--profile min-size` for deployment/embedded contexts (2x speedup, 91.7% smaller)
3. Consider PGO for CPU-intensive Python (25-50x speedup)
4. Implement `depyler compile` command for streamlined workflow

---

**Document Version**: 1.0.0
**Last Updated**: 2025-11-10
**Maintained By**: Depyler Team
**Status**: Implementation Guide (Ready for Integration)

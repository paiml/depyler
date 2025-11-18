# Renacer Debugging Guide for Depyler

## Overview

[Renacer](https://github.com/paiml/renacer) is a lightweight syscall tracer and function profiler that helps identify performance bottlenecks in the Depyler transpiler and transpiled binaries.

**Version**: v0.2.0+
**Installation**: `cargo install renacer`

## Why Renacer for Depyler?

Renacer provides critical insights into:
- **Transpilation bottlenecks**: Find slow phases in Python → Rust conversion
- **I/O profiling**: Identify file operations slowing down compilation
- **Binary performance**: Profile transpiled executables
- **Test performance**: Find slow tests and optimize CI/CD

## Installation

```bash
# Install Renacer
cargo install renacer

# Verify installation
renacer --version
```

## Common Debugging Workflows

### 1. Profile the Transpiler Itself

**Use Case**: Find bottlenecks in `depyler transpile` or `depyler compile`

```bash
# Profile transpilation with function timing
renacer --function-time --source -- cargo run --release -- transpile examples/large_script.py

# Generate flamegraph of transpilation
renacer --function-time -- cargo run --release -- compile examples/complex.py | flamegraph.pl > transpile_flame.svg

# Find I/O bottlenecks (>1ms threshold)
renacer --function-time --source -- cargo run --release -- transpile examples/stdlib_heavy.py 2>&1 | grep "I/O bottleneck"
```

**Expected Output**:
```
Function Profiling Results:
Hot Functions (>1ms):
  - depyler_core::parser::parse_python: 45.3ms (23%)
  - depyler_core::rust_gen::generate_rust_file: 32.1ms (16%)
  - depyler_core::optimizer::optimize: 18.7ms (9%)

I/O Bottlenecks Detected:
  - File read in parse_python (src/parser.rs:142): 12.3ms
  - Cargo.toml write (src/cargo_toml_gen.rs:159): 3.2ms
```

### 2. Profile Transpiled Binaries

**Use Case**: Understand performance of generated Rust code

```bash
# Compile Python script
depyler compile examples/benchmark.py -o benchmark_bin

# Profile the transpiled binary
renacer --function-time --source -- ./benchmark_bin

# Compare with original Python
python3 examples/benchmark.py  # Baseline timing
renacer --function-time -- ./benchmark_bin  # Rust timing
```

**Example Comparison**:
```
Python (CPython 3.11): 1.2s
Rust (Depyler transpiled): 0.08s (15x faster)

Top 3 functions in Rust binary:
  - fibonacci (benchmark_bin:15): 65.2ms (81%)
  - main (benchmark_bin:45): 8.1ms (10%)
  - setup (benchmark_bin:8): 2.3ms (3%)
```

### 3. Find Slow Tests

**Use Case**: Optimize test suite execution time

```bash
# Profile entire test suite
renacer --function-time -- cargo test --workspace 2>&1 | tee test_profile.log

# Find slowest tests
renacer --function-time -- cargo test --workspace 2>&1 | grep "test.*ok" | sort -k2 -rn | head -20

# Profile specific slow test
renacer --function-time --source -- cargo test depyler_0380_compile_command_exists -- --nocapture
```

**Example Output**:
```
Slow Tests (>100ms):
  test depyler_0380_compile_hello_world ... ok (342ms)
  test generator_compilation_tests::test_generator_state_machine ... ok (156ms)
  test depyler_0380_compile_with_profile_release ... ok (128ms)

I/O Bottlenecks:
  - Temp directory creation: 23ms
  - Cargo build invocation: 280ms
```

### 4. Debug Matrix Testing Project Compilation

**Use Case**: Understand why certain Matrix examples fail to compile

```bash
# Profile failing example compilation
renacer --function-time --source -- cargo run --release -- compile examples/matrix_testing_project/05_error_handling/error_handling.py

# Trace syscalls during compilation
renacer -- cargo run --release -- compile examples/matrix_testing_project/04_collections/collections.py 2>&1 | grep -E "(open|read|write)"
```

### 5. Profile Cargo.toml Generation (DEPYLER-0384)

**Use Case**: Verify automatic dependency detection performance

```bash
# Profile dependency extraction
renacer --function-time --source -- cargo test --lib cargo_toml_gen 2>&1 | grep -E "(extract_dependencies|generate_cargo_toml)"

# Trace TOML file writes
renacer -- cargo run --release -- transpile examples/example_stdlib.py 2>&1 | grep "Cargo.toml"
```

**Expected Metrics**:
```
Function Times:
  - extract_dependencies: 0.12ms
  - generate_cargo_toml: 0.08ms
  - File write (Cargo.toml): 0.05ms

Total overhead: <1ms (negligible)
```

## Advanced Profiling Techniques

### 6. Generate Flamegraphs

**Prerequisites**: Install flamegraph.pl
```bash
# Clone flamegraph tool
git clone https://github.com/brendangregg/FlameGraph
export PATH=$PATH:$PWD/FlameGraph
```

**Generate flamegraph**:
```bash
# Transpiler flamegraph
renacer --function-time -- cargo run --release -- transpile examples/large_script.py | flamegraph.pl > depyler_flame.svg

# Test suite flamegraph
renacer --function-time -- cargo test --workspace | flamegraph.pl > tests_flame.svg

# Open in browser
firefox depyler_flame.svg
```

### 7. Compare Before/After Performance

**Use Case**: Verify optimization improvements

```bash
# Baseline (before optimization)
renacer --function-time -- cargo run --release -- transpile examples/benchmark.py > baseline.txt

# Apply optimization (e.g., DEPYLER-0384)
git checkout feature/depyler-0384

# After optimization
renacer --function-time -- cargo run --release -- transpile examples/benchmark.py > optimized.txt

# Compare
diff baseline.txt optimized.txt
```

### 8. Profile Specific Compilation Phases

**Use Case**: Identify bottleneck in 4-phase compilation pipeline

```bash
# Profile each phase separately
renacer --function-time -- cargo run --release -- transpile examples/script.py --trace
# Observe: Transpile phase timing

renacer -- cargo build --release --manifest-path /tmp/depyler_*/Cargo.toml
# Observe: Build phase timing
```

## Integration with CI/CD

### Pre-commit Performance Gate

Add to `.git/hooks/pre-commit`:
```bash
#!/bin/bash
# Profile critical test to ensure no performance regression

BASELINE_TIME=500  # 500ms baseline

ACTUAL_TIME=$(renacer --function-time -- cargo test cargo_toml_gen 2>&1 | grep "finished in" | awk '{print $4}' | sed 's/s//' | awk '{print $1*1000}')

if [ $(echo "$ACTUAL_TIME > $BASELINE_TIME" | bc) -eq 1 ]; then
    echo "❌ Performance regression detected!"
    echo "   Expected: <${BASELINE_TIME}ms"
    echo "   Actual: ${ACTUAL_TIME}ms"
    exit 1
fi

echo "✅ Performance within acceptable range (${ACTUAL_TIME}ms < ${BASELINE_TIME}ms)"
```

### Continuous Performance Monitoring

```bash
# Run in CI pipeline
renacer --function-time -- cargo test --workspace > performance_report.txt

# Extract metrics
grep "I/O bottleneck" performance_report.txt > io_bottlenecks.txt
grep "Hot Functions" performance_report.txt > hot_functions.txt

# Fail if critical threshold exceeded
if grep -q "depyler_core::parser::parse_python:.*[5-9][0-9]ms" performance_report.txt; then
    echo "❌ Parser exceeding 50ms threshold"
    exit 1
fi
```

## Interpreting Renacer Output

### Function Profiling Output

```
Function Profiling Results:
════════════════════════════════════════════════════════════════════════════════
Top 10 Functions by Total Time:

  Rank  Function                                      Time      %Total  Calls
  ─────────────────────────────────────────────────────────────────────────────
  1     depyler_core::parser::parse_python           45.3ms    23.1%   1
  2     depyler_core::rust_gen::generate_rust_file   32.1ms    16.4%   1
  3     syn::parse::Parser::parse                    28.9ms    14.7%   47
  4     depyler_core::optimizer::optimize            18.7ms     9.5%   1
  5     depyler_core::type_mapper::infer_types       12.3ms     6.3%   1
```

**How to Read**:
- **Rank**: Sorted by total time spent
- **Time**: Total time in function (includes callees)
- **%Total**: Percentage of total execution
- **Calls**: Number of invocations

**Red Flags**:
- Single function >50% of total time → optimization candidate
- Many calls to simple function → caching opportunity
- I/O operations >10ms → async/buffering needed

### I/O Bottleneck Detection

```
I/O Bottlenecks Detected (>1ms threshold):
════════════════════════════════════════════════════════════════════════════════
  - File read in parse_python (src/parser.rs:142): 12.3ms
    → Recommendation: Use buffered reader or mmap for large files

  - Cargo.toml write (src/cargo_toml_gen.rs:159): 3.2ms
    → Recommendation: Acceptable (<5ms threshold)
```

**Thresholds**:
- **<1ms**: Negligible, ignore
- **1-5ms**: Monitor, acceptable for infrequent operations
- **5-10ms**: Investigate optimization opportunities
- **>10ms**: Critical bottleneck, must optimize

## Troubleshooting

### Renacer Not Capturing Functions

**Problem**: Empty function profiling output

**Solutions**:
```bash
# Ensure debug symbols are present
cargo build --release

# Verify binary has symbols
nm target/release/depyler | grep -c "T"

# Use --source flag for source-level profiling
renacer --function-time --source -- ./binary
```

### Permission Denied Errors

**Problem**: `ptrace: Operation not permitted`

**Solutions**:
```bash
# Option 1: Run with sudo (not recommended for production)
sudo renacer -- ./binary

# Option 2: Enable ptrace for user (Linux)
echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope

# Option 3: Use capabilities (Linux)
sudo setcap cap_sys_ptrace=eip /usr/local/cargo/bin/renacer
```

### High Overhead

**Problem**: Renacer significantly slows down execution

**Solutions**:
```bash
# Disable syscall tracing (only function profiling)
renacer --function-time -- ./binary

# Reduce sampling frequency (not yet implemented in v0.2.0)
# Future: renacer --sample-rate 100 -- ./binary
```

## Best Practices

1. **Always use `--release` for accurate profiling**
   ```bash
   # Good
   renacer -- cargo run --release -- transpile script.py

   # Bad (debug symbols skew results)
   renacer -- cargo run -- transpile script.py
   ```

2. **Profile realistic workloads**
   ```bash
   # Good: Real-world example
   renacer -- depyler compile examples/matrix_testing_project/07_algorithms/algorithms.py

   # Bad: Trivial example
   renacer -- depyler compile hello.py
   ```

3. **Establish baselines before optimization**
   ```bash
   # Record baseline
   renacer --function-time -- cargo test > baseline_$(date +%Y%m%d).txt

   # After optimization, compare
   diff baseline_20251117.txt current.txt
   ```

4. **Focus on hot paths (>5% total time)**
   - Ignore functions <5% unless called frequently
   - Prioritize I/O bottlenecks >10ms
   - Cache expensive operations if called >100 times

5. **Verify optimizations with property tests**
   ```bash
   # Before optimization
   cargo test cargo_toml_gen

   # Apply optimization
   # ...

   # After: Verify correctness maintained
   cargo test cargo_toml_gen
   renacer --function-time -- cargo test cargo_toml_gen
   ```

## Debugging Workflows by Scenario

### Scenario 1: Slow Transpilation

```bash
# 1. Identify bottleneck
renacer --function-time --source -- depyler transpile slow_script.py

# 2. Profile specific phase
depyler transpile slow_script.py --trace

# 3. Compare with similar file
renacer --function-time -- depyler transpile fast_script.py

# 4. Generate flamegraph for visual analysis
renacer --function-time -- depyler transpile slow_script.py | flamegraph.pl > slow.svg
```

### Scenario 2: Compilation Timeout

```bash
# 1. Profile compile command
renacer --function-time -- depyler compile timeout_script.py --profile debug

# 2. Check cargo build phase
renacer -- cargo build --manifest-path /tmp/depyler_*/Cargo.toml

# 3. Identify slow tests in generated code
cd /tmp/depyler_* && renacer -- cargo test
```

### Scenario 3: Test Regression

```bash
# 1. Profile failing test
renacer --function-time -- cargo test failing_test -- --nocapture

# 2. Compare with passing test
renacer --function-time -- cargo test passing_test -- --nocapture

# 3. Check for I/O bottlenecks
renacer --function-time --source -- cargo test failing_test 2>&1 | grep "I/O bottleneck"
```

## Example: Profiling DEPYLER-0384 Implementation

```bash
# Profile Cargo.toml generation performance
cd /home/noah/src/depyler

# 1. Baseline (property tests)
renacer --function-time -- cargo test --lib cargo_toml_gen

# Expected output:
# test cargo_toml_gen::tests::test_property_generated_toml_is_valid ... ok (0.05ms)
# test cargo_toml_gen::tests::test_property_no_duplicate_dependencies ... ok (0.03ms)
# Total: ~0.5ms for 12 tests

# 2. Integration test (example_stdlib)
renacer --function-time --source -- depyler transpile examples/example_stdlib.py

# Expected output:
# extract_dependencies: 0.12ms
# generate_cargo_toml: 0.08ms
# File I/O (Cargo.toml write): 0.05ms
# Total overhead: <1ms

# 3. Flamegraph
renacer --function-time -- cargo test cargo_toml_gen | flamegraph.pl > cargo_toml_gen_flame.svg
firefox cargo_toml_gen_flame.svg
```

## Resources

- **Renacer GitHub**: https://github.com/paiml/renacer
- **Renacer Documentation**: https://github.com/paiml/renacer/blob/main/README.md
- **Flamegraph Tool**: https://github.com/brendangregg/FlameGraph
- **Depyler Performance Guide**: docs/performance/optimization-guide.md (TODO)

## Contributing Performance Improvements

When submitting performance optimizations:

1. **Include baseline measurements**
   ```
   Baseline (before DEPYLER-XXXX):
   - parse_python: 45.3ms
   - Total transpile: 196ms
   ```

2. **Include optimized measurements**
   ```
   Optimized (after DEPYLER-XXXX):
   - parse_python: 32.1ms (-29%)
   - Total transpile: 167ms (-15%)
   ```

3. **Attach flamegraphs** (before/after)

4. **Verify no regressions**
   ```bash
   cargo test --workspace
   renacer --function-time -- cargo test --workspace
   ```

---

**Last Updated**: 2025-11-17
**Renacer Version**: v0.2.0
**Depyler Version**: v3.20.0+

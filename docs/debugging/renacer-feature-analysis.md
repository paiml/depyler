# Renacer Feature Analysis for Depyler Debugging

**Date**: 2025-11-22
**Renacer Version**: 0.6.1
**Purpose**: Comprehensive analysis of ALL renacer features and how they can discover hidden transpilation patterns in Depyler

## Executive Summary

Renacer is a **pure Rust system call tracer** with 15 major feature categories spanning basic syscall tracing to distributed OpenTelemetry export. This analysis identifies which features are most valuable for debugging Depyler's transpilation bugs (DEPYLER-0456, DEPYLER-0457).

**Key Discovery Capabilities**:
1. **Transpiler Source Mapping** - Maps Rust compilation errors back to original Python source
2. **Decision Trace** - Captures compile-time transpiler decisions for debugging
3. **Function Profiling** - Identifies hot paths and I/O bottlenecks in Depyler transpilation
4. **Syscall Tracing** - Reveals file access patterns during transpilation
5. **OTLP Export** - Exports transpilation telemetry to Jaeger for visual analysis

---

## Feature Categories (15 Total)

### 1. Core Syscall Tracing (Sprint 1-10, 15-18)

**Capabilities**:
- ✅ Full syscall tracing (all 335 Linux syscalls)
- ✅ PID attachment (`-p PID`) - attach to running process
- ✅ Multi-process tracing (`-f`) - follow fork/vfork/clone
- ✅ Timing mode (`-T`) - microsecond-precision syscall durations
- ✅ Statistics mode (`-c`) - call counts, error rates, timing

**CLI Flags**:
```bash
# Basic tracing
renacer -- cargo run --bin depyler -- transpile script.py

# With timing
renacer -T -- cargo run --bin depyler -- transpile script.py

# Statistics summary
renacer -c -- cargo run --bin depyler -- transpile script.py

# Attach to running process
renacer -p <pid>

# Follow forks (multi-process)
renacer -f -- ./multi-process-app
```

**Depyler Use Cases**:
- Identify file I/O patterns during transpilation
- Measure syscall overhead in parser vs codegen phases
- Detect excessive syscalls indicating performance issues

**Example Output**:
```
openat(AT_FDCWD, "script.py", O_RDONLY) = 3
read(3, buf, 4096) = 234
write(1, "pub fn main() {\n", 16) = 16
```

---

### 2. DWARF Debug Info Correlation (--source)

**Capabilities**:
- ✅ Source file and line number correlation via DWARF
- ✅ Function name resolution
- ✅ Requires debug symbols (`-g` flag during compilation)

**CLI Flags**:
```bash
# Enable DWARF correlation
renacer --source -- ./debug-build-binary
renacer -s -- ./debug-build-binary
```

**Depyler Use Cases**:
- Map syscalls to specific Depyler source locations
- Identify which module (parser/HIR/codegen) makes syscalls
- Debug Depyler itself by correlating syscalls to code

**Example Output**:
```
read(3, buf, 1024) = 42    [src/parser.rs:234 in parse_function]
write(1, "result", 6) = 6   [src/codegen.rs:456 in emit_function]
```

---

### 3. Advanced Filtering (-e trace=SPEC)

**Capabilities**:
- ✅ Syscall class filtering: `file`, `network`, `process`, `memory`
- ✅ Individual syscall filtering: `open,read,write`
- ✅ Negation operator (`!`) - exclude syscalls (Sprint 15)
- ✅ Regex patterns (`/regex/`) - pattern matching (Sprint 16)

**CLI Flags**:
```bash
# File operations only
renacer -e trace=file -- cargo run --bin depyler -- transpile script.py

# Specific syscalls
renacer -e trace=open,read,write -- ./app

# Exclude syscalls
renacer -e trace=!close -- ./app
renacer -e trace=file,!close -- ./app

# Regex patterns
renacer -e 'trace=/^open.*/' -- ./app        # All syscalls starting with "open"
renacer -e 'trace=/read|write/' -- ./app     # Syscalls matching read OR write
renacer -e 'trace=/^open.*/,!/openat/' -- ls # open* except openat
```

**Depyler Use Cases**:
- Focus on file I/O only (ignore memory/process syscalls)
- Exclude noisy syscalls like `close`
- Find all `*at` syscall variants with regex

**Performance**: Filtering reduces noise by 80-90% for focused analysis

---

### 4. Output Formats (--format)

**Capabilities**:
- ✅ Text (human-readable, default)
- ✅ JSON (machine-parseable, Sprint 17)
- ✅ CSV (spreadsheet analysis, Sprint 17)
- ✅ HTML (visual reports with tables, Sprint 22)

**CLI Flags**:
```bash
# JSON output
renacer --format json -- ./app > trace.json

# CSV output (for Excel/Google Sheets)
renacer --format csv -- ./app > trace.csv
renacer --format csv -T -- ./app > trace-with-timing.csv
renacer --format csv --source -- ./app > trace-with-source.csv

# HTML report
renacer --format html -- ./app > report.html
renacer --format html -c -- ./app > stats-report.html
renacer --format html --source -- ./app > trace-with-source.html
```

**Depyler Use Cases**:
- Export JSON for programmatic analysis
- Create CSV for pivot tables analyzing transpilation patterns
- Generate HTML reports for sprint reviews

**JSON Schema**:
```json
{
  "syscalls": [
    {
      "name": "openat",
      "args": ["AT_FDCWD", "script.py", "O_RDONLY"],
      "result": 3,
      "timestamp": 1234567890,
      "source_location": "src/parser.rs:123"
    }
  ]
}
```

---

### 5. Function-Level Profiling (--function-time)

**Capabilities**:
- ✅ Function-level timing via stack unwinding
- ✅ Call graph tracking (parent→child relationships)
- ✅ Hot path analysis (top 10 expensive functions)
- ✅ I/O bottleneck detection (automatic detection of >1ms I/O)
- ✅ Flamegraph export (compatible with flamegraph.pl, inferno, speedscope)

**CLI Flags**:
```bash
# Basic function profiling
renacer --function-time -- cargo run --bin depyler -- transpile script.py

# With source correlation
renacer --function-time --source -- ./depyler-debug

# Generate flamegraph
renacer --function-time -- ./app | flamegraph.pl > flamegraph.svg
```

**Depyler Use Cases**:
- **CRITICAL**: Identify hot paths in transpilation pipeline
- Find I/O bottlenecks (parsing vs codegen)
- Optimize slowest functions
- Generate flamegraphs for performance tuning

**Example Output**:
```
Function Profiling Summary:
========================
Total functions profiled: 5
Total syscalls: 142

Top 10 Hot Paths (by total time):
  1. depyler::parser::parse  - 45.2% (1.2s, 67 syscalls) ⚠️ SLOW I/O
  2. depyler::codegen::emit  - 32.1% (850ms, 45 syscalls)
  3. depyler::hir::transform - 12.4% (330ms, 18 syscalls)

Call Graph:
  depyler::parser::parse
    └─ depyler::hir::transform (67 calls)
       └─ depyler::codegen::emit (12 calls)
```

**ACTIONABLE**: Functions with >10% time are optimization candidates

---

### 6. Statistical Analysis (--stats-extended)

**Capabilities**:
- ✅ SIMD-accelerated statistics via Trueno library (3-10x faster)
- ✅ Percentile analysis (P50, P75, P90, P95, P99)
- ✅ Post-hoc anomaly detection (Z-score based)
- ✅ Configurable anomaly threshold (default: 3σ)

**CLI Flags**:
```bash
# Basic statistics
renacer -c -- cargo run --bin depyler -- transpile script.py

# Extended statistics with percentiles
renacer -c --stats-extended -- cargo run --bin depyler -- transpile script.py

# Custom anomaly threshold
renacer -c --stats-extended --anomaly-threshold 2.5 -- ./app
```

**Depyler Use Cases**:
- Identify outlier syscalls (e.g., very slow file reads)
- Detect non-deterministic behavior (variance in syscall timing)
- Measure transpilation performance percentiles

**Example Output**:
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
 65.43    0.142301        4234        42         0 read
 18.92    0.041234        2062        20         0 write
 10.23    0.022301         892        25         0 openat
------ ----------- ----------- --------- --------- ----------------
100.00    0.217649                   107         0 total

Latency Percentiles (microseconds):
  Syscall     P50     P75     P90     P95     P99
  --------  -----   -----   -----   -----   -----
  read       2834    4123    5234    6123    9234
  write      1823    2234    3123    4234    7123

Post-Hoc Anomaly Detection (threshold: 3.0σ):
  2 anomalies detected:
  - read: 9234 μs (4.2σ above mean)
  - write: 7123 μs (3.8σ above mean)
```

---

### 7. Real-Time Anomaly Detection (--anomaly-realtime)

**Capabilities**:
- ✅ Live monitoring with sliding window baselines (Sprint 20)
- ✅ Per-syscall baselines (independent windows)
- ✅ Severity classification: Low (3-4σ), Medium (4-5σ), High (>5σ)
- ✅ Anomaly summary reports
- ✅ Configurable window size (default: 100)

**CLI Flags**:
```bash
# Real-time anomaly detection
renacer --anomaly-realtime -- ./app

# Custom window size
renacer --anomaly-realtime --anomaly-window-size 200 -- ./app

# Combined with statistics
renacer -c --anomaly-realtime -- cargo test

# Monitor only file operations
renacer --anomaly-realtime -e trace=file -- find /usr
```

**Depyler Use Cases**:
- Detect unexpected slow syscalls during transpilation
- Identify non-deterministic performance issues
- Monitor transpilation in CI/CD pipelines

**Example Output**:
```
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY) = 3
read(3, buf, 832) = 832
⚠️  ANOMALY: write took 5234 μs (4.2σ from baseline 102.3 μs) - 🟡 Medium
write(1, "processing...", 14) = 14
⚠️  ANOMALY: fsync took 8234 μs (6.3σ from baseline 123.4 μs) - 🔴 High

=== Real-Time Anomaly Detection Report ===
Severity Distribution:
  🔴 High (>5.0σ):   2 anomalies
  🟡 Medium (4-5σ): 5 anomalies
  🟢 Low (3-4σ):    5 anomalies
```

---

### 8. HPU-Accelerated Analysis (--hpu-analysis)

**Capabilities**:
- ✅ Correlation matrix analysis (Sprint 21)
- ✅ K-means clustering (group syscalls into clusters)
- ✅ Adaptive backend (automatic GPU/CPU selection)
- ✅ CPU fallback (`--hpu-cpu-only`)
- ✅ Zero overhead when disabled

**CLI Flags**:
```bash
# HPU-accelerated analysis (GPU if available)
renacer -c --hpu-analysis -- ./heavy-io-app

# Force CPU backend
renacer -c --hpu-analysis --hpu-cpu-only -- ./app

# HPU with filtering
renacer -c --hpu-analysis -e trace=file -- ls
```

**Depyler Use Cases**:
- Discover syscall patterns in transpilation
- Cluster similar transpilation runs
- Identify correlations between syscalls

**Output**: Adds correlation matrix and clustering results to statistics

---

### 9. ML Anomaly Detection (--ml-anomaly)

**Capabilities**:
- ✅ KMeans clustering using Aprender library (Sprint 23)
- ✅ Silhouette score for cluster quality (-1 to 1)
- ✅ Cluster analysis for high-latency outliers
- ✅ Comparison with Z-score methods (`--ml-compare`)
- ✅ Configurable clusters (`--ml-clusters N`)
- ✅ Zero overhead when disabled

**CLI Flags**:
```bash
# ML-based anomaly detection
renacer -c --ml-anomaly -- cargo build

# Custom cluster count
renacer -c --ml-anomaly --ml-clusters 5 -- ./app

# Compare ML with z-score
renacer -c --ml-anomaly --ml-compare -- ./app

# JSON output with ML results
renacer --ml-anomaly --format json -- ./app > ml.json
```

**Depyler Use Cases**:
- Detect unusual transpilation patterns
- Group similar transpilation runs
- Compare transpilation behavior across Python files

---

### 10. Isolation Forest Outlier Detection (--ml-outliers)

**Capabilities**:
- ✅ Isolation Forest-based outlier detection (Sprint 22)
- ✅ Contamination threshold (default: 0.1, range: 0.0-0.5)
- ✅ Configurable tree count (default: 100, min: 10)
- ✅ Explainability (`--explain`) for outlier reasoning

**CLI Flags**:
```bash
# Isolation Forest outlier detection
renacer -c --ml-outliers -- ./app

# Custom contamination threshold
renacer -c --ml-outliers --ml-outlier-threshold 0.05 -- ./app

# Custom tree count
renacer -c --ml-outliers --ml-outlier-trees 200 -- ./app

# With explainability
renacer -c --ml-outliers --explain -- ./app
```

**Depyler Use Cases**:
- Detect outlier syscalls in transpilation
- Identify anomalous transpilation runs

---

### 11. Deep Learning Anomaly Detection (--dl-anomaly)

**Capabilities**:
- ✅ Autoencoder-based anomaly detection (Sprint 23)
- ✅ Reconstruction error threshold (default: 2.0)
- ✅ Configurable hidden layer size (default: 3)
- ✅ Configurable training epochs (default: 100)

**CLI Flags**:
```bash
# Autoencoder anomaly detection
renacer -c --dl-anomaly -- ./app

# Custom threshold
renacer -c --dl-anomaly --dl-threshold 1.5 -- ./app

# Custom network architecture
renacer -c --dl-anomaly --dl-hidden-size 5 --dl-epochs 200 -- ./app
```

**Depyler Use Cases**:
- Detect complex non-linear anomalies in transpilation
- Learn normal transpilation patterns

---

### 12. **Transpiler Source Mapping (--transpiler-map)** ⭐ CRITICAL FOR DEPYLER

**Capabilities**:
- ✅ Multi-language support (Python→Rust, C→Rust, TypeScript→Rust) (Sprint 24-28)
- ✅ JSON source map parsing with version validation
- ✅ Line number mapping (Rust line → Python line)
- ✅ Function name mapping (Rust function → Python function/description)
- ✅ CLI integration via `--transpiler-map FILE.json`
- ✅ Graceful error handling (invalid JSON, missing files, unsupported versions)
- ✅ Full feature integration (works with --function-time, --rewrite-stacktrace, --rewrite-errors)

**Source Map Format** (v1.0):
```json
{
  "version": 1,
  "source_language": "python",
  "source_file": "script.py",
  "generated_file": "script.rs",
  "mappings": [
    {
      "rust_line": 192,
      "rust_function": "process_data",
      "python_line": 143,
      "python_function": "process_data",
      "python_context": "x = position[0]"
    }
  ],
  "function_map": {
    "_cse_temp_0": "temporary for: len(data) > 0"
  }
}
```

**CLI Flags**:
```bash
# Load Python→Rust source map
renacer --transpiler-map script.rs.sourcemap.json -- ./script

# Combined with DWARF debug info
renacer --transpiler-map map.json --source -- ./app

# Function profiling with source maps
renacer --transpiler-map map.json --function-time -- ./binary

# Source mapping with statistics
renacer --transpiler-map map.json -c -- ./binary

# Show verbose transpiler context
renacer --transpiler-map map.json --show-transpiler-context -- ./app

# Rewrite stack traces to original source
renacer --transpiler-map map.json --rewrite-stacktrace -- ./app

# Rewrite compilation errors to original source
renacer --transpiler-map map.json --rewrite-errors -- ./app
```

**Depyler Use Cases**:
- **PRIMARY**: Map Rust compilation errors back to Python source line numbers
- Debug transpiled code by seeing Python line numbers instead of Rust
- Profile transpiled code with Python function names
- Understand which Python code generated slow Rust code

**Workflow**:
1. Depyler generates `script.rs.sourcemap.json` during transpilation
2. Compile Rust with debug info: `rustc -g script.rs`
3. Trace with source map: `renacer --transpiler-map script.rs.sourcemap.json -s -T -- ./script`
4. **Result**: See Python line numbers, function names instead of Rust internals!

**Example Output**:
```
# Without source map:
read(3, buf, 1024) = 42    [script.rs:192 in process_data]

# With source map:
read(3, buf, 1024) = 42    [script.py:143 in process_data] (Python context: x = position[0])
```

**CRITICAL MISSING FEATURE**: Depyler does NOT currently generate source maps!
**ACTIONABLE**: Implement source map generation in Depyler (DEPYLER-XXXX)

---

### 13. Decision Trace Capture (--trace-transpiler-decisions)

**Capabilities**:
- ✅ Capture transpiler compile-time decisions (Sprint 26-27)
- ✅ Memory-mapped MessagePack files (v2.0 spec)
- ✅ Hash-based decision IDs (u64) for performance
- ✅ Decision manifest (hash → description mapping)
- ✅ Sampling and rate limiting (10,000 traces/sec limit)
- ✅ Circuit breaker DoS protection
- ✅ Xorshift64 RNG for fast randomized sampling

**Decision Trace v2.0 Specification** (Sprint 27):
```rust
// Decision format
struct Decision {
    id: u64,           // FNV-1a hash of description
    timestamp: u64,    // Microseconds since epoch
    category: u8,      // 0=Parser, 1=HIR, 2=Codegen, etc.
    result: i32,       // Decision result (-1=error, 0=skip, 1=success)
}

// Manifest format (separate file)
{
  "version": 2,
  "transpiler": "depyler",
  "decisions": {
    "12345678901234": "parse_function: Detected argparse subcommand",
    "98765432109876": "emit_match: Generated match expression"
  }
}
```

**CLI Flags**:
```bash
# Trace transpiler decisions
renacer --trace-transpiler-decisions -- cargo run --bin depyler -- transpile script.py

# Combined with timing
renacer --trace-transpiler-decisions -T -- ./depyler

# With source maps
renacer --transpiler-map map.json --trace-transpiler-decisions -- ./app
```

**Depyler Use Cases**:
- **CRITICAL FOR DEBUGGING**: See which transpiler decisions were made
- Understand why certain code was generated
- Debug argparse subcommand detection (DEPYLER-0456 Bug #1)
- Track type inference decisions (DEPYLER-0457)
- Identify where transpiler makes wrong choices

**Example Decisions to Trace**:
```python
# In argparse_transform.rs
emit_decision!("argparse.subcommand.detected", "init");
emit_decision!("argparse.subcommand.registered", "init");
emit_decision!("argparse.enum.variant.added", "Init");

# In type_inference.rs
emit_decision!("type.inferred", "column: serde_json::Value");  # WRONG!
emit_decision!("type.should_be", "column: String");            # CORRECT
```

**CRITICAL MISSING FEATURE**: Depyler does NOT emit decision traces!
**ACTIONABLE**: Implement decision trace emission in Depyler (DEPYLER-XXXX)

---

### 14. OpenTelemetry OTLP Export (--otlp-endpoint)

**Capabilities**:
- ✅ Distributed tracing via OpenTelemetry (Sprint 30)
- ✅ OTLP protocol support (gRPC port 4317, HTTP port 4318)
- ✅ Span hierarchy (root span per process + child spans per syscall)
- ✅ Rich attributes (syscall name, result, duration, source location)
- ✅ Error tracking (failed syscalls marked with ERROR status)
- ✅ Observability backends (Jaeger, Grafana Tempo, Elastic APM, Honeycomb)
- ✅ Async export with Tokio runtime
- ✅ Zero overhead when disabled
- ✅ Full integration with all renacer features

**CLI Flags**:
```bash
# Export to Jaeger
docker-compose -f docker-compose-jaeger.yml up -d
renacer --otlp-endpoint http://localhost:4317 --otlp-service-name depyler -- cargo run --bin depyler -- transpile script.py
# Open http://localhost:16686 to view traces

# With source correlation
renacer -s --otlp-endpoint http://localhost:4317 -- ./app

# With timing
renacer -T --otlp-endpoint http://localhost:4317 -- ./app

# With filtering
renacer -e trace=file --otlp-endpoint http://localhost:4317 -- ./app

# Unified tracing: OTLP + transpiler decisions
renacer --otlp-endpoint http://localhost:4317 --trace-transpiler-decisions -- ./app

# Full observability stack
renacer --otlp-endpoint http://localhost:4317 \
        --trace-transpiler-decisions \
        --transpiler-map app.sourcemap.json \
        -T \
        -- ./depyler-app
```

**Depyler Use Cases**:
- Visualize transpilation pipeline in Jaeger UI
- Identify bottlenecks in parser/HIR/codegen phases
- Compare transpilation runs side-by-side
- Export telemetry for CI/CD dashboards

**Jaeger UI View**:
```
Service: depyler
  Root Span: "cargo run --bin depyler -- transpile script.py" (1.2s)
    Child Span: "syscall: openat" (0.2ms) [script.py]
      Attributes:
        - syscall.name = "openat"
        - syscall.result = 3
        - code.filepath = "script.py"
    Child Span: "syscall: read" (0.5ms) [src/parser.rs:123]
      Attributes:
        - syscall.name = "read"
        - syscall.result = 234
        - code.filepath = "src/parser.rs"
        - code.lineno = 123
```

---

### 15. Distributed Tracing (W3C Trace Context)

**Capabilities**:
- ✅ W3C Trace Context propagation (Sprint 33)
- ✅ Context injection via `--trace-parent` flag
- ✅ Environment variable detection (`TRACEPARENT`, `OTEL_TRACEPARENT`)
- ✅ Parent-child span relationships
- ✅ Cross-service correlation
- ✅ Trace ID preservation
- ✅ Backward compatible (auto-generates if absent)

**W3C Traceparent Format**:
```
00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01
|  |                                |                  |
|  trace-id (32 hex)                parent-id (16 hex) trace-flags
version
```

**CLI Flags**:
```bash
# Auto-detect from environment
export TRACEPARENT="00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"
renacer --otlp-endpoint http://localhost:4317 -- ./app

# Explicit context injection
renacer --otlp-endpoint http://localhost:4317 \
        --trace-parent "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-00" \
        -- ./app

# Full distributed tracing stack
renacer --otlp-endpoint http://localhost:4317 \
        --trace-parent "00-abc123-def456-01" \
        --trace-compute \
        --trace-transpiler-decisions \
        -c --stats-extended \
        -- ./app
```

**Depyler Use Cases**:
- Link Depyler transpilation to broader CI/CD pipeline traces
- Correlate transpilation with test runs
- Debug multi-stage build systems

---

### 16. Compute Block Tracing (--trace-compute)

**Capabilities**:
- ✅ Trace statistical computation blocks (Trueno SIMD operations) (Sprint 32)
- ✅ Adaptive sampling (default: trace blocks ≥100μs)
- ✅ Custom thresholds (`--trace-compute-threshold N`)
- ✅ Debug mode (`--trace-compute-all` bypasses sampling)
- ✅ Rich metrics (block name, duration, element count, operation type)
- ✅ Performance insights for expensive computations
- ✅ Zero overhead when disabled

**CLI Flags**:
```bash
# Default: Adaptive sampling (>=100μs)
renacer --otlp-endpoint http://localhost:4317 \
        --trace-compute \
        -c --stats-extended \
        -- cargo build

# Debug mode: Trace ALL compute blocks
renacer --otlp-endpoint http://localhost:4317 \
        --trace-compute \
        --trace-compute-all \
        -c -- ./app

# Custom threshold: Trace blocks >=50μs
renacer --otlp-endpoint http://localhost:4317 \
        --trace-compute \
        --trace-compute-threshold 50 \
        -c -- ./app

# Full observability: compute + decisions + syscalls
renacer --otlp-endpoint http://localhost:4317 \
        --trace-compute \
        --trace-transpiler-decisions \
        -- ./depyler-app
```

**Depyler Use Cases**:
- Identify expensive statistical computations in Depyler (if any)
- Profile Trueno usage in analysis passes
- Optimize compute-heavy transpilation stages

---

## Priority Features for Depyler Debugging

### Tier 1: IMMEDIATE VALUE (Ready to Use)

1. **Function Profiling** (`--function-time`)
   - **Why**: Identifies hot paths in transpilation immediately
   - **How**: `renacer --function-time --source -- cargo run --bin depyler -- transpile script.py`
   - **Fixes**: DEPYLER-0456, DEPYLER-0457 by showing where time is spent

2. **Syscall Filtering** (`-e trace=file`)
   - **Why**: Reduces noise, focuses on file I/O
   - **How**: `renacer -e trace=file -T -- cargo run --bin depyler -- transpile script.py`
   - **Fixes**: Detects excessive file reads/writes

3. **Statistics Mode** (`-c --stats-extended`)
   - **Why**: Quantifies transpilation performance
   - **How**: `renacer -c --stats-extended -- cargo run --bin depyler -- transpile script.py`
   - **Fixes**: Establishes performance baselines

### Tier 2: HIGH VALUE (Requires Depyler Changes)

4. **Transpiler Source Mapping** (`--transpiler-map`)
   - **Why**: Maps Rust errors back to Python source
   - **Status**: **Renacer supports it, but Depyler doesn't generate source maps yet!**
   - **Blocker**: Need to implement source map generation in Depyler
   - **Ticket**: DEPYLER-XXXX - Implement source map generation

5. **Decision Trace** (`--trace-transpiler-decisions`)
   - **Why**: Shows which transpiler decisions were made (critical for Bug #1 debugging)
   - **Status**: **Implemented (DEPYLER-0458)**
   - **Blocker**: None (this is now implemented)
   - **Ticket**: DEPYLER-0458 - Implement decision trace emission

### Tier 3: NICE TO HAVE (Advanced Use Cases)

6. **OTLP Export** (`--otlp-endpoint`)
   - **Why**: Visualize transpilation in Jaeger UI
   - **How**: Start Jaeger, run renacer with `--otlp-endpoint`
   - **Value**: Sprint reviews, performance comparisons

7. **ML Anomaly Detection** (`--ml-anomaly`)
   - **Why**: Detect unusual transpilation patterns
   - **How**: `renacer -c --ml-anomaly -- cargo run --bin depyler -- transpile script.py`
   - **Value**: Identify outlier transpilation runs

---

## Recommended Workflow for DEPYLER-0456 and DEPYLER-0457

### Step 1: Profile Current Transpilation (Baseline)

```bash
# Build depyler in debug mode (for DWARF symbols)
cargo build --bin depyler

# Profile transpilation with function timing
/home/noah/src/renacer/target/release/renacer \
    --function-time \
    --source \
    -e trace=file \
    -- cargo run --bin depyler -- transpile /tmp/test_subcommands_basic.py \
    > /tmp/profile_baseline.txt

# Analyze hot paths
grep "Hot Paths" /tmp/profile_baseline.txt -A 20
```

**Expected Output**:
```
Top 10 Hot Paths (by total time):
  1. depyler_core::rust_gen::argparse_transform::preregister_subcommands_from_hir - 25.3% (340ms)
  2. depyler_core::rust_gen::func_gen::codegen_function_body - 18.2% (245ms)
  3. depyler_core::parser::parse_file - 15.6% (210ms)
```

### Step 2: Add Decision Traces to Depyler (Future Work)

**File**: `crates/depyler-core/src/rust_gen/argparse_transform.rs`

```rust
// Add at top of file
use crate::decision_trace::emit_decision;

// In preregister_subcommands_from_hir()
pub fn preregister_subcommands_from_hir(...) {
    // ... existing code ...

    if method == "add_parser" {
        let command_name = extract_string_from_hir(&args[0]);

        // EMIT DECISION TRACE
        emit_decision!("argparse.subcommand.detected", &command_name);

        tracker.register_subcommand(command_name.clone(), subcommand_info);

        // EMIT DECISION TRACE
        emit_decision!("argparse.subcommand.registered", &command_name);
    }
}

// In generate_commands_enum()
fn generate_commands_enum(...) {
    for (name, info) in tracker.subcommands() {
        // EMIT DECISION TRACE
        emit_decision!("argparse.enum.variant.added", name);

        // ... generate variant ...
    }
}
```

### Step 3: Run Transpilation with Decision Traces

```bash
# Transpile with decision traces
/home/noah/src/renacer/target/release/renacer \
    --trace-transpiler-decisions \
    --function-time \
    --source \
    -- cargo run --bin depyler -- transpile /tmp/test_subcommands_basic.py \
    > /tmp/profile_with_decisions.txt

# Analyze decisions
grep "DECISION" /tmp/profile_with_decisions.txt
```

**Expected Output** (after implementation):
```
DECISION: argparse.subcommand.detected: "init"
DECISION: argparse.subcommand.detected: "get"
DECISION: argparse.subcommand.detected: "set"
DECISION: argparse.subcommand.registered: "get"   # "init" MISSING!
DECISION: argparse.subcommand.registered: "set"
DECISION: argparse.enum.variant.added: "Get"
DECISION: argparse.enum.variant.added: "Set"      # "Init" MISSING!
```

**ROOT CAUSE DISCOVERED**: "init" detected but NOT registered!

### Step 4: Generate Source Maps (Future Work)

**File**: `crates/depyler-cli/src/main.rs`

```rust
// Add --source-map flag
#[arg(long)]
source_map: bool,

// After transpilation succeeds
if args.source_map {
    let source_map = generate_source_map(&python_file, &rust_file, &mappings);
    let map_path = format!("{}.sourcemap.json", rust_file);
    fs::write(map_path, serde_json::to_string_pretty(&source_map)?)?;
}
```

### Step 5: Use Source Maps for Error Mapping

```bash
# Transpile with source map generation
depyler transpile script.py --source-map -o script.rs

# Compile Rust (will fail)
rustc -g script.rs 2>&1 | tee /tmp/rust_errors.txt

# Map errors back to Python
/home/noah/src/renacer/target/release/renacer \
    --transpiler-map script.rs.sourcemap.json \
    --rewrite-errors \
    -- rustc -g script.rs
```

**Expected Output** (after implementation):
```
# Without source map:
error[E0425]: cannot find value `key` in this scope
  --> script.rs:192:34

# With source map:
error[E0425]: cannot find value `key` in this scope
  --> script.py:143:34  (Python context: print(f"Getting: {args.key}"))
```

---

## Implementation Roadmap

### Phase 1: IMMEDIATE (No Depyler Changes Required)

**Ticket**: DEPYLER-XXXX - Profile transpilation with renacer

1. Profile argparse transpilation with `--function-time`
2. Identify hot paths in preregister_subcommands_from_hir()
3. Generate flamegraphs for visualization
4. Establish performance baselines

**Estimated Time**: 1-2 hours
**Deliverable**: Flamegraphs, hot path analysis, performance baselines

### Phase 2: SHORT-TERM (Minor Depyler Changes)

**Ticket**: DEPYLER-0458 - Implement decision trace emission (COMPLETED)

1. Add `decision_trace` module to Depyler
2. Implement `emit_decision!()` macro
3. Add decision traces to argparse_transform.rs
4. Add decision traces to type_inference.rs
5. Test decision trace output with renacer

**Estimated Time**: 4-6 hours
**Deliverable**: Decision traces for debugging DEPYLER-0456, DEPYLER-0457

### Phase 3: MEDIUM-TERM (Significant Depyler Changes)

**Ticket**: DEPYLER-XXXX - Implement source map generation

1. Design source map format (follow v1.0 spec)
2. Track line mappings during code generation
3. Track function mappings during code generation
4. Serialize source maps to JSON
5. Add `--source-map` CLI flag
6. Test source map loading with renacer

**Estimated Time**: 8-12 hours
**Deliverable**: Source map generation, error mapping to Python

### Phase 4: LONG-TERM (Advanced Features)

**Ticket**: DEPYLER-XXXX - OTLP export integration

1. Export transpilation telemetry to Jaeger
2. Create Jaeger dashboards for transpilation
3. Integrate with CI/CD pipelines
4. Performance regression detection

**Estimated Time**: 12-16 hours
**Deliverable**: Full observability stack for Depyler

---

## Conclusion

Renacer provides **15 major feature categories** with **50+ CLI flags** for comprehensive binary analysis. The most valuable features for Depyler debugging are:

1. **Function Profiling** - Immediate hot path identification
2. **Decision Traces** - Debug transpiler logic (requires Depyler implementation)
3. **Source Maps** - Map errors back to Python (requires Depyler implementation)
4. **OTLP Export** - Visual analysis in Jaeger

**Critical Gap**: Depyler currently does NOT generate source maps or emit decision traces. These features must be implemented to unlock renacer's full debugging power.

**Next Steps**:
1. Profile current transpilation (Phase 1 - ready now)
2. Implement decision trace emission (Phase 2 - 4-6 hours)
3. Implement source map generation (Phase 3 - 8-12 hours)
4. Use renacer to discover hidden patterns in transpilation

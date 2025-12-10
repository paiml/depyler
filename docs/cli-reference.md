# Depyler CLI Reference

Complete command-line interface documentation for the Depyler Python-to-Rust
transpiler.

## Installation

```bash
# From source (recommended for development)
git clone https://github.com/paiml/depyler
cd depyler
cargo install --path crates/depyler

# Verify installation
depyler --version
```

## Global Options

```bash
depyler [OPTIONS] <COMMAND>

Options:
  -v, --verbose          Enable verbose output
  -q, --quiet           Suppress non-error output
  -h, --help            Print help information
  -V, --version         Print version information
```

## Commands

### `transpile` - Convert Python to Rust

Convert Python source files to idiomatic Rust code with optional verification.

```bash
depyler transpile [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file (.py)

Options:
  -o, --output <FILE>   Output Rust file (default: INPUT.rs)
  --verify              Enable verification during transpilation
  --verify-level <LEVEL>
                        Verification level [default: basic]
                        [possible values: none, basic, strict, paranoid]
  --no-format          Don't format generated Rust code
  --emit-hir           Also emit HIR intermediate representation
  --target <TARGET>    Target Rust edition [default: 2021]
  -f, --force          Overwrite existing output files
```

#### Examples

```bash
# Basic transpilation
depyler transpile examples/showcase/binary_search.py

# Transpile with strict verification
depyler transpile input.py --verify-level strict -o output.rs

# Transpile multiple files
depyler transpile examples/showcase/*.py --verify

# Generate HIR for debugging
depyler transpile input.py --emit-hir --output debug/
```

#### Verification Levels

- **none**: No verification, fastest transpilation
- **basic**: Type safety and basic property checks
- **strict**: Full property verification with contracts
- **paranoid**: Formal verification with proof generation

### `verify` - Verify Python Code Properties

Analyze Python code for transpilation compatibility and safety properties.

```bash
depyler verify [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  --property-tests      Generate property-based tests
  --contracts          Verify pre/post conditions
  --quickcheck         Enable QuickCheck integration
  --termination        Verify loop termination
  --bounds-check       Verify array bounds safety
  --memory-safety      Verify memory usage patterns
  --report <FORMAT>    Output format [default: text]
                       [possible values: text, json, markdown]
```

#### Examples

```bash
# Basic verification
depyler verify examples/showcase/

# Full verification suite
depyler verify input.py --property-tests --contracts --quickcheck

# Generate verification report
depyler verify project/ --report json > verification_report.json
```

### `analyze` - Code Analysis and Metrics

Analyze Python code for complexity, energy efficiency, and performance
characteristics.

```bash
depyler analyze [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  --complexity         Calculate complexity metrics
  --energy             Estimate energy consumption
  --performance        Analyze performance characteristics
  --types              Show inferred type information
  --dependencies       Analyze dependency structure
  --format <FORMAT>    Output format [default: text]
                       [possible values: text, json, csv]
```

#### Examples

```bash
# Complexity analysis
depyler analyze src/ --complexity

# Energy efficiency analysis
depyler analyze compute_heavy.py --energy --performance

# Type analysis for debugging
depyler analyze input.py --types --format json
```

### `check` - Compatibility Check

Check Python code for transpilation compatibility without full conversion.

```bash
depyler check [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  --strict             Use strict compatibility rules
  --show-unsupported   List unsupported constructs
  --suggest-fixes      Suggest code modifications
  --report <FORMAT>    Output format [default: text]
```

#### Examples

```bash
# Quick compatibility check
depyler check legacy_code.py

# Detailed compatibility report
depyler check project/ --show-unsupported --suggest-fixes
```

### `init` - Initialize New Project

Create a new Depyler-compatible Python project with templates.

```bash
depyler init [OPTIONS] <NAME>

Arguments:
  <NAME>                Project name

Options:
  --template <TEMPLATE> Project template [default: basic]
                        [possible values: basic, scientific, web, cli]
  --rust-target         Include Rust target configuration
  --git                 Initialize git repository
  --license <LICENSE>   Add license file [default: MIT]
```

#### Examples

```bash
# Create basic project
depyler init my-project

# Create scientific computing project
depyler init simulation --template scientific --rust-target

# Create CLI tool project
depyler init cli-tool --template cli --git --license Apache-2.0
```

### `interactive` - Interactive Transpilation

Run interactive transpilation with annotation suggestions and real-time
feedback.

```bash
depyler interactive [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file

Options:
  --annotate           Enable annotation suggestion mode
```

#### Examples

```bash
# Interactive transpilation
depyler interactive game.py

# With annotation suggestions
depyler interactive complex_code.py --annotate
```

The interactive mode provides:

- Step-by-step transpilation feedback
- Annotation suggestions for optimization
- Interactive selection of improvements
- Diff visualization of changes
- Backup creation before modifications

### `inspect` - AST/HIR Inspection

Inspect intermediate representations during transpilation for debugging and
optimization.

```bash
depyler inspect [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file

Options:
  -r, --repr <TYPE>     Representation to inspect [default: hir]
                        [possible values: python-ast, hir, typed-hir]
  -f, --format <FORMAT> Output format [default: pretty]
                        [possible values: pretty, json, debug]
  -o, --output <FILE>   Output to file instead of stdout
```

#### Examples

```bash
# Inspect HIR with pretty formatting
depyler inspect marco_polo.py

# Get Python AST as JSON
depyler inspect code.py --repr python-ast --format json

# Save HIR analysis to file
depyler inspect complex.py --repr hir --format json -o analysis.json

# Debug representation for troubleshooting
depyler inspect broken.py --repr python-ast --format debug
```

#### Representations

- **python-ast**: Original Python AST from rustpython-parser
- **hir**: Depyler's High-level Intermediate Representation with types and
  annotations
- **typed-hir**: Enhanced HIR with additional type analysis

#### Use Cases

- Understanding Python AST structure
- Debugging transpilation issues
- Verifying annotation extraction
- Analyzing function properties (pure, terminates, panic-free)
- Optimizing code based on HIR analysis

### `lsp` - Language Server Protocol

Start the Language Server Protocol server for IDE integration.

```bash
depyler lsp [OPTIONS]

Options:
  --port <PORT>         Port to listen on [default: 6008]
  --stdio               Use stdio instead of TCP (for IDE integration)
  --log-file <FILE>     Log to file instead of stderr
  --trace               Enable trace-level logging
```

#### Examples

```bash
# Start LSP server on default port
depyler lsp

# Use stdio for VSCode integration
depyler lsp --stdio

# Start with custom port and logging
depyler lsp --port 9999 --log-file ~/.depyler/lsp.log
```

#### IDE Configuration

**VSCode** (settings.json):

```json
{
  "depyler.lsp.path": "depyler",
  "depyler.lsp.args": ["lsp", "--stdio"]
}
```

**Neovim** (init.lua):

```lua
vim.lsp.start({
  name = 'depyler',
  cmd = {'depyler', 'lsp', '--stdio'},
  root_dir = vim.fs.dirname(vim.fs.find({'.git', 'pyproject.toml'}, { upward = true })[1]),
})
```

### `profile` - Performance Profiling

Profile Python code to analyze performance characteristics and predict Rust
performance.

```bash
depyler profile [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file

Options:
  --flamegraph          Generate flamegraph visualization
  --compare             Compare Python vs Rust performance
  --export <FILE>       Export profiling data to file
  --functions           Show per-function metrics
  --hot-paths           Identify performance hot paths
  --memory              Include memory allocation analysis
  --iterations <N>      Number of profiling iterations [default: 100]
```

#### Examples

```bash
# Basic profiling
depyler profile compute.py

# Generate flamegraph
depyler profile algorithm.py --flamegraph

# Compare Python vs Rust performance
depyler profile benchmark.py --compare

# Detailed function analysis
depyler profile complex.py --functions --hot-paths --memory

# Export profiling data
depyler profile app.py --export profile_results.json
```

#### Output Format

```
🔥 Performance Profile: compute.py
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Hot Paths:
  1. compute_matrix (45.2%) - Called 1000 times
  2. inner_loop (32.1%) - Called 1000000 times
  3. validate_input (8.7%) - Called 1000 times

Performance Predictions:
  Python execution time: 2.34s
  Rust execution time (estimated): 0.18s
  Speedup: 13x
  Memory reduction: 78%

Optimization Suggestions:
  ⚡ Use Iterator::fold instead of manual accumulation
  ⚡ Consider SIMD operations for vector math
  ⚡ Pre-allocate collections with known sizes
```

### `docs` - Documentation Generation

Generate documentation from Python source code.

```bash
depyler docs [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Python source file or directory

Options:
  -o, --output <DIR>    Output directory [default: ./docs]
  --format <FORMAT>     Documentation format [default: markdown]
                        [possible values: markdown, html, json]
  --level <LEVEL>       Documentation detail level [default: api]
                        [possible values: api, usage, full]
  --include-examples    Include usage examples from docstrings
  --include-tests       Generate test documentation
  --theme <THEME>       HTML theme (for HTML format)
                        [possible values: light, dark, auto]
```

#### Examples

```bash
# Generate API documentation
depyler docs mymodule.py

# Generate full documentation suite in HTML
depyler docs src/ --format html --level full --output ./api-docs

# Include examples and tests
depyler docs library.py --include-examples --include-tests

# Generate JSON for custom processing
depyler docs api.py --format json --output api.json
```

#### Documentation Levels

- **api**: Function signatures, parameters, return types
- **usage**: API + usage examples and patterns
- **full**: Complete documentation including internals

### `debug` - Debugging Support

Generate debugging information and helper scripts.

```bash
depyler debug [OPTIONS] <SUBCOMMAND>

Subcommands:
  tips              Show debugging tips and best practices
  generate-scripts  Generate debugger scripts for transpiled code
  source-map        Generate source mapping information

Options (for generate-scripts):
  <INPUT>           Transpiled Rust file
  --debugger <TYPE> Debugger type [default: gdb]
                    [possible values: gdb, lldb, rr]
  --output <FILE>   Output script file
```

#### Examples

```bash
# Show debugging tips
depyler debug tips

# Generate GDB script
depyler debug generate-scripts output.rs

# Generate LLDB script
depyler debug generate-scripts output.rs --debugger lldb

# Generate source mapping
depyler debug source-map input.py output.rs
```

#### Debug Levels in Transpilation

When transpiling with debug support:

```bash
# Enable debug information
depyler transpile input.py --debug

# Generate source mapping
depyler transpile input.py --debug --source-map

# Full debug mode
depyler transpile input.py --debug --source-map --verify-level none
```

### `explain` - Explainable Transpilation Errors (GH-214)

Analyze Rust compilation errors and correlate them with transpiler decisions.
Provides human-readable explanations of why transpilation produced certain errors.

```bash
depyler explain [OPTIONS] <INPUT>

Arguments:
  <INPUT>               Rust source file (.rs) from transpilation

Options:
  --trace <FILE>        Optional trace file from transpilation
  --error-code <CODE>   Filter to specific error code (e.g., E0277)
  --verbose             Show detailed transpiler decision trace
  --format <FORMAT>     Output format [default: terminal]
                        [possible values: terminal, json]
```

#### Examples

```bash
# Analyze transpiled Rust file for errors
depyler explain output.rs

# Filter to specific error type
depyler explain output.rs --error-code E0277

# Get detailed verbose output
depyler explain output.rs --verbose

# JSON output for automation
depyler explain output.rs --format json
```

#### Output Format (Terminal)

```
🔍 Depyler Explain (Issue #214)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📄 Analyzing: output.rs

📊 Found 5 compilation errors:

  ❌ Error #1: E0277 - the trait bound is not satisfied
     Location: output.rs:42:15
     Category: TypeMapping
     Decision: Transpiler chose HashMap but method expects trait

  ❌ Error #2: E0599 - no method named `xyz` found
     Location: output.rs:78:9
     Category: MethodResolution
     Decision: Python method not mapped to Rust equivalent

📈 Error Summary:
  E0277: 2 (Type trait errors)
  E0599: 3 (Method resolution errors)

💡 Recommendations:
  - E0277: Add type annotations in Python source
  - E0599: Check stdlib mapping documentation
```

#### Output Format (JSON)

```json
{
  "file": "output.rs",
  "total_errors": 5,
  "errors": [
    {
      "code": "E0277",
      "message": "the trait bound is not satisfied",
      "line": 42,
      "column": 15,
      "category": "TypeMapping",
      "decision_trace": "Transpiler chose HashMap for dict type"
    }
  ],
  "summary": {
    "E0277": 2,
    "E0599": 3
  }
}
```

#### Error Categories

| Category | Description | Common Causes |
|----------|-------------|---------------|
| TypeMapping | Type trait errors | Missing type annotations |
| MethodResolution | Method not found | Unmapped Python methods |
| Ownership | Borrow checker | Complex ownership patterns |
| LifetimeInfer | Lifetime errors | Reference patterns |

---

### `converge` - Automated Compilation Convergence (GH-158)

Automated convergence loop to achieve 100% compilation rate across all examples.
The command iteratively compiles examples, classifies errors by root cause, and
optionally applies fixes to reach the target compilation rate.

```bash
depyler converge [OPTIONS]

Options:
  -i, --input-dir <DIR>        Directory containing Python examples [default: ./examples]
  -t, --target-rate <RATE>     Target compilation rate (0-100) [default: 100]
  -m, --max-iterations <N>     Maximum iterations before stopping [default: 50]
      --auto-fix               Automatically apply transpiler fixes
      --dry-run                Show what would be fixed without applying
      --fix-confidence <F>     Minimum confidence for auto-fix (0.0-1.0) [default: 0.8]
      --checkpoint-dir <DIR>   Directory to save/resume checkpoints
  -p, --parallel-jobs <N>      Number of parallel compilation jobs [default: 4]
      --display <MODE>         Display mode [default: rich]
                               [possible values: rich, minimal, json, silent]
```

#### Examples

```bash
# Basic convergence check (dry run)
depyler converge --input-dir examples --dry-run

# Run until 100% compilation rate
depyler converge --input-dir examples --target-rate 100

# Auto-fix with checkpointing for resumability
depyler converge --input-dir examples --auto-fix --checkpoint-dir .converge

# Run with lower confidence threshold
depyler converge --input-dir examples --auto-fix --fix-confidence 0.6

# Target 95% compilation rate with parallel jobs
depyler converge --input-dir examples --target-rate 95 --parallel-jobs 8
```

#### Output Format

```
╔══════════════════════════════════════════════════════════════╗
║               DEPYLER CONVERGENCE LOOP                       ║
╠══════════════════════════════════════════════════════════════╣
║ Input Directory: ./examples                                  ║
║ Target Rate:     100.0%                                      ║
║ Max Iterations:     50                                       ║
║ Auto-fix:           ON                                       ║
╚══════════════════════════════════════════════════════════════╝

┌──────────────────────────────────────────────────────────────┐
│ Iteration   1 │ Rate:  75.0% │ Passing:   75/100            │
├──────────────────────────────────────────────────────────────┤
│ Top Cluster: E0599 (15 blocked, 90% confidence)              │
│ Root Cause: missing_method @ expr_gen.rs                     │
└──────────────────────────────────────────────────────────────┘

╔══════════════════════════════════════════════════════════════╗
║                    CONVERGENCE COMPLETE                      ║
╠══════════════════════════════════════════════════════════════╣
║ Status:          ✅ TARGET REACHED                           ║
║ Final Rate:      100.0% (100/100)                            ║
║ Iterations:          5                                       ║
║ Fixes Applied:       3                                       ║
╚══════════════════════════════════════════════════════════════╝
```

#### Error Categories

The convergence loop classifies errors into categories:

| Error Code | Category | Description |
| ---------- | -------- | ----------- |
| E0599 | TranspilerGap | Missing method or field |
| E0308 | TranspilerGap | Type mismatch |
| E0277 | TranspilerGap | Missing trait implementation |
| E0425 | TranspilerGap | Undefined variable |
| E0433 | TranspilerGap | Missing import |
| E0382/E0502/E0507 | TranspilerGap | Borrow checker issues |

### `utol` - Unified Training Oracle Loop (Toyota Way)

Automated self-correcting compilation feedback system with rich visual feedback.
Replaces manual "Apex Hunt" prompt-driven cycles with deterministic convergence
following Toyota Way principles: Jidoka (自働化), Kaizen (改善), Andon (行灯).

```bash
depyler utol [OPTIONS]

Options:
  -c, --corpus <CORPUS>            Path to corpus directory
  -t, --target-rate <RATE>         Target compilation success rate (0.0-1.0) [default: 0.80]
  -m, --max-iterations <N>         Maximum iterations before stopping [default: 50]
      --patience <N>               Stop if no improvement for N iterations [default: 5]
      --display <MODE>             Display mode [default: rich]
                                   [possible values: rich, minimal, json, silent]
  -o, --output <FILE>              Output results to JSON file
      --config <FILE>              Path to YAML configuration file
      --status                     Show status and exit (don't run loop)
  -w, --watch                      Watch mode: continuously monitor corpus and re-run on changes
      --watch-debounce <MS>        Watch debounce interval in milliseconds [default: 500]
```

#### Examples

```bash
# Show current UTOL status (model, corpus, target)
depyler utol --status

# Run UTOL loop on default corpus
depyler utol

# Run on custom corpus with 90% target
depyler utol --corpus ./my-python-project --target-rate 0.90

# Minimal output for CI pipelines
depyler utol --corpus ./src --display minimal --max-iterations 10

# Export results to JSON
depyler utol --corpus ./examples --output utol_results.json

# Silent mode for automation
depyler utol --corpus ./src --display silent --output results.json

# Watch mode - continuously monitor and re-run on file changes
depyler utol --watch

# Watch mode with custom debounce (2 seconds)
depyler utol --watch --watch-debounce 2000

# Watch mode with limited iterations per run
depyler utol --watch --max-iterations 5
```

#### Output Format (Rich Mode)

```
╔══════════════════════════════════════════════════════════════════════╗
║                    UTOL - Unified Training Oracle Loop               ║
╠══════════════════════════════════════════════════════════════════════╣
║  Corpus: ../reprorusted-python-cli                                   ║
║  Target: 80.0%                                                       ║
║  Max Iterations: 50                                                  ║
╚══════════════════════════════════════════════════════════════════════╝

Iteration: [████████████░░░░░░░░] 12/50 (24%)
Compile Rate: 75.3% → Target: 80.0%  ✓ ON TRACK

Compile Rate: ▁▂▃▄▅▆▇█ 75.3% (+2.1%)
Drift Status: ● STABLE

═══════════════════════════════════════════════════════════════════════
                         UTOL Final Report
═══════════════════════════════════════════════════════════════════════

  Status:       ✅ CONVERGED
  Final Rate:   82.1%
  Iterations:   15
  Duration:     45.3s
  Model:        oracle-utol-3.21.0
```

#### Configuration File

Create `.depyler/utol.yaml` for persistent configuration:

```yaml
utol:
  version: "1.0"

  corpus:
    path: "../my-corpus"
    include_patterns:
      - "**/*.py"
    exclude_patterns:
      - "**/test_*.py"
      - "**/__pycache__/**"

  convergence:
    target_rate: 0.80
    max_iterations: 50
    patience: 5
    min_delta: 0.005

  display:
    mode: "rich"
    refresh_ms: 500
    show_sparklines: true
```

#### Toyota Way Principles

| Principle | Japanese | Application in UTOL |
|-----------|----------|---------------------|
| **Jidoka** | 自働化 | Auto-stop on failure, self-diagnose |
| **Kaizen** | 改善 | Each iteration improves incrementally |
| **Andon** | 行灯 | Visual feedback system (sparklines) |
| **Poka-Yoke** | ポカヨケ | Deterministic seeds prevent errors |

### `cache` - Compilation Cache Management (DEPYLER-CACHE-001)

O(1) incremental compilation cache for avoiding redundant transpilation work.
Implements the WiscKey pattern with SQLite index + Content-Addressable Storage (CAS).

```bash
depyler cache <COMMAND>

Commands:
  stats    Show cache statistics
  gc       Run garbage collection to reclaim space
  clear    Clear the entire cache
  warm     Warm cache by pre-transpiling files
```

#### `cache stats` - View Cache Statistics

```bash
depyler cache stats [OPTIONS]

Options:
  -c, --cache-dir <PATH>   Cache directory [default: ~/.depyler/cache]
  -f, --format <FORMAT>    Output format [default: text]
                           [possible values: text, json]
```

##### Examples

```bash
# View cache statistics
depyler cache stats

# JSON output for scripting
depyler cache stats --format json

# Custom cache directory
depyler cache stats --cache-dir /tmp/depyler-cache
```

##### Output Format (Text)

```
📊 Cache Statistics (DEPYLER-CACHE-001)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📁 Cache directory: /home/user/.depyler/cache
📦 Entries: 142
💾 Total size: 15.43 MB
✅ Cache hits: 1,247
❌ Cache misses: 89
📈 Hit rate: 93.3%
```

##### Output Format (JSON)

```json
{
  "entries": 142,
  "total_size_bytes": 16178585,
  "hits": 1247,
  "misses": 89,
  "hit_rate": 0.933,
  "cache_dir": "/home/user/.depyler/cache"
}
```

#### `cache gc` - Garbage Collection

Run LRU-based garbage collection to reclaim disk space.

```bash
depyler cache gc [OPTIONS]

Options:
  -c, --cache-dir <PATH>        Cache directory [default: ~/.depyler/cache]
      --max-size-mb <SIZE>      Maximum cache size in MB [default: 1024]
      --max-age-hours <HOURS>   Maximum entry age in hours [default: 168]
      --dry-run                 Show what would be deleted without deleting
```

##### Examples

```bash
# Run garbage collection with defaults (1GB max, 7 days max age)
depyler cache gc

# Limit cache to 500MB
depyler cache gc --max-size-mb 500

# Keep only entries from last 24 hours
depyler cache gc --max-age-hours 24

# Dry run - see what would be deleted
depyler cache gc --dry-run
```

##### Output Format

```
🧹 Garbage Collection Results
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🗑️  Entries removed: 45
💾 Space reclaimed: 8.72 MB
```

#### `cache clear` - Clear Entire Cache

Remove all cached transpilation results.

```bash
depyler cache clear [OPTIONS]

Options:
  -c, --cache-dir <PATH>   Cache directory [default: ~/.depyler/cache]
  -f, --force              Skip confirmation prompt
```

##### Examples

```bash
# Clear cache (prompts for confirmation)
depyler cache clear

# Force clear without confirmation
depyler cache clear --force

# Clear custom cache directory
depyler cache clear --cache-dir /tmp/depyler-cache --force
```

#### `cache warm` - Pre-populate Cache

Pre-transpile Python files to warm the cache for faster subsequent runs.

```bash
depyler cache warm [OPTIONS]

Options:
  -i, --input-dir <DIR>    Directory containing Python files
  -c, --cache-dir <PATH>   Cache directory [default: ~/.depyler/cache]
  -j, --jobs <N>           Number of parallel jobs [default: 4]
```

##### Examples

```bash
# Warm cache for all Python files in examples
depyler cache warm --input-dir ./examples

# Warm with 8 parallel jobs
depyler cache warm --input-dir ./src --jobs 8
```

#### Cache Architecture

The cache uses a **hermetic cache key** composed of:

| Component | Description |
|-----------|-------------|
| Source Hash | SHA256 of Python source code |
| Transpiler Hash | SHA256 of depyler binary |
| Environment Hash | Deterministic hash of env vars |
| Config Hash | Hash of transpiler configuration |

This ensures **cache invalidation** happens automatically when:
- Source code changes
- Depyler is upgraded
- Environment variables change
- Configuration changes

#### Academic Foundation

The cache design is based on peer-reviewed research:

- **Build Systems à la Carte** (Mokhov et al., ICFP 2018) - Minimal rebuild theory
- **Pluto** (Erdweg et al., PLDI 2015) - Sound incremental build
- **Nix** (Dolstra et al., ICSE 2004) - Input addressing
- **WiscKey** (Lu et al., FAST 2016) - Key-value separation pattern

## Configuration

### Project Configuration File

Create a `depyler.toml` file in your project root:

```toml
[project]
name = \"my-project\"
version = \"0.1.0\"
edition = \"2021\"

[transpilation]
verify_level = \"strict\"
target_edition = \"2021\"
format_output = true
emit_hir = false

[verification]
property_tests = true
contracts = true
quickcheck = true
termination_analysis = true

[analysis]
complexity_threshold = 10
energy_analysis = true
performance_profiling = false

[output]
preserve_comments = true
generate_docs = true
include_source_map = true
```

### Environment Variables

```bash
# Enable debug logging
export DEPYLER_LOG=debug

# Set custom verification timeout
export DEPYLER_VERIFY_TIMEOUT=30

# Configure parallel processing
export DEPYLER_THREADS=8

# Set cache directory
export DEPYLER_CACHE_DIR=\"~/.cache/depyler\"
```

## Exit Codes

| Code | Meaning               |
| ---- | --------------------- |
| 0    | Success               |
| 1    | General error         |
| 2    | Parse error           |
| 3    | Type error            |
| 4    | Verification failure  |
| 5    | Unsupported construct |
| 6    | I/O error             |
| 7    | Configuration error   |

## Error Handling

### Common Errors and Solutions

#### Parse Errors

```bash
Error: Failed to parse Python source
  --> input.py:15:23
   |
15 |     result = [x for x in range(10) if x % 2 == 0
   |                                               ^ Expected ']'

Solution: Fix Python syntax errors before transpilation
```

#### Type Inference Errors

```bash
Error: Cannot infer type for variable 'data'
  --> input.py:8:5
   |
8  |     data = get_data()
   |     ^^^^ Type annotation required

Solution: Add type hints or use --verify-level none
```

#### Unsupported Construct Errors

```bash
Error: Unsupported construct: async function
  --> input.py:12:1
   |
12 | async def fetch_data():
   | ^^^^^ async/await not supported in current version

Solution: Use synchronous alternatives or wait for async support
```

## Performance Tips

### Optimizing Transpilation Speed

1. **Use appropriate verification levels**:
   ```bash
   # Fast development iteration
   depyler transpile --verify-level none

   # Production builds
   depyler transpile --verify-level strict
   ```

2. **Parallel processing**:
   ```bash
   # Process multiple files in parallel
   export DEPYLER_THREADS=$(nproc)
   depyler transpile src/*.py
   ```

3. **Caching**:
   ```bash
   # Enable persistent caching
   export DEPYLER_CACHE_DIR=\"~/.cache/depyler\"
   ```

### Memory Usage Optimization

```bash
# For large codebases
export DEPYLER_MAX_MEMORY=8G
export DEPYLER_STREAMING=true

# Process files individually for very large projects
find src -name \"*.py\" -exec depyler transpile {} \\;
```

## Integration Examples

### Makefile Integration

```makefile
# Makefile
.PHONY: transpile verify clean

transpile:
\t@echo \"Transpiling Python to Rust...\"
\tdepyler transpile src/ --verify-level strict

verify:
\t@echo \"Verifying Python code...\"
\tdepyler verify src/ --property-tests --contracts

check:
\t@echo \"Checking compatibility...\"
\tdepyler check src/ --strict --show-unsupported

clean:
\t@echo \"Cleaning generated files...\"
\trm -rf target/ *.rs

# Continuous integration
ci: check verify transpile
\tcargo test --all
\tcargo clippy -- -D warnings
```

### GitHub Actions Integration

```yaml
# .github/workflows/depyler.yml
name: Depyler CI

on: [push, pull_request]

jobs:
  transpile:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Depyler
        run: |
          curl -sSf https://install.depyler.rs | sh
          echo \"$HOME/.depyler/bin\" >> $GITHUB_PATH

      - name: Check Python compatibility
        run: depyler check src/ --strict

      - name: Verify Python properties
        run: depyler verify src/ --property-tests --contracts

      - name: Transpile to Rust
        run: depyler transpile src/ --verify-level strict

      - name: Test generated Rust
        run: cargo test --all
```

### VS Code Integration

Create `.vscode/tasks.json`:

```json
{
    \"version\": \"2.0.0\",
    \"tasks\": [
        {
            \"label\": \"Depyler: Transpile\",
            \"type\": \"shell\",
            \"command\": \"depyler\",
            \"args\": [\"transpile\", \"${file}\", \"--verify\"],
            \"group\": \"build\",
            \"presentation\": {
                \"echo\": true,
                \"reveal\": \"always\",
                \"panel\": \"new\"
            }
        },
        {
            \"label\": \"Depyler: Verify\",
            \"type\": \"shell\",
            \"command\": \"depyler\",
            \"args\": [\"verify\", \"${file}\", \"--property-tests\"],
            \"group\": \"test\"
        }
    ]
}
```

## Troubleshooting

### Common Issues

1. **Installation Problems**:
   ```bash
   # Ensure Rust is installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Update PATH
   source ~/.cargo/env
   ```

2. **Memory Issues**:
   ```bash
   # Increase memory limit
   export DEPYLER_MAX_MEMORY=16G

   # Use streaming mode for large files
   export DEPYLER_STREAMING=true
   ```

3. **Verification Timeout**:
   ```bash
   # Increase timeout for complex verification
   export DEPYLER_VERIFY_TIMEOUT=300

   # Or disable problematic checks
   depyler verify --no-termination-analysis
   ```

### Debug Mode

```bash
# Enable comprehensive debugging
export DEPYLER_LOG=trace
export RUST_BACKTRACE=1

# Run with debug output
depyler transpile input.py --verbose 2> debug.log
```

### Reporting Issues

When reporting issues, include:

1. **Version information**:
   ```bash
   depyler --version
   rustc --version
   python --version
   ```

2. **Minimal reproduction case**:
   ```python
   # minimal_example.py
   def problematic_function():
       # Code that causes issues
       pass
   ```

3. **Full command and output**:
   ```bash
   depyler transpile minimal_example.py --verbose 2>&1 | tee issue_report.txt
   ```

## See Also

- [User Guide](user-guide.md) - Comprehensive usage documentation
- [Project Overview](project-overview.md) - Architecture and design
- [Energy Efficiency](energy-efficiency.md) - Sustainability focus
- [V0 Specification](v0-spec.md) - Technical specification

---

_Generated: 2025-01-04_\
_Version: 2.1.0_\
_For issues and contributions: https://github.com/paiml/depyler_

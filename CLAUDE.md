# CLAUDE.md - Depyler Compiler Implementation Protocol

## Prime Directive
**Generate correct Rust code that compiles on first attempt. Quality is built-in, not bolted-on.**

## Project Context
Depyler is a Python-to-Rust transpiler focusing on energy-efficient, safe code generation with progressive verification.

## Python Packaging Protocol
**MANDATORY: Use `uv` for ALL Python operations**
- `uv add <package>` not pip install
- `uv run pytest` not python -m pytest
- `uv run <script.py>` not python3

## Build Environment (High-Performance Storage)
**Pre-configured for maximum build speed:**

| Component | Location | Size | Purpose |
|-----------|----------|------|---------|
| Cargo Target | `/Volumes/LambdaCache/cargo-target` | 256GB | All cargo builds (APFS disk image) |
| Cargo Overflow | `/Volumes/LambdaOverflow` | - | Secondary cache |

**Configuration** (`~/.cargo/config.toml`):
```toml
[build]
jobs = 16

[profile.dev]
incremental = true

[env]
CARGO_TARGET_DIR = "/Volumes/LambdaCache/cargo-target"
```

**DO NOT create additional RAM disks** - LambdaCache is already optimized.
**DO NOT use /tmp for transpiler output** - Use LambdaCache or in-memory when possible.

## 🚨 CRITICAL: A+ Code Standard
**ABSOLUTE REQUIREMENTS**:
- Maximum Cyclomatic Complexity: ≤10
- Maximum Cognitive Complexity: ≤10
- Function Size: ≤30 lines
- Zero SATD (TODO/FIXME/HACK)
- TDD Mandatory
- Test Coverage: 80% minimum (cargo-llvm-cov)

## EXTREME TDD Protocol
**ANY TRANSPILER/CODEGEN BUG REQUIRES IMMEDIATE EXTREME TDD**:
1. HALT ALL OTHER WORK
2. Create comprehensive test suites (unit + integration + property + fuzz)
3. Add failing test BEFORE fixing
4. Validate all language features after fix

## 🔴 CRITICAL: Never Add -D warnings to Convergence Compilation

**THIS BUG HAS BEEN FIXED 100+ TIMES. DO NOT ADD IT BACK.**

**File**: `crates/depyler/src/converge/compiler.rs`
**Function**: `compile_with_cargo()`

**THE BUG**: Adding `RUSTFLAGS=-D warnings` to cargo build during convergence causes:
- Convergence rate drops from 80%+ to near 0%
- Valid code fails compilation due to unused imports
- Days of debugging wasted tracking phantom failures

**THE FIX**: DO NOT treat warnings as errors during convergence compilation.
- Code quality is enforced via clippy in CI, NOT during convergence
- Convergence needs to know if code COMPILES, not if it's warning-free
- Generated code often has unused imports that are harmless

**REGRESSION TESTS**: Three tests in `compiler.rs` prevent this:
1. `test_no_d_warnings_flag_in_source` - Static analysis, fails if -D warnings added
2. `test_regression_warnings_must_not_cause_failure` - Integration test
3. `test_uses_cargo_when_cargo_toml_exists` - Cargo path verification

**IF YOU SEE CONVERGENCE RATE NEAR 0%**:
1. Check `compile_with_cargo()` for RUSTFLAGS
2. Run `cargo test -p depyler -- test_no_d_warnings_flag`
3. If test fails, REMOVE the RUSTFLAGS line

## EXTREME CLI VALIDATION
**15-Tool Validation Gates** (MANDATORY for ALL examples):
1. rustc --deny warnings
2. clippy -D warnings
3. rustfmt --check
4. Basic compilation
5-15. LLVM IR, ASM, MIR, parse, type check, cargo tree, rustdoc, macro expansion, HIR dump, dead code, complexity

**STOP THE LINE Protocol**: NEVER bypass gates. Fix transpiler, not output.

## Scientific Method Protocol
**WE DON'T GUESS, WE PROVE**:
1. NO ASSUMPTIONS - prove with tests
2. MEASURE EVERYTHING - use metrics
3. REPRODUCE ISSUES - minimal test cases
4. QUANTIFY IMPROVEMENTS - before/after
5. DOCUMENT EVIDENCE - reproducible steps

## Debugging Toolkit
**MANDATORY: Use system-level tracing for runtime behavior analysis**

### Renacer - Pure Rust System Call Tracer
**Location**: `/home/noah/src/renacer/target/debug/renacer`

**Use Cases**:
- Debug transpiled Rust binary runtime behavior
- Identify syscall issues in generated code
- Correlate syscalls with source code (DWARF debug info)
- Profile syscall timing and frequency

**Basic Usage**:
```bash
# Trace all syscalls
/home/noah/src/renacer/target/debug/renacer -- ./target/debug/my_binary

# Filter specific syscalls (file operations)
/home/noah/src/renacer/target/debug/renacer -e trace=file -- ./target/debug/my_binary

# Show syscall timing
/home/noah/src/renacer/target/debug/renacer -T -- ./target/debug/my_binary

# Statistics summary
/home/noah/src/renacer/target/debug/renacer -c -- ./target/debug/my_binary

# Source correlation (with debug info)
/home/noah/src/renacer/target/debug/renacer -s -- ./target/debug/my_binary

# JSON output for parsing
/home/noah/src/renacer/target/debug/renacer --format json -- ./target/debug/my_binary
```

**Debugging Workflow**:
1. Transpile Python to Rust with debug info: `depyler transpile --debug <file.py>`
2. Build Rust binary: `rustc -g <output.rs>` or `cargo build`
3. Trace execution: `renacer -s -T -- ./binary`
4. Analyze syscalls to identify runtime issues
5. Fix transpiler based on findings

**Quick Alias** (add to shell):
```bash
alias renacer='/home/noah/src/renacer/target/debug/renacer'
```

**NEW: Transpiler Source Mapping & Decision Tracing (v0.5.0)**:
Renacer now supports mapping Rust errors/profiling back to original Python source, plus transpiler decision tracing!

**Source Map Format** (generated by depyler):
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

**Usage with Source Maps**:
```bash
# Basic usage with transpiler source map
renacer --transpiler-map output.rs.sourcemap.json -- ./transpiled_binary

# Combined with statistics
renacer --transpiler-map map.json -c -- ./binary

# Combined with DWARF debug info
renacer --transpiler-map map.json --source -- ./binary

# Full debugging stack (transpiler map + DWARF + timing)
renacer --transpiler-map map.json -s -T -- ./binary
```

**Debugging Workflow with Source Maps**:
1. Transpile Python with source map: `depyler transpile --source-map <file.py>`
2. Build Rust binary: `rustc -g <output.rs>` or `cargo build`
3. Trace with mapping: `renacer --transpiler-map <output.rs.sourcemap.json> -s -T -- ./binary`
4. **Result**: See Python line numbers, function names instead of Rust internals!

**Example - Debug transpiled code**:
```bash
# Transpile Python with source map
depyler transpile my_script.py --source-map -o my_script.rs
# Generates: my_script.rs + my_script.rs.sourcemap.json

# Compile with debug info
rustc -g my_script.rs -o my_script

# Trace execution with timing
renacer -T -- ./my_script

# Get statistics summary
renacer -c -- ./my_script

# Focus on file operations
renacer -e trace=file -- ./my_script
```

## QDD (Quality-Driven Development)
**Quality Metrics FIRST**:
```bash
# Before task - Create baseline
pmat tdg . --format json --with-git-context > baseline.json

# During development - Real-time monitoring
pmat tdg dashboard &

# After each function - Check progress with explanations
pmat tdg <file> --explain --threshold 10 --baseline main

# Before commit - Quality gate enforcement
pmat tdg check-quality --min-grade A-
pmat tdg check-regression --baseline baseline.json

# Repository health check
pmat rust-project-score --detailed
```

## Development Principles
### 自働化 (Jidoka) - Build Quality In
Never ship incomplete transpilation. Verification-first development.

### 現地現物 (Genchi Genbutsu) - Direct Observation
Test against real Rust. Measure actual compilation.

### 反省 (Hansei) - Fix Before Adding
Fix broken functionality before new features.

### 改善 (Kaizen) - Continuous Improvement
Incremental verification. Performance baselines.

## Critical Invariants
1. Type safety: Must pass `cargo check`
2. Determinism: Same input → identical output
3. Memory safety: No UB or leaks

## Python→Rust Transpilation Workflow
```bash
# Transpile
depyler transpile <input.py>

# With verification
depyler transpile <input.py> --verify --gen-tests

# Validate
rustc --crate-type lib --deny warnings <output.rs>
```

**MANDATORY Header** in all .rs files:
```rust
// Generated by: depyler transpile <source.py>
// Source: <source.py>
```

## 🛑 STOP THE LINE: MANDATORY Bug-Fix Protocol (NON-NEGOTIABLE)

**ABSOLUTE REQUIREMENT - NOT OPTIONAL**: When ANY defect is discovered in transpiled output, **STOP ALL WORK IMMEDIATELY**. This protocol is MANDATORY and cannot be bypassed, deferred, or delegated.

### Why This Exists
Every transpiler bug affects ALL future code generated. Fixing bugs immediately prevents:
- Technical debt accumulation
- Cascading failures in downstream projects
- Loss of user trust
- Compounding maintenance costs

### The 8-Step Protocol (MANDATORY - NO EXCEPTIONS)

1. **🛑 STOP THE LINE** - Halt ALL feature work immediately
   - Do not "finish this one thing first"
   - Do not "just commit what I have"
   - STOP means STOP

2. **📋 DOCUMENT THE BUG** - Create comprehensive bug document
   - File: `docs/bugs/DEPYLER-XXXX-<description>.md`
   - Template: See DEPYLER-0279, DEPYLER-0280 examples
   - Must include: Problem, Root Cause, Solution, Test Plan
   - Minimum 200 lines of analysis (comprehensive is required)

3. **🎫 ASSIGN TICKET NUMBER** - Sequential DEPYLER-XXXX
   - Check last ticket number in `docs/bugs/`
   - Increment by 1
   - Update roadmap.yaml with ticket reference

4. **🔍 ROOT CAUSE ANALYSIS** - Find transpiler bug source
   - Never blame "user error" - transpiler must handle it
   - Locate exact file + line number in transpiler
   - Understand WHY the bug exists (not just WHAT)

5. **🔧 FIX THE TRANSPILER** - NEVER fix generated output
   - SACRED RULE: Fix the generator, not the generated code
   - Add regression test BEFORE implementing fix
   - Ensure fix meets all quality gates (≤10 complexity)

6. **♻️ RE-TRANSPILE EVERYTHING** - Regenerate ALL affected files
   - Matrix Testing Project examples
   - Showcase examples
   - Documentation examples
   - Any other transpiled code

7. **✅ VERIFY COMPREHENSIVELY** - All quality gates must pass
   - Transpiled code compiles (rustc --deny warnings)
   - All tests pass (cargo test)
   - Quality gates pass (pmat, clippy, llvm-cov)
   - No regressions in other examples

8. **▶️ RESUME WORK** - Only after 100% verification
   - Update CLAUDE.md if protocol improved
   - Document lessons learned
   - Continue previous work

### Enforcement

**This protocol is MANDATORY and enforced by**:
- GitHub issue templates
- Pre-commit hooks
- Code review requirements
- CI/CD pipeline gates

**Violations Result In**:
- Commit rejection
- PR block
- Build failure

**NO EXCEPTIONS** - Even for:
- "Quick fixes" ❌
- "Just this once" ❌
- "We'll fix it later" ❌
- "It's only a warning" ❌

### Examples

**CORRECT** ✅:
```
Discover bug → STOP → Document → Fix transpiler → Re-transpile → Verify → Resume
Timeline: 2-4 hours of focused work
Result: Bug fixed forever, all code benefits
```

**INCORRECT** ❌:
```
Discover bug → "Note it for later" → Continue feature work
Result: Bug persists, affects all new code, technical debt grows
```

### Recent Examples

- **DEPYLER-0279**: Dict codegen bugs (2 issues fixed, 419-line doc)
- **DEPYLER-0280**: Duplicate mod tests (1 issue fixed, 582-line doc)

Both followed this protocol religiously. See `docs/bugs/` for templates.

### Full Protocol Document

Detailed procedures: [docs/processes/stop-the-line.md](docs/processes/stop-the-line.md)

**Defect Severity Classification**:
- **P0 (STOP ALL WORK)**: Compilation failures, type safety violations, memory safety issues
- **P1 (BLOCK RELEASE)**: Clippy warnings, non-idiomatic code, performance regressions
- **P2/P3 (TRACK)**: Optimization opportunities, documentation gaps

## Testing Strategy
### Multi-Level Testing (MANDATORY)
1. **Unit Tests**: ≥5 per module, 85% coverage
2. **Property Tests**: ≥3 per module, 1000 iterations each
3. **Doctests**: ≥2 per public function
4. **Integration Tests**: Full transpile→compile→execute

### Mutation Testing
```bash
cargo mutants --workspace
# Target: ≥75% mutation kill rate
```

### Test Naming Convention
```rust
#[test]
fn test_DEPYLER_XXXX_<section>_<feature>_<scenario>() {}
```

## PMAT TDG Quality Enforcement (MANDATORY - BLOCKING)

### TDG (Technical Debt Grading) Overview
TDG is PMAT's comprehensive quality metric that quantifies technical debt on a 0-5 scale:
- **0.0-1.5**: Excellent (A+/A)
- **1.5-2.0**: Good (A-/B+)
- **2.0-2.5**: Acceptable (B/B-)
- **2.5-3.5**: Warning (C)
- **3.5+**: Critical (D/F) - BLOCKS COMMITS

**Workspace Policy** (from Cargo.toml):
```toml
[workspace.metadata.quality]
max_tdg_score = 2.0  # Any file above = technical debt violation
```

### TDG Command Reference
**LATEST**: Use `pmat tdg` (top-level command with enhanced features):
```bash
# Full project analysis with component breakdown
pmat tdg . --include-components

# Function-level breakdown with explanations (Issue #78)
pmat tdg . --explain --threshold 10

# Progress tracking against baseline (Issue #78)
pmat tdg . --explain --baseline main --threshold 10

# JSON output for automation with git context
pmat tdg . --format json --with-git-context > tdg_report.json

# Dashboard mode (real-time monitoring)
pmat tdg dashboard

# Quality regression detection (Sprint 66)
pmat tdg check-regression --baseline baseline.json
pmat tdg check-quality --min-grade A-
```

### Git-Enforced TDG Quality Gates
**Pre-commit hook enforcement** (`.git/hooks/pre-commit`):
- **Scope**: Only `crates/` directory (optimized for performance)
- **Threshold**: TDG ≤ 2.0 (from workspace config)
- **Timeout**: 30 seconds (skips if exceeded)
- **Blocking**: Fails commit if critical files found (TDG > 2.5)

**Current Status** (as of 2025-10-29):
- **Total Files (crates/)**: 188
- **Critical Files**: 0 (0.0%) ✅
- **Warning Files**: 7 (3.7%)
- **Average TDG**: 0.70
- **95th Percentile**: 1.45
- **99th Percentile**: 1.79
- **Estimated Debt**: 576.9 hours

### TDG Development Workflow
**Before ANY code changes**:
```bash
# Baseline analysis with git context
pmat tdg crates --include-components --with-git-context

# Quality gate check
pmat quality-gate --fail-on-violation
```

**During development** (after each function/module):
```bash
# File-level TDG with explanations
pmat tdg <file.rs> --explain --threshold 10

# Traditional complexity (backup)
pmat analyze complexity --file <file.rs> --max-cyclomatic 10 --fail-on-violation

# SATD zero-tolerance
pmat analyze satd --path <file.rs> --fail-on-violation

# Mutation testing (Sprint 61)
pmat analyze mutate <file.rs>
```

**Before commit (MANDATORY - BLOCKS COMMITS)**:
```bash
# TDG quality check (automated in pre-commit hook)
pmat tdg check-quality --min-grade A-

# Regression detection
pmat tdg check-regression --baseline baseline.json

# Comprehensive quality gate
pmat quality-gate --fail-on-violation --format=detailed

# Coverage enforcement
cargo llvm-cov --all-features --workspace --fail-under-lines 80
```

## MANDATORY: Roadmap and Ticket Tracking
1. **ALWAYS Use Ticket Numbers**: Every commit MUST reference DEPYLER-XXX
2. **Roadmap-First Development**: No work without roadmap entry
3. **Traceability**: All changes traceable via tickets

### Commit Message Format (MANDATORY)
```
[TICKET-ID] Brief description

Detailed explanation
- Improvements
- Tests added
- Performance impact

TDG Score Changes (MANDATORY):
- src/file.rs: 85.3→87.1 (B+→A-) [+1.8]

PMAT Verification:
- Complexity: ≤10 ✅
- SATD: 0 ✅
- Coverage: 80.5% → 82.1% ✅

Closes: TICKET-ID
```

### PMAT Work Workflow (Issue #75)
**Unified GitHub/YAML workflow commands for ticket management**:

**Starting work on a ticket**:
```bash
# Start work on GitHub issue
pmat work start 123

# Start work on YAML ticket
pmat work start DEPYLER-0452

# Initialize roadmap and hooks (first time)
pmat work init
```

**During development**:
```bash
# Continue work on existing ticket
pmat work continue DEPYLER-0452

# Check work status
pmat work status

# Synchronize GitHub and YAML roadmap
pmat work sync
```

**Completing work**:
```bash
# Mark ticket as complete
pmat work complete DEPYLER-0452

# Status shows completed work
pmat work status
```

**Benefits**:
- Automatic ticket status updates
- GitHub issue synchronization
- YAML roadmap integration
- Pre-commit hook setup
- Quality gate enforcement

## MANDATORY Quality Gates (BLOCKING)
### RED-GREEN-REFACTOR Workflow
**Phase 1: RED** (Write failing tests)
```bash
cargo test test_DEPYLER_XXXX  # MUST FAIL
git commit --no-verify -m "[RED] DEPYLER-XXXX: Add failing tests"
```

**Phase 2: GREEN** (Minimal implementation)
```bash
cargo test test_DEPYLER_XXXX  # MUST PASS
git commit -m "[GREEN] DEPYLER-XXXX: Implement <feature>"
```

**Phase 3: REFACTOR** (Meet quality standards)
```bash
pmat tdg check-quality --min-grade A-
pmat analyze complexity --max-cyclomatic 10 --fail-on-violation
cargo clippy --all-targets -- -D warnings
cargo llvm-cov report --fail-under-lines 80
git commit -m "[REFACTOR] DEPYLER-XXXX: Meet quality standards"
```

### Pre-commit Hooks (MANDATORY)
**Quality Gates** (`.git/hooks/pre-commit`):
1. **Documentation Synchronization**: roadmap.md + CHANGELOG.md must be updated with code changes
2. **Complexity Enforcement**: ≤10 cyclomatic/cognitive (`pmat analyze complexity --max-cyclomatic 10`)
3. **SATD Zero-Tolerance**: No TODO/FIXME/HACK (`pmat analyze satd --fail-on-violation`)
4. **TDG Quality Check**: Min grade A- (`pmat tdg check-quality --min-grade A-`)
5. **Coverage Minimum**: ≥80% (`cargo llvm-cov --fail-under-lines 80`)
6. **Clippy Zero-Warnings**: All warnings = errors (`cargo clippy -- -D warnings`)
7. **Dead Code Detection**: `pmat analyze dead-code` (30s timeout)
8. **Duplicate Code**: `pmat analyze duplicates --threshold 0.8` (15s timeout)
9. **Mutation Testing**: `pmat analyze mutate` on changed files (Sprint 61)
10. **Hallucination Detection**: `pmat red-team commit` before push

**SACRED RULE**: NEVER `git commit --no-verify` (except RED phase in TDD)

**Performance Optimizations**:
- TDG: Only analyzes `crates/` directory (not examples/playground) with 30s timeout
- Dead Code: Only changed files, 30s timeout
- Duplicates: Only `crates/` directory, 15s timeout

## The Make Lint Contract
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
`-D warnings` treats EVERY clippy warning as hard error. Zero technical debt tolerance.

## Sprint Hygiene
**Pre-Sprint Cleanup**:
```bash
rm -f test_* debug_*
find . -type f -executable -not -path "./target/*" -delete
cargo clean
```

## The Development Flow (PMAT-Enforced)
1. START WORK: `pmat work start DEPYLER-XXXX` or `pmat work continue DEPYLER-XXXX`
2. BASELINE: `pmat tdg . --format json --with-git-context > baseline.json`
3. QUALITY CHECK: `pmat quality-gate --fail-on-violation`
4. WRITE: Property test FIRST (TDD)
5. IMPLEMENT: <10 complexity (`pmat analyze complexity`)
6. VERIFY: Generated Rust compiles
7. TDG CHECK: `pmat tdg check-quality --min-grade A-`
8. REGRESSION CHECK: `pmat tdg check-regression --baseline baseline.json`
9. COVERAGE: ≥80% (`cargo llvm-cov`)
10. COMMIT: With ticket reference (only if ALL gates pass)
11. COMPLETE: `pmat work complete DEPYLER-XXXX`

## 🔍 Renacer Debugging Integration (Performance Profiling)

**Tool**: [Renacer v0.5.0](https://github.com/paiml/renacer) - Syscall tracer, function profiler & transpiler decision tracer
**Install**: `cargo install renacer`
**Latest Release**: v0.5.0 (2025-11-19) - Transpiler source mapping & decision tracing

### When to Use Renacer

**MANDATORY** for performance-critical work:
- ❌ NEVER optimize without profiling first (no guessing!)
- ✅ ALWAYS profile before claiming "performance improvement"
- ✅ REQUIRED for any commit mentioning "optimization", "faster", or "performance"

### Quick Start Scripts

**Profile Transpiler**:
```bash
# Basic profiling
./scripts/profile_transpiler.sh examples/benchmark.py

# Generate flamegraph
./scripts/profile_transpiler.sh examples/matrix_testing_project/07_algorithms/algorithms.py --flamegraph

# Find I/O bottlenecks only
./scripts/profile_transpiler.sh examples/example_stdlib.py --io-only
```

**Profile Tests**:
```bash
# Find slow tests (>100ms)
./scripts/profile_tests.sh --slow-only

# Profile specific test
./scripts/profile_tests.sh cargo_toml_gen

# Full test suite profiling
./scripts/profile_tests.sh
```

**Profile DEPYLER-0384 (Cargo.toml Generation)**:
```bash
# Comprehensive profiling of automatic dependency tracking
./scripts/profile_cargo_toml_gen.sh
```

### Performance Thresholds (ENFORCED)

**Transpilation Performance**:
- Parse phase: <50ms for typical scripts
- Codegen phase: <100ms for typical scripts
- Total transpilation: <200ms for typical scripts

**I/O Thresholds**:
- File read: <10ms (use buffering if exceeded)
- File write: <5ms (acceptable for Cargo.toml generation)
- Temp file operations: <1ms per operation

**Test Performance**:
- Unit tests: <10ms each
- Property tests: <100ms total (1000 iterations)
- Integration tests: <500ms each (compilation allowed)

### Profiling Workflow (Scientific Method)

**MANDATORY steps for optimization commits**:

1. **BASELINE** - Establish current performance
   ```bash
   renacer --function-time -- cargo run --release -- transpile script.py > baseline.txt
   ```

2. **IDENTIFY** - Find hot functions (>5% total time)
   ```bash
   ./scripts/profile_transpiler.sh script.py --hot-functions
   ```

3. **HYPOTHESIZE** - Form optimization hypothesis
   - Example: "Caching type lookups will reduce parser time by 30%"

4. **OPTIMIZE** - Apply targeted optimization
   - ONLY optimize hot paths identified in step 2
   - NEVER optimize without profiling data

5. **MEASURE** - Verify improvement
   ```bash
   renacer --function-time -- cargo run --release -- transpile script.py > optimized.txt
   diff baseline.txt optimized.txt
   ```

6. **VALIDATE** - Ensure correctness maintained
   ```bash
   cargo test --workspace
   cargo clippy -- -D warnings
   ```

### Commit Message Requirements

For performance-related commits, MUST include:
```
[DEPYLER-XXXX] Optimize parser caching (30% speedup)

Performance Metrics (Renacer v0.5.0):
  Baseline:
    - parse_python: 45.3ms (23%)
    - Total: 196ms

  Optimized:
    - parse_python: 31.7ms (16%) [-30%]
    - Total: 167ms [-15%]

  I/O Bottlenecks:
    - Before: File read 12.3ms
    - After: Buffered read 2.1ms [-83%]

Verification:
  - Tests: 690/690 passing ✅
  - Flamegraph: attached (depyler_optimized.svg)
  - No regressions ✅
```

### Example: DEPYLER-0384 Profiling

```bash
# Profile Cargo.toml generation overhead
./scripts/profile_cargo_toml_gen.sh

# Expected results:
#   - extract_dependencies: <0.2ms
#   - generate_cargo_toml: <0.1ms
#   - Total overhead: <1% of transpilation time ✅
```

### Flamegraph Best Practices

**Prerequisites**:
```bash
git clone https://github.com/brendangregg/FlameGraph
export PATH=$PATH:$PWD/FlameGraph
```

**Generate flamegraph**:
```bash
renacer --function-time -- cargo run --release -- transpile large_script.py | \
    flamegraph.pl > transpile_flame.svg
```

**Attach to commits**: Include flamegraphs in optimization PRs

### Debugging Scenarios

**Slow transpilation**:
```bash
./scripts/profile_transpiler.sh slow_script.py --flamegraph
depyler transpile slow_script.py --trace  # See pipeline phases
```

**Compilation timeout**:
```bash
renacer --function-time -- depyler compile timeout_script.py
```

**Test regression**:
```bash
./scripts/profile_tests.sh failing_test
```

**Full Documentation**: [docs/debugging/renacer-debugging-guide.md](docs/debugging/renacer-debugging-guide.md)

---

## 🔍 Golden Tracing for Debugging (USE NOW)

**Problem**: We repeatedly encounter similar compilation and semantic equivalence failures across different examples. This "spinning of wheels" stems from a lack of unified, deterministic context when comparing the original Python behavior to the transpiled Rust output.

**Solution**: Golden Tracing using **Renacer** provides a single source of truth by capturing and comparing causally-ordered syscall traces, allowing us to definitively validate behavior and prevent recurring bugs. Adopting this methodology is critical to making steady, forward progress.

### When to Use Golden Tracing for Debugging

**Use NOW for failing examples**:
- ✅ Debug type inference issues (understand what types Python actually uses at runtime)
- ✅ Understand stdlib mapping behavior (see which Python stdlib calls map to Rust)
- ✅ Identify missing method calls (compare syscall patterns)
- ✅ Validate constructor patterns (trace object instantiation)
- ✅ Compare execution flow between Python and Rust (even if Rust doesn't compile)

**Key Insight**: You can trace Python execution BEFORE Rust compiles to understand expected behavior.

### Cross-Modal Debugging Workflow

**Step 1: Capture Python baseline** (establishes expected behavior)
```bash
cd /home/noah/src/reprorusted-python-cli/examples/example_subprocess

# Trace Python execution
renacer -T --format json -- python task_runner.py echo "test" --capture > golden_python.json

# Analyze syscalls to understand runtime behavior
grep "execve\|clone\|wait4" golden_python.json | jq '.syscall_name'
# Shows: Python uses these syscalls for subprocess.run()
```

**Step 2: Analyze Rust compilation errors**
```bash
cargo build 2>&1 | tee build_errors.txt

# Extract error patterns
grep "error\[E0277\]" build_errors.txt
# Shows: serde_json::Value doesn't implement AsRef<OsStr>
# Insight: Type inference defaulted to Value instead of Vec<String>
```

**Step 3: Five Whys root cause analysis**
```bash
# Compare Python trace with Rust error:
# - Python trace shows: execve receives char** (string array)
# - Rust error shows: Type system expects AsRef<OsStr>
# - Root cause: Type inference didn't propagate Vec<String> constraint
```

**Step 4: Fix transpiler** (never fix generated code)
```bash
# Update type inference in depyler-core/src/type_system/
# Add failing test first (TDD)
# Re-build transpiler
cargo build --release
```

**Step 5: Re-transpile and capture Rust trace**
```bash
depyler transpile task_runner.py --source-map -o task_runner.rs
cargo build --release

# Trace Rust execution (if it compiles)
renacer -T --format json -- ./target/release/task_runner echo "test" --capture > golden_rust.json

# Compare syscalls
diff <(jq '.syscall_name' golden_python.json) <(jq '.syscall_name' golden_rust.json)
# Expected: Similar syscall patterns (execve, wait4, etc.)
```

### Cross-Modal Context Features

**Transpiler Source Mapping** (Renacer Sprint 24-28):
```bash
# Generate source map during transpilation
depyler transpile example.py --source-map -o example.rs
# Creates: example.rs + example.rs.sourcemap.json

# Use source map during debugging
renacer --transpiler-map example.rs.sourcemap.json -T -- ./target/release/example

# Output shows Python line numbers instead of Rust line numbers:
# [Python:42 process_data()] write(1, "output", 6) = 6
```

**Unified Tracing** (Renacer Sprint 31 - OTLP + Transpiler Decisions):
```bash
# Trace with transpiler decisions
renacer --otlp-endpoint http://localhost:4318 -- ./target/release/example

# View in Jaeger (http://localhost:16686)
# Single trace shows:
# - Python source locations
# - Rust syscalls
# - Transpiler decisions that generated the code
```

### Example: Debugging example_subprocess Type Inference

**Problem**: Rust won't compile - `serde_json::Value: AsRef<OsStr>` not satisfied

**Debug Process**:
```bash
# 1. Capture Python baseline
cd /home/noah/src/reprorusted-python-cli/examples/example_subprocess
renacer -c -- python task_runner.py echo "test" > python_stats.txt

# 2. Analyze Python syscalls
grep "execve" python_stats.txt
# Shows: execve("echo", ["echo", "test"], ...) - string array

# 3. Check Rust error
cargo build 2>&1 | grep -A5 "E0277"
# Shows: cmd: &serde_json::Value used in Command::new(&cmd[0])
# Error: Value doesn't implement AsRef<OsStr>

# 4. Root cause identified
# Type inference: cmd should be Vec<String>, not Value
# Evidence: Python uses string array, Rust expects AsRef<OsStr>

# 5. Fix transpiler type inference (not generated code)
# Create test: test_subprocess_cmd_type_inference()
# Update: type_system to propagate Vec<String> from usage
```

### Systematic Debugging Pattern

For EVERY failing example, follow this pattern:
1. **Capture Python golden trace** - Establish syscall baseline
2. **Analyze compilation errors** - Identify type/trait mismatches
3. **Compare traces with errors** - Find root cause in transpiler logic
4. **Five Whys analysis** - Trace error back to transpiler decision
5. **Fix transpiler** (NEVER fix generated code)
6. **Re-transpile and validate** - Capture Rust trace, compare

**Documentation**: See `single_shot_compilation_failure_analysis.md` for detailed examples.

---

## 🎯 Renacer Golden Trace Validation (GOLDEN-001 Epic)

**MANDATORY**: Use Renacer golden traces to validate semantic equivalence of Python→Rust transpilations AFTER achieving 100% compilation.

### What is a Golden Trace?

A **Golden Trace** is a canonical execution trace that:
1. Captures syscall-level behavior of program execution
2. Provides causal ordering guarantees using Lamport clocks (not wall-clock time)
3. Enables semantic equivalence checking between Python and Rust implementations
4. Serves as regression test baseline in CI/CD pipelines

**Toyota Way Principles**:
- **Jidoka (Autonomation)**: Automatic detection of semantic divergence
- **Andon (Stop the Line)**: Build-time assertions fail CI on performance regression
- **Poka-Yoke (Error-Proofing)**: Lamport clocks eliminate race condition false positives

### Integration Guide

**Configuration**: `/home/noah/src/reprorusted-python-cli/renacer.toml`
**Documentation**: `/home/noah/src/reprorusted-python-cli/docs/integration-report-golden-trace.md`

### Validation Workflow (4-Step Protocol)

**Step 1: Capture Python Baseline**
```bash
# Run original Python CLI
python example_cli.py --arg "test" > python_output.txt

# Trace Python execution (syscall-level)
renacer --format json -- python example_cli.py --arg "test" > golden_python.json
```

**Step 2: Generate Rust Transpilation**
```bash
# Transpile with Depyler
depyler transpile example_cli.py --source-map -o example_cli.rs

# Build Rust binary with debug symbols
rustc -g example_cli.rs -o example_cli
# or
cargo build --release
```

**Step 3: Capture Rust Trace**
```bash
# Run Rust CLI (same inputs as Python)
./example_cli --arg "test" > rust_output.txt

# Trace Rust execution
renacer --format json -- ./example_cli --arg "test" > golden_rust.json
```

**Step 4: Validate Semantic Equivalence**
```bash
# Compare outputs (must be identical)
diff python_output.txt rust_output.txt
# ✅ Expected: No differences

# Validate syscall-level equivalence
renacer-compare golden_python.json golden_rust.json

# Expected output:
# ✅ Semantic Equivalence: PASS
#    - Write patterns: Identical
#    - File operations: Identical
#    - Output correctness: Identical
# ✅ Performance: 8.5× faster (Rust: 10ms, Python: 85ms)
```

### CI/CD Integration

Add to `.github/workflows/ci.yml`:
```yaml
- name: Golden Trace Validation
  run: |
    # Install Renacer
    cargo install renacer

    # For each example
    for example in example_simple example_argparse example_config; do
      cd examples/$example

      # Capture Python trace
      renacer --format json -- python ${example}.py > golden_python.json

      # Build Rust
      cargo build --release

      # Capture Rust trace
      renacer --format json -- ./target/release/${example} > golden_rust.json

      # Validate equivalence
      cargo test --test semantic_equivalence_${example}

      # Check performance budgets
      cargo test --test performance_regression_${example}
    done
```

### Performance Budgets (renacer.toml)

Enforce performance budgets at build time:
```toml
[[assertion]]
name = "cli_startup_time"
type = "critical_path"
max_duration_ms = 15  # CLI must start in <15ms
fail_on_violation = true  # STOP THE LINE on violation

[[assertion]]
name = "max_syscalls"
type = "span_count"
max_spans = 150  # Limit syscall overhead
fail_on_violation = true
```

**Enforcement**: Tests fail if performance budgets violated (Andon principle).

### Semantic Equivalence Testing

Example test suite integration:
```rust
// tests/semantic_equivalence.rs
use renacer::semantic_equivalence::{SemanticValidator, ValidationResult};
use renacer::unified_trace::UnifiedTrace;

#[test]
fn test_example_simple_equivalence() {
    // Load traces
    let python_trace = UnifiedTrace::from_file("golden_python.json").unwrap();
    let rust_trace = UnifiedTrace::from_file("golden_rust.json").unwrap();

    // Validate
    let validator = SemanticValidator::new();
    let result = validator.validate(&python_trace, &rust_trace);

    match result {
        ValidationResult::Pass { performance, .. } => {
            println!("✅ Transpilation validated! Speedup: {}×", performance.speedup);
            assert!(performance.speedup >= 3.0, "Rust must be ≥3× faster");
        }
        ValidationResult::Fail { reason, .. } => {
            panic!("❌ Semantic divergence: {}", reason);
        }
    }
}
```

### When to Use Golden Traces

**MANDATORY**:
- ✅ After fixing any transpiler bug (regression prevention)
- ✅ Before releasing to crates.io (validation gate)
- ✅ When changing core codegen logic (semantic preservation)

**RECOMMENDED**:
- ✅ For all new CLI examples (baseline establishment)
- ✅ When performance regression suspected (quantification)
- ✅ During refactoring (correctness verification)

### Overhead Characteristics

| Metric | Renacer | strace | Improvement |
|--------|---------|--------|-------------|
| Hot path latency | 200ns | 50μs | **250× faster** |
| CPU overhead | <1% | 10-30% | **10-30× lower** |
| Observer effect | Minimal | High | **Production-safe** |

**Conclusion**: Renacer has <1% overhead, making it safe for CI/CD without performance penalty.

### Anti-Pattern Detection

Renacer can detect common issues in transpiled code:
```toml
[[assertion]]
name = "prevent_god_process"
type = "anti_pattern"
pattern = "GodProcess"
threshold = 0.8
fail_on_violation = false  # Warning only
```

**Detects**:
- Single process doing too much work
- Missing parallelization opportunities
- Tight loops (busy-wait instead of async)

### Troubleshooting

**Permission Denied**:
```bash
# Enable ptrace for user
echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
```

**No Source Correlation**:
```bash
# Rebuild with debug symbols
RUSTFLAGS="-C debuginfo=2" cargo build --release
```

**Full Integration Guide**: `/home/noah/src/reprorusted-python-cli/docs/integration-report-golden-trace.md`

---

## PMAT Hooks Management (TICKET-PMAT-5034)

**Automated pre-commit hook setup and management**:

```bash
# Install pre-commit hooks with quality gates
pmat hooks install

# Uninstall hooks
pmat hooks uninstall

# Check hook status
pmat hooks status

# Update hooks to latest version
pmat hooks update
```

**Hook Features**:
- TDG quality checks (min grade A-)
- Complexity enforcement (≤10)
- SATD detection (zero tolerance)
- Mutation testing on changed files
- Hallucination detection before push
- Automatic roadmap/CHANGELOG sync check

## PMAT Agent (Background Quality Monitoring)

**Continuous quality monitoring daemon**:

```bash
# Start background agent
pmat agent start

# Stop agent
pmat agent stop

# Check agent status
pmat agent status

# View agent logs
pmat agent logs
```

**Agent Capabilities**:
- Real-time TDG monitoring
- Automatic regression detection
- Quality alert notifications
- Dashboard integration
- Git hook automation

## PMAT Semantic Search & Code Intelligence (PMAT-SEARCH-011)

**Advanced code search and analysis**:

```bash
# Semantic code search (finds similar code by meaning)
pmat semantic search "error handling patterns"

# Cluster code by semantic similarity
pmat analyze cluster --path crates --min-similarity 0.8

# Extract semantic topics from codebase
pmat analyze topics --path crates --num-topics 10

# Create embeddings for fast search
pmat embed create --path crates
pmat embed update --path crates
```

**Use Cases**:
- Find similar bug patterns across codebase
- Identify duplicate logic (not just text)
- Discover refactoring opportunities
- Code reuse recommendations

## Repository Health Scoring

**Comprehensive project quality metrics**:

```bash
# General repository health score (0-110 scale)
pmat repo-score

# Rust-specific project score (0-106 scale)
pmat rust-project-score

# Detailed breakdown
pmat repo-score --detailed

# Compare with baseline
pmat repo-score --baseline main
```

**Scoring Factors**:
- TDG grades (40%)
- Test coverage (20%)
- Documentation quality (15%)
- Complexity metrics (15%)
- CI/CD health (10%)

## Hallucination Detection & Red Team Mode

**Automated validation of documentation and commits**:

```bash
# Validate README for hallucinations
pmat validate-readme README.md

# Check commit messages for false claims
pmat red-team commit HEAD

# Validate documentation links
pmat validate-docs docs/
```

**Benefits**:
- Prevents false performance claims
- Validates feature documentation
- Checks broken links
- Verifies API examples

## Corpus Convergence Protocol

**CRITICAL**: Use the convergence protocol to achieve 100% compilation rate on real-world corpora.

**Prompt Location**: `docs/prompts/converge_reprorusted_100.md`

**Quick Start**:
```bash
# Build release binary
cargo build --release --bin depyler

# Warm cache for O(1) subsequent lookups
depyler cache warm --input-dir /path/to/corpus

# Run UTOL automated convergence (Toyota Way)
depyler utol --corpus /path/to/corpus --target-rate 0.80

# Or manual convergence with explain + oracle
depyler explain out.rs --trace trace.json --verbose
depyler oracle improve --corpus /path/to/corpus --target-rate 1.0
```

**Key Commands**:
| Task | Command |
|------|---------|
| Cache Warm | `depyler cache warm --input-dir $CORPUS` |
| Cache Stats | `depyler cache stats` |
| Explain Errors | `depyler explain <file.rs> --trace <trace.json>` |
| Oracle Train | `depyler oracle train --corpus $CORPUS` |
| Oracle Improve | `depyler oracle improve --corpus $CORPUS` |
| UTOL Loop | `depyler utol --corpus $CORPUS --target-rate 0.80` |

**Toyota Way Principles**: Jidoka (stop on defect), Kaizen (continuous improvement), Andon (visual feedback).

## Release Checklist
- [ ] All examples transpile and run
- [ ] Property tests 100% coverage on supported features
- [ ] Generated code passes clippy -D warnings
- [ ] No performance regression
- [ ] Documentation examples tested
- [ ] CHANGELOG updated
- [ ] Version bump (semver)

## 📅 Release Cadence (MANDATORY)

**CRITICAL RULE**: NEVER release to crates.io until FRIDAY. We ONLY release once per week.

**Release Schedule**:
- **Release Day**: Friday only
- **Frequency**: Once per week maximum
- **No Exceptions**: Emergency fixes wait until Friday

**Rationale**:
- Allows full week for testing and validation
- Prevents rushed releases
- Ensures quality over speed
- Gives users predictable update schedule

**Process**:
1. **Monday-Thursday**: Development, testing, bug fixes
2. **Thursday EOD**: Freeze code, final validation
3. **Friday**: Release to crates.io if all gates pass
4. **Weekend**: Monitor for issues

**Violations**:
- ❌ NO mid-week releases
- ❌ NO "hotfix" releases (wait for Friday)
- ❌ NO multiple releases per week

---

**Remember**: Perfect transpilation > feature-complete transpilation. Every line of generated Rust must be idiomatic. Ship nothing that doesn't meet these standards.


## Stack Documentation Search

**IMPORTANT: Proactively use the batuta RAG oracle when:**
- Looking up Python→Rust transpilation patterns
- Finding HuggingFace/JAX/vLLM Python idioms to transpile
- Understanding Rust equivalents for Python stdlib
- Researching type inference strategies from ground truth corpora

```bash
# Search across the entire Sovereign AI Stack
batuta oracle --rag "your question here"

# Examples for depyler development
batuta oracle --rag "Python subprocess to Rust Command"
batuta oracle --rag "type inference for generic collections"
batuta oracle --rag "HuggingFace tokenizer implementation"
batuta oracle --rag "async Python to Rust tokio"
batuta oracle --rag "error handling Python exceptions to Result"

# Reindex if needed (persists to ~/.cache/batuta/rag/)
batuta oracle --rag-index
```

The RAG index includes 335 documents across:
- All Sovereign AI Stack repos (trueno, aprender, realizar, etc.)
- Python ground truth corpora (HuggingFace, JAX, vLLM patterns)
- Rust ground truth corpora (TGI inference, MLOps patterns)

Index auto-updates via post-commit hooks and `ora-fresh` on shell login.
To manually check freshness: `ora-fresh`
To force full reindex: `batuta oracle --rag-index --force`

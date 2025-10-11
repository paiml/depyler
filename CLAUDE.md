# CLAUDE.md - Depyler Compiler Implementation Protocol

## IMPORTANT: Auto-Generated Files
**NEVER EDIT `deep_context.md`** - This file is auto-generated and will be overwritten. Any changes should be made to the source files instead.

## Prime Directive

**Generate correct Rust code that compiles on first attempt. Quality is built-in, not bolted-on.**

## Project Context

Depyler is a Python-to-Rust transpiler focusing on energy-efficient, safe code
generation with progressive verification. The system must produce idiomatic Rust
code with formal correctness guarantees for a practical Python subset.

## Python Packaging Protocol

**MANDATORY: Use `uv` for ALL Python operations**

- **Package Management**: Use `uv` instead of pip, pip3, or virtualenv
- **Running Tests**: `uv run pytest` instead of `python -m pytest`
- **Installing Packages**: `uv add <package>` or `uv pip install <package>`
- **Creating Environments**: `uv venv` (automatic with `uv run`)
- **Running Scripts**: `uv run <script.py>`

**Examples**:
```bash
# ❌ BAD: Using pip or python directly
pip install pytest
python -m pytest tests/
python3 script.py

# ✅ GOOD: Using uv
uv add pytest
uv run pytest tests/
uv run script.py
```

**Why uv?**
- 10-100x faster than pip
- Better dependency resolution
- Automatic virtual environment management
- Consistent, reproducible builds
- No manual venv activation needed

## 🚨 CRITICAL: A+ Code Standard (From paiml-mcp-agent-toolkit)

**ABSOLUTE REQUIREMENT**: All NEW code MUST achieve A+ quality standards:
- **Maximum Cyclomatic Complexity**: ≤10 (not 20, not 15, TEN!)
- **Maximum Cognitive Complexity**: ≤10 (simple, readable, maintainable)
- **Function Size**: ≤30 lines (if longer, decompose it)
- **Single Responsibility**: Each function does ONE thing well
- **Zero SATD**: No TODO, FIXME, HACK, or "temporary" solutions
- **TDD Mandatory**: Write test FIRST, then implementation
- **Test Coverage**: 80% minimum (enforced via cargo-llvm-cov)

**Enforcement Example**:
```rust
// ❌ BAD: Complexity 15+
fn process_ast(items: Vec<AstNode>) -> Result<HirNode> {
    let mut results = Vec::new();
    for item in items {
        if item.valid {
            if item.node_type == NodeType::Expr {
                // ... 20 more lines of nested logic
            }
        }
    }
    // ... more complexity
}

// ✅ GOOD: Complexity ≤10
fn process_ast(items: Vec<AstNode>) -> Result<HirNode> {
    items.into_iter()
        .filter(|item| item.valid)
        .map(process_single_node)
        .collect()
}

fn process_single_node(item: AstNode) -> Result<HirNode> {
    match item.node_type {
        NodeType::Expr => process_expr(item),
        NodeType::Stmt => process_stmt(item),
    }
}
```

## EXTREME TDD Protocol (CRITICAL RESPONSE TO TRANSPILER FAILURES)

**ANY TRANSPILER OR CODE GENERATION BUG REQUIRES IMMEDIATE EXTREME TDD RESPONSE:**

### Critical Bug Response (MANDATORY):
1. **HALT ALL OTHER WORK**: Stop everything when transpiler/codegen bugs found
2. **EXTREME TEST COVERAGE**: Create comprehensive test suites immediately:
   - Unit tests for every transpilation rule
   - Integration tests for complete programs
   - Property tests with random inputs (10,000+ iterations)
   - Fuzz tests for edge cases
   - Doctests in every public function
   - `cargo run --example` MUST pass 100%
3. **REGRESSION PREVENTION**: Add failing test BEFORE fixing bug
4. **COMPREHENSIVE VALIDATION**: Test all language features after any fix

### Test Coverage Requirements (MANDATORY):
- **Transpiler Tests**: Every Python AST → Rust HIR mapping
- **Codegen Tests**: Every HIR → Rust code generation pattern
- **Integration Tests**: Full transpile → compile → execute pipeline
- **Property Tests**: Automated generation of valid/invalid programs (80% target)
- **Fuzz Tests**: Random input stress testing (AFL, cargo-fuzz)
- **Examples Tests**: All examples/ must transpile and compile

## Scientific Method Protocol

**WE DON'T GUESS, WE PROVE VIA QUANTITATIVE METHODS AND TESTING.**

### Evidence-Based Development Rules:
1. **NO ASSUMPTIONS**: Every claim must be backed by concrete evidence
2. **MEASURE EVERYTHING**: Use tests, benchmarks, and metrics to validate behavior
3. **REPRODUCE ISSUES**: Create minimal test cases that demonstrate problems
4. **QUANTIFY IMPROVEMENTS**: Before/after metrics prove effectiveness
5. **DOCUMENT EVIDENCE**: All findings must be recorded with reproducible steps

### Investigation Protocol:
1. **Hypothesis**: State what you believe is happening
2. **Test**: Create specific tests that prove/disprove the hypothesis
3. **Measure**: Collect concrete data (test results, timings, coverage)
4. **Analyze**: Draw conclusions only from the evidence
5. **Document**: Record findings and next steps

## QDD (Quality-Driven Development) Protocol

**QUALITY IS THE DRIVER, NOT AN AFTERTHOUGHT - BASED ON PMAT BOOK CH14**

### QDD Core Principles:
1. **Quality Metrics First**: Define quality metrics BEFORE writing code
2. **Continuous Monitoring**: Real-time quality tracking during development
3. **Automated Enforcement**: Quality gates that cannot be bypassed
4. **Data-Driven Decisions**: Let metrics guide development priorities
5. **Preventive Maintenance**: Fix quality issues before they become technical debt

### QDD Implementation with PMAT:
```bash
# BEFORE starting any task - establish quality baseline
pmat tdg . --min-grade A- --format=json > quality_baseline.json
pmat analyze complexity --format=csv > complexity_baseline.csv

# DURING development - continuous quality monitoring
pmat tdg dashboard --port 8080 --update-interval 5 &  # Real-time monitoring
watch -n 5 'pmat quality-gate --quiet || echo "QUALITY DEGRADATION DETECTED"'

# AFTER each function/module - verify quality maintained
pmat tdg <file> --compare-baseline quality_baseline.json
pmat analyze complexity <file> --max-cyclomatic 10 --max-cognitive 10

# BEFORE commit - comprehensive quality validation
pmat tdg . --min-grade A- --fail-on-violation
pmat quality-gate --fail-on-violation --format=detailed
```

## Development Principles

### 自働化 (Jidoka) - Build Quality In

- **Never ship incomplete transpilation**: All HIR transformations must include
  complete error handling paths
- **Verification-first development**: Every new AST-to-Rust mapping requires
  corresponding property verification
- **Example**: When implementing control flow transpilation:
  ```rust
  // CORRECT: Complete error handling
  match stmt {
      While { test, body, .. } => {
          verify_termination_bounds(&test)?;
          emit_rust_while(test, body)
      }
      _ => Err(TranspileError::UnsupportedStatement(stmt.clone()))
  }
  // NEVER: Partial implementations with TODO
  ```

### 現地現物 (Genchi Genbutsu) - Direct Observation

- **Test against real Rust**: Don't rely on syn parsing alone; test generated
  code with `cargo check`
- **Profile actual compilation**: Measure transpilation time/memory on realistic
  Python codebases
- **Debug at the Rust level**: When transpilation fails, examine the actual
  generated Rust code, not just the HIR

### 反省 (Hansei) - Fix Before Adding

- **Current broken functionality to prioritize**:
  1. Type inference generates incorrect ownership patterns
  2. String handling creates unnecessary allocations
  3. Property verification doesn't catch all lifetime violations
- **Do not add**: Advanced async support, class inheritance, or SMT verification
  until core function transpilation is bulletproof

### 改善 (Kaizen) - Continuous Improvement

- **Incremental verification**: Start with `--verify basic`, achieve 100%
  coverage on V1 subset, then advance to `strict`
- **Performance baselines**: Generated Rust must compile in <500ms for typical
  functions
- **Code quality targets**: Output should pass `clippy::pedantic` without
  warnings

## Critical Invariants

1. **Type safety**: Every generated Rust program must pass `cargo check` without
   errors
2. **Determinism**: Same Python input must produce identical Rust output across
   runs
3. **Memory safety**: No generated code can cause undefined behavior or memory
   leaks

## Build Commands

```bash
# Run full test suite with property verification
cargo test --workspace

# Transpile with verification
cargo run -- transpile examples/showcase/binary_search.py --verify

# Run benchmarks
cargo bench

# Check generated code quality
cargo clippy --workspace -- -D warnings
```

## Python→Rust Transpilation Workflow

**CRITICAL**: ALL .rs example files are generated by depyler transpilation from .py source files.

### Standard Transpilation Command

```bash
# Basic transpilation (output defaults to <input>.rs)
depyler transpile <INPUT.py>

# With explicit output
depyler transpile <INPUT.py> --output <OUTPUT.rs>

# With verification and test generation
depyler transpile <INPUT.py> --verify --gen-tests
```

### Example Workflow

```bash
# Step 1: Transpile Python to Rust
depyler transpile examples/showcase/binary_search.py

# Step 2: Verify it compiles
cargo check --all-targets

# Step 3: Validate quality gates
make validate-example FILE=examples/showcase/binary_search.rs
```

### Required Header in All .rs Examples

**MANDATORY**: Every transpiled .rs file MUST include this header comment:

```rust
// Generated by: depyler transpile examples/showcase/binary_search.py
// Source: examples/showcase/binary_search.py
// Command: depyler transpile examples/showcase/binary_search.py
```

This ensures:
1. Traceability: Know which Python file generated the Rust code
2. Reproducibility: Exact command to regenerate the file
3. Verification: Can verify transpilation output matches expected result

### Transpilation Output

Depyler provides comprehensive feedback:
- **Type Inference Hints**: Suggests types for variables
- **Performance Warnings**: Identifies optimization opportunities
- **Profiling Report**: Estimates instruction count, hot paths
- **Performance Predictions**: Estimates Rust speedup (typically 1.3x-1.6x vs Python)
- **Throughput Metrics**: Parse time, KB/s processed

### Verification Workflow

```bash
# Verify transpilation is deterministic
depyler transpile examples/showcase/binary_search.py --output /tmp/test.rs
diff /tmp/test.rs examples/showcase/binary_search.rs
# Should output: Files are identical ✅

# Validate all 66 examples
make validate-examples
```

## 🛑 Stop the Line: Validation-Driven Transpiler Development

**CRITICAL PHILOSOPHY**: We WANT to find problems in generated code. Each issue improves the transpiler for ALL future code.

### The Jidoka Principle (自働化) for Transpilers

Inspired by Toyota's "stop the line" manufacturing principle: **Never pass defects downstream**.

```
Python Input → Transpile → Validate → 🛑 STOP if issues → Fix Transpiler → Continue
                              ↓
                         Issue Found?
                              ↓
                    🛑 STOP THE LINE
                              ↓
                    Create Ticket (DEPYLER-XXXX)
                              ↓
                    Fix TRANSPILER (not output)
                              ↓
                    Re-transpile ALL examples
                              ↓
                    Verify Fix
                              ↓
                    ✅ Resume Development
```

### Validation Methodology

**Goal A**: Prove the transpiler works (correctness, types, ownership)
**Goal B**: Find edge cases and feed them back to improve transpiler quality

#### What We Validate

1. **Functional Correctness** ✅
   - Generated code compiles
   - Types are correct
   - Ownership/borrowing is safe
   - Logic matches Python source

2. **Code Quality** ⚠️ (This is where we find issues!)
   - Idiomatic Rust patterns
   - Zero clippy warnings (with `-D warnings`)
   - No unused imports
   - No unnecessary complexity

3. **Production Readiness**
   - Passes all quality gates
   - Would be accepted in code review
   - Maintainable by humans

#### When We Find Issues

**IMMEDIATE RESPONSE** (Don't continue until fixed):

```bash
# 1. STOP - Don't continue transpiling more examples
🛑 VALIDATION PAUSED

# 2. DOCUMENT - Capture the issue
#    - What: Specific code pattern that's wrong
#    - Why: Root cause in transpiler
#    - Impact: How many files affected

# 3. TICKET - Create roadmap entry
#    Format: DEPYLER-XXXX: Fix [specific transpiler issue]
#    Priority: P0 (blocks production readiness)
#    Type: Transpiler Bug (Upstream)

# 4. ANALYZE - Root cause analysis
#    - Which transpiler module?
#    - Code generation or AST translation?
#    - Template issue or logic bug?

# 5. FIX TRANSPILER - Not the output!
#    - Contribute fix upstream (if external project)
#    - Or fix in crates/depyler-core/
#    - Add test case for the edge case

# 6. RE-TRANSPILE - Regenerate ALL affected examples
depyler transpile examples/showcase/*.py

# 7. VERIFY - Confirm fix works
cargo clippy --all-targets -- -D warnings  # Must pass
rustc --crate-type lib examples/**/*.rs    # Zero warnings

# 8. RESUME - Continue validation
✅ Issue fixed, transpiler improved, continue!
```

#### Example: DEPYLER-0095 (Real Issue Found)

**Discovery** (2025-10-07):
```rust
// Transpiler generated (WRONG):
let mut _cse_temp_0 = (n == 0);  // Unnecessary parens
while(0 <= right) {              // Unnecessary parens

// Should generate (IDIOMATIC):
let mut _cse_temp_0 = n == 0;
while 0 <= right {
```

**Response**:
- 🛑 **STOPPED** validation immediately
- 📋 **CREATED** DEPYLER-0095: Fix Code Generation Quality Issues
- 🔍 **ANALYZED** Root cause: `rust_gen.rs` adds defensive parentheses
- 📝 **DOCUMENTED** 16 warnings in 3/4 showcase examples
- ⏸️  **PAUSED** further work until transpiler fixed
- 🎯 **GOAL** Fix transpiler → Re-transpile → Verify → Resume

**Key Insight**: Don't waste time fixing generated code manually - fix the generator!

### Validation Commands (Correct Method)

**WRONG** (What we initially did):
```bash
# This SKIPS examples/ directory!
cargo clippy --all-targets --all-features -- -D warnings
```

**RIGHT** (What we should do):
```bash
# Method 1: Check each example directly
for file in examples/**/*.rs; do
    rustc --crate-type lib "$file" --deny warnings
done

# Method 2: Add examples/ to workspace (TODO)
# Then cargo clippy will check them

# Method 3: Use validation script
make validate-examples  # Checks each file individually
```

### Upstream Feedback Loop

When we find transpiler issues:

1. **Document Issue**
   - Minimal reproducible example
   - Expected vs actual output
   - Suggested fix (if known)

2. **Create GitHub Issue** (Upstream Project)
   - Title: "Generated code has unnecessary parentheses"
   - Labels: `codegen`, `quality`
   - Attach: showcase examples demonstrating issue

3. **Contribute Fix** (Optional but encouraged)
   - Fork repo
   - Write failing test
   - Implement fix
   - Submit PR with test case

4. **Track in Roadmap**
   - DEPYLER-XXXX: [Upstream] Fix [issue]
   - Link to GitHub issue
   - Update when merged

### Success Metrics

**Quality Gate**: Generated code must pass:
```bash
rustc --crate-type lib <file.rs> --deny warnings  # ✅ Zero warnings
cargo clippy -- -D warnings                        # ✅ Zero warnings
cargo test                                          # ✅ All pass
cargo llvm-cov --fail-under-lines 80               # ✅ ≥80%
pmat analyze complexity --max-cyclomatic 10        # ✅ All ≤10
pmat analyze satd                                  # ✅ Zero SATD
```

**Transpiler Quality**: Measured by:
- Percentage of examples passing all gates (Target: 100%)
- Number of issues found per 100 examples (Lower is better)
- Time to fix issues (Upstream contribution velocity)

### Documentation Requirement

**EVERY issue found must be documented**:
- Ticket in roadmap (DEPYLER-XXXX)
- Analysis report (what, why, impact)
- Upstream issue (GitHub)
- Fix verification (before/after)

**Files**:
- `docs/execution/roadmap.yaml` - Ticket tracking (PMAT YAML format)
- `docs/issues/DEPYLER-XXXX.md` - Detailed analysis
- GitHub Issues - Upstream feedback

### The Mindset Shift

❌ **OLD**: "The transpiler is perfect, just validate output"
✅ **NEW**: "The transpiler is improving, find and fix issues"

❌ **OLD**: "16 warnings? Let's fix the generated files"
✅ **NEW**: "16 warnings? Stop! Fix the transpiler!"

❌ **OLD**: "Validation passed = we're done"
✅ **NEW**: "Validation passed = try harder to break it"

### Continuous Improvement Cycle

```
Week 1: Find 16 warnings → Fix transpiler → Re-transpile → Verify
Week 2: Find 8 warnings → Fix transpiler → Re-transpile → Verify
Week 3: Find 2 warnings → Fix transpiler → Re-transpile → Verify
Week 4: Zero warnings → Transpiler generates perfect code! 🎉
```

**Result**: Every issue makes the transpiler better for EVERYONE.

---

## Testing Strategy

```bash
# Verify core transpilation pipeline
cargo test -p depyler-core

# Run property-based tests on generated code
cargo test -p depyler-verify

# Test MCP fallback integration
cargo test -p depyler-mcp
```

## PMAT TDG Quality Enforcement (MANDATORY - BLOCKING)

**CRITICAL**: PMAT TDG (Technical Debt Grading) quality gates are MANDATORY and BLOCKING. NO EXCEPTIONS.

### TDG Quality Standards (Zero Tolerance):
- **Overall Grade**: Must maintain A- or higher (≥85 points) - HARD LIMIT
- **Structural Complexity**: ≤10 per function (enforced via TDG)
- **Semantic Complexity**: Cognitive complexity ≤10 (enforced via TDG)
- **Code Duplication**: <10% code duplication (measured via TDG)
- **Documentation Coverage**: >70% for public APIs (tracked via TDG)
- **Technical Debt**: Zero SATD comments (zero-tolerance via TDG)
- **Test Coverage**: ≥80% via cargo-llvm-cov (not tarpaulin)

### MANDATORY TDG Commands (All Development):

#### Before ANY Code Changes:
```bash
# MANDATORY: TDG baseline check with comprehensive analysis
pmat tdg . --min-grade A- --fail-on-violation
pmat quality-gate --fail-on-violation --format=summary
```

#### During Development (After Each Function/Module):
```bash
# MANDATORY: File-level TDG analysis
pmat tdg <file.rs> --include-components --min-grade B+

# MANDATORY: Traditional complexity verification (backup)
pmat analyze complexity --max-cyclomatic 10 --max-cognitive 10 --fail-on-violation

# MANDATORY: SATD detection (zero tolerance)
pmat analyze satd --format=summary --fail-on-violation
```

#### Before Commit (MANDATORY - BLOCKS COMMITS):
```bash
# MANDATORY: Comprehensive TDG quality gate
pmat tdg . --min-grade A- --fail-on-violation
pmat quality-gate --fail-on-violation --format=detailed

# MANDATORY: Coverage enforcement (cargo-llvm-cov)
cargo llvm-cov --all-features --workspace --summary-only | grep -q "80" || {
    echo "❌ BLOCKED: Coverage below 80%"
    exit 1
}
```

## MANDATORY: Roadmap and Ticket Tracking

**CRITICAL**: ALL development work MUST follow roadmap-driven development:

1. **ALWAYS Use Ticket Numbers**: Every commit, PR, and task MUST reference a ticket ID from docs/execution/roadmap.yaml
2. **Roadmap-First Development**: No work begins without a corresponding roadmap entry
3. **Ticket Format**: Use format "DEPYLER-XXX" per roadmap
4. **Traceability**: Every change must be traceable back to requirements via ticket system
5. **Sprint Planning**: Work is organized by sprint with clear task dependencies and priorities

### Commit Message Format (MANDATORY with TDG Tracking)
```
[TICKET-ID] Brief description

Detailed explanation of changes
- Specific improvements made
- Test coverage added
- Performance impact
- Breaking changes (if any)

TDG Score Changes (MANDATORY):
- src/file1.rs: 85.3→87.1 (B+→A-) [+1.8 improvement]
- src/file2.rs: 72.5→72.5 (B-→B-) [stable]

PMAT Verification:
- Complexity: All functions ≤10
- SATD: 0 violations maintained
- Coverage: 80.5% → 82.1% (+1.6%)

Closes: TICKET-ID
```

## MANDATORY Quality Gates (BLOCKING - Not Advisory)

**CRITICAL**: Quality gates are now BLOCKING and ENFORCED. No commit shall pass without meeting all gates.

### SACRED RULE: NEVER BYPASS QUALITY GATES

**ABSOLUTELY FORBIDDEN**:
- `git commit --no-verify` - NEVER use this - NO EXCEPTIONS EVER
- Skipping tests "temporarily" - NO exceptions
- Ignoring failing quality checks - Must fix EVERY defect
- Dismissing warnings as "unrelated" - All defects matter

**Toyota Way Principle**: Stop the line for ANY defect. No defect is too small. No shortcut is acceptable.

### Pre-commit Hooks (MANDATORY)
```bash
#!/bin/bash
# scripts/pre-commit - BLOCKS commits that violate quality
set -e

echo "🔍 Depyler Quality Gates - Checking documentation synchronization..."

# Documentation files that MUST be updated with code changes
REQUIRED_DOCS=(
    "docs/execution/roadmap.md"
    "CHANGELOG.md"
)

# Check if any Rust/source files are being committed
if git diff --cached --name-only | grep -qE '\.(rs)$'; then
    echo "📝 Source changes detected - verifying documentation updates..."

    # Ensure at least one documentation file is updated
    DOC_UPDATED=false
    for doc in "${REQUIRED_DOCS[@]}"; do
        if git diff --cached --name-only | grep -q "$doc"; then
            DOC_UPDATED=true
            break
        fi
    done

    if [ "$DOC_UPDATED" = false ]; then
        echo "❌ ERROR: Code changes require documentation updates!"
        echo "📋 Must update at least one of:"
        for doc in "${REQUIRED_DOCS[@]}"; do
            echo "   - $doc"
        done
        echo ""
        echo "💡 Quick fix:"
        echo "   1. Update docs/execution/roadmap.md with task status"
        echo "   2. Update CHANGELOG.md with feature/fix"
        exit 1
    fi
fi

# Verify roadmap.md structure
if git diff --cached --name-only | grep -q "docs/execution/roadmap.md"; then
    ROADMAP=$(git show :docs/execution/roadmap.md 2>/dev/null || cat docs/execution/roadmap.md)

    # Ensure task ID format (DEPYLER-XXXX)
    if ! echo "$ROADMAP" | grep -qE 'DEPYLER-[0-9]{4}'; then
        echo "⚠️  Warning: roadmap.md should use DEPYLER-XXXX task ID format"
    fi

    # Check for status markers
    if ! echo "$ROADMAP" | grep -qE '\[[ x]\]'; then
        echo "⚠️  Warning: roadmap.md should include [x] completion markers"
    fi
fi

echo "🔧 Running PMAT quality analysis..."

for file in $(git diff --cached --name-only --diff-filter=ACM | grep -E '\.rs$'); do
    echo "  Checking $file..."

    # Skip target directory
    if [[ "$file" == target/* ]]; then
        continue
    fi

    # Complexity check
    if command -v pmat &> /dev/null; then
        pmat analyze complexity "$file" \
            --max-cyclomatic 10 \
            --max-cognitive 10 \
            --fail-on-violation || {
            echo "❌ Complexity violation in $file"
            echo "Run: pmat refactor auto --file $file"
            exit 1
        }

        # SATD check (zero tolerance)
        pmat analyze satd "$file" --fail-on-violation || {
            echo "❌ SATD violation in $file"
            echo "Remove all TODO/FIXME/HACK comments"
            exit 1
        }
    fi
done

# TDG Grade Check
if command -v pmat &> /dev/null; then
    echo "📊 Running TDG grade check..."
    pmat tdg . --min-grade A- --fail-on-violation || {
        echo "❌ BLOCKED: TDG grade below A- threshold"
        echo "Run: pmat tdg . --include-components --top-files 5"
        exit 1
    }
fi

# Coverage check (cargo-llvm-cov)
if command -v cargo-llvm-cov &> /dev/null; then
    echo "📊 Running coverage check..."
    COVERAGE=$(cargo llvm-cov --all-features --workspace --summary-only 2>/dev/null | grep -oP '\d+\.\d+%' | head -1 | sed 's/%//')
    if (( $(echo "$COVERAGE < 80" | bc -l) )); then
        echo "❌ BLOCKED: Coverage $COVERAGE% below 80% threshold"
        exit 1
    fi
fi

# Clippy check
echo "🔧 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "❌ BLOCKED: Clippy warnings found"
    exit 1
}

echo "✅ All quality gates passed!"
```

### CI/CD Pipeline Enforcement
```yaml
# .github/workflows/quality-gates.yml
name: MANDATORY Quality Gates
on: [push, pull_request]

jobs:
  quality-gates:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Quality Tools
        run: |
          cargo install pmat --locked
          cargo install cargo-llvm-cov --locked

      - name: GATE 1 - TDG Grade Check
        run: |
          pmat tdg . --min-grade A- --fail-on-violation

      - name: GATE 2 - Complexity Enforcement
        run: |
          pmat analyze complexity --max-cyclomatic 10 --max-cognitive 10 --fail-on-violation

      - name: GATE 3 - Zero SATD
        run: |
          pmat analyze satd --fail-on-violation

      - name: GATE 4 - Lint Zero Tolerance
        run: |
          cargo clippy --all-targets --all-features -- -D warnings

      - name: GATE 5 - Coverage Gate (80% minimum)
        run: |
          cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
          cargo llvm-cov report --fail-under-lines 80

      - name: GATE 6 - All Tests Pass
        run: |
          cargo test --all-features --workspace
```

## The Make Lint Contract (Zero Warnings Allowed)
```bash
# make lint command from Makefile:
cargo clippy --all-targets --all-features -- -D warnings
```

**Critical**: The `-D warnings` flag treats EVERY clippy warning as a hard error. This ensures zero technical debt accumulation.

### What This Means for Your Code

```rust
// Standard clippy: These would be warnings
x.to_string();           // Warning: could use .into()
&vec![1, 2, 3];         // Warning: could use slice
if x == true { }        // Warning: could omit == true

// With make lint: These FAIL the build
x.to_string();          // ERROR - build fails
&vec![1, 2, 3];        // ERROR - build fails  
if x == true { }       // ERROR - build fails
```

### Surviving -D warnings

```rust
// Write defensive code from the start:
x.into();               // Prefer into() over to_string()
&[1, 2, 3];            // Use slice literals
if x { }               // Omit redundant comparisons

// For unavoidable warnings, be explicit:
#[allow(clippy::specific_lint)]  // Document why
fn special_case() { }
```

## Performance Invariants

### Transpilation Throughput

```rust
#[bench]
fn bench_transpile_throughput(b: &mut Bencher) {
    let input = include_str!("../corpus/large.py"); // 10K LOC Python
    b.iter(|| {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(input)
    });
    
    // Invariant: >10MB/s transpilation
    assert!(b.bytes_per_second() > 10_000_000);
}
```

### Memory Safety Verification

```rust
#[bench]
fn bench_verification_latency(b: &mut Bencher) {
    let hir = test_hir_module();
    b.iter(|| {
        let verifier = PropertyVerifier::new();
        verifier.verify(&hir)
    });
    
    // Invariant: <50ms for typical module
    assert!(b.ns_per_iter() < 50_000_000);
}
```

## Architectural Patterns

### HIR Pattern - Immutable Transformations

```rust
impl HirModule {
    fn transform<F>(&self, f: F) -> Self 
    where 
        F: Fn(&HirExpr) -> HirExpr
    {
        // Immutable transformation preserving structure
        self.map_expressions(f)
    }
}
```

### Ownership Inference - Conservative Defaults

```rust
impl OwnershipInferencer {
    fn infer_ownership(&mut self, expr: &HirExpr) -> Ownership {
        match expr {
            HirExpr::Variable(name) => {
                // Conservative: default to borrowed
                self.env.lookup(name)
                    .unwrap_or(Ownership::Borrowed)
            }
            HirExpr::FunctionCall(func, args) => {
                // Move semantics for non-Copy types
                if self.is_move_required(func) {
                    Ownership::Owned
                } else {
                    Ownership::Borrowed
                }
            }
            _ => Ownership::Borrowed
        }
    }
}
```

## Error Diagnostics Quality

### Elm-Level Error Messages

```rust
impl ErrorReporter {
    fn render(&self, error: &TranspileError) -> String {
        let mut output = String::new();
        
        // Source context with highlighting
        writeln!(output, "{}", self.source_snippet(error.span));
        writeln!(output, "{}", "^".repeat(error.span.len()).red());
        
        // Primary message
        writeln!(output, "\n{}: {}", "Error".red().bold(), error.message);
        
        // Type mismatch details
        if let Some(expected) = &error.expected_type {
            writeln!(output, "  {} {}", "Expected:".yellow(), expected);
            writeln!(output, "  {} {}", "Found:".yellow(), error.found_type);
        }
        
        // Actionable suggestion
        if let Some(suggestion) = &error.suggestion {
            writeln!(output, "\n{}: {}", "Hint".green(), suggestion);
            writeln!(output, "{}", self.render_suggestion_diff(suggestion));
        }
        
        output
    }
}
```

## Sprint Hygiene Protocol

### Pre-Sprint Cleanup (MANDATORY)
```bash
# Remove all debug artifacts before starting sprint
rm -f test_* debug_* 
find . -type f -executable -not -path "./target/*" -not -path "./.git/*" -delete

# Verify no large files
find . -type f -size +10M -not -path "./target/*" -not -path "./.git/*"

# Clean build artifacts
cargo clean
```

### Post-Sprint Checklist
```bash
# 1. Remove debug artifacts
rm -f test_* debug_* *.o *.a

# 2. Run all quality gates
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace

# 3. Verify no cruft
git status --ignored

# 4. Push with clean history
git push origin main
```

## The Development Flow (PMAT-Enforced)

### MANDATORY: PMAT Quality at Every Step
```
1. BASELINE CHECK: Run `pmat quality-gate --fail-on-violation`
2. LOCATE task in docs/execution/roadmap.yaml with ticket number
3. VERIFY dependencies complete via roadmap
4. WRITE property test FIRST (TDD mandatory)
5. IMPLEMENT with <10 complexity (verified by `pmat analyze complexity`)
6. VERIFY generated Rust compiles and runs correctly
7. VALIDATE: Run `pmat quality-gate` before ANY commit
8. COVERAGE: Ensure 80%+ via `cargo llvm-cov`
9. COMMIT with ticket reference (only if ALL gates pass)
```

### MANDATORY TDD Protocol:
```bash
# STEP 1: Pre-development baseline
pmat tdg . --min-grade A- --format=table
pmat quality-gate --fail-on-violation --format=summary

# STEP 2: Write failing test FIRST
cargo test <new_test_name> -- --nocapture  # Should fail

# STEP 3: Implement minimal code to pass
# ... write code ...

# STEP 4: Verify test passes
cargo test <new_test_name> -- --nocapture  # Should pass

# STEP 5: File-level verification
pmat tdg <modified-file.rs> --include-components --min-grade B+
pmat analyze complexity <modified-file.rs> --max-cyclomatic 10

# STEP 6: Coverage verification
cargo llvm-cov --html --open  # Verify new code is covered

# STEP 7: Pre-commit validation (MANDATORY - BLOCKS COMMITS)
pmat tdg . --min-grade A- --fail-on-violation
pmat quality-gate --fail-on-violation --format=detailed
cargo llvm-cov report --fail-under-lines 80
```

## Architecture Patterns

### HIR Builder Pattern

```rust
impl HirBuilder {
    fn build_function(&mut self, func: &ast::FunctionDef) -> Result<HirFunction> {
        // Type annotations are mandatory for transpilation
        let params = func.args.args.iter()
            .map(|arg| self.build_param(arg))
            .collect::<Result<Vec<_>>>()?;
            
        let ret_type = func.returns
            .as_ref()
            .map(|ann| self.resolve_type(ann))
            .unwrap_or(Ok(Type::Unit))?;
            
        let body = self.build_block(&func.body)?;
        
        Ok(HirFunction {
            name: func.name.clone(),
            params,
            ret_type,
            body,
            properties: self.infer_properties(&body)?,
        })
    }
}
```

### Ownership Inference

```rust
impl OwnershipAnalyzer {
    fn infer_ownership(&mut self, expr: &HirExpr) -> Result<Ownership> {
        match expr {
            HirExpr::Variable(name) => {
                let usage = self.track_usage(name)?;
                match usage {
                    Usage::MovedFrom => Ok(Ownership::Moved),
                    Usage::BorrowedMut => Ok(Ownership::MutBorrow),
                    Usage::BorrowedShared => Ok(Ownership::SharedBorrow),
                    Usage::Owned => Ok(Ownership::Owned),
                }
            }
            HirExpr::MethodCall { receiver, method, .. } => {
                // Check if method moves or borrows
                let method_sig = self.lookup_method(receiver, method)?;
                self.apply_method_semantics(receiver, method_sig)
            }
            _ => self.default_ownership(expr)
        }
    }
}
```

## Memory Management

### String Interning for Optimization

```rust
pub struct StringInterner {
    strings: FxHashMap<String, InternedString>,
    usage_counts: FxHashMap<InternedString, usize>,
}

impl StringInterner {
    pub fn should_intern(&self, s: &str) -> bool {
        // Intern strings used more than 3 times
        self.usage_counts.get(s).map_or(false, |&count| count > 3)
    }
    
    pub fn intern(&mut self, s: String) -> InternedString {
        *self.strings.entry(s.clone())
            .or_insert_with(|| {
                let id = InternedString(self.strings.len());
                *self.usage_counts.entry(id).or_insert(0) += 1;
                id
            })
    }
}
```

## Release Checklist

Before ANY release:

- [ ] All examples transpile and run correctly
- [ ] Property tests achieve 100% coverage on supported features
- [ ] Generated code passes `cargo clippy -- -D warnings`
- [ ] Benchmarks show no performance regression
- [ ] Documentation examples are tested
- [ ] CHANGELOG updated with breaking changes
- [ ] Version bump follows semver

---

**Remember**: Perfect transpilation is better than feature-complete transpilation. Every line of generated Rust must be idiomatic. Every error must guide the user to a solution. Ship nothing that doesn't meet these standards.
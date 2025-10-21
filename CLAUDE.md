# CLAUDE.md - Depyler Compiler Implementation Protocol

## IMPORTANT: Auto-Generated Files
**NEVER EDIT `deep_context.md`** - This file is auto-generated and will be overwritten. Any changes should be made to the source files instead.

## Prime Directive

**Generate correct Rust code that compiles on first attempt. Quality is built-in, not bolted-on.**

## Project Context

Depyler is a Python-to-Rust transpiler focusing on energy-efficient, safe code
generation with progressive verification. The system must produce idiomatic Rust
code with formal correctness guarantees for a practical Python subset.

## Transpilation Workflows

Depyler supports bidirectional Python-Rust workflows inspired by bashrs "purify" methodology:

### PRIMARY WORKFLOW: Python → Safe Rust

Write actual Python code, test with standard Python tooling, then transpile to verified, panic-free Rust.

**Why this is powerful**:
- Write REAL Python code (with type annotations)
- Use standard Python tooling: pytest, mypy, black
- Test with `uv run pytest` BEFORE transpiling
- Get verified, panic-free Rust output
- Guaranteed safe against runtime panics
- Idiomatic Rust (passes clippy with `-D warnings`)

### SECONDARY WORKFLOW (Future): Python → Rust → Purified Python

Ingest legacy Python scripts, convert to Rust with tests, then transpile back to purified Python.

**Purification pipeline** (planned):
1. Parse legacy Python (with dynamic typing, mutable globals, side effects)
2. Convert to Rust + generate comprehensive tests
3. Transpile back to purified Python (typed, functional, deterministic)

This "cleans up" existing Python scripts by running them through the Depyler safety pipeline.

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

## EXTREME CLI VALIDATION (From ruchy)

**CRITICAL**: Every transpiled Rust file MUST work with native Rust tooling. NO EXCEPTIONS.

### 15-Tool Validation Protocol (MANDATORY for ALL Examples)

Every transpiled .rs file MUST pass validation with these native tools:

```bash
# GATE 1: Compilation
rustc --crate-type lib example.rs --deny warnings

# GATE 2: Clippy (strict mode)
rustc --crate-type lib example.rs -Z extra-lints -- -D warnings

# GATE 3: Format check
rustfmt --check example.rs

# GATE 4: Basic compilation
rustc example.rs --crate-type lib

# GATE 5: LLVM IR generation
rustc example.rs --emit=llvm-ir -o /dev/null

# GATE 6: Assembly generation
rustc example.rs --emit=asm -o /dev/null

# GATE 7: MIR generation
rustc example.rs --emit=mir -o /dev/null

# GATE 8: Syntax check
rustc example.rs --parse-only

# GATE 9: Type check
rustc example.rs --crate-type lib 2>&1 | grep -v "warning"

# GATE 10: Dependency analysis
cargo tree (if part of workspace)

# GATE 11: Documentation build
rustdoc example.rs --crate-type lib -o /tmp/docs

# GATE 12: Macro expansion check
rustc example.rs -Z unpretty=expanded > /dev/null

# GATE 13: HIR dump (verify HIR generation)
rustc example.rs -Z unpretty=hir > /dev/null

# GATE 14: Dead code analysis
rustc example.rs --crate-type lib -W dead-code

# GATE 15: Complexity analysis
pmat analyze complexity example.rs --max-cyclomatic 10
```

### STOP THE LINE Protocol (Zero Bypass Tolerance)

**SACRED RULE**: NEVER bypass validation gates. EVER.

**ABSOLUTELY FORBIDDEN**:
- Skipping tools "because they don't apply" - FIX the transpiler instead
- Marking gates as "optional" - ALL gates are MANDATORY
- Working around tool failures - Fix root cause
- Deferring fixes to "later" - Fix IMMEDIATELY

**When ANY gate fails**:
1. 🛑 **HALT ALL WORK** - Stop transpiling new examples
2. 📋 **CREATE TICKET** - Document failure in roadmap (DEPYLER-XXXX)
3. 🔍 **ROOT CAUSE ANALYSIS** - Identify transpiler bug
4. 🧪 **WRITE FAILING TEST** - Reproduce issue with unit test
5. 🔧 **FIX TRANSPILER** - Implement fix with EXTREME TDD
6. ✅ **RE-VALIDATE ALL** - Re-transpile and re-test ALL examples
7. ▶️ **RESUME WORK** - Continue only when ALL gates pass

### Validation Script

See `examples/validate_all.sh` for full implementation of 15-tool validation protocol.

### Missing Feature Protocol (From ruchy)

**CRITICAL**: When transpiler lacks a feature needed for validation, FIX IT IMMEDIATELY.

**WRONG Approach** ❌:
- Skip examples that need the feature
- Work around with manual edits
- Mark as "TODO" and continue

**RIGHT Approach** ✅:
1. Create DEPYLER-XXXX ticket for missing feature
2. Mark ticket as P0 (blocks validation)
3. Implement feature with EXTREME TDD
4. Add property tests (10K+ iterations)
5. Verify with mutation testing
6. ONLY THEN resume validation

**Example**: If transpiler doesn't generate proper `#[derive(Debug)]` annotations:
```bash
# WRONG: Skip validation for affected files ❌
echo "Skipping examples/foo.rs - needs Debug derive"

# RIGHT: Fix transpiler immediately ✅
1. Create DEPYLER-0218: Generate #[derive(Debug)] for structs
2. Write failing test: test_derive_debug_generation()
3. Implement code generation logic
4. Verify all 731 tests pass
5. Re-transpile ALL examples
6. Resume validation
```

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

### Multi-Level Testing (From decy/ruchy)

Every module MUST have 4 types of tests:

#### 1. Unit Tests (≥5 per module)
```bash
# Location: #[cfg(test)] mod tests or separate *_tests.rs
cargo test -p depyler-core

# Target: 85% coverage minimum
```

**Requirements**:
- Test each function independently
- Mock external dependencies
- Cover happy path + error cases
- ≥5 unit tests per module

#### 2. Property Tests (≥3 per module)
```bash
# Location: tests/*_property_tests.rs
cargo test --features proptest-tests

# Target: 100 properties × 1000 cases = 100K+ total tests
```

**Property Test Requirements**:
- ≥3 properties per module, 1000 iterations each
- Cover invariants: determinism, panic-free, idempotency
- Use `proptest` crate with custom strategies

#### 3. Doctests (≥2 per public function)
```bash
# Run doctests
cargo test --doc

# Target: 100% of public functions have working examples
```


#### 4. Integration Tests (End-to-End)
```bash
# Location: tests/*.rs (not in #[cfg(test)])
cargo test --test '*'

# Target: Full transpile → compile → execute pipeline
```

Integration tests must validate the full transpile → compile → execute pipeline.

### Mutation Testing (From ruchy)

**CRITICAL**: Tests must PROVE they catch real bugs.

```bash
# Run mutation testing (CI only - too slow for local)
cargo install cargo-mutants
cargo mutants --workspace

# Target: ≥75% mutation kill rate (ruchy standard)
# Aspirational: ≥90% mutation kill rate (decy standard)
```

**What is Mutation Testing?**
- Automatically introduces bugs into your code
- Verifies tests catch those bugs
- If tests still pass with buggy code, your tests are weak
- Empirical proof that tests are effective

**Example Mutation**:
```rust
// Original code:
fn add(a: i32, b: i32) -> i32 {
    a + b  // ← Mutation: change to a - b
}

// If tests still pass, mutation "survived" (BAD)
// If tests fail, mutation was "killed" (GOOD)
```

**Mutation Testing Requirements**:
- Run in CI on every PR
- ≥75% kill rate minimum
- Track mutation score per sprint
- Improve weak tests identified by mutations

### Test Naming Convention (From ruchy)

**MANDATORY**: All tests MUST follow this naming pattern for traceability:

```rust
#[test]
fn test_DEPYLER_XXXX_<section>_<feature>_<scenario>() {
    // Example: test_DEPYLER_0216_codegen_int_cast_bool_to_int()
}
```

**Pattern Breakdown**:
- `DEPYLER_XXXX`: Ticket ID for traceability
- `<section>`: Module/component (codegen, parser, analyzer, etc.)
- `<feature>`: Specific feature being tested
- `<scenario>`: Test scenario (happy_path, error_case, edge_case, etc.)

**Why This Matters**:
- Trace test failures back to requirements
- Know which ticket a test validates
- Enable roadmap-driven test coverage tracking
- Facilitate issue investigation

### Coverage Requirements

```bash
# Generate coverage report (using cargo-llvm-cov, NOT tarpaulin)
cargo llvm-cov --all-features --workspace --html --open

# Minimum: 80% line coverage
# Target: 85% line coverage
# Critical modules (codegen, properties): 90%
```

**Per-Module Coverage Targets**:
- `depyler-core`: 85% minimum
- `depyler-ast-bridge`: 85% minimum
- `depyler-codegen`: 90% minimum (critical path)
- `depyler-properties`: 90% minimum (safety critical)
- `depyler-verify`: 85% minimum
- `depyler-mcp`: 70% minimum (external integration)

### Testing Workflow Summary

```bash
# STEP 1: Run all test levels
cargo test --workspace                    # Unit + integration
cargo test --doc                          # Doctests
cargo test --features proptest-tests      # Property tests

# STEP 2: Check coverage
cargo llvm-cov --all-features --workspace --fail-under-lines 80

# STEP 3: Mutation testing (CI only)
cargo mutants --workspace --fail-under 75

# STEP 4: Validate examples with 15-tool protocol
./examples/validate_all.sh

# ALL MUST PASS before commit
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

### RED-GREEN-REFACTOR Workflow (From decy)

**MANDATORY**: Every DEPYLER ticket MUST follow the RED-GREEN-REFACTOR cycle:

#### Phase 1: RED (Write Failing Tests)
```bash
# Write comprehensive failing tests FIRST
cargo test test_DEPYLER_XXXX -- --nocapture  # Tests MUST FAIL

# Commit RED phase (ONLY time --no-verify is allowed)
git commit --no-verify -m "[RED] DEPYLER-XXXX: Add failing tests for <feature>"

# Update roadmap.yaml: phase: RED, status: in_progress
```

**RED Phase Requirements**:
- ≥5 unit tests demonstrating expected behavior
- ≥2 integration tests for end-to-end workflow
- ≥1 property test (if applicable)
- ALL tests must FAIL (proving they test new behavior)

#### Phase 2: GREEN (Minimal Implementation)
```bash
# Implement MINIMAL code to make tests pass
cargo test test_DEPYLER_XXXX -- --nocapture  # Tests MUST PASS

# Commit GREEN phase
git commit -m "[GREEN] DEPYLER-XXXX: Implement <feature>"

# Update roadmap.yaml: phase: GREEN
```

**GREEN Phase Requirements**:
- ALL tests from RED phase must PASS
- Implementation can be "ugly" - focus on correctness
- No quality gates required yet

#### Phase 3: REFACTOR (Meet Quality Standards)
```bash
# Clean up code to meet ALL quality gates
pmat tdg . --min-grade A- --fail-on-violation
pmat analyze complexity --max-cyclomatic 10 --fail-on-violation
cargo clippy --all-targets --all-features -- -D warnings
cargo llvm-cov report --fail-under-lines 80

# Commit REFACTOR phase (quality gates enforced)
git commit -m "[REFACTOR] DEPYLER-XXXX: Meet quality standards

- Complexity: All functions ≤10
- Coverage: 82.3% (target: 80%)
- SATD: 0 violations
- TDG Grade: A-

Closes #XXXX"

# Update roadmap.yaml: phase: DONE, status: done
```

**REFACTOR Phase Requirements**:
- TDG grade A- or higher
- Complexity ≤10 per function
- Zero SATD comments
- Coverage ≥80%
- All quality gates PASS

#### Phase 4: SQUASH (Final Commit)
```bash
# Squash RED/GREEN/REFACTOR into single commit
git rebase -i HEAD~3

# Final commit message format:
git commit -m "DEPYLER-XXXX: <Feature description>

- Coverage: 82.3% ✅
- TDG Grade: A- ✅
- Complexity: ≤10 all functions ✅
- SATD: 0 ✅

Closes #XXXX"
```

### SACRED RULE: NEVER BYPASS QUALITY GATES

**ABSOLUTELY FORBIDDEN** (except during RED phase):
- `git commit --no-verify` - ONLY allowed during RED phase
- Skipping tests "temporarily" - NO exceptions
- Ignoring failing quality checks - Must fix EVERY defect
- Dismissing warnings as "unrelated" - All defects matter

**Toyota Way Principle**: Stop the line for ANY defect. No defect is too small. No shortcut is acceptable.

**ONLY EXCEPTION**: During RED phase, use `--no-verify` to commit failing tests. ALL other commits MUST pass quality gates.

### Pre-commit Hooks (MANDATORY)

Pre-commit hooks enforce:
- Documentation updates (roadmap.md, CHANGELOG.md) for Rust changes
- PMAT complexity ≤10, zero SATD
- TDG grade A- minimum
- Coverage ≥80%
- Clippy `-D warnings`

See `scripts/pre-commit` for full implementation.

### CI/CD Pipeline Enforcement

CI enforces 6 mandatory gates: TDG grade A-, complexity ≤10, zero SATD, clippy `-D warnings`, coverage ≥80%, all tests pass. See `.github/workflows/quality-gates.yml`.

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

- Transpilation: >10MB/s throughput
- Verification: <50ms for typical module
- Generated Rust: compiles in <500ms

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


## Error Diagnostics Quality

Error messages must be Elm-level: show source context, highlight errors, provide expected vs found types, and suggest actionable fixes.

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

String interning optimizes repeated string literals (>3 uses). Conservative ownership defaults prevent lifetime errors.

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
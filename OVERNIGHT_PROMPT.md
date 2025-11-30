# DEPYLER Overnight Autonomous Fix Session

## Mission
Run the CITL convergence loop, identify and fix the top error clusters blocking transpilation, and drive compilation success rate as high as possible. Work autonomously for hours without stopping until all actionable fixes are exhausted.

## Session Reference
- **Previous Session**: DEPYLER-0614 (chained assignments), DEPYLER-0615 (stub functions)
- **Expected Top Blocker**: E0433 (missing imports) - 2744 blocked examples
  - ⚠️ Verify with Phase 1 baseline - if E0425 persists, address it first
- **Next Ticket**: DEPYLER-0616 onwards

---

## Phase 1: Baseline Assessment (ALWAYS START HERE)

```bash
cd /home/noah/src/depyler

# 1. Build release binary
cargo build --release 2>&1 | tail -20

# 2. Run all tests to verify baseline
cargo test 2>&1 | tail -50

# 3. Run convergence analysis to get current error clusters
./target/release/depyler converge \
    --input-dir ../reprorusted-python-cli/examples \
    --target-rate 100 \
    --max-iterations 1 \
    --verbose 2>&1 | tee /tmp/converge_baseline.log

# 4. Extract top error clusters from output
grep -E "^(E[0-9]{4}|error\[|blocked|cluster)" /tmp/converge_baseline.log | head -30
```

Record these metrics:
- Total examples
- Compilation success rate
- Top 5 error codes with blocked counts

---

## Phase 2: Iterative Fix Loop

For EACH error cluster (starting with highest impact):

### Step 2.1: Analyze Error Pattern
```bash
# Get sample errors for the top cluster
./target/release/depyler transpile ../reprorusted-python-cli/examples/example_simple/trivial_cli.py --trace --explain 2>&1

# For specific error code, find all affected examples
grep -r "E0433\|E0599\|E0308" ../reprorusted-python-cli/examples/*/main.rs 2>/dev/null | head -20
```

### Step 2.2: Locate Fix in Depyler Core

Key files by error type:

| Error Code | Root Cause | Fix Location |
|-----------|------------|--------------|
| **E0433** | Missing crate/module | `rust_gen/import_gen.rs`, `rust_gen/context.rs` |
| **E0599** | Missing method | `rust_gen/expr_gen.rs`, `rust_gen/builtin_conversions.rs` |
| **E0308** | Type mismatch | `type_mapper.rs`, `rust_gen/expr_gen.rs` |
| **E0277** | Missing trait impl | `rust_gen/expr_gen.rs` (Display vs Debug) |
| **E0425** | Undefined variable | `rust_gen/func_gen.rs`, `rust_gen/stmt_gen.rs` |
| **E0382/E0502/E0507** | Borrow issues | `borrowing_context.rs`, `lifetime_analysis.rs` |
| **TRANS** | Unsupported pattern | `ast_bridge/converters.rs`, `hir.rs` |

### Step 2.3: Implement Fix

Pattern from DEPYLER-0614/0615:
1. Add new variant to HIR if needed (`hir.rs`)
2. Update AST conversion (`ast_bridge/converters.rs`)
3. Update code generation (`rust_gen/*.rs`)
4. Handle exhaustive matches in ALL files:
   - `hir.rs`
   - `converters.rs`
   - `stmt_gen.rs`
   - `borrowing_context.rs`
   - `codegen.rs`
   - `direct_rules.rs`
   - `lifetime_analysis.rs`
   - `type_flow.rs`

### Step 2.4: Validate Fix
```bash
cd /home/noah/src/depyler

# Run all tests
cargo test 2>&1 | tail -30

# Check clippy
cargo clippy -- -D warnings 2>&1 | tail -30

# Run convergence to measure improvement
./target/release/depyler converge \
    --input-dir ../reprorusted-python-cli/examples \
    --target-rate 100 \
    --max-iterations 1 \
    --verbose 2>&1 | tee /tmp/converge_after_fix.log

# Compare: did blocked count decrease?
```

### Step 2.5: Commit Fix
```bash
git add -A
git commit -m "$(cat <<'EOF'
fix: Handle [ERROR_TYPE] in transpiler ([DESCRIPTION])

- Added [SPECIFIC CHANGE]
- Updated exhaustive matches in [FILES]
- Reduces [ERROR_CODE] cluster from X to Y blocked examples

Refs DEPYLER-06XX

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"

# Continue to next error cluster
```

---

## Phase 3: Error-Specific Fix Patterns

### E0433 - Missing Imports (Current Top Blocker)

**Root Cause**: Generated Rust uses crates not in Cargo.toml or modules not imported

**Fix Strategy**:
1. In `rust_gen/import_gen.rs`, detect when stdlib mappings require external crates
2. Generate `Cargo.toml` dependencies or add missing `use` statements
3. For cross-file imports, generate stub modules (done in DEPYLER-0615)

**Key Code Location**: `/home/noah/src/depyler/crates/depyler-core/src/rust_gen/import_gen.rs`

### E0599 - Missing Method

**Root Cause**: Python method called that has no Rust equivalent mapping

**Fix Strategy**:
1. Check `builtin_conversions.rs` for method mappings
2. Add conversion for Python→Rust method (e.g., `.append()` → `.push()`)
3. Handle method chaining properly

### E0277 - Trait Not Implemented

**Root Cause**: Often `println!("{}", vec)` where Vec doesn't implement Display

**Fix Strategy**:
1. In `expr_gen.rs`, detect when format string uses `{}` with Vec/HashMap
2. Convert to `{:?}` for Debug trait
3. Add `#[derive(Debug)]` to generated structs

### TRANS Errors - Unsupported Patterns

**From corpus analysis, these are blocking:**
- Forward reference type annotations (string literals as types)
- Dynamic function calls (`f(a)` where `f` is a variable)
- Complex subscript calls (`self.on_exit[self.current_state]`)

**Fix Strategy**:
1. In `ast_bridge/converters.rs`, handle `Constant` type annotations → resolve to actual type
2. For dynamic calls, generate trait bounds or use `dyn Fn` types
3. For subscript calls, detect callable-in-collection patterns

---

## Phase 4: Long-Running Convergence

After fixing major clusters, run extended convergence:

```bash
cd /home/noah/src/depyler

# Extended overnight run
./target/release/depyler converge \
    --input-dir ../reprorusted-python-cli/examples \
    --target-rate 100 \
    --max-iterations 100 \
    --auto-fix \
    --fix-confidence 0.8 \
    --checkpoint-dir ./nightly_checkpoints \
    --parallel-jobs $(nproc) \
    --verbose 2>&1 | tee logs/overnight_$(date +%Y%m%d_%H%M%S).log
```

---

## Critical Rules

1. **NEVER stop after one fix** - Continue to next error cluster
2. **ALWAYS run tests after each fix** - No broken builds
3. **ALWAYS commit working changes** - Small, focused commits
4. **NEVER work on branches** - All work on master directly
5. **Track progress** - Use TodoWrite to track error clusters tackled
6. **Measure impact** - Record blocked count before/after each fix
7. **Prioritize by impact** - `blocked_count * fix_confidence`

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Compilation success rate | Increase from baseline |
| E0433 blocked count | < 1000 (from 2744) |
| Total error clusters | Decrease each iteration |
| Test suite | 100% passing |
| Clippy | Zero warnings |

---

## File Quick Reference

```
/home/noah/src/depyler/
├── crates/depyler-core/src/
│   ├── hir.rs                    # HIR definitions
│   ├── codegen.rs                # Main codegen entry
│   ├── borrowing_context.rs      # Borrow checker analysis
│   ├── lifetime_analysis.rs      # Lifetime inference
│   ├── direct_rules.rs           # Pattern matching rules
│   ├── type_flow.rs              # Type propagation
│   ├── ast_bridge/
│   │   └── converters.rs         # Python AST → HIR
│   └── rust_gen/
│       ├── expr_gen.rs           # Expression codegen (14k lines)
│       ├── stmt_gen.rs           # Statement codegen
│       ├── func_gen.rs           # Function codegen
│       ├── import_gen.rs         # Import codegen
│       ├── context.rs            # Codegen context
│       └── builtin_conversions.rs# Python→Rust builtins
└── training_corpus/
    └── errors.jsonl              # Error patterns for training
```

---

## Starting Command

```bash
# Begin autonomous overnight session
cd /home/noah/src/depyler && cargo build --release && cargo test && \
./target/release/depyler converge --input-dir ../reprorusted-python-cli/examples --target-rate 100 --max-iterations 1 --verbose
```

Work through each error cluster systematically. Do not stop until you've addressed all high-impact clusters or exhausted all actionable fixes.

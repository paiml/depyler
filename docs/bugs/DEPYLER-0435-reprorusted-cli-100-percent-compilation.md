# DEPYLER-0435: reprorusted-python-cli 100% Compilation Master Ticket

## Status: IN PROGRESS
- **Created**: 2025-11-19
- **Priority**: HIGH
- **Type**: Epic/Tracker
- **Project**: reprorusted-python-cli
- **Goal**: Achieve 13/13 examples compiling (100%)

## Current Status

**Compilation Rate**: 4/13 (30.8%)
**Transpilation Rate**: 13/13 (100%) ✅

### Compiling Successfully (4/13)
1. ✅ trivial_cli - Basic argparse
2. ✅ flag_parser - Boolean flags
3. ✅ positional_args - Positional arguments
4. ✅ git_clone - Subcommands (FIXED: DEPYLER-0425)

### Transpiling But Not Compiling (9/13)
5. ❌ complex_cli - 13 errors (needs DEPYLER-0428)
6. ❌ stdlib_integration - 41 errors (needs DEPYLER-0430)
7. ❌ config_manager - 43 errors (needs DEPYLER-0430)
8. ❌ task_runner - 22 errors (needs DEPYLER-0429)
9. ❌ env_info - 27 errors (needs DEPYLER-0430)
10. ❌ pattern_matcher - 46 errors (needs DEPYLER-0431)
11. ❌ stream_processor - 36 errors (needs DEPYLER-0432)
12. ❌ csv_filter - 25 errors (needs DEPYLER-0433)
13. ❌ log_analyzer - 36 errors (needs DEPYLER-0434)

---

## Sub-Tickets (7 Total)

### HIGH Priority (5-7 hours) - Target: 6-7/13 (46-54%)

#### DEPYLER-0428: argparse.ArgumentTypeError Exception Handling
- **Status**: RED phase complete, needs GREEN+REFACTOR
- **Effort**: 3-4 hours
- **Blocks**: complex_cli (13 errors)
- **Impact**: +1-2 examples
- **Test Suite**: `crates/depyler-core/tests/depyler_0428_argument_type_error.rs`
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Implementation**:
  - Map `raise argparse.ArgumentTypeError(msg)` → `return Err(msg.to_string())`
  - Update function return types: `T` → `Result<T, String>`
  - Update callers to handle Result
  - Fix try/except ValueError patterns
- **Files**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`, `func_gen.rs`
- **Next Step**: `pmat prompt show continue DEPYLER-0428`

#### DEPYLER-0429: subprocess Module Support
- **Status**: Not started
- **Effort**: 2-3 hours
- **Blocks**: task_runner (22 errors)
- **Impact**: +1 example
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Implementation**:
  - Add subprocess to stdlib mapping
  - Implement `subprocess.run(cmd, capture_output=True, cwd=path, check=True)`
  - Map to `std::process::Command`
  - Handle `CompletedProcess` return type with stdout/stderr
- **Files**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (try_convert_subprocess_method)
- **Next Step**: `pmat prompt show continue DEPYLER-0429`

---

### MEDIUM Priority (8-12 hours) - Target: 10-11/13 (77-85%)

#### DEPYLER-0430: os/sys/platform Module Gaps
- **Status**: Not started
- **Effort**: 4-6 hours
- **Blocks**: env_info (27 errors), config_manager (43 errors), stdlib_integration (41 errors)
- **Impact**: +2-3 examples
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Missing Implementations**:
  - `os.path.expanduser()` → `dirs::home_dir()` + path join
  - `os.makedirs(path, exist_ok=True)` → `std::fs::create_dir_all()`
  - `os.path.isfile()`, `os.path.isdir()` → `path.is_file()`, `path.is_dir()`
  - `platform.system()` → `std::env::consts::OS`
  - `platform.release()` → OS release detection
  - `sys.version` → Rust version constant
  - `os.environ["VAR"]` → `std::env::var("VAR")`
- **Files**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (try_convert_os_method, try_convert_sys_method, try_convert_platform_method)
- **Next Step**: `pmat prompt show continue DEPYLER-0430`

#### DEPYLER-0431: re (regex) Module Improvements
- **Status**: Not started
- **Effort**: 2-3 hours
- **Blocks**: pattern_matcher (46 errors)
- **Impact**: +1 example
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Missing Implementations**:
  - `re.Match.group(n)` → extract capture group
  - `re.Match.groups()` → all groups as tuple
  - `re.finditer(pattern, text)` → iterator over matches
  - Proper Option<Match> handling when no match
  - `re.Match.span()`, `re.Match.start()`, `re.Match.end()`
- **Files**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (try_convert_re_method)
- **Next Step**: `pmat prompt show continue DEPYLER-0431`

#### DEPYLER-0432: sys.stdin/stdout Stream Handling
- **Status**: Not started
- **Effort**: 2-3 hours
- **Blocks**: stream_processor (36 errors)
- **Impact**: +1 example
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Implementation**:
  - `sys.stdin` → `std::io::stdin()`
  - `sys.stdout` → `std::io::stdout()`
  - `for line in sys.stdin:` → `for line in stdin().lock().lines()`
  - `sys.stdin.read()` → read all to string
  - `sys.stdout.write()` → write bytes
- **Files**: `crates/depyler-core/src/rust_gen/expr_gen.rs` (try_convert_sys_method)
- **Next Step**: `pmat prompt show continue DEPYLER-0432`

---

### LOW Priority (3-5 hours) - Target: 13/13 (100%)

#### DEPYLER-0433: csv_filter Remaining Compilation Issues
- **Status**: Not started (DictWriter kwargs fixed via DEPYLER-0426)
- **Effort**: 1-2 hours
- **Blocks**: csv_filter (25 errors)
- **Impact**: +1 example
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Known Issues**:
  - Field access errors (likely fixed by other tickets)
  - Type conversion issues
  - May be resolved by DEPYLER-0428, 0430, 0431
- **Next Step**: Re-test after HIGH/MEDIUM tickets, then `pmat prompt show continue DEPYLER-0433`

#### DEPYLER-0434: log_analyzer Remaining Compilation Issues
- **Status**: Not started (Nested functions fixed via DEPYLER-0427)
- **Effort**: 2-3 hours
- **Blocks**: log_analyzer (36 errors)
- **Impact**: +1 example
- **MANDATORY Pre-Work**: Debug with `--trace` and Renacer (see Debugging Workflow section)
- **Known Issues**:
  - datetime module gaps (strptime, strftime)
  - collections.defaultdict support
  - File I/O patterns
  - May be partially resolved by DEPYLER-0430
- **Next Step**: Re-test after MEDIUM tickets, then `pmat prompt show continue DEPYLER-0434`

---

## Implementation Plan

### Phase 1: HIGH Priority (Target: 46-54%)
1. `pmat prompt show continue DEPYLER-0428` (ArgumentTypeError) - 3-4 hours
2. `pmat prompt show continue DEPYLER-0429` (subprocess) - 2-3 hours
3. Test: Expect 6-7/13 compiling

### Phase 2: MEDIUM Priority (Target: 77-85%)
1. `pmat prompt show continue DEPYLER-0430` (os/sys/platform) - 4-6 hours
2. `pmat prompt show continue DEPYLER-0431` (regex) - 2-3 hours
3. `pmat prompt show continue DEPYLER-0432` (streams) - 2-3 hours
4. Test: Expect 10-11/13 compiling

### Phase 3: LOW Priority (Target: 100%)
1. Re-test csv_filter and log_analyzer (may be auto-fixed)
2. `pmat prompt show continue DEPYLER-0433` if needed - 1-2 hours
3. `pmat prompt show continue DEPYLER-0434` if needed - 2-3 hours
4. Test: Expect 13/13 compiling ✅

---

## MANDATORY: Debugging Workflow (ALWAYS Use Before Implementation)

### 🚨 CRITICAL: Use Renacer + --trace FIRST

**NEVER implement fixes without debugging first!** Use these tools to understand root causes:

#### 1. Renacer - System Call Tracer
**Location**: `/home/noah/src/renacer/target/debug/renacer`

**Debugging transpiled binaries**:
```bash
# Example: Debug complex_cli after transpilation
cd /home/noah/src/reprorusted-python-cli/examples/example_complex

# Transpile with debug info
/home/noah/src/depyler/target/release/depyler transpile complex_cli.py -o complex_cli.rs

# Build with debug info (even if it fails, partial binaries can be traced)
rustc -g complex_cli.rs -o complex_cli 2>&1 | tee build_errors.txt

# If binary was created, trace it
/home/noah/src/renacer/target/debug/renacer -s -T -- ./complex_cli --help

# Analyze syscalls to find runtime issues
/home/noah/src/renacer/target/debug/renacer -c -- ./complex_cli
```

**Renacer Quick Reference**:
- `-s` : Source correlation (DWARF debug info)
- `-T` : Show syscall timing
- `-c` : Statistics summary
- `-e trace=file` : Filter to file operations only
- `--format json` : JSON output for parsing

#### 2. Depyler --trace Flag
**Use to see transpilation pipeline phases**:
```bash
cd /home/noah/src/depyler

# Trace transpilation to see where it fails/succeeds
cargo run --release --bin depyler -- transpile \
  /home/noah/src/reprorusted-python-cli/examples/example_complex/complex_cli.py \
  --trace

# Example output shows:
# [TRACE] Phase 1: Python AST parsing
# [TRACE] Phase 2: HIR conversion
# [TRACE] Phase 3: Type inference
# [TRACE] Phase 4: Rust codegen
```

#### 3. Mandatory Pre-Implementation Checklist

**For EVERY sub-ticket (DEPYLER-0428 through 0434)**:

1. **Understand the Error**:
   ```bash
   # Get exact error messages
   cd /home/noah/src/reprorusted-python-cli/examples/<example_name>
   cargo build --release 2>&1 | tee errors.txt
   ```

2. **Trace Transpilation**:
   ```bash
   # See where transpiler logic diverges
   cd /home/noah/src/depyler
   cargo run --release --bin depyler -- transpile <file.py> --trace
   ```

3. **Compare Python vs Generated Rust**:
   ```bash
   # Side-by-side analysis
   diff -u <file.py> <file.rs> | less
   ```

4. **Identify Root Cause**:
   - Is it missing stdlib mapping? (expr_gen.rs)
   - Is it type inference failure? (type_flow.rs)
   - Is it exception handling gap? (stmt_gen.rs)

5. **Write Test FIRST** (RED phase):
   - Add failing test to `crates/depyler-core/tests/depyler_0XXX_<feature>.rs`
   - Test must demonstrate the EXACT error from real example

6. **Only THEN Implement Fix** (GREEN + REFACTOR phases)

#### 4. Example: Debugging DEPYLER-0428 (ArgumentTypeError)

```bash
# Step 1: Understand the error
cd /home/noah/src/reprorusted-python-cli/examples/example_complex
cargo build --release 2>&1 | grep "ArgumentTypeError" | head -5

# Step 2: Trace transpilation
cd /home/noah/src/depyler
cargo run --release --bin depyler -- transpile \
  /home/noah/src/reprorusted-python-cli/examples/example_complex/complex_cli.py \
  --trace | grep -A 5 "ArgumentTypeError"

# Step 3: Find where transpiler handles exceptions
cd /home/noah/src/depyler
grep -rn "ArgumentTypeError" crates/depyler-core/src/

# Step 4: Check existing exception handling patterns
grep -rn "ValueError" crates/depyler-core/src/rust_gen/

# Step 5: Write test demonstrating the issue
cat > crates/depyler-core/tests/depyler_0428_argument_type_error.rs <<'EOF'
#[test]
fn test_argument_type_error_in_validator() {
    // Copy EXACT pattern from complex_cli.py
    let py = r#"
import argparse

def validate_port(value):
    try:
        port = int(value)
        if not 1024 <= port <= 65535:
            raise argparse.ArgumentTypeError(f"Port must be 1024-65535, got {port}")
        return port
    except ValueError:
        raise argparse.ArgumentTypeError(f"Invalid port: {value}")
"#;
    // Should transpile to: Result<i32, String>
}
EOF
```

---

## Testing Commands

### Quick Status Check
```bash
cd /home/noah/src/reprorusted-python-cli
./test_compile_proper.sh
```

### Test Single Example (WITH DEBUGGING)
```bash
cd /home/noah/src/reprorusted-python-cli/examples/<example_name>
cargo clean

# Transpile with trace
/home/noah/src/depyler/target/release/depyler transpile <file>.py -o <file>.rs --trace

# Count errors
cargo build --release 2>&1 | grep -c "^error"

# Get detailed errors
cargo build --release 2>&1 | tee build_errors.txt
```

### Full Validation
```bash
# All examples
cd /home/noah/src/reprorusted-python-cli
for ex in example_*; do
    echo "Testing $ex..."
    cd "$ex"
    cargo build --release >/dev/null 2>&1 && echo "✅ PASS" || echo "❌ FAIL"
    cd ..
done
```

---

## Quality Gates (MANDATORY for each sub-ticket)

### EXTREME TDD Process
1. **RED Phase**: Write failing test first
2. **GREEN Phase**: Minimal fix to pass test
3. **REFACTOR Phase**: Meet quality standards
   - TDG ≤ 2.0
   - Cyclomatic complexity ≤ 10
   - Cognitive complexity ≤ 15
   - Test coverage ≥ 80%
   - Zero SATD (TODO/FIXME/HACK)
4. **Verify**: Real-world example compiles
5. **Commit**: Proper format with TDG scores

### Pre-commit Checklist
- [ ] All tests passing (`cargo test --workspace`)
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Quality gates passing (`pmat quality-gate --fail-on-violation`)
- [ ] Example compiles (`cargo build --release` in example dir)
- [ ] No regressions (other examples still compile)

---

## Progress Tracking

### Completed Sub-Tickets
- None yet

### In Progress
- DEPYLER-0428 (RED phase complete)

### Blocked
- None

---

## Success Criteria

- [ ] All 7 sub-tickets completed
- [ ] 13/13 examples compile successfully (100%)
- [ ] All quality gates passing
- [ ] Zero regressions in existing examples
- [ ] Comprehensive test coverage for all fixes
- [ ] Documentation updated

---

## Related Tickets (Already Complete)

- ✅ DEPYLER-0424: Handler function parameter types
- ✅ DEPYLER-0425: Subcommand field access pattern matching
- ✅ DEPYLER-0426: csv.DictWriter keyword argument support
- ✅ DEPYLER-0427: Nested function support

---

## Estimated Total Effort

| Phase | Hours | Target Rate |
|-------|-------|-------------|
| Phase 1 (HIGH) | 5-7 | 46-54% |
| Phase 2 (MEDIUM) | 8-12 | 77-85% |
| Phase 3 (LOW) | 3-5 | 100% |
| **TOTAL** | **16-24** | **13/13** |

---

## Notes

- All sub-tickets follow EXTREME TDD methodology
- Each ticket maintains project quality (Grade A, TDG ≤2.0)
- Use `pmat prompt show continue DEPYLER-XXXX` for each sub-ticket
- Test after each phase to validate progress
- Some LOW priority tickets may auto-resolve after MEDIUM fixes

---

## References

- Test Suite Location: `/home/noah/src/depyler/crates/depyler-core/tests/`
- Example Location: `/home/noah/src/reprorusted-python-cli/examples/`
- Test Script: `/home/noah/src/reprorusted-python-cli/test_compile_proper.sh`
- Quality Guidelines: `/home/noah/src/depyler/CLAUDE.md`
- PMAT Continue: `pmat prompt show continue`

---

**Last Updated**: 2025-11-19
**Status**: 4/13 compiling (30.8%), 9 remaining
**Next Action**: Read "MANDATORY: Debugging Workflow" section, then `pmat prompt show continue DEPYLER-0428`
**Critical Reminder**: ALWAYS use `--trace` and Renacer debugging BEFORE implementation!

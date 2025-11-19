# DEPYLER-0429: subprocess Module Support - Exception Variable Binding

## Status: IN PROGRESS (RED Phase)
- **Created**: 2025-11-19
- **Priority**: P0 (CRITICAL - STOP THE LINE)
- **Type**: Transpiler Bug + Feature Gap
- **Parent**: DEPYLER-0435 (reprorusted-python-cli 100% compilation)
- **Blocks**: task_runner.py (22 compilation errors)
- **Estimated Effort**: 2-3 hours
- **Actual Effort**: TBD

## Problem Statement

The task_runner.py example fails to compile with 22 errors due to TWO distinct root causes:

### Issue 1: Exception Variable Binding (CRITICAL - P0)
**Status**: NEW DEFECT - **STOP THE LINE**

```python
except subprocess.CalledProcessError as e:
    print(f"Command failed with exit code {e.returncode}", file=sys.stderr)
    sys.exit(e.returncode)
```

**Current (WRONG)**:
```rust
println!(
    "{}",
    format!("Command failed with exit code {:?}", e.returncode)
);  // ❌ E0425: cannot find value `e` in this scope
std::process::exit(e.returncode);  // ❌ E0425: cannot find value `e`
```

**Root Cause**: The transpiler recognizes `except Exception as e:` syntax but **DOES NOT bind the exception variable `e`** in the generated Rust code. The exception handler body references `e` but it's never declared.

**Expected (CORRECT)**:
```rust
// Pattern 1: Match-based (if handler uses exception variable)
match result {
    Ok(value) => { /* try body */ },
    Err(e) => {  // ✅ Variable `e` bound here
        eprintln!("Command failed with exit code {}", e.returncode);
        std::process::exit(e.returncode);
    }
}

// Pattern 2: If-let based (simpler cases)
if let Err(e) = result {
    eprintln!("Command failed with exit code {}", e.returncode);
    std::process::exit(e.returncode);
}
```

**Impact**: BLOCKS ALL Python code using `except Exception as e:` pattern (extremely common!)

### Issue 2: subprocess.run() Type Inference (Feature Gap)
**Status**: Working (subprocess.run() already transpiled correctly!)

The transpiler ALREADY converts `subprocess.run()` to `std::process::Command`! No work needed here! ✅

**Evidence**:
```rust
// Generated correctly (lines 37-53)
result = {
    let cmd_list = cmd;
    let mut cmd = std::process::Command::new(&cmd_list[0]);  // ✅ Correct!
    cmd.args(&cmd_list[1..]);
    cmd.current_dir(cwd);
    let output = cmd.output().expect("subprocess.run() failed");
    struct SubprocessResult {
        returncode: i32,
        stdout: String,
        stderr: String,
    }
    SubprocessResult {
        returncode: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
};
```

**Remaining Issues** (type inference - separate from exception binding):
- Parameter types: `serde_json::Value` instead of `Vec<String>`, `bool`, `Option<PathBuf>`
- These are TYPE INFERENCE bugs, NOT subprocess bugs

## Root Cause Analysis

### Exception Variable Binding Bug

**Location**: `crates/depyler-core/src/rust_gen/stmt_gen.rs::codegen_try_stmt()`

**Current Behavior**:
1. Parser correctly extracts exception type: "CalledProcessError"
2. Parser correctly extracts variable name: "e"
3. Properties analysis recognizes this as an exception handler
4. **BUG**: codegen_try_stmt() generates handler body but NEVER binds the variable

**Analysis**:
```rust
// Line 2296-2335 in stmt_gen.rs - DEPYLER-0437 fix
if handlers.len() == 1
    && handlers[0].exception_type.as_deref() == Some("ValueError")
    && handlers[0].name.is_none()  // ❌ BUG: Only handles unnamed exceptions!
{
    // ... match generation ...
}

// Lines after 2335 - Sequential handler generation (WRONG)
for handler in handlers {
    let handler_body = codegen_stmts(handler.body, ctx)?;
    // ❌ BUG: handler_body uses `e` but we never bind it!
    // ❌ Missing: if let Some(exc_var) = handler.name { ... }
}
```

**THE FIX**: When `handler.name.is_some()`, we MUST bind the exception variable:

```rust
// CORRECT pattern:
if let Some(exc_var) = &handlers[0].name {
    // Exception variable is bound
    let ok_body = /* try body */;
    let err_var = safe_ident(exc_var);
    let err_body = /* handler body - can reference exc_var */;

    quote! {
        match result {
            Ok(_) => { #ok_body },
            Err(#err_var) => { #err_body }  // ✅ Variable bound!
        }
    }
} else {
    // No exception variable - just catch and ignore
    quote! {
        if let Err(_) = result {
            #handler_body
        }
    }
}
```

## Files Affected

### Primary Bug (Exception Binding):
- `crates/depyler-core/src/rust_gen/stmt_gen.rs`
  - Function: `codegen_try_stmt()` (lines 2085-2376)
  - **BUG**: Does not bind exception variable when `handler.name.is_some()`
  - **FIX**: Generate match/if-let with bound variable

### Verification:
- `crates/depyler-core/src/ast_bridge/properties.rs`
  - Function: `analyze_exception_flow()` (lines 200-252)
  - Correctly extracts `handler.name` - no changes needed ✅

### Test Files:
- `crates/depyler-core/tests/depyler_0429_exception_variable_binding.rs` (NEW)

## Test Plan

### Unit Tests (depyler_0429_exception_variable_binding.rs)

```rust
#[test]
fn test_DEPYLER_0429_exception_binding_simple_except() {
    // Python:
    // try:
    //     x = int("abc")
    // except ValueError as e:
    //     print(e)

    // Expected: match with bound variable
    // match value.parse::<i32>() {
    //     Ok(x) => { /* ... */ },
    //     Err(e) => {  // ✅ Variable `e` bound
    //         println!("{}", e);
    //     }
    // }
}

#[test]
fn test_DEPYLER_0429_exception_binding_with_attribute_access() {
    // Python:
    // except subprocess.CalledProcessError as e:
    //     sys.exit(e.returncode)

    // Expected: Access exception attributes
    // Err(e) => { std::process::exit(e.returncode); }
}

#[test]
fn test_DEPYLER_0429_exception_binding_without_variable() {
    // Python:
    // except FileNotFoundError:  # No variable
    //     print("Not found")

    // Expected: Match without binding
    // Err(_) => { println!("Not found"); }
}

#[test]
fn test_DEPYLER_0429_subprocess_run_integration() {
    // Full task_runner.py transpilation test
    // Verify subprocess.run() + exception handling work together
}
```

### Integration Tests

1. **task_runner.py compilation**: Must compile with zero errors
2. **Exception variable usage**: All `e.returncode`, `e.attribute` references resolve
3. **subprocess.run()**: Correctly mapped to std::process::Command (already working!)

## Implementation Plan

### Phase 1: RED - Write Failing Tests ✅
```bash
# Create test file
touch crates/depyler-core/tests/depyler_0429_exception_variable_binding.rs

# Add 4 tests (simple, attribute access, no variable, integration)
cargo test test_DEPYLER_0429  # MUST FAIL initially
```

### Phase 2: GREEN - Fix Exception Variable Binding
```rust
// In codegen_try_stmt():

// Step 1: Check if exception variable is bound
if let Some(exc_var) = &handlers[0].name {
    // Step 2: Generate try body as Ok branch
    let ok_body = quote! { #(#try_stmts)* };

    // Step 3: Bind exception variable in Err branch
    let err_var = safe_ident(exc_var);
    let err_body = &handler_tokens[0];

    // Step 4: Generate match expression
    return Ok(quote! {
        match #result_expr {
            Ok(_) => { #ok_body },
            Err(#err_var) => { #err_body }
        }
    });
}
```

### Phase 3: REFACTOR - Clean Up + Edge Cases
- Handle multiple except blocks with different variables
- Handle `except Exception as e1: ... except OSError as e2: ...`
- Ensure complexity ≤10, test coverage ≥80%

## Verification Checklist

- [ ] All 4 unit tests passing
- [ ] task_runner.py compiles (0 errors)
- [ ] task_runner.py runs correctly
- [ ] Exception variable `e` accessible in handler
- [ ] subprocess.run() still works (regression test)
- [ ] Complexity ≤10 (pmat analyze complexity)
- [ ] Coverage ≥80% (cargo llvm-cov)
- [ ] No clippy warnings (cargo clippy -D warnings)

## Success Criteria

**MUST ACHIEVE**:
1. ✅ task_runner.py compiles with zero errors
2. ✅ Exception variable binding works for all `except X as e:` patterns
3. ✅ subprocess.run() continues to work (no regression)
4. ✅ All quality gates pass (complexity, coverage, clippy)
5. ✅ 5/13 examples compiling (was 4/13)

**Compilation Progress**:
- Current: 4/13 (30.8%)
- Target: 5/13 (38.5%)
- Impact: +1 example (task_runner)

## Time Tracking

- **Debug & Analysis**: 30 minutes (DONE)
- **RED Phase**: 45 minutes (estimated)
- **GREEN Phase**: 1 hour (estimated)
- **REFACTOR Phase**: 30 minutes (estimated)
- **Total**: 2-3 hours

## Related Tickets

- **DEPYLER-0428**: Exception flow analysis (COMPLETE) - Foundation for this fix
- **DEPYLER-0437**: Try/except match expressions (COMPLETE) - Similar pattern
- **DEPYLER-0435**: Master ticket (IN PROGRESS)

## References

- Python docs: https://docs.python.org/3/tutorial/errors.html#handling-exceptions
- Rust Result: https://doc.rust-lang.org/std/result/
- subprocess module: https://docs.python.org/3/library/subprocess.html

---

## Debugging Notes

### Renacer Analysis (NEW: v0.4.1)

```bash
# Verify exception variable is extracted in AST
depyler transpile task_runner.py --trace-transpiler-decisions 2>&1 | grep "except"

# Expected output:
# [EXCEPTION_ANALYSIS] except CalledProcessError as e:
#   - exception_type: CalledProcessError
#   - variable_name: e ✅
#   - handler_body uses: e.returncode (2 references)
#   - DECISION: Must bind variable `e` in Err branch

# Current (WRONG):
# [EXCEPTION_ANALYSIS] except CalledProcessError as e:
#   - exception_type: CalledProcessError
#   - variable_name: e ✅
#   - handler_body uses: e.returncode (2 references)
#   - DECISION: Sequential handler (no binding) ❌ BUG!
```

### Manual Verification

```bash
# Check generated code for exception binding
grep -A 10 "CalledProcessError" /home/noah/src/reprorusted-python-cli/examples/example_subprocess/task_runner.rs

# Current: NO binding of `e` (lines 104-106)
# Expected: `Err(e) => { ... }` with `e` bound
```

---

**STATUS**: Ready for RED phase implementation
**NEXT STEP**: `pmat prompt show continue DEPYLER-0429` to begin RED phase

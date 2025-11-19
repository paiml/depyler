# DEPYLER-0437: Try/Except Control Flow - Exception Handler Branching

**Status**: RED phase complete, GREEN phase ready for implementation
**Priority**: P0 (CRITICAL - STOP THE LINE)
**Effort**: 3-4 hours
**Parent**: DEPYLER-0428 (ArgumentTypeError support)
**Blocks**: complex_cli (unreachable code warnings)

---

## Problem Statement

Try/except blocks are transpiled as sequential code without proper exception handling branching. Exception handlers are appended after the try block, making them unreachable.

### Current Output (WRONG)

```python
def validator(value):
    try:
        num = int(value)
        return num
    except ValueError:
        return -1
```

Generates:
```rust
pub fn validator(value: &str) -> i32 {
    let num = value.parse::<i32>().unwrap_or_default();  // ❌ Handler lost
    return num;
    // Except handler code is never generated or is unreachable!
}
```

### Expected Output (CORRECT)

```rust
pub fn validator(value: &str) -> i32 {
    match value.parse::<i32>() {
        Ok(num) => num,        // ✅ Try body in Ok branch
        Err(_) => -1,          // ✅ Handler in Err branch
    }
}
```

---

## Root Cause Analysis

### Location
`crates/depyler-core/src/rust_gen/stmt_gen.rs::codegen_try_stmt()` (lines 2085-2450)

### The Function Structure

The `codegen_try_stmt` function has **5 special case handlers**:

1. **Lines 2096-2136**: Simple pattern detection
   - Detects: `try { return int(x) } except ValueError { return literal }`
   - Action: Set `simple_pattern_info` for later optimization

2. **Lines 2148-2215**: ZeroDivisionError handler
   - Detects: Floor division with `except ZeroDivisionError`
   - Action: Generate `if divisor == 0 { handler } else { result }`

3. **Lines 2296-2332**: String replacement optimization
   - Condition: `simple_pattern_info` is set AND try_stmts contains `unwrap_or_default`
   - Action: String replace `unwrap_or_default()` → `unwrap_or(exception_value)`
   - **ISSUE**: Uses `.unwrap_or()` which hides the exception, doesn't generate match

4. **Lines 2334-2353**: Sequential concatenation (THE BUG!)
   - Condition: `simple_pattern_info` is set BUT no `unwrap_or_default` found
   - Action: Concatenate try_stmts + handler_code sequentially
   - **ISSUE**: Handler code runs AFTER try body, not in exception path!

5. **Lines 2360-2430**: Exception binding with int() calls
   - Condition: Single handler with `except ValueError as e:` AND `int()` call
   - Action: Generate match expression with `Err(e)` binding
   - **PARTIAL FIX**: Only works when exception is bound to a variable

6. **Lines 2431+**: Default fallback
   - Action: Sequential concatenation of try + handlers + finally
   - **ISSUE**: No match expression, handlers unreachable

### Key Issue

**Most argparse validators** don't bind exceptions (`except ValueError:` not `except ValueError as e:`), so they hit the buggy fallback cases (#3, #4, #6) which either:
- Use `.unwrap_or()` (loses error handling semantics)
- Concatenate sequentially (makes handler unreachable)

---

## Solution Design

### Strategy

Enhance case #3 (lines 2296-2332) to generate proper match expressions instead of string replacement.

### Detection Logic

```rust
// In codegen_try_stmt, after line 2299
if let Some((exception_value_str, exception_type)) = simple_pattern_info {
    let try_code = quote! { #(#try_stmts)* };
    let try_str = try_code.to_string();

    // DEPYLER-0437: Detect .parse() calls that should use match
    if try_str.contains("parse") && exception_type.as_deref() == Some("ValueError") {
        // Generate match expression instead of unwrap_or
        return generate_parse_match_expression(body, handlers, finalbody, ctx);
    }

    // Fall through to existing string replacement logic
}
```

### Match Generation Logic

```rust
fn generate_parse_match_expression(
    body: &[HirStmt],
    handlers: &[ExceptHandler],
    finalbody: &Option<Vec<HirStmt>>,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // 1. Extract the int(value) call from try body
    //    Pattern: Assign { target: "num", value: Call { func: "int", args: [var] } }

    // 2. Generate parse expression
    let parse_expr = quote! { value.parse::<i32>() };

    // 3. Extract variable name (e.g., "num")
    let ok_var = syn::Ident::new("num", proc_macro2::Span::call_site());

    // 4. Generate Ok branch (rest of try body after the assignment)
    let ok_body = generate_try_body_continuation(body, ctx)?;

    // 5. Generate Err branch (handler body)
    let err_body = generate_handler_body(&handlers[0], ctx)?;

    // 6. Build match expression
    Ok(quote! {
        match #parse_expr {
            Ok(#ok_var) => { #ok_body },
            Err(_) => { #err_body }
        }
    })
}
```

### Helper Functions Needed

1. **`extract_parse_call(body: &[HirStmt]) -> Option<(String, HirExpr)>`**
   - Find the statement with `int(value)` call
   - Return: `(variable_name, argument_expr)`
   - Example: `("num", HirExpr::Var("value"))`

2. **`generate_try_body_continuation(body: &[HirStmt], start_index: usize) -> Result<TokenStream>`**
   - Generate code for statements AFTER the int() call
   - Handle if/raise/return statements in Ok branch

3. **`generate_handler_body(handler: &ExceptHandler) -> Result<TokenStream>`**
   - Generate code for except clause body
   - Already exists at line 2255-2262, reuse that

---

## Implementation Plan

### Phase 1: Add Helper Functions (30 minutes)

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`

**Location**: After `codegen_try_stmt` function (around line 2450)

```rust
/// DEPYLER-0437: Extract int() parse call from try body
fn extract_parse_call(body: &[HirStmt]) -> Option<(String, HirExpr, usize)> {
    for (idx, stmt) in body.iter().enumerate() {
        if let HirStmt::Assign { target: AssignTarget::Symbol(var), value, .. } = stmt {
            if let HirExpr::Call { func, args, .. } = value {
                if func == "int" && args.len() == 1 {
                    return Some((var.clone(), args[0].clone(), idx));
                }
            }
        }
    }
    None
}

/// DEPYLER-0437: Generate statements after parse call for Ok branch
fn generate_ok_branch_body(
    body: &[HirStmt],
    start_idx: usize,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let stmts: Vec<_> = body[start_idx + 1..]
        .iter()
        .map(|s| s.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! { #(#stmts)* })
}
```

### Phase 2: Modify codegen_try_stmt (1 hour)

**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`
**Location**: Lines 2296-2332 (the string replacement section)

**Change**:
```rust
// DEPYLER-0437: Generate proper match for ValueError + int() patterns
if let Some((exception_value_str, exception_type)) = simple_pattern_info {
    // Check if this is a ValueError handler with int() call
    if exception_type.as_deref() == Some("ValueError") {
        if let Some((var_name, parse_arg, assign_idx)) = extract_parse_call(body) {
            // Generate parse expression
            let arg_expr = parse_arg.to_rust_expr(ctx)?;
            let ok_var = safe_ident(&var_name);

            // Generate Ok branch (rest of try body)
            let ok_body = generate_ok_branch_body(body, assign_idx, ctx)?;

            // Generate Err branch (handler body)
            let err_body = &handler_tokens[0];

            // Build match expression
            return Ok(quote! {
                match #arg_expr.parse::<i32>() {
                    Ok(#ok_var) => { #ok_body },
                    Err(_) => { #err_body }
                }
            });
        }
    }

    // Fall through to existing unwrap_or logic if not a match pattern
    let try_code = quote! { #(#try_stmts)* };
    // ... rest of existing code
}
```

### Phase 3: Handle Multiple Statements in Try Block (1 hour)

**Test Case**:
```python
def port_validator(value):
    try:
        port = int(value)
        if port < 1 or port > 65535:  # ← Multiple statements!
            raise ValueError("bad port")
        return port
    except ValueError:
        return -1
```

**Expected**:
```rust
match value.parse::<i32>() {
    Ok(port) => {
        if (port < 1) || (port > 65535) {
            return Err(...);
        }
        port  // ← Last expression is the return value
    },
    Err(_) => -1
}
```

**Implementation**: The `generate_ok_branch_body` function already handles this via `body[start_idx + 1..]`.

### Phase 4: Testing (1 hour)

Run tests iteratively:
```bash
cargo test --test depyler_0437_try_except_control_flow

# Expected progression:
# Iteration 1: 1/5 passing → 2/5 passing (basic match works)
# Iteration 2: 2/5 passing → 4/5 passing (multiple statements work)
# Iteration 3: 4/5 passing → 5/5 passing (all edge cases covered)
```

Debug failures:
```bash
# If test fails, transpile the Python and inspect output
./target/release/depyler transpile /tmp/test_validator.py

# Check for:
# - Match expression present
# - Ok branch has correct variable name
# - Err branch has handler code
# - No unreachable code warnings
```

---

## Test Suite Status

**File**: `crates/depyler-core/tests/depyler_0437_try_except_control_flow.rs`

**Current Results**: 4/5 failing (RED phase complete ✅)

1. ❌ `test_DEPYLER_0437_try_except_generates_match`
   - **Checks**: Match expression with .parse() exists
   - **Current**: Uses unwrap_or_default()

2. ❌ `test_DEPYLER_0437_except_handler_in_err_branch`
   - **Checks**: Handler code (`return -1`) is in Err branch
   - **Current**: Handler missing entirely

3. ❌ `test_DEPYLER_0437_multiple_statements_in_try`
   - **Checks**: Validation logic in Ok branch after parse
   - **Current**: Sequential execution

4. ✅ `test_DEPYLER_0437_compiles_without_warnings`
   - **Checks**: No unreachable code warnings
   - **Current**: PASSING (but wrong semantics)

5. ❌ `test_DEPYLER_0437_nested_try_in_ok_branch`
   - **Checks**: Multiple operations in Ok branch
   - **Current**: No match structure

---

## Edge Cases to Handle

### 1. Exception Binding

```python
except ValueError as e:
    return str(e)
```

**Solution**: Already handled by lines 2360-2430 (existing code path)

### 2. Multiple Exception Types

```python
except (ValueError, TypeError):
    return None
```

**Solution**: Phase 2 implementation - check if exception_type contains multiple types

### 3. Bare Except

```python
except:
    return None
```

**Solution**: Fall through to existing concatenation logic (line 2431+)

### 4. Finally Clause

```python
try:
    ...
except ValueError:
    ...
finally:
    cleanup()
```

**Solution**: Wrap match expression in a block with finally after:
```rust
{
    let result = match ... { ... };
    cleanup();  // Finally code
    result
}
```

---

## Debugging Workflow

### If Tests Fail

1. **Enable --trace for transpiler decisions**:
   ```bash
   ./target/release/depyler transpile --trace test.py > output.rs 2>&1
   ```

2. **Check which code path was taken**:
   - Search output for "DEPYLER-0437" or "simple_pattern_info"
   - Check if `extract_parse_call` was called

3. **Inspect generated code**:
   ```bash
   rustc --crate-type lib output.rs 2>&1 | grep -E "(unreachable|error)"
   ```

4. **Use Renacer for runtime behavior** (if needed):
   ```bash
   renacer --trace-transpiler-decisions -- ./target/debug/depyler transpile test.py
   ```

---

## Success Criteria

- ✅ All 5 tests in `depyler_0437_try_except_control_flow.rs` passing
- ✅ complex_cli.py compiles without errors
- ✅ No unreachable code warnings
- ✅ Generated code uses match expressions for ValueError handlers
- ✅ No regressions in existing try/except tests

---

## Next Steps

1. Resume from GREEN phase implementation
2. Follow Implementation Plan phases 1-4
3. Run full test suite: `cargo test --workspace`
4. Verify complex_cli compiles: `depyler transpile examples/example_complex/complex_cli.py`
5. Commit GREEN phase
6. REFACTOR phase if needed
7. Move to DEPYLER-0438 (Custom Error Types)

---

## Related Tickets

- **Parent**: DEPYLER-0428 (ArgumentTypeError) - ✅ Complete
- **Sibling**: DEPYLER-0436 (Type Inference) - ✅ Complete
- **Sibling**: DEPYLER-0438 (Error Types) - Next
- **Master**: DEPYLER-0435 (100% Compilation)

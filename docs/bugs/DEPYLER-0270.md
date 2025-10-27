# DEPYLER-0270: Result Unwrapping at Call Sites

**Date**: 2025-10-27
**Status**: 🔴 RED PHASE (Tests created, awaiting GREEN phase implementation)
**Priority**: P1 - MEDIUM (Error handling pattern)
**Category**: Code Generation Bug - Type System (Result handling)
**Discovered**: Performance Benchmarking Campaign (compute_intensive.py validation)

---

## Summary

When Python functions return dict/list types and the transpiler generates `Result<T, E>` return types (due to potential indexing errors), call sites attempt to access methods directly on the `Result` without unwrapping, causing compilation errors.

**Expected behavior**: Call sites should unwrap Result types before accessing inner type methods
**Actual behavior**: Generated code tries to call methods on Result type directly

---

## Bug Evidence

### Python Source (lines 74-76)
```python
stats = calculate_statistics(fib_sequence)  # Returns dict[str, int]
print(f"Count: {stats['count']} | Max: {stats['max']}")
```

### Generated Rust (BROKEN)
```rust
// Function signature (line 47)
pub fn calculate_statistics<'a>(numbers: &'a Vec<i32>)
    -> Result<HashMap<String, i32>, IndexError> { ... }

// Call site (lines 109, 116-117)
let stats = calculate_statistics(fib_sequence);
// stats is Result<HashMap<String, i32>, IndexError>

println!("{}", format!("...",
    stats.get("count").cloned().unwrap_or_default(),  // ❌ ERROR
    stats.get("max").cloned().unwrap_or_default()     // ❌ ERROR
));
```

### Compilation Error
```
error[E0599]: no method named `get` found for enum `Result` in the current scope
   --> compute_intensive_transpiled.rs:116:15
    |
116 |             stats.get("count").cloned().unwrap_or_default(),
    |                   ^^^ method not found in `Result<HashMap<String, i32>, IndexError>`
```

### Expected Generated Code (CORRECT)
```rust
let stats = calculate_statistics(fib_sequence).unwrap();
// OR
let stats = calculate_statistics(fib_sequence)?;

println!("{}", format!("...",
    stats.get("count").cloned().unwrap_or_default(),  // ✅ OK
    stats.get("max").cloned().unwrap_or_default()     // ✅ OK
));
```

---

## Root Cause Analysis

### Where the Bug Occurs
**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs` (assignment statements)
**Logic**: When generating assignment statements for function call results

### Why It Happens
1. **Function Return Types**: Functions that perform indexing operations return `Result<T, IndexError>` for safety
2. **Call Site Generation**: Assignment statements generate direct variable binding without Result handling
3. **Method Access**: Subsequent code tries to call methods on `Result<T>` instead of unwrapped `T`

### Code Flow
1. Python: `stats = calculate_statistics(numbers)` - returns `dict[str, int]`
2. Transpiler generates function signature: `-> Result<HashMap<String, i32>, IndexError>`
3. Transpiler generates call site: `let stats = calculate_statistics(numbers);`
   - `stats` has type `Result<HashMap<String, i32>, IndexError>`
4. Transpiler generates method call: `stats.get("count")` - ERROR (Result doesn't have .get())

### Missing Logic
**Need**: Automatic Result unwrapping at call sites when function returns Result type

---

## Affected Code Patterns

### Pattern 1: Dict-Returning Functions with Indexing
```python
def process() -> dict[str, int]:
    data = {}
    data["key"] = values[0]  # Indexing triggers Result wrapper
    return data

result = process()
value = result["key"]  # ERROR: Result doesn't support indexing
```

### Pattern 2: List-Returning Functions with Indexing
```python
def get_list() -> list[int]:
    nums = [1, 2, 3]
    first = nums[0]  # Indexing triggers Result wrapper
    return nums

result = get_list()
length = len(result)  # ERROR: Result doesn't have len()
```

### Pattern 3: Chained Method Calls
```python
def get_data() -> dict[str, str]:
    return {"name": "value"}

name = get_data()["name"].upper()  # ERROR: Result doesn't support indexing
```

---

## Test Cases

### Test 1: Dict Access After Result-Returning Function
**Python**:
```python
def calculate_stats(numbers: list[int]) -> dict[str, int]:
    if not numbers:
        return {"count": 0}
    return {"count": len(numbers), "max": numbers[0]}

def main() -> None:
    data = [1, 2, 3]
    stats = calculate_stats(data)
    print(stats["count"])
```

**Expected Rust**:
```rust
let stats = calculate_stats(&data).unwrap();  // OR: ?
println!("{}", stats.get("count").cloned().unwrap_or_default());
```

### Test 2: Multiple Result Accesses
**Python**:
```python
def get_values() -> dict[str, int]:
    nums = [10, 20]
    return {"a": nums[0], "b": nums[1]}

def main() -> None:
    vals = get_values()
    x = vals["a"]
    y = vals["b"]
    print(x + y)
```

### Test 3: Nested Dict Access
**Python**:
```python
def get_nested() -> dict[str, dict[str, int]]:
    inner = {"value": 42}
    data = [inner]
    return {"inner": data[0]}

def main() -> None:
    result = get_nested()
    value = result["inner"]["value"]
    print(value)
```

---

## Implementation Strategy

### Option 1: Automatic .unwrap() (Simplest)
**Approach**: Always append `.unwrap()` when assigning Result-returning function calls

**Pros**:
- Simple to implement
- Matches Python's exception-on-error semantics
- Clear crash on error (panic)

**Cons**:
- Not idiomatic Rust (should prefer `?` in functions that return Result)
- Panics instead of propagating errors

**Implementation Location**: `stmt_gen.rs` - assignment statement generation

### Option 2: Automatic ? Propagation (Idiomatic)
**Approach**: Use `?` operator when containing function returns Result

**Pros**:
- Idiomatic Rust error propagation
- Composes with Rust error handling
- No panics

**Cons**:
- Requires tracking function return types
- Only works if containing function returns Result (not for main())

**Implementation Location**: `stmt_gen.rs` + context tracking for function signatures

### Option 3: Hybrid Approach (Recommended)
**Approach**:
- Use `?` if containing function returns Result/Option
- Use `.unwrap()` for functions returning `()` (like main())
- Add proper error handling in generated code

**Implementation**:
1. Check if current function returns Result/Option type
2. If yes: append `?` operator
3. If no: append `.unwrap()` for panic-on-error

---

## Implementation Plan (Hybrid Approach - RECOMMENDED)

### Phase 1: Detection
1. **Add Result detection helper** to `CodeGenContext`:
   ```rust
   pub fn is_result_type(ty: &RustType) -> bool {
       matches!(ty, RustType::Result(_, _))
   }
   ```

2. **Track current function return type** in context during function generation

### Phase 2: Assignment Statement Modification
**File**: `crates/depyler-core/src/rust_gen/stmt_gen.rs`
**Location**: Assignment statement generation (around line ~100-200)

**Logic**:
```rust
HirStmt::Assign { target, value, .. } => {
    // Generate RHS expression
    let rhs_expr = value.to_rust_expr(ctx)?;

    // Check if RHS is a function call that returns Result
    if let HirExpr::Call { func, .. } = value {
        if let Some(func_return_type) = ctx.get_function_return_type(func) {
            if CodeGenContext::is_result_type(&func_return_type) {
                // Need to unwrap Result
                rhs_expr = if ctx.current_function_returns_result() {
                    parse_quote! { #rhs_expr? }  // Propagate error
                } else {
                    parse_quote! { #rhs_expr.unwrap() }  // Panic on error
                };
            }
        }
    }

    // Generate assignment
    quote! { let #target = #rhs_expr; }
}
```

### Phase 3: Context Enhancement
**File**: `crates/depyler-core/src/rust_gen/context.rs`

**Add fields**:
```rust
pub struct CodeGenContext<'a> {
    // ... existing fields ...
    pub function_signatures: HashMap<String, RustType>,  // Store return types
    pub current_function_return_type: Option<RustType>,   // Track current function
}
```

**Add methods**:
```rust
impl<'a> CodeGenContext<'a> {
    pub fn register_function_signature(&mut self, name: String, return_type: RustType) {
        self.function_signatures.insert(name, return_type);
    }

    pub fn get_function_return_type(&self, name: &str) -> Option<&RustType> {
        self.function_signatures.get(name)
    }

    pub fn current_function_returns_result(&self) -> bool {
        self.current_function_return_type
            .as_ref()
            .map(Self::is_result_type)
            .unwrap_or(false)
    }
}
```

### Phase 4: Function Generation Integration
**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`

**On function generation**:
```rust
pub fn codegen_function(func: &HirFunction, ctx: &mut CodeGenContext) -> Result<TokenStream> {
    // Determine return type
    let return_type = determine_return_type(func);

    // Register in context
    ctx.register_function_signature(func.name.clone(), return_type.clone());
    ctx.current_function_return_type = Some(return_type.clone());

    // ... rest of function generation ...

    // Clear current function context
    ctx.current_function_return_type = None;
}
```

---

## Validation Criteria

### Compilation
✅ Generated code compiles with `rustc --deny warnings`
✅ No `method not found in Result` errors
✅ Proper error handling (? or .unwrap()) at all Result call sites

### Runtime Behavior
✅ Functions returning valid Results execute correctly
✅ Error cases either panic (.unwrap()) or propagate (?)
✅ No silent failures or incorrect type usage

### Test Coverage
✅ Dict-returning functions with Result wrapper
✅ List-returning functions with Result wrapper
✅ Multiple sequential Result unwraps
✅ Nested Result access patterns
✅ Regression test: non-Result functions still work

---

## Related Bugs

- **DEPYLER-0269**: Function parameter borrowing (reference vs value) - Separate issue
- **DEPYLER-0264**: DynamicType undefined - Fixed in v3.19.20
- **DEPYLER-0265**: Iterator dereferencing - Fixed in v3.19.20

---

## Complexity Estimate

**Cyclomatic Complexity Target**: ≤10 (A+ standard)
**Estimated Complexity**: 6-8 (medium complexity - context tracking + conditional logic)

**Breakdown**:
- Result type detection: Complexity 2-3
- Assignment statement modification: Complexity 3-4
- Context method addition: Complexity 1-2

**Total Lines Changed**: ~50-80 lines across 3 files

---

## Success Metrics

**Before Fix**:
- compute_intensive.py transpiles but fails compilation (2 errors on lines 116-117)
- Any dict/list-returning function with indexing causes Result access errors

**After Fix**:
- All Result-returning functions automatically unwrapped at call sites
- Compilation succeeds with proper error handling
- Zero `method not found in Result` errors
- Full test suite passes with zero regressions

---

**Ticket Created**: 2025-10-27
**Assigned To**: STOP THE LINE Campaign
**Next Step**: RED Phase - Create comprehensive failing tests

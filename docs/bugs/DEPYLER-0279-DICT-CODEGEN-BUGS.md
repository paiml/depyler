# DEPYLER-0279: Dictionary Codegen Bugs (Unused mut + Borrow After Move)

**Status**: FIXED ✅
**Priority**: P2 (Medium - blocks showcase 100% compilation)
**Discovered**: 2025-10-28
**Fixed**: 2025-10-28
**Root Cause**: Dictionary and loop codegen don't consider mutation and ownership patterns

## Issue

When Python code uses dictionaries with loops, the transpiler generates Rust code with two bugs:

1. **Unused `mut` warning**: Empty dict literal generates unnecessary `mut` modifier
2. **Borrow after move error**: Dict update in loop moves key, then tries to borrow it

### Example

**Python**:
```python
def count_words(text: str) -> Dict[str, int]:
    word_count = {}  # Bug 1: generates unnecessary mut
    words = text.split()
    for word in words:  # Bug 2: borrow after move in dict update
        if word in word_count:
            word_count[word] += 1
        else:
            word_count[word] = 1
    return word_count
```

**Generated Rust (BROKEN)**:
```rust
pub fn count_words(text: &str) -> Result<HashMap<String, i32>, IndexError> {
    let mut word_count = {
        let mut map = HashMap::new();  // BUG 1: unnecessary mut
        map
    };
    let words = text.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    for word in words.iter().cloned() {
        if word_count.contains_key(&word) {
            // BUG 2: `word` moved here ↓, then borrowed here ↓
            word_count.insert(word, word_count.get(&word).cloned().unwrap_or_default() + 1);
            //                ^^^^                 ^^^^^
            //                move                 borrow after move!
        } else {
            word_count.insert(word, 1);
        }
    }
    Ok(word_count)
}
```

**Compilation Errors**:
```
warning: variable does not need to be mutable
  --> test.rs:22:13
   |
22 |         let mut map = HashMap::new();
   |             ----^^^
   |             |
   |             help: remove this `mut`

error[E0382]: borrow of moved value: `word`
  --> test.rs:31:52
   |
29 |     for word in words.iter().cloned() {
   |         ---- move occurs because `word` has type `String`, which does not implement the `Copy` trait
30 |         if word_count.contains_key(&word) {
31 |             word_count.insert(word, word_count.get(&word).cloned().unwrap_or_default() + 1);
   |                               ----                 ^^^^^ value borrowed here after move
   |                               |
   |                               value moved here
```

## Root Cause Analysis

### Bug 1: Unnecessary `mut` in Empty Dict Literal

**Location**: `crates/depyler-core/src/rust_gen/expr_gen.rs:2499-2527`

**Function**: `convert_dict()`

**Problem**:
```rust
fn convert_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
    self.ctx.needs_hashmap = true;

    let mut insert_stmts = Vec::new();
    for (key, value) in items {
        // ... generate insert statements
        insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
    }

    // PROBLEM: Always generates `mut` even when items.is_empty()
    Ok(parse_quote! {
        {
            let mut map = HashMap::new();  // ← Always mut!
            #(#insert_stmts)*
            map
        }
    })
}
```

When `items` is empty (Python `{}`), this generates:
```rust
{
    let mut map = HashMap::new();  // Warning: unused mut
    map
}
```

**Fix**: Only add `mut` if there are items to insert:
```rust
let mutability = if items.is_empty() {
    quote! {}  // No mut
} else {
    quote! { mut }  // Add mut
};

Ok(parse_quote! {
    {
        let #mutability map = HashMap::new();
        #(#insert_stmts)*
        map
    }
})
```

### Bug 2: Borrow After Move in Dict Update

**Location**: `crates/depyler-core/src/rust_gen/stmt_gen.rs` (augmented assignment handling)

**Problem**: When transpiling `dict[key] += value`, the current logic generates:
```rust
dict.insert(key, dict.get(&key).cloned().unwrap_or_default() + value)
```

This works fine when `key` is `Copy` (like `i32`), but fails when `key` is `String`:
- `.insert(key, ...)` takes ownership of `key` (moves it)
- `.get(&key)` tries to borrow `key` - but it was already moved!

**Current Code Pattern**:
```rust
// In stmt_gen.rs (augmented assignment for dict)
HirStmt::AugAssign { target: HirExpr::Subscript { value, index }, op, .. } => {
    let dict_expr = value.to_rust_expr(ctx)?;
    let key_expr = index.to_rust_expr(ctx)?;

    // Problem: uses key_expr twice - once as move, once as borrow
    quote! {
        #dict_expr.insert(
            #key_expr,  // ← Moves key
            #dict_expr.get(&#key_expr).cloned().unwrap_or_default() + #value_expr
            //              ^^^^^^^^^ Borrow after move!
        )
    }
}
```

**Fix Options**:

**Option A**: Get value first, then insert with cloned key
```rust
quote! {
    {
        let key = #key_expr;
        let old_value = #dict_expr.get(&key).cloned().unwrap_or_default();
        #dict_expr.insert(key, old_value + #value_expr);
    }
}
```

**Option B**: Clone key for the borrow
```rust
quote! {
    #dict_expr.insert(
        #key_expr,
        #dict_expr.get(&#key_expr.clone()).cloned().unwrap_or_default() + #value_expr
    )
}
```

**Option C (Best)**: Use `entry()` API for efficiency
```rust
quote! {
    *#dict_expr.entry(#key_expr).or_insert(0) += #value_expr
}
```

The `entry()` API is idiomatic Rust and avoids double lookup.

## Impact

**Severity**: P2 (Medium)
- Blocks 100% showcase compilation success
- Affects common Python pattern (dict with loop updates)
- Causes compilation errors, not runtime bugs

**Scope**:
- Any code with empty dict literals (`{}`) - warning only
- Any code with dict updates in loops with non-Copy keys - compilation error

**Examples Affected**:
- `annotated_example.py` (count_words function)
- Any user code following similar patterns

## Solution Plan

### Phase 1: Fix Bug 1 (Unused mut) ✅

1. **Test (RED)**: Create test that checks for no unused mut warnings
2. **Implement (GREEN)**: Conditionally add `mut` based on `items.is_empty()`
3. **Verify (REFACTOR)**: Run clippy with `-D warnings` on test output

### Phase 2: Fix Bug 2 (Borrow after move) 🔄

1. **Test (RED)**: Create test with dict update in loop
2. **Implement (GREEN)**: Use `entry()` API or clone key appropriately
3. **Verify (REFACTOR)**: Compile annotated_example.rs successfully

### Phase 3: Validate 🔄

1. Re-transpile annotated_example.py
2. Verify compilation with zero errors and warnings
3. Run test suite - ensure no regressions
4. Update documentation

## Test Cases

### Test Case 1: Empty Dict

**Python**:
```python
def make_dict() -> Dict[str, int]:
    return {}
```

**Expected Rust** (after fix):
```rust
pub fn make_dict() -> Result<HashMap<String, i32>, IndexError> {
    Ok({
        let map = HashMap::new();  // No mut!
        map
    })
}
```

### Test Case 2: Dict with Initial Values

**Python**:
```python
def make_dict() -> Dict[str, int]:
    return {"a": 1, "b": 2}
```

**Expected Rust**:
```rust
pub fn make_dict() -> Result<HashMap<String, i32>, IndexError> {
    Ok({
        let mut map = HashMap::new();  // mut is necessary
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map
    })
}
```

### Test Case 3: Dict Update in Loop

**Python**:
```python
def count(items: List[str]) -> Dict[str, int]:
    counts = {}
    for item in items:
        if item in counts:
            counts[item] += 1
        else:
            counts[item] = 1
    return counts
```

**Expected Rust** (after fix):
```rust
pub fn count(items: &Vec<String>) -> Result<HashMap<String, i32>, IndexError> {
    let mut counts = {
        let map = HashMap::new();  // No mut - empty
        map
    };
    for item in items.iter().cloned() {
        *counts.entry(item).or_insert(0) += 1;  // Using entry() API
    }
    Ok(counts)
}
```

## Files Modified

- `crates/depyler-core/src/rust_gen/expr_gen.rs` (convert_dict)
- `crates/depyler-core/src/rust_gen/stmt_gen.rs` (augmented assignment for dicts)
- `examples/showcase/annotated_example.rs` (re-transpiled)
- Tests (TBD)

## Verification ✅

### Pre-Fix Status
- annotated_example.rs: 1 error, 1 warning ❌
- test_dict_loop.rs: 1 error, 1 warning ❌

### Post-Fix Status
- annotated_example.rs: 0 errors, 0 warnings ✅
- test_dict_loop.rs: 0 errors, 0 warnings ✅
- Test suite: No regressions (same 4 DEPYLER-0269 failures) ✅
- Showcase: annotated_example.rs now compiles cleanly ✅

### Implementation Details

**Bug 1 Fix** (expr_gen.rs:2523-2529):
```rust
// DEPYLER-0279: Only add `mut` if there are items to insert
if items.is_empty() {
    Ok(parse_quote! {
        {
            let map = HashMap::new();  // No mut!
            map
        }
    })
} else {
    Ok(parse_quote! {
        {
            let mut map = HashMap::new();  // mut only when needed
            #(#insert_stmts)*
            map
        }
    })
}
```

**Bug 2 Fix** (stmt_gen.rs:556-611):
```rust
// DEPYLER-0279: Detect dict augmented assignment pattern
if is_dict_augassign_pattern(target, value) {
    if let AssignTarget::Index { base, index } = target {
        if let HirExpr::Binary { op, left: _, right } = value {
            // Generate code that evaluates old value first
            return Ok(quote! {
                {
                    let _key = #index_expr;
                    let _old_val = #base_expr.get(&_key).cloned().unwrap_or_default();
                    #base_expr.insert(_key, _old_val #op_token #right_expr);
                }
            });
        }
    }
}
```

### Generated Code Comparison

**Before** (broken):
```rust
// Bug 1: Unnecessary mut
let mut word_count = {
    let mut map = HashMap::new();  // ← Unused mut warning!
    map
};
// Bug 2: Borrow after move
for word in words.iter().cloned() {
    if word_count.contains_key(&word) {
        word_count.insert(word, word_count.get(&word)...);  // ← Error!
        //                ^^^^                 ^^^^^
        //                move                 borrow after move!
    }
}
```

**After** (fixed):
```rust
// Bug 1: No mut for empty dict
let mut word_count = {
    let map = HashMap::new();  // ← No unused mut!
    map
};
// Bug 2: Evaluate old value first
for word in words.iter().cloned() {
    if word_count.contains_key(&word) {
        {
            let _key = word;                                    // ← Bind key
            let _old_val = word_count.get(&_key)...;           // ← Get old value
            word_count.insert(_key, _old_val + 1);            // ← Insert with owned key
        }
    }
}
```

## Related Issues

- DEPYLER-0269: Test generation bugs (separate issue)
- Showcase validation campaign (v3.19.x)

## Extreme TDD Cycle

- **RED**: annotated_example.rs fails compilation ✅
- **GREEN**: Fix dict codegen (in progress)
- **REFACTOR**: Verify all showcase examples compile

## Future Enhancements

1. **Smart mutation analysis**: Track if dict is actually mutated
2. **Optimize dict operations**: Use more efficient Rust patterns
3. **Better ownership inference**: Reduce unnecessary clones

---

**Generated**: 2025-10-28
**Status**: IN PROGRESS 🔄
**Assigned**: Claude Code
**Estimated Time**: 2-3 hours

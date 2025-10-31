# DEPYLER-0318: Function Parameter Mutability Inference

**Date Created**: 2025-10-31
**Status**: 📋 ANALYSIS - Ready for Implementation  
**Priority**: P2 - Medium (affects common dict/list operations)
**Estimate**: 3-4 hours
**Related**: DEPYLER-0304 (discovered during 09_dictionary_operations validation)

## Problem Statement

Transpiler generates `&T` function parameters when `&mut T` is required, causing compilation errors when mutating methods are called.

**Discovery Context**: Found during DEPYLER-0304 campaign when validating 09_dictionary_operations Matrix example.

## Examples

### Example 1: dict.pop() Requires Mutable Borrow

**Python**:
```python
def pop_entry(d: dict[str, int], key: str) -> int:
    """Remove and return value."""
    return d.pop(key, -1)  # Mutates dictionary
```

**Generated Rust** (WRONG):
```rust
pub fn pop_entry<'b, 'a>(d: &'a HashMap<String, i32>, key: &'b str) -> i32 {
    //                         ^-- Should be &'a mut HashMap
    d.remove(key).unwrap_or(-1)  // ❌ ERROR: .remove() requires &mut
}
```

**Error**:
```
error[E0596]: cannot borrow `*d` as mutable, as it is behind a `&` reference
  --> lib.rs:69:5
   |
69 |     d.remove(key).unwrap_or(-1)
   |     ^ `d` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

**Correct Rust**:
```rust
pub fn pop_entry<'b, 'a>(d: &'a mut HashMap<String, i32>, key: &'b str) -> i32 {
    //                             +++-- Add mut
    d.remove(key).unwrap_or(-1)  // ✅ Now works
}
```

### Example 2: dict.pop() in Conditional

**Python**:
```python
def pop_entry_no_default(d: dict[str, int], key: str) -> int:
    if key in d:
        return d.pop(key)  # Mutates dictionary
    return -1
```

**Generated Rust** (WRONG):
```rust
pub fn pop_entry_no_default<'b, 'a>(d: &'a HashMap<String, i32>, key: &'b str) -> i32 {
    let _cse_temp_0 = d.contains_key(key);
    if _cse_temp_0 {
        return d.remove(key).expect("KeyError: key not found");  // ❌ ERROR
    }
    -1
}
```

**Correct Rust**:
```rust
pub fn pop_entry_no_default<'b, 'a>(d: &'a mut HashMap<String, i32>, key: &'b str) -> i32 {
    //                                        +++
    let _cse_temp_0 = d.contains_key(key);
    if _cse_temp_0 {
        return d.remove(key).expect("KeyError: key not found");  // ✅ Works
    }
    -1
}
```

## Root Cause Analysis

### Current Behavior
Transpiler infers function parameter types based on Python type hints:
- `d: dict[str, int]` → `d: &HashMap<String, i32>` (always immutable)

### Missing Logic
Transpiler should analyze function body for **mutating operations** and infer `&mut` when needed:
- `.pop()` → `.remove()` (requires `&mut`)
- `.clear()` (requires `&mut`)
- `.insert()` (requires `&mut`)
- `.extend()` (requires `&mut`)
- `list.append()`, `list.remove()`, etc.

## Implementation Strategy

### Phase 1: Mutation Detection (2 hours)

**Location**: `crates/depyler-core/src/hir_gen/` or `crates/depyler-core/src/rust_gen/func_gen.rs`

**Approach**:
1. Traverse function body HIR
2. Track method calls on each parameter
3. Build map: `parameter_name → requires_mut: bool`

**Pseudocode**:
```rust
fn analyze_parameter_mutability(func: &HirFunction) -> HashMap<String, bool> {
    let mut mutability_map = HashMap::new();
    
    for param in &func.params {
        let requires_mut = body_mutates_parameter(&func.body, &param.name);
        mutability_map.insert(param.name.clone(), requires_mut);
    }
    
    mutability_map
}

fn body_mutates_parameter(body: &HirExpr, param_name: &str) -> bool {
    // Visit all method calls in body
    match body {
        HirExpr::MethodCall { object, method, .. } => {
            if let HirExpr::Var(var_name) = object {
                if var_name == param_name && is_mutating_method(method) {
                    return true;
                }
            }
        }
        // Recursively check all sub-expressions
        _ => { /* traverse children */ }
    }
    false
}

fn is_mutating_method(method: &str) -> bool {
    matches!(method, 
        "pop" | "remove" | "insert" | "clear" | "extend" | "append" | 
        "update" | "sort" | "reverse" | "push" | "pop_front" | ...
    )
}
```

### Phase 2: Signature Generation (1-2 hours)

**Location**: `crates/depyler-core/src/rust_gen/func_gen.rs` (parameter type generation)

**Approach**:
1. Use mutability map from Phase 1
2. When generating parameter type, check if `&mut` is required
3. Generate `&mut T` instead of `&T`

**Code Change** (approximate location):
```rust
// In func_gen.rs, parameter type generation
fn generate_param_type(param: &HirParam, mutability_map: &HashMap<String, bool>) -> syn::Type {
    let base_type = convert_type(&param.type_annotation);
    
    // Check if this parameter needs mutable borrow
    let needs_mut = mutability_map.get(&param.name).copied().unwrap_or(false);
    
    if needs_mut {
        parse_quote! { &mut #base_type }  // ✅ Generate &mut
    } else {
        parse_quote! { &#base_type }      // Immutable borrow
    }
}
```

## Affected Operations

### HashMap/Dict Operations
- ✅ `.pop(key)` → `.remove(key)` (requires `&mut`)
- ✅ `.clear()` (requires `&mut`)
- ✅ `.update(other)` → `for (k,v) in other { .insert(k,v) }` (requires `&mut`)
- ❓ `.insert(key, value)` (requires `&mut`) - check subscript assignment

### List/Vec Operations  
- ✅ `.append(item)` → `.push(item)` (requires `&mut`)
- ✅ `.remove(item)` (requires `&mut`)
- ✅ `.pop()` (requires `&mut`)
- ✅ `.extend(items)` (requires `&mut`)
- ✅ `.sort()` (requires `&mut`)
- ✅ `.reverse()` (requires `&mut`)

### Set Operations
- ✅ `.add(item)` → `.insert(item)` (requires `&mut`)
- ✅ `.remove(item)` (requires `&mut`)
- ✅ `.pop()` (requires `&mut`)

## Testing Strategy

### Test Cases

**Test 1**: Dict pop with default
```python
def test_dict_pop_default(d: dict[str, int], key: str) -> int:
    return d.pop(key, 0)  # Should generate &mut HashMap
```

**Test 2**: List append
```python
def test_list_append(items: list[int], value: int) -> list[int]:
    items.append(value)  # Should generate &mut Vec
    return items
```

**Test 3**: Mixed mutable/immutable
```python
def test_mixed(d1: dict[str, int], d2: dict[str, int]) -> int:
    d1.pop("key")  # d1 needs &mut
    return len(d2)  # d2 can be &
```

### Success Criteria

✅ **All test cases compile** without E0596 errors
✅ **Generated signatures match manual Rust code**
✅ **Zero regressions** in existing tests (455/458 pass rate maintained)
✅ **09_dictionary_operations** compiles with 2 fewer errors (from 4 to 2)

## Implementation Checklist

- [ ] Phase 1: Implement mutation detection visitor
- [ ] Phase 1: Build parameter mutability map
- [ ] Phase 1: Test mutation detection with unit tests
- [ ] Phase 2: Integrate mutability map into func_gen.rs
- [ ] Phase 2: Generate `&mut` parameters when required
- [ ] Phase 2: Verify generated code compiles
- [ ] Testing: Add regression tests for dict.pop(), list.append(), etc.
- [ ] Testing: Run full test suite (ensure 455/458 pass rate)
- [ ] Docs: Update CHANGELOG.md
- [ ] Commit: Create detailed commit message

## Related Issues

- **DEPYLER-0304**: HashMap Type Inference (parent campaign)
- **DEPYLER-0319**: Numeric Type Conversion (related, discovered same time)
- **Future**: May need similar analysis for `mut` local variables

## Priority Justification

**P2 - Medium** because:
- ✅ Affects common operations (dict.pop(), list.append())
- ✅ Blocks 09_dictionary_operations from compiling
- ⚠️ Workaround exists (manual signature fixing)
- ⚠️ Not blocking critical path (Matrix Project can continue)

## Estimated Impact

**After Fix**:
- 09_dictionary_operations: 4 errors → 2 errors (50% reduction)
- Enables correct transpilation of all mutating methods
- Improves generated code quality (more idiomatic Rust)

---
**Status**: Ready for implementation
**Next Step**: Implement Phase 1 (mutation detection visitor)

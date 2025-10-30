# DEPYLER-0309-0313: Matrix Project 07_algorithms Bug Analysis

**Date**: 2025-10-30
**Status**: Analysis Complete - Ready for Implementation
**Example**: python-to-rust-conversion-examples/examples/07_algorithms
**Errors**: 16 compilation errors across 5 distinct bug patterns

## Executive Summary

Matrix Project validation of 07_algorithms revealed 16 compilation errors across 5 bug patterns. All are transpiler code generation issues, not fundamental language limitations. Estimated total fix time: 6-8 hours across 5 tickets.

**Quick Win Potential**: Tickets DEPYLER-0309 (2h) and DEPYLER-0312 (1h) are independent quick wins that will fix 3/16 errors (19%).

---

## Error Distribution

| Error Type | Count | Ticket | Priority | Estimate |
|------------|-------|--------|----------|----------|
| Box<dyn Error> wrapper missing | 8 | DEPYLER-0310 | P1 | 2-3h |
| Vec slice concatenation | 2 | DEPYLER-0311 | P1 | 2h |
| HashSet.contains_key() → contains() | 1 | DEPYLER-0309 | P1 | 1-2h |
| Function parameter mutability | 2 | DEPYLER-0312 | P1 | 1h |
| .abs() type ambiguity | 1 | DEPYLER-0313 | P2 | 30min |
| Misc type mismatches | 2 | - | P2 | 1h |

---

## DEPYLER-0309: Track set() Constructor for Type Inference ⚡ Quick Win

**Priority**: P1 - Quick Win (1-2 hours)
**Impact**: 1/16 errors (6%), enables HashSet method inference
**Complexity**: Low (parallel to existing class tracking)

### Problem

`set()` constructor calls aren't tracked in `var_types`, causing HashSet variables to fall through to HashMap method dispatch:

```python
# Python
def remove_duplicates(items: list[int]) -> list[int]:
    seen = set()  # Creates HashSet
    result = []
    for item in items:
        if item not in seen:  # BinOp::NotIn
            seen.add(item)
            result.append(item)
    return result
```

```rust
// Generated Rust (WRONG)
pub fn remove_duplicates(items: &Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut result = vec![];
    for item in items.iter().cloned() {
        if !seen.contains_key(item) {  // ❌ HashSet doesn't have contains_key()!
            //        ^^^^^^^^^^^^^^^^
            seen.insert(item);
            result.push(item);
        }
    }
    result
}
```

**Error**:
```
error[E0599]: no method named `contains_key` found for struct `HashSet` in the current scope
   --> /tmp/07_test.rs:181:20
    |
181 |         if !seen.contains_key(item) {
    |                    ^^^^^^^^^^^^ method not found in `HashSet<i32>`
```

### Root Cause

In `stmt_gen.rs:codegen_assign_stmt()`, we track class constructors:

```rust
match value {
    HirExpr::Call { func, .. } => {
        // Check if this is a user-defined class constructor
        if ctx.class_names.contains(func) {
            ctx.var_types.insert(var_name.clone(), Type::Custom(func.clone()));
        }
    }
    // ...
}
```

But we DON'T track builtin constructors like `set()`, `dict()`, `list()`.

### Solution

**Part 1**: Track `set()` constructor in `stmt_gen.rs` (similar to DEPYLER-0232):

```rust
match value {
    HirExpr::Call { func, .. } => {
        // Track user-defined class constructors
        if ctx.class_names.contains(func) {
            ctx.var_types.insert(var_name.clone(), Type::Custom(func.clone()));
        }
        // DEPYLER-0309: Track builtin collection constructors
        else if func == "set" {
            // Infer element type from type annotation or default to Unknown
            let elem_type = if let Some(Type::Set(elem)) = type_annotation {
                elem.as_ref().clone()
            } else {
                Type::Int  // Default for untyped sets
            };
            ctx.var_types.insert(var_name.clone(), Type::Set(Box::new(elem_type)));
        }
        else if func == "dict" {
            // Similar logic for dict()
            ctx.var_types.insert(var_name.clone(), Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)));
        }
    }
    // ... existing Set/FrozenSet literal tracking (DEPYLER-0224)
}
```

**Part 2**: Existing code already handles sets correctly once tracked:

```rust
// expr_gen.rs:123-147 (already works!)
BinOp::In => {
    let is_set = self.is_set_expr(right) || self.is_set_var(right);
    if is_set {
        Ok(parse_quote! { #right_expr.contains(&#left_expr) })  // ✅ Correct!
    } else {
        Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })  // HashMap
    }
}
```

### Testing

```bash
# After fix
$ cargo run --bin depyler -- transpile .../07_algorithms/column_a/column_a.py --output /tmp/test.rs
$ rustc --crate-type lib /tmp/test.rs 2>&1 | grep contains_key
# Should be empty (no more contains_key errors)
```

### Files Modified

1. `crates/depyler-core/src/rust_gen/stmt_gen.rs` - Add `set()` tracking in `codegen_assign_stmt`
2. Tests - Add regression test for `set()` constructor tracking

### Verification

- Core tests: 453/453 must pass (zero regressions)
- Matrix 07_algorithms: 16 errors → 15 errors
- Pattern: All `set()` assignments now tracked

---

## DEPYLER-0310: Box::new() Wrapper for Mixed Error Types

**Priority**: P1 - High Impact (2-3 hours)
**Impact**: 8/16 errors (50%), critical for error handling patterns
**Complexity**: Medium (requires error type analysis)

### Problem

Functions returning `Result<T, Box<dyn Error>>` (mixed error types) generate `Err(ValueError::new(...))` without `Box::new()` wrapper:

```rust
// Generated (WRONG)
pub fn find_min(items: &Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    if items.len() == 0 {
        return Err(ValueError::new("Cannot find min of empty list".to_string()));
        //     ^^^ Expected Box<dyn Error>, found ValueError
    }
    // ...
}
```

**Error**:
```
error[E0308]: mismatched types
   --> /tmp/07_test.rs:103:20
    |
103 |         return Err(ValueError::new("Cannot find min of empty list".to_string()));
    |                --- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn Error>`, found `ValueError`
```

### Root Cause

In `stmt_gen.rs:codegen_raise_stmt()`, we generate `Err(exc_expr)` without checking if the function's error type is `Box<dyn Error>`:

```rust
pub(crate) fn codegen_raise_stmt(...) -> Result<TokenStream> {
    if let Some(exc) = exception {
        let exc_expr = exc.to_rust_expr(ctx)?;
        Ok(quote! { return Err(#exc_expr); })  // ❌ No Box::new() wrapper!
    }
    // ...
}
```

### Solution

**Part 1**: Track if current function uses `Box<dyn Error>` in `func_gen.rs`:

```rust
// Add to CodeGenContext
pub struct CodeGenContext<'a> {
    // ...
    pub current_error_type: Option<ErrorType>,  // DEPYLER-0310
}

pub enum ErrorType {
    Concrete(Type),           // Result<T, ValueError>
    DynBox,                   // Result<T, Box<dyn Error>>
}
```

**Part 2**: Set error type when entering function:

```rust
impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        // DEPYLER-0310: Detect if function uses Box<dyn Error>
        ctx.current_error_type = if self.properties.can_fail {
            // Check if function has multiple exception types or generic errors
            if self.has_mixed_exception_types() {
                Some(ErrorType::DynBox)
            } else {
                Some(ErrorType::Concrete(/* infer specific error type */))
            }
        } else {
            None
        };
        // ... rest of function generation
    }
}
```

**Part 3**: Wrap with `Box::new()` when needed:

```rust
pub(crate) fn codegen_raise_stmt(...) -> Result<TokenStream> {
    if let Some(exc) = exception {
        let exc_expr = exc.to_rust_expr(ctx)?;

        // DEPYLER-0310: Wrap with Box::new() if function uses Box<dyn Error>
        let err_expr = if matches!(ctx.current_error_type, Some(ErrorType::DynBox)) {
            parse_quote! { Box::new(#exc_expr) }
        } else {
            exc_expr
        };

        Ok(quote! { return Err(#err_expr); })
    }
    // ...
}
```

### Testing

- Core tests: 453/453 must pass
- Matrix 07_algorithms: 16 errors → 8 errors (-50%)
- Pattern: All `raise ValueError` in `Box<dyn Error>` functions now compile

---

## DEPYLER-0311: Vec Slice Concatenation

**Priority**: P1 - Common Pattern (2 hours)
**Impact**: 2/16 errors (13%), list rotation algorithms
**Complexity**: Medium (expression-level analysis)

### Problem

Slice concatenation `items[k:] + items[:k]` generates invalid `Vec + Vec`:

```python
# Python
def rotate_left(items: list[int], k: int) -> list[int]:
    if len(items) == 0:
        return []
    k = k % len(items)
    return items[k:] + items[:k]  # Slice concatenation
```

```rust
// Generated (WRONG)
pub fn rotate_left(items: &Vec<i32>, mut k: i32) -> Result<Vec<i32>, ZeroDivisionError> {
    if items.len() as i32 == 0 {
        return Ok(vec![]);
    }
    k = k % items.len() as i32;
    Ok({
        let base = items;
        let start = (k).max(0) as usize;
        if start < base.len() {
            base[start..].to_vec()
        } else {
            Vec::new()
        }
    } + {  // ❌ Cannot add Vec<i32> to Vec<i32>!
        let base = items;
        let stop = (k).max(0) as usize;
        base[..stop.min(base.len())].to_vec()
    })
}
```

**Error**:
```
error[E0369]: cannot add `Vec<i32>` to `Vec<i32>`
   --> /tmp/07_test.rs:145:7
```

### Root Cause

DEPYLER-0290 fixed `list1 + list2` for variables, but didn't handle:
1. Slice expression results (`.to_vec()` creates owned Vec)
2. Complex expressions (not simple variables)

### Solution

**Extend DEPYLER-0290** in `expr_gen.rs:BinOp::Add`:

```rust
BinOp::Add => {
    // DEPYLER-0290: Vec + Vec should use .extend() pattern
    let both_vecs = self.is_vec_expr(left) && self.is_vec_expr(right);

    // DEPYLER-0311: Also handle slice concatenation
    let is_slice_concat = matches!(left, HirExpr::Slice { .. })
                       && matches!(right, HirExpr::Slice { .. });

    if both_vecs || is_slice_concat {
        // Generate chaining pattern for slice concatenation
        let left_expr = left.to_rust_expr(self.ctx)?;
        let right_expr = right.to_rust_expr(self.ctx)?;
        Ok(parse_quote! {
            {
                let mut _temp = #left_expr;
                _temp.extend(#right_expr);
                _temp
            }
        })
    } else {
        // Regular numeric addition
        Ok(parse_quote! { #left_expr + #right_expr })
    }
}
```

### Testing

- Core tests: 453/453 must pass
- Matrix 07_algorithms: 8 errors → 6 errors
- Pattern: `items[k:] + items[:k]` now compiles correctly

---

## DEPYLER-0312: Function Parameter Mutability Detection ⚡ Quick Win

**Priority**: P1 - Quick Win (1 hour)
**Impact**: 2/16 errors (13%), swap/exchange patterns
**Complexity**: Low (extend existing analysis)

### Problem

Function parameters that are reassigned aren't marked `mut`:

```python
# Python
def gcd(a: int, b: int) -> int:
    """Calculate GCD using Euclidean algorithm."""
    while b != 0:
        temp = b
        b = a % b  # Reassigns parameter b
        a = temp   # Reassigns parameter a
    return a
```

```rust
// Generated (WRONG)
pub fn gcd(a: i32, b: i32) -> Result<i32, ZeroDivisionError> {
//        ^       ^
//        Should be `mut a`, `mut b`
    while b != 0 {
        let temp = b;
        b = a % b;  // ❌ cannot assign to immutable argument `b`
        a = temp;   // ❌ cannot assign to immutable argument `a`
    }
    Ok(a)
}
```

**Error**:
```
error[E0384]: cannot assign to immutable argument `a`
   --> /tmp/07_test.rs:229:9
error[E0384]: cannot assign to immutable argument `b`
   --> /tmp/07_test.rs:228:9
```

### Root Cause

`analyze_mutable_vars()` in `rust_gen.rs` only tracks local variables, not function parameters.

### Solution

**Extend existing analysis** in `rust_gen.rs:analyze_mutable_vars()`:

```rust
fn analyze_mutable_vars(stmts: &[HirStmt], ctx: &mut CodeGenContext, params: &[HirParam]) {
    let mut declared = HashSet::new();

    // DEPYLER-0312: Pre-populate declared with function parameters
    for param in params {
        declared.insert(param.name.clone());
    }

    // ... existing analysis (now detects parameter reassignments)
}
```

**Call site** in `func_gen.rs`:

```rust
impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        // DEPYLER-0312: Pass params to mutability analysis
        analyze_mutable_vars(&self.body, ctx, &self.params);

        // Generate parameters with mut if needed
        let params: Vec<syn::FnArg> = self.params.iter().map(|p| {
            let mut_token = if ctx.mutable_vars.contains(&p.name) {
                Some(quote! { mut })
            } else {
                None
            };
            // ...
        }).collect();
        // ...
    }
}
```

### Testing

- Core tests: 453/453 must pass
- Matrix 07_algorithms: 6 errors → 4 errors
- Pattern: All parameter reassignments now compile

---

## DEPYLER-0313: Type Annotations for Ambiguous Numeric Operations

**Priority**: P2 - Edge Case (30 minutes)
**Impact**: 1/16 errors (6%), range step calculations
**Complexity**: Trivial (add type annotation)

### Problem

`(-1).abs()` without type annotation creates ambiguity:

```rust
// Generated (WRONG)
for i in {
    let step = (-1).abs() as usize;  // ❌ Ambiguous numeric type {integer}
    if step == 0 {
        panic!("range() arg 3 must not be zero");
    }
    // ...
}
```

**Error**:
```
error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`
   --> /tmp/07_test.rs:111:22
```

### Solution

**Add type annotation** in range generation:

```rust
// In range step calculation
let step = (-1i32).abs() as usize;  // ✅ Explicit i32 type
```

### Testing

- Core tests: 453/453 must pass
- Matrix 07_algorithms: 4 errors → 3 errors
- Pattern: All range steps compile

---

## Implementation Plan

### Phase 1: Quick Wins (3 hours total, 3/16 errors fixed)
1. **DEPYLER-0309**: set() constructor tracking (1-2h) → 1 error fixed
2. **DEPYLER-0312**: Parameter mutability (1h) → 2 errors fixed

### Phase 2: High Impact (5 hours total, 10/16 errors fixed)
3. **DEPYLER-0310**: Box::new() wrapper (2-3h) → 8 errors fixed
4. **DEPYLER-0311**: Vec slice concatenation (2h) → 2 errors fixed

### Phase 3: Polish (30 minutes, 1/16 errors fixed)
5. **DEPYLER-0313**: Type annotations (30min) → 1 error fixed

**Total**: 8.5 hours estimated, 14/16 errors fixed (88%)
**Remaining**: 2 misc type mismatches (likely related to above fixes)

### Success Criteria

- ✅ Matrix 07_algorithms: 16 errors → 0 errors
- ✅ Pass rate: 56% → 67% (+11% improvement)
- ✅ Core tests: 453/453 pass (zero regressions)
- ✅ All quality gates pass

---

## Next Session Recommendations

**Start with**: DEPYLER-0309 (set() tracking) - 1-2 hour quick win that unblocks pattern recognition

**Rationale**:
- Independent of other fixes
- Low complexity (parallel to existing code)
- Immediate visible impact
- Builds momentum

**Follow with**: DEPYLER-0312 (parameter mutability) - another quick win

**Then tackle**: DEPYLER-0310 (Box wrapper) - highest impact (50% of errors)


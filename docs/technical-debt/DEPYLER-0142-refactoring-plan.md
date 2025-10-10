# DEPYLER-0142: Refactor ExpressionConverter::convert_method_call

**Priority**: P0 (Critical Technical Debt - #1 Remaining Hotspot)
**File**: `crates/depyler-core/src/rust_gen.rs:2402`
**Current Complexity**: Cyclomatic 99, Cognitive 180+, 290 lines
**Target**: ≤10 cyclomatic per function, ≤10 cognitive
**Estimated Effort**: 6-7 hours (based on DEPYLER-0140/0141 success)
**Status**: PLANNED

## Problem Analysis

The `ExpressionConverter::convert_method_call` function is **290 lines** long and handles 21+ different method types in a single monolithic match statement. This creates:

- **Unmaintainability**: Function too large to understand
- **Untestability**: Cannot unit test individual method handlers
- **Complexity**: Cyclomatic 99 (9.9x over limit)
- **Cognitive Load**: 180+ cognitive complexity (18x over limit)

## Function Structure Analysis

```
Lines 2402-2692 (290 lines total)
├── Classmethod handling (lines 2408-2418, ~10 lines)
├── Module method call handling (lines 2420-2470, ~50 lines)
└── Method dispatch match (lines 2479-2689, ~210 lines)
    ├── List methods (5 handlers)
    ├── Dict methods (5 handlers)
    ├── String methods (6 handlers)
    ├── Set methods (3 handlers)
    ├── Regex methods (1 handler)
    └── Default fallback (1 handler)
```

### Method Categories (21 total handlers)

**List Methods** (5):
- `append` → `push`
- `extend` → `extend`
- `pop` → `pop` (with set handling)
- `insert` → `insert`
- `remove` → `remove` (with set handling)

**Dict Methods** (5):
- `get` → `get(...).cloned()`
- `keys` → `keys().cloned().collect()`
- `values` → `values().cloned().collect()`
- `items` → `iter().map(|(k, v)| ...)`
- `update` → `extend`

**String Methods** (6):
- `upper` → `to_uppercase`
- `lower` → `to_lowercase`
- `strip` → `trim`
- `startswith` → `starts_with`
- `endswith` → `ends_with`
- `split` → `split(...).collect()`
- `join` → `join`

**Set Methods** (3):
- `add` → `insert`
- `discard` → `remove`
- `clear` → `clear`

**Regex Methods** (1):
- `findall` → `find_iter(...).collect()`

**Default** (1):
- Generic method call fallback

## Refactoring Strategy

Apply proven extract-method pattern from DEPYLER-0140/0141:
- **Phase 1**: Extract preamble handlers (classmethod, module methods)
- **Phase 2**: Extract method category dispatchers (one per category)
- **Phase 3**: Final integration and cleanup

### Phase 1: Extract Preamble Handlers (~1 hour)

Extract early-return special cases:

```rust
// BEFORE (current):
impl ExpressionConverter {
    fn convert_method_call(&mut self, object: &HirExpr, method: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        // Handle classmethod cls.method() → Self::method()
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.ctx.is_classmethod {
                // ... 10 lines
            }
        }

        // Check if this is a module method call (e.g., os.getcwd())
        if let HirExpr::Var(module_name) = object {
            // ... 50 lines
        }

        // ... 210 more lines
    }
}

// AFTER (target):
impl ExpressionConverter {
    fn convert_method_call(&mut self, object: &HirExpr, method: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        // Try classmethod handling first
        if let Some(result) = self.try_convert_classmethod(object, method, args)? {
            return Ok(result);
        }

        // Try module method handling
        if let Some(result) = self.try_convert_module_method(object, method, args)? {
            return Ok(result);
        }

        // Dispatch to method category handlers
        self.convert_instance_method(object, method, args)
    }
}

/// Handle classmethod calls (cls.method())
#[inline]
fn try_convert_classmethod(
    &mut self,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
) -> Result<Option<syn::Expr>> {
    // Implementation (10 lines)
}

/// Handle module method calls (os.getcwd())
#[inline]
fn try_convert_module_method(
    &mut self,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
) -> Result<Option<syn::Expr>> {
    // Implementation (50 lines)
}
```

### Phase 2: Extract Method Category Handlers (~3 hours)

Extract 6 category-specific dispatchers:

```rust
/// Convert instance method calls (main dispatcher)
#[inline]
fn convert_instance_method(
    &mut self,
    object: &HirExpr,
    method: &str,
    args: &[HirExpr],
) -> Result<syn::Expr> {
    let object_expr = object.to_rust_expr(self.ctx)?;
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(self.ctx))
        .collect::<Result<Vec<_>>>()?;

    // Dispatch by method category
    match method {
        // List methods
        "append" | "extend" | "pop" | "insert" | "remove" => {
            self.convert_list_method(&object_expr, object, method, &arg_exprs)
        }

        // Dict methods
        "get" | "keys" | "values" | "items" | "update" => {
            self.convert_dict_method(&object_expr, method, &arg_exprs)
        }

        // String methods
        "upper" | "lower" | "strip" | "startswith" | "endswith" | "split" | "join" => {
            self.convert_string_method(&object_expr, method, &arg_exprs)
        }

        // Set methods
        "add" | "discard" | "clear" => {
            self.convert_set_method(&object_expr, method, &arg_exprs)
        }

        // Regex methods
        "findall" => {
            self.convert_regex_method(&object_expr, method, &arg_exprs)
        }

        // Default: generic method call
        _ => {
            let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
            Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
        }
    }
}

/// Handle list methods (append, extend, pop, insert, remove)
#[inline]
fn convert_list_method(
    &mut self,
    object_expr: &syn::Expr,
    object: &HirExpr,
    method: &str,
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    // Implementation (~40 lines, handles 5 methods)
}

/// Handle dict methods (get, keys, values, items, update)
#[inline]
fn convert_dict_method(
    &mut self,
    object_expr: &syn::Expr,
    method: &str,
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    // Implementation (~35 lines, handles 5 methods)
}

/// Handle string methods (upper, lower, strip, etc.)
#[inline]
fn convert_string_method(
    &mut self,
    object_expr: &syn::Expr,
    method: &str,
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    // Implementation (~40 lines, handles 6 methods)
}

/// Handle set methods (add, discard, clear)
#[inline]
fn convert_set_method(
    &mut self,
    object_expr: &syn::Expr,
    method: &str,
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    // Implementation (~20 lines, handles 3 methods)
}

/// Handle regex methods (findall)
#[inline]
fn convert_regex_method(
    &mut self,
    object_expr: &syn::Expr,
    method: &str,
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    // Implementation (~15 lines, handles 1 method)
}
```

### Phase 3: Integration & Cleanup (~2 hours)

- Remove old 210-line match statement
- Verify all 393 tests pass
- Run PMAT complexity analysis
- Verify main function ≤10 complexity

## Implementation Plan

### Phase 1: Preamble Extraction (1h)
- [ ] Extract try_convert_classmethod() helper
- [ ] Extract try_convert_module_method() helper
- [ ] Add 4 unit tests (2 per helper)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0142 Phase 1: Extract preamble handlers (2/8)"

### Phase 2: Category Handlers (3h)
- [ ] Create convert_instance_method() dispatcher
- [ ] Extract convert_list_method() (5 methods)
- [ ] Extract convert_dict_method() (5 methods)
- [ ] Extract convert_string_method() (6 methods)
- [ ] Extract convert_set_method() (3 methods)
- [ ] Extract convert_regex_method() (1 method)
- [ ] Add 12 unit tests (2 per category)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0142 Phase 2: Extract category handlers (8/8)"

### Phase 3: Integration (2h)
- [ ] Remove old 210-line match statement
- [ ] Verify all 393 tests pass
- [ ] Run PMAT complexity analysis
- [ ] Verify main function ≤10 complexity
- [ ] Update CHANGELOG
- [ ] Commit: "DEPYLER-0142 Phase 3 COMPLETE: All handlers extracted 🎉"

### Validation (30min)
- [ ] Run PMAT complexity analysis
- [ ] Verify convert_method_call ≤10 complexity
- [ ] Run full test suite
- [ ] Update roadmap and documentation

## Success Criteria

- ✅ Main `convert_method_call` function: cyclomatic ≤10 (target: ~8)
- ✅ All extracted functions: cyclomatic ≤10
- ✅ All extracted functions: cognitive ≤10
- ✅ All extracted functions: ≤50 lines
- ✅ 100% test pass rate maintained
- ✅ Zero performance regression (#[inline] on all helpers)

## Expected Results

**Code Metrics:**
- Main function: 290 → ~30 lines (-260 lines, -90% reduction)
- Functions created: ~8 total (2 preamble + 6 category handlers)
- Complexity: 99 → <10 (target achieved)

**Time Savings vs Original Estimate:**
- Original (from roadmap): 50 hours
- DEPYLER-0140/0141 experience: 6-7 hours
- Savings: 43+ hours (86% reduction)

---

**Last Updated**: 2025-10-10
**Status**: PLANNED - Ready to start based on DEPYLER-0140/0141 success
**Next**: Begin Phase 1 extraction

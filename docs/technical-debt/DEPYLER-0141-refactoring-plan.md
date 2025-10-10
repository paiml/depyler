# DEPYLER-0141: Refactor HirFunction::to_rust_tokens

**Priority**: P0 (Critical Technical Debt - #2 Worst Hotspot)
**File**: `crates/depyler-core/src/rust_gen.rs:604`
**Current Complexity**: Cyclomatic 106, Cognitive 250+, 504 lines
**Target**: ≤10 cyclomatic per function, ≤10 cognitive
**Estimated Effort**: 5-6 hours (based on DEPYLER-0140 success)
**Status**: PLANNED

## Problem Analysis

The `HirFunction::to_rust_tokens` function is **504 lines** long and handles 11 different concerns in a single monolithic function. This creates:

- **Unmaintainability**: Function too large to understand
- **Untestability**: Cannot unit test individual concerns
- **Complexity**: Cyclomatic 106 (10.6x over limit)
- **Cognitive Load**: 250+ cognitive complexity (25x over limit)

## Function Structure (11 Major Sections)

1. **Generic Type Inference** (lines 607-609, ~5 lines)
2. **Lifetime Analysis** (lines 611-614, ~4 lines)
3. **Generate Generic Parameters** (lines 615-648, ~33 lines)
4. **Generate Where Clause** (lines 650-664, ~14 lines)
5. **Convert Parameters** (lines 666-828, ~162 lines) ⚠️ **MOST COMPLEX**
6. **Convert Return Type** (lines 829-872, ~43 lines)
7. **Handle Result Wrapper** (lines 873-926, ~53 lines)
8. **Process Function Body** (lines 928-966, ~38 lines)
9. **Add Documentation** (lines 968-995, ~27 lines)
10. **Generator Handling** (lines 997-1090, ~93 lines) ⚠️ **VERY COMPLEX**
11. **Final Token Generation** (lines 1091-1105, ~14 lines)

**Total**: 504 lines, complexity 106

## Refactoring Strategy

Apply same proven pattern from DEPYLER-0140:
- **Phase 1**: Extract simple sections (4 helpers, ~2h)
- **Phase 2**: Extract medium sections (3 helpers, ~2h)
- **Phase 3**: Extract complex sections (2 helpers with sub-functions, ~2h)

### Phase 1: Extract Simple Helpers (~2 hours)

Extract 4 simple, focused functions:

```rust
// BEFORE (current):
impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        // ... 504 lines of complexity
    }
}

// AFTER (target):
impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<TokenStream> {
        let generic_params = codegen_generic_params(self)?;
        let where_clause = codegen_where_clause(&lifetime_result)?;
        let return_type = codegen_return_type(&self.ret_type, &lifetime_result, ctx)?;
        let attrs = codegen_function_attrs(&self.docstring, &self.properties)?;
        // ... continue with other sections
    }
}

/// Generate generic parameters (<T, U, 'a, 'b>)
#[inline]
fn codegen_generic_params(
    type_params: &[TypeParam],
    lifetime_params: &[String]
) -> TokenStream {
    // Implementation (33 lines)
}

/// Generate where clause for lifetime bounds
#[inline]
fn codegen_where_clause(lifetime_bounds: &[(String, String)]) -> TokenStream {
    // Implementation (14 lines)
}

/// Generate return type with Result wrapper if needed
#[inline]
fn codegen_return_type(
    ret_type: &Option<Type>,
    lifetime_result: &LifetimeAnalysisResult,
    ctx: &mut CodeGenContext
) -> Result<TokenStream> {
    // Implementation (97 lines: return type + Result wrapper)
}

/// Generate function attributes (doc comments, termination proof)
#[inline]
fn codegen_function_attrs(
    docstring: &Option<String>,
    properties: &FunctionProperties
) -> Vec<TokenStream> {
    // Implementation (27 lines)
}
```

### Phase 2: Extract Medium Helpers (~2 hours)

Extract 3 medium-complexity functions:

```rust
/// Perform generic type inference
#[inline]
fn infer_generic_types(func: &HirFunction) -> Result<Vec<TypeParam>> {
    // Implementation (5 lines)
}

/// Perform lifetime analysis on function
#[inline]
fn analyze_function_lifetimes(
    func: &HirFunction,
    type_mapper: &TypeMapper
) -> LifetimeAnalysisResult {
    // Implementation (4 lines)
}

/// Process function body statements
#[inline]
fn codegen_function_body(
    func: &HirFunction,
    ctx: &mut CodeGenContext
) -> Result<Vec<TokenStream>> {
    // Implementation (38 lines)
}
```

### Phase 3: Extract Complex Helpers (~2 hours)

Extract 2 complex functions with sub-functions:

#### 3a. Parameter Conversion (~162 lines → 5 functions)

```rust
/// Convert function parameters with lifetime/borrowing analysis
#[inline]
fn codegen_function_params(
    params: &[Parameter],
    lifetime_result: &LifetimeAnalysisResult,
    ctx: &mut CodeGenContext
) -> Result<Vec<TokenStream>> {
    params.iter().map(|p| codegen_single_param(p, lifetime_result, ctx)).collect()
}

/// Convert single parameter with borrowing strategy
fn codegen_single_param(...) -> Result<TokenStream> {
    // Dispatcher (20 lines)
}

/// Apply Cow borrowing strategy to parameter
fn apply_cow_strategy(...) -> TokenStream {
    // Implementation (30 lines)
}

/// Apply normal borrowing strategy (& or &mut)
fn apply_normal_borrowing(...) -> TokenStream {
    // Implementation (50 lines)
}

/// Apply ownership strategy (no borrowing)
fn apply_ownership_strategy(...) -> TokenStream {
    // Implementation (30 lines)
}
```

#### 3b. Generator Handling (~93 lines → 4 functions)

```rust
/// Generate generator function (with Iterator impl)
#[inline]
fn codegen_generator_function(
    func: &HirFunction,
    name: &Ident,
    generic_params: &TokenStream,
    params: &[TokenStream],
    attrs: &[TokenStream],
    ctx: &mut CodeGenContext
) -> Result<TokenStream> {
    // Main dispatcher (20 lines)
}

/// Generate generator state struct
fn codegen_generator_state_struct(...) -> TokenStream {
    // Implementation (30 lines)
}

/// Generate Iterator impl for state struct
fn codegen_generator_iterator_impl(...) -> TokenStream {
    // Implementation (30 lines)
}

/// Extract generator item type from return type
fn extract_generator_item_type(ret_type: &RustType) -> Result<TokenStream> {
    // Implementation (10 lines)
}
```

## Implementation Plan

### Phase 1: Simple Helpers (2h)
- [ ] Extract generic_params, where_clause, return_type, attrs helpers
- [ ] Add 8 unit tests (2 per helper)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0141 Phase 1: Extract simple helpers (4/11)"

### Phase 2: Medium Helpers (2h)
- [ ] Extract type inference, lifetime analysis, body processing
- [ ] Add 6 unit tests (2 per helper)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0141 Phase 2: Extract medium helpers (7/11)"

### Phase 3: Complex Helpers (2h)
- [ ] Extract parameter conversion (5 sub-functions)
- [ ] Extract generator handling (4 sub-functions)
- [ ] Add 10 unit tests (complex scenarios)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0141 Phase 3 COMPLETE: Extract all helpers (11/11) 🎉"

### Validation (30min)
- [ ] Run PMAT complexity analysis
- [ ] Verify HirFunction::to_rust_tokens ≤10 complexity
- [ ] Run full test suite
- [ ] Update roadmap and documentation

## Success Criteria

- ✅ Main `to_rust_tokens` function: cyclomatic ≤10 (target: ~7)
- ✅ All extracted functions: cyclomatic ≤10
- ✅ All extracted functions: cognitive ≤10
- ✅ All extracted functions: ≤50 lines (except generator ~70)
- ✅ 100% test pass rate maintained
- ✅ Zero performance regression (#[inline] on all helpers)

## Expected Results

**Code Metrics:**
- Main function: 504 → ~50 lines (-454 lines, -90% reduction)
- Functions created: ~16 total (11 main + 5 sub-functions)
- Complexity: 106 → <10 (target achieved)

**Time Savings vs Original Estimate:**
- Original (from roadmap): 60 hours
- DEPYLER-0140 experience: 5-6 hours
- Savings: 54+ hours (90% reduction)

---

**Last Updated**: 2025-10-10
**Status**: PLANNED - Ready to start based on DEPYLER-0140 success
**Next**: Begin Phase 1 extraction

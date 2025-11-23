# DEPYLER-0477: Varargs Parameter Support Analysis

**Status**: 🔍 ANALYSIS COMPLETE - Phase 2 Architecture Required
**Date**: 2025-11-23
**Complexity**: HIGH - Requires HIR changes + AST bridge + codegen
**Impact**: Fixes 3 errors in example_environment, enables broader Python compatibility

## Problem Statement

Python functions with varargs parameters (`*args`) are not transpiled correctly:

```python
def join_paths(*parts):
    result = os.path.join(*parts)
    return result
```

**Current Transpilation** (broken):
```rust
pub fn join_paths() -> String {  // ❌ Missing parameter!
    let result = parts.join(...);  // ❌ parts not found (E0425)
}
```

**Required Transpilation**:
```rust
pub fn join_paths(parts: Vec<String>) -> String {
    let result = parts.join(std::path::MAIN_SEPARATOR_STR);
}
```

## Root Cause Analysis

### AST Bridge Limitation

**File**: `crates/depyler-core/src/ast_bridge.rs`
**Function**: `convert_parameters()` (lines 1287-1365)

**Current Implementation**:
```rust
fn convert_parameters(args: &ast::Arguments) -> Result<Vec<HirParam>> {
    let mut params = Vec::new();

    // ONLY processes regular parameters
    for (i, arg) in args.args.iter().enumerate() {
        // ... process regular parameter ...
    }

    // ❌ COMPLETELY IGNORES:
    // - args.vararg   (for *args)
    // - args.kwarg    (for **kwargs)

    Ok(params)
}
```

**Python AST Structure** (from rustpython_parser):
```rust
pub struct Arguments {
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,              // Regular parameters (processed ✅)
    pub vararg: Option<Box<Arg>>,    // *args (IGNORED ❌)
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<Option<Expr>>,
    pub kwarg: Option<Box<Arg>>,     // **kwargs (IGNORED ❌)
    pub defaults: Vec<Expr>,
}
```

### HIR Limitation

**File**: `crates/depyler-core/src/hir.rs`
**Struct**: `HirParam` (lines 231-236)

**Current Structure**:
```rust
pub struct HirParam {
    pub name: Symbol,
    pub ty: Type,
    pub default: Option<HirExpr>,
    // ❌ No vararg/kwarg flag!
}
```

**Missing Capability**:
- No way to mark parameter as varargs
- No way to represent `*args` vs `args` distinction

### Code Generation Gap

**File**: `crates/depyler-core/src/rust_gen/func_gen.rs`
**Function**: `codegen_single_param()`

**Current Behavior**:
- Assumes all parameters are regular (non-vararg)
- Generates parameter as-is from type
- No special handling for `Vec<T>` varargs parameters

## Architectural Requirements

### Option 1: Extend HirParam (Recommended)

**Pros**: Minimal changes, clear semantics
**Cons**: Adds complexity to existing struct

```rust
pub struct HirParam {
    pub name: Symbol,
    pub ty: Type,
    pub default: Option<HirExpr>,
    pub is_vararg: bool,    // *args
    pub is_kwarg: bool,     // **kwargs
}
```

**Changes Required**:
1. Update `HirParam` struct definition
2. Update `convert_parameters()` to set flags
3. Update `codegen_single_param()` to check flags
4. Update all `HirParam` construction sites

### Option 2: Separate VarArgs Type

**Pros**: Clearer type safety, explicit modeling
**Cons**: More invasive changes, complex refactoring

```rust
pub enum HirParameter {
    Regular(HirParam),
    VarArgs { name: Symbol, element_type: Type },
    KwArgs { name: Symbol, value_type: Type },
}

pub struct HirFunction {
    pub params: Vec<HirParameter>,  // Changed from Vec<HirParam>
    // ...
}
```

**Changes Required**:
1. Create new `HirParameter` enum
2. Update `HirFunction.params` type
3. Update ALL pattern matches on `params`
4. Update ALL code generation logic

### Option 3: Hybrid Approach (Phased)

**Phase 2.1**: Add `is_vararg` flag (Option 1)
**Phase 2.2**: Refactor to enum (Option 2) if needed

## Implementation Plan (Phased Approach)

### Phase 2.1: Basic Varargs Support (Week 1-2)

**Goal**: `*args` → `Vec<String>` for simple cases

#### Step 1: HIR Extension (2 hours)
```rust
// crates/depyler-core/src/hir.rs
pub struct HirParam {
    pub name: Symbol,
    pub ty: Type,
    pub default: Option<HirExpr>,
    pub is_vararg: bool,    // DEPYLER-0477: Support *args
}
```

**Changes**:
- Update all `HirParam { name, ty, default }` → add `is_vararg: false`
- Estimate: ~20 construction sites across codebase

#### Step 2: AST Bridge Update (3 hours)
```rust
// crates/depyler-core/src/ast_bridge.rs
fn convert_parameters(args: &ast::Arguments) -> Result<Vec<HirParam>> {
    let mut params = Vec::new();

    // Process regular parameters
    for arg in args.args.iter() {
        params.push(HirParam {
            name: arg.def.arg.to_string(),
            ty: infer_type(arg),
            default: extract_default(arg),
            is_vararg: false,  // Regular param
        });
    }

    // DEPYLER-0477: Process varargs (*args)
    if let Some(vararg) = &args.vararg {
        params.push(HirParam {
            name: vararg.def.arg.to_string(),
            ty: Type::Vec(Box::new(Type::String)),  // Start with Vec<String>
            default: None,
            is_vararg: true,
        });
    }

    // TODO Phase 2.2: Process kwargs (**kwargs)

    Ok(params)
}
```

#### Step 3: Code Generation Update (2 hours)
```rust
// crates/depyler-core/src/rust_gen/func_gen.rs
fn codegen_single_param(...) -> Result<proc_macro2::TokenStream> {
    // ... existing code ...

    // DEPYLER-0477: Handle varargs parameters
    if param.is_vararg {
        // Varargs always take ownership (not borrowed)
        let param_ident = safe_ident(&param.name);
        let elem_type = if let Type::Vec(elem) = &param.ty {
            rust_type_to_syn(elem)?
        } else {
            return Err(anyhow!("Varargs parameter must have Vec<T> type"));
        };

        return Ok(quote! { #param_ident: Vec<#elem_type> });
    }

    // ... existing regular parameter logic ...
}
```

#### Step 4: Call Site Handling (4 hours)
**Pattern**: `join_paths(*args.parts)` → Need to detect and handle

**Challenges**:
- Detect varargs expansion (`*expr`) in call arguments
- Generate `.clone()` or move based on ownership analysis
- Handle both `*variable` and `*list_literal` patterns

**Initial Implementation** (simple case):
```rust
// For: join_paths(*args.parts)
// Generate: join_paths(args.parts.clone())
```

#### Step 5: Testing (2 hours)
- Unit test: `*args` parameter detection
- Unit test: Vec<String> parameter generation
- Integration test: example_environment.join_paths compiles
- Property test: Various varargs patterns

**Estimated Total**: 13 hours (2 days)

### Phase 2.2: Advanced Varargs (Week 3-4)

#### Features:
1. Type inference for varargs elements (not just String)
2. `**kwargs` support → `HashMap<String, T>`
3. Varargs expansion optimization (move vs clone)
4. Variadic function calls (passing varargs to varargs)

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_varargs_parameter_detection() {
    let python = "def foo(*args): pass";
    let hir = parse_to_hir(python);
    assert!(hir.functions[0].params[0].is_vararg);
}

#[test]
fn test_varargs_codegen() {
    let python = "def foo(*args): return len(args)";
    let rust = transpile(python);
    assert!(rust.contains("args: Vec<String>"));
}
```

### Integration Tests
```rust
#[test]
fn test_example_environment_join_paths() {
    transpile_and_compile("examples/example_environment/env_info.py");
    // Should reduce errors from 16 → 13 (3 *parts errors fixed)
}
```

## Impact Analysis

### Examples Fixed (Partial)
- **example_environment**: 16 → 13 errors (3 *parts errors fixed)
  - Remaining: Subcommand field extraction (2 errors), type conversions (11 errors)

### Examples Enabled (Future)
- Any example using `*args` parameters
- Enables Python-to-Rust transpilation of:
  - Variadic utility functions
  - Wrapper functions
  - Argument forwarding patterns

### Broader Impact
- **Python Compatibility**: Major step toward full Python parameter support
- **Standard Library**: Enables transpiling functions like `os.path.join(*parts)`
- **User Code**: Common pattern in Python (variadic helpers, wrappers)

## Risks & Mitigations

### Risk 1: Type Inference Complexity
**Issue**: Determining element type `T` in `Vec<T>` for varargs
**Mitigation**: Start with `Vec<String>` for Phase 2.1, add inference in Phase 2.2

### Risk 2: Call Site Complexity
**Issue**: Detecting and handling `*expr` varargs expansion
**Mitigation**: Simple `.clone()` for Phase 2.1, optimize in Phase 2.2

### Risk 3: Ownership Analysis
**Issue**: Deciding when to move vs clone varargs
**Mitigation**: Conservative approach (always clone), optimize later

### Risk 4: HIR Refactoring Scope
**Issue**: Updating all `HirParam` construction sites
**Mitigation**: Use compiler to find all sites, systematic updates

## Alternative Approaches Considered

### Alt 1: Macro-based Varargs
**Idea**: Use Rust macros for varargs functions
**Rejected**: Complex, non-idiomatic, limits type checking

### Alt 2: Tuple Parameters
**Idea**: `*args` → `(T, T, T, ...)`
**Rejected**: Fixed arity, doesn't match Python semantics

### Alt 3: Skip Varargs Functions
**Idea**: Don't transpile functions with `*args`
**Rejected**: Blocks too many common Python patterns

## Decision Required

**Question**: Which HIR approach to use?
- Option 1 (extend HirParam) - Faster, less invasive
- Option 2 (separate enum) - Cleaner, more future-proof
- Option 3 (phased) - Balanced, allows iteration

**Recommendation**: **Option 1 (extend HirParam)** for Phase 2.1
- Fastest path to working varargs
- Can refactor to Option 2 later if needed
- Enables example_environment progress

## Next Steps

1. **Design Review**: Confirm HIR approach (extend HirParam vs new enum)
2. **Implement Phase 2.1**: Basic varargs support (~13 hours)
3. **Test**: Verify example_environment error reduction
4. **Document**: Update single-shot-compile roadmap
5. **Iterate**: Plan Phase 2.2 based on learnings

---

**Analysis Complete - Ready for Phase 2 Implementation**

# DEPYLER-0143: Refactor rust_type_to_syn_type

**Priority**: P0 (Critical Technical Debt - #3 Remaining Hotspot)
**File**: `crates/depyler-core/src/direct_rules.rs:761`
**Current Complexity**: Cyclomatic 73, Cognitive 120+, 123 lines
**Target**: ≤10 cyclomatic per function, ≤10 cognitive
**Estimated Effort**: 4-5 hours (based on DEPYLER-0140/0141/0142 success)
**Status**: PLANNED

## Problem Analysis

The `rust_type_to_syn_type` function is **123 lines** long and handles 18+ different RustType variants in a single monolithic match statement. This creates:

- **Unmaintainability**: Function too large to understand
- **Untestability**: Cannot unit test individual type converters
- **Complexity**: Cyclomatic 73 (7.3x over limit)
- **Cognitive Load**: 120+ cognitive complexity (12x over limit)

## Function Structure Analysis

```
Lines 761-884 (123 lines total)
├── Unit handling (1 line)
├── Primitive handling (lines 765-784, ~19 lines) - nested match with 14 types
├── String handling (1 line)
├── Str handling (lines 786-794, ~8 lines) - with lifetime
├── Cow handling (lines 795-799, ~4 lines) - with lifetime
├── Vec handling (lines 800-803, ~3 lines) - recursive
├── HashMap handling (lines 804-808, ~4 lines) - recursive
├── Option handling (lines 809-812, ~3 lines) - recursive
├── Result handling (lines 813-817, ~4 lines) - recursive
├── Tuple handling (lines 818-823, ~5 lines) - recursive
├── Custom handling (lines 824-827, ~3 lines)
├── Unsupported handling (lines 828-835, ~7 lines)
├── TypeParam handling (lines 836-839, ~3 lines)
├── Generic handling (lines 840-846, ~6 lines) - recursive
├── Enum handling (lines 847-850, ~3 lines)
├── Reference handling (lines 851-858, ~7 lines) - recursive with mutability
├── Array handling (lines 859-878, ~19 lines) - complex const generic handling
└── HashSet handling (lines 879-882, ~3 lines) - recursive
```

### Type Categories (18 total handlers)

**Simple Types** (5):
- `Unit` → `()`
- `String` → `String`
- `Custom(name)` → `#ident`
- `TypeParam(name)` → `#ident`
- `Enum{name, ..}` → `#ident`

**Primitive Types** (1 nested match with 14 variants):
- `Primitive(prim_type)` → match 14 primitive types (bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64)

**Lifetime Types** (2):
- `Str{lifetime}` → `&'a str` or `&str`
- `Cow{lifetime}` → `std::borrow::Cow<'a, str>`

**Simple Recursive Types** (5):
- `Vec(inner)` → `Vec<T>`
- `HashMap(key, value)` → `HashMap<K, V>`
- `Option(inner)` → `Option<T>`
- `Result(ok, err)` → `Result<T, E>`
- `HashSet(inner)` → `HashSet<T>`

**Complex Recursive Types** (3):
- `Tuple(types)` → `(T1, T2, ...)`
- `Generic{base, params}` → `Base<T1, T2, ...>`
- `Reference{inner, mutable}` → `&T` or `&mut T`

**Complex Types** (2):
- `Array{element_type, size}` → `[T; N]` with 3 const generic sub-cases (Literal, Parameter, Expression)
- `Unsupported(name)` → placeholder type with name munging

## Refactoring Strategy

Apply proven extract-method pattern from DEPYLER-0140/0141/0142:
- **Phase 1**: Extract simple and primitive type handlers (~2 hours)
- **Phase 2**: Extract recursive type handlers (~2 hours)

### Phase 1: Extract Simple Type Handlers (~2 hours)

Extract handlers for non-recursive types:

```rust
// BEFORE (current):
fn rust_type_to_syn_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Unit => parse_quote! { () },
        Primitive(prim_type) => {
            // ... 19 lines of primitive matching
        },
        String => parse_quote! { String },
        // ... 100 more lines
    })
}

// AFTER (target):
fn rust_type_to_syn_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        // Simple types
        Unit | String | Custom(_) | TypeParam(_) | Enum { .. } => {
            convert_simple_type(rust_type)?
        }

        // Primitive types
        Primitive(prim_type) => convert_primitive_type(prim_type)?,

        // Lifetime types
        Str { .. } | Cow { .. } => convert_lifetime_type(rust_type)?,

        // Recursive types
        Vec(_) | HashMap(_, _) | Option(_) | Result(_, _) | HashSet(_) => {
            convert_container_type(rust_type)?
        }

        // Complex types
        Tuple(_) | Generic { .. } | Reference { .. } => {
            convert_complex_type(rust_type)?
        }

        // Array with const generics
        Array { .. } => convert_array_type(rust_type)?,

        // Special case
        Unsupported(_) => convert_unsupported_type(rust_type)?,
    })
}

/// Convert simple non-recursive types (Unit, String, Custom, TypeParam, Enum)
#[inline]
fn convert_simple_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Unit => parse_quote! { () },
        String => parse_quote! { String },
        Custom(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        TypeParam(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        Enum { name, .. } => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        _ => unreachable!("convert_simple_type called with non-simple type"),
    })
}

/// Convert primitive types (bool, integers, floats)
#[inline]
fn convert_primitive_type(prim_type: &crate::type_mapper::PrimitiveType) -> Result<syn::Type> {
    use crate::type_mapper::PrimitiveType;
    Ok(match prim_type {
        PrimitiveType::Bool => parse_quote! { bool },
        PrimitiveType::I8 => parse_quote! { i8 },
        PrimitiveType::I16 => parse_quote! { i16 },
        PrimitiveType::I32 => parse_quote! { i32 },
        PrimitiveType::I64 => parse_quote! { i64 },
        PrimitiveType::I128 => parse_quote! { i128 },
        PrimitiveType::ISize => parse_quote! { isize },
        PrimitiveType::U8 => parse_quote! { u8 },
        PrimitiveType::U16 => parse_quote! { u16 },
        PrimitiveType::U32 => parse_quote! { u32 },
        PrimitiveType::U64 => parse_quote! { u64 },
        PrimitiveType::U128 => parse_quote! { u128 },
        PrimitiveType::USize => parse_quote! { usize },
        PrimitiveType::F32 => parse_quote! { f32 },
        PrimitiveType::F64 => parse_quote! { f64 },
    })
}

/// Convert lifetime-parameterized types (Str, Cow)
#[inline]
fn convert_lifetime_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Str { lifetime } => {
            if let Some(lt) = lifetime {
                let lifetime_token =
                    syn::Lifetime::new(&format!("'{}", lt), proc_macro2::Span::call_site());
                parse_quote! { &#lifetime_token str }
            } else {
                parse_quote! { &str }
            }
        }
        Cow { lifetime } => {
            let lifetime_token =
                syn::Lifetime::new(&format!("'{}", lifetime), proc_macro2::Span::call_site());
            parse_quote! { std::borrow::Cow<#lifetime_token, str> }
        }
        _ => unreachable!("convert_lifetime_type called with non-lifetime type"),
    })
}

/// Convert unsupported types with placeholder names
#[inline]
fn convert_unsupported_type(name: &str) -> Result<syn::Type> {
    let ident = syn::Ident::new(
        &format!("UnsupportedType_{}", name.replace(" ", "_")),
        proc_macro2::Span::call_site(),
    );
    Ok(parse_quote! { #ident })
}
```

### Phase 2: Extract Recursive Type Handlers (~2 hours)

Extract handlers for recursive types:

```rust
/// Convert container types (Vec, HashMap, Option, Result, HashSet)
#[inline]
fn convert_container_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Vec(inner) => {
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { Vec<#inner_type> }
        }
        HashMap(key, value) => {
            let key_type = rust_type_to_syn_type(key)?;
            let value_type = rust_type_to_syn_type(value)?;
            parse_quote! { HashMap<#key_type, #value_type> }
        }
        Option(inner) => {
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { Option<#inner_type> }
        }
        Result(ok, err) => {
            let ok_type = rust_type_to_syn_type(ok)?;
            let err_type = rust_type_to_syn_type(err)?;
            parse_quote! { Result<#ok_type, #err_type> }
        }
        HashSet(inner) => {
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { HashSet<#inner_type> }
        }
        _ => unreachable!("convert_container_type called with non-container type"),
    })
}

/// Convert complex recursive types (Tuple, Generic, Reference)
#[inline]
fn convert_complex_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Tuple(types) => {
            let type_tokens: Result<Vec<_>> =
                types.iter().map(rust_type_to_syn_type).collect();
            let type_tokens = type_tokens?;
            parse_quote! { (#(#type_tokens),*) }
        }
        Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: Result<Vec<_>> =
                params.iter().map(rust_type_to_syn_type).collect();
            let param_types = param_types?;
            parse_quote! { #base_ident<#(#param_types),*> }
        }
        Reference { inner, mutable, .. } => {
            let inner_type = rust_type_to_syn_type(inner)?;
            if *mutable {
                parse_quote! { &mut #inner_type }
            } else {
                parse_quote! { &#inner_type }
            }
        }
        _ => unreachable!("convert_complex_type called with non-complex type"),
    })
}

/// Convert array types with const generic handling
#[inline]
fn convert_array_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    if let Array { element_type, size } = rust_type {
        let element = rust_type_to_syn_type(element_type)?;
        Ok(match size {
            crate::type_mapper::RustConstGeneric::Literal(n) => {
                let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                parse_quote! { [#element; #size_lit] }
            }
            crate::type_mapper::RustConstGeneric::Parameter(name) => {
                let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                parse_quote! { [#element; #param_ident] }
            }
            crate::type_mapper::RustConstGeneric::Expression(expr) => {
                let expr_tokens: proc_macro2::TokenStream = expr
                    .parse()
                    .unwrap_or_else(|_| "/* invalid const expression */".parse().unwrap());
                parse_quote! { [#element; #expr_tokens] }
            }
        })
    } else {
        unreachable!("convert_array_type called with non-array type")
    }
}
```

## Implementation Plan

### Phase 1: Simple Type Handlers (2h)
- [ ] Extract convert_simple_type() helper (5 types)
- [ ] Extract convert_primitive_type() helper (14 types)
- [ ] Extract convert_lifetime_type() helper (2 types)
- [ ] Extract convert_unsupported_type() helper (1 type)
- [ ] Add 8 unit tests (2 per helper)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0143 Phase 1: Extract simple type handlers (4/8)"

### Phase 2: Recursive Type Handlers (2h)
- [ ] Extract convert_container_type() helper (5 types)
- [ ] Extract convert_complex_type() helper (3 types)
- [ ] Extract convert_array_type() helper (1 type with 3 sub-cases)
- [ ] Update main function to dispatcher pattern
- [ ] Add 6 unit tests (2 per category)
- [ ] Verify all existing tests pass
- [ ] Commit: "DEPYLER-0143 Phase 2 COMPLETE: Extract all type handlers (8/8) 🎉"

### Validation (30min)
- [ ] Run PMAT complexity analysis
- [ ] Verify rust_type_to_syn_type ≤10 complexity
- [ ] Run full test suite
- [ ] Update roadmap and documentation

## Success Criteria

- ✅ Main `rust_type_to_syn_type` function: cyclomatic ≤10 (target: ~8)
- ✅ All extracted functions: cyclomatic ≤10
- ✅ All extracted functions: cognitive ≤10
- ✅ All extracted functions: ≤30 lines
- ✅ 100% test pass rate maintained
- ✅ Zero performance regression (#[inline] on all helpers)

## Expected Results

**Code Metrics:**
- Main function: 123 → ~20 lines (-103 lines, -84% reduction)
- Functions created: ~8 total (4 simple + 3 recursive + 1 array)
- Complexity: 73 → <10 (target achieved)

**Time Savings vs Original Estimate:**
- Original (from roadmap): 40 hours
- DEPYLER-0140/0141/0142 experience: 4-5 hours
- Savings: 35+ hours (87% reduction)

---

**Last Updated**: 2025-10-10
**Status**: PLANNED - Ready to start based on DEPYLER-0140/0141/0142 success
**Next**: Begin Phase 1 extraction

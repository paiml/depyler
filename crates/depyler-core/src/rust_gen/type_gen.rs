//! Type conversion utilities
//!
//! This module provides conversion functions from internal RustType representation
//! to syn::Type tokens used for code generation.

use crate::hir::BinOp;
use crate::rust_gen::CodeGenContext;
use anyhow::{bail, Result};
use syn::{self, parse_quote};

/// Convert logical and bitwise binary operators to syn::BinOp
///
/// # Complexity
/// 8 (match with 7 arms)
#[inline]
fn convert_logical_bitwise_binop(op: BinOp) -> Option<Result<syn::BinOp>> {
    use BinOp::*;

    // DEPYLER-1005: Use syn::BinOp variants directly to avoid tokenization issues
    // parse_quote! for compound operators can produce incorrect spacing when re-quoted
    Some(Ok(match op {
        // Logical operators
        And => syn::BinOp::And(Default::default()),
        Or => syn::BinOp::Or(Default::default()),

        // Bitwise operators
        BitAnd => syn::BinOp::BitAnd(Default::default()),
        BitOr => syn::BinOp::BitOr(Default::default()),
        BitXor => syn::BinOp::BitXor(Default::default()),
        LShift => syn::BinOp::Shl(Default::default()),
        RShift => syn::BinOp::Shr(Default::default()),

        _ => return None,
    }))
}

/// Convert binary operator to syn::BinOp
///
/// Maps Python binary operators to their Rust equivalents.
/// Special operators like FloorDiv and Pow are handled separately
/// in convert_binary with Python semantics.
///
/// # Arguments
/// * `op` - The HIR binary operator
///
/// # Returns
/// The corresponding syn::BinOp token
///
/// # Errors
/// Returns error for special operators (FloorDiv, Pow, In, NotIn) that
/// require custom handling in convert_binary
///
/// # Complexity
/// 10 (main match + logical/bitwise helper)
pub fn convert_binop(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;

    // Try logical/bitwise operators first
    if let Some(result) = convert_logical_bitwise_binop(op) {
        return result;
    }

    match op {
        // Arithmetic operators - use syn::BinOp variants directly to avoid tokenization issues
        // DEPYLER-1005: parse_quote! for compound operators (==, !=, etc) can produce
        // incorrect tokenization (= = instead of ==) when re-quoted
        Add => Ok(syn::BinOp::Add(Default::default())),
        Sub => Ok(syn::BinOp::Sub(Default::default())),
        Mul => Ok(syn::BinOp::Mul(Default::default())),
        Div => Ok(syn::BinOp::Div(Default::default())),
        Mod => Ok(syn::BinOp::Rem(Default::default())),

        // Special arithmetic cases handled by convert_binary
        FloorDiv => {
            bail!("Floor division handled by convert_binary with Python semantics")
        }
        Pow => bail!("Power operator handled by convert_binary with type-specific logic"),

        // Comparison operators - DEPYLER-1005: Use syn::BinOp variants directly
        Eq => Ok(syn::BinOp::Eq(Default::default())),
        NotEq => Ok(syn::BinOp::Ne(Default::default())),
        Lt => Ok(syn::BinOp::Lt(Default::default())),
        LtEq => Ok(syn::BinOp::Le(Default::default())),
        Gt => Ok(syn::BinOp::Gt(Default::default())),
        GtEq => Ok(syn::BinOp::Ge(Default::default())),

        // Special membership operators handled in convert_binary
        In | NotIn => bail!("in/not in operators should be handled by convert_binary"),

        // Logical/bitwise handled above
        And | Or | BitAnd | BitOr | BitXor | LShift | RShift => {
            unreachable!("Logical/bitwise operators handled by convert_logical_bitwise_binop")
        }
    }
}

/// Convert Str type with optional lifetime to syn::Type
///
/// Handles both `&str` and `&'a str` variants.
///
/// # Complexity
/// 2 (single if/else branch)
fn str_type_to_syn(lifetime: &Option<String>) -> syn::Type {
    if let Some(lt) = lifetime {
        let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
        parse_quote! { &#lt_ident str }
    } else {
        parse_quote! { &str }
    }
}

/// Convert Reference type with mutable and lifetime to syn::Type
///
/// Handles all 4 combinations of mutable × lifetime:
/// - `&T`, `&mut T`, `&'a T`, `&'a mut T`
///
/// # Complexity
/// 5 (nested if/else for mutable and lifetime)
fn reference_type_to_syn(
    lifetime: &Option<String>,
    mutable: bool,
    inner: &crate::type_mapper::RustType,
) -> Result<syn::Type> {
    let inner_ty = rust_type_to_syn(inner)?;

    Ok(if mutable {
        if let Some(lt) = lifetime {
            let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
            parse_quote! { &#lt_ident mut #inner_ty }
        } else {
            parse_quote! { &mut #inner_ty }
        }
    } else if let Some(lt) = lifetime {
        let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
        parse_quote! { &#lt_ident #inner_ty }
    } else {
        parse_quote! { &#inner_ty }
    })
}

/// Convert Array type with const generic size to syn::Type
///
/// Handles 3 const generic size variants:
/// - Literal: `[T; 10]`
/// - Parameter: `[T; N]`
/// - Expression: `[T; SIZE * 2]`
///
/// # Complexity
/// 4 (match with 3 arms)
fn array_type_to_syn(
    element_type: &crate::type_mapper::RustType,
    size: &crate::type_mapper::RustConstGeneric,
) -> Result<syn::Type> {
    let element = rust_type_to_syn(element_type)?;

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
            let expr_tokens: proc_macro2::TokenStream = expr.parse().unwrap_or_else(|_| {
                quote::quote! { /* invalid const expression */ }
            });
            parse_quote! { [#element; #expr_tokens] }
        }
    })
}

/// Convert collection types (Vec, HashMap, HashSet, Option, Result, Tuple)
///
/// # Complexity
/// 7 (match with 6 arms + recursive call)
#[inline]
fn collection_type_to_syn(rust_type: &crate::type_mapper::RustType) -> Result<Option<syn::Type>> {
    use crate::type_mapper::RustType;

    Ok(Some(match rust_type {
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Vec<#inner_ty> }
        }
        // DEPYLER-0685: Use fully qualified path for HashMap to avoid import issues
        RustType::HashMap(k, v) => {
            let key_ty = rust_type_to_syn(k)?;
            let val_ty = rust_type_to_syn(v)?;
            parse_quote! { std::collections::HashMap<#key_ty, #val_ty> }
        }
        // DEPYLER-0685: Use fully qualified path for HashSet to avoid import issues
        RustType::HashSet(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { std::collections::HashSet<#inner_ty> }
        }
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Option<#inner_ty> }
        }
        RustType::Result(ok, err) => {
            let ok_ty = rust_type_to_syn(ok)?;
            let err_ty = rust_type_to_syn(err)?;
            parse_quote! { Result<#ok_ty, #err_ty> }
        }
        RustType::Tuple(types) => {
            let tys: Vec<_> = types
                .iter()
                .map(rust_type_to_syn)
                .collect::<Result<Vec<_>>>()?;
            parse_quote! { (#(#tys),*) }
        }
        _ => return Ok(None),
    }))
}

/// Convert RustType to syn::Type
///
/// Main type conversion function that recursively converts internal RustType
/// representation to syn::Type tokens for code generation.
///
/// # Arguments
/// * `rust_type` - The internal RustType to convert
///
/// # Returns
/// The corresponding syn::Type token
///
/// # Errors
/// Returns error for unsupported types or invalid custom type strings
///
/// # Complexity
/// 10 (main match + collection helper)
pub fn rust_type_to_syn(rust_type: &crate::type_mapper::RustType) -> Result<syn::Type> {
    use crate::type_mapper::RustType;

    // Try collection types first
    if let Some(ty) = collection_type_to_syn(rust_type)? {
        return Ok(ty);
    }

    Ok(match rust_type {
        RustType::Primitive(p) => {
            let ident = syn::Ident::new(p.to_rust_string(), proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::String => parse_quote! { String },
        RustType::Str { lifetime } => str_type_to_syn(lifetime),
        RustType::Cow { lifetime } => {
            let lt_ident = syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
            parse_quote! { Cow<#lt_ident, str> }
        }
        RustType::Reference {
            lifetime,
            mutable,
            inner,
        } => reference_type_to_syn(lifetime, *mutable, inner)?,
        RustType::Unit => parse_quote! { () },
        RustType::Custom(name) => {
            let ty: syn::Type = syn::parse_str(name)?;
            ty
        }
        RustType::Unsupported(reason) => bail!("Unsupported Rust type: {}", reason),
        RustType::TypeParam(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: Vec<_> = params
                .iter()
                .map(rust_type_to_syn)
                .collect::<Result<Vec<_>>>()?;
            parse_quote! { #base_ident<#(#param_types),*> }
        }
        RustType::Enum { name, .. } => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::Array { element_type, size } => array_type_to_syn(element_type, size)?,
        // Collection types handled above
        RustType::Vec(_)
        | RustType::HashMap(_, _)
        | RustType::HashSet(_)
        | RustType::Option(_)
        | RustType::Result(_, _)
        | RustType::Tuple(_) => {
            unreachable!("Collection types handled by collection_type_to_syn")
        }
    })
}

/// Updates import needs for custom type names
///
/// Detects specific custom types (FnvHashMap, AHashMap, Arc, Rc, HashMap)
/// and sets appropriate import flags.
///
/// # Complexity
/// 6 (if-else chain for 5 patterns)
#[inline]
fn update_custom_type_imports(ctx: &mut CodeGenContext, name: &str) {
    if name.contains("FnvHashMap") {
        ctx.needs_fnv_hashmap = true;
    } else if name.contains("AHashMap") {
        ctx.needs_ahash_hashmap = true;
    } else if name.contains("Arc<") {
        ctx.needs_arc = true;
    } else if name.contains("Rc<") {
        ctx.needs_rc = true;
    } else if name.contains("HashMap<")
        && !name.contains("FnvHashMap")
        && !name.contains("AHashMap")
    {
        ctx.needs_hashmap = true;
    }
}

/// Updates the import needs based on the rust type being used
///
/// Recursively walks through type structure to determine which imports
/// are needed (HashMap, Cow, Arc, Rc, etc.)
///
/// # Arguments
/// * `ctx` - Code generation context to update import flags
/// * `rust_type` - The type to analyze for import needs
///
/// # Complexity
/// 9 (match with 8 arms + helper function)
pub fn update_import_needs(ctx: &mut CodeGenContext, rust_type: &crate::type_mapper::RustType) {
    match rust_type {
        crate::type_mapper::RustType::HashMap(_, _) => ctx.needs_hashmap = true,
        crate::type_mapper::RustType::HashSet(inner) => {
            ctx.needs_hashset = true;
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Cow { .. } => ctx.needs_cow = true,
        crate::type_mapper::RustType::Custom(name) => update_custom_type_imports(ctx, name),
        crate::type_mapper::RustType::Reference { inner, .. } => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Vec(inner) => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Option(inner) => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Result(ok, err) => {
            update_import_needs(ctx, ok);
            update_import_needs(ctx, err);
        }
        crate::type_mapper::RustType::Tuple(types) => {
            for t in types {
                update_import_needs(ctx, t);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::{PrimitiveType, RustConstGeneric, RustType};
    use quote::ToTokens;

    // ============ convert_logical_bitwise_binop tests ============

    #[test]
    fn test_logical_binop_and() {
        let result = convert_logical_bitwise_binop(BinOp::And);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_logical_binop_or() {
        let result = convert_logical_bitwise_binop(BinOp::Or);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_bitwise_binop_bitand() {
        let result = convert_logical_bitwise_binop(BinOp::BitAnd);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_bitwise_binop_bitor() {
        let result = convert_logical_bitwise_binop(BinOp::BitOr);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_bitwise_binop_bitxor() {
        let result = convert_logical_bitwise_binop(BinOp::BitXor);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_bitwise_binop_lshift() {
        let result = convert_logical_bitwise_binop(BinOp::LShift);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_bitwise_binop_rshift() {
        let result = convert_logical_bitwise_binop(BinOp::RShift);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_logical_bitwise_returns_none_for_add() {
        let result = convert_logical_bitwise_binop(BinOp::Add);
        assert!(result.is_none());
    }

    #[test]
    fn test_logical_bitwise_returns_none_for_mul() {
        let result = convert_logical_bitwise_binop(BinOp::Mul);
        assert!(result.is_none());
    }

    // ============ convert_binop tests - arithmetic ============

    #[test]
    fn test_convert_binop_arithmetic() {
        assert!(convert_binop(BinOp::Add).is_ok());
        assert!(convert_binop(BinOp::Sub).is_ok());
        assert!(convert_binop(BinOp::Mul).is_ok());
        assert!(convert_binop(BinOp::Div).is_ok());
        assert!(convert_binop(BinOp::Mod).is_ok());
    }

    #[test]
    fn test_convert_binop_add_token() {
        let op = convert_binop(BinOp::Add).unwrap();
        assert!(matches!(op, syn::BinOp::Add(_)));
    }

    #[test]
    fn test_convert_binop_sub_token() {
        let op = convert_binop(BinOp::Sub).unwrap();
        assert!(matches!(op, syn::BinOp::Sub(_)));
    }

    #[test]
    fn test_convert_binop_mul_token() {
        let op = convert_binop(BinOp::Mul).unwrap();
        assert!(matches!(op, syn::BinOp::Mul(_)));
    }

    #[test]
    fn test_convert_binop_div_token() {
        let op = convert_binop(BinOp::Div).unwrap();
        assert!(matches!(op, syn::BinOp::Div(_)));
    }

    #[test]
    fn test_convert_binop_mod_token() {
        let op = convert_binop(BinOp::Mod).unwrap();
        assert!(matches!(op, syn::BinOp::Rem(_)));
    }

    // ============ convert_binop tests - comparison ============

    #[test]
    fn test_convert_binop_eq() {
        let op = convert_binop(BinOp::Eq).unwrap();
        assert!(matches!(op, syn::BinOp::Eq(_)));
    }

    #[test]
    fn test_convert_binop_noteq() {
        let op = convert_binop(BinOp::NotEq).unwrap();
        assert!(matches!(op, syn::BinOp::Ne(_)));
    }

    #[test]
    fn test_convert_binop_lt() {
        let op = convert_binop(BinOp::Lt).unwrap();
        assert!(matches!(op, syn::BinOp::Lt(_)));
    }

    #[test]
    fn test_convert_binop_lteq() {
        let op = convert_binop(BinOp::LtEq).unwrap();
        assert!(matches!(op, syn::BinOp::Le(_)));
    }

    #[test]
    fn test_convert_binop_gt() {
        let op = convert_binop(BinOp::Gt).unwrap();
        assert!(matches!(op, syn::BinOp::Gt(_)));
    }

    #[test]
    fn test_convert_binop_gteq() {
        let op = convert_binop(BinOp::GtEq).unwrap();
        assert!(matches!(op, syn::BinOp::Ge(_)));
    }

    // ============ convert_binop tests - logical/bitwise ============

    #[test]
    fn test_convert_binop_and() {
        let op = convert_binop(BinOp::And).unwrap();
        assert!(matches!(op, syn::BinOp::And(_)));
    }

    #[test]
    fn test_convert_binop_or() {
        let op = convert_binop(BinOp::Or).unwrap();
        assert!(matches!(op, syn::BinOp::Or(_)));
    }

    #[test]
    fn test_convert_binop_bitand() {
        let op = convert_binop(BinOp::BitAnd).unwrap();
        assert!(matches!(op, syn::BinOp::BitAnd(_)));
    }

    #[test]
    fn test_convert_binop_bitor() {
        let op = convert_binop(BinOp::BitOr).unwrap();
        assert!(matches!(op, syn::BinOp::BitOr(_)));
    }

    #[test]
    fn test_convert_binop_bitxor() {
        let op = convert_binop(BinOp::BitXor).unwrap();
        assert!(matches!(op, syn::BinOp::BitXor(_)));
    }

    #[test]
    fn test_convert_binop_lshift() {
        let op = convert_binop(BinOp::LShift).unwrap();
        assert!(matches!(op, syn::BinOp::Shl(_)));
    }

    #[test]
    fn test_convert_binop_rshift() {
        let op = convert_binop(BinOp::RShift).unwrap();
        assert!(matches!(op, syn::BinOp::Shr(_)));
    }

    // ============ convert_binop tests - special cases ============

    #[test]
    fn test_convert_binop_special() {
        assert!(convert_binop(BinOp::FloorDiv).is_err());
        assert!(convert_binop(BinOp::Pow).is_err());
    }

    #[test]
    fn test_convert_binop_floordiv_error_message() {
        match convert_binop(BinOp::FloorDiv) {
            Err(e) => assert!(e.to_string().contains("Floor division")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_convert_binop_pow_error_message() {
        match convert_binop(BinOp::Pow) {
            Err(e) => assert!(e.to_string().contains("Power operator")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_convert_binop_in_error() {
        match convert_binop(BinOp::In) {
            Err(e) => assert!(e.to_string().contains("in/not in")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_convert_binop_notin_error() {
        match convert_binop(BinOp::NotIn) {
            Err(e) => assert!(e.to_string().contains("in/not in")),
            Ok(_) => panic!("Expected error"),
        }
    }

    // ============ str_type_to_syn tests ============

    #[test]
    fn test_str_type_no_lifetime() {
        let ty = str_type_to_syn(&None);
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("str"));
        assert!(!tokens.contains("'"));
    }

    #[test]
    fn test_str_type_with_lifetime() {
        let ty = str_type_to_syn(&Some("'a".to_string()));
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("str"));
        assert!(tokens.contains("'a"));
    }

    #[test]
    fn test_str_type_with_static_lifetime() {
        let ty = str_type_to_syn(&Some("'static".to_string()));
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("'static"));
    }

    // ============ reference_type_to_syn tests ============

    #[test]
    fn test_reference_immut_no_lifetime() {
        let inner = RustType::Primitive(PrimitiveType::I32);
        let ty = reference_type_to_syn(&None, false, &inner).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("&"));
        assert!(tokens.contains("i32"));
        assert!(!tokens.contains("mut"));
    }

    #[test]
    fn test_reference_mut_no_lifetime() {
        let inner = RustType::Primitive(PrimitiveType::I32);
        let ty = reference_type_to_syn(&None, true, &inner).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("& mut"));
        assert!(tokens.contains("i32"));
    }

    #[test]
    fn test_reference_immut_with_lifetime() {
        let inner = RustType::Primitive(PrimitiveType::I32);
        let ty = reference_type_to_syn(&Some("'a".to_string()), false, &inner).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("'a"));
        assert!(tokens.contains("i32"));
        assert!(!tokens.contains("mut"));
    }

    #[test]
    fn test_reference_mut_with_lifetime() {
        let inner = RustType::Primitive(PrimitiveType::I32);
        let ty = reference_type_to_syn(&Some("'a".to_string()), true, &inner).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("'a"));
        assert!(tokens.contains("mut"));
        assert!(tokens.contains("i32"));
    }

    #[test]
    fn test_reference_nested() {
        let inner = RustType::Vec(Box::new(RustType::String));
        let ty = reference_type_to_syn(&None, false, &inner).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("Vec"));
        assert!(tokens.contains("String"));
    }

    // ============ array_type_to_syn tests ============

    #[test]
    fn test_array_literal_size() {
        let element = RustType::Primitive(PrimitiveType::I32);
        let size = RustConstGeneric::Literal(10);
        let ty = array_type_to_syn(&element, &size).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("["));
        assert!(tokens.contains("i32"));
        assert!(tokens.contains("10"));
    }

    #[test]
    fn test_array_parameter_size() {
        let element = RustType::Primitive(PrimitiveType::U8);
        let size = RustConstGeneric::Parameter("N".to_string());
        let ty = array_type_to_syn(&element, &size).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("u8"));
        assert!(tokens.contains("N"));
    }

    #[test]
    fn test_array_expression_size() {
        let element = RustType::Primitive(PrimitiveType::F64);
        let size = RustConstGeneric::Expression("SIZE * 2".to_string());
        let ty = array_type_to_syn(&element, &size).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("f64"));
        assert!(tokens.contains("SIZE"));
    }

    #[test]
    fn test_array_zero_size() {
        let element = RustType::Primitive(PrimitiveType::I32);
        let size = RustConstGeneric::Literal(0);
        let ty = array_type_to_syn(&element, &size).unwrap();
        let tokens = ty.to_token_stream().to_string();
        assert!(tokens.contains("0"));
    }

    // ============ collection_type_to_syn tests ============

    #[test]
    fn test_collection_vec() {
        let ty = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
        let tokens = result.unwrap().to_token_stream().to_string();
        assert!(tokens.contains("Vec"));
    }

    #[test]
    fn test_collection_hashmap() {
        let ty = RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(PrimitiveType::I32)),
        );
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
        let tokens = result.unwrap().to_token_stream().to_string();
        assert!(tokens.contains("HashMap"));
    }

    #[test]
    fn test_collection_hashset() {
        let ty = RustType::HashSet(Box::new(RustType::String));
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
        let tokens = result.unwrap().to_token_stream().to_string();
        assert!(tokens.contains("HashSet"));
    }

    #[test]
    fn test_collection_option() {
        let ty = RustType::Option(Box::new(RustType::String));
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
        let tokens = result.unwrap().to_token_stream().to_string();
        assert!(tokens.contains("Option"));
    }

    #[test]
    fn test_collection_result() {
        let ty = RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("Error".to_string())),
        );
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
        let tokens = result.unwrap().to_token_stream().to_string();
        assert!(tokens.contains("Result"));
    }

    #[test]
    fn test_collection_tuple_empty() {
        let ty = RustType::Tuple(vec![]);
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_collection_tuple_single() {
        let ty = RustType::Tuple(vec![RustType::Primitive(PrimitiveType::I32)]);
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_collection_tuple_multiple() {
        let ty = RustType::Tuple(vec![
            RustType::String,
            RustType::Primitive(PrimitiveType::I32),
            RustType::Primitive(PrimitiveType::Bool),
        ]);
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_collection_returns_none_for_primitive() {
        let ty = RustType::Primitive(PrimitiveType::I32);
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_collection_returns_none_for_string() {
        let ty = RustType::String;
        let result = collection_type_to_syn(&ty).unwrap();
        assert!(result.is_none());
    }

    // ============ rust_type_to_syn tests - primitives ============

    #[test]
    fn test_rust_type_to_syn_primitive() {
        let ty = RustType::Primitive(PrimitiveType::I32);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "i32");
    }

    #[test]
    fn test_rust_type_to_syn_i8() {
        let ty = RustType::Primitive(PrimitiveType::I8);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "i8");
    }

    #[test]
    fn test_rust_type_to_syn_i16() {
        let ty = RustType::Primitive(PrimitiveType::I16);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "i16");
    }

    #[test]
    fn test_rust_type_to_syn_i64() {
        let ty = RustType::Primitive(PrimitiveType::I64);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "i64");
    }

    #[test]
    fn test_rust_type_to_syn_i128() {
        let ty = RustType::Primitive(PrimitiveType::I128);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "i128");
    }

    #[test]
    fn test_rust_type_to_syn_isize() {
        let ty = RustType::Primitive(PrimitiveType::ISize);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "isize");
    }

    #[test]
    fn test_rust_type_to_syn_u8() {
        let ty = RustType::Primitive(PrimitiveType::U8);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "u8");
    }

    #[test]
    fn test_rust_type_to_syn_u16() {
        let ty = RustType::Primitive(PrimitiveType::U16);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "u16");
    }

    #[test]
    fn test_rust_type_to_syn_u32() {
        let ty = RustType::Primitive(PrimitiveType::U32);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "u32");
    }

    #[test]
    fn test_rust_type_to_syn_u64() {
        let ty = RustType::Primitive(PrimitiveType::U64);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "u64");
    }

    #[test]
    fn test_rust_type_to_syn_u128() {
        let ty = RustType::Primitive(PrimitiveType::U128);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "u128");
    }

    #[test]
    fn test_rust_type_to_syn_usize() {
        let ty = RustType::Primitive(PrimitiveType::USize);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "usize");
    }

    #[test]
    fn test_rust_type_to_syn_f32() {
        let ty = RustType::Primitive(PrimitiveType::F32);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "f32");
    }

    #[test]
    fn test_rust_type_to_syn_f64() {
        let ty = RustType::Primitive(PrimitiveType::F64);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "f64");
    }

    #[test]
    fn test_rust_type_to_syn_bool() {
        let ty = RustType::Primitive(PrimitiveType::Bool);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "bool");
    }

    // Note: Char is not a variant in PrimitiveType enum

    // ============ rust_type_to_syn tests - string types ============

    #[test]
    fn test_rust_type_to_syn_string() {
        let ty = RustType::String;
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "String");
    }

    #[test]
    fn test_rust_type_to_syn_str_no_lifetime() {
        let ty = RustType::Str { lifetime: None };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("str"));
    }

    #[test]
    fn test_rust_type_to_syn_str_with_lifetime() {
        let ty = RustType::Str {
            lifetime: Some("'a".to_string()),
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("'a"));
        assert!(tokens.contains("str"));
    }

    #[test]
    fn test_rust_type_to_syn_cow() {
        let ty = RustType::Cow {
            lifetime: "'a".to_string(),
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Cow"));
        assert!(tokens.contains("'a"));
    }

    // ============ rust_type_to_syn tests - references ============

    #[test]
    fn test_rust_type_to_syn_reference_immut() {
        let ty = RustType::Reference {
            lifetime: None,
            mutable: false,
            inner: Box::new(RustType::Primitive(PrimitiveType::I32)),
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("&"));
        assert!(tokens.contains("i32"));
    }

    #[test]
    fn test_rust_type_to_syn_reference_mut() {
        let ty = RustType::Reference {
            lifetime: None,
            mutable: true,
            inner: Box::new(RustType::Primitive(PrimitiveType::I32)),
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("mut"));
    }

    // ============ rust_type_to_syn tests - special types ============

    #[test]
    fn test_rust_type_to_syn_unit() {
        let ty = RustType::Unit;
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("("));
        assert!(tokens.contains(")"));
    }

    #[test]
    fn test_rust_type_to_syn_custom() {
        let ty = RustType::Custom("MyStruct".to_string());
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("MyStruct"));
    }

    #[test]
    fn test_rust_type_to_syn_custom_with_path() {
        let ty = RustType::Custom("std::io::Error".to_string());
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("std"));
        assert!(tokens.contains("io"));
        assert!(tokens.contains("Error"));
    }

    #[test]
    fn test_rust_type_to_syn_unsupported() {
        let ty = RustType::Unsupported("cannot convert".to_string());
        let result = rust_type_to_syn(&ty);
        assert!(result.is_err());
        // Check error message by matching
        match result {
            Err(e) => assert!(e.to_string().contains("cannot convert")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_rust_type_to_syn_type_param() {
        let ty = RustType::TypeParam("T".to_string());
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert_eq!(tokens, "T");
    }

    #[test]
    fn test_rust_type_to_syn_generic() {
        let ty = RustType::Generic {
            base: "Container".to_string(),
            params: vec![
                RustType::Primitive(PrimitiveType::I32),
                RustType::String,
            ],
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Container"));
        assert!(tokens.contains("i32"));
        assert!(tokens.contains("String"));
    }

    #[test]
    fn test_rust_type_to_syn_enum() {
        let ty = RustType::Enum {
            name: "Status".to_string(),
            variants: vec![],
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert_eq!(tokens, "Status");
    }

    #[test]
    fn test_rust_type_to_syn_array() {
        let ty = RustType::Array {
            element_type: Box::new(RustType::Primitive(PrimitiveType::I32)),
            size: RustConstGeneric::Literal(5),
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("i32"));
        assert!(tokens.contains("5"));
    }

    // ============ rust_type_to_syn tests - collections ============

    #[test]
    fn test_rust_type_to_syn_vec() {
        let ty = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Vec"));
        assert!(tokens.contains("i32"));
    }

    #[test]
    fn test_rust_type_to_syn_hashmap() {
        let ty = RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(PrimitiveType::I32)),
        );
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("HashMap"));
    }

    #[test]
    fn test_rust_type_to_syn_hashset() {
        let ty = RustType::HashSet(Box::new(RustType::String));
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("HashSet"));
    }

    #[test]
    fn test_rust_type_to_syn_option() {
        let ty = RustType::Option(Box::new(RustType::String));
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Option"));
        assert!(tokens.contains("String"));
    }

    #[test]
    fn test_rust_type_to_syn_result() {
        let ty = RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("MyError".to_string())),
        );
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Result"));
    }

    #[test]
    fn test_rust_type_to_syn_tuple() {
        let ty = RustType::Tuple(vec![
            RustType::Primitive(PrimitiveType::I32),
            RustType::String,
        ]);
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("i32"));
        assert!(tokens.contains("String"));
    }

    // ============ update_custom_type_imports tests ============

    #[test]
    fn test_update_imports_fnv_hashmap() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "FnvHashMap<String, i32>");
        assert!(ctx.needs_fnv_hashmap);
    }

    #[test]
    fn test_update_imports_ahash_hashmap() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "AHashMap<String, i32>");
        assert!(ctx.needs_ahash_hashmap);
    }

    #[test]
    fn test_update_imports_arc() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "Arc<Mutex<Data>>");
        assert!(ctx.needs_arc);
    }

    #[test]
    fn test_update_imports_rc() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "Rc<RefCell<Data>>");
        assert!(ctx.needs_rc);
    }

    #[test]
    fn test_update_imports_hashmap() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "HashMap<String, i32>");
        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_update_imports_hashmap_not_fnv() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "FnvHashMap<String, i32>");
        // Should NOT set needs_hashmap since it's FnvHashMap
        assert!(!ctx.needs_hashmap);
    }

    #[test]
    fn test_update_imports_hashmap_not_ahash() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "AHashMap<String, i32>");
        // Should NOT set needs_hashmap since it's AHashMap
        assert!(!ctx.needs_hashmap);
    }

    #[test]
    fn test_update_imports_unrecognized() {
        let mut ctx = CodeGenContext::default();
        update_custom_type_imports(&mut ctx, "MyCustomType");
        // Nothing should be set
        assert!(!ctx.needs_fnv_hashmap);
        assert!(!ctx.needs_ahash_hashmap);
        assert!(!ctx.needs_arc);
        assert!(!ctx.needs_rc);
        assert!(!ctx.needs_hashmap);
    }

    // ============ update_import_needs tests ============

    #[test]
    fn test_import_needs_hashmap() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(PrimitiveType::I32)),
        );
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_import_needs_hashset() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::HashSet(Box::new(RustType::String));
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashset);
    }

    #[test]
    fn test_import_needs_cow() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Cow {
            lifetime: "'a".to_string(),
        };
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_cow);
    }

    #[test]
    fn test_import_needs_custom_arc() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Custom("Arc<T>".to_string());
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_arc);
    }

    #[test]
    fn test_import_needs_custom_rc() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Custom("Rc<T>".to_string());
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_rc);
    }

    #[test]
    fn test_import_needs_reference_recursive() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Reference {
            lifetime: None,
            mutable: false,
            inner: Box::new(RustType::HashMap(
                Box::new(RustType::String),
                Box::new(RustType::Primitive(PrimitiveType::I32)),
            )),
        };
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_import_needs_vec_recursive() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Vec(Box::new(RustType::HashSet(Box::new(RustType::String))));
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashset);
    }

    #[test]
    fn test_import_needs_option_recursive() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Option(Box::new(RustType::Cow {
            lifetime: "'a".to_string(),
        }));
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_cow);
    }

    #[test]
    fn test_import_needs_result_recursive() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Result(
            Box::new(RustType::HashMap(
                Box::new(RustType::String),
                Box::new(RustType::Primitive(PrimitiveType::I32)),
            )),
            Box::new(RustType::Custom("Arc<Error>".to_string())),
        );
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashmap);
        assert!(ctx.needs_arc);
    }

    #[test]
    fn test_import_needs_tuple_recursive() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Tuple(vec![
            RustType::HashSet(Box::new(RustType::String)),
            RustType::Cow {
                lifetime: "'a".to_string(),
            },
        ]);
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashset);
        assert!(ctx.needs_cow);
    }

    #[test]
    fn test_import_needs_hashset_inner_recursive() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::HashSet(Box::new(RustType::Cow {
            lifetime: "'a".to_string(),
        }));
        update_import_needs(&mut ctx, &ty);
        assert!(ctx.needs_hashset);
        assert!(ctx.needs_cow);
    }

    #[test]
    fn test_import_needs_primitive_noop() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Primitive(PrimitiveType::I32);
        update_import_needs(&mut ctx, &ty);
        // Nothing should be set
        assert!(!ctx.needs_hashmap);
        assert!(!ctx.needs_hashset);
        assert!(!ctx.needs_cow);
        assert!(!ctx.needs_arc);
        assert!(!ctx.needs_rc);
    }

    #[test]
    fn test_import_needs_string_noop() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::String;
        update_import_needs(&mut ctx, &ty);
        // Nothing should be set
        assert!(!ctx.needs_hashmap);
    }

    #[test]
    fn test_import_needs_unit_noop() {
        let mut ctx = CodeGenContext::default();
        let ty = RustType::Unit;
        update_import_needs(&mut ctx, &ty);
        // Nothing should be set
        assert!(!ctx.needs_hashmap);
    }

    // ============ Edge cases and integration tests ============

    #[test]
    fn test_nested_vec_of_options() {
        let ty = RustType::Vec(Box::new(RustType::Option(Box::new(RustType::String))));
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Vec"));
        assert!(tokens.contains("Option"));
        assert!(tokens.contains("String"));
    }

    #[test]
    fn test_deeply_nested_type() {
        let ty = RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Vec(Box::new(RustType::Option(Box::new(
                RustType::Tuple(vec![
                    RustType::Primitive(PrimitiveType::I32),
                    RustType::String,
                ]),
            ))))),
        );
        let result = rust_type_to_syn(&ty).unwrap();
        assert!(result.to_token_stream().to_string().len() > 0);
    }

    #[test]
    fn test_custom_generic_type() {
        let ty = RustType::Custom("MyStruct<T, U>".to_string());
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("MyStruct"));
    }

    #[test]
    fn test_array_with_nested_type() {
        let ty = RustType::Array {
            element_type: Box::new(RustType::Option(Box::new(RustType::String))),
            size: RustConstGeneric::Literal(10),
        };
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Option"));
        assert!(tokens.contains("String"));
        assert!(tokens.contains("10"));
    }
}

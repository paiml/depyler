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

    Some(Ok(match op {
        // Logical operators
        And => parse_quote! { && },
        Or => parse_quote! { || },

        // Bitwise operators
        BitAnd => parse_quote! { & },
        BitOr => parse_quote! { | },
        BitXor => parse_quote! { ^ },
        LShift => parse_quote! { << },
        RShift => parse_quote! { >> },

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
        // Arithmetic operators
        Add => Ok(parse_quote! { + }),
        Sub => Ok(parse_quote! { - }),
        Mul => Ok(parse_quote! { * }),
        Div => Ok(parse_quote! { / }),
        Mod => Ok(parse_quote! { % }),

        // Special arithmetic cases handled by convert_binary
        FloorDiv => {
            bail!("Floor division handled by convert_binary with Python semantics")
        }
        Pow => bail!("Power operator handled by convert_binary with type-specific logic"),

        // Comparison operators
        Eq => Ok(parse_quote! { == }),
        NotEq => Ok(parse_quote! { != }),
        Lt => Ok(parse_quote! { < }),
        LtEq => Ok(parse_quote! { <= }),
        Gt => Ok(parse_quote! { > }),
        GtEq => Ok(parse_quote! { >= }),

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
        RustType::HashMap(k, v) => {
            let key_ty = rust_type_to_syn(k)?;
            let val_ty = rust_type_to_syn(v)?;
            parse_quote! { HashMap<#key_ty, #val_ty> }
        }
        RustType::HashSet(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { HashSet<#inner_ty> }
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
    use crate::type_mapper::{PrimitiveType, RustType};
    use quote::ToTokens;

    #[test]
    fn test_convert_binop_arithmetic() {
        assert!(convert_binop(BinOp::Add).is_ok());
        assert!(convert_binop(BinOp::Sub).is_ok());
        assert!(convert_binop(BinOp::Mul).is_ok());
        assert!(convert_binop(BinOp::Div).is_ok());
        assert!(convert_binop(BinOp::Mod).is_ok());
    }

    #[test]
    fn test_convert_binop_special() {
        // FloorDiv should bail
        assert!(convert_binop(BinOp::FloorDiv).is_err());
        // Pow should bail
        assert!(convert_binop(BinOp::Pow).is_err());
    }

    #[test]
    fn test_rust_type_to_syn_primitive() {
        let ty = RustType::Primitive(PrimitiveType::I32);
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "i32");
    }

    #[test]
    fn test_rust_type_to_syn_string() {
        let ty = RustType::String;
        let result = rust_type_to_syn(&ty).unwrap();
        assert_eq!(result.to_token_stream().to_string(), "String");
    }

    #[test]
    fn test_rust_type_to_syn_vec() {
        let ty = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Vec"));
        assert!(tokens.contains("i32"));
    }

    #[test]
    fn test_rust_type_to_syn_option() {
        let ty = RustType::Option(Box::new(RustType::String));
        let result = rust_type_to_syn(&ty).unwrap();
        let tokens = result.to_token_stream().to_string();
        assert!(tokens.contains("Option"));
        assert!(tokens.contains("String"));
    }
}

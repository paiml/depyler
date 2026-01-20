//! Borrowing helper functions extracted from expr_gen.rs
//! DEPYLER-COVERAGE-95: Extracted for testability

use crate::hir::Type;
use syn::parse_quote;

/// DEPYLER-0465: Add & to borrow a path expression if it's a simple variable
/// This prevents moving String parameters in PathBuf::from() and File::open()
///
/// DEPRECATED: Use `borrow_if_needed_typed` when type information is available.
pub fn borrow_if_needed(expr: &syn::Expr) -> syn::Expr {
    borrow_if_needed_typed(expr, None)
}

/// DEPYLER-1154: Type-aware borrowing - skip borrowing for Copy types
///
/// This function decides whether to add `&` to borrow an expression based on:
/// 1. Expression structure (only simple variables may need borrowing)
/// 2. Type information (Copy types don't need borrowing)
///
/// # Arguments
/// * `expr` - The expression to potentially borrow
/// * `var_type` - Optional type information for the variable
///
/// # Returns
/// The expression, potentially wrapped with `&` if borrowing is needed.
///
/// # Examples
/// ```ignore
/// // Copy type (i32) - no borrowing
/// borrow_if_needed_typed(&x, Some(&Type::Int)) -> x
///
/// // Non-Copy type (String) - borrows
/// borrow_if_needed_typed(&name, Some(&Type::String)) -> &name
///
/// // Unknown type - defensive borrowing
/// borrow_if_needed_typed(&val, None) -> &val
/// ```
pub fn borrow_if_needed_typed(expr: &syn::Expr, var_type: Option<&Type>) -> syn::Expr {
    match expr {
        // If it's a simple path (variable), check if borrowing is needed
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            // DEPYLER-1154: If we know the type is Copy, don't borrow
            if let Some(ty) = var_type {
                if ty.is_copy() {
                    return expr.clone();
                }
            }
            // Non-Copy or unknown type - borrow defensively
            parse_quote! { &#expr }
        }
        // Already a reference - don't double-borrow
        syn::Expr::Reference(_) => expr.clone(),
        // Otherwise, use as-is (literals, method calls, etc.)
        _ => expr.clone(),
    }
}

/// Wrap expression in parentheses
pub fn wrap_in_parens(expr: syn::Expr) -> syn::Expr {
    parse_quote! { (#expr) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    #[test]
    fn test_borrow_simple_variable() {
        let expr: syn::Expr = parse_quote! { filename };
        let result = borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "& filename");
    }

    #[test]
    fn test_borrow_string_literal_unchanged() {
        let expr: syn::Expr = parse_quote! { "hello" };
        let result = borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "\"hello\"");
    }

    #[test]
    fn test_borrow_method_call_unchanged() {
        let expr: syn::Expr = parse_quote! { path.to_string() };
        let result = borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "path . to_string ()");
    }

    #[test]
    fn test_borrow_qualified_path_unchanged() {
        let expr: syn::Expr = parse_quote! { std::path::Path };
        let result = borrow_if_needed(&expr);
        // Multi-segment path should not be borrowed
        assert_eq!(result.to_token_stream().to_string(), "std :: path :: Path");
    }

    #[test]
    fn test_wrap_in_parens() {
        let expr: syn::Expr = parse_quote! { a + b };
        let result = wrap_in_parens(expr);
        assert_eq!(result.to_token_stream().to_string(), "(a + b)");
    }

    #[test]
    fn test_wrap_simple_var_in_parens() {
        let expr: syn::Expr = parse_quote! { x };
        let result = wrap_in_parens(expr);
        assert_eq!(result.to_token_stream().to_string(), "(x)");
    }

    #[test]
    fn test_borrow_numeric_literal_unchanged() {
        let expr: syn::Expr = parse_quote! { 42 };
        let result = borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "42");
    }

    #[test]
    fn test_borrow_function_call_unchanged() {
        let expr: syn::Expr = parse_quote! { get_path() };
        let result = borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "get_path ()");
    }

    // ========================================================================
    // DEPYLER-1154: TYPE-AWARE BORROWING TESTS
    // ========================================================================

    #[test]
    fn test_DEPYLER_1154_int_type_no_borrow() {
        // Int type is Copy - should NOT be borrowed
        let expr: syn::Expr = parse_quote! { count };
        let result = borrow_if_needed_typed(&expr, Some(&Type::Int));
        assert_eq!(
            result.to_token_stream().to_string(),
            "count",
            "Int (Copy) should not be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_float_type_no_borrow() {
        // Float type is Copy - should NOT be borrowed
        let expr: syn::Expr = parse_quote! { value };
        let result = borrow_if_needed_typed(&expr, Some(&Type::Float));
        assert_eq!(
            result.to_token_stream().to_string(),
            "value",
            "Float (Copy) should not be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_bool_type_no_borrow() {
        // Bool type is Copy - should NOT be borrowed
        let expr: syn::Expr = parse_quote! { flag };
        let result = borrow_if_needed_typed(&expr, Some(&Type::Bool));
        assert_eq!(
            result.to_token_stream().to_string(),
            "flag",
            "Bool (Copy) should not be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_string_type_borrows() {
        // String type is NOT Copy - should be borrowed
        let expr: syn::Expr = parse_quote! { name };
        let result = borrow_if_needed_typed(&expr, Some(&Type::String));
        assert_eq!(
            result.to_token_stream().to_string(),
            "& name",
            "String (non-Copy) should be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_list_type_borrows() {
        // List type is NOT Copy - should be borrowed
        let expr: syn::Expr = parse_quote! { items };
        let result = borrow_if_needed_typed(&expr, Some(&Type::List(Box::new(Type::Int))));
        assert_eq!(
            result.to_token_stream().to_string(),
            "& items",
            "List (non-Copy) should be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_unknown_type_defensive_borrow() {
        // Unknown type - defensive borrowing
        let expr: syn::Expr = parse_quote! { unknown_var };
        let result = borrow_if_needed_typed(&expr, None);
        assert_eq!(
            result.to_token_stream().to_string(),
            "& unknown_var",
            "Unknown type should be borrowed defensively"
        );
    }

    #[test]
    fn test_DEPYLER_1154_already_reference_no_double_borrow() {
        // Expression is already a reference - don't double-borrow
        let expr: syn::Expr = parse_quote! { &some_ref };
        let result = borrow_if_needed_typed(&expr, Some(&Type::String));
        let tokens = result.to_token_stream().to_string();
        assert!(
            !tokens.contains("& &"),
            "Should not double-borrow: {}",
            tokens
        );
    }

    #[test]
    fn test_DEPYLER_1154_tuple_of_copy_types_no_borrow() {
        // Tuple(Int, Bool) is Copy - should NOT be borrowed
        let expr: syn::Expr = parse_quote! { pair };
        let result = borrow_if_needed_typed(
            &expr,
            Some(&Type::Tuple(vec![Type::Int, Type::Bool])),
        );
        assert_eq!(
            result.to_token_stream().to_string(),
            "pair",
            "Tuple of Copy types should not be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_tuple_with_string_borrows() {
        // Tuple(Int, String) is NOT Copy - should be borrowed
        let expr: syn::Expr = parse_quote! { pair };
        let result = borrow_if_needed_typed(
            &expr,
            Some(&Type::Tuple(vec![Type::Int, Type::String])),
        );
        assert_eq!(
            result.to_token_stream().to_string(),
            "& pair",
            "Tuple with non-Copy element should be borrowed"
        );
    }

    #[test]
    fn test_DEPYLER_1154_optional_copy_no_borrow() {
        // Optional(Int) is still Copy - should NOT be borrowed
        let expr: syn::Expr = parse_quote! { maybe_count };
        let result = borrow_if_needed_typed(
            &expr,
            Some(&Type::Optional(Box::new(Type::Int))),
        );
        assert_eq!(
            result.to_token_stream().to_string(),
            "maybe_count",
            "Optional<Copy> should not be borrowed"
        );
    }
}

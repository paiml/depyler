//! Borrowing helper functions extracted from expr_gen.rs
//! DEPYLER-COVERAGE-95: Extracted for testability

use syn::parse_quote;

/// DEPYLER-0465: Add & to borrow a path expression if it's a simple variable
/// This prevents moving String parameters in PathBuf::from() and File::open()
pub fn borrow_if_needed(expr: &syn::Expr) -> syn::Expr {
    match expr {
        // If it's a simple path (variable), add &
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            parse_quote! { &#expr }
        }
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
}

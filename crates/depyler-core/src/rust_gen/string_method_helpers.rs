//! DEPYLER-1025: String method helpers for Pattern trait handling
//!
//! Extracted from expr_gen.rs to reduce complexity and improve testability.
//! These helpers handle Python string methods that require Rust Pattern trait.

use crate::hir::{HirExpr, Literal};
use std::collections::HashSet;
use syn::parse_quote;

/// Extract a Pattern-compatible expression from a HIR argument.
///
/// The Rust Pattern trait is implemented for &str but NOT for String.
/// This helper ensures we extract bare string literals or borrow String variables.
///
/// # Arguments
/// * `hir_arg` - The HIR expression for the argument
/// * `rust_arg` - The already-converted Rust expression
/// * `fn_str_params` - Set of function parameter names that are already &str
///
/// # Returns
/// A syn::Expr that is guaranteed to implement Pattern trait
#[allow(dead_code)]
pub fn extract_pattern_arg(
    hir_arg: &HirExpr,
    rust_arg: &syn::Expr,
    fn_str_params: &HashSet<String>,
) -> syn::Expr {
    match hir_arg {
        // String literals are already &str - use directly
        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
        // Variables that are function params with `str` type are already &str
        HirExpr::Var(name) if fn_str_params.contains(name) => rust_arg.clone(),
        // Everything else is assumed to be owned String - borrow it
        _ => {
            parse_quote! { &#rust_arg }
        }
    }
}

/// Check if a variable name suggests it's a char from iteration.
/// Used for char-specific string method handling.
#[allow(dead_code)]
pub fn is_char_variable_name(name: &str) -> bool {
    matches!(name, "char" | "ch" | "c" | "character")
}

/// Simple string method mappings (no arguments, direct 1:1 mapping).
/// Returns the Rust method name for zero-argument string methods.
#[allow(dead_code)]
pub fn simple_string_method(method: &str) -> Option<&'static str> {
    match method {
        "upper" => Some("to_uppercase"),
        "lower" => Some("to_lowercase"),
        "title" => Some("to_titlecase"), // Note: custom implementation needed
        "swapcase" => Some("to_swapcase"), // Note: custom implementation needed
        "casefold" => Some("to_lowercase"), // casefold is like lower for ASCII
        "isalpha" => Some("chars().all(|c| c.is_alphabetic())"),
        "isdigit" => Some("chars().all(|c| c.is_ascii_digit())"),
        "isalnum" => Some("chars().all(|c| c.is_alphanumeric())"),
        "isspace" => Some("chars().all(|c| c.is_whitespace())"),
        "isupper" => Some("chars().all(|c| c.is_uppercase())"),
        "islower" => Some("chars().all(|c| c.is_lowercase())"),
        "isascii" => Some("is_ascii()"),
        _ => None,
    }
}

/// String methods that return a new String with trimming (zero arguments).
#[allow(dead_code)]
pub fn trim_string_method(method: &str) -> Option<&'static str> {
    match method {
        "strip" => Some("trim().to_string()"),
        "lstrip" => Some("trim_start().to_string()"),
        "rstrip" => Some("trim_end().to_string()"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_char_variable_name() {
        assert!(is_char_variable_name("char"));
        assert!(is_char_variable_name("ch"));
        assert!(is_char_variable_name("c"));
        assert!(is_char_variable_name("character"));
        assert!(!is_char_variable_name("x"));
        assert!(!is_char_variable_name("letter"));
    }

    #[test]
    fn test_simple_string_method() {
        assert_eq!(simple_string_method("upper"), Some("to_uppercase"));
        assert_eq!(simple_string_method("lower"), Some("to_lowercase"));
        assert_eq!(
            simple_string_method("isalpha"),
            Some("chars().all(|c| c.is_alphabetic())")
        );
        assert_eq!(simple_string_method("unknown"), None);
    }

    #[test]
    fn test_trim_string_method() {
        assert_eq!(trim_string_method("strip"), Some("trim().to_string()"));
        assert_eq!(
            trim_string_method("lstrip"),
            Some("trim_start().to_string()")
        );
        assert_eq!(trim_string_method("rstrip"), Some("trim_end().to_string()"));
        assert_eq!(trim_string_method("unknown"), None);
    }

    #[test]
    fn test_extract_pattern_arg_with_literal() {
        let hir_arg = HirExpr::Literal(Literal::String("test".to_string()));
        let rust_arg: syn::Expr = parse_quote! { test };
        let fn_str_params = HashSet::new();

        let result = extract_pattern_arg(&hir_arg, &rust_arg, &fn_str_params);
        // Result should be the literal string directly
        let result_str = quote::quote!(#result).to_string();
        assert!(
            result_str.contains("test"),
            "Expected literal, got: {}",
            result_str
        );
    }

    #[test]
    fn test_extract_pattern_arg_with_str_param() {
        let hir_arg = HirExpr::Var("prefix".to_string());
        let rust_arg: syn::Expr = parse_quote! { prefix };
        let mut fn_str_params = HashSet::new();
        fn_str_params.insert("prefix".to_string());

        let result = extract_pattern_arg(&hir_arg, &rust_arg, &fn_str_params);
        // Result should be the variable directly (not borrowed)
        let result_str = quote::quote!(#result).to_string();
        assert!(
            result_str.contains("prefix"),
            "Expected var, got: {}",
            result_str
        );
        assert!(!result_str.contains("&"), "Should not borrow &str param");
    }

    #[test]
    fn test_extract_pattern_arg_with_owned_string() {
        let hir_arg = HirExpr::Var("pattern".to_string());
        let rust_arg: syn::Expr = parse_quote! { pattern };
        let fn_str_params = HashSet::new();

        let result = extract_pattern_arg(&hir_arg, &rust_arg, &fn_str_params);
        // Result should be borrowed
        let result_str = quote::quote!(#result).to_string();
        assert!(
            result_str.contains("&"),
            "Should borrow owned String, got: {}",
            result_str
        );
    }
}

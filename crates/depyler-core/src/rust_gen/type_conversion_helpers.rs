//! Type Conversion Helpers for Code Generation
//!
//! This module contains helpers for applying type conversions during
//! Rust code generation. Extracted from stmt_gen.rs for better testability.
//!
//! DEPYLER-0455: Type conversion for assignment expressions

use crate::hir::Type;
use syn::parse_quote;

/// Apply type conversion to value expression
///
/// Wraps the expression with appropriate conversion based on target type:
/// - `Type::Int` -> `expr as i32` (handles usize->i32 conversions)
/// - `Type::String` -> `expr.to_string()` (handles &str->String)
/// - Other types -> expression unchanged
///
/// # Examples
///
/// ```ignore
/// // Int conversion
/// let expr: syn::Expr = parse_quote!(len);
/// let result = apply_type_conversion(expr, &Type::Int);
/// // Result: len as i32
///
/// // String conversion
/// let expr: syn::Expr = parse_quote!(name);
/// let result = apply_type_conversion(expr, &Type::String);
/// // Result: name.to_string()
/// ```
pub fn apply_type_conversion(value_expr: syn::Expr, target_type: &Type) -> syn::Expr {
    match target_type {
        Type::Int => {
            // Convert to i32 using 'as' cast
            // This handles usize->i32 conversions
            parse_quote! { #value_expr as i32 }
        }
        Type::String => {
            // DEPYLER-0455 Bug 7: Convert &str to String using .to_string()
            // This handles validator function parameters (&str) returned as String
            parse_quote! { #value_expr.to_string() }
        }
        _ => value_expr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    // Helper to convert Expr to string for comparison
    fn expr_to_string(expr: syn::Expr) -> String {
        quote::quote!(#expr).to_string().replace(" ", "")
    }

    // ============ Type::Int conversion tests ============

    #[test]
    fn test_int_conversion_simple_var() {
        let expr: syn::Expr = parse_quote!(x);
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "xasi32");
    }

    #[test]
    fn test_int_conversion_len_call() {
        let expr: syn::Expr = parse_quote!(arr.len());
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "arr.len()asi32");
    }

    #[test]
    fn test_int_conversion_method_call() {
        let expr: syn::Expr = parse_quote!(vec.count());
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "vec.count()asi32");
    }

    #[test]
    fn test_int_conversion_literal() {
        let expr: syn::Expr = parse_quote!(42usize);
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "42usizeasi32");
    }

    #[test]
    fn test_int_conversion_binary_op() {
        let expr: syn::Expr = parse_quote!(a + b);
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "a+basi32");
    }

    #[test]
    fn test_int_conversion_index_access() {
        let expr: syn::Expr = parse_quote!(arr[0]);
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "arr[0]asi32");
    }

    #[test]
    fn test_int_conversion_field_access() {
        let expr: syn::Expr = parse_quote!(obj.field);
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "obj.fieldasi32");
    }

    #[test]
    fn test_int_conversion_function_call() {
        let expr: syn::Expr = parse_quote!(get_size());
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "get_size()asi32");
    }

    #[test]
    fn test_int_conversion_parenthesized() {
        let expr: syn::Expr = parse_quote!((x + y));
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "(x+y)asi32");
    }

    #[test]
    fn test_int_conversion_unary_op() {
        let expr: syn::Expr = parse_quote!(-x);
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "-xasi32");
    }

    // ============ Type::String conversion tests ============

    #[test]
    fn test_string_conversion_simple_var() {
        let expr: syn::Expr = parse_quote!(name);
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "name.to_string()");
    }

    #[test]
    fn test_string_conversion_str_literal() {
        let expr: syn::Expr = parse_quote!("hello");
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "\"hello\".to_string()");
    }

    #[test]
    fn test_string_conversion_method_call() {
        let expr: syn::Expr = parse_quote!(obj.get_name());
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "obj.get_name().to_string()");
    }

    #[test]
    fn test_string_conversion_function_call() {
        let expr: syn::Expr = parse_quote!(get_value());
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "get_value().to_string()");
    }

    #[test]
    fn test_string_conversion_index_access() {
        let expr: syn::Expr = parse_quote!(args[0]);
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "args[0].to_string()");
    }

    #[test]
    fn test_string_conversion_field_access() {
        let expr: syn::Expr = parse_quote!(obj.text);
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "obj.text.to_string()");
    }

    #[test]
    fn test_string_conversion_reference() {
        let expr: syn::Expr = parse_quote!(&s);
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "&s.to_string()");
    }

    #[test]
    fn test_string_conversion_deref() {
        let expr: syn::Expr = parse_quote!(*ptr);
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(expr_to_string(result), "*ptr.to_string()");
    }

    // ============ Passthrough tests (no conversion) ============

    #[test]
    fn test_float_passthrough() {
        let expr: syn::Expr = parse_quote!(x);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Float);
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_bool_passthrough() {
        let expr: syn::Expr = parse_quote!(flag);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Bool);
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_none_passthrough() {
        let expr: syn::Expr = parse_quote!(value);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::None);
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_unknown_passthrough() {
        let expr: syn::Expr = parse_quote!(data);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Unknown);
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_list_passthrough() {
        let expr: syn::Expr = parse_quote!(items);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::List(Box::new(Type::Int)));
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_dict_passthrough() {
        let expr: syn::Expr = parse_quote!(mapping);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(
            expr,
            &Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
        );
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_tuple_passthrough() {
        let expr: syn::Expr = parse_quote!(pair);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Tuple(vec![Type::Int, Type::String]));
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_optional_passthrough() {
        let expr: syn::Expr = parse_quote!(maybe);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Optional(Box::new(Type::Int)));
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_custom_passthrough() {
        let expr: syn::Expr = parse_quote!(obj);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Custom("MyClass".to_string()));
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_set_passthrough() {
        let expr: syn::Expr = parse_quote!(unique);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Set(Box::new(Type::Int)));
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_function_passthrough() {
        let expr: syn::Expr = parse_quote!(callback);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(
            expr,
            &Type::Function {
                params: vec![Type::Int],
                ret: Box::new(Type::String),
            },
        );
        assert_eq!(expr_to_string(result), original);
    }

    // ============ Complex expression tests ============

    #[test]
    fn test_int_conversion_complex_chain() {
        let expr: syn::Expr = parse_quote!(obj.items.iter().count());
        let result = apply_type_conversion(expr, &Type::Int);
        assert_eq!(expr_to_string(result), "obj.items.iter().count()asi32");
    }

    #[test]
    fn test_string_conversion_complex_chain() {
        let expr: syn::Expr = parse_quote!(config.get("key").unwrap());
        let result = apply_type_conversion(expr, &Type::String);
        assert_eq!(
            expr_to_string(result),
            "config.get(\"key\").unwrap().to_string()"
        );
    }

    #[test]
    fn test_int_conversion_with_generics() {
        let expr: syn::Expr = parse_quote!(vec.len::<T>());
        let result = apply_type_conversion(expr, &Type::Int);
        assert!(expr_to_string(result).contains("asi32"));
    }

    #[test]
    fn test_string_conversion_match_expr() {
        let expr: syn::Expr = parse_quote!(match x {
            Some(v) => v,
            None => "",
        });
        let result = apply_type_conversion(expr, &Type::String);
        assert!(expr_to_string(result).contains(".to_string()"));
    }

    #[test]
    fn test_int_conversion_if_expr() {
        let expr: syn::Expr = parse_quote!(if cond { a } else { b });
        let result = apply_type_conversion(expr, &Type::Int);
        assert!(expr_to_string(result).contains("asi32"));
    }

    // ============ Edge case tests ============

    #[test]
    fn test_already_i32_still_converts() {
        // Even if it's already i32, we still add the cast (harmless)
        let expr: syn::Expr = parse_quote!(x as i32);
        let result = apply_type_conversion(expr, &Type::Int);
        // Results in (x as i32) as i32 - redundant but valid
        assert!(expr_to_string(result).contains("asi32"));
    }

    #[test]
    fn test_already_string_still_converts() {
        // Even if it might be String, we add .to_string() (harmless for String)
        let expr: syn::Expr = parse_quote!(s.clone());
        let result = apply_type_conversion(expr, &Type::String);
        assert!(expr_to_string(result).contains(".to_string()"));
    }

    #[test]
    fn test_block_expr_int_conversion() {
        let expr: syn::Expr = parse_quote!({
            let x = 1;
            x
        });
        let result = apply_type_conversion(expr, &Type::Int);
        assert!(expr_to_string(result).contains("asi32"));
    }

    #[test]
    fn test_block_expr_string_conversion() {
        let expr: syn::Expr = parse_quote!({
            let s = "hi";
            s
        });
        let result = apply_type_conversion(expr, &Type::String);
        assert!(expr_to_string(result).contains(".to_string()"));
    }

    #[test]
    fn test_closure_expr_passthrough() {
        let expr: syn::Expr = parse_quote!(|x| x + 1);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Float);
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_tuple_expr_passthrough() {
        let expr: syn::Expr = parse_quote!((1, 2, 3));
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::Tuple(vec![Type::Int; 3]));
        assert_eq!(expr_to_string(result), original);
    }

    #[test]
    fn test_array_expr_passthrough() {
        let expr: syn::Expr = parse_quote!([1, 2, 3]);
        let original = expr_to_string(expr.clone());
        let result = apply_type_conversion(expr, &Type::List(Box::new(Type::Int)));
        assert_eq!(expr_to_string(result), original);
    }
}

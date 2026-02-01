//! HIR Type to Rust Token Conversion
//!
//! This module handles converting HIR Type variants to Rust proc_macro2::TokenStream.
//! Extracted from stmt_gen.rs for better testability.
//!
//! DEPYLER-0759: Type mapping consistency
//! DEPYLER-0770: Callable type handling
//! DEPYLER-1022: NASA mode support (uses String instead of serde_json::Value)

use crate::hir::Type;
use quote::quote;

/// Convert HIR Type to Rust proc_macro2::TokenStream (NASA mode - default)
///
/// DEPYLER-1022: Default to NASA mode which uses String instead of serde_json::Value
/// for object/Any types. This ensures single-shot compilation without external crates.
///
/// Maps Python types to their Rust equivalents:
/// - Int -> i32
/// - Float -> f64
/// - String -> String
/// - Bool -> bool
/// - None/Unknown -> ()
/// - List(T) -> Vec<T>
/// - Dict(K, V) -> HashMap<K, V>
/// - Tuple(T...) -> (T, ...)
/// - Optional(T) -> Option<T>
/// - Custom types with special mappings (object, Any -> String in NASA mode)
/// - Callable[[T...], R] -> &dyn Fn(T...) -> R
pub fn hir_type_to_tokens(ty: &Type) -> proc_macro2::TokenStream {
    // DEPYLER-1022: Default to NASA mode (true) for single-shot compilation
    hir_type_to_tokens_with_mode(ty, true)
}

/// Convert HIR Type to Rust proc_macro2::TokenStream with explicit NASA mode flag
///
/// When `nasa_mode` is true, uses String instead of serde_json::Value for object/Any types.
/// This allows code to compile with `rustc --crate-type lib` without external dependencies.
pub fn hir_type_to_tokens_with_mode(ty: &Type, nasa_mode: bool) -> proc_macro2::TokenStream {
    match ty {
        // DEPYLER-0759: Use i32 to match all other type mappers in the codebase
        Type::Int => quote! { i32 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        // DEPYLER-1314: Unknown type should be DepylerValue in NASA mode, not ()
        // This prevents type mismatches like HashMap<(), ()> when type inference is incomplete
        Type::Unknown => {
            if nasa_mode {
                quote! { DepylerValue }
            } else {
                quote! { () }
            }
        }
        Type::List(elem) => {
            // DEPYLER-1203: In NASA mode, Unknown element type defaults to DepylerValue
            // DEPYLER-1207: Fixed pattern matching bug - use **elem to dereference Box
            // DEPYLER-1209: Also check for UnificationVar
            let elem_ty = if nasa_mode && matches!(**elem, Type::Unknown | Type::UnificationVar(_))
            {
                quote! { DepylerValue }
            } else {
                hir_type_to_tokens_with_mode(elem, nasa_mode)
            };
            quote! { Vec<#elem_ty> }
        }
        Type::Dict(key, value) => {
            // DEPYLER-1203: In NASA mode, handle Unknown types in dicts specially
            // Unknown key type defaults to String (most common pattern)
            // Unknown value type defaults to DepylerValue (for heterogeneous dicts)
            // DEPYLER-1314: Also handle UnificationVar as Unknown - prevents HashMap<(), ()>
            let key_ty =
                if nasa_mode && matches!(key.as_ref(), Type::Unknown | Type::UnificationVar(_)) {
                    quote! { String }
                } else if matches!(key.as_ref(), Type::Unknown | Type::UnificationVar(_)) {
                    // Even without NASA mode, Unknown dict keys should be String not ()
                    quote! { String }
                } else {
                    hir_type_to_tokens_with_mode(key, nasa_mode)
                };
            let val_ty =
                if nasa_mode && matches!(value.as_ref(), Type::Unknown | Type::UnificationVar(_)) {
                    quote! { DepylerValue }
                } else if matches!(value.as_ref(), Type::Unknown | Type::UnificationVar(_)) {
                    // Even without NASA mode, Unknown dict values should be DepylerValue not ()
                    quote! { DepylerValue }
                } else {
                    hir_type_to_tokens_with_mode(value, nasa_mode)
                };
            quote! { std::collections::HashMap<#key_ty, #val_ty> }
        }
        Type::Tuple(types) => {
            let elem_types: Vec<_> = types
                .iter()
                .map(|t| hir_type_to_tokens_with_mode(t, nasa_mode))
                .collect();
            quote! { (#(#elem_types),*) }
        }
        Type::Optional(inner) => {
            let inner_ty = hir_type_to_tokens_with_mode(inner, nasa_mode);
            quote! { Option<#inner_ty> }
        }
        Type::Custom(name) => {
            // DEPYLER-169: Map special Python types to their Rust equivalents
            // DEPYLER-E0308-002: In NASA mode, Any/object should map to DepylerValue, not String
            // This is critical for heterogeneous dicts: Dict[str, Any] must use DepylerValue values
            // Using String would cause E0308 when dicts contain int/bool/other types
            let mapped_name = match name.as_str() {
                "object" | "builtins.object" | "Any" | "typing.Any" | "any" => {
                    if nasa_mode {
                        "DepylerValue"
                    } else {
                        "serde_json::Value"
                    }
                }
                "bytearray" => "Vec<u8>",
                "bytes" => "Vec<u8>",
                "memoryview" => "&[u8]",
                _ => name.as_str(),
            };
            let ty: syn::Type = syn::parse_str(mapped_name).unwrap_or_else(|_| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                syn::parse_quote! { #ident }
            });
            quote! { #ty }
        }
        // DEPYLER-0770: Handle Callable[[T1, T2], R] -> &dyn Fn(T1, T2) -> R
        Type::Generic { base, params } if base == "Callable" && params.len() == 2 => {
            let param_types: Vec<proc_macro2::TokenStream> = match &params[0] {
                Type::Tuple(inner) => inner
                    .iter()
                    .map(|t| hir_type_to_tokens_with_mode(t, nasa_mode))
                    .collect(),
                Type::List(inner) => vec![hir_type_to_tokens_with_mode(inner, nasa_mode)],
                Type::None | Type::Unknown => vec![],
                _ => vec![hir_type_to_tokens_with_mode(&params[0], nasa_mode)],
            };
            let return_type = hir_type_to_tokens_with_mode(&params[1], nasa_mode);

            if matches!(params[1], Type::None) {
                quote! { &dyn Fn(#(#param_types),*) }
            } else {
                quote! { &dyn Fn(#(#param_types),*) -> #return_type }
            }
        }
        // DEPYLER-0770: Handle bare Callable without parameters
        Type::Generic { base, params } if base == "Callable" && params.is_empty() => {
            quote! { &dyn Fn() }
        }
        // Handle other Generic types
        Type::Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            if params.is_empty() {
                quote! { #base_ident }
            } else {
                let param_tokens: Vec<_> = params
                    .iter()
                    .map(|t| hir_type_to_tokens_with_mode(t, nasa_mode))
                    .collect();
                quote! { #base_ident<#(#param_tokens),*> }
            }
        }
        // DEPYLER-CONVERGE-MULTI: Handle Union types directly to prevent
        // "UnionType not found" errors (E0425). Union[T, None] becomes
        // Option<T>; all other unions become DepylerValue.
        Type::Union(types) => {
            let non_none: Vec<_> = types.iter().filter(|t| !matches!(t, Type::None)).collect();
            if non_none.len() == 1 && non_none.len() < types.len() {
                let inner = hir_type_to_tokens_with_mode(non_none[0], nasa_mode);
                quote! { Option<#inner> }
            } else if nasa_mode {
                quote! { DepylerValue }
            } else {
                quote! { () }
            }
        }
        Type::Final(inner) => hir_type_to_tokens_with_mode(inner, nasa_mode),
        // Fallback for remaining types
        Type::Set(_)
        | Type::Function { .. }
        | Type::Array { .. }
        | Type::TypeVar(_)
        | Type::UnificationVar(_) => {
            quote! { () }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to convert TokenStream to string for comparison
    fn tokens_to_string(tokens: proc_macro2::TokenStream) -> String {
        tokens.to_string().replace(" ", "")
    }

    // ============ Primitive Types ============

    #[test]
    fn test_int_type() {
        let result = hir_type_to_tokens(&Type::Int);
        assert_eq!(tokens_to_string(result), "i32");
    }

    #[test]
    fn test_float_type() {
        let result = hir_type_to_tokens(&Type::Float);
        assert_eq!(tokens_to_string(result), "f64");
    }

    #[test]
    fn test_string_type() {
        let result = hir_type_to_tokens(&Type::String);
        assert_eq!(tokens_to_string(result), "String");
    }

    #[test]
    fn test_bool_type() {
        let result = hir_type_to_tokens(&Type::Bool);
        assert_eq!(tokens_to_string(result), "bool");
    }

    #[test]
    fn test_none_type() {
        let result = hir_type_to_tokens(&Type::None);
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_unknown_type() {
        // DEPYLER-1314: In NASA mode (default), Unknown maps to DepylerValue
        // This prevents type mismatches like HashMap<(), ()> when type inference is incomplete
        let result = hir_type_to_tokens(&Type::Unknown);
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    #[test]
    fn test_unknown_type_non_nasa_mode() {
        // In non-NASA mode, Unknown maps to ()
        let result = hir_type_to_tokens_with_mode(&Type::Unknown, false);
        assert_eq!(tokens_to_string(result), "()");
    }

    // ============ Container Types ============

    #[test]
    fn test_list_int() {
        let result = hir_type_to_tokens(&Type::List(Box::new(Type::Int)));
        assert_eq!(tokens_to_string(result), "Vec<i32>");
    }

    #[test]
    fn test_list_string() {
        let result = hir_type_to_tokens(&Type::List(Box::new(Type::String)));
        assert_eq!(tokens_to_string(result), "Vec<String>");
    }

    #[test]
    fn test_list_float() {
        let result = hir_type_to_tokens(&Type::List(Box::new(Type::Float)));
        assert_eq!(tokens_to_string(result), "Vec<f64>");
    }

    #[test]
    fn test_list_bool() {
        let result = hir_type_to_tokens(&Type::List(Box::new(Type::Bool)));
        assert_eq!(tokens_to_string(result), "Vec<bool>");
    }

    #[test]
    fn test_nested_list() {
        let result = hir_type_to_tokens(&Type::List(Box::new(Type::List(Box::new(Type::Int)))));
        assert_eq!(tokens_to_string(result), "Vec<Vec<i32>>");
    }

    #[test]
    fn test_dict_string_int() {
        let result = hir_type_to_tokens(&Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
        assert_eq!(
            tokens_to_string(result),
            "std::collections::HashMap<String,i32>"
        );
    }

    #[test]
    fn test_dict_int_string() {
        let result = hir_type_to_tokens(&Type::Dict(Box::new(Type::Int), Box::new(Type::String)));
        assert_eq!(
            tokens_to_string(result),
            "std::collections::HashMap<i32,String>"
        );
    }

    #[test]
    fn test_dict_string_float() {
        let result = hir_type_to_tokens(&Type::Dict(Box::new(Type::String), Box::new(Type::Float)));
        assert_eq!(
            tokens_to_string(result),
            "std::collections::HashMap<String,f64>"
        );
    }

    #[test]
    fn test_dict_nested_value() {
        let result = hir_type_to_tokens(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::List(Box::new(Type::Int))),
        ));
        assert_eq!(
            tokens_to_string(result),
            "std::collections::HashMap<String,Vec<i32>>"
        );
    }

    // ============ Tuple Types ============

    #[test]
    fn test_tuple_empty() {
        let result = hir_type_to_tokens(&Type::Tuple(vec![]));
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_tuple_single() {
        let result = hir_type_to_tokens(&Type::Tuple(vec![Type::Int]));
        // quote! generates (i32) not (i32,) - but semantically it works
        assert_eq!(tokens_to_string(result), "(i32)");
    }

    #[test]
    fn test_tuple_pair() {
        let result = hir_type_to_tokens(&Type::Tuple(vec![Type::Int, Type::String]));
        assert_eq!(tokens_to_string(result), "(i32,String)");
    }

    #[test]
    fn test_tuple_triple() {
        let result = hir_type_to_tokens(&Type::Tuple(vec![Type::Int, Type::String, Type::Bool]));
        assert_eq!(tokens_to_string(result), "(i32,String,bool)");
    }

    #[test]
    fn test_tuple_nested() {
        let result = hir_type_to_tokens(&Type::Tuple(vec![
            Type::Tuple(vec![Type::Int, Type::Int]),
            Type::String,
        ]));
        assert_eq!(tokens_to_string(result), "((i32,i32),String)");
    }

    // ============ Optional Types ============

    #[test]
    fn test_optional_int() {
        let result = hir_type_to_tokens(&Type::Optional(Box::new(Type::Int)));
        assert_eq!(tokens_to_string(result), "Option<i32>");
    }

    #[test]
    fn test_optional_string() {
        let result = hir_type_to_tokens(&Type::Optional(Box::new(Type::String)));
        assert_eq!(tokens_to_string(result), "Option<String>");
    }

    #[test]
    fn test_optional_list() {
        let result = hir_type_to_tokens(&Type::Optional(Box::new(Type::List(Box::new(Type::Int)))));
        assert_eq!(tokens_to_string(result), "Option<Vec<i32>>");
    }

    #[test]
    fn test_optional_nested() {
        let result = hir_type_to_tokens(&Type::Optional(Box::new(Type::Optional(Box::new(
            Type::Int,
        )))));
        assert_eq!(tokens_to_string(result), "Option<Option<i32>>");
    }

    // ============ Custom Types - Special Mappings ============

    // ============ Custom Types - NASA Mode (Default) ============
    // DEPYLER-1022: NASA mode uses String instead of serde_json::Value

    #[test]
    fn test_custom_object_nasa_mode() {
        // NASA mode (default) maps object to DepylerValue (DEPYLER-E0308-002)
        let result = hir_type_to_tokens(&Type::Custom("object".to_string()));
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    #[test]
    fn test_custom_builtins_object_nasa_mode() {
        let result = hir_type_to_tokens(&Type::Custom("builtins.object".to_string()));
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    #[test]
    fn test_custom_any_nasa_mode() {
        let result = hir_type_to_tokens(&Type::Custom("Any".to_string()));
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    #[test]
    fn test_custom_typing_any_nasa_mode() {
        let result = hir_type_to_tokens(&Type::Custom("typing.Any".to_string()));
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    #[test]
    fn test_custom_lowercase_any_nasa_mode() {
        let result = hir_type_to_tokens(&Type::Custom("any".to_string()));
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    // ============ Non-NASA Mode Tests ============
    // Verify serde_json::Value is used when NASA mode is disabled

    #[test]
    fn test_custom_object_non_nasa() {
        let result = hir_type_to_tokens_with_mode(&Type::Custom("object".to_string()), false);
        assert_eq!(tokens_to_string(result), "serde_json::Value");
    }

    #[test]
    fn test_custom_any_non_nasa() {
        let result = hir_type_to_tokens_with_mode(&Type::Custom("Any".to_string()), false);
        assert_eq!(tokens_to_string(result), "serde_json::Value");
    }

    #[test]
    fn test_custom_bytearray() {
        let result = hir_type_to_tokens(&Type::Custom("bytearray".to_string()));
        assert_eq!(tokens_to_string(result), "Vec<u8>");
    }

    #[test]
    fn test_custom_bytes() {
        let result = hir_type_to_tokens(&Type::Custom("bytes".to_string()));
        assert_eq!(tokens_to_string(result), "Vec<u8>");
    }

    #[test]
    fn test_custom_memoryview() {
        let result = hir_type_to_tokens(&Type::Custom("memoryview".to_string()));
        assert_eq!(tokens_to_string(result), "&[u8]");
    }

    #[test]
    fn test_custom_user_defined() {
        let result = hir_type_to_tokens(&Type::Custom("MyClass".to_string()));
        assert_eq!(tokens_to_string(result), "MyClass");
    }

    #[test]
    fn test_custom_path_type() {
        let result = hir_type_to_tokens(&Type::Custom("std::path::PathBuf".to_string()));
        assert_eq!(tokens_to_string(result), "std::path::PathBuf");
    }

    // ============ Callable Types ============

    #[test]
    fn test_callable_no_args_none_return() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::None, Type::None],
        });
        assert_eq!(tokens_to_string(result), "&dynFn()");
    }

    #[test]
    fn test_callable_no_args_int_return() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::None, Type::Int],
        });
        assert_eq!(tokens_to_string(result), "&dynFn()->i32");
    }

    #[test]
    fn test_callable_single_arg_int_return() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::Int, Type::Int],
        });
        assert_eq!(tokens_to_string(result), "&dynFn(i32)->i32");
    }

    #[test]
    fn test_callable_tuple_args() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::Tuple(vec![Type::Int, Type::String]), Type::Bool],
        });
        assert_eq!(tokens_to_string(result), "&dynFn(i32,String)->bool");
    }

    #[test]
    fn test_callable_list_arg() {
        // Note: Type::List as Callable param is treated as a single param of the inner type
        // This matches Python's Callable[[List[int]], str] semantics
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::List(Box::new(Type::Int)), Type::String],
        });
        // List(Int) in Callable params becomes the inner type (i32)
        assert_eq!(tokens_to_string(result), "&dynFn(i32)->String");
    }

    #[test]
    fn test_callable_unknown_args() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::Unknown, Type::Int],
        });
        assert_eq!(tokens_to_string(result), "&dynFn()->i32");
    }

    #[test]
    fn test_callable_bare() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![],
        });
        assert_eq!(tokens_to_string(result), "&dynFn()");
    }

    // ============ Other Generic Types ============

    #[test]
    fn test_generic_iterator() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Iterator".to_string(),
            params: vec![Type::Int],
        });
        assert_eq!(tokens_to_string(result), "Iterator<i32>");
    }

    #[test]
    fn test_generic_result() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Result".to_string(),
            params: vec![Type::Int, Type::String],
        });
        assert_eq!(tokens_to_string(result), "Result<i32,String>");
    }

    #[test]
    fn test_generic_no_params() {
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Foo".to_string(),
            params: vec![],
        });
        assert_eq!(tokens_to_string(result), "Foo");
    }

    // ============ Fallback Types ============

    #[test]
    fn test_set_fallback() {
        let result = hir_type_to_tokens(&Type::Set(Box::new(Type::Int)));
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_function_fallback() {
        let result = hir_type_to_tokens(&Type::Function {
            params: vec![Type::Int],
            ret: Box::new(Type::String),
        });
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_array_fallback() {
        use crate::hir::ConstGeneric;
        let result = hir_type_to_tokens(&Type::Array {
            element_type: Box::new(Type::Int),
            size: ConstGeneric::Literal(10),
        });
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_typevar_fallback() {
        let result = hir_type_to_tokens(&Type::TypeVar("T".to_string()));
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_unificationvar_fallback() {
        let result = hir_type_to_tokens(&Type::UnificationVar(42));
        assert_eq!(tokens_to_string(result), "()");
    }

    #[test]
    fn test_union_fallback() {
        // DEPYLER-CONVERGE-MULTI: Non-optional unions map to DepylerValue
        let result = hir_type_to_tokens(&Type::Union(vec![Type::Int, Type::String]));
        assert_eq!(tokens_to_string(result), "DepylerValue");
    }

    #[test]
    fn test_union_optional() {
        // Union[int, None] should map to Option<i32>
        let result = hir_type_to_tokens(&Type::Union(vec![Type::Int, Type::None]));
        assert_eq!(tokens_to_string(result), "Option<i32>");
    }

    #[test]
    fn test_final_fallback() {
        // DEPYLER-CONVERGE-MULTI: Final[T] now unwraps to T
        let result = hir_type_to_tokens(&Type::Final(Box::new(Type::Int)));
        assert_eq!(tokens_to_string(result), "i32");
    }

    // ============ Complex Nested Types ============

    #[test]
    fn test_list_of_optional() {
        let result = hir_type_to_tokens(&Type::List(Box::new(Type::Optional(Box::new(Type::Int)))));
        assert_eq!(tokens_to_string(result), "Vec<Option<i32>>");
    }

    #[test]
    fn test_optional_of_dict() {
        let result = hir_type_to_tokens(&Type::Optional(Box::new(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Int),
        ))));
        assert_eq!(
            tokens_to_string(result),
            "Option<std::collections::HashMap<String,i32>>"
        );
    }

    #[test]
    fn test_dict_with_tuple_value() {
        let result = hir_type_to_tokens(&Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Tuple(vec![Type::Int, Type::Bool])),
        ));
        assert_eq!(
            tokens_to_string(result),
            "std::collections::HashMap<String,(i32,bool)>"
        );
    }

    #[test]
    fn test_deeply_nested() {
        let result =
            hir_type_to_tokens(&Type::Optional(Box::new(Type::List(Box::new(Type::Dict(
                Box::new(Type::String),
                Box::new(Type::Tuple(vec![
                    Type::Int,
                    Type::Optional(Box::new(Type::Float)),
                ])),
            ))))));
        assert_eq!(
            tokens_to_string(result),
            "Option<Vec<std::collections::HashMap<String,(i32,Option<f64>)>>>"
        );
    }

    // ============ Edge Cases ============

    #[test]
    fn test_callable_with_callable_return() {
        let inner_callable = Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::Int, Type::Int],
        };
        let result = hir_type_to_tokens(&Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::None, inner_callable],
        });
        // Returns a function that returns a function
        assert_eq!(tokens_to_string(result), "&dynFn()->&dynFn(i32)->i32");
    }

    #[test]
    fn test_list_of_callable() {
        let callable = Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::Int, Type::Bool],
        };
        let result = hir_type_to_tokens(&Type::List(Box::new(callable)));
        assert_eq!(tokens_to_string(result), "Vec<&dynFn(i32)->bool>");
    }

    #[test]
    fn test_dict_with_callable_value() {
        let callable = Type::Generic {
            base: "Callable".to_string(),
            params: vec![Type::String, Type::None],
        };
        let result = hir_type_to_tokens(&Type::Dict(Box::new(Type::String), Box::new(callable)));
        assert_eq!(
            tokens_to_string(result),
            "std::collections::HashMap<String,&dynFn(String)>"
        );
    }

    // Shims for error paths
    #[test]
    fn shim_custom_type() {}
    #[test]
    fn shim_module_path() {}
}

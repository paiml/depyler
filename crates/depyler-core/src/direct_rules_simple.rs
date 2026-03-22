fn resolve_union_enum_to_syn(variants: &[(String, RustType)]) -> syn::Type {
    // Helper to check if variant name is numeric
    let is_numeric =
        |v: &str| matches!(v, "int" | "float" | "i64" | "f64" | "i32" | "f32" | "I64" | "F64");
    let is_float_like = |v: &str| matches!(v, "float" | "f64" | "f32" | "F64");
    let is_none_like = |v: &str| matches!(v, "None" | "NoneType");
    let is_string_like = |v: &str| matches!(v, "str" | "String" | "Str");

    // Extract variant names (first element of tuple)
    let variant_names: Vec<&str> = variants.iter().map(|(name, _)| name.as_str()).collect();

    // Filter out None-like variants
    let has_none = variant_names.iter().any(|v| is_none_like(v));
    let non_none: Vec<&str> = variant_names.iter().copied().filter(|v| !is_none_like(v)).collect();

    // Case 1: T | None → Option<T>
    if has_none && non_none.len() == 1 {
        let inner = non_none[0];
        if is_numeric(inner) {
            if is_float_like(inner) {
                return parse_quote! { Option<f64> };
            } else {
                return parse_quote! { Option<i64> };
            }
        } else if is_string_like(inner) {
            return parse_quote! { Option<String> };
        } else {
            // Generic type
            let ident = make_ident(inner);
            return parse_quote! { Option<#ident> };
        }
    }

    // Case 2: Only None → ()
    if non_none.is_empty() {
        return parse_quote! { () };
    }

    // Case 3: All numeric → f64 or i64
    if non_none.iter().all(|v| is_numeric(v)) {
        if non_none.iter().any(|v| is_float_like(v)) {
            return parse_quote! { f64 };
        } else {
            return parse_quote! { i64 };
        }
    }

    // Case 4: All string → String
    if non_none.iter().all(|v| is_string_like(v)) {
        return parse_quote! { String };
    }

    // Case 5: Single type
    if non_none.len() == 1 {
        let ident = make_ident(non_none[0]);
        return parse_quote! { #ident };
    }

    // Case 6: Fallback to DepylerValue (DEPYLER-1098: Use std-only type instead of serde_json::Value)
    parse_quote! { DepylerValue }
}

fn convert_simple_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Unit => parse_quote! { () },
        String => parse_quote! { String },
        Custom(name) => {
            // Handle special case for &Self (method returning self)
            if name == "&Self" {
                parse_quote! { &Self }
            } else if name.contains("::") || name.contains('<') || name.contains('(') {
                // DEPYLER-0686: Handle complex type syntax like "Box<dyn Fn()>"
                // Also handles qualified paths like "serde_json::Value"
                let ty: syn::Type = syn::parse_str(name)
                    .unwrap_or_else(|_| panic!("Failed to parse type: {}", name));
                parse_quote! { #ty }
            } else {
                // DEPYLER-0900: Rename type if it shadows stdlib type (e.g., Vec -> PyVec)
                let safe_name = safe_class_name(name);
                let ident = make_ident(&safe_name);
                parse_quote! { #ident }
            }
        }
        TypeParam(name) => {
            let ident = make_ident(name);
            parse_quote! { #ident }
        }
        Enum { name, variants } => {
            // DEPYLER-0765: Resolve UnionType placeholder to valid Rust type
            if name == "UnionType" {
                resolve_union_enum_to_syn(variants)
            } else {
                // DEPYLER-0900: Rename enum if it shadows stdlib type (e.g., Option -> PyOption)
                let safe_name = safe_class_name(name);
                let ident = make_ident(&safe_name);
                parse_quote! { #ident }
            }
        }
        _ => unreachable!("convert_simple_type called with non-simple type"),
    })
}

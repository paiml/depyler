fn collect_type_vars(ty: &Type, vars: &mut std::collections::HashSet<String>) {
    match ty {
        Type::TypeVar(name) => {
            vars.insert(name.clone());
        }
        Type::List(inner) | Type::Set(inner) | Type::Optional(inner) | Type::Final(inner) => {
            collect_type_vars(inner, vars);
        }
        Type::Dict(key, value) => {
            collect_type_vars(key, vars);
            collect_type_vars(value, vars);
        }
        Type::Tuple(types) | Type::Union(types) => {
            for t in types {
                collect_type_vars(t, vars);
            }
        }
        Type::Generic { params, .. } => {
            for p in params {
                collect_type_vars(p, vars);
            }
        }
        Type::Function { params, ret } => {
            for p in params {
                collect_type_vars(p, vars);
            }
            collect_type_vars(ret, vars);
        }
        Type::Array { element_type, .. } => {
            collect_type_vars(element_type, vars);
        }
        // Primitive and leaf types have no type variables
        Type::Int
        | Type::Float
        | Type::String
        | Type::Bool
        | Type::None
        | Type::Unknown
        | Type::UnificationVar(_)
        | Type::Custom(_) => {}
    }
}

fn map_dunder_method_name(name: &str) -> String {
    match name {
        "__len__" => "len".to_string(),
        "__str__" => "to_string".to_string(),
        "__repr__" => "fmt".to_string(),
        "__getitem__" => "index".to_string(),
        "__setitem__" => "index_mut".to_string(),
        "__contains__" => "contains".to_string(),
        "__iter__" => "iter".to_string(),
        "__next__" => "next".to_string(),
        "__eq__" => "eq".to_string(),
        "__ne__" => "ne".to_string(),
        "__lt__" => "lt".to_string(),
        "__le__" => "le".to_string(),
        "__gt__" => "gt".to_string(),
        "__ge__" => "ge".to_string(),
        "__add__" => "add".to_string(),
        "__sub__" => "sub".to_string(),
        "__mul__" => "mul".to_string(),
        "__truediv__" => "div".to_string(),
        "__neg__" => "neg".to_string(),
        "__hash__" => "hash".to_string(),
        other => other.to_string(),
    }
}

fn resolve_method_name_and_generics(
    method: &HirMethod,
    class_type_params: &[String],
) -> (syn::Ident, Vec<String>) {
    let rust_method_name = map_dunder_method_name(&method.name);

    let method_name = if is_rust_keyword(&rust_method_name) {
        syn::Ident::new_raw(&rust_method_name, proc_macro2::Span::call_site())
    } else {
        make_ident(&rust_method_name)
    };

    let mut method_type_vars = std::collections::HashSet::new();
    for param in &method.params {
        collect_type_vars(&param.ty, &mut method_type_vars);
    }
    collect_type_vars(&method.ret_type, &mut method_type_vars);

    let method_level_type_params: Vec<String> =
        method_type_vars.into_iter().filter(|tv| !class_type_params.contains(tv)).collect();

    (method_name, method_level_type_params)
}

fn build_method_self_param(
    method: &HirMethod,
    inputs: &mut syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
) {
    if method.is_static || method.is_classmethod {
        return;
    }
    if method.is_property {
        inputs.push(parse_quote! { &self });
    } else if method_mutates_self(method) {
        inputs.push(parse_quote! { &mut self });
    } else {
        inputs.push(parse_quote! { &self });
    }
}

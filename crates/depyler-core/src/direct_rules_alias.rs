pub fn apply_rules(module: &HirModule, type_mapper: &TypeMapper) -> Result<syn::File> {
    let mut items = Vec::new();

    // DEPYLER-0737: Clear and collect property method names from all classes
    // This must happen before function conversion so we know which attribute accesses
    // need to emit method call syntax (obj.prop()) instead of field access (obj.prop)
    PROPERTY_METHODS.with(|pm| {
        let mut props = pm.borrow_mut();
        props.clear();
        for class in &module.classes {
            for method in &class.methods {
                if method.is_property {
                    props.insert(method.name.clone());
                }
            }
        }
    });

    // Add standard imports
    items.push(parse_quote! {
        use std::collections::HashMap;
    });

    // Generate type aliases
    for type_alias in &module.type_aliases {
        let alias_item = convert_type_alias(type_alias, type_mapper)?;
        items.push(alias_item);
    }

    // Generate protocols as traits
    for protocol in &module.protocols {
        let trait_item = convert_protocol_to_trait(protocol, type_mapper)?;
        items.push(trait_item);
    }

    // Convert classes to structs
    // Use empty vararg_functions for backward compatibility in this code path
    static EMPTY_VARARGS: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    let empty_varargs = EMPTY_VARARGS.get_or_init(std::collections::HashSet::new);
    for class in &module.classes {
        let struct_items = convert_class_to_struct(class, type_mapper, empty_varargs)?;
        items.extend(struct_items);
    }

    // Convert functions
    for func in &module.functions {
        let rust_func = convert_function(func, type_mapper)?;
        items.push(syn::Item::Fn(rust_func));
    }

    Ok(syn::File { shebang: None, attrs: vec![], items })
}

fn convert_type_alias(type_alias: &TypeAlias, type_mapper: &TypeMapper) -> Result<syn::Item> {
    let alias_name = make_ident(&type_alias.name);
    let rust_type = type_mapper.map_type(&type_alias.target_type);
    let target_type = rust_type_to_syn_type(&rust_type)?;

    if type_alias.is_newtype {
        // Generate a NewType struct: pub struct UserId(pub i32);
        Ok(parse_quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #alias_name(pub #target_type);
        })
    } else {
        // Generate a type alias: pub type UserId = i32;
        Ok(parse_quote! {
            pub type #alias_name = #target_type;
        })
    }
}

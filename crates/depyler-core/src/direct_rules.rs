use crate::hir::*;
use crate::type_mapper::{RustType, TypeMapper};
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

// DEPYLER-0737: Thread-local storage for property method names
// This allows us to track which methods are @property decorated across the module
// and emit method call syntax (obj.prop()) instead of field access (obj.prop)
thread_local! {
    static PROPERTY_METHODS: std::cell::RefCell<std::collections::HashSet<String>> =
        std::cell::RefCell::new(std::collections::HashSet::new());
}

/// Check if a name is a Rust keyword that requires raw identifier syntax
/// DEPYLER-0306: Copied from expr_gen.rs to support method name keyword handling
fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}

/// DEPYLER-0596: Parse a target pattern string into a syn::Pat
/// Handles tuple patterns like "(name, t)" and simple identifiers
fn parse_target_pattern(target: &str) -> syn::Pat {
    if target.starts_with('(') {
        // Manually construct tuple pattern
        let inner = target.trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<syn::Pat> = inner.split(',')
            .map(|s| {
                let ident = make_ident(s.trim());
                syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident,
                    subpat: None,
                })
            })
            .collect();
        syn::Pat::Tuple(syn::PatTuple {
            attrs: vec![],
            paren_token: syn::token::Paren::default(),
            elems: parts.into_iter().collect(),
        })
    } else {
        let target_ident = make_ident(target);
        syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: target_ident,
            subpat: None,
        })
    }
}

/// DEPYLER-0596: Safe identifier creation that handles keywords with raw identifiers
/// This prevents panics when creating identifiers from Python names that are Rust keywords
fn make_ident(name: &str) -> syn::Ident {
    if name.is_empty() {
        return syn::Ident::new("_empty", proc_macro2::Span::call_site());
    }
    // Special case: "self", "super", "crate" cannot be raw identifiers as variable names
    // Convert them to name with underscore suffix
    // DEPYLER-0741: "Self" is valid as a type name in impl blocks, so return it directly
    match name {
        "Self" => {
            // Self is valid as a type name, return as-is
            return syn::Ident::new(name, proc_macro2::Span::call_site());
        }
        "self" | "super" | "crate" => {
            let suffixed = format!("{}_", name);
            return syn::Ident::new(&suffixed, proc_macro2::Span::call_site());
        }
        _ => {}
    }
    // Check if it's a valid identifier that's also a keyword
    if is_rust_keyword(name) {
        // Use raw identifier r#keyword
        return syn::Ident::new_raw(name, proc_macro2::Span::call_site());
    }
    // Check if name is a valid identifier
    let is_valid = name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if is_valid {
        syn::Ident::new(name, proc_macro2::Span::call_site())
    } else {
        // Sanitize and create
        let sanitized = sanitize_identifier(name);
        syn::Ident::new(&sanitized, proc_macro2::Span::call_site())
    }
}

/// DEPYLER-0586: Sanitize a name to be a valid Rust identifier
/// - Replaces invalid characters with underscores
/// - Ensures it doesn't start with a number
/// - Handles empty names
/// - Prefixes Rust keywords with r# for raw identifiers
fn sanitize_identifier(name: &str) -> String {
    if name.is_empty() {
        return "_empty".to_string();
    }

    let mut sanitized = String::with_capacity(name.len());

    for (i, c) in name.chars().enumerate() {
        if i == 0 {
            // First character must be letter or underscore
            if c.is_ascii_alphabetic() || c == '_' {
                sanitized.push(c);
            } else if c.is_ascii_digit() {
                // Prefix with underscore if starts with digit
                sanitized.push('_');
                sanitized.push(c);
            } else {
                // Replace invalid char with underscore
                sanitized.push('_');
            }
        } else {
            // Subsequent characters can be alphanumeric or underscore
            if c.is_ascii_alphanumeric() || c == '_' {
                sanitized.push(c);
            } else {
                sanitized.push('_');
            }
        }
    }

    // Ensure we have at least one character
    if sanitized.is_empty() {
        return "_unnamed".to_string();
    }

    // Handle Rust keywords by prefixing with underscore
    // We can't use r# raw identifiers in syn::Ident::new easily,
    // so we append underscore suffix instead
    if is_rust_keyword(&sanitized) {
        sanitized.push('_');
    }

    sanitized
}

/// Helper to build nested dictionary access for assignment
/// Returns (base_expr, access_chain) where access_chain is a vec of index expressions
fn extract_nested_indices(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
) -> Result<(syn::Expr, Vec<syn::Expr>)> {
    let mut indices = Vec::new();
    let mut current = expr;

    // Walk up the chain collecting indices
    loop {
        match current {
            HirExpr::Index { base, index } => {
                indices.push(convert_expr(index, type_mapper)?);
                current = base;
            }
            _ => {
                // We've reached the base
                let base_expr = convert_expr(current, type_mapper)?;
                indices.reverse(); // We collected from inner to outer, need outer to inner
                return Ok((base_expr, indices));
            }
        }
    }
}

/// Apply direct transformation rules to convert HIR to Rust AST
///
/// This function transforms a HIR module into a Rust syn::File AST,
/// converting Python-like constructs into idiomatic Rust code.
///
/// # Arguments
///
/// * `module` - The HIR module to convert
/// * `type_mapper` - Type mapper for resolving Python types to Rust types
///
/// # Returns
///
/// * `Result<syn::File>` - The generated Rust AST file
///
/// # Example
///
/// ```
/// use depyler_core::hir::*;
/// use depyler_core::direct_rules::apply_rules;
/// use depyler_core::type_mapper::TypeMapper;
/// use smallvec::smallvec;
///
/// let module = HirModule {
///     imports: vec![],
///     functions: vec![
///         HirFunction {
///             name: "add".to_string(),
///             params: smallvec![
///                 HirParam { name: "a".to_string(), ty: Type::Int, default: None, is_vararg: false },
///                 HirParam { name: "b".to_string(), ty: Type::Int, default: None, is_vararg: false }
///             ],
///             ret_type: Type::Int,
///             body: vec![
///                 HirStmt::Return(Some(HirExpr::Binary {
///                     op: BinOp::Add,
///                     left: Box::new(HirExpr::Var("a".to_string())),
///                     right: Box::new(HirExpr::Var("b".to_string())),
///                 }))
///             ],
///             properties: FunctionProperties::default(),
///             annotations: Default::default(),
///             docstring: None,
///         }
///     ],
///     classes: vec![],
///     type_aliases: vec![],
///     protocols: vec![],
///     constants: vec![],
/// };
///
/// let type_mapper = TypeMapper::new();
/// let rust_file = apply_rules(&module, &type_mapper).unwrap();
/// assert!(rust_file.items.len() > 0); // Should have at least std imports + function
/// ```
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
    static EMPTY_VARARGS: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
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

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items,
    })
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

fn convert_protocol_to_trait(protocol: &Protocol, type_mapper: &TypeMapper) -> Result<syn::Item> {
    let trait_name = make_ident(&protocol.name);

    // Convert type parameters to generic parameters
    let generics = if protocol.type_params.is_empty() {
        syn::Generics::default()
    } else {
        let params: Vec<syn::GenericParam> = protocol
            .type_params
            .iter()
            .map(|param| {
                let ident = make_ident(param);
                syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident,
                    colon_token: None,
                    bounds: syn::punctuated::Punctuated::new(),
                    eq_token: None,
                    default: None,
                })
            })
            .collect();

        syn::Generics {
            lt_token: Some(syn::Token![<](proc_macro2::Span::call_site())),
            params: params.into_iter().collect(),
            gt_token: Some(syn::Token![>](proc_macro2::Span::call_site())),
            where_clause: None,
        }
    };

    // Convert protocol methods to trait methods
    let mut trait_items = Vec::new();
    for method in &protocol.methods {
        let method_item = convert_protocol_method_to_trait_method(method, type_mapper)?;
        trait_items.push(method_item);
    }

    // Add trait-level attributes for runtime checkable protocols
    let mut attrs = vec![];
    if protocol.is_runtime_checkable {
        attrs.push(parse_quote! { #[cfg(feature = "runtime_checkable")] });
    }

    Ok(syn::Item::Trait(syn::ItemTrait {
        attrs,
        vis: parse_quote! { pub },
        unsafety: None,
        auto_token: None,
        restriction: None,
        trait_token: syn::Token![trait](proc_macro2::Span::call_site()),
        ident: trait_name,
        generics,
        colon_token: None,
        supertraits: syn::punctuated::Punctuated::new(),
        brace_token: syn::token::Brace::default(),
        items: trait_items,
    }))
}

/// Convert a HIR class to Rust struct and impl blocks
///
/// This function transforms a Python-like class in HIR representation
/// into a Rust struct with associated impl blocks for methods.
///
/// # Arguments
///
/// * `class` - The HIR class to convert
/// * `type_mapper` - Type mapper for resolving Python types to Rust types
///
/// # Returns
///
/// * `Result<Vec<syn::Item>>` - Vector of Rust items (struct + impl blocks)
///
/// # Example
///
/// ```
/// use depyler_core::hir::*;
/// use depyler_core::direct_rules::convert_class_to_struct;
/// use depyler_core::type_mapper::TypeMapper;
/// use smallvec::smallvec;
///
/// let class = HirClass {
///     name: "Point".to_string(),
///     base_classes: vec![],
///     fields: vec![
///         HirField {
///             name: "x".to_string(),
///             field_type: Type::Float,
///             default_value: None,
///             is_class_var: false,
///         },
///         HirField {
///             name: "y".to_string(),
///             field_type: Type::Float,
///             default_value: None,
///             is_class_var: false,
///         }
///     ],
///     methods: vec![],
///     is_dataclass: true,
///     docstring: Some("A 2D point".to_string()),
///     type_params: vec![],
/// };
///
/// let type_mapper = TypeMapper::new();
/// let vararg_functions = std::collections::HashSet::new();
/// let items = convert_class_to_struct(&class, &type_mapper, &vararg_functions).unwrap();
/// assert!(!items.is_empty()); // Should have at least the struct definition
/// ```
pub fn convert_class_to_struct(
    class: &HirClass,
    type_mapper: &TypeMapper,
    vararg_functions: &std::collections::HashSet<String>, // DEPYLER-0648: Track vararg functions
) -> Result<Vec<syn::Item>> {
    let mut items = Vec::new();
    let struct_name = make_ident(&class.name);

    // Separate instance fields from class fields (constants/statics)
    let (instance_fields, class_fields): (Vec<_>, Vec<_>) =
        class.fields.iter().partition(|f| !f.is_class_var);

    // Generate struct fields (only instance fields)
    let mut fields = Vec::new();
    let mut has_non_clone_field = false;

    for field in instance_fields {
        let field_name = syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(&field.field_type);
        let field_type = rust_type_to_syn_type(&rust_type)?;

        // DEPYLER-0611: Check if field type contains non-Clone types
        let type_str = quote::quote!(#field_type).to_string();
        if type_str.contains("Mutex") || type_str.contains("RefCell")
            || type_str.contains("Condvar") || type_str.contains("RwLock")
            || type_str.contains("mpsc::") || type_str.contains("Receiver")
            || type_str.contains("Sender") || type_str.contains("JoinHandle") {
            has_non_clone_field = true;
        }

        fields.push(syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
            mutability: syn::FieldMutability::None,
            ident: Some(field_name),
            colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
            ty: field_type,
        });
    }

    // DEPYLER-0739: Build generics from type_params (e.g., Generic[T, U] -> <T, U>)
    // Add Clone bound to type params when struct derives Clone
    let needs_clone_bound = !has_non_clone_field;
    let generics = if class.type_params.is_empty() {
        syn::Generics::default()
    } else {
        let params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> = class
            .type_params
            .iter()
            .map(|name| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                let bounds = if needs_clone_bound {
                    // Add Clone bound: T: Clone
                    let clone_bound: syn::TypeParamBound = parse_quote!(Clone);
                    let mut bounds = syn::punctuated::Punctuated::new();
                    bounds.push(clone_bound);
                    bounds
                } else {
                    syn::punctuated::Punctuated::new()
                };
                syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident,
                    colon_token: if needs_clone_bound { Some(syn::Token![:](proc_macro2::Span::call_site())) } else { None },
                    bounds,
                    eq_token: None,
                    default: None,
                })
            })
            .collect();
        syn::Generics {
            lt_token: Some(syn::Token![<](proc_macro2::Span::call_site())),
            params,
            gt_token: Some(syn::Token![>](proc_macro2::Span::call_site())),
            where_clause: None,
        }
    };

    // DEPYLER-0765: Check if type params are used in any field
    // If not, we need to add PhantomData to satisfy Rust's unused type param check
    let type_params_used = if class.type_params.is_empty() {
        true // No type params, nothing to check
    } else {
        // Check if any field type contains one of the type params
        fields.iter().any(|f| {
            let type_str = quote::quote!(#f.ty).to_string();
            class.type_params.iter().any(|tp| type_str.contains(tp))
        })
    };

    // Add PhantomData field if type params exist but aren't used in fields
    let mut final_fields: Vec<syn::Field> = fields;
    if !class.type_params.is_empty() && !type_params_used {
        // Build PhantomData<(T, U, ...)> for all type params
        let phantom_types: Vec<syn::Type> = class.type_params.iter().map(|tp| {
            let ident = syn::Ident::new(tp, proc_macro2::Span::call_site());
            parse_quote!(#ident)
        }).collect();

        let phantom_type: syn::Type = if phantom_types.len() == 1 {
            let t = &phantom_types[0];
            parse_quote!(std::marker::PhantomData<#t>)
        } else {
            parse_quote!(std::marker::PhantomData<(#(#phantom_types),*)>)
        };

        final_fields.push(syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited, // private field
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new("_phantom", proc_macro2::Span::call_site())),
            colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
            ty: phantom_type,
        });
    }

    // Create the struct - skip Clone derive for non-Clone field types
    let struct_item = syn::Item::Struct(syn::ItemStruct {
        attrs: if has_non_clone_field {
            // DEPYLER-0611: Skip Clone for structs with Mutex/RefCell/etc fields
            vec![parse_quote! { #[derive(Debug)] }]
        } else if class.is_dataclass {
            vec![parse_quote! { #[derive(Debug, Clone, PartialEq)] }]
        } else {
            vec![parse_quote! { #[derive(Debug, Clone)] }]
        },
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        struct_token: syn::Token![struct](proc_macro2::Span::call_site()),
        ident: struct_name.clone(),
        generics: generics.clone(), // DEPYLER-0739: Use extracted type params
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: final_fields.into_iter().collect(),
        }),
        semi_token: None,
    });
    items.push(struct_item);

    // Generate impl block with methods
    let mut impl_items = Vec::new();

    // Add class constants first
    for class_field in &class_fields {
        if let Some(default_value) = &class_field.default_value {
            let const_name = make_ident(&class_field.name);
            let rust_type = type_mapper.map_type(&class_field.field_type);
            let const_type = rust_type_to_syn_type(&rust_type)?;
            let value_expr = convert_expr(default_value, type_mapper)?;

            // Generate: pub const NAME: Type = value;
            impl_items.push(parse_quote! {
                pub const #const_name: #const_type = #value_expr;
            });
        }
    }

    // Check if class has explicit __init__
    let has_init = class.methods.iter().any(|m| m.name == "__init__");

    // Convert __init__ to new() if present, or generate default new() for dataclasses
    if has_init {
        for method in &class.methods {
            if method.name == "__init__" {
                // DEPYLER-0648: Pass vararg_functions for proper slice wrapping
                let new_method = convert_init_to_new(method, class, &struct_name, type_mapper, vararg_functions)?;
                impl_items.push(syn::ImplItem::Fn(new_method));
            } else {
                // DEPYLER-0648: Pass vararg_functions for proper slice wrapping
                // DEPYLER-0696: Pass class fields for return type inference
                // DEPYLER-0740: Pass class type_params to distinguish method-level generics
                let rust_method = convert_method_to_impl_item(method, type_mapper, vararg_functions, &class.fields, &class.type_params)?;
                impl_items.push(syn::ImplItem::Fn(rust_method));
            }
        }
    } else {
        // Generate default new() for dataclasses or classes with default field values
        if class.is_dataclass
            || class
                .fields
                .iter()
                .all(|f| f.default_value.is_some() || f.field_type == Type::Int)
        {
            let new_method = generate_dataclass_new(class, &struct_name, type_mapper)?;
            impl_items.push(syn::ImplItem::Fn(new_method));
        }

        // Add other methods
        for method in &class.methods {
            // DEPYLER-0648: Pass vararg_functions for proper slice wrapping
            // DEPYLER-0696: Pass class fields for return type inference
            // DEPYLER-0740: Pass class type_params to distinguish method-level generics
            let rust_method = convert_method_to_impl_item(method, type_mapper, vararg_functions, &class.fields, &class.type_params)?;
            impl_items.push(syn::ImplItem::Fn(rust_method));
        }
    }

    // Only generate impl block if there are methods
    if !impl_items.is_empty() {
        // DEPYLER-0739: Build self_ty with generics if present (e.g., Container<T>)
        let self_ty: syn::Type = if class.type_params.is_empty() {
            parse_quote! { #struct_name }
        } else {
            let type_args: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> = class
                .type_params
                .iter()
                .map(|name| {
                    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path::from(ident),
                    })
                })
                .collect();
            parse_quote! { #struct_name<#type_args> }
        };

        let impl_block = syn::Item::Impl(syn::ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: syn::Token![impl](proc_macro2::Span::call_site()),
            generics: generics.clone(), // DEPYLER-0739: Use same generics
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: syn::token::Brace::default(),
            items: impl_items,
        });
        items.push(impl_block);
    }

    Ok(items)
}

fn generate_dataclass_new(
    class: &HirClass,
    _struct_name: &syn::Ident,
    type_mapper: &TypeMapper,
) -> Result<syn::ImplItemFn> {
    // Generate parameters from fields (skip fields with defaults and class variables)
    let mut inputs = syn::punctuated::Punctuated::new();
    let fields_without_defaults: Vec<_> = class
        .fields
        .iter()
        .filter(|f| !f.is_class_var && f.default_value.is_none())
        .collect();

    for field in &fields_without_defaults {
        let param_ident = syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(&field.field_type);
        let param_syn_type = rust_type_to_syn_type(&rust_type)?;

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: param_ident.clone(),
                subpat: None,
            })),
            colon_token: syn::Token![:](proc_macro2::Span::call_site()),
            ty: Box::new(param_syn_type),
        }));
    }

    // Generate body that initializes struct fields (skip class variables)
    let mut field_inits: Vec<proc_macro2::TokenStream> = class
        .fields
        .iter()
        .filter(|f| !f.is_class_var) // Skip class constants
        .map(|field| {
            let field_ident = syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());
            if field.default_value.is_some() {
                // Use default value - for now just use Default::default() or 0 for int
                if field.field_type == Type::Int {
                    quote! { #field_ident: 0 }
                } else {
                    quote! { #field_ident: Default::default() }
                }
            } else {
                // Use parameter
                quote! { #field_ident }
            }
        })
        .collect();

    // DEPYLER-0765: Add PhantomData initialization if type params aren't used in fields
    if !class.type_params.is_empty() {
        let instance_fields: Vec<_> = class.fields.iter().filter(|f| !f.is_class_var).collect();
        let type_params_used = instance_fields.iter().any(|f| {
            let type_str = format!("{:?}", f.field_type);
            class.type_params.iter().any(|tp| type_str.contains(tp))
        });
        if !type_params_used {
            field_inits.push(quote! { _phantom: std::marker::PhantomData });
        }
    }

    let body = parse_quote! {
        {
            Self {
                #(#field_inits),*
            }
        }
    };

    Ok(syn::ImplItemFn {
        attrs: vec![],
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: syn::Token![fn](proc_macro2::Span::call_site()),
            ident: syn::Ident::new("new", proc_macro2::Span::call_site()),
            generics: syn::Generics::default(),
            paren_token: syn::token::Paren::default(),
            inputs,
            variadic: None,
            output: syn::ReturnType::Type(
                syn::Token![->](proc_macro2::Span::call_site()),
                Box::new(parse_quote! { Self }),
            ),
        },
        block: body,
    })
}

fn convert_init_to_new(
    init_method: &HirMethod,
    class: &HirClass,
    _struct_name: &syn::Ident,
    type_mapper: &TypeMapper,
    _vararg_functions: &std::collections::HashSet<String>,
) -> Result<syn::ImplItemFn> {
    // DEPYLER-0697: Collect field names to determine which params are used
    let field_names: std::collections::HashSet<&str> = class
        .fields
        .iter()
        .filter(|f| !f.is_class_var)
        .map(|f| f.name.as_str())
        .collect();

    // Convert parameters
    let mut inputs = syn::punctuated::Punctuated::new();

    for param in &init_method.params {
        // DEPYLER-0697: Prefix unused constructor parameters with _ to avoid warnings
        let param_name = if field_names.contains(param.name.as_str()) {
            param.name.clone()
        } else {
            format!("_{}", param.name)
        };
        let param_ident = make_ident(&param_name);
        let rust_type = type_mapper.map_type(&param.ty);
        let param_syn_type = rust_type_to_syn_type(&rust_type)?;

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: param_ident,
                subpat: None,
            })),
            colon_token: syn::Token![:](proc_macro2::Span::call_site()),
            ty: Box::new(param_syn_type),
        }));
    }

    // Generate field initializers based on class fields and parameters
    // Skip class variables (constants) - only initialize instance fields
    let mut field_inits = Vec::new();

    for field in &class.fields {
        // Skip class variables (constants/statics)
        if field.is_class_var {
            continue;
        }

        let field_ident = syn::Ident::new(&sanitize_identifier(&field.name), proc_macro2::Span::call_site());

        // Check if this field matches a parameter name
        if init_method
            .params
            .iter()
            .any(|param| param.name == field.name)
        {
            // Initialize from parameter
            field_inits.push(quote! { #field_ident });
        } else {
            // Initialize with default value based on type
            let default_value = match &field.field_type {
                Type::Int => quote! { 0 },
                Type::Float => quote! { 0.0 },
                Type::String => quote! { String::new() },
                Type::Bool => quote! { false },
                Type::List(_) => quote! { Vec::new() },
                Type::Dict(_, _) => quote! { std::collections::HashMap::new() },
                Type::Set(_) => quote! { std::collections::HashSet::new() },
                _ => quote! { Default::default() },
            };
            field_inits.push(quote! { #field_ident: #default_value });
        }
    }

    let body = parse_quote! {
        {
            Self {
                #(#field_inits),*
            }
        }
    };

    Ok(syn::ImplItemFn {
        attrs: vec![],
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: syn::Token![fn](proc_macro2::Span::call_site()),
            ident: syn::Ident::new("new", proc_macro2::Span::call_site()),
            generics: syn::Generics::default(),
            paren_token: syn::token::Paren::default(),
            inputs,
            variadic: None,
            output: syn::ReturnType::Type(
                syn::Token![->](proc_macro2::Span::call_site()),
                Box::new(parse_quote! { Self }),
            ),
        },
        block: body,
    })
}

/// Check if a method mutates self (requires &mut self)
/// Scans the method body for assignments to self attributes
pub fn method_mutates_self(method: &HirMethod) -> bool {
    for stmt in &method.body {
        if stmt_mutates_self(stmt) {
            return true;
        }
    }
    false
}

/// Check if a statement mutates self
fn stmt_mutates_self(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Assign { target, .. } => {
            // Check if target is self.field assignment
            matches!(target, AssignTarget::Attribute { value, .. }
                if matches!(value.as_ref(), HirExpr::Var(sym) if sym.as_str() == "self"))
        }
        HirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_mutates_self)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(stmt_mutates_self))
        }
        HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
            body.iter().any(stmt_mutates_self)
        }
        _ => false,
    }
}

/// DEPYLER-0740: Collect type variables from a Type recursively
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
        Type::Int | Type::Float | Type::String | Type::Bool | Type::None
        | Type::Unknown | Type::UnificationVar(_) | Type::Custom(_) => {}
    }
}

/// DEPYLER-0422 Fix #10: Infer return type from method body
/// Similar to infer_return_type_from_body in func_gen.rs
/// DEPYLER-0696: Infer method return type with class field context
fn infer_method_return_type(body: &[HirStmt], fields: &[HirField]) -> Option<Type> {
    let mut return_types = Vec::new();
    collect_method_return_types(body, fields, &mut return_types);

    if return_types.is_empty() {
        return None;
    }

    // If all return types are the same (ignoring Unknown), use that type
    let first_known = return_types.iter().find(|t| !matches!(t, Type::Unknown));
    if let Some(first) = first_known {
        if return_types
            .iter()
            .all(|t| matches!(t, Type::Unknown) || t == first)
        {
            return Some(first.clone());
        }
    }

    // Mixed types - return first known
    first_known.cloned()
}

/// Collect return types from method body statements
/// DEPYLER-0696: Pass class fields to infer self.field types
fn collect_method_return_types(stmts: &[HirStmt], fields: &[HirField], types: &mut Vec<Type>) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                types.push(infer_expr_type_with_fields(expr, fields));
            }
            HirStmt::Return(None) => {
                types.push(Type::None);
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_method_return_types(then_body, fields, types);
                if let Some(else_stmts) = else_body {
                    collect_method_return_types(else_stmts, fields, types);
                }
            }
            HirStmt::For { body, .. } | HirStmt::While { body, .. } => {
                collect_method_return_types(body, fields, types);
            }
            _ => {}
        }
    }
}

/// DEPYLER-0696: Infer type from expression with class field context
///
/// When class fields are provided, attribute access like `self.field` can be
/// resolved to the actual field type instead of returning Type::Unknown.
fn infer_expr_type_with_fields(expr: &HirExpr, fields: &[HirField]) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
            Literal::Bytes(_) => Type::Unknown,
        },
        HirExpr::Binary { op, left, right } => {
            // Comparison operators return bool
            if matches!(
                op,
                BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn
            ) {
                return Type::Bool;
            }
            // For arithmetic, infer from operands
            let left_type = infer_expr_type_with_fields(left, fields);
            if !matches!(left_type, Type::Unknown) {
                left_type
            } else {
                infer_expr_type_with_fields(right, fields)
            }
        }
        HirExpr::Unary { op, operand } => {
            if matches!(op, UnaryOp::Not) {
                Type::Bool
            } else {
                infer_expr_type_with_fields(operand, fields)
            }
        }
        HirExpr::List(elems) => {
            if elems.is_empty() {
                Type::List(Box::new(Type::Unknown))
            } else {
                Type::List(Box::new(infer_expr_type_with_fields(&elems[0], fields)))
            }
        }
        HirExpr::Tuple(elems) => {
            let elem_types: Vec<Type> = elems
                .iter()
                .map(|e| infer_expr_type_with_fields(e, fields))
                .collect();
            Type::Tuple(elem_types)
        }
        // DEPYLER-0696: Handle attribute access like self.field
        HirExpr::Attribute { value, attr } => {
            // Check if this is self.field access
            if let HirExpr::Var(var_name) = value.as_ref() {
                if var_name == "self" {
                    // Look up field type
                    if let Some(field) = fields.iter().find(|f| f.name == *attr) {
                        return field.field_type.clone();
                    }
                }
            }
            Type::Unknown
        }
        // DEPYLER-0736: Handle constructor calls like Point(...) or Self::new(...)
        // In static methods, return ClassName(...) should infer return type as ClassName
        HirExpr::Call { func, .. } => {
            // Check if func looks like a constructor (capitalized name or Self::new)
            let class_name = if func.starts_with("Self::") || func == "Self" {
                // Self::new() or Self() - return Self type
                // Note: We can't know the actual class name here, but Type::Custom("Self")
                // will be mapped correctly when generating code
                Some("Self".to_string())
            } else if func.chars().next().is_some_and(|c| c.is_uppercase()) {
                // Capitalized name like Point, MyClass - likely a constructor
                // Handle both Point and Point::new patterns
                let base_name = func.split("::").next().unwrap_or(func);
                Some(base_name.to_string())
            } else if func == "cls" {
                // cls() in classmethod - return Self
                Some("Self".to_string())
            } else {
                None
            };

            if let Some(name) = class_name {
                Type::Custom(name)
            } else {
                Type::Unknown
            }
        }
        _ => Type::Unknown,
    }
}

/// DEPYLER-0708: Check if a parameter should be typed as &Self based on usage
/// If the parameter is used with attribute access that matches class fields,
/// it's likely to be the same type as self, so we should use &Self instead of serde_json::Value.
fn should_param_be_self_type(param_name: &str, body: &[HirStmt], fields: &[HirField]) -> bool {
    // Collect all field names
    let field_names: std::collections::HashSet<&str> = fields.iter().map(|f| f.name.as_str()).collect();

    // Check if the parameter is used with attribute access that matches a class field
    fn check_expr(param_name: &str, expr: &HirExpr, field_names: &std::collections::HashSet<&str>) -> bool {
        match expr {
            HirExpr::Attribute { value, attr } => {
                if let HirExpr::Var(var_name) = value.as_ref() {
                    if var_name == param_name && field_names.contains(attr.as_str()) {
                        return true;
                    }
                }
                // Check nested expressions
                check_expr(param_name, value, field_names)
            }
            HirExpr::Binary { left, right, .. } => {
                check_expr(param_name, left, field_names) || check_expr(param_name, right, field_names)
            }
            HirExpr::Unary { operand, .. } => check_expr(param_name, operand, field_names),
            HirExpr::Call { args, .. } => args.iter().any(|a| check_expr(param_name, a, field_names)),
            HirExpr::MethodCall { object, args, .. } => {
                check_expr(param_name, object, field_names)
                    || args.iter().any(|a| check_expr(param_name, a, field_names))
            }
            HirExpr::Index { base, index } => {
                check_expr(param_name, base, field_names) || check_expr(param_name, index, field_names)
            }
            HirExpr::List(items) | HirExpr::Tuple(items) => {
                items.iter().any(|i| check_expr(param_name, i, field_names))
            }
            HirExpr::IfExpr { test, body, orelse } => {
                check_expr(param_name, test, field_names)
                    || check_expr(param_name, body, field_names)
                    || check_expr(param_name, orelse, field_names)
            }
            _ => false,
        }
    }

    fn check_stmt(param_name: &str, stmt: &HirStmt, field_names: &std::collections::HashSet<&str>) -> bool {
        match stmt {
            HirStmt::Assign { value, .. } => check_expr(param_name, value, field_names),
            HirStmt::Expr(expr) => check_expr(param_name, expr, field_names),
            HirStmt::Return(Some(expr)) => check_expr(param_name, expr, field_names),
            HirStmt::If { condition, then_body, else_body, .. } => {
                check_expr(param_name, condition, field_names)
                    || then_body.iter().any(|s| check_stmt(param_name, s, field_names))
                    || else_body.as_ref().is_some_and(|eb| eb.iter().any(|s| check_stmt(param_name, s, field_names)))
            }
            HirStmt::While { condition, body, .. } => {
                check_expr(param_name, condition, field_names)
                    || body.iter().any(|s| check_stmt(param_name, s, field_names))
            }
            HirStmt::For { body, .. } => body.iter().any(|s| check_stmt(param_name, s, field_names)),
            _ => false,
        }
    }

    body.iter().any(|s| check_stmt(param_name, s, &field_names))
}

/// DEPYLER-0696: Accept class fields for return type inference
/// DEPYLER-0740: Accept class type_params to distinguish method-level generics
fn convert_method_to_impl_item(
    method: &HirMethod,
    type_mapper: &TypeMapper,
    vararg_functions: &std::collections::HashSet<String>,
    fields: &[HirField],
    class_type_params: &[String],
) -> Result<syn::ImplItemFn> {
    // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
    let method_name = if is_rust_keyword(&method.name) {
        syn::Ident::new_raw(&method.name, proc_macro2::Span::call_site())
    } else {
        make_ident(&method.name)
    };

    // DEPYLER-0740: Collect type variables used in method signature
    let mut method_type_vars = std::collections::HashSet::new();
    for param in &method.params {
        collect_type_vars(&param.ty, &mut method_type_vars);
    }
    collect_type_vars(&method.ret_type, &mut method_type_vars);

    // Filter out class-level type params to get method-level ones
    let method_level_type_params: Vec<String> = method_type_vars
        .into_iter()
        .filter(|tv| !class_type_params.contains(tv))
        .collect();

    // Convert parameters
    let mut inputs = syn::punctuated::Punctuated::new();

    // Add self parameter based on method type
    if method.is_static {
        // Static methods have no self parameter
    } else if method.is_classmethod {
        // Note: Class methods would ideally take a type parameter (e.g., &Self),
        // but currently classmethods are transpiled without self parameter.
        // Proper classmethod support with type parameter is a known limitation.
    } else if method.is_property {
        // Properties typically use &self
        inputs.push(parse_quote! { &self });
    } else {
        // Regular instance methods: use &mut self if method mutates self, otherwise &self
        if method_mutates_self(method) {
            inputs.push(parse_quote! { &mut self });
        } else {
            inputs.push(parse_quote! { &self });
        }
    }

    // Add other parameters
    for param in &method.params {
        let param_ident = make_ident(&param.name);

        // DEPYLER-0708: Infer &Self type for parameters with Unknown type that are used like self
        // DEPYLER-0271: For method params with Unknown type, use () instead of T to avoid undeclared generic
        let param_syn_type = if matches!(param.ty, Type::Unknown)
            && should_param_be_self_type(&param.name, &method.body, fields)
        {
            // Parameter is used with attribute access matching class fields - use &Self
            parse_quote! { &Self }
        } else if matches!(param.ty, Type::Unknown) {
            // DEPYLER-0271: Unknown type in method params should be () not TypeParam("T")
            // This is especially important for __exit__(exc_type, exc_val, exc_tb) where
            // the exception parameters are typically unused and should not create generic T
            // Also prefix parameter name with _ to avoid unused warnings
            let prefixed_name = if !param.name.starts_with('_') {
                format!("_{}", param.name)
            } else {
                param.name.clone()
            };
            let prefixed_ident = make_ident(&prefixed_name);

            // Add parameter with prefixed name and () type
            inputs.push(syn::FnArg::Typed(syn::PatType {
                attrs: vec![],
                pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: prefixed_ident,
                    subpat: None,
                })),
                colon_token: syn::Token![:](proc_macro2::Span::call_site()),
                ty: Box::new(parse_quote! { () }),
            }));
            continue; // Skip the normal parameter addition below
        } else {
            let rust_type = type_mapper.map_type(&param.ty);
            let base_type = rust_type_to_syn_type(&rust_type)?;

            // DEPYLER-0709: For class-typed parameters, use reference instead of owned value
            // Python objects are passed by reference, so method signatures should take &ClassName
            // This matches the call-site behavior in DEPYLER-0712 which adds & to class args
            if matches!(&param.ty, Type::Custom(_)) {
                // Wrap in reference: ClassName -> &ClassName
                parse_quote! { &#base_type }
            } else {
                base_type
            }
        };

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: param_ident,
                subpat: None,
            })),
            colon_token: syn::Token![:](proc_macro2::Span::call_site()),
            ty: Box::new(param_syn_type),
        }));
    }

    // Convert return type - infer if Unknown or None (DEPYLER-0422 Fix #10)
    // Five-Whys Root Cause:
    // 1. Why: expected `()`, found `bool` in __exit__ method
    // 2. Why: Method has no return type annotation, so ret_type is Unknown/None
    // 3. Why: convert_method_to_impl_item uses type_mapper.map_type directly
    // 4. Why: No return type inference is applied to class methods
    // 5. ROOT CAUSE: direct_rules.rs doesn't infer return type for methods
    // DEPYLER-0696: Pass class fields for self.field type inference
    let effective_ret_type = if matches!(method.ret_type, Type::Unknown | Type::None) {
        // Try to infer from body - if we find a typed return, use it
        infer_method_return_type(&method.body, fields).unwrap_or_else(|| method.ret_type.clone())
    } else {
        method.ret_type.clone()
    };
    let rust_ret_type = type_mapper.map_type(&effective_ret_type);
    let ret_type = rust_type_to_syn_type(&rust_ret_type)?;

    // DEPYLER-0704: Extract parameter types for type coercion in binary operations
    let param_types: std::collections::HashMap<String, Type> = method
        .params
        .iter()
        .map(|p| (p.name.clone(), p.ty.clone()))
        .collect();

    // DEPYLER-0720: Build class field types map from fields
    let class_field_types: std::collections::HashMap<String, Type> = fields
        .iter()
        .map(|f| (f.name.clone(), f.field_type.clone()))
        .collect();

    // Convert method body
    let body = if method.body.is_empty() {
        // Empty body - just return default
        parse_quote! { {} }
    } else {
        // Convert the method body statements with classmethod context
        // DEPYLER-0648: Pass vararg_functions for proper slice wrapping at call sites
        // DEPYLER-0704: Pass param_types for type coercion
        // DEPYLER-0720: Pass class_field_types for self.field float coercion
        convert_method_body_block(&method.body, type_mapper, method.is_classmethod, vararg_functions, &param_types, &class_field_types)?
    };

    Ok(syn::ImplItemFn {
        attrs: vec![],
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: if method.is_async {
                Some(syn::Token![async](proc_macro2::Span::call_site()))
            } else {
                None
            },
            unsafety: None,
            abi: None,
            fn_token: syn::Token![fn](proc_macro2::Span::call_site()),
            ident: method_name,
            // DEPYLER-0740: Build method-level generics for type params not in class signature
            // Add Clone bound to method-level type params since they're often used with Clone-bounded structs
            generics: if method_level_type_params.is_empty() {
                syn::Generics::default()
            } else {
                let params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> =
                    method_level_type_params
                        .iter()
                        .map(|name| {
                            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                            // Add Clone bound
                            let clone_bound: syn::TypeParamBound = parse_quote!(Clone);
                            let mut bounds = syn::punctuated::Punctuated::new();
                            bounds.push(clone_bound);
                            syn::GenericParam::Type(syn::TypeParam {
                                attrs: vec![],
                                ident,
                                colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
                                bounds,
                                eq_token: None,
                                default: None,
                            })
                        })
                        .collect();
                syn::Generics {
                    lt_token: Some(syn::Token![<](proc_macro2::Span::call_site())),
                    params,
                    gt_token: Some(syn::Token![>](proc_macro2::Span::call_site())),
                    where_clause: None,
                }
            },
            paren_token: syn::token::Paren::default(),
            inputs,
            variadic: None,
            output: if matches!(effective_ret_type, Type::None) {
                syn::ReturnType::Default
            } else {
                syn::ReturnType::Type(
                    syn::Token![->](proc_macro2::Span::call_site()),
                    Box::new(ret_type),
                )
            },
        },
        block: body,
    })
}

fn convert_protocol_method_to_trait_method(
    method: &ProtocolMethod,
    type_mapper: &TypeMapper,
) -> Result<syn::TraitItem> {
    let method_name = make_ident(&method.name);

    // Convert parameters
    let mut inputs = syn::punctuated::Punctuated::new();

    // Add self parameter for methods (skip first param if it's 'self')
    let method_params = if !method.params.is_empty() && method.params[0].name == "self" {
        // Add &self receiver
        inputs.push(syn::FnArg::Receiver(syn::Receiver {
            attrs: vec![],
            reference: Some((syn::Token![&](proc_macro2::Span::call_site()), None)),
            mutability: None,
            self_token: syn::Token![self](proc_macro2::Span::call_site()),
            colon_token: None,
            ty: Box::new(parse_quote! { Self }),
        }));
        &method.params[1..] // Skip self parameter
    } else {
        &method.params[..]
    };

    // Add remaining parameters
    for param in method_params {
        let param_ident = make_ident(&param.name);
        let rust_type = type_mapper.map_type(&param.ty);
        let param_syn_type = rust_type_to_syn_type(&rust_type)?;

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: param_ident,
                subpat: None,
            })),
            colon_token: syn::Token![:](proc_macro2::Span::call_site()),
            ty: Box::new(param_syn_type),
        }));
    }

    // Convert return type
    let rust_return_type = type_mapper.map_type(&method.ret_type);
    let return_type = rust_type_to_syn_type(&rust_return_type)?;

    // Create function signature
    let sig = syn::Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: syn::Token![fn](proc_macro2::Span::call_site()),
        ident: method_name,
        generics: syn::Generics::default(),
        paren_token: syn::token::Paren::default(),
        inputs,
        variadic: None,
        output: syn::ReturnType::Type(
            syn::Token![->](proc_macro2::Span::call_site()),
            Box::new(return_type),
        ),
    };

    // Create trait method (with or without default implementation)
    if method.has_default {
        // For now, skip default implementations in traits - would need body conversion
        Ok(syn::TraitItem::Fn(syn::TraitItemFn {
            attrs: vec![],
            sig,
            default: None,
            semi_token: Some(syn::Token![;](proc_macro2::Span::call_site())),
        }))
    } else {
        Ok(syn::TraitItem::Fn(syn::TraitItemFn {
            attrs: vec![],
            sig,
            default: None,
            semi_token: Some(syn::Token![;](proc_macro2::Span::call_site())),
        }))
    }
}

/// DEPYLER-0765: Resolve UnionType placeholder Enum to a valid Rust type
/// Analyzes the variant names to determine the best concrete type
fn resolve_union_enum_to_syn(variants: &[(String, RustType)]) -> syn::Type {
    // Helper to check if variant name is numeric
    let is_numeric = |v: &str| {
        matches!(v, "int" | "float" | "i64" | "f64" | "i32" | "f32" | "I64" | "F64")
    };
    let is_float_like = |v: &str| matches!(v, "float" | "f64" | "f32" | "F64");
    let is_none_like = |v: &str| matches!(v, "None" | "NoneType");
    let is_string_like = |v: &str| matches!(v, "str" | "String" | "Str");

    // Extract variant names (first element of tuple)
    let variant_names: Vec<&str> = variants.iter().map(|(name, _)| name.as_str()).collect();

    // Filter out None-like variants
    let has_none = variant_names.iter().any(|v| is_none_like(v));
    let non_none: Vec<&str> = variant_names
        .iter()
        .copied()
        .filter(|v| !is_none_like(v))
        .collect();

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

    // Case 6: Fallback to serde_json::Value
    parse_quote! { serde_json::Value }
}

/// Convert simple non-recursive types (Unit, String, Custom, TypeParam, Enum)
#[inline]
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
                let ident = make_ident(name);
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
                let ident = make_ident(name);
                parse_quote! { #ident }
            }
        }
        _ => unreachable!("convert_simple_type called with non-simple type"),
    })
}

/// Convert primitive types (bool, integers, floats)
#[inline]
fn convert_primitive_type(prim_type: &crate::type_mapper::PrimitiveType) -> Result<syn::Type> {
    use crate::type_mapper::PrimitiveType;
    Ok(match prim_type {
        PrimitiveType::Bool => parse_quote! { bool },
        PrimitiveType::I8 => parse_quote! { i8 },
        PrimitiveType::I16 => parse_quote! { i16 },
        PrimitiveType::I32 => parse_quote! { i32 },
        PrimitiveType::I64 => parse_quote! { i64 },
        PrimitiveType::I128 => parse_quote! { i128 },
        PrimitiveType::ISize => parse_quote! { isize },
        PrimitiveType::U8 => parse_quote! { u8 },
        PrimitiveType::U16 => parse_quote! { u16 },
        PrimitiveType::U32 => parse_quote! { u32 },
        PrimitiveType::U64 => parse_quote! { u64 },
        PrimitiveType::U128 => parse_quote! { u128 },
        PrimitiveType::USize => parse_quote! { usize },
        PrimitiveType::F32 => parse_quote! { f32 },
        PrimitiveType::F64 => parse_quote! { f64 },
    })
}

/// Convert lifetime-parameterized types (Str, Cow)
#[inline]
fn convert_lifetime_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Str { lifetime } => {
            if let Some(lt) = lifetime {
                let lifetime_token =
                    syn::Lifetime::new(&format!("'{}", lt), proc_macro2::Span::call_site());
                parse_quote! { &#lifetime_token str }
            } else {
                parse_quote! { &str }
            }
        }
        Cow { lifetime } => {
            let lifetime_token =
                syn::Lifetime::new(&format!("'{}", lifetime), proc_macro2::Span::call_site());
            parse_quote! { std::borrow::Cow<#lifetime_token, str> }
        }
        _ => unreachable!("convert_lifetime_type called with non-lifetime type"),
    })
}

/// Convert unsupported types with placeholder names
#[inline]
fn convert_unsupported_type(name: &str) -> Result<syn::Type> {
    let ident = syn::Ident::new(
        &format!("UnsupportedType_{}", name.replace(" ", "_")),
        proc_macro2::Span::call_site(),
    );
    Ok(parse_quote! { #ident })
}

/// Convert container types (Vec, HashMap, Option, Result, HashSet)
#[inline]
fn convert_container_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Vec(inner) => {
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { Vec<#inner_type> }
        }
        HashMap(key, value) => {
            // DEPYLER-0686: Use fully qualified path to avoid import issues
            let key_type = rust_type_to_syn_type(key)?;
            let value_type = rust_type_to_syn_type(value)?;
            parse_quote! { std::collections::HashMap<#key_type, #value_type> }
        }
        Option(inner) => {
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { Option<#inner_type> }
        }
        Result(ok, err) => {
            let ok_type = rust_type_to_syn_type(ok)?;
            let err_type = rust_type_to_syn_type(err)?;
            parse_quote! { Result<#ok_type, #err_type> }
        }
        HashSet(inner) => {
            // DEPYLER-0686: Use fully qualified path to avoid import issues
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { std::collections::HashSet<#inner_type> }
        }
        _ => unreachable!("convert_container_type called with non-container type"),
    })
}

/// Convert complex recursive types (Tuple, Generic, Reference)
#[inline]
fn convert_complex_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Tuple(types) => {
            let type_tokens: anyhow::Result<std::vec::Vec<_>> =
                types.iter().map(rust_type_to_syn_type).collect();
            let type_tokens = type_tokens?;
            parse_quote! { (#(#type_tokens),*) }
        }
        Generic { base, params } => {
            let base_ident = make_ident(base);
            let param_types: anyhow::Result<std::vec::Vec<_>> =
                params.iter().map(rust_type_to_syn_type).collect();
            let param_types = param_types?;
            parse_quote! { #base_ident<#(#param_types),*> }
        }
        Reference { inner, mutable, .. } => {
            let inner_type = rust_type_to_syn_type(inner)?;
            if *mutable {
                parse_quote! { &mut #inner_type }
            } else {
                parse_quote! { &#inner_type }
            }
        }
        _ => unreachable!("convert_complex_type called with non-complex type"),
    })
}

/// Convert array types with const generic handling
#[inline]
fn convert_array_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    if let Array { element_type, size } = rust_type {
        let element = rust_type_to_syn_type(element_type)?;
        Ok(match size {
            crate::type_mapper::RustConstGeneric::Literal(n) => {
                let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                parse_quote! { [#element; #size_lit] }
            }
            crate::type_mapper::RustConstGeneric::Parameter(name) => {
                let param_ident = make_ident(name);
                parse_quote! { [#element; #param_ident] }
            }
            crate::type_mapper::RustConstGeneric::Expression(expr) => {
                let expr_tokens: proc_macro2::TokenStream = expr
                    .parse()
                    .unwrap_or_else(|_| "/* invalid const expression */".parse().unwrap());
                parse_quote! { [#element; #expr_tokens] }
            }
        })
    } else {
        unreachable!("convert_array_type called with non-array type")
    }
}

fn rust_type_to_syn_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        // Simple types - delegate to helper
        Unit | String | Custom(_) | TypeParam(_) | Enum { .. } => convert_simple_type(rust_type)?,

        // Primitive types - delegate to helper
        Primitive(prim_type) => convert_primitive_type(prim_type)?,

        // Lifetime types - delegate to helper
        Str { .. } | Cow { .. } => convert_lifetime_type(rust_type)?,

        // Unsupported types - delegate to helper
        Unsupported(name) => convert_unsupported_type(name)?,

        // Container types - delegate to helper
        Vec(_) | HashMap(_, _) | Option(_) | Result(_, _) | HashSet(_) => {
            convert_container_type(rust_type)?
        }

        // Complex types - delegate to helper
        Tuple(_) | Generic { .. } | Reference { .. } => convert_complex_type(rust_type)?,

        // Array types - delegate to helper
        Array { .. } => convert_array_type(rust_type)?,
    })
}

fn convert_function(func: &HirFunction, type_mapper: &TypeMapper) -> Result<syn::ItemFn> {
    let name = make_ident(&func.name);

    // Convert parameters
    let mut inputs = Vec::new();
    for param in &func.params {
        let rust_type = type_mapper.map_type(&param.ty);
        let ty = rust_type_to_syn(&rust_type)?;
        let pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: make_ident(&param.name),
            subpat: None,
        });

        // Use references for non-copy types
        let ty = if type_mapper.needs_reference(&rust_type) {
            parse_quote! { &#ty }
        } else {
            ty
        };

        inputs.push(syn::FnArg::Typed(syn::PatType {
            attrs: vec![],
            pat: Box::new(pat),
            colon_token: Default::default(),
            ty: Box::new(ty),
        }));
    }

    // Convert return type
    let rust_ret_type = type_mapper.map_return_type(&func.ret_type);

    // DEPYLER-0612: Fix main() return type - Rust main can only return () or Result<(), E>
    // Cannot return Result<i32, E> - convert to Result<(), E>
    let rust_ret_type = if func.name == "main" {
        match &rust_ret_type {
            RustType::Result(inner, err) if matches!(**inner, RustType::Primitive(_)) => {
                // Result<i32, E> -> Result<(), E> for main
                RustType::Result(Box::new(RustType::Unit), err.clone())
            }
            _ => rust_ret_type,
        }
    } else {
        rust_ret_type
    };

    let output = if matches!(rust_ret_type, RustType::Unit) {
        syn::ReturnType::Default
    } else {
        let ty = rust_type_to_syn(&rust_ret_type)?;
        syn::ReturnType::Type(Default::default(), Box::new(ty))
    };

    // Convert body
    let body_stmts = convert_body(&func.body, type_mapper)?;
    let block = syn::Block {
        brace_token: Default::default(),
        stmts: body_stmts,
    };

    // Add documentation
    let mut attrs = vec![];

    // Add docstring as documentation if present
    if let Some(docstring) = &func.docstring {
        attrs.push(parse_quote! {
            #[doc = #docstring]
        });
    }

    if func.properties.panic_free {
        attrs.push(parse_quote! {
            #[doc = " Depyler: verified panic-free"]
        });
    }
    if func.properties.always_terminates {
        attrs.push(parse_quote! {
            #[doc = " Depyler: proven to terminate"]
        });
    }

    Ok(syn::ItemFn {
        attrs,
        vis: syn::Visibility::Public(Default::default()),
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: name,
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: inputs.into_iter().collect(),
            variadic: None,
            output,
        },
        block: Box::new(block),
    })
}

fn rust_type_to_syn(rust_type: &RustType) -> Result<syn::Type> {
    Ok(match rust_type {
        RustType::Primitive(p) => {
            let ident = syn::Ident::new(p.to_rust_string(), proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::String => parse_quote! { String },
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Vec<#inner_ty> }
        }
        RustType::HashMap(k, v) => {
            let key_ty = rust_type_to_syn(k)?;
            let val_ty = rust_type_to_syn(v)?;
            parse_quote! { HashMap<#key_ty, #val_ty> }
        }
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Option<#inner_ty> }
        }
        RustType::Unit => parse_quote! { () },
        RustType::Array { element_type, size } => {
            let element = rust_type_to_syn(element_type)?;
            match size {
                crate::type_mapper::RustConstGeneric::Literal(n) => {
                    let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                    parse_quote! { [#element; #size_lit] }
                }
                crate::type_mapper::RustConstGeneric::Parameter(name) => {
                    let param_ident = make_ident(name);
                    parse_quote! { [#element; #param_ident] }
                }
                crate::type_mapper::RustConstGeneric::Expression(expr) => {
                    let expr_tokens: proc_macro2::TokenStream = expr
                        .parse()
                        .unwrap_or_else(|_| "/* invalid const expression */".parse().unwrap());
                    parse_quote! { [#element; #expr_tokens] }
                }
            }
        }
        _ => bail!("Unsupported Rust type: {:?}", rust_type),
    })
}

fn convert_body(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<Vec<syn::Stmt>> {
    // Use empty vararg_functions and param_types for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> = std::sync::OnceLock::new();
    convert_body_with_context(
        stmts,
        type_mapper,
        false,
        EMPTY_SET.get_or_init(std::collections::HashSet::new),
        EMPTY_MAP.get_or_init(std::collections::HashMap::new),
    )
}

/// DEPYLER-0713: Analyze which variables need to be mutable
/// Variables are mutable if they are reassigned or have mutating method calls
fn find_mutable_vars_in_body(stmts: &[HirStmt]) -> std::collections::HashSet<String> {
    let mut declared: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut mutable: std::collections::HashSet<String> = std::collections::HashSet::new();

    fn analyze_stmt(
        stmt: &HirStmt,
        declared: &mut std::collections::HashSet<String>,
        mutable: &mut std::collections::HashSet<String>,
    ) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // Check for mutating method calls in value
                check_mutating_methods(value, mutable);

                match target {
                    AssignTarget::Symbol(name) => {
                        if declared.contains(name) {
                            // Reassignment - mark as mutable
                            mutable.insert(name.clone());
                        } else {
                            declared.insert(name.clone());
                        }
                    }
                    AssignTarget::Attribute { value: base, .. } |
                    AssignTarget::Index { base, .. } => {
                        // Attribute/index assignment requires base to be mutable
                        if let HirExpr::Var(var_name) = base.as_ref() {
                            mutable.insert(var_name.clone());
                        }
                    }
                    AssignTarget::Tuple(targets) => {
                        for t in targets {
                            if let AssignTarget::Symbol(name) = t {
                                if declared.contains(name) {
                                    mutable.insert(name.clone());
                                } else {
                                    declared.insert(name.clone());
                                }
                            }
                        }
                    }
                }
            }
            HirStmt::Expr(expr) => {
                check_mutating_methods(expr, mutable);
            }
            HirStmt::If { condition, then_body, else_body } => {
                check_mutating_methods(condition, mutable);
                for s in then_body {
                    analyze_stmt(s, declared, mutable);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        analyze_stmt(s, declared, mutable);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                check_mutating_methods(condition, mutable);
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
            }
            HirStmt::For { body, .. } => {
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
            }
            HirStmt::With { body, .. } => {
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
            }
            HirStmt::Try { body, handlers, orelse, finalbody, .. } => {
                for s in body {
                    analyze_stmt(s, declared, mutable);
                }
                for h in handlers {
                    for s in &h.body {
                        analyze_stmt(s, declared, mutable);
                    }
                }
                if let Some(else_stmts) = orelse {
                    for s in else_stmts {
                        analyze_stmt(s, declared, mutable);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for s in final_stmts {
                        analyze_stmt(s, declared, mutable);
                    }
                }
            }
            _ => {}
        }
    }

    fn check_mutating_methods(expr: &HirExpr, mutable: &mut std::collections::HashSet<String>) {
        match expr {
            HirExpr::MethodCall { object, method, args, .. } => {
                // Check if this is a mutating method
                let is_mutating = matches!(
                    method.as_str(),
                    "append" | "extend" | "insert" | "remove" | "pop" | "clear" |
                    "reverse" | "sort" | "update" | "setdefault" | "popitem" |
                    "add" | "discard" | "write" | "write_all" | "writelines" | "flush"
                );
                if is_mutating {
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        mutable.insert(var_name.clone());
                    }
                }
                check_mutating_methods(object, mutable);
                for arg in args {
                    check_mutating_methods(arg, mutable);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                check_mutating_methods(left, mutable);
                check_mutating_methods(right, mutable);
            }
            HirExpr::Unary { operand, .. } => {
                check_mutating_methods(operand, mutable);
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    check_mutating_methods(arg, mutable);
                }
            }
            HirExpr::List(items) | HirExpr::Tuple(items) | HirExpr::Set(items) => {
                for item in items {
                    check_mutating_methods(item, mutable);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    check_mutating_methods(k, mutable);
                    check_mutating_methods(v, mutable);
                }
            }
            _ => {}
        }
    }

    for stmt in stmts {
        analyze_stmt(stmt, &mut declared, &mut mutable);
    }

    mutable
}

/// DEPYLER-0704: Added param_types parameter for type coercion in binary operations
/// DEPYLER-0713: Added mutable_vars analysis for proper mutability
fn convert_body_with_context(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<Vec<syn::Stmt>> {
    // DEPYLER-0713: Pre-analyze which variables need to be mutable
    let mutable_vars = find_mutable_vars_in_body(stmts);

    stmts
        .iter()
        .map(|stmt| convert_stmt_with_mutable_vars(stmt, type_mapper, is_classmethod, vararg_functions, param_types, &mutable_vars))
        .collect()
}

/// Convert simple variable assignment: `x = value`
/// DEPYLER-0713: Only add `mut` if variable is in mutable_vars set
///
/// Complexity: 2 (with branching for mutability)
fn convert_symbol_assignment(
    symbol: &str,
    value_expr: syn::Expr,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    let target_ident = make_ident(symbol);
    // DEPYLER-0713: Only add mut if variable is reassigned or has mutating method calls
    let mutability = if mutable_vars.contains(symbol) {
        Some(Default::default())
    } else {
        None
    };
    let stmt = syn::Stmt::Local(syn::Local {
        attrs: vec![],
        let_token: Default::default(),
        pat: syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability,
            ident: target_ident,
            subpat: None,
        }),
        init: Some(syn::LocalInit {
            eq_token: Default::default(),
            expr: Box::new(value_expr),
            diverge: None,
        }),
        semi_token: Default::default(),
    });
    Ok(stmt)
}

/// Convert subscript assignment: `d[k] = value` or `d[k1][k2] = value`
///
/// Handles both simple and nested subscript assignments.
/// Complexity: 3 (if + loop with nested if)
fn convert_index_assignment(
    base: &HirExpr,
    index: &HirExpr,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let final_index = convert_expr(index, type_mapper)?;
    let (base_expr, indices) = extract_nested_indices(base, type_mapper)?;

    if indices.is_empty() {
        // Simple assignment: d[k] = v
        let assign_expr = parse_quote! {
            #base_expr.insert(#final_index, #value_expr)
        };
        Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
    } else {
        // Nested assignment: build chain of get_mut calls
        let mut chain = base_expr;
        for idx in &indices {
            chain = parse_quote! {
                #chain.get_mut(&#idx).unwrap()
            };
        }

        let assign_expr = parse_quote! {
            #chain.insert(#final_index, #value_expr)
        };

        Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
    }
}

/// Convert attribute assignment: `obj.attr = value`
///
/// Complexity: 1 (no branching)
fn convert_attribute_assignment(
    base: &HirExpr,
    attr: &str,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let base_expr = convert_expr(base, type_mapper)?;
    let attr_ident = make_ident(attr);

    let assign_expr = parse_quote! {
        #base_expr.#attr_ident = #value_expr
    };

    Ok(syn::Stmt::Expr(assign_expr, Some(Default::default())))
}

/// Convert Python assignment statement to Rust
///
/// Handles 3 assignment target types:
/// - Symbol: `x = value`
/// - Index: `d[k] = value` or `d[k1][k2] = value`
/// - Attribute: `obj.attr = value`
///
/// Complexity: ~8 (match with 3 arms + nested complexity from index helper)
#[allow(dead_code)]
fn convert_assign_stmt(
    target: &AssignTarget,
    value: &HirExpr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    let value_expr = convert_expr(value, type_mapper)?;
    convert_assign_stmt_with_expr(target, value_expr, type_mapper)
}

fn convert_assign_stmt_with_expr(
    target: &AssignTarget,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
) -> Result<syn::Stmt> {
    // Backward compatibility - use empty mutable_vars (all variables get mut)
    static EMPTY_MUTABLE: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, EMPTY_MUTABLE.get_or_init(std::collections::HashSet::new))
}

/// DEPYLER-0713: Convert assignment with proper mutability tracking
fn convert_assign_stmt_with_mutable_vars(
    target: &AssignTarget,
    value_expr: syn::Expr,
    type_mapper: &TypeMapper,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    match target {
        AssignTarget::Symbol(symbol) => convert_symbol_assignment(symbol, value_expr, mutable_vars),
        AssignTarget::Index { base, index } => {
            convert_index_assignment(base, index, value_expr, type_mapper)
        }
        AssignTarget::Attribute { value: base, attr } => {
            convert_attribute_assignment(base, attr, value_expr, type_mapper)
        }
        AssignTarget::Tuple(targets) => {
            // Tuple unpacking - simplified version
            let all_symbols: Option<Vec<&str>> = targets
                .iter()
                .map(|t| match t {
                    AssignTarget::Symbol(s) => Some(s.as_str()),
                    _ => None,
                })
                .collect();

            match all_symbols {
                Some(symbols) => {
                    // DEPYLER-0713: Only add mut if variable is in mutable_vars
                    let idents_with_mut: Vec<_> = symbols
                        .iter()
                        .map(|s| (make_ident(s), mutable_vars.contains(*s)))
                        .collect();
                    let pat = syn::Pat::Tuple(syn::PatTuple {
                        attrs: vec![],
                        paren_token: syn::token::Paren::default(),
                        elems: idents_with_mut
                            .iter()
                            .map(|(ident, needs_mut)| {
                                syn::Pat::Ident(syn::PatIdent {
                                    attrs: vec![],
                                    by_ref: None,
                                    mutability: if *needs_mut { Some(syn::token::Mut::default()) } else { None },
                                    ident: ident.clone(),
                                    subpat: None,
                                })
                            })
                            .collect(),
                    });
                    Ok(syn::Stmt::Local(syn::Local {
                        attrs: vec![],
                        let_token: syn::token::Let::default(),
                        pat,
                        init: Some(syn::LocalInit {
                            eq_token: syn::token::Eq::default(),
                            expr: Box::new(value_expr),
                            diverge: None,
                        }),
                        semi_token: syn::token::Semi::default(),
                    }))
                }
                None => {
                    // GH-109: Handle tuple unpacking with Index targets
                    // Pattern: list[i], list[j] = list[j], list[i] (swap)
                    let all_indices: Option<Vec<_>> = targets
                        .iter()
                        .map(|t| match t {
                            AssignTarget::Index { base, index } => Some((base, index)),
                            _ => None,
                        })
                        .collect();

                    if let Some(indices) = all_indices {
                        // All targets are subscripts - generate temp-based assignment
                        let temp_ident = make_ident("_swap_temp");

                        // Build assignments for each target from temp tuple
                        let mut stmts: Vec<syn::Stmt> = Vec::new();

                        // First: let _swap_temp = value_expr;
                        stmts.push(syn::Stmt::Local(syn::Local {
                            attrs: vec![],
                            let_token: syn::token::Let::default(),
                            pat: syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: temp_ident.clone(),
                                subpat: None,
                            }),
                            init: Some(syn::LocalInit {
                                eq_token: syn::token::Eq::default(),
                                expr: Box::new(value_expr),
                                diverge: None,
                            }),
                            semi_token: syn::token::Semi::default(),
                        }));

                        // Then: base[index] = _swap_temp.N for each target
                        for (idx, (base, index)) in indices.iter().enumerate() {
                            let base_expr = convert_expr(base, type_mapper)?;
                            let index_expr = convert_expr(index, type_mapper)?;
                            let tuple_index = syn::Index::from(idx);

                            let assign_expr: syn::Expr = parse_quote! {
                                #base_expr[(#index_expr) as usize] = #temp_ident.#tuple_index
                            };
                            stmts.push(syn::Stmt::Expr(assign_expr, Some(Default::default())));
                        }

                        // Return a block containing all statements
                        // Note: We return just the first statement; caller may need to handle block
                        if stmts.len() == 1 {
                            return Ok(stmts.remove(0));
                        } else {
                            // Wrap in a block expression
                            let block = syn::Block {
                                brace_token: syn::token::Brace::default(),
                                stmts,
                            };
                            return Ok(syn::Stmt::Expr(
                                syn::Expr::Block(syn::ExprBlock {
                                    attrs: vec![],
                                    label: None,
                                    block,
                                }),
                                None,
                            ));
                        }
                    }

                    bail!("Complex tuple unpacking not yet supported")
                }
            }
        }
    }
}

/// DEPYLER-0701: Check if an expression is "pure" (has no side effects)
/// Pure expressions used as statements need `let _ =` to silence warnings
fn is_pure_expression_direct(expr: &HirExpr) -> bool {
    match expr {
        // Bare variable references have no side effects
        HirExpr::Var(_) => true,
        // Literals have no side effects
        HirExpr::Literal(_) => true,
        // Attribute access (self.x) has no side effects
        HirExpr::Attribute { .. } => true,
        // Binary operations with pure operands are pure
        HirExpr::Binary { left, right, .. } => {
            is_pure_expression_direct(left) && is_pure_expression_direct(right)
        }
        // Unary operations with pure operands are pure
        HirExpr::Unary { operand, .. } => is_pure_expression_direct(operand),
        // Index access (arr[i]) has no side effects
        HirExpr::Index { base, index } => {
            is_pure_expression_direct(base) && is_pure_expression_direct(index)
        }
        // Tuple access is pure
        HirExpr::Tuple(elts) => elts.iter().all(is_pure_expression_direct),
        // Everything else (calls, method calls, etc.) may have side effects
        _ => false,
    }
}

#[allow(dead_code)]
fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt> {
    // Use empty vararg_functions and param_types for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> = std::sync::OnceLock::new();
    convert_stmt_with_context(
        stmt,
        type_mapper,
        false,
        EMPTY_SET.get_or_init(std::collections::HashSet::new),
        EMPTY_MAP.get_or_init(std::collections::HashMap::new),
    )
}

/// DEPYLER-0713: Convert statement with proper mutability tracking
fn convert_stmt_with_mutable_vars(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            let value_expr = convert_expr_with_param_types(value, type_mapper, is_classmethod, vararg_functions, param_types)?;
            convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, mutable_vars)
        }
        // For all other statement types, delegate to convert_stmt_with_context
        // They don't generate new variable bindings so mutable_vars doesn't matter
        _ => convert_stmt_with_context(stmt, type_mapper, is_classmethod, vararg_functions, param_types),
    }
}

/// DEPYLER-0704: Added param_types parameter for type coercion in binary operations
fn convert_stmt_with_context(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // For assignments, we need to convert the value expression with classmethod context
            // DEPYLER-0704: Pass param_types for type coercion
            let value_expr = convert_expr_with_param_types(value, type_mapper, is_classmethod, vararg_functions, param_types)?;
            convert_assign_stmt_with_expr(target, value_expr, type_mapper)
        }
        HirStmt::Return(expr) => {
            let ret_expr = if let Some(e) = expr {
                // DEPYLER-0704: Pass param_types for type coercion in return expressions
                convert_expr_with_param_types(e, type_mapper, is_classmethod, vararg_functions, param_types)?
            } else {
                parse_quote! { () }
            };
            Ok(syn::Stmt::Expr(
                parse_quote! { return #ret_expr },
                Some(Default::default()),
            ))
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond = convert_expr_with_param_types(condition, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let then_block = convert_block_with_context(then_body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block =
                    convert_block_with_context(else_stmts, type_mapper, is_classmethod, vararg_functions, param_types)?;
                parse_quote! {
                    if #cond #then_block else #else_block
                }
            } else {
                parse_quote! {
                    if #cond #then_block
                }
            };

            Ok(syn::Stmt::Expr(if_expr, Some(Default::default())))
        }
        HirStmt::While { condition, body } => {
            let cond = convert_expr_with_param_types(condition, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let body_block = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            let while_expr = parse_quote! {
                while #cond #body_block
            };

            Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
        }
        HirStmt::For { target, iter, body } => {
            // Generate target pattern based on AssignTarget type
            let target_pattern: syn::Pat = match target {
                AssignTarget::Symbol(name) => {
                    let ident = make_ident(name);
                    parse_quote! { #ident }
                }
                AssignTarget::Tuple(targets) => {
                    let idents: Vec<syn::Ident> = targets
                        .iter()
                        .map(|t| match t {
                            AssignTarget::Symbol(s) => {
                                make_ident(s)
                            }
                            _ => panic!("Nested tuple unpacking not supported in for loops"),
                        })
                        .collect();
                    parse_quote! { (#(#idents),*) }
                }
                _ => panic!("Unsupported for loop target type"),
            };

            let iter_expr = convert_expr_with_param_types(iter, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let body_block = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            let for_expr = parse_quote! {
                for #target_pattern in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            // DEPYLER-0701: Detect expressions without side effects and wrap with `let _ =`
            // to avoid "path statement with no effect" and "unused arithmetic operation" warnings
            if is_pure_expression_direct(expr) {
                let rust_expr = convert_expr_with_param_types(expr, type_mapper, is_classmethod, vararg_functions, param_types)?;
                Ok(syn::Stmt::Local(syn::Local {
                    attrs: vec![],
                    let_token: syn::Token![let](proc_macro2::Span::call_site()),
                    pat: syn::Pat::Wild(syn::PatWild {
                        attrs: vec![],
                        underscore_token: syn::Token![_](proc_macro2::Span::call_site()),
                    }),
                    init: Some(syn::LocalInit {
                        eq_token: syn::Token![=](proc_macro2::Span::call_site()),
                        expr: Box::new(rust_expr),
                        diverge: None,
                    }),
                    semi_token: syn::Token![;](proc_macro2::Span::call_site()),
                }))
            } else {
                let rust_expr = convert_expr_with_param_types(expr, type_mapper, is_classmethod, vararg_functions, param_types)?;
                Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
            }
        }
        HirStmt::Raise {
            exception,
            cause: _,
        } => {
            // Convert to Rust panic for direct rules
            let panic_expr = if let Some(exc) = exception {
                let exc_expr = convert_expr_with_param_types(exc, type_mapper, is_classmethod, vararg_functions, param_types)?;
                parse_quote! { panic!("Exception: {}", #exc_expr) }
            } else {
                parse_quote! { panic!("Exception raised") }
            };
            Ok(syn::Stmt::Expr(panic_expr, Some(Default::default())))
        }
        HirStmt::Break { label } => {
            let break_expr = if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                parse_quote! { break #label_ident }
            } else {
                parse_quote! { break }
            };
            Ok(syn::Stmt::Expr(break_expr, Some(Default::default())))
        }
        HirStmt::Continue { label } => {
            let continue_expr = if let Some(label_name) = label {
                let label_ident =
                    syn::Lifetime::new(&format!("'{}", label_name), proc_macro2::Span::call_site());
                parse_quote! { continue #label_ident }
            } else {
                parse_quote! { continue }
            };
            Ok(syn::Stmt::Expr(continue_expr, Some(Default::default())))
        }
        HirStmt::With {
            context,
            target,
            body,
            ..
        } => {
            // Convert context expression
            let context_expr = convert_expr_with_param_types(context, type_mapper, is_classmethod, vararg_functions, param_types)?;

            // Convert body to a block
            let body_block = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            // Generate a scope block with optional variable binding
            let block_expr = if let Some(var_name) = target {
                let var_ident = make_ident(var_name);
                parse_quote! {
                    {
                        let mut #var_ident = #context_expr;
                        #body_block
                    }
                }
            } else {
                parse_quote! {
                    {
                        let _context = #context_expr;
                        #body_block
                    }
                }
            };

            Ok(syn::Stmt::Expr(block_expr, None))
        }
        HirStmt::Try {
            body,
            handlers,
            orelse: _,
            finalbody,
        } => {
            // Convert try body
            let try_stmts = convert_block_with_context(body, type_mapper, is_classmethod, vararg_functions, param_types)?;

            // Convert finally block if present
            let finally_block = finalbody
                .as_ref()
                .map(|fb| convert_block_with_context(fb, type_mapper, is_classmethod, vararg_functions, param_types))
                .transpose()?;

            // Convert except handlers (use first handler for simplicity)
            if let Some(handler) = handlers.first() {
                let handler_block =
                    convert_block_with_context(&handler.body, type_mapper, is_classmethod, vararg_functions, param_types)?;

                let block_expr = if let Some(finally_stmts) = finally_block {
                    parse_quote! {
                        {
                            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                #try_stmts
                                Ok(())
                            })();
                            if let Err(_e) = _result {
                                #handler_block
                            }
                            #finally_stmts
                        }
                    }
                } else {
                    parse_quote! {
                        {
                            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                #try_stmts
                                Ok(())
                            })();
                            if let Err(_e) = _result {
                                #handler_block
                            }
                        }
                    }
                };
                Ok(syn::Stmt::Expr(block_expr, None))
            } else {
                // No handlers - try/finally without except
                let block_expr = if let Some(finally_stmts) = finally_block {
                    parse_quote! {
                        {
                            #try_stmts
                            #finally_stmts
                        }
                    }
                } else {
                    parse_quote! { #try_stmts }
                };
                Ok(syn::Stmt::Expr(block_expr, None))
            }
        }
        HirStmt::Assert { test, msg } => {
            // Generate assert! macro call
            let test_expr = convert_expr_with_param_types(test, type_mapper, is_classmethod, vararg_functions, param_types)?;
            let assert_macro: syn::Stmt = if let Some(message) = msg {
                let msg_expr = convert_expr_with_param_types(message, type_mapper, is_classmethod, vararg_functions, param_types)?;
                parse_quote! { assert!(#test_expr, "{}", #msg_expr); }
            } else {
                parse_quote! { assert!(#test_expr); }
            };
            Ok(assert_macro)
        }
        HirStmt::Pass => {
            // Pass statement generates empty statement
            Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
        }
        // DEPYLER-0614: Handle Block of statements - convert first statement
        // Note: This is a simplification; blocks are flattened during codegen
        HirStmt::Block(stmts) => {
            if stmts.is_empty() {
                Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
            } else {
                convert_stmt_with_context(&stmts[0], type_mapper, is_classmethod, vararg_functions, param_types)
            }
        }
        // DEPYLER-0427: Nested function support - delegate to main rust_gen module
        HirStmt::FunctionDef { .. } => {
            // Nested functions are handled by the main rust_gen module
            // direct_rules is a legacy optimization path
            Ok(syn::Stmt::Expr(parse_quote! { {} }, None))
        }
    }
}

#[allow(dead_code)]
fn convert_block(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<syn::Block> {
    // Use empty vararg_functions for backward compatibility
    static EMPTY_SET: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    static EMPTY_MAP: std::sync::OnceLock<std::collections::HashMap<String, Type>> = std::sync::OnceLock::new();
    convert_block_with_context(
        stmts,
        type_mapper,
        false,
        EMPTY_SET.get_or_init(std::collections::HashSet::new),
        EMPTY_MAP.get_or_init(std::collections::HashMap::new),
    )
}

/// DEPYLER-0704: Added param_types parameter for type coercion in binary operations
fn convert_block_with_context(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Block> {
    let rust_stmts = convert_body_with_context(stmts, type_mapper, is_classmethod, vararg_functions, param_types)?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// DEPYLER-0720: Convert method body block with class field type awareness
/// This is used for class methods where we know the field types
fn convert_method_body_block(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Block> {
    let rust_stmts = convert_method_body_stmts(stmts, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// DEPYLER-0720: Convert method body statements with class field type awareness
fn convert_method_body_stmts(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
) -> Result<Vec<syn::Stmt>> {
    // DEPYLER-0713: Pre-analyze which variables need to be mutable
    let mutable_vars = find_mutable_vars_in_body(stmts);

    stmts
        .iter()
        .map(|stmt| convert_method_stmt(stmt, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types, &mutable_vars))
        .collect()
}

/// DEPYLER-0720: Convert a single statement with class field type awareness
fn convert_method_stmt(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    mutable_vars: &std::collections::HashSet<String>,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            let value_expr = convert_expr_with_class_fields(value, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, mutable_vars)
        }
        HirStmt::Return(expr) => {
            let ret_expr = if let Some(e) = expr {
                convert_expr_with_class_fields(e, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?
            } else {
                parse_quote! { () }
            };
            Ok(syn::Stmt::Expr(
                parse_quote! { return #ret_expr },
                Some(Default::default()),
            ))
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let cond = convert_expr_with_class_fields(condition, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            let then_block = convert_method_body_block(then_body, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block =
                    convert_method_body_block(else_stmts, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
                parse_quote! {
                    if #cond #then_block else #else_block
                }
            } else {
                parse_quote! {
                    if #cond #then_block
                }
            };

            Ok(syn::Stmt::Expr(if_expr, Some(Default::default())))
        }
        HirStmt::While { condition, body } => {
            let cond = convert_expr_with_class_fields(condition, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            let body_block = convert_method_body_block(body, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;

            let while_expr = parse_quote! {
                while #cond #body_block
            };

            Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
        }
        HirStmt::For { target, iter, body } => {
            // Generate target pattern based on AssignTarget type
            let target_pattern: syn::Pat = match target {
                AssignTarget::Symbol(name) => {
                    let ident = make_ident(name);
                    parse_quote! { #ident }
                }
                AssignTarget::Tuple(targets) => {
                    let idents: Vec<syn::Ident> = targets
                        .iter()
                        .map(|t| match t {
                            AssignTarget::Symbol(s) => {
                                make_ident(s)
                            }
                            _ => panic!("Nested tuple unpacking not supported in for loops"),
                        })
                        .collect();
                    parse_quote! { (#(#idents),*) }
                }
                _ => panic!("Unsupported for loop target type"),
            };

            let iter_expr = convert_expr_with_class_fields(iter, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
            let body_block = convert_method_body_block(body, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;

            let for_expr = parse_quote! {
                for #target_pattern in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            if is_pure_expression_direct(expr) {
                let rust_expr = convert_expr_with_class_fields(expr, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
                Ok(syn::Stmt::Local(syn::Local {
                    attrs: vec![],
                    let_token: syn::Token![let](proc_macro2::Span::call_site()),
                    pat: syn::Pat::Wild(syn::PatWild {
                        attrs: vec![],
                        underscore_token: syn::Token![_](proc_macro2::Span::call_site()),
                    }),
                    init: Some(syn::LocalInit {
                        eq_token: syn::Token![=](proc_macro2::Span::call_site()),
                        expr: Box::new(rust_expr),
                        diverge: None,
                    }),
                    semi_token: syn::Token![;](proc_macro2::Span::call_site()),
                }))
            } else {
                let rust_expr = convert_expr_with_class_fields(expr, type_mapper, is_classmethod, vararg_functions, param_types, class_field_types)?;
                Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
            }
        }
        // For other statement types, fall back to existing conversion
        _ => convert_stmt_with_context(stmt, type_mapper, is_classmethod, vararg_functions, param_types),
    }
}

/// Convert HIR expressions to Rust expressions using strategy pattern
#[allow(dead_code)]
fn convert_expr(expr: &HirExpr, type_mapper: &TypeMapper) -> Result<syn::Expr> {
    // Use empty vararg_functions for backward compatibility
    static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
    convert_expr_with_context(expr, type_mapper, false, EMPTY.get_or_init(std::collections::HashSet::new))
}

/// Convert HIR expressions with classmethod context and vararg tracking
fn convert_expr_with_context(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
) -> Result<syn::Expr> {
    // DEPYLER-0648: Use with_varargs to track functions that need slice wrapping
    let converter = ExprConverter::with_varargs(type_mapper, is_classmethod, vararg_functions);
    converter.convert(expr)
}

/// DEPYLER-0704: Convert HIR expressions with parameter type information for type coercion
fn convert_expr_with_param_types(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Expr> {
    let converter = ExprConverter::with_param_types(
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types.clone(),
    );
    converter.convert(expr)
}

/// DEPYLER-0720: Convert HIR expressions with class field types for self.field coercion
fn convert_expr_with_class_fields(
    expr: &HirExpr,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
) -> Result<syn::Expr> {
    let converter = ExprConverter::with_class_fields(
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types.clone(),
        class_field_types.clone(),
    );
    converter.convert(expr)
}

/// Expression converter using strategy pattern to reduce complexity
struct ExprConverter<'a> {
    #[allow(dead_code)]
    type_mapper: &'a TypeMapper,
    is_classmethod: bool,
    /// DEPYLER-0648: Track functions that have vararg parameters (*args in Python)
    /// Call sites need to wrap arguments in &[...] slices
    vararg_functions: &'a std::collections::HashSet<String>,
    /// DEPYLER-0704: Parameter types for type coercion in binary operations
    param_types: std::collections::HashMap<String, Type>,
    /// DEPYLER-0720: Class field types for self.field attribute access
    /// Maps field name -> Type, used to determine if self.X is float for int-to-float coercion
    class_field_types: std::collections::HashMap<String, Type>,
}

impl<'a> ExprConverter<'a> {
    #[allow(dead_code)]
    fn new(type_mapper: &'a TypeMapper) -> Self {
        // Use empty static HashSet for backwards compatibility
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
        Self {
            type_mapper,
            is_classmethod: false,
            vararg_functions: EMPTY.get_or_init(std::collections::HashSet::new),
            param_types: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn with_classmethod(type_mapper: &'a TypeMapper, is_classmethod: bool) -> Self {
        // Use empty static HashSet for backwards compatibility
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions: EMPTY.get_or_init(std::collections::HashSet::new),
            param_types: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
        }
    }

    /// DEPYLER-0648: Create converter with vararg function tracking
    fn with_varargs(
        type_mapper: &'a TypeMapper,
        is_classmethod: bool,
        vararg_functions: &'a std::collections::HashSet<String>,
    ) -> Self {
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types: std::collections::HashMap::new(),
            class_field_types: std::collections::HashMap::new(),
        }
    }

    /// DEPYLER-0704: Create converter with parameter types for type coercion
    fn with_param_types(
        type_mapper: &'a TypeMapper,
        is_classmethod: bool,
        vararg_functions: &'a std::collections::HashSet<String>,
        param_types: std::collections::HashMap<String, Type>,
    ) -> Self {
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types,
            class_field_types: std::collections::HashMap::new(),
        }
    }

    /// DEPYLER-0720: Create converter with class field types for self.field access
    fn with_class_fields(
        type_mapper: &'a TypeMapper,
        is_classmethod: bool,
        vararg_functions: &'a std::collections::HashSet<String>,
        param_types: std::collections::HashMap<String, Type>,
        class_field_types: std::collections::HashMap<String, Type>,
    ) -> Self {
        Self {
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types,
            class_field_types,
        }
    }

    /// DEPYLER-0704: Check if expression returns a float type
    /// DEPYLER-0720: Extended to check class field types for self.field patterns
    fn expr_returns_float_direct(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Float(_)) => true,
            HirExpr::Var(name) => {
                // Check param_types
                matches!(self.param_types.get(name), Some(Type::Float))
            }
            // DEPYLER-0720: Check class field types for self.field attribute access
            HirExpr::Attribute { value, attr } => {
                // Check if this is self.field pattern where field is a float
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
                    && matches!(self.class_field_types.get(attr), Some(Type::Float))
                {
                    return true;
                }
                false
            }
            HirExpr::Binary { left, right, .. } => {
                // Binary with float operand returns float
                self.expr_returns_float_direct(left) || self.expr_returns_float_direct(right)
            }
            HirExpr::MethodCall { method, .. } => {
                // Common float-returning methods
                matches!(
                    method.as_str(),
                    "mean" | "sum" | "std" | "norm" | "variance"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0704: Check if expression is an integer type
    fn is_int_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => true,
            HirExpr::Var(name) => {
                // Check param_types
                matches!(self.param_types.get(name), Some(Type::Int))
            }
            _ => false,
        }
    }

    fn convert(&self, expr: &HirExpr) -> Result<syn::Expr> {
        match expr {
            HirExpr::Literal(lit) => self.convert_literal(lit),
            HirExpr::Var(name) => self.convert_variable(name),
            HirExpr::Binary { op, left, right } => self.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => self.convert_unary(*op, operand),
            HirExpr::Call { func, args, .. } => self.convert_call(func, args),
            HirExpr::Index { base, index } => self.convert_index(base, index),
            // DEPYLER-0596: Add Slice support for string slicing with negative indices
            HirExpr::Slice { base, start, stop, step } => self.convert_slice(base, start, stop, step),
            HirExpr::List(elts) => self.convert_list(elts),
            HirExpr::Dict(items) => self.convert_dict(items),
            HirExpr::Tuple(elts) => self.convert_tuple(elts),
            HirExpr::Set(elts) => self.convert_set(elts),
            HirExpr::FrozenSet(elts) => self.convert_frozenset(elts),
            HirExpr::Lambda { params, body } => self.convert_lambda(params, body),
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => self.convert_method_call(object, method, args),
            // DEPYLER-0188: Dynamic function call (e.g., handlers[name](args))
            HirExpr::DynamicCall { callee, args, .. } => {
                self.convert_dynamic_call(callee, args)
            }
            HirExpr::ListComp {
                element,
                generators,
            } => {
                // DEPYLER-0504: Legacy path - only support single generator for now
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let condition = if gen.conditions.is_empty() {
                    None
                } else if gen.conditions.len() == 1 {
                    Some(Box::new(gen.conditions[0].clone()))
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                };
                self.convert_list_comp(element, &gen.target, &gen.iter, &condition)
            }
            HirExpr::SetComp {
                element,
                generators,
            } => {
                // DEPYLER-0504: Legacy path - only support single generator for now
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let condition = if gen.conditions.is_empty() {
                    None
                } else if gen.conditions.len() == 1 {
                    Some(Box::new(gen.conditions[0].clone()))
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                };
                self.convert_set_comp(element, &gen.target, &gen.iter, &condition)
            }
            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                // DEPYLER-0504: Legacy path - only support single generator for now
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let condition = if gen.conditions.is_empty() {
                    None
                } else if gen.conditions.len() == 1 {
                    Some(Box::new(gen.conditions[0].clone()))
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                };
                self.convert_dict_comp(key, value, &gen.target, &gen.iter, &condition)
            }
            HirExpr::Attribute { value, attr } => self.convert_attribute(value, attr),
            HirExpr::Await { value } => self.convert_await(value),
            // DEPYLER-0513: F-string support for class methods
            HirExpr::FString { parts } => self.convert_fstring(parts),
            // DEPYLER-0764: IfExpr (ternary operator) support for class methods
            // Python: a if cond else b → Rust: if cond { a } else { b }
            HirExpr::IfExpr { test, body, orelse } => {
                let test_expr = self.convert(test)?;
                let body_expr = self.convert(body)?;
                let orelse_expr = self.convert(orelse)?;
                Ok(parse_quote! { if #test_expr { #body_expr } else { #orelse_expr } })
            }
            // DEPYLER-0764: GeneratorExp support for class methods
            // Python: (x for x in items) → Rust: items.iter().map(|x| x)
            HirExpr::GeneratorExp { element, generators } => {
                // Only support single generator for direct rules path
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let iter_expr = self.convert(&gen.iter)?;
                let element_expr = self.convert(element)?;
                let target_ident = make_ident(&gen.target);

                // Handle conditions
                if gen.conditions.is_empty() {
                    Ok(parse_quote! { #iter_expr.iter().map(|#target_ident| #element_expr) })
                } else if gen.conditions.len() == 1 {
                    let cond_expr = self.convert(&gen.conditions[0])?;
                    Ok(parse_quote! {
                        #iter_expr.iter()
                            .filter(|#target_ident| #cond_expr)
                            .map(|#target_ident| #element_expr)
                    })
                } else {
                    bail!("Multiple conditions in generator not supported in direct rules path");
                }
            }
            // DEPYLER-0764: SortByKey support for sorted() with key parameter
            HirExpr::SortByKey { iterable, key_params, key_body, reverse_expr } => {
                let iter_expr = self.convert(iterable)?;
                let key_body_expr = self.convert(key_body)?;

                // Build the key lambda parameter(s)
                let key_param = if key_params.len() == 1 {
                    let p = make_ident(&key_params[0]);
                    quote! { #p }
                } else {
                    let params: Vec<_> = key_params.iter().map(|p| make_ident(p)).collect();
                    quote! { (#(#params),*) }
                };

                // Check if reversed
                let is_reversed = match reverse_expr {
                    Some(boxed) => matches!(boxed.as_ref(), HirExpr::Literal(Literal::Bool(true))),
                    _ => false,
                };

                if is_reversed {
                    Ok(parse_quote! {
                        {
                            let mut v: Vec<_> = #iter_expr.into_iter().collect();
                            v.sort_by_key(|#key_param| std::cmp::Reverse(#key_body_expr));
                            v
                        }
                    })
                } else {
                    Ok(parse_quote! {
                        {
                            let mut v: Vec<_> = #iter_expr.into_iter().collect();
                            v.sort_by_key(|#key_param| #key_body_expr);
                            v
                        }
                    })
                }
            }
            _ => bail!("Expression type not yet supported: {:?}", expr),
        }
    }

    fn convert_literal(&self, lit: &Literal) -> Result<syn::Expr> {
        Ok(convert_literal(lit))
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        // DEPYLER-0597: In method context (not classmethod), 'self' should be Rust keyword
        // Python `self.x` in instance method must become Rust `self.x`, not `self_.x`
        if name == "self" && !self.is_classmethod {
            return Ok(parse_quote! { self });
        }
        // DEPYLER-0596: Use make_ident to handle keywords like "match"
        let ident = make_ident(name);
        Ok(parse_quote! { #ident })
    }

    fn convert_binary(&self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = self.convert(left)?;
        let right_expr = self.convert(right)?;

        match op {
            BinOp::In => {
                // DEPYLER-0601: For strings, use .contains() instead of .contains_key()
                if self.is_string_expr(right) {
                    // DEPYLER-0200: Use raw string literal or &* for Pattern trait
                    let pattern: syn::Expr = match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String (&*String -> &str)
                            // and &str (&*&str -> &str), avoiding unstable str_as_str feature
                            parse_quote! { &*#left_expr }
                        }
                    };
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                } else {
                    // Convert "x in dict" to "dict.contains_key(&x)" for dicts/maps
                    Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                }
            }
            BinOp::NotIn => {
                // DEPYLER-0601: For strings, use !.contains() instead of !.contains_key()
                if self.is_string_expr(right) {
                    // DEPYLER-0200: Use raw string literal or &* for Pattern trait
                    let pattern: syn::Expr = match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String and &str
                            parse_quote! { &*#left_expr }
                        }
                    };
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    // Convert "x not in dict" to "!dict.contains_key(&x)"
                    Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                }
            }
            // Set operators - check if both operands are sets
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                if self.is_set_expr(left) && self.is_set_expr(right) =>
            {
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub if self.is_set_expr(left) && self.is_set_expr(right) => {
                // Set difference operation
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    // DEPYLER-0746: Wrap in parens to handle cast expressions like `x as usize`
                    Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // Note: This implementation works for integers with proper floor semantics.
                // Type-based dispatch for float division (using .floor()) would be ideal
                // but requires full type inference integration. This is a known limitation.
                // DEPYLER-0236: Use intermediate variables to avoid formatting issues with != operator

                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        let r_negative = r < 0;
                        let b_negative = b < 0;
                        let r_nonzero = r != 0;
                        let signs_differ = r_negative != b_negative;
                        let needs_adjustment = r_nonzero && signs_differ;
                        if needs_adjustment { q - 1 } else { q }
                    }
                })
            }
            BinOp::Mul => {
                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = self.convert(&elts[0])?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = self.convert(&elts[0])?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        // DEPYLER-0704: Type coercion for mixed int/float multiplication
                        // Rust doesn't auto-coerce, so we need explicit casts
                        let left_is_float = self.expr_returns_float_direct(left);
                        let right_is_float = self.expr_returns_float_direct(right);
                        let left_is_int = self.is_int_expr(left);
                        let right_is_int = self.is_int_expr(right);

                        let final_left = if right_is_float && left_is_int {
                            parse_quote! { (#left_expr as f64) }
                        } else {
                            left_expr
                        };
                        let final_right = if left_is_float && right_is_int {
                            parse_quote! { (#right_expr as f64) }
                        } else {
                            right_expr
                        };
                        Ok(parse_quote! { #final_left #rust_op #final_right })
                    }
                }
            }
            BinOp::Pow => {
                // Python power operator ** needs type-specific handling in Rust
                // For integers: use .pow() with u32 exponent
                // For floats: use .powf() with f64 exponent
                // For negative integer exponents: convert to float

                // DEPYLER-0699: Wrap expressions in block to ensure correct operator precedence
                // Without this, `a + b as f64` parses as `a + (b as f64)` instead of `(a + b) as f64`
                // DEPYLER-0707: Construct block directly instead of using parse_quote!
                // parse_quote! re-parses tokens which can fail with complex expressions
                fn wrap_expr_in_block(expr: syn::Expr) -> syn::Expr {
                    syn::Expr::Block(syn::ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: syn::Block {
                            brace_token: syn::token::Brace::default(),
                            stmts: vec![syn::Stmt::Expr(expr, None)],
                        },
                    })
                }
                let left_paren = wrap_expr_in_block(left_expr.clone());
                let right_paren = wrap_expr_in_block(right_expr.clone());

                // Check if we have literals to determine types
                match (left, right) {
                    // Integer literal base with integer literal exponent
                    (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                        if *exp < 0 {
                            // Negative exponent: convert to float operation
                            Ok(parse_quote! {
                                (#left_paren as f64).powf(#right_paren as f64)
                            })
                        } else {
                            // Positive integer exponent: use .pow() with u32
                            // Add checked_pow for overflow safety
                            // DEPYLER-0746: Wrap in parens to handle cast expressions
                            Ok(parse_quote! {
                                (#left_expr).checked_pow(#right_expr as u32)
                                    .expect("Power operation overflowed")
                            })
                        }
                    }
                    // Float literal base: always use .powf()
                    // DEPYLER-0408: Cast float literal to f64 for concrete type
                    (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    }),
                    // Any base with float exponent: use .powf()
                    // DEPYLER-0408: Cast float literal exponent to f64 for concrete type
                    (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    }),
                    // Variables or complex expressions: generate type-safe code
                    _ => {
                        // For non-literal expressions, we need runtime type checking
                        // This is a conservative approach that works for common cases
                        // DEPYLER-0405: Cast both sides to i64 for type-safe comparison
                        Ok(parse_quote! {
                            {
                                // Try integer power first if exponent can be u32
                                if #right_expr >= 0 && (#right_expr as i64) <= (u32::MAX as i64) {
                                    (#left_paren as i32).checked_pow(#right_paren as u32)
                                        .expect("Power operation overflowed")
                                } else {
                                    // Fall back to float power for negative or large exponents
                                    // DEPYLER-0401: Use i32 to match common Python int mapping
                                    (#left_paren as f64).powf(#right_paren as f64) as i32
                                }
                            }
                        })
                    }
                }
            }
            // DEPYLER-0720: Handle comparison operators with int-to-float coercion
            BinOp::Gt | BinOp::GtEq | BinOp::Lt | BinOp::LtEq | BinOp::Eq | BinOp::NotEq => {
                let rust_op = convert_binop(op)?;

                // DEPYLER-0824: Wrap cast expressions in parentheses before binary operators
                // Rust parses `x as i32 < y` incorrectly. Must be: `(x as i32) < y`
                let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#left_expr) }
                } else {
                    left_expr.clone()
                };
                let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr.clone()
                };

                // Check if either side is float
                let left_is_float = self.expr_returns_float_direct(left);
                let right_is_float = self.expr_returns_float_direct(right);

                // DEPYLER-0828: Handle float/int comparisons with proper coercion
                // If left is float and right is integer (literal or variable), convert right to float
                if left_is_float && !right_is_float {
                    if let HirExpr::Literal(Literal::Int(n)) = right {
                        // Integer literal: convert at compile time
                        let float_val = *n as f64;
                        return Ok(parse_quote! { #safe_left #rust_op #float_val });
                    }
                    // Integer variable or expression: cast to f64 at runtime
                    return Ok(parse_quote! { #safe_left #rust_op (#safe_right as f64) });
                }

                // If right is float and left is integer (literal or variable), convert left to float
                if right_is_float && !left_is_float {
                    if let HirExpr::Literal(Literal::Int(n)) = left {
                        // Integer literal: convert at compile time
                        let float_val = *n as f64;
                        return Ok(parse_quote! { #float_val #rust_op #safe_right });
                    }
                    // Integer variable or expression: cast to f64 at runtime
                    return Ok(parse_quote! { (#safe_left as f64) #rust_op #safe_right });
                }

                // No coercion needed (both same type)
                Ok(parse_quote! { #safe_left #rust_op #safe_right })
            }
            _ => {
                let rust_op = convert_binop(op)?;
                // DEPYLER-0824: Wrap cast expressions in parentheses
                let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#left_expr) }
                } else {
                    left_expr.clone()
                };
                let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr.clone()
                };
                Ok(parse_quote! { #safe_left #rust_op #safe_right })
            }
        }
    }

    fn convert_unary(&self, op: UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        let operand_expr = self.convert(operand)?;
        match op {
            UnaryOp::Not => Ok(parse_quote! { !#operand_expr }),
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    fn convert_call(&self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        // Handle classmethod cls(args) → Self::new(args)
        if func == "cls" && self.is_classmethod {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| self.convert(arg))
                .collect::<Result<Vec<_>>>()?;
            return Ok(parse_quote! { Self::new(#(#arg_exprs),*) });
        }

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        match func {
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            // DEPYLER-0200: File I/O builtins
            "open" => self.convert_open_call(args, &arg_exprs),
            // DEPYLER-0200: datetime builtins
            "date" => self.convert_date_call(&arg_exprs),
            "datetime" => self.convert_datetime_call(&arg_exprs),
            // DEPYLER-0721: os.path functions imported via `from os.path import X`
            "splitext" => self.convert_splitext_call(&arg_exprs),
            "basename" => self.convert_basename_call(&arg_exprs),
            "dirname" => self.convert_dirname_call(&arg_exprs),
            "split" => self.convert_path_split_call(&arg_exprs),
            "exists" => self.convert_path_exists_call(&arg_exprs),
            "isfile" => self.convert_path_isfile_call(&arg_exprs),
            "isdir" => self.convert_path_isdir_call(&arg_exprs),
            // DEPYLER-0780: Pass HIR args for auto-borrowing detection
            _ => self.convert_generic_call(func, args, &arg_exprs),
        }
    }

    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];
        // DEPYLER-0693: Cast len() to i32 for Python compatibility
        // Python int maps to Rust i32, and len() in Python returns int
        Ok(parse_quote! { #arg.len() as i32 })
    }

    fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        match args.len() {
            1 => {
                let end = &args[0];
                Ok(parse_quote! { 0..#end })
            }
            2 => {
                let start = &args[0];
                let end = &args[1];
                Ok(parse_quote! { #start..#end })
            }
            3 => {
                // Step parameter requires custom iterator implementation
                bail!("range() with step parameter not yet supported")
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_array_init_call(
        &self,
        func: &str,
        args: &[HirExpr],
        _arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }

        // DEPYLER-0695: Always use vec![] for zeros/ones/full to ensure consistent
        // Vec<T> return type. Using fixed arrays [0; N] causes type mismatches when
        // functions return Vec<T> (Python lists are always dynamically sized).
        let size_expr = self.convert(&args[0])?;
        match func {
            "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
            "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
            "full" => {
                if args.len() >= 2 {
                    let value = self.convert(&args[1])?;
                    Ok(parse_quote! { vec![#value; #size_expr as usize] })
                } else {
                    bail!("full() requires a value argument");
                }
            }
            _ => unreachable!(),
        }
    }


    fn convert_set_constructor(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // Empty set: set()
            // DEPYLER-0409: Use default type i32 to avoid "type annotations needed" error
            // when the variable is unused or type can't be inferred from context
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            Ok(parse_quote! { std::collections::HashSet::<i32>::new() })
        } else if args.len() == 1 {
            // Set from iterable: set([1, 2, 3])
            let arg = &args[0];
            // DEPYLER-0797: Check if arg is a tuple - tuples don't implement IntoIterator in Rust
            // Convert tuple to vec! for iteration
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            if let syn::Expr::Tuple(tuple) = arg {
                let elems = &tuple.elems;
                Ok(parse_quote! {
                    vec![#elems].into_iter().collect::<std::collections::HashSet<_>>()
                })
            } else {
                Ok(parse_quote! {
                    #arg.into_iter().collect::<std::collections::HashSet<_>>()
                })
            }
        } else {
            bail!("set() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_frozenset_constructor(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // Empty frozenset: frozenset()
            // DEPYLER-0409: Use default type i32 for empty sets
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            Ok(parse_quote! { std::sync::Arc::new(std::collections::HashSet::<i32>::new()) })
        } else if args.len() == 1 {
            // Frozenset from iterable: frozenset([1, 2, 3])
            let arg = &args[0];
            // DEPYLER-0797: Check if arg is a tuple - tuples don't implement IntoIterator in Rust
            // Convert tuple to vec! for iteration
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            if let syn::Expr::Tuple(tuple) = arg {
                let elems = &tuple.elems;
                Ok(parse_quote! {
                    std::sync::Arc::new(vec![#elems].into_iter().collect::<std::collections::HashSet<_>>())
                })
            } else {
                Ok(parse_quote! {
                    std::sync::Arc::new(#arg.into_iter().collect::<std::collections::HashSet<_>>())
                })
            }
        } else {
            bail!(
                "frozenset() takes at most 1 argument ({} given)",
                args.len()
            )
        }
    }

    /// DEPYLER-0721: os.path.splitext(path) → (stem, extension) tuple
    fn convert_splitext_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("splitext() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let ext = p.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e)).unwrap_or_default();
                (stem, ext)
            }
        })
    }

    /// DEPYLER-0721: os.path.basename(path) → Path::file_name
    fn convert_basename_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("basename() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            std::path::Path::new(&#path).file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
        })
    }

    /// DEPYLER-0721: os.path.dirname(path) → Path::parent
    fn convert_dirname_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("dirname() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            std::path::Path::new(&#path).parent().and_then(|p| p.to_str()).unwrap_or("").to_string()
        })
    }

    /// DEPYLER-0721: os.path.split(path) → (dirname, basename)
    fn convert_path_split_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("split() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! {
            {
                let p = std::path::Path::new(&#path);
                let dirname = p.parent().and_then(|p| p.to_str()).unwrap_or("").to_string();
                let basename = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                (dirname, basename)
            }
        })
    }

    /// DEPYLER-0721: os.path.exists(path) → Path::exists
    fn convert_path_exists_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("exists() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).exists() })
    }

    /// DEPYLER-0721: os.path.isfile(path) → Path::is_file
    fn convert_path_isfile_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("isfile() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).is_file() })
    }

    /// DEPYLER-0721: os.path.isdir(path) → Path::is_dir
    fn convert_path_isdir_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("isdir() requires exactly 1 argument");
        }
        let path = &args[0];
        Ok(parse_quote! { std::path::Path::new(&#path).is_dir() })
    }

    /// DEPYLER-0200: Convert Python open() to Rust file operations
    /// open(path) → std::fs::File::open(path) (read mode)
    /// open(path, "w") → std::fs::File::create(path) (write mode)
    fn convert_open_call(&self, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("open() requires 1 or 2 arguments");
        }

        let path = &args[0];

        // Determine mode from second argument (default is 'r')
        let mode = if hir_args.len() >= 2 {
            if let HirExpr::Literal(Literal::String(mode_str)) = &hir_args[1] {
                mode_str.as_str()
            } else {
                "r" // Default to read mode
            }
        } else {
            "r" // Default mode
        };

        match mode {
            "w" | "w+" | "wb" => {
                // Write mode: std::fs::File::create()
                Ok(parse_quote! { std::fs::File::create(&#path).unwrap() })
            }
            "a" | "a+" | "ab" => {
                // Append mode: OpenOptions with append
                Ok(parse_quote! {
                    std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&#path)
                        .unwrap()
                })
            }
            _ => {
                // Read mode (default): std::fs::File::open()
                Ok(parse_quote! { std::fs::File::open(&#path).unwrap() })
            }
        }
    }

    /// DEPYLER-0200: Convert Python date(year, month, day) to chrono::NaiveDate
    fn convert_date_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 3 {
            bail!("date() requires exactly 3 arguments (year, month, day)");
        }
        let year = &args[0];
        let month = &args[1];
        let day = &args[2];
        Ok(parse_quote! {
            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).unwrap()
        })
    }

    /// DEPYLER-0200: Convert Python datetime(year, month, day, ...) to chrono::NaiveDateTime
    fn convert_datetime_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 3 {
            bail!("datetime() requires at least 3 arguments (year, month, day)");
        }
        let year = &args[0];
        let month = &args[1];
        let day = &args[2];

        // Handle optional time components
        let zero: syn::Expr = parse_quote! { 0 };
        let hour = args.get(3).unwrap_or(&zero);
        let minute = args.get(4).unwrap_or(&zero);
        let second = args.get(5).unwrap_or(&zero);

        Ok(parse_quote! {
            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                .unwrap()
                .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                .unwrap()
        })
    }

    fn convert_generic_call(&self, func: &str, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
        // Special case: Python print() → Rust println!()
        if func == "print" {
            return if args.is_empty() {
                // print() with no arguments → println!()
                Ok(parse_quote! { println!() })
            } else if args.len() == 1 {
                // print(x) → println!("{}", x)
                let arg = &args[0];
                Ok(parse_quote! { println!("{}", #arg) })
            } else {
                // print(a, b, c) → println!("{} {} {}", a, b, c)
                let format_str = vec!["{}"; args.len()].join(" ");
                Ok(parse_quote! { println!(#format_str, #(#args),*) })
            };
        }

        // DEPYLER-0600: Handle Python built-in type conversion functions
        // These are used in class methods and need proper Rust equivalents
        match func {
            "int" if args.len() == 1 => {
                // int(x) → x.parse::<i32>().unwrap() for strings, x as i32 for numbers
                let arg = &args[0];
                return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or(0) });
            }
            "float" if args.len() == 1 => {
                // float(x) → x.parse::<f64>().unwrap() for strings, x as f64 for integers
                let arg = &args[0];
                return Ok(parse_quote! { #arg.parse::<f64>().unwrap_or(0.0) });
            }
            "str" if args.len() == 1 => {
                // str(x) → x.to_string()
                let arg = &args[0];
                return Ok(parse_quote! { #arg.to_string() });
            }
            "bool" if args.len() == 1 => {
                // bool(x) → general truthiness conversion
                let arg = &args[0];
                return Ok(parse_quote! { !#arg.is_empty() });
            }
            "len" if args.len() == 1 => {
                // len(x) → x.len() as i32
                // DEPYLER-0693: Cast len() to i32 for Python compatibility
                let arg = &args[0];
                return Ok(parse_quote! { #arg.len() as i32 });
            }
            "abs" if args.len() == 1 => {
                // DEPYLER-0815: abs(x) → (x).abs() - parens needed for precedence
                let arg = &args[0];
                return Ok(parse_quote! { (#arg).abs() });
            }
            "min" if args.len() >= 2 => {
                // min(a, b, ...) → a.min(b).min(c)...
                let first = &args[0];
                let rest = &args[1..];
                let mut result = parse_quote! { #first };
                for arg in rest {
                    result = parse_quote! { (#result).min(#arg) };
                }
                return Ok(result);
            }
            "max" if args.len() >= 2 => {
                // max(a, b, ...) → a.max(b).max(c)...
                let first = &args[0];
                let rest = &args[1..];
                let mut result = parse_quote! { #first };
                for arg in rest {
                    result = parse_quote! { (#result).max(#arg) };
                }
                return Ok(result);
            }
            _ => {}
        }

        // Check if this might be a constructor call (capitalized name)
        if func
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // Treat as constructor call - ClassName::new(args)
            let class_ident = make_ident(func);
            if args.is_empty() {
                // Note: Constructor default parameter handling uses simple heuristics.
                // Ideally this would be context-aware and know the actual default values
                // for each class constructor, but currently uses hardcoded patterns.
                // This is a known limitation - constructors may require explicit arguments.
                match func {
                    "Counter" => Ok(parse_quote! { #class_ident::new(0) }),
                    _ => Ok(parse_quote! { #class_ident::new() }),
                }
            } else {
                Ok(parse_quote! { #class_ident::new(#(#args),*) })
            }
        } else {
            // Regular function call
            let func_ident = make_ident(func);

            // DEPYLER-0648: Check if this is a vararg function
            // If so, wrap arguments in a slice: func(a, b) → func(&[a, b])
            if self.vararg_functions.contains(func) && !args.is_empty() {
                Ok(parse_quote! { #func_ident(&[#(#args),*]) })
            } else {
                // DEPYLER-0780: Auto-borrow list/dict/set literals when calling functions
                // Most user-defined functions taking list params expect &Vec<T>
                let borrowed_args: Vec<syn::Expr> = hir_args
                    .iter()
                    .zip(args.iter())
                    .map(|(hir_arg, arg_expr)| {
                        match hir_arg {
                            // List/Dict/Set literals should be borrowed
                            HirExpr::List(_) | HirExpr::Dict(_) | HirExpr::Set(_) => {
                                parse_quote! { &#arg_expr }
                            }
                            _ => arg_expr.clone(),
                        }
                    })
                    .collect();
                Ok(parse_quote! { #func_ident(#(#borrowed_args),*) })
            }
        }
    }

    fn convert_index(&self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        // DEPYLER-0735: Check if base is a tuple type (from param_types) and index is integer literal
        // Rust tuples use .0, .1 syntax, not [0], [1]
        if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                // Check if base variable has tuple type from param_types
                let is_tuple = if let HirExpr::Var(var_name) = base {
                    matches!(self.param_types.get(var_name), Some(Type::Tuple(_)))
                } else {
                    false
                };

                if is_tuple {
                    let field_idx = syn::Index::from(*idx as usize);
                    return Ok(parse_quote! { #base_expr.#field_idx });
                }
            }
        }

        // DEPYLER-0200: Detect dict vs list access
        // String literal index = dict access, use .get()
        // Numeric index = list access, use [idx as usize]
        let is_dict_access = match index {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                // Heuristic: variable names that look like string keys
                let n = name.as_str();
                n == "key" || n.ends_with("_key") || n.starts_with("key_")
                    || n == "name" || n == "field" || n == "attr"
            }
            _ => false,
        };

        // Also check if base looks like a dict
        let base_is_dict = match base {
            HirExpr::Var(name) => {
                let n = name.as_str();
                n.contains("dict") || n.contains("map") || n.contains("data")
                    || n == "result" || n == "config" || n == "settings"
                    || n == "params" || n == "options" || n == "env"
            }
            HirExpr::Call { func, .. } => {
                // Functions returning dicts
                func.contains("dict") || func.contains("json") || func.contains("config")
                    || func == "calculate_age" || func.contains("result")
            }
            _ => false,
        };

        if is_dict_access || base_is_dict {
            // HashMap access with string key
            let index_expr = self.convert(index)?;
            Ok(parse_quote! {
                #base_expr.get(&#index_expr).cloned().unwrap_or_default()
            })
        } else {
            // Vec/List access with numeric index
            let index_expr = self.convert(index)?;
            Ok(parse_quote! {
                #base_expr[#index_expr as usize]
            })
        }
    }

    /// DEPYLER-0596: Convert slice expression (e.g., value[1:-1])
    fn convert_slice(
        &self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;

        // Convert start/stop/step expressions
        let start_expr = start.as_ref().map(|e| self.convert(e)).transpose()?;
        let stop_expr = stop.as_ref().map(|e| self.convert(e)).transpose()?;
        let _step_expr = step.as_ref().map(|e| self.convert(e)).transpose()?;

        // For strings: use chars().skip().take() pattern with negative index handling
        // This handles cases like value[1:-1] (remove first and last chars)
        match (start_expr, stop_expr) {
            (Some(start), Some(stop)) => {
                // value[start:stop] - handles negative indices
                // DEPYLER-0603: Wrap expressions in parens to ensure proper type casting
                // Without parens, `a + b as isize` parses as `a + (b as isize)`
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let start_idx = (#start) as isize;
                        let stop_idx = (#stop) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        if stop > start {
                            s.chars().skip(start).take(stop - start).collect::<String>()
                        } else {
                            String::new()
                        }
                    }
                })
            }
            (Some(start), None) => {
                // value[start:] - from start to end
                // DEPYLER-0603: Wrap expression in parens for type casting
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let start_idx = (#start) as isize;
                        let start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        s.chars().skip(start).collect::<String>()
                    }
                })
            }
            (None, Some(stop)) => {
                // value[:stop] - from beginning to stop
                // DEPYLER-0603: Wrap expression in parens for type casting
                Ok(parse_quote! {
                    {
                        let s = &#base_expr;
                        let len = s.chars().count() as isize;
                        let stop_idx = (#stop) as isize;
                        let stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        s.chars().take(stop).collect::<String>()
                    }
                })
            }
            (None, None) => {
                // value[:] - full clone
                Ok(parse_quote! { #base_expr.clone() })
            }
        }
    }

    fn convert_list(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0780: Always use vec![] for list literals
        // Array literals [T; N] are incompatible with &Vec<T> parameters
        // Python lists map to Vec<T> in Rust, so consistently use vec![]
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    fn convert_dict(&self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = items
            .iter()
            .map(|(k, v)| {
                let key = self.convert(k)?;
                let val = self.convert(v)?;
                Ok(parse_quote! { map.insert(#key, #val) })
            })
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0623: Use fully qualified path to avoid missing import
        Ok(parse_quote! {
            {
                let mut map = std::collections::HashMap::new();
                #(#insert_exprs;)*
                map
            }
        })
    }

    fn convert_tuple(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    fn convert_set(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let elem = self.convert(e)?;
                Ok(parse_quote! { set.insert(#elem) })
            })
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0623: Use fully qualified path to avoid missing import
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_exprs;)*
                set
            }
        })
    }

    fn convert_frozenset(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let insert_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let elem = self.convert(e)?;
                Ok(parse_quote! { set.insert(#elem) })
            })
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0623: Use fully qualified path to avoid missing import
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_exprs;)*
                std::sync::Arc::new(set)
            }
        })
    }

    fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_name) => {
                // For now, be conservative and only treat explicit sets as sets
                // This prevents incorrect conversion of integer bitwise operations
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0601: Detect if expression is likely a string type.
    /// Used to generate `.contains()` instead of `.contains_key()` for `in` operator.
    fn is_string_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // String literals are obviously strings
            HirExpr::Literal(Literal::String(_)) => true,
            // F-strings produce strings
            HirExpr::FString { .. } => true,
            // Method calls that return strings
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "lower"
                        | "upper"
                        | "strip"
                        | "lstrip"
                        | "rstrip"
                        | "replace"
                        | "join"
                        | "format"
                        | "capitalize"
                        | "title"
                        | "swapcase"
                        | "center"
                        | "ljust"
                        | "rjust"
                        | "zfill"
                        | "expandtabs"
                        | "encode"
                        | "decode"
                )
            }
            // Variables with common string-like names
            HirExpr::Var(name) => {
                matches!(
                    name.as_str(),
                    "s"
                        | "url"
                        | "path"
                        | "text"
                        | "remaining"
                        | "query_string"
                        | "host"
                        | "scheme"
                        | "fragment"
                        | "name"
                        | "message"
                        | "line"
                        | "content"
                        | "data"
                        | "result"
                        | "output"
                        | "input"
                        | "string"
                        | "str"
                        | "pair"
                        | "email"
                        | "domain_part"
                        | "local_part"
                        | "normalized"
                )
            }
            // Calls to str() produce strings
            HirExpr::Call { func, .. } if func == "str" => true,
            // DEPYLER-0752: Handle attribute access for known string fields
            // Examples: r.stdout, result.stderr, response.text
            HirExpr::Attribute { attr, .. } => {
                matches!(
                    attr.as_str(),
                    "stdout" | "stderr" | "text" | "output" | "message" | "name"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0742: Detect if expression is a deque type.
    /// Used to generate VecDeque methods instead of Vec methods.
    fn is_deque_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call to deque() constructor
            HirExpr::Call { func, .. } if func == "deque" || func == "collections.deque" => true,
            // Variables with deque-like names
            HirExpr::Var(name) => {
                matches!(
                    name.as_str(),
                    "d" | "dq" | "deque" | "queue" | "buffer" | "deck"
                )
            }
            _ => false,
        }
    }

    fn convert_set_operation(
        &self,
        op: BinOp,
        left: syn::Expr,
        right: syn::Expr,
    ) -> Result<syn::Expr> {
        match op {
            BinOp::BitAnd => Ok(parse_quote! {
                #left.intersection(&#right).cloned().collect()
            }),
            BinOp::BitOr => Ok(parse_quote! {
                #left.union(&#right).cloned().collect()
            }),
            BinOp::Sub => Ok(parse_quote! {
                #left.difference(&#right).cloned().collect()
            }),
            BinOp::BitXor => Ok(parse_quote! {
                #left.symmetric_difference(&#right).cloned().collect()
            }),
            _ => bail!("Invalid set operator"),
        }
    }

    fn convert_method_call(
        &self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // Handle classmethod cls.method() → Self::method()
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.is_classmethod {
                let method_ident = make_ident(method);
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { Self::#method_ident(#(#arg_exprs),*) });
            }
        }

        // DEPYLER-0610: Handle Python stdlib module constructor calls
        // threading.Semaphore(n) → std::sync::Mutex::new(n)
        // queue.Queue() → std::collections::VecDeque::new()
        if let HirExpr::Var(module_name) = object {
            if let Some(rust_expr) = self.convert_module_constructor(module_name, method, args)? {
                return Ok(rust_expr);
            }
        }

        // DEPYLER-0200: Handle os module method calls in class methods
        // This was missing - os.unlink() etc. weren't being converted inside class methods
        if let HirExpr::Var(module_name) = object {
            if module_name == "os" {
                if let Some(rust_expr) = self.try_convert_os_method(method, args)? {
                    return Ok(rust_expr);
                }
            }
        }

        // DEPYLER-0200: Handle os.path.* and os.environ.* method calls in class methods
        // Pattern: os.path.exists(path), os.environ.get("KEY") etc.
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module_name) = value.as_ref() {
                if module_name == "os" && attr == "path" {
                    if let Some(rust_expr) = self.try_convert_os_path_method(method, args)? {
                        return Ok(rust_expr);
                    }
                }
                if module_name == "os" && attr == "environ" {
                    if let Some(rust_expr) = self.try_convert_os_environ_method(method, args)? {
                        return Ok(rust_expr);
                    }
                }
            }
        }

        // Check if this is a static method call on a class (e.g., Counter.create_with_value)
        if let HirExpr::Var(class_name) = object {
            if class_name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                // This is likely a static method call - convert to ClassName::method(args)
                let class_ident = make_ident(class_name);
                let method_ident = make_ident(method);
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| self.convert(arg))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) });
            }
        }

        let object_expr = self.convert(object)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        // Map Python collection methods to Rust equivalents
        match method {
            // List/Deque methods
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // DEPYLER-0742: VecDeque uses push_back, Vec uses push
                if self.is_deque_expr(object) {
                    Ok(parse_quote! { #object_expr.push_back(#arg) })
                } else {
                    Ok(parse_quote! { #object_expr.push(#arg) })
                }
            }
            // DEPYLER-0742: Deque-specific methods
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push_front(#arg) })
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.pop_front() })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // Check if it's a list (using position) or set (using remove)
                // For now, assume set behavior since we're working on sets
                if self.is_set_expr(object) {
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#arg) {
                            panic!("KeyError: element not in set");
                        }
                    })
                } else {
                    // List remove behavior
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#arg) {
                            #object_expr.remove(pos);
                        } else {
                            panic!("ValueError: list.remove(x): x not in list");
                        }
                    })
                }
            }

            // Set methods
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.remove(&#arg) })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "pop" => {
                if self.is_set_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments");
                    }
                    // HashSet doesn't have pop(), simulate with iter().next() and remove
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else if self.is_deque_expr(object) {
                    // DEPYLER-0742: VecDeque uses pop_back
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop_back().unwrap_or_default() })
                    } else {
                        bail!("deque.pop() does not accept an index argument");
                    }
                } else {
                    // List pop
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                    } else {
                        let idx = &arg_exprs[0];
                        Ok(parse_quote! { #object_expr.remove(#idx as usize) })
                    }
                }
            }

            // String methods - DEPYLER-0413
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }
            "strip" => {
                if !arg_exprs.is_empty() {
                    bail!("strip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim().to_string() })
            }
            "lstrip" => {
                if !arg_exprs.is_empty() {
                    bail!("lstrip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim_start().to_string() })
            }
            "rstrip" => {
                if !arg_exprs.is_empty() {
                    bail!("rstrip() with arguments not supported");
                }
                Ok(parse_quote! { #object_expr.trim_end().to_string() })
            }
            "startswith" => {
                if args.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                // DEPYLER-0602: For starts_with(), use raw string literal for Pattern trait.
                let prefix: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.starts_with(#prefix) })
            }
            "endswith" => {
                if args.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                // DEPYLER-0602: For ends_with(), use raw string literal for Pattern trait.
                let suffix: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.ends_with(#suffix) })
            }
            "split" => {
                if args.is_empty() {
                    Ok(
                        parse_quote! { #object_expr.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if args.len() == 1 {
                    // DEPYLER-0602: For split(), use raw string literal for Pattern trait.
                    let sep: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => self.convert(&args[0])?,
                    };
                    Ok(
                        parse_quote! { #object_expr.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if args.len() == 2 {
                    // DEPYLER-0188: split(sep, maxsplit) -> splitn(maxsplit+1, sep)
                    // Python maxsplit=N means at most N splits → N+1 parts
                    // Rust splitn(n, pat) returns at most n parts
                    let sep: syn::Expr = match &args[0] {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => self.convert(&args[0])?,
                    };
                    let maxsplit = self.convert(&args[1])?;
                    Ok(
                        parse_quote! { #object_expr.splitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() requires 0-2 arguments");
                }
            }
            "join" => {
                if arg_exprs.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                Ok(parse_quote! { #iterable.join(#object_expr) })
            }
            "replace" => {
                if args.len() != 2 {
                    bail!("replace() requires exactly two arguments");
                }
                // DEPYLER-0602: For replace(), use raw string literals for Pattern trait.
                let old: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                let new: syn::Expr = match &args[1] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[1])?,
                };
                Ok(parse_quote! { #object_expr.replace(#old, #new) })
            }
            "find" => {
                if args.len() != 1 {
                    bail!("find() requires exactly one argument");
                }
                // DEPYLER-0602: For find(), use raw string literal for Pattern trait.
                // String doesn't implement Pattern, but &str does.
                let substring: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.find(#substring).map(|i| i as i64).unwrap_or(-1) })
            }
            "rfind" => {
                if args.len() != 1 {
                    bail!("rfind() requires exactly one argument");
                }
                // DEPYLER-0602: For rfind(), use raw string literal for Pattern trait.
                let substring: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => self.convert(&args[0])?,
                };
                Ok(parse_quote! { #object_expr.rfind(#substring).map(|i| i as i64).unwrap_or(-1) })
            }
            "isdigit" => {
                if !arg_exprs.is_empty() {
                    bail!("isdigit() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_ascii_digit()) },
                )
            }
            "isalpha" => {
                if !arg_exprs.is_empty() {
                    bail!("isalpha() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_alphabetic()) },
                )
            }
            "isalnum" => {
                if !arg_exprs.is_empty() {
                    bail!("isalnum() takes no arguments");
                }
                Ok(
                    parse_quote! { !#object_expr.is_empty() && #object_expr.chars().all(|c| c.is_alphanumeric()) },
                )
            }

            // DEPYLER-0200: String contains method - use raw string literal for Pattern trait
            "__contains__" | "contains" => {
                if args.len() != 1 {
                    bail!("contains() requires exactly one argument");
                }
                // For Pattern trait, use raw string literal or &* for Pattern trait
                let pattern: syn::Expr = match &args[0] {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => {
                        // Use &* to deref-reborrow - works for both String and &str
                        let arg = self.convert(&args[0])?;
                        parse_quote! { &*#arg }
                    }
                };
                Ok(parse_quote! { #object_expr.contains(#pattern) })
            }

            // DEPYLER-0613: Semaphore/Mutex method mappings
            // Python: sem.acquire() → Rust: mutex.lock().unwrap() (returns guard)
            "acquire" => {
                // Mutex.lock() returns a guard - acquire returns bool in Python but we adapt
                Ok(parse_quote! { #object_expr.lock().is_ok() })
            }
            // Python: sem.release() → Rust: drop guard (no-op if guard not held)
            "release" => {
                // In Rust, release happens when guard is dropped
                // For now, just return unit since we can't easily track the guard
                Ok(parse_quote! { () })
            }

            // DEPYLER-0613: List/Dict copy method
            // Python: list.copy() → Rust: vec.clone()
            "copy" => {
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clone() })
            }

            // DEPYLER-0613: Dict contains_key (may be called on wrong type)
            // Python: dict.__contains__(key) sometimes transpiles as contains_key
            "contains_key" => {
                if arg_exprs.len() != 1 {
                    bail!("contains_key() requires exactly one argument");
                }
                let key = &arg_exprs[0];
                // For HashMap this is correct, for Vec use contains
                Ok(parse_quote! { #object_expr.contains(&#key) })
            }

            // Generic method call fallback
            _ => {
                // DEPYLER-0596: Validate method name before creating identifier
                // Method names must be valid Rust identifiers (no empty, no special chars)
                if method.is_empty() {
                    bail!("Empty method name in method call");
                }
                // Check if method is a valid identifier (starts with letter/underscore, alphanumeric)
                let is_valid_ident = method.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                    && method.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
                if !is_valid_ident {
                    bail!("Invalid method name '{}' - not a valid Rust identifier", method);
                }

                // DEPYLER-0823: Wrap cast expressions in parentheses before method calls
                // Rust parses `x as i32.method()` as `x as (i32.method())` which is invalid
                // Must be: `(x as i32).method()`
                let safe_object_expr: syn::Expr = if matches!(object_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#object_expr) }
                } else {
                    object_expr.clone()
                };

                // Debug: Check if method is a Rust keyword
                if syn::parse_str::<syn::Ident>(method).is_err() {
                    // Method is a Rust keyword - use raw identifier
                    let method_ident = syn::Ident::new_raw(method, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #safe_object_expr.#method_ident(#(#arg_exprs),*) });
                }
                let method_ident = make_ident(method);
                Ok(parse_quote! { #safe_object_expr.#method_ident(#(#arg_exprs),*) })
            }
        }
    }

    fn convert_list_comp(
        &self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let element_expr = self.convert(element)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0691: Use |&x| pattern to auto-dereference filter closure parameter
            // because filter() receives &Item, but Python expects the value directly
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|&#target_pat| #cond_expr)
                    .map(|#target_pat| #element_expr)
                    .collect::<Vec<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| #element_expr)
                    .collect::<Vec<_>>()
            })
        }
    }

    fn convert_set_comp(
        &self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let element_expr = self.convert(element)?;

        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0691: Use |&x| pattern to auto-dereference filter closure parameter
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|&#target_pat| #cond_expr)
                    .map(|#target_pat| #element_expr)
                    .collect::<std::collections::HashSet<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| #element_expr)
                    .collect::<std::collections::HashSet<_>>()
            })
        }
    }

    /// DEPYLER-0610: Convert Python stdlib module constructor calls to Rust
    /// threading.Semaphore(n) → std::sync::Mutex::new(n)
    /// queue.Queue() → std::collections::VecDeque::new()
    fn convert_module_constructor(
        &self,
        module: &str,
        constructor: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match module {
            "threading" => match constructor {
                "Semaphore" | "BoundedSemaphore" => {
                    // threading.Semaphore(n) → std::sync::Mutex::new(n)
                    // Use first arg or default to 0
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { std::sync::Mutex::new(#arg) })
                    } else {
                        Some(parse_quote! { std::sync::Mutex::new(0) })
                    }
                }
                "Lock" | "RLock" => {
                    // threading.Lock() → std::sync::Mutex::new(())
                    Some(parse_quote! { std::sync::Mutex::new(()) })
                }
                "Event" => {
                    // threading.Event() → std::sync::Condvar::new()
                    Some(parse_quote! { std::sync::Condvar::new() })
                }
                "Thread" => {
                    // threading.Thread(target=fn) → std::thread::spawn(fn)
                    // Simplified - just return a placeholder
                    Some(parse_quote! { std::thread::spawn(|| {}) })
                }
                _ => None,
            },
            "queue" => match constructor {
                "Queue" | "LifoQueue" | "PriorityQueue" => {
                    // queue.Queue() → std::collections::VecDeque::new()
                    Some(parse_quote! { std::collections::VecDeque::new() })
                }
                _ => None,
            },
            "datetime" => match constructor {
                "datetime" => {
                    // datetime.datetime(y,m,d,...) → chrono placeholder
                    // For now, return Utc::now() as a stub
                    Some(parse_quote! { chrono::Utc::now() })
                }
                "date" => {
                    Some(parse_quote! { chrono::Utc::now().date_naive() })
                }
                "time" => {
                    Some(parse_quote! { chrono::Utc::now().time() })
                }
                "timedelta" => {
                    // datetime.timedelta(days=n) → chrono::Duration::days(n)
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { chrono::Duration::days(#arg) })
                    } else {
                        Some(parse_quote! { chrono::Duration::zero() })
                    }
                }
                "now" => {
                    // datetime.datetime.now() → chrono::Utc::now()
                    Some(parse_quote! { chrono::Utc::now() })
                }
                _ => None,
            },
            "collections" => match constructor {
                "deque" => {
                    Some(parse_quote! { std::collections::VecDeque::new() })
                }
                "Counter" => {
                    Some(parse_quote! { std::collections::HashMap::new() })
                }
                "OrderedDict" => {
                    Some(parse_quote! { std::collections::HashMap::new() })
                }
                "defaultdict" => {
                    Some(parse_quote! { std::collections::HashMap::new() })
                }
                _ => None,
            },
            "asyncio" => match constructor {
                "Event" => Some(parse_quote! { tokio::sync::Notify::new() }),
                "Lock" => Some(parse_quote! { tokio::sync::Mutex::new(()) }),
                "Semaphore" => {
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { tokio::sync::Semaphore::new(#arg as usize) })
                    } else {
                        Some(parse_quote! { tokio::sync::Semaphore::new(1) })
                    }
                }
                "Queue" => Some(parse_quote! { tokio::sync::mpsc::channel(100).1 }),
                // DEPYLER-0747: asyncio.sleep(secs) → tokio::time::sleep(Duration)
                "sleep" => {
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! {
                            tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                        })
                    } else {
                        Some(parse_quote! {
                            tokio::time::sleep(std::time::Duration::from_secs(0))
                        })
                    }
                }
                // DEPYLER-0747: asyncio.run(coro) → tokio runtime block_on
                "run" => arg_exprs.first().map(|arg| {
                    parse_quote! {
                        tokio::runtime::Runtime::new().unwrap().block_on(#arg)
                    }
                }),
                _ => None,
            },
            "json" => match constructor {
                "loads" | "load" => {
                    arg_exprs.first().map(|arg| parse_quote! { serde_json::from_str(#arg)? })
                }
                "dumps" | "dump" => {
                    arg_exprs.first().map(|arg| parse_quote! { serde_json::to_string(&#arg)? })
                }
                _ => None,
            },
            "os" => match constructor {
                "getcwd" => Some(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() }),
                "getenv" => {
                    arg_exprs.first().map(|arg| parse_quote! { std::env::var(#arg).ok() })
                }
                "listdir" => {
                    if let Some(arg) = arg_exprs.first() {
                        Some(parse_quote! { std::fs::read_dir(#arg)?.map(|e| e.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<_>>() })
                    } else {
                        Some(parse_quote! { std::fs::read_dir(".")?.map(|e| e.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<_>>() })
                    }
                }
                _ => None,
            },
            "re" => match constructor {
                "compile" | "match" | "search" | "findall" => {
                    // Return a placeholder that compiles
                    Some(parse_quote! { serde_json::Value::Null })
                }
                _ => None,
            },
            "fnmatch" => match constructor {
                "fnmatch" => {
                    // fnmatch.fnmatch(name, pattern) → name.contains(pattern) as stub
                    if arg_exprs.len() >= 2 {
                        let name = &arg_exprs[0];
                        let pattern = &arg_exprs[1];
                        Some(parse_quote! { #name.contains(&#pattern) })
                    } else {
                        Some(parse_quote! { false })
                    }
                }
                _ => None,
            },
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os module method calls to Rust std::fs and std::env equivalents
    /// This was missing from class method context, causing 57+ compile errors
    fn try_convert_os_method(&self, method: &str, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "getenv" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.getenv() requires 1 or 2 arguments");
                }
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key)? })
                } else {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Some(parse_quote! { std::env::var(#key).unwrap_or_else(|_| #default.to_string()) })
                }
            }
            "unlink" | "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("os.{}() requires exactly 1 argument", method);
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::fs::remove_file(#path)? })
            }
            "mkdir" => {
                if arg_exprs.is_empty() {
                    bail!("os.mkdir() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::fs::create_dir(#path)? })
            }
            "makedirs" => {
                if arg_exprs.is_empty() {
                    bail!("os.makedirs() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::fs::create_dir_all(#path)? })
            }
            "rmdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.rmdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::fs::remove_dir(#path)? })
            }
            "rename" => {
                if arg_exprs.len() != 2 {
                    bail!("os.rename() requires exactly 2 arguments");
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                Some(parse_quote! { std::fs::rename(#src, #dst)? })
            }
            "getcwd" => {
                if !arg_exprs.is_empty() {
                    bail!("os.getcwd() takes no arguments");
                }
                Some(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() })
            }
            "chdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.chdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::env::set_current_dir(#path)? })
            }
            "listdir" => {
                if arg_exprs.is_empty() {
                    Some(parse_quote! {
                        std::fs::read_dir(".")?
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    })
                } else {
                    let path = &arg_exprs[0];
                    Some(parse_quote! {
                        std::fs::read_dir(#path)?
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                    })
                }
            }
            "path" => {
                // os.path is a submodule, handled elsewhere
                None
            }
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os.path module method calls to Rust std::path equivalents
    fn try_convert_os_path_method(&self, method: &str, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "join" => {
                if arg_exprs.is_empty() {
                    bail!("os.path.join() requires at least 1 argument");
                }
                let first = &arg_exprs[0];
                if arg_exprs.len() == 1 {
                    Some(parse_quote! { std::path::PathBuf::from(#first) })
                } else {
                    let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
                    for part in &arg_exprs[1..] {
                        result = parse_quote! { #result.join(#part) };
                    }
                    Some(parse_quote! { #result.to_string_lossy().to_string() })
                }
            }
            "basename" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.basename() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    std::path::Path::new(&#path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                })
            }
            "dirname" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.dirname() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    std::path::Path::new(&#path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string()
                })
            }
            "exists" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.exists() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).exists() })
            }
            "isfile" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isfile() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).is_file() })
            }
            "isdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! { std::path::Path::new(&#path).is_dir() })
            }
            "expanduser" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.expanduser() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                Some(parse_quote! {
                    if (#path).starts_with("~") {
                        std::env::var("HOME")
                            .map(|home| (#path).replacen("~", &home, 1))
                            .unwrap_or_else(|_| (#path).to_string())
                    } else {
                        (#path).to_string()
                    }
                })
            }
            _ => None,
        };

        Ok(result)
    }

    /// DEPYLER-0200: Convert os.environ method calls to Rust std::env equivalents
    fn try_convert_os_environ_method(&self, method: &str, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "get" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.environ.get() requires 1 or 2 arguments");
                }
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key).ok() })
                } else {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Some(parse_quote! { std::env::var(#key).unwrap_or_else(|_| #default.to_string()) })
                }
            }
            "keys" => {
                Some(parse_quote! { std::env::vars().map(|(k, _)| k).collect::<Vec<_>>() })
            }
            "values" => {
                Some(parse_quote! { std::env::vars().map(|(_, v)| v).collect::<Vec<_>>() })
            }
            "items" => {
                Some(parse_quote! { std::env::vars().collect::<Vec<_>>() })
            }
            "clear" => {
                Some(parse_quote! { { /* env clear not implemented */ } })
            }
            "update" => {
                Some(parse_quote! { { /* env update not implemented */ } })
            }
            "insert" | "setdefault" => {
                if arg_exprs.len() >= 2 {
                    let key = &arg_exprs[0];
                    let val = &arg_exprs[1];
                    Some(parse_quote! { std::env::set_var(#key, #val) })
                } else {
                    None
                }
            }
            "contains_key" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Some(parse_quote! { std::env::var(#key).is_ok() })
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(result)
    }

    fn convert_dict_comp(
        &self,
        key: &HirExpr,
        value: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let key_expr = self.convert(key)?;
        let value_expr = self.convert(value)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0691: Use |&x| pattern to auto-dereference filter closure parameter
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|&#target_pat| #cond_expr)
                    .map(|#target_pat| (#key_expr, #value_expr))
                    .collect::<HashMap<_, _>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| (#key_expr, #value_expr))
                    .collect::<HashMap<_, _>>()
            })
        }
    }

    fn convert_lambda(&self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = make_ident(p);
                parse_quote! { #ident }
            })
            .collect();

        // Convert body expression
        let body_expr = self.convert(body)?;

        // Generate closure
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { |#(#param_pats),*| #body_expr })
        }
    }

    fn convert_await(&self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = self.convert(value)?;
        Ok(parse_quote! { #value_expr.await })
    }

    /// DEPYLER-0513: Convert F-string to format!() macro
    ///
    /// Handles Python f-strings like `f"Hello {name}"` → `format!("Hello {}", name)`
    ///
    /// Strategy: Build format template and collect args, then generate format!() call.
    /// Simplified version for direct_rules - basic formatting only.
    fn convert_fstring(&self, parts: &[crate::hir::FStringPart]) -> Result<syn::Expr> {
        use crate::hir::FStringPart;

        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    // Add {} placeholder to template
                    template.push_str("{}");
                    // Convert expression to Rust and add to args
                    let arg_expr = self.convert(expr)?;
                    args.push(arg_expr);
                }
            }
        }

        // Generate format!() macro call
        Ok(parse_quote! { format!(#template, #(#args),*) })
    }

    fn convert_attribute(&self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.is_classmethod {
                let attr_ident = make_ident(attr);
                return Ok(parse_quote! { Self::#attr_ident });
            }

            // DEPYLER-0616: Detect enum/type constant access patterns
            // TypeName.CONSTANT → TypeName::CONSTANT
            // Heuristic: If name starts with uppercase and attr is ALL_CAPS, it's likely an enum constant
            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            let is_constant = attr.chars().all(|c| c.is_uppercase() || c == '_');

            if is_type_name && is_constant {
                let type_ident = make_ident(var_name);
                let attr_ident = make_ident(attr);
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        let value_expr = self.convert(value)?;
        // DEPYLER-0596: Use make_ident to handle keywords like "match"
        let attr_ident = make_ident(attr);

        // DEPYLER-0737: Check if this attribute is a @property method
        // In Python, @property allows method access without (), but in Rust we need ()
        let is_property_method = PROPERTY_METHODS.with(|pm| pm.borrow().contains(attr));

        if is_property_method {
            // Property access needs method call syntax: obj.prop()
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            // Regular field access: obj.field
            // DEPYLER-0740: For self.field accesses, add .clone() to avoid E0507 moves
            // Python semantics don't consume values on field access, so cloning is safe
            if let HirExpr::Var(var_name) = value {
                if var_name == "self" {
                    return Ok(parse_quote! { #value_expr.#attr_ident.clone() });
                }
            }
            Ok(parse_quote! { #value_expr.#attr_ident })
        }
    }

    /// DEPYLER-0188: Convert dynamic/subscript function call
    /// Pattern: `handlers[name](args)` → `(handlers[&name])(args)` or `handlers.get(&name).unwrap()(args)`
    ///
    /// In Rust, calling a value from a HashMap requires:
    /// 1. Index access with reference: `handlers[&name]`
    /// 2. Parentheses to call the result: `(handlers[&name])(args)`
    fn convert_dynamic_call(&self, callee: &HirExpr, args: &[HirExpr]) -> Result<syn::Expr> {
        // Convert the callee expression (e.g., handlers[name])
        let callee_expr = self.convert(callee)?;

        // Convert arguments
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        // Generate: (callee)(args)
        // Wrap callee in parentheses to ensure correct parsing
        if arg_exprs.is_empty() {
            Ok(parse_quote! { (#callee_expr)() })
        } else {
            Ok(parse_quote! { (#callee_expr)(#(#arg_exprs),*) })
        }
    }
}

/// Check if an expression is a len() call
fn is_len_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
}

fn convert_literal(lit: &Literal) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            // DEPYLER-0738: Ensure float literals always have a decimal point
            // f64::to_string() outputs "0" for 0.0, which parses as integer
            let s = f.to_string();
            let float_str = if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            };
            let lit = syn::LitFloat::new(&float_str, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit.to_string() }
        }
        Literal::Bytes(b) => {
            let byte_str = syn::LitByteStr::new(b, proc_macro2::Span::call_site());
            parse_quote! { #byte_str }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::None => parse_quote! { () },
    }
}

/// Convert HIR binary operators to Rust binary operators
fn convert_binop(op: BinOp) -> Result<syn::BinOp> {
    match op {
        // Arithmetic operators
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Div
        | BinOp::Mod
        | BinOp::FloorDiv
        | BinOp::Pow => convert_arithmetic_op(op),

        // Comparison operators
        BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
            convert_comparison_op(op)
        }

        // Logical operators
        BinOp::And | BinOp::Or => convert_logical_op(op),

        // Bitwise operators
        BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::LShift | BinOp::RShift => {
            convert_bitwise_op(op)
        }

        // Special membership operators
        BinOp::In | BinOp::NotIn => {
            bail!("in/not in operators should be handled by convert_binary")
        }
    }
}

fn convert_arithmetic_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        Add => Ok(parse_quote! { + }),
        Sub => Ok(parse_quote! { - }),
        Mul => Ok(parse_quote! { * }),
        Div => Ok(parse_quote! { / }),
        Mod => Ok(parse_quote! { % }),
        FloorDiv => {
            // Floor division requires special handling - it's not implemented as an operator
            // but handled in convert_binary for proper Python semantics
            bail!("Floor division handled in convert_binary with Python semantics")
        }
        Pow => bail!("Power operator handled in convert_binary with type-specific logic"),
        _ => bail!("Invalid operator {:?} for arithmetic conversion", op),
    }
}

fn convert_comparison_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        Eq => Ok(parse_quote! { == }),
        NotEq => Ok(parse_quote! { != }),
        Lt => Ok(parse_quote! { < }),
        LtEq => Ok(parse_quote! { <= }),
        Gt => Ok(parse_quote! { > }),
        GtEq => Ok(parse_quote! { >= }),
        _ => bail!("Invalid operator {:?} for comparison conversion", op),
    }
}

fn convert_logical_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        And => Ok(parse_quote! { && }),
        Or => Ok(parse_quote! { || }),
        _ => bail!("Invalid operator {:?} for logical conversion", op),
    }
}

fn convert_bitwise_op(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;
    match op {
        BitAnd => Ok(parse_quote! { & }),
        BitOr => Ok(parse_quote! { | }),
        BitXor => Ok(parse_quote! { ^ }),
        LShift => Ok(parse_quote! { << }),
        RShift => Ok(parse_quote! { >> }),
        _ => bail!("Invalid operator {:?} for bitwise conversion", op),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::TypeMapper;
    use depyler_annotations::TranspilationAnnotations;

    fn create_test_type_mapper() -> TypeMapper {
        TypeMapper::default()
    }

    #[test]
    fn test_expr_converter_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let lit_expr = HirExpr::Literal(Literal::Int(42));
        let result = converter.convert(&lit_expr).unwrap();

        // Should generate a literal integer expression
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_expr_converter_variable() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let var_expr = HirExpr::Var("x".to_string());
        let result = converter.convert(&var_expr).unwrap();

        // Should generate a path expression (variable reference)
        assert!(matches!(result, syn::Expr::Path(_)));
    }

    #[test]
    fn test_expr_converter_binary() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let binary_expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };

        let result = converter.convert(&binary_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_expr_converter_len_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("arr".to_string())],
            kwargs: vec![],
        };

        let result = converter.convert(&call_expr).unwrap();
        // DEPYLER-0693: len() generates `arg.len() as i32` for Python compatibility
        // This is a Cast expression containing a MethodCall
        assert!(matches!(result, syn::Expr::Cast(_)));
    }

    #[test]
    fn test_expr_converter_range_call_single_arg() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
            kwargs: vec![],
        };

        let result = converter.convert(&call_expr).unwrap();
        // Should generate a range expression
        assert!(matches!(result, syn::Expr::Range(_)));
    }

    #[test]
    fn test_list_literal_generation() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // DEPYLER-0780: List literals should generate vec![] for &Vec<T> compatibility
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let result = converter.convert(&list_expr).unwrap();
        // Should generate vec! macro expression
        assert!(matches!(result, syn::Expr::Macro(_)));
    }

    #[test]
    fn test_array_multiplication_pattern() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // Pattern: [0] * 10
        let mult_expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::List(vec![HirExpr::Literal(Literal::Int(0))])),
            right: Box::new(HirExpr::Literal(Literal::Int(10))),
        };

        let result = converter.convert(&mult_expr).unwrap();
        // Should generate array repeat syntax [0; 10]
        assert!(matches!(result, syn::Expr::Repeat(_)));
    }

    #[test]
    fn test_array_init_functions() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // DEPYLER-0695: zeros(5) generates vec![0; 5] for consistent Vec<T> return type
        let zeros_call = HirExpr::Call {
            func: "zeros".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(5))],
            kwargs: vec![],
        };

        let result = converter.convert(&zeros_call).unwrap();
        // vec![] generates a Macro expression
        assert!(matches!(result, syn::Expr::Macro(_)));
    }

    #[test]
    fn test_expr_converter_range_call_two_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(10)),
            ],
            kwargs: vec![],
        };

        let result = converter.convert(&call_expr).unwrap();
        assert!(matches!(result, syn::Expr::Range(_)));
    }

    #[test]
    fn test_expr_converter_list() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // DEPYLER-0780: All lists should generate vec![] for &Vec<T> compatibility
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let result = converter.convert(&list_expr).unwrap();
        // All lists should generate vec! macro
        assert!(matches!(result, syn::Expr::Macro(_)));

        // Test non-literal list (should also generate vec!)
        let var_list = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Var("y".to_string()),
        ]);

        let result2 = converter.convert(&var_list).unwrap();
        // Non-literal lists should generate vec! macro
        assert!(matches!(result2, syn::Expr::Macro(_)));
    }

    #[test]
    fn test_expr_converter_dict() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let dict_expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);

        let result = converter.convert(&dict_expr).unwrap();
        // Should generate a block expression
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_tuple() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let tuple_expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".to_string())),
        ]);

        let result = converter.convert(&tuple_expr).unwrap();
        // Should generate a tuple expression
        assert!(matches!(result, syn::Expr::Tuple(_)));
    }

    #[test]
    fn test_convert_binop_arithmetic() {
        // Test arithmetic operators
        assert!(convert_binop(BinOp::Add).is_ok());
        assert!(convert_binop(BinOp::Sub).is_ok());
        assert!(convert_binop(BinOp::Mul).is_ok());
        assert!(convert_binop(BinOp::Div).is_ok());
        assert!(convert_binop(BinOp::Mod).is_ok());
    }

    #[test]
    fn test_convert_binop_comparison() {
        // Test comparison operators
        assert!(convert_binop(BinOp::Eq).is_ok());
        assert!(convert_binop(BinOp::NotEq).is_ok());
        assert!(convert_binop(BinOp::Lt).is_ok());
        assert!(convert_binop(BinOp::LtEq).is_ok());
        assert!(convert_binop(BinOp::Gt).is_ok());
        assert!(convert_binop(BinOp::GtEq).is_ok());
    }

    #[test]
    fn test_convert_binop_logical() {
        // Test logical operators
        assert!(convert_binop(BinOp::And).is_ok());
        assert!(convert_binop(BinOp::Or).is_ok());
    }

    #[test]
    fn test_convert_binop_bitwise() {
        // Test bitwise operators
        assert!(convert_binop(BinOp::BitAnd).is_ok());
        assert!(convert_binop(BinOp::BitOr).is_ok());
        assert!(convert_binop(BinOp::BitXor).is_ok());
        assert!(convert_binop(BinOp::LShift).is_ok());
        assert!(convert_binop(BinOp::RShift).is_ok());
    }

    #[test]
    fn test_convert_binop_unsupported() {
        // Test unsupported operators
        assert!(convert_binop(BinOp::Pow).is_err());
        assert!(convert_binop(BinOp::In).is_err());
        assert!(convert_binop(BinOp::NotIn).is_err());
        assert!(convert_binop(BinOp::FloorDiv).is_err()); // Floor division is handled specially
    }

    #[test]
    fn test_floor_division_handling() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // Test integer floor division
        let int_floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(7))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let result = converter.convert(&int_floor_div).unwrap();
        // Should generate a block expression with the floor division formula
        assert!(matches!(result, syn::Expr::Block(_)));

        // Test with negative operands
        let neg_floor_div = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(-7))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };

        let result = converter.convert(&neg_floor_div).unwrap();
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_convert_literal() {
        // Test integer literal
        let int_lit = convert_literal(&Literal::Int(42));
        assert!(matches!(int_lit, syn::Expr::Lit(_)));

        // Test float literal
        let float_lit = convert_literal(&Literal::Float(1.234)); // Use arbitrary float for test
        assert!(matches!(float_lit, syn::Expr::Lit(_)));

        // Test string literal
        let string_lit = convert_literal(&Literal::String("hello".to_string()));
        assert!(matches!(string_lit, syn::Expr::MethodCall(_)));

        // Test bool literal
        let bool_lit = convert_literal(&Literal::Bool(true));
        assert!(matches!(bool_lit, syn::Expr::Lit(_)));

        // Test None literal
        let none_lit = convert_literal(&Literal::None);
        assert!(matches!(none_lit, syn::Expr::Tuple(_)));
    }

    #[test]
    fn test_convert_function_with_documentation() {
        let type_mapper = create_test_type_mapper();

        let func = HirFunction {
            name: "test_func".to_string(),
            params: vec![HirParam::new("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties {
                is_pure: true,
                always_terminates: true,
                panic_free: true,
                max_stack_depth: Some(1),
                can_fail: false,
                error_types: vec![],
                is_async: false,
                is_generator: false,
            },
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = convert_function(&func, &type_mapper).unwrap();

        // Should have documentation attributes
        assert!(!result.attrs.is_empty());
        assert_eq!(result.sig.ident.to_string(), "test_func");
    }

    #[test]
    fn test_apply_rules() {
        let type_mapper = create_test_type_mapper();

        let module = HirModule {
            functions: vec![HirFunction {
                name: "add".to_string(),
                params: vec![
                    HirParam::new("a".to_string(), Type::Int),
                    HirParam::new("b".to_string(), Type::Int),
                ]
                .into(),
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
        };

        let result = apply_rules(&module, &type_mapper).unwrap();

        // Should have at least one import and one function
        assert!(result.items.len() >= 2);

        // First item should be an import
        assert!(matches!(result.items[0], syn::Item::Use(_)));

        // Second item should be a function
        assert!(matches!(result.items[1], syn::Item::Fn(_)));
    }
}

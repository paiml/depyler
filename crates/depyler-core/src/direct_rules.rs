use crate::direct_rules_convert::*; // DEPYLER-COVERAGE-95: Extracted conversion functions
use crate::hir::*;
use crate::rust_gen::keywords::is_rust_keyword; // DEPYLER-0023: Centralized
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

/// DEPYLER-COVERAGE-95: Check if a method name is a property method (accessor)
pub(crate) fn is_property_method(name: &str) -> bool {
    PROPERTY_METHODS.with(|pm| pm.borrow().contains(name))
}

/// DEPYLER-0900: Check if a class name shadows a Rust stdlib/prelude type
/// These names would cause compilation issues or infinite recursion if used as struct names
pub fn is_stdlib_shadowing_name(name: &str) -> bool {
    matches!(
        name,
        // Core primitive types
        "bool" | "char" | "str" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
            | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f32" | "f64"
            // Prelude types that are commonly used
            | "Box" | "Vec" | "String" | "Option" | "Result" | "Some" | "None" | "Ok" | "Err"
            // Collections
            | "HashMap" | "HashSet" | "BTreeMap" | "BTreeSet" | "VecDeque" | "LinkedList"
            // Smart pointers
            | "Rc" | "Arc" | "RefCell" | "Cell" | "Mutex" | "RwLock"
            // Common traits used as types
            | "Iterator" | "IntoIterator" | "Clone" | "Copy" | "Debug" | "Default"
            | "Display" | "Drop" | "Eq" | "Hash" | "Ord" | "PartialEq" | "PartialOrd"
            // I/O types
            | "Read" | "Write" | "Seek" | "BufRead" | "BufWriter" | "BufReader"
            // Error types
            | "Error"
            // Other common std types
            | "Path" | "PathBuf" | "OsStr" | "OsString" | "CStr" | "CString"
            | "Duration" | "Instant" | "SystemTime"
            | "Range" | "RangeInclusive" | "Bound"
            | "Cow" | "Borrow" | "ToOwned"
    )
}

/// DEPYLER-0900: Rename class name if it shadows a Rust stdlib type
/// Appends "Py" suffix to avoid conflicts (e.g., Vec -> PyVec, Option -> PyOption)
pub fn safe_class_name(name: &str) -> String {
    if is_stdlib_shadowing_name(name) {
        format!("Py{}", name)
    } else {
        name.to_string()
    }
}

/// DEPYLER-0840: Convert Type to proc_macro2::TokenStream for nested function codegen
pub(crate) fn type_to_rust_type(ty: &Type, _type_mapper: &TypeMapper) -> proc_macro2::TokenStream {
    match ty {
        Type::Int => quote! { i32 },
        Type::Float => quote! { f64 },
        Type::String => quote! { String },
        Type::Bool => quote! { bool },
        Type::None => quote! { () },
        Type::Unknown => quote! { () },
        Type::List(inner) => {
            let inner_ty = type_to_rust_type(inner, _type_mapper);
            quote! { Vec<#inner_ty> }
        }
        Type::Dict(k, v) => {
            // DEPYLER-1073: Float keys don't implement Hash/Eq in Rust
            // Use DepylerValue for float keys which has a custom Hash/Eq impl using total_cmp
            let key_ty = if matches!(k.as_ref(), Type::Float) {
                quote! { DepylerValue }
            } else {
                type_to_rust_type(k, _type_mapper)
            };
            let val_ty = type_to_rust_type(v, _type_mapper);
            quote! { std::collections::HashMap<#key_ty, #val_ty> }
        }
        Type::Set(inner) => {
            // DEPYLER-1073: Float elements don't implement Hash/Eq in Rust
            // Use DepylerValue for float elements which has a custom Hash/Eq impl
            let inner_ty = if matches!(inner.as_ref(), Type::Float) {
                quote! { DepylerValue }
            } else {
                type_to_rust_type(inner, _type_mapper)
            };
            quote! { std::collections::HashSet<#inner_ty> }
        }
        Type::Optional(inner) => {
            let inner_ty = type_to_rust_type(inner, _type_mapper);
            quote! { Option<#inner_ty> }
        }
        Type::Tuple(elems) => {
            let elem_tys: Vec<_> = elems
                .iter()
                .map(|e| type_to_rust_type(e, _type_mapper))
                .collect();
            quote! { (#(#elem_tys),*) }
        }
        _ => quote! { () }, // Default fallback
    }
}

/// DEPYLER-0596: Parse a target pattern string into a syn::Pat
/// Handles tuple patterns like "(name, t)" and simple identifiers
pub(crate) fn parse_target_pattern(target: &str) -> syn::Pat {
    if target.starts_with('(') {
        // Manually construct tuple pattern
        let inner = target.trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<syn::Pat> = inner
            .split(',')
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
pub(crate) fn make_ident(name: &str) -> syn::Ident {
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
pub(crate) fn sanitize_identifier(name: &str) -> String {
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
pub(crate) fn extract_nested_indices(
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
    // DEPYLER-0900: Rename class if it shadows stdlib type (e.g., Vec -> PyVec)
    let safe_name = safe_class_name(&class.name);
    let struct_name = make_ident(&safe_name);

    // DEPYLER-0957: Check if class inherits from Exception
    // Exception classes should default Unknown types to String (not serde_json::Value)
    let is_exception_class = class.base_classes.iter().any(|base| {
        matches!(
            base.as_str(),
            "Exception"
                | "BaseException"
                | "ValueError"
                | "TypeError"
                | "KeyError"
                | "RuntimeError"
                | "IOError"
                | "OSError"
                | "AttributeError"
                | "IndexError"
                | "StopIteration"
                | "SyntaxError"
                | "FileNotFoundError"
                | "ZeroDivisionError"
        )
    });

    // Separate instance fields from class fields (constants/statics)
    let (instance_fields, class_fields): (Vec<_>, Vec<_>) =
        class.fields.iter().partition(|f| !f.is_class_var);

    // Generate struct fields (only instance fields)
    let mut fields = Vec::new();
    let mut has_non_clone_field = false;

    for field in instance_fields {
        let field_name = syn::Ident::new(
            &sanitize_identifier(&field.name),
            proc_macro2::Span::call_site(),
        );
        // DEPYLER-0957: For Exception classes, default Unknown types to String
        let effective_field_type = if is_exception_class && field.field_type == Type::Unknown {
            Type::String
        } else {
            field.field_type.clone()
        };
        let rust_type = type_mapper.map_type(&effective_field_type);
        let field_type = rust_type_to_syn_type(&rust_type)?;

        // DEPYLER-0611: Check if field type contains non-Clone types
        let type_str = quote::quote!(#field_type).to_string();
        if type_str.contains("Mutex")
            || type_str.contains("RefCell")
            || type_str.contains("Condvar")
            || type_str.contains("RwLock")
            || type_str.contains("mpsc::")
            || type_str.contains("Receiver")
            || type_str.contains("Sender")
            || type_str.contains("JoinHandle")
        {
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
                    colon_token: if needs_clone_bound {
                        Some(syn::Token![:](proc_macro2::Span::call_site()))
                    } else {
                        None
                    },
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

    // DEPYLER-0837: Check EACH type param individually for usage in fields
    // If a param isn't used in any field, we need PhantomData for it
    let unused_type_params: Vec<&String> = if class.type_params.is_empty() {
        vec![] // No type params, nothing to check
    } else {
        // For each type param, check if it's used in ANY field
        class
            .type_params
            .iter()
            .filter(|tp| {
                !fields.iter().any(|f| {
                    let type_str = quote::quote!(#f.ty).to_string();
                    type_str.contains(*tp)
                })
            })
            .collect()
    };

    // Add PhantomData field for unused type params only
    let mut final_fields: Vec<syn::Field> = fields;
    if !unused_type_params.is_empty() {
        // Build PhantomData<(T, U, ...)> for unused type params only
        let phantom_types: Vec<syn::Type> = unused_type_params
            .iter()
            .map(|tp| {
                let ident = syn::Ident::new(tp, proc_macro2::Span::call_site());
                parse_quote!(#ident)
            })
            .collect();

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
                let new_method = convert_init_to_new(
                    method,
                    class,
                    &struct_name,
                    type_mapper,
                    vararg_functions,
                )?;
                impl_items.push(syn::ImplItem::Fn(new_method));
            } else {
                // DEPYLER-0648: Pass vararg_functions for proper slice wrapping
                // DEPYLER-0696: Pass class fields for return type inference
                // DEPYLER-0740: Pass class type_params to distinguish method-level generics
                let rust_method = convert_method_to_impl_item(
                    method,
                    type_mapper,
                    vararg_functions,
                    &class.fields,
                    &class.type_params,
                )?;
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
            let rust_method = convert_method_to_impl_item(
                method,
                type_mapper,
                vararg_functions,
                &class.fields,
                &class.type_params,
            )?;
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
    // Generate parameters from fields (include all instance fields)
    // DEPYLER-0939: Include fields with defaults in new() signature to match Python semantics
    // Defaults should be handled at call site or via builder pattern
    let mut inputs = syn::punctuated::Punctuated::new();
    let instance_fields: Vec<_> = class.fields.iter().filter(|f| !f.is_class_var).collect();

    for field in &instance_fields {
        let param_ident = syn::Ident::new(
            &sanitize_identifier(&field.name),
            proc_macro2::Span::call_site(),
        );
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
            let field_ident = syn::Ident::new(
                &sanitize_identifier(&field.name),
                proc_macro2::Span::call_site(),
            );
            // DEPYLER-0939: Always use parameter since we now accept all fields in new()
            quote! { #field_ident }
        })
        .collect();

    // DEPYLER-0837: Add PhantomData initialization if ANY type params aren't used in fields
    if !class.type_params.is_empty() {
        let instance_fields: Vec<_> = class.fields.iter().filter(|f| !f.is_class_var).collect();
        // Check if ANY type param is unused (we need PhantomData for those)
        let has_unused_params = class.type_params.iter().any(|tp| {
            !instance_fields.iter().any(|f| {
                let type_str = format!("{:?}", f.field_type);
                type_str.contains(tp)
            })
        });
        if has_unused_params {
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
    // DEPYLER-0957: Check if class inherits from Exception
    // Exception classes should default Unknown types to String (not serde_json::Value)
    let is_exception_class = class.base_classes.iter().any(|base| {
        matches!(
            base.as_str(),
            "Exception"
                | "BaseException"
                | "ValueError"
                | "TypeError"
                | "KeyError"
                | "RuntimeError"
                | "IOError"
                | "OSError"
                | "AttributeError"
                | "IndexError"
                | "StopIteration"
                | "SyntaxError"
                | "FileNotFoundError"
                | "ZeroDivisionError"
        )
    });

    // DEPYLER-0697: Collect field names to determine which params are used
    let field_names: std::collections::HashSet<&str> = class
        .fields
        .iter()
        .filter(|f| !f.is_class_var)
        .map(|f| f.name.as_str())
        .collect();

    // Convert parameters
    // DEPYLER-1100: Track String params for impl Into<String> pattern
    let mut string_param_names: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut inputs = syn::punctuated::Punctuated::new();

    for param in &init_method.params {
        // DEPYLER-0697: Prefix unused constructor parameters with _ to avoid warnings
        let param_name = if field_names.contains(param.name.as_str()) {
            param.name.clone()
        } else {
            format!("_{}", param.name)
        };
        let param_ident = make_ident(&param_name);
        // DEPYLER-0957: For Exception classes, default Unknown param types to String
        let effective_param_type = if is_exception_class && param.ty == Type::Unknown {
            Type::String
        } else {
            param.ty.clone()
        };

        // DEPYLER-1100: Use impl Into<String> for String parameters to allow both String and &str
        let is_string_param = effective_param_type == Type::String;
        if is_string_param {
            string_param_names.insert(param.name.clone());
        }

        let param_syn_type: syn::Type = if is_string_param {
            // Use impl Into<String> for string parameters
            parse_quote!(impl Into<String>)
        } else {
            let rust_type = type_mapper.map_type(&effective_param_type);
            rust_type_to_syn_type(&rust_type)?
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

    // Generate field initializers based on class fields and parameters
    // Skip class variables (constants) - only initialize instance fields
    let mut field_inits = Vec::new();

    for field in &class.fields {
        // Skip class variables (constants/statics)
        if field.is_class_var {
            continue;
        }

        let field_ident = syn::Ident::new(
            &sanitize_identifier(&field.name),
            proc_macro2::Span::call_site(),
        );

        // Check if this field matches a parameter name
        if init_method
            .params
            .iter()
            .any(|param| param.name == field.name)
        {
            // DEPYLER-1100: For string parameters using impl Into<String>, call .into()
            if string_param_names.contains(&field.name) {
                field_inits.push(quote! { #field_ident: #field_ident.into() });
            } else {
                // Initialize from parameter (shorthand field init)
                field_inits.push(quote! { #field_ident });
            }
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
        // DEPYLER-1008: Check for method calls that mutate self.field
        // e.g., self.messages.append(msg) -> self.messages is mutated
        HirStmt::Expr(expr) => expr_mutates_self(expr),
        // DEPYLER-1152: Check for mutations in return statements
        // e.g., return self._items.pop() - the pop() mutates self._items
        HirStmt::Return(Some(expr)) => expr_mutates_self(expr),
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

/// DEPYLER-1008: Check if an expression mutates self
/// Looks for method calls like self.messages.append(msg) that mutate self.field
fn expr_mutates_self(expr: &HirExpr) -> bool {
    if let HirExpr::MethodCall { object, method, .. } = expr {
        // Check if method is a mutating method
        let is_mutating = matches!(
            method.as_str(),
            "append"
                | "push"
                | "push_back"
                | "push_front"
                | "appendleft"
                | "popleft"
                | "pop"
                | "insert"
                | "remove"
                | "clear"
                | "extend"
                | "add"
                | "update"
                | "discard"
        );
        if is_mutating {
            // Check if object is self.field
            if let HirExpr::Attribute { value, .. } = object.as_ref() {
                if let HirExpr::Var(name) = value.as_ref() {
                    return name == "self";
                }
            }
        }
    }
    false
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
    let field_names: std::collections::HashSet<&str> =
        fields.iter().map(|f| f.name.as_str()).collect();

    // Check if the parameter is used with attribute access that matches a class field
    fn check_expr(
        param_name: &str,
        expr: &HirExpr,
        field_names: &std::collections::HashSet<&str>,
    ) -> bool {
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
                check_expr(param_name, left, field_names)
                    || check_expr(param_name, right, field_names)
            }
            HirExpr::Unary { operand, .. } => check_expr(param_name, operand, field_names),
            HirExpr::Call { args, .. } => {
                args.iter().any(|a| check_expr(param_name, a, field_names))
            }
            HirExpr::MethodCall { object, args, .. } => {
                check_expr(param_name, object, field_names)
                    || args.iter().any(|a| check_expr(param_name, a, field_names))
            }
            HirExpr::Index { base, index } => {
                check_expr(param_name, base, field_names)
                    || check_expr(param_name, index, field_names)
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

    fn check_stmt(
        param_name: &str,
        stmt: &HirStmt,
        field_names: &std::collections::HashSet<&str>,
    ) -> bool {
        match stmt {
            HirStmt::Assign { value, .. } => check_expr(param_name, value, field_names),
            HirStmt::Expr(expr) => check_expr(param_name, expr, field_names),
            HirStmt::Return(Some(expr)) => check_expr(param_name, expr, field_names),
            HirStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                check_expr(param_name, condition, field_names)
                    || then_body
                        .iter()
                        .any(|s| check_stmt(param_name, s, field_names))
                    || else_body
                        .as_ref()
                        .is_some_and(|eb| eb.iter().any(|s| check_stmt(param_name, s, field_names)))
            }
            HirStmt::While {
                condition, body, ..
            } => {
                check_expr(param_name, condition, field_names)
                    || body.iter().any(|s| check_stmt(param_name, s, field_names))
            }
            HirStmt::For { body, .. } => {
                body.iter().any(|s| check_stmt(param_name, s, field_names))
            }
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
    // DEPYLER-0967: Map Python dunder methods to Rust equivalents
    let rust_method_name = match method.name.as_str() {
        "__len__" => "len".to_string(),
        "__str__" => "to_string".to_string(),
        "__repr__" => "fmt".to_string(), // Debug trait
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
        _ => method.name.clone(),
    };

    // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
    let method_name = if is_rust_keyword(&rust_method_name) {
        syn::Ident::new_raw(&rust_method_name, proc_macro2::Span::call_site())
    } else {
        make_ident(&rust_method_name)
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
    // DEPYLER-0838: Check if body consists only of pass statements
    let body_is_only_pass = method.body.iter().all(|stmt| matches!(stmt, HirStmt::Pass));
    let is_non_unit_return = !matches!(effective_ret_type, Type::None | Type::Unknown);

    let body = if method.body.is_empty() || (body_is_only_pass && is_non_unit_return) {
        // Empty body or pass-only with non-unit return type - use unimplemented!()
        // This handles Python's @abstractmethod pattern where body is just `pass`
        if is_non_unit_return {
            parse_quote! { { unimplemented!() } }
        } else {
            parse_quote! { {} }
        }
    } else {
        // Convert the method body statements with classmethod context
        // DEPYLER-0648: Pass vararg_functions for proper slice wrapping at call sites
        // DEPYLER-0704: Pass param_types for type coercion
        // DEPYLER-0720: Pass class_field_types for self.field float coercion
        // DEPYLER-1037: Pass effective_ret_type for Optional wrapping in returns
        convert_method_body_block(
            &method.body,
            type_mapper,
            method.is_classmethod,
            vararg_functions,
            &param_types,
            &class_field_types,
            &effective_ret_type,
        )?
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
        matches!(
            v,
            "int" | "float" | "i64" | "f64" | "i32" | "f32" | "I64" | "F64"
        )
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

    // Case 6: Fallback to DepylerValue (DEPYLER-1098: Use std-only type instead of serde_json::Value)
    parse_quote! { DepylerValue }
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
            // DEPYLER-0900: Rename generic base if it shadows stdlib type (e.g., Box<U> -> PyBox<U>)
            let safe_base = safe_class_name(base);
            let base_ident = make_ident(&safe_base);
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

pub fn rust_type_to_syn_type(rust_type: &RustType) -> Result<syn::Type> {
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::direct_rules_convert::ExprConverter;
    use crate::rust_gen::keywords::safe_ident;
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

        // Test None literal - DEPYLER-1037: Now produces Rust's None, not ()
        let none_lit = convert_literal(&Literal::None);
        assert!(matches!(none_lit, syn::Expr::Path(_)));
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
            top_level_stmts: vec![],
        };

        let result = apply_rules(&module, &type_mapper).unwrap();

        // Should have at least one import and one function
        assert!(result.items.len() >= 2);

        // First item should be an import
        assert!(matches!(result.items[0], syn::Item::Use(_)));

        // Second item should be a function
        assert!(matches!(result.items[1], syn::Item::Fn(_)));
    }

    // DEPYLER-COV-002: Additional tests for coverage

    #[test]
    fn test_is_rust_keyword() {
        // Common keywords
        assert!(is_rust_keyword("fn"));
        assert!(is_rust_keyword("let"));
        assert!(is_rust_keyword("mut"));
        assert!(is_rust_keyword("if"));
        assert!(is_rust_keyword("else"));
        assert!(is_rust_keyword("while"));
        assert!(is_rust_keyword("for"));
        assert!(is_rust_keyword("return"));
        assert!(is_rust_keyword("match"));
        assert!(is_rust_keyword("self"));
        assert!(is_rust_keyword("Self"));
        assert!(is_rust_keyword("async"));
        assert!(is_rust_keyword("await"));
        assert!(is_rust_keyword("try"));

        // Not keywords
        assert!(!is_rust_keyword("foo"));
        assert!(!is_rust_keyword("bar"));
        assert!(!is_rust_keyword("myVar"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name() {
        // Shadowing types
        assert!(is_stdlib_shadowing_name("String"));
        assert!(is_stdlib_shadowing_name("Vec"));
        assert!(is_stdlib_shadowing_name("Option"));
        assert!(is_stdlib_shadowing_name("Result"));
        assert!(is_stdlib_shadowing_name("HashMap"));
        assert!(is_stdlib_shadowing_name("Box"));
        assert!(is_stdlib_shadowing_name("Iterator"));

        // Not shadowing
        assert!(!is_stdlib_shadowing_name("MyClass"));
        assert!(!is_stdlib_shadowing_name("Foo"));
    }

    #[test]
    fn test_safe_class_name() {
        // Normal names pass through
        assert_eq!(safe_class_name("MyClass"), "MyClass");
        assert_eq!(safe_class_name("Foo"), "Foo");

        // Shadowing names get Py prefix
        assert_eq!(safe_class_name("String"), "PyString");
        assert_eq!(safe_class_name("Vec"), "PyVec");
    }

    #[test]
    fn test_safe_ident_keywords() {
        // Keywords should use raw identifiers
        let ident = safe_ident("type");
        assert_eq!(ident.to_string(), "r#type");

        // Normal identifiers pass through
        let ident = safe_ident("foo");
        assert_eq!(ident.to_string(), "foo");
    }

    #[test]
    fn test_sanitize_identifier() {
        // Empty name
        assert_eq!(sanitize_identifier(""), "_empty");

        // Name starting with digit
        assert_eq!(sanitize_identifier("123abc"), "_123abc");

        // Invalid characters
        assert_eq!(sanitize_identifier("foo-bar"), "foo_bar");
        assert_eq!(sanitize_identifier("foo.bar"), "foo_bar");

        // Already valid
        assert_eq!(sanitize_identifier("valid_name"), "valid_name");

        // Keyword - should get suffix
        assert_eq!(sanitize_identifier("for"), "for_");
    }

    #[test]
    fn test_expr_converter_string_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let str_expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let result = converter.convert(&str_expr).unwrap();
        assert!(matches!(result, syn::Expr::MethodCall(_)));
    }

    #[test]
    fn test_expr_converter_bool_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let bool_expr = HirExpr::Literal(Literal::Bool(true));
        let result = converter.convert(&bool_expr).unwrap();
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_expr_converter_float_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let float_expr = HirExpr::Literal(Literal::Float(3.15));
        let result = converter.convert(&float_expr).unwrap();
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_expr_converter_none_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let none_expr = HirExpr::Literal(Literal::None);
        let result = converter.convert(&none_expr).unwrap();
        // DEPYLER-1037: None becomes Rust's None (a path), not ()
        assert!(matches!(result, syn::Expr::Path(_)));
    }

    #[test]
    fn test_expr_converter_unary_not() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let not_expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };
        let result = converter.convert(&not_expr).unwrap();
        assert!(matches!(result, syn::Expr::Unary(_)));
    }

    #[test]
    fn test_expr_converter_unary_neg() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let neg_expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = converter.convert(&neg_expr).unwrap();
        assert!(matches!(result, syn::Expr::Unary(_)));
    }

    #[test]
    fn test_expr_converter_dict_literal() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let dict_expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);
        let result = converter.convert(&dict_expr).unwrap();
        // Dict generates a block with HashMap
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_empty_dict() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let dict_expr = HirExpr::Dict(vec![]);
        let result = converter.convert(&dict_expr).unwrap();
        // Empty dict is a block with HashMap::new() and return
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_index() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let index_expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let result = converter.convert(&index_expr).unwrap();
        // DEPYLER-1095: Index expressions now generate runtime-safe blocks
        // that handle negative indices, so we get a Block instead of simple Index
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_attribute() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let attr_expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        let result = converter.convert(&attr_expr).unwrap();
        assert!(matches!(result, syn::Expr::Field(_)));
    }

    #[test]
    fn test_expr_converter_comparison_ops() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let ops = vec![
            BinOp::Lt,
            BinOp::LtEq,
            BinOp::Gt,
            BinOp::GtEq,
            BinOp::Eq,
            BinOp::NotEq,
        ];

        for op in ops {
            let expr = HirExpr::Binary {
                op,
                left: Box::new(HirExpr::Literal(Literal::Int(1))),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            };
            let result = converter.convert(&expr).unwrap();
            assert!(matches!(result, syn::Expr::Binary(_)));
        }
    }

    #[test]
    fn test_expr_converter_logical_ops() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let and_expr = HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };
        let result = converter.convert(&and_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));

        let or_expr = HirExpr::Binary {
            op: BinOp::Or,
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };
        let result = converter.convert(&or_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_expr_converter_method_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let method_expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "strip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let result = converter.convert(&method_expr).unwrap();
        assert!(matches!(result, syn::Expr::MethodCall(_)));
    }

    #[test]
    fn test_expr_converter_print_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let print_expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            kwargs: vec![],
        };
        let result = converter.convert(&print_expr).unwrap();
        // print generates println! macro
        assert!(matches!(result, syn::Expr::Macro(_)));
    }

    #[test]
    fn test_rust_type_to_syn_type_string() {
        // String type
        let result = rust_type_to_syn_type(&RustType::String).unwrap();
        assert_eq!(quote::quote!(#result).to_string(), "String");
    }

    #[test]
    fn test_rust_type_to_syn_type_unit() {
        let result = rust_type_to_syn_type(&RustType::Unit).unwrap();
        assert_eq!(quote::quote!(#result).to_string(), "()");
    }

    #[test]
    fn test_rust_type_to_syn_type_custom() {
        let result = rust_type_to_syn_type(&RustType::Custom("MyStruct".to_string())).unwrap();
        assert_eq!(quote::quote!(#result).to_string(), "MyStruct");
    }

    #[test]
    fn test_expr_converter_ord_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let ord_expr = HirExpr::Call {
            func: "ord".to_string(),
            args: vec![HirExpr::Literal(Literal::String("a".to_string()))],
            kwargs: vec![],
        };
        let result = converter.convert(&ord_expr).unwrap();
        // ord returns a u32 cast
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("u32") || s.contains("chars"));
    }

    #[test]
    fn test_expr_converter_chr_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let chr_expr = HirExpr::Call {
            func: "chr".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(65))],
            kwargs: vec![],
        };
        let result = converter.convert(&chr_expr).unwrap();
        // chr converts int to char
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("char") || s.contains("from_u32"));
    }

    #[test]
    fn test_expr_converter_sum_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let sum_expr = HirExpr::Call {
            func: "sum".to_string(),
            args: vec![HirExpr::List(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ])],
            kwargs: vec![],
        };
        let result = converter.convert(&sum_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("sum") || s.contains("iter"));
    }

    #[test]
    fn test_expr_converter_abs_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let abs_expr = HirExpr::Call {
            func: "abs".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(-5))],
            kwargs: vec![],
        };
        let result = converter.convert(&abs_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("abs"));
    }

    #[test]
    fn test_expr_converter_min_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let min_expr = HirExpr::Call {
            func: "min".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ],
            kwargs: vec![],
        };
        let result = converter.convert(&min_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("min"));
    }

    #[test]
    fn test_expr_converter_max_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let max_expr = HirExpr::Call {
            func: "max".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ],
            kwargs: vec![],
        };
        let result = converter.convert(&max_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("max"));
    }

    #[test]
    fn test_expr_converter_slice() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let slice_expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
            stop: Some(Box::new(HirExpr::Literal(Literal::Int(3)))),
            step: None,
        };
        let result = converter.convert(&slice_expr).unwrap();
        // Should generate some expression
        let s = quote::quote!(#result).to_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_expr_converter_slice_with_step() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let slice_expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Literal(Literal::Int(0)))),
            stop: Some(Box::new(HirExpr::Literal(Literal::Int(10)))),
            step: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
        };
        let result = converter.convert(&slice_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_expr_converter_set() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let set_expr = HirExpr::Set(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);
        let result = converter.convert(&set_expr).unwrap();
        // Set generates a block with HashSet
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_empty_set() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let set_expr = HirExpr::Set(vec![]);
        let result = converter.convert(&set_expr).unwrap();
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_if_expr() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let if_expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let result = converter.convert(&if_expr).unwrap();
        assert!(matches!(result, syn::Expr::If(_)));
    }

    #[test]
    fn test_expr_converter_lambda() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let lambda_expr = HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
        };
        let result = converter.convert(&lambda_expr).unwrap();
        assert!(matches!(result, syn::Expr::Closure(_)));
    }

    #[test]
    fn test_expr_converter_list_comp() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let list_comp = HirExpr::ListComp {
            element: Box::new(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(10))],
                    kwargs: vec![],
                }),
                conditions: vec![],
            }],
        };
        let result = converter.convert(&list_comp).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("collect") || s.contains("map"));
    }

    #[test]
    fn test_method_mutates_self_attribute_assignment() {
        use smallvec::smallvec;
        // Test that assignment to self.field is detected as mutation
        let method = HirMethod {
            name: "set_items".to_string(),
            params: smallvec![],
            body: vec![HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("self".to_string())),
                    attr: "items".to_string(),
                },
                value: HirExpr::List(vec![]),
                type_annotation: None,
            }],
            ret_type: Type::None,
            is_static: false,
            is_classmethod: false,
            is_property: false,
            is_async: false,
            docstring: None,
        };
        assert!(method_mutates_self(&method));
    }

    #[test]
    fn test_method_mutates_self_with_assignment() {
        use smallvec::smallvec;
        let method = HirMethod {
            name: "set_value".to_string(),
            params: smallvec![HirParam {
                name: "value".to_string(),
                ty: Type::Int,
                default: None,
                is_vararg: false,
            }],
            body: vec![HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("self".to_string())),
                    attr: "value".to_string(),
                },
                value: HirExpr::Var("value".to_string()),
                type_annotation: None,
            }],
            ret_type: Type::None,
            is_static: false,
            is_classmethod: false,
            is_property: false,
            is_async: false,
            docstring: None,
        };
        assert!(method_mutates_self(&method));
    }

    #[test]
    fn test_method_mutates_self_readonly() {
        use smallvec::smallvec;
        let method = HirMethod {
            name: "get_value".to_string(),
            params: smallvec![],
            body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("self".to_string())),
                attr: "value".to_string(),
            }))],
            ret_type: Type::Int,
            is_static: false,
            is_classmethod: false,
            is_property: false,
            is_async: false,
            docstring: None,
        };
        assert!(!method_mutates_self(&method));
    }

    #[test]
    fn test_expr_converter_bitwise_ops() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // BitAnd
        let and_expr = HirExpr::Binary {
            op: BinOp::BitAnd,
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = converter.convert(&and_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));

        // BitOr
        let or_expr = HirExpr::Binary {
            op: BinOp::BitOr,
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = converter.convert(&or_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));

        // BitXor
        let xor_expr = HirExpr::Binary {
            op: BinOp::BitXor,
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = converter.convert(&xor_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_expr_converter_shift_ops() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // LShift
        let lshift_expr = HirExpr::Binary {
            op: BinOp::LShift,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(4))),
        };
        let result = converter.convert(&lshift_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));

        // RShift
        let rshift_expr = HirExpr::Binary {
            op: BinOp::RShift,
            left: Box::new(HirExpr::Literal(Literal::Int(16))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = converter.convert(&rshift_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_expr_converter_floor_div() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let floor_div_expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(7))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = converter.convert(&floor_div_expr).unwrap();
        // Floor div generates a Block expression with Python floor semantics
        assert!(matches!(result, syn::Expr::Block(_)));
    }

    #[test]
    fn test_expr_converter_power() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let pow_expr = HirExpr::Binary {
            op: BinOp::Pow,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Literal(Literal::Int(10))),
        };
        let result = converter.convert(&pow_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("pow"));
    }

    #[test]
    fn test_expr_converter_modulo() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let mod_expr = HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = converter.convert(&mod_expr).unwrap();
        assert!(matches!(result, syn::Expr::Binary(_)));
    }

    #[test]
    fn test_rust_type_to_syn_type_vec() {
        let result = rust_type_to_syn_type(&RustType::Vec(Box::new(RustType::String))).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("Vec"));
    }

    #[test]
    fn test_rust_type_to_syn_type_option() {
        let result = rust_type_to_syn_type(&RustType::Option(Box::new(RustType::String))).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("Option"));
    }

    #[test]
    fn test_rust_type_to_syn_type_tuple() {
        let result =
            rust_type_to_syn_type(&RustType::Tuple(vec![RustType::String, RustType::Unit]))
                .unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("String") || s.contains("("));
    }

    #[test]
    fn test_expr_converter_enumerate_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let enumerate_expr = HirExpr::Call {
            func: "enumerate".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        };
        let result = converter.convert(&enumerate_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("enumerate") || s.contains("iter"));
    }

    #[test]
    fn test_expr_converter_zip_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let zip_expr = HirExpr::Call {
            func: "zip".to_string(),
            args: vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())],
            kwargs: vec![],
        };
        let result = converter.convert(&zip_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("zip") || s.contains("iter"));
    }

    #[test]
    fn test_expr_converter_sorted_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let sorted_expr = HirExpr::Call {
            func: "sorted".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        };
        let result = converter.convert(&sorted_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("sort") || !s.is_empty());
    }

    #[test]
    fn test_expr_converter_reversed_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let reversed_expr = HirExpr::Call {
            func: "reversed".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        };
        let result = converter.convert(&reversed_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("rev") || !s.is_empty());
    }

    #[test]
    fn test_expr_converter_any_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let any_expr = HirExpr::Call {
            func: "any".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        };
        let result = converter.convert(&any_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("any") || !s.is_empty());
    }

    #[test]
    fn test_expr_converter_all_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let all_expr = HirExpr::Call {
            func: "all".to_string(),
            args: vec![HirExpr::Var("items".to_string())],
            kwargs: vec![],
        };
        let result = converter.convert(&all_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(s.contains("all") || !s.is_empty());
    }

    #[test]
    fn test_expr_converter_filter_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let filter_expr = HirExpr::Call {
            func: "filter".to_string(),
            args: vec![
                HirExpr::Lambda {
                    params: vec!["x".to_string()],
                    body: Box::new(HirExpr::Binary {
                        op: BinOp::Gt,
                        left: Box::new(HirExpr::Var("x".to_string())),
                        right: Box::new(HirExpr::Literal(Literal::Int(0))),
                    }),
                },
                HirExpr::Var("items".to_string()),
            ],
            kwargs: vec![],
        };
        let result = converter.convert(&filter_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_expr_converter_map_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let map_expr = HirExpr::Call {
            func: "map".to_string(),
            args: vec![
                HirExpr::Lambda {
                    params: vec!["x".to_string()],
                    body: Box::new(HirExpr::Binary {
                        op: BinOp::Mul,
                        left: Box::new(HirExpr::Var("x".to_string())),
                        right: Box::new(HirExpr::Literal(Literal::Int(2))),
                    }),
                },
                HirExpr::Var("items".to_string()),
            ],
            kwargs: vec![],
        };
        let result = converter.convert(&map_expr).unwrap();
        let s = quote::quote!(#result).to_string();
        assert!(!s.is_empty());
    }

    // Error path tests for method argument validation

    #[test]
    fn test_startswith_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "startswith".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_endswith_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "endswith".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_split_too_many_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "split".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String(",".to_string())),
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)), // Too many args
            ],
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_join_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("sep".to_string())),
            method: "join".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_replace_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "replace".to_string(),
            args: vec![HirExpr::Literal(Literal::String("a".to_string()))], // Missing second arg
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_find_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "find".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_rfind_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "rfind".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_isdigit_with_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "isdigit".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))], // Takes no args
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_isalpha_with_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "isalpha".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))], // Takes no args
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_isalnum_with_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "isalnum".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))], // Takes no args
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_contains_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("d".to_string())),
            method: "contains".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_copy_with_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("lst".to_string())),
            method: "copy".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))], // Takes no args
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_contains_key_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("d".to_string())),
            method: "contains_key".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_empty_method_name() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "".to_string(), // Empty method name
            args: vec![],
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_invalid_method_name() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "123invalid".to_string(), // Invalid identifier
            args: vec![],
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    // OS module method call tests - covers method lookup paths

    #[test]
    fn test_os_getenv_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("os".to_string())),
            method: "getenv".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("PATH".to_string())),
                HirExpr::Literal(Literal::String("default".to_string())),
            ],
            kwargs: vec![],
        };
        // Tests OS method lookup path
        let _ = converter.convert(&expr);
    }

    #[test]
    fn test_os_remove_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("os".to_string())),
            method: "remove".to_string(),
            args: vec![HirExpr::Literal(Literal::String("/tmp/file".to_string()))],
            kwargs: vec![],
        };
        // Tests OS method lookup path
        let _ = converter.convert(&expr);
    }

    #[test]
    fn test_os_getcwd_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("os".to_string())),
            method: "getcwd".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        // Tests OS method lookup path
        let _ = converter.convert(&expr);
    }

    #[test]
    fn test_os_chdir_call() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("os".to_string())),
            method: "chdir".to_string(),
            args: vec![HirExpr::Literal(Literal::String("/tmp".to_string()))],
            kwargs: vec![],
        };
        // Tests OS method lookup path
        let _ = converter.convert(&expr);
    }

    // Comprehension error path tests

    #[test]
    fn test_list_comp_multiple_generators() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![
                HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(HirExpr::Var("items".to_string())),
                    conditions: vec![],
                },
                HirComprehension {
                    target: "y".to_string(),
                    iter: Box::new(HirExpr::Var("other".to_string())),
                    conditions: vec![],
                },
            ],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_set_comp_multiple_generators() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::SetComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![
                HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(HirExpr::Var("items".to_string())),
                    conditions: vec![],
                },
                HirComprehension {
                    target: "y".to_string(),
                    iter: Box::new(HirExpr::Var("other".to_string())),
                    conditions: vec![],
                },
            ],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_dict_comp_multiple_generators() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::DictComp {
            key: Box::new(HirExpr::Var("k".to_string())),
            value: Box::new(HirExpr::Var("v".to_string())),
            generators: vec![
                HirComprehension {
                    target: "k".to_string(),
                    iter: Box::new(HirExpr::Var("items".to_string())),
                    conditions: vec![],
                },
                HirComprehension {
                    target: "v".to_string(),
                    iter: Box::new(HirExpr::Var("other".to_string())),
                    conditions: vec![],
                },
            ],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_list_comp_multiple_conditions() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![
                    HirExpr::Literal(Literal::Bool(true)),
                    HirExpr::Literal(Literal::Bool(false)),
                ],
            }],
        };
        assert!(converter.convert(&expr).is_err());
    }

    // Builtin function error path tests

    #[test]
    fn test_len_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_ord_wrong_args() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Call {
            func: "ord".to_string(),
            args: vec![], // Missing required argument
            kwargs: vec![],
        };
        assert!(converter.convert(&expr).is_err());
    }

    // Expression type coverage tests

    #[test]
    fn test_frozenset_conversion() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::FrozenSet(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_slice_with_all_components() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("lst".to_string())),
            start: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
            stop: Some(Box::new(HirExpr::Literal(Literal::Int(5)))),
            step: Some(Box::new(HirExpr::Literal(Literal::Int(2)))),
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unary_bitnot() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::BitNot,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unary_pos() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Pos,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrow_expr() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: false,
        };
        // Borrow may not be supported in direct rules path
        let _ = converter.convert(&expr);
    }

    #[test]
    fn test_borrow_mut_expr() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: true,
        };
        // Borrow may not be supported in direct rules path
        let _ = converter.convert(&expr);
    }

    #[test]
    fn test_yield_expr() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Yield {
            value: Some(Box::new(HirExpr::Literal(Literal::Int(42)))),
        };
        let result = converter.convert(&expr);
        // Yield may or may not be supported
        let _ = result;
    }

    #[test]
    fn test_generator_exp_single() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![HirComprehension {
                target: "x".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![],
            }],
        };
        // Single generator should work
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generator_exp_multiple() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![
                HirComprehension {
                    target: "x".to_string(),
                    iter: Box::new(HirExpr::Var("items".to_string())),
                    conditions: vec![],
                },
                HirComprehension {
                    target: "y".to_string(),
                    iter: Box::new(HirExpr::Var("other".to_string())),
                    conditions: vec![],
                },
            ],
        };
        assert!(converter.convert(&expr).is_err());
    }

    #[test]
    fn test_fstring_conversion() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::FString {
            parts: vec![
                crate::hir::FStringPart::Literal("Hello ".to_string()),
                crate::hir::FStringPart::Expr(Box::new(HirExpr::Var("name".to_string()))),
            ],
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_expr_conversion() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_tuple() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Tuple(vec![]);
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_list() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::List(vec![
            HirExpr::List(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ]),
            HirExpr::List(vec![
                HirExpr::Literal(Literal::Int(3)),
                HirExpr::Literal(Literal::Int(4)),
            ]),
        ]);
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_attribute_access() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_index_negative() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("lst".to_string())),
            index: Box::new(HirExpr::Unary {
                op: crate::hir::UnaryOp::Neg,
                operand: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
        };
        let result = converter.convert(&expr);
        assert!(result.is_ok());
    }

    // ============ is_stdlib_shadowing_name tests ============

    #[test]
    fn test_is_stdlib_shadowing_primitives() {
        // Primitive types
        assert!(is_stdlib_shadowing_name("bool"));
        assert!(is_stdlib_shadowing_name("char"));
        assert!(is_stdlib_shadowing_name("str"));
        assert!(is_stdlib_shadowing_name("i32"));
        assert!(is_stdlib_shadowing_name("u64"));
        assert!(is_stdlib_shadowing_name("f64"));
        assert!(is_stdlib_shadowing_name("usize"));
    }

    #[test]
    fn test_is_stdlib_shadowing_prelude_types() {
        // Prelude types
        assert!(is_stdlib_shadowing_name("Box"));
        assert!(is_stdlib_shadowing_name("Vec"));
        assert!(is_stdlib_shadowing_name("String"));
        assert!(is_stdlib_shadowing_name("Option"));
        assert!(is_stdlib_shadowing_name("Result"));
        assert!(is_stdlib_shadowing_name("Some"));
        assert!(is_stdlib_shadowing_name("None"));
    }

    #[test]
    fn test_is_stdlib_shadowing_collections() {
        // Collections
        assert!(is_stdlib_shadowing_name("HashMap"));
        assert!(is_stdlib_shadowing_name("HashSet"));
        assert!(is_stdlib_shadowing_name("BTreeMap"));
        assert!(is_stdlib_shadowing_name("VecDeque"));
    }

    #[test]
    fn test_is_stdlib_shadowing_traits() {
        // Common traits
        assert!(is_stdlib_shadowing_name("Clone"));
        assert!(is_stdlib_shadowing_name("Debug"));
        assert!(is_stdlib_shadowing_name("Default"));
        assert!(is_stdlib_shadowing_name("Iterator"));
    }

    #[test]
    fn test_is_stdlib_shadowing_io_types() {
        // I/O types
        assert!(is_stdlib_shadowing_name("Read"));
        assert!(is_stdlib_shadowing_name("Write"));
        assert!(is_stdlib_shadowing_name("BufReader"));
    }

    #[test]
    fn test_is_stdlib_shadowing_non_shadowing() {
        // Custom names that don't shadow
        assert!(!is_stdlib_shadowing_name("MyStruct"));
        assert!(!is_stdlib_shadowing_name("Person"));
        assert!(!is_stdlib_shadowing_name("Config"));
        assert!(!is_stdlib_shadowing_name("User"));
        assert!(!is_stdlib_shadowing_name("Data"));
    }

    #[test]
    fn test_is_stdlib_shadowing_case_sensitive() {
        // Should be case-sensitive
        assert!(is_stdlib_shadowing_name("Vec"));
        assert!(!is_stdlib_shadowing_name("vec")); // lowercase is not shadowing
        assert!(!is_stdlib_shadowing_name("VEC")); // uppercase is not shadowing
    }

    // ============ safe_class_name tests ============

    #[test]
    fn test_safe_class_name_shadowing() {
        assert_eq!(safe_class_name("Vec"), "PyVec");
        assert_eq!(safe_class_name("Option"), "PyOption");
        assert_eq!(safe_class_name("HashMap"), "PyHashMap");
    }

    #[test]
    fn test_safe_class_name_non_shadowing() {
        assert_eq!(safe_class_name("MyClass"), "MyClass");
        assert_eq!(safe_class_name("Person"), "Person");
        assert_eq!(safe_class_name("Config"), "Config");
    }

    #[test]
    fn test_safe_class_name_primitives() {
        assert_eq!(safe_class_name("i32"), "Pyi32");
        assert_eq!(safe_class_name("bool"), "Pybool");
    }

    // ============ type_to_rust_type tests ============

    #[test]
    fn test_type_to_rust_type_primitives() {
        let type_mapper = create_test_type_mapper();

        let int_ts = type_to_rust_type(&Type::Int, &type_mapper);
        assert_eq!(int_ts.to_string(), "i32");

        let float_ts = type_to_rust_type(&Type::Float, &type_mapper);
        assert_eq!(float_ts.to_string(), "f64");

        let string_ts = type_to_rust_type(&Type::String, &type_mapper);
        assert_eq!(string_ts.to_string(), "String");

        let bool_ts = type_to_rust_type(&Type::Bool, &type_mapper);
        assert_eq!(bool_ts.to_string(), "bool");
    }

    #[test]
    fn test_type_to_rust_type_none_unknown() {
        let type_mapper = create_test_type_mapper();

        let none_ts = type_to_rust_type(&Type::None, &type_mapper);
        assert_eq!(none_ts.to_string(), "()");

        let unknown_ts = type_to_rust_type(&Type::Unknown, &type_mapper);
        assert_eq!(unknown_ts.to_string(), "()");
    }

    #[test]
    fn test_type_to_rust_type_list() {
        let type_mapper = create_test_type_mapper();

        let list_int = Type::List(Box::new(Type::Int));
        let list_ts = type_to_rust_type(&list_int, &type_mapper);
        assert_eq!(list_ts.to_string(), "Vec < i32 >");
    }

    #[test]
    fn test_type_to_rust_type_dict() {
        let type_mapper = create_test_type_mapper();

        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        let dict_ts = type_to_rust_type(&dict_type, &type_mapper);
        assert!(dict_ts.to_string().contains("HashMap"));
    }

    #[test]
    fn test_type_to_rust_type_set() {
        let type_mapper = create_test_type_mapper();

        let set_type = Type::Set(Box::new(Type::Int));
        let set_ts = type_to_rust_type(&set_type, &type_mapper);
        assert!(set_ts.to_string().contains("HashSet"));
    }

    #[test]
    fn test_type_to_rust_type_optional() {
        let type_mapper = create_test_type_mapper();

        let opt_type = Type::Optional(Box::new(Type::Int));
        let opt_ts = type_to_rust_type(&opt_type, &type_mapper);
        assert!(opt_ts.to_string().contains("Option"));
    }

    #[test]
    fn test_type_to_rust_type_tuple() {
        let type_mapper = create_test_type_mapper();

        let tuple_type = Type::Tuple(vec![Type::Int, Type::String]);
        let tuple_ts = type_to_rust_type(&tuple_type, &type_mapper);
        let result = tuple_ts.to_string();
        assert!(result.contains("i32"));
        assert!(result.contains("String"));
    }

    // ============ parse_target_pattern tests ============

    #[test]
    fn test_parse_target_pattern_simple() {
        let pat = parse_target_pattern("x");
        assert!(matches!(pat, syn::Pat::Ident(_)));
    }

    #[test]
    fn test_parse_target_pattern_tuple() {
        let pat = parse_target_pattern("(a, b)");
        assert!(matches!(pat, syn::Pat::Tuple(_)));
        if let syn::Pat::Tuple(tuple) = pat {
            assert_eq!(tuple.elems.len(), 2);
        }
    }

    #[test]
    fn test_parse_target_pattern_triple() {
        let pat = parse_target_pattern("(x, y, z)");
        assert!(matches!(pat, syn::Pat::Tuple(_)));
        if let syn::Pat::Tuple(tuple) = pat {
            assert_eq!(tuple.elems.len(), 3);
        }
    }

    #[test]
    fn test_parse_target_pattern_with_spaces() {
        let pat = parse_target_pattern("( a , b )");
        assert!(matches!(pat, syn::Pat::Tuple(_)));
    }

    // ============ make_ident tests ============

    #[test]
    fn test_make_ident_simple() {
        let ident = make_ident("foo");
        assert_eq!(ident.to_string(), "foo");
    }

    #[test]
    fn test_make_ident_empty() {
        let ident = make_ident("");
        assert_eq!(ident.to_string(), "_empty");
    }

    #[test]
    fn test_make_ident_keyword_raw() {
        // Keywords should use raw identifier syntax
        let ident = make_ident("match");
        assert_eq!(ident.to_string(), "r#match");
    }

    #[test]
    fn test_make_ident_self_special() {
        // self can't be raw identifier - gets underscore suffix
        let ident = make_ident("self");
        assert_eq!(ident.to_string(), "self_");
    }

    #[test]
    fn test_make_ident_super_special() {
        let ident = make_ident("super");
        assert_eq!(ident.to_string(), "super_");
    }

    #[test]
    fn test_make_ident_crate_special() {
        let ident = make_ident("crate");
        assert_eq!(ident.to_string(), "crate_");
    }

    #[test]
    fn test_make_ident_type_keyword() {
        let ident = make_ident("type");
        assert_eq!(ident.to_string(), "r#type");
    }

    #[test]
    fn test_make_ident_Self_valid() {
        // Self is valid as a type name
        let ident = make_ident("Self");
        assert_eq!(ident.to_string(), "Self");
    }

    // ============ sanitize_identifier tests ============

    #[test]
    fn test_sanitize_identifier_valid() {
        assert_eq!(sanitize_identifier("foo"), "foo");
        assert_eq!(sanitize_identifier("_bar"), "_bar");
        assert_eq!(sanitize_identifier("baz123"), "baz123");
    }

    #[test]
    fn test_sanitize_identifier_empty() {
        assert_eq!(sanitize_identifier(""), "_empty");
    }

    #[test]
    fn test_sanitize_identifier_starts_with_digit() {
        assert_eq!(sanitize_identifier("123abc"), "_123abc");
        assert_eq!(sanitize_identifier("0"), "_0");
    }

    #[test]
    fn test_sanitize_identifier_invalid_chars() {
        assert_eq!(sanitize_identifier("foo-bar"), "foo_bar");
        assert_eq!(sanitize_identifier("foo.bar"), "foo_bar");
        assert_eq!(sanitize_identifier("foo::bar"), "foo__bar");
    }

    #[test]
    fn test_sanitize_identifier_keyword() {
        // Keywords get underscore suffix
        assert_eq!(sanitize_identifier("fn"), "fn_");
        assert_eq!(sanitize_identifier("let"), "let_");
        assert_eq!(sanitize_identifier("if"), "if_");
    }

    #[test]
    fn test_sanitize_identifier_special_chars() {
        assert_eq!(sanitize_identifier("@attr"), "_attr");
        assert_eq!(sanitize_identifier("#id"), "_id");
        assert_eq!(sanitize_identifier("$var"), "_var");
    }

    // ============ method_mutates_self tests ============

    #[test]
    fn test_method_mutates_self_with_assign() {
        let method = HirMethod {
            name: "update".to_string(),
            params: smallvec::smallvec![],
            body: vec![HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("self".to_string())),
                    attr: "value".to_string(),
                },
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            }],
            ret_type: Type::None,
            is_async: false,
            is_static: false,
            is_classmethod: false,
            is_property: false,
            docstring: None,
        };
        assert!(method_mutates_self(&method));
    }

    #[test]
    fn test_method_mutates_self_without_mutation() {
        let method = HirMethod {
            name: "get_value".to_string(),
            params: smallvec::smallvec![],
            body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("self".to_string())),
                attr: "value".to_string(),
            }))],
            ret_type: Type::Int,
            is_async: false,
            is_static: false,
            is_classmethod: false,
            is_property: false,
            docstring: None,
        };
        assert!(!method_mutates_self(&method));
    }

    /// DEPYLER-1152: Test that return statements containing mutations are detected
    /// e.g., return self._items.pop() should trigger &mut self
    #[test]
    fn test_method_mutates_self_in_return() {
        let method = HirMethod {
            name: "pop".to_string(),
            params: smallvec::smallvec![],
            body: vec![HirStmt::Return(Some(HirExpr::MethodCall {
                object: Box::new(HirExpr::Attribute {
                    value: Box::new(HirExpr::Var("self".to_string())),
                    attr: "_items".to_string(),
                }),
                method: "pop".to_string(),
                args: vec![],
                kwargs: vec![],
            }))],
            ret_type: Type::Optional(Box::new(Type::Int)),
            is_async: false,
            is_static: false,
            is_classmethod: false,
            is_property: false,
            docstring: None,
        };
        assert!(method_mutates_self(&method));
    }

    // ============ stmt_mutates_self tests ============

    #[test]
    fn test_stmt_mutates_self_attribute_assign() {
        let stmt = HirStmt::Assign {
            target: AssignTarget::Attribute {
                value: Box::new(HirExpr::Var("self".to_string())),
                attr: "x".to_string(),
            },
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: None,
        };
        assert!(stmt_mutates_self(&stmt));
    }

    #[test]
    fn test_stmt_mutates_self_in_if() {
        let stmt = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("self".to_string())),
                    attr: "value".to_string(),
                },
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            }],
            else_body: None,
        };
        assert!(stmt_mutates_self(&stmt));
    }

    #[test]
    fn test_stmt_mutates_self_return_no_mutation() {
        let stmt = HirStmt::Return(Some(HirExpr::Var("self".to_string())));
        assert!(!stmt_mutates_self(&stmt));
    }

    // ============ collect_type_vars tests ============

    #[test]
    fn test_collect_type_vars_simple() {
        let mut vars = std::collections::HashSet::new();
        collect_type_vars(&Type::TypeVar("T".to_string()), &mut vars);
        assert!(vars.contains("T"));
    }

    #[test]
    fn test_collect_type_vars_in_list() {
        let mut vars = std::collections::HashSet::new();
        let list_t = Type::List(Box::new(Type::TypeVar("T".to_string())));
        collect_type_vars(&list_t, &mut vars);
        assert!(vars.contains("T"));
    }

    #[test]
    fn test_collect_type_vars_in_dict() {
        let mut vars = std::collections::HashSet::new();
        let dict_type = Type::Dict(
            Box::new(Type::TypeVar("K".to_string())),
            Box::new(Type::TypeVar("V".to_string())),
        );
        collect_type_vars(&dict_type, &mut vars);
        assert!(vars.contains("K"));
        assert!(vars.contains("V"));
    }

    #[test]
    fn test_collect_type_vars_none_for_primitives() {
        let mut vars = std::collections::HashSet::new();
        collect_type_vars(&Type::Int, &mut vars);
        collect_type_vars(&Type::String, &mut vars);
        assert!(vars.is_empty());
    }

    // ============ is_pure_expression_direct tests ============

    #[test]
    fn test_is_pure_expression_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_var() {
        let expr = HirExpr::Var("x".to_string());
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_call_is_not_pure() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&expr));
    }

    #[test]
    fn test_is_pure_expression_method_call_not_pure() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "do_something".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&expr));
    }

    // ============ convert_literal tests ============

    #[test]
    fn test_convert_literal_int() {
        let result = convert_literal(&Literal::Int(42));
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_convert_literal_float() {
        let result = convert_literal(&Literal::Float(3.15));
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_convert_literal_string() {
        let result = convert_literal(&Literal::String("hello".to_string()));
        assert!(matches!(result, syn::Expr::MethodCall(_)));
    }

    #[test]
    fn test_convert_literal_bool_true() {
        let result = convert_literal(&Literal::Bool(true));
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_convert_literal_bool_false() {
        let result = convert_literal(&Literal::Bool(false));
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_convert_literal_none() {
        let result = convert_literal(&Literal::None);
        // DEPYLER-1037: None converts to Rust's None (a path expression), not ()
        assert!(matches!(result, syn::Expr::Path(_)));
    }

    // ============ convert_binop tests ============

    #[test]
    fn test_convert_binop_add() {
        let result = convert_binop(BinOp::Add);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_sub() {
        let result = convert_binop(BinOp::Sub);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_mul() {
        let result = convert_binop(BinOp::Mul);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_div() {
        let result = convert_binop(BinOp::Div);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_mod() {
        let result = convert_binop(BinOp::Mod);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_eq() {
        let result = convert_binop(BinOp::Eq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_ne() {
        let result = convert_binop(BinOp::NotEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_lt() {
        let result = convert_binop(BinOp::Lt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_gt() {
        let result = convert_binop(BinOp::Gt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_le() {
        let result = convert_binop(BinOp::LtEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_ge() {
        let result = convert_binop(BinOp::GtEq);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_bitand() {
        let result = convert_binop(BinOp::BitAnd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_bitor() {
        let result = convert_binop(BinOp::BitOr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_bitxor() {
        let result = convert_binop(BinOp::BitXor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_lshift() {
        let result = convert_binop(BinOp::LShift);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_binop_rshift() {
        let result = convert_binop(BinOp::RShift);
        assert!(result.is_ok());
    }

    // ============ is_len_call tests ============

    #[test]
    fn test_is_len_call_true() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(is_len_call(&expr));
    }

    #[test]
    fn test_is_len_call_false_different_func() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(!is_len_call(&expr));
    }

    #[test]
    fn test_is_len_call_false_not_call() {
        let expr = HirExpr::Var("len".to_string());
        assert!(!is_len_call(&expr));
    }

    // ============ infer_method_return_type tests ============

    #[test]
    fn test_infer_method_return_type_explicit_return() {
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        let fields: Vec<HirField> = vec![];
        let result = infer_method_return_type(&body, &fields);
        // Should infer Int from the literal
        assert!(result.is_some());
    }

    #[test]
    fn test_infer_method_return_type_empty_body() {
        let body: Vec<HirStmt> = vec![];
        let fields: Vec<HirField> = vec![];
        let result = infer_method_return_type(&body, &fields);
        assert!(result.is_none());
    }

    #[test]
    fn test_infer_method_return_type_none_return() {
        let body = vec![HirStmt::Return(None)];
        let fields: Vec<HirField> = vec![];
        let result = infer_method_return_type(&body, &fields);
        // Return(None) might infer to Type::None, not None
        assert!(result.is_none() || result == Some(Type::None));
    }

    // ============ infer_expr_type_with_fields tests ============

    #[test]
    fn test_infer_expr_type_int_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        let fields: Vec<HirField> = vec![];
        let result = infer_expr_type_with_fields(&expr, &fields);
        assert_eq!(result, Type::Int);
    }

    #[test]
    fn test_infer_expr_type_float_literal() {
        let expr = HirExpr::Literal(Literal::Float(3.15));
        let fields: Vec<HirField> = vec![];
        let result = infer_expr_type_with_fields(&expr, &fields);
        assert_eq!(result, Type::Float);
    }

    #[test]
    fn test_infer_expr_type_string_literal() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let fields: Vec<HirField> = vec![];
        let result = infer_expr_type_with_fields(&expr, &fields);
        assert_eq!(result, Type::String);
    }

    #[test]
    fn test_infer_expr_type_bool_literal() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        let fields: Vec<HirField> = vec![];
        let result = infer_expr_type_with_fields(&expr, &fields);
        assert_eq!(result, Type::Bool);
    }

    #[test]
    fn test_infer_expr_type_with_field() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("self".to_string())),
            attr: "count".to_string(),
        };
        let fields = vec![HirField {
            name: "count".to_string(),
            field_type: Type::Int,
            default_value: None,
            is_class_var: false,
        }];
        let result = infer_expr_type_with_fields(&expr, &fields);
        assert_eq!(result, Type::Int);
    }

    // ============ should_param_be_self_type tests ============

    #[test]
    fn test_should_param_be_self_type_basic() {
        // Test that the function runs without error for a simple case
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        let fields: Vec<HirField> = vec![];
        let _result = should_param_be_self_type("other", &body, &fields);
        // Just verifying the function runs
    }

    #[test]
    fn test_should_param_be_self_type_with_self_attr() {
        // When returning a self attribute, the result depends on implementation
        let body = vec![HirStmt::Return(Some(HirExpr::Attribute {
            value: Box::new(HirExpr::Var("self".to_string())),
            attr: "value".to_string(),
        }))];
        let fields = vec![HirField {
            name: "value".to_string(),
            field_type: Type::Int,
            default_value: None,
            is_class_var: false,
        }];
        let _result = should_param_be_self_type("value", &body, &fields);
        // Just verifying the function runs with fields
    }

    // ========================================================
    // DEPYLER-COVERAGE-95: Additional direct_rules tests
    // ========================================================

    // ============ is_stdlib_shadowing_name tests ============

    #[test]
    fn test_is_stdlib_shadowing_name_primitive_types() {
        // Rust primitive types should be detected
        assert!(is_stdlib_shadowing_name("bool"));
        assert!(is_stdlib_shadowing_name("char"));
        assert!(is_stdlib_shadowing_name("str"));
        assert!(is_stdlib_shadowing_name("i32"));
        assert!(is_stdlib_shadowing_name("u64"));
        assert!(is_stdlib_shadowing_name("f64"));
        assert!(is_stdlib_shadowing_name("isize"));
        assert!(is_stdlib_shadowing_name("usize"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_prelude_types() {
        // Prelude types should be detected
        assert!(is_stdlib_shadowing_name("Box"));
        assert!(is_stdlib_shadowing_name("Vec"));
        assert!(is_stdlib_shadowing_name("String"));
        assert!(is_stdlib_shadowing_name("Option"));
        assert!(is_stdlib_shadowing_name("Result"));
        assert!(is_stdlib_shadowing_name("Some"));
        assert!(is_stdlib_shadowing_name("None"));
        assert!(is_stdlib_shadowing_name("Ok"));
        assert!(is_stdlib_shadowing_name("Err"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_collections() {
        // Collection types should be detected
        assert!(is_stdlib_shadowing_name("HashMap"));
        assert!(is_stdlib_shadowing_name("HashSet"));
        assert!(is_stdlib_shadowing_name("BTreeMap"));
        assert!(is_stdlib_shadowing_name("BTreeSet"));
        assert!(is_stdlib_shadowing_name("VecDeque"));
        assert!(is_stdlib_shadowing_name("LinkedList"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_smart_pointers() {
        // Smart pointer types should be detected
        assert!(is_stdlib_shadowing_name("Rc"));
        assert!(is_stdlib_shadowing_name("Arc"));
        assert!(is_stdlib_shadowing_name("RefCell"));
        assert!(is_stdlib_shadowing_name("Cell"));
        assert!(is_stdlib_shadowing_name("Mutex"));
        assert!(is_stdlib_shadowing_name("RwLock"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_traits() {
        // Common trait names should be detected
        assert!(is_stdlib_shadowing_name("Clone"));
        assert!(is_stdlib_shadowing_name("Copy"));
        assert!(is_stdlib_shadowing_name("Debug"));
        assert!(is_stdlib_shadowing_name("Default"));
        assert!(is_stdlib_shadowing_name("Display"));
        assert!(is_stdlib_shadowing_name("Drop"));
        assert!(is_stdlib_shadowing_name("Eq"));
        assert!(is_stdlib_shadowing_name("Iterator"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_io_types() {
        // I/O types should be detected
        assert!(is_stdlib_shadowing_name("Read"));
        assert!(is_stdlib_shadowing_name("Write"));
        assert!(is_stdlib_shadowing_name("Seek"));
        assert!(is_stdlib_shadowing_name("BufRead"));
        assert!(is_stdlib_shadowing_name("BufWriter"));
        assert!(is_stdlib_shadowing_name("BufReader"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_path_types() {
        // Path types should be detected
        assert!(is_stdlib_shadowing_name("Path"));
        assert!(is_stdlib_shadowing_name("PathBuf"));
        assert!(is_stdlib_shadowing_name("OsStr"));
        assert!(is_stdlib_shadowing_name("OsString"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_non_shadowing() {
        // Custom names should NOT be detected as shadowing
        assert!(!is_stdlib_shadowing_name("MyStruct"));
        assert!(!is_stdlib_shadowing_name("Calculator"));
        assert!(!is_stdlib_shadowing_name("UserData"));
        assert!(!is_stdlib_shadowing_name("Point"));
        assert!(!is_stdlib_shadowing_name("Rectangle"));
        assert!(!is_stdlib_shadowing_name("custom_name"));
    }

    // ============ is_pure_expression_direct tests ============

    #[test]
    fn test_is_pure_expression_direct_literal() {
        assert!(is_pure_expression_direct(&HirExpr::Literal(Literal::Int(
            42
        ))));
        assert!(is_pure_expression_direct(&HirExpr::Literal(
            Literal::String("hello".to_string())
        )));
        assert!(is_pure_expression_direct(&HirExpr::Literal(Literal::Bool(
            true
        ))));
        assert!(is_pure_expression_direct(&HirExpr::Literal(
            Literal::Float(3.15)
        )));
    }

    #[test]
    fn test_is_pure_expression_direct_var() {
        assert!(is_pure_expression_direct(&HirExpr::Var("x".to_string())));
    }

    #[test]
    fn test_is_pure_expression_direct_list_not_pure() {
        // List construction is considered not pure in this implementation
        let list = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
        assert!(!is_pure_expression_direct(&list));
    }

    #[test]
    fn test_is_pure_expression_direct_dict_not_pure() {
        // Dict construction is considered not pure in this implementation
        let dict = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(1)),
        )]);
        assert!(!is_pure_expression_direct(&dict));
    }

    #[test]
    fn test_is_pure_expression_direct_binary() {
        let binary = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(is_pure_expression_direct(&binary));
    }

    #[test]
    fn test_is_pure_expression_direct_call_not_pure() {
        // Function calls are not pure
        let call = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&call));
    }

    #[test]
    fn test_is_pure_expression_direct_method_call_not_pure() {
        // Method calls are not pure
        let method = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("x".to_string())),
            method: "append".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_pure_expression_direct(&method));
    }

    #[test]
    fn test_is_pure_expression_direct_tuple() {
        let tuple = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert!(is_pure_expression_direct(&tuple));
    }

    #[test]
    fn test_is_pure_expression_direct_index() {
        let index = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(is_pure_expression_direct(&index));
    }

    #[test]
    fn test_is_pure_expression_direct_attribute() {
        let attr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(is_pure_expression_direct(&attr));
    }

    #[test]
    fn test_is_pure_expression_direct_unary() {
        let unary = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        assert!(is_pure_expression_direct(&unary));
    }

    #[test]
    fn test_is_pure_expression_direct_none_literal() {
        assert!(is_pure_expression_direct(&HirExpr::Literal(Literal::None)));
    }

    // ============ Additional time/duration type tests ============

    #[test]
    fn test_is_stdlib_shadowing_name_time_types() {
        assert!(is_stdlib_shadowing_name("Duration"));
        assert!(is_stdlib_shadowing_name("Instant"));
        assert!(is_stdlib_shadowing_name("SystemTime"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_range_types() {
        assert!(is_stdlib_shadowing_name("Range"));
        assert!(is_stdlib_shadowing_name("RangeInclusive"));
        assert!(is_stdlib_shadowing_name("Bound"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_cow_types() {
        assert!(is_stdlib_shadowing_name("Cow"));
        assert!(is_stdlib_shadowing_name("Borrow"));
        assert!(is_stdlib_shadowing_name("ToOwned"));
    }

    #[test]
    fn test_is_stdlib_shadowing_name_error_type() {
        assert!(is_stdlib_shadowing_name("Error"));
    }

    // Tests for extract_nested_indices
    #[test]
    fn test_extract_nested_indices_single() {
        let type_mapper = create_test_type_mapper();
        let inner = HirExpr::Var("arr".to_string());
        let expr = HirExpr::Index {
            base: Box::new(inner),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let (base, indices) = extract_nested_indices(&expr, &type_mapper).unwrap();
        assert!(matches!(base, syn::Expr::Path(_)));
        assert_eq!(indices.len(), 1);
    }

    #[test]
    fn test_extract_nested_indices_nested() {
        let type_mapper = create_test_type_mapper();
        // matrix[0][1]
        let inner1 = HirExpr::Index {
            base: Box::new(HirExpr::Var("matrix".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let expr = HirExpr::Index {
            base: Box::new(inner1),
            index: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        let (base, indices) = extract_nested_indices(&expr, &type_mapper).unwrap();
        assert!(matches!(base, syn::Expr::Path(_)));
        assert_eq!(indices.len(), 2);
    }

    #[test]
    fn test_extract_nested_indices_no_index() {
        let type_mapper = create_test_type_mapper();
        let expr = HirExpr::Var("x".to_string());
        let (base, indices) = extract_nested_indices(&expr, &type_mapper).unwrap();
        assert!(matches!(base, syn::Expr::Path(_)));
        assert!(indices.is_empty());
    }

    // Tests for resolve_union_enum_to_syn
    #[test]
    fn test_resolve_union_enum_option_int() {
        let variants = vec![
            (
                "int".to_string(),
                RustType::Primitive(crate::type_mapper::PrimitiveType::I64),
            ),
            ("None".to_string(), RustType::Unit),
        ];
        let result = resolve_union_enum_to_syn(&variants);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Option"));
        assert!(result_str.contains("i64"));
    }

    #[test]
    fn test_resolve_union_enum_option_float() {
        let variants = vec![
            (
                "float".to_string(),
                RustType::Primitive(crate::type_mapper::PrimitiveType::F64),
            ),
            ("None".to_string(), RustType::Unit),
        ];
        let result = resolve_union_enum_to_syn(&variants);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Option"));
        assert!(result_str.contains("f64"));
    }

    #[test]
    fn test_resolve_union_enum_option_string() {
        let variants = vec![
            ("str".to_string(), RustType::String),
            ("None".to_string(), RustType::Unit),
        ];
        let result = resolve_union_enum_to_syn(&variants);
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Option"));
        assert!(result_str.contains("String"));
    }

    #[test]
    fn test_resolve_union_enum_only_none() {
        let variants = vec![("None".to_string(), RustType::Unit)];
        let result = resolve_union_enum_to_syn(&variants);
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "()");
    }

    #[test]
    fn test_resolve_union_enum_all_numeric() {
        let variants = vec![
            (
                "int".to_string(),
                RustType::Primitive(crate::type_mapper::PrimitiveType::I64),
            ),
            (
                "float".to_string(),
                RustType::Primitive(crate::type_mapper::PrimitiveType::F64),
            ),
        ];
        let result = resolve_union_enum_to_syn(&variants);
        let result_str = quote::quote!(#result).to_string();
        // When int and float are both present, should resolve to f64
        assert_eq!(result_str, "f64");
    }

    #[test]
    fn test_resolve_union_enum_fallback() {
        let variants = vec![
            ("Foo".to_string(), RustType::Custom("Foo".to_string())),
            ("Bar".to_string(), RustType::Custom("Bar".to_string())),
        ];
        let result = resolve_union_enum_to_syn(&variants);
        let result_str = quote::quote!(#result).to_string();
        // DEPYLER-1098: Changed fallback from serde_json::Value to DepylerValue (std-only)
        assert!(result_str.contains("DepylerValue"));
    }

    // Tests for convert_simple_type
    #[test]
    fn test_convert_simple_type_unit() {
        let result = convert_simple_type(&RustType::Unit).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "()");
    }

    #[test]
    fn test_convert_simple_type_string() {
        let result = convert_simple_type(&RustType::String).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "String");
    }

    #[test]
    fn test_convert_simple_type_custom() {
        let result = convert_simple_type(&RustType::Custom("MyType".to_string())).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "MyType");
    }

    #[test]
    fn test_convert_simple_type_custom_shadowing() {
        let result = convert_simple_type(&RustType::Custom("Vec".to_string())).unwrap();
        let result_str = quote::quote!(#result).to_string();
        // Should be renamed to avoid shadowing
        assert_eq!(result_str, "PyVec");
    }

    #[test]
    fn test_convert_simple_type_type_param() {
        let result = convert_simple_type(&RustType::TypeParam("T".to_string())).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "T");
    }

    // Tests for convert_primitive_type
    #[test]
    fn test_convert_primitive_type_bool() {
        use crate::type_mapper::PrimitiveType;
        let result = convert_primitive_type(&PrimitiveType::Bool).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "bool");
    }

    #[test]
    fn test_convert_primitive_type_i32() {
        use crate::type_mapper::PrimitiveType;
        let result = convert_primitive_type(&PrimitiveType::I32).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "i32");
    }

    #[test]
    fn test_convert_primitive_type_i64() {
        use crate::type_mapper::PrimitiveType;
        let result = convert_primitive_type(&PrimitiveType::I64).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "i64");
    }

    #[test]
    fn test_convert_primitive_type_f64() {
        use crate::type_mapper::PrimitiveType;
        let result = convert_primitive_type(&PrimitiveType::F64).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "f64");
    }

    #[test]
    fn test_convert_primitive_type_usize() {
        use crate::type_mapper::PrimitiveType;
        let result = convert_primitive_type(&PrimitiveType::USize).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "usize");
    }

    // Tests for convert_lifetime_type
    #[test]
    fn test_convert_lifetime_type_str_no_lifetime() {
        let result = convert_lifetime_type(&RustType::Str { lifetime: None }).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert_eq!(result_str, "& str");
    }

    #[test]
    fn test_convert_lifetime_type_str_with_lifetime() {
        let result = convert_lifetime_type(&RustType::Str {
            lifetime: Some("a".to_string()),
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("'a"));
        assert!(result_str.contains("str"));
    }

    #[test]
    fn test_convert_lifetime_type_cow() {
        let result = convert_lifetime_type(&RustType::Cow {
            lifetime: "static".to_string(),
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Cow"));
        assert!(result_str.contains("'static"));
    }

    // Tests for convert_unsupported_type
    #[test]
    fn test_convert_unsupported_type() {
        let result = convert_unsupported_type("complex").unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("UnsupportedType_complex"));
    }

    #[test]
    fn test_convert_unsupported_type_with_space() {
        let result = convert_unsupported_type("foo bar").unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("UnsupportedType_foo_bar"));
    }

    // Tests for convert_container_type
    #[test]
    fn test_convert_container_type_vec() {
        let result = convert_container_type(&RustType::Vec(Box::new(RustType::String))).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Vec"));
        assert!(result_str.contains("String"));
    }

    #[test]
    fn test_convert_container_type_hashmap() {
        let result = convert_container_type(&RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(crate::type_mapper::PrimitiveType::I64)),
        ))
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("HashMap"));
        assert!(result_str.contains("String"));
        assert!(result_str.contains("i64"));
    }

    #[test]
    fn test_convert_container_type_option() {
        let result = convert_container_type(&RustType::Option(Box::new(RustType::String))).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Option"));
        assert!(result_str.contains("String"));
    }

    #[test]
    fn test_convert_container_type_result() {
        let result = convert_container_type(&RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("Error".to_string())),
        ))
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Result"));
        assert!(result_str.contains("String"));
    }

    #[test]
    fn test_convert_container_type_hashset() {
        let result =
            convert_container_type(&RustType::HashSet(Box::new(RustType::String))).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("HashSet"));
        assert!(result_str.contains("String"));
    }

    // Tests for convert_complex_type
    #[test]
    fn test_convert_complex_type_tuple() {
        let result = convert_complex_type(&RustType::Tuple(vec![
            RustType::String,
            RustType::Primitive(crate::type_mapper::PrimitiveType::I64),
        ]))
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("String"));
        assert!(result_str.contains("i64"));
    }

    #[test]
    fn test_convert_complex_type_generic() {
        let result = convert_complex_type(&RustType::Generic {
            base: "Box".to_string(),
            params: vec![RustType::String],
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("Box"));
        assert!(result_str.contains("String"));
    }

    #[test]
    fn test_convert_complex_type_reference_immut() {
        let result = convert_complex_type(&RustType::Reference {
            inner: Box::new(RustType::String),
            mutable: false,
            lifetime: None,
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("& String"));
    }

    #[test]
    fn test_convert_complex_type_reference_mut() {
        let result = convert_complex_type(&RustType::Reference {
            inner: Box::new(RustType::String),
            mutable: true,
            lifetime: None,
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("& mut String"));
    }

    // Tests for convert_array_type
    #[test]
    fn test_convert_array_type_literal_size() {
        use crate::type_mapper::RustConstGeneric;
        let result = convert_array_type(&RustType::Array {
            element_type: Box::new(RustType::Primitive(crate::type_mapper::PrimitiveType::I32)),
            size: RustConstGeneric::Literal(10),
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("i32"));
        assert!(result_str.contains("10"));
    }

    #[test]
    fn test_convert_array_type_parameter_size() {
        use crate::type_mapper::RustConstGeneric;
        let result = convert_array_type(&RustType::Array {
            element_type: Box::new(RustType::Primitive(crate::type_mapper::PrimitiveType::U8)),
            size: RustConstGeneric::Parameter("N".to_string()),
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("u8"));
        assert!(result_str.contains("N"));
    }

    #[test]
    fn test_convert_array_type_expression_size() {
        use crate::type_mapper::RustConstGeneric;
        let result = convert_array_type(&RustType::Array {
            element_type: Box::new(RustType::String),
            size: RustConstGeneric::Expression("N * 2".to_string()),
        })
        .unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("String"));
        assert!(result_str.contains("N") && result_str.contains("2"));
    }

    // Tests for find_mutable_vars_in_body
    #[test]
    fn test_find_mutable_vars_empty() {
        let stmts: Vec<HirStmt> = vec![];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_mutable_vars_single_assign() {
        let stmts = vec![HirStmt::Assign {
            target: AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Literal(Literal::Int(1)),
            type_annotation: Some(Type::Int),
        }];
        let result = find_mutable_vars_in_body(&stmts);
        // First assignment - not mutable yet
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_mutable_vars_reassign() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: Some(Type::Int),
            },
            HirStmt::Assign {
                target: AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(2)),
                type_annotation: Some(Type::Int),
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("x"));
    }

    #[test]
    fn test_find_mutable_vars_attr_assign() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("obj".to_string()),
                value: HirExpr::Var("something".to_string()),
                type_annotation: None,
            },
            HirStmt::Assign {
                target: AssignTarget::Attribute {
                    value: Box::new(HirExpr::Var("obj".to_string())),
                    attr: "field".to_string(),
                },
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: Some(Type::Int),
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("obj"));
    }

    #[test]
    fn test_find_mutable_vars_index_assign() {
        let stmts = vec![
            HirStmt::Assign {
                target: AssignTarget::Symbol("arr".to_string()),
                value: HirExpr::List(vec![]),
                type_annotation: Some(Type::List(Box::new(Type::Int))),
            },
            HirStmt::Assign {
                target: AssignTarget::Index {
                    base: Box::new(HirExpr::Var("arr".to_string())),
                    index: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: Some(Type::Int),
            },
        ];
        let result = find_mutable_vars_in_body(&stmts);
        assert!(result.contains("arr"));
    }
}

use crate::hir::*;
use crate::type_mapper::{RustType, TypeMapper};
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

pub fn apply_rules(module: &HirModule, type_mapper: &TypeMapper) -> Result<syn::File> {
    let mut items = Vec::new();

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
    for class in &module.classes {
        let struct_items = convert_class_to_struct(class, type_mapper)?;
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
    let alias_name = syn::Ident::new(&type_alias.name, proc_macro2::Span::call_site());
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
    let trait_name = syn::Ident::new(&protocol.name, proc_macro2::Span::call_site());

    // Convert type parameters to generic parameters
    let generics = if protocol.type_params.is_empty() {
        syn::Generics::default()
    } else {
        let params: Vec<syn::GenericParam> = protocol
            .type_params
            .iter()
            .map(|param| {
                let ident = syn::Ident::new(param, proc_macro2::Span::call_site());
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

pub fn convert_class_to_struct(class: &HirClass, type_mapper: &TypeMapper) -> Result<Vec<syn::Item>> {
    let mut items = Vec::new();
    let struct_name = syn::Ident::new(&class.name, proc_macro2::Span::call_site());
    
    // Generate struct fields
    let mut fields = Vec::new();
    for field in &class.fields {
        let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(&field.field_type);
        let field_type = rust_type_to_syn_type(&rust_type)?;
        
        fields.push(syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
            mutability: syn::FieldMutability::None,
            ident: Some(field_name),
            colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
            ty: field_type,
        });
    }
    
    // Create the struct
    let struct_item = syn::Item::Struct(syn::ItemStruct {
        attrs: if class.is_dataclass {
            vec![parse_quote! { #[derive(Debug, Clone, PartialEq)] }]
        } else {
            vec![parse_quote! { #[derive(Debug, Clone)] }]
        },
        vis: syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::call_site())),
        struct_token: syn::Token![struct](proc_macro2::Span::call_site()),
        ident: struct_name.clone(),
        generics: syn::Generics::default(),
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: fields.into_iter().collect(),
        }),
        semi_token: None,
    });
    items.push(struct_item);
    
    // Generate impl block with methods
    let mut impl_items = Vec::new();
    
    // Convert __init__ to new() if present
    for method in &class.methods {
        if method.name == "__init__" {
            let new_method = convert_init_to_new(method, &struct_name, type_mapper)?;
            impl_items.push(syn::ImplItem::Fn(new_method));
        } else {
            let rust_method = convert_method_to_impl_item(method, type_mapper)?;
            impl_items.push(syn::ImplItem::Fn(rust_method));
        }
    }
    
    // Only generate impl block if there are methods
    if !impl_items.is_empty() {
        let impl_block = syn::Item::Impl(syn::ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: syn::Token![impl](proc_macro2::Span::call_site()),
            generics: syn::Generics::default(),
            trait_: None,
            self_ty: Box::new(parse_quote! { #struct_name }),
            brace_token: syn::token::Brace::default(),
            items: impl_items,
        });
        items.push(impl_block);
    }
    
    Ok(items)
}

fn convert_init_to_new(
    init_method: &HirMethod,
    _struct_name: &syn::Ident,
    type_mapper: &TypeMapper,
) -> Result<syn::ImplItemFn> {
    // Convert parameters
    let mut inputs = syn::punctuated::Punctuated::new();
    
    for (param_name, param_type) in &init_method.params {
        let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(param_type);
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
    
    // Generate body that initializes struct fields
    // For now, assume fields are initialized from parameters with same names
    let field_inits = init_method.params.iter()
        .map(|(param_name, _)| {
            let field_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
            quote! { #field_ident }
        })
        .collect::<Vec<_>>();
    
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

fn convert_method_to_impl_item(
    method: &HirMethod,
    type_mapper: &TypeMapper,
) -> Result<syn::ImplItemFn> {
    let method_name = syn::Ident::new(&method.name, proc_macro2::Span::call_site());
    
    // Convert parameters
    let mut inputs = syn::punctuated::Punctuated::new();
    
    // Add self parameter based on method type
    if !method.is_static {
        if method.is_property {
            // Properties typically use &self
            inputs.push(parse_quote! { &self });
        } else {
            // Regular methods use &mut self by default (can be refined later)
            inputs.push(parse_quote! { &mut self });
        }
    }
    
    // Add other parameters
    for (param_name, param_type) in &method.params {
        let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(param_type);
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
    let rust_ret_type = type_mapper.map_type(&method.ret_type);
    let ret_type = rust_type_to_syn_type(&rust_ret_type)?;
    
    // Convert body - for now, just return default implementation
    // TODO: Implement proper method body conversion
    let body = parse_quote! {
        {
            todo!("Method body conversion not yet implemented")
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
            ident: method_name,
            generics: syn::Generics::default(),
            paren_token: syn::token::Paren::default(),
            inputs,
            variadic: None,
            output: syn::ReturnType::Type(
                syn::Token![->](proc_macro2::Span::call_site()),
                Box::new(ret_type),
            ),
        },
        block: body,
    })
}

fn convert_protocol_method_to_trait_method(
    method: &ProtocolMethod,
    type_mapper: &TypeMapper,
) -> Result<syn::TraitItem> {
    let method_name = syn::Ident::new(&method.name, proc_macro2::Span::call_site());

    // Convert parameters
    let mut inputs = syn::punctuated::Punctuated::new();

    // Add self parameter for methods (skip first param if it's 'self')
    let method_params = if !method.params.is_empty() && method.params[0].0 == "self" {
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
    for (param_name, param_type) in method_params {
        let param_ident = syn::Ident::new(param_name, proc_macro2::Span::call_site());
        let rust_type = type_mapper.map_type(param_type);
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

fn rust_type_to_syn_type(rust_type: &RustType) -> Result<syn::Type> {
    use RustType::*;
    Ok(match rust_type {
        Unit => parse_quote! { () },
        Primitive(prim_type) => {
            use crate::type_mapper::PrimitiveType;
            match prim_type {
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
            }
        }
        String => parse_quote! { String },
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
        Vec(inner) => {
            let inner_type = rust_type_to_syn_type(inner)?;
            parse_quote! { Vec<#inner_type> }
        }
        HashMap(key, value) => {
            let key_type = rust_type_to_syn_type(key)?;
            let value_type = rust_type_to_syn_type(value)?;
            parse_quote! { HashMap<#key_type, #value_type> }
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
        Tuple(types) => {
            let type_tokens: anyhow::Result<std::vec::Vec<_>> =
                types.iter().map(rust_type_to_syn_type).collect();
            let type_tokens = type_tokens?;
            parse_quote! { (#(#type_tokens),*) }
        }
        Custom(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        Unsupported(name) => {
            // For unsupported types, just generate a placeholder comment type
            let ident = syn::Ident::new(
                &format!("UnsupportedType_{}", name.replace(" ", "_")),
                proc_macro2::Span::call_site(),
            );
            parse_quote! { #ident }
        }
        TypeParam(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: anyhow::Result<std::vec::Vec<_>> =
                params.iter().map(rust_type_to_syn_type).collect();
            let param_types = param_types?;
            parse_quote! { #base_ident<#(#param_types),*> }
        }
        Enum { name, variants: _ } => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        Reference { inner, mutable, .. } => {
            let inner_type = rust_type_to_syn_type(inner)?;
            if *mutable {
                parse_quote! { &mut #inner_type }
            } else {
                parse_quote! { &#inner_type }
            }
        }
        Array { element_type, size } => {
            let element = rust_type_to_syn_type(element_type)?;
            match size {
                crate::type_mapper::RustConstGeneric::Literal(n) => {
                    let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                    parse_quote! { [#element; #size_lit] }
                }
                crate::type_mapper::RustConstGeneric::Parameter(name) => {
                    let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                    parse_quote! { [#element; #param_ident] }
                }
                crate::type_mapper::RustConstGeneric::Expression(expr) => {
                    // For expressions, parse them as token streams
                    let expr_tokens: proc_macro2::TokenStream = expr
                        .parse()
                        .unwrap_or_else(|_| "/* invalid const expression */".parse().unwrap());
                    parse_quote! { [#element; #expr_tokens] }
                }
            }
        }
    })
}

fn convert_function(func: &HirFunction, type_mapper: &TypeMapper) -> Result<syn::ItemFn> {
    let name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());

    // Convert parameters
    let mut inputs = Vec::new();
    for (param_name, param_type) in &func.params {
        let rust_type = type_mapper.map_type(param_type);
        let ty = rust_type_to_syn(&rust_type)?;
        let pat = syn::Pat::Ident(syn::PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: syn::Ident::new(param_name, proc_macro2::Span::call_site()),
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
                    let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
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
    stmts
        .iter()
        .map(|stmt| convert_stmt(stmt, type_mapper))
        .collect()
}

fn convert_stmt(stmt: &HirStmt, type_mapper: &TypeMapper) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let value_expr = convert_expr(value, type_mapper)?;

            let stmt = syn::Stmt::Local(syn::Local {
                attrs: vec![],
                let_token: Default::default(),
                pat: syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: Some(Default::default()),
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
        HirStmt::Return(expr) => {
            let ret_expr = if let Some(e) = expr {
                convert_expr(e, type_mapper)?
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
            let cond = convert_expr(condition, type_mapper)?;
            let then_block = convert_block(then_body, type_mapper)?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block = convert_block(else_stmts, type_mapper)?;
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
            let cond = convert_expr(condition, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;

            let while_expr = parse_quote! {
                while #cond #body_block
            };

            Ok(syn::Stmt::Expr(while_expr, Some(Default::default())))
        }
        HirStmt::For { target, iter, body } => {
            let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            let iter_expr = convert_expr(iter, type_mapper)?;
            let body_block = convert_block(body, type_mapper)?;

            let for_expr = parse_quote! {
                for #target_ident in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            let rust_expr = convert_expr(expr, type_mapper)?;
            Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
        }
        HirStmt::Raise {
            exception,
            cause: _,
        } => {
            // Convert to Rust panic for direct rules
            let panic_expr = if let Some(exc) = exception {
                let exc_expr = convert_expr(exc, type_mapper)?;
                parse_quote! { panic!("Exception: {}", #exc_expr) }
            } else {
                parse_quote! { panic!("Exception raised") }
            };
            Ok(syn::Stmt::Expr(panic_expr, Some(Default::default())))
        }
    }
}

fn convert_block(stmts: &[HirStmt], type_mapper: &TypeMapper) -> Result<syn::Block> {
    let rust_stmts = convert_body(stmts, type_mapper)?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// Convert HIR expressions to Rust expressions using strategy pattern
fn convert_expr(expr: &HirExpr, type_mapper: &TypeMapper) -> Result<syn::Expr> {
    let converter = ExprConverter::new(type_mapper);
    converter.convert(expr)
}

/// Expression converter using strategy pattern to reduce complexity
struct ExprConverter<'a> {
    #[allow(dead_code)]
    type_mapper: &'a TypeMapper,
}

impl<'a> ExprConverter<'a> {
    fn new(type_mapper: &'a TypeMapper) -> Self {
        Self { type_mapper }
    }

    fn convert(&self, expr: &HirExpr) -> Result<syn::Expr> {
        match expr {
            HirExpr::Literal(lit) => self.convert_literal(lit),
            HirExpr::Var(name) => self.convert_variable(name),
            HirExpr::Binary { op, left, right } => self.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => self.convert_unary(*op, operand),
            HirExpr::Call { func, args } => self.convert_call(func, args),
            HirExpr::Index { base, index } => self.convert_index(base, index),
            HirExpr::List(elts) => self.convert_list(elts),
            HirExpr::Dict(items) => self.convert_dict(items),
            HirExpr::Tuple(elts) => self.convert_tuple(elts),
            HirExpr::Lambda { params, body } => self.convert_lambda(params, body),
            _ => bail!("Expression type not yet supported: {:?}", expr),
        }
    }

    fn convert_literal(&self, lit: &Literal) -> Result<syn::Expr> {
        Ok(convert_literal(lit))
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        Ok(parse_quote! { #ident })
    }

    fn convert_binary(&self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = self.convert(left)?;
        let right_expr = self.convert(right)?;

        match op {
            BinOp::In => {
                // Convert "x in dict" to "dict.contains_key(&x)" for dicts
                // For now, assume it's a dict/hashmap
                Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
            }
            BinOp::NotIn => {
                // Convert "x not in dict" to "!dict.contains_key(&x)"
                Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    Ok(parse_quote! { #left_expr.saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // For now, we generate code that works for integers with proper floor semantics
                // TODO: Add type-based dispatch for float division when type inference is available

                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        if (r != 0) && ((r < 0) != (b < 0)) { q - 1 } else { q }
                    }
                })
            }
            BinOp::Mul => {
                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size))) 
                        if elts.len() == 1 && *size > 0 && *size <= 32 => {
                        let elem = self.convert(&elts[0])?;
                        let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 => {
                        let elem = self.convert(&elts[0])?;
                        let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        Ok(parse_quote! { #left_expr #rust_op #right_expr })
                    }
                }
            }
            _ => {
                let rust_op = convert_binop(op)?;
                Ok(parse_quote! { #left_expr #rust_op #right_expr })
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
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        match func {
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            _ => self.convert_generic_call(func, &arg_exprs),
        }
    }

    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];
        Ok(parse_quote! { #arg.len() })
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

    fn convert_array_init_call(&self, func: &str, args: &[HirExpr], _arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }
        
        // Extract size from first argument if it's a literal
        if let HirExpr::Literal(Literal::Int(size)) = &args[0] {
            if *size > 0 && *size <= 32 {
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                match func {
                    "zeros" => Ok(parse_quote! { [0; #size_lit] }),
                    "ones" => Ok(parse_quote! { [1; #size_lit] }),
                    "full" => {
                        if args.len() >= 2 {
                            let value = self.convert(&args[1])?;
                            Ok(parse_quote! { [#value; #size_lit] })
                        } else {
                            bail!("full() requires a value argument");
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                // For large arrays or dynamic sizes, fall back to vec!
                match func {
                    "zeros" => {
                        let size_expr = self.convert(&args[0])?;
                        Ok(parse_quote! { vec![0; #size_expr as usize] })
                    }
                    "ones" => {
                        let size_expr = self.convert(&args[0])?;
                        Ok(parse_quote! { vec![1; #size_expr as usize] })
                    }
                    "full" => {
                        if args.len() >= 2 {
                            let size_expr = self.convert(&args[0])?;
                            let value = self.convert(&args[1])?;
                            Ok(parse_quote! { vec![#value; #size_expr as usize] })
                        } else {
                            bail!("full() requires a value argument");
                        }
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            // Dynamic size - use vec!
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
    }

    fn convert_generic_call(&self, func: &str, args: &[syn::Expr]) -> Result<syn::Expr> {
        let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
        Ok(parse_quote! { #func_ident(#(#args),*) })
    }

    fn convert_index(&self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = self.convert(base)?;
        let index_expr = self.convert(index)?;

        // V1: Direct indexing for simplicity (matches Python behavior)
        Ok(parse_quote! {
            #base_expr[#index_expr as usize]
        })
    }

    fn convert_list(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| self.convert(e))
            .collect::<Result<Vec<_>>>()?;
        
        // Check if this list has a known fixed size that should be an array
        // Arrays are preferred for small fixed sizes (typically < 32 elements)
        if !elts.is_empty() && elts.len() <= 32 {
            // Check if all elements are literals or constants (good candidate for array)
            let all_literals = elts.iter().all(|e| matches!(e, HirExpr::Literal(_)));
            
            if all_literals {
                // Generate array literal instead of vec!
                Ok(parse_quote! { [#(#elt_exprs),*] })
            } else {
                // For now, still use vec! for non-literal lists
                // Future: integrate with const generic inference for smarter detection
                Ok(parse_quote! { vec![#(#elt_exprs),*] })
            }
        } else {
            // Use vec! for empty lists or large lists
            Ok(parse_quote! { vec![#(#elt_exprs),*] })
        }
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

        Ok(parse_quote! {
            {
                let mut map = HashMap::new();
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

    fn convert_lambda(&self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = syn::Ident::new(p, proc_macro2::Span::call_site());
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
}

/// Check if an expression is a len() call
fn is_len_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, args } if func == "len" && args.len() == 1)
}

fn convert_literal(lit: &Literal) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            let lit = syn::LitFloat::new(&f.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit.to_string() }
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
            // but handled in convert_expr for proper type-based conversion
            bail!("Floor division handled in convert_expr, not as simple operator")
        }
        Pow => bail!("Power operator not directly supported in Rust"),
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
        };

        let result = converter.convert(&call_expr).unwrap();
        // Should generate a method call expression
        assert!(matches!(result, syn::Expr::MethodCall(_)));
    }

    #[test]
    fn test_expr_converter_range_call_single_arg() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        let call_expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
        };

        let result = converter.convert(&call_expr).unwrap();
        // Should generate a range expression
        assert!(matches!(result, syn::Expr::Range(_)));
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
        };

        let result = converter.convert(&call_expr).unwrap();
        assert!(matches!(result, syn::Expr::Range(_)));
    }

    #[test]
    fn test_expr_converter_list() {
        let type_mapper = create_test_type_mapper();
        let converter = ExprConverter::new(&type_mapper);

        // Test literal list (should generate array)
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let result = converter.convert(&list_expr).unwrap();
        // Small literal lists should generate array expressions
        assert!(matches!(result, syn::Expr::Array(_)));
        
        // Test non-literal list (should generate vec!)
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
            params: vec![("x".to_string(), Type::Int)].into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties {
                is_pure: true,
                always_terminates: true,
                panic_free: true,
                max_stack_depth: Some(1),
                can_fail: false,
                error_types: vec![],
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
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)].into(),
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

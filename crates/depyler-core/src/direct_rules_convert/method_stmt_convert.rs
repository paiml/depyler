//! Method body statement conversion for direct rules
//!
//! Contains convert_method_body_block, convert_method_body_stmts, convert_method_stmt.

use crate::direct_rules::make_ident;
use crate::hir::*;
use crate::type_mapper::TypeMapper;
use anyhow::{bail, Result};
use syn::parse_quote;

use super::body_convert::*;
use super::stmt_convert::*;
use super::{convert_condition_expr_with_class_fields, convert_expr_with_class_fields, ExprConverter};

/// DEPYLER-0720: Convert method body block with class field type awareness
/// This is used for class methods where we know the field types
/// DEPYLER-1037: Added ret_type parameter for Optional wrapping in return statements
pub(crate) fn convert_method_body_block(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    ret_type: &Type,
) -> Result<syn::Block> {
    let rust_stmts = convert_method_body_stmts(
        stmts,
        type_mapper,
        is_classmethod,
        vararg_functions,
        param_types,
        class_field_types,
        ret_type,
    )?;
    Ok(syn::Block {
        brace_token: Default::default(),
        stmts: rust_stmts,
    })
}

/// DEPYLER-0720: Convert method body statements with class field type awareness
/// DEPYLER-1037: Added ret_type parameter for Optional wrapping in return statements
pub(crate) fn convert_method_body_stmts(
    stmts: &[HirStmt],
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    ret_type: &Type,
) -> Result<Vec<syn::Stmt>> {
    // DEPYLER-0713: Pre-analyze which variables need to be mutable
    let mutable_vars = find_mutable_vars_in_body(stmts);

    stmts
        .iter()
        .map(|stmt| {
            convert_method_stmt(
                stmt,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                class_field_types,
                &mutable_vars,
                ret_type,
            )
        })
        .collect()
}

/// DEPYLER-0720: Convert a single statement with class field type awareness
/// DEPYLER-1037: Added ret_type parameter for Optional wrapping in return statements
#[allow(clippy::too_many_arguments)]
pub(crate) fn convert_method_stmt(
    stmt: &HirStmt,
    type_mapper: &TypeMapper,
    is_classmethod: bool,
    vararg_functions: &std::collections::HashSet<String>,
    param_types: &std::collections::HashMap<String, Type>,
    class_field_types: &std::collections::HashMap<String, Type>,
    mutable_vars: &std::collections::HashSet<String>,
    ret_type: &Type,
) -> Result<syn::Stmt> {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            let value_expr = convert_expr_with_class_fields(
                value,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                class_field_types,
            )?;
            convert_assign_stmt_with_mutable_vars(target, value_expr, type_mapper, mutable_vars)
        }
        HirStmt::Return(expr) => {
            // DEPYLER-1037: Check if return type is Optional for proper wrapping
            let is_optional_return = matches!(ret_type, Type::Optional(_));

            // DEPYLER-1122: Check if return type is bare dict (HashMap<DepylerValue, DepylerValue>)
            // DEPYLER-1166: Also check for Dict[str, Any] which maps to Type::Dict(String, Custom("DepylerValue"))
            let is_bare_dict_return = matches!(
                ret_type,
                Type::Dict(k, v) if matches!((k.as_ref(), v.as_ref()), (Type::Unknown, Type::Unknown))
            ) || matches!(
                ret_type,
                Type::Custom(name) if name == "dict" || name == "Dict"
            ) || matches!(
                ret_type,
                Type::Dict(_, v) if matches!(v.as_ref(), Type::Unknown)
            ) || matches!(
                ret_type,
                Type::Dict(_, v) if matches!(v.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any" || name == "serde_json::Value")
            );

            let ret_expr = if let Some(e) = expr {
                // Check if expression is None literal
                let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));

                // DEPYLER-1122: If returning dict literal with bare dict return type,
                // use specialized converter with DepylerValue wrapping
                // DEPYLER-1166: Pass ret_type to determine if keys should be String or DepylerValue
                if is_bare_dict_return {
                    if let HirExpr::Dict(items) = e {
                        let converter = ExprConverter::new(type_mapper);
                        converter.convert_dict_to_depyler_value(
                            items,
                            class_field_types,
                            ret_type,
                        )?
                    } else {
                        // Not a dict literal, use normal conversion
                        convert_expr_with_class_fields(
                            e,
                            type_mapper,
                            is_classmethod,
                            vararg_functions,
                            param_types,
                            class_field_types,
                        )?
                    }
                } else {
                    let converted = convert_expr_with_class_fields(
                        e,
                        type_mapper,
                        is_classmethod,
                        vararg_functions,
                        param_types,
                        class_field_types,
                    )?;

                    if is_optional_return && !is_none_literal {
                        // Wrap non-None values in Some() for Optional return types
                        parse_quote! { Some(#converted) }
                    } else {
                        converted
                    }
                }
            } else {
                // Bare return statement
                if is_optional_return {
                    parse_quote! { None }
                } else {
                    parse_quote! { () }
                }
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
            // DEPYLER-99MODE: Use truthiness-aware conversion for if conditions
            let cond = convert_condition_expr_with_class_fields(
                condition,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                class_field_types,
            )?;
            let then_block = convert_method_body_block(
                then_body,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                class_field_types,
                ret_type,
            )?;

            let if_expr = if let Some(else_stmts) = else_body {
                let else_block = convert_method_body_block(
                    else_stmts,
                    type_mapper,
                    is_classmethod,
                    vararg_functions,
                    param_types,
                    class_field_types,
                    ret_type,
                )?;
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
            // DEPYLER-99MODE: Use truthiness-aware conversion for while conditions
            let cond = convert_condition_expr_with_class_fields(
                condition,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                class_field_types,
            )?;
            let body_block = convert_method_body_block(
                body,
                type_mapper,
                is_classmethod,
                vararg_functions,
                param_types,
                class_field_types,
                ret_type,
            )?;

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
                            AssignTarget::Symbol(s) => Ok(make_ident(s)),
                            _ => bail!("Nested tuple unpacking not supported in for loops"),
                        })
                        .collect::<Result<Vec<_>>>()?;
                    parse_quote! { (#(#idents),*) }
                }
                _ => bail!("Unsupported for loop target type"),
            };

            // DEPYLER-1051: Extract loop variable type from iterator for type coercion
            // When iterating over self.field where field has type List[T], the loop variable has type T
            let mut loop_param_types = param_types.clone();
            let elem_type = match iter {
                HirExpr::Attribute { value, attr, .. } => {
                    // Check if this is self.field pattern
                    if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                        class_field_types.get(attr).and_then(|t| match t {
                            Type::List(elem_t) => Some(*elem_t.clone()),
                            Type::Set(elem_t) => Some(*elem_t.clone()),
                            _ => None,
                        })
                    } else {
                        None
                    }
                }
                HirExpr::Call { func, args, .. } if func == "enumerate" => {
                    // enumerate(self.field) - extract element type and pair with Int
                    if let Some(HirExpr::Attribute { value, attr, .. }) = args.first() {
                        if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                            class_field_types.get(attr).and_then(|t| match t {
                                Type::List(elem_t) => {
                                    Some(Type::Tuple(vec![Type::Int, *elem_t.clone()]))
                                }
                                Type::Set(elem_t) => {
                                    Some(Type::Tuple(vec![Type::Int, *elem_t.clone()]))
                                }
                                _ => None,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };

            // Add loop variable types to param_types for body processing
            if let Some(elem_type) = elem_type {
                match (target, &elem_type) {
                    (AssignTarget::Symbol(name), _) => {
                        loop_param_types.insert(name.clone(), elem_type);
                    }
                    (AssignTarget::Tuple(targets), Type::Tuple(elem_types))
                        if targets.len() == elem_types.len() =>
                    {
                        for (t, typ) in targets.iter().zip(elem_types.iter()) {
                            if let AssignTarget::Symbol(s) = t {
                                loop_param_types.insert(s.clone(), typ.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }

            // GH-207-PHASE2: Handle dict.items() in for loop context
            // Python: for k, v in dict.items() → Rust: for (k, v) in dict.iter()
            // The standard expr conversion outputs .items() which doesn't exist on HashMap
            let iter_expr = if let HirExpr::MethodCall { object, method, .. } = iter {
                if method == "items" {
                    // Generate object.iter() for iteration
                    let obj_expr = convert_expr_with_class_fields(
                        object,
                        type_mapper,
                        is_classmethod,
                        vararg_functions,
                        param_types,
                        class_field_types,
                    )?;
                    parse_quote! { #obj_expr.iter() }
                } else if method == "keys" {
                    let obj_expr = convert_expr_with_class_fields(
                        object,
                        type_mapper,
                        is_classmethod,
                        vararg_functions,
                        param_types,
                        class_field_types,
                    )?;
                    parse_quote! { #obj_expr.keys() }
                } else if method == "values" {
                    let obj_expr = convert_expr_with_class_fields(
                        object,
                        type_mapper,
                        is_classmethod,
                        vararg_functions,
                        param_types,
                        class_field_types,
                    )?;
                    parse_quote! { #obj_expr.values() }
                } else {
                    convert_expr_with_class_fields(
                        iter,
                        type_mapper,
                        is_classmethod,
                        vararg_functions,
                        param_types,
                        class_field_types,
                    )?
                }
            } else {
                convert_expr_with_class_fields(
                    iter,
                    type_mapper,
                    is_classmethod,
                    vararg_functions,
                    param_types,
                    class_field_types,
                )?
            };
            let body_block = convert_method_body_block(
                body,
                type_mapper,
                is_classmethod,
                vararg_functions,
                &loop_param_types,
                class_field_types,
                ret_type,
            )?;

            let for_expr = parse_quote! {
                for #target_pattern in #iter_expr #body_block
            };

            Ok(syn::Stmt::Expr(for_expr, Some(Default::default())))
        }
        HirStmt::Expr(expr) => {
            if is_pure_expression_direct(expr) {
                let rust_expr = convert_expr_with_class_fields(
                    expr,
                    type_mapper,
                    is_classmethod,
                    vararg_functions,
                    param_types,
                    class_field_types,
                )?;
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
                let rust_expr = convert_expr_with_class_fields(
                    expr,
                    type_mapper,
                    is_classmethod,
                    vararg_functions,
                    param_types,
                    class_field_types,
                )?;
                Ok(syn::Stmt::Expr(rust_expr, Some(Default::default())))
            }
        }
        // For other statement types, fall back to existing conversion
        _ => convert_stmt_with_context(
            stmt,
            type_mapper,
            is_classmethod,
            vararg_functions,
            param_types,
        ),
    }
}

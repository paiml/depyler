//! Statement and expression conversion functions for direct rules
//!
//! DEPYLER-COVERAGE-95: Extracted from direct_rules.rs to reduce file size
//! and improve testability. Contains body/statement/expression conversion.

mod body_convert;
mod expr_advanced;
mod expr_builtins;
mod expr_collections;
mod expr_index_slice;
mod expr_methods;
mod expr_methods_os;
mod expr_operators;
mod method_stmt_convert;
mod operators;
mod stdlib_calls;
mod stmt_convert;
pub(crate) use body_convert::*;
pub(crate) use method_stmt_convert::*;
pub(crate) use operators::*;
pub(crate) use stmt_convert::*;

use crate::direct_rules::{
    extract_nested_indices, make_ident, parse_target_pattern, safe_class_name, type_to_rust_type,
};
use crate::hir::*;
use crate::rust_gen::keywords::safe_ident;
use crate::rust_gen::precedence;
use crate::type_mapper::TypeMapper;
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};


/// Convert HIR expressions to Rust expressions using strategy pattern
#[allow(dead_code)]
pub(crate) fn convert_expr(expr: &HirExpr, type_mapper: &TypeMapper) -> Result<syn::Expr> {
    // Use empty vararg_functions for backward compatibility
    static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    convert_expr_with_context(
        expr,
        type_mapper,
        false,
        EMPTY.get_or_init(std::collections::HashSet::new),
    )
}

/// Convert HIR expressions with classmethod context and vararg tracking
pub(crate) fn convert_expr_with_context(
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
pub(crate) fn convert_expr_with_param_types(
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
pub(crate) fn convert_expr_with_class_fields(
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

/// DEPYLER-1096: Convert condition expression with truthiness coercion
/// Python allows any type in if/while conditions; Rust requires bool.
/// This function converts the expression and applies appropriate truthiness checks.
pub(crate) fn convert_condition_expr(
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
    let rust_expr = converter.convert(expr)?;
    Ok(converter.apply_truthiness_coercion(expr, rust_expr))
}

/// Expression converter using strategy pattern to reduce complexity
pub(crate) struct ExprConverter<'a> {
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
    pub(crate) fn new(type_mapper: &'a TypeMapper) -> Self {
        // Use empty static HashSet for backwards compatibility
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> =
            std::sync::OnceLock::new();
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
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<String>> =
            std::sync::OnceLock::new();
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

    /// DEPYLER-1100: Infer the element type of an iterable expression.
    /// Used to propagate types into generator expression loop variables.
    /// Returns Some(Type::Float) if iterating over floats, None if unknown.
    fn infer_iterable_element_type(&self, iter_expr: &HirExpr) -> Option<Type> {
        match iter_expr {
            // Direct variable reference - check if it's a known float collection
            HirExpr::Var(name) => {
                // Check if it's Vec<float>, List<float>, etc. in param types
                if let Some(Type::List(elem_type) | Type::Set(elem_type)) =
                    self.param_types.get(name)
                {
                    return Some((**elem_type).clone());
                }
                // Check class fields
                if let Some(Type::List(elem_type) | Type::Set(elem_type)) =
                    self.class_field_types.get(name)
                {
                    return Some((**elem_type).clone());
                }
                None
            }
            // Method call like data.values(), items.keys()
            HirExpr::MethodCall { object, method, .. } => {
                match method.as_str() {
                    "values" | "items" | "keys" => {
                        // Recursively check the dict type
                        // For now, just check if the base is a known dict
                        if let HirExpr::Var(name) = object.as_ref() {
                            if let Some(Type::Dict(_, val_type)) = self.param_types.get(name) {
                                if method == "values" {
                                    return Some((**val_type).clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
                None
            }
            // Attribute access like self.data
            HirExpr::Attribute { value, attr } => {
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                    if let Some(Type::List(elem_type) | Type::Set(elem_type)) =
                        self.class_field_types.get(attr)
                    {
                        return Some((**elem_type).clone());
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// DEPYLER-1100: Create a new converter with additional loop variable types.
    /// Used when converting generator expressions to propagate element types.
    fn with_additional_param(&self, var_name: String, var_type: Type) -> Self {
        let mut new_params = self.param_types.clone();
        new_params.insert(var_name, var_type);
        Self {
            type_mapper: self.type_mapper,
            is_classmethod: self.is_classmethod,
            vararg_functions: self.vararg_functions,
            param_types: new_params,
            class_field_types: self.class_field_types.clone(),
        }
    }

    pub(crate) fn convert(&self, expr: &HirExpr) -> Result<syn::Expr> {
        match expr {
            HirExpr::Literal(lit) => self.convert_literal(lit),
            HirExpr::Var(name) => self.convert_variable(name),
            HirExpr::Binary { op, left, right } => self.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => self.convert_unary(*op, operand),
            HirExpr::Call { func, args, .. } => self.convert_call(func, args),
            HirExpr::Index { base, index } => self.convert_index(base, index),
            // DEPYLER-0596: Add Slice support for string slicing with negative indices
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => self.convert_slice(base, start, stop, step),
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
            HirExpr::DynamicCall { callee, args, .. } => self.convert_dynamic_call(callee, args),
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
            // DEPYLER-1100: Propagate element type to loop variable for proper literal coercion
            HirExpr::GeneratorExp {
                element,
                generators,
            } => {
                // Only support single generator for direct rules path
                if generators.len() != 1 {
                    bail!("Multiple generators not supported in direct rules path");
                }
                let gen = &generators[0];
                let iter_expr = self.convert(&gen.iter)?;
                let target_ident = make_ident(&gen.target);

                // DEPYLER-1100: Infer element type and create converter with loop variable typed
                let element_type = self.infer_iterable_element_type(&gen.iter);
                let inner_converter = if let Some(elem_type) = element_type {
                    self.with_additional_param(gen.target.clone(), elem_type)
                } else {
                    // No type info - use self
                    Self {
                        type_mapper: self.type_mapper,
                        is_classmethod: self.is_classmethod,
                        vararg_functions: self.vararg_functions,
                        param_types: self.param_types.clone(),
                        class_field_types: self.class_field_types.clone(),
                    }
                };

                let element_expr = inner_converter.convert(element)?;

                // Handle conditions
                if gen.conditions.is_empty() {
                    Ok(parse_quote! { #iter_expr.iter().map(|#target_ident| #element_expr) })
                } else if gen.conditions.len() == 1 {
                    let cond_expr = inner_converter.convert(&gen.conditions[0])?;
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
            HirExpr::SortByKey {
                iterable,
                key_params,
                key_body,
                reverse_expr,
            } => {
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

}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;


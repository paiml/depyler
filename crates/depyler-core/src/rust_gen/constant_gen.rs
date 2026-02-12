//! Constant generation for module-level Python constants
//!
//! This module handles generating Rust code for Python module-level constants.
//! It includes:
//! - Simple `pub const` declarations for literals (int, float, string, bool)
//! - `Lazy`/`LazyLock` runtime-initialized statics for complex types (Dict, List, Set, etc.)
//! - Type inference for constant expressions
//! - Path constant detection (pathlib support)
//! - Homogeneous collection type inference for concrete typing

use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::func_gen;
use crate::rust_gen::type_gen;
use anyhow::Result;
use quote::quote;

/// Generate a single runtime-initialized constant (Lazy)
///
/// Used for complex constants like Dict/List that need runtime initialization.
/// Complexity: 6 (nested if-else with match arms)
/// DEPYLER-0846: Convert impl Fn to Box<dyn Fn> to avoid E0666 nested impl Trait
pub(super) fn generate_lazy_constant(
    constant: &HirConstant,
    name_ident: syn::Ident,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-1016: Use std::sync::LazyLock in NASA mode (std-only)
    let nasa_mode = ctx.type_mapper.nasa_mode;
    if nasa_mode {
        ctx.needs_lazy_lock = true;
    } else {
        ctx.needs_once_cell = true;
    }

    // DEPYLER-0846: Track if we need to box the closure
    let mut needs_box_wrap = false;

    let type_annotation = if let Some(ref ty) = constant.type_annotation {
        let rust_type = ctx.type_mapper.map_type(ty);
        let syn_type = type_gen::rust_type_to_syn(&rust_type)?;
        // DEPYLER-0846: Convert impl Fn to Box<dyn Fn> for Lazy<> contexts
        // Rust doesn't allow impl Trait in static type positions (E0562)
        let type_str = quote! { #syn_type }.to_string();
        if type_str.contains("impl Fn") {
            needs_box_wrap = true;
            let boxed = type_str.replace("impl Fn", "Box<dyn Fn") + ">";
            // DEPYLER-1022: Use NASA mode aware fallback
            let fallback = if ctx.type_mapper.nasa_mode {
                "String"
            } else {
                ctx.needs_serde_json = true;
                "serde_json::Value"
            };
            let boxed_type: syn::Type =
                syn::parse_str(&boxed).unwrap_or_else(|_| syn::parse_str(fallback).unwrap());
            quote! { #boxed_type }
        } else {
            quote! { #syn_type }
        }
    } else {
        // DEPYLER-0107: Infer type from value expression
        let inferred = infer_lazy_constant_type(&constant.value, ctx);
        // DEPYLER-0846: Also convert inferred types - impl Fn not allowed in static positions (E0562)
        let inferred_str = inferred.to_string();
        if inferred_str.contains("impl Fn") {
            needs_box_wrap = true;
            let boxed = inferred_str.replace("impl Fn", "Box<dyn Fn") + ">";
            // DEPYLER-1022: Use NASA mode aware fallback
            let fallback = if ctx.type_mapper.nasa_mode {
                "String"
            } else {
                ctx.needs_serde_json = true;
                "serde_json::Value"
            };
            let boxed_type: syn::Type =
                syn::parse_str(&boxed).unwrap_or_else(|_| syn::parse_str(fallback).unwrap());
            quote! { #boxed_type }
        } else {
            inferred
        }
    };

    // DEPYLER-0107: Dict/List literals return HashMap/Vec, convert to Value type
    // DEPYLER-0714: Function calls may return Result, unwrap them
    // DEPYLER-0846: Wrap in Box::new() if we converted to Box<dyn Fn>
    // DEPYLER-1016: Skip serde_json in NASA mode
    let final_expr = if constant.type_annotation.is_none() {
        match &constant.value {
            HirExpr::Dict(_) | HirExpr::List(_) => {
                if nasa_mode {
                    // NASA mode: return the value directly without serde_json
                    quote! { #value_expr }
                } else {
                    ctx.needs_serde_json = true;
                    quote! { serde_json::to_value(#value_expr).expect("serde_json serialization failed") }
                }
            }
            HirExpr::Call { .. } => {
                // DEPYLER-0714: Function calls may return Result - unwrap them
                // Python semantics expect the value, not Result
                if needs_box_wrap {
                    quote! { Box::new(#value_expr.expect("function call result unwrap failed")) }
                } else {
                    quote! { #value_expr.expect("function call result unwrap failed") }
                }
            }
            _ => {
                if needs_box_wrap {
                    quote! { Box::new(#value_expr) }
                } else {
                    quote! { #value_expr }
                }
            }
        }
    } else if needs_box_wrap {
        quote! { Box::new(#value_expr) }
    } else {
        quote! { #value_expr }
    };

    // DEPYLER-1016: Use std::sync::LazyLock in NASA mode
    if nasa_mode {
        Ok(quote! {
            pub static #name_ident: std::sync::LazyLock<#type_annotation> = std::sync::LazyLock::new(|| #final_expr);
        })
    } else {
        Ok(quote! {
            pub static #name_ident: once_cell::sync::Lazy<#type_annotation> = once_cell::sync::Lazy::new(|| #final_expr);
        })
    }
}

/// DEPYLER-0107: Infer type for Lazy constants based on value expression
///
/// Most complex constants default to serde_json::Value for compatibility.
/// DEPYLER-0188: Path expressions return std::path::PathBuf.
/// DEPYLER-0714: Function calls use the function's return type (unwrapped if Result).
pub(super) fn infer_lazy_constant_type(
    value: &HirExpr,
    ctx: &mut CodeGenContext,
) -> proc_macro2::TokenStream {
    // DEPYLER-0188: Path expressions should be typed as PathBuf
    if is_path_constant_expr(value) {
        return quote! { std::path::PathBuf };
    }

    // DEPYLER-0714: Function calls - look up return type
    // For Unknown return types, fall through to serde_json::Value default
    if let HirExpr::Call { func, .. } = value {
        if let Some(ret_type) = ctx.function_return_types.get(func) {
            // Skip Unknown - fall through to default
            if !matches!(ret_type, crate::hir::Type::Unknown) {
                if let Ok(syn_type) =
                    type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(ret_type))
                {
                    return quote! { #syn_type };
                }
            }
        }
    }

    // DEPYLER-1016/1060: Handle Dict/List properly in NASA mode using DepylerValue
    // DEPYLER-1060: Use DepylerValue for keys to support non-string keys like {1: "a"}
    // DEPYLER-1128: For homogeneous lists, use concrete types instead of DepylerValue
    if ctx.type_mapper.nasa_mode {
        match value {
            HirExpr::Dict(_) => {
                ctx.needs_hashmap = true;
                ctx.needs_depyler_value_enum = true;
                return quote! { std::collections::HashMap<DepylerValue, DepylerValue> };
            }
            HirExpr::List(elems) => {
                // DEPYLER-1128: Check if list is homogeneous - if so, use concrete type
                if let Some(elem_type) = infer_homogeneous_list_type(elems) {
                    return elem_type;
                }
                // Heterogeneous list - use DepylerValue
                ctx.needs_depyler_value_enum = true;
                return quote! { Vec<DepylerValue> };
            }
            HirExpr::Set(elems) => {
                // DEPYLER-1128: Check if set is homogeneous - if so, use concrete type
                if let Some(elem_type) = infer_homogeneous_set_type(elems) {
                    ctx.needs_hashset = true;
                    return elem_type;
                }
                ctx.needs_hashset = true;
                ctx.needs_depyler_value_enum = true;
                return quote! { std::collections::HashSet<DepylerValue> };
            }
            // DEPYLER-1148: Slice into collections - infer slice type from base
            // A slice of a list is still a list: base[start:stop] where base is Vec<T> -> Vec<T>
            // A slice of a string is still a string: base[start:stop] where base is String -> String
            HirExpr::Slice { base, .. } => {
                if let HirExpr::Var(base_name) = base.as_ref() {
                    if let Some(base_type) = ctx.var_types.get(base_name) {
                        match base_type {
                            // List slice: return Vec<T> (same type as the list)
                            Type::List(elem_type) => {
                                if let Ok(syn_type) =
                                    type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(elem_type))
                                {
                                    return quote! { Vec<#syn_type> };
                                }
                            }
                            // String slice: return String
                            Type::String => {
                                return quote! { String };
                            }
                            _ => {}
                        }
                    }
                }
                // Default for unknown base types: use String (common case)
                return quote! { String };
            }
            // DEPYLER-1060/DEPYLER-1145: Index into collections - infer element type
            // DEPYLER-1145: For homogeneous lists, return the concrete element type, not DepylerValue
            // This fixes: `list_index = list_example[0]` where list_example is Vec<i32>
            // Previously returned DepylerValue causing "expected DepylerValue, found i32" errors
            HirExpr::Index { base, .. } => {
                // Check if base is a variable we can look up
                if let HirExpr::Var(base_name) = base.as_ref() {
                    // Check module-level constants for the base type
                    if let Some(base_type) = ctx.var_types.get(base_name) {
                        match base_type {
                            // Homogeneous list: return element type
                            Type::List(elem_type) => {
                                if let Ok(syn_type) =
                                    type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(elem_type))
                                {
                                    return quote! { #syn_type };
                                }
                            }
                            // Dict: return value type (may be DepylerValue for heterogeneous dicts)
                            Type::Dict(_, val_type) => {
                                if let Ok(syn_type) =
                                    type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(val_type))
                                {
                                    return quote! { #syn_type };
                                }
                            }
                            // Tuple: return Unknown for now (tuple indexing is complex)
                            Type::Tuple(_) => {
                                // Fall through to default handling
                            }
                            _ => {}
                        }
                    }
                }
                // Default: use DepylerValue for unknown bases or dicts with unknown value types
                ctx.needs_depyler_value_enum = true;
                return quote! { DepylerValue };
            }
            // DEPYLER-1128: Handle tuple literals with proper tuple types
            HirExpr::Tuple(elems) => {
                if let Some(tuple_type) = infer_tuple_type(elems) {
                    return tuple_type;
                }
                // Fall through to default if cannot infer
            }
            // DEPYLER-1128: Handle binary expressions - infer from operand types
            HirExpr::Binary { op, left, .. } => {
                if let Some(result_type) = infer_binary_expr_type(op, left) {
                    return result_type;
                }
                // Fall through to default if cannot infer
            }
            // DEPYLER-1128: Handle unary expressions
            HirExpr::Unary { op, operand } => {
                if let Some(result_type) = infer_unary_expr_type(op, operand) {
                    return result_type;
                }
            }
            // DEPYLER-1149: Handle list comprehensions - infer element type from expression
            // `[x*2 for x in range(10)]` produces Vec<i32> (integer arithmetic)
            // `[str(x) for x in items]` produces Vec<String> (string conversion)
            HirExpr::ListComp { element, .. } => {
                if let Some(elem_type) = infer_comprehension_element_type(element) {
                    return quote! { Vec<#elem_type> };
                }
                // Default to Vec<i32> for numeric comprehensions
                return quote! { Vec<i32> };
            }
            // DEPYLER-1149: Handle set comprehensions
            HirExpr::SetComp { element, .. } => {
                ctx.needs_hashset = true;
                if let Some(elem_type) = infer_comprehension_element_type(element) {
                    return quote! { std::collections::HashSet<#elem_type> };
                }
                return quote! { std::collections::HashSet<i32> };
            }
            // DEPYLER-1149: Handle dict comprehensions
            HirExpr::DictComp { key, value, .. } => {
                ctx.needs_hashmap = true;
                let key_type =
                    infer_comprehension_element_type(key).unwrap_or_else(|| quote! { i32 });
                let val_type =
                    infer_comprehension_element_type(value).unwrap_or_else(|| quote! { i32 });
                return quote! { std::collections::HashMap<#key_type, #val_type> };
            }
            // DEPYLER-1172: Handle math module constants (math.pi, math.e, etc.)
            HirExpr::Attribute {
                value: attr_obj,
                attr,
            } => {
                if let HirExpr::Var(module_name) = attr_obj.as_ref() {
                    if module_name == "math" {
                        match attr.as_str() {
                            "pi" | "e" | "tau" | "inf" | "nan" => {
                                return quote! { f64 };
                            }
                            _ => {}
                        }
                    }
                }
            }
            // DEPYLER-1172: Handle math method calls like (16).sqrt(), math.sqrt(16)
            HirExpr::MethodCall { method, .. } => {
                match method.as_str() {
                    // Float-returning math methods
                    "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh"
                    | "tanh" | "exp" | "log" | "log10" | "log2" | "floor" | "ceil" | "trunc"
                    | "fract" | "abs" => {
                        return quote! { f64 };
                    }
                    // String methods
                    "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "join"
                    | "format" | "to_string" | "to_uppercase" | "to_lowercase" | "trim" => {
                        return quote! { String };
                    }
                    // Int methods
                    "count" | "index" | "find" | "rfind" | "len" => {
                        return quote! { i32 };
                    }
                    _ => {}
                }
            }
            // DEPYLER-1172: Handle math function calls like math.sqrt(16)
            HirExpr::Call { func, .. } => match func.as_str() {
                "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh"
                | "tanh" | "exp" | "log" | "log10" | "log2" | "floor" | "ceil" | "trunc"
                | "fabs" => {
                    return quote! { f64 };
                }
                "abs" | "len" | "ord" | "hash" => {
                    return quote! { i32 };
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Default: use serde_json::Value for Lazy constants
    // DEPYLER-1016: Use String in NASA mode
    if ctx.type_mapper.nasa_mode {
        quote! { String }
    } else {
        ctx.needs_serde_json = true;
        quote! { serde_json::Value }
    }
}

/// DEPYLER-1128: Infer type for homogeneous list literals
///
/// Returns `Some(type)` if all elements are the same primitive type,
/// `None` if heterogeneous (requires DepylerValue).
pub(super) fn infer_homogeneous_list_type(elems: &[HirExpr]) -> Option<proc_macro2::TokenStream> {
    if elems.is_empty() {
        // Empty list defaults to Vec<i32> for simplicity
        return Some(quote! { Vec<i32> });
    }

    // Check first element type
    let first_type = match &elems[0] {
        HirExpr::Literal(Literal::Int(_)) => "int",
        HirExpr::Literal(Literal::Float(_)) => "float",
        HirExpr::Literal(Literal::String(_)) => "string",
        HirExpr::Literal(Literal::Bool(_)) => "bool",
        _ => return None, // Non-literal or complex expression - use DepylerValue
    };

    // Verify all elements match
    let all_same = elems.iter().all(|e| {
        matches!(
            (first_type, e),
            ("int", HirExpr::Literal(Literal::Int(_)))
                | ("float", HirExpr::Literal(Literal::Float(_)))
                | ("string", HirExpr::Literal(Literal::String(_)))
                | ("bool", HirExpr::Literal(Literal::Bool(_)))
        )
    });

    if all_same {
        Some(match first_type {
            "int" => quote! { Vec<i32> },
            "float" => quote! { Vec<f64> },
            "string" => quote! { Vec<String> },
            "bool" => quote! { Vec<bool> },
            _ => return None,
        })
    } else {
        None // Heterogeneous - needs DepylerValue
    }
}

/// DEPYLER-1128: Infer type for binary expressions
pub(super) fn infer_binary_expr_type(
    op: &crate::hir::BinOp,
    left: &HirExpr,
) -> Option<proc_macro2::TokenStream> {
    use crate::hir::BinOp;

    // Determine result type from operator and left operand
    match op {
        // Comparison operators always return bool
        BinOp::Eq
        | BinOp::NotEq
        | BinOp::Lt
        | BinOp::LtEq
        | BinOp::Gt
        | BinOp::GtEq
        | BinOp::In
        | BinOp::NotIn => Some(quote! { bool }),

        // Logical operators return bool
        BinOp::And | BinOp::Or => Some(quote! { bool }),

        // Division always returns f64 in Python semantics (true division)
        BinOp::Div => Some(quote! { f64 }),

        // Arithmetic operators - infer from left operand
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Mod
        | BinOp::Pow
        | BinOp::FloorDiv
        | BinOp::BitAnd
        | BinOp::BitOr
        | BinOp::BitXor
        | BinOp::LShift
        | BinOp::RShift => {
            match left {
                HirExpr::Literal(Literal::Int(_)) => Some(quote! { i32 }),
                HirExpr::Literal(Literal::Float(_)) => Some(quote! { f64 }),
                HirExpr::Literal(Literal::String(_)) => Some(quote! { String }),
                HirExpr::Binary {
                    op: inner_op,
                    left: inner_left,
                    ..
                } => {
                    // Recursively infer from nested binary expr
                    infer_binary_expr_type(inner_op, inner_left)
                }
                _ => None,
            }
        }
    }
}

/// DEPYLER-1128: Infer type for unary expressions
pub(super) fn infer_unary_expr_type(
    op: &crate::hir::UnaryOp,
    operand: &HirExpr,
) -> Option<proc_macro2::TokenStream> {
    use crate::hir::UnaryOp;

    match op {
        UnaryOp::Not => Some(quote! { bool }),
        UnaryOp::Neg | UnaryOp::Pos => match operand {
            HirExpr::Literal(Literal::Int(_)) => Some(quote! { i32 }),
            HirExpr::Literal(Literal::Float(_)) => Some(quote! { f64 }),
            _ => None,
        },
        UnaryOp::BitNot => Some(quote! { i32 }), // Bitwise NOT returns int
    }
}

/// DEPYLER-1149: Infer element type from comprehension expression
///
/// Analyzes the comprehension element expression to determine output type.
/// `[x*2 for x in range(10)]` -> i32 (integer arithmetic on loop variable)
/// `[str(x) for x in items]` -> String (string conversion)
pub(super) fn infer_comprehension_element_type(
    element: &HirExpr,
) -> Option<proc_macro2::TokenStream> {
    match element {
        // Direct literals
        HirExpr::Literal(Literal::Int(_)) => Some(quote! { i32 }),
        HirExpr::Literal(Literal::Float(_)) => Some(quote! { f64 }),
        HirExpr::Literal(Literal::String(_)) => Some(quote! { String }),
        HirExpr::Literal(Literal::Bool(_)) => Some(quote! { bool }),

        // Binary expressions - infer from operator and operands
        HirExpr::Binary { op, left, .. } => infer_binary_expr_type(op, left),

        // Variable reference - assume i32 for loop variables (most common case)
        // e.g., `[x for x in range(10)]` where x is the loop variable
        HirExpr::Var(_) => Some(quote! { i32 }),

        // Function/method calls that produce known types
        HirExpr::Call { func, .. } => {
            match func.as_str() {
                "str" | "repr" | "chr" => Some(quote! { String }),
                "int" | "len" | "ord" | "hash" => Some(quote! { i32 }),
                "float" => Some(quote! { f64 }),
                "bool" => Some(quote! { bool }),
                "abs" => Some(quote! { i32 }), // Commonly used with integers
                "round" => Some(quote! { i32 }),
                "min" | "max" | "sum" => Some(quote! { i32 }),
                _ => None,
            }
        }

        // Method calls
        HirExpr::MethodCall { method, .. } => match method.as_str() {
            "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "replace" | "join" | "format" => {
                Some(quote! { String })
            }
            "count" | "index" | "find" | "rfind" => Some(quote! { i32 }),
            _ => None,
        },

        // Unary expressions
        HirExpr::Unary { op, operand } => infer_unary_expr_type(op, operand),

        // Tuple - use tuple type
        HirExpr::Tuple(_) => None, // Complex, fall through to default

        _ => None,
    }
}

/// DEPYLER-1145: Infer element type from list/set literal for module-level constant tracking
///
/// Returns the HIR Type of list elements, used to track concrete types in var_types.
/// This enables proper type inference when indexing into homogeneous lists.
pub(super) fn infer_list_element_type(elems: &[HirExpr]) -> Type {
    if elems.is_empty() {
        // Empty list defaults to Int for simplicity
        return Type::Int;
    }

    // Check first element type
    match &elems[0] {
        HirExpr::Literal(Literal::Int(_)) => {
            // Verify all elements are integers
            if elems
                .iter()
                .all(|e| matches!(e, HirExpr::Literal(Literal::Int(_))))
            {
                Type::Int
            } else {
                Type::Unknown // Heterogeneous
            }
        }
        HirExpr::Literal(Literal::Float(_)) => {
            // Verify all elements are floats (or ints - promote to float)
            if elems.iter().all(|e| {
                matches!(
                    e,
                    HirExpr::Literal(Literal::Float(_)) | HirExpr::Literal(Literal::Int(_))
                )
            }) {
                Type::Float
            } else {
                Type::Unknown
            }
        }
        HirExpr::Literal(Literal::String(_)) => {
            if elems
                .iter()
                .all(|e| matches!(e, HirExpr::Literal(Literal::String(_))))
            {
                Type::String
            } else {
                Type::Unknown
            }
        }
        HirExpr::Literal(Literal::Bool(_)) => {
            if elems
                .iter()
                .all(|e| matches!(e, HirExpr::Literal(Literal::Bool(_))))
            {
                Type::Bool
            } else {
                Type::Unknown
            }
        }
        _ => Type::Unknown, // Non-literal or complex expression
    }
}

/// DEPYLER-1128: Infer type for tuple literals
pub(super) fn infer_tuple_type(elems: &[HirExpr]) -> Option<proc_macro2::TokenStream> {
    if elems.is_empty() {
        return Some(quote! { () });
    }

    // Generate tuple type based on element types
    let elem_types: Vec<_> = elems
        .iter()
        .map(|e| match e {
            HirExpr::Literal(Literal::Int(_)) => Some(quote! { i32 }),
            HirExpr::Literal(Literal::Float(_)) => Some(quote! { f64 }),
            HirExpr::Literal(Literal::String(_)) => Some(quote! { String }),
            HirExpr::Literal(Literal::Bool(_)) => Some(quote! { bool }),
            HirExpr::Literal(Literal::None) => Some(quote! { Option<()> }),
            _ => None, // Complex expression - cannot infer
        })
        .collect();

    // If all elements have known types, return the tuple type
    if elem_types.iter().all(|t| t.is_some()) {
        let types: Vec<_> = elem_types.into_iter().map(|t| t.unwrap()).collect();
        Some(quote! { (#(#types),*) })
    } else {
        None
    }
}

/// DEPYLER-1128: Infer type for homogeneous set literals
pub(super) fn infer_homogeneous_set_type(elems: &[HirExpr]) -> Option<proc_macro2::TokenStream> {
    if elems.is_empty() {
        return Some(quote! { std::collections::HashSet<i32> });
    }

    // Check first element type
    let first_type = match &elems[0] {
        HirExpr::Literal(Literal::Int(_)) => "int",
        HirExpr::Literal(Literal::String(_)) => "string",
        _ => return None,
    };

    // Verify all elements match
    let all_same = elems.iter().all(|e| {
        matches!(
            (first_type, e),
            ("int", HirExpr::Literal(Literal::Int(_)))
                | ("string", HirExpr::Literal(Literal::String(_)))
        )
    });

    if all_same {
        Some(match first_type {
            "int" => quote! { std::collections::HashSet<i32> },
            "string" => quote! { std::collections::HashSet<String> },
            _ => return None,
        })
    } else {
        None
    }
}

/// DEPYLER-1128: Check if expression contains operations that can't be const-evaluated
///
/// Returns true if expression uses methods like .to_string() or comparisons
/// that generate non-const code.
pub(super) fn expr_contains_non_const_ops(expr: &HirExpr) -> bool {
    match expr {
        // String comparisons with != or == generate .to_string() calls
        HirExpr::Binary { op, left, right } => {
            let is_string_comparison =
                matches!(
                    (&**left, &**right),
                    (HirExpr::Literal(Literal::String(_)), _)
                        | (_, HirExpr::Literal(Literal::String(_)))
                ) && matches!(op, crate::hir::BinOp::Eq | crate::hir::BinOp::NotEq);

            is_string_comparison
                || expr_contains_non_const_ops(left)
                || expr_contains_non_const_ops(right)
        }
        // Unary operations might contain non-const ops
        HirExpr::Unary { operand, .. } => expr_contains_non_const_ops(operand),
        // Method calls are generally not const
        HirExpr::MethodCall { .. } => true,
        // Function calls generally not const
        HirExpr::Call { .. } => true,
        _ => false,
    }
}

/// Generate a single simple constant (pub const)
///
/// Used for literals and simple expressions that can be const-evaluated.
/// Complexity: 4 (if-else with helper call)
///
/// DEPYLER-0599: Resolved string literal const type mismatch.
/// String literals at module level should be `&str` without `.to_string()`.
pub(super) fn generate_simple_constant(
    constant: &HirConstant,
    name_ident: syn::Ident,
    value_expr: syn::Expr,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    let type_annotation = if let Some(ref ty) = constant.type_annotation {
        // DEPYLER-0714: Skip Unknown type annotation - would generate TypeParam("T")
        // which is undefined. Fall through to inference to get proper type.
        if matches!(ty, crate::hir::Type::Unknown) {
            infer_constant_type(&constant.value, ctx)
        } else {
            let rust_type = ctx.type_mapper.map_type(ty);
            let syn_type = type_gen::rust_type_to_syn(&rust_type)?;
            quote! { : #syn_type }
        }
    } else {
        infer_constant_type(&constant.value, ctx)
    };

    // DEPYLER-0599: For string literals assigned to const, use raw literal (no .to_string())
    // The string optimizer may have added .to_string() but for const &str we need the bare literal
    let final_value_expr = if let HirExpr::Literal(Literal::String(s)) = &constant.value {
        // Generate raw string literal for const &str
        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
        syn::parse_quote! { #lit }
    } else {
        value_expr
    };

    Ok(quote! {
        pub const #name_ident #type_annotation = #final_value_expr;
    })
}

/// DEPYLER-0516: Infer type annotation for constant expression
///
/// Determines the Rust type for module-level constant expressions.
/// Complexity: 7 (match with 6 arms + default)
pub(super) fn infer_constant_type(
    value: &HirExpr,
    ctx: &mut CodeGenContext,
) -> proc_macro2::TokenStream {
    match value {
        // Literal types
        HirExpr::Literal(Literal::Int(_)) => quote! { : i32 },
        HirExpr::Literal(Literal::Float(_)) => quote! { : f64 },
        HirExpr::Literal(Literal::String(_)) => quote! { : &str },
        HirExpr::Literal(Literal::Bool(_)) => quote! { : bool },
        // DEPYLER-0798: None literal should be Option<()>, not ()
        // Python `None` maps to Rust `Option::None`, which requires Option<T> type
        HirExpr::Literal(Literal::None) => quote! { : Option<()> },

        // DEPYLER-0516: Unary operations preserve type (helper extracts unary logic)
        HirExpr::Unary { op, operand } => infer_unary_type(op, operand, ctx),

        // DEPYLER-0188: Path expressions should be typed as PathBuf
        // Detect Path() calls, .parent, .join method chains, and path / segment division
        _ if is_path_constant_expr(value) => {
            quote! { : std::path::PathBuf }
        }

        // DEPYLER-0713: Function calls - look up return type from function signatures
        // DEPYLER-1022: Use fallback_type_annotation for NASA mode support
        HirExpr::Call { func, .. } => {
            if let Some(ret_type) = ctx.function_return_types.get(func) {
                // DEPYLER-0714: Skip Unknown return type - would generate TypeParam("T")
                // Fall through to inference instead
                if matches!(ret_type, crate::hir::Type::Unknown) {
                    ctx.fallback_type_annotation()
                } else {
                    // Use the function's return type
                    match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(ret_type)) {
                        Ok(syn_type) => quote! { : #syn_type },
                        Err(_) => ctx.fallback_type_annotation(),
                    }
                }
            } else {
                // DEPYLER-0713: Try infer_expr_type_simple for builtin calls
                let inferred = func_gen::infer_expr_type_simple(value);
                if !matches!(inferred, crate::hir::Type::Unknown) {
                    match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(&inferred)) {
                        Ok(syn_type) => quote! { : #syn_type },
                        Err(_) => ctx.fallback_type_annotation(),
                    }
                } else {
                    ctx.fallback_type_annotation()
                }
            }
        }

        // DEPYLER-0713: Variable references - look up tracked type
        HirExpr::Var(name) => {
            if let Some(var_type) = ctx.var_types.get(name) {
                match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(var_type)) {
                    Ok(syn_type) => quote! { : #syn_type },
                    Err(_) => ctx.fallback_type_annotation(),
                }
            } else {
                ctx.fallback_type_annotation()
            }
        }

        // DEPYLER-1172: Handle math module constants (math.pi, math.e, etc.)
        // These are f64 constants, not String
        HirExpr::Attribute { value, attr } => {
            // Check if this is a math module attribute
            if let HirExpr::Var(module_name) = value.as_ref() {
                if module_name == "math" {
                    // Math module constants are all f64
                    match attr.as_str() {
                        "pi" | "e" | "tau" | "inf" | "nan" => return quote! { : f64 },
                        _ => {}
                    }
                }
            }
            // Fall through to default inference
            let inferred = func_gen::infer_expr_type_simple(value);
            if !matches!(inferred, crate::hir::Type::Unknown) {
                match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(&inferred)) {
                    Ok(syn_type) => quote! { : #syn_type },
                    Err(_) => ctx.fallback_type_annotation(),
                }
            } else {
                ctx.fallback_type_annotation()
            }
        }

        // Default fallback
        _ => {
            // DEPYLER-0713: Try infer_expr_type_simple before falling back
            // DEPYLER-1022: Use NASA mode aware fallback
            let inferred = func_gen::infer_expr_type_simple(value);
            if !matches!(inferred, crate::hir::Type::Unknown) {
                match type_gen::rust_type_to_syn(&ctx.type_mapper.map_type(&inferred)) {
                    Ok(syn_type) => quote! { : #syn_type },
                    Err(_) => ctx.fallback_type_annotation(),
                }
            } else {
                ctx.fallback_type_annotation()
            }
        }
    }
}

/// DEPYLER-0188: Check if expression is a pathlib Path constant
///
/// Detects Path expressions for correct type inference in module-level constants.
pub(super) fn is_path_constant_expr(value: &HirExpr) -> bool {
    match value {
        // Path() or pathlib.Path() call
        HirExpr::Call { func, .. } => {
            matches!(func.as_str(), "Path" | "PurePath" | "PathBuf")
        }
        // .parent, .join, etc. method calls return paths
        HirExpr::MethodCall { method, object, .. } => {
            matches!(
                method.as_str(),
                "parent"
                    | "join"
                    | "resolve"
                    | "absolute"
                    | "with_name"
                    | "with_suffix"
                    | "to_path_buf"
            ) || is_path_constant_expr(object)
        }
        // .parent attribute access
        HirExpr::Attribute { attr, value, .. } => {
            matches!(attr.as_str(), "parent" | "root" | "anchor") || is_path_constant_expr(value)
        }
        // path / segment division
        HirExpr::Binary {
            left,
            op: BinOp::Div,
            ..
        } => is_path_constant_expr(left),
        _ => false,
    }
}

/// DEPYLER-0516: Infer type annotation for unary expressions
///
/// Handles type inference for unary operations like -1, +1, --1, -1.5, !True, ~0xFF, etc.
/// DEPYLER-1022: Uses NASA mode aware fallback type
/// DEPYLER-1040b: Handles Not and BitNot correctly (no fallthrough to String)
/// Complexity: 7 (recursive pattern matching with early returns)
pub(super) fn infer_unary_type(
    op: &UnaryOp,
    operand: &HirExpr,
    ctx: &mut CodeGenContext,
) -> proc_macro2::TokenStream {
    match (op, operand) {
        // Negation/Positive of int literal -> i32
        (UnaryOp::Neg | UnaryOp::Pos, HirExpr::Literal(Literal::Int(_))) => {
            quote! { : i32 }
        }
        // Negation/Positive of float literal -> f64
        (UnaryOp::Neg | UnaryOp::Pos, HirExpr::Literal(Literal::Float(_))) => {
            quote! { : f64 }
        }
        // DEPYLER-1040b: Logical NOT on bool literal -> bool
        (UnaryOp::Not, HirExpr::Literal(Literal::Bool(_))) => {
            quote! { : bool }
        }
        // DEPYLER-1040b: Bitwise NOT on int literal -> i32
        (UnaryOp::BitNot, HirExpr::Literal(Literal::Int(_))) => {
            quote! { : i32 }
        }
        // Nested unary (e.g., --1, !!True, ~~0xFF) - recursively check inner operand
        (UnaryOp::Neg | UnaryOp::Pos, HirExpr::Unary { operand: inner, .. }) => {
            match inner.as_ref() {
                HirExpr::Literal(Literal::Int(_)) => quote! { : i32 },
                HirExpr::Literal(Literal::Float(_)) => quote! { : f64 },
                _ => ctx.fallback_type_annotation(),
            }
        }
        // DEPYLER-1040b: Nested logical NOT (e.g., !!True)
        (
            UnaryOp::Not,
            HirExpr::Unary {
                operand: inner,
                op: UnaryOp::Not,
            },
        ) => match inner.as_ref() {
            HirExpr::Literal(Literal::Bool(_)) => quote! { : bool },
            _ => ctx.fallback_type_annotation(),
        },
        // DEPYLER-1040b: Nested bitwise NOT (e.g., ~~0xFF)
        (
            UnaryOp::BitNot,
            HirExpr::Unary {
                operand: inner,
                op: UnaryOp::BitNot,
            },
        ) => match inner.as_ref() {
            HirExpr::Literal(Literal::Int(_)) => quote! { : i32 },
            _ => ctx.fallback_type_annotation(),
        },
        // DEPYLER-1040b: NOT on identifier - fallback to bool (logical not always returns bool)
        (UnaryOp::Not, _) => {
            quote! { : bool }
        }
        // Other unary operations - fallback (DEPYLER-1022: NASA mode aware)
        _ => ctx.fallback_type_annotation(),
    }
}

/// Generate module-level constant tokens
///
/// Generates `pub const` declarations for module-level constants.
/// For simple literal values (int, float, string, bool), generates const.
/// For complex expressions (Dict, List), uses once_cell::Lazy for runtime init.
///
/// # DEPYLER-REARCH-001: Phase 3.2 - Fix const initialization
/// HashMap::new() and .insert() are not const-evaluable, so we use Lazy for
/// complex collections.
pub(super) fn generate_constant_tokens(
    constants: &[HirConstant],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    use std::collections::HashMap;

    // DEPYLER-0201: Deduplicate constants by name, keeping LAST occurrence (Python semantics)
    // Python allows reassignment at module level: NAME = "old"; NAME = "new"
    // We must emit only the last value to avoid Rust error E0428 (duplicate definitions)
    let mut last_by_name: HashMap<&str, &HirConstant> = HashMap::new();
    for constant in constants {
        last_by_name.insert(&constant.name, constant);
    }

    // DEPYLER-1060/DEPYLER-1145: Pre-register module-level constants in var_types
    // This enables is_dict_expr() to work for module-level statics like `d = {1: "a"}`
    // DEPYLER-1145: Use concrete element types (e.g., List(Int) not List(Unknown))
    // so that `list_index = list_example[0]` gets typed as i32, not DepylerValue
    for constant in constants {
        let const_type = match &constant.value {
            HirExpr::Dict(_) => Some(crate::hir::Type::Dict(
                Box::new(crate::hir::Type::Unknown),
                Box::new(crate::hir::Type::Unknown),
            )),
            // DEPYLER-1145: Infer concrete element type from list literal
            HirExpr::List(elems) => {
                let elem_type = infer_list_element_type(elems);
                Some(crate::hir::Type::List(Box::new(elem_type)))
            }
            HirExpr::Set(elems) => {
                let elem_type = infer_list_element_type(elems);
                Some(crate::hir::Type::Set(Box::new(elem_type)))
            }
            _ => None,
        };
        if let Some(t) = const_type {
            ctx.var_types.insert(constant.name.clone(), t.clone());
        }
    }

    let mut items = Vec::new();

    // Process in original order but only emit constants that are the "last" for each name
    let mut emitted: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for constant in constants {
        // Skip if we already emitted this name or if this isn't the last occurrence
        if emitted.contains(constant.name.as_str()) {
            continue;
        }
        // Check if this is the last occurrence of this name
        if let Some(&last) = last_by_name.get(constant.name.as_str()) {
            if !std::ptr::eq(constant, last) {
                // Not the last occurrence, skip
                continue;
            }
        }
        emitted.insert(&constant.name);

        let name_ident = syn::Ident::new(&constant.name, proc_macro2::Span::call_site());

        // DEPYLER-0188: Lambdas at module level should become functions, not consts
        // Closures cannot be assigned to const in Rust
        if let HirExpr::Lambda { params, body } = &constant.value {
            let token = super::generate_lambda_as_function(&constant.name, params, body, ctx)?;
            items.push(token);
            continue;
        }

        // DEPYLER-0673: Skip TypeVar calls - they're for type checking, not runtime code
        // Python: T = TypeVar("T")
        // This is only used for static type checking, skip in Rust code generation
        if let HirExpr::Call { func, .. } = &constant.value {
            if func == "TypeVar" {
                continue;
            }
        }

        let value_expr = constant.value.to_rust_expr(ctx)?;

        // DEPYLER-REARCH-001: Complex types need runtime initialization (Lazy)
        // DEPYLER-0188: PathBuf expressions also need runtime init (not const-evaluable)
        // DEPYLER-0714: Function calls also need runtime init - can't be const
        // DEPYLER-1060: Index expressions into statics need runtime init
        // DEPYLER-1128: Binary expressions use PyOps traits which aren't const - need LazyLock
        // DEPYLER-1148: Slice expressions need runtime init for proper type inference
        // DEPYLER-1149: Comprehensions use iterator methods which aren't const
        let needs_runtime_init = matches!(
            &constant.value,
            HirExpr::Dict(_)
                | HirExpr::List(_)
                | HirExpr::Set(_)
                | HirExpr::Tuple(_)
                | HirExpr::Call { .. }
                | HirExpr::Index { .. }
                | HirExpr::Binary { .. }
                | HirExpr::Slice { .. }
                | HirExpr::ListComp { .. }
                | HirExpr::SetComp { .. }
                | HirExpr::DictComp { .. }
        ) || is_path_constant_expr(&constant.value)
            || expr_contains_non_const_ops(&constant.value);

        let token = if needs_runtime_init {
            generate_lazy_constant(constant, name_ident, value_expr, ctx)?
        } else {
            generate_simple_constant(constant, name_ident, value_expr, ctx)?
        };

        items.push(token);
    }

    Ok(items)
}

//! JSON Module Code Generation - EXTREME TDD
//!
//! Handles Python `json` module method conversions to Rust serde_json.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::{HirExpr, Type};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python json module method calls to Rust serde_json
///
/// # Supported Methods
/// - `json.dumps(obj)` → `serde_json::to_string(&obj).unwrap()`
/// - `json.dumps(obj, indent=n)` → `serde_json::to_string_pretty(&obj).unwrap()`
/// - `json.loads(s)` → `serde_json::from_str::<Value>(&s).unwrap()`
/// - `json.dump(obj, file)` → `serde_json::to_writer(file, &obj).unwrap()`
/// - `json.load(file)` → `serde_json::from_reader::<_, Value>(file).unwrap()`
///
/// DEPYLER-1022: NASA mode support - returns stub implementations that don't
/// require serde_json external crate.
///
/// # Complexity: 5 (match with 5 branches)
pub fn convert_json_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    // Convert arguments first
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    // DEPYLER-1022: In NASA mode, use stub implementations
    if ctx.type_mapper.nasa_mode {
        let result = match method {
            "dumps" => convert_dumps_nasa(&arg_exprs)?,
            "loads" => convert_loads_nasa(&arg_exprs, ctx)?,
            "dump" => convert_dump_nasa(&arg_exprs)?,
            "load" => convert_load_nasa(&arg_exprs, ctx)?,
            _ => bail!("json.{} not implemented yet", method),
        };
        return Ok(Some(result));
    }

    // Mark that we need serde_json crate (non-NASA mode)
    ctx.needs_serde_json = true;

    let result = match method {
        "dumps" => convert_dumps(&arg_exprs)?,
        "loads" => convert_loads(&arg_exprs, ctx)?,
        "dump" => convert_dump(&arg_exprs)?,
        "load" => convert_load(&arg_exprs, ctx)?,
        _ => bail!("json.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// Convert json.dumps() call
fn convert_dumps(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 2 {
        bail!("json.dumps() requires 1 or 2 arguments");
    }
    let obj = &arg_exprs[0];

    // DEPYLER-0377: Check if indent parameter is provided
    if arg_exprs.len() >= 2 {
        // json.dumps(obj, indent=n) → serde_json::to_string_pretty(&obj).unwrap()
        Ok(parse_quote! { serde_json::to_string_pretty(&#obj).expect("JSON operation failed") })
    } else {
        // json.dumps(obj) → serde_json::to_string(&obj).unwrap()
        Ok(parse_quote! { serde_json::to_string(&#obj).expect("JSON operation failed") })
    }
}

/// Convert json.loads() call
fn convert_loads(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("json.loads() requires exactly 1 argument");
    }
    let s = &arg_exprs[0];

    // DEPYLER-0962: Check if return type is a Union of dict|list
    if let Some(union_name) = return_type_is_dict_list_union(ctx) {
        ctx.needs_hashmap = true;
        let union_ident: syn::Ident = syn::Ident::new(&union_name, proc_macro2::Span::call_site());
        Ok(parse_quote! {
            {
                let __json_val = serde_json::from_str::<serde_json::Value>(&#s).expect("JSON operation failed");
                match __json_val {
                    serde_json::Value::Object(obj) => #union_ident::Dict(obj.into_iter().collect::<std::collections::HashMap<String, serde_json::Value>>()),
                    serde_json::Value::Array(arr) => #union_ident::List(arr),
                    _ => panic!("json.loads expected dict or list"),
                }
            }
        })
    } else if return_type_needs_json_dict(ctx) {
        // DEPYLER-0703: Check if return type is Dict[str, Any] → HashMap<String, Value>
        ctx.needs_hashmap = true;
        Ok(
            parse_quote! { serde_json::from_str::<std::collections::HashMap<String, serde_json::Value>>(&#s).expect("JSON operation failed") },
        )
    } else {
        // json.loads(s) → serde_json::from_str::<Value>(&s).unwrap()
        Ok(parse_quote! { serde_json::from_str::<serde_json::Value>(&#s).expect("JSON operation failed") })
    }
}

/// Convert json.dump() call
fn convert_dump(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("json.dump() requires exactly 2 arguments (obj, file)");
    }
    let obj = &arg_exprs[0];
    let file = &arg_exprs[1];
    Ok(parse_quote! { serde_json::to_writer(#file, &#obj).expect("JSON operation failed") })
}

/// Convert json.load() call
fn convert_load(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("json.load() requires exactly 1 argument (file)");
    }
    let file = &arg_exprs[0];

    // DEPYLER-0962: Check if return type is a Union of dict|list
    if let Some(union_name) = return_type_is_dict_list_union(ctx) {
        ctx.needs_hashmap = true;
        let union_ident: syn::Ident = syn::Ident::new(&union_name, proc_macro2::Span::call_site());
        Ok(parse_quote! {
            {
                let __json_val = serde_json::from_reader::<_, serde_json::Value>(#file).expect("JSON operation failed");
                match __json_val {
                    serde_json::Value::Object(obj) => #union_ident::Dict(obj.into_iter().collect::<std::collections::HashMap<String, serde_json::Value>>()),
                    serde_json::Value::Array(arr) => #union_ident::List(arr),
                    _ => panic!("json.load expected dict or list"),
                }
            }
        })
    } else {
        Ok(parse_quote! { serde_json::from_reader::<_, serde_json::Value>(#file).expect("JSON operation failed") })
    }
}

/// DEPYLER-0962: Check if return type is a Union of dict and list
pub fn return_type_is_dict_list_union(ctx: &CodeGenContext) -> Option<String> {
    if let Some(Type::Union(types)) = ctx.current_return_type.as_ref() {
        let has_dict = types.iter().any(|t| matches!(t, Type::Dict(_, _)));
        let has_list = types.iter().any(|t| matches!(t, Type::List(_)));
        if has_dict && has_list && types.len() == 2 {
            return Some("DictOrListUnion".to_string());
        }
    }
    None
}

/// DEPYLER-0560: Check if function return type requires serde_json::Value
pub fn return_type_needs_json_dict(ctx: &CodeGenContext) -> bool {
    // Check assignment target type first
    if let Some(ref assign_type) = ctx.current_assign_type {
        match assign_type {
            Type::Dict(_, value_type) => {
                if is_json_value_type(value_type.as_ref()) {
                    return true;
                }
            }
            Type::Custom(s) if s.contains("HashMap") && s.contains("Value") => return true,
            // DEPYLER-1004: Handle bare Dict from typing import (becomes Type::Custom("Dict"))
            Type::Custom(s) if s == "Dict" => return true,
            _ => {}
        }
    }

    // Check function return type
    if let Some(ref ret_type) = ctx.current_return_type {
        match ret_type {
            Type::Dict(_, value_type) => is_json_value_type(value_type.as_ref()),
            Type::Custom(s) if s.contains("HashMap") && s.contains("Value") => true,
            // DEPYLER-1004: Handle bare Dict from typing import (becomes Type::Custom("Dict"))
            Type::Custom(s) if s == "Dict" => true,
            _ => false,
        }
    } else {
        false
    }
}

/// Check if a type represents serde_json::Value
fn is_json_value_type(t: &Type) -> bool {
    match t {
        Type::Unknown => true,
        Type::Custom(s) => s.contains("Value") || s.contains("Any"),
        _ => false,
    }
}

/// DEPYLER-1153: Check if return type expects DepylerValue keys
/// DEPYLER-1318-FIX: Bare Dict and Dict(Unknown, _) use String keys, NOT DepylerValue keys!
/// This aligns with stmt_gen.rs line 6885 and type_mapper.rs DEPYLER-1318 Dict Unification.
/// Only explicit DepylerValue key type (rare) would return true here.
fn return_type_needs_depyler_value_keys(ctx: &CodeGenContext) -> bool {
    if let Some(ref ret_type) = ctx.current_return_type {
        match ret_type {
            // DEPYLER-1318-FIX: Bare Dict maps to HashMap<String, DepylerValue> (String keys!)
            // Per stmt_gen.rs DEPYLER-1203 and type_mapper.rs DEPYLER-1318 Dict Unification
            Type::Custom(s) if s == "Dict" => false,
            // DEPYLER-1318-FIX: Dict(Unknown, _) uses String keys per type_mapper.rs line 200
            Type::Dict(key_type, _) => {
                // Only use DepylerValue keys when EXPLICITLY specified as such
                matches!(key_type.as_ref(), Type::Custom(s) if s == "DepylerValue")
            }
            _ => false,
        }
    } else {
        // No return type specified - default to String keys (most common pattern)
        false
    }
}

// ============ NASA Mode Stub Functions ============
// DEPYLER-1022: These functions provide std-only implementations that compile
// without external crates. They use format!("{:?}", ...) for serialization
// and return empty containers for deserialization.

/// NASA mode: json.dumps() → format!("{:?}", obj)
fn convert_dumps_nasa(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 2 {
        bail!("json.dumps() requires 1 or 2 arguments");
    }
    let obj = &arg_exprs[0];
    // Use Debug formatting as a simple serialization
    Ok(parse_quote! { format!("{:?}", #obj) })
}

/// NASA mode: json.loads() → empty HashMap stub
/// DEPYLER-1051: Uses DepylerValue for Hybrid Fallback Strategy
/// DEPYLER-1153: Use DepylerValue keys when return type is bare Dict
fn convert_loads_nasa(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("json.loads() requires exactly 1 argument");
    }
    let _s = &arg_exprs[0];
    ctx.needs_hashmap = true;
    ctx.needs_depyler_value_enum = true; // DEPYLER-1051: Need DepylerValue enum
                                         // Return empty HashMap as stub - actual parsing requires serde_json
                                         // DEPYLER-1153: Check if return type expects DepylerValue keys (bare Dict)
    if return_type_needs_depyler_value_keys(ctx) {
        Ok(parse_quote! { std::collections::HashMap::<DepylerValue, DepylerValue>::new() })
    } else {
        // DEPYLER-1051: Use String keys when return type is Dict[str, Any] or similar
        Ok(parse_quote! { std::collections::HashMap::<String, DepylerValue>::new() })
    }
}

/// NASA mode: json.dump() → write Debug format to file (stub)
fn convert_dump_nasa(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("json.dump() requires exactly 2 arguments (obj, file)");
    }
    let obj = &arg_exprs[0];
    let _file = &arg_exprs[1];
    // Stub: just format the object (file writing requires more complex handling)
    Ok(parse_quote! { format!("{:?}", #obj) })
}

/// NASA mode: json.load() → empty HashMap stub
/// DEPYLER-1051: Uses DepylerValue for Hybrid Fallback Strategy
/// DEPYLER-1153: Use DepylerValue keys when return type is bare Dict
fn convert_load_nasa(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("json.load() requires exactly 1 argument (file)");
    }
    let _file = &arg_exprs[0];
    ctx.needs_hashmap = true;
    ctx.needs_depyler_value_enum = true; // DEPYLER-1051: Need DepylerValue enum
                                         // Return empty HashMap as stub
                                         // DEPYLER-1153: Check if return type expects DepylerValue keys (bare Dict)
    if return_type_needs_depyler_value_keys(ctx) {
        Ok(parse_quote! { std::collections::HashMap::<DepylerValue, DepylerValue>::new() })
    } else {
        // DEPYLER-1051: Use String keys when return type is Dict[str, Any] or similar
        Ok(parse_quote! { std::collections::HashMap::<String, DepylerValue>::new() })
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    /// Create context with NASA mode disabled for testing serde_json integration
    fn ctx_with_serde_json() -> CodeGenContext<'static> {
        let mut ctx = CodeGenContext::default();
        ctx.type_mapper = Box::leak(Box::new(ctx.type_mapper.clone().with_nasa_mode(false)));
        ctx
    }

    // ============ NASA Mode Tests (Default) ============

    #[test]
    fn test_nasa_mode_json_dumps_returns_format() {
        let mut ctx = CodeGenContext::default(); // Default is NASA mode
        let args = vec![HirExpr::Var("data".to_string())];

        let result = convert_json_method("dumps", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        assert!(!ctx.needs_serde_json); // NASA mode doesn't need serde_json
    }

    #[test]
    fn test_nasa_mode_json_loads_returns_hashmap() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("json_str".to_string())];

        let result = convert_json_method("loads", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(!ctx.needs_serde_json); // NASA mode doesn't need serde_json
        assert!(ctx.needs_hashmap); // But does need HashMap
    }

    // ============ Non-NASA Mode Tests (serde_json) ============

    #[test]
    fn test_convert_json_dumps_single_arg() {
        let mut ctx = ctx_with_serde_json();
        let args = vec![HirExpr::Var("data".to_string())];

        let result = convert_json_method("dumps", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        assert!(ctx.needs_serde_json);
    }

    #[test]
    fn test_convert_json_dumps_with_indent() {
        let mut ctx = ctx_with_serde_json();
        let args = vec![
            HirExpr::Var("data".to_string()),
            HirExpr::Literal(Literal::Int(2)),
        ];

        let result = convert_json_method("dumps", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_serde_json);
    }

    #[test]
    fn test_convert_json_dumps_no_args_error() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];

        let result = convert_json_method("dumps", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_json_dumps_too_many_args_error() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("a".to_string()),
            HirExpr::Var("b".to_string()),
            HirExpr::Var("c".to_string()),
        ];

        let result = convert_json_method("dumps", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_json_loads_single_arg() {
        let mut ctx = ctx_with_serde_json();
        let args = vec![HirExpr::Var("json_str".to_string())];

        let result = convert_json_method("loads", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_serde_json);
    }

    #[test]
    fn test_convert_json_loads_wrong_args_error() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];

        let result = convert_json_method("loads", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_json_loads_with_dict_list_union_return() {
        let mut ctx = ctx_with_serde_json();
        ctx.current_return_type = Some(Type::Union(vec![
            Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
            Type::List(Box::new(Type::Unknown)),
        ]));
        let args = vec![HirExpr::Var("json_str".to_string())];

        let result = convert_json_method("loads", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_convert_json_loads_with_dict_any_return() {
        let mut ctx = ctx_with_serde_json();
        ctx.current_return_type = Some(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Unknown), // Unknown represents Any
        ));
        let args = vec![HirExpr::Var("json_str".to_string())];

        let result = convert_json_method("loads", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_convert_json_dump_correct_args() {
        let mut ctx = ctx_with_serde_json(); // Non-NASA mode
        let args = vec![
            HirExpr::Var("data".to_string()),
            HirExpr::Var("file".to_string()),
        ];

        let result = convert_json_method("dump", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_serde_json);
    }

    #[test]
    fn test_convert_json_dump_wrong_args_error() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("data".to_string())];

        let result = convert_json_method("dump", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_json_load_single_arg() {
        let mut ctx = ctx_with_serde_json(); // Non-NASA mode
        let args = vec![HirExpr::Var("file".to_string())];

        let result = convert_json_method("load", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_serde_json);
    }

    #[test]
    fn test_convert_json_load_wrong_args_error() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];

        let result = convert_json_method("load", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_json_load_with_dict_list_union_return() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Union(vec![
            Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
            Type::List(Box::new(Type::Unknown)),
        ]));
        let args = vec![HirExpr::Var("file".to_string())];

        let result = convert_json_method("load", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_convert_json_unknown_method_error() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("data".to_string())];

        let result = convert_json_method("unknown_method", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ return_type_is_dict_list_union tests ============

    #[test]
    fn test_return_type_is_dict_list_union_true() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Union(vec![
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            Type::List(Box::new(Type::Int)),
        ]));

        assert!(return_type_is_dict_list_union(&ctx).is_some());
    }

    #[test]
    fn test_return_type_is_dict_list_union_false_no_dict() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Union(vec![
            Type::String,
            Type::List(Box::new(Type::Int)),
        ]));

        assert!(return_type_is_dict_list_union(&ctx).is_none());
    }

    #[test]
    fn test_return_type_is_dict_list_union_false_no_list() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Union(vec![
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            Type::String,
        ]));

        assert!(return_type_is_dict_list_union(&ctx).is_none());
    }

    #[test]
    fn test_return_type_is_dict_list_union_false_too_many_types() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Union(vec![
            Type::Dict(Box::new(Type::String), Box::new(Type::Int)),
            Type::List(Box::new(Type::Int)),
            Type::String,
        ]));

        assert!(return_type_is_dict_list_union(&ctx).is_none());
    }

    #[test]
    fn test_return_type_is_dict_list_union_false_not_union() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::String);

        assert!(return_type_is_dict_list_union(&ctx).is_none());
    }

    #[test]
    fn test_return_type_is_dict_list_union_false_no_return_type() {
        let ctx = CodeGenContext::default();
        assert!(return_type_is_dict_list_union(&ctx).is_none());
    }

    // ============ return_type_needs_json_dict tests ============

    #[test]
    fn test_return_type_needs_json_dict_true_any() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Custom("Any".to_string())), // Any as custom type
        ));

        assert!(return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_true_unknown() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)));

        assert!(return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_true_custom_hashmap_value() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Custom("HashMap<String, Value>".to_string()));

        assert!(return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_false_int_value() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::Dict(Box::new(Type::String), Box::new(Type::Int)));

        assert!(!return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_false_not_dict() {
        let mut ctx = CodeGenContext::default();
        ctx.current_return_type = Some(Type::String);

        assert!(!return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_false_no_return_type() {
        let ctx = CodeGenContext::default();
        assert!(!return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_assign_type_any() {
        let mut ctx = CodeGenContext::default();
        ctx.current_assign_type = Some(Type::Dict(
            Box::new(Type::String),
            Box::new(Type::Unknown), // Unknown represents Any
        ));

        assert!(return_type_needs_json_dict(&ctx));
    }

    #[test]
    fn test_return_type_needs_json_dict_assign_type_custom() {
        let mut ctx = CodeGenContext::default();
        ctx.current_assign_type = Some(Type::Custom("HashMap<String, Value>".to_string()));

        assert!(return_type_needs_json_dict(&ctx));
    }

    // ============ is_json_value_type tests ============

    #[test]
    fn test_is_json_value_type_unknown() {
        assert!(is_json_value_type(&Type::Unknown));
    }

    #[test]
    fn test_is_json_value_type_custom_any() {
        assert!(is_json_value_type(&Type::Custom("Any".to_string())));
    }

    #[test]
    fn test_is_json_value_type_custom_value() {
        assert!(is_json_value_type(&Type::Custom("Value".to_string())));
    }

    #[test]
    fn test_is_json_value_type_int() {
        assert!(!is_json_value_type(&Type::Int));
    }

    #[test]
    fn test_is_json_value_type_string() {
        assert!(!is_json_value_type(&Type::String));
    }
}

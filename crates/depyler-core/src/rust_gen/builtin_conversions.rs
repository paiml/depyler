//! Builtin type conversion code generation
//!
//! This module handles Python builtin type conversions: int(), float(), bool(), str(), len()
//! Extracted from expr_gen.rs as part of DEPYLER-REFACTOR-001 (God File split)
//!
//! # DEPYLER-REFACTOR-001 Traceability
//! - Original location: expr_gen.rs lines 1622-1922
//! - Extraction date: 2025-11-25
//! - Tests: tests/refactor_builtin_conversions_test.rs

use crate::hir::{BinOp, HirExpr, Literal, Type, UnaryOp};
use crate::rust_gen::context::CodeGenContext;
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python len() call to Rust .len() with i32 cast
///
/// Python's len() returns int (maps to i32), but Rust's .len() returns usize.
/// CSE optimization runs before return statement processing, so we need the cast
/// to avoid type mismatches when CSE extracts len() into a temporary variable.
///
/// # DEPYLER-0276: Keep cast for CSE compatibility
///
/// # Complexity: 2
pub fn convert_len_call(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("len() requires exactly one argument");
    }
    let arg = &args[0];
    Ok(parse_quote! { #arg.len() as i32 })
}

/// Convert Python int() call to Rust integer parsing or casting
///
/// Python int() serves multiple purposes:
/// 1. Parse strings to integers (requires .parse())
/// 2. Convert floats to integers (truncation via as i32)
/// 3. Convert bools to integers (False→0, True→1 via as i32)
/// 4. Handle base conversion: int("ff", 16) → i64::from_str_radix
///
/// # DEPYLER-0307 Fix #7: String variables need .parse() not cast
/// # DEPYLER-0327 Fix #1: Improved type inference for method calls
/// # DEPYLER-REFACTOR-001: Handle int(string, base) with from_str_radix
///
/// # Complexity: 9 (within limit)
pub fn convert_int_cast(
    ctx: &CodeGenContext,
    hir_args: &[HirExpr],
    arg_exprs: &[syn::Expr],
    is_string_method_call_fn: impl Fn(&HirExpr, &str, &[HirExpr]) -> bool,
    is_bool_expr_fn: impl Fn(&HirExpr) -> Option<bool>,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 2 {
        bail!("int() requires 1-2 arguments");
    }
    let arg = &arg_exprs[0];

    // Handle int(string, base) with from_str_radix
    // DEPYLER-0653: Add & to convert String to &str
    if arg_exprs.len() == 2 {
        let base = &arg_exprs[1];
        return Ok(parse_quote! { i64::from_str_radix(&#arg, #base).unwrap() });
    }

    if !hir_args.is_empty() {
        match &hir_args[0] {
            // Integer literals don't need casting
            HirExpr::Literal(Literal::Int(_)) => return Ok(arg.clone()),

            // String literals need parsing
            HirExpr::Literal(Literal::String(_)) => {
                return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or_default() });
            }

            // Check if variable is String type
            HirExpr::Var(var_name) => {
                let is_known_string = ctx
                    .var_types
                    .get(var_name)
                    .is_some_and(|t| matches!(t, Type::String));

                // Heuristic: variable names that look like strings
                let name = var_name.as_str();
                let looks_like_string = name.ends_with("_str")
                    || name.ends_with("_string")
                    || name == "s"
                    || name == "string"
                    || name == "text"
                    || name == "word"
                    || name == "line"
                    || name == "value"
                    || name == "value_str"
                    || name.starts_with("str_")
                    || name.starts_with("string_");

                if is_known_string || looks_like_string {
                    return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or_default() });
                }
                return Ok(parse_quote! { (#arg) as i32 });
            }

            // Check if method call returns String type
            HirExpr::MethodCall {
                object,
                method,
                args: method_args,
                ..
            } => {
                if is_string_method_call_fn(object, method, method_args) {
                    return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or_default() });
                }
                return Ok(parse_quote! { (#arg) as i32 });
            }

            // DEPYLER-0654: Check attribute access for string-like attribute names
            HirExpr::Attribute { attr, .. } => {
                let attr_name = attr.as_str();
                let looks_like_string = attr_name.ends_with("_str")
                    || attr_name.ends_with("_string")
                    || attr_name == "text"
                    || attr_name == "string"
                    || attr_name == "word"
                    || attr_name == "line"
                    || attr_name == "input"
                    || attr_name == "name"
                    || attr_name == "message";
                if looks_like_string {
                    return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or_default() });
                }
                return Ok(parse_quote! { (#arg) as i32 });
            }

            // Check if it's a known bool expression
            expr => {
                if let Some(is_bool) = is_bool_expr_fn(expr) {
                    if is_bool {
                        return Ok(parse_quote! { (#arg) as i32 });
                    }
                }
                return Ok(parse_quote! { (#arg) as i32 });
            }
        }
    }

    // DEPYLER-0654: Fallback - check syn::Expr for string-like variable names
    // This handles cases where hir_args is empty but variable name suggests string type
    let check_ident = |ident: &syn::Ident| -> bool {
        let name = ident.to_string();
        name.ends_with("_str")
            || name.ends_with("_string")
            || name == "s"
            || name == "string"
            || name == "text"
            || name == "word"
            || name == "line"
            || name == "input"
            || name == "value_str"
            || name.starts_with("str_")
            || name.starts_with("string_")
    };

    // Check direct path
    if let syn::Expr::Path(path) = arg {
        if let Some(ident) = path.path.get_ident() {
            if check_ident(ident) {
                return Ok(parse_quote! { #arg.parse::<i32>().unwrap_or_default() });
            }
        }
    }

    // Check parenthesized expression like (text)
    if let syn::Expr::Paren(paren) = arg {
        if let syn::Expr::Path(path) = paren.expr.as_ref() {
            if let Some(ident) = path.path.get_ident() {
                if check_ident(ident) {
                    let inner = &paren.expr;
                    return Ok(parse_quote! { #inner.parse::<i32>().unwrap_or_default() });
                }
            }
        }
    }

    Ok(parse_quote! { (#arg) as i32 })
}

/// Convert Python float() call to Rust float parsing or casting
///
/// Python float() serves two purposes:
/// 1. Parse strings to floats (requires .parse())
/// 2. Convert integers to floats (via as f64)
///
/// # Complexity: 6
pub fn convert_float_cast(
    ctx: &CodeGenContext,
    hir_args: &[HirExpr],
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("float() requires exactly one argument");
    }
    let arg = &arg_exprs[0];

    if !hir_args.is_empty() {
        match &hir_args[0] {
            // String literals need parsing
            HirExpr::Literal(Literal::String(_)) => {
                return Ok(parse_quote! { #arg.parse::<f64>().unwrap() });
            }

            // Integer/float literals can use direct cast
            HirExpr::Literal(Literal::Int(_) | Literal::Float(_)) => {
                return Ok(parse_quote! { (#arg) as f64 });
            }

            // Check if variable is known to be String type
            HirExpr::Var(var_name) => {
                let is_string = ctx
                    .var_types
                    .get(var_name)
                    .is_some_and(|t| matches!(t, Type::String))
                    || {
                        // Heuristic: only names that strongly suggest string type
                        // Note: "value" is NOT included - it's too generic and could be any type
                        let name = var_name.as_str();
                        name.ends_with("_str")
                            || name.ends_with("_string")
                            || name == "s"
                            || name == "string"
                            || name == "text"
                    };

                if is_string {
                    return Ok(parse_quote! { #arg.parse::<f64>().unwrap() });
                }
                return Ok(parse_quote! { (#arg) as f64 });
            }

            // DEPYLER-0200: Detect method calls that return strings
            // Expressions like out.split()[0], s.get(), etc. return String
            HirExpr::MethodCall { method, .. } => {
                let string_producing_methods = [
                    "split", "get", "replace", "strip", "lstrip", "rstrip",
                    "upper", "lower", "capitalize", "title", "join",
                    "format", "trim", "read", "readline",
                ];
                if string_producing_methods.contains(&method.as_str()) {
                    return Ok(parse_quote! { #arg.parse::<f64>().unwrap() });
                }
                return Ok(parse_quote! { (#arg) as f64 });
            }

            // DEPYLER-0200: Index on likely-string collections
            // Expressions like words[0] where words is from split() return String
            // DEPYLER-0813: But dict[key] where dict is Dict[str, int] returns int, not String
            // DEPYLER-0813: Also list[i] where list is List[int] returns int, not String
            HirExpr::Index { base, .. } => {
                // Check if base is a dict/list variable with known numeric value type
                if let HirExpr::Var(base_name) = base.as_ref() {
                    if let Some(var_type) = ctx.var_types.get(base_name) {
                        // Dict[K, int] or Dict[K, float] - value is numeric, use as f64
                        if let Type::Dict(_, value_type) = var_type {
                            if matches!(
                                value_type.as_ref(),
                                Type::Int | Type::Float | Type::Unknown
                            ) {
                                return Ok(parse_quote! { (#arg) as f64 });
                            }
                        }
                        // List[int] or List[float] - element is numeric, use as f64
                        if let Type::List(elem_type) = var_type {
                            if matches!(
                                elem_type.as_ref(),
                                Type::Int | Type::Float | Type::Unknown
                            ) {
                                return Ok(parse_quote! { (#arg) as f64 });
                            }
                        }
                    }
                }
                // Default: use parse() for index operations on string collections
                return Ok(parse_quote! { #arg.parse::<f64>().unwrap() });
            }

            // Default: cast for numeric types
            _ => return Ok(parse_quote! { (#arg) as f64 }),
        }
    }

    Ok(parse_quote! { (#arg) as f64 })
}

/// Convert Python str() call to Rust .to_string()
///
/// # DEPYLER-GH121: Wrap argument in parentheses to handle cast expressions
/// Without parens, `x as f32.to_string()` is invalid Rust syntax.
/// With parens, `(x as f32).to_string()` is valid.
///
/// # DEPYLER-0188: PathBuf doesn't implement Display, use .display().to_string()
///
/// # Complexity: 4
pub fn convert_str_conversion(
    hir_args: &[HirExpr],
    args: &[syn::Expr],
    is_path_expr_fn: impl Fn(&HirExpr) -> bool,
) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("str() requires exactly one argument");
    }
    let arg = &args[0];

    // DEPYLER-0188: PathBuf/Path needs .display().to_string()
    if !hir_args.is_empty() && is_path_expr_fn(&hir_args[0]) {
        return Ok(parse_quote! { (#arg).display().to_string() });
    }

    // DEPYLER-GH121: Wrap in parens to handle cast expressions
    Ok(parse_quote! { (#arg).to_string() })
}

/// Convert Python bool() call to Rust truthiness check
///
/// Python bool() checks truthiness:
/// - Strings: non-empty → true, empty → false
/// - Integers: non-zero → true, zero → false
/// - Floats: non-zero → true, zero → false
/// - Lists/collections: non-empty → true, empty → false
///
/// # Complexity: 9 (within limit)
pub fn convert_bool_cast(
    ctx: &CodeGenContext,
    hir_args: &[HirExpr],
    arg_exprs: &[syn::Expr],
) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("bool() requires exactly one argument");
    }
    let arg = &arg_exprs[0];

    // First check syn::Expr for string literals
    if let Some(s) = extract_str_literal(arg) {
        let is_true = !s.is_empty();
        return Ok(parse_quote! { #is_true });
    }

    if !hir_args.is_empty() {
        match &hir_args[0] {
            // String literals: check non-empty
            HirExpr::Literal(Literal::String(s)) => {
                let is_true = !s.is_empty();
                return Ok(parse_quote! { #is_true });
            }

            // Integer literals: check non-zero
            HirExpr::Literal(Literal::Int(n)) => {
                let is_true = *n != 0;
                return Ok(parse_quote! { #is_true });
            }

            // Float literals: check non-zero
            HirExpr::Literal(Literal::Float(f)) => {
                let is_true = *f != 0.0;
                return Ok(parse_quote! { #is_true });
            }

            // Bool literals: identity
            HirExpr::Literal(Literal::Bool(b)) => {
                return Ok(parse_quote! { #b });
            }

            // Variables: check type to determine truthiness check
            HirExpr::Var(var_name) => {
                return convert_bool_var(ctx, var_name, arg);
            }

            // For other expressions, use != 0 for numbers
            _ => {
                return Ok(parse_quote! { #arg != 0 });
            }
        }
    }

    Ok(parse_quote! { #arg != 0 })
}

/// Convert bool() for a variable based on its type
///
/// # Complexity: 7
fn convert_bool_var(ctx: &CodeGenContext, var_name: &str, arg: &syn::Expr) -> Result<syn::Expr> {
    let var_type = ctx.var_types.get(var_name);
    match var_type {
        Some(Type::String) => Ok(parse_quote! { !#arg.is_empty() }),
        Some(Type::Int) => Ok(parse_quote! { #arg != 0 }),
        Some(Type::Float) => Ok(parse_quote! { #arg != 0.0 }),
        Some(Type::Bool) => Ok(arg.clone()),
        Some(Type::List(_) | Type::Set(_) | Type::Dict(_, _)) => {
            Ok(parse_quote! { !#arg.is_empty() })
        }
        _ => {
            // Heuristic for unknown types
            let name = var_name;
            if name.ends_with("_str") || name == "s" || name == "string" {
                Ok(parse_quote! { !#arg.is_empty() })
            } else {
                Ok(parse_quote! { #arg != 0 })
            }
        }
    }
}

/// Extract string literal from various wrapping forms
///
/// Handles:
/// - Direct string literals
/// - Parenthesized expressions
/// - Grouped expressions
/// - .to_string() method calls on literals
///
/// # Complexity: 5
fn extract_str_literal(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Lit(expr_lit) => {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                return Some(lit_str.value());
            }
        }
        syn::Expr::Paren(paren) => {
            return extract_str_literal(&paren.expr);
        }
        syn::Expr::Group(group) => {
            return extract_str_literal(&group.expr);
        }
        syn::Expr::MethodCall(mc) if mc.method == "to_string" => {
            return extract_str_literal(&mc.receiver);
        }
        _ => {}
    }
    None
}

/// Check if object.method() returns String type
///
/// Used to detect .get() on Vec<String> and similar patterns
///
/// # Complexity: 6
pub fn is_string_method_call(
    ctx: &CodeGenContext,
    object: &HirExpr,
    method: &str,
    _args: &[HirExpr],
) -> bool {
    // Check if object is Vec<String> and method is .get()
    if method == "get" {
        if let HirExpr::Var(var_name) = object {
            if let Some(Type::List(inner_type)) = ctx.var_types.get(var_name) {
                return matches!(inner_type.as_ref(), Type::String);
            }
            // Heuristic: Variable names containing "data", "items", "strings"
            let name = var_name.as_str();
            return name.contains("str") || name.contains("data") || name.contains("text");
        }
    }

    // String methods that return String
    matches!(
        method,
        "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "title" | "replace" | "format"
    )
}

/// Check if expression is a boolean expression
///
/// Returns Some(true) if definitely bool, None if unknown
///
/// # Complexity: 5
pub fn is_bool_expr(expr: &HirExpr) -> Option<bool> {
    match expr {
        // Comparison operations always return bool
        HirExpr::Binary {
            op:
                BinOp::Eq
                | BinOp::NotEq
                | BinOp::Lt
                | BinOp::LtEq
                | BinOp::Gt
                | BinOp::GtEq
                | BinOp::In
                | BinOp::NotIn,
            ..
        } => Some(true),
        // Method calls that return bool
        HirExpr::MethodCall { method, .. }
            if matches!(
                method.as_str(),
                "startswith"
                    | "endswith"
                    | "isdigit"
                    | "isalpha"
                    | "isspace"
                    | "isupper"
                    | "islower"
                    | "issubset"
                    | "issuperset"
                    | "isdisjoint"
            ) =>
        {
            Some(true)
        }
        // Boolean literals
        HirExpr::Literal(Literal::Bool(_)) => Some(true),
        // Logical operations
        HirExpr::Unary {
            op: UnaryOp::Not, ..
        } => Some(true),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_len_call_single_arg() {
        let args: Vec<syn::Expr> = vec![parse_quote! { data }];
        let result = convert_len_call(&args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("len"));
        assert!(result_str.contains("as i32"));
    }

    #[test]
    fn test_convert_len_call_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        assert!(convert_len_call(&args).is_err());
    }

    #[test]
    fn test_convert_str_conversion() {
        let hir_args: Vec<HirExpr> = vec![HirExpr::Literal(Literal::Int(42))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 42 }];
        let result = convert_str_conversion(&hir_args, &args, |_| false).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("to_string"));
    }

    #[test]
    fn test_convert_str_conversion_path() {
        let hir_args: Vec<HirExpr> = vec![HirExpr::Var("path".to_string())];
        let args: Vec<syn::Expr> = vec![parse_quote! { path }];
        // Simulate path detection
        let result = convert_str_conversion(&hir_args, &args, |_| true).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("display"), "Expected .display(), got: {}", result_str);
    }

    #[test]
    fn test_extract_str_literal_direct() {
        let expr: syn::Expr = parse_quote! { "hello" };
        assert_eq!(extract_str_literal(&expr), Some("hello".to_string()));
    }

    #[test]
    fn test_extract_str_literal_paren() {
        let expr: syn::Expr = parse_quote! { ("hello") };
        assert_eq!(extract_str_literal(&expr), Some("hello".to_string()));
    }

    #[test]
    fn test_extract_str_literal_to_string() {
        let expr: syn::Expr = parse_quote! { "hello".to_string() };
        assert_eq!(extract_str_literal(&expr), Some("hello".to_string()));
    }

    #[test]
    fn test_is_bool_expr_comparison() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Eq,
            right: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_is_bool_expr_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "startswith".to_string(),
            args: vec![HirExpr::Literal(Literal::String("foo".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_convert_int_cast_string_literal() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::String("42".to_string()))];
        let args: Vec<syn::Expr> = vec![parse_quote! { "42" }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| false, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse, got: {}", result_str);
    }

    #[test]
    fn test_convert_int_cast_int_literal() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::Int(42))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 42 }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| false, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("42"), "Expected 42, got: {}", result_str);
    }

    #[test]
    fn test_convert_int_cast_with_base() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::String("ff".to_string()))];
        let args: Vec<syn::Expr> = vec![parse_quote! { "ff" }, parse_quote! { 16 }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| false, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("from_str_radix"), "Expected from_str_radix, got: {}", result_str);
    }

    #[test]
    fn test_convert_int_cast_var_string_like() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Var("value_str".to_string())];
        let args: Vec<syn::Expr> = vec![parse_quote! { value_str }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| false, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse for string-like var, got: {}", result_str);
    }

    #[test]
    fn test_convert_int_cast_var_numeric() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Var("count".to_string())];
        let args: Vec<syn::Expr> = vec![parse_quote! { count }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| false, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("as i32"), "Expected as i32, got: {}", result_str);
    }

    #[test]
    fn test_convert_int_cast_method_call_string() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "strip".to_string(),
            args: vec![],
            kwargs: vec![],
        }];
        let args: Vec<syn::Expr> = vec![parse_quote! { s.strip() }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| true, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse for string method, got: {}", result_str);
    }

    #[test]
    fn test_convert_int_cast_attribute_string_like() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "text".to_string(),
        }];
        let args: Vec<syn::Expr> = vec![parse_quote! { obj.text }];
        let result = convert_int_cast(&ctx, &hir_args, &args, |_, _, _| false, |_| None).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse for string-like attr, got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_string_literal() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::String("3.14".to_string()))];
        let args: Vec<syn::Expr> = vec![parse_quote! { "3.14" }];
        let result = convert_float_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse, got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_int_literal() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::Int(42))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 42 }];
        let result = convert_float_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("as f64"), "Expected as f64, got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_var_string_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("s".to_string(), Type::String);
        let hir_args = vec![HirExpr::Var("s".to_string())];
        let args: Vec<syn::Expr> = vec![parse_quote! { s }];
        let result = convert_float_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse for String type, got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_method_split() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "split".to_string(),
            args: vec![],
            kwargs: vec![],
        }];
        let args: Vec<syn::Expr> = vec![parse_quote! { s.split() }];
        let result = convert_float_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("parse"), "Expected parse for split(), got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_index_dict_numeric() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("d".to_string(), Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
        let hir_args = vec![HirExpr::Index {
            base: Box::new(HirExpr::Var("d".to_string())),
            index: Box::new(HirExpr::Literal(Literal::String("key".to_string()))),
        }];
        let args: Vec<syn::Expr> = vec![parse_quote! { d["key"] }];
        let result = convert_float_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("as f64"), "Expected as f64 for dict[key], got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_index_list_numeric() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("lst".to_string(), Type::List(Box::new(Type::Int)));
        let hir_args = vec![HirExpr::Index {
            base: Box::new(HirExpr::Var("lst".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        }];
        let args: Vec<syn::Expr> = vec![parse_quote! { lst[0] }];
        let result = convert_float_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("as f64"), "Expected as f64 for list[i], got: {}", result_str);
    }

    #[test]
    fn test_convert_float_cast_wrong_args() {
        let ctx = CodeGenContext::default();
        let args: Vec<syn::Expr> = vec![];
        assert!(convert_float_cast(&ctx, &[], &args).is_err());
    }

    #[test]
    fn test_convert_bool_cast_int_literal() {
        let ctx = CodeGenContext::default();
        // For int literal 42, bool() returns true directly (compile-time eval)
        let hir_args = vec![HirExpr::Literal(Literal::Int(42))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 42 }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("true"), "Expected true, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_int_literal_zero() {
        let ctx = CodeGenContext::default();
        // For int literal 0, bool() returns false directly
        let hir_args = vec![HirExpr::Literal(Literal::Int(0))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 0 }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("false"), "Expected false, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_float_literal() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::Float(3.14))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 3.14 }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("true"), "Expected true, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_float_literal_zero() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::Float(0.0))];
        let args: Vec<syn::Expr> = vec![parse_quote! { 0.0 }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("false"), "Expected false, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_bool_literal_true() {
        let ctx = CodeGenContext::default();
        let hir_args = vec![HirExpr::Literal(Literal::Bool(true))];
        let args: Vec<syn::Expr> = vec![parse_quote! { true }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("true"), "Expected true, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_string_literal() {
        let ctx = CodeGenContext::default();
        // Non-empty string literal gets evaluated at compile time to true
        let hir_args = vec![HirExpr::Literal(Literal::String("hello".to_string()))];
        let args: Vec<syn::Expr> = vec![parse_quote! { "hello" }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("true"), "Expected true, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_empty_string_literal() {
        let ctx = CodeGenContext::default();
        // Empty string literal gets evaluated at compile time to false
        let hir_args = vec![HirExpr::Literal(Literal::String(String::new()))];
        let args: Vec<syn::Expr> = vec![parse_quote! { "" }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("false"), "Expected false, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_var_int_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("n".to_string(), Type::Int);
        let hir_args = vec![HirExpr::Var("n".to_string())];
        let args: Vec<syn::Expr> = vec![parse_quote! { n }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("!= 0"), "Expected != 0 for int, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_var_string_type() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("s".to_string(), Type::String);
        let hir_args = vec![HirExpr::Var("s".to_string())];
        let args: Vec<syn::Expr> = vec![parse_quote! { s }];
        let result = convert_bool_cast(&ctx, &hir_args, &args).unwrap();
        let result_str = quote::quote!(#result).to_string();
        assert!(result_str.contains("is_empty"), "Expected is_empty for String, got: {}", result_str);
    }

    #[test]
    fn test_convert_bool_cast_wrong_args() {
        let ctx = CodeGenContext::default();
        let args: Vec<syn::Expr> = vec![];
        assert!(convert_bool_cast(&ctx, &[], &args).is_err());
    }

    #[test]
    fn test_is_string_method_call_upper() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        // "upper" is in the matches! list
        assert!(is_string_method_call(&ctx, &obj, "upper", &[]));
    }

    #[test]
    fn test_is_string_method_call_lower() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "lower", &[]));
    }

    #[test]
    fn test_is_string_method_call_strip() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "strip", &[]));
    }

    #[test]
    fn test_is_string_method_call_replace() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "replace", &[]));
    }

    #[test]
    fn test_is_string_method_call_title() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "title", &[]));
    }

    #[test]
    fn test_is_string_method_call_lstrip() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "lstrip", &[]));
    }

    #[test]
    fn test_is_string_method_call_rstrip() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "rstrip", &[]));
    }

    #[test]
    fn test_is_string_method_call_format() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("s".to_string());
        assert!(is_string_method_call(&ctx, &obj, "format", &[]));
    }

    #[test]
    fn test_is_string_method_call_get_on_string_list() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("strings".to_string(), Type::List(Box::new(Type::String)));
        let obj = HirExpr::Var("strings".to_string());
        // get on Vec<String> should return true
        assert!(is_string_method_call(&ctx, &obj, "get", &[]));
    }

    #[test]
    fn test_is_string_method_call_get_on_int_list() {
        let mut ctx = CodeGenContext::default();
        ctx.var_types.insert("numbers".to_string(), Type::List(Box::new(Type::Int)));
        let obj = HirExpr::Var("numbers".to_string());
        // get on Vec<i32> should return false
        assert!(!is_string_method_call(&ctx, &obj, "get", &[]));
    }

    #[test]
    fn test_is_string_method_call_not_string() {
        let ctx = CodeGenContext::default();
        let obj = HirExpr::Var("n".to_string());
        // append is not a string method
        assert!(!is_string_method_call(&ctx, &obj, "append", &[]));
    }

    #[test]
    fn test_is_string_method_call_get_heuristic() {
        let ctx = CodeGenContext::default();
        // Variable name "text_data" contains "text" which is a heuristic
        let obj = HirExpr::Var("text_data".to_string());
        assert!(is_string_method_call(&ctx, &obj, "get", &[]));
    }

    #[test]
    fn test_is_string_method_call_get_no_heuristic() {
        let ctx = CodeGenContext::default();
        // Variable name "numbers" doesn't match any string heuristic
        let obj = HirExpr::Var("numbers".to_string());
        assert!(!is_string_method_call(&ctx, &obj, "get", &[]));
    }

    #[test]
    fn test_is_bool_expr_not() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_is_bool_expr_and() {
        // And/Or are not in the checked operators, return None
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::And,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), None);
    }

    #[test]
    fn test_is_bool_expr_or() {
        // And/Or are not in the checked operators, return None
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Or,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), None);
    }

    #[test]
    fn test_is_bool_expr_lt() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Lt,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_is_bool_expr_gt() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Gt,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_is_bool_expr_not_in() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::NotIn,
            right: Box::new(HirExpr::Var("lst".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_is_bool_expr_in() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::In,
            right: Box::new(HirExpr::Var("lst".to_string())),
        };
        assert_eq!(is_bool_expr(&expr), Some(true));
    }

    #[test]
    fn test_is_bool_expr_literal_int() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(is_bool_expr(&expr), None);
    }

    #[test]
    fn test_extract_str_literal_non_string() {
        let expr: syn::Expr = parse_quote! { 42 };
        assert_eq!(extract_str_literal(&expr), None);
    }

    #[test]
    fn test_convert_len_call_two_args() {
        let args: Vec<syn::Expr> = vec![parse_quote! { a }, parse_quote! { b }];
        assert!(convert_len_call(&args).is_err());
    }
}

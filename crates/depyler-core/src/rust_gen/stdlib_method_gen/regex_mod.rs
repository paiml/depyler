//! Regular Expression Module Code Generation - EXTREME TDD
//!
//! Handles Python `re` module method conversions to Rust regex crate.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::{HirExpr, Literal};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python re module method calls to Rust regex
///
/// # Supported Methods
/// - `re.search(pattern, text)` → `Regex::new(pattern).unwrap().find(text)`
/// - `re.match(pattern, text)` → `Regex::new(pattern).unwrap().find(text)`
/// - `re.findall(pattern, text)` → `Regex::new(pattern).find_iter(text).collect()`
/// - `re.finditer(pattern, text)` → `Regex::new(pattern).find_iter(text)`
/// - `re.sub(pattern, repl, text)` → `Regex::new(pattern).replace_all(text, repl)`
/// - `re.subn(pattern, repl, text)` → `(result, count)`
/// - `re.compile(pattern)` → `Regex::new(pattern).unwrap()`
/// - `re.split(pattern, text)` → `Regex::new(pattern).split(text)`
/// - `re.escape(text)` → `regex::escape(text)`
///
/// # Complexity: 10 (match with 10 branches)
/// DEPYLER-1070: Added NASA mode support using DepylerRegexMatch
pub fn convert_re_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    // Convert arguments first
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let nasa_mode = ctx.type_mapper.nasa_mode;

    // DEPYLER-1070: Use DepylerRegexMatch in NASA mode, regex crate otherwise
    if nasa_mode {
        ctx.needs_depyler_regex_match = true;
        return convert_re_method_nasa(method, args, &arg_exprs);
    }

    // Mark that we need regex crate
    ctx.needs_regex = true;

    let result = match method {
        "search" => convert_search(args, &arg_exprs)?,
        "match" => convert_match(args, &arg_exprs)?,
        "findall" => convert_findall(args, &arg_exprs)?,
        "finditer" => convert_finditer(args, &arg_exprs)?,
        "sub" => convert_sub(args, &arg_exprs)?,
        "subn" => convert_subn(args, &arg_exprs)?,
        "compile" => convert_compile(&arg_exprs)?,
        "split" => convert_split(&arg_exprs)?,
        "escape" => convert_escape(&arg_exprs)?,
        "fullmatch" => convert_fullmatch(args, &arg_exprs)?,
        _ => bail!("re.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// DEPYLER-1070: NASA mode regex conversion using DepylerRegexMatch
/// Uses simple string methods instead of regex crate
fn convert_re_method_nasa(
    method: &str,
    args: &[HirExpr],
    arg_exprs: &[syn::Expr],
) -> Result<Option<syn::Expr>> {
    let result = match method {
        "search" => {
            if arg_exprs.len() < 2 {
                bail!("re.search() requires at least 2 arguments (pattern, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let text = extract_str_arg(args, arg_exprs, 1);
            parse_quote! { DepylerRegexMatch::search(#pattern, #text) }
        }
        "match" => {
            if arg_exprs.len() < 2 {
                bail!("re.match() requires at least 2 arguments (pattern, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let text = extract_str_arg(args, arg_exprs, 1);
            parse_quote! { DepylerRegexMatch::match_start(#pattern, #text) }
        }
        "findall" => {
            if arg_exprs.len() < 2 {
                bail!("re.findall() requires at least 2 arguments (pattern, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let text = extract_str_arg(args, arg_exprs, 1);
            parse_quote! { DepylerRegexMatch::findall(#pattern, #text) }
        }
        "finditer" => {
            // NASA mode: return iterator of matches
            if arg_exprs.len() < 2 {
                bail!("re.finditer() requires at least 2 arguments (pattern, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let text = extract_str_arg(args, arg_exprs, 1);
            // Return Vec that can be iterated
            parse_quote! { DepylerRegexMatch::findall(#pattern, #text).into_iter() }
        }
        "sub" => {
            if arg_exprs.len() < 3 {
                bail!("re.sub() requires at least 3 arguments (pattern, repl, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let repl = extract_str_arg(args, arg_exprs, 1);
            let text = extract_str_arg(args, arg_exprs, 2);
            parse_quote! { DepylerRegexMatch::sub(#pattern, #repl, #text) }
        }
        "subn" => {
            if arg_exprs.len() < 3 {
                bail!("re.subn() requires at least 3 arguments (pattern, repl, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let repl = extract_str_arg(args, arg_exprs, 1);
            let text = extract_str_arg(args, arg_exprs, 2);
            parse_quote! {
                {
                    let result = DepylerRegexMatch::sub(#pattern, #repl, #text);
                    let count = (#text).matches(#pattern).count();
                    (result, count)
                }
            }
        }
        "split" => {
            if arg_exprs.len() < 2 {
                bail!("re.split() requires at least 2 arguments (pattern, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let text = extract_str_arg(args, arg_exprs, 1);
            parse_quote! { DepylerRegexMatch::split(#pattern, #text) }
        }
        "compile" => {
            // In NASA mode, compile just returns the pattern string
            // Actual matching happens when methods are called
            if arg_exprs.is_empty() {
                bail!("re.compile() requires at least 1 argument (pattern)");
            }
            let pattern = &arg_exprs[0];
            parse_quote! { #pattern.to_string() }
        }
        "escape" => {
            if arg_exprs.is_empty() {
                bail!("re.escape() requires at least 1 argument (text)");
            }
            let text = &arg_exprs[0];
            // NASA mode: just return the string (no regex metachar escaping needed)
            parse_quote! { #text.to_string() }
        }
        "fullmatch" => {
            // fullmatch checks if entire string matches pattern
            if arg_exprs.len() < 2 {
                bail!("re.fullmatch() requires at least 2 arguments (pattern, string)");
            }
            let pattern = extract_str_arg(args, arg_exprs, 0);
            let text = extract_str_arg(args, arg_exprs, 1);
            parse_quote! {
                if #text == #pattern {
                    Some(DepylerRegexMatch::new(#text, 0, #text.len()))
                } else {
                    None
                }
            }
        }
        _ => bail!("re.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// Helper to extract bare string literals for regex methods
/// Regex::new() and find() expect &str, not String
fn extract_str_arg(args: &[HirExpr], arg_exprs: &[syn::Expr], idx: usize) -> syn::Expr {
    match args.get(idx) {
        Some(HirExpr::Literal(Literal::String(s))) => {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        _ => arg_exprs
            .get(idx)
            .cloned()
            .unwrap_or_else(|| parse_quote! { "" }),
    }
}

/// Convert re.search() call
fn convert_search(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("re.search() requires at least 2 arguments (pattern, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let text = extract_str_arg(args, arg_exprs, 1);

    // Handle optional flags
    if arg_exprs.len() >= 3 {
        Ok(parse_quote! {
            regex::RegexBuilder::new(#pattern)
                .case_insensitive(true)
                .build()
                .unwrap()
                .find(#text)
        })
    } else {
        Ok(parse_quote! { regex::Regex::new(#pattern).unwrap().find(#text) })
    }
}

/// Convert re.match() call
fn convert_match(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("re.match() requires at least 2 arguments (pattern, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let text = extract_str_arg(args, arg_exprs, 1);

    // DEPYLER-0389: re.match() in Python only matches at the beginning
    Ok(parse_quote! { regex::Regex::new(#pattern).unwrap().find(#text) })
}

/// Convert re.findall() call
fn convert_findall(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("re.findall() requires at least 2 arguments (pattern, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let text = extract_str_arg(args, arg_exprs, 1);

    Ok(parse_quote! {
        regex::Regex::new(#pattern)
            .unwrap()
            .find_iter(#text)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<_>>()
    })
}

/// Convert re.finditer() call
fn convert_finditer(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("re.finditer() requires at least 2 arguments (pattern, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let text = extract_str_arg(args, arg_exprs, 1);

    Ok(parse_quote! {
        regex::Regex::new(#pattern)
            .unwrap()
            .find_iter(#text)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<_>>()
    })
}

/// Convert re.sub() call
fn convert_sub(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 3 {
        bail!("re.sub() requires at least 3 arguments (pattern, repl, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let repl = extract_str_arg(args, arg_exprs, 1);
    let text = extract_str_arg(args, arg_exprs, 2);

    Ok(parse_quote! {
        regex::Regex::new(#pattern)
            .unwrap()
            .replace_all(#text, #repl)
            .to_string()
    })
}

/// Convert re.subn() call
fn convert_subn(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 3 {
        bail!("re.subn() requires at least 3 arguments (pattern, repl, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let repl = extract_str_arg(args, arg_exprs, 1);
    let text = extract_str_arg(args, arg_exprs, 2);

    Ok(parse_quote! {
        {
            let re = regex::Regex::new(#pattern).unwrap();
            let count = re.find_iter(#text).count();
            let result = re.replace_all(#text, #repl).to_string();
            (result, count)
        }
    })
}

/// Convert re.compile() call
fn convert_compile(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("re.compile() requires at least 1 argument (pattern)");
    }
    let pattern = &arg_exprs[0];

    if arg_exprs.len() >= 2 {
        Ok(parse_quote! {
            regex::RegexBuilder::new(#pattern)
                .case_insensitive(true)
                .build()
                .unwrap()
        })
    } else {
        Ok(parse_quote! { regex::Regex::new(#pattern).unwrap() })
    }
}

/// Convert re.split() call
fn convert_split(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("re.split() requires at least 2 arguments (pattern, string)");
    }
    let pattern = &arg_exprs[0];
    let text = &arg_exprs[1];

    if arg_exprs.len() >= 3 {
        let maxsplit = &arg_exprs[2];
        Ok(parse_quote! {
            regex::Regex::new(#pattern)
                .unwrap()
                .splitn(#text, #maxsplit + 1)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
    } else {
        Ok(parse_quote! {
            regex::Regex::new(#pattern)
                .unwrap()
                .split(#text)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
    }
}

/// Convert re.escape() call
fn convert_escape(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("re.escape() requires exactly 1 argument");
    }
    let text = &arg_exprs[0];
    Ok(parse_quote! { regex::escape(#text).to_string() })
}

/// DEPYLER-1070: Convert re.fullmatch() call
/// fullmatch checks if the entire string matches the pattern
fn convert_fullmatch(args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("re.fullmatch() requires at least 2 arguments (pattern, string)");
    }
    let pattern = extract_str_arg(args, arg_exprs, 0);
    let text = extract_str_arg(args, arg_exprs, 1);

    // Wrap pattern with ^ and $ to ensure full match
    Ok(parse_quote! {
        regex::Regex::new(&format!("^(?:{})$", #pattern))
            .unwrap()
            .find(#text)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ convert_re_method tests ============

    #[test]
    fn test_convert_re_search_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("abc123".to_string())),
        ];
        let result = convert_re_method("search", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_regex); // Not in NASA mode
    }

    #[test]
    fn test_convert_re_search_with_flags() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("abc123".to_string())),
            HirExpr::Literal(Literal::Int(2)), // Flags as integer
        ];
        let result = convert_re_method("search", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_search_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(r"\d+".to_string()))];
        let result = convert_re_method("search", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_match_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("123abc".to_string())),
        ];
        let result = convert_re_method("match", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_match_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(r"\d+".to_string()))];
        let result = convert_re_method("match", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_findall_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("a1b2c3".to_string())),
        ];
        let result = convert_re_method("findall", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_findall_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(r"\d+".to_string()))];
        let result = convert_re_method("findall", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_finditer_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\w+".to_string())),
            HirExpr::Literal(Literal::String("hello world".to_string())),
        ];
        let result = convert_re_method("finditer", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_finditer_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(r"\w+".to_string()))];
        let result = convert_re_method("finditer", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_sub_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("X".to_string())),
            HirExpr::Literal(Literal::String("a1b2c3".to_string())),
        ];
        let result = convert_re_method("sub", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_sub_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("X".to_string())),
        ];
        let result = convert_re_method("sub", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_subn_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("X".to_string())),
            HirExpr::Literal(Literal::String("a1b2c3".to_string())),
        ];
        let result = convert_re_method("subn", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_subn_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::String("X".to_string())),
        ];
        let result = convert_re_method("subn", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_compile_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(r"\d+".to_string()))];
        let result = convert_re_method("compile", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_compile_with_flags() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\d+".to_string())),
            HirExpr::Literal(Literal::Int(2)), // Flags as integer (re.IGNORECASE = 2)
        ];
        let result = convert_re_method("compile", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_compile_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_re_method("compile", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_split_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\s+".to_string())),
            HirExpr::Literal(Literal::String("hello world".to_string())),
        ];
        let result = convert_re_method("split", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_split_with_maxsplit() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String(r"\s+".to_string())),
            HirExpr::Literal(Literal::String("a b c d".to_string())),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_re_method("split", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_split_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(r"\s+".to_string()))];
        let result = convert_re_method("split", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_escape_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String(
            "hello.*world".to_string(),
        ))];
        let result = convert_re_method("escape", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_re_escape_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_re_method("escape", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_re_unknown_method() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("test".to_string()))];
        let result = convert_re_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ extract_str_arg tests ============

    #[test]
    fn test_extract_str_arg_with_literal() {
        let args = vec![HirExpr::Literal(Literal::String("test".to_string()))];
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote! { "test".to_string() }];
        let result = extract_str_arg(&args, &arg_exprs, 0);
        // Should return the bare literal without .to_string()
        assert!(matches!(result, syn::Expr::Lit(_)));
    }

    #[test]
    fn test_extract_str_arg_with_variable() {
        let args = vec![HirExpr::Var("pattern".to_string())];
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote! { pattern }];
        let result = extract_str_arg(&args, &arg_exprs, 0);
        // Should return the expression from arg_exprs
        assert!(matches!(result, syn::Expr::Path(_)));
    }

    #[test]
    fn test_extract_str_arg_out_of_bounds() {
        let args: Vec<HirExpr> = vec![];
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = extract_str_arg(&args, &arg_exprs, 0);
        // Should return empty string literal as fallback
        assert!(matches!(result, syn::Expr::Lit(_)));
    }
}

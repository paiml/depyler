//! Regex method handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size. Contains the `convert_regex_method`
//! handler covering: findall, match, search, group, groups, start, end, span, as_str.

use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Handle regex methods (findall)
    #[inline]
    /// DEPYLER-0431: Convert regex instance method calls
    /// Handles both compiled Regex methods and Match object methods
    pub(super) fn convert_regex_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
            // Compiled Regex methods
            "findall" => {
                if arg_exprs.is_empty() {
                    bail!("findall() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.find_iter(#text)
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<String>>()
                })
            }

            // DEPYLER-0431: compiled.match(text) → compiled.find(text)
            // Python re.match() only matches at start, but Rust .find() searches anywhere
            // NOTE: Full .groups() support requires proper regex type tracking (DEPYLER-0563)
            "match" => {
                if arg_exprs.is_empty() {
                    bail!("match() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.find(#text) })
            }

            // compiled.search(text) → compiled.find(text)
            "search" => {
                if arg_exprs.is_empty() {
                    bail!("search() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.find(#text) })
            }

            // DEPYLER-0519: Match object methods - handle Option<Match> from .find() results
            // Python's re.match/find returns None or Match, Rust's .find() returns Option<Match>
            // We need to unwrap before calling Match methods like .start(), .as_str()

            // match.group(0) → match.as_str() (for group 0)
            // match.group(n) → match.get(n).map(|m| m.as_str()) (for other groups)
            "group" => {
                if arg_exprs.is_empty() {
                    // No args: default to group 0
                    // DEPYLER-0519: Use map for Option safety
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
                } else {
                    // Check if group_num is literal 0
                    if matches!(arg_exprs[0], syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref lit), .. }) if lit.base10_parse::<i32>().ok() == Some(0))
                    {
                        Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
                    } else {
                        // Non-zero group: needs captures API
                        bail!(
                            "match.group(n) for n>0 requires .captures() API (not yet implemented)"
                        )
                    }
                }
            }

            // match.groups() → extract all capture groups
            // DEPYLER-0442: Implement match.groups() using captured group extraction
            // Python: match.groups() returns tuple of all captured groups (excluding group 0)
            // NOTE: Full implementation requires regex type tracking (DEPYLER-0563)
            // For now, return empty vec - generator type system uses serde_json::Value as fallback
            "groups" => {
                // DEPYLER-0563: Implement proper capture group extraction when regex types are tracked
                Ok(parse_quote! {
                    Vec::<String>::new()
                })
            }

            // match.start() → match.start() (passthrough, with Option handling)
            "start" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.start()).unwrap_or(0) })
                } else {
                    bail!("match.start(group) with group number not yet implemented")
                }
            }

            // match.end() → match.end() (passthrough, with Option handling)
            "end" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.end()).unwrap_or(0) })
                } else {
                    bail!("match.end(group) with group number not yet implemented")
                }
            }

            // match.span() → (match.start(), match.end())
            "span" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(
                        parse_quote! { #object_expr.as_ref().map(|m| (m.start(), m.end())).unwrap_or((0, 0)) },
                    )
                } else {
                    bail!("match.span(group) with group number not yet implemented")
                }
            }

            // match.as_str() → match.as_str() (passthrough, with Option handling)
            "as_str" => {
                if !arg_exprs.is_empty() {
                    bail!("as_str() takes no arguments");
                }
                // DEPYLER-0519: Handle Option<Match>
                Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
            }

            _ => bail!("Unknown regex method: {}", method),
        }
    }
}

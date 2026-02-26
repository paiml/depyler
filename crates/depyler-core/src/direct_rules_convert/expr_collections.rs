//! Collection literal conversion and type predicates for ExprConverter
//!
//! Handles list, dict, tuple, set, frozenset literals and type checking helpers.

use crate::hir::*;
use anyhow::{bail, Result};
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_list(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> =
            elts.iter().map(|e| self.convert(e)).collect::<Result<Vec<_>>>()?;

        // DEPYLER-0780: Always use vec![] for list literals
        // Array literals [T; N] are incompatible with &Vec<T> parameters
        // Python lists map to Vec<T> in Rust, so consistently use vec![]
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    pub(super) fn convert_dict(&self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        // DEPYLER-1166: Check if dict has mixed value types and needs DepylerValue wrapping
        let has_mixed_types = self.dict_has_mixed_value_types(items);

        if has_mixed_types {
            // Use DepylerValue wrapping for mixed-type dicts
            // Use String keys since most Python dicts have string keys
            let nested_type = Type::Dict(Box::new(Type::String), Box::new(Type::Unknown));
            return self.convert_dict_to_depyler_value(
                items,
                &std::collections::HashMap::new(),
                &nested_type,
            );
        }

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

    /// DEPYLER-1166: Check if dict literal has mixed value types (requires DepylerValue)
    pub(super) fn dict_has_mixed_value_types(&self, items: &[(HirExpr, HirExpr)]) -> bool {
        if items.len() <= 1 {
            return false; // Single or empty dict, no mixing
        }

        // Quick check: if any value is a dict or list, it's potentially mixed
        let has_dict_or_list =
            items.iter().any(|(_, v)| matches!(v, HirExpr::Dict(_) | HirExpr::List(_)));

        if has_dict_or_list {
            return true;
        }

        // Check if literal types are mixed
        let value_type = |v: &HirExpr| -> u8 {
            match v {
                HirExpr::Literal(Literal::Int(_)) => 1,
                HirExpr::Literal(Literal::Float(_)) => 2,
                HirExpr::Literal(Literal::String(_)) => 3,
                HirExpr::Literal(Literal::Bool(_)) => 4,
                HirExpr::Literal(Literal::None) => 5,
                HirExpr::Var(_) => 6, // Variables have unknown type
                _ => 7,               // Other expressions
            }
        };

        let first_type = value_type(&items[0].1);

        // If any value has a different type, it's mixed
        // Variables (type 6) and other expressions (type 7) always count as different
        items.iter().any(|(_, v)| {
            let t = value_type(v);
            t != first_type || t >= 6
        })
    }

    /// DEPYLER-1122: Convert dict literal with DepylerValue wrapping
    /// Used when returning a dict from a method with bare `dict` return type,
    /// which maps to HashMap<DepylerValue, DepylerValue>
    /// DEPYLER-1166: Also handles Dict[str, Any] which has String keys
    pub(super) fn convert_dict_to_depyler_value(
        &self,
        items: &[(HirExpr, HirExpr)],
        class_field_types: &std::collections::HashMap<String, Type>,
        ret_type: &Type,
    ) -> Result<syn::Expr> {
        // DEPYLER-1166: Determine if keys should be String (Dict[str, Any]) or DepylerValue (bare dict)
        // DEPYLER-1213: Bare dict annotation stored as Custom("dict") should use String keys
        // (produces HashMap<String, DepylerValue>)
        let use_string_keys = match ret_type {
            // Dict[str, Any] or Dict[str, unknown] - use String keys
            Type::Dict(k, _) => matches!(k.as_ref(), Type::String | Type::Unknown),
            // Bare `dict` annotation - also use String keys (common case Dict[str, Any])
            Type::Custom(name) => name == "dict" || name == "Dict",
            _ => false,
        };

        let insert_exprs: Vec<syn::Expr> = items
            .iter()
            .map(|(k, v)| {
                let key_raw = self.convert(k)?;
                let val_raw = self.convert(v)?;

                // DEPYLER-1166: For Dict[str, Any], use raw string keys; for bare dict, wrap in DepylerValue
                let key: syn::Expr = if use_string_keys {
                    // Dict[str, Any] - use String keys directly
                    key_raw.clone()
                } else {
                    // Bare dict - wrap key in DepylerValue
                    match k {
                        HirExpr::Literal(Literal::Int(_)) => parse_quote! { DepylerValue::Int(#key_raw as i64) },
                        HirExpr::Literal(Literal::Float(_)) => parse_quote! { DepylerValue::Float(#key_raw as f64) },
                        HirExpr::Literal(Literal::String(_)) => parse_quote! { DepylerValue::Str(#key_raw.to_string()) },
                        HirExpr::Literal(Literal::Bool(_)) => parse_quote! { DepylerValue::Bool(#key_raw) },
                        _ => parse_quote! { DepylerValue::Str(format!("{:?}", #key_raw)) },
                    }
                };

                // Wrap value in DepylerValue based on its type
                // DEPYLER-99MODE-S9: String literals need .to_string() for DepylerValue::Str(String)
                let val: syn::Expr = match v {
                    HirExpr::Literal(Literal::Int(_)) => parse_quote! { DepylerValue::Int(#val_raw as i64) },
                    HirExpr::Literal(Literal::Float(_)) => parse_quote! { DepylerValue::Float(#val_raw as f64) },
                    HirExpr::Literal(Literal::String(_)) => parse_quote! { DepylerValue::Str(#val_raw.to_string()) },
                    HirExpr::Literal(Literal::Bool(_)) => parse_quote! { DepylerValue::Bool(#val_raw) },
                    HirExpr::Literal(Literal::None) => parse_quote! { DepylerValue::None },
                    HirExpr::Attribute { value, attr } => {
                        // self.field access - check field type
                        if let HirExpr::Var(name) = value.as_ref() {
                            if name == "self" {
                                if let Some(field_type) = class_field_types.get(attr) {
                                    return Ok(match field_type {
                                        Type::Int => parse_quote! { map.insert(#key, DepylerValue::Int(#val_raw as i64)) },
                                        Type::Float => parse_quote! { map.insert(#key, DepylerValue::Float(#val_raw as f64)) },
                                        Type::String => parse_quote! { map.insert(#key, DepylerValue::Str(#val_raw.to_string())) },
                                        Type::Bool => parse_quote! { map.insert(#key, DepylerValue::Bool(#val_raw)) },
                                        Type::List(_) => parse_quote! { map.insert(#key, DepylerValue::List(#val_raw.iter().map(|x| DepylerValue::from(x.clone())).collect())) },
                                        _ => parse_quote! { map.insert(#key, DepylerValue::Str(format!("{:?}", #val_raw))) },
                                    });
                                }
                            }
                        }
                        parse_quote! { DepylerValue::Str(format!("{:?}", #val_raw)) }
                    }
                    HirExpr::Var(name) => {
                        // Check if we know the variable type from class fields
                        if let Some(field_type) = class_field_types.get(name) {
                            match field_type {
                                Type::Int => parse_quote! { DepylerValue::Int(#val_raw as i64) },
                                Type::Float => parse_quote! { DepylerValue::Float(#val_raw as f64) },
                                Type::String => parse_quote! { DepylerValue::Str(#val_raw.to_string()) },
                                Type::Bool => parse_quote! { DepylerValue::Bool(#val_raw) },
                                _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_raw)) },
                            }
                        } else {
                            parse_quote! { DepylerValue::Str(format!("{:?}", #val_raw)) }
                        }
                    }
                    // DEPYLER-1166: Recursively handle nested Dict values
                    HirExpr::Dict(nested_items) => {
                        // Create a nested HashMap with DepylerValue keys and values
                        // For bare dict return types, inner dicts also have DepylerValue keys
                        let nested_type = Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown));
                        let nested_dict = self.convert_dict_to_depyler_value(nested_items, class_field_types, &nested_type)?;
                        // The recursive call already produces HashMap<DepylerValue, DepylerValue>, just wrap it
                        parse_quote! { DepylerValue::Dict(#nested_dict) }
                    }
                    // DEPYLER-1166: Handle nested List values
                    HirExpr::List(items) => {
                        // Convert each list element to DepylerValue
                        let list_elements: Vec<syn::Expr> = items
                            .iter()
                            .map(|item| {
                                let item_raw = self.convert(item)?;
                                let wrapped: syn::Expr = match item {
                                    HirExpr::Literal(Literal::Int(_)) => parse_quote! { DepylerValue::Int(#item_raw as i64) },
                                    HirExpr::Literal(Literal::Float(_)) => parse_quote! { DepylerValue::Float(#item_raw as f64) },
                                    HirExpr::Literal(Literal::String(_)) => parse_quote! { DepylerValue::Str(#item_raw.to_string()) },
                                    HirExpr::Literal(Literal::Bool(_)) => parse_quote! { DepylerValue::Bool(#item_raw) },
                                    HirExpr::Literal(Literal::None) => parse_quote! { DepylerValue::None },
                                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #item_raw)) },
                                };
                                Ok(wrapped)
                            })
                            .collect::<Result<Vec<_>>>()?;
                        parse_quote! { DepylerValue::List(vec![#(#list_elements),*]) }
                    }
                    _ => parse_quote! { DepylerValue::Str(format!("{:?}", #val_raw)) },
                };

                Ok(parse_quote! { map.insert(#key, #val) })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(parse_quote! {
            {
                let mut map = std::collections::HashMap::new();
                #(#insert_exprs;)*
                map
            }
        })
    }

    pub(super) fn convert_tuple(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> =
            elts.iter().map(|e| self.convert(e)).collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    pub(super) fn convert_set(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
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

    pub(super) fn convert_frozenset(&self, elts: &[HirExpr]) -> Result<syn::Expr> {
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

    pub(super) fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(name) => {
                // DEPYLER-99MODE-S9: Check param_types for set-typed variables
                // to correctly handle set subtraction (s1 - s2 → s1.difference(&s2))
                if let Some(var_type) = self.param_types.get(name) {
                    matches!(var_type, Type::Set(_))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0601: Detect if expression is likely a string type.
    /// Used to generate `.contains()` instead of `.contains_key()` for `in` operator.
    pub(super) fn is_string_expr(&self, expr: &HirExpr) -> bool {
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
                    "s" | "url"
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
    pub(super) fn is_deque_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call to deque() constructor
            HirExpr::Call { func, .. } if func == "deque" || func == "collections.deque" => true,
            // Variables with deque-like names
            HirExpr::Var(name) => {
                matches!(name.as_str(), "d" | "dq" | "deque" | "queue" | "buffer" | "deck")
            }
            _ => false,
        }
    }

    /// DEPYLER-0832: Detect if expression is a tuple (for `in` operator).
    /// Tuples should use `.contains()` on an array, not `.contains_key()`.
    pub(super) fn is_tuple_or_list_expr(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Tuple(_) | HirExpr::List(_))
    }

    /// DEPYLER-0960: Detect if expression is a dict/HashMap type.
    /// Used to ensure `key in dict` generates `.contains_key()` not `.contains()`.
    pub(super) fn is_dict_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Dict literal
            HirExpr::Dict { .. } => true,
            // Variables with common dict-like names or typed as dict
            HirExpr::Var(name) => {
                // DEPYLER-99MODE-S9: Check if variable is explicitly typed as a set FIRST
                // This prevents set variables named "visited", "seen", etc. from being
                // misidentified as dicts by the name heuristic below
                if let Some(t) = self.param_types.get(name) {
                    if matches!(t, Type::Set(_)) {
                        return false;
                    }
                    if matches!(t, Type::Dict(_, _)) {
                        return true;
                    }
                }
                if let Some(t) = self.class_field_types.get(name) {
                    if matches!(t, Type::Set(_)) {
                        return false;
                    }
                    if matches!(t, Type::Dict(_, _)) {
                        return true;
                    }
                }
                let n = name.as_str();
                n.contains("dict")
                    || n.contains("map")
                    || n.contains("hash")
                    || n == "config"
                    || n == "settings"
                    || n == "params"
                    || n == "options"
                    || n == "env"
                    || n == "data"
                    || n == "result"
                    || n == "cache"
                    || n == "d"
                    || n == "m"
                    // DEPYLER-99MODE: Common algorithm dict names
                    || n == "memo"
                    || n == "counts"
                    || n == "freq"
                    || n == "frequency"
                    || n == "lookup"
                    || n == "index"
                    || n == "graph"
                    || n == "adj"
                    || n == "dp"
                    || n.ends_with("_map")
                    || n.ends_with("_dict")
                    || n.ends_with("_cache")
                    || n.ends_with("_index")
                    || n.ends_with("_lookup")
            }
            // Calls to dict() or functions returning dicts
            HirExpr::Call { func, .. } => {
                func == "dict"
                    || func.contains("json")
                    || func.contains("config")
                    || func.contains("load")
                    || func.contains("parse")
            }
            _ => false,
        }
    }

    pub(super) fn convert_set_operation(
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
}

//! DEPYLER-1115: Phantom Structure Bindings
//!
//! Generates Rust "phantom" struct wrappers for external Python library types.
//! When Python code uses `requests.get()` returning `Response`, this module
//! generates corresponding Rust struct definitions to satisfy E0412 (Type Not Found).
//!
//! The "Phantom" pattern wraps external types around `serde_json::Value` without
//! attempting to replicate internal layouts.
//!
//! This module is only active when the `sovereign-types` feature is enabled.

// Note: This module is feature-gated at the module declaration in rust_gen.rs

use crate::hir::{HirExpr, HirModule, HirStmt};
use anyhow::Result;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};

use depyler_knowledge::{TypeFact, TypeQuery};

/// Collected external symbols from HIR analysis
///
/// Note: Some fields are reserved for future use in more sophisticated binding generation.
#[derive(Debug, Default)]
#[allow(dead_code)] // Fields reserved for future expansion
pub struct UsedExternalSymbols {
    /// Module imports used (e.g., "requests")
    pub modules: HashSet<String>,
    /// Function calls on modules (e.g., ("requests", "get"))
    pub module_functions: HashSet<(String, String)>,
    /// Types referenced (e.g., "Response" from requests.models)
    pub referenced_types: HashSet<String>,
    /// Method calls on external types (e.g., ("Response", "json"))
    pub type_methods: HashSet<(String, String)>,
}

/// Parsed method signature from TypeFact
#[derive(Debug, Clone)]
pub struct ParsedSignature {
    /// Parameters as (name, type) pairs
    pub params: Vec<(String, String)>,
    /// Return type string
    pub return_type: String,
    /// Whether this is an instance method (has self)
    pub is_self_method: bool,
}

/// Parse a Python signature string into structured form
///
/// Examples:
/// - `"(self, x: int) -> Response"` -> params=[("x", "int")], return_type="Response", is_self=true
/// - `"(url: str, **kwargs) -> Response"` -> params=[("url", "str")], return_type="Response", is_self=false
pub fn parse_signature(signature: &str) -> ParsedSignature {
    let trimmed = signature.trim();

    // Split on "->" to get params and return type
    let (params_str, return_type) = if let Some(idx) = trimmed.rfind("->") {
        let params = trimmed[..idx].trim();
        let ret = trimmed[idx + 2..].trim().to_string();
        (params, ret)
    } else {
        (trimmed, "()".to_string())
    };

    // Remove outer parentheses
    let params_str = params_str
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim();

    let mut params = Vec::new();
    let mut is_self_method = false;

    // Handle empty params
    if params_str.is_empty() {
        return ParsedSignature {
            params,
            return_type,
            is_self_method,
        };
    }

    // Parse each parameter, handling nested brackets
    for param in split_params(params_str) {
        let param = param.trim();

        // Skip empty params
        if param.is_empty() {
            continue;
        }

        // Handle self
        if param == "self" {
            is_self_method = true;
            continue;
        }

        // Skip *args and **kwargs
        if param.starts_with('*') {
            continue;
        }

        // Handle "name: type" format
        if let Some((name, ty)) = param.split_once(':') {
            let name = name.trim().to_string();
            let ty = ty.trim().to_string();
            // Skip default values (e.g., "x: int = 0" -> "x: int")
            let ty = if let Some(idx) = ty.find('=') {
                ty[..idx].trim().to_string()
            } else {
                ty
            };
            params.push((name, ty));
        }
        // Handle positional-only param without type annotation
        else if !param.contains('=') {
            params.push((param.to_string(), "Any".to_string()));
        }
    }

    ParsedSignature {
        params,
        return_type,
        is_self_method,
    }
}

/// Split parameters respecting nested brackets
fn split_params(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '[' | '(' | '{' => depth += 1,
            ']' | ')' | '}' => depth -= 1,
            ',' if depth == 0 => {
                result.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }

    // Don't forget the last segment
    if start < s.len() {
        result.push(&s[start..]);
    }

    result
}

/// Map Python type string to Rust TokenStream
pub fn python_type_to_rust(py_type: &str) -> TokenStream {
    let py_type = py_type.trim();

    match py_type {
        // Primitives
        "int" => quote! { i64 },
        "float" => quote! { f64 },
        "str" => quote! { String },
        "bool" => quote! { bool },
        "bytes" => quote! { Vec<u8> },
        "None" | "()" | "" => quote! { () },

        // Collections without generics
        "dict" => quote! { std::collections::HashMap<String, serde_json::Value> },
        "list" => quote! { Vec<serde_json::Value> },
        "tuple" => quote! { Vec<serde_json::Value> },
        "set" => quote! { std::collections::HashSet<serde_json::Value> },

        // Handle generics
        s if s.starts_with("List[") || s.starts_with("list[") => {
            let inner = extract_generic_param(s);
            let inner_ty = python_type_to_rust(inner);
            quote! { Vec<#inner_ty> }
        }
        s if s.starts_with("Dict[") || s.starts_with("dict[") => {
            let (key, val) = extract_dict_params(s);
            let key_ty = python_type_to_rust(key);
            let val_ty = python_type_to_rust(val);
            quote! { std::collections::HashMap<#key_ty, #val_ty> }
        }
        s if s.starts_with("Optional[") => {
            let inner = extract_generic_param(s);
            let inner_ty = python_type_to_rust(inner);
            quote! { Option<#inner_ty> }
        }
        s if s.starts_with("Set[") || s.starts_with("set[") => {
            let inner = extract_generic_param(s);
            let inner_ty = python_type_to_rust(inner);
            quote! { std::collections::HashSet<#inner_ty> }
        }
        s if s.starts_with("Tuple[") || s.starts_with("tuple[") => {
            // For simplicity, map to Vec (proper tuple handling is complex)
            quote! { Vec<serde_json::Value> }
        }

        // Any and unknown types - use serde_json::Value as universal fallback
        "Any" | "object" => quote! { serde_json::Value },
        // Unknown types fall back to serde_json::Value
        _ => quote! { serde_json::Value },
    }
}

/// Extract the type parameter from a generic like "List[int]" -> "int"
fn extract_generic_param(s: &str) -> &str {
    let start = s.find('[').map(|i| i + 1).unwrap_or(0);
    let end = s.rfind(']').unwrap_or(s.len());
    s[start..end].trim()
}

/// Extract key and value types from "Dict[str, int]" -> ("str", "int")
fn extract_dict_params(s: &str) -> (&str, &str) {
    let inner = extract_generic_param(s);
    // Split on first comma, respecting nested brackets
    let mut depth = 0;
    for (i, c) in inner.char_indices() {
        match c {
            '[' | '(' | '{' => depth += 1,
            ']' | ')' | '}' => depth -= 1,
            ',' if depth == 0 => {
                return (inner[..i].trim(), inner[i + 1..].trim());
            }
            _ => {}
        }
    }
    // Fallback if no comma found
    (inner, "Any")
}

/// Generate nested module hierarchy from dotted path
///
/// Example: "requests.models" with struct Response generates:
/// ```ignore
/// pub mod requests {
///     pub mod models {
///         pub struct Response(...);
///     }
/// }
/// ```
pub fn generate_module_hierarchy(path: &str, content: TokenStream) -> TokenStream {
    if path.is_empty() {
        return content;
    }

    let parts: Vec<&str> = path.split('.').collect();

    // Build from innermost to outermost
    let mut result = content;
    for part in parts.iter().rev() {
        // Sanitize module name (handle keywords)
        let mod_name = sanitize_identifier(part);
        let mod_ident = syn::Ident::new(&mod_name, Span::call_site());
        result = quote! {
            pub mod #mod_ident {
                #result
            }
        };
    }
    result
}

/// Sanitize an identifier to be a valid Rust identifier
fn sanitize_identifier(name: &str) -> String {
    // Handle Rust keywords
    match name {
        "type" => "r#type".to_string(),
        "match" => "r#match".to_string(),
        "async" => "r#async".to_string(),
        "await" => "r#await".to_string(),
        "yield" => "r#yield".to_string(),
        "move" => "r#move".to_string(),
        "ref" => "r#ref".to_string(),
        "self" => "self_".to_string(),
        "Self" => "Self_".to_string(),
        _ => name.to_string(),
    }
}

/// Generate a method stub from a parsed signature
fn generate_method_stub(method_name: &str, signature: &ParsedSignature) -> TokenStream {
    let method_ident = syn::Ident::new(&sanitize_identifier(method_name), Span::call_site());
    let return_ty = python_type_to_rust(&signature.return_type);

    // Generate parameter list (skip self)
    let params: Vec<TokenStream> = signature
        .params
        .iter()
        .map(|(name, ty)| {
            let param_ident = syn::Ident::new(&sanitize_identifier(name), Span::call_site());
            let param_ty = python_type_to_rust(ty);
            quote! { #param_ident: #param_ty }
        })
        .collect();

    // Generate underscored params for unused warning suppression
    let underscore_params: Vec<TokenStream> = signature
        .params
        .iter()
        .map(|(name, _)| {
            let param_ident = syn::Ident::new(&sanitize_identifier(name), Span::call_site());
            quote! { let _ = #param_ident; }
        })
        .collect();

    let method_name_str = method_name;

    if signature.is_self_method {
        quote! {
            pub fn #method_ident(&self #(, #params)*) -> #return_ty {
                let _ = &self.0;
                #(#underscore_params)*
                todo!(concat!("Generated stub for ", #method_name_str))
            }
        }
    } else {
        quote! {
            pub fn #method_ident(#(#params),*) -> #return_ty {
                #(#underscore_params)*
                todo!(concat!("Generated stub for ", #method_name_str))
            }
        }
    }
}

/// Generate a phantom struct with method stubs
#[cfg(feature = "sovereign-types")]
pub fn generate_phantom_struct(class_name: &str, methods: &[TypeFact]) -> TokenStream {
    let struct_ident = syn::Ident::new(&sanitize_identifier(class_name), Span::call_site());

    // Generate method implementations
    let method_impls: Vec<TokenStream> = methods
        .iter()
        .filter_map(|method| {
            // Extract method name from "ClassName.method_name" format
            let parts: Vec<&str> = method.symbol.splitn(2, '.').collect();
            let method_name = parts.get(1)?;

            let sig = parse_signature(&method.signature);
            Some(generate_method_stub(method_name, &sig))
        })
        .collect();

    quote! {
        /// Phantom wrapper for external library type
        /// Generated by DEPYLER-1115 from Sovereign Type Database
        #[derive(Debug, Clone)]
        pub struct #struct_ident(pub serde_json::Value);

        impl #struct_ident {
            /// Create a new instance wrapping a serde_json::Value
            pub fn new(inner: serde_json::Value) -> Self {
                Self(inner)
            }

            /// Get a reference to the inner Value
            pub fn inner(&self) -> &serde_json::Value {
                &self.0
            }

            #(#method_impls)*
        }
    }
}

/// Main binding generator
#[cfg(feature = "sovereign-types")]
pub struct BindingGenerator<'a> {
    type_query: &'a mut TypeQuery,
    used_symbols: UsedExternalSymbols,
    /// Track which types we've already generated to avoid duplicates
    generated_types: HashSet<String>,
}

#[cfg(feature = "sovereign-types")]
impl<'a> BindingGenerator<'a> {
    /// Create a new binding generator
    pub fn new(type_query: &'a mut TypeQuery) -> Self {
        Self {
            type_query,
            used_symbols: UsedExternalSymbols::default(),
            generated_types: HashSet::new(),
        }
    }

    /// Collect all used external symbols from the HIR module
    pub fn collect_symbols(&mut self, module: &HirModule) {
        // Collect imports first
        for import in &module.imports {
            self.used_symbols.modules.insert(import.module.clone());
        }

        // Walk all functions
        for func in &module.functions {
            for stmt in &func.body {
                self.collect_from_stmt(stmt);
            }
        }

        // Walk all classes
        for class in &module.classes {
            for method in &class.methods {
                for stmt in &method.body {
                    self.collect_from_stmt(stmt);
                }
            }
        }
    }

    fn collect_from_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { value, .. } => self.collect_from_expr(value),
            HirStmt::Return(value) => {
                if let Some(expr) = value {
                    self.collect_from_expr(expr);
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_from_expr(condition);
                for s in then_body {
                    self.collect_from_stmt(s);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_from_stmt(s);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.collect_from_expr(condition);
                for s in body {
                    self.collect_from_stmt(s);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.collect_from_expr(iter);
                for s in body {
                    self.collect_from_stmt(s);
                }
            }
            HirStmt::With { context, body, .. } => {
                self.collect_from_expr(context);
                for s in body {
                    self.collect_from_stmt(s);
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for s in body {
                    self.collect_from_stmt(s);
                }
                for handler in handlers {
                    for s in &handler.body {
                        self.collect_from_stmt(s);
                    }
                }
                if let Some(else_stmts) = orelse {
                    for s in else_stmts {
                        self.collect_from_stmt(s);
                    }
                }
                if let Some(final_stmts) = finalbody {
                    for s in final_stmts {
                        self.collect_from_stmt(s);
                    }
                }
            }
            HirStmt::Expr(expr) => self.collect_from_expr(expr),
            HirStmt::Assert { test, msg } => {
                self.collect_from_expr(test);
                if let Some(m) = msg {
                    self.collect_from_expr(m);
                }
            }
            HirStmt::Raise { exception, cause } => {
                if let Some(e) = exception {
                    self.collect_from_expr(e);
                }
                if let Some(c) = cause {
                    self.collect_from_expr(c);
                }
            }
            HirStmt::Block(stmts) => {
                for s in stmts {
                    self.collect_from_stmt(s);
                }
            }
            HirStmt::FunctionDef { body, .. } => {
                for s in body {
                    self.collect_from_stmt(s);
                }
            }
            // Terminal statements - no expressions to collect
            HirStmt::Break { .. } | HirStmt::Continue { .. } | HirStmt::Pass => {}
        }
    }

    fn collect_from_expr(&mut self, expr: &HirExpr) {
        match expr {
            // Pattern: requests.get(url) -> MethodCall on module variable
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => {
                // Check if object is a module reference
                if let HirExpr::Var(module_name) = object.as_ref() {
                    if self.used_symbols.modules.contains(module_name) {
                        self.used_symbols
                            .module_functions
                            .insert((module_name.clone(), method.clone()));
                    }
                }
                // Recurse
                self.collect_from_expr(object);
                for arg in args {
                    self.collect_from_expr(arg);
                }
                for (_, arg) in kwargs {
                    self.collect_from_expr(arg);
                }
            }

            // Pattern: response.attr -> Attribute access
            HirExpr::Attribute { value, .. } => {
                self.collect_from_expr(value);
            }

            // Call has func as Symbol (String), not expression
            HirExpr::Call { args, kwargs, .. } => {
                for arg in args {
                    self.collect_from_expr(arg);
                }
                for (_, arg) in kwargs {
                    self.collect_from_expr(arg);
                }
            }

            // DynamicCall has callee as expression
            HirExpr::DynamicCall {
                callee,
                args,
                kwargs,
            } => {
                self.collect_from_expr(callee);
                for arg in args {
                    self.collect_from_expr(arg);
                }
                for (_, arg) in kwargs {
                    self.collect_from_expr(arg);
                }
            }

            HirExpr::Binary { left, right, .. } => {
                self.collect_from_expr(left);
                self.collect_from_expr(right);
            }

            HirExpr::Unary { operand, .. } => {
                self.collect_from_expr(operand);
            }

            HirExpr::IfExpr { test, body, orelse } => {
                self.collect_from_expr(test);
                self.collect_from_expr(body);
                self.collect_from_expr(orelse);
            }

            HirExpr::List(items)
            | HirExpr::Tuple(items)
            | HirExpr::Set(items)
            | HirExpr::FrozenSet(items) => {
                for item in items {
                    self.collect_from_expr(item);
                }
            }

            // Dict is Vec<(key, value)>
            HirExpr::Dict(pairs) => {
                for (key, val) in pairs {
                    self.collect_from_expr(key);
                    self.collect_from_expr(val);
                }
            }

            HirExpr::Index { base, index } => {
                self.collect_from_expr(base);
                self.collect_from_expr(index);
            }

            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                self.collect_from_expr(base);
                if let Some(s) = start {
                    self.collect_from_expr(s);
                }
                if let Some(s) = stop {
                    self.collect_from_expr(s);
                }
                if let Some(s) = step {
                    self.collect_from_expr(s);
                }
            }

            HirExpr::Lambda { body, .. } => {
                self.collect_from_expr(body);
            }

            HirExpr::ListComp {
                element,
                generators,
            }
            | HirExpr::SetComp {
                element,
                generators,
            } => {
                self.collect_from_expr(element);
                for gen in generators {
                    self.collect_from_expr(&gen.iter);
                    for cond in &gen.conditions {
                        self.collect_from_expr(cond);
                    }
                }
            }

            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                self.collect_from_expr(key);
                self.collect_from_expr(value);
                for gen in generators {
                    self.collect_from_expr(&gen.iter);
                    for cond in &gen.conditions {
                        self.collect_from_expr(cond);
                    }
                }
            }

            HirExpr::GeneratorExp {
                element,
                generators,
            } => {
                self.collect_from_expr(element);
                for gen in generators {
                    self.collect_from_expr(&gen.iter);
                    for cond in &gen.conditions {
                        self.collect_from_expr(cond);
                    }
                }
            }

            HirExpr::Await { value } => {
                self.collect_from_expr(value);
            }

            HirExpr::Yield { value } => {
                if let Some(v) = value {
                    self.collect_from_expr(v);
                }
            }

            HirExpr::Borrow { expr, .. } => {
                self.collect_from_expr(expr);
            }

            HirExpr::SortByKey {
                iterable,
                key_body,
                reverse_expr,
                ..
            } => {
                self.collect_from_expr(iterable);
                self.collect_from_expr(key_body);
                if let Some(r) = reverse_expr {
                    self.collect_from_expr(r);
                }
            }

            HirExpr::FString { parts } => {
                for part in parts {
                    if let crate::hir::FStringPart::Expr(e) = part {
                        self.collect_from_expr(e);
                    }
                }
            }

            HirExpr::NamedExpr { value, .. } => {
                self.collect_from_expr(value);
            }

            // Terminal expressions - no recursion needed
            HirExpr::Var(_) | HirExpr::Literal(_) => {}
        }
    }

    /// Generate phantom bindings for all collected external symbols
    pub fn generate_bindings(&mut self) -> Result<TokenStream> {
        // Warm the cache for efficient lookups
        let _ = self.type_query.warm_cache();

        let mut all_bindings: HashMap<String, Vec<TokenStream>> = HashMap::new();

        // For each module function call, look up return types and generate structs
        for (module, function) in &self.used_symbols.module_functions.clone() {
            // Query TypeDB for return type of this function
            if let Ok(return_type) = self.type_query.find_return_type(module, function) {
                // Parse the return type to get the class name
                // e.g., "requests.models.Response" or just "Response"
                let class_name = return_type.split('.').next_back().unwrap_or(&return_type);

                // Skip if we've already generated this type
                let full_type_name = format!("{}.{}", module, class_name);
                if self.generated_types.contains(&full_type_name) {
                    continue;
                }
                self.generated_types.insert(full_type_name.clone());

                // Find the module containing this class
                // Try common patterns: module.models.ClassName, module.ClassName
                let module_patterns = vec![
                    format!("{}.models", module),
                    format!("{}.api", module),
                    module.clone(),
                ];

                for mod_prefix in &module_patterns {
                    // Look for class methods
                    if let Ok(methods) = self.type_query.find_methods(mod_prefix, class_name) {
                        if !methods.is_empty() {
                            let struct_tokens = generate_phantom_struct(class_name, &methods);
                            all_bindings
                                .entry(mod_prefix.clone())
                                .or_default()
                                .push(struct_tokens);
                            break;
                        }
                    }
                }

                // If no methods found, still generate a basic struct
                if !all_bindings.values().any(|v| !v.is_empty()) {
                    let struct_tokens = generate_phantom_struct(class_name, &[]);
                    all_bindings
                        .entry(module.clone())
                        .or_default()
                        .push(struct_tokens);
                }
            }
        }

        // If no bindings were generated, return empty
        if all_bindings.is_empty() {
            return Ok(quote! {});
        }

        // Combine all bindings into module hierarchy
        let mut final_tokens = Vec::new();

        for (module_path, structs) in all_bindings {
            let combined_structs = quote! { #(#structs)* };
            let module_tokens = generate_module_hierarchy(&module_path, combined_structs);
            final_tokens.push(module_tokens);
        }

        Ok(quote! {
            // DEPYLER-1115: Phantom bindings for external library types
            #(#final_tokens)*
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_signature() {
        let sig = parse_signature("(self, x: int) -> Response");
        assert!(sig.is_self_method);
        assert_eq!(sig.params.len(), 1);
        assert_eq!(sig.params[0].0, "x");
        assert_eq!(sig.params[0].1, "int");
        assert_eq!(sig.return_type, "Response");
    }

    #[test]
    fn test_parse_kwargs_signature() {
        let sig = parse_signature("(url: str, **kwargs) -> Response");
        assert!(!sig.is_self_method);
        assert_eq!(sig.params.len(), 1);
        assert_eq!(sig.params[0].0, "url");
        assert_eq!(sig.params[0].1, "str");
    }

    #[test]
    fn test_parse_no_params() {
        let sig = parse_signature("() -> None");
        assert!(!sig.is_self_method);
        assert!(sig.params.is_empty());
        assert_eq!(sig.return_type, "None");
    }

    #[test]
    fn test_parse_self_only() {
        let sig = parse_signature("(self) -> dict");
        assert!(sig.is_self_method);
        assert!(sig.params.is_empty());
        assert_eq!(sig.return_type, "dict");
    }

    #[test]
    fn test_parse_default_values() {
        let sig = parse_signature("(x: int = 0, y: str = 'hello') -> bool");
        assert!(!sig.is_self_method);
        assert_eq!(sig.params.len(), 2);
        assert_eq!(sig.params[0], ("x".to_string(), "int".to_string()));
        assert_eq!(sig.params[1], ("y".to_string(), "str".to_string()));
    }

    #[test]
    fn test_parse_generic_types() {
        let sig = parse_signature("(items: List[int]) -> Dict[str, int]");
        assert!(!sig.is_self_method);
        assert_eq!(sig.params.len(), 1);
        assert_eq!(sig.params[0].1, "List[int]");
        assert_eq!(sig.return_type, "Dict[str, int]");
    }

    #[test]
    fn test_python_type_to_rust_primitives() {
        assert_eq!(python_type_to_rust("int").to_string(), "i64");
        assert_eq!(python_type_to_rust("float").to_string(), "f64");
        assert_eq!(python_type_to_rust("str").to_string(), "String");
        assert_eq!(python_type_to_rust("bool").to_string(), "bool");
    }

    #[test]
    fn test_python_type_to_rust_collections() {
        let dict = python_type_to_rust("dict").to_string();
        assert!(dict.contains("HashMap"));

        let list = python_type_to_rust("list").to_string();
        assert!(list.contains("Vec"));
    }

    #[test]
    fn test_python_type_to_rust_generics() {
        let list_int = python_type_to_rust("List[int]").to_string();
        assert!(list_int.contains("Vec"));
        assert!(list_int.contains("i64"));

        let opt_str = python_type_to_rust("Optional[str]").to_string();
        assert!(opt_str.contains("Option"));
        assert!(opt_str.contains("String"));
    }

    #[test]
    fn test_module_hierarchy() {
        let content = quote! { pub struct Response; };
        let result = generate_module_hierarchy("requests.models", content);
        let code = result.to_string();

        assert!(code.contains("pub mod requests"));
        assert!(code.contains("pub mod models"));
        assert!(code.contains("pub struct Response"));
    }

    #[test]
    fn test_module_hierarchy_single() {
        let content = quote! { pub struct Client; };
        let result = generate_module_hierarchy("httpx", content);
        let code = result.to_string();

        assert!(code.contains("pub mod httpx"));
        assert!(code.contains("pub struct Client"));
    }

    #[test]
    fn test_module_hierarchy_empty_path() {
        let content = quote! { pub struct Foo; };
        let result = generate_module_hierarchy("", content);
        let code = result.to_string();

        assert!(code.contains("pub struct Foo"));
        assert!(!code.contains("pub mod"));
    }

    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("type"), "r#type");
        assert_eq!(sanitize_identifier("match"), "r#match");
        assert_eq!(sanitize_identifier("normal"), "normal");
    }

    #[test]
    fn test_split_params_nested() {
        let params = split_params("x: Dict[str, int], y: List[int]");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].trim(), "x: Dict[str, int]");
        assert_eq!(params[1].trim(), "y: List[int]");
    }

    #[test]
    fn test_extract_generic_param() {
        assert_eq!(extract_generic_param("List[int]"), "int");
        assert_eq!(extract_generic_param("Optional[str]"), "str");
        assert_eq!(extract_generic_param("Dict[str, int]"), "str, int");
    }

    #[test]
    fn test_extract_dict_params() {
        let (key, val) = extract_dict_params("Dict[str, int]");
        assert_eq!(key, "str");
        assert_eq!(val, "int");

        let (key2, val2) = extract_dict_params("Dict[str, List[int]]");
        assert_eq!(key2, "str");
        assert_eq!(val2, "List[int]");
    }
}

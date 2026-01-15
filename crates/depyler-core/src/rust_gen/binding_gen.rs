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
            // DEPYLER-1115: Skip parameters with default values - they are optional in Python
            // For phantom bindings, only include required (non-default) parameters
            if ty.contains('=') {
                continue;
            }
            params.push((name, ty));
        }
        // Handle positional-only param without type annotation
        // Skip if it has a default value
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
///
/// DEPYLER-1115: For phantom bindings, use simplified types for NASA mode compatibility
/// Complex types like dict use HashMap<String, String> to avoid scope issues with DepylerValue
pub fn python_type_to_rust(py_type: &str) -> TokenStream {
    let py_type = py_type.trim();

    match py_type {
        // Primitives
        "int" => quote! { i64 },
        "float" => quote! { f64 },
        // DEPYLER-1115: Use &str for parameters to avoid .to_string() at call sites
        "str" => quote! { &str },
        "bool" => quote! { bool },
        "bytes" => quote! { Vec<u8> },
        "None" | "()" | "" => quote! { () },

        // Collections without generics - use String for NASA mode compatibility
        // DEPYLER-1115: Simplified types avoid scope issues with DepylerValue in nested modules
        "dict" => quote! { std::collections::HashMap<String, String> },
        "list" => quote! { Vec<String> },
        "tuple" => quote! { Vec<String> },
        "set" => quote! { std::collections::HashSet<String> },

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
            quote! { Vec<String> }
        }

        // Any and unknown types - use &str for parameter ergonomics
        // DEPYLER-1115: Phantom binding parameters should accept &str since that's what
        // the transpiler generates for user code (e.g., fn fetch(url: &str))
        "Any" | "object" => quote! { &str },
        // Unknown types fall back to &str for ergonomic call sites
        _ => quote! { &str },
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

/// DEPYLER-1116: Generate a default return value based on Python type
///
/// This implements the proxy pattern for phantom bindings - instead of panicking
/// with todo!(), return sensible default values so the code is actually runnable.
fn generate_default_return(py_type: &str) -> TokenStream {
    let py_type = py_type.trim();
    match py_type {
        // Primitives
        "int" => quote! { 0i64 },
        "float" => quote! { 0.0f64 },
        "str" => quote! { "" },
        "bool" => quote! { false },
        "bytes" => quote! { Vec::new() },
        "None" | "()" | "" => quote! { () },

        // Collections - return empty collections
        "dict" => quote! { std::collections::HashMap::new() },
        "list" => quote! { Vec::new() },
        "tuple" => quote! { Vec::new() },
        "set" => quote! { std::collections::HashSet::new() },

        // Generic collections
        s if s.starts_with("List[") || s.starts_with("list[") => quote! { Vec::new() },
        s if s.starts_with("Dict[") || s.starts_with("dict[") => {
            quote! { std::collections::HashMap::new() }
        }
        s if s.starts_with("Set[") || s.starts_with("set[") => {
            quote! { std::collections::HashSet::new() }
        }
        s if s.starts_with("Optional[") => quote! { None },
        s if s.starts_with("Tuple[") || s.starts_with("tuple[") => quote! { Vec::new() },

        // Any and unknown - return empty string (most common case)
        "Any" | "object" => quote! { "" },
        _ => quote! { "" },
    }
}

/// Generate a method stub from a parsed signature
///
/// DEPYLER-1116: Uses proxy pattern - returns default values instead of todo!()
/// This makes generated code actually runnable, not just compilable.
fn generate_method_stub(method_name: &str, signature: &ParsedSignature) -> TokenStream {
    let method_ident = syn::Ident::new(&sanitize_identifier(method_name), Span::call_site());
    let return_ty = python_type_to_rust(&signature.return_type);
    let default_return = generate_default_return(&signature.return_type);

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

    if signature.is_self_method {
        quote! {
            pub fn #method_ident(&self #(, #params)*) -> #return_ty {
                let _ = &self.0;
                #(#underscore_params)*
                #default_return
            }
        }
    } else {
        quote! {
            pub fn #method_ident(#(#params),*) -> #return_ty {
                #(#underscore_params)*
                #default_return
            }
        }
    }
}

/// Generate a module-level function stub that returns a phantom type
///
/// Example: `requests.get(url)` -> `pub fn get(url: &str) -> models::Response { ... }`
///
/// DEPYLER-1116: Uses proxy pattern - constructs phantom struct with default inner value
/// This makes generated code actually runnable, not just compilable.
#[cfg(feature = "sovereign-types")]
pub fn generate_module_function(
    function_name: &str,
    signature: &ParsedSignature,
    return_type_path: &str,
) -> TokenStream {
    let func_ident = syn::Ident::new(&sanitize_identifier(function_name), Span::call_site());

    // Generate parameter list
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

    // Parse return type path into tokens (e.g., "models::Response")
    let return_type: TokenStream = return_type_path.parse().unwrap_or_else(|_| quote! { String });

    // DEPYLER-1116: Construct the return type using its ::new() constructor
    // Phantom structs wrap String, so we create an instance with empty string
    let return_construct: TokenStream = format!("{}::new(String::new())", return_type_path)
        .parse()
        .unwrap_or_else(|_| quote! { String::new() });

    quote! {
        /// Generated stub for module function
        pub fn #func_ident(#(#params),*) -> #return_type {
            #(#underscore_params)*
            #return_construct
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

    // DEPYLER-1115: Use String wrapper for NASA mode compatibility
    // Note: In NASA mode, complex types are simplified to String for single-shot compile
    // This avoids dependency on DepylerValue which may be at a different scope
    quote! {
        /// Phantom wrapper for external library type
        /// Generated by DEPYLER-1115 from Sovereign Type Database
        #[derive(Debug, Clone)]
        pub struct #struct_ident(pub String);

        impl #struct_ident {
            /// Create a new instance wrapping a String
            pub fn new(inner: String) -> Self {
                Self(inner)
            }

            /// Get a reference to the inner Value
            pub fn inner(&self) -> &String {
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

        // Track structs by their containing module (e.g., "requests.models")
        let mut struct_bindings: HashMap<String, Vec<TokenStream>> = HashMap::new();
        // Track module-level functions (e.g., "requests" -> [get, post, ...])
        let mut module_functions: HashMap<String, Vec<TokenStream>> = HashMap::new();

        // For each module function call, look up signatures and generate bindings
        for (module, function) in &self.used_symbols.module_functions.clone() {
            // Query TypeDB for return type and signature of this function
            let return_type_result = self.type_query.find_return_type(module, function);
            let signature_result = self.type_query.find_signature(module, function);

            if let Ok(return_type) = return_type_result {
                // Parse the return type to get the class name
                // e.g., "requests.models.Response" or just "Response"
                let class_name = return_type.split('.').next_back().unwrap_or(&return_type);

                // Generate the module function that returns the phantom type
                // Return type path relative to module root: "models::Response"
                let return_type_path = format!("models::{}", class_name);

                if let Ok(sig_str) = signature_result {
                    let sig = parse_signature(&sig_str);
                    let func_tokens = generate_module_function(function, &sig, &return_type_path);
                    module_functions
                        .entry(module.clone())
                        .or_default()
                        .push(func_tokens);
                }

                // Skip struct generation if we've already generated this type
                let full_type_name = format!("{}.{}", module, class_name);
                if self.generated_types.contains(&full_type_name) {
                    continue;
                }
                self.generated_types.insert(full_type_name.clone());

                // Find the module containing this class and generate struct
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
                            struct_bindings
                                .entry(mod_prefix.clone())
                                .or_default()
                                .push(struct_tokens);
                            break;
                        }
                    }
                }

                // If no methods found, still generate a basic struct
                if !struct_bindings.values().any(|v| !v.is_empty()) {
                    let struct_tokens = generate_phantom_struct(class_name, &[]);
                    struct_bindings
                        .entry(format!("{}.models", module))
                        .or_default()
                        .push(struct_tokens);
                }
            }
        }

        // If no bindings were generated, return empty
        if struct_bindings.is_empty() && module_functions.is_empty() {
            return Ok(quote! {});
        }

        // Build final module structure
        let mut final_tokens = Vec::new();

        // Group all bindings by top-level module
        let mut module_contents: HashMap<String, (Vec<TokenStream>, Vec<TokenStream>)> =
            HashMap::new();

        // Add struct bindings (nested in submodules like "models")
        for (module_path, structs) in struct_bindings {
            let parts: Vec<&str> = module_path.split('.').collect();
            let top_module = parts[0].to_string();
            let subpath = parts[1..].join(".");

            let combined_structs = quote! { #(#structs)* };
            let nested = if !subpath.is_empty() {
                generate_module_hierarchy(&subpath, combined_structs)
            } else {
                combined_structs
            };

            module_contents
                .entry(top_module)
                .or_default()
                .0
                .push(nested);
        }

        // Add module-level functions
        for (module, funcs) in module_functions {
            module_contents.entry(module).or_default().1.extend(funcs);
        }

        // Generate final module hierarchy
        for (top_module, (struct_content, func_content)) in module_contents {
            let mod_ident = syn::Ident::new(&sanitize_identifier(&top_module), Span::call_site());
            let structs = quote! { #(#struct_content)* };
            let funcs = quote! { #(#func_content)* };

            final_tokens.push(quote! {
                pub mod #mod_ident {
                    #structs
                    #funcs
                }
            });
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
        // DEPYLER-1115: Parameters with default values are SKIPPED for phantom bindings
        // because they are optional in Python and phantom functions only need required params
        let sig = parse_signature("(x: int = 0, y: str = 'hello') -> bool");
        assert!(!sig.is_self_method);
        assert_eq!(sig.params.len(), 0); // All params have defaults, so none are required

        // Test with a mix of required and optional params
        let sig2 = parse_signature("(url: str, timeout: int = 30) -> Response");
        assert_eq!(sig2.params.len(), 1); // Only url is required
        assert_eq!(sig2.params[0], ("url".to_string(), "str".to_string()));
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
        // DEPYLER-1115: str maps to &str for phantom bindings (ergonomic call sites)
        assert_eq!(python_type_to_rust("str").to_string(), "& str");
        assert_eq!(python_type_to_rust("bool").to_string(), "bool");
    }

    #[test]
    fn test_python_type_to_rust_collections() {
        let dict = python_type_to_rust("dict").to_string();
        assert!(dict.contains("HashMap"));
        assert!(dict.contains("String")); // DEPYLER-1115: NASA mode compatible

        let list = python_type_to_rust("list").to_string();
        assert!(list.contains("Vec"));
        assert!(list.contains("String")); // DEPYLER-1115: NASA mode compatible
    }

    #[test]
    fn test_python_type_to_rust_generics() {
        let list_int = python_type_to_rust("List[int]").to_string();
        assert!(list_int.contains("Vec"));
        assert!(list_int.contains("i64"));

        // DEPYLER-1115: Optional[str] maps to Option<&str> for phantom bindings
        let opt_str = python_type_to_rust("Optional[str]").to_string();
        assert!(opt_str.contains("Option"));
        assert!(opt_str.contains("str")); // Will contain "& str"
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

    #[cfg(feature = "sovereign-types")]
    #[test]
    fn test_generate_phantom_struct_uses_string_wrapper() {
        // DEPYLER-1115: Verify phantom struct uses String for NASA mode compatibility
        // NASA mode replaces serde_json::Value with String, so we use String directly
        let struct_tokens = generate_phantom_struct("TestResponse", &[]);
        let code = struct_tokens.to_string();

        // Should contain String wrapper for NASA mode compatibility
        assert!(code.contains("pub String"),
            "Expected String wrapper but got: {}", code);
        assert!(code.contains("pub struct TestResponse"));
        assert!(code.contains("fn new (inner : String)"),
            "Expected new(inner: String) constructor but got: {}", code);
    }

    #[test]
    fn test_generate_default_return_primitives() {
        // DEPYLER-1116: Test proxy pattern - default return values
        assert_eq!(generate_default_return("int").to_string(), "0i64");
        assert_eq!(generate_default_return("float").to_string(), "0.0f64");
        assert_eq!(generate_default_return("str").to_string(), "\"\"");
        assert_eq!(generate_default_return("bool").to_string(), "false");
        assert_eq!(generate_default_return("None").to_string(), "()");
    }

    #[test]
    fn test_generate_default_return_collections() {
        // DEPYLER-1116: Test proxy pattern - default collection values
        let dict = generate_default_return("dict").to_string();
        assert!(dict.contains("HashMap"));
        assert!(dict.contains("new"));

        let list = generate_default_return("list").to_string();
        assert!(list.contains("Vec"));
        assert!(list.contains("new"));

        let optional = generate_default_return("Optional[str]").to_string();
        assert_eq!(optional, "None");
    }
}

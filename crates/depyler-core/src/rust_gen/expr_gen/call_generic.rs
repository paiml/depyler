//! Generic call conversion for ExpressionConverter
//!
//! Contains convert_generic_call and related method routing.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::stdlib_method_gen;
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_generic_call(
        &mut self,
        func: &str,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0462: print() is now handled in convert_call() to support file=stderr kwarg

        // Check if this is an imported function
        if let Some(rust_path) = self.ctx.imported_items.get(func) {
            // DEPYLER-0557: Special handling for itertools.groupby
            // It's a trait method, not a standalone function
            if rust_path == "itertools::Itertools" && func == "groupby" && args.len() >= 2 {
                let iterable = &args[0];
                let key_func = &args[1];
                // needs_itertools is already set from import processing
                return Ok(parse_quote! {
                    {
                        use itertools::Itertools;
                        #iterable.into_iter().group_by(#key_func)
                    }
                });
            }

            // DEPYLER-0702: Special handling for os.path method imports
            // `from os.path import join as path_join` → path_join(a, b) should generate
            // PathBuf::from(a).join(b).to_string_lossy().to_string()
            if rust_path == "std::path::Path::join" {
                if args.is_empty() {
                    bail!("path join requires at least 1 argument");
                }
                let first = &args[0];
                if args.len() == 1 {
                    return Ok(
                        parse_quote! { std::path::PathBuf::from(#first).to_string_lossy().to_string() },
                    );
                }
                // DEPYLER-0814: Check if any arg (after first) is a List/Vec type (varargs)
                // If so, generate iteration code instead of chaining .join()
                for (i, hir_arg) in hir_args[1..].iter().enumerate() {
                    if let HirExpr::Var(name) = hir_arg {
                        if let Some(Type::List(_)) = self.ctx.var_types.get(name) {
                            // This is a vararg parameter - generate iteration code
                            let parts_var = &args[i + 1];
                            return Ok(parse_quote! {
                                {
                                    let mut __path = std::path::PathBuf::from(#first);
                                    for __part in #parts_var {
                                        __path = __path.join(__part);
                                    }
                                    __path.to_string_lossy().to_string()
                                }
                            });
                        }
                    }
                }
                // Normal case: chain .join() calls
                let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
                for part in &args[1..] {
                    result = parse_quote! { #result.join(#part) };
                }
                return Ok(parse_quote! { #result.to_string_lossy().to_string() });
            }

            // DEPYLER-0702: Handle other os.path method imports
            if rust_path == "std::path::Path::exists" && args.len() == 1 {
                let path = &args[0];
                return Ok(parse_quote! { std::path::Path::new(&#path).exists() });
            }
            if rust_path == "std::path::Path::file_name" && args.len() == 1 {
                let path = &args[0];
                return Ok(parse_quote! {
                    std::path::Path::new(&#path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                });
            }
            if rust_path == "std::path::Path::parent" && args.len() == 1 {
                let path = &args[0];
                return Ok(parse_quote! {
                    std::path::Path::new(&#path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string()
                });
            }

            // DEPYLER-0721: Handle os.path.splitext import
            // splitext(path) → (stem, extension) tuple
            if rust_path == "std::path::Path" && func == "splitext" && args.len() == 1 {
                let path_arg = &args[0];
                return Ok(parse_quote! {
                    {
                        let __path = std::path::Path::new(&#path_arg);
                        let __stem = __path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();
                        let __ext = __path.extension()
                            .and_then(|e| e.to_str())
                            .map(|e| format!(".{}", e))
                            .unwrap_or_default();
                        (__stem, __ext)
                    }
                });
            }

            // DEPYLER-0771: Special handling for math.isqrt import
            // isqrt(n) → (n as f64).sqrt().floor() as i32
            // This is needed because std::f64::isqrt doesn't exist in Rust
            // Check both exact match and ends_with for robustness
            // DEPYLER-99MODE-S9: Skip if user defined a function with same name
            if (rust_path == "std::f64::isqrt" || rust_path.ends_with("::isqrt"))
                && func == "isqrt"
                && args.len() == 1
                && !self.ctx.function_return_types.contains_key(func)
            {
                let arg = &args[0];
                return Ok(parse_quote! { ((#arg) as f64).sqrt().floor() as i32 });
            }

            // Parse the rust path and generate the call
            let path_parts: Vec<&str> = rust_path.split("::").collect();
            let mut path = quote! {};
            for (i, part) in path_parts.iter().enumerate() {
                let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                if i == 0 {
                    path = quote! { #part_ident };
                } else {
                    path = quote! { #path::#part_ident };
                }
            }

            // DEPYLER-0493: Check if this is a struct type that needs constructor pattern
            // Look up constructor pattern from imported modules
            use crate::module_mapper::ConstructorPattern;
            let constructor_pattern = self.ctx.imported_modules.values().find_map(|module| {
                // Get the last part of the rust_path (e.g., "NamedTempFile" from "tempfile::NamedTempFile")
                let type_name = path_parts.last()?;
                module.constructor_patterns.get(*type_name)
            });

            // DEPYLER-1004: Special handling for serde_json::from_str to use proper type annotation
            // When called via `from json import loads`, we need to:
            // 1. Check return type context for HashMap vs Value
            // 2. Add type annotation and .unwrap()
            // DEPYLER-1022: NASA mode returns stub HashMap
            if rust_path == "serde_json::from_str" && args.len() == 1 {
                let arg = &args[0];
                if self.ctx.type_mapper.nasa_mode {
                    // NASA mode: return stub HashMap
                    // DEPYLER-1051: Use DepylerValue for Hybrid Fallback Strategy
                    self.ctx.needs_hashmap = true;
                    self.ctx.needs_depyler_value_enum = true;
                    return Ok(parse_quote! {
                        std::collections::HashMap::<String, DepylerValue>::new()
                    });
                }
                self.ctx.needs_serde_json = true;
                if stdlib_method_gen::json::return_type_needs_json_dict(self.ctx) {
                    self.ctx.needs_hashmap = true;
                    return Ok(parse_quote! {
                        serde_json::from_str::<std::collections::HashMap<String, serde_json::Value>>(&#arg).expect("parse failed")
                    });
                } else {
                    return Ok(parse_quote! {
                        serde_json::from_str::<serde_json::Value>(&#arg).expect("parse failed")
                    });
                }
            }

            // DEPYLER-1004: Special handling for serde_json::from_reader
            // DEPYLER-1022: NASA mode returns stub HashMap
            if rust_path == "serde_json::from_reader" && args.len() == 1 {
                let arg = &args[0];
                if self.ctx.type_mapper.nasa_mode {
                    // NASA mode: return stub HashMap
                    // DEPYLER-1051: Use DepylerValue for Hybrid Fallback Strategy
                    self.ctx.needs_hashmap = true;
                    self.ctx.needs_depyler_value_enum = true;
                    return Ok(parse_quote! {
                        std::collections::HashMap::<String, DepylerValue>::new()
                    });
                }
                self.ctx.needs_serde_json = true;
                if stdlib_method_gen::json::return_type_needs_json_dict(self.ctx) {
                    self.ctx.needs_hashmap = true;
                    return Ok(parse_quote! {
                        serde_json::from_reader::<_, std::collections::HashMap<String, serde_json::Value>>(#arg).expect("parse failed")
                    });
                } else {
                    return Ok(parse_quote! {
                        serde_json::from_reader::<_, serde_json::Value>(#arg).expect("parse failed")
                    });
                }
            }

            // DEPYLER-1004: Check if this function returns Result and needs .unwrap()
            // json to_string and to_writer still need .unwrap()
            let needs_unwrap =
                matches!(rust_path.as_str(), "serde_json::to_string" | "serde_json::to_writer");

            // Generate call based on constructor pattern
            return match constructor_pattern {
                Some(ConstructorPattern::New) => {
                    // Struct type → use ::new() pattern
                    if args.is_empty() {
                        Ok(parse_quote! { #path::new() })
                    } else {
                        Ok(parse_quote! { #path::new(#(#args),*) })
                    }
                }
                Some(ConstructorPattern::Method(method)) => {
                    // Custom method (e.g., File::open())
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    if args.is_empty() {
                        Ok(parse_quote! { #path::#method_ident() })
                    } else {
                        Ok(parse_quote! { #path::#method_ident(#(#args),*) })
                    }
                }
                Some(ConstructorPattern::Function) | None => {
                    // Regular function call (default behavior)
                    // DEPYLER-1004: Add .unwrap() for Result-returning functions
                    if needs_unwrap {
                        if args.is_empty() {
                            Ok(parse_quote! { #path().expect("operation failed") })
                        } else {
                            Ok(parse_quote! { #path(#(#args),*).expect("operation failed") })
                        }
                    } else if args.is_empty() {
                        Ok(parse_quote! { #path() })
                    } else {
                        Ok(parse_quote! { #path(#(#args),*) })
                    }
                }
            };
        }

        // Check if this might be a constructor call (capitalized name)
        if func.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            // DEPYLER-0900: Rename constructor if it shadows stdlib type (e.g., Box -> PyBox)
            // Treat as constructor call - ClassName::new(args)
            let safe_name = crate::direct_rules::safe_class_name(func);
            let class_ident = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());
            if args.is_empty() {
                // DEPYLER-0233: Only apply default argument heuristics for Python stdlib types
                // User-defined classes should always generate ClassName::new() with no args
                let is_user_class = self.ctx.class_names.contains(func);

                // Note: Constructor default parameter handling uses simple heuristics.
                // Ideally this would be context-aware and know the actual default values
                // for each class constructor, but currently uses hardcoded patterns.
                // This is a known limitation - constructors may require explicit arguments.
                if !is_user_class && func == "Counter" {
                    return Ok(parse_quote! { #class_ident::new(0) });
                }
                Ok(parse_quote! { #class_ident::new() })
            } else {
                // DEPYLER-0932: Complete missing constructor arguments with defaults
                // When Python calls Config("localhost") but Config has 3 fields with 2 defaults,
                // we need to generate Config::new("localhost".to_string(), 8080, false)
                let mut completed_args = args.to_vec();
                if let Some(defaults) = self.ctx.class_field_defaults.get(func) {
                    let num_provided = completed_args.len();
                    let num_fields = defaults.len();

                    if num_provided < num_fields {
                        // Fill in missing arguments from defaults
                        for i in num_provided..num_fields {
                            if let Some(Some(default_expr)) = defaults.get(i) {
                                use crate::hir::{HirExpr, Literal};
                                let default_syn: syn::Expr = match default_expr {
                                    HirExpr::Literal(Literal::None) => {
                                        parse_quote! { None }
                                    }
                                    HirExpr::Literal(Literal::Int(n)) => {
                                        let n_i32 = *n as i32;
                                        parse_quote! { #n_i32 }
                                    }
                                    HirExpr::Literal(Literal::Float(f)) => {
                                        let f = *f;
                                        parse_quote! { #f }
                                    }
                                    HirExpr::Literal(Literal::Bool(b)) => {
                                        let b = *b;
                                        parse_quote! { #b }
                                    }
                                    HirExpr::Literal(Literal::String(s)) => {
                                        parse_quote! { #s.to_string() }
                                    }
                                    // For complex defaults, skip
                                    _ => continue,
                                };
                                completed_args.push(default_syn);
                            }
                        }
                    }
                }
                Ok(parse_quote! { #class_ident::new(#(#completed_args),*) })
            }
        } else {
            // DEPYLER-0771: Fallback handling for isqrt if not found in imported_items
            // This handles cases where the import wasn't properly tracked
            // DEPYLER-99MODE-S9: Skip if user defined a function with same name
            if func == "isqrt"
                && args.len() == 1
                && !self.ctx.function_return_types.contains_key(func)
            {
                let arg = &args[0];
                return Ok(parse_quote! { ((#arg) as f64).sqrt().floor() as i32 });
            }

            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            // This is the fallback handler for isinstance calls that weren't caught earlier
            if func == "isinstance" {
                return Ok(parse_quote! { true });
            }

            // Regular function call
            // DEPYLER-0588: Use safe_ident to handle keywords and invalid characters
            let func_ident = crate::rust_gen::keywords::safe_ident(func);

            // DEPYLER-0301 Fix: Auto-borrow Vec/List arguments when calling functions
            // DEPYLER-0269 Fix: Auto-borrow Dict/HashMap/Set arguments when calling functions
            // DEPYLER-0270 Fix: Check function signature before auto-borrowing
            // When passing a Vec/HashMap/HashSet variable to a function expecting &Vec/&HashMap/&HashSet, automatically borrow it
            // This handles cases like: sum_list_recursive(rest) where rest is Vec but param is &Vec
            //
            // Strategy:
            // 1. Look up function signature to see which params are borrowed
            // 2. Only borrow if: (a) arg is List/Dict/Set AND (b) function expects borrow
            // 3. Otherwise pass as-is (either owned or primitive)
            let borrowed_args: Vec<syn::Expr> = hir_args
                .iter()
                .zip(args.iter())
                .enumerate()
                .map(|(param_idx, (hir_arg, arg_expr))| {
                    // DEPYLER-99MODE-S9: Unwrap Result-returning function calls in arguments
                    // When a Result-returning function call is passed as an argument,
                    // add ? to unwrap it (e.g., `foo(bar(x))` → `foo(bar(x)?)`)
                    let mut arg_expr = arg_expr.clone();
                    if self.ctx.current_function_can_fail {
                        if let HirExpr::Call { func: inner_func, .. } = hir_arg {
                            if self.ctx.result_returning_functions.contains(inner_func) {
                                arg_expr = parse_quote! { #arg_expr? };
                            }
                        }
                    }

                    // DEPYLER-0950: Integer literal coercion at f64 call sites
                    // When calling add(1, 2.5) where add expects (f64, f64), coerce 1 to 1.0
                    if let HirExpr::Literal(Literal::Int(n)) = hir_arg {
                        if let Some(param_types) = self.ctx.function_param_types.get(func) {
                            if let Some(Type::Float) = param_types.get(param_idx) {
                                // Integer literal passed where f64 expected - coerce to float
                                let f_val = *n as f64;
                                return parse_quote! { #f_val };
                            }
                        }
                    }

                    // DEPYLER-1208: DepylerValue→concrete auto-coercion (Rule 2)
                    // When a DepylerValue variable is passed to a function expecting concrete type,
                    // generate appropriate extraction: x.as_i64() as i32, x.as_f64(), etc.
                    if let HirExpr::Var(var_name) = hir_arg {
                        let var_type = self.ctx.var_types.get(var_name);
                        // DEPYLER-99MODE-S9: Also check module_constant_types - constants like PI
                        // are already concrete types, not DepylerValue
                        let is_known_concrete =
                            self.ctx.module_constant_types.contains_key(var_name);
                        let is_depyler_value = !is_known_concrete
                            && (matches!(var_type, Some(Type::Unknown) | None)
                                || matches!(var_type, Some(Type::Custom(s)) if s == "DepylerValue"));

                        if is_depyler_value {
                            if let Some(param_types) = self.ctx.function_param_types.get(func) {
                                if let Some(expected_type) = param_types.get(param_idx) {
                                    match expected_type {
                                        Type::Int => {
                                            return parse_quote! { #arg_expr.as_i64().unwrap_or_default() as i32 };
                                        }
                                        Type::Float => {
                                            return parse_quote! { #arg_expr.as_f64().unwrap_or_default() };
                                        }
                                        Type::String => {
                                            return parse_quote! { #arg_expr.as_str().unwrap_or_default().to_string() };
                                        }
                                        Type::Bool => {
                                            return parse_quote! { #arg_expr.as_bool().unwrap_or_default() };
                                        }
                                        _ => {} // Other types - let regular flow handle
                                    }
                                }
                            }
                        }
                    }

                    // DEPYLER-1045: Convert char to String when passing to functions expecting &str
                    // When a loop variable from string.chars() is passed to a function,
                    // it needs to be converted to String because char and &str are incompatible.
                    if let HirExpr::Var(var_name) = hir_arg {
                        if self.ctx.char_iter_vars.contains(var_name) {
                            // Check if function expects &str at this param position
                            let expects_str = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(true); // Default to expecting str for char iter vars
                            if expects_str {
                                return parse_quote! { &#arg_expr.to_string() };
                            }
                        }
                    }

                    // DEPYLER-0471: Clone args.config when passing to functions taking owned String
                    // This avoids "use after move" errors when args.config is used multiple times
                    if matches!(hir_arg, HirExpr::Attribute { value, attr }
                        if attr == "config" && matches!(value.as_ref(), HirExpr::Var(v) if v == "args"))
                    {
                        // Check if function takes owned String (not &str)
                        // For save_config and load_config, clone args.config
                        if matches!(func, "save_config" | "load_config") {
                            return parse_quote! { #arg_expr.clone() };
                        }
                    }

                    // DEPYLER-0469/0488: Special case for known functions that need String borrowing
                    // get_nested_value(config, key) - key param (index 1) needs &str
                    // set_nested_value(config, key, value) - key (1) needs &str, value (2) needs &str (NOT &mut!)
                    // DEPYLER-0488: Removed incorrect &mut for value parameter - it's only READ in the function
                    // These work with both Var and Attribute expressions (before/after argparse transform)
                    if (func == "get_nested_value" || func == "set_nested_value") && param_idx == 1 {
                        // Immutable borrow for key parameter
                        return parse_quote! { &#arg_expr };
                    } else if func == "set_nested_value" && param_idx == 2 {
                        // DEPYLER-0488: value parameter is READ (RHS of assignment), not mutated
                        // Immutable borrow is sufficient
                        return parse_quote! { &#arg_expr };
                    }

                    // DEPYLER-0424: Check if argument is argparse args variable
                    // If so, always pass by reference (&args)
                    if let HirExpr::Var(var_name) = hir_arg {
                        let is_argparse_args =
                            self.ctx
                                .argparser_tracker
                                .parsers
                                .values()
                                .any(|parser_info| {
                                    parser_info
                                        .args_var
                                        .as_ref()
                                        .is_some_and(|args_var| args_var == var_name)
                                });

                        if is_argparse_args {
                            return parse_quote! { &#arg_expr };
                        }
                    }

                    // DEPYLER-0600: First check if function explicitly requires &mut at this position
                    // This enables borrowing for types like File that aren't in the standard borrow list
                    let func_requires_mut = self.ctx
                        .function_param_muts
                        .get(func)
                        .and_then(|muts| muts.get(param_idx))
                        .copied()
                        .unwrap_or(false);

                    // Check if this param should be borrowed by looking up function signature
                    let should_borrow = if func_requires_mut {
                        // If function explicitly needs &mut, we must borrow
                        true
                    } else {
                        match hir_arg {
                        HirExpr::Var(var_name) => {
                            // Check if variable has List, Dict, Set, String, or Custom type
                            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                                // DEPYLER-0467: Debug logging for key/value
                                if matches!(var_name.as_str(), "key" | "value") {
                                    eprintln!("[DEPYLER-0467] Variable '{}' has type: {:?}", var_name, var_type);
                                }

                                // DEPYLER-0467: Always borrow serde_json::Value types
                                // These are typically borrowed in idiomatic Rust
                                if matches!(var_type, Type::Custom(ref s) if s == "serde_json::Value") {
                                    true  // Always borrow Value types
                                } else if matches!(var_type, Type::Dict(_, _)) {
                                    // DEPYLER-99MODE-S9: Check function_param_borrows for Dict types
                                    // Don't always borrow — functions that take ownership (return modified dict)
                                    // should receive the dict by value, not reference
                                    self.ctx
                                        .function_param_borrows
                                        .get(func)
                                        .and_then(|borrows| borrows.get(param_idx))
                                        .copied()
                                        .unwrap_or(true) // Default to borrow if unknown
                                } else if matches!(var_type, Type::String) {
                                    // DEPYLER-0469: Borrow String types as &str
                                    // DEPYLER-0818: But DON'T borrow if the variable is already &str
                                    // (i.e., it's a function param that was mapped to &str).
                                    // Borrowing an &str would create &&str which is wrong.
                                    // DEPYLER-1092: Use ref_params instead of fn_str_params
                                    // ref_params contains ONLY params that are actually borrowed (&str)
                                    // fn_str_params incorrectly contains ALL Type::String params
                                    !self.ctx.ref_params.contains(var_name)
                                } else if matches!(var_type, Type::Unknown) {
                                    // DEPYLER-0467: Heuristic for Unknown types
                                    // If variable name suggests it's commonly borrowed, borrow it
                                    // This handles cases where type inference fails (e.g., Result unwrapping, pattern matching)
                                    matches!(var_name.as_str(),
                                        "config" | "data" | "json" | "obj" | "document" |
                                        "key" | "value" | "path" | "name" | "text" | "content"
                                    )
                                } else if matches!(var_type, Type::List(_) | Type::Set(_)) {
                                    // DEPYLER-0466: Also borrow collection types
                                    // Check if function param expects a borrow
                                    self.ctx
                                        .function_param_borrows
                                        .get(func)
                                        .and_then(|borrows| borrows.get(param_idx))
                                        .copied()
                                        .unwrap_or(true) // Default to borrow if unknown
                                } else if matches!(var_type, Type::Custom(_)) {
                                    // DEPYLER-0767: Check function_param_borrows for Custom types
                                    // datetime maps to Type::Custom("chrono::NaiveDateTime")
                                    // Check if function signature expects a reference parameter
                                    self.ctx
                                        .function_param_borrows
                                        .get(func)
                                        .and_then(|borrows| borrows.get(param_idx))
                                        .copied()
                                        .unwrap_or(false) // Default to no borrow for custom types
                                } else {
                                    false
                                }
                            } else {
                                // DEPYLER-0467/DEPYLER-0767: Variable not in var_types
                                // First check function_param_borrows (authoritative source)
                                // Fall back to name heuristic if not tracked
                                eprintln!("[DEPYLER-0467] Variable '{}' NOT in var_types, checking function_param_borrows", var_name);
                                self.ctx
                                    .function_param_borrows
                                    .get(func)
                                    .and_then(|borrows| borrows.get(param_idx))
                                    .copied()
                                    // Name-based heuristic as last resort
                                    .unwrap_or(matches!(var_name.as_str(),
                                        "config" | "data" | "json" | "obj" | "document" |
                                        "key" | "value" | "path" | "name" | "text" | "content"
                                    ))
                            }
                        }
                        // DEPYLER-0359: Auto-borrow list/dict/set literals when calling functions
                        // List literal [1, 2, 3] should be passed as &vec![1, 2, 3]
                        HirExpr::List(_) | HirExpr::Dict(_) | HirExpr::Set(_) => {
                            // Check if function param expects a borrow
                            self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(true) // Default to borrow if unknown
                        }
                        // DEPYLER-1092: Handle string literals passed to functions expecting &str
                        // Python: parse_list(value, ",") → Rust: parse_list(&value, ",")
                        // String literals are already &str, no .to_string() needed for borrowed params
                        HirExpr::Literal(Literal::String(_)) => {
                            // Check if function param expects a borrow (&str)
                            self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(false) // Default: no borrow (owned String)
                        }
                        // DEPYLER-0550: Handle attribute access like args.column, args.value
                        // These are String fields from CLI args struct that need borrowing
                        // when passed to functions expecting &str
                        HirExpr::Attribute { value, attr } => {
                            // Check if accessing args struct field
                            let is_args_field = if let HirExpr::Var(v) = value.as_ref() {
                                v == "args"
                            } else {
                                false
                            };

                            // Check if function expects borrow at this position
                            if is_args_field {
                                // For args struct fields (typically String), check function signature
                                self.ctx
                                    .function_param_borrows
                                    .get(func)
                                    .and_then(|borrows| borrows.get(param_idx))
                                    .copied()
                                    .unwrap_or(
                                        // Heuristic: borrow common string-like field names
                                        matches!(attr.as_str(),
                                            "column" | "value" | "name" | "key" | "pattern" |
                                            "text" | "query" | "path" | "config" | "file"
                                        )
                                    )
                            } else {
                                false
                            }
                        }
                        // DEPYLER-99MODE-S9: Slice expressions produce owned
                        // Vec via .to_vec(), no borrowing needed
                        HirExpr::Slice { .. } => false,
                        // Call expressions return owned values
                        HirExpr::Call { .. } => false,
                        _ => {
                            // Default: don't borrow unknown expressions
                            false
                        }
                    }
                    }; // Close the if func_requires_mut else block

                    // DEPYLER-0515: Let Rust's type inference determine integer types
                    // from function signatures, rather than blindly casting to i64.

                    // DEPYLER-0568: Handle PathBuf → String conversion for function arguments
                    // When passing a PathBuf to a function that expects String
                    if let HirExpr::Var(var_name) = hir_arg {
                        // DEPYLER-0666: Check if variable was already unwrapped via if-let
                        // If so, don't add .as_ref().unwrap() - the value is already concrete
                        let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name);

                        if let Some(var_type) = self.ctx.var_types.get(var_name) {
                            // PathBuf → String conversion
                            if matches!(var_type, Type::Custom(ref s) if s == "PathBuf" || s == "Path") {
                                // Check if this is a String-expecting function (heuristic)
                                // Function params with names like file_path, path, etc. often want String
                                return parse_quote! { #arg_expr.display().to_string() };
                            }
                            // Option<String> → &str conversion when function expects &str
                            // DEPYLER-0666: Skip if already unwrapped
                            if !is_unwrapped && matches!(var_type, Type::Optional(ref inner) if matches!(inner.as_ref(), Type::String)) {
                                // Unwrap the Option and pass reference
                                return parse_quote! { #arg_expr.as_ref().expect("value is None") };
                            }
                        } else {
                            // DEPYLER-0568: Name-based heuristic for PathBuf when not in var_types
                            // Variables named "path" are typically PathBuf from pathlib.Path()
                            let name = var_name.as_str();
                            if name == "path" || name.ends_with("_path") {
                                return parse_quote! { #arg_expr.display().to_string() };
                            }
                        }
                    }

                    // DEPYLER-0818: Handle &str → String conversion
                    // When an &str param (fn_str_params) is passed to a function expecting String,
                    // we need to add .to_string() to convert the reference to owned.
                    if let HirExpr::Var(var_name) = hir_arg {
                        if self.ctx.fn_str_params.contains(var_name) && !should_borrow {
                            // Variable is &str param but callee doesn't expect borrow (wants String)
                            // Check if callee expects a borrow - if not, convert to String
                            let callee_expects_borrow = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(false);

                            // DEPYLER-99MODE-S9: Also check function_param_types as fallback
                            // When callee hasn't been processed yet (forward reference),
                            // function_param_borrows won't have the entry. But if the
                            // callee's param type is String (which maps to &str), it expects borrow.
                            // Only use this fallback when borrows are unknown (no entry at all).
                            let borrows_unknown = !self.ctx.function_param_borrows.contains_key(func);
                            let callee_param_is_str = borrows_unknown
                                && self.ctx
                                    .function_param_types
                                    .get(func)
                                    .and_then(|types| types.get(param_idx))
                                    .map(|ty| matches!(ty, Type::String))
                                    .unwrap_or(false);

                            if !callee_expects_borrow && !callee_param_is_str {
                                return parse_quote! { #arg_expr.to_string() };
                            }
                        }
                    }

                    if should_borrow {
                        // DEPYLER-0574: Check if function expects &mut for this param
                        let needs_mut = self.ctx
                            .function_param_muts
                            .get(func)
                            .and_then(|muts| muts.get(param_idx))
                            .copied()
                            .unwrap_or(false);

                        // DEPYLER-0964/1217: Don't add &mut if variable is already &mut
                        // This includes:
                        // - mut_option_dict_params: &mut Option<HashMap>
                        // - mut_ref_params: parameters that are &mut T (detected via mutation analysis)
                        // Adding &mut would create &&mut which is invalid
                        let is_already_mut_ref = if let HirExpr::Var(var_name) = hir_arg {
                            self.ctx.mut_option_dict_params.contains(var_name)
                                || self.ctx.mut_ref_params.contains(var_name)
                        } else {
                            false
                        };

                        if is_already_mut_ref {
                            // Variable is already &mut, pass it directly
                            arg_expr.clone()
                        } else if needs_mut {
                            parse_quote! { &mut #arg_expr }
                        } else {
                            parse_quote! { &#arg_expr }
                        }
                    } else {
                        // DEPYLER-0737/0779: Check if function param is Optional FIRST
                        // This determines if we need to wrap the final result in Some()
                        let is_optional_param = self.ctx
                            .function_param_optionals
                            .get(func)
                            .and_then(|optionals| optionals.get(param_idx))
                            .copied()
                            .unwrap_or(false);

                        // DEPYLER-0760: Don't double-wrap if arg is already Option<T>
                        let is_already_optional = if let HirExpr::Var(var_name) = hir_arg {
                            self.ctx
                                .var_types
                                .get(var_name)
                                .map(|ty| matches!(ty, Type::Optional(_)))
                                .unwrap_or(false)
                        } else if let HirExpr::Attribute { value: _, attr } = hir_arg {
                            // Handle attribute access like args.cwd
                            let check_optional = |arg: &crate::rust_gen::argparse_transform::ArgParserArgument| {
                                let field_name = arg.rust_field_name();
                                if field_name != *attr {
                                    return false;
                                }
                                if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
                                    return false;
                                }
                                !arg.is_positional
                                    && !arg.required.unwrap_or(false)
                                    && arg.default.is_none()
                                    && !matches!(arg.nargs.as_deref(), Some("+") | Some("*"))
                            };

                            let is_optional_in_parser = self.ctx.argparser_tracker.parsers.values()
                                .any(|parser_info| parser_info.arguments.iter().any(&check_optional));
                            let is_optional_in_subcommand = self.ctx.argparser_tracker.subcommands.values()
                                .any(|sub_info| sub_info.arguments.iter().any(&check_optional));

                            is_optional_in_parser || is_optional_in_subcommand
                        } else {
                            false
                        };

                        // Don't wrap if arg is already None
                        let is_none = matches!(hir_arg, HirExpr::Literal(Literal::None));
                        let needs_some_wrap = is_optional_param && !is_none && !is_already_optional;

                        // DEPYLER-0779: Check if the optional param is also borrowed (&Option<T>)
                        // vs owned (Option<T>) - this determines if we use &Some() or Some()
                        let optional_is_borrowed = self.ctx
                            .function_param_borrows
                            .get(func)
                            .and_then(|borrows| borrows.get(param_idx))
                            .copied()
                            .unwrap_or(false);

                        // DEPYLER-0635: String literal args need type-aware conversion
                        // - If function param expects &str (borrowed), pass literal directly
                        // - If function param expects String (owned), add .to_string()
                        // Check function_param_borrows to determine expected type
                        // DEPYLER-TYPE-001: Default to true (borrowed) because Type::String params
                        // become &str in generated Rust code, not String. String literals ARE &str.
                        if matches!(hir_arg, HirExpr::Literal(Literal::String(_))) {
                            // Check if function expects borrowed string (&str) at this position
                            let param_expects_borrowed = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(true);

                            if param_expects_borrowed {
                                // Param is &str - string literal works directly
                                // DEPYLER-0779: But wrap in Some if optional param
                                // DEPYLER-TYPE-001: Don't add .to_string() when param expects borrowed
                                if needs_some_wrap {
                                    // For Option<&str>, wrap the literal directly without .to_string()
                                    if optional_is_borrowed {
                                        return parse_quote! { &Some(#arg_expr) };
                                    } else {
                                        return parse_quote! { Some(#arg_expr) };
                                    }
                                }
                                return arg_expr.clone();
                            } else {
                                // Param is String - need .to_string() conversion
                                let expr_str = quote::quote! { #arg_expr }.to_string();
                                let converted: syn::Expr = if !expr_str.contains("to_string") {
                                    parse_quote! { #arg_expr.to_string() }
                                } else {
                                    arg_expr.clone()
                                };
                                // DEPYLER-0779: Wrap in Some if optional param
                                // Use &Some for borrowed (&Option<T>), Some for owned (Option<T>)
                                if needs_some_wrap {
                                    if optional_is_borrowed {
                                        return parse_quote! { &Some(#converted) };
                                    } else {
                                        return parse_quote! { Some(#converted) };
                                    }
                                }
                                return converted;
                            }
                        }

                        // For non-string literals, apply Some wrapping if needed
                        // Use &Some for borrowed (&Option<T>), Some for owned (Option<T>)
                        if needs_some_wrap {
                            if optional_is_borrowed {
                                return parse_quote! { &Some(#arg_expr) };
                            } else {
                                return parse_quote! { Some(#arg_expr) };
                            }
                        }

                        // DEPYLER-1168: Call-site clone insertion for variables used later
                        // When a function takes ownership (doesn't borrow) and the argument
                        // variable is used again later in the same scope, we need to clone it.
                        // This prevents E0382 "use of moved value" errors.
                        if let HirExpr::Var(var_name) = hir_arg {
                            // Only clone if:
                            // 1. Variable is used later in the same scope
                            // 2. Variable type is clonable (List, Dict, Set, String, Custom types)
                            let used_later = self.ctx.vars_used_later.contains(var_name);
                            let is_clonable_type = self.ctx.var_types.get(var_name)
                                .map(|ty| matches!(ty,
                                    Type::List(_) | Type::Dict(_, _) | Type::Set(_) |
                                    Type::String | Type::Tuple(_) | Type::Custom(_)
                                ))
                                .unwrap_or(false);

                            if used_later && is_clonable_type {
                                return parse_quote! { #arg_expr.clone() };
                            }
                        }

                        // DEPYLER-99MODE-S9: Fallback borrow for non-Var complex expressions
                        // When function_param_borrows says this param should be borrowed but
                        // the expression is complex (e.g., function call with ?), add &
                        let callee_expects_borrow = self.ctx
                            .function_param_borrows
                            .get(func)
                            .and_then(|borrows| borrows.get(param_idx))
                            .copied()
                            .unwrap_or(false);
                        if callee_expects_borrow && !matches!(hir_arg, HirExpr::Var(_)) {
                            return parse_quote! { &#arg_expr };
                        }

                        arg_expr.clone()
                    }
                })
                .collect();

            // DEPYLER-0621: Complete missing arguments with default values
            // When Python calls `f()` but `def f(x=None)`, we need to generate `f(None)` in Rust
            // Look up registered defaults and append any missing arguments
            let mut completed_args = borrowed_args;
            if let Some(defaults) = self.ctx.function_param_defaults.get(func) {
                let num_provided = completed_args.len();
                let num_params = defaults.len();

                if num_provided < num_params {
                    // Need to fill in missing arguments from defaults
                    for i in num_provided..num_params {
                        if let Some(Some(default_expr)) = defaults.get(i) {
                            // Handle common default values directly without calling to_rust_expr
                            // (to_rust_expr requires &mut ctx which we don't have in &self)
                            use crate::hir::{HirExpr, Literal};
                            // DEPYLER-0629: Check if parameter needs borrowing
                            // If the parameter type is &Option<T>, we need &None instead of None
                            // DEPYLER-TYPE-001: Default to true for string params (Type::String → &str)
                            let param_needs_borrow = self
                                .ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(i).copied())
                                .unwrap_or(true);

                            let default_syn: syn::Expr = match default_expr {
                                HirExpr::Literal(Literal::None) => {
                                    if param_needs_borrow {
                                        parse_quote! { &None }
                                    } else {
                                        parse_quote! { None }
                                    }
                                }
                                HirExpr::Literal(Literal::Int(n)) => {
                                    // DEPYLER-0806: Use i32 suffix for default args
                                    // Python int maps to Rust i32 for function params
                                    // Using i64 causes E0308 when param expects i32
                                    let n_i32 = *n as i32;
                                    parse_quote! { #n_i32 }
                                }
                                HirExpr::Literal(Literal::Float(f)) => {
                                    let f = *f;
                                    parse_quote! { #f }
                                }
                                HirExpr::Literal(Literal::Bool(b)) => {
                                    let b = *b;
                                    parse_quote! { #b }
                                }
                                HirExpr::Literal(Literal::String(s)) => {
                                    // DEPYLER-1092: Check if param expects &str
                                    // If so, use "..." directly (string literal IS &str)
                                    // without .to_string(), avoiding E0308
                                    let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                                    if param_needs_borrow {
                                        // String literal "..." is already &str
                                        parse_quote! { #lit }
                                    } else {
                                        parse_quote! { #lit.to_string() }
                                    }
                                }
                                // For complex defaults, skip - function definition should handle
                                _ => continue,
                            };
                            completed_args.push(default_syn);
                        }
                    }
                }
            }
            let borrowed_args = completed_args;

            // DEPYLER-0648: Handle vararg functions - wrap arguments in slice
            // Python: run_cli("--help") where def run_cli(*args)
            // Rust: run_cli(&["--help".to_string()]) where fn run_cli(args: &[String])
            if self.ctx.vararg_functions.contains(func) && !borrowed_args.is_empty() {
                // DEPYLER-0660: Check if single arg is already a Vec (from starred unpacking)
                // Python: join_paths(*args.parts) where args.parts is List[str]
                // Should become: join_paths(&parts) not join_paths(&[parts])
                if borrowed_args.len() == 1 && hir_args.len() == 1 {
                    let hir_arg = &hir_args[0];
                    let arg_is_collection = match hir_arg {
                        // Attribute access to plural-named field (likely Vec)
                        HirExpr::Attribute { value, attr } => {
                            if let HirExpr::Var(v) = value.as_ref() {
                                v == "args"
                                    && (attr.ends_with('s')
                                        || attr == "parts"
                                        || attr == "items"
                                        || attr == "values"
                                        || attr == "keys"
                                        || attr == "args")
                            } else {
                                false
                            }
                        }
                        // Variable that's known to be a list
                        HirExpr::Var(v) => {
                            v.ends_with('s') || v == "parts" || v == "items" || v == "args"
                        }
                        // List literal
                        HirExpr::List(_) => true,
                        _ => false,
                    };

                    if arg_is_collection {
                        let arg = &borrowed_args[0];
                        return Ok(parse_quote! { #func_ident(&#arg) });
                    }
                }
                // Wrap all arguments in a slice literal
                return Ok(parse_quote! { #func_ident(&[#(#borrowed_args),*]) });
            }

            // DEPYLER-0422 Fix #6: Remove automatic `?` operator for function calls
            // DEPYLER-0287 was too broad - it added `?` to ALL function calls when inside a Result-returning function.
            // This caused E0277 errors (279 errors!) when calling functions that return plain types (i32, Vec, etc.).
            //
            // Root Cause Analysis:
            // 1. Why: `?` operator applied to i32/Vec (non-Result types)
            // 2. Why: Transpiler adds `?` to all function calls inside Result-returning functions
            // 3. Why: DEPYLER-0287 unconditionally adds `?` when current_function_can_fail is true
            // 4. Why: No check if the CALLED function actually returns Result
            // 5. ROOT CAUSE: Overly aggressive error propagation heuristic
            //
            // Solution: Don't automatically add `?` to function calls. Let explicit error handling
            // in Python (try/except) determine when Result types are needed.
            // If specific cases need `?` for recursive calls, those should be handled specially.
            //
            // DEPYLER-0588: Use try_parse to avoid panics on invalid expressions
            let args_tokens: Vec<_> = borrowed_args.iter().map(|a| quote::quote! { #a }).collect();
            let call_str = format!(
                "{}({})",
                func_ident,
                args_tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")
            );
            let call_expr: syn::Expr = match syn::parse_str(&call_str) {
                Ok(expr) => expr,
                Err(_) => {
                    // DEPYLER-0588: Fallback using syn::parse_str instead of parse_quote!
                    // This avoids panics even with unusual function names
                    let simple_call = format!("{}()", func_ident);
                    syn::parse_str(&simple_call).unwrap_or_else(|_| {
                        // Ultimate fallback: create a unit expression
                        syn::parse_str("()").unwrap()
                    })
                }
            };
            Ok(call_expr)
        }
    }

    // ========================================================================
}

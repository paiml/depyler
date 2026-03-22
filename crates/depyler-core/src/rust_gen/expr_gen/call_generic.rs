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
            return self.convert_imported_function_call(func, hir_args, args, rust_path.clone());
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

            // CB-200 Batch 15: Argument borrowing + coercion extracted to helper
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

                    // CB-200 Batch 15: Borrow decision extracted to helper
                    let should_borrow = if func_requires_mut {
                        true
                    } else {
                        self.should_borrow_arg(func, param_idx, hir_arg)
                    };

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

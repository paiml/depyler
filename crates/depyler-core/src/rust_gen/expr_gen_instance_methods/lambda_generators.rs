//! Lambda, generator, formatting, and conditional expression conversion
//!
//! Contains convert_lambda, convert_await, convert_yield, convert_fstring,
//! convert_ifexpr, apply_truthiness_conversion, convert_sort_by_key,
//! convert_generator_expression, convert_nested_generators, build_nested_chain,
//! parse_target_pattern, convert_named_expr.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::keywords;
use crate::trace_decision;
use crate::rust_gen::truthiness_helpers::{
    is_collection_generic_base, is_collection_type_name, is_collection_var_name,
    is_option_var_name, is_string_var_name,
};
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_lambda(
        &mut self,
        params: &[String],
        body: &HirExpr,
    ) -> Result<syn::Expr> {
        // CITL: Trace lambda/closure conversion decision
        trace_decision!(
            category = DecisionCategory::Ownership,
            name = "lambda_closure",
            chosen = "closure",
            alternatives = ["fn_pointer", "closure_move", "closure_ref", "boxed_fn"],
            confidence = 0.87
        );

        // DEPYLER-1202: Variable Capture Pass - Identify captured variables from outer scope
        // Python lambdas freely capture outer scope variables. Rust move closures need
        // to have non-Copy types cloned before capture to avoid use-after-move errors.
        let param_set: std::collections::HashSet<String> = params.iter().cloned().collect();
        let body_vars = crate::rust_gen::var_analysis::collect_vars_in_expr(body);
        let captured_vars: Vec<String> = body_vars
            .into_iter()
            .filter(|v| !param_set.contains(v))
            .collect();

        // DEPYLER-1202: Generate clone statements for non-Copy captured variables
        let mut clone_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut clone_mappings: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for var_name in &captured_vars {
            // Skip 'self' - it's handled differently
            if var_name == "self" {
                continue;
            }

            // DEPYLER-1202 FIX: Only capture variables that EXIST in the outer scope
            // Variables not in ctx.var_types are likely:
            // - Loop variables from comprehensions (x in [x*2 for x in lst])
            // - Builtins (True, False, None)
            // - Function/class names
            // These don't need to be captured since they're not outer scope variables
            let var_type = match self.ctx.var_types.get(var_name) {
                Some(ty) => ty.clone(),
                None => continue, // Not in outer scope - skip
            };

            // Only clone non-Copy types
            if !var_type.is_copy() {
                let safe_var = crate::rust_gen::keywords::safe_ident(var_name);
                let clone_var_name = format!("{}_capture", var_name);
                let clone_var = crate::rust_gen::keywords::safe_ident(&clone_var_name);

                // Generate: let prefix_capture = prefix.clone();
                clone_stmts.push(quote::quote! {
                    let #clone_var = #safe_var.clone();
                });

                clone_mappings.insert(var_name.clone(), clone_var_name);
            }
        }

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
        // DEPYLER-1117: Add type annotations to fix E0282 errors
        // Parameters are typed based on body expression analysis
        let param_tokens: Vec<proc_macro2::TokenStream> = params
            .iter()
            .map(|p| {
                let ident = crate::rust_gen::keywords::safe_ident(p);
                // DEPYLER-1117: Infer type from body usage
                if let Some(ty) = self.infer_lambda_param_type(p, body) {
                    quote::quote! { #ident: #ty }
                } else {
                    quote::quote! { #ident }
                }
            })
            .collect();

        // DEPYLER-1202: Convert body expression with captured variable substitution
        // For captured variables that we cloned, we need to reference the clone
        let body_expr = if clone_mappings.is_empty() {
            body.to_rust_expr(self.ctx)?
        } else {
            // Substitute captured variables with their cloned versions in the body
            let substituted_body = self.substitute_captured_vars(body, &clone_mappings);
            substituted_body.to_rust_expr(self.ctx)?
        };

        // Generate closure
        // DEPYLER-0837: Use `move` closures to match Python's closure semantics
        // Python closures capture variables by reference but extend their lifetime
        // Rust requires `move` when returning closures that capture local variables
        let closure: syn::Expr = if params.is_empty() {
            // No parameters
            parse_quote! { move || #body_expr }
        } else if params.len() == 1 {
            // Single parameter with type annotation
            let param = &param_tokens[0];
            parse_quote! { move |#param| #body_expr }
        } else {
            // Multiple parameters with type annotations
            parse_quote! { move |#(#param_tokens),*| #body_expr }
        };

        // DEPYLER-1202: Wrap closure in a block with clone statements if needed
        if clone_stmts.is_empty() {
            Ok(closure)
        } else {
            Ok(parse_quote! {
                {
                    #(#clone_stmts)*
                    #closure
                }
            })
        }
    }

    /// DEPYLER-1202: Substitute captured variable references with their cloned versions
    /// This creates a modified copy of the body expression with renamed variables
    fn substitute_captured_vars(
        &self,
        expr: &HirExpr,
        mappings: &std::collections::HashMap<String, String>,
    ) -> HirExpr {
        match expr {
            HirExpr::Var(name) => {
                if let Some(new_name) = mappings.get(name) {
                    HirExpr::Var(new_name.clone())
                } else {
                    expr.clone()
                }
            }
            HirExpr::Binary { left, op, right } => HirExpr::Binary {
                left: Box::new(self.substitute_captured_vars(left, mappings)),
                op: *op,
                right: Box::new(self.substitute_captured_vars(right, mappings)),
            },
            HirExpr::Unary { op, operand } => HirExpr::Unary {
                op: *op,
                operand: Box::new(self.substitute_captured_vars(operand, mappings)),
            },
            HirExpr::Call { func, args, kwargs } => HirExpr::Call {
                func: func.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_captured_vars(a, mappings))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.substitute_captured_vars(v, mappings)))
                    .collect(),
            },
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => HirExpr::MethodCall {
                object: Box::new(self.substitute_captured_vars(object, mappings)),
                method: method.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_captured_vars(a, mappings))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.substitute_captured_vars(v, mappings)))
                    .collect(),
            },
            HirExpr::Index { base, index } => HirExpr::Index {
                base: Box::new(self.substitute_captured_vars(base, mappings)),
                index: Box::new(self.substitute_captured_vars(index, mappings)),
            },
            HirExpr::Attribute { value, attr } => HirExpr::Attribute {
                value: Box::new(self.substitute_captured_vars(value, mappings)),
                attr: attr.clone(),
            },
            HirExpr::IfExpr { test, body, orelse } => HirExpr::IfExpr {
                test: Box::new(self.substitute_captured_vars(test, mappings)),
                body: Box::new(self.substitute_captured_vars(body, mappings)),
                orelse: Box::new(self.substitute_captured_vars(orelse, mappings)),
            },
            HirExpr::List(elements) => HirExpr::List(
                elements
                    .iter()
                    .map(|e| self.substitute_captured_vars(e, mappings))
                    .collect(),
            ),
            HirExpr::Tuple(elements) => HirExpr::Tuple(
                elements
                    .iter()
                    .map(|e| self.substitute_captured_vars(e, mappings))
                    .collect(),
            ),
            HirExpr::Set(elements) => HirExpr::Set(
                elements
                    .iter()
                    .map(|e| self.substitute_captured_vars(e, mappings))
                    .collect(),
            ),
            HirExpr::Dict(pairs) => HirExpr::Dict(
                pairs
                    .iter()
                    .map(|(k, v)| {
                        (
                            self.substitute_captured_vars(k, mappings),
                            self.substitute_captured_vars(v, mappings),
                        )
                    })
                    .collect(),
            ),
            // Lambda within lambda - recursively substitute, but inner lambda params shadow
            HirExpr::Lambda { params, body } => {
                // Create new mappings excluding shadowed params
                let mut inner_mappings = mappings.clone();
                for p in params {
                    inner_mappings.remove(p);
                }
                HirExpr::Lambda {
                    params: params.clone(),
                    body: Box::new(self.substitute_captured_vars(body, &inner_mappings)),
                }
            }
            // Other expression types - clone as-is (they don't contain variable references
            // or are handled through other means)
            _ => expr.clone(),
        }
    }


    pub(crate) fn convert_await(&mut self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = value.to_rust_expr(self.ctx)?;
        // DEPYLER-1024: In NASA mode, strip .await since async is converted to sync
        if self.ctx.type_mapper.nasa_mode {
            Ok(value_expr)
        } else {
            Ok(parse_quote! { #value_expr.await })
        }
    }

    pub(crate) fn convert_yield(&mut self, value: &Option<Box<HirExpr>>) -> Result<syn::Expr> {
        if self.ctx.in_generator {
            // Inside Iterator::next() - convert to return Some(value)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { return Some(#value_expr) })
            } else {
                Ok(parse_quote! { return None })
            }
        } else {
            // Outside generator context - keep as yield (placeholder for future)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { yield #value_expr })
            } else {
                Ok(parse_quote! { yield })
            }
        }
    }

    pub(crate) fn convert_fstring(&mut self, parts: &[FStringPart]) -> Result<syn::Expr> {
        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    // DEPYLER-0438/0441/0446: Smart formatting based on expression type
                    // - Collections (Vec, HashMap, HashSet): Use {:?} debug formatting
                    // - Scalars (String, i32, f64, bool): Use {} Display formatting
                    // - Option types: Unwrap with .unwrap_or_default() or display "None"
                    // This matches Python semantics where lists/dicts have their own repr
                    let arg_expr = expr.to_rust_expr(self.ctx)?;

                    // DEPYLER-0446: Check if this is an argparse Option<T> field (should be wrapped to String)
                    let is_argparse_option = match expr.as_ref() {
                        HirExpr::Attribute { value, attr } => {
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Check if this argument is optional (Option<T> type, not boolean)
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                let field_name = arg.rust_field_name();
                                                if field_name != *attr {
                                                    return false;
                                                }

                                                // Argument is NOT an Option if it has action="store_true" or "store_false"
                                                if matches!(
                                                    arg.action.as_deref(),
                                                    Some("store_true") | Some("store_false")
                                                ) {
                                                    return false;
                                                }

                                                // Argument is an Option<T> if: not required AND no default value AND not positional
                                                !arg.is_positional
                                                    && !arg.required.unwrap_or(false)
                                                    && arg.default.is_none()
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0497: Determine if expression needs {:?} Debug formatting
                    // Required for: collections, Result, Option, Vec, and any non-Display type
                    let needs_debug_fmt = match expr.as_ref() {
                        // Case 1: Simple variable (e.g., targets)
                        HirExpr::Var(var_name) => {
                            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                                // Known type: check if it needs Debug formatting
                                // DEPYLER-0712: Added Tuple - tuples don't implement Display
                                matches!(
                                    var_type,
                                    Type::List(_)
                                        | Type::Dict(_, _)
                                        | Type::Set(_)
                                        | Type::Tuple(_)     // DEPYLER-0712: Tuples need {:?}
                                        | Type::Optional(_) // DEPYLER-0497: Options need {:?}
                                )
                            } else {
                                // Unknown type defaults to {:?} because Debug is more universally
                                // implemented than Display, preventing E0277 for Option/Result/Vec
                                true
                            }
                        }
                        // DEPYLER-0497: Function calls that return Result<T> OR Option<T> need {:?}
                        HirExpr::Call { func, .. } => {
                            self.ctx.result_returning_functions.contains(func)
                                || self.ctx.option_returning_functions.contains(func)
                        }
                        // DEPYLER-0519: Method calls that return Vec types need {:?}
                        HirExpr::MethodCall { method, .. } => {
                            let vec_returning_methods = [
                                "groups",
                                "split",
                                "split_whitespace",
                                "splitlines",
                                "findall",
                                "keys",
                                "values",
                                "items",
                            ];
                            vec_returning_methods.contains(&method.as_str())
                        }
                        // Case 2: Attribute access (e.g., args.targets)
                        HirExpr::Attribute { value, attr } => {
                            // Check if this is accessing a field from argparse Args struct
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                // Check if obj_name is the args variable from ArgumentParser
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Look up the field type in argparse arguments
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                // Match field name (normalized from Python argument name)
                                                let field_name = arg.rust_field_name();
                                                if field_name == *attr {
                                                    // Check if this field is a collection type
                                                    // Either explicit type annotation OR inferred from nargs
                                                    let is_vec_from_nargs = matches!(
                                                        arg.nargs.as_deref(),
                                                        Some("+") | Some("*")
                                                    );
                                                    let is_collection_type =
                                                        if let Some(ref arg_type) = arg.arg_type {
                                                            matches!(
                                                                arg_type,
                                                                Type::List(_)
                                                                    | Type::Dict(_, _)
                                                                    | Type::Set(_)
                                                            )
                                                        } else {
                                                            false
                                                        };
                                                    is_vec_from_nargs || is_collection_type
                                                } else {
                                                    false
                                                }
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0446: Wrap argparse Option types to handle Display trait
                    // Only wrap argparse Optional fields, not regular Option variables
                    // DEPYLER-0930: Check if expression is a PathBuf type that needs .display()
                    // PathBuf doesn't implement Display, so we need to call .display() to format it
                    let is_pathbuf = match expr.as_ref() {
                        HirExpr::Var(var_name) => self
                            .ctx
                            .var_types
                            .get(var_name)
                            .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                            .unwrap_or(false),
                        HirExpr::MethodCall { method, .. } => {
                            // Methods that return PathBuf
                            matches!(
                                method.as_str(),
                                "parent" | "with_name" | "with_suffix" | "with_stem" | "join"
                            )
                        }
                        HirExpr::Attribute { value, attr } => {
                            // path.parent returns PathBuf
                            if matches!(attr.as_str(), "parent") {
                                if let HirExpr::Var(var_name) = value.as_ref() {
                                    self.ctx
                                        .var_types
                                        .get(var_name)
                                        .map(|t| {
                                            matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                                        })
                                        .unwrap_or(false)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    let final_arg = if is_argparse_option {
                        // Argparse Option<T> should display as value or "None" string
                        parse_quote! {
                            {
                                match &#arg_expr {
                                    Some(v) => format!("{}", v),
                                    None => "None".to_string(),
                                }
                            }
                        }
                    } else if is_pathbuf {
                        // DEPYLER-0930: PathBuf needs .display() to implement Display
                        parse_quote! { #arg_expr.display() }
                    } else {
                        arg_expr
                    };

                    // DEPYLER-0497: Use {:?} for non-Display types (Result, Vec, collections, Option)
                    // Use {} for Display types (primitives, String, wrapped argparse Options)
                    // DEPYLER-0930: PathBuf with .display() can use {} (Display trait)
                    if is_argparse_option || is_pathbuf {
                        // Argparse Option was wrapped to String, PathBuf has .display(), use {}
                        template.push_str("{}");
                    } else if needs_debug_fmt {
                        // Non-Display types (Vec, Result, Option, collections) need {:?}
                        template.push_str("{:?}");
                    } else {
                        // Regular Display types (i32, String, etc.)
                        template.push_str("{}");
                    }

                    args.push(final_arg);
                }
            }
        }

        // Generate format!() macro call
        if args.is_empty() {
            // No arguments (shouldn't happen but be safe)
            Ok(parse_quote! { #template.to_string() })
        } else {
            // Build the format! call with template and arguments
            Ok(parse_quote! { format!(#template, #(#args),*) })
        }
    }

    pub(crate) fn convert_ifexpr(
        &mut self,
        test: &HirExpr,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0377: Optimize `x if x else default` pattern
        // Python: `args.include if args.include else []` (check if list is non-empty)
        // Rust: Just `args.include` (clap initializes Vec to empty, so redundant check)
        // This pattern is common with argparse + Vec/Option fields
        if test == body {
            // Pattern: `x if x else y` → just use `x` (the condition is redundant)
            // This avoids type errors where Vec/Option can't be used as bool
            return body.to_rust_expr(self.ctx);
        }

        // DEPYLER-1071: Handle Option variable ternary with method call on Option
        // Pattern: `option_var.method() if option_var else default`
        // Python: `m.group(0) if m else None` where m = re.search(...)
        // Rust: `if let Some(ref m_val) = m { Some(m_val.group(0)) } else { None }`
        if let HirExpr::Var(var_name) = test {
            let is_option_var = self.is_option_variable(var_name);
            if is_option_var {
                // Check if body uses this variable in a method call
                if self.body_uses_option_var_method(body, var_name) {
                    return self.generate_option_if_let_expr(var_name, body, orelse);
                }
            }
        }

        let mut test_expr = test.to_rust_expr(self.ctx)?;
        let body_expr = body.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // DEPYLER-0377: Apply Python truthiness conversion to ternary expressions
        // Python: `val if val else default` where val is String/List/Dict/Set/Optional/Int/Float
        // Without conversion: `if val` fails (expected bool, found Vec/String/etc)
        // With conversion: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
        test_expr = Self::apply_truthiness_conversion(test, test_expr, self.ctx);

        // DEPYLER-0544: Detect File vs Stdout type mismatch
        // Python: `open(path, "w") if path else sys.stdout`
        // Rust: Needs Box<dyn Write> to unify File and Stdout types
        let body_is_file = self.is_file_creating_expr(body);
        let orelse_is_stdout = self.is_stdout_expr(orelse);
        let orelse_is_file = self.is_file_creating_expr(orelse);
        let body_is_stdout = self.is_stdout_expr(body);

        if (body_is_file && orelse_is_stdout) || (body_is_stdout && orelse_is_file) {
            // Wrap both sides in Box::new() for trait object unification
            return Ok(parse_quote! {
                if #test_expr { Box::new(#body_expr) as Box<dyn std::io::Write> } else { Box::new(#orelse_expr) }
            });
        }

        // DEPYLER-0927: Type unification for numeric IfExpr branches
        // When body returns float and orelse is integer literal, coerce orelse to float
        // Example: `dot / (norm_a * norm_b) if cond else 0` → `... else 0.0`
        let body_is_float = self.expr_returns_float(body);
        let body_is_f32 = self.expr_returns_f32(body);
        let orelse_is_int_literal = matches!(orelse, HirExpr::Literal(Literal::Int(_)));

        if body_is_float && orelse_is_int_literal {
            if let HirExpr::Literal(Literal::Int(n)) = orelse {
                let coerced_orelse: syn::Expr = if body_is_f32 {
                    let float_val = *n as f32;
                    parse_quote! { #float_val }
                } else {
                    let float_val = *n as f64;
                    parse_quote! { #float_val }
                };
                return Ok(parse_quote! {
                    if #test_expr { #body_expr } else { #coerced_orelse }
                });
            }
        }

        // DEPYLER-1085: Value Lifting for DepylerValue/concrete type mismatches
        // When one branch yields DepylerValue and the other a concrete type,
        // wrap the concrete branch in DepylerValue to unify types
        let body_is_depyler_value = self.expr_returns_depyler_value(body);
        let orelse_is_depyler_value = self.expr_returns_depyler_value(orelse);

        if body_is_depyler_value && !orelse_is_depyler_value {
            // Body is DepylerValue, orelse is concrete - lift orelse
            let lifted_orelse = self.lift_to_depyler_value(orelse, orelse_expr);
            return Ok(parse_quote! {
                if #test_expr { #body_expr } else { #lifted_orelse }
            });
        }

        if !body_is_depyler_value && orelse_is_depyler_value {
            // Orelse is DepylerValue, body is concrete - lift body
            let lifted_body = self.lift_to_depyler_value(body, body_expr);
            return Ok(parse_quote! {
                if #test_expr { #lifted_body } else { #orelse_expr }
            });
        }

        Ok(parse_quote! {
            if #test_expr { #body_expr } else { #orelse_expr }
        })
    }
    /// DEPYLER-1071: Generate `if let Some(ref val) = option_var { body } else { orelse }`
    /// with the option variable replaced by the unwrapped val in the body
    fn generate_option_if_let_expr(
        &mut self,
        var_name: &str,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        let var_ident = keywords::safe_ident(var_name);
        let val_name = format!("{}_val", var_name);
        let val_ident = keywords::safe_ident(&val_name);

        // Create a temporary context with the unwrapped variable name
        // We'll transform the body to use the unwrapped value
        let body_with_substitution = self.substitute_var_in_expr(body, var_name, &val_name);
        let body_expr = body_with_substitution.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // Check if orelse is None - if so, use Option::map pattern
        if matches!(orelse, HirExpr::Literal(Literal::None)) {
            // Pattern: `x.method() if x else None` → `x.map(|x_val| x_val.method())`
            Ok(parse_quote! {
                #var_ident.as_ref().map(|#val_ident| #body_expr)
            })
        } else {
            // Pattern: `x.method() if x else default` → `if let Some(ref x_val) = x { body } else { orelse }`
            Ok(parse_quote! {
                if let Some(ref #val_ident) = #var_ident { #body_expr } else { #orelse_expr }
            })
        }
    }

    /// DEPYLER-1071: Recursively substitute a variable name in an expression
    fn substitute_var_in_expr(&self, expr: &HirExpr, old_name: &str, new_name: &str) -> HirExpr {
        match expr {
            HirExpr::Var(name) if name == old_name => HirExpr::Var(new_name.to_string()),
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => HirExpr::MethodCall {
                object: Box::new(self.substitute_var_in_expr(object, old_name, new_name)),
                method: method.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_var_in_expr(a, old_name, new_name))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            self.substitute_var_in_expr(v, old_name, new_name),
                        )
                    })
                    .collect(),
            },
            HirExpr::Attribute { value, attr } => HirExpr::Attribute {
                value: Box::new(self.substitute_var_in_expr(value, old_name, new_name)),
                attr: attr.clone(),
            },
            HirExpr::Call { func, args, kwargs } => HirExpr::Call {
                func: func.clone(),
                args: args
                    .iter()
                    .map(|a| self.substitute_var_in_expr(a, old_name, new_name))
                    .collect(),
                kwargs: kwargs
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            self.substitute_var_in_expr(v, old_name, new_name),
                        )
                    })
                    .collect(),
            },
            // For other expression types, return as-is (could be extended if needed)
            _ => expr.clone(),
        }
    }

    /// Apply Python truthiness conversion to non-boolean conditions
    /// Python: `if val:` where val is String/List/Dict/Set/Optional/Int/Float
    /// Rust: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
    pub(crate) fn apply_truthiness_conversion(
        condition: &HirExpr,
        cond_expr: syn::Expr,
        ctx: &CodeGenContext,
    ) -> syn::Expr {
        // Check if this is a variable reference that needs truthiness conversion
        if let HirExpr::Var(var_name) = condition {
            if let Some(var_type) = ctx.var_types.get(var_name) {
                match var_type {
                    // Already boolean - no conversion needed
                    Type::Bool => return cond_expr,

                    // String/List/Dict/Set - check if empty
                    Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                        return parse_quote! { !#cond_expr.is_empty() };
                    }

                    // Optional - check if Some
                    Type::Optional(_) => {
                        return parse_quote! { #cond_expr.is_some() };
                    }

                    // Numeric types - check if non-zero
                    Type::Int => {
                        return parse_quote! { #cond_expr != 0 };
                    }
                    Type::Float => {
                        return parse_quote! { #cond_expr != 0.0 };
                    }

                    // DEPYLER-1071: Custom types that are collections
                    Type::Custom(type_name) => {
                        if is_collection_type_name(type_name) {
                            return parse_quote! { !#cond_expr.is_empty() };
                        }
                        // Fall through to heuristics
                    }

                    // DEPYLER-1071: Generic types that are collections
                    Type::Generic { base, .. } => {
                        if is_collection_generic_base(base) {
                            return parse_quote! { !#cond_expr.is_empty() };
                        }
                        // Fall through to heuristics
                    }

                    // Unknown - fall through to heuristics
                    Type::Unknown => {}

                    // Other types - fall through to heuristics
                    _ => {}
                }
            }

            // DEPYLER-1071: Heuristic fallback for common string variable names
            if is_string_var_name(var_name) {
                return parse_quote! { !#cond_expr.is_empty() };
            }

            // DEPYLER-1071: Heuristic fallback for common collection variable names
            if is_collection_var_name(var_name) {
                return parse_quote! { !#cond_expr.is_empty() };
            }

            // DEPYLER-1071: Heuristic fallback for common Option variable names
            // This handles regex match results and other optional values
            // Pattern: `if m:` where m is a regex match result (Option<Match>)
            if is_option_var_name(var_name) {
                return parse_quote! { #cond_expr.is_some() };
            }
        }

        // Not a variable or no type info - use as-is
        cond_expr
    }

    pub(crate) fn convert_sort_by_key(
        &mut self,
        iterable: &HirExpr,
        key_params: &[String],
        key_body: &HirExpr,
        reverse_expr: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let iter_expr = iterable.to_rust_expr(self.ctx)?;

        // DEPYLER-0502: Convert reverse_expr to Rust expression (supports variables and expressions)
        // If None, default to false (no reversal)
        let reverse_rust_expr = if let Some(expr) = reverse_expr {
            expr.to_rust_expr(self.ctx)?
        } else {
            parse_quote! { false }
        };

        // DEPYLER-0307: Check if this is an identity function (lambda x: x)
        // If so, use simple .sort() instead of .sort_by_key()
        let is_identity =
            key_params.len() == 1 && matches!(key_body, HirExpr::Var(v) if v == &key_params[0]);

        if is_identity {
            // Identity function: just sort() + conditional reverse()
            return Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort();
                    if #reverse_rust_expr {
                        __sorted_result.reverse();
                    }
                    __sorted_result
                }
            });
        }

        // Non-identity key function: use sort_by_key
        let body_expr = key_body.to_rust_expr(self.ctx)?;

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in sorted key lambda parameters
        let param_pat: syn::Pat = if key_params.len() == 1 {
            let param = crate::rust_gen::keywords::safe_ident(&key_params[0]);
            parse_quote! { #param }
        } else {
            bail!("sorted() key lambda must have exactly one parameter");
        };

        // DEPYLER-0502: Generate code with runtime conditional reverse
        // { let mut result = iterable.clone(); result.sort_by_key(|param| body); if reverse_expr { result.reverse(); } result }
        Ok(parse_quote! {
            {
                let mut __sorted_result = #iter_expr.clone();
                __sorted_result.sort_by_key(|#param_pat| #body_expr);
                if #reverse_rust_expr {
                    __sorted_result.reverse();
                }
                __sorted_result
            }
        })
    }

    pub(crate) fn convert_generator_expression(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Strategy: Simple cases use iterator chains, nested use flat_map

        if generators.is_empty() {
            bail!("Generator expression must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-1077: Pre-register char_iter_vars BEFORE converting element expression
            // This ensures ord(c) knows c is a char when iterating over a string
            let is_string_iter_precheck = if let HirExpr::Var(var_name) = &*gen.iter {
                self.ctx
                    .var_types
                    .get(var_name)
                    .map(|ty| matches!(ty, crate::hir::Type::String))
                    .unwrap_or(false)
            } else {
                false
            };
            if is_string_iter_precheck {
                self.ctx.char_iter_vars.insert(gen.target.clone());
            }

            // Now convert element expression (with char_iter_vars populated)
            let element_expr = element.to_rust_expr(self.ctx)?;

            // DEPYLER-0454: Detect CSV reader variables in generator expressions
            let is_csv_reader = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "reader"
                    || var_name.contains("csv")
                    || var_name.ends_with("_reader")
                    || var_name.starts_with("reader_")
            } else {
                false
            };

            // DEPYLER-0523: Detect file variables for BufReader wrapping
            // Same heuristics as stmt_gen.rs for loop file iteration
            let is_file_iter = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "f"
                    || var_name == "file"
                    || var_name == "input"
                    || var_name == "output"
                    || var_name.ends_with("_file")
                    || var_name.starts_with("file_")
            } else {
                false
            };

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr =
                if !is_csv_reader && !is_file_iter && !matches!(&*gen.iter, HirExpr::Var(_)) {
                    self.wrap_range_in_parens(iter_expr)
                } else {
                    iter_expr
                };

            // DEPYLER-0307 Fix #10: Use .iter().copied() for borrowed collections
            // DEPYLER-0454 Extension: Use .deserialize() for CSV readers
            // DEPYLER-0523: Use BufReader for file iteration
            // When the iterator is a variable (likely a borrowed parameter like &Vec<i32>),
            // use .iter().copied() to get owned values instead of references
            // This prevents type mismatches like `&i32` vs `i32` in generator expressions
            let mut chain: syn::Expr = if is_csv_reader {
                // DEPYLER-0454: CSV reader - use deserialize pattern
                self.ctx.needs_csv = true;
                parse_quote! { #iter_expr.deserialize::<std::collections::HashMap<String, String>>().filter_map(|result| result.ok()) }
            } else if is_file_iter {
                // DEPYLER-0523: File variable - use BufReader for line iteration
                self.ctx.needs_bufread = true;
                parse_quote! { std::io::BufReader::new(#iter_expr).lines().map(|l| l.unwrap_or_default()) }
            } else if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                // DEPYLER-0909: Use .cloned() instead of .copied() for compatibility with non-Copy types
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else if self.is_json_value_iteration(&gen.iter) {
                // DEPYLER-0607: JSON Value iteration in generator expression
                // serde_json::Value doesn't implement IntoIterator, must convert first
                parse_quote! { #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-1077: Check if variable is a string type - strings use .chars()
                let is_string_type = if let HirExpr::Var(var_name) = &*gen.iter {
                    self.ctx
                        .var_types
                        .get(var_name)
                        .map(|ty| matches!(ty, crate::hir::Type::String))
                        .unwrap_or(false)
                } else {
                    false
                };
                if is_string_type {
                    // DEPYLER-1077: String iteration uses .chars() not .iter()
                    // Also register target as a char iteration variable for ord() handling
                    self.ctx.char_iter_vars.insert(gen.target.clone());
                    parse_quote! { #iter_expr.chars() }
                } else {
                    // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                    parse_quote! { #iter_expr.iter().cloned() }
                }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-1079: Check if iterator is a zip() call on reference types
            // zip() on &Vec produces (&T, &T) tuples that need dereferencing for owned returns
            // Pattern: (a, b) for (a, b) in zip(list1, list2) where list1/list2 are &Vec
            let is_zip_call = matches!(&*gen.iter, HirExpr::Call { func, .. } if func == "zip");

            if is_zip_call && gen.target.contains(',') {
                // Parse target pattern to extract tuple variable names
                // Target is like "(a, b)" or "a, b" - strip parens and split
                let target_clean = gen.target.trim_start_matches('(').trim_end_matches(')');
                let vars: Vec<&str> = target_clean.split(',').map(|s| s.trim()).collect();
                if vars.len() == 2 && !vars[0].is_empty() && !vars[1].is_empty() {
                    let a = syn::Ident::new(vars[0], proc_macro2::Span::call_site());
                    let b = syn::Ident::new(vars[1], proc_macro2::Span::call_site());
                    // Add map to clone/dereference tuple elements
                    chain = parse_quote! { #chain.map(|(#a, #b)| (#a.clone(), #b.clone())) };
                }
            }

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820/1074: filter() receives &Item (even after .cloned())
            // Use |&#target_pat| to destructure the reference, getting owned value
            // This allows comparisons like x > 0 to work without type errors
            //
            // DEPYLER-1074: Register target variable's element type so numeric coercion works
            // When iterating over List[float], target x is float, so x > 0 should coerce to x > 0.0
            let element_type = if let HirExpr::Var(iter_var) = &*gen.iter {
                self.ctx.var_types.get(iter_var).and_then(|ty| match ty {
                    crate::hir::Type::List(elem) => Some(elem.as_ref().clone()),
                    crate::hir::Type::Set(elem) => Some(elem.as_ref().clone()),
                    _ => None,
                })
            } else {
                None
            };

            // Temporarily register target variable with element type for condition conversion
            let target_var_name = gen.target.clone();
            if let Some(ref elem_ty) = element_type {
                self.ctx
                    .var_types
                    .insert(target_var_name.clone(), elem_ty.clone());
            }

            // DEPYLER-1076: When function returns impl Iterator, closures need `move`
            // to take ownership of captured local variables (like min_val, factor, etc.)
            let needs_move = self.ctx.returns_impl_iterator;

            // DEPYLER-1081: Check if target is a tuple pattern
            // For tuples like (i, v), using |&(i, v)| causes E0507 for non-Copy elements
            // Instead, use |(i, v)| which receives references without trying to move
            let is_tuple_pattern = gen.target.contains(',');

            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                if is_tuple_pattern {
                    // DEPYLER-1081: Tuple patterns - use |(a, b)| to avoid move out of shared ref
                    // Rust's match ergonomics will handle &(A, B) with |(a, b)| pattern
                    if needs_move {
                        chain = parse_quote! { #chain.filter(move |#target_pat| #cond_expr) };
                    } else {
                        chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
                    }
                } else if needs_move {
                    chain = parse_quote! { #chain.filter(move |&#target_pat| #cond_expr) };
                } else {
                    chain = parse_quote! { #chain.filter(|&#target_pat| #cond_expr) };
                }
            }

            // Clean up: remove the temporary target variable
            if element_type.is_some() {
                self.ctx.var_types.remove(&target_var_name);
            }

            // Add the map transformation
            // DEPYLER-1076: Use move when returning impl Iterator
            if needs_move {
                chain = parse_quote! { #chain.map(move |#target_pat| #element_expr) };
            } else {
                chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };
            }

            return Ok(chain);
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: (x + y for x in range(3) for y in range(3))
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y))

        self.convert_nested_generators(element, generators)
    }

    pub(crate) fn convert_nested_generators(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build the nested expression recursively
        let inner_expr = self.build_nested_chain(element, generators, 1)?;

        // Start the chain with the first generator
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-1076: When function returns impl Iterator, closures need `move`
        let needs_move = self.ctx.returns_impl_iterator;

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if needs_move {
                chain = parse_quote! { #chain.filter(move |#first_pat| #cond_expr) };
            } else {
                chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
            }
        }

        // Use flat_map for the first generator
        // DEPYLER-1076: Use move when returning impl Iterator
        if needs_move {
            chain = parse_quote! { #chain.flat_map(move |#first_pat| #inner_expr) };
        } else {
            chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };
        }

        Ok(chain)
    }

    pub(crate) fn build_nested_chain(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
        depth: usize,
    ) -> Result<syn::Expr> {
        if depth >= generators.len() {
            // Base case: no more generators, return the element expression
            let element_expr = element.to_rust_expr(self.ctx)?;
            return Ok(element_expr);
        }

        let gen = &generators[depth];
        let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
        let target_pat = self.parse_target_pattern(&gen.target)?;

        // Build the inner expression (recursive)
        let inner_expr = self.build_nested_chain(element, generators, depth + 1)?;

        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let iter_expr = self.wrap_range_in_parens(iter_expr);

        // Build the chain for this level
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // DEPYLER-0691: Add filters for this generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        // DEPYLER-1076: Use move when returning impl Iterator (for captured locals)
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if self.ctx.returns_impl_iterator {
                chain = parse_quote! { #chain.filter(move |#target_pat| #cond_expr) };
            } else {
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }
        }

        // Use flat_map for intermediate generators, map for the last
        // Note: These already use `move` for capturing outer loop variables
        if depth < generators.len() - 1 {
            // Intermediate generator: use flat_map
            chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_expr) };
        } else {
            // Last generator: use map
            // DEPYLER-1082: Check if element is just the target variable (identity pattern)
            // In this case, use .copied() instead of .map(|x| x) to dereference
            // This handles (x for lst in lists for x in lst) where lst is &Vec<i32>
            let is_identity = matches!(element, HirExpr::Var(v) if v == &gen.target);
            if is_identity {
                // DEPYLER-1082: Use .copied() for primitive types to dereference
                // This converts Iterator<Item=&T> to Iterator<Item=T> for Copy types
                chain = parse_quote! { #chain.copied() };
            } else {
                chain = parse_quote! { #chain.map(move |#target_pat| #inner_expr) };
            }
        }

        Ok(chain)
    }

    pub(crate) fn parse_target_pattern(&self, target: &str) -> Result<syn::Pat> {
        // Handle simple variable: x
        // Handle tuple: (x, y)
        if target.starts_with('(') && target.ends_with(')') {
            // Tuple pattern
            let inner = &target[1..target.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            let idents: Vec<syn::Ident> = parts
                .iter()
                .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                .collect();
            Ok(parse_quote! { ( #(#idents),* ) })
        } else {
            // Simple variable
            let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            Ok(parse_quote! { #ident })
        }
    }

    /// DEPYLER-0188: Convert walrus operator (assignment expression)
    /// Python: (x := expr) assigns expr to x and evaluates to expr
    /// Rust: { let x = expr; x } - block expression that assigns and returns
    pub(crate) fn convert_named_expr(
        &mut self,
        target: &str,
        value: &HirExpr,
    ) -> Result<syn::Expr> {
        let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let value_expr = value.to_rust_expr(self.ctx)?;

        // Generate: { let target = value; target }
        // This assigns the value and returns it, matching Python's walrus semantics
        Ok(parse_quote! {
            {
                let #ident = #value_expr;
                #ident
            }
        })
    }

}

//! Unary operator conversion and main convert_call for ExpressionConverter
//!
//! Contains convert_unary, convert_call (main dispatcher), try_convert_map_with_zip,
//! and individual builtin converters (len, int_cast, float_cast, str, bool, range,
//! set, frozenset, counter, defaultdict, dict, deque, list, bytes, bytearray,
//! tuple, filter, format, ord, open, getattr).

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::array_initialization;
use crate::rust_gen::builtin_conversions;
use crate::rust_gen::collection_constructors;
use crate::rust_gen::context::CodeGenContext;
use crate::rust_gen::expr_analysis::{self, get_wrapped_chained_pyops};
use crate::rust_gen::keywords;
use crate::rust_gen::stdlib_method_gen;
use crate::string_optimization::{StringContext, StringOptimizer};
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_unary(&mut self, op: &UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        // CITL: Trace unary operation decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "unary_operation",
            chosen = &format!("{:?}", op),
            alternatives = ["not_bool", "is_empty", "is_none", "negate"],
            confidence = 0.88
        );

        let operand_expr = operand.to_rust_expr(self.ctx)?;
        match op {
            UnaryOp::Not => {
                // DEPYLER-0266: Check if operand is a collection type
                // For collections (list, dict, set, string), use .is_empty() instead of !
                // because Rust doesn't allow ! operator on non-bool types
                let is_collection = if let HirExpr::Var(var_name) = operand {
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(
                            var_type,
                            Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::String
                        )
                    } else {
                        false
                    }
                } else if let HirExpr::Attribute { value, attr } = operand {
                    // DEPYLER-0966: Check for self.field collection access (truthiness transformation)
                    // Python: `if not self.heap:` where self.heap is list[int]
                    // Rust: Must use `.is_empty()` instead of `!` for Vec types
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                            matches!(
                                field_type,
                                Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::String
                            )
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                // DEPYLER-0767: Check if operand is an Optional type variable
                // Python: `if value:` where value is Optional[T] (e.g., from os.environ.get())
                // Rust: Cannot use ! on Option<T>, need .is_none()
                let is_optional_var = if let HirExpr::Var(var_name) = operand {
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(var_type, Type::Optional(_))
                    } else {
                        false
                    }
                } else if let HirExpr::Attribute { value, attr } = operand {
                    // DEPYLER-0966: Check for self.field Optional access
                    // Python: `if not self.cached_value:` where self.cached_value is Optional[T]
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                            matches!(field_type, Type::Optional(_))
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                // DEPYLER-0443: Check if operand is a regex method call returning Option<Match>
                // Python: `if not re.match(...)` or `if not compiled.find(...)`
                // Rust: Cannot use ! on Option<Match>, need .is_none()
                let is_option_returning_call = if let HirExpr::MethodCall {
                    object: _,
                    method,
                    args: _,
                    kwargs: _,
                } = operand
                {
                    // Regex methods that return Option<Match>
                    matches!(method.as_str(), "find" | "search" | "match")
                } else if let HirExpr::Call {
                    func,
                    args: _,
                    kwargs: _,
                } = operand
                {
                    // Module-level regex functions (re.match, re.search, re.find)
                    matches!(func.as_str(), "match" | "search" | "find")
                } else {
                    false
                };

                // DEPYLER-99MODE-S9: Check if operand is a call to a user-defined
                // function in NASA mode. User functions return Result<T>, so we
                // need `?` to unwrap before applying `!`.
                let is_result_returning_call = if self.ctx.type_mapper.nasa_mode {
                    if let HirExpr::Call { func, .. } = operand {
                        self.ctx.function_return_types.contains_key(func)
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_collection {
                    Ok(parse_quote! { #operand_expr.is_empty() })
                } else if is_optional_var || is_option_returning_call {
                    // DEPYLER-0767: For Optional type variables and Option-returning methods,
                    // use .is_none() instead of !
                    Ok(parse_quote! { #operand_expr.is_none() })
                } else if is_result_returning_call {
                    let unwrapped: syn::Expr = parse_quote! { #operand_expr? };
                    Ok(parse_quote! { !#unwrapped })
                } else {
                    Ok(parse_quote! { !#operand_expr })
                }
            }
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    pub(crate) fn convert_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // CITL: Trace function call dispatch decision
        trace_decision!(
            category = DecisionCategory::MethodDispatch,
            name = "function_call",
            chosen = func,
            alternatives = ["builtin", "stdlib", "user_defined", "constructor"],
            confidence = 0.90
        );

        // DEPYLER-E0282-FIX: Handle Ok(chained_pyops) and Some(chained_pyops) type inference
        // When generating Ok(expr) or Some(expr) where expr contains chained arithmetic operations
        // like ((a).py_add(b)).py_add(c), Rust can't infer the intermediate types.
        // Fix: Generate Ok({ let _r: T = expr; _r }) to provide explicit type annotation.
        if self.ctx.type_mapper.nasa_mode && (func == "Ok" || func == "Some") && args.len() == 1 {
            // Check if the argument has chained PyOps
            if let Some(inner_expr) = get_wrapped_chained_pyops(&HirExpr::Call {
                func: func.to_string(),
                args: args.to_vec(),
                kwargs: vec![],
            }) {
                // Determine the expected type from the return type context
                let inner_type = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .and_then(|rt| match rt {
                        Type::Optional(inner) => Some(inner.as_ref()),
                        _ => None,
                    });
                // Also check for explicit type hints on the inner expression
                let ty_tokens: Option<syn::Type> = match inner_type {
                    Some(Type::Int) => Some(parse_quote! { i32 }),
                    Some(Type::Float) => Some(parse_quote! { f64 }),
                    _ => {
                        // Fallback: check if inner expr is arithmetic, use i32 as default
                        if matches!(inner_expr, HirExpr::Binary { .. }) {
                            Some(parse_quote! { i32 })
                        } else {
                            None
                        }
                    }
                };
                if let Some(ty) = ty_tokens {
                    let inner_tokens = inner_expr.to_rust_expr(self.ctx)?;
                    let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #func_ident({ let _r: #ty = #inner_tokens; _r }) });
                }
            }
        }

        // DEPYLER-0608: Transform calls to cmd_*/handle_* handlers in subcommand match arms
        // When calling a handler with `args`, pass the extracted subcommand fields instead
        // Pattern: cmd_list(args) → cmd_list(archive) (where archive is extracted in match pattern)
        if self.ctx.in_subcommand_match_arm
            && (func.starts_with("cmd_") || func.starts_with("handle_"))
            && args.len() == 1
            && matches!(&args[0], HirExpr::Var(v) if v == "args")
        {
            let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
            let field_args: Vec<syn::Expr> = self
                .ctx
                .subcommand_match_fields
                .iter()
                .map(|f| {
                    let field_ident = syn::Ident::new(f, proc_macro2::Span::call_site());
                    parse_quote! { #field_ident }
                })
                .collect();
            return Ok(parse_quote! { #func_ident(#(#field_args),*) });
        }

        // DEPYLER-0382: Handle os.path.join(*parts) starred unpacking
        if func == "__os_path_join_starred" {
            if args.len() != 1 {
                bail!("__os_path_join_starred expects exactly 1 argument");
            }
            let parts = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! {
                if #parts.is_empty() {
                    String::new()
                } else {
                    #parts.join(std::path::MAIN_SEPARATOR_STR)
                }
            });
        }

        // DEPYLER-0382: Handle print(*items) starred unpacking
        if func == "__print_starred" {
            if args.len() != 1 {
                bail!("__print_starred expects exactly 1 argument");
            }
            let items = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! {
                {
                    for item in #items {
                        print!("{} ", item);
                    }
                    println!();
                }
            });
        }

        // DEPYLER-REFACTOR-001 Phase 2.14: Removed redundant zeros/ones/full early handlers
        // These are now handled by the final match block via convert_array_init_call
        // which delegates to array_initialization module for consistent handling

        // DEPYLER-0363: Handle ArgumentParser() → Skip for now, will be replaced with struct generation
        // ArgumentParser pattern requires complex transformation:
        // - Accumulate add_argument() calls
        // - Generate #[derive(Parser)] struct
        // - Replace parse_args() with Args::parse()
        // For now, return unit to make code compile while transformation is implemented
        if func.contains("ArgumentParser") {
            // NOTE: Full argparse implementation requires generating Args struct with clap derives (tracked in DEPYLER-0363)
            // For now, just return unit to allow compilation
            return Ok(parse_quote! { () });
        }

        // Handle classmethod cls(args) → Self::new(args)
        if func == "cls" && self.ctx.is_classmethod {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return Ok(parse_quote! { Self::new(#(#arg_exprs),*) });
        }

        // Handle map() with lambda → convert to Rust iterator pattern
        if func == "map" && args.len() >= 2 {
            if let Some(result) = self.try_convert_map_with_zip(args)? {
                return Ok(result);
            }
        }

        // DEPYLER-0178: Handle filter() with lambda → convert to Rust iterator pattern
        // DEPYLER-0754: Use .iter().cloned() instead of .into_iter() to produce Vec<T> not Vec<&T>
        // When iterable is &Vec<T>, .into_iter() yields &T references, causing type mismatch.
        // .iter().cloned() properly clones elements to produce owned iterator.
        if func == "filter" && args.len() == 2 {
            if let HirExpr::Lambda { params, body } = &args[0] {
                if params.len() != 1 {
                    bail!("filter() lambda must have exactly one parameter");
                }
                let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
                let param_ident = crate::rust_gen::keywords::safe_ident(&params[0]);

                // DEPYLER-1053: Infer element type from iterable and add lambda param to var_types
                // This enables type coercion in comparisons like `x != 0` where x is f64
                let elem_type = self.infer_iterable_element_type(&args[1]);
                let param_name = params[0].clone();
                if let Some(ref elem_t) = elem_type {
                    self.ctx
                        .var_types
                        .insert(param_name.clone(), elem_t.clone());
                }

                let body_expr = body.to_rust_expr(self.ctx)?;

                // DEPYLER-1053: Remove lambda param from var_types to avoid polluting context
                if elem_type.is_some() {
                    self.ctx.var_types.remove(&param_name);
                }

                // DEPYLER-1053: Use |&x| pattern because filter() always receives &Item
                // Even with .cloned(), filter's closure parameter is a reference to the owned value
                return Ok(parse_quote! {
                    #iterable_expr.iter().cloned().filter(|&#param_ident| #body_expr)
                });
            }
        }

        // DEPYLER-REFACTOR-001 Phase 2.18: Delegate sum calls to helper
        if let Some(result) = self.try_convert_sum_call(func, args) {
            return result;
        }

        // DEPYLER-0950: Handle max(generator_exp) → generator_exp.max().unwrap_or_default()
        // Iterator::max() returns Option<T>, must unwrap for use in ranges/arithmetic
        if func == "max" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.max().unwrap_or_default() });
        }

        // DEPYLER-0950: Handle min(generator_exp) → generator_exp.min().unwrap_or_default()
        if func == "min" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.min().unwrap_or_default() });
        }

        // DEPYLER-REFACTOR-001: sorted() and reversed() handlers consolidated
        // to final match block using convert_sorted_builtin/convert_reversed_builtin

        // DEPYLER-0022: Handle memoryview(data) → data (identity/no-op)
        // Rust byte slices (&[u8]) already provide memoryview functionality (zero-copy view)
        // Python's memoryview provides a buffer interface - Rust slices are already references
        if func == "memoryview" && args.len() == 1 {
            return args[0].to_rust_expr(self.ctx);
        }

        // DEPYLER-REFACTOR-001 Phase 2.18: sum handlers removed - now handled by try_convert_sum_call

        // DEPYLER-REFACTOR-001 Phase 2.19: Delegate min/max calls to helper
        if let Some(result) = self.try_convert_minmax_call(func, args) {
            return result;
        }

        // DEPYLER-0248: Handle abs(value) → (value).abs()
        // DEPYLER-0815: Parens required for correct precedence (abs(n - 10) → (n - 10).abs())
        if func == "abs" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { (#value_expr).abs() });
        }

        // DEPYLER-REFACTOR-001 Phase 2.20: Delegate any/all calls to helper
        if let Some(result) = self.try_convert_any_all_call(func, args) {
            return result;
        }

        // DEPYLER-0251: Handle round(value) → value.round() as i32
        // DEPYLER-0357: Add `as i32` cast because Python round() returns int
        // but Rust f64::round() returns f64
        if func == "round" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #value_expr.round() as i32 });
        }

        // DEPYLER-0252: Handle pow(base, exp) → base.pow(exp as u32)
        // Rust's pow() requires u32 exponent, so we cast
        if func == "pow" && args.len() == 2 {
            let base_expr = args[0].to_rust_expr(self.ctx)?;
            let exp_expr = args[1].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #base_expr.pow(#exp_expr as u32) });
        }

        // DEPYLER-REFACTOR-001: chr() and ord() handlers consolidated
        // to final match block using convert_chr_builtin/convert_ord_builtin

        // DEPYLER-0255: Handle bool(value) → type-aware truthiness check
        // DEPYLER-REFACTOR-001: Handles different types correctly
        if func == "bool" && args.len() == 1 {
            let arg = &args[0];
            match arg {
                // String literals: non-empty → true, empty → false
                HirExpr::Literal(Literal::String(s)) => {
                    let is_true = !s.is_empty();
                    return Ok(parse_quote! { #is_true });
                }
                // Integer literals: non-zero → true, zero → false
                HirExpr::Literal(Literal::Int(n)) => {
                    let is_true = *n != 0;
                    return Ok(parse_quote! { #is_true });
                }
                // Float literals: non-zero → true, zero → false
                HirExpr::Literal(Literal::Float(f)) => {
                    let is_true = *f != 0.0;
                    return Ok(parse_quote! { #is_true });
                }
                // Bool literals: identity
                HirExpr::Literal(Literal::Bool(b)) => {
                    return Ok(parse_quote! { #b });
                }
                // Variables: check type
                HirExpr::Var(var_name) => {
                    let value_expr = arg.to_rust_expr(self.ctx)?;
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        return match var_type {
                            Type::String => Ok(parse_quote! { !#value_expr.is_empty() }),
                            Type::Float => Ok(parse_quote! { #value_expr != 0.0 }),
                            Type::List(_) | Type::Set(_) | Type::Dict(_, _) => {
                                Ok(parse_quote! { !#value_expr.is_empty() })
                            }
                            _ => Ok(parse_quote! { #value_expr != 0 }),
                        };
                    }
                    // Default for unknown variables: assume integer-like
                    return Ok(parse_quote! { #value_expr != 0 });
                }
                // Other expressions: default to != 0
                _ => {
                    let value_expr = arg.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { #value_expr != 0 });
                }
            }
        }

        // DEPYLER-REFACTOR-001 Phase 2.12: Delegate numeric type constructors to helper
        // Handles: Decimal, Fraction
        if let Some(result) = self.try_convert_numeric_type_call(func, args) {
            return result;
        }

        // DEPYLER-REFACTOR-001 Phase 2.11: Delegate stdlib type constructors to helper
        // Handles: Path, datetime, date, time, timedelta
        if let Some(result) = self.try_convert_stdlib_type_call(func, args) {
            return result;
        }

        // DEPYLER-REFACTOR-001 Phase 2.13: Delegate iterator utility calls to helper
        // Handles enumerate, zip, isinstance
        if let Some(result) = self.try_convert_iterator_util_call(func, args) {
            return result;
        }

        // DEPYLER-0230: Check if func is a user-defined class before treating as builtin
        let is_user_class = self.ctx.class_names.contains(func);

        // DEPYLER-0234: For user-defined class constructors, convert string literals to String
        // This fixes "expected String, found &str" errors when calling constructors
        // DEPYLER-1144: Also coerce list literals when class has Vec<f64> field
        // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
        let class_has_vec_f64_field = self
            .ctx
            .class_field_types
            .values()
            .any(|t| matches!(t, Type::List(elem) if matches!(**elem, Type::Float)));
        let arg_exprs: Vec<syn::Expr> = if is_user_class {
            args.iter()
                .map(|arg| {
                    // DEPYLER-1144: For list literals when class expects Vec<f64>, coerce integers
                    if class_has_vec_f64_field {
                        if let HirExpr::List(elems) = arg {
                            return self.convert_list_with_float_coercion(elems);
                        }
                    }
                    // DEPYLER-CLASS-STR-FIX: Add .to_string() for string literals in class constructors
                    // Python dataclass fields with `name: str` are owned Strings in Rust.
                    // String literals need .to_string() to convert &str to String.
                    if let HirExpr::Literal(Literal::String(s)) = arg {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        return Ok(parse_quote! { #lit.to_string() });
                    }
                    arg.to_rust_expr(self.ctx)
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            // DEPYLER-1215/DEPYLER-1218: Convert args with type context for dict and list literals
            // When a dict/list literal is passed to a function expecting Dict/List with Unknown/Any,
            // we need to set current_assign_type to trigger DepylerValue wrapping
            let mut converted_args = Vec::with_capacity(args.len());
            for (param_idx, arg) in args.iter().enumerate() {
                // DEPYLER-1215/DEPYLER-1218: Check if param expects Dict/List with Unknown/Any value type
                let prev_assign_type = if matches!(arg, HirExpr::Dict(_) | HirExpr::List(_)) {
                    if let Some(param_types) = self.ctx.function_param_types.get(func) {
                        if let Some(param_type) = param_types.get(param_idx) {
                            // Check if param is Dict[_, Any/Unknown] or List[Any/Unknown] or bare dict/list
                            let needs_depyler_value = match param_type {
                                Type::Dict(_, val_type) => {
                                    matches!(val_type.as_ref(), Type::Unknown)
                                        || matches!(val_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any")
                                }
                                // DEPYLER-1218: List with Unknown/DepylerValue element type
                                Type::List(elem_type) => {
                                    matches!(elem_type.as_ref(), Type::Unknown)
                                        || matches!(elem_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any")
                                }
                                Type::Custom(name) if name == "dict" || name == "Dict" => true,
                                Type::Custom(name) if name == "list" || name == "List" => true,
                                _ => false,
                            };
                            if needs_depyler_value {
                                let old = self.ctx.current_assign_type.clone();
                                self.ctx.current_assign_type = Some(param_type.clone());
                                Some(old)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let expr = arg.to_rust_expr(self.ctx)?;

                // Restore previous assign type if we changed it
                if let Some(old_type) = prev_assign_type {
                    self.ctx.current_assign_type = old_type;
                }

                // DEPYLER-0458: Add & prefix for Lazy const variables (e.g., DEFAULT_CONFIG)
                // When passing a const (all uppercase) to a function, it's likely a Lazy<T>
                // that needs to be borrowed (&) so Deref converts it to &T
                if let HirExpr::Var(var_name) = arg {
                    let is_const = var_name.chars().all(|c| c.is_uppercase() || c == '_');
                    if is_const {
                        converted_args.push(parse_quote! { &#expr });
                        continue;
                    }
                }
                converted_args.push(expr);
            }
            converted_args
        };

        // DEPYLER-0364: Convert kwargs to positional arguments
        // Python: greet(name="Alice", greeting="Hello") → Rust: greet("Alice", "Hello")
        // For now, we append kwargs as additional positional arguments. This works for
        // common cases where functions accept positional or keyword arguments in order.
        // DEPYLER-0477: Future work - look up function signatures to determine
        // the correct parameter order and merge positional + kwargs properly
        let kwarg_exprs: Vec<syn::Expr> = if is_user_class {
            // For user-defined classes, convert string literals to String
            // This prevents "expected String, found &str" errors in constructors
            // DEPYLER-1144: Also coerce list literals to match field types (e.g., [0] → vec![0.0] for Vec<f64>)
            kwargs
                .iter()
                .map(|(name, value)| {
                    // DEPYLER-1144: Check if field expects Vec<f64> and value is list of integers
                    if let Some(Type::List(elem_type)) = self.ctx.class_field_types.get(name) {
                        if matches!(elem_type.as_ref(), Type::Float) {
                            if let HirExpr::List(elems) = value {
                                // Convert list with integer coercion to f64
                                return self.convert_list_with_float_coercion(elems);
                            }
                        }
                    }
                    let expr = value.to_rust_expr(self.ctx)?;
                    if matches!(value, HirExpr::Literal(Literal::String(_))) {
                        Ok(parse_quote! { #expr.to_string() })
                    } else {
                        Ok(expr)
                    }
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            // For built-in functions and regular calls, use standard conversion
            kwargs
                .iter()
                .map(|(_name, value)| value.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?
        };

        // Merge positional args and kwargs (both HIR and converted Rust exprs)
        // This creates a single argument list that will be passed to the function
        let mut all_args = arg_exprs.clone();
        all_args.extend(kwarg_exprs);

        let mut all_hir_args: Vec<HirExpr> = args.to_vec();
        for (_name, value) in kwargs {
            all_hir_args.push(value.clone());
        }

        // DEPYLER-REFACTOR-001 Phase 2.17: Delegate print call to helper
        if let Some(result) = self.try_convert_print_call(func, args, &arg_exprs, kwargs) {
            return result;
        }

        match func {
            // Python built-in type conversions → Rust casting
            "int" => self.convert_int_cast(&all_hir_args, &arg_exprs),
            "float" => self.convert_float_cast(&all_hir_args, &arg_exprs),
            "str" => self.convert_str_conversion(&all_hir_args, &arg_exprs),
            "bool" => self.convert_bool_cast(&all_hir_args, &arg_exprs),
            // Other built-in functions
            // DEPYLER-0659: Handle len() on serde_json::Value
            "len" => self.convert_len_call_with_type(&all_hir_args, &arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => {
                self.convert_array_init_call(func, &all_hir_args, &arg_exprs)
            }
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            // DEPYLER-0171, 0172, 0173, 0174: Collection conversion builtins
            // DEPYLER-0230: Only treat as builtin if not a user-defined class
            // DEPYLER-0751: Pass HIR args to detect string type for .chars()
            "Counter" if !is_user_class => self.convert_counter_builtin(&all_hir_args, &arg_exprs),
            "defaultdict" if !is_user_class => self.convert_defaultdict_builtin(&arg_exprs),
            "dict" if !is_user_class => self.convert_dict_builtin(&arg_exprs),
            "deque" if !is_user_class => self.convert_deque_builtin(&arg_exprs),
            "list" if !is_user_class => self.convert_list_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-0935: bytes() builtin - convert to Vec<u8>
            "bytes" if !is_user_class => self.convert_bytes_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-0936: bytearray() builtin - convert to Vec<u8>
            "bytearray" if !is_user_class => {
                self.convert_bytearray_builtin(&all_hir_args, &arg_exprs)
            }
            // DEPYLER-0937: tuple() builtin - convert iterable to collected tuple-like Vec
            "tuple" if !is_user_class => self.convert_tuple_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-STDLIB-BUILTINS: Pure builtin functions delegated to extracted module
            // DEPYLER-COVERAGE-95: Extracted to stdlib_method_gen::builtin_functions for testability
            "all" => stdlib_method_gen::builtin_functions::convert_all_builtin(&arg_exprs),
            "any" => stdlib_method_gen::builtin_functions::convert_any_builtin(&arg_exprs),
            "divmod" => stdlib_method_gen::builtin_functions::convert_divmod_builtin(&arg_exprs),
            "enumerate" => {
                stdlib_method_gen::builtin_functions::convert_enumerate_builtin(&arg_exprs)
            }
            "zip" => stdlib_method_gen::builtin_functions::convert_zip_builtin(&arg_exprs),
            "reversed" => {
                stdlib_method_gen::builtin_functions::convert_reversed_builtin(&arg_exprs)
            }
            "sorted" => stdlib_method_gen::builtin_functions::convert_sorted_builtin(&arg_exprs),
            "filter" => self.convert_filter_builtin(&all_hir_args, &arg_exprs),
            "sum" => stdlib_method_gen::builtin_functions::convert_sum_builtin(&arg_exprs),
            // DEPYLER-STDLIB-BUILTINS: Final batch for 50% milestone
            "round" => stdlib_method_gen::builtin_functions::convert_round_builtin(&arg_exprs),
            "abs" => stdlib_method_gen::builtin_functions::convert_abs_builtin(&arg_exprs),
            "min" => stdlib_method_gen::builtin_functions::convert_min_builtin(&arg_exprs),
            "max" => stdlib_method_gen::builtin_functions::convert_max_builtin(&arg_exprs),
            "pow" => stdlib_method_gen::builtin_functions::convert_pow_builtin(&arg_exprs),
            "hex" => stdlib_method_gen::builtin_functions::convert_hex_builtin(&arg_exprs),
            "bin" => stdlib_method_gen::builtin_functions::convert_bin_builtin(&arg_exprs),
            "oct" => stdlib_method_gen::builtin_functions::convert_oct_builtin(&arg_exprs),
            // DEPYLER-0579: format(value, spec) builtin - needs HIR for literal extraction
            "format" => self.convert_format_builtin(&arg_exprs, &all_hir_args),
            "chr" => stdlib_method_gen::builtin_functions::convert_chr_builtin(&arg_exprs),
            // ord() needs context for char_iter_vars check
            "ord" => self.convert_ord_builtin(&arg_exprs, &all_hir_args),
            "hash" => stdlib_method_gen::builtin_functions::convert_hash_builtin(&arg_exprs),
            "repr" => stdlib_method_gen::builtin_functions::convert_repr_builtin(&arg_exprs),
            // DEPYLER-0387: File I/O builtin - needs context for needs_io_* flags
            "open" => self.convert_open_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-STDLIB-50: next(), getattr(), iter(), type()
            "next" => stdlib_method_gen::builtin_functions::convert_next_builtin(&arg_exprs),
            "getattr" => self.convert_getattr_builtin(&arg_exprs),
            "iter" => stdlib_method_gen::builtin_functions::convert_iter_builtin(&arg_exprs),
            "type" => stdlib_method_gen::builtin_functions::convert_type_builtin(&arg_exprs),
            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            "isinstance" => Ok(parse_quote! { true }),
            // DEPYLER-1205: E0425 Vocabulary Expansion - input(), hasattr()
            "input" => stdlib_method_gen::builtin_functions::convert_input_builtin(&arg_exprs),
            "hasattr" => stdlib_method_gen::builtin_functions::convert_hasattr_builtin(&arg_exprs),
            "setattr" => stdlib_method_gen::builtin_functions::convert_setattr_builtin(&arg_exprs),
            // GH-204: Additional E0425 Vocabulary Expansion
            "callable" => {
                stdlib_method_gen::builtin_functions::convert_callable_builtin(&arg_exprs)
            }
            "id" => stdlib_method_gen::builtin_functions::convert_id_builtin(&arg_exprs),
            "ascii" => stdlib_method_gen::builtin_functions::convert_ascii_builtin(&arg_exprs),
            "vars" => stdlib_method_gen::builtin_functions::convert_vars_builtin(&arg_exprs),
            "dir" => stdlib_method_gen::builtin_functions::convert_dir_builtin(&arg_exprs),
            "globals" => stdlib_method_gen::builtin_functions::convert_globals_builtin(&arg_exprs),
            "locals" => stdlib_method_gen::builtin_functions::convert_locals_builtin(&arg_exprs),
            "delattr" => stdlib_method_gen::builtin_functions::convert_delattr_builtin(&arg_exprs),
            "staticmethod" => {
                stdlib_method_gen::builtin_functions::convert_staticmethod_builtin(&arg_exprs)
            }
            "classmethod" => {
                stdlib_method_gen::builtin_functions::convert_classmethod_builtin(&arg_exprs)
            }
            "property" => {
                stdlib_method_gen::builtin_functions::convert_property_builtin(&arg_exprs)
            }
            "breakpoint" => {
                stdlib_method_gen::builtin_functions::convert_breakpoint_builtin(&arg_exprs)
            }
            "exit" => stdlib_method_gen::builtin_functions::convert_exit_builtin(&arg_exprs),
            "quit" => stdlib_method_gen::builtin_functions::convert_quit_builtin(&arg_exprs),
            _ => self.convert_generic_call(func, &all_hir_args, &all_args),
        }
    }

    pub(crate) fn try_convert_map_with_zip(
        &mut self,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // DEPYLER-0793: Handle map(str, iterable) → iterable.iter().map(|x| x.to_string())
        // Python's str builtin converts elements to strings, in Rust use .to_string()
        if args.len() == 2 {
            if let HirExpr::Var(func_name) = &args[0] {
                if func_name == "str" {
                    let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                    return Ok(Some(parse_quote! {
                        #iterable_expr.iter().map(|x| x.to_string())
                    }));
                }
                // DEPYLER-0793: Handle map(int, iterable) → iterable.iter().map(|x| *x as i32)
                // For converting strings to int, this is a simplified version
                if func_name == "int" {
                    let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                    return Ok(Some(parse_quote! {
                        #iterable_expr.iter().filter_map(|x| x.parse::<i32>().ok())
                    }));
                }
                // DEPYLER-0793: Handle map(float, iterable) → iterable.iter().filter_map(|x| x.parse::<f64>().ok())
                if func_name == "float" {
                    let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                    return Ok(Some(parse_quote! {
                        #iterable_expr.iter().filter_map(|x| x.parse::<f64>().ok())
                    }));
                }
            }
        }

        // Check if first argument is a lambda
        if let HirExpr::Lambda { params, body } = &args[0] {
            let num_iterables = args.len() - 1;

            // Check if lambda has matching number of parameters
            if params.len() != num_iterables {
                bail!(
                    "Lambda has {} parameters but map() called with {} iterables",
                    params.len(),
                    num_iterables
                );
            }

            // Convert the iterables
            let mut iterable_exprs: Vec<syn::Expr> = Vec::new();
            for iterable in &args[1..] {
                iterable_exprs.push(iterable.to_rust_expr(self.ctx)?);
            }

            // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
            let param_idents: Vec<syn::Ident> = params
                .iter()
                .map(|p| crate::rust_gen::keywords::safe_ident(p))
                .collect();

            // DEPYLER-1053: Infer element types from iterables and add lambda params to var_types
            // This enables type coercion in comparisons like `x > 0` where x is f64
            let mut added_params: Vec<String> = Vec::new();
            for (i, param) in params.iter().enumerate() {
                if let Some(iterable) = args.get(i + 1) {
                    if let Some(elem_type) = self.infer_iterable_element_type(iterable) {
                        self.ctx.var_types.insert(param.clone(), elem_type);
                        added_params.push(param.clone());
                    }
                }
            }

            // Convert lambda body
            let body_expr = body.to_rust_expr(self.ctx)?;

            // DEPYLER-1053: Remove lambda params from var_types to avoid polluting context
            for param in &added_params {
                self.ctx.var_types.remove(param);
            }

            // Handle based on number of iterables
            if num_iterables == 1 {
                // Single iterable: iterable.iter().map(|&x| ...).collect()
                // DEPYLER-1053: Use |&x| pattern because iter() yields references
                let iter_expr = &iterable_exprs[0];
                let param = &param_idents[0];
                Ok(Some(parse_quote! {
                    #iter_expr.iter().map(|&#param| #body_expr).collect::<Vec<_>>()
                }))
            } else {
                // Multiple iterables: use zip pattern
                // Build the zip chain
                let first_iter = &iterable_exprs[0];
                let mut zip_expr: syn::Expr = parse_quote! { #first_iter.iter() };

                for iter_expr in &iterable_exprs[1..] {
                    zip_expr = parse_quote! { #zip_expr.zip(#iter_expr.iter()) };
                }

                // Build the tuple pattern based on number of parameters
                let tuple_pat: syn::Pat = if param_idents.len() == 2 {
                    let p0 = &param_idents[0];
                    let p1 = &param_idents[1];
                    parse_quote! { (#p0, #p1) }
                } else if param_idents.len() == 3 {
                    // For 3 parameters, zip creates ((a, b), c)
                    let p0 = &param_idents[0];
                    let p1 = &param_idents[1];
                    let p2 = &param_idents[2];
                    parse_quote! { ((#p0, #p1), #p2) }
                } else {
                    // For 4+ parameters, continue the nested pattern
                    bail!("map() with more than 3 iterables is not yet supported");
                };

                // Generate the final expression
                Ok(Some(parse_quote! {
                    #zip_expr.map(|#tuple_pat| #body_expr).collect::<Vec<_>>()
                }))
            }
        } else {
            // Not a lambda, fall through to normal handling
            Ok(None)
        }
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    #[allow(dead_code)]
    pub(crate) fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        builtin_conversions::convert_len_call(args)
    }

    /// DEPYLER-0659: Handle len() with type awareness for serde_json::Value
    /// serde_json::Value doesn't have a direct .len() method
    /// - Arrays: use .as_array().map(|a| a.len()).unwrap_or(0)
    /// - Objects: use .as_object().map(|o| o.len()).unwrap_or(0)
    /// - Strings: use .as_str().map(|s| s.len()).unwrap_or(0)
    ///
    /// DEPYLER-DAY2-BUG-002: Handle len() on tuples
    /// Rust tuples don't have .len() method - size is known at compile time
    /// - Tuples: return compile-time constant (e.g., 4 for (i32, i32, i32, i32))
    pub(crate) fn convert_len_call_with_type(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 || hir_args.is_empty() {
            return builtin_conversions::convert_len_call(arg_exprs);
        }

        let arg = &arg_exprs[0];
        let hir_arg = &hir_args[0];

        // DEPYLER-DAY2-BUG-002: Check if argument is a tuple type
        // Rust tuples don't have .len() - return compile-time constant
        if let HirExpr::Var(name) = hir_arg {
            if let Some(Type::Tuple(types)) = self.ctx.var_types.get(name) {
                let len = types.len() as i32;
                return Ok(parse_quote! { #len });
            }
        }

        // Check if the argument is a JSON Value (NOT a typed HashMap)
        // DEPYLER-0689: Only use as_array/as_object for serde_json::Value, not typed dicts
        // Typed dicts like dict[str, int] map to HashMap which has direct .len()
        if self.is_serde_json_value_expr(hir_arg) {
            // For JSON arrays: .as_array().map(|a| a.len()).unwrap_or(0)
            // This also works for objects and is the most common case
            Ok(parse_quote! {
                #arg.as_array().map(|a| a.len()).unwrap_or_else(||
                    #arg.as_object().map(|o| o.len()).unwrap_or(0)
                ) as i32
            })
        } else {
            // Default behavior for other types
            builtin_conversions::convert_len_call(arg_exprs)
        }
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    pub(crate) fn convert_int_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_int_cast(
            self.ctx,
            hir_args,
            arg_exprs,
            |obj, method, args| {
                builtin_conversions::is_string_method_call(self.ctx, obj, method, args)
            },
            builtin_conversions::is_bool_expr,
        )
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    pub(crate) fn convert_float_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_float_cast(self.ctx, hir_args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    /// DEPYLER-0188: Pass HirExpr to detect PathBuf for .display().to_string()
    /// DEPYLER-0722: Handle Option<T> types - use .unwrap().to_string()
    /// GH-207: Don't add .unwrap() if expression already has .unwrap_or()
    pub(crate) fn convert_str_conversion(
        &self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0722: Check if argument is an Optional type
        if !hir_args.is_empty() && args.len() == 1 {
            let var_name = match &hir_args[0] {
                HirExpr::Var(name) => Some(name.as_str()),
                HirExpr::Attribute { attr, .. } => Some(attr.as_str()),
                _ => None,
            };
            if let Some(name) = var_name {
                if let Some(Type::Optional(_)) = self.ctx.var_types.get(name) {
                    let arg = &args[0];
                    // GH-207: Check if the arg already contains unwrap_or - if so, it's already
                    // unwrapped and we shouldn't add another .unwrap()
                    let arg_str = quote::quote!(#arg).to_string();
                    if arg_str.contains("unwrap_or") {
                        // Already unwrapped via unwrap_or - just call .to_string()
                        return Ok(parse_quote! { (#arg).to_string() });
                    }
                    return Ok(parse_quote! { (#arg).expect("value is None").to_string() });
                }
            }
        }
        builtin_conversions::convert_str_conversion(hir_args, args, |e| self.is_path_expr(e))
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    pub(crate) fn convert_bool_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_bool_cast(self.ctx, hir_args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to array_initialization module
    pub(crate) fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        array_initialization::convert_range_call(args)
    }

    /// DEPYLER-REFACTOR-001: Delegated to array_initialization module
    pub(crate) fn convert_array_init_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        array_initialization::convert_array_init_call(self.ctx, func, args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    pub(crate) fn convert_set_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_set_constructor(self.ctx, args)
    }

    /// DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    pub(crate) fn convert_frozenset_constructor(
        &mut self,
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        collection_constructors::convert_frozenset_constructor(self.ctx, args)
    }

    // ========================================================================
    // DEPYLER-0171, 0172, 0173, 0174: Collection Conversion Builtins
    // DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    // ========================================================================

    /// DEPYLER-0751: Handle Counter(string) by using .chars() instead of .into_iter()
    pub(crate) fn convert_counter_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        self.ctx.needs_hashmap = true;
        // DEPYLER-0751: Handle Counter(string) → string.chars().fold(...)
        // String doesn't implement IntoIterator, need to use .chars()
        if hir_args.len() == 1 && args.len() == 1 {
            let hir_arg = &hir_args[0];
            let is_string = self.is_string_type(hir_arg)
                || matches!(
                    hir_arg,
                    HirExpr::Var(name) if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::String))
                );
            if is_string {
                let arg = &args[0];
                return Ok(parse_quote! {
                    #arg.chars().fold(HashMap::new(), |mut acc, item| {
                        *acc.entry(item).or_insert(0) += 1;
                        acc
                    })
                });
            }
        }
        collection_constructors::convert_counter_builtin(self.ctx, args)
    }

    pub(crate) fn convert_defaultdict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_defaultdict_builtin(self.ctx, args)
    }

    pub(crate) fn convert_dict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_dict_builtin(self.ctx, args)
    }

    pub(crate) fn convert_deque_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_deque_builtin(self.ctx, args)
    }

    pub(crate) fn convert_list_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0651: Handle list(string) → string.chars().collect()
        // String doesn't implement IntoIterator, need to use .chars()
        if hir_args.len() == 1 && args.len() == 1 {
            let hir_arg = &hir_args[0];
            let is_string = self.is_string_type(hir_arg)
                || matches!(
                    hir_arg,
                    HirExpr::Var(name) if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::String))
                );
            if is_string {
                let arg = &args[0];
                return Ok(parse_quote! { #arg.chars().collect::<Vec<_>>() });
            }
        }
        collection_constructors::convert_list_builtin(self.ctx, args)
    }

    /// DEPYLER-0935: Convert Python bytes() constructor to Vec<u8>
    /// bytes() → Vec::<u8>::new()
    /// bytes(n) → vec![0u8; n]
    /// bytes([1, 2, 3]) → vec![1u8, 2u8, 3u8]
    /// bytes(string) → string.as_bytes().to_vec()
    pub(crate) fn convert_bytes_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytes() → Vec::<u8>::new()
            return Ok(parse_quote! { Vec::<u8>::new() });
        }

        if args.len() == 1 {
            let hir_arg = &hir_args[0];
            let arg = &args[0];

            // bytes([1, 2, 3]) → list collected as Vec<u8>
            if matches!(hir_arg, HirExpr::List { .. }) {
                return Ok(parse_quote! { #arg.into_iter().map(|x| x as u8).collect::<Vec<u8>>() });
            }

            // bytes(string) → string.as_bytes().to_vec()
            if self.is_string_type(hir_arg) {
                return Ok(parse_quote! { #arg.as_bytes().to_vec() });
            }

            // bytes(bytearray_or_bytes) → just return the bytes/bytearray variable
            // Check if arg is a variable with list type (bytearray is Vec<u8> = List)
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::List(_)))
                {
                    return Ok(parse_quote! { #arg });
                }
            }

            // DEPYLER-0935: bytes(n) where n is numeric expression → vec![0u8; n as usize]
            // Check for int literal first
            if matches!(hir_arg, HirExpr::Literal(crate::hir::Literal::Int(_))) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Check for int variable
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::Int))
                {
                    return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
                }
            }

            // For method calls like .len(), assume they return size
            if matches!(hir_arg, HirExpr::MethodCall { .. }) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Default: assume it's a collection/bytes that should be returned as-is
            // This handles bytes(some_bytearray) → some_bytearray
            return Ok(parse_quote! { #arg });
        }

        // bytes with encoding args: bytes(source, encoding)
        if args.len() >= 2 {
            let arg = &args[0];
            return Ok(parse_quote! { #arg.as_bytes().to_vec() });
        }

        Ok(parse_quote! { Vec::<u8>::new() })
    }

    /// DEPYLER-0674: Convert Python bytearray() constructor to Vec<u8>
    /// bytearray() → Vec::new()
    /// bytearray(n) → vec![0u8; n]
    /// bytearray([1, 2, 3]) → vec![1u8, 2u8, 3u8]
    /// bytearray(b"hello") → b"hello".to_vec()
    pub(crate) fn convert_bytearray_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytearray() → Vec::<u8>::new()
            return Ok(parse_quote! { Vec::<u8>::new() });
        }

        if args.len() == 1 {
            // Safety check to prevent panic if hir_args is out of sync
            if hir_args.is_empty() {
                let arg = &args[0];
                return Ok(parse_quote! { #arg.to_vec() });
            }

            let hir_arg = &hir_args[0];
            let arg = &args[0];

            // bytearray([1, 2, 3]) → list.into_iter() and collect as Vec<u8>
            if matches!(hir_arg, HirExpr::List { .. }) {
                return Ok(parse_quote! { #arg.into_iter().collect::<Vec<u8>>() });
            }

            // bytearray(string) → string.as_bytes().to_vec()
            if self.is_string_type(hir_arg) {
                return Ok(parse_quote! { #arg.as_bytes().to_vec() });
            }

            // bytearray(bytes) → copy the bytes into a new vec
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::List(_)))
                {
                    return Ok(parse_quote! { #arg.to_vec() });
                }
            }

            // DEPYLER-0936: bytearray(n) where n is numeric → vec![0u8; n as usize]
            // Check for int literal
            if matches!(hir_arg, HirExpr::Literal(crate::hir::Literal::Int(_))) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Check for int variable
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::Int))
                {
                    return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
                }
            }

            // For method calls like .len(), assume they return size
            if matches!(hir_arg, HirExpr::MethodCall { .. }) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Default: assume it's a collection that should be collected
            return Ok(parse_quote! { #arg.to_vec() });
        }

        // bytearray with multiple args (source, encoding, errors) - just get bytes
        if args.len() >= 2 {
            let arg = &args[0];
            return Ok(parse_quote! { #arg.as_bytes().to_vec() });
        }

        Ok(parse_quote! { Vec::<u8>::new() })
    }

    /// DEPYLER-0937: Convert Python tuple() constructor to Vec
    /// In Rust, we represent Python tuples as Vec since Rust tuples are fixed-size.
    /// tuple() → vec![]
    /// tuple([1, 2, 3]) → vec![1, 2, 3]
    /// tuple(iterable) → iterable.into_iter().collect::<Vec<_>>()
    pub(crate) fn convert_tuple_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            // tuple() → Vec::new()
            return Ok(parse_quote! { Vec::new() });
        }

        if args.len() == 1 {
            let hir_arg = &hir_args[0];
            let arg = &args[0];

            // tuple(string) → string.chars().collect()
            if self.is_string_type(hir_arg) {
                return Ok(parse_quote! { #arg.chars().collect::<Vec<_>>() });
            }

            // tuple(list) or tuple(iterable) → collect to Vec
            return Ok(parse_quote! { #arg.into_iter().collect::<Vec<_>>() });
        }

        // tuple doesn't take multiple args, but fallback to first arg
        let arg = &args[0];
        Ok(parse_quote! { #arg.into_iter().collect::<Vec<_>>() })
    }

    // DEPYLER-COVERAGE-95: all, any, divmod, enumerate, zip, reversed, sorted
    // moved to stdlib_method_gen::builtin_functions module for testability

    pub(crate) fn convert_filter_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.len() != 2 {
            bail!("filter() requires exactly 2 arguments");
        }
        // Check if first arg is lambda
        // DEPYLER-0754: Use .iter().cloned() instead of .into_iter() to produce owned values
        // When iterable is &Vec<T>, .into_iter() yields &T references, causing type mismatch.
        // .iter().cloned() properly clones elements to produce owned values.
        if let HirExpr::Lambda { params, body } = &hir_args[0] {
            if params.len() != 1 {
                bail!("filter() lambda must have exactly 1 parameter");
            }
            // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
            let param_ident = crate::rust_gen::keywords::safe_ident(&params[0]);

            // DEPYLER-1053: Infer element type from iterable and add lambda param to var_types
            // This enables type coercion in comparisons like `x != 0` where x is f64
            let elem_type = self.infer_iterable_element_type(&hir_args[1]);
            let param_name = params[0].clone();
            if let Some(ref elem_t) = elem_type {
                self.ctx
                    .var_types
                    .insert(param_name.clone(), elem_t.clone());
            }

            let body_expr = body.to_rust_expr(self.ctx)?;

            // DEPYLER-1053: Remove lambda param from var_types to avoid polluting context
            if elem_type.is_some() {
                self.ctx.var_types.remove(&param_name);
            }

            let iterable = &args[1];
            // DEPYLER-1053: Use |&x| pattern because filter() always receives &Item
            // Even with .cloned(), filter's closure parameter is a reference to the owned value
            Ok(parse_quote! {
                #iterable.iter().cloned().filter(|&#param_ident| #body_expr)
            })
        } else {
            let predicate = &args[0];
            let iterable = &args[1];
            Ok(parse_quote! {
                #iterable.iter().cloned().filter(#predicate)
            })
        }
    }

    // DEPYLER-COVERAGE-95: sum, round, abs, min, max, pow, hex, bin, oct
    // moved to stdlib_method_gen::builtin_functions module for testability

    /// DEPYLER-0579: Python format(value, spec) builtin
    /// format(num, "b") → binary string
    /// format(num, "o") → octal string
    /// format(num, "x") → hex string
    /// format(num, "d") → decimal string
    pub(crate) fn convert_format_builtin(
        &self,
        args: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if args.len() != 2 {
            bail!("format() requires exactly 2 arguments (value, spec)");
        }
        let value = &args[0];
        // Extract format spec from HIR to get the actual string
        if let HirExpr::Literal(Literal::String(spec)) = &hir_args[1] {
            match spec.as_str() {
                "b" => Ok(parse_quote! { format!("{:b}", #value) }),
                "o" => Ok(parse_quote! { format!("{:o}", #value) }),
                "x" => Ok(parse_quote! { format!("{:x}", #value) }),
                "X" => Ok(parse_quote! { format!("{:X}", #value) }),
                "d" => Ok(parse_quote! { format!("{}", #value) }),
                "" => Ok(parse_quote! { format!("{}", #value) }),
                _ => {
                    // For unknown format specs, fall back to generic format
                    let spec_str = spec.as_str();
                    // Try to parse as f-string format spec
                    let format_str = format!("{{:{}}}", spec_str);
                    let format_lit: syn::LitStr = syn::parse_str(&format!("\"{}\"", format_str))?;
                    Ok(parse_quote! { format!(#format_lit, #value) })
                }
            }
        } else {
            // Dynamic format spec - can't handle at compile time
            bail!("format() requires a string literal format specifier");
        }
    }

    // DEPYLER-COVERAGE-95: chr moved to stdlib_method_gen::builtin_functions

    pub(crate) fn convert_ord_builtin(
        &self,
        args: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("ord() requires exactly 1 argument");
        }
        let char_str = &args[0];

        // DEPYLER-0795: Check if argument is a char iteration variable
        // When iterating over string.chars(), the loop variable is already a char,
        // so we should use `var as u32` instead of `var.chars().next().unwrap() as i32`
        if let Some(HirExpr::Var(var_name)) = hir_args.first() {
            if self.ctx.char_iter_vars.contains(var_name) {
                // Variable is a char from string iteration - just cast it
                return Ok(parse_quote! {
                    #char_str as u32 as i32
                });
            }
        }

        // Default: assume it's a string and get first char
        Ok(parse_quote! {
            #char_str.chars().next().expect("empty string") as i32
        })
    }

    /// Convert Python open() to Rust file I/O
    /// DEPYLER-0387: File I/O builtin for context managers
    ///
    /// Maps Python open() to Rust std::fs:
    /// - open(path) or open(path, 'r') → std::fs::File::open(path)?
    /// - open(path, 'w') → std::fs::File::create(path)?
    /// - open(path, 'a') → std::fs::OpenOptions::new().append(true).open(path)?
    ///
    /// # Complexity
    /// ≤10 (match with 3 branches)
    pub(crate) fn convert_open_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("open() requires 1 or 2 arguments");
        }

        // DEPYLER-0458: File handles need Read/Write traits
        self.ctx.needs_io_read = true;
        self.ctx.needs_io_write = true;

        let path = &args[0];

        // Determine mode from second argument (default is 'r')
        let mode = if args.len() == 2 {
            // Try to extract string literal from HIR
            if let Some(HirExpr::Literal(Literal::String(mode_str))) = hir_args.get(1) {
                mode_str.as_str()
            } else {
                // If not a literal, default to read mode
                "r"
            }
        } else {
            "r" // Default mode
        };

        // DEPYLER-0541: Handle Option<String> paths with proper unwrapping
        // DEPYLER-0465: Borrow path to avoid moving String parameters
        let borrowed_path = if let Some(hir_arg) = hir_args.first() {
            self.borrow_path_with_option_check(path, hir_arg)
        } else {
            Self::borrow_if_needed(path)
        };

        // DEPYLER-0561: In generator context, use .ok()? since next() returns Option, not Result
        let in_generator = self.ctx.in_generator;

        match mode {
            "r" | "rb" => {
                // Read mode → std::fs::File::open(path)?
                if in_generator {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path).ok()? })
                } else {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path)? })
                }
            }
            "w" | "wb" => {
                // Write mode → std::fs::File::create(path)?
                if in_generator {
                    Ok(parse_quote! { std::fs::File::create(#borrowed_path).ok()? })
                } else {
                    Ok(parse_quote! { std::fs::File::create(#borrowed_path)? })
                }
            }
            "a" | "ab" => {
                // Append mode → OpenOptions with append
                if in_generator {
                    Ok(parse_quote! {
                        std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(#borrowed_path).ok()?
                    })
                } else {
                    Ok(parse_quote! {
                        std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(#borrowed_path)?
                    })
                }
            }
            _ => {
                // Unsupported mode, default to read
                if in_generator {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path).ok()? })
                } else {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path)? })
                }
            }
        }
    }

    // DEPYLER-COVERAGE-95: hash, repr, next, iter, type
    // moved to stdlib_method_gen::builtin_functions module for testability

    // DEPYLER-STDLIB-50: getattr() - get attribute by name (needs context-specific error)
    pub(crate) fn convert_getattr_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 || args.len() > 3 {
            bail!("getattr() requires 2 or 3 arguments (object, name, optional default)");
        }
        // Note: This is a simplified implementation
        // Full getattr() requires runtime attribute lookup which isn't possible in Rust
        // For now, we'll bail as it needs special handling
        bail!("getattr() requires dynamic attribute access not fully supported yet")
    }

    // DEPYLER-REFACTOR-001: Helper functions moved to collection_constructors module:
    // already_collected, is_range_expr, is_iterator_expr, is_csv_reader_var


}

//! Call dispatch helpers for ExpressionConverter
//!
//! Contains try_convert_stdlib_type_call, try_convert_numeric_type_call,
//! try_convert_iterator_util_call, print/sum/minmax/any_all call handlers,
//! and helper methods (needs_debug_format, is_pathbuf_expr, infer_numeric_type_token).

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use anyhow::Result;
use quote::quote;
use syn::{self, parse_quote};

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn try_convert_stdlib_type_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        match func {
            // DEPYLER-STDLIB-PATHLIB: Handle Path() constructor
            // DEPYLER-0559: Handle Optional args from argparse (Option<String>)
            "Path" if args.len() == 1 => {
                let path_expr = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                // Check if this is an argparse Optional field (args.field where field is Option<T>)
                let is_optional_arg = if let HirExpr::Attribute { value, attr } = &args[0] {
                    if let HirExpr::Var(var_name) = &**value {
                        // Check if this is args.field pattern with Optional field
                        if var_name == "args" {
                            // Look through parsers for this argument
                            self.ctx
                                .argparser_tracker
                                .get_first_parser()
                                .map(|p| {
                                    p.arguments
                                        .iter()
                                        .find(|a| a.rust_field_name() == *attr)
                                        .map(|a| a.rust_type().starts_with("Option<"))
                                        .unwrap_or(false)
                                })
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_optional_arg {
                    // Unwrap the Option before PathBuf::from
                    Some(Ok(
                        parse_quote! { std::path::PathBuf::from(#path_expr.as_ref().expect("value is None")) },
                    ))
                } else {
                    let borrowed_path = Self::borrow_if_needed(&path_expr);
                    Some(Ok(parse_quote! { std::path::PathBuf::from(#borrowed_path) }))
                }
            }

            // DEPYLER-STDLIB-DATETIME/1025: Handle datetime constructors
            "datetime" if args.len() >= 3 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                let year = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let month = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let day = match args[2].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                if args.len() == 3 {
                    if nasa_mode {
                        // DEPYLER-1067: NASA mode - use DepylerDateTime::new()
                        self.ctx.needs_depyler_datetime = true;
                        Some(Ok(
                            parse_quote! { DepylerDateTime::new(#year as u32, #month as u32, #day as u32, 0, 0, 0, 0) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                                .expect("invalid date")
                                .and_hms_opt(0, 0, 0)
                                .expect("invalid time")
                        }))
                    }
                } else if args.len() >= 6 {
                    let hour = match args[3].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let minute = match args[4].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let second = match args[5].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    // DEPYLER-1067: Handle optional microsecond argument
                    let microsecond = if args.len() >= 7 {
                        match args[6].to_rust_expr(self.ctx) {
                            Ok(e) => e,
                            Err(e) => return Some(Err(e)),
                        }
                    } else {
                        parse_quote! { 0 }
                    };
                    if nasa_mode {
                        // DEPYLER-1067: NASA mode - use DepylerDateTime::new()
                        self.ctx.needs_depyler_datetime = true;
                        Some(Ok(
                            parse_quote! { DepylerDateTime::new(#year as u32, #month as u32, #day as u32, #hour as u32, #minute as u32, #second as u32, #microsecond as u32) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                                .expect("invalid date")
                                .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                                .expect("invalid time")
                        }))
                    }
                } else {
                    Some(Err(anyhow::anyhow!("datetime() requires 3 or 6+ arguments")))
                }
            }
            "datetime" => Some(Err(anyhow::anyhow!(
                "datetime() requires at least 3 arguments (year, month, day)"
            ))),

            // DEPYLER-1025/1066: date(year, month, day) - NASA mode uses DepylerDate struct
            "date" if args.len() == 3 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                } else {
                    // DEPYLER-1066: Mark that we need the DepylerDate struct
                    self.ctx.needs_depyler_date = true;
                }
                let year = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let month = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let day = match args[2].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                if nasa_mode {
                    // DEPYLER-1066: Use DepylerDate::new() instead of raw tuple
                    Some(Ok(
                        parse_quote! { DepylerDate::new(#year as u32, #month as u32, #day as u32) },
                    ))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).expect("invalid date")
                    }))
                }
            }

            // DEPYLER-0938/1025: time() with no args - NASA mode uses tuple
            "time" if args.is_empty() => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                if nasa_mode {
                    Some(Ok(parse_quote! { (0u32, 0u32, 0u32) }))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("invalid time")
                    }))
                }
            }

            // DEPYLER-0938/1025: time(hour) - NASA mode uses tuple
            "time" if args.len() == 1 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                let hour = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                if nasa_mode {
                    Some(Ok(parse_quote! { (#hour as u32, 0u32, 0u32) }))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveTime::from_hms_opt(#hour as u32, 0, 0).expect("invalid time")
                    }))
                }
            }

            // DEPYLER-1025: time(hour, minute, second) - NASA mode uses tuple
            "time" if args.len() >= 2 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                let hour = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let minute = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                if args.len() == 2 {
                    if nasa_mode {
                        Some(Ok(parse_quote! { (#hour as u32, #minute as u32, 0u32) }))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, 0).expect("invalid time")
                        }))
                    }
                } else {
                    let second = match args[2].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(parse_quote! { (#hour as u32, #minute as u32, #second as u32) }))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, #second as u32).expect("invalid time")
                        }))
                    }
                }
            }

            // DEPYLER-1025/1068: timedelta(days=..., seconds=...) - NASA mode uses DepylerTimeDelta
            "timedelta" => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                } else {
                    self.ctx.needs_depyler_timedelta = true;
                }
                if args.is_empty() {
                    if nasa_mode {
                        Some(Ok(parse_quote! { DepylerTimeDelta::new(0, 0, 0) }))
                    } else {
                        Some(Ok(parse_quote! { chrono::Duration::zero() }))
                    }
                } else if args.len() == 1 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(parse_quote! { DepylerTimeDelta::new(#days as i64, 0, 0) }))
                    } else {
                        Some(Ok(parse_quote! { chrono::Duration::days(#days as i64) }))
                    }
                } else if args.len() == 2 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let seconds = match args[1].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(
                            parse_quote! { DepylerTimeDelta::new(#days as i64, #seconds as i64, 0) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::Duration::days(#days as i64) + chrono::Duration::seconds(#seconds as i64)
                        }))
                    }
                } else {
                    None // Let it fall through
                }
            }

            _ => None,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.12: Extracted numeric type constructors helper
    ///
    /// Handles Decimal and Fraction constructors.
    /// Returns Some(result) if handled, None if not a numeric type constructor.
    ///
    /// # Complexity: 7
    pub(crate) fn try_convert_numeric_type_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        match func {
            // DEPYLER-STDLIB-DECIMAL: Handle Decimal() constructor
            "Decimal" if args.len() == 1 => {
                self.ctx.needs_rust_decimal = true;
                let arg = &args[0];

                let result = match arg {
                    HirExpr::Literal(Literal::String(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_str(&#arg_expr).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Int(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(parse_quote! { rust_decimal::Decimal::from(#arg_expr) }),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Float(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_f64_retain(#arg_expr).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                    _ => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_str(&(#arg_expr).to_string()).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                };
                Some(result)
            }

            // DEPYLER-STDLIB-FRACTIONS: Handle Fraction() constructor
            "Fraction" if args.len() == 1 => {
                self.ctx.needs_num_rational = true;
                let arg = &args[0];

                let result = match arg {
                    HirExpr::Literal(Literal::String(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(parse_quote! {
                            {
                                let s = #arg_expr;
                                let parts: Vec<&str> = s.split('/').collect();
                                if parts.len() == 2 {
                                    let num = parts[0].trim().parse::<i32>().expect("parse failed");
                                    let denom = parts[1].trim().parse::<i32>().expect("parse failed");
                                    num::rational::Ratio::new(num, denom)
                                } else {
                                    let num = s.parse::<i32>().expect("parse failed");
                                    num::rational::Ratio::from_integer(num)
                                }
                            }
                        }),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Int(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => {
                            Ok(parse_quote! { num::rational::Ratio::from_integer(#arg_expr) })
                        }
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Float(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { num::rational::Ratio::approximate_float(#arg_expr).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                    _ => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { num::rational::Ratio::approximate_float(#arg_expr as f64).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                };
                Some(result)
            }

            "Fraction" if args.len() == 2 => {
                self.ctx.needs_num_rational = true;
                let num_expr = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let denom_expr = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok(parse_quote! { num::rational::Ratio::new(#num_expr, #denom_expr) }))
            }

            "Fraction" => Some(Err(anyhow::anyhow!("Fraction() requires 1 or 2 arguments"))),

            _ => None,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.13: Extracted iterator utility call helper
    ///
    /// Handles enumerate, zip, and isinstance calls.
    /// Returns Some(result) if handled, None if not an iterator utility call.
    ///
    /// # Complexity: 6
    pub(crate) fn try_convert_iterator_util_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        match func {
            // DEPYLER-0519: enumerate(items) → items.iter().cloned().enumerate()
            // Use iter().cloned() to preserve original collection (Python doesn't consume)
            // DEPYLER-0305: For file variables, use BufReader for line iteration
            "enumerate" if args.len() == 1 => {
                // Check if arg is a file variable (heuristic based on name)
                let is_file_var = if let HirExpr::Var(var_name) = &args[0] {
                    var_name == "f"
                        || var_name == "file"
                        || var_name == "input"
                        || var_name == "output"
                        || var_name.ends_with("_file")
                        || var_name.starts_with("file_")
                } else {
                    false
                };

                // DEPYLER-99MODE-S9: Check if arg is a string variable
                // Strings use .chars() not .iter() for iteration
                let is_string_var = if let HirExpr::Var(var_name) = &args[0] {
                    self.ctx.var_types.get(var_name).is_some_and(|t| matches!(t, Type::String))
                        || self.ctx.fn_str_params.contains(var_name)
                } else {
                    false
                };

                match args[0].to_rust_expr(self.ctx) {
                    Ok(items_expr) => {
                        if is_file_var {
                            // DEPYLER-0305: File iteration with enumerate
                            // DEPYLER-0692: Convert usize index to i32 for Python compatibility
                            self.ctx.needs_bufread = true;
                            Some(Ok(parse_quote! {
                                std::io::BufReader::new(#items_expr)
                                    .lines()
                                    .map(|l| l.unwrap_or_default())
                                    .enumerate()
                                    .map(|(i, x)| (i as i32, x))
                            }))
                        } else if is_string_var {
                            // DEPYLER-99MODE-S9: String iteration uses .chars()
                            Some(Ok(
                                parse_quote! { #items_expr.chars().enumerate().map(|(i, x)| (i as i32, x)) },
                            ))
                        } else {
                            // DEPYLER-0692: Convert usize index to i32 for Python compatibility
                            Some(Ok(
                                parse_quote! { #items_expr.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) },
                            ))
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            }

            // zip(a, b, ...) → a.into_iter().zip(b.into_iter())...
            "zip" if args.len() >= 2 => {
                let arg_exprs: Vec<syn::Expr> = match args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()
                {
                    Ok(exprs) => exprs,
                    Err(e) => return Some(Err(e)),
                };

                // Determine if we should use .into_iter() or .iter()
                let use_into_iter = args.iter().all(|arg| self.is_owned_collection(arg));

                let first = &arg_exprs[0];
                let mut chain: syn::Expr = if use_into_iter {
                    parse_quote! { #first.into_iter() }
                } else {
                    parse_quote! { #first.iter() }
                };

                for arg in &arg_exprs[1..] {
                    chain = if use_into_iter {
                        parse_quote! { #chain.zip(#arg.into_iter()) }
                    } else {
                        parse_quote! { #chain.zip(#arg.iter()) }
                    };
                }

                Some(Ok(chain))
            }

            // isinstance(value, type) → true (Rust's type system guarantees correctness)
            "isinstance" if args.len() == 2 => Some(Ok(parse_quote! { true })),

            _ => None,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.15: Extracted debug format detection helper
    ///
    /// Determines if a HirExpr needs {:?} debug formatting instead of {} display formatting.
    /// Used by print() handler to select appropriate format specifiers.
    ///
    /// Returns true for:
    /// - Collection types (List, Dict, Set, Optional, Unknown)
    /// - Collection literals (list, dict, set, frozenset)
    /// - Function calls that return Result types
    /// - Variables named "value" (heuristic for Option<T>)
    ///
    /// # Complexity: 4
    pub(crate) fn needs_debug_format(&self, hir_arg: &HirExpr) -> bool {
        match hir_arg {
            HirExpr::Var(name) => {
                // DEPYLER-0468: Use debug formatter for collections and Optional types
                let type_based = self
                    .ctx
                    .var_types
                    .get(name)
                    .map(|t| {
                        matches!(
                            t,
                            Type::List(_)
                                | Type::Dict(_, _)
                                | Type::Set(_)
                                | Type::Optional(_)
                                | Type::Unknown
                        )
                    })
                    .unwrap_or(false);

                // Heuristic: "value" often comes from functions returning Option<T>
                let name_based = name == "value";

                type_based || name_based
            }
            // DEPYLER-0600 #6: Comprehension types also produce collections
            HirExpr::List(_)
            | HirExpr::Dict(_)
            | HirExpr::Set(_)
            | HirExpr::FrozenSet(_)
            | HirExpr::ListComp { .. }
            | HirExpr::DictComp { .. }
            | HirExpr::SetComp { .. } => true,
            // DEPYLER-1365: Result-returning calls should be unwrapped, not debug-formatted
            // See try_convert_print_call for unwrap handling
            HirExpr::Call { .. } => false,
            _ => false,
        }
    }

    /// DEPYLER-0930: Check if expression is a PathBuf type that needs .display()
    ///
    /// PathBuf doesn't implement Display trait, so we need to detect it and wrap
    /// with .display() when used in print statements or format strings.
    ///
    /// # Complexity: 4
    pub(crate) fn is_pathbuf_expr(&self, hir_arg: &HirExpr) -> bool {
        match hir_arg {
            HirExpr::Var(name) => {
                // Check var_types for PathBuf/Path type
                self.ctx
                    .var_types
                    .get(name)
                    .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                    .unwrap_or(false)
            }
            HirExpr::MethodCall { object, method, .. } => {
                // Methods that return PathBuf - only match when receiver is PathBuf
                // DEPYLER-0930: `join` on String is different from `join` on PathBuf
                let is_pathbuf_method =
                    matches!(method.as_str(), "parent" | "with_name" | "with_suffix" | "with_stem");
                if is_pathbuf_method {
                    return true;
                }
                // For `join`, check if receiver is PathBuf type
                if method == "join" {
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        return self
                            .ctx
                            .var_types
                            .get(var_name)
                            .map(|t| {
                                matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                            })
                            .unwrap_or(false);
                    }
                }
                false
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
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.16: Extracted numeric type token inference helper
    ///
    /// Infers the numeric type token for sum/aggregate operations based on
    /// the current function's return type context.
    ///
    /// Returns:
    /// - `quote! { i32 }` for Int return type
    /// - `quote! { f64 }` for Float return type
    /// - `quote! { i32 }` as default for other/unknown types
    ///
    /// # Complexity: 2
    pub(crate) fn infer_numeric_type_token(&self) -> proc_macro2::TokenStream {
        self.ctx
            .current_return_type
            .as_ref()
            .and_then(|t| match t {
                Type::Int => Some(quote! { i32 }),
                Type::Float => Some(quote! { f64 }),
                _ => None,
            })
            .unwrap_or_else(|| quote! { i32 })
    }

    /// DEPYLER-REFACTOR-001 Phase 2.17: Extracted print call handler
    ///
    /// Handles Python print() function conversion to Rust println!/eprintln!.
    ///
    /// Features:
    /// - print() with no args → println!()
    /// - print(single_arg) → println!("{}", arg) or println!("{:?}", arg) for debug types
    /// - print(multiple_args) → println!("{} {} ...", arg1, arg2, ...)
    /// - file=sys.stderr kwarg → eprintln! variants
    ///
    /// Returns Some(Ok(expr)) if handled, None if not a print call.
    ///
    /// # Complexity: 5
    pub(crate) fn try_convert_print_call(
        &self,
        func: &str,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        kwargs: &[(String, HirExpr)],
    ) -> Option<Result<syn::Expr>> {
        if func != "print" {
            return None;
        }

        // DEPYLER-0462: Check if file=sys.stderr keyword is present
        let use_stderr = kwargs.iter().any(|(name, value)| {
            name == "file"
                && matches!(value, HirExpr::Attribute {
                    value: attr_value,
                    attr
                } if matches!(&**attr_value, HirExpr::Var(module) if module == "sys") && attr == "stderr")
        });

        // DEPYLER-0945: Process arguments to handle PathBuf.display() correctly
        // This handles both single and multiple arguments uniformly
        let processed_args: Vec<syn::Expr> = args
            .iter()
            .zip(arg_exprs.iter())
            .map(|(hir, syn)| {
                if self.is_pathbuf_expr(hir) {
                    parse_quote! { #syn.display() }
                } else {
                    syn.clone()
                }
            })
            .collect();

        let result = if args.is_empty() {
            // print() with no arguments
            if use_stderr {
                Ok(parse_quote! { eprintln!() })
            } else {
                Ok(parse_quote! { println!() })
            }
        } else if args.len() == 1 {
            // Single argument print
            let needs_debug = args.first().map(|a| self.needs_debug_format(a)).unwrap_or(false);

            // DEPYLER-1365: Check if argument is a Result-returning call that needs unwrapping
            let is_result_call = matches!(&args[0], HirExpr::Call { func, .. }
                if self.ctx.result_returning_functions.contains(func));

            let arg = &processed_args[0];

            if is_result_call {
                // DEPYLER-1365: Unwrap Result types for semantic parity with Python
                // Python: print(divide(17, 5)) → "3"
                // Rust:   println!("{}", divide(17, 5).unwrap()) → "3"
                if use_stderr {
                    Ok(parse_quote! { eprintln!("{}", #arg.expect("operation failed")) })
                } else {
                    Ok(parse_quote! { println!("{}", #arg.expect("operation failed")) })
                }
            } else {
                let format_str = if needs_debug { "{:?}" } else { "{}" };

                if use_stderr {
                    Ok(parse_quote! { eprintln!(#format_str, #arg) })
                } else {
                    Ok(parse_quote! { println!(#format_str, #arg) })
                }
            }
        } else {
            // Multiple arguments - build format string with per-arg detection
            // DEPYLER-1365: Also handle Result-returning calls by unwrapping
            let format_specs: Vec<&str> = args
                .iter()
                .map(|hir_arg| if self.needs_debug_format(hir_arg) { "{:?}" } else { "{}" })
                .collect();
            let format_str = format_specs.join(" ");

            // DEPYLER-1365: Process args to unwrap Result-returning calls
            let final_args: Vec<syn::Expr> = args
                .iter()
                .zip(processed_args.iter())
                .map(|(hir_arg, syn_arg)| {
                    if let HirExpr::Call { func, .. } = hir_arg {
                        if self.ctx.result_returning_functions.contains(func) {
                            return parse_quote! { #syn_arg.expect("operation failed") };
                        }
                    }
                    syn_arg.clone()
                })
                .collect();

            if use_stderr {
                Ok(parse_quote! { eprintln!(#format_str, #(#final_args),*) })
            } else {
                Ok(parse_quote! { println!(#format_str, #(#final_args),*) })
            }
        };

        Some(result)
    }

    /// DEPYLER-REFACTOR-001 Phase 2.18: Extracted sum call handler
    ///
    /// Handles Python sum() function conversion to Rust iterator patterns.
    ///
    /// Variants:
    /// - sum(generator_exp) → gen_expr.sum::<T>()
    /// - sum(range(...)) → (range_expr).sum::<T>()
    /// - sum(d.values()) / sum(d.keys()) → optimized iterator chain
    /// - sum(iterable) → iterable.iter().sum::<T>()
    ///
    /// Returns Some(Ok(expr)) if handled, None if not a sum call.
    ///
    /// # Complexity: 6
    pub(crate) fn try_convert_sum_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        if func != "sum" || args.len() != 1 {
            return None;
        }

        // DEPYLER-0247: Handle sum(generator_exp) → gen_expr.sum::<T>()
        if matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };
            let target_type = self.infer_numeric_type_token();
            return Some(Ok(parse_quote! { #gen_expr.sum::<#target_type>() }));
        }

        // DEPYLER-0307: Handle sum(range(...)) → (range_expr).sum::<T>()
        if let HirExpr::Call { func: range_func, .. } = &args[0] {
            if range_func == "range" {
                let range_expr = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let target_type = self.infer_numeric_type_token();
                return Some(Ok(parse_quote! { (#range_expr).sum::<#target_type>() }));
            }
        }

        // DEPYLER-0303: Handle sum(d.values()) and sum(d.keys()) - optimized path
        if let HirExpr::MethodCall { object, method, args: method_args, .. } = &args[0] {
            if (method == "values" || method == "keys") && method_args.is_empty() {
                let object_expr = match object.to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                // DEPYLER-0328: Infer sum type from collection element type
                let target_type = if method == "values" {
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        self.ctx.var_types.get(var_name).and_then(|var_type| {
                            if let Type::Dict(_key_type, value_type) = var_type {
                                match value_type.as_ref() {
                                    Type::Int => Some(quote! { i32 }),
                                    Type::Float => Some(quote! { f64 }),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                } else {
                    None // .keys() typically returns strings
                }
                .unwrap_or_else(|| quote! { i32 });

                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                return Some(Ok(parse_quote! {
                    #object_expr.#method_ident().cloned().sum::<#target_type>()
                }));
            }
        }

        // Default: sum(iterable) → iterable.iter().sum::<T>()
        let iter_expr = match args[0].to_rust_expr(self.ctx) {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };
        let target_type = self.infer_numeric_type_token();
        Some(Ok(parse_quote! { #iter_expr.iter().sum::<#target_type>() }))
    }

    /// DEPYLER-REFACTOR-001 Phase 2.19: Extracted min/max call handler
    ///
    /// Handles Python min()/max() function conversion to Rust.
    ///
    /// Variants:
    /// - max(a, b) / min(a, b) → std::cmp::max/min or f64.max/min for floats
    /// - max(iterable) / min(iterable) → iter.max/min().unwrap()
    ///
    /// Returns Some(Ok(expr)) if handled, None if not a min/max call.
    ///
    /// # Complexity: 5
    pub(crate) fn try_convert_minmax_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        if func != "max" && func != "min" {
            return None;
        }

        let is_max = func == "max";

        // Handle max(a, b) / min(a, b) with mixed numeric types
        if args.len() == 2 {
            let arg1 = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };
            let arg2 = match args[1].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            // DEPYLER-0515: Check if either argument is a float literal
            let has_float = matches!(args[0], HirExpr::Literal(Literal::Float(_)))
                || matches!(args[1], HirExpr::Literal(Literal::Float(_)));

            if has_float {
                // Use f64 method call: (a as f64).max/min(b as f64)
                return if is_max {
                    Some(Ok(parse_quote! { (#arg1 as f64).max(#arg2 as f64) }))
                } else {
                    Some(Ok(parse_quote! { (#arg1 as f64).min(#arg2 as f64) }))
                };
            }

            // DEPYLER-1062: Use depyler_min/depyler_max helpers for safe comparison
            // These handle PartialOrd correctly and work with f64/DepylerValue
            // Parenthesize arguments to handle casts safely
            return if is_max {
                Some(Ok(parse_quote! { depyler_max((#arg1).clone(), (#arg2).clone()) }))
            } else {
                Some(Ok(parse_quote! { depyler_min((#arg1).clone(), (#arg2).clone()) }))
            };
        }

        // Handle max(iterable) / min(iterable)
        if args.len() == 1 {
            let iter_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            return if is_max {
                Some(Ok(parse_quote! { *#iter_expr.iter().max().expect("empty collection") }))
            } else {
                Some(Ok(parse_quote! { *#iter_expr.iter().min().expect("empty collection") }))
            };
        }

        None
    }

    /// DEPYLER-REFACTOR-001 Phase 2.20: Extracted any/all call handler
    ///
    /// Handles Python any()/all() function conversion to Rust.
    ///
    /// Variants:
    /// - any(generator_exp) / all(generator_exp) → gen.any/all(|x| x)
    /// - any(iterable) / all(iterable) → iter.any/all(|&x| x)
    ///
    /// Returns Some(Ok(expr)) if handled, None if not an any/all call.
    ///
    /// # Complexity: 4
    pub(crate) fn try_convert_any_all_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        if (func != "any" && func != "all") || args.len() != 1 {
            return None;
        }

        let is_any = func == "any";

        // Handle any/all with generator expressions - don't call .iter()
        if matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };
            return if is_any {
                Some(Ok(parse_quote! { #gen_expr.any(|x| x) }))
            } else {
                Some(Ok(parse_quote! { #gen_expr.all(|x| x) }))
            };
        }

        // Handle any/all with iterables - need .iter()
        let iter_expr = match args[0].to_rust_expr(self.ctx) {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };

        if is_any {
            Some(Ok(parse_quote! { #iter_expr.iter().any(|&x| x) }))
        } else {
            Some(Ok(parse_quote! { #iter_expr.iter().all(|&x| x) }))
        }
    }
}

//! Expression code generation
//!
//! This module handles converting HIR expressions to Rust syn::Expr nodes.
//! It includes the ExpressionConverter for complex expression transformations
//! and the ToRustExpr trait implementation for HirExpr.

use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::return_type_expects_float;
use crate::rust_gen::type_gen::convert_binop;
use crate::string_optimization::{StringContext, StringOptimizer};
use anyhow::{bail, Result};
use quote::quote;
use syn::{self, parse_quote};

struct ExpressionConverter<'a, 'b> {
    ctx: &'a mut CodeGenContext<'b>,
}

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    fn new(ctx: &'a mut CodeGenContext<'b>) -> Self {
        Self { ctx }
    }

    /// Check if a name is a Rust keyword that requires raw identifier syntax
    fn is_rust_keyword(name: &str) -> bool {
        matches!(
            name,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "async"
                | "await"
                | "dyn"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "macro"
                | "override"
                | "priv"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
                | "try"
        )
    }

    /// Check if a keyword cannot be used as a raw identifier
    /// These special keywords (self, Self, super, crate) cannot use r# syntax
    fn is_non_raw_keyword(name: &str) -> bool {
        matches!(name, "self" | "Self" | "super" | "crate")
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        // Check for special keywords that cannot be raw identifiers
        if Self::is_non_raw_keyword(name) {
            bail!(
                "Python variable '{}' conflicts with a special Rust keyword that cannot be escaped. \
                 Please rename this variable (e.g., '{}_var' or 'py_{}')",
                name, name, name
            );
        }

        // Inside generators, check if variable is a state variable
        if self.ctx.in_generator && self.ctx.generator_state_vars.contains(name) {
            // Generate self.field for state variables
            let ident = if Self::is_rust_keyword(name) {
                syn::Ident::new_raw(name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(name, proc_macro2::Span::call_site())
            };
            Ok(parse_quote! { self.#ident })
        } else {
            // Regular variable - use raw identifier if it's a Rust keyword
            let ident = if Self::is_rust_keyword(name) {
                syn::Ident::new_raw(name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(name, proc_macro2::Span::call_site())
            };
            Ok(parse_quote! { #ident })
        }
    }

    fn convert_binary(&mut self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = left.to_rust_expr(self.ctx)?;
        let right_expr = right.to_rust_expr(self.ctx)?;

        match op {
            BinOp::In => {
                // Convert "x in container" to appropriate method call
                // - HashSet: container.contains(&x)
                // - HashMap/dict: container.contains_key(&x)
                // Check if right side is a set based on type information
                let is_set = self.is_set_expr(right) || self.is_set_var(right);

                // DEPYLER-0303: Don't add & for string literals or simple variables
                // String literals are already &str after conversion
                // Variables (especially parameters) might already be &str, so let compiler infer
                let needs_ref = !matches!(left, HirExpr::Literal(Literal::String(_)) | HirExpr::Var(_));

                if is_set {
                    if needs_ref {
                        Ok(parse_quote! { #right_expr.contains(&#left_expr) })
                    } else {
                        Ok(parse_quote! { #right_expr.contains(#left_expr) })
                    }
                } else {
                    // HashMap/dict
                    if needs_ref {
                        Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                    } else {
                        Ok(parse_quote! { #right_expr.contains_key(#left_expr) })
                    }
                }
            }
            BinOp::NotIn => {
                // Convert "x not in container" to !container.method(&x)
                // Check if right side is a set based on type information
                let is_set = self.is_set_expr(right) || self.is_set_var(right);

                // DEPYLER-0303: Don't add & for string literals or simple variables (same as BinOp::In)
                let needs_ref = !matches!(left, HirExpr::Literal(Literal::String(_)) | HirExpr::Var(_));

                if is_set {
                    if needs_ref {
                        Ok(parse_quote! { !#right_expr.contains(&#left_expr) })
                    } else {
                        Ok(parse_quote! { !#right_expr.contains(#left_expr) })
                    }
                } else {
                    // HashMap/dict
                    if needs_ref {
                        Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                    } else {
                        Ok(parse_quote! { !#right_expr.contains_key(#left_expr) })
                    }
                }
            }
            BinOp::Add => {
                // DEPYLER-0290 FIX: Special handling for list concatenation
                // DEPYLER-0299 Pattern #4 FIX: Don't assume all Var + Var is list concatenation

                // Check if we're dealing with lists/vectors (explicit detection only)
                let is_definitely_list = self.is_list_expr(left) || self.is_list_expr(right);

                // Check if we're dealing with strings (literals or type-inferred)
                let is_definitely_string = matches!(left, HirExpr::Literal(Literal::String(_)))
                    || matches!(right, HirExpr::Literal(Literal::String(_)))
                    || matches!(self.ctx.current_return_type, Some(Type::String));

                if is_definitely_list && !is_definitely_string {
                    // List concatenation - use iterator chain
                    // Convert: list1 + list2
                    // To: list1.iter().chain(list2.iter()).cloned().collect()
                    Ok(parse_quote! {
                        {
                            let mut __temp = #left_expr.clone();
                            __temp.extend(#right_expr.iter().cloned());
                            __temp
                        }
                    })
                } else if is_definitely_string {
                    // This is string concatenation - use format! to handle references properly
                    Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
                } else {
                    // Regular arithmetic addition or unknown types
                    // Default to arithmetic - safer assumption for scalar types
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // For now, we generate code that works for integers with proper floor semantics
                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        // Avoid != in boolean expression due to formatting issues
                        let r_negative = r < 0;
                        let b_negative = b < 0;
                        let r_nonzero = r != 0;
                        let signs_differ = r_negative != b_negative;
                        let needs_adjustment = r_nonzero && signs_differ;
                        if needs_adjustment { q - 1 } else { q }
                    }
                })
            }
            // DEPYLER-0303 Phase 3 Fix #7: Dict merge operator |
            // Python 3.9+ supports d1 | d2 for dictionary merge
            // Translate to: { let mut result = d1; result.extend(d2); result }
            BinOp::BitOr if self.is_dict_expr(left) || self.is_dict_expr(right) => {
                self.ctx.needs_hashmap = true;
                Ok(parse_quote! {
                    {
                        let mut __merge_result = #left_expr.clone();
                        __merge_result.extend(#right_expr.iter().map(|(k, v)| (k.clone(), *v)));
                        __merge_result
                    }
                })
            }
            // Set operators - check if both operands are sets
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                if self.is_set_expr(left) && self.is_set_expr(right) =>
            {
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub if self.is_set_expr(left) && self.is_set_expr(right) => {
                // Set difference operation
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if self.is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    // Wrap left_expr in parens because it contains a cast: (arr.len() as i32).saturating_sub(x)
                    // Without parens, Rust parses "as i32.saturating_sub" incorrectly
                    Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::Mul => {
                // DEPYLER-0302 Phase 2: String repetition (s * n or n * s)
                // Check if we have string * integer or integer * string
                let left_is_string = self.is_string_base(left);
                let right_is_string = self.is_string_base(right);
                let left_is_int = matches!(left, HirExpr::Literal(Literal::Int(_)) | HirExpr::Var(_));
                let right_is_int = matches!(right, HirExpr::Literal(Literal::Int(_)) | HirExpr::Var(_));

                if left_is_string && right_is_int {
                    // Pattern: s * n (string * integer)
                    return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
                } else if left_is_int && right_is_string {
                    // Pattern: n * s (integer * string)
                    return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
                }

                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = elts[0].to_rust_expr(self.ctx)?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = elts[0].to_rust_expr(self.ctx)?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        Ok(parse_quote! { #left_expr #rust_op #right_expr })
                    }
                }
            }
            BinOp::Div => {
                // v3.16.0 Phase 2: Python's `/` always returns float
                // Rust's `/` does integer division when both operands are integers
                // Check if we need to cast to float based on return type context
                let needs_float_division = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .map(return_type_expects_float)
                    .unwrap_or(false);

                if needs_float_division {
                    // Cast both operands to f64 for Python float division semantics
                    Ok(parse_quote! { (#left_expr as f64) / (#right_expr as f64) })
                } else {
                    // Regular division (int/int → int, float/float → float)
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::Pow => {
                // Python power operator ** needs type-specific handling in Rust
                // For integers: use .pow() with u32 exponent
                // For floats: use .powf() with f64 exponent
                // For negative integer exponents: convert to float

                // Check if we have literals to determine types
                match (left, right) {
                    // Integer literal base with integer literal exponent
                    (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                        if *exp < 0 {
                            // Negative exponent: convert to float operation
                            Ok(parse_quote! {
                                (#left_expr as f64).powf(#right_expr as f64)
                            })
                        } else {
                            // Positive integer exponent: use .pow() with u32
                            // Add checked_pow for overflow safety
                            Ok(parse_quote! {
                                #left_expr.checked_pow(#right_expr as u32)
                                    .expect("Power operation overflowed")
                            })
                        }
                    }
                    // Float literal base: always use .powf()
                    (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                        #left_expr.powf(#right_expr as f64)
                    }),
                    // Any base with float exponent: use .powf()
                    (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                        (#left_expr as f64).powf(#right_expr)
                    }),
                    // Variables or complex expressions: generate type-safe code
                    _ => {
                        // For non-literal expressions, we need runtime type checking
                        // This is a conservative approach that works for common cases
                        // Determine the target type for casting from context
                        let target_type = self
                            .ctx
                            .current_return_type
                            .as_ref()
                            .and_then(|t| match t {
                                Type::Int => Some(quote! { i32 }),
                                Type::Float => Some(quote! { f64 }),
                                _ => None,
                            })
                            .unwrap_or_else(|| quote! { i32 });

                        Ok(parse_quote! {
                            {
                                // Try integer power first if exponent can be u32
                                if #right_expr >= 0 && #right_expr <= u32::MAX as i64 {
                                    #left_expr.checked_pow(#right_expr as u32)
                                        .expect("Power operation overflowed")
                                } else {
                                    // Fall back to float power for negative or large exponents
                                    (#left_expr as f64).powf(#right_expr as f64) as #target_type
                                }
                            }
                        })
                    }
                }
            }
            _ => {
                let rust_op = convert_binop(op)?;
                Ok(parse_quote! { #left_expr #rust_op #right_expr })
            }
        }
    }

    fn convert_unary(&mut self, op: &UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
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
                } else {
                    false
                };

                if is_collection {
                    Ok(parse_quote! { #operand_expr.is_empty() })
                } else {
                    Ok(parse_quote! { !#operand_expr })
                }
            }
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    fn convert_call(&mut self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
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
        if func == "filter" && args.len() == 2 {
            if let HirExpr::Lambda { params, body } = &args[0] {
                if params.len() != 1 {
                    bail!("filter() lambda must have exactly one parameter");
                }
                let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                let param_ident = syn::Ident::new(&params[0], proc_macro2::Span::call_site());
                let body_expr = body.to_rust_expr(self.ctx)?;

                return Ok(parse_quote! {
                    #iterable_expr.into_iter().filter(|#param_ident| #body_expr)
                });
            }
        }

        // Handle sum(generator_exp) → generator_exp.sum::<T>()
        // Need turbofish type annotation to help Rust's type inference
        if func == "sum" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;

            // Infer the target type from return type context
            let target_type = self
                .ctx
                .current_return_type
                .as_ref()
                .and_then(|t| match t {
                    Type::Int => Some(quote! { i32 }),
                    Type::Float => Some(quote! { f64 }),
                    _ => None,
                })
                .unwrap_or_else(|| quote! { i32 });

            return Ok(parse_quote! { #gen_expr.sum::<#target_type>() });
        }

        // Handle max(generator_exp) → generator_exp.max()
        if func == "max" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.max() });
        }

        // DEPYLER-0190: Handle sorted(iterable) → { let mut result = iterable.clone(); result.sort(); result }
        if func == "sorted" && args.len() == 1 {
            let iter_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort();
                    __sorted_result
                }
            });
        }

        // DEPYLER-0191: Handle reversed(iterable) → iterable.into_iter().rev().collect()
        if func == "reversed" && args.len() == 1 {
            let iter_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! {
                {
                    let mut __reversed_result = #iter_expr.clone();
                    __reversed_result.reverse();
                    __reversed_result
                }
            });
        }

        // DEPYLER-0022: Handle memoryview(data) → data (identity/no-op)
        // Rust byte slices (&[u8]) already provide memoryview functionality (zero-copy view)
        // Python's memoryview provides a buffer interface - Rust slices are already references
        if func == "memoryview" && args.len() == 1 {
            return args[0].to_rust_expr(self.ctx);
        }

        // DEPYLER-0247: Handle sum(iterable) → iterable.iter().sum::<T>()
        // Need turbofish type annotation to help Rust's type inference
        // DEPYLER-0307 Fix #2: Handle sum(range(...))
        // Ranges in Rust (0..n) are already iterators - don't call .iter() on them
        // DEPYLER-0307 Phase 2 Fix: Wrap range in parentheses for correct precedence
        if func == "sum" && args.len() == 1 {
            if let HirExpr::Call {
                func: range_func, ..
            } = &args[0]
            {
                if range_func == "range" {
                    let range_expr = args[0].to_rust_expr(self.ctx)?;

                    let target_type = self
                        .ctx
                        .current_return_type
                        .as_ref()
                        .and_then(|t| match t {
                            Type::Int => Some(quote! { i32 }),
                            Type::Float => Some(quote! { f64 }),
                            _ => None,
                        })
                        .unwrap_or_else(|| quote! { i32 });

                    // Wrap range in parentheses to fix precedence: (0..n).sum() not 0..n.sum()
                    return Ok(parse_quote! { (#range_expr).sum::<#target_type>() });
                }
            }

            // DEPYLER-0303 Phase 3 Fix #8: Optimize sum(d.values()) and sum(d.keys())
            // .values()/.keys() already return Vec, but we can optimize by using the iterator directly
            // This avoids .collect::<Vec<_>>().iter() pattern
            if let HirExpr::MethodCall {
                object,
                method,
                args: method_args,
            } = &args[0]
            {
                if (method == "values" || method == "keys") && method_args.is_empty() {
                    let object_expr = object.to_rust_expr(self.ctx)?;

                    let target_type = self
                        .ctx
                        .current_return_type
                        .as_ref()
                        .and_then(|t| match t {
                            Type::Int => Some(quote! { i32 }),
                            Type::Float => Some(quote! { f64 }),
                            _ => None,
                        })
                        .unwrap_or_else(|| quote! { i32 });

                    // Use .values().cloned().sum() directly - skip the .collect()
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    return Ok(parse_quote! {
                        #object_expr.#method_ident().cloned().sum::<#target_type>()
                    });
                }
            }

            // Default: assume iterable that needs .iter()
            let iter_expr = args[0].to_rust_expr(self.ctx)?;

            // Infer the target type from return type context
            let target_type = self
                .ctx
                .current_return_type
                .as_ref()
                .and_then(|t| match t {
                    Type::Int => Some(quote! { i32 }),
                    Type::Float => Some(quote! { f64 }),
                    _ => None,
                })
                .unwrap_or_else(|| quote! { i32 });

            return Ok(parse_quote! { #iter_expr.iter().sum::<#target_type>() });
        }

        // DEPYLER-0307 Fix #3: Handle max(a, b) with 2 arguments → std::cmp::max(a, b)
        if func == "max" && args.len() == 2 {
            let arg1 = args[0].to_rust_expr(self.ctx)?;
            let arg2 = args[1].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { std::cmp::max(#arg1, #arg2) });
        }

        // DEPYLER-0193: Handle max(iterable) → iterable.iter().copied().max().unwrap()
        if func == "max" && args.len() == 1 {
            let iter_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { *#iter_expr.iter().max().unwrap() });
        }

        // DEPYLER-0307 Fix #3: Handle min(a, b) with 2 arguments → std::cmp::min(a, b)
        if func == "min" && args.len() == 2 {
            let arg1 = args[0].to_rust_expr(self.ctx)?;
            let arg2 = args[1].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { std::cmp::min(#arg1, #arg2) });
        }

        // DEPYLER-0194: Handle min(iterable) → iterable.iter().copied().min().unwrap()
        if func == "min" && args.len() == 1 {
            let iter_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { *#iter_expr.iter().min().unwrap() });
        }

        // DEPYLER-0248: Handle abs(value) → value.abs()
        if func == "abs" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #value_expr.abs() });
        }

        // DEPYLER-0307 Fix #1: Handle any() with generator expressions
        // Generator expressions (e.g., any(n > 0 for n in numbers)) return iterators
        // Don't call .iter() on them - call .any() directly
        if func == "any" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.any(|x| x) });
        }

        // DEPYLER-0249: Handle any(iterable) → iterable.iter().any(|&x| x)
        if func == "any" && args.len() == 1 {
            let iter_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #iter_expr.iter().any(|&x| x) });
        }

        // DEPYLER-0307 Fix #1: Handle all() with generator expressions
        // Generator expressions (e.g., all(n > 0 for n in numbers)) return iterators
        // Don't call .iter() on them - call .all() directly
        if func == "all" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.all(|x| x) });
        }

        // DEPYLER-0250: Handle all(iterable) → iterable.iter().all(|&x| x)
        if func == "all" && args.len() == 1 {
            let iter_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #iter_expr.iter().all(|&x| x) });
        }

        // DEPYLER-0251: Handle round(value) → value.round()
        if func == "round" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #value_expr.round() });
        }

        // DEPYLER-0252: Handle pow(base, exp) → base.pow(exp as u32)
        // Rust's pow() requires u32 exponent, so we cast
        if func == "pow" && args.len() == 2 {
            let base_expr = args[0].to_rust_expr(self.ctx)?;
            let exp_expr = args[1].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #base_expr.pow(#exp_expr as u32) });
        }

        // DEPYLER-0253: Handle chr(code) → char::from_u32(code as u32).unwrap().to_string()
        if func == "chr" && args.len() == 1 {
            let code_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { char::from_u32(#code_expr as u32).unwrap().to_string() });
        }

        // DEPYLER-0254: Handle ord(char) → char.chars().next().unwrap() as u32
        if func == "ord" && args.len() == 1 {
            let char_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #char_expr.chars().next().unwrap() as u32 });
        }

        // DEPYLER-0255: Handle bool(value) → value != 0
        if func == "bool" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #value_expr != 0 });
        }

        // Handle enumerate(items) → items.into_iter().enumerate()
        if func == "enumerate" && args.len() == 1 {
            let items_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #items_expr.into_iter().enumerate() });
        }

        // Handle zip(a, b, ...) → a.into_iter().zip(b.into_iter())...
        // DEPYLER-0303 Phase 3 Fix #6: Use .into_iter() for owned collections
        // When zip() receives function parameters of type Vec<T>, we need to consume them
        // to yield owned values, not references. This is critical for dict(zip(...)) patterns.
        if func == "zip" && args.len() >= 2 {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;

            // Determine if we should use .into_iter() or .iter()
            // Use .into_iter() if all arguments are owned collections (Vec, not slices)
            let use_into_iter = args.iter().all(|arg| self.is_owned_collection(arg));

            // Start with first.into_iter() or first.iter()
            let first = &arg_exprs[0];
            let mut chain: syn::Expr = if use_into_iter {
                parse_quote! { #first.into_iter() }
            } else {
                parse_quote! { #first.iter() }
            };

            // Chain .zip() for each subsequent argument
            for arg in &arg_exprs[1..] {
                chain = if use_into_iter {
                    parse_quote! { #chain.zip(#arg.into_iter()) }
                } else {
                    parse_quote! { #chain.zip(#arg.iter()) }
                };
            }

            return Ok(chain);
        }

        // DEPYLER-0269: Handle isinstance(value, type) → true
        // In statically-typed Rust, type system guarantees make runtime checks unnecessary
        // isinstance(x, T) where x: T is always true at compile-time
        if func == "isinstance" && args.len() == 2 {
            // Return literal true since Rust's type system guarantees correctness
            return Ok(parse_quote! { true });
        }

        // DEPYLER-0230: Check if func is a user-defined class before treating as builtin
        let is_user_class = self.ctx.class_names.contains(func);

        // DEPYLER-0234: For user-defined class constructors, convert string literals to String
        // This fixes "expected String, found &str" errors when calling constructors
        let arg_exprs: Vec<syn::Expr> = if is_user_class {
            args.iter()
                .map(|arg| {
                    let expr = arg.to_rust_expr(self.ctx)?;
                    // Wrap string literals with .to_string()
                    if matches!(arg, HirExpr::Literal(Literal::String(_))) {
                        Ok(parse_quote! { #expr.to_string() })
                    } else {
                        Ok(expr)
                    }
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            args.iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?
        };

        match func {
            // Python built-in type conversions → Rust casting
            "int" => self.convert_int_cast(args, &arg_exprs),
            "float" => self.convert_float_cast(&arg_exprs),
            "str" => self.convert_str_conversion(&arg_exprs),
            "bool" => self.convert_bool_cast(&arg_exprs),
            // Other built-in functions
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            // DEPYLER-0171, 0172, 0173, 0174: Collection conversion builtins
            // DEPYLER-0230: Only treat as builtin if not a user-defined class
            "Counter" if !is_user_class => self.convert_counter_builtin(&arg_exprs),
            "dict" if !is_user_class => self.convert_dict_builtin(&arg_exprs),
            "deque" if !is_user_class => self.convert_deque_builtin(&arg_exprs),
            "list" if !is_user_class => self.convert_list_builtin(&arg_exprs),
            _ => self.convert_generic_call(func, &arg_exprs),
        }
    }

    fn try_convert_map_with_zip(&mut self, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
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

            // Create lambda parameter pattern
            let param_idents: Vec<syn::Ident> = params
                .iter()
                .map(|p| syn::Ident::new(p, proc_macro2::Span::call_site()))
                .collect();

            // Convert lambda body
            let body_expr = body.to_rust_expr(self.ctx)?;

            // Handle based on number of iterables
            if num_iterables == 1 {
                // Single iterable: iterable.iter().map(|x| ...).collect()
                let iter_expr = &iterable_exprs[0];
                let param = &param_idents[0];
                Ok(Some(parse_quote! {
                    #iter_expr.iter().map(|#param| #body_expr).collect::<Vec<_>>()
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

    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];

        // DEPYLER-0276: Keep cast for CSE compatibility
        // Python's len() returns int (maps to i32)
        // Rust's .len() returns usize, so we cast to i32
        // CSE optimization runs before return statement processing, so we need the cast here
        // to avoid type mismatches when CSE extracts len() into a temporary variable
        Ok(parse_quote! { #arg.len() as i32 })
    }

    fn convert_int_cast(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.is_empty() || arg_exprs.len() > 2 {
            bail!("int() requires 1-2 arguments");
        }
        let arg = &arg_exprs[0];

        // Python int() serves four purposes:
        // 1. Parse strings to integers (requires .parse())
        // 2. Convert floats to integers (truncation via as i32)
        // 3. Convert bools to integers (False→0, True→1 via as i32)
        // 4. Ensure integer type for indexing (via as i32)
        //
        // DEPYLER-0307 Fix #7: Check if variable is String type
        // String variables need .parse().unwrap_or_default() not 'as i32' cast
        //
        // Strategy:
        // - For String variables/params → .parse().unwrap_or_default()
        // - For known bool expressions → as i32 cast
        // - For integer literals → no cast needed
        // - For other variables → as i32 cast conservatively
        if !hir_args.is_empty() {
            match &hir_args[0] {
                // Integer literals don't need casting
                HirExpr::Literal(Literal::Int(_)) => return Ok(arg.clone()),

                // DEPYLER-0307 Fix #7: Check if variable is String type
                HirExpr::Var(var_name) => {
                    // Check if this variable is known to be String type
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        if matches!(var_type, Type::String) {
                            // String → int requires parsing, not casting
                            return Ok(parse_quote! { #arg.parse().unwrap_or_default() });
                        }
                    }
                    // Default: use as i32 cast for other types
                    return Ok(parse_quote! { (#arg) as i32 });
                }

                // Check if it's a known bool expression
                expr => {
                    if let Some(is_bool) = self.is_bool_expr(expr) {
                        if is_bool {
                            return Ok(parse_quote! { (#arg) as i32 });
                        }
                    }
                    // For other complex expressions, apply cast conservatively
                    return Ok(parse_quote! { (#arg) as i32 });
                }
            }
        }

        // Default: cast for safety
        Ok(parse_quote! { (#arg) as i32 })
    }

    fn convert_float_cast(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("float() requires exactly one argument");
        }
        let arg = &args[0];
        Ok(parse_quote! { (#arg) as f64 })
    }

    fn convert_str_conversion(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("str() requires exactly one argument");
        }
        let arg = &args[0];
        Ok(parse_quote! { #arg.to_string() })
    }

    fn convert_bool_cast(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("bool() requires exactly one argument");
        }
        let arg = &args[0];
        // In Python, bool(x) checks truthiness
        // In Rust, we cast to bool or use appropriate conversion
        Ok(parse_quote! { (#arg) as bool })
    }

    fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        match args.len() {
            1 => {
                let end = &args[0];
                Ok(parse_quote! { 0..#end })
            }
            2 => {
                let start = &args[0];
                let end = &args[1];
                Ok(parse_quote! { #start..#end })
            }
            3 => self.convert_range_with_step(&args[0], &args[1], &args[2]),
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_range_with_step(
        &self,
        start: &syn::Expr,
        end: &syn::Expr,
        step: &syn::Expr,
    ) -> Result<syn::Expr> {
        // Check if step is negative by looking at the expression
        let is_negative_step =
            matches!(step, syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)));

        if is_negative_step {
            self.convert_range_negative_step(start, end, step)
        } else {
            self.convert_range_positive_step(start, end, step)
        }
    }

    fn convert_range_negative_step(
        &self,
        start: &syn::Expr,
        end: &syn::Expr,
        step: &syn::Expr,
    ) -> Result<syn::Expr> {
        // For negative steps, we need to reverse the range
        // Python: range(10, 0, -1) → Rust: (0..10).rev()
        Ok(parse_quote! {
            {
                let step = (#step).abs() as usize;
                if step == 0 {
                    panic!("range() arg 3 must not be zero");
                }
                if step == 1 {
                    (#end..#start).rev()
                } else {
                    (#end..#start).rev().step_by(step)
                }
            }
        })
    }

    fn convert_range_positive_step(
        &self,
        start: &syn::Expr,
        end: &syn::Expr,
        step: &syn::Expr,
    ) -> Result<syn::Expr> {
        // Positive step - check for zero
        Ok(parse_quote! {
            {
                let step = #step as usize;
                if step == 0 {
                    panic!("range() arg 3 must not be zero");
                }
                (#start..#end).step_by(step)
            }
        })
    }

    fn convert_array_init_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        _arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }

        // Extract size from first argument if it's a literal
        if let HirExpr::Literal(Literal::Int(size)) = &args[0] {
            if *size > 0 && *size <= 32 {
                self.convert_array_small_literal(func, args, *size)
            } else {
                self.convert_array_large_literal(func, args)
            }
        } else {
            self.convert_array_dynamic_size(func, args)
        }
    }

    fn convert_array_small_literal(
        &mut self,
        func: &str,
        args: &[HirExpr],
        size: i64,
    ) -> Result<syn::Expr> {
        let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
        match func {
            "zeros" => Ok(parse_quote! { [0; #size_lit] }),
            "ones" => Ok(parse_quote! { [1; #size_lit] }),
            "full" => {
                if args.len() >= 2 {
                    let value = args[1].to_rust_expr(self.ctx)?;
                    Ok(parse_quote! { [#value; #size_lit] })
                } else {
                    bail!("full() requires a value argument");
                }
            }
            _ => unreachable!(),
        }
    }

    fn convert_array_large_literal(&mut self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        let size_expr = args[0].to_rust_expr(self.ctx)?;
        match func {
            "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
            "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
            "full" => {
                if args.len() >= 2 {
                    let value = args[1].to_rust_expr(self.ctx)?;
                    Ok(parse_quote! { vec![#value; #size_expr as usize] })
                } else {
                    bail!("full() requires a value argument");
                }
            }
            _ => unreachable!(),
        }
    }

    fn convert_array_dynamic_size(&mut self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        let size_expr = args[0].to_rust_expr(self.ctx)?;
        match func {
            "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
            "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
            "full" => {
                if args.len() >= 2 {
                    let value = args[1].to_rust_expr(self.ctx)?;
                    Ok(parse_quote! { vec![#value; #size_expr as usize] })
                } else {
                    bail!("full() requires a value argument");
                }
            }
            _ => unreachable!(),
        }
    }

    fn convert_set_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        if args.is_empty() {
            // Empty set: set()
            Ok(parse_quote! { HashSet::new() })
        } else if args.len() == 1 {
            // Set from iterable: set([1, 2, 3])
            let arg = &args[0];
            Ok(parse_quote! {
                #arg.into_iter().collect::<HashSet<_>>()
            })
        } else {
            bail!("set() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_frozenset_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        if args.is_empty() {
            // Empty frozenset: frozenset()
            // In Rust, we can use Arc<HashSet> to make it immutable
            Ok(parse_quote! { std::sync::Arc::new(HashSet::new()) })
        } else if args.len() == 1 {
            // Frozenset from iterable: frozenset([1, 2, 3])
            let arg = &args[0];
            Ok(parse_quote! {
                std::sync::Arc::new(#arg.into_iter().collect::<HashSet<_>>())
            })
        } else {
            bail!(
                "frozenset() takes at most 1 argument ({} given)",
                args.len()
            )
        }
    }

    // ========================================================================
    // DEPYLER-0171, 0172, 0173, 0174: Collection Conversion Builtins
    // ========================================================================

    fn convert_counter_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        // DEPYLER-0171: Counter(iterable) counts elements and creates HashMap
        self.ctx.needs_hashmap = true;
        if args.is_empty() {
            // Counter() with no args → empty HashMap
            Ok(parse_quote! { HashMap::new() })
        } else if args.len() == 1 {
            // Counter(iterable) → count elements using fold
            let arg = &args[0];
            Ok(parse_quote! {
                #arg.into_iter().fold(HashMap::new(), |mut acc, item| {
                    *acc.entry(item).or_insert(0) += 1;
                    acc
                })
            })
        } else {
            bail!("Counter() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_dict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        // DEPYLER-0172: dict() converts mapping/iterable to HashMap
        self.ctx.needs_hashmap = true;
        if args.is_empty() {
            // dict() with no args → empty HashMap
            Ok(parse_quote! { HashMap::new() })
        } else if args.len() == 1 {
            // dict(mapping) → convert to HashMap
            let arg = &args[0];
            Ok(parse_quote! {
                #arg.into_iter().collect::<HashMap<_, _>>()
            })
        } else {
            bail!("dict() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_deque_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        // DEPYLER-0173: deque(iterable) creates VecDeque from iterable
        self.ctx.needs_vecdeque = true;
        if args.is_empty() {
            // deque() with no args → empty VecDeque
            Ok(parse_quote! { VecDeque::new() })
        } else if args.len() == 1 {
            // deque(iterable) → VecDeque::from()
            let arg = &args[0];
            Ok(parse_quote! {
                VecDeque::from(#arg)
            })
        } else {
            bail!("deque() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_list_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        // DEPYLER-0174: list(iterable) converts iterable to Vec
        if args.is_empty() {
            // list() with no args → empty Vec
            Ok(parse_quote! { Vec::new() })
        } else if args.len() == 1 {
            let arg = &args[0];

            // DEPYLER-0177: Check if expression already collected
            // map(lambda...) already includes .collect(), don't add another
            if self.already_collected(arg) {
                Ok(arg.clone())
            } else if self.is_range_expr(arg) {
                // DEPYLER-0179: range(5) → (0..5).collect()
                Ok(parse_quote! {
                    (#arg).collect::<Vec<_>>()
                })
            } else if self.is_iterator_expr(arg) {
                // DEPYLER-0176: zip(), enumerate() return iterators
                // Don't add redundant .into_iter()
                Ok(parse_quote! {
                    #arg.collect::<Vec<_>>()
                })
            } else {
                // Regular iterable → collect to Vec
                Ok(parse_quote! {
                    #arg.into_iter().collect::<Vec<_>>()
                })
            }
        } else {
            bail!("list() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// Check if expression already ends with .collect()
    fn already_collected(&self, expr: &syn::Expr) -> bool {
        if let syn::Expr::MethodCall(method_call) = expr {
            method_call.method == "collect"
        } else {
            false
        }
    }

    /// Check if expression is a range (0..5, start..end, etc.)
    fn is_range_expr(&self, expr: &syn::Expr) -> bool {
        matches!(expr, syn::Expr::Range(_))
    }

    /// Check if expression is an iterator-producing expression
    fn is_iterator_expr(&self, expr: &syn::Expr) -> bool {
        // Check if it's a method call that returns an iterator
        if let syn::Expr::MethodCall(method_call) = expr {
            let method_name = method_call.method.to_string();
            matches!(
                method_name.as_str(),
                "iter"
                    | "iter_mut"
                    | "into_iter"
                    | "zip"
                    | "map"
                    | "filter"
                    | "enumerate"
                    | "chain"
                    | "flat_map"
                    | "take"
                    | "skip"
                    | "collect"
            )
        } else {
            false
        }
    }

    fn convert_generic_call(&self, func: &str, args: &[syn::Expr]) -> Result<syn::Expr> {
        // Special case: Python print() → Rust println!()
        if func == "print" {
            return if args.is_empty() {
                // print() with no arguments → println!()
                Ok(parse_quote! { println!() })
            } else if args.len() == 1 {
                // print(x) → println!("{}", x)
                let arg = &args[0];
                Ok(parse_quote! { println!("{}", #arg) })
            } else {
                // print(a, b, c) → println!("{} {} {}", a, b, c)
                let format_str = vec!["{}"; args.len()].join(" ");
                Ok(parse_quote! { println!(#format_str, #(#args),*) })
            };
        }

        // Check if this is an imported function
        if let Some(rust_path) = self.ctx.imported_items.get(func) {
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
            if args.is_empty() {
                return Ok(parse_quote! { #path() });
            } else {
                return Ok(parse_quote! { #path(#(#args),*) });
            }
        }

        // Check if this might be a constructor call (capitalized name)
        if func
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // Treat as constructor call - ClassName::new(args)
            let class_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
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
                Ok(parse_quote! { #class_ident::new(#(#args),*) })
            }
        } else {
            // Regular function call
            let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());

            // DEPYLER-0287 Fix Part 1: Borrow Vec arguments for functions expecting &Vec
            // When passing a local Vec to a function, automatically borrow it
            // This handles cases like: sum_list_recursive(rest) where rest is Vec but param is &Vec
            //
            // IMPORTANT: Only borrow expressions that create new Vecs (like .to_vec())
            // Do NOT borrow simple identifiers or literals (those are Copy types like i32)
            let borrowed_args: Vec<syn::Expr> = args
                .iter()
                .map(|arg_expr| {
                    // Only borrow if the expression creates a new Vec via .to_vec()
                    let expr_string = quote! { #arg_expr }.to_string();
                    if expr_string.contains("to_vec") {
                        // Borrow the Vec that's being created
                        parse_quote! { &#arg_expr }
                    } else {
                        // Don't borrow - it's likely a Copy type (i32, bool, etc.)
                        arg_expr.clone()
                    }
                })
                .collect();

            // DEPYLER-0287 Fix Part 2: Add `?` operator for recursive calls in Result-returning functions
            // If we're in a function that can fail (returns Result), and we're calling another
            // function (potentially recursive), propagate errors with `?` operator.
            // This is needed for recursive functions that perform operations like list indexing
            // which return Result<T, E>.
            if self.ctx.current_function_can_fail {
                Ok(parse_quote! { #func_ident(#(#borrowed_args),*)? })
            } else {
                Ok(parse_quote! { #func_ident(#(#borrowed_args),*) })
            }
        }
    }

    // ========================================================================
    // DEPYLER-0142 Phase 1: Preamble Helpers
    // ========================================================================

    /// Try to convert classmethod call (cls.method())
    #[inline]
    fn try_convert_classmethod(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.ctx.is_classmethod {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(Some(parse_quote! { Self::#method_ident(#(#arg_exprs),*) }));
            }
        }
        Ok(None)
    }

    /// DEPYLER-0021: Handle struct module methods (pack, unpack, calcsize)
    /// Only supports format codes 'i' (signed 32-bit int) and 'ii' (two ints)
    fn try_convert_struct_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "pack" => {
                if args.is_empty() {
                    bail!("struct.pack() requires at least a format argument");
                }

                // First arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.pack() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    if count != args.len() - 1 {
                        bail!(
                            "struct.pack() format '{}' expects {} values, got {}",
                            format,
                            count,
                            args.len() - 1
                        );
                    }

                    // Convert value arguments
                    let value_exprs: Vec<syn::Expr> = args[1..]
                        .iter()
                        .map(|arg| arg.to_rust_expr(self.ctx))
                        .collect::<Result<Vec<_>>>()?;

                    if count == 1 {
                        // struct.pack('i', value) → (value as i32).to_le_bytes().to_vec()
                        let val = &value_exprs[0];
                        Ok(Some(parse_quote! {
                            (#val as i32).to_le_bytes().to_vec()
                        }))
                    } else {
                        // struct.pack('ii', a, b) → { let mut v = Vec::new(); v.extend_from_slice(&(a as i32).to_le_bytes()); ... }
                        Ok(Some(parse_quote! {
                            {
                                let mut __struct_pack_result = Vec::new();
                                #(__struct_pack_result.extend_from_slice(&(#value_exprs as i32).to_le_bytes());)*
                                __struct_pack_result
                            }
                        }))
                    }
                } else {
                    bail!("struct.pack() requires string literal format (dynamic formats not supported)");
                }
            }
            "unpack" => {
                if args.len() != 2 {
                    bail!("struct.unpack() requires exactly 2 arguments (format, bytes)");
                }

                // First arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.unpack() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    let bytes_expr = args[1].to_rust_expr(self.ctx)?;

                    if count == 1 {
                        // struct.unpack('i', bytes) → (i32::from_le_bytes(bytes[0..4].try_into().unwrap()),)
                        Ok(Some(parse_quote! {
                            (i32::from_le_bytes(#bytes_expr[0..4].try_into().unwrap()),)
                        }))
                    } else if count == 2 {
                        // struct.unpack('ii', bytes) → (i32::from_le_bytes(...), i32::from_le_bytes(...))
                        Ok(Some(parse_quote! {
                            (
                                i32::from_le_bytes(#bytes_expr[0..4].try_into().unwrap()),
                                i32::from_le_bytes(#bytes_expr[4..8].try_into().unwrap()),
                            )
                        }))
                    } else {
                        bail!(
                            "struct.unpack() only supports 'i' and 'ii' formats (got {} ints)",
                            count
                        );
                    }
                } else {
                    bail!("struct.unpack() requires string literal format (dynamic formats not supported)");
                }
            }
            "calcsize" => {
                if args.len() != 1 {
                    bail!("struct.calcsize() requires exactly 1 argument");
                }

                // Arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.calcsize() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    let size = (count * 4) as i32;
                    Ok(Some(parse_quote! { #size }))
                } else {
                    bail!("struct.calcsize() requires string literal format (dynamic formats not supported)");
                }
            }
            _ => {
                bail!("struct.{} not implemented", method);
            }
        }
    }

    /// Try to convert module method call (e.g., os.getcwd())
    #[inline]
    fn try_convert_module_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if let HirExpr::Var(module_name) = object {
            // DEPYLER-0021: Handle struct module (pack, unpack, calcsize)
            if module_name == "struct" {
                return self.try_convert_struct_method(method, args);
            }

            let rust_name_opt = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| mapping.item_map.get(method).cloned());

            if let Some(rust_name) = rust_name_opt {
                // Convert args
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                // Build the Rust function path
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                let mut path = quote! { std };
                for part in path_parts {
                    let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                    path = quote! { #path::#part_ident };
                }

                // Special handling for certain functions
                let result = match rust_name.as_str() {
                    "env::current_dir" => {
                        // current_dir returns Result<PathBuf>, we need to convert to String
                        parse_quote! {
                            #path().unwrap().to_string_lossy().to_string()
                        }
                    }
                    "Regex::new" => {
                        // re.compile(pattern) -> Regex::new(pattern)
                        if arg_exprs.is_empty() {
                            bail!("re.compile() requires a pattern argument");
                        }
                        let pattern = &arg_exprs[0];
                        parse_quote! {
                            regex::Regex::new(#pattern).unwrap()
                        }
                    }
                    _ => {
                        if arg_exprs.is_empty() {
                            parse_quote! { #path() }
                        } else {
                            parse_quote! { #path(#(#arg_exprs),*) }
                        }
                    }
                };
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    // ========================================================================
    // DEPYLER-0142 Phase 2: Category Handlers
    // ========================================================================

    /// Handle list methods (append, extend, pop, insert, remove)
    #[inline]
    fn convert_list_method(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        match method {
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            "extend" => {
                // DEPYLER-0292: Handle iterator conversion for extend()
                if arg_exprs.len() != 1 {
                    bail!("extend() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // extend() expects IntoIterator<Item = T>, but we often pass &Vec<T>
                // which gives IntoIterator<Item = &T>. Add .iter().cloned() to fix this.
                // Check if arg is a reference (most common case for function parameters)
                let arg_string = quote! { #arg }.to_string();
                if arg_string.contains("&") || !arg_string.contains(".into_iter()") {
                    // Likely a reference or direct variable - add iterator conversion
                    Ok(parse_quote! { #object_expr.extend(#arg.iter().cloned()) })
                } else {
                    // Already an iterator or owned value
                    Ok(parse_quote! { #object_expr.extend(#arg) })
                }
            }
            "pop" => {
                // DEPYLER-0210 FIX: Handle pop() for sets, dicts, and lists
                // Disambiguate based on argument count FIRST, then object type

                if arg_exprs.len() == 2 {
                    // Only dict.pop(key, default) takes 2 arguments
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    // DEPYLER-0303: Don't add & for string literals or variables
                    let needs_ref = !hir_args.is_empty()
                        && !matches!(
                            hir_args[0],
                            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                        );
                    if needs_ref {
                        Ok(parse_quote! { #object_expr.remove(&#key).unwrap_or(#default) })
                    } else {
                        Ok(parse_quote! { #object_expr.remove(#key).unwrap_or(#default) })
                    }
                } else if arg_exprs.len() > 2 {
                    bail!("pop() takes at most 2 arguments");
                } else if self.is_set_expr(object) {
                    // Set.pop() - must have 0 arguments
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments for sets");
                    }
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else if self.is_dict_expr(object) {
                    // Dict literal - pop(key) with 1 argument
                    if arg_exprs.len() != 1 {
                        bail!("dict literal pop() requires exactly 1 argument (key)");
                    }
                    let key = &arg_exprs[0];
                    // DEPYLER-0303: Don't add & for string literals or variables
                    let needs_ref = !hir_args.is_empty()
                        && !matches!(
                            hir_args[0],
                            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                        );
                    if needs_ref {
                        Ok(parse_quote! { #object_expr.remove(&#key).expect("KeyError: key not found") })
                    } else {
                        Ok(parse_quote! { #object_expr.remove(#key).expect("KeyError: key not found") })
                    }
                } else if arg_exprs.is_empty() {
                    // List.pop() with no arguments - remove last element
                    Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                } else {
                    // 1 argument: could be list.pop(index) OR dict.pop(key)
                    // Use multiple heuristics to disambiguate:
                    let arg = &arg_exprs[0];

                    // Heuristic 1: Explicit list literal
                    let is_list = self.is_list_expr(object);

                    // Heuristic 2: Explicit dict literal
                    let is_dict = self.is_dict_expr(object);

                    // Heuristic 3: Integer argument suggests list index
                    let arg_is_int = !hir_args.is_empty()
                        && matches!(hir_args[0], HirExpr::Literal(crate::hir::Literal::Int(_)));

                    if is_list || (!is_dict && arg_is_int) {
                        // List.pop(index) - use Vec::remove() which takes usize by value
                        Ok(parse_quote! { #object_expr.remove(#arg as usize) })
                    } else {
                        // dict.pop(key) - HashMap::remove() takes &K by reference
                        // DEPYLER-0303: Don't add & for string literals or variables
                        let needs_ref = !hir_args.is_empty()
                            && !matches!(
                                hir_args[0],
                                HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                            );
                        if needs_ref {
                            Ok(parse_quote! { #object_expr.remove(&#arg).expect("KeyError: key not found") })
                        } else {
                            Ok(parse_quote! { #object_expr.remove(#arg).expect("KeyError: key not found") })
                        }
                    }
                }
            }
            "insert" => {
                if arg_exprs.len() != 2 {
                    bail!("insert() requires exactly two arguments");
                }
                let index = &arg_exprs[0];
                let value = &arg_exprs[1];
                Ok(parse_quote! { #object_expr.insert(#index as usize, #value) })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                if self.is_set_expr(object) {
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#value) {
                            panic!("KeyError: element not in set");
                        }
                    })
                } else {
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#value) {
                            #object_expr.remove(pos)
                        } else {
                            panic!("ValueError: list.remove(x): x not in list")
                        }
                    })
                }
            }
            "index" => {
                // Python: list.index(value) -> returns index of first occurrence
                // Rust: list.iter().position(|x| x == &value).ok_or(...)
                if arg_exprs.len() != 1 {
                    bail!("index() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter()
                        .position(|x| x == &#value)
                        .map(|i| i as i32)
                        .expect("ValueError: value is not in list")
                })
            }
            "count" => {
                // Python: list.count(value) -> counts occurrences
                // Rust: list.iter().filter(|x| **x == value).count()
                if arg_exprs.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter().filter(|x| **x == #value).count() as i32
                })
            }
            "copy" => {
                // Python: list.copy() -> shallow copy OR copy.copy(x) -> shallow copy
                // Rust: list.clone() OR x.clone()
                // DEPYLER-0024 FIX: Handle copy.copy(x) from copy module
                if arg_exprs.len() == 1 {
                    // This is copy.copy(x) from the copy module being misparsed as method call
                    // Just clone the argument directly
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! { #arg.clone() });
                }
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                // This is list.copy() method - clone the list
                Ok(parse_quote! { #object_expr.clone() })
            }
            "clear" => {
                // Python: list.clear() -> removes all elements
                // Rust: list.clear()
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "reverse" => {
                // Python: list.reverse() -> reverses in place
                // Rust: list.reverse()
                if !arg_exprs.is_empty() {
                    bail!("reverse() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.reverse() })
            }
            "sort" => {
                // Python: list.sort() -> sorts in place
                // Rust: list.sort()
                if !arg_exprs.is_empty() {
                    bail!("sort() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.sort() })
            }
            _ => bail!("Unknown list method: {}", method),
        }
    }

    /// Handle dict methods (get, keys, values, items, update)
    #[inline]
    fn convert_dict_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        match method {
            "get" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    // DEPYLER-0303 Phase 2: Check if return type is Optional
                    // If function returns Option<T>, dict.get() should return Option directly
                    // Otherwise, unwrap with unwrap_or_default()
                    let returns_option = matches!(
                        self.ctx.current_return_type.as_ref(),
                        Some(Type::Optional(_))
                    );

                    // DEPYLER-0227: String literals need & prefix, but variables with &str type don't
                    // Check if key is a string literal that was converted to .to_string()
                    let key_expr: syn::Expr =
                        if matches!(hir_args.first(), Some(HirExpr::Literal(Literal::String(_)))) {
                            // String literal - add & to borrow the String
                            parse_quote! { &#key }
                        } else {
                            // Variable or other expression - already properly typed
                            parse_quote! { #key }
                        };

                    if returns_option {
                        // Return Option directly - don't unwrap
                        Ok(parse_quote! { #object_expr.get(#key_expr).cloned() })
                    } else {
                        // Unwrap with default for non-Optional returns
                        Ok(parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or_default() })
                    }
                } else if arg_exprs.len() == 2 {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    // DEPYLER-0227: String literals need & prefix, but variables with &str type don't
                    let key_expr: syn::Expr =
                        if matches!(hir_args.first(), Some(HirExpr::Literal(Literal::String(_)))) {
                            // String literal - add & to borrow the String
                            parse_quote! { &#key }
                        } else {
                            // Variable or other expression - already properly typed
                            parse_quote! { #key }
                        };
                    Ok(parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) })
                } else {
                    bail!("get() requires 1 or 2 arguments");
                }
            }
            "keys" => {
                if !arg_exprs.is_empty() {
                    bail!("keys() takes no arguments");
                }
                // DEPYLER-0303 Phase 3 Fix #8: Return Vec for compatibility
                // .keys() returns an iterator, but Python's dict.keys() returns a list-like view
                // We collect to Vec for better ergonomics (indexing, len(), etc.)
                Ok(parse_quote! { #object_expr.keys().cloned().collect::<Vec<_>>() })
            }
            "values" => {
                if !arg_exprs.is_empty() {
                    bail!("values() takes no arguments");
                }
                // DEPYLER-0303 Phase 3 Fix #8: Return Vec for compatibility
                // However, this causes redundant .collect().iter() in sum(d.values())
                // TODO: Consider context-aware return type (Vec vs Iterator)
                Ok(parse_quote! { #object_expr.values().cloned().collect::<Vec<_>>() })
            }
            "items" => {
                if !arg_exprs.is_empty() {
                    bail!("items() takes no arguments");
                }
                Ok(
                    parse_quote! { #object_expr.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
                )
            }
            "update" => {
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! {
                    for (k, v) in #arg {
                        #object_expr.insert(k, v);
                    }
                })
            }
            "setdefault" => {
                // dict.setdefault(key, default) - get or insert with default
                // Python: dict.setdefault(key, default) returns value at key, or inserts default and returns it
                // Rust: entry().or_insert(default).clone()
                if arg_exprs.len() != 2 {
                    bail!("setdefault() requires exactly 2 arguments (key, default)");
                }
                let key = &arg_exprs[0];
                let default = &arg_exprs[1];
                Ok(parse_quote! {
                    #object_expr.entry(#key).or_insert(#default).clone()
                })
            }
            "popitem" => {
                // dict.popitem() - remove and return arbitrary (key, value) pair
                // Python: dict.popitem() removes and returns arbitrary item, or raises KeyError
                // Rust: iter().next() to get first item, then remove it
                if !arg_exprs.is_empty() {
                    bail!("popitem() takes no arguments");
                }
                Ok(parse_quote! {
                    {
                        let key = #object_expr.keys().next().cloned()
                            .expect("KeyError: popitem(): dictionary is empty");
                        let value = #object_expr.remove(&key)
                            .expect("KeyError: key disappeared");
                        (key, value)
                    }
                })
            }
            _ => bail!("Unknown dict method: {}", method),
        }
    }

    /// Handle string methods (upper, lower, strip, startswith, endswith, split, join, replace, find, count, isdigit, isalpha)
    #[inline]
    fn convert_string_method(
        &mut self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        match method {
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }
            "strip" => {
                if !arg_exprs.is_empty() {
                    bail!("strip() with arguments not supported in V1");
                }
                Ok(parse_quote! { #object_expr.trim().to_string() })
            }
            "startswith" => {
                if hir_args.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                // Extract bare string literal for Pattern trait compatibility
                let prefix = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! { #object_expr.starts_with(#prefix) })
            }
            "endswith" => {
                if hir_args.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                // Extract bare string literal for Pattern trait compatibility
                let suffix = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! { #object_expr.ends_with(#suffix) })
            }
            "split" => {
                if arg_exprs.is_empty() {
                    Ok(
                        parse_quote! { #object_expr.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    // DEPYLER-0225: Extract bare string literal for Pattern trait compatibility
                    let sep = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        _ => arg_exprs[0].clone(),
                    };
                    Ok(
                        parse_quote! { #object_expr.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() with maxsplit not supported in V1");
                }
            }
            "join" => {
                // DEPYLER-0196: sep.join(iterable) → iterable.join(sep)
                // Use bare string literal for separator without .to_string()
                if hir_args.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                // Extract bare string literal for separator
                let separator = match hir_object {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => object_expr.clone(),
                };
                Ok(parse_quote! { #iterable.join(#separator) })
            }
            "replace" => {
                // DEPYLER-0195: str.replace(old, new) → .replace(old, new)
                // DEPYLER-0301: str.replace(old, new, count) → .replacen(old, new, count)
                // Use bare string literals without .to_string() for correct types
                if hir_args.len() < 2 || hir_args.len() > 3 {
                    bail!("replace() requires 2 or 3 arguments");
                }
                // Extract bare string literals for arguments
                let old = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                let new = match &hir_args[1] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[1].clone(),
                };

                if hir_args.len() == 3 {
                    // Python: str.replace(old, new, count)
                    // Rust: str.replacen(old, new, count as usize)
                    let count = &arg_exprs[2];
                    Ok(parse_quote! { #object_expr.replacen(#old, #new, #count as usize) })
                } else {
                    // Python: str.replace(old, new)
                    // Rust: str.replace(old, new) - replaces all
                    Ok(parse_quote! { #object_expr.replace(#old, #new) })
                }
            }
            "find" => {
                // DEPYLER-0197: str.find(sub) → .find(sub).map(|i| i as i32).unwrap_or(-1)
                // Python's find() returns -1 if not found, Rust's returns Option<usize>
                if hir_args.len() != 1 {
                    bail!("find() requires exactly one argument");
                }
                // Extract bare string literal for Pattern trait compatibility
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.find(#substring)
                        .map(|i| i as i32)
                        .unwrap_or(-1)
                })
            }
            "count" => {
                // DEPYLER-0198/0226: str.count(sub) → .matches(sub).count() as i32
                // Extract bare string literal for Pattern trait compatibility
                if hir_args.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! { #object_expr.matches(#substring).count() as i32 })
            }
            "isdigit" => {
                // DEPYLER-0199: str.isdigit() → .chars().all(|c| c.is_numeric())
                if !arg_exprs.is_empty() {
                    bail!("isdigit() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_numeric()) })
            }
            "isalpha" => {
                // DEPYLER-0200: str.isalpha() → .chars().all(|c| c.is_alphabetic())
                if !arg_exprs.is_empty() {
                    bail!("isalpha() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_alphabetic()) })
            }
            "lstrip" => {
                // DEPYLER-0302: str.lstrip() → .trim_start()
                if !arg_exprs.is_empty() {
                    bail!("lstrip() with arguments not supported in V1");
                }
                Ok(parse_quote! { #object_expr.trim_start().to_string() })
            }
            "rstrip" => {
                // DEPYLER-0302: str.rstrip() → .trim_end()
                if !arg_exprs.is_empty() {
                    bail!("rstrip() with arguments not supported in V1");
                }
                Ok(parse_quote! { #object_expr.trim_end().to_string() })
            }
            "isalnum" => {
                // DEPYLER-0302: str.isalnum() → .chars().all(|c| c.is_alphanumeric())
                if !arg_exprs.is_empty() {
                    bail!("isalnum() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_alphanumeric()) })
            }
            "title" => {
                // DEPYLER-0302 Phase 2: str.title() → custom title case implementation
                // Python's title() capitalizes the first letter of each word
                if !arg_exprs.is_empty() {
                    bail!("title() takes no arguments");
                }
                Ok(parse_quote! {
                    #object_expr
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
            }
            _ => bail!("Unknown string method: {}", method),
        }
    }

    /// Handle set methods (add, discard, clear)
    #[inline]
    fn convert_set_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "remove" => {
                // DEPYLER-0224: Set.remove(value) - remove value or panic if not found
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! {
                    if !#object_expr.remove(&#arg) {
                        panic!("KeyError: element not in set")
                    }
                })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.remove(&#arg) })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "update" => {
                // DEPYLER-0211 FIX: Set.update(other) - add all elements from other set
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    for item in #other {
                        #object_expr.insert(item);
                    }
                })
            }
            "intersection_update" => {
                // DEPYLER-0212 FIX: Set.intersection_update(other) - keep only common elements
                // Note: This generates an expression that returns (), suitable for ExprStmt
                if arg_exprs.len() != 1 {
                    bail!("intersection_update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let temp: std::collections::HashSet<_> = #object_expr.intersection(&#other).cloned().collect();
                        #object_expr.clear();
                        #object_expr.extend(temp);
                    }
                })
            }
            "difference_update" => {
                // DEPYLER-0213 FIX: Set.difference_update(other) - remove elements in other
                // Note: This generates an expression that returns (), suitable for ExprStmt
                if arg_exprs.len() != 1 {
                    bail!("difference_update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let temp: std::collections::HashSet<_> = #object_expr.difference(&#other).cloned().collect();
                        #object_expr.clear();
                        #object_expr.extend(temp);
                    }
                })
            }
            "union" => {
                // Set.union(other) - return new set with elements from both sets
                if arg_exprs.len() != 1 {
                    bail!("union() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.union(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "intersection" => {
                // Set.intersection(other) - return new set with common elements
                if arg_exprs.len() != 1 {
                    bail!("intersection() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.intersection(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "difference" => {
                // Set.difference(other) - return new set with elements not in other
                if arg_exprs.len() != 1 {
                    bail!("difference() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.difference(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "symmetric_difference" => {
                // Set.symmetric_difference(other) - return new set with elements in either but not both
                if arg_exprs.len() != 1 {
                    bail!("symmetric_difference() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.symmetric_difference(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "issubset" => {
                // Set.issubset(other) - check if all elements are in other
                if arg_exprs.len() != 1 {
                    bail!("issubset() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_subset(&#other)
                })
            }
            "issuperset" => {
                // Set.issuperset(other) - check if contains all elements of other
                if arg_exprs.len() != 1 {
                    bail!("issuperset() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_superset(&#other)
                })
            }
            "isdisjoint" => {
                // Set.isdisjoint(other) - check if no common elements
                if arg_exprs.len() != 1 {
                    bail!("isdisjoint() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_disjoint(&#other)
                })
            }
            _ => bail!("Unknown set method: {}", method),
        }
    }

    /// Handle regex methods (findall)
    #[inline]
    fn convert_regex_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
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
            _ => bail!("Unknown regex method: {}", method),
        }
    }

    /// Convert instance method calls (main dispatcher)
    #[inline]
    fn convert_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0232 FIX: Check for user-defined class instances FIRST
        // User-defined classes can have methods with names like "add" that conflict with
        // built-in collection methods. We must prioritize user-defined methods.
        if self.is_class_instance(object) {
            // This is a user-defined class instance - use generic method call
            let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
            return Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) });
        }

        // DEPYLER-0211 FIX: Check object type first for ambiguous methods like update()
        // Both sets and dicts have update(), so we need to disambiguate

        // Check for set-specific context first
        if self.is_set_expr(object) {
            match method {
                "add"
                | "remove"
                | "discard"
                | "update"
                | "intersection_update"
                | "difference_update"
                | "union"
                | "intersection"
                | "difference"
                | "symmetric_difference"
                | "issubset"
                | "issuperset"
                | "isdisjoint" => {
                    return self.convert_set_method(object_expr, method, arg_exprs);
                }
                _ => {}
            }
        }

        // Check for dict-specific context
        if self.is_dict_expr(object) {
            match method {
                "get" | "keys" | "values" | "items" | "update" => {
                    return self.convert_dict_method(object_expr, method, arg_exprs, hir_args);
                }
                _ => {}
            }
        }

        // Fallback to method name dispatch
        match method {
            // List methods
            "append" | "extend" | "pop" | "insert" | "remove" | "index" | "copy" | "clear"
            | "reverse" | "sort" => {
                self.convert_list_method(object_expr, object, method, arg_exprs, hir_args)
            }

            // DEPYLER-0226: Disambiguate count() for list vs string
            // DEPYLER-0302: Improved heuristic using is_string_base()
            "count" => {
                // Heuristic: Check if object is string-typed using is_string_base()
                // This covers string literals, variables with str type annotations, and string method results
                if self.is_string_base(object) {
                    // String: use str.count() → .matches().count()
                    self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
                } else {
                    // List: use list.count() → .iter().filter().count()
                    self.convert_list_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // DEPYLER-0223: Disambiguate update() for dict vs set
            "update" => {
                // Check if argument is a set or dict literal
                if !hir_args.is_empty() && self.is_set_expr(&hir_args[0]) {
                    // numbers.update({3, 4}) - set update
                    self.convert_set_method(object_expr, method, arg_exprs)
                } else {
                    // data.update({"b": 2}) - dict update (default for variables)
                    self.convert_dict_method(object_expr, method, arg_exprs, hir_args)
                }
            }

            // Dict methods (for variables without type info)
            "get" | "keys" | "values" | "items" | "setdefault" | "popitem" => {
                self.convert_dict_method(object_expr, method, arg_exprs, hir_args)
            }

            // String methods
            // Note: "count" handled separately above with disambiguation logic
            "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "startswith" | "endswith"
            | "split" | "join" | "replace" | "find" | "isdigit" | "isalpha" | "isalnum" | "title" => {
                self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
            }

            // Set methods (for variables without type info)
            // Note: "update" handled separately above with disambiguation logic
            // Note: "remove" is ambiguous (list vs set) - keep in list fallback for now
            "add"
            | "discard"
            | "intersection_update"
            | "difference_update"
            | "symmetric_difference_update"
            | "union"
            | "intersection"
            | "difference"
            | "symmetric_difference"
            | "issubset"
            | "issuperset"
            | "isdisjoint" => self.convert_set_method(object_expr, method, arg_exprs),

            // Regex methods
            "findall" => self.convert_regex_method(object_expr, method, arg_exprs),

            // Default: generic method call
            _ => {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
            }
        }
    }

    fn convert_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // Try classmethod handling first
        if let Some(result) = self.try_convert_classmethod(object, method, args)? {
            return Ok(result);
        }

        // Try module method handling
        if let Some(result) = self.try_convert_module_method(object, method, args)? {
            return Ok(result);
        }

        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Dispatch to instance method handler
        self.convert_instance_method(object, &object_expr, method, &arg_exprs, args)
    }

    fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // DEPYLER-0307 Fix #9: Handle tuple indexing with integer literals
        // Python: tuple[0], tuple[1] → Rust: tuple.0, tuple.1
        // HEURISTIC: Use tuple syntax for variables with tuple-suggesting names
        // This is a temporary solution until proper type tracking is implemented
        // Common tuple iteration variable names: pair, entry, item, elem, tuple, row
        let should_use_tuple_syntax = if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                if let HirExpr::Var(var_name) = base {
                    // Heuristic: Check if variable name suggests tuple iteration
                    // TODO: Replace with proper type tracking via ctx.var_types
                    matches!(
                        var_name.as_str(),
                        "pair" | "entry" | "item" | "elem" | "tuple" | "row"
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

        if should_use_tuple_syntax {
            if let HirExpr::Literal(Literal::Int(idx)) = index {
                let field_idx = syn::Index::from(*idx as usize);
                return Ok(parse_quote! { #base_expr.#field_idx });
            }
        }

        // DEPYLER-0299 Pattern #3 FIX: Check if base is a String type for character access
        let is_string_base = self.is_string_base(base);

        // Discriminate between HashMap and Vec access based on base type or index type
        let is_string_key = self.is_string_index(base, index)?;

        if is_string_key {
            // HashMap/Dict access with string keys
            match index {
                HirExpr::Literal(Literal::String(s)) => {
                    // String literal - use it directly without .to_string()
                    Ok(parse_quote! {
                        #base_expr.get(#s).cloned().unwrap_or_default()
                    })
                }
                _ => {
                    // String variable - needs proper referencing
                    // HashMap.get() expects &K, so we need to borrow the key
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    Ok(parse_quote! {
                        #base_expr.get(&#index_expr).cloned().unwrap_or_default()
                    })
                }
            }
        } else if is_string_base {
            // DEPYLER-0299 Pattern #3: String character access with numeric index
            // Strings cannot use .get(usize), must use .chars().nth() or byte slicing
            let index_expr = index.to_rust_expr(self.ctx)?;

            // For string indexing, we use .chars().nth() which returns Option<char>
            // Then convert char to String
            Ok(parse_quote! {
                {
                    // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                    let base = &#base_expr;
                    let idx: i32 = #index_expr;
                    let actual_idx = if idx < 0 {
                        base.len().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.get(actual_idx..=actual_idx).unwrap_or("").to_string()
                }
            })
        } else {
            // Vec/List access with numeric index
            let index_expr = index.to_rust_expr(self.ctx)?;

            // Check if index is a negative literal
            if let HirExpr::Unary {
                op: UnaryOp::Neg,
                operand,
            } = index
            {
                if let HirExpr::Literal(Literal::Int(n)) = **operand {
                    // Negative index literal: arr[-1] → arr.get(arr.len() - 1)
                    let offset = n as usize;
                    return Ok(parse_quote! {
                        {
                            // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                            let base = &#base_expr;
                            // DEPYLER-0267: Use .cloned() instead of .copied() for non-Copy types (String, Vec, etc.)
                            base.get(base.len().saturating_sub(#offset)).cloned().unwrap_or_default()
                        }
                    });
                }
            }

            // DEPYLER-0306 FIX: Check if index is a simple variable (not a complex expression)
            // Simple variables in for loops like `for i in range(len(arr))` are guaranteed >= 0
            // For these, we can use simpler inline code that works in range contexts
            let is_simple_var = matches!(index, HirExpr::Var(_));

            if is_simple_var {
                // Simple variable index - use inline expression (works in range contexts)
                // This avoids block expressions that break in `for j in 0..matrix[i].len()`
                Ok(parse_quote! {
                    #base_expr.get(#index_expr as usize).cloned().unwrap_or_default()
                })
            } else {
                // Complex expression - use block with full negative index handling
                // DEPYLER-0288: Explicitly type idx as i32 to support negation
                Ok(parse_quote! {
                    {
                        // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                        let base = &#base_expr;
                        let idx: i32 = #index_expr;
                        let actual_idx = if idx < 0 {
                            // Use .abs() instead of negation to avoid "Neg not implemented for usize" error
                            base.len().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        // DEPYLER-0267: Use .cloned() instead of .copied() for non-Copy types (String, Vec, etc.)
                        base.get(actual_idx).cloned().unwrap_or_default()
                    }
                })
            }
        }
    }

    /// Check if the index expression is a string key (for HashMap access)
    /// Returns true if: index is string literal, OR base is Dict/HashMap type
    fn is_string_index(&self, base: &HirExpr, index: &HirExpr) -> Result<bool> {
        // Check 1: Is index a string literal?
        if matches!(index, HirExpr::Literal(Literal::String(_))) {
            return Ok(true);
        }

        // Check 2: Is base expression a Dict/HashMap type?
        // We need to look at the base's inferred type
        if let HirExpr::Var(sym) = base {
            // Try to find the variable's type in the current function context
            // For parameters, we can check the function signature
            // For local variables, this is harder without full type inference
            //
            // Heuristic: If the symbol name contains "dict" or "data" or "map"
            // and index doesn't look numeric, assume HashMap
            let name = sym.as_str();
            if (name.contains("dict") || name.contains("data") || name.contains("map"))
                && !self.is_numeric_index(index)
            {
                return Ok(true);
            }
        }

        // Check 3: Does the index expression look like a string variable?
        if self.is_string_variable(index) {
            return Ok(true);
        }

        // Default: assume numeric index (Vec/List access)
        Ok(false)
    }

    /// Check if expression is likely a string variable (heuristic)
    fn is_string_variable(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(sym) => {
                let name = sym.as_str();
                // Heuristic: variable names like "key", "name", "id", "word", etc.
                name == "key"
                    || name == "name"
                    || name == "id"
                    || name == "word"
                    || name == "text"
                    || name.ends_with("_key")
                    || name.ends_with("_name")
            }
            _ => false,
        }
    }

    /// Check if expression is likely numeric (heuristic)
    fn is_numeric_index(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => true,
            HirExpr::Var(sym) => {
                let name = sym.as_str();
                // Common numeric index names
                name == "i"
                    || name == "j"
                    || name == "k"
                    || name == "idx"
                    || name == "index"
                    || name.starts_with("idx_")
                    || name.ends_with("_idx")
                    || name.ends_with("_index")
            }
            HirExpr::Binary { .. } => true, // Arithmetic expressions are numeric
            HirExpr::Call { .. } => false,  // Could be anything
            _ => false,
        }
    }

    /// DEPYLER-0299 Pattern #3: Check if base expression is a String type (heuristic)
    /// Returns true if base is likely a String/str type (not Vec/List)
    fn is_string_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(sym) => {
                let name = sym.as_str();
                // Common string variable names
                name == "word"
                    || name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name.starts_with("word")
                    || name.starts_with("text")
                    || name.starts_with("str")
                    || name.ends_with("_str")
                    || name.ends_with("_string")
                    || name.ends_with("_word")
                    || name.ends_with("_text")
            }
            HirExpr::MethodCall { method, .. }
                if method.as_str().contains("upper")
                    || method.as_str().contains("lower")
                    || method.as_str().contains("strip")
                    || method.as_str().contains("lstrip")
                    || method.as_str().contains("rstrip")
                    || method.as_str().contains("title") =>
            {
                true
            }
            HirExpr::Call { func, .. } if func.as_str() == "str" => true,
            _ => false,
        }
    }

    fn convert_slice(
        &mut self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // Convert slice parameters
        let start_expr = if let Some(s) = start {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let stop_expr = if let Some(s) = stop {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let step_expr = if let Some(s) = step {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        // Generate slice code based on the parameters
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: base[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        let step = #step;
                        if step == 1 {
                            base.clone()
                        } else if step > 0 {
                            base.iter().step_by(step as usize).cloned().collect::<Vec<_>>()
                        } else if step == -1 {
                            base.iter().rev().cloned().collect::<Vec<_>>()
                        } else {
                            // Negative step with abs value
                            let abs_step = (-step) as usize;
                            base.iter().rev().step_by(abs_step).cloned().collect::<Vec<_>>()
                        }
                    }
                })
            }

            // Start and stop: base[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start = (#start).max(0) as usize;
                    let stop = (#stop).max(0) as usize;
                    if start < base.len() {
                        base[start..stop.min(base.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Start only: base[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start = (#start).max(0) as usize;
                    if start < base.len() {
                        base[start..].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop only: base[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop = (#stop).max(0) as usize;
                    base[..stop.min(base.len())].to_vec()
                }
            }),

            // Full slice: base[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.clone() }),

            // Start, stop, and step: base[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        let start = (#start).max(0) as usize;
                        let stop = (#stop).max(0) as usize;
                        let step = #step;

                        if step == 1 {
                            if start < base.len() {
                                base[start..stop.min(base.len())].to_vec()
                            } else {
                                Vec::new()
                            }
                        } else if step > 0 {
                            base[start..stop.min(base.len())]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            // Negative step - slice in reverse
                            let abs_step = (-step) as usize;
                            if start < base.len() {
                                base[start..stop.min(base.len())]
                                    .iter()
                                    .rev()
                                    .step_by(abs_step)
                                    .cloned()
                                    .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            }
                        }
                    }
                })
            }

            // Start and step: base[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start = (#start).max(0) as usize;
                    let step = #step;

                    if start < base.len() {
                        if step == 1 {
                            base[start..].to_vec()
                        } else if step > 0 {
                            base[start..]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else if step == -1 {
                            base[start..]
                                .iter()
                                .rev()
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            let abs_step = (-step) as usize;
                            base[start..]
                                .iter()
                                .rev()
                                .step_by(abs_step)
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop and step: base[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop = (#stop).max(0) as usize;
                    let step = #step;

                    if step == 1 {
                        base[..stop.min(base.len())].to_vec()
                    } else if step > 0 {
                        base[..stop.min(base.len())]
                            .iter()
                            .step_by(step as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    } else if step == -1 {
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .cloned()
                            .collect::<Vec<_>>()
                    } else {
                        let abs_step = (-step) as usize;
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .step_by(abs_step)
                            .cloned()
                            .collect::<Vec<_>>()
                    }
                }
            }),
        }
    }

    fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Always use vec! for now to ensure mutability works
        // In the future, we should analyze if the list is mutated before deciding
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    fn convert_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        self.ctx.needs_hashmap = true;

        // Check if return type is Dict with String keys
        let needs_owned_keys = matches!(
            &self.ctx.current_return_type,
            Some(Type::Dict(key_type, _)) if **key_type == Type::String
        );

        let mut insert_stmts = Vec::new();
        for (key, value) in items {
            let mut key_expr = key.to_rust_expr(self.ctx)?;
            let val_expr = value.to_rust_expr(self.ctx)?;

            // If function returns HashMap<String, V>, convert &str keys to String
            if needs_owned_keys && matches!(key, HirExpr::Literal(Literal::String(_))) {
                key_expr = parse_quote! { #key_expr.to_string() };
            }

            insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
        }

        // DEPYLER-0279: Only add `mut` if there are items to insert
        // Empty dicts don't need mutable bindings
        if items.is_empty() {
            Ok(parse_quote! {
                {
                    let map = HashMap::new();
                    map
                }
            })
        } else {
            Ok(parse_quote! {
                {
                    let mut map = HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            })
        }
    }

    fn convert_tuple(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    fn convert_set(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        let mut insert_stmts = Vec::new();
        for elem in elts {
            let elem_expr = elem.to_rust_expr(self.ctx)?;
            insert_stmts.push(quote! { set.insert(#elem_expr); });
        }
        Ok(parse_quote! {
            {
                let mut set = HashSet::new();
                #(#insert_stmts)*
                set
            }
        })
    }

    fn convert_frozenset(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        self.ctx.needs_arc = true;
        let mut insert_stmts = Vec::new();
        for elem in elts {
            let elem_expr = elem.to_rust_expr(self.ctx)?;
            insert_stmts.push(quote! { set.insert(#elem_expr); });
        }
        Ok(parse_quote! {
            {
                let mut set = HashSet::new();
                #(#insert_stmts)*
                std::sync::Arc::new(set)
            }
        })
    }

    fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.ctx.is_classmethod {
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { Self::#attr_ident });
            }
        }

        // Check if this is a module attribute access
        if let HirExpr::Var(module_name) = value {
            let rust_name_opt = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| mapping.item_map.get(attr).cloned());

            if let Some(rust_name) = rust_name_opt {
                // Map to the Rust equivalent
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                if path_parts.len() > 1 {
                    // It's a path like "env::current_dir"
                    let mut path = quote! { std };
                    for part in path_parts {
                        let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                        path = quote! { #path::#part_ident };
                    }
                    return Ok(parse_quote! { #path });
                } else {
                    // Simple identifier
                    let ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #ident });
                }
            }
        }

        // Default behavior for non-module attributes
        let value_expr = value.to_rust_expr(self.ctx)?;
        let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
        Ok(parse_quote! { #value_expr.#attr_ident })
    }

    fn convert_borrow(&mut self, expr: &HirExpr, mutable: bool) -> Result<syn::Expr> {
        let expr_tokens = expr.to_rust_expr(self.ctx)?;
        if mutable {
            Ok(parse_quote! { &mut #expr_tokens })
        } else {
            Ok(parse_quote! { &#expr_tokens })
        }
    }

    fn convert_list_comp(
        &mut self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let iter_expr = iter.to_rust_expr(self.ctx)?;
        let element_expr = element.to_rust_expr(self.ctx)?;

        // DEPYLER-0299 FIX: Proper iterator handling for comprehensions
        // Strategy:
        // - Ranges are already iterators, use them directly
        // - Collections need .iter().cloned() to convert &T to T
        // - Place .cloned() AFTER .filter() for correct types
        // - DON'T use pattern matching in filter (|x| not |&x|) to avoid operator issues
        //
        // Key insight: .filter(|x| ...) ALWAYS receives &Item.
        // With .iter().filter() we get &&T, so we need **x to dereference.
        // This is verbose but works universally for all operators.

        let is_range = self.is_range_expr(&iter_expr);

        if let Some(cond) = condition {
            // With condition: iter().filter().cloned().map().collect()
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if is_range {
                // Ranges are already iterators, don't call .iter()
                // Range items are owned (i32, etc.), filter receives &i32
                Ok(parse_quote! {
                    (#iter_expr)
                        .filter(|#target_ident| #cond_expr)
                        .map(|#target_ident| #element_expr)
                        .collect::<Vec<_>>()
                })
            } else {
                // Collections need .iter().filter().cloned().map()
                // NOTE: This generates filter closures that need **x dereference
                // TODO: Add context tracking to auto-insert * in condition expressions
                Ok(parse_quote! {
                    #iter_expr
                        .iter()
                        .filter(|#target_ident| #cond_expr)
                        .cloned()
                        .map(|#target_ident| #element_expr)
                        .collect::<Vec<_>>()
                })
            }
        } else {
            // Without condition: iter().cloned().map().collect()
            if is_range {
                // Ranges are already iterators, don't call .iter()
                Ok(parse_quote! {
                    (#iter_expr)
                        .map(|#target_ident| #element_expr)
                        .collect::<Vec<_>>()
                })
            } else {
                // Collections need .iter().cloned().map()
                // .cloned() converts &T to T for map
                Ok(parse_quote! {
                    #iter_expr
                        .iter()
                        .cloned()
                        .map(|#target_ident| #element_expr)
                        .collect::<Vec<_>>()
                })
            }
        }
    }

    fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_) => {
                // Check type information in context for variables
                self.is_set_var(expr)
            }
            _ => false,
        }
    }

    /// Check if a variable has a set type based on type information in context
    fn is_set_var(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types in context to see if this variable is a set
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Set(_))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #7: Check if expression is a dict/HashMap
    /// Used for dict merge operator (|) and other dict-specific operations
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    fn is_dict_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Dict(_) => true,
            HirExpr::Call { func, .. } if func == "dict" => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a dict/HashMap
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, _))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_list_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::List(_) => true,
            HirExpr::Call { func, .. } if func == "list" => true,
            HirExpr::Var(_name) => {
                // For rust_gen, we're more conservative since we don't have type info
                // Only treat explicit list literals and calls as lists
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #6: Check if expression is an owned collection
    /// Used to determine if zip() should use .into_iter() (owned) vs .iter() (borrowed)
    ///
    /// Returns true if:
    /// - Expression is a Var with type List (Vec<T>) - function parameters are owned
    /// - Expression is a list literal - always owned
    /// - Expression is a list() call - creates owned Vec
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    fn is_owned_collection(&self, expr: &HirExpr) -> bool {
        match expr {
            // List literals are always owned
            HirExpr::List(_) => true,
            // list() calls create owned Vec
            HirExpr::Call { func, .. } if func == "list" => true,
            // Check if variable has List type (function parameters of type Vec<T>)
            HirExpr::Var(name) => {
                if let Some(ty) = self.ctx.var_types.get(name) {
                    matches!(ty, Type::List(_))
                } else {
                    // No type info - conservative default is borrowed
                    false
                }
            }
            _ => false,
        }
    }

    /// Check if an expression is a user-defined class instance
    fn is_class_instance(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a user-defined class
                if let Some(Type::Custom(class_name)) = self.ctx.var_types.get(name) {
                    // Check if this is a user-defined class (not a builtin)
                    self.ctx.class_names.contains(class_name)
                } else {
                    false
                }
            }
            HirExpr::Call { func, .. } => {
                // Direct constructor call like Calculator(10)
                self.ctx.class_names.contains(func)
            }
            _ => false,
        }
    }

    fn is_bool_expr(&self, expr: &HirExpr) -> Option<bool> {
        match expr {
            // Comparison operations always return bool
            HirExpr::Binary {
                op:
                    BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn,
                ..
            } => Some(true),
            // Method calls that return bool
            HirExpr::MethodCall { method, .. }
                if matches!(
                    method.as_str(),
                    "startswith"
                        | "endswith"
                        | "isdigit"
                        | "isalpha"
                        | "isspace"
                        | "isupper"
                        | "islower"
                        | "issubset"
                        | "issuperset"
                        | "isdisjoint"
                ) =>
            {
                Some(true)
            }
            // Boolean literals
            HirExpr::Literal(Literal::Bool(_)) => Some(true),
            // Logical operations
            HirExpr::Unary {
                op: UnaryOp::Not, ..
            } => Some(true),
            _ => None,
        }
    }

    fn convert_set_operation(
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

    fn convert_set_comp(
        &mut self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        // DEPYLER-0299 Pattern #5 FIX: Proper iterator handling for set comprehensions
        // Uses the same strategy as list comprehensions:
        // - Ranges are already iterators, use them directly
        // - Collections need .iter() to get an iterator
        // - Use .cloned() to convert &T to T
        // - Place .cloned() AFTER .filter() so filter sees &T
        //
        // Key insight: .filter(|x| ...) receives &Item where Item is the iterator's item type.
        // So .iter().filter() means x is &&T, but .iter().filter().cloned() means filter gets &T
        // and then .cloned() converts to T for the next stage.

        self.ctx.needs_hashset = true;
        let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let iter_expr = iter.to_rust_expr(self.ctx)?;
        let element_expr = element.to_rust_expr(self.ctx)?;

        let is_range = self.is_range_expr(&iter_expr);

        if let Some(cond) = condition {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if is_range {
                // Ranges are already iterators, don't call .iter()
                // Range items are owned (i32, etc.), so no dereference needed
                Ok(parse_quote! {
                    (#iter_expr)
                        .filter(|#target_ident| #cond_expr)
                        .map(|#target_ident| #element_expr)
                        .collect::<HashSet<_>>()
                })
            } else {
                // Collections need .iter().filter().cloned().map()
                // Use |&target| pattern to automatically dereference in filter closure
                Ok(parse_quote! {
                    #iter_expr
                        .iter()
                        .filter(|&#target_ident| #cond_expr)
                        .cloned()
                        .map(|#target_ident| #element_expr)
                        .collect::<HashSet<_>>()
                })
            }
        } else if is_range {
            Ok(parse_quote! {
                (#iter_expr)
                    .map(|#target_ident| #element_expr)
                    .collect::<HashSet<_>>()
            })
        } else {
            Ok(parse_quote! {
                #iter_expr
                    .iter()
                    .cloned()
                    .map(|#target_ident| #element_expr)
                    .collect::<HashSet<_>>()
            })
        }
    }

    fn convert_dict_comp(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        // DEPYLER-0299 Pattern #5 FIX: Proper iterator handling for dict comprehensions
        // Uses the same strategy as list comprehensions:
        // - Ranges are already iterators, use them directly
        // - Collections need .iter() to get an iterator
        // - Use .cloned() to convert &T to T
        // - Place .cloned() AFTER .filter() so filter sees &T
        //
        // Key insight: .filter(|x| ...) receives &Item where Item is the iterator's item type.
        // So .iter().filter() means x is &&T, but .iter().filter().cloned() means filter gets &T
        // and then .cloned() converts to T for the next stage.

        self.ctx.needs_hashmap = true;
        let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let iter_expr = iter.to_rust_expr(self.ctx)?;
        let key_expr = key.to_rust_expr(self.ctx)?;
        let value_expr = value.to_rust_expr(self.ctx)?;

        let is_range = self.is_range_expr(&iter_expr);

        if let Some(cond) = condition {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            if is_range {
                // Ranges are already iterators, don't call .iter()
                // Range items are owned (i32, etc.), so no dereference needed
                Ok(parse_quote! {
                    (#iter_expr)
                        .filter(|#target_ident| #cond_expr)
                        .map(|#target_ident| (#key_expr, #value_expr))
                        .collect::<HashMap<_, _>>()
                })
            } else {
                // Collections need .iter().filter().cloned().map()
                // Use |&target| pattern to automatically dereference in filter closure
                Ok(parse_quote! {
                    #iter_expr
                        .iter()
                        .filter(|&#target_ident| #cond_expr)
                        .cloned()
                        .map(|#target_ident| (#key_expr, #value_expr))
                        .collect::<HashMap<_, _>>()
                })
            }
        } else if is_range {
            Ok(parse_quote! {
                (#iter_expr)
                    .map(|#target_ident| (#key_expr, #value_expr))
                    .collect::<HashMap<_, _>>()
            })
        } else {
            Ok(parse_quote! {
                #iter_expr
                    .iter()
                    .cloned()
                    .map(|#target_ident| (#key_expr, #value_expr))
                    .collect::<HashMap<_, _>>()
            })
        }
    }

    fn convert_lambda(&mut self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = syn::Ident::new(p, proc_macro2::Span::call_site());
                parse_quote! { #ident }
            })
            .collect();

        // Convert body expression
        let body_expr = body.to_rust_expr(self.ctx)?;

        // Generate closure
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { |#(#param_pats),*| #body_expr })
        }
    }

    /// Check if an expression is a len() call
    fn is_len_call(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Call { func, args } if func == "len" && args.len() == 1)
    }

    fn convert_await(&mut self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = value.to_rust_expr(self.ctx)?;
        Ok(parse_quote! { #value_expr.await })
    }

    fn convert_yield(&mut self, value: &Option<Box<HirExpr>>) -> Result<syn::Expr> {
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

    fn convert_fstring(&mut self, parts: &[FStringPart]) -> Result<syn::Expr> {
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
                    template.push_str("{}");
                    let arg_expr = expr.to_rust_expr(self.ctx)?;
                    args.push(arg_expr);
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

    fn convert_ifexpr(
        &mut self,
        test: &HirExpr,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        let test_expr = test.to_rust_expr(self.ctx)?;
        let body_expr = body.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        Ok(parse_quote! {
            if #test_expr { #body_expr } else { #orelse_expr }
        })
    }

    fn convert_sort_by_key(
        &mut self,
        iterable: &HirExpr,
        key_params: &[String],
        key_body: &HirExpr,
        reverse: bool,
    ) -> Result<syn::Expr> {
        let iter_expr = iterable.to_rust_expr(self.ctx)?;
        let body_expr = key_body.to_rust_expr(self.ctx)?;

        // Create the closure parameter pattern
        let param_pat: syn::Pat = if key_params.len() == 1 {
            let param = syn::Ident::new(&key_params[0], proc_macro2::Span::call_site());
            parse_quote! { #param }
        } else {
            bail!("sorted() key lambda must have exactly one parameter");
        };

        // Generate: { let mut result = iterable.clone(); result.sort_by_key(|param| body); [result.reverse();] result }
        if reverse {
            Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort_by_key(|#param_pat| #body_expr);
                    __sorted_result.reverse();
                    __sorted_result
                }
            })
        } else {
            Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort_by_key(|#param_pat| #body_expr);
                    __sorted_result
                }
            })
        }
    }

    fn convert_generator_expression(
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
            let element_expr = element.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0307 Fix #10: Use .iter().copied() for borrowed collections
            // When the iterator is a variable (likely a borrowed parameter like &Vec<i32>),
            // use .iter().copied() to get owned values instead of references
            // This prevents type mismatches like `&i32` vs `i32` in generator expressions
            let mut chain: syn::Expr = if matches!(&*gen.iter, HirExpr::Var(_)) {
                // Variable iteration - likely borrowed, use .iter().copied()
                parse_quote! { #iter_expr.iter().copied() }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // Add filters for each condition
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }

            // Add the map transformation
            chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };

            return Ok(chain);
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: (x + y for x in range(3) for y in range(3))
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y))

        self.convert_nested_generators(element, generators)
    }

    fn convert_nested_generators(
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
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // Add filters for first generator's conditions
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
        }

        // Use flat_map for the first generator
        chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };

        Ok(chain)
    }

    fn build_nested_chain(
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

        // Build the chain for this level
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // Add filters for this generator's conditions
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
        }

        // Use flat_map for intermediate generators, map for the last
        if depth < generators.len() - 1 {
            // Intermediate generator: use flat_map
            chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_expr) };
        } else {
            // Last generator: use map
            chain = parse_quote! { #chain.map(move |#target_pat| #inner_expr) };
        }

        Ok(chain)
    }

    fn parse_target_pattern(&self, target: &str) -> Result<syn::Pat> {
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
}

impl ToRustExpr for HirExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
        let mut converter = ExpressionConverter::new(ctx);

        match self {
            HirExpr::Literal(lit) => {
                let expr = literal_to_rust_expr(lit, &ctx.string_optimizer, &ctx.needs_cow, ctx);
                if let Literal::String(s) = lit {
                    let context = StringContext::Literal(s.clone());
                    if matches!(
                        ctx.string_optimizer.get_optimal_type(&context),
                        crate::string_optimization::OptimalStringType::CowStr
                    ) {
                        ctx.needs_cow = true;
                    }
                }
                Ok(expr)
            }
            HirExpr::Var(name) => converter.convert_variable(name),
            HirExpr::Binary { op, left, right } => converter.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => converter.convert_unary(op, operand),
            HirExpr::Call { func, args } => converter.convert_call(func, args),
            HirExpr::MethodCall {
                object,
                method,
                args,
            } => converter.convert_method_call(object, method, args),
            HirExpr::Index { base, index } => converter.convert_index(base, index),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => converter.convert_slice(base, start, stop, step),
            HirExpr::List(elts) => converter.convert_list(elts),
            HirExpr::Dict(items) => converter.convert_dict(items),
            HirExpr::Tuple(elts) => converter.convert_tuple(elts),
            HirExpr::Set(elts) => converter.convert_set(elts),
            HirExpr::FrozenSet(elts) => converter.convert_frozenset(elts),
            HirExpr::Attribute { value, attr } => converter.convert_attribute(value, attr),
            HirExpr::Borrow { expr, mutable } => converter.convert_borrow(expr, *mutable),
            HirExpr::ListComp {
                element,
                target,
                iter,
                condition,
            } => converter.convert_list_comp(element, target, iter, condition),
            HirExpr::Lambda { params, body } => converter.convert_lambda(params, body),
            HirExpr::SetComp {
                element,
                target,
                iter,
                condition,
            } => converter.convert_set_comp(element, target, iter, condition),
            HirExpr::DictComp {
                key,
                value,
                target,
                iter,
                condition,
            } => converter.convert_dict_comp(key, value, target, iter, condition),
            HirExpr::Await { value } => converter.convert_await(value),
            HirExpr::Yield { value } => converter.convert_yield(value),
            HirExpr::FString { parts } => converter.convert_fstring(parts),
            HirExpr::IfExpr { test, body, orelse } => converter.convert_ifexpr(test, body, orelse),
            HirExpr::SortByKey {
                iterable,
                key_params,
                key_body,
                reverse,
            } => converter.convert_sort_by_key(iterable, key_params, key_body, *reverse),
            HirExpr::GeneratorExp {
                element,
                generators,
            } => converter.convert_generator_expression(element, generators),
        }
    }
}

fn literal_to_rust_expr(
    lit: &Literal,
    string_optimizer: &StringOptimizer,
    _needs_cow: &bool,
    ctx: &CodeGenContext,
) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            // Ensure float literals always have a decimal point
            // f64::to_string() outputs "0" for 0.0, which parses as integer
            let s = f.to_string();
            let float_str = if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            };
            let lit = syn::LitFloat::new(&float_str, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            // Check if this string should be interned
            if let Some(interned_name) = string_optimizer.get_interned_name(s) {
                let ident = syn::Ident::new(&interned_name, proc_macro2::Span::call_site());
                parse_quote! { #ident }
            } else {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());

                // Use string optimizer to determine if we need .to_string()
                let context = StringContext::Literal(s.clone());
                match string_optimizer.get_optimal_type(&context) {
                    crate::string_optimization::OptimalStringType::StaticStr => {
                        // For read-only strings, just use the literal
                        parse_quote! { #lit }
                    }
                    crate::string_optimization::OptimalStringType::BorrowedStr { .. } => {
                        // Use &'static str for literals that can be borrowed
                        parse_quote! { #lit }
                    }
                    crate::string_optimization::OptimalStringType::CowStr => {
                        // Check if we're in a context where String is required
                        if let Some(Type::String) = &ctx.current_return_type {
                            // Function returns String, so convert to owned
                            parse_quote! { #lit.to_string() }
                        } else {
                            // Use Cow for flexible ownership
                            parse_quote! { std::borrow::Cow::Borrowed(#lit) }
                        }
                    }
                    crate::string_optimization::OptimalStringType::OwnedString => {
                        // Only use .to_string() when absolutely necessary
                        parse_quote! { #lit.to_string() }
                    }
                }
            }
        }
        Literal::Bytes(b) => {
            // Generate Rust byte array: &[u8] slice from byte values
            // Python: b"hello" → Rust: &[104_u8, 101, 108, 108, 111]
            let byte_str = syn::LitByteStr::new(b, proc_macro2::Span::call_site());
            parse_quote! { #byte_str }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::None => {
            // Python None maps to Rust unit type ()
            // This is correct for both:
            // 1. Functions returning None (-> None becomes -> () implicitly)
            // 2. Optional types (Option<T> uses None, but that's handled separately)
            parse_quote! { () }
        }
    }
}

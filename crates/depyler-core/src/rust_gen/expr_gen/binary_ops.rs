//! Binary operator conversion for ExpressionConverter
//!
//! Contains convert_binary, convert_pow_op, convert_mul_op,
//! convert_add_op, convert_containment_op, wrap_in_parens.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::context::CodeGenContext;
use crate::trace_decision;
use crate::rust_gen::expr_analysis;
use crate::rust_gen::precedence;
use crate::rust_gen::return_type_expects_float;
use crate::rust_gen::type_gen::convert_binop;
use crate::string_optimization::{StringContext, StringOptimizer};
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_binary(
        &mut self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
    ) -> Result<syn::Expr> {
        // CITL: Trace binary operation type mapping decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "binop_conversion",
            chosen = &format!("{:?}", op),
            alternatives = ["arithmetic", "comparison", "logical", "bitwise"],
            confidence = 0.95
        );

        // DEPYLER-99MODE-S9: List repetition [x] * n → vec![x; n as usize]
        // Must be caught before general expression conversion to avoid DepylerValue wrapping
        // in NASA mode. Python `[0] * n` or `[big] * (amount + 1)` becomes `vec![elem; count]`.
        if matches!(op, BinOp::Mul) {
            if let HirExpr::List(elts) = left {
                if elts.len() == 1 {
                    let elem_expr = elts[0].to_rust_expr(self.ctx)?;
                    let count_expr = right.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { vec![#elem_expr; (#count_expr) as usize] });
                }
            }
            if let HirExpr::List(elts) = right {
                if elts.len() == 1 {
                    let elem_expr = elts[0].to_rust_expr(self.ctx)?;
                    let count_expr = left.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { vec![#elem_expr; (#count_expr) as usize] });
                }
            }
        }

        // DEPYLER-0496: Check if operands return Result types (need ? operator)
        let left_returns_result = self.expr_returns_result(left);
        let right_returns_result = self.expr_returns_result(right);

        // DEPYLER-0498: Check if operands are Option types (need unwrap for comparisons)
        let left_is_option = self.expr_is_option(left);
        let right_is_option = self.expr_is_option(right);

        let mut left_expr = left.to_rust_expr(self.ctx)?;
        let mut right_expr = right.to_rust_expr(self.ctx)?;

        // DEPYLER-0496: Add ? operator for Result-returning expressions in binary operations
        // Only add ? if we're in a Result-returning context (current function can fail)
        // DEPYLER-99MODE-S9: Wrap cast expressions in parens before adding ? to avoid
        // "casts cannot be followed by ?" parse error (e.g., `x as i32?` → `(x as i32)?`)
        if self.ctx.current_function_can_fail {
            if left_returns_result {
                if matches!(left_expr, syn::Expr::Cast(_)) {
                    left_expr = parse_quote! { (#left_expr)? };
                } else {
                    left_expr = parse_quote! { #left_expr? };
                }
            }
            if right_returns_result {
                if matches!(right_expr, syn::Expr::Cast(_)) {
                    right_expr = parse_quote! { (#right_expr)? };
                } else {
                    right_expr = parse_quote! { #right_expr? };
                }
            }
        }

        // DEPYLER-0498: Unwrap Option types in comparison operations
        // Use unwrap_or with appropriate defaults for comparison
        let is_comparison = matches!(
            op,
            BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq | BinOp::Eq | BinOp::NotEq
        );

        // DEPYLER-1109: Universal PyOps Dispatch (NASA Mode)
        // Delegate arithmetic/indexing to PyOps traits to handle type coercion (i32+f64, etc.)
        // Note: Wrap left_expr in parens to handle cast expressions (a as f64).method() is invalid
        if self.ctx.type_mapper.nasa_mode && !is_comparison {
            // DEPYLER-1163: Check if return type expects int for division
            // py_div always returns f64, so we need to cast when int is expected
            let return_expects_int = self
                .ctx
                .current_return_type
                .as_ref()
                .map(crate::rust_gen::func_gen::return_type_expects_int)
                .unwrap_or(false);

            // DEPYLER-E0282-FIX: Add i32 suffix to integer literals to resolve
            // type inference ambiguity with PyOps traits that have multiple impls
            let left_pyops = if let HirExpr::Literal(Literal::Int(n)) = left {
                // Only add suffix for small integers that fit in i32
                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    let lit_str = format!("{}i32", n);
                    let lit = syn::LitInt::new(&lit_str, proc_macro2::Span::call_site());
                    parse_quote! { #lit }
                } else {
                    left_expr.clone()
                }
            } else {
                left_expr.clone()
            };
            let right_pyops = if let HirExpr::Literal(Literal::Int(n)) = right {
                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    let lit_str = format!("{}i32", n);
                    let lit = syn::LitInt::new(&lit_str, proc_macro2::Span::call_site());
                    parse_quote! { #lit }
                } else {
                    right_expr.clone()
                }
            } else {
                right_expr.clone()
            };

            // DEPYLER-E0282-FIX: When left operand is also a binary arithmetic expression,
            // add explicit type cast to help Rust infer intermediate types in chains.
            // Pattern: ((a).py_add(b)).py_add(c) fails type inference
            // Fix: ((a).py_add(b) as i32).py_add(c) provides type anchor for intermediate result
            //
            // DEPYLER-STRING-CONCAT-FIX: Skip cast for string concatenation
            // String + String = String, casting to i32 is wrong
            let left_is_string = self.expr_is_string_type(left);
            let right_is_string = self.expr_is_string_type(right);
            // DEPYLER-99MODE-S9: Detect char iteration variables - char doesn't implement PyAdd
            let left_is_char_iter = if let HirExpr::Var(name) = left {
                self.ctx.char_iter_vars.contains(name.as_str())
            } else {
                false
            };
            let right_is_char_iter = if let HirExpr::Var(name) = right {
                self.ctx.char_iter_vars.contains(name.as_str())
            } else {
                false
            };
            let is_string_concat =
                left_is_string || right_is_string || left_is_char_iter || right_is_char_iter;

            let left_is_chain = if let HirExpr::Binary { op: inner_op, .. } = left {
                matches!(
                    inner_op,
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod
                )
            } else {
                false
            };
            // Add type cast when we detect a chain to help Rust infer intermediate types
            // DEPYLER-STRING-CONCAT-FIX: Skip cast for string operations
            let left_typed: syn::Expr = if left_is_chain && !is_string_concat {
                let return_expects_float = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .map(crate::rust_gen::func_gen::return_type_expects_float)
                    .unwrap_or(false);
                if return_expects_float {
                    parse_quote! { (#left_pyops as f64) }
                } else {
                    parse_quote! { (#left_pyops as i32) }
                }
            } else {
                left_pyops
            };

            match op {
                // DEPYLER-99MODE-S9: Skip py_add for string/char concat - fall through to regular path
                BinOp::Add if !is_string_concat => {
                    return Ok(parse_quote! { (#left_typed).py_add(#right_pyops) });
                }
                BinOp::Sub => return Ok(parse_quote! { (#left_typed).py_sub(#right_pyops) }),
                BinOp::Mul => return Ok(parse_quote! { (#left_typed).py_mul(#right_pyops) }),
                BinOp::Div => {
                    // DEPYLER-1163: Cast py_div result to i32 when return type expects int
                    if return_expects_int {
                        return Ok(parse_quote! { ((#left_typed).py_div(#right_pyops) as i32) });
                    } else {
                        return Ok(parse_quote! { (#left_typed).py_div(#right_pyops) });
                    }
                }
                BinOp::Mod => return Ok(parse_quote! { (#left_typed).py_mod(#right_pyops) }),
                _ => {}
            }
        }

        if is_comparison {
            // DEPYLER-99MODE-S9: Convert `x == []` to `x.is_empty()` and `x != []` to `!x.is_empty()`
            // This avoids E0283 type annotation errors when comparing with empty vec![]
            if matches!(op, BinOp::Eq | BinOp::NotEq) {
                let left_is_empty_list =
                    matches!(left, HirExpr::List(elts) if elts.is_empty());
                let right_is_empty_list =
                    matches!(right, HirExpr::List(elts) if elts.is_empty());

                if right_is_empty_list {
                    if matches!(op, BinOp::Eq) {
                        return Ok(parse_quote! { #left_expr.is_empty() });
                    } else {
                        return Ok(parse_quote! { !#left_expr.is_empty() });
                    }
                }
                if left_is_empty_list {
                    if matches!(op, BinOp::Eq) {
                        return Ok(parse_quote! { #right_expr.is_empty() });
                    } else {
                        return Ok(parse_quote! { !#right_expr.is_empty() });
                    }
                }
            }

            if left_is_option && !right_is_option {
                // Left is Option, right is plain - unwrap left
                left_expr = parse_quote! { #left_expr.unwrap_or_default() };
            }
            if right_is_option && !left_is_option {
                // Right is Option, left is plain - unwrap right for comparison
                // For less-than: unwrap_or(i32::MAX) so None is treated as "very large"
                // For greater-than: unwrap_or(i32::MIN) so None is treated as "very small"
                // For equality: unwrap_or_default()
                match op {
                    BinOp::Lt | BinOp::LtEq => {
                        right_expr = parse_quote! { #right_expr.unwrap_or(i32::MAX) };
                    }
                    BinOp::Gt | BinOp::GtEq => {
                        right_expr = parse_quote! { #right_expr.unwrap_or(i32::MIN) };
                    }
                    _ => {
                        right_expr = parse_quote! { #right_expr.unwrap_or_default() };
                    }
                }
            }

            // DEPYLER-1074: Auto-dereference variables that are references (e.g. from iterators or ref params)
            // This fixes E0308 errors where we compare &T with T (e.g. &i32 == 0)
            let left_is_ref = if let HirExpr::Var(name) = left {
                self.ctx.ref_params.contains(name)
            } else {
                false
            };

            let right_is_ref = if let HirExpr::Var(name) = right {
                self.ctx.ref_params.contains(name)
            } else {
                false
            };

            if left_is_ref && !right_is_ref {
                left_expr = parse_quote! { (*#left_expr) };
            } else if right_is_ref && !left_is_ref {
                right_expr = parse_quote! { (*#right_expr) };
            }

            // DEPYLER-0550: Handle serde_json::Value comparisons
            // When comparing Option<String> (from dict.get()) with serde_json::Value,
            // convert the Value to Option<String> for compatibility
            // Pattern: row.get(col).cloned() == val where val comes from JSON .items()
            let left_is_dict_get =
                matches!(left, HirExpr::MethodCall { method, .. } if method == "get");
            let right_is_json_value = self.is_serde_json_value_expr(right);

            if left_is_dict_get && right_is_json_value {
                // Convert serde_json::Value to Option<String> for comparison
                right_expr = parse_quote! { #right_expr.as_str().map(|s| s.to_string()) };
            }

            // Also handle the reverse case
            let right_is_dict_get =
                matches!(right, HirExpr::MethodCall { method, .. } if method == "get");
            let left_is_json_value = self.is_serde_json_value_expr(left);

            if right_is_dict_get && left_is_json_value {
                left_expr = parse_quote! { #left_expr.as_str().map(|s| s.to_string()) };
            }

            // DEPYLER-0575: Coerce integer literal to float when comparing with float expression
            // DEPYLER-0720: Extended to ALL integer literals, not just 0
            // DEPYLER-0828: Extended to ALL integer expressions (variables, not just literals)
            // DEPYLER-0920: Use f32 literals for trueno/numpy f32 results
            // Example: `self.balance > 0` -> `self.balance > 0.0` when balance is f64
            // Example: `std > 0` -> `std > 0f32` when std is trueno f32 result
            // Example: `x < y` where x:f64, y:i32 -> `x < (y as f64)`
            let left_is_float = self.expr_returns_float(left);
            let right_is_float = self.expr_returns_float(right);
            let left_is_f32 = self.expr_returns_f32(left);
            let right_is_f32 = self.expr_returns_f32(right);

            if left_is_float && !right_is_float {
                if let HirExpr::Literal(Literal::Int(n)) = right {
                    // Integer literal: convert at compile time
                    // DEPYLER-0920: Use f32 for trueno results
                    if left_is_f32 {
                        let float_val = *n as f32;
                        right_expr = parse_quote! { #float_val };
                    } else {
                        let float_val = *n as f64;
                        right_expr = parse_quote! { #float_val };
                    }
                } else {
                    // DEPYLER-0828: Integer variable/expression: cast at runtime
                    if left_is_f32 {
                        right_expr = parse_quote! { (#right_expr as f32) };
                    } else {
                        right_expr = parse_quote! { (#right_expr as f64) };
                    }
                }
            } else if right_is_float && !left_is_float {
                if let HirExpr::Literal(Literal::Int(n)) = left {
                    // Integer literal: convert at compile time
                    // DEPYLER-0920: Use f32 for trueno results
                    if right_is_f32 {
                        let float_val = *n as f32;
                        left_expr = parse_quote! { #float_val };
                    } else {
                        let float_val = *n as f64;
                        left_expr = parse_quote! { #float_val };
                    }
                } else {
                    // DEPYLER-0828: Integer variable/expression: cast at runtime
                    if right_is_f32 {
                        left_expr = parse_quote! { (#left_expr as f32) };
                    } else {
                        left_expr = parse_quote! { (#left_expr as f64) };
                    }
                }
            }

            // DEPYLER-STRING-COMPARE: Handle string comparison type mismatches
            // String >= &str doesn't work (PartialOrd not implemented)
            // String == &String doesn't work directly
            // Convert String operands to &str for comparison
            let is_ordering_compare =
                matches!(op, BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq);

            // Check if left is string index (produces String) and needs .as_str()
            let left_is_string_index =
                matches!(left, HirExpr::Index { base, .. } if self.is_string_base(base));
            // Check if right is string index (produces String) and needs .as_str()
            let right_is_string_index =
                matches!(right, HirExpr::Index { base, .. } if self.is_string_base(base));

            // Check if left is a String-typed variable
            // First check var_types, then fall back to heuristics
            // IMPORTANT: If var_types says it's NOT a string (e.g., Int), don't use heuristic
            let left_is_string_var = if let HirExpr::Var(name) = left {
                // Check explicit type info first
                if let Some(ty) = self.ctx.var_types.get(name) {
                    // If we have explicit type info, use it
                    matches!(ty, Type::String)
                } else {
                    // No type info - use heuristic for char/string variable names
                    // But be conservative: only match these names if right side is a string literal
                    false // Don't use name heuristic alone - too error-prone
                }
            } else {
                false
            };

            // Check if right side is a single-character string literal (like "a", "z")
            // This indicates we're comparing a character variable against a char literal
            let right_is_char_literal = matches!(
                right,
                HirExpr::Literal(Literal::String(s)) if s.len() == 1
            );

            // DEPYLER-99MODE-S9: Check if left is a char iteration variable
            // `for ch in text:` → ch is `char`, not `String`
            // char doesn't have .as_str(), compare with char literals instead
            let left_is_char_iter_var = if let HirExpr::Var(name) = left {
                self.ctx.char_iter_vars.contains(name.as_str())
            } else {
                false
            };

            // If comparing a char iter var with a single-char string literal,
            // convert the string literal to a char literal (e.g., "a" → 'a')
            if is_ordering_compare && left_is_char_iter_var && right_is_char_literal {
                if let HirExpr::Literal(Literal::String(s)) = right {
                    if let Some(ch) = s.chars().next() {
                        let ch_lit = syn::LitChar::new(ch, proc_macro2::Span::call_site());
                        right_expr = parse_quote! { #ch_lit };
                    }
                }
            }

            // If comparing a variable with a single-char string literal in ordering comparison,
            // the variable is likely a String that needs .as_str() conversion
            // But NOT for char iteration variables (they're already char type)
            let left_needs_as_str = is_ordering_compare
                && matches!(left, HirExpr::Var(_))
                && right_is_char_literal
                && !left_is_char_iter_var;

            // DEPYLER-99MODE-S9: Check if right is a String-typed variable (symmetric with left)
            let right_is_string_var = if let HirExpr::Var(name) = right {
                if let Some(ty) = self.ctx.var_types.get(name) {
                    matches!(ty, Type::String)
                } else {
                    false
                }
            } else {
                false
            };

            // For ordering comparisons with string expressions, convert to &str
            // because String doesn't implement PartialOrd<&str>
            let left_converting = left_is_string_index || left_is_string_var || left_needs_as_str;
            if is_ordering_compare && left_converting {
                left_expr = parse_quote! { (#left_expr).as_str() };
            }
            // Convert right side to &str if it's a string index or string variable
            // (when left is also converting, both sides must be &str for PartialOrd)
            if is_ordering_compare
                && (right_is_string_index || (right_is_string_var && left_converting))
            {
                right_expr = parse_quote! { (#right_expr).as_str() };
            }
            // DEPYLER-99MODE-S9: When left is converting to &str and right is a string literal,
            // ensure right is also &str (bare literal) not String (.to_string())
            // Otherwise we get &str >= String which fails
            if is_ordering_compare && left_converting {
                if let HirExpr::Literal(Literal::String(s)) = right {
                    let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                    right_expr = parse_quote! { #lit };
                }
            }

            // For equality comparisons, handle String == &String case
            // by dereferencing the &String side to get String
            // Right side could be:
            // - A variable (HirExpr::Var)
            // - An attribute like args.target (HirExpr::Attribute)
            let right_is_ref_pattern =
                matches!(right, HirExpr::Var(_)) || matches!(right, HirExpr::Attribute { .. });
            if matches!(op, BinOp::Eq | BinOp::NotEq)
                && left_is_string_index
                && right_is_ref_pattern
            {
                // Right side might be &String from ref pattern - deref to String for comparison
                right_expr = parse_quote! { *#right_expr };
            }

            // DEPYLER-1045: Handle char vs &str comparison
            // When iterating over string.chars(), the loop variable is Rust `char` type.
            // Comparing char with &str doesn't work - need to convert char to String.
            if matches!(op, BinOp::Eq | BinOp::NotEq) {
                let left_is_char_iter = if let HirExpr::Var(name) = left {
                    self.ctx.char_iter_vars.contains(name)
                } else {
                    false
                };
                let right_is_char_iter = if let HirExpr::Var(name) = right {
                    self.ctx.char_iter_vars.contains(name)
                } else {
                    false
                };

                // Convert char to String for comparison
                if left_is_char_iter && !right_is_char_iter {
                    left_expr = parse_quote! { #left_expr.to_string() };
                }
                if right_is_char_iter && !left_is_char_iter {
                    right_expr = parse_quote! { #right_expr.to_string() };
                }
            }
        }

        match op {
            // DEPYLER-REFACTOR-001 Phase 2.7: Delegate to extracted helper
            BinOp::In => self.convert_containment_op(false, left, right, left_expr, right_expr),
            BinOp::NotIn => self.convert_containment_op(true, left, right, left_expr, right_expr),
            // DEPYLER-0926: Vector-Vector addition for trueno
            // trueno Vector doesn't implement Add trait, use method call instead
            BinOp::Add if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.add(&#right_expr).expect("arithmetic overflow") })
            }
            // DEPYLER-0928: Vector + scalar - element-wise addition
            BinOp::Add if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#left_expr.as_slice().iter().map(|&x| x + #right_expr as f32).collect())
                })
            }
            // DEPYLER-0928: scalar + Vector - element-wise addition (commutative)
            BinOp::Add if self.expr_returns_float(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#right_expr.as_slice().iter().map(|&x| x + #left_expr as f32).collect())
                })
            }
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Add => self.convert_add_op(left, right, left_expr, right_expr, op),
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
            // DEPYLER-0575: Vector-scalar subtraction for trueno
            // trueno Vector doesn't implement Sub<f32>, so use as_slice().iter().map()
            BinOp::Sub if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#left_expr.as_slice().iter().map(|&x| x - #right_expr).collect())
                })
            }
            // DEPYLER-0926: Vector-Vector subtraction for trueno
            // trueno Vector doesn't implement Sub trait, use method call instead
            BinOp::Sub if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.sub(&#right_expr).expect("arithmetic overflow") })
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if self.is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    // Wrap left_expr in parens because it contains a cast: (arr.len() as i32).saturating_sub(x)
                    // Without parens, Rust parses "as i32.saturating_sub" incorrectly
                    Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) })
                } else {
                    // DEPYLER-0758: Dereference borrowed params in arithmetic operations
                    // Fixes E0369: cannot subtract NaiveDate from &NaiveDate
                    let left_deref = self.deref_if_borrowed_param(left, left_expr);
                    let right_deref = self.deref_if_borrowed_param(right, right_expr);

                    // DEPYLER-0582: Coerce int to float if operating with float
                    let left_coerced = self.coerce_int_to_float_if_needed(left_deref, left, right);
                    let right_coerced =
                        self.coerce_int_to_float_if_needed(right_deref, right, left);

                    // DEPYLER-1109: Universal PyOps Dispatch for subtraction
                    if self.ctx.type_mapper.nasa_mode {
                        return Ok(parse_quote! { #left_coerced.py_sub(#right_coerced) });
                    }

                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_coerced #rust_op #right_coerced })
                }
            }
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Mul => self.convert_mul_op(left, right, left_expr, right_expr, op),
            // DEPYLER-0575: Vector-scalar division for trueno
            // trueno Vector doesn't implement Div<f32>, so use as_slice().iter().map()
            BinOp::Div if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#left_expr.as_slice().iter().map(|&x| x / #right_expr).collect())
                })
            }
            // DEPYLER-0926: Vector-Vector division for trueno
            // trueno Vector doesn't implement Div trait, use method call instead
            BinOp::Div if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.div(&#right_expr).expect("division failed") })
            }
            BinOp::Div => {
                // DEPYLER-0188: Check if this is pathlib Path division (path / "segment")
                // Python: Path(__file__).parent / "file.py"
                // Rust: PathBuf::from(file!()).parent().unwrap().join("file.py")
                if self.is_path_expr(left) {
                    // Convert division to .join() for path concatenation
                    return Ok(parse_quote! { #left_expr.join(#right_expr) });
                }

                // DEPYLER-1109: Universal PyOps Dispatch for division
                if self.ctx.type_mapper.nasa_mode {
                    return Ok(parse_quote! { #left_expr.py_div(#right_expr) });
                }

                // DEPYLER-0658: Check if either operand is a float
                // Rust can't divide i32 by f64 or vice versa - need to cast both to f64
                let left_is_float = self.expr_returns_float(left);
                let right_is_float = self.expr_returns_float(right);
                let has_float_operand = left_is_float || right_is_float;

                // v3.16.0 Phase 2: Python's `/` always returns float
                // Rust's `/` does integer division when both operands are integers
                // Check if we need to cast to float based on return type context
                let needs_float_division = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .map(return_type_expects_float)
                    .unwrap_or(false);

                if needs_float_division || has_float_operand {
                    // Cast both operands to f64 for Python float division semantics
                    // or for mixed int/float operations
                    // DEPYLER-0802: Double-wrap operands to ensure correct operator precedence
                    // Without inner parens: (n - 1 as f64) parses as (n - (1 as f64)) due to `as` precedence
                    // With inner parens: ((n - 1) as f64) correctly casts the entire expression
                    Ok(parse_quote! { ((#left_expr) as f64) / ((#right_expr) as f64) })
                } else {
                    // Regular division (int/int → int, float/float → float)
                    let rust_op = convert_binop(op)?;
                    // DEPYLER-0582: Wrap operands in parens if they have lower precedence
                    let left_wrapped = precedence::parenthesize_if_lower_precedence(left_expr, op);
                    let right_wrapped =
                        precedence::parenthesize_if_lower_precedence(right_expr, op);
                    Ok(syn::Expr::Binary(syn::ExprBinary {
                        attrs: vec![],
                        left: Box::new(left_wrapped),
                        op: rust_op,
                        right: Box::new(right_wrapped),
                    }))
                }
            }
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Pow => self.convert_pow_op(left, right, left_expr, right_expr),
            // DEPYLER-0422: Logical operators need Python truthiness conversion
            // Python: `if a and b:` where a, b are strings/lists/etc.
            // Rust: `if (!a.is_empty()) && (!b.is_empty())`
            BinOp::And | BinOp::Or => {
                // DEPYLER-0633: For Option or default pattern, use unwrap_or instead of ||
                // Python: path = env.get("KEY") or "default"
                // Rust: path = env.get("KEY").unwrap_or("default")
                if matches!(op, BinOp::Or) && expr_analysis::looks_like_option_expr(left) {
                    // The right side is the default value - convert to unwrap_or
                    return Ok(parse_quote! { #left_expr.unwrap_or(#right_expr.to_string()) });
                }

                // DEPYLER-0786: Python `or` returns first truthy value, not a boolean
                // For strings: `value or default` → `if value.is_empty() { default } else { value }`
                // This preserves the string type instead of returning bool
                if matches!(op, BinOp::Or) {
                    let left_is_string = self.expr_is_string_type(left);
                    let right_is_string = self.expr_is_string_type(right)
                        || matches!(right, HirExpr::Literal(Literal::String(_)));

                    // DEPYLER-0786: If right is a string literal, assume left is also string
                    // This handles cases where function parameters aren't tracked in var_types
                    // Example: `email or ""` where email: &str is a function parameter
                    let infer_left_from_right =
                        matches!(right, HirExpr::Literal(Literal::String(_)));

                    if (left_is_string || infer_left_from_right) && right_is_string {
                        // Generate: if left.is_empty() { right } else { left }
                        // Need to clone left_expr since we use it twice
                        return Ok(
                            parse_quote! { if #left_expr.is_empty() { #right_expr.to_string() } else { #left_expr.to_string() } },
                        );
                    }
                }

                // DEPYLER-1127: Python `or`/`and` are VALUE-RETURNING, not boolean operators
                // Python: x or y → returns x if truthy, else y (not True/False!)
                // Python: x and y → returns x if falsy, else y (not True/False!)
                // This is fundamentally different from Rust's || and && which return bool.
                //
                // Pattern: `wait = get_time() or 0.1` should return the time or 0.1, not bool
                // Pattern: `result = data and process(data)` should return data or process result
                //
                // We detect non-boolean operands and generate value-returning if-else:
                // x or y → { let _v = x; if _v.is_true() { _v } else { y } }
                // x and y → { let _v = x; if !_v.is_true() { _v } else { y } }
                let left_is_bool_expr = self.expr_is_boolean_expr(left);
                let right_is_bool_expr = self.expr_is_boolean_expr(right);

                // If BOTH operands are boolean expressions, use standard && / ||
                // Otherwise, generate value-returning pattern
                if !left_is_bool_expr || !right_is_bool_expr {
                    // Check if either operand is DepylerValue (needs special handling)
                    let left_is_depyler = self.expr_is_depyler_value(left);
                    let right_is_depyler = self.expr_is_depyler_value(right);
                    let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));
                    let right_is_float_literal =
                        matches!(right, HirExpr::Literal(Literal::Float(_)));

                    // When either side is DepylerValue, wrap the other side too
                    if left_is_depyler || right_is_depyler {
                        // Wrap right-hand side if it's a literal and left is DepylerValue
                        let right_wrapped: syn::Expr = if left_is_depyler && !right_is_depyler {
                            if right_is_int_literal {
                                parse_quote! { DepylerValue::Int(#right_expr as i64) }
                            } else if right_is_float_literal {
                                parse_quote! { DepylerValue::Float(#right_expr as f64) }
                            } else {
                                // Try .into() conversion for other types
                                parse_quote! { DepylerValue::from(#right_expr) }
                            }
                        } else {
                            right_expr.clone()
                        };

                        // Wrap left-hand side if right is DepylerValue and left is not
                        let left_wrapped: syn::Expr = if right_is_depyler && !left_is_depyler {
                            let left_is_int = matches!(left, HirExpr::Literal(Literal::Int(_)));
                            let left_is_float = matches!(left, HirExpr::Literal(Literal::Float(_)));
                            if left_is_int {
                                parse_quote! { DepylerValue::Int(#left_expr as i64) }
                            } else if left_is_float {
                                parse_quote! { DepylerValue::Float(#left_expr as f64) }
                            } else {
                                parse_quote! { DepylerValue::from(#left_expr) }
                            }
                        } else {
                            left_expr.clone()
                        };

                        // Generate value-returning pattern with PyTruthy
                        return match op {
                            BinOp::Or => Ok(parse_quote! {
                                {
                                    let _or_lhs = #left_wrapped;
                                    if _or_lhs.is_true() { _or_lhs } else { #right_wrapped }
                                }
                            }),
                            BinOp::And => Ok(parse_quote! {
                                {
                                    let _and_lhs = #left_wrapped;
                                    if !_and_lhs.is_true() { _and_lhs } else { #right_wrapped }
                                }
                            }),
                            _ => unreachable!(),
                        };
                    }

                    // For non-DepylerValue, non-boolean expressions:
                    // Generate value-returning pattern only for numeric defaults
                    // DEPYLER-1127: If left could be DepylerValue (e.g., from dict.get chain),
                    // we need to wrap the literal in DepylerValue to ensure type match.
                    // Detection of dict chains: .get(), .cloned(), .unwrap_or() patterns
                    let left_might_be_depyler = self.expr_might_be_depyler_value(left);

                    if right_is_int_literal || right_is_float_literal {
                        // If left might be DepylerValue, wrap the literal
                        let right_safe: syn::Expr = if left_might_be_depyler {
                            if right_is_int_literal {
                                parse_quote! { DepylerValue::Int(#right_expr as i64) }
                            } else {
                                parse_quote! { DepylerValue::Float(#right_expr as f64) }
                            }
                        } else {
                            right_expr.clone()
                        };

                        return match op {
                            BinOp::Or => Ok(parse_quote! {
                                {
                                    let _or_lhs = #left_expr;
                                    if _or_lhs.is_true() { _or_lhs } else { #right_safe }
                                }
                            }),
                            BinOp::And => Ok(parse_quote! {
                                {
                                    let _and_lhs = #left_expr;
                                    if !_and_lhs.is_true() { _and_lhs } else { #right_safe }
                                }
                            }),
                            _ => unreachable!(),
                        };
                    }
                }

                // Fall through: Apply truthiness conversion to both operands for bool context
                let left_converted = Self::apply_truthiness_conversion(left, left_expr, self.ctx);
                let right_converted =
                    Self::apply_truthiness_conversion(right, right_expr, self.ctx);

                // Generate the logical operator
                match op {
                    BinOp::And => Ok(parse_quote! { (#left_converted) && (#right_converted) }),
                    BinOp::Or => Ok(parse_quote! { (#left_converted) || (#right_converted) }),
                    _ => unreachable!(),
                }
            }
            _ => {
                // DEPYLER-1109: Universal PyOps Dispatch for modulo
                // In NASA mode, use PyOps trait methods for ALL arithmetic operations
                if matches!(op, BinOp::Mod) && self.ctx.type_mapper.nasa_mode {
                    return Ok(parse_quote! { #left_expr.py_mod(#right_expr) });
                }

                let rust_op = convert_binop(op)?;
                // DEPYLER-0339: Construct syn::ExprBinary directly instead of using parse_quote!
                // parse_quote! doesn't properly handle interpolated syn::BinOp values

                // DEPYLER-1051: Coerce int to float for comparison operators
                // Python allows comparing float with int: `x == 0` where x is float
                // Rust requires same types: `x == 0.0`
                let is_comparison = matches!(
                    op,
                    BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq
                );
                let (left_coerced, right_coerced) = if is_comparison {
                    (
                        self.coerce_int_to_float_if_needed(left_expr, left, right),
                        self.coerce_int_to_float_if_needed(right_expr, right, left),
                    )
                } else {
                    (left_expr, right_expr)
                };

                // DEPYLER-0576: Parenthesize right side when it's a unary negation
                // Prevents "<-" tokenization issue: x < -20.0 becomes x<- 20.0 without parens
                let right_expr_final = if matches!(
                    right,
                    HirExpr::Unary {
                        op: UnaryOp::Neg,
                        ..
                    }
                ) {
                    parse_quote! { (#right_coerced) }
                } else {
                    right_coerced
                };

                Ok(syn::Expr::Binary(syn::ExprBinary {
                    attrs: vec![],
                    left: Box::new(left_coerced),
                    op: rust_op,
                    right: Box::new(right_expr_final),
                }))
            }
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.8: Extracted power operator helper
    ///
    /// Handles Python power operator with type-aware behavior:
    /// - Integer base with positive int exp: base.checked_pow(exp as u32)
    /// - Integer base with negative exp: (base as f64).powf(exp as f64)
    /// - Float base or exp: (base as f64).powf(exp as f64)
    /// - Variables: runtime type selection
    ///
    /// # Complexity: 7
    pub(crate) fn convert_pow_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        // CITL: Trace power operation type decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "pow_operation",
            chosen = "runtime_dispatch",
            alternatives = ["checked_pow", "powf_float", "powi_int"],
            confidence = 0.82
        );

        // DEPYLER-0699: Wrap expressions in explicit parentheses to ensure
        // correct operator precedence when casting (as binds tighter than * and +)
        let left_paren = Self::wrap_in_parens(left_expr.clone());
        let right_paren = Self::wrap_in_parens(right_expr.clone());

        match (left, right) {
            // Integer literal base with integer literal exponent
            (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                if *exp < 0 {
                    // Negative exponent: convert to float
                    Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    })
                } else {
                    // Positive integer exponent: use checked_pow
                    // DEPYLER-0405: Cast to i32 for concrete type
                    Ok(parse_quote! {
                        (#left_paren as i32).checked_pow(#right_paren as u32)
                            .expect("Power operation overflowed")
                    })
                }
            }
            // Float literal base: always use .powf()
            // DEPYLER-0408: Cast to f64 for concrete type
            (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // Any base with float exponent: use .powf()
            (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // DEPYLER-1072: Float-typed expression as exponent (e.g., 1.0 / n)
            // Division in Python always returns float, so 1.0/n is float
            _ if self.expr_returns_float(right) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // DEPYLER-1072: Float-typed expression as base
            _ if self.expr_returns_float(left) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // Variables or complex expressions: runtime type selection
            _ => {
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

                // DEPYLER-0405: Runtime type selection
                Ok(parse_quote! {
                    {
                        if #right_expr >= 0 && (#right_expr as i64) <= (u32::MAX as i64) {
                            (#left_paren as i32).checked_pow(#right_paren as u32)
                                .expect("Power operation overflowed")
                        } else {
                            (#left_paren as f64).powf(#right_paren as f64) as #target_type
                        }
                    }
                })
            }
        }
    }

    /// DEPYLER-0699: Wrap expression in explicit parentheses
    /// This ensures correct operator precedence when casting
    /// Uses a block expression { expr } which is guaranteed to not be optimized away
    pub(crate) fn wrap_in_parens(expr: syn::Expr) -> syn::Expr {
        // DEPYLER-0707: Construct block directly instead of using parse_quote!
        // parse_quote! re-parses tokens which can fail with complex expressions
        // that have unusual token spacing (e.g., "u32 :: MAX" instead of "u32::MAX")
        syn::Expr::Block(syn::ExprBlock {
            attrs: vec![],
            label: None,
            block: syn::Block {
                brace_token: syn::token::Brace::default(),
                stmts: vec![syn::Stmt::Expr(expr, None)],
            },
        })
    }

    /// DEPYLER-REFACTOR-001 Phase 2.8: Extracted multiplication operator helper
    ///
    /// Handles Python multiplication with type-aware behavior:
    /// - String repetition: "abc" * 3 → "abc".repeat(3)
    /// - Array creation: [0] * 5 → [0; 5]
    /// - Arithmetic multiplication: a * b
    ///
    /// # Complexity: 7
    pub(crate) fn convert_mul_op(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        // DEPYLER-0302: String repetition
        // DEPYLER-0908: Resolved false positive when variable could be either string or int
        // ONLY use .repeat() when one side is DEFINITELY a string LITERAL
        // Variables are NEVER treated as strings for multiplication because:
        // 1. var_types can have stale type info from different branches
        // 2. It's safer to generate `*` which will fail at compile time if wrong
        //    than to generate `.repeat()` which produces wrong semantics silently
        let left_is_string_literal = matches!(left, HirExpr::Literal(Literal::String(_)));
        let right_is_string_literal = matches!(right, HirExpr::Literal(Literal::String(_)));
        let left_is_int_literal = matches!(left, HirExpr::Literal(Literal::Int(_)));
        let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));

        // DEPYLER-0908: Only trust literals, not variable type inference
        // This is conservative but correct - produces compile error rather than wrong behavior
        if left_is_string_literal && right_is_int_literal {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_string_literal {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // DEPYLER-0950: String literal * int variable (e.g., "=" * width)
        // Safe because string literal is definite, and we verify int variable type
        let right_is_int_var_from_type = if let HirExpr::Var(sym) = right {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };
        let left_is_int_var_from_type = if let HirExpr::Var(sym) = left {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };

        if left_is_string_literal && right_is_int_var_from_type {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_var_from_type && right_is_string_literal {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // For variable * literal patterns, check if variable is DEFINITELY not numeric
        // by looking for clear string method calls in its lineage
        let left_is_string_var = if let HirExpr::Var(sym) = left {
            // Only consider it string if we see string-specific patterns
            // NOT from var_types which can be stale across branches
            let name = sym.as_str();
            name == "text" || name == "s" || name == "line" || name.ends_with("_str")
        } else {
            false
        };
        let right_is_string_var = if let HirExpr::Var(sym) = right {
            let name = sym.as_str();
            name == "text" || name == "s" || name == "line" || name.ends_with("_str")
        } else {
            false
        };

        // Variable * int literal - only use repeat if variable name strongly suggests string
        if left_is_string_var && right_is_int_literal {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_string_var {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // DEPYLER-0950: String var * int var - use explicit type annotations from context
        // This is safer than arbitrary inference because param types come from annotations
        let right_is_int_var = if let HirExpr::Var(sym) = right {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };
        let left_is_int_var = if let HirExpr::Var(sym) = left {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };

        // String-named var * int-typed var → use .repeat()
        if left_is_string_var && right_is_int_var {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_var && right_is_string_var {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // DEPYLER-0817: Byte string repetition
        // Python: b"hello" * n → Rust: b"hello".repeat(n as usize)
        // Returns Vec<u8> which matches Python bytes behavior
        let left_is_bytes = matches!(left, HirExpr::Literal(Literal::Bytes(_)));
        let right_is_bytes = matches!(right, HirExpr::Literal(Literal::Bytes(_)));
        if left_is_bytes && right_is_int_literal {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_bytes {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // Array creation: [value] * n or n * [value]
        // DEPYLER-1129: Always use Vec for consistency with PyMul trait
        match (left, right) {
            // Pattern: [x] * n (any size → Vec)
            (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                if elts.len() == 1 && *size > 0 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { vec![#elem; #size_lit] })
            }
            // Pattern: n * [x] (any size → Vec)
            (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                if elts.len() == 1 && *size > 0 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { vec![#elem; #size_lit] })
            }
            // DEPYLER-0579: Pattern: [x] * var (variable size → Vec)
            // Example: [0.0] * n_params → vec![0.0; n_params as usize]
            (HirExpr::List(elts), HirExpr::Var(_)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; #right_expr as usize] })
            }
            // DEPYLER-0579: Pattern: var * [x] (variable size → Vec)
            (HirExpr::Var(_), HirExpr::List(elts)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; #left_expr as usize] })
            }
            // DEPYLER-0794: Pattern: [x] * expr (any expression for size → Vec)
            // Example: [True] * (limit + 1) → vec![true; (limit + 1) as usize]
            // Note: Parentheses needed because `as` has lower precedence than arithmetic
            (HirExpr::List(elts), _) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; (#right_expr) as usize] })
            }
            // DEPYLER-0794: Pattern: expr * [x] (any expression for size → Vec)
            (_, HirExpr::List(elts)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; (#left_expr) as usize] })
            }
            // DEPYLER-0926: Vector-Vector multiplication for trueno
            // trueno Vector doesn't implement Mul trait, use method call instead
            _ if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.mul(&#right_expr).expect("multiplication overflow") })
            }
            // DEPYLER-0926: Vector-scalar multiplication for trueno
            // trueno Vector has scale() method for scalar multiplication
            _ if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! { #left_expr.scale(#right_expr as f32).expect("scale failed") })
            }
            // DEPYLER-0926: scalar-Vector multiplication for trueno (commutative)
            _ if self.expr_returns_float(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #right_expr.scale(#left_expr as f32).expect("scale failed") })
            }
            // DEPYLER-0928: Vector * integer - convert integer to f32 for scale()
            _ if self.is_numpy_array_expr(left)
                && matches!(right, HirExpr::Literal(Literal::Int(_))) =>
            {
                Ok(parse_quote! { #left_expr.scale(#right_expr as f32).expect("scale failed") })
            }
            // DEPYLER-0928: integer * Vector - convert integer to f32 for scale()
            _ if matches!(left, HirExpr::Literal(Literal::Int(_)))
                && self.is_numpy_array_expr(right) =>
            {
                Ok(parse_quote! { #right_expr.scale(#left_expr as f32).expect("scale failed") })
            }
            // Default multiplication
            _ => {
                let rust_op = convert_binop(op)?;
                // DEPYLER-0582: Coerce int to float if operating with float
                let left_coerced = self.coerce_int_to_float_if_needed(left_expr, left, right);
                let right_coerced = self.coerce_int_to_float_if_needed(right_expr, right, left);

                // DEPYLER-1109: Universal PyOps Dispatch for multiplication
                // In NASA mode, use PyOps trait methods for ALL arithmetic operations
                // This delegates type coercion to Rust's trait system (robust, compiled)
                // instead of transpiler logic (complex, brittle)
                if self.ctx.type_mapper.nasa_mode {
                    // Use .py_mul() for all multiplication - traits handle i32*f64, etc.
                    return Ok(parse_quote! { #left_coerced.py_mul(#right_coerced) });
                }

                // DEPYLER-0582: Wrap operands in parens if they have lower precedence
                let left_wrapped = precedence::parenthesize_if_lower_precedence(left_coerced, op);
                let right_wrapped = precedence::parenthesize_if_lower_precedence(right_coerced, op);
                Ok(parse_quote! { #left_wrapped #rust_op #right_wrapped })
            }
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.8: Extracted addition operator helper
    ///
    /// Handles Python addition with type-aware behavior:
    /// - List concatenation: iter().chain().cloned().collect()
    /// - String concatenation: format!("{}{}", a, b)
    /// - Arithmetic addition: a + b
    ///
    /// # Complexity: 5
    pub(crate) fn convert_add_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        // DEPYLER-0290/0299/0271: Type-aware list detection
        let is_definitely_list = self.is_list_expr(left) || self.is_list_expr(right);

        let is_list_var = match (left, right) {
            (HirExpr::Var(name), _) | (_, HirExpr::Var(name)) => self
                .ctx
                .var_types
                .get(name)
                .map(|t| matches!(t, Type::List(_)))
                .unwrap_or(false),
            _ => false,
        };

        // DEPYLER-0311: Slice concatenation
        let is_slice_concat =
            matches!(left, HirExpr::Slice { .. }) || matches!(right, HirExpr::Slice { .. });

        // DEPYLER-STRING-CONCAT: String variable detection for concatenation
        // Check if either operand is a String-typed variable
        let left_is_str_var = if let HirExpr::Var(name) = left {
            self.ctx
                .var_types
                .get(name)
                .is_some_and(|t| matches!(t, Type::String))
        } else {
            false
        };
        let right_is_str_var = if let HirExpr::Var(name) = right {
            self.ctx
                .var_types
                .get(name)
                .is_some_and(|t| matches!(t, Type::String))
        } else {
            false
        };
        let is_string_var = left_is_str_var || right_is_str_var;

        // DEPYLER-STRING-CONCAT: Check for str() calls, .to_string(), and string indexing
        let is_str_producing_expr = |expr: &HirExpr| -> bool {
            match expr {
                // str(x) call
                HirExpr::Call { func, .. } if func == "str" => true,
                // x.to_string() method call (not in HIR but detect common patterns)
                HirExpr::MethodCall { method, .. }
                    if method == "to_string" || method == "format" =>
                {
                    true
                }
                // String indexing: text[i] produces a character/String
                // Check if base is a string type variable
                HirExpr::Index { base, .. } => {
                    if let HirExpr::Var(var_name) = base.as_ref() {
                        // Check var_types for String type
                        self.ctx
                            .var_types
                            .get(var_name)
                            .map(|t| matches!(t, Type::String))
                            .unwrap_or(false)
                            || self.is_string_base(base)
                    } else if let HirExpr::Attribute { attr: _, .. } = base.as_ref() {
                        // args.text, args.prefix etc.
                        self.is_string_base(base)
                    } else {
                        false
                    }
                }
                _ => false,
            }
        };

        // DEPYLER-99MODE-S9: Detect char iteration variables in string concat
        // When iterating over string chars, the loop var is `char` type.
        // char + String or String + char is string concatenation.
        let left_is_char_iter = if let HirExpr::Var(name) = left {
            self.ctx.char_iter_vars.contains(name)
        } else {
            false
        };
        let right_is_char_iter = if let HirExpr::Var(name) = right {
            self.ctx.char_iter_vars.contains(name)
        } else {
            false
        };
        let is_char_string_concat =
            (left_is_char_iter && (is_string_var || is_str_producing_expr(right)))
                || (right_is_char_iter && (is_string_var || is_str_producing_expr(left)))
                || (left_is_char_iter
                    && matches!(right, HirExpr::Literal(Literal::String(_))))
                || (right_is_char_iter
                    && matches!(left, HirExpr::Literal(Literal::String(_))));

        // String detection - includes literals, variable types, str() calls, string indexing
        // NOTE: Do NOT use current_return_type here - just because a function returns String
        // doesn't mean all + operations are string concatenation (e.g., loop counter: i = i + 1)
        let is_definitely_string = matches!(left, HirExpr::Literal(Literal::String(_)))
            || matches!(right, HirExpr::Literal(Literal::String(_)))
            || is_string_var
            || is_str_producing_expr(left)
            || is_str_producing_expr(right)
            || is_char_string_concat;

        // DEPYLER-0672: Additional heuristic - check generated expressions for string patterns
        // Detect CSE temp vars from format!() and .unwrap_or_default() patterns
        let left_str = quote! { #left_expr }.to_string();
        let right_str = quote! { #right_expr }.to_string();
        // DEPYLER-0693: Be more precise about string detection
        // unwrap_or_default on array indexing (get()) returns the element default, not necessarily string
        // Only treat as string if we see string-producing patterns like to_string() or format!()
        let has_to_string = left_str.contains("to_string") || right_str.contains("to_string");
        let has_format = left_str.contains("format !") || right_str.contains("format !");
        let looks_like_string = has_format
            || (left_str.contains("_cse_temp_") && has_to_string)
            || (right_str.contains("_cse_temp_") && has_to_string);

        if (is_definitely_list || is_slice_concat || is_list_var) && !is_definitely_string {
            // List/slice concatenation
            Ok(parse_quote! {
                #left_expr.iter().chain(#right_expr.iter()).cloned().collect::<Vec<_>>()
            })
        } else if is_definitely_string || looks_like_string {
            // String concatenation
            Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
        } else {
            // Arithmetic addition
            let rust_op = convert_binop(op)?;
            // DEPYLER-0582: Coerce int to float if operating with float
            let left_coerced = self.coerce_int_to_float_if_needed(left_expr.clone(), left, right);
            let right_coerced = self.coerce_int_to_float_if_needed(right_expr.clone(), right, left);

            // DEPYLER-1064: Type annotate integer literals when used with DepylerValue
            // This prevents Rust's type inference ambiguity with multiple Add impls
            let is_left_depyler = if let HirExpr::Var(name) = left {
                self.ctx.type_mapper.nasa_mode
                    && self.ctx.var_types.get(name).is_some_and(|t| {
                        matches!(t, Type::Unknown)
                            || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                    })
            } else {
                false
            };
            let is_right_depyler = if let HirExpr::Var(name) = right {
                self.ctx.type_mapper.nasa_mode
                    && self.ctx.var_types.get(name).is_some_and(|t| {
                        matches!(t, Type::Unknown)
                            || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                    })
            } else {
                false
            };

            // DEPYLER-1109: Universal PyOps Dispatch
            // In NASA mode, use PyOps trait methods for ALL arithmetic operations
            // This delegates type coercion to Rust's trait system (robust, compiled)
            // instead of transpiler logic (complex, brittle)
            if self.ctx.type_mapper.nasa_mode {
                // Use .py_add() for all arithmetic - traits handle i32+f64, String+&str, etc.
                return Ok(parse_quote! { #left_coerced.py_add(#right_coerced) });
            }

            // If one side is DepylerValue and other is int literal, type annotate the literal
            let final_left =
                if is_right_depyler && matches!(left, HirExpr::Literal(Literal::Int(_))) {
                    // Annotate left as i64 for DepylerValue's Add<i64>
                    parse_quote! { #left_coerced as i64 }
                } else {
                    left_coerced
                };
            let final_right =
                if is_left_depyler && matches!(right, HirExpr::Literal(Literal::Int(_))) {
                    // Annotate right as i64 for DepylerValue's Add<i64>
                    parse_quote! { #right_coerced as i64 }
                } else {
                    right_coerced
                };

            Ok(parse_quote! { #final_left #rust_op #final_right })
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.7: Extracted containment operator helper
    ///
    /// Handles `In` and `NotIn` binary operators with type-aware method selection.
    /// - String: .contains(&value)
    /// - Set: .contains(&value)
    /// - List: .contains(&value)
    /// - Dict/HashMap: .get(&key).is_some()
    ///
    /// # Arguments
    /// * `negate` - true for NotIn operator, false for In operator
    /// * `left` - HIR expression for the left operand (for os.environ detection)
    /// * `right` - HIR expression for the right operand (container, for type detection)
    /// * `left_expr` - Generated Rust expression for left operand
    /// * `right_expr` - Generated Rust expression for right operand
    ///
    /// # Complexity: 6
    pub(crate) fn convert_containment_op(
        &self,
        negate: bool,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0380 #3: Handle `var in os.environ` / `var not in os.environ`
        if let HirExpr::Attribute { value, attr } = right {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    // os.environ maps to std::env::var().is_ok()
                    return if negate {
                        Ok(parse_quote! { !std::env::var(#left_expr).is_ok() })
                    } else {
                        Ok(parse_quote! { std::env::var(#left_expr).is_ok() })
                    };
                }
            }
        }

        // DEPYLER-0964: Handle containment check on &mut Option<HashMap<K, V>> parameters
        // When checking `key in memo` where memo is a mut_option_dict_param,
        // we need to unwrap the Option first: memo.as_ref().unwrap().get(&key).is_some()
        if let HirExpr::Var(var_name) = right {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let needs_borrow = match left {
                    HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
                    HirExpr::Literal(Literal::String(_)) => false,
                    _ => true,
                };
                if negate {
                    if needs_borrow {
                        return Ok(
                            parse_quote! { #right_expr.as_ref().expect("value is None").get(&#left_expr).is_none() },
                        );
                    } else {
                        return Ok(
                            parse_quote! { #right_expr.as_ref().expect("value is None").get(#left_expr).is_none() },
                        );
                    }
                } else if needs_borrow {
                    return Ok(
                        parse_quote! { #right_expr.as_ref().expect("value is None").get(&#left_expr).is_some() },
                    );
                } else {
                    return Ok(
                        parse_quote! { #right_expr.as_ref().expect("value is None").get(#left_expr).is_some() },
                    );
                }
            }
        }

        // DEPYLER-0960: Check dict FIRST before string (overlapping names like "data", "result")
        // Dict check must come before string check because some names are ambiguous
        // Use .get().is_some() instead of .contains_key() for compatibility with both
        // HashMap and serde_json::Value types
        if self.is_dict_expr(right) {
            // DEPYLER-0559: Check if left side needs borrowing
            let needs_borrow = match left {
                HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
                HirExpr::Literal(Literal::String(_)) => false,
                _ => true,
            };
            // Dict/HashMap uses .get(&key).is_some() for compatibility
            if negate {
                if needs_borrow {
                    return Ok(parse_quote! { #right_expr.get(&#left_expr).is_none() });
                } else {
                    return Ok(parse_quote! { #right_expr.get(#left_expr).is_none() });
                }
            } else if needs_borrow {
                return Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() });
            } else {
                return Ok(parse_quote! { #right_expr.get(#left_expr).is_some() });
            }
        }

        // DEPYLER-0321: Type-aware container detection
        let is_string = self.is_string_type(right);
        let is_set = self.is_set_expr(right) || self.is_set_var(right);
        let is_list = self.is_list_expr(right);
        // DEPYLER-0618: Detect tuple expressions for containment check
        let is_tuple = matches!(right, HirExpr::Tuple(_));

        // DEPYLER-0559: Check if left side is already a borrowed &str
        // &str params and string literals don't need additional borrowing
        let needs_borrow = match left {
            HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
            HirExpr::Literal(Literal::String(_)) => false, // String literals are &str, no borrow needed
            _ => true, // Other expressions typically need borrowing
        };

        // DEPYLER-0618: Handle tuple containment check
        // Python: x in ("a", "b", "c") → Rust: [a, b, c].contains(&x)
        // Tuples don't have .contains() or .get(), so wrap in array slice and use .contains()
        // The right_expr is already converted, e.g., ("a".to_string(), "b".to_string())
        // We convert tuple (a, b, c) to [a, b, c] by string manipulation
        if is_tuple {
            // Convert tuple expression to array slice for .contains() support
            let right_str = right_expr.to_token_stream().to_string();
            // Replace outer parens with brackets: (a, b) → [a, b]
            let array_str = if right_str.starts_with('(') && right_str.ends_with(')') {
                format!("[{}]", &right_str[1..right_str.len() - 1])
            } else {
                format!("[{}]", right_str)
            };
            if let Ok(array_expr) = syn::parse_str::<syn::Expr>(&array_str) {
                if negate {
                    if needs_borrow {
                        return Ok(parse_quote! { !#array_expr.contains(&#left_expr) });
                    } else {
                        return Ok(parse_quote! { !#array_expr.contains(#left_expr) });
                    }
                } else if needs_borrow {
                    return Ok(parse_quote! { #array_expr.contains(&#left_expr) });
                } else {
                    return Ok(parse_quote! { #array_expr.contains(#left_expr) });
                }
            }
            // If parsing fails, fall through to default
        }

        if is_string || is_set || is_list {
            // DEPYLER-0555: For list contains with strings, use .iter().any(|s| s == value)
            // This handles both Vec<String>.contains(&str) and Vec<&str>.contains(&&str) correctly
            // because String implements PartialEq<str> and PartialEq<&str>
            //
            // Detect if right side is a list that likely contains strings:
            // - List literal with string elements
            // - Variable that could be Vec<String>
            let is_string_list = if let HirExpr::List(elems) = right {
                // Check if first element is a string literal (heuristic for list type)
                elems
                    .first()
                    .is_some_and(|e| matches!(e, HirExpr::Literal(Literal::String(_))))
            } else {
                false
            };

            // Use .iter().any() for string lists (handles &str vs String type mismatches)
            if is_list && is_string_list {
                // Use .iter().any() which works with mixed String/&str types
                if negate {
                    Ok(parse_quote! { !#right_expr.iter().any(|s| s == #left_expr) })
                } else {
                    Ok(parse_quote! { #right_expr.iter().any(|s| s == #left_expr) })
                }
            } else if is_string || is_set {
                // DEPYLER-0200: For string contains, use raw string literal or &* for Pattern trait
                // String literals should not have .to_string() added when used as patterns
                let pattern: syn::Expr = if is_string {
                    match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        HirExpr::Var(var_name) if self.ctx.char_iter_vars.contains(var_name) => {
                            // DEPYLER-1045: char can be passed directly to String.contains()
                            // No &* dereference needed - char implements Pattern trait
                            left_expr.clone()
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String (&*String -> &str)
                            // and &str (&*&str -> &str), avoiding unstable str_as_str feature
                            parse_quote! { &*#left_expr }
                        }
                    }
                } else if needs_borrow {
                    parse_quote! { &#left_expr }
                } else {
                    left_expr.clone()
                };

                // String and Set use .contains(&value)
                if negate {
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                }
            } else {
                // Regular list contains
                if negate {
                    if needs_borrow {
                        Ok(parse_quote! { !#right_expr.contains(&#left_expr) })
                    } else {
                        Ok(parse_quote! { !#right_expr.contains(#left_expr) })
                    }
                } else if needs_borrow {
                    Ok(parse_quote! { #right_expr.contains(&#left_expr) })
                } else {
                    Ok(parse_quote! { #right_expr.contains(#left_expr) })
                }
            }
        } else {
            // DEPYLER-0449: Check for serde_json::Value FIRST (dict-like key lookup)
            // Must check before left_is_string because dict keys are also strings
            let right_is_json_value = self.is_serde_json_value_expr(right);
            if right_is_json_value {
                // Dict/HashMap/serde_json::Value uses .get(key).is_some() for compatibility
                if negate {
                    if needs_borrow {
                        return Ok(parse_quote! { !#right_expr.get(&#left_expr).is_some() });
                    } else {
                        return Ok(parse_quote! { !#right_expr.get(#left_expr).is_some() });
                    }
                } else if needs_borrow {
                    return Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() });
                } else {
                    return Ok(parse_quote! { #right_expr.get(#left_expr).is_some() });
                }
            }

            // DEPYLER-0935: Check if left side is a string - if so, this is likely a substring check
            // Python: pattern in entry (where both are strings) → Rust: entry.contains(&*pattern)
            // This handles cases where type inference didn't detect the right side as a string
            let left_is_string = self.is_string_type(left);

            if left_is_string {
                // Substring containment check - use .contains()
                let pattern: syn::Expr = match left {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => {
                        // Use &* to deref-reborrow for Pattern trait compatibility
                        parse_quote! { &*#left_expr }
                    }
                };
                if negate {
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                }
            } else {
                // DEPYLER-0449: Dict/HashMap uses .get(key).is_some() for compatibility
                // Works for both HashMap and serde_json::Value
                if negate {
                    if needs_borrow {
                        Ok(parse_quote! { !#right_expr.get(&#left_expr).is_some() })
                    } else {
                        Ok(parse_quote! { !#right_expr.get(#left_expr).is_some() })
                    }
                } else if needs_borrow {
                    Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() })
                } else {
                    Ok(parse_quote! { #right_expr.get(#left_expr).is_some() })
                }
            }
        }
    }

}

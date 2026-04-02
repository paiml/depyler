//! Binary operator conversion for ExpressionConverter
//!
//! Contains convert_binary, convert_pow_op, convert_mul_op,
//! convert_add_op, convert_containment_op, wrap_in_parens.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_analysis;
use crate::rust_gen::precedence;
use crate::rust_gen::return_type_expects_float;
use crate::rust_gen::type_gen::convert_binop;
use crate::trace_decision;
use anyhow::Result;
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

        // CB-200 Final: NASA PyOps dispatch for arithmetic operators
        if self.ctx.type_mapper.nasa_mode && !is_comparison {
            if let Some(result) =
                self.try_nasa_pyops_dispatch(op, left, right, &left_expr, &right_expr)?
            {
                return Ok(result);
            }
        }

        if is_comparison {
            // CB-200 Batch 16: Delegate comparison transforms to extracted helper
            if let Some(early) =
                self.try_empty_list_comparison(op, left, right, &left_expr, &right_expr)?
            {
                return Ok(early);
            }
            self.apply_comparison_option_unwrap(
                op,
                left_is_option,
                right_is_option,
                &mut left_expr,
                &mut right_expr,
            );
            self.apply_comparison_ref_deref(left, right, &mut left_expr, &mut right_expr);
            self.apply_comparison_json_value_coerce(left, right, &mut left_expr, &mut right_expr);
            self.apply_comparison_float_coerce(left, right, &mut left_expr, &mut right_expr);
            self.apply_comparison_string_transforms(
                op,
                left,
                right,
                &mut left_expr,
                &mut right_expr,
            );
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
            // CB-200 Final: Subtraction operator delegated to extracted helper
            BinOp::Sub => self.convert_sub_op(left, right, left_expr, right_expr, op),
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
            // CB-200 Final: Division operator delegated to extracted helper
            BinOp::Div => self.convert_div_op(left, right, left_expr, right_expr, op),
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Pow => self.convert_pow_op(left, right, left_expr, right_expr),
            // CB-200 Final: Logical operators delegated to extracted helper
            BinOp::And | BinOp::Or => {
                self.convert_logical_op(op, left, right, left_expr, right_expr)
            }
            // CB-200 Final: Default operator handling delegated to extracted helper
            _ => self.convert_default_op(op, left, right, left_expr, right_expr),
        }
    }

    // ========================================================================
    // CB-200 Batch 16: Comparison helpers extracted from convert_binary
    // ========================================================================

    /// CB-200 Batch 16: Try converting empty list comparisons to .is_empty()
    fn try_empty_list_comparison(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        if !matches!(op, BinOp::Eq | BinOp::NotEq) {
            return Ok(None);
        }
        let left_is_empty_list = matches!(left, HirExpr::List(elts) if elts.is_empty());
        let right_is_empty_list = matches!(right, HirExpr::List(elts) if elts.is_empty());
        if right_is_empty_list {
            if matches!(op, BinOp::Eq) {
                return Ok(Some(parse_quote! { #left_expr.is_empty() }));
            } else {
                return Ok(Some(parse_quote! { !#left_expr.is_empty() }));
            }
        }
        if left_is_empty_list {
            if matches!(op, BinOp::Eq) {
                return Ok(Some(parse_quote! { #right_expr.is_empty() }));
            } else {
                return Ok(Some(parse_quote! { !#right_expr.is_empty() }));
            }
        }
        Ok(None)
    }

    /// CB-200 Batch 16: Unwrap Option operands for comparison
    fn apply_comparison_option_unwrap(
        &self,
        op: BinOp,
        left_is_option: bool,
        right_is_option: bool,
        left_expr: &mut syn::Expr,
        right_expr: &mut syn::Expr,
    ) {
        if left_is_option && !right_is_option {
            *left_expr = parse_quote! { #left_expr.unwrap_or_default() };
        }
        if right_is_option && !left_is_option {
            match op {
                BinOp::Lt | BinOp::LtEq => {
                    *right_expr = parse_quote! { #right_expr.unwrap_or(i32::MAX) };
                }
                BinOp::Gt | BinOp::GtEq => {
                    *right_expr = parse_quote! { #right_expr.unwrap_or(i32::MIN) };
                }
                _ => {
                    *right_expr = parse_quote! { #right_expr.unwrap_or_default() };
                }
            }
        }
    }

    /// CB-200 Batch 16: Auto-dereference ref params in comparisons (DEPYLER-1074)
    fn apply_comparison_ref_deref(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &mut syn::Expr,
        right_expr: &mut syn::Expr,
    ) {
        let left_is_ref =
            if let HirExpr::Var(name) = left { self.ctx.ref_params.contains(name) } else { false };
        let right_is_ref =
            if let HirExpr::Var(name) = right { self.ctx.ref_params.contains(name) } else { false };
        if left_is_ref && !right_is_ref {
            *left_expr = parse_quote! { (*#left_expr) };
        } else if right_is_ref && !left_is_ref {
            *right_expr = parse_quote! { (*#right_expr) };
        }
    }

    /// CB-200 Batch 16: Handle serde_json::Value comparisons (DEPYLER-0550)
    fn apply_comparison_json_value_coerce(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &mut syn::Expr,
        right_expr: &mut syn::Expr,
    ) {
        let left_is_dict_get =
            matches!(left, HirExpr::MethodCall { method, .. } if method == "get");
        let right_is_json_value = self.is_serde_json_value_expr(right);
        if left_is_dict_get && right_is_json_value {
            *right_expr = parse_quote! { #right_expr.as_str().map(|s| s.to_string()) };
        }
        let right_is_dict_get =
            matches!(right, HirExpr::MethodCall { method, .. } if method == "get");
        let left_is_json_value = self.is_serde_json_value_expr(left);
        if right_is_dict_get && left_is_json_value {
            *left_expr = parse_quote! { #left_expr.as_str().map(|s| s.to_string()) };
        }
    }

    /// CB-200 Batch 16: Coerce int to float in comparisons (DEPYLER-0575/0720/0828/0920)
    fn apply_comparison_float_coerce(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &mut syn::Expr,
        right_expr: &mut syn::Expr,
    ) {
        let left_is_float = self.expr_returns_float(left);
        let right_is_float = self.expr_returns_float(right);
        let left_is_f32 = self.expr_returns_f32(left);
        let right_is_f32 = self.expr_returns_f32(right);

        if left_is_float && !right_is_float {
            Self::coerce_operand_to_float(right, right_expr, left_is_f32);
        } else if right_is_float && !left_is_float {
            Self::coerce_operand_to_float(left, left_expr, right_is_f32);
        }
    }

    /// CB-200 Batch 16: Coerce a single operand from int to float
    fn coerce_operand_to_float(hir: &HirExpr, expr: &mut syn::Expr, use_f32: bool) {
        if let HirExpr::Literal(Literal::Int(n)) = hir {
            if use_f32 {
                let float_val = *n as f32;
                *expr = parse_quote! { #float_val };
            } else {
                let float_val = *n as f64;
                *expr = parse_quote! { #float_val };
            }
        } else if use_f32 {
            *expr = parse_quote! { (#expr as f32) };
        } else {
            *expr = parse_quote! { (#expr as f64) };
        }
    }

    /// CB-200 Batch 16: Handle string comparison transforms (ordering, equality, char iter)
    fn apply_comparison_string_transforms(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &mut syn::Expr,
        right_expr: &mut syn::Expr,
    ) {
        let is_ordering_compare = matches!(op, BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq);

        let left_is_string_index =
            matches!(left, HirExpr::Index { base, .. } if self.is_string_base(base));
        let right_is_string_index =
            matches!(right, HirExpr::Index { base, .. } if self.is_string_base(base));

        let left_is_string_var = if let HirExpr::Var(name) = left {
            if let Some(ty) = self.ctx.var_types.get(name) {
                matches!(ty, Type::String)
            } else {
                false
            }
        } else {
            false
        };

        let right_is_char_literal = matches!(
            right,
            HirExpr::Literal(Literal::String(s)) if s.len() == 1
        );

        let left_is_char_iter_var = if let HirExpr::Var(name) = left {
            self.ctx.char_iter_vars.contains(name.as_str())
        } else {
            false
        };

        // Convert char iter var vs single-char string to char literal
        if is_ordering_compare && left_is_char_iter_var && right_is_char_literal {
            if let HirExpr::Literal(Literal::String(s)) = right {
                if let Some(ch) = s.chars().next() {
                    let ch_lit = syn::LitChar::new(ch, proc_macro2::Span::call_site());
                    *right_expr = parse_quote! { #ch_lit };
                }
            }
        }

        let left_needs_as_str = is_ordering_compare
            && matches!(left, HirExpr::Var(_))
            && right_is_char_literal
            && !left_is_char_iter_var;

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
        let left_converting = left_is_string_index || left_is_string_var || left_needs_as_str;
        if is_ordering_compare && left_converting {
            *left_expr = parse_quote! { (#left_expr).as_str() };
        }
        if is_ordering_compare
            && (right_is_string_index || (right_is_string_var && left_converting))
        {
            *right_expr = parse_quote! { (#right_expr).as_str() };
        }
        if is_ordering_compare && left_converting {
            if let HirExpr::Literal(Literal::String(s)) = right {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                *right_expr = parse_quote! { #lit };
            }
        }

        // Equality: handle String == &String
        let right_is_ref_pattern =
            matches!(right, HirExpr::Var(_)) || matches!(right, HirExpr::Attribute { .. });
        if matches!(op, BinOp::Eq | BinOp::NotEq) && left_is_string_index && right_is_ref_pattern {
            *right_expr = parse_quote! { *#right_expr };
        }

        // DEPYLER-1045: Handle char vs &str comparison
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
            if left_is_char_iter && !right_is_char_iter {
                *left_expr = parse_quote! { #left_expr.to_string() };
            }
            if right_is_char_iter && !left_is_char_iter {
                *right_expr = parse_quote! { #right_expr.to_string() };
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

    // ========================================================================
    // CB-200 Final: Helpers extracted from convert_binary
    // ========================================================================

    /// CB-200 Final: NASA PyOps dispatch for arithmetic operators
    fn try_nasa_pyops_dispatch(
        &mut self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let return_expects_int = self
            .ctx
            .current_return_type
            .as_ref()
            .map(crate::rust_gen::func_gen::return_type_expects_int)
            .unwrap_or(false);

        let left_pyops = Self::pyops_suffix_int_literal(left, left_expr);
        let right_pyops = Self::pyops_suffix_int_literal(right, right_expr);

        let is_string_concat = self.is_string_or_char_concat(left, right);
        let left_typed = self.pyops_chain_type_cast(left, right, &left_pyops, is_string_concat);

        let is_set_sub =
            matches!(op, BinOp::Sub) && (self.is_set_expr(left) || self.is_set_expr(right));

        match op {
            BinOp::Add if !is_string_concat => {
                Ok(Some(parse_quote! { (#left_typed).py_add(#right_pyops) }))
            }
            BinOp::Sub if !is_set_sub => {
                Ok(Some(parse_quote! { (#left_typed).py_sub(#right_pyops) }))
            }
            BinOp::Mul => Ok(Some(parse_quote! { (#left_typed).py_mul(#right_pyops) })),
            BinOp::Div => {
                if return_expects_int {
                    Ok(Some(parse_quote! { ((#left_typed).py_div(#right_pyops) as i32) }))
                } else {
                    Ok(Some(parse_quote! { (#left_typed).py_div(#right_pyops) }))
                }
            }
            BinOp::Mod => Ok(Some(parse_quote! { (#left_typed).py_mod(#right_pyops) })),
            _ => Ok(None),
        }
    }

    /// CB-200 Final: Add i32 suffix to integer literals for PyOps type inference
    fn pyops_suffix_int_literal(hir: &HirExpr, expr: &syn::Expr) -> syn::Expr {
        if let HirExpr::Literal(Literal::Int(n)) = hir {
            if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                let lit_str = format!("{}i32", n);
                let lit = syn::LitInt::new(&lit_str, proc_macro2::Span::call_site());
                return parse_quote! { #lit };
            }
        }
        expr.clone()
    }

    /// CB-200 Final: Check if operands involve string/char concatenation
    fn is_string_or_char_concat(&self, left: &HirExpr, right: &HirExpr) -> bool {
        let left_is_string = self.expr_is_string_type(left);
        let right_is_string = self.expr_is_string_type(right);
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
        left_is_string || right_is_string || left_is_char_iter || right_is_char_iter
    }

    /// CB-200 Final: Add type cast for chained PyOps expressions
    fn pyops_chain_type_cast(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_pyops: &syn::Expr,
        is_string_concat: bool,
    ) -> syn::Expr {
        let left_is_chain = if let HirExpr::Binary { op: inner_op, .. } = left {
            matches!(inner_op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod)
        } else {
            false
        };
        if left_is_chain && !is_string_concat {
            let return_expects_float = self
                .ctx
                .current_return_type
                .as_ref()
                .map(crate::rust_gen::func_gen::return_type_expects_float)
                .unwrap_or(false);
            let operand_is_float = self.expr_is_float_type(left) || self.expr_is_float_type(right);
            if return_expects_float || operand_is_float {
                parse_quote! { (#left_pyops as f64) }
            } else {
                parse_quote! { (#left_pyops as i32) }
            }
        } else {
            left_pyops.clone()
        }
    }

    /// CB-200 Final: Subtraction operator handler
    fn convert_sub_op(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        if self.is_len_call(left) {
            return Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) });
        }
        let left_deref = self.deref_if_borrowed_param(left, left_expr);
        let right_deref = self.deref_if_borrowed_param(right, right_expr);
        let left_coerced = self.coerce_int_to_float_if_needed(left_deref, left, right);
        let right_coerced = self.coerce_int_to_float_if_needed(right_deref, right, left);

        if self.ctx.type_mapper.nasa_mode {
            return Ok(parse_quote! { #left_coerced.py_sub(#right_coerced) });
        }

        let rust_op = convert_binop(op)?;
        Ok(parse_quote! { #left_coerced #rust_op #right_coerced })
    }

    /// CB-200 Final: Division operator handler
    fn convert_div_op(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        if self.is_path_expr(left) {
            return Ok(parse_quote! { #left_expr.join(#right_expr) });
        }
        if self.ctx.type_mapper.nasa_mode {
            return Ok(parse_quote! { #left_expr.py_div(#right_expr) });
        }

        let left_is_float = self.expr_returns_float(left);
        let right_is_float = self.expr_returns_float(right);
        let has_float_operand = left_is_float || right_is_float;
        let needs_float_division =
            self.ctx.current_return_type.as_ref().map(return_type_expects_float).unwrap_or(false);

        if needs_float_division || has_float_operand {
            Ok(parse_quote! { ((#left_expr) as f64) / ((#right_expr) as f64) })
        } else {
            let rust_op = convert_binop(op)?;
            let left_wrapped = precedence::parenthesize_if_lower_precedence(left_expr, op);
            let right_wrapped = precedence::parenthesize_if_lower_precedence(right_expr, op);
            Ok(syn::Expr::Binary(syn::ExprBinary {
                attrs: vec![],
                left: Box::new(left_wrapped),
                op: rust_op,
                right: Box::new(right_wrapped),
            }))
        }
    }

    /// CB-200 Final: Logical And/Or operator handler
    fn convert_logical_op(
        &mut self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        // Option or default pattern
        if matches!(op, BinOp::Or) && expr_analysis::looks_like_option_expr(left) {
            return Ok(parse_quote! { #left_expr.unwrap_or(#right_expr.to_string()) });
        }

        // String or pattern
        if let Some(result) = self.try_string_or_pattern(op, left, right, &left_expr, &right_expr) {
            return Ok(result);
        }

        // Value-returning pattern for non-boolean operands
        if let Some(result) =
            self.try_value_returning_logical(op, left, right, &left_expr, &right_expr)?
        {
            return Ok(result);
        }

        // Fall through: standard boolean && / ||
        let left_converted = Self::apply_truthiness_conversion(left, left_expr, self.ctx);
        let right_converted = Self::apply_truthiness_conversion(right, right_expr, self.ctx);
        match op {
            BinOp::And => Ok(parse_quote! { (#left_converted) && (#right_converted) }),
            BinOp::Or => Ok(parse_quote! { (#left_converted) || (#right_converted) }),
            _ => unreachable!(),
        }
    }

    /// CB-200 Final: Try string or pattern (value or default)
    fn try_string_or_pattern(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Option<syn::Expr> {
        if !matches!(op, BinOp::Or) {
            return None;
        }
        let left_is_string = self.expr_is_string_type(left);
        let right_is_string = self.expr_is_string_type(right)
            || matches!(right, HirExpr::Literal(Literal::String(_)));
        let infer_left_from_right = matches!(right, HirExpr::Literal(Literal::String(_)));

        if (left_is_string || infer_left_from_right) && right_is_string {
            Some(
                parse_quote! { if #left_expr.is_empty() { #right_expr.to_string() } else { #left_expr.to_string() } },
            )
        } else {
            None
        }
    }

    /// CB-200 Final: Try value-returning logical operators for non-boolean operands
    fn try_value_returning_logical(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let left_is_bool_expr = self.expr_is_boolean_expr(left);
        let right_is_bool_expr = self.expr_is_boolean_expr(right);
        let either_is_bool_var = self.either_is_bool_var(left, right);

        if (left_is_bool_expr && right_is_bool_expr) || either_is_bool_var {
            return Ok(None);
        }

        // DepylerValue operands
        if let Some(result) =
            self.try_depyler_value_logical(op, left, right, left_expr, right_expr)?
        {
            return Ok(Some(result));
        }

        // Numeric literal defaults
        if let Some(result) =
            self.try_numeric_default_logical(op, left, right, left_expr, right_expr)?
        {
            return Ok(Some(result));
        }

        Ok(None)
    }

    /// CB-200 Final: Check if either operand is a declared bool variable
    fn either_is_bool_var(&self, left: &HirExpr, right: &HirExpr) -> bool {
        let l = if let HirExpr::Var(name) = left {
            matches!(self.ctx.var_types.get(name.as_str()), Some(Type::Bool))
        } else {
            false
        };
        let r = if let HirExpr::Var(name) = right {
            matches!(self.ctx.var_types.get(name.as_str()), Some(Type::Bool))
        } else {
            false
        };
        l || r
    }

    /// CB-200 Final: Handle DepylerValue operands in logical expressions
    fn try_depyler_value_logical(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let left_is_depyler = self.expr_is_depyler_value(left);
        let right_is_depyler = self.expr_is_depyler_value(right);

        if !left_is_depyler && !right_is_depyler {
            return Ok(None);
        }

        let right_wrapped =
            self.wrap_for_depyler_value(right, right_expr, left_is_depyler, right_is_depyler);
        let left_wrapped =
            self.wrap_for_depyler_value_left(left, left_expr, left_is_depyler, right_is_depyler);

        match op {
            BinOp::Or => Ok(Some(parse_quote! {
                { let _or_lhs = #left_wrapped; if _or_lhs.is_true() { _or_lhs } else { #right_wrapped } }
            })),
            BinOp::And => Ok(Some(parse_quote! {
                { let _and_lhs = #left_wrapped; if !_and_lhs.is_true() { _and_lhs } else { #right_wrapped } }
            })),
            _ => unreachable!(),
        }
    }

    /// CB-200 Final: Wrap right operand for DepylerValue logical expression
    fn wrap_for_depyler_value(
        &self,
        right: &HirExpr,
        right_expr: &syn::Expr,
        left_is_depyler: bool,
        right_is_depyler: bool,
    ) -> syn::Expr {
        if left_is_depyler && !right_is_depyler {
            if matches!(right, HirExpr::Literal(Literal::Int(_))) {
                parse_quote! { DepylerValue::Int(#right_expr as i64) }
            } else if matches!(right, HirExpr::Literal(Literal::Float(_))) {
                parse_quote! { DepylerValue::Float(#right_expr as f64) }
            } else {
                parse_quote! { DepylerValue::from(#right_expr) }
            }
        } else {
            right_expr.clone()
        }
    }

    /// CB-200 Final: Wrap left operand for DepylerValue logical expression
    fn wrap_for_depyler_value_left(
        &self,
        left: &HirExpr,
        left_expr: &syn::Expr,
        left_is_depyler: bool,
        right_is_depyler: bool,
    ) -> syn::Expr {
        if right_is_depyler && !left_is_depyler {
            if matches!(left, HirExpr::Literal(Literal::Int(_))) {
                parse_quote! { DepylerValue::Int(#left_expr as i64) }
            } else if matches!(left, HirExpr::Literal(Literal::Float(_))) {
                parse_quote! { DepylerValue::Float(#left_expr as f64) }
            } else {
                parse_quote! { DepylerValue::from(#left_expr) }
            }
        } else {
            left_expr.clone()
        }
    }

    /// CB-200 Final: Handle numeric literal defaults in logical expressions
    fn try_numeric_default_logical(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));
        let right_is_float_literal = matches!(right, HirExpr::Literal(Literal::Float(_)));

        if !right_is_int_literal && !right_is_float_literal {
            return Ok(None);
        }

        let left_might_be_depyler = self.expr_might_be_depyler_value(left);
        let right_safe: syn::Expr = if left_might_be_depyler {
            if right_is_int_literal {
                parse_quote! { DepylerValue::Int(#right_expr as i64) }
            } else {
                parse_quote! { DepylerValue::Float(#right_expr as f64) }
            }
        } else {
            right_expr.clone()
        };

        match op {
            BinOp::Or => Ok(Some(parse_quote! {
                { let _or_lhs = #left_expr; if _or_lhs.is_true() { _or_lhs } else { #right_safe } }
            })),
            BinOp::And => Ok(Some(parse_quote! {
                { let _and_lhs = #left_expr; if !_and_lhs.is_true() { _and_lhs } else { #right_safe } }
            })),
            _ => unreachable!(),
        }
    }

    /// CB-200 Final: Default operator handling (comparisons, bitwise, modulo fallback)
    fn convert_default_op(
        &mut self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        if matches!(op, BinOp::Mod) && self.ctx.type_mapper.nasa_mode {
            return Ok(parse_quote! { #left_expr.py_mod(#right_expr) });
        }

        let rust_op = convert_binop(op)?;
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

        let right_expr_final = if matches!(right, HirExpr::Unary { op: UnaryOp::Neg, .. }) {
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

    /// DEPYLER-REFACTOR-001 Phase 2.7: Extracted multiplication operator helper
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
        if let Some(result) = self.try_string_repeat(left, right, &left_expr, &right_expr) {
            return Ok(result);
        }

        if let Some(result) = self.try_bytes_repeat(left, right, &left_expr, &right_expr) {
            return Ok(result);
        }

        if let Some(result) = self.try_array_creation(left, right, &left_expr, &right_expr)? {
            return Ok(result);
        }

        self.convert_mul_numpy_or_default(left, right, left_expr, right_expr, op)
    }

    fn try_string_repeat(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Option<syn::Expr> {
        let left_is_string_literal = matches!(left, HirExpr::Literal(Literal::String(_)));
        let right_is_string_literal = matches!(right, HirExpr::Literal(Literal::String(_)));
        let left_is_int_literal = matches!(left, HirExpr::Literal(Literal::Int(_)));
        let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));

        if left_is_string_literal && right_is_int_literal {
            return Some(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_string_literal {
            return Some(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        let right_is_int_var_from_type = is_int_typed_var(right, self.ctx);
        let left_is_int_var_from_type = is_int_typed_var(left, self.ctx);

        if left_is_string_literal && right_is_int_var_from_type {
            return Some(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_var_from_type && right_is_string_literal {
            return Some(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        let left_is_string_var = is_string_named_var(left);
        let right_is_string_var = is_string_named_var(right);

        if left_is_string_var && right_is_int_literal {
            return Some(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_string_var {
            return Some(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        let right_is_int_var = is_int_typed_var(right, self.ctx);
        let left_is_int_var = is_int_typed_var(left, self.ctx);
        if left_is_string_var && right_is_int_var {
            return Some(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_var && right_is_string_var {
            return Some(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        None
    }

    fn try_bytes_repeat(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Option<syn::Expr> {
        let left_is_bytes = matches!(left, HirExpr::Literal(Literal::Bytes(_)));
        let right_is_bytes = matches!(right, HirExpr::Literal(Literal::Bytes(_)));
        let left_is_int_literal = matches!(left, HirExpr::Literal(Literal::Int(_)));
        let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));

        if left_is_bytes && right_is_int_literal {
            Some(parse_quote! { #left_expr.repeat(#right_expr as usize) })
        } else if left_is_int_literal && right_is_bytes {
            Some(parse_quote! { #right_expr.repeat(#left_expr as usize) })
        } else {
            None
        }
    }

    fn try_array_creation(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<Option<syn::Expr>> {
        match (left, right) {
            (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                if elts.len() == 1 && *size > 0 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(Some(parse_quote! { vec![#elem; #size_lit] }))
            }
            (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                if elts.len() == 1 && *size > 0 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(Some(parse_quote! { vec![#elem; #size_lit] }))
            }
            (HirExpr::List(elts), HirExpr::Var(_)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(Some(parse_quote! { vec![#elem; #right_expr as usize] }))
            }
            (HirExpr::Var(_), HirExpr::List(elts)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(Some(parse_quote! { vec![#elem; #left_expr as usize] }))
            }
            (HirExpr::List(elts), _) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(Some(parse_quote! { vec![#elem; (#right_expr) as usize] }))
            }
            (_, HirExpr::List(elts)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(Some(parse_quote! { vec![#elem; (#left_expr) as usize] }))
            }
            _ => Ok(None),
        }
    }

    fn convert_mul_numpy_or_default(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        if let Some(result) = self.try_numpy_mul(left, right, &left_expr, &right_expr) {
            return Ok(result);
        }

        let rust_op = convert_binop(op)?;
        let left_coerced = self.coerce_int_to_float_if_needed(left_expr, left, right);
        let right_coerced = self.coerce_int_to_float_if_needed(right_expr, right, left);

        if self.ctx.type_mapper.nasa_mode {
            return Ok(parse_quote! { #left_coerced.py_mul(#right_coerced) });
        }

        let left_wrapped = precedence::parenthesize_if_lower_precedence(left_coerced, op);
        let right_wrapped = precedence::parenthesize_if_lower_precedence(right_coerced, op);
        Ok(parse_quote! { #left_wrapped #rust_op #right_wrapped })
    }

    fn try_numpy_mul(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Option<syn::Expr> {
        if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) {
            return Some(
                parse_quote! { #left_expr.mul(&#right_expr).expect("multiplication overflow") },
            );
        }
        if self.is_numpy_array_expr(left) && self.expr_returns_float(right) {
            return Some(
                parse_quote! { #left_expr.scale(#right_expr as f32).expect("scale failed") },
            );
        }
        if self.expr_returns_float(left) && self.is_numpy_array_expr(right) {
            return Some(
                parse_quote! { #right_expr.scale(#left_expr as f32).expect("scale failed") },
            );
        }
        if self.is_numpy_array_expr(left) && matches!(right, HirExpr::Literal(Literal::Int(_))) {
            return Some(
                parse_quote! { #left_expr.scale(#right_expr as f32).expect("scale failed") },
            );
        }
        if matches!(left, HirExpr::Literal(Literal::Int(_))) && self.is_numpy_array_expr(right) {
            return Some(
                parse_quote! { #right_expr.scale(#left_expr as f32).expect("scale failed") },
            );
        }
        None
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
            (HirExpr::Var(name), _) | (_, HirExpr::Var(name)) => {
                self.ctx.var_types.get(name).map(|t| matches!(t, Type::List(_))).unwrap_or(false)
            }
            _ => false,
        };

        // DEPYLER-0311: Slice concatenation
        let is_slice_concat =
            matches!(left, HirExpr::Slice { .. }) || matches!(right, HirExpr::Slice { .. });

        // DEPYLER-STRING-CONCAT: String variable detection for concatenation
        // Check if either operand is a String-typed variable
        let left_is_str_var = if let HirExpr::Var(name) = left {
            self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::String))
        } else {
            false
        };
        let right_is_str_var = if let HirExpr::Var(name) = right {
            self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::String))
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
        let is_char_string_concat = (left_is_char_iter
            && (is_string_var || is_str_producing_expr(right)))
            || (right_is_char_iter && (is_string_var || is_str_producing_expr(left)))
            || (left_is_char_iter && matches!(right, HirExpr::Literal(Literal::String(_))))
            || (right_is_char_iter && matches!(left, HirExpr::Literal(Literal::String(_))));

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
        if let Some(result) = self.try_os_environ_containment(negate, right, &left_expr) {
            return result;
        }

        if let Some(result) =
            self.try_mut_option_dict_containment(negate, left, right, &left_expr, &right_expr)
        {
            return result;
        }

        if self.is_dict_expr(right) {
            let needs_borrow = self.left_needs_borrow(left);
            return Self::emit_get_check(negate, needs_borrow, &left_expr, &right_expr);
        }

        let is_string = self.is_string_type(right);
        let is_set = self.is_set_expr(right) || self.is_set_var(right);
        let is_list = self.is_list_expr(right);
        let is_tuple = matches!(right, HirExpr::Tuple(_));
        let needs_borrow = self.left_needs_borrow(left);

        if is_tuple {
            if let Some(result) =
                self.try_tuple_containment(negate, needs_borrow, &left_expr, &right_expr)
            {
                return result;
            }
        }

        if is_string || is_set || is_list {
            return self.convert_collection_containment(
                negate,
                needs_borrow,
                is_string,
                is_set,
                is_list,
                left,
                right,
                &left_expr,
                &right_expr,
            );
        }

        self.convert_fallback_containment(
            negate,
            needs_borrow,
            left,
            right,
            &left_expr,
            &right_expr,
        )
    }

    fn try_os_environ_containment(
        &self,
        negate: bool,
        right: &HirExpr,
        left_expr: &syn::Expr,
    ) -> Option<Result<syn::Expr>> {
        if let HirExpr::Attribute { value, attr } = right {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    return if negate {
                        Some(Ok(parse_quote! { !std::env::var(#left_expr).is_ok() }))
                    } else {
                        Some(Ok(parse_quote! { std::env::var(#left_expr).is_ok() }))
                    };
                }
            }
        }
        None
    }

    fn try_mut_option_dict_containment(
        &self,
        negate: bool,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Option<Result<syn::Expr>> {
        if let HirExpr::Var(var_name) = right {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let needs_borrow = self.left_needs_borrow(left);
                return Some(Self::emit_option_get_check(
                    negate,
                    needs_borrow,
                    left_expr,
                    right_expr,
                ));
            }
        }
        None
    }

    fn left_needs_borrow(&self, left: &HirExpr) -> bool {
        match left {
            HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
            HirExpr::Literal(Literal::String(_)) => false,
            _ => true,
        }
    }

    fn emit_option_get_check(
        negate: bool,
        needs_borrow: bool,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        let check = if negate { "is_none" } else { "is_some" };
        if needs_borrow {
            if check == "is_none" {
                Ok(
                    parse_quote! { #right_expr.as_ref().expect("value is None").get(&#left_expr).is_none() },
                )
            } else {
                Ok(
                    parse_quote! { #right_expr.as_ref().expect("value is None").get(&#left_expr).is_some() },
                )
            }
        } else if check == "is_none" {
            Ok(
                parse_quote! { #right_expr.as_ref().expect("value is None").get(#left_expr).is_none() },
            )
        } else {
            Ok(
                parse_quote! { #right_expr.as_ref().expect("value is None").get(#left_expr).is_some() },
            )
        }
    }

    fn emit_get_check(
        negate: bool,
        needs_borrow: bool,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        if negate {
            if needs_borrow {
                Ok(parse_quote! { #right_expr.get(&#left_expr).is_none() })
            } else {
                Ok(parse_quote! { #right_expr.get(#left_expr).is_none() })
            }
        } else if needs_borrow {
            Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() })
        } else {
            Ok(parse_quote! { #right_expr.get(#left_expr).is_some() })
        }
    }

    fn try_tuple_containment(
        &self,
        negate: bool,
        needs_borrow: bool,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Option<Result<syn::Expr>> {
        let right_str = right_expr.to_token_stream().to_string();
        let array_str = if right_str.starts_with('(') && right_str.ends_with(')') {
            format!("[{}]", &right_str[1..right_str.len() - 1])
        } else {
            format!("[{}]", right_str)
        };
        if let Ok(array_expr) = syn::parse_str::<syn::Expr>(&array_str) {
            return Some(Self::emit_contains_check(negate, needs_borrow, left_expr, &array_expr));
        }
        None
    }

    fn emit_contains_check(
        negate: bool,
        needs_borrow: bool,
        left_expr: &syn::Expr,
        container_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        if negate {
            if needs_borrow {
                Ok(parse_quote! { !#container_expr.contains(&#left_expr) })
            } else {
                Ok(parse_quote! { !#container_expr.contains(#left_expr) })
            }
        } else if needs_borrow {
            Ok(parse_quote! { #container_expr.contains(&#left_expr) })
        } else {
            Ok(parse_quote! { #container_expr.contains(#left_expr) })
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn convert_collection_containment(
        &self,
        negate: bool,
        needs_borrow: bool,
        is_string: bool,
        is_set: bool,
        is_list: bool,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        let is_string_list = if let HirExpr::List(elems) = right {
            elems.first().is_some_and(|e| matches!(e, HirExpr::Literal(Literal::String(_))))
        } else {
            false
        };

        if is_list && is_string_list {
            return Self::emit_iter_any_check(negate, left_expr, right_expr);
        }
        if is_string || is_set {
            let pattern =
                self.build_string_or_set_pattern(is_string, needs_borrow, left, left_expr);
            return Self::emit_contains_pattern(negate, &pattern, right_expr);
        }
        Self::emit_contains_check(negate, needs_borrow, left_expr, right_expr)
    }

    fn emit_iter_any_check(
        negate: bool,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        if negate {
            Ok(parse_quote! { !#right_expr.iter().any(|s| s == #left_expr) })
        } else {
            Ok(parse_quote! { #right_expr.iter().any(|s| s == #left_expr) })
        }
    }

    fn build_string_or_set_pattern(
        &self,
        is_string: bool,
        needs_borrow: bool,
        left: &HirExpr,
        left_expr: &syn::Expr,
    ) -> syn::Expr {
        if is_string {
            match left {
                HirExpr::Literal(Literal::String(s)) => {
                    let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                    parse_quote! { #lit }
                }
                HirExpr::Var(var_name) if self.ctx.char_iter_vars.contains(var_name) => {
                    left_expr.clone()
                }
                _ => {
                    parse_quote! { &*#left_expr }
                }
            }
        } else if needs_borrow {
            parse_quote! { &#left_expr }
        } else {
            left_expr.clone()
        }
    }

    fn emit_contains_pattern(
        negate: bool,
        pattern: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        if negate {
            Ok(parse_quote! { !#right_expr.contains(#pattern) })
        } else {
            Ok(parse_quote! { #right_expr.contains(#pattern) })
        }
    }

    fn convert_fallback_containment(
        &self,
        negate: bool,
        needs_borrow: bool,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
        let right_is_json_value = self.is_serde_json_value_expr(right);
        if right_is_json_value {
            return Self::emit_get_check_negated(negate, needs_borrow, left_expr, right_expr);
        }

        let left_is_string = self.is_string_type(left);
        if left_is_string {
            let pattern = self.build_string_pattern_for_left(left, left_expr);
            return Self::emit_contains_pattern(negate, &pattern, right_expr);
        }

        Self::emit_get_check_negated(negate, needs_borrow, left_expr, right_expr)
    }

    fn emit_get_check_negated(
        negate: bool,
        needs_borrow: bool,
        left_expr: &syn::Expr,
        right_expr: &syn::Expr,
    ) -> Result<syn::Expr> {
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

    fn build_string_pattern_for_left(&self, left: &HirExpr, left_expr: &syn::Expr) -> syn::Expr {
        match left {
            HirExpr::Literal(Literal::String(s)) => {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                parse_quote! { #lit }
            }
            _ => {
                parse_quote! { &*#left_expr }
            }
        }
    }
}

fn is_int_typed_var(expr: &HirExpr, ctx: &crate::rust_gen::context::CodeGenContext) -> bool {
    if let HirExpr::Var(sym) = expr {
        matches!(ctx.var_types.get(sym), Some(crate::hir::Type::Int))
    } else {
        false
    }
}

fn is_string_named_var(expr: &HirExpr) -> bool {
    if let HirExpr::Var(sym) = expr {
        let name = sym.as_str();
        name == "text" || name == "s" || name == "line" || name.ends_with("_str")
    } else {
        false
    }
}
